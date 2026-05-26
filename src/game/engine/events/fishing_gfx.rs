use crate::runtime::Cpu;

pub(crate) fn load_fishing_gfx(cpu: &mut Cpu) {
    cpu.pc = 0x44b3;
    cpu.a = cpu.read_byte(crate::defs::R_VBK as u16);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_VBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.set_de(0x44f2);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xd472);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((crate::defs::PLAYERGENDER_FEMALE_F as u8) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become load_fishing_gfx_got_gender(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_de(0x4582);
    cpu.pc += 3;
    cpu.cycle(12);
    become load_fishing_gfx_got_gender(cpu);
}

pub(crate) fn load_fishing_gfx_got_gender(cpu: &mut Cpu) {
    cpu.pc = 0x44c7;
    cpu.set_hl({ const N: i32 = ((0x8000 as i32) + ((crate::defs::TILE_SIZE as u16 as i32) * (0x02 as i32))); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    load_fishing_gfx_load_gfx(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = ((0x8000 as i32) + ((crate::defs::TILE_SIZE as u16 as i32) * (0x06 as i32))); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    load_fishing_gfx_load_gfx(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = ((0x8000 as i32) + ((crate::defs::TILE_SIZE as u16 as i32) * (0x0a as i32))); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    load_fishing_gfx_load_gfx(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = ((0x8000 as i32) + ((crate::defs::TILE_SIZE as u16 as i32) * (0xfc as i32))); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    load_fishing_gfx_load_gfx(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte(crate::defs::R_VBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn load_fishing_gfx_load_gfx(cpu: &mut Cpu) {
    cpu.pc = 0x44e3;
    cpu.set_bc({ const N: i32 = ((((0x2e as i32) & (0xff as i32)) << (8 as i32)) | ((2 as i32) & (0xff as i32))); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.push_word(cpu.de());
    cpu.pc += 1;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::gfx::get2bpp(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_de(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.set_hl(0x0020);
    cpu.pc += 3;
    cpu.cycle(12);
    let lhs = cpu.hl();
    let rhs = cpu.de();
    let (result, carry) = lhs.overflowing_add(rhs);
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0fff) + (rhs & 0x0fff) > 0x0fff;
    cpu.flag_c = carry;
    cpu.set_hl(result);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.d = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.e = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

