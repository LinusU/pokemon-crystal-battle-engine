mod cpu;
mod gpu;
mod keypad;
mod mbc;

pub use cpu::{Cpu, CycleLimit, RngDraw};
pub use gpu::{Gpu, SCREEN_H, SCREEN_W};
pub use keypad::{Keypad, KeypadEvent, KeypadKey};

#[derive(Clone, Copy, Debug)]
pub struct Routine {
    pub bank: u8,
    pub addr: u16,
    pub func: fn(&mut Cpu),
}

#[derive(Clone, Copy, Debug, Default)]
pub struct InterruptVectors {
    pub vblank: Option<Routine>,
    pub lcd_stat: Option<Routine>,
    pub timer: Option<Routine>,
    pub serial: Option<Routine>,
    pub joypad: Option<Routine>,
}

pub use crate::engine::mem::ReadMem;

impl ReadMem for Cpu {
    fn read_byte(&self, addr: u16) -> u8 {
        Cpu::read_byte(self, addr)
    }
}
