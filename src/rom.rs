//! The ROM image (`pokecrystal.gbc`, gitignored): what the emulated original
//! runs, and what the harness's text renderer reads battle text from.

pub const ROM: &[u8] = include_bytes!("../pokecrystal.gbc");

/// Flat ROM offset of `bank:addr`.
pub const fn flat(bank: u8, addr: u16) -> usize {
    if addr < 0x4000 { addr as usize } else { bank as usize * 0x4000 + (addr as usize - 0x4000) }
}

#[inline]
pub fn byte(offset: usize) -> u8 {
    ROM[offset]
}

/// Little-endian pointer at `offset`.
#[inline]
pub fn ptr(offset: usize) -> u16 {
    u16::from(ROM[offset]) | u16::from(ROM[offset + 1]) << 8
}

/// The bank a flat offset lives in.
pub const fn bank_of(offset: usize) -> u8 {
    (offset / 0x4000) as u8
}
