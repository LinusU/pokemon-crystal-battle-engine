use crate::runtime::Cpu;

pub(crate) fn give_dratini(cpu: &mut Cpu) {
    cpu.pc = 0x7170;
    cpu.a = cpu.read_byte(0xc2dd);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x2;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_c {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_bc(0xdcd7);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(crate::defs::MON_SPECIES as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    give_dratini_get_nth_party_mon(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_de(crate::defs::PARTYMON_STRUCT_LENGTH as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    become give_dratini_check_for_dratini(cpu);
}

pub(crate) fn give_dratini_check_for_dratini(cpu: &mut Cpu) {
    cpu.pc = 0x7185;
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = crate::defs::DRATINI as u8;
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
        become give_dratini_give_moveset(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.e;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.d;
    let carry_in = u8::from(cpu.flag_c);
    let lhs = cpu.a;
    let (partial, borrow1) = lhs.overflowing_sub(value);
    let (result, borrow2) = partial.overflowing_sub(carry_in);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < ((value & 0x0f) + carry_in);
    cpu.flag_c = borrow1 || borrow2;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.h = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.c;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.c = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(12);
        become give_dratini_check_for_dratini(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn give_dratini_give_moveset(cpu: &mut Cpu) {
    cpu.pc = 0x7194;
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xc2dd);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0x71c4);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc({ const N: i32 = (0x71c9 as i32) - (0x71c4 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::array::add_n_times(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_de(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    become give_dratini_give_moves(cpu);
}

pub(crate) fn give_dratini_give_moves(cpu: &mut Cpu) {
    cpu.pc = 0x71a4;
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.a;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.push_word(cpu.de());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl({ const N: i32 = (0x5afb as i32) + (crate::defs::MOVE_PP as u16 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(crate::defs::MOVE_LENGTH as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::array::add_n_times(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x10;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::get_far_byte(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = (crate::defs::MON_PP as u16 as i32) - (crate::defs::MON_MOVES as u16 as i32); N as u16 });
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
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.pop_word();
    cpu.set_de(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.cycle(12);
    become give_dratini_give_moves(cpu);
}

pub(crate) fn give_dratini_get_nth_party_mon(cpu: &mut Cpu) {
    cpu.pc = 0x71ce;
    cpu.set_de(0xdcdf);
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
    let value = cpu.a;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become give_dratini_empty_party(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(crate::defs::PARTYMON_STRUCT_LENGTH as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    become give_dratini_loop(cpu);
}

pub(crate) fn give_dratini_loop(cpu: &mut Cpu) {
    cpu.pc = 0x71da;
    let lhs = cpu.hl();
    let rhs = cpu.de();
    let (result, carry) = lhs.overflowing_add(rhs);
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0fff) + (rhs & 0x0fff) > 0x0fff;
    cpu.flag_c = carry;
    cpu.set_hl(result);
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(12);
        become give_dratini_loop(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn give_dratini_empty_party(cpu: &mut Cpu) {
    cpu.pc = 0x71df;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = true;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}
