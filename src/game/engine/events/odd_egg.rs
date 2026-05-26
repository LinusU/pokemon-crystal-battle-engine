use crate::runtime::Cpu;

pub const N: u8 = 12;

pub(crate) fn _give_odd_egg(cpu: &mut Cpu) {
    cpu.pc = 0x74b6;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::random::random(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0x7552);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.c = 0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.b = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    become _give_odd_egg_loop(cpu);
}

pub(crate) fn _give_odd_egg_loop(cpu: &mut Cpu) {
    cpu.pc = 0x74bf;
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.d = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = (((0xffff as u32) >> 8) & 0x00ff) as u8;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(12);
        become _give_odd_egg_not_done(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = ((0xffff as u32) & 0x00ff) as u8;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become _give_odd_egg_done(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become _give_odd_egg_not_done(cpu);
}

pub(crate) fn _give_odd_egg_not_done(cpu: &mut Cpu) {
    cpu.pc = 0x74cd;
    cpu.a = cpu.read_byte(0xffe2);
    cpu.pc += 2;
    cpu.cycle(12);
    let value = cpu.d;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_c {
        cpu.cycle(12);
        become _give_odd_egg_done(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become _give_odd_egg_ok(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become _give_odd_egg_next(cpu);
}

pub(crate) fn _give_odd_egg_ok(cpu: &mut Cpu) {
    cpu.pc = 0x74d6;
    cpu.a = cpu.read_byte(0xffe1);
    cpu.pc += 2;
    cpu.cycle(12);
    let value = cpu.e;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_c {
        cpu.cycle(12);
        become _give_odd_egg_done(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become _give_odd_egg_done(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become _give_odd_egg_next(cpu);
}

pub(crate) fn _give_odd_egg_next(cpu: &mut Cpu) {
    cpu.pc = 0x74dd;
    cpu.set_bc(cpu.bc().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.cycle(12);
    become _give_odd_egg_loop(cpu);
}

pub(crate) fn _give_odd_egg_done(cpu: &mut Cpu) {
    cpu.pc = 0x74e0;
    cpu.set_hl(0x756e);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = crate::defs::NICKNAMED_MON_STRUCT_LENGTH as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::array::add_n_times(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_de(0xc608);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc({ const N: i32 = (crate::defs::NICKNAMED_MON_STRUCT_LENGTH as u16 as i32) + (crate::defs::NAME_LENGTH as u16 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = crate::defs::EGG_TICKET as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xd106, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xd10c, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = -1i16 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xd107, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xd892);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::item::toss_item(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = crate::defs::EGG as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd2a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = ((((0xcd2a as u32) - (1 as u32))) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd20, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = (((((0xcd2a as u32) - (1 as u32))) >> 8) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte({ const N: i32 = (0xcd20 as i32) + (1 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = ((0xc608u32) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd22, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = (((0xc608u32) >> 8) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte({ const N: i32 = (0xcd22 as i32) + (1 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0x7546);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xcd2b);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(crate::defs::MON_NAME_LENGTH as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = ((0xcd2bu32) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd24, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = (((0xcd2bu32) >> 8) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte({ const N: i32 = (0xcd24 as i32) + (1 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = ((0xc638u32) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd26, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = (((0xc638u32) >> 8) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte({ const N: i32 = (0xcd26 as i32) + (1 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x46;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x798f);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

