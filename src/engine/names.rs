//! `home/names.asm` and `home/copy_name.asm`: names into `wStringBuffer1`.

use super::battle::Engine;
use super::rom;
use super::rsym as r;
use super::sym::*;
use crate::defs::{ITEM_NAME, MON_NAME, MOVE_NAME, TM01};

const MON_NAME_LENGTH: u16 = 11;
const ITEM_NAME_LENGTH: u16 = 13;

impl Engine {
    /// A byte at `bank:addr`, which may be ROM or (bank 0) WRAM.
    fn banked_byte(&self, bank: u8, addr: u16) -> u8 {
        if addr >= 0xc000 { self.mem.rb(addr) } else { rom::byte(rom::flat(bank, addr)) }
    }

    /// `GetPokemonName`: name of species `wNamedObjectIndex` into
    /// `wStringBuffer1` (10 bytes + `@`). Returns `de` (`wStringBuffer1`).
    pub fn get_pokemon_name(&mut self) -> u16 {
        let index = u16::from(self.mem.rb(W_NAMED_OBJECT_INDEX).wrapping_sub(1));
        let src = r::POKEMON_NAMES + usize::from(index) * 10;
        for i in 0..MON_NAME_LENGTH - 1 {
            self.mem.wb(W_STRING_BUFFER1 + i, rom::byte(src + usize::from(i)));
        }
        self.mem.wb(W_STRING_BUFFER1 + MON_NAME_LENGTH - 1, CHAR_TERMINATOR);
        W_STRING_BUFFER1
    }

    /// `GetName`: name `wCurSpecies` of list `wNamedObjectType` into
    /// `wStringBuffer1`.
    pub fn get_name(&mut self) {
        let kind = self.mem.rb(W_NAMED_OBJECT_TYPE);
        let de = if kind == MON_NAME {
            let species = self.mem.rb(W_CUR_SPECIES);
            self.mem.wb(W_NAMED_OBJECT_INDEX, species);
            self.get_pokemon_name() + MON_NAME_LENGTH
        } else {
            // NamesPointers: `dba` = bank, then a little-endian address.
            let entry = r::NAMES_POINTERS + usize::from(kind - 1) * 3;
            let bank = rom::byte(entry);
            let mut addr = rom::ptr(entry + 1);
            // GetNthString(wCurSpecies - 1)
            let n = self.mem.rb(W_CUR_SPECIES).wrapping_sub(1);
            for _ in 0..n {
                while self.banked_byte(bank, addr) != CHAR_TERMINATOR {
                    addr = addr.wrapping_add(1);
                }
                addr = addr.wrapping_add(1);
            }
            for i in 0..ITEM_NAME_LENGTH {
                let b = self.banked_byte(bank, addr.wrapping_add(i));
                self.mem.wb(W_STRING_BUFFER1 + i, b);
            }
            W_STRING_BUFFER1 + ITEM_NAME_LENGTH
        };
        self.mem.ww_le(W_UNUSED_NAMES_POINTER, de);
    }

    /// `GetMoveName`: move `wNamedObjectIndex` into `wStringBuffer1`.
    pub fn get_move_name(&mut self) -> u16 {
        self.mem.wb(W_NAMED_OBJECT_TYPE, MOVE_NAME);
        let mv = self.mem.rb(W_NAMED_OBJECT_INDEX);
        self.mem.wb(W_CUR_SPECIES, mv);
        self.get_name();
        W_STRING_BUFFER1
    }

    /// `GetItemName`: item `wNamedObjectIndex` into `wStringBuffer1`.
    pub fn get_item_name(&mut self) -> u16 {
        let item = self.mem.rb(W_NAMED_OBJECT_INDEX);
        if item >= TM01 {
            self.get_tmhm_name();
            return W_STRING_BUFFER1;
        }
        self.mem.wb(W_CUR_SPECIES, item);
        self.mem.wb(W_NAMED_OBJECT_TYPE, ITEM_NAME);
        self.get_name();
        W_STRING_BUFFER1
    }

    /// `CopyName1`: the `@`-terminated string at `de` into `wStringBuffer2`.
    pub fn copy_name1(&mut self, de: u16) {
        self.copy_name2(de, W_STRING_BUFFER2);
    }

    /// `CopyName2`: the `@`-terminated string at `de` to `hl`.
    pub fn copy_name2(&mut self, mut de: u16, mut hl: u16) {
        loop {
            let a = self.mem.rb(de);
            de += 1;
            self.mem.wb(hl, a);
            hl += 1;
            if a == CHAR_TERMINATOR {
                return;
            }
        }
    }
}

/// `'@'` in the game's charmap.
pub const CHAR_TERMINATOR: u8 = 0x50;
