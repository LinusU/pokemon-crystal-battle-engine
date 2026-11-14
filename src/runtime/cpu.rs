use super::{mbc::Mbc, Gpu, InterruptVectors, Keypad, KeypadEvent, Routine};

const WRAM_SIZE: usize = 0x8000;
const ZRAM_SIZE: usize = 0x7f;

pub struct Cpu {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub sp: u16,
    pub pc: u16,
    pub cycles: u64,
    pub halted: bool,
    pub ime: bool,
    pub interrupt_vectors: InterruptVectors,
    pub rom_bank: usize,
    pub flag_z: bool,
    pub flag_n: bool,
    pub flag_h: bool,
    pub flag_c: bool,
    pub memory: std::collections::BTreeMap<u16, u8>,
    pub gpu: Gpu,
    pub keypad: Keypad,
    /// When true, the boot flow returns at the start of `intro_sequence`
    /// (after `game_init`'s display init has run). The battle-engine harness
    /// uses this to take over before the game enters the intro/title sequence.
    pub skip_intro_to_battle: bool,
    /// Move slot (0..3) stashed by the CLI action menu and consumed by the
    /// CLI patch in `move_selection_screen`. Lets one terminal prompt cover
    /// both the action and the move pick even though the engine re-enters
    /// `move_selection_screen` separately.
    pub cli_pending_move: Option<u8>,
    /// Set by the CLI capture path (`bootstrap::cli` `Action::Ball`) when a
    /// thrown Poké Ball caught the wild mon. The engine reports a catch as a
    /// plain WIN in `wBattleResult`, so there is no in-WRAM signal that
    /// distinguishes "won by KO" from "won by capture". This flag is the
    /// harness side-channel `snapshot::read_outcome` keys on to report
    /// `Outcome::Caught`. Reset per battle is unnecessary — a fresh `Cpu` is
    /// built for each run.
    pub caught_wild_mon: bool,
    /// When true, the battle is a headless "link battle": both parties are
    /// pre-seeded in WRAM and BOTH sides are driven by the CLI / playback (no
    /// AI). The harness sets `wLinkMode` so the engine takes its real link
    /// path — skipping AI, the ROM enemy-party load, and reading the enemy
    /// action from `wBattleAction`. This flag gates the three link-battle
    /// `HARNESS-PATCH` sites that replace serial I/O the link path would
    /// otherwise block on: `link_battle_send_receive_action` (enemy action
    /// comes from the CLI, not the cable), `_battle_random` (keep the seeded
    /// LCG instead of the exchanged-RNs buffer), and `exit_battle` (take the
    /// non-linked end path instead of the link-result SRAM/record display).
    /// See `bootstrap::launch_link_battle` and docs/transpiled_code.md.
    pub link_battle: bool,
    /// Open recording file. When `Some`, the CLI driver appends one
    /// `[[step]]` block per resolved decision (the action plus the `expect`
    /// state it produced) and flushes after each write so a mid-battle crash
    /// still leaves a usable recording. The header
    /// (resolved seed etc.) is written by `bootstrap::run` before any
    /// CLI prompt fires. Wrapped in `RefCell` so the immutable-borrow CLI
    /// read sites can record without an `&mut Cpu` upgrade dance.
    pub recorder: Option<std::cell::RefCell<crate::bootstrap::recorder::Recorder>>,
    /// Pre-parsed playback script. When `Some`, the CLI driver consumes one
    /// step per decision point *before* reading stdin — see
    /// [`crate::bootstrap::player`]. Wrapped in `RefCell` for the same
    /// reason as `recorder`: the CLI prompt sites that see this take `&Cpu`,
    /// not `&mut Cpu`.
    pub player: Option<std::cell::RefCell<crate::bootstrap::player::Player>>,
    /// Decision source consulted at every decision point when no playback
    /// script supplies the step (and before falling back to stdin). The
    /// library API installs one. See [`crate::bootstrap::decision`].
    pub decider: Option<std::cell::RefCell<Box<dyn crate::bootstrap::decision::Decider>>>,
    /// Set by the CLI action menu when the player picks FIGHT with no usable
    /// move (Struggle). Consumed by the `move_selection_screen` HARNESS-PATCH,
    /// which then runs the engine's own `CheckPlayerHasUsableMoves` to force
    /// Struggle exactly like the original.
    pub cli_pending_struggle: bool,
    /// Bag item picked at the CLI action menu as `(id, target, move_slot)`.
    /// The action menu then runs the engine's real `BattleMenu_Pack`; the
    /// `battle_pack_loop` HARNESS-PATCH consumes this in place of the bag UI
    /// and runs the item's real effect routine.
    pub cli_pending_item: Option<(u8, Option<u8>, Option<u8>)>,
    /// Party slot the `party_menu_select` HARNESS-PATCH returns while an item
    /// effect asks which Pokémon to use it on.
    pub cli_pending_party_target: Option<u8>,
    /// Move slot the `move_selection_screen` HARNESS-PATCH returns for the
    /// Ether/Max Ether/MysteryBerry move menu (`wMoveSelectionMenuType == 2`).
    pub cli_pending_item_move: Option<u8>,
    /// Battle text / HP-bar capture, fed by observe-only HARNESS-PATCH hooks.
    /// `None` (the default) disables capture. See [`crate::bootstrap::events`].
    pub events: Option<std::cell::RefCell<crate::bootstrap::events::EventCapture>>,
    /// Per-byte ROM overrides applied before `start_battle`. The bootstrap
    /// uses this to poke the trainer class's `TrainerClassAttributes` entry
    /// (items, AI flags) without touching the immutable static ROM slice. Each
    /// pair is a `(flat_offset, value)` in the full ROM image; `read_rom`
    /// checks this list before delegating to the MBC. At most one entry per
    /// flat offset is meaningful — duplicates are last-write-wins. Empty by
    /// default (no overhead for battles that need no poke).
    pub rom_patches: Vec<(usize, u8)>,
    /// When true, the harness has already seeded `wOTParty*` with an
    /// `[[enemy_party]]` override for a trainer battle, and
    /// `read_trainer_party` must not clobber it. The HARNESS-PATCH at the
    /// top of `read_trainer_party` early-returns when this flag is set.
    /// See `bootstrap::launch_trainer_battle` and docs/transpiled_code.md.
    pub trainer_party_override: bool,
    mbc: Mbc,
    wram: [u8; WRAM_SIZE],
    zram: [u8; ZRAM_SIZE],
    wrambank: usize,
    intf: u8,
    inte: u8,
    hdma: [u8; 4],
    hdma_status: DmaType,
    hdma_src: u16,
    hdma_dst: u16,
    hdma_len: u8,
    speed_double: bool,
    speed_switch_req: bool,
    interrupt_disable_delay: u8,
    interrupt_enable_delay: u8,
    servicing_interrupt: bool,
    div: u8,
    div_counter: u32,
    /// Seedable PRNG state used by the harness-patched [`crate::game::home::random::random`]
    /// in place of the engine's original DIV-driven entropy. xorshift64; never zero.
    /// Reseed with [`Cpu::seed_rng`].
    rng_state: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DmaType {
    NoDma,
    Gdma,
    Hdma,
}

impl Default for Cpu {
    fn default() -> Self {
        let mut cpu = Self {
            a: 0x11,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            sp: 0,
            pc: 0,
            cycles: 0,
            halted: false,
            ime: true,
            interrupt_vectors: InterruptVectors::default(),
            rom_bank: 1,
            flag_z: false,
            flag_n: false,
            flag_h: false,
            flag_c: false,
            memory: std::collections::BTreeMap::new(),
            gpu: Gpu::new_cgb(None),
            keypad: Keypad::default(),
            skip_intro_to_battle: false,
            cli_pending_move: None,
            caught_wild_mon: false,
            link_battle: false,
            recorder: None,
            player: None,
            decider: None,
            cli_pending_struggle: false,
            cli_pending_item: None,
            cli_pending_party_target: None,
            cli_pending_item_move: None,
            events: None,
            rom_patches: Vec::new(),
            trainer_party_override: false,
            mbc: Mbc::new(crate::rom::ROM),
            wram: [0; WRAM_SIZE],
            zram: [0; ZRAM_SIZE],
            wrambank: 1,
            intf: 0,
            inte: 0,
            hdma: [0; 4],
            hdma_status: DmaType::NoDma,
            hdma_src: 0,
            hdma_dst: 0,
            hdma_len: 0xff,
            speed_double: false,
            speed_switch_req: false,
            interrupt_disable_delay: 0,
            interrupt_enable_delay: 0,
            servicing_interrupt: false,
            div: 0,
            div_counter: 0,
            rng_state: Self::DEFAULT_RNG_SEED,
        };
        cpu.set_initial_hardware();
        cpu
    }
}

impl Cpu {
    const R_IF: u16 = 0xff0f;
    const R_IE: u16 = 0xffff;
    /// Fixed default seed for the harness PRNG. Picked once so behaviour is
    /// reproducible across runs even when no caller has reseeded.
    const DEFAULT_RNG_SEED: u64 = 0x9e37_79b9_7f4a_7c15;

    /// Reseed the harness PRNG used by [`crate::game::home::random::random`].
    /// A zero seed is mapped to the default constant — xorshift64 cannot
    /// recover from all-zero state.
    pub fn seed_rng(&mut self, seed: u64) {
        self.rng_state = if seed == 0 { Self::DEFAULT_RNG_SEED } else { seed };
    }

    /// Advance the harness PRNG and return the next byte. xorshift64 with the
    /// standard 13/7/17 triple, low byte taken — quality is more than enough
    /// for the engine's per-call needs (damage rolls, accuracy, AI tiebreaks).
    pub fn next_rng_byte(&mut self) -> u8 {
        let mut x = self.rng_state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.rng_state = x;
        x as u8
    }

    pub fn new_cgb(
        update_screen: Option<std::sync::mpsc::SyncSender<Vec<u8>>>,
        keypad_events: Option<std::sync::mpsc::Receiver<KeypadEvent>>,
    ) -> Self {
        let mut cpu = Self::default();
        cpu.gpu = Gpu::new_cgb(update_screen);
        cpu.keypad = Keypad::new(keypad_events);
        cpu.set_initial_hardware();
        cpu
    }

    fn set_initial_hardware(&mut self) {
        self.write_byte(0xff05, 0);
        self.write_byte(0xff06, 0);
        self.write_byte(0xff07, 0);
        self.write_byte(0xff10, 0x80);
        self.write_byte(0xff11, 0xbf);
        self.write_byte(0xff12, 0xf3);
        self.write_byte(0xff14, 0xbf);
        self.write_byte(0xff16, 0x3f);
        self.write_byte(0xff17, 0);
        self.write_byte(0xff19, 0xbf);
        self.write_byte(0xff1a, 0x7f);
        self.write_byte(0xff1b, 0xff);
        self.write_byte(0xff1c, 0x9f);
        self.write_byte(0xff1e, 0xff);
        self.write_byte(0xff20, 0xff);
        self.write_byte(0xff21, 0);
        self.write_byte(0xff22, 0);
        self.write_byte(0xff23, 0xbf);
        self.write_byte(0xff24, 0x77);
        self.write_byte(0xff25, 0xf3);
        self.write_byte(0xff26, 0xf1);
        self.write_byte(0xff40, 0x91);
        self.write_byte(0xff42, 0);
        self.write_byte(0xff43, 0);
        self.write_byte(0xff45, 0);
        self.write_byte(0xff47, 0xfc);
        self.write_byte(0xff48, 0xff);
        self.write_byte(0xff49, 0xff);
        self.write_byte(0xff4a, 0);
        self.write_byte(0xff4b, 0);
    }

    pub fn bc(&self) -> u16 {
        u16::from_be_bytes([self.b, self.c])
    }

    pub fn set_bc(&mut self, value: u16) {
        let [hi, lo] = value.to_be_bytes();
        self.b = hi;
        self.c = lo;
    }

    pub fn de(&self) -> u16 {
        u16::from_be_bytes([self.d, self.e])
    }

    pub fn set_de(&mut self, value: u16) {
        let [hi, lo] = value.to_be_bytes();
        self.d = hi;
        self.e = lo;
    }

    pub fn hl(&self) -> u16 {
        u16::from_be_bytes([self.h, self.l])
    }

    pub fn set_hl(&mut self, value: u16) {
        let [hi, lo] = value.to_be_bytes();
        self.h = hi;
        self.l = lo;
    }

    pub fn flags(&self) -> u8 {
        (u8::from(self.flag_z) << 7)
            | (u8::from(self.flag_n) << 6)
            | (u8::from(self.flag_h) << 5)
            | (u8::from(self.flag_c) << 4)
    }

    pub fn set_flags(&mut self, value: u8) {
        self.flag_z = value & 0x80 != 0;
        self.flag_n = value & 0x40 != 0;
        self.flag_h = value & 0x20 != 0;
        self.flag_c = value & 0x10 != 0;
    }

    pub fn af(&self) -> u16 {
        u16::from_be_bytes([self.a, self.flags()])
    }

    pub fn set_af(&mut self, value: u16) {
        let [a, flags] = value.to_be_bytes();
        self.a = a;
        self.set_flags(flags & 0xf0);
    }

    pub fn cycle(&mut self, cycles: u64) {
        let vram_cycles = self.perform_vramdma();
        self.cycles += cycles + u64::from(vram_cycles);
        self.gpu.do_cycle(cycles as u32 + vram_cycles);
        self.keypad.do_cycle();
        self.tick_div(cycles as u32 + vram_cycles);
        self.request_interrupt(self.gpu.interrupt);
        self.gpu.interrupt = 0;
        self.request_interrupt(self.keypad.interrupt);
        self.keypad.interrupt = 0;
        self.update_interrupt_master();
        self.service_interrupt();
    }

    fn tick_div(&mut self, cycles: u32) {
        self.div_counter = self.div_counter.wrapping_add(cycles);
        while self.div_counter >= 256 {
            self.div_counter -= 256;
            self.div = self.div.wrapping_add(1);
        }
    }

    pub fn current_rom_bank(&self) -> u8 {
        self.mbc.current_rom_bank() as u8
    }

    pub fn cartridge_has_battery(&self) -> bool {
        self.mbc.is_battery_backed()
    }

    pub fn load_cartridge_ram(&mut self, data: &[u8]) -> Result<(), &'static str> {
        self.mbc.load_ram(data)
    }

    pub fn dump_cartridge_ram(&self) -> Vec<u8> {
        self.mbc.dump_ram()
    }

    pub fn check_and_reset_cartridge_ram_updated(&mut self) -> bool {
        self.mbc.check_and_reset_ram_updated()
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x7fff => self.read_rom(addr),
            0x8000..=0x9fff => self.gpu.rb(addr),
            0xa000..=0xbfff => self.mbc.read_ram(addr),
            0xc000..=0xcfff | 0xe000..=0xefff => self.wram[usize::from(addr & 0x0fff)],
            0xd000..=0xdfff | 0xf000..=0xfdff => {
                self.wram[(self.wrambank * 0x1000) | usize::from(addr & 0x0fff)]
            }
            0xfe00..=0xfe9f => self.gpu.rb(addr),
            0xff00 => self.keypad.rb(),
            0xff04 => self.div,
            Self::R_IF => self.intf | 0xe0,
            0xff4d => 0x7e | if self.speed_double { 0x80 } else { 0 } | u8::from(self.speed_switch_req),
            0xff40..=0xff4f => self.gpu.rb(addr),
            0xff51..=0xff55 => self.hdma_read(addr),
            0xff68..=0xff6b => self.gpu.rb(addr),
            0xff70 => self.wrambank as u8,
            0xff80..=0xfffe => self.zram[usize::from(addr & 0x007f)],
            Self::R_IE => self.inte,
            _ => self.memory.get(&addr).copied().unwrap_or(0),
        }
    }

    pub fn read_word(&self, addr: u16) -> u16 {
        u16::from_le_bytes([self.read_byte(addr), self.read_byte(addr.wrapping_add(1))])
    }

    pub fn write_byte(&mut self, addr: u16, value: u8) {
        match addr {
            0x0000..=0x7fff => self.write_rom_control(addr, value),
            0x8000..=0x9fff => self.gpu.wb(addr, value),
            0xa000..=0xbfff => self.mbc.write_ram(addr, value),
            0xc000..=0xcfff | 0xe000..=0xefff => self.wram[usize::from(addr & 0x0fff)] = value,
            0xd000..=0xdfff | 0xf000..=0xfdff => {
                self.wram[(self.wrambank * 0x1000) | usize::from(addr & 0x0fff)] = value;
            }
            0xfe00..=0xfe9f => self.gpu.wb(addr, value),
            0xff00 => self.keypad.wb(value),
            0xff04 => {
                self.div = 0;
                self.div_counter = 0;
            }
            Self::R_IF => self.intf = value,
            0xff46 => self.oamdma(value),
            0xff4d => self.speed_switch_req = value & 0x01 != 0,
            0xff40..=0xff4f => self.gpu.wb(addr, value),
            0xff51..=0xff55 => self.hdma_write(addr, value),
            0xff68..=0xff6b => self.gpu.wb(addr, value),
            0xff70 => {
                self.wrambank = match value & 0x07 {
                    0 => 1,
                    n => usize::from(n),
                };
            }
            0xff80..=0xfffe => self.zram[usize::from(addr & 0x007f)] = value,
            Self::R_IE => self.inte = value,
            _ => {
                self.memory.insert(addr, value);
            }
        }
    }

    fn read_rom(&self, addr: u16) -> u8 {
        // Check the per-byte ROM overlay first. The overlay is empty for most
        // battles; the linear scan over ≤7 entries is negligible.
        if !self.rom_patches.is_empty() {
            let bank = if addr < 0x4000 { 0 } else { self.rom_bank };
            let flat = (bank * 0x4000) | usize::from(addr & 0x3fff);
            if let Some(&(_, v)) = self.rom_patches.iter().find(|&&(o, _)| o == flat) {
                return v;
            }
        }
        self.mbc.read_rom(crate::rom::ROM, addr)
    }

    /// Poke a single byte at `flat_offset` in the ROM image. Used by the
    /// bootstrap to override `TrainerClassAttributes` entries (AI flags, AI
    /// items) before `start_battle` without allocating a full ROM copy. Adds a
    /// new `(offset, value)` pair; if the same offset was already patched the
    /// new value replaces it (last-write-wins, keeps the list short).
    pub fn poke_rom(&mut self, flat_offset: usize, value: u8) {
        if let Some(entry) = self.rom_patches.iter_mut().find(|(o, _)| *o == flat_offset) {
            entry.1 = value;
        } else {
            self.rom_patches.push((flat_offset, value));
        }
    }

    fn write_rom_control(&mut self, addr: u16, value: u8) {
        self.mbc.write_rom(addr, value);
        self.rom_bank = self.mbc.current_rom_bank();
    }

    pub fn write_word(&mut self, addr: u16, value: u16) {
        let [lo, hi] = value.to_le_bytes();
        self.write_byte(addr, lo);
        self.write_byte(addr.wrapping_add(1), hi);
    }

    pub fn push_word(&mut self, value: u16) {
        let [hi, lo] = value.to_be_bytes();
        self.sp = self.sp.wrapping_sub(1);
        self.write_byte(self.sp, hi);
        self.sp = self.sp.wrapping_sub(1);
        self.write_byte(self.sp, lo);
    }

    pub fn pop_word(&mut self) -> u16 {
        let lo = self.read_byte(self.sp);
        self.sp = self.sp.wrapping_add(1);
        let hi = self.read_byte(self.sp);
        self.sp = self.sp.wrapping_add(1);
        u16::from_be_bytes([hi, lo])
    }

    pub fn reti(&mut self) {
        self.ime = true;
        self.interrupt_enable_delay = 0;
        self.pc = self.pop_word();
        self.cycle(16);
    }

    pub fn halt(&mut self) {
        self.halted = true;
        self.pc += 1;
        self.cycle(4);
        while self.halted {
            self.cycle(4);
        }
    }

    pub fn stop(&mut self) {
        // `stop` is encoded as two bytes (stop + implicit nop) by rgbasm. The
        // common CGB use is the speed-switch sequence which sets bit 0 of
        // KEY1 (rSPD = 0xff4d) before executing `stop`; in that case the CPU
        // toggles its speed and resumes execution.
        if self.speed_switch_req {
            self.speed_double = !self.speed_double;
            self.speed_switch_req = false;
        }
        self.pc = self.pc.wrapping_add(2);
        self.cycle(4);
    }

    pub fn daa(&mut self) {
        // Decimal-adjust register A after BCD addition or subtraction. Mirrors
        // rboy's `alu_daa` which is the standard reference implementation.
        let mut a = self.a;
        let mut adjust = if self.flag_c { 0x60 } else { 0x00 };
        if self.flag_h {
            adjust |= 0x06;
        }
        if !self.flag_n {
            if a & 0x0f > 0x09 {
                adjust |= 0x06;
            }
            if a > 0x99 {
                adjust |= 0x60;
            }
            a = a.wrapping_add(adjust);
        } else {
            a = a.wrapping_sub(adjust);
        }

        self.flag_c = adjust >= 0x60;
        self.flag_h = false;
        self.flag_z = a == 0;
        self.a = a;
        self.pc = self.pc.wrapping_add(1);
        self.cycle(4);
    }

    pub fn enable_interrupts_after_next_instruction(&mut self) {
        self.interrupt_enable_delay = 2;
        self.interrupt_disable_delay = 0;
    }

    pub fn disable_interrupts_after_next_instruction(&mut self) {
        self.interrupt_disable_delay = 2;
        self.interrupt_enable_delay = 0;
    }

    pub fn request_interrupt(&mut self, mask: u8) {
        self.intf |= mask & 0x1f;
    }

    fn update_interrupt_master(&mut self) {
        self.interrupt_disable_delay = match self.interrupt_disable_delay {
            2 => 1,
            1 => {
                self.ime = false;
                0
            }
            _ => 0,
        };
        self.interrupt_enable_delay = match self.interrupt_enable_delay {
            2 => 1,
            1 => {
                self.ime = true;
                0
            }
            _ => 0,
        };
    }

    fn service_interrupt(&mut self) -> bool {
        let enabled = self.inte;
        let flags = self.intf;
        let triggered = enabled & flags & 0x1f;
        if triggered == 0 {
            return false;
        }

        self.halted = false;
        if self.servicing_interrupt
            || self.interrupt_disable_delay != 0
            || self.interrupt_enable_delay != 0
        {
            return false;
        }

        if !self.ime {
            return false;
        }

        self.ime = false;
        let bit = triggered.trailing_zeros() as u8;
        self.intf = flags & !(1 << bit);
        self.push_word(self.pc);
        self.pc = 0x0040 | (u16::from(bit) << 3);
        self.cycles += 20;
        self.gpu.do_cycle(20);

        if let Some(routine) = self.interrupt_routine(bit) {
            let was_servicing_interrupt = self.servicing_interrupt;
            self.servicing_interrupt = true;
            (routine.func)(self);
            self.servicing_interrupt = was_servicing_interrupt;
        }

        true
    }

    fn interrupt_routine(&self, bit: u8) -> Option<Routine> {
        match bit {
            0 => self.interrupt_vectors.vblank,
            1 => self.interrupt_vectors.lcd_stat,
            2 => self.interrupt_vectors.timer,
            3 => self.interrupt_vectors.serial,
            4 => self.interrupt_vectors.joypad,
            _ => None,
        }
    }

    fn oamdma(&mut self, value: u8) {
        let base = u16::from(value) << 8;
        for offset in 0..0xa0 {
            let byte = self.read_byte(base + offset);
            self.gpu.wb(0xfe00 + offset, byte);
        }
    }

    fn hdma_read(&self, addr: u16) -> u8 {
        match addr {
            0xff51..=0xff54 => self.hdma[usize::from(addr - 0xff51)],
            0xff55 => self.hdma_len | if self.hdma_status == DmaType::NoDma { 0x80 } else { 0 },
            _ => 0xff,
        }
    }

    fn hdma_write(&mut self, addr: u16, value: u8) {
        match addr {
            0xff51 => self.hdma[0] = value,
            0xff52 => self.hdma[1] = value & 0xf0,
            0xff53 => self.hdma[2] = value & 0x1f,
            0xff54 => self.hdma[3] = value & 0xf0,
            0xff55 => {
                if self.hdma_status == DmaType::Hdma {
                    if value & 0x80 == 0 {
                        self.hdma_status = DmaType::NoDma;
                    }
                    return;
                }

                self.hdma_src = (u16::from(self.hdma[0]) << 8) | u16::from(self.hdma[1]);
                self.hdma_dst =
                    (u16::from(self.hdma[2]) << 8) | u16::from(self.hdma[3]) | 0x8000;
                self.hdma_len = value & 0x7f;
                self.hdma_status = if value & 0x80 != 0 {
                    DmaType::Hdma
                } else {
                    DmaType::Gdma
                };
            }
            _ => {}
        }
    }

    fn perform_vramdma(&mut self) -> u32 {
        match self.hdma_status {
            DmaType::NoDma => 0,
            DmaType::Gdma => self.perform_gdma(),
            DmaType::Hdma => self.perform_hdma(),
        }
    }

    fn perform_hdma(&mut self) -> u32 {
        if !self.gpu.may_hdma() {
            return 0;
        }

        self.perform_vramdma_row();
        if self.hdma_len == 0x7f {
            self.hdma_status = DmaType::NoDma;
        }

        8
    }

    fn perform_gdma(&mut self) -> u32 {
        let rows = u32::from(self.hdma_len) + 1;
        for _ in 0..rows {
            self.perform_vramdma_row();
        }
        self.hdma_status = DmaType::NoDma;
        rows * 8
    }

    fn perform_vramdma_row(&mut self) {
        let src = self.hdma_src;
        for offset in 0..0x10 {
            let byte = self.read_byte(src + offset);
            // The VRAM-DMA destination is a 13-bit offset within VRAM; the high
            // bits are forced so it always lands in 0x8000..=0x9fff. Without
            // this mask a long transfer that starts near the end of VRAM walks
            // off into 0xa000+ and the GPU panics on the write.
            let dst = 0x8000 | (self.hdma_dst.wrapping_add(offset) & 0x1fff);
            self.gpu.wb(dst, byte);
        }
        self.hdma_src = self.hdma_src.wrapping_add(0x10);
        self.hdma_dst = self.hdma_dst.wrapping_add(0x10);
        self.hdma_len = if self.hdma_len == 0 {
            0x7f
        } else {
            self.hdma_len - 1
        };
    }
}

#[cfg(test)]
mod tests {
    use super::Cpu;

    /// Same seed → same byte stream. The whole reason the harness exists.
    #[test]
    fn seeded_rng_is_deterministic() {
        let mut a = Cpu::default();
        let mut b = Cpu::default();
        a.seed_rng(0xdeadbeef);
        b.seed_rng(0xdeadbeef);
        let stream_a: Vec<u8> = (0..32).map(|_| a.next_rng_byte()).collect();
        let stream_b: Vec<u8> = (0..32).map(|_| b.next_rng_byte()).collect();
        assert_eq!(stream_a, stream_b);
    }

    /// Different seeds quickly diverge — guards against accidentally
    /// hard-coding the seed somewhere in the pipeline.
    #[test]
    fn different_seeds_diverge() {
        let mut a = Cpu::default();
        let mut b = Cpu::default();
        a.seed_rng(1);
        b.seed_rng(2);
        let stream_a: Vec<u8> = (0..16).map(|_| a.next_rng_byte()).collect();
        let stream_b: Vec<u8> = (0..16).map(|_| b.next_rng_byte()).collect();
        assert_ne!(stream_a, stream_b);
    }

    /// xorshift64 cannot recover from all-zero state; seed_rng must fold it
    /// to the default so a TOML `seed = 0` doesn't produce a stuck PRNG.
    #[test]
    fn zero_seed_falls_back() {
        let mut cpu = Cpu::default();
        cpu.seed_rng(0);
        // Two non-zero bytes is enough evidence the state isn't stuck.
        assert_ne!(cpu.next_rng_byte(), 0);
        assert_ne!(cpu.next_rng_byte(), 0);
    }
}
