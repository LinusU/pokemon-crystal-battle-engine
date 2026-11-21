//! The engine's memory: the original game's WRAM (banks 0 and 1) and HRAM,
//! byte for byte.
//!
//! The battle engine keeps all of its state in these bytes, and the original
//! code depends on their layout (block copies in Transform and Baton Pass,
//! overlapping structs, known glitches). Keeping the same layout makes the
//! port exact by construction and lets the differential tester compare the
//! engine's memory with the original's directly. Named accessors live next to
//! the code that uses them; addresses come from [`super::sym`].

/// Read access to Game Boy memory. Implemented by the engine's [`Mem`] and by
/// the emulated original's CPU, so code that only inspects battle state
/// (legal actions, state snapshots) runs unchanged on either.
pub trait ReadMem {
    fn read_byte(&self, addr: u16) -> u8;

    /// The game whose memory this is: Pokémon Crystal unless the reader
    /// knows (its data and rules).
    fn data(&self) -> &'static crate::data::GameData {
        crate::data::GameData::crystal()
    }

    /// A move's base PP, from the game's data (`Moves`).
    fn base_pp(&self, move_id: u8) -> u8 {
        self.data().mv(move_id).pp
    }
}

/// Memory read alongside the game's data, so that what depends on both (a
/// move's max PP) reads the game's.
pub struct WithData<'a> {
    pub mem: &'a Mem,
    pub data: &'static crate::data::GameData,
}

impl ReadMem for WithData<'_> {
    fn read_byte(&self, addr: u16) -> u8 {
        self.mem.read_byte(addr)
    }

    fn data(&self) -> &'static crate::data::GameData {
        self.data
    }
}

const WRAM_START: u16 = 0xc000;
const WRAM_LEN: usize = 0x2000;
const HRAM_START: u16 = 0xff80;
const HRAM_LEN: usize = 0x7f;

/// WRAM the engine has but doesn't keep the way the original does: the live
/// part of the CPU stack, sound, graphics buffers and menu/UI scratch. They
/// hold whatever the start state left there. The bottom of the stack
/// (c000-c07f) is kept: the original never pushes that deep in a battle
/// (its deepest is around c091), so both sides keep it zero.
pub const NOT_KEPT: &[(u16, u16)] = &[
    (0xc080, 0xc2c5), // live stack, wMusic* / wChannel* / wCry* / wSFX* (audio), wLZ*
    (0xc4a0, 0xc608), // wTilemap (shadow OAM stays zero)
    (0xc741, 0xc7d4), // after wBattleEnd: code the transition copies into WRAM
    (0xcd98, 0xcf50), // wBGMapBuffer, wAttrmap
    (0xcf63, 0xcf6c), // wBattleTower*, wBattleTransition*, wRequested2bpp*
    (0xcf71, 0xcf73), // wMenuMetadata
    (0xcf81, 0xcf92), // menu cursor / border / data
    (0xcfb2, 0xcfb3), // wTextDelayFrames
    (0xcfb4, 0xcfbb), // wPredef*, wFarCallBC
    (0xcfc4, 0xcfca), // wPlaceBalls*, wBGP, wOBP*
    (0xd003, 0xd008), // wBillsPCTempListIndex.. (scratch)
    (0xd0ed, 0xd0ee), // wStateFlags
    (0xd153, 0xd154), // wBGMapAnchor
    (0xd1ea, 0xd1f0), // wBattleHUDTiles
];

/// WRAM the original keeps names in, as text: the name buffers, nicknames,
/// OT names, the player's and the rival's names. The engine keeps names as
/// strings instead
/// (`engine::names`), so these bytes stay zero: the differential testers
/// don't compare them, and a glitch read of them (Beat Up past the party)
/// reads zeros on both engines.
pub const NAMES: &[(u16, u16)] = {
    use super::sym::*;
    &[
    (W_ENEMY_MON_NICKNAME, W_ENEMY_MON_NICKNAME + 11),
    (W_BATTLE_MON_NICKNAME, W_BATTLE_MON_NICKNAME + 11),
    (W_OT_CLASS_NAME, W_OT_CLASS_NAME + 13),
    (W_MON_OR_ITEM_NAME_BUFFER, W_MON_OR_ITEM_NAME_BUFFER + 13),
    (W_STRING_BUFFER1, W_STRING_BUFFER5 + 19),
    (W_OT_PLAYER_NAME, W_OT_PLAYER_NAME + 11),
    (W_OT_PARTY_MON_O_TS, W_OT_PARTY_DATA_END),
    (W_PLAYER_NAME, W_PLAYER_NAME + 11),
    (W_RIVAL_NAME, W_RIVAL_NAME + 11),
    (W_PARTY_MON_O_TS, W_PARTY_MON_NICKNAMES_END),
    ]
};

/// Whether `addr` is in [`NAMES`].
pub fn is_name(addr: u16) -> bool {
    NAMES.iter().any(|&(a, b)| (a..b).contains(&addr))
}

/// Whether `addr` is in [`NOT_KEPT`].
pub fn is_not_kept(addr: u16) -> bool {
    NOT_KEPT.iter().any(|&(a, b)| (a..b).contains(&addr))
}

/// WRAM that is the same on both engines at the start but drifts during the
/// battle: what the original's menus, sprite and battle animations write, and
/// the scratch they leave behind (base data, temp mon). Found by comparing
/// every byte at every decision of random battles (all the others stay
/// equal).
pub const DRIFTS: &[(u16, u16)] = &[
    (0xc300, 0xc4a0), // wSpriteAnim*, wCurIcon / wCurAnim*, wShadowOAM
    (0xcf5f, 0xcfcb), // wMonType, wCurSpecies, window stack, 2D menu, wFXAnimID, wBattleAfterAnim
    (0xd108, 0xd10b), // wCurPartySpecies, wWhichHPBar
    (0xd123, 0xd13e), // wTempMon
    (0xd1f0, 0xd1f2), // wCurHPAnimPal, wCurEnemyItem
    (0xd234, 0xd266), // wUnownLetter, wBaseData, wApplyStatLevelMultipliersToEnemy
];

/// Whether a glitch read of `addr` reads the same byte on both engines: ROM
/// bank 0 and WRAM (direct or through echo RAM) that is neither
/// [`NOT_KEPT`], [`NAMES`] nor [`DRIFTS`]. Not: the live stack, sound, graphics
/// buffers, UI scratch, VRAM, OAM, I/O, HRAM.
pub fn glitch_read_kept(addr: u16) -> bool {
    let kept = |a: u16| !is_not_kept(a) && !is_name(a) && !DRIFTS.iter().any(|&(lo, hi)| (lo..hi).contains(&a));
    match addr {
        0x0000..=0x3fff => true,
        0xc000..=0xdfff => kept(addr),
        0xe000..=0xfdff => kept(addr - 0x2000),
        _ => false,
    }
}

/// Whether all of `addr..addr + len` is [`glitch_read_kept`].
pub fn glitch_range_kept(addr: u16, len: u16) -> bool {
    (0..len).all(|i| glitch_read_kept(addr.wrapping_add(i)))
}

/// Whether all of `addr..addr + len` can be read as a name: [`glitch_read_kept`]
/// memory or a name (Beat Up reads a party nickname, possibly past the
/// party).
pub fn glitch_name_range_kept(addr: u16, len: u16) -> bool {
    (0..len).all(|i| {
        let a = addr.wrapping_add(i);
        glitch_read_kept(a) || is_name(a)
    })
}

/// Where Beat Up's glitch reads of state neither engine keeps are sent: the
/// bottom of the stack, which stays zero (see [`NOT_KEPT`]). Those bytes are
/// hardware state the transpiled original doesn't reproduce either (its stack
/// holds what the transpiled code pushed, its I/O registers the emulator's
/// timing), so both engines read zeros there instead. See docs/engine.md.
pub const GLITCH_ZERO: u16 = 0xc000;

#[derive(Clone, PartialEq, Eq)]
pub struct Mem {
    wram: Box<[u8; WRAM_LEN]>,
    hram: [u8; HRAM_LEN],
}

impl std::fmt::Debug for Mem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Mem { .. }")
    }
}

impl Default for Mem {
    fn default() -> Self {
        Mem { wram: Box::new([0; WRAM_LEN]), hram: [0; HRAM_LEN] }
    }
}

impl Mem {
    /// Copy WRAM banks 0–1 and HRAM out of any memory (the emulated Cpu at a
    /// decision point, normally). WRAM bank 1 must be the one mapped.
    pub fn copy_from(src: &impl ReadMem) -> Self {
        let mut mem = Mem::default();
        for i in 0..WRAM_LEN {
            mem.wram[i] = src.read_byte(WRAM_START + i as u16);
        }
        for i in 0..HRAM_LEN {
            mem.hram[i] = src.read_byte(HRAM_START + i as u16);
        }
        mem
    }

    #[inline]
    pub fn rb(&self, addr: u16) -> u8 {
        match addr {
            0xc000..=0xdfff => self.wram[usize::from(addr - WRAM_START)],
            0xff80..=0xfffe => self.hram[usize::from(addr - HRAM_START)],
            _ => self.rb_glitch(addr),
        }
    }

    /// Reads only glitches reach: Beat Up's party index running past the
    /// party (`GetBeatupMonLocation` with a stale loop counter) walks through
    /// echo RAM and wraps around to ROM bank 0, the game's code, which the
    /// library doesn't have: that glitch is unsupported.
    #[cold]
    fn rb_glitch(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x3fff => panic!("unsupported glitch: a read of the game's code (ROM bank 0, {addr:#06x})"),
            // Echo RAM mirrors c000-ddff (WRAM bank 1 is the one mapped in
            // battle). Exact where the engine keeps that WRAM the way the
            // original does.
            0xe000..=0xfdff => {
                let mirror = addr - 0x2000;
                if is_not_kept(mirror) {
                    panic!(
                        "engine read of {addr:#06x}, the echo of {mirror:#06x}: stack, sound or graphics state the engine doesn't keep"
                    );
                }
                self.wram[usize::from(mirror - WRAM_START)]
            }
            _ => panic!("engine read of {addr:#06x}: graphics or I/O state the engine doesn't model"),
        }
    }

    #[inline]
    pub fn wb(&mut self, addr: u16, value: u8) {
        match addr {
            0xc000..=0xdfff => self.wram[usize::from(addr - WRAM_START)] = value,
            0xff80..=0xfffe => self.hram[usize::from(addr - HRAM_START)] = value,
            _ => panic!("engine write outside WRAM/HRAM: {addr:#06x}"),
        }
    }

    /// Big-endian word (the game's HP/stat byte order).
    #[inline]
    pub fn rw(&self, addr: u16) -> u16 {
        u16::from(self.rb(addr)) << 8 | u16::from(self.rb(addr + 1))
    }

    #[inline]
    pub fn ww(&mut self, addr: u16, value: u16) {
        self.wb(addr, (value >> 8) as u8);
        self.wb(addr + 1, value as u8);
    }

    /// Little-endian word (pointers, some buffers).
    #[inline]
    pub fn rw_le(&self, addr: u16) -> u16 {
        u16::from(self.rb(addr)) | u16::from(self.rb(addr + 1)) << 8
    }

    #[inline]
    pub fn ww_le(&mut self, addr: u16, value: u16) {
        self.wb(addr, value as u8);
        self.wb(addr + 1, (value >> 8) as u8);
    }

    /// `CopyBytes`: `len` bytes from `src` to `dst`, ascending, one at a time
    /// (overlapping copies behave like the original).
    pub fn copy(&mut self, dst: u16, src: u16, len: u16) {
        for i in 0..len {
            let b = self.rb(src.wrapping_add(i));
            self.wb(dst.wrapping_add(i), b);
        }
    }

    /// `ByteFill`.
    pub fn fill(&mut self, dst: u16, len: u16, value: u8) {
        for i in 0..len {
            self.wb(dst.wrapping_add(i), value);
        }
    }

    pub fn set_bit(&mut self, addr: u16, bit: u8) {
        let v = self.rb(addr);
        self.wb(addr, v | 1 << bit);
    }

    pub fn res_bit(&mut self, addr: u16, bit: u8) {
        let v = self.rb(addr);
        self.wb(addr, v & !(1 << bit));
    }

    pub fn bit(&self, addr: u16, bit: u8) -> bool {
        self.rb(addr) & 1 << bit != 0
    }

    /// Raw WRAM bytes (`0xc000..0xe000`).
    pub fn wram(&self) -> &[u8] {
        &self.wram[..]
    }
}

impl ReadMem for Mem {
    fn read_byte(&self, addr: u16) -> u8 {
        self.rb(addr)
    }
}
