use crate::runtime::Cpu;

pub const CURRENT_TABLE_WIDTH: u8 = 3;

pub const W: u8 = 68;

pub const X: u8 = 204;

pub const Y: i32 = 0;

pub(crate) fn cries(cpu: &mut Cpu) {
    cpu.pc = 0x51b0;
}
