use crate::runtime::Cpu;

pub fn count_set_bits(cpu: &mut Cpu) {
    cpu.pc = 0x335f;
    cpu.c = 0;
    cpu.pc += 2;
    cpu.cycle(8);
    become count_set_bits_next(cpu);
}

pub(crate) fn count_set_bits_next(cpu: &mut Cpu) {
    cpu.pc = 0x3361;
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.d = 8;
    cpu.pc += 2;
    cpu.cycle(8);
    become count_set_bits_count(cpu);
}

pub(crate) fn count_set_bits_count(cpu: &mut Cpu) {
    cpu.pc = 0x3365;
    let value = cpu.e;
    let result = value >> 1;
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x01 != 0;
    cpu.e = result;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.c;
    let carry_in = u8::from(cpu.flag_c);
    let lhs = cpu.a;
    let (partial, carry1) = lhs.overflowing_add(value);
    let (result, carry2) = partial.overflowing_add(carry_in);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) + carry_in > 0x0f;
    cpu.flag_c = carry1 || carry2;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.d;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.d = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(12);
        become count_set_bits_count(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.b;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.b = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(12);
        become count_set_bits_next(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xd265, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub fn get_weekday(cpu: &mut Cpu) {
    cpu.pc = 0x3376;
    cpu.a = cpu.read_byte(0xd4cb);
    cpu.pc += 3;
    cpu.cycle(16);
    become get_weekday_mod(cpu);
}

pub(crate) fn get_weekday_mod(cpu: &mut Cpu) {
    cpu.pc = 0x3379;
    let value = 7;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_c {
        cpu.cycle(12);
        become get_weekday_mod(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 7;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_add(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) > 0x0f;
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub fn set_seen_and_caught_mon(cpu: &mut Cpu) {
    cpu.pc = 0x3380;
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl(0xde99);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = crate::defs::SET_FLAG as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    pokedex_flag_action(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    become set_seen_mon(cpu);
}

pub fn set_seen_mon(cpu: &mut Cpu) {
    cpu.pc = 0x338b;
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl(0xdeb9);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = crate::defs::SET_FLAG as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become pokedex_flag_action(cpu);
}

pub fn check_caught_mon(cpu: &mut Cpu) {
    cpu.pc = 0x3393;
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl(0xde99);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = crate::defs::CHECK_FLAG as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become pokedex_flag_action(cpu);
}

pub fn check_seen_mon(cpu: &mut Cpu) {
    cpu.pc = 0x339b;
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl(0xdeb9);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = crate::defs::CHECK_FLAG as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    become pokedex_flag_action(cpu);
}

pub fn pokedex_flag_action(cpu: &mut Cpu) {
    cpu.pc = 0x33a1;
    cpu.d = 0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = { let value = (((0x4574 as i32) - (0x456b as i32)) / (3 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::predef::predef(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.a;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}
