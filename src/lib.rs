#![feature(explicit_tail_calls)]
#![allow(dead_code)]
#![allow(incomplete_features)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unreachable_code)]
#![allow(unconditional_recursion)]
#![allow(unused_comparisons)]
#![allow(unused_parens)]

pub mod battle;
pub mod bootstrap;
pub mod defs;
pub mod dispatch;
pub mod game;
pub mod macros;
pub mod rom;
pub mod runtime;

pub fn entry_routine() -> crate::runtime::Routine {
    crate::runtime::Routine {
        bank: 0x00,
        addr: 0x0100,
        func: crate::game::home::header::start,
    }
}

pub fn interrupt_vectors() -> crate::runtime::InterruptVectors {
    crate::runtime::InterruptVectors {
        vblank: Some(crate::runtime::Routine {
        bank: 0x00,
        addr: 0x0283,
        func: crate::game::home::vblank::v_blank,
    }),
        lcd_stat: Some(crate::runtime::Routine {
        bank: 0x00,
        addr: 0x0552,
        func: crate::game::home::lcd::lcd,
    }),
        timer: Some(crate::runtime::Routine {
        bank: 0x00,
        addr: 0x3e93,
        func: crate::game::home::mobile::mobile_timer,
    }),
        serial: Some(crate::runtime::Routine {
        bank: 0x00,
        addr: 0x06ef,
        func: crate::game::home::serial::serial,
    }),
        joypad: Some(crate::runtime::Routine {
        bank: 0x00,
        addr: 0x092e,
        func: crate::game::home::joypad::joypad,
    }),
    }
}

pub fn run_entry() -> crate::runtime::Cpu {
    let routine = entry_routine();
    let mut cpu = crate::runtime::Cpu::default();
    cpu.interrupt_vectors = interrupt_vectors();
    (routine.func)(&mut cpu);
    cpu
}
