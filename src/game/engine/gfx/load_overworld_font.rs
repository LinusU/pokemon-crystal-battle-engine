use crate::runtime::Cpu;

pub fn load_overworld_font(cpu: &mut Cpu) {
    cpu.pc = 0x6594;
    cpu.set_de(0x65ad);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0x8800);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc({ const N: i32 = ((((0x41 as i32) & (0xff as i32)) << (8 as i32)) | ((0x80 as i32) & (0xff as i32))); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::gfx::get2bpp(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_de(0x6dad);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl({ const N: i32 = ((0x9000 as i32) + ((crate::defs::TILE_SIZE as u16 as i32) * (0x7f as i32))); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc({ const N: i32 = ((((0x41 as i32) & (0xff as i32)) << (8 as i32)) | ((1 as i32) & (0xff as i32))); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::gfx::get2bpp(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

