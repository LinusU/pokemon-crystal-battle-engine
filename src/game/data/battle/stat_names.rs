use crate::runtime::Cpu;

pub const LIST_INDEX: u8 = 0;

pub const LIST_ITEM_LENGTH: u8 = 0;

// duplicate assembler constant LIST_ITEM_LENGTH

pub const X: u8 = 8;

pub(crate) fn stat_names(cpu: &mut Cpu) {
    cpu.pc = 0x64a6;
}
