//! Read-only game data, straight from the ROM image.
//!
//! Every table the battle code consults (moves, base stats, the type chart,
//! effect scripts, AI lists, trainer data) is read from `pokecrystal.gbc` at
//! the address `pokecrystal.sym` gives, so the data is exact by construction.

use crate::rom::ROM;

/// Flat ROM offset of `bank:addr`.
pub const fn flat(bank: u8, addr: u16) -> usize {
    if addr < 0x4000 { addr as usize } else { bank as usize * 0x4000 + (addr as usize - 0x4000) }
}

#[inline]
pub fn byte(offset: usize) -> u8 {
    ROM[offset]
}

/// Little-endian pointer at `offset` (as `dw` stores it).
#[inline]
pub fn ptr(offset: usize) -> u16 {
    u16::from(ROM[offset]) | u16::from(ROM[offset + 1]) << 8
}

/// The bank a flat offset lives in.
pub const fn bank_of(offset: usize) -> u8 {
    (offset / 0x4000) as u8
}
