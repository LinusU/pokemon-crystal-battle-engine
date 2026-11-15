//! Battle text: a port of the original text engine's command interpreter
//! (`home/text.asm`) that renders a text label to the exact strings the
//! emulated harness captures (see `bootstrap::events`): one string per
//! message, text-box lines separated by `\n`.
//!
//! Rendering only reads memory: the logic of `text_asm` blocks that change the
//! battle is ported where the battle code prints them; here each block only
//! chooses which text to print next.

use super::battle::Engine;
use super::rom;
use super::rsym as r;
use super::sym::*;
use crate::bootstrap::events::decode;

/// Where `PrintTextboxText` starts the cursor: `(TEXTBOX_INNERX, TEXTBOX_INNERY)`.
const TEXT_LINE1: u16 = W_TILEMAP + 14 * 20 + 1;
/// `(TEXTBOX_INNERX, TEXTBOX_INNERY + 2)`: `<LINE>`, `<CONT>`, `text_low`, `text_scroll`.
const TEXT_LINE2: u16 = W_TILEMAP + 16 * 20 + 1;
/// The tilemap (`SCREEN_WIDTH` × `SCREEN_HEIGHT`), which the engine doesn't keep.
const TILEMAP: std::ops::Range<u16> = W_TILEMAP..W_TILEMAP + 20 * 18;

/// Text being assembled into messages.
#[derive(Default)]
struct Out {
    line: String,
    lines: Vec<String>,
    messages: Vec<String>,
    /// `PlaceEnemysName` took its trainer-battle branch, which calls
    /// `Battle_GetTrainerName`.
    trainer_name: bool,
    /// The original's tile cursor while placing a string (`hl`).
    pos: u16,
    /// Its `bc`: the cursor between text commands, and where `PlaceString`
    /// leaves the end of a string.
    bc: u16,
    /// A command whose effect on the cursor isn't modelled ran (`text_asm`,
    /// `text_box`, a wait or sound): characters are captured as if on
    /// screen, nothing is stored, and `<MOBILE>` can't run.
    lost: bool,
    /// Memory the text wrote, to apply once it is rendered. Only glitches
    /// write: `<MOBILE>` scripts and the characters they send off the screen.
    writes: Vec<(u16, u8)>,
    /// `<MOBILE>` scripts running inside each other's strings.
    mobile_depth: u8,
    /// Strings placed inside each other (names, dict expansions).
    depth: u8,
}

impl Out {
    fn new() -> Self {
        Out { pos: TEXT_LINE1, bc: TEXT_LINE1, ..Default::default() }
    }

    fn push(&mut self, s: &str) {
        self.line.push_str(s);
    }

    fn newline(&mut self) {
        let line = std::mem::take(&mut self.line);
        let line = line.trim_end();
        if !line.is_empty() {
            self.lines.push(line.to_string());
        }
    }

    fn end_message(&mut self) {
        self.newline();
        if !self.lines.is_empty() {
            self.messages.push(std::mem::take(&mut self.lines).join("\n"));
        }
    }
}

/// `RunMobileScript`'s registers.
struct Mobile {
    bc: u16,
    de: Ptr,
    hl: u16,
}

/// A place text can live: ROM (flat offset) or RAM.
#[derive(Clone, Copy)]
enum Ptr {
    Rom(usize),
    Ram(u16),
    /// A CPU address with `bank` mapped at 4000-7fff, for pointers that may
    /// run from ROM into another region (`<MOBILE>` scripts' pointers).
    Cpu { addr: u16, bank: u8 },
}

impl Ptr {
    fn add(self, n: u16) -> Ptr {
        match self {
            Ptr::Rom(o) => Ptr::Rom(o + usize::from(n)),
            Ptr::Ram(a) => Ptr::Ram(a.wrapping_add(n)),
            Ptr::Cpu { addr, bank } => Ptr::Cpu { addr: addr.wrapping_add(n), bank },
        }
    }

    /// The RAM address this reads, if it reads RAM.
    fn ram(self) -> Option<u16> {
        match self {
            Ptr::Ram(a) => Some(a),
            Ptr::Cpu { addr, .. } if addr >= 0x8000 => Some(addr),
            _ => None,
        }
    }

    /// The address the CPU sees (ROM banks map at 4000-7fff).
    fn cpu_addr(self) -> u16 {
        match self {
            Ptr::Rom(o) if o < 0x4000 => o as u16,
            Ptr::Rom(o) => 0x4000 | (o % 0x4000) as u16,
            Ptr::Ram(a) | Ptr::Cpu { addr: a, .. } => a,
        }
    }
}

impl Engine {
    fn text_byte(&self, out: &Out, p: Ptr) -> u8 {
        match p {
            Ptr::Rom(o) => rom::byte(o),
            Ptr::Ram(a) => self.peek(out, a),
            Ptr::Cpu { addr: a @ 0x0000..=0x3fff, .. } => rom::byte(usize::from(a)),
            Ptr::Cpu { addr: a @ 0x4000..=0x7fff, bank } => rom::byte(rom::flat(bank, a)),
            Ptr::Cpu { addr, .. } => self.peek(out, addr),
        }
    }

    /// Memory as the text sees it: what it wrote so far, else the engine's.
    fn peek(&self, out: &Out, addr: u16) -> u8 {
        let a = if (0xe000..=0xfdff).contains(&addr) { addr - 0x2000 } else { addr };
        match out.writes.iter().rev().find(|&&(w, _)| w == a) {
            Some(&(_, v)) => v,
            None => self.mem.rb(addr),
        }
    }

    /// A write the text engine makes at `addr` (`ld [hl], a`).
    fn store(&self, out: &mut Out, addr: u16, value: u8) {
        if out.lost {
            return;
        }
        let a = if (0xe000..=0xfdff).contains(&addr) { addr - 0x2000 } else { addr };
        match a {
            // The tilemap: what's on screen, which the engine doesn't keep.
            _ if TILEMAP.contains(&a) => {}
            0xc000..=0xdfff | 0xff80..=0xfffe => out.writes.push((a, value)),
            // MBC3 RAM enable / clock latch: nothing the battle reads.
            0x0000..=0x1fff | 0x6000..=0x7fff => {}
            _ => panic!("text placed at {addr:#06x}: ROM/RAM bank switches, VRAM or I/O the engine doesn't model"),
        }
    }

    /// `.place` (`CheckDict`): `ld [hli], a` with the tile, captured as battle
    /// text when it lands in the text box.
    fn place_tile(&self, out: &mut Out, tile: u8) {
        if out.lost || crate::bootstrap::events::in_text_area(out.pos) {
            out.push(decode(tile));
        }
        self.store(out, out.pos, tile);
        out.pos = out.pos.wrapping_add(1);
    }

    /// `PlaceCommandCharacter` and friends: `PlaceString` at the cursor, then
    /// carry on where it ended (`ld h, b` / `ld l, c`).
    fn place_nested(&self, p: Ptr, out: &mut Out) {
        self.place_string(p, out);
        out.pos = out.bc;
    }

    /// Whether printing the text at `label` places `<ENEMY>` as a trainer's
    /// name, which in the original calls `Battle_GetTrainerName` (writing
    /// `wStringBuffer1`). Memoized per label: which branch `<ENEMY>` takes
    /// depends only on the battle kind, which the caller checks.
    pub fn text_places_trainer_name(&self, label: usize, choices: &[usize]) -> bool {
        thread_local! {
            static SEEN: std::cell::RefCell<std::collections::HashMap<usize, bool>> = Default::default();
        }
        if let Some(v) = SEEN.with(|m| m.borrow().get(&label).copied()) {
            return v;
        }
        let mut out = Out::new();
        self.run_text_commands(Ptr::Rom(label), &mut out, 0, &mut choices.iter());
        SEEN.with(|m| m.borrow_mut().insert(label, out.trainer_name));
        out.trainer_name
    }

    /// Render the text at ROM `label` to its messages, applying what the
    /// text engine writes to memory (only glitches write: see [`Out::writes`]).
    pub fn render_text(&mut self, label: usize, choices: &[usize]) -> Vec<String> {
        let mut out = Out::new();
        let mut choices = choices.iter();
        self.run_text_commands(Ptr::Rom(label), &mut out, 0, &mut choices);
        out.end_message();
        for &(addr, value) in &out.writes {
            self.mem.wb(addr, value);
        }
        out.messages
    }

    /// `DoTextUntilTerminator`.
    fn run_text_commands(&self, start: Ptr, out: &mut Out, depth: u8, choices: &mut std::slice::Iter<'_, usize>) {
        assert!(depth < 16, "text nesting too deep");
        let mut p = start;
        let next = |p: Ptr, n: usize| p.add(n as u16);
        let bank = |p: Ptr| match p {
            Ptr::Rom(o) => rom::bank_of(o),
            Ptr::Ram(_) => 0,
            Ptr::Cpu { bank, .. } => bank,
        };
        let word = |e: &Self, out: &Out, p: Ptr| u16::from(e.text_byte(out, p)) | u16::from(e.text_byte(out, next(p, 1))) << 8;
        loop {
            let cmd = self.text_byte(out, p);
            p = next(p, 1);
            match cmd {
                0x50 => return, // TX_END
                0x00 => {
                    // TX_START
                    out.pos = out.bc;
                    match self.place_string(p, out) {
                        Some(n) => p = n,
                        None => return,
                    }
                }
                0x01 => {
                    // TX_RAM: a <DONE> in the string only returns from PlaceString.
                    let addr = word(self, out, p);
                    p = next(p, 2);
                    out.pos = out.bc;
                    self.place_string(Ptr::Ram(addr), out);
                }
                0x02 => {
                    // TX_BCD
                    self.unported("text_bcd");
                }
                0x03 => {
                    // TX_MOVE
                    out.bc = word(self, out, p);
                    p = next(p, 2);
                }
                0x04 => {
                    // TX_BOX: draws a box, no characters.
                    p = next(p, 4);
                    out.lost = true;
                }
                0x05 | 0x07 => {
                    // TX_LOW, TX_SCROLL
                    out.newline();
                    out.bc = TEXT_LINE2;
                }
                0x06 | 0x0d => out.end_message(), // TX_PROMPT_BUTTON, TX_WAIT_BUTTON
                0x08 => {
                    // TX_START_ASM: the code picks the next text.
                    let here = match p {
                        Ptr::Rom(o) => o,
                        _ => panic!("text_asm outside ROM text"),
                    };
                    out.lost = true;
                    match self.text_asm(here, choices) {
                        Some(label) => return self.run_text_commands(Ptr::Rom(label), out, depth + 1, choices),
                        None => return,
                    }
                }
                0x09 => {
                    // TX_DECIMAL: address, then (bytes << 4) | digits.
                    let addr = word(self, out, p);
                    let param = self.text_byte(out, next(p, 2));
                    p = next(p, 3);
                    let bytes = param >> 4;
                    let digits = usize::from(param & 0xf);
                    let mut value: u32 = 0;
                    for i in 0..u16::from(bytes) {
                        value = value << 8 | u32::from(self.mem.rb(addr + i));
                    }
                    out.pos = out.bc;
                    for c in print_num_left_aligned(value, digits as u8) {
                        self.place_tile(out, c);
                    }
                    out.bc = out.pos;
                }
                0x0a | 0x0b | 0x0e..=0x13 => {} // pause and sounds
                0x0c => {
                    // TX_DOTS: n ellipses.
                    let n = self.text_byte(out, p);
                    p = next(p, 1);
                    out.pos = out.bc;
                    for _ in 0..n {
                        self.place_tile(out, 0x75);
                    }
                    out.bc = out.pos;
                }
                0x14 => {
                    // TX_STRINGBUFFER
                    self.unported("text_buffer");
                }
                0x15 => self.unported("text_today"),
                0x16 => {
                    // TX_FAR: address, bank.
                    let addr = word(self, out, p);
                    let b = self.text_byte(out, next(p, 2));
                    p = next(p, 3);
                    self.run_text_commands(Ptr::Rom(rom::flat(b, addr)), out, depth + 1, choices);
                }
                _ => panic!("unknown text command {cmd:#04x} at bank {:02x}", bank(p)),
            }
        }
    }

    /// `PlaceString` / `PlaceNextChar` at the cursor (`out.pos`): characters
    /// up to `@`, which leaves the end of the string in `bc`. Returns the
    /// pointer past the terminator, or `None` when `<DONE>` / `<PROMPT>`
    /// returned early (a `text_start` string then ends the text; a `text_ram`
    /// one doesn't, `DoneText` only returns from `PlaceString`).
    fn place_string(&self, p: Ptr, out: &mut Out) -> Option<Ptr> {
        assert!(out.depth < 64, "strings placed inside each other too deep: the original overflows its stack");
        out.depth += 1;
        let end = self.place_string_inner(p, out);
        out.depth -= 1;
        end
    }

    fn place_string_inner(&self, mut p: Ptr, out: &mut Out) -> Option<Ptr> {
        // `PlaceString` pushes the cursor it starts at: `<LINE>`, `<NEXT>`,
        // `<LF>` and `<CR>` move this saved line start (and the cursor to it).
        let mut line = out.pos;
        loop {
            if let Some(a) = p.ram() {
                // Only garbage reads on into memory the engine doesn't keep.
                assert!(
                    super::mem::glitch_read_kept(a),
                    "text read from {a:#06x}: state the engine doesn't keep"
                );
            }
            let c = self.text_byte(out, p);
            p = p.add(1);
            match c {
                0x50 => {
                    // '@'
                    out.bc = out.pos;
                    return Some(p);
                }
                0x4f => {
                    out.newline(); // <LINE>
                    line = TEXT_LINE2;
                    out.pos = line;
                }
                0x4e => {
                    out.newline(); // <NEXT>
                    line = line.wrapping_add(2 * 20);
                    out.pos = line;
                    out.bc = 2 * 20; // `ld bc, SCREEN_WIDTH * 2`
                }
                0x22 => {
                    out.newline(); // <LF>
                    line = line.wrapping_add(20);
                    out.pos = line;
                    out.bc = 20; // `ld bc, SCREEN_WIDTH`
                }
                0x4b | 0x4c => {
                    out.newline(); // <_CONT> <SCROLL>
                    out.pos = TEXT_LINE2;
                    out.bc = 0; // TextScroll: ByteFill, DelayFrames
                }
                0x55 => {
                    // <CONT>: places "<_CONT>@".
                    out.newline();
                    out.pos = TEXT_LINE2;
                    out.bc = TEXT_LINE2;
                }
                0x51 => {
                    out.end_message(); // <PARA>
                    out.pos = TEXT_LINE1;
                    out.bc = 0; // ClearBox / DelayFrames
                }
                0x57 | 0x58 => {
                    // <DONE> / <PROMPT>; a link battle's PromptButton waits
                    // with DelayFrames (c = 0).
                    if c == 0x58 && self.link_mode() != 0 {
                        out.bc &= 0xff00;
                    }
                    out.end_message();
                    return None;
                }
                0x52 => self.place_nested(Ptr::Ram(W_PLAYER_NAME), out),
                // <PLAY_G>: the player's name (its suffix is "@" in English).
                0x14 => self.place_nested(Ptr::Ram(W_PLAYER_NAME), out),
                0x53 => self.place_nested(Ptr::Ram(W_RIVAL_NAME), out),
                0x38 => self.place_nested(Ptr::Ram(W_REDS_NAME), out),
                0x39 => self.place_nested(Ptr::Ram(W_GREENS_NAME), out),
                0x49 => self.place_nested(Ptr::Ram(W_MOMS_NAME), out),
                0x54 => self.place_nested(Ptr::Rom(r::PLACE_PO_KE_TEXT), out),
                0x56 => self.place_nested(Ptr::Rom(r::SIX_DOTS_CHAR_TEXT), out),
                0x5b => self.place_nested(Ptr::Rom(r::PC_CHAR_TEXT), out),
                0x5c => self.place_nested(Ptr::Rom(r::TM_CHAR_TEXT), out),
                0x5d => self.place_nested(Ptr::Rom(r::TRAINER_CHAR_TEXT), out),
                0x5e => self.place_nested(Ptr::Rom(r::ROCKET_CHAR_TEXT), out),
                0x4a => self.place_nested(Ptr::Rom(r::PLACE_PKMN_TEXT), out),
                0x24 => self.place_nested(Ptr::Rom(r::PLACE_POKE_TEXT), out),
                0x23 => self.place_nested(Ptr::Rom(r::KOUGEKI_TEXT), out), // <KOUGEKI>
                // <ROUTE> / <WATASHI> / <KOKO_WA>: dummied-out "@" strings.
                0x35..=0x37 => out.bc = out.pos,
                0x1f => self.place_tile(out, 0x7f), // <BSP>: ' '
                0x25 => {}                          // <WBR>
                0x59 | 0x5a => {
                    // <TARGET> / <USER>
                    let turn = self.turn() ^ u8::from(c == 0x59);
                    if turn == 0 {
                        self.place_nested(Ptr::Ram(W_BATTLE_MON_NICKNAME), out);
                    } else {
                        self.place_nested(Ptr::Rom(r::ENEMY_TEXT), out);
                        self.place_nested(Ptr::Ram(W_ENEMY_MON_NICKNAME), out);
                    }
                }
                0x3f => {
                    // <ENEMY>
                    if self.link_mode() != 0 {
                        self.place_nested(Ptr::Ram(W_OT_CLASS_NAME), out);
                    } else {
                        let class = self.mem.rb(W_TRAINER_CLASS);
                        if class == crate::defs::RIVAL1 || class == crate::defs::RIVAL2 {
                            self.place_nested(Ptr::Ram(W_RIVAL_NAME), out);
                        } else {
                            out.trainer_name = true;
                            self.place_nested(Ptr::Ram(W_OT_CLASS_NAME), out);
                            self.place_nested(Ptr::Rom(r::STRING_SPACE), out);
                            // Battle_GetTrainerName, as the name is placed (it
                            // can overwrite the very string being printed).
                            for (i, v) in self.trainer_name_bytes().into_iter().enumerate() {
                                out.writes.push((W_STRING_BUFFER1 + i as u16, v));
                            }
                            self.place_nested(Ptr::Ram(W_STRING_BUFFER1), out);
                        }
                    }
                }
                // Only reachable through garbage (Beat Up's names past the party).
                0x5f => {
                    // <DEXEND>: '.' (not captured), then returns from
                    // PlaceString without setting bc.
                    self.store(out, out.pos, 0xe8);
                    return Some(p);
                }
                0x00 => {
                    // <NULL>: '?' with a plain `ld [hli], a` (not captured).
                    self.store(out, out.pos, 0xe6);
                    out.pos = out.pos.wrapping_add(1);
                }
                0x16 => {
                    // <CR>: the start of the next row, plus the column in the
                    // byte after it.
                    let mut hl = line.wrapping_sub(W_TILEMAP);
                    let mut rows: u8 = 1;
                    while hl >= 20 {
                        hl = hl.wrapping_sub(20);
                        rows = rows.wrapping_add(1);
                    }
                    let col = self.text_byte(out, p);
                    p = p.add(1);
                    out.bc = u16::from(col);
                    line = W_TILEMAP.wrapping_add(20 * u16::from(rows)).wrapping_add(u16::from(col));
                    out.pos = line;
                }
                0x15 => p = self.mobile_script_char(p, out),
                _ => {
                    // The kana with a (han)dakuten load the mark into `b`.
                    match c {
                        0x01..=0x3f => out.bc = 0xe500 | (out.bc & 0xff),
                        0x40..=0x5f => out.bc = 0xe400 | (out.bc & 0xff),
                        _ => {}
                    }
                    self.place_tile(out, placed_tile(c));
                }
            }
        }
    }

    // --- <MOBILE> -----------------------------------------------------------------
    //
    // `MobileScriptChar` runs `RunMobileScript` (bank 5f): the bytes after the
    // character are a script for the mobile adapter's text. Ported as far as
    // it stays in the memory the engine models (WRAM banks 0-1, HRAM, ROM);
    // what reads SRAM, switches the WRAM bank or jumps to garbage panics.

    /// `MobileScriptChar`: run the script at `de` (just past the character)
    /// with the cursor in `bc`. Returns where `PlaceNextChar` carries on.
    fn mobile_script_char(&self, script: Ptr, out: &mut Out) -> Ptr {
        assert!(!out.lost, "<MOBILE> after a text command whose effect on the cursor isn't modelled");
        assert!(out.mobile_depth < 32, "<MOBILE> scripts nested too deep: the original overflows its stack");
        out.mobile_depth += 1;
        let mut m = Mobile { bc: out.pos, de: script, hl: Ptr::Rom(r::RUN_MOBILE_SCRIPT).cpu_addr() };
        // `RunMobileScript` opens SRAM bank 6 around the script (no effect on
        // the battle). `farcall` left `hl` = RunMobileScript.
        loop {
            // _RunMobileScript
            let a = self.text_byte(out, m.de);
            m.de = m.de.add(1);
            if a == 0x50 || a >= 0x10 {
                break;
            }
            let s = m.de; // the handlers `pop hl` this
            let carry = match a {
                0 => panic!("<MOBILE> script command 0: the original jumps through _RunMobileScript.Jumptable[-1] into garbage"),
                1 => self.mobile_skip(out, &mut m, s, 4, 7), // Function17f081
                2 => self.mobile_skip(out, &mut m, s, 3, 4), // Function17f0f8
                3 => {
                    // Function17f154
                    self.mobile_in_skip_mode(out);
                    m.hl = s.add(2).cpu_addr();
                    m.de = s.add(2);
                    self.text_byte(out, m.de) == 0x50
                }
                4..=7 => self.mobile_skip(out, &mut m, s, 2, 3), // Function17f181 .. Function17f27b
                8 => self.mobile_print_index(out, &mut m, s),     // Function17f2cb
                9 => panic!("<MOBILE> script command 9 (MobileScript_PlayerName) leaves WRAM bank 4 mapped, which the engine doesn't model"),
                10 | 11 => panic!("<MOBILE> script command {a} reads the mobile data in SRAM, which the engine doesn't model"),
                12 => panic!("<MOBILE> script command 12 (EZ Chat words) reads SRAM, which the engine doesn't model"),
                13 => self.mobile_print_table_string(out, &mut m, s), // Function17f3f0
                14 => {
                    // Function17f41d: the start of the next row, plus a column.
                    let col = self.text_byte(out, s);
                    let mut hl = m.bc.wrapping_sub(W_TILEMAP);
                    let mut rows: u8 = 1;
                    while hl >= 20 {
                        hl = hl.wrapping_sub(20);
                        rows = rows.wrapping_add(1);
                    }
                    m.bc = 0x3b00 | u16::from(rows); // `ld bc, -wTilemap`, then `ld c, 1`
                    m.hl = W_TILEMAP.wrapping_add(20 * u16::from(rows)).wrapping_add(u16::from(col));
                    m.de = s.add(1);
                    false
                }
                _ => self.mobile_skip(out, &mut m, s, 5, 8), // 15: Function17f44f
            };
            if carry {
                break;
            }
        }
        out.pos = m.hl;
        out.bc = m.bc;
        out.mobile_depth -= 1;
        m.de
    }

    /// `Function17f524`: carry when the mobile text state (`wcd2e`, `wcd42`,
    /// `wcd68`) says to print the command's value. Battles leave those bytes
    /// zero, so the commands only skip; their printing paths read SRAM and
    /// mobile state the engine doesn't model.
    fn mobile_in_skip_mode(&self, out: &Out) {
        let a = self.peek(out, 0xcd2e);
        let carry = a == self.peek(out, 0xcd42).wrapping_sub(1) || a < self.peek(out, 0xcd68);
        assert!(!carry, "<MOBILE> script with mobile text state set (wcd2e/wcd42/wcd68): not modelled");
    }

    /// The skip path the value commands share: the cursor moves by the count
    /// at `s + count_at`, the script goes on at `s + next_at` (and stops
    /// there on a '@'). Leaves `bc` = the count.
    fn mobile_skip(&self, out: &mut Out, m: &mut Mobile, s: Ptr, count_at: u16, next_at: u16) -> bool {
        self.mobile_in_skip_mode(out);
        let count = self.text_byte(out, s.add(count_at));
        m.de = s.add(next_at);
        m.hl = m.bc.wrapping_add(u16::from(count));
        m.bc = u16::from(count);
        self.text_byte(out, m.de) == 0x50
    }

    /// `Function17f50f`: with `a` = 0 the cursor goes to the end of what was
    /// printed (`wcd52`), else `a` tiles on from `hl`.
    fn mobile_advance(&self, out: &Out, m: &mut Mobile, hl: u16, a: u8) {
        if a != 0 {
            m.hl = hl.wrapping_add(u16::from(a));
            m.bc = m.hl;
        } else {
            let end = u16::from(self.peek(out, 0xcd52)) | u16::from(self.peek(out, 0xcd53)) << 8;
            m.hl = end;
            m.bc = end;
        }
    }

    /// `Function17f2cb`: print `wcd2e + 1` with the digit count in the script.
    fn mobile_print_index(&self, out: &mut Out, m: &mut Mobile, s: Ptr) -> bool {
        let digits = self.text_byte(out, s);
        let advance = self.text_byte(out, s.add(1));
        out.writes.push((0xcd54, digits));
        out.writes.push((0xcd55, advance));
        let value = self.peek(out, 0xcd2e).wrapping_add(1);
        out.writes.push((0xcd56, value));
        let start = m.bc;
        let end = self.mobile_print_num(out, &[value], 1, digits, start);
        out.writes.push((0xcd52, end as u8));
        out.writes.push((0xcd53, (end >> 8) as u8));
        self.mobile_advance(out, m, start, advance);
        m.de = s.add(2);
        false
    }

    /// `Function17f3f0`: a string from a table the script points at, indexed
    /// by a byte in memory; the script ends after it.
    fn mobile_print_table_string(&self, out: &mut Out, m: &mut Mobile, s: Ptr) -> bool {
        let skip = self.text_byte(out, s);
        let ptr = u16::from(self.text_byte(out, s.add(1))) | u16::from(self.text_byte(out, s.add(2))) << 8;
        let index = self.mobile_read(out, ptr);
        let entry = s.add(3).add(2 * u16::from(index));
        let offset = u16::from(self.text_byte(out, entry)) | u16::from(self.text_byte(out, entry.add(1))) << 8;
        // wBGPals1 + offset, read with WRAM bank 1 mapped.
        let string = self.mobile_ptr(out, 0xd000u16.wrapping_add(offset));
        out.pos = m.bc;
        out.bc = u16::from(index); // `ld c, a` / `ld b, 0`
        self.place_string(string, out);
        m.de = s.add(2 * u16::from(skip) + 3);
        m.hl = out.bc;
        m.bc = m.hl;
        true
    }

    /// A byte the script reads through a pointer of its own.
    fn mobile_read(&self, out: &Out, addr: u16) -> u8 {
        if (0x4000..=0x7fff).contains(&addr) {
            return rom::byte(rom::flat(0x5f, addr));
        }
        assert!(
            super::mem::glitch_read_kept(addr),
            "<MOBILE> script reads {addr:#06x}: state the engine doesn't keep"
        );
        self.peek(out, addr)
    }

    /// A string the script points at, as a [`Ptr`] (bank 5f is mapped while
    /// it runs).
    fn mobile_ptr(&self, _out: &Out, addr: u16) -> Ptr {
        // (`place_string` checks each RAM byte it reads is kept.)
        Ptr::Cpu { addr, bank: 0x5f }
    }

    /// `_MobilePrintNum` (bank 41) at `hl`: `bytes` (low nibble of `b`; this
    /// caller has no flags) and `digits` (`c`, any value: the original's digit
    /// loop then reads on past its table of powers of ten). Writes the digits
    /// without capturing them; returns the cursor after them.
    fn mobile_print_num(&self, out: &mut Out, number: &[u8], b: u8, digits: u8, mut hl: u16) -> u16 {
        const POW_9: usize = 0x41 * 0x4000 + (0x6292 - 0x4000); // _MobilePrintNum._9
        const POW_2: usize = 0x41 * 0x4000 + (0x62ae - 0x4000); // _MobilePrintNum._2
        let mut buf = [0u8; 10];
        for (i, v) in buf.iter_mut().enumerate() {
            *v = self.peek(out, H_PRINT_NUM_BUFFER + i as u16);
        }
        buf[0] = 0;
        buf[1] = 0;
        buf[2] = 0;
        match b & 0xf {
            1 => buf[3] = number[0],
            2 => buf[2..4].copy_from_slice(&number[..2]),
            3 => buf[1..4].copy_from_slice(&number[..3]),
            _ => buf[0..4].copy_from_slice(&number[..4]),
        }
        buf[8] = 0;
        buf[9] = b;
        let advance = |buf: &[u8; 10], hl: &mut u16| {
            // .Function1062ff
            let f = buf[9];
            if f & 0x80 != 0 || f & 0x40 == 0 || buf[8] != 0 {
                *hl = hl.wrapping_add(1);
            }
        };
        if digits != 2 {
            // de = ._N for N = digits - 1 (3..=9 digits), else ._9; at its low byte.
            let mut de = if (3..=9).contains(&digits) { POW_2 - 4 * usize::from(digits - 3) } else { POW_9 } + 3;
            let mut n = digits.wrapping_sub(2);
            loop {
                // .Function1062b2: how many times the power fits.
                let value = u32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]);
                let power = u32::from_be_bytes([rom::byte(de - 3), rom::byte(de - 2), rom::byte(de - 1), rom::byte(de)]);
                assert!(power != 0, "_MobilePrintNum divides by a zero 'power of ten' read past its table: the original loops forever");
                let (q, rem) = (value / power, value % power);
                let last = rem.wrapping_sub(power); // the failed subtraction
                buf[4..8].copy_from_slice(&last.to_be_bytes());
                buf[0..4].copy_from_slice(&rem.to_be_bytes());
                let c = q as u8;
                if buf[8] | c == 0 {
                    // .LoadMinusTenIfNegative
                    if buf[9] & 0x80 != 0 {
                        self.store(out, hl, 0xf6);
                    }
                } else {
                    let digit = 0xf6u8.wrapping_add(c);
                    self.store(out, hl, digit);
                    buf[8] = digit;
                }
                advance(&buf, &mut hl);
                de += 4;
                n = n.wrapping_sub(1);
                if n == 0 {
                    break;
                }
            }
        }
        // .two_digits
        let (tens, units) = (buf[3] / 10, buf[3] % 10);
        buf[8] |= tens;
        if buf[8] != 0 {
            self.store(out, hl, 0xf6u8.wrapping_add(tens));
        } else if buf[9] & 0x80 != 0 {
            self.store(out, hl, 0xf6);
        }
        advance(&buf, &mut hl);
        self.store(out, hl, 0xf6u8.wrapping_add(units));
        hl = hl.wrapping_add(1);
        for (i, &v) in buf.iter().enumerate() {
            out.writes.push((H_PRINT_NUM_BUFFER + i as u16, v));
        }
        hl
    }

    /// The `text_asm` block at ROM `addr` (just after its `text_asm` byte):
    /// returns the text it hands back in `hl`, or `None` to stop. Blocks
    /// whose choice the battle code already computed take it from `choices`.
    fn text_asm(&self, addr: usize, choices: &mut std::slice::Iter<'_, usize>) -> Option<usize> {
        let name = super::rsym_name(addr);
        let base = name.split('+').next().unwrap_or("");
        let grammar = self.mem.rb(W_MOVE_GRAMMAR);
        let disobeyed = self.mem.rb(W_ALREADY_DISOBEYED) != 0;
        Some(match base {
            "UsedMoveText" => {
                if disobeyed || grammar < 3 { r::USED_MOVE2_TEXT } else { r::USED_MOVE1_TEXT }
            }
            // UsedMove2Text's text_asm falls straight into this label.
            "UsedMove1Text" | "UsedMove2Text" | "UsedMoveText_CheckObedience" => {
                if disobeyed { r::USED_MOVE_TEXT_CHECK_OBEDIENCE_USED_INSTEAD_TEXT } else { r::MOVE_NAME_TEXT }
            }
            "UsedMoveText_CheckObedience.UsedInsteadText" | "UsedMoveText_CheckObedience.GetMoveNameText" => r::MOVE_NAME_TEXT,
            "MoveNameText" => [
                r::END_USED_MOVE1_TEXT,
                r::END_USED_MOVE2_TEXT,
                r::END_USED_MOVE3_TEXT,
                r::END_USED_MOVE4_TEXT,
                r::END_USED_MOVE5_TEXT,
            ][usize::from(grammar)],
            "GoMonText" | "DoItMonText" | "GoForItMonText" | "YourFoesWeakGetmMonText" | "PrepareBattleMonNicknameText" => r::BATTLE_MON_NICKNAME_TEXT,
            "WithdrawMonText.WithdrawMonText" => *choices.next().expect("WithdrawMonText needs its choice"),
            "Text_MonGainedExpPoint" => {
                if self.mem.rb(W_STRING_BUFFER2 + 2) == 0 { r::EXP_POINTS_TEXT } else { r::BOOSTED_EXP_POINTS_TEXT }
            }
            "BattleCommand_StatUpMessage" | "BattleCommand_StatUpMessage.stat" => {
                if self.mem.rb(W_LOWERED_STAT) & 0xf0 == 0 {
                    r::BATTLE_COMMAND_STAT_UP_MESSAGE_BATTLE_STAT_WENT_UP_TEXT
                } else {
                    r::BATTLE_COMMAND_STAT_UP_MESSAGE_BATTLE_STAT_WENT_WAY_UP_TEXT
                }
            }
            "BattleCommand_StatDownMessage" | "BattleCommand_StatDownMessage.stat" => {
                if self.mem.rb(W_LOWERED_STAT) & 0xf0 == 0 {
                    r::BATTLE_COMMAND_STAT_DOWN_MESSAGE_BATTLE_STAT_FELL_TEXT
                } else {
                    r::BATTLE_COMMAND_STAT_DOWN_MESSAGE_BATTLE_STAT_SHARPLY_FELL_TEXT
                }
            }
            "BattleCommand_Charge.UsedText" => {
                use crate::defs::*;
                match self.battle_var(BATTLE_VARS_MOVE_ANIM) {
                    RAZOR_WIND => r::BATTLE_COMMAND_CHARGE_BATTLE_MADE_WHIRLWIND_TEXT,
                    SOLARBEAM => r::BATTLE_COMMAND_CHARGE_BATTLE_TOOK_SUNLIGHT_TEXT,
                    SKULL_BASH => r::BATTLE_COMMAND_CHARGE_BATTLE_LOWERED_HEAD_TEXT,
                    SKY_ATTACK => r::BATTLE_COMMAND_CHARGE_BATTLE_GLOWING_TEXT,
                    FLY => r::BATTLE_COMMAND_CHARGE_BATTLE_FLEW_TEXT,
                    // DIG — and, as in the original, `hl` stays DugText for anything else.
                    _ => r::BATTLE_COMMAND_CHARGE_BATTLE_DUG_TEXT,
                }
            }
            "Text_GotchaMonWasCaught" => r::WAIT_BUTTON_TEXT,
            "Text_1_2_and_Poof" => r::TEXT_1_2_AND_POOF_MOVE_FORGOT_TEXT,
            _ => panic!("unported text_asm at {addr:#08x} ({name})"),
        })
    }
}

/// `_PrintNum` as `TextCommand_DECIMAL` calls it (left-aligned, no leading
/// zeros, no money sign): the characters it writes for `value`. Faithful to
/// the original's quirks: a digit count outside 2..=6 prints seven places,
/// a place's digit can exceed 9 when `value` doesn't fit the places (it is
/// then the character after `'9'`), and the final two places only look at
/// the low byte of what is left.
fn print_num_left_aligned(value: u32, digits: u8) -> Vec<u8> {
    const ZERO: u8 = 0xf6; // '0'
    let places: &[u32] = match digits {
        2 => &[],
        3 => &[100],
        4 => &[1000, 100],
        5 => &[10000, 1000, 100],
        6 => &[100_000, 10000, 1000, 100],
        _ => &[1_000_000, 100_000, 10000, 1000, 100],
    };
    let mut v = value & 0xff_ffff;
    let mut out = Vec::new();
    let mut started = false;
    for &place in places {
        let c = (v / place) as u8;
        v %= place;
        if started || c != 0 {
            started = true;
            out.push(ZERO.wrapping_add(c));
        }
    }
    // .two: tens and units from the low byte.
    let low = (v & 0xff) as u8;
    let (tens, units) = (low / 10, low % 10);
    if started || tens != 0 {
        out.push(ZERO.wrapping_add(tens));
    }
    out.push(ZERO + units);
    out
}

/// The tile `PlaceString` (`CheckDict.not_diacritic`) places for a character
/// that isn't a control character: below `FIRST_REGULAR_TEXT_CHAR` ($60) the
/// Japanese kana with a (han)dakuten become the plain kana (the mark goes on
/// the line above, which the capture doesn't record).
fn placed_tile(c: u8) -> u8 {
    match c {
        0x60.. => c,
        0x44.. => c.wrapping_add(0xca_u8.wrapping_sub(0x44)), // 'は' - 'ぱ'
        0x40.. => c.wrapping_add(0x99_u8.wrapping_sub(0x40)), // 'ハ' - 'パ'
        0x20.. => c.wrapping_add(0xb6_u8.wrapping_sub(0x26)), // 'か' - 'が'
        _ => c.wrapping_add(0x85_u8.wrapping_sub(0x05)),      // 'カ' - 'ガ'
    }
}
