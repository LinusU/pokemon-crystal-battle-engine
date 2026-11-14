//! The engine's memory: the original game's WRAM (banks 0 and 1) and HRAM,
//! byte for byte.
//!
//! The battle engine keeps all of its state in these bytes, and the original
//! code depends on their layout (block copies in Transform and Baton Pass,
//! overlapping structs, known glitches). Keeping the same layout makes the
//! port exact by construction and lets the differential tester compare the
//! engine's memory with the original's directly. Named accessors live next to
//! the code that uses them; addresses come from [`super::sym`].

use crate::runtime::ReadMem;

const WRAM_START: u16 = 0xc000;
const WRAM_LEN: usize = 0x2000;
const HRAM_START: u16 = 0xff80;
const HRAM_LEN: usize = 0x7f;

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
            // Echo RAM (e000-fdff) mirrors c000-ddff, including the tilemap
            // and other graphics buffers the engine doesn't keep: a glitch
            // that reads it (Beat Up's stale loop counter walking past the
            // party) depends on what's on screen. See docs/goal.md.
            _ => panic!("engine read outside WRAM/HRAM: {addr:#06x} (graphics state the engine doesn't model)"),
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
