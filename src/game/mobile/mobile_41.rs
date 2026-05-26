use crate::runtime::Cpu;

pub const X: u8 = 7;

pub fn stubbed_trainer_rankings_hall_of_fame2(cpu: &mut Cpu) {
    cpu.pc = 0x5ef6;
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
    return;
    cpu.a = 0x05;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xd4c4);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xa001);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(4 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xa010);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xa005);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(4 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xa039);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xa009);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(4 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xa01b);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xa00d);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(3 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    update_trainer_rankings_checksum(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::close_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn stubbed_trainer_rankings_magikarp_length(cpu: &mut Cpu) {
    cpu.pc = 0x5f33;
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
    return;
    cpu.a = 0x05;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_de(0xd1ea);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0xa07b);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.read_byte(cpu.hl());
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 1;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become stubbed_trainer_rankings_magikarp_length_is_low_byte_higher(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_c {
        cpu.cycle(12);
        become stubbed_trainer_rankings_magikarp_length_new_record_longest(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become stubbed_trainer_rankings_magikarp_length_check_shortest(cpu);
}

pub(crate) fn stubbed_trainer_rankings_magikarp_length_is_low_byte_higher(cpu: &mut Cpu) {
    cpu.pc = 0x5f47;
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.read_byte(cpu.hl());
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    if cpu.flag_c {
        cpu.cycle(12);
        become stubbed_trainer_rankings_magikarp_length_check_shortest(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become stubbed_trainer_rankings_magikarp_length_new_record_longest(cpu);
}

pub(crate) fn stubbed_trainer_rankings_magikarp_length_new_record_longest(cpu: &mut Cpu) {
    cpu.pc = 0x5f4f;
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    become stubbed_trainer_rankings_magikarp_length_check_shortest(cpu);
}

pub(crate) fn stubbed_trainer_rankings_magikarp_length_check_shortest(cpu: &mut Cpu) {
    cpu.pc = 0x5f55;
    cpu.set_hl(0xa07d);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.read_byte(cpu.hl());
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become stubbed_trainer_rankings_magikarp_length_new_record_shortest(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.read_byte(cpu.hl());
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 1;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become stubbed_trainer_rankings_magikarp_length_is_low_byte_lower(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_c {
        cpu.cycle(12);
        become stubbed_trainer_rankings_magikarp_length_new_record_shortest(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become stubbed_trainer_rankings_magikarp_length_done(cpu);
}

pub(crate) fn stubbed_trainer_rankings_magikarp_length_is_low_byte_lower(cpu: &mut Cpu) {
    cpu.pc = 0x5f65;
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.read_byte(cpu.hl());
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 1;
    cpu.cycle(8);
    if !cpu.flag_c {
        cpu.cycle(12);
        become stubbed_trainer_rankings_magikarp_length_done(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    become stubbed_trainer_rankings_magikarp_length_new_record_shortest(cpu);
}

pub(crate) fn stubbed_trainer_rankings_magikarp_length_new_record_shortest(cpu: &mut Cpu) {
    cpu.pc = 0x5f6d;
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    become stubbed_trainer_rankings_magikarp_length_done(cpu);
}

pub(crate) fn stubbed_trainer_rankings_magikarp_length_done(cpu: &mut Cpu) {
    cpu.pc = 0x5f72;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    update_trainer_rankings_checksum(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::close_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn stubbed_trainer_rankings_bug_contest_score(cpu: &mut Cpu) {
    cpu.pc = 0x5f79;
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
    return;
    cpu.a = 0x05;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xffb3);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.set_hl(0xa07f);
    cpu.pc += 3;
    cpu.cycle(12);
    let value = cpu.read_byte(cpu.hl());
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 1;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become stubbed_trainer_rankings_bug_contest_score_is_low_byte_higher(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_c {
        cpu.cycle(12);
        become stubbed_trainer_rankings_bug_contest_score_new_high_score(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become stubbed_trainer_rankings_bug_contest_score_done(cpu);
}

pub(crate) fn stubbed_trainer_rankings_bug_contest_score_is_low_byte_higher(cpu: &mut Cpu) {
    cpu.pc = 0x5f8b;
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xffb4);
    cpu.pc += 2;
    cpu.cycle(12);
    let value = cpu.read_byte(cpu.hl());
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 1;
    cpu.cycle(8);
    if cpu.flag_c {
        cpu.cycle(12);
        become stubbed_trainer_rankings_bug_contest_score_done(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    become stubbed_trainer_rankings_bug_contest_score_new_high_score(cpu);
}

pub(crate) fn stubbed_trainer_rankings_bug_contest_score_new_high_score(cpu: &mut Cpu) {
    cpu.pc = 0x5f92;
    cpu.a = cpu.read_byte(0xffb3);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xffb4);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    become stubbed_trainer_rankings_bug_contest_score_done(cpu);
}

pub(crate) fn stubbed_trainer_rankings_bug_contest_score_done(cpu: &mut Cpu) {
    cpu.pc = 0x5f98;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    update_trainer_rankings_checksum(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::close_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn stubbed_trainer_rankings_add_to_slots_win_streak(cpu: &mut Cpu) {
    cpu.pc = 0x5f9f;
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
    return;
    cpu.a = 0x05;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = (0xa06f as i32) + (1 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    let addr = cpu.hl();
    let value = cpu.read_byte(addr);
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.write_byte(addr, result);
    cpu.pc += 1;
    cpu.cycle(12);
    if !cpu.flag_z {
        cpu.cycle(12);
        become stubbed_trainer_rankings_add_to_slots_win_streak_no_carry(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let addr = cpu.hl();
    let value = cpu.read_byte(addr);
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.write_byte(addr, result);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    become stubbed_trainer_rankings_add_to_slots_win_streak_no_carry(cpu);
}

pub(crate) fn stubbed_trainer_rankings_add_to_slots_win_streak_no_carry(cpu: &mut Cpu) {
    cpu.pc = 0x5fae;
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xa071);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.read_byte(cpu.hl());
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 1;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become stubbed_trainer_rankings_add_to_slots_win_streak_is_low_byte_higher(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_c {
        cpu.cycle(12);
        become stubbed_trainer_rankings_add_to_slots_win_streak_new_record_streak(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become stubbed_trainer_rankings_add_to_slots_win_streak_done(cpu);
}

pub(crate) fn stubbed_trainer_rankings_add_to_slots_win_streak_is_low_byte_higher(cpu: &mut Cpu) {
    cpu.pc = 0x5fb9;
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte({ const N: i32 = (0xa071 as i32) + (1 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.read_byte(cpu.hl());
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 1;
    cpu.cycle(8);
    if !cpu.flag_c {
        cpu.cycle(12);
        become stubbed_trainer_rankings_add_to_slots_win_streak_done(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    become stubbed_trainer_rankings_add_to_slots_win_streak_new_record_streak(cpu);
}

pub(crate) fn stubbed_trainer_rankings_add_to_slots_win_streak_new_record_streak(cpu: &mut Cpu) {
    cpu.pc = 0x5fc1;
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(0xa071, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte({ const N: i32 = (0xa071 as i32) + (1 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    become stubbed_trainer_rankings_add_to_slots_win_streak_done(cpu);
}

pub(crate) fn stubbed_trainer_rankings_add_to_slots_win_streak_done(cpu: &mut Cpu) {
    cpu.pc = 0x5fc9;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    update_trainer_rankings_checksum(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::close_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn stubbed_trainer_rankings_end_slots_win_streak(cpu: &mut Cpu) {
    cpu.pc = 0x5fd0;
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
    return;
    cpu.a = 0x05;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xa06f);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    update_trainer_rankings_checksum(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::close_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn stubbed_trainer_rankings_add_to_slots_payouts(cpu: &mut Cpu) {
    cpu.pc = 0x5fe3;
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
    return;
    cpu.a = 0x05;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = (0xa073 as i32) + (3 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.read_byte(cpu.hl());
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_add(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) > 0x0f;
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.read_byte(cpu.hl());
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
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    if !cpu.flag_c {
        cpu.cycle(12);
        become stubbed_trainer_rankings_add_to_slots_payouts_done(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let addr = cpu.hl();
    let value = cpu.read_byte(addr);
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.write_byte(addr, result);
    cpu.pc += 1;
    cpu.cycle(12);
    if !cpu.flag_z {
        cpu.cycle(12);
        become stubbed_trainer_rankings_add_to_slots_payouts_done(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let addr = cpu.hl();
    let value = cpu.read_byte(addr);
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.write_byte(addr, result);
    cpu.pc += 1;
    cpu.cycle(12);
    if !cpu.flag_z {
        cpu.cycle(12);
        become stubbed_trainer_rankings_add_to_slots_payouts_done(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0xff;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    become stubbed_trainer_rankings_add_to_slots_payouts_done(cpu);
}

pub(crate) fn stubbed_trainer_rankings_add_to_slots_payouts_done(cpu: &mut Cpu) {
    cpu.pc = 0x6001;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    update_trainer_rankings_checksum(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::close_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn stubbed_trainer_rankings_add_to_battle_payouts(cpu: &mut Cpu) {
    cpu.pc = 0x6008;
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
    return;
    cpu.a = 0x05;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = (0xa077 as i32) + (3 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_bc(cpu.bc().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.read_byte(cpu.hl());
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_add(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) > 0x0f;
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_bc(cpu.bc().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.read_byte(cpu.hl());
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
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.read_byte(cpu.hl());
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
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    if !cpu.flag_c {
        cpu.cycle(12);
        become stubbed_trainer_rankings_add_to_battle_payouts_done(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let addr = cpu.hl();
    let value = cpu.read_byte(addr);
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.write_byte(addr, result);
    cpu.pc += 1;
    cpu.cycle(12);
    if !cpu.flag_z {
        cpu.cycle(12);
        become stubbed_trainer_rankings_add_to_battle_payouts_done(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0xff;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    become stubbed_trainer_rankings_add_to_battle_payouts_done(cpu);
}

pub(crate) fn stubbed_trainer_rankings_add_to_battle_payouts_done(cpu: &mut Cpu) {
    cpu.pc = 0x6027;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    update_trainer_rankings_checksum(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::close_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn stubbed_trainer_rankings_step_count(cpu: &mut Cpu) {
    cpu.pc = 0x602e;
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
    return;
    cpu.set_hl(0xa010);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become stubbed_trainer_rankings_increment4_byte(cpu);
}

pub(crate) fn stubbed_trainer_rankings_battle_tower_wins(cpu: &mut Cpu) {
    cpu.pc = 0x6035;
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
    return;
    cpu.a = 0x05;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xaa8d);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.a;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::close_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    if !cpu.flag_z {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(0xa014);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become stubbed_trainer_rankings_increment2_byte(cpu);
}

pub(crate) fn stubbed_trainer_rankings_t_ms_h_ms_taught(cpu: &mut Cpu) {
    cpu.pc = 0x6049;
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
    return;
    cpu.set_hl(0xa018);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become stubbed_trainer_rankings_increment3_byte(cpu);
}

pub(crate) fn stubbed_trainer_rankings_battles(cpu: &mut Cpu) {
    cpu.pc = 0x6050;
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
    return;
    cpu.a = cpu.read_byte(0xd230);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = crate::defs::BATTLETYPE_TUTORIAL as u8;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(0xa01b);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become stubbed_trainer_rankings_increment3_byte(cpu);
}

pub(crate) fn stubbed_trainer_rankings_wild_battles(cpu: &mut Cpu) {
    cpu.pc = 0x605d;
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
    return;
    cpu.a = cpu.read_byte(0xd230);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = crate::defs::BATTLETYPE_TUTORIAL as u8;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(0xa01e);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become stubbed_trainer_rankings_increment3_byte(cpu);
}

pub(crate) fn stubbed_trainer_rankings_trainer_battles(cpu: &mut Cpu) {
    cpu.pc = 0x606a;
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
    return;
    cpu.set_hl(0xa021);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become stubbed_trainer_rankings_increment3_byte(cpu);
}

pub(crate) fn stubbed_trainer_rankings_unused1(cpu: &mut Cpu) {
    cpu.pc = 0x6071;
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
    return;
    cpu.set_hl(0xa024);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become stubbed_trainer_rankings_increment3_byte(cpu);
}

pub fn stubbed_trainer_rankings_hall_of_fame(cpu: &mut Cpu) {
    cpu.pc = 0x6078;
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
    return;
    cpu.set_hl(0xa027);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become stubbed_trainer_rankings_increment3_byte(cpu);
}

pub(crate) fn stubbed_trainer_rankings_wild_mons_caught(cpu: &mut Cpu) {
    cpu.pc = 0x607f;
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
    return;
    cpu.set_hl(0xa02a);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become stubbed_trainer_rankings_increment3_byte(cpu);
}

pub(crate) fn stubbed_trainer_rankings_hooked_encounters(cpu: &mut Cpu) {
    cpu.pc = 0x6086;
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
    return;
    cpu.set_hl(0xa02d);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become stubbed_trainer_rankings_increment3_byte(cpu);
}

pub(crate) fn stubbed_trainer_rankings_eggs_hatched(cpu: &mut Cpu) {
    cpu.pc = 0x608d;
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
    return;
    cpu.set_hl(0xa030);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become stubbed_trainer_rankings_increment3_byte(cpu);
}

pub(crate) fn stubbed_trainer_rankings_mons_evolved(cpu: &mut Cpu) {
    cpu.pc = 0x6094;
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
    return;
    cpu.set_hl(0xa033);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become stubbed_trainer_rankings_increment3_byte(cpu);
}

pub(crate) fn stubbed_trainer_rankings_fruit_picked(cpu: &mut Cpu) {
    cpu.pc = 0x609b;
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
    return;
    cpu.set_hl(0xa036);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become stubbed_trainer_rankings_increment3_byte(cpu);
}

pub(crate) fn stubbed_trainer_rankings_healings(cpu: &mut Cpu) {
    cpu.pc = 0x60a2;
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
    return;
    cpu.set_hl(0xa039);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become stubbed_trainer_rankings_increment3_byte(cpu);
}

pub(crate) fn stubbed_trainer_rankings_mystery_gift(cpu: &mut Cpu) {
    cpu.pc = 0x60a9;
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
    return;
    cpu.set_hl(0xa03c);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(12);
    become stubbed_trainer_rankings_increment3_byte(cpu);
}

pub(crate) fn stubbed_trainer_rankings_trades(cpu: &mut Cpu) {
    cpu.pc = 0x60af;
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
    return;
    cpu.set_hl(0xa03f);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(12);
    become stubbed_trainer_rankings_increment3_byte(cpu);
}

pub(crate) fn stubbed_trainer_rankings_fly(cpu: &mut Cpu) {
    cpu.pc = 0x60b5;
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
    return;
    cpu.set_hl(0xa042);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(12);
    become stubbed_trainer_rankings_increment3_byte(cpu);
}

pub(crate) fn stubbed_trainer_rankings_surf(cpu: &mut Cpu) {
    cpu.pc = 0x60bb;
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
    return;
    cpu.set_hl(0xa045);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(12);
    become stubbed_trainer_rankings_increment3_byte(cpu);
}

pub(crate) fn stubbed_trainer_rankings_waterfall(cpu: &mut Cpu) {
    cpu.pc = 0x60c1;
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
    return;
    cpu.set_hl(0xa048);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(12);
    become stubbed_trainer_rankings_increment3_byte(cpu);
}

pub(crate) fn stubbed_trainer_rankings_white_outs(cpu: &mut Cpu) {
    cpu.pc = 0x60c7;
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
    return;
    cpu.set_hl(0xa04b);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(12);
    become stubbed_trainer_rankings_increment3_byte(cpu);
}

pub(crate) fn stubbed_trainer_rankings_lucky_number_show(cpu: &mut Cpu) {
    cpu.pc = 0x60cd;
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
    return;
    cpu.set_hl(0xa04e);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(12);
    become stubbed_trainer_rankings_increment2_byte(cpu);
}

pub(crate) fn stubbed_trainer_rankings_phone_calls(cpu: &mut Cpu) {
    cpu.pc = 0x60d3;
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
    return;
    cpu.set_hl(0xa051);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(12);
    become stubbed_trainer_rankings_increment3_byte(cpu);
}

pub(crate) fn stubbed_trainer_rankings_unused2(cpu: &mut Cpu) {
    cpu.pc = 0x60d9;
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
    return;
    cpu.set_hl(0xa054);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(12);
    become stubbed_trainer_rankings_increment3_byte(cpu);
}

pub(crate) fn stubbed_trainer_rankings_link_battles(cpu: &mut Cpu) {
    cpu.pc = 0x60df;
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
    return;
    cpu.set_hl(0xa057);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(12);
    become stubbed_trainer_rankings_increment3_byte(cpu);
}

pub(crate) fn stubbed_trainer_rankings_splash(cpu: &mut Cpu) {
    cpu.pc = 0x60e5;
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
    return;
    cpu.a = cpu.read_byte(0xffe4);
    cpu.pc += 2;
    cpu.cycle(12);
    let value = cpu.a;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(0xa05a);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(12);
    become stubbed_trainer_rankings_increment3_byte(cpu);
}

pub(crate) fn stubbed_trainer_rankings_tree_encounters(cpu: &mut Cpu) {
    cpu.pc = 0x60ef;
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
    return;
    cpu.set_hl(0xa05d);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(12);
    become stubbed_trainer_rankings_increment3_byte(cpu);
}

pub(crate) fn stubbed_trainer_rankings_unused3(cpu: &mut Cpu) {
    cpu.pc = 0x60f5;
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
    return;
    cpu.set_hl(0xa060);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(12);
    become stubbed_trainer_rankings_increment3_byte(cpu);
}

pub(crate) fn stubbed_trainer_rankings_colosseum_wins(cpu: &mut Cpu) {
    cpu.pc = 0x60fb;
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
    return;
    cpu.set_hl(0xa063);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(12);
    become stubbed_trainer_rankings_increment3_byte(cpu);
}

pub(crate) fn stubbed_trainer_rankings_colosseum_losses(cpu: &mut Cpu) {
    cpu.pc = 0x6101;
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
    return;
    cpu.set_hl(0xa066);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(12);
    become stubbed_trainer_rankings_increment3_byte(cpu);
}

pub(crate) fn stubbed_trainer_rankings_colosseum_draws(cpu: &mut Cpu) {
    cpu.pc = 0x6107;
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
    return;
    cpu.set_hl(0xa069);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(12);
    become stubbed_trainer_rankings_increment3_byte(cpu);
}

pub(crate) fn stubbed_trainer_rankings_selfdestruct(cpu: &mut Cpu) {
    cpu.pc = 0x610d;
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
    return;
    cpu.a = cpu.read_byte(0xffe4);
    cpu.pc += 2;
    cpu.cycle(12);
    let value = cpu.a;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(0xa06c);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(12);
    become stubbed_trainer_rankings_increment3_byte(cpu);
}

pub(crate) fn stubbed_trainer_rankings_increment4_byte(cpu: &mut Cpu) {
    cpu.pc = 0x6117;
    cpu.push_word(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.set_bc(3 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(12);
    become stubbed_trainer_rankings_increment(cpu);
}

pub(crate) fn stubbed_trainer_rankings_increment3_byte(cpu: &mut Cpu) {
    cpu.pc = 0x611d;
    cpu.push_word(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.set_bc(2 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(12);
    become stubbed_trainer_rankings_increment(cpu);
}

pub(crate) fn stubbed_trainer_rankings_increment2_byte(cpu: &mut Cpu) {
    cpu.pc = 0x6123;
    cpu.push_word(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.set_bc(1 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(12);
    become stubbed_trainer_rankings_increment(cpu);
}

pub(crate) fn stubbed_trainer_rankings_increment1_byte(cpu: &mut Cpu) {
    cpu.pc = 0x6129;
    cpu.push_word(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.set_bc(0 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    become stubbed_trainer_rankings_increment(cpu);
}

pub(crate) fn stubbed_trainer_rankings_increment(cpu: &mut Cpu) {
    cpu.pc = 0x612d;
    cpu.a = 0x05;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.push_word(cpu.de());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.e = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.e;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.e = result;
    cpu.pc += 1;
    cpu.cycle(4);
    become stubbed_trainer_rankings_increment_asm_106136(cpu);
}

pub(crate) fn stubbed_trainer_rankings_increment_asm_106136(cpu: &mut Cpu) {
    cpu.pc = 0x6136;
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(12);
        become stubbed_trainer_rankings_increment_asm_10613d(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.e;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.e = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(12);
        become stubbed_trainer_rankings_increment_asm_106136(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become stubbed_trainer_rankings_increment_asm_10613d(cpu);
}

pub(crate) fn stubbed_trainer_rankings_increment_asm_10613d(cpu: &mut Cpu) {
    cpu.pc = 0x613d;
    let value = cpu.pop_word();
    cpu.set_de(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    if cpu.flag_z {
        cpu.cycle(12);
        become stubbed_trainer_rankings_increment_asm_10614d(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let lhs = cpu.hl();
    let rhs = cpu.bc();
    let (result, carry) = lhs.overflowing_add(rhs);
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0fff) + (rhs & 0x0fff) > 0x0fff;
    cpu.flag_c = carry;
    cpu.set_hl(result);
    cpu.pc += 1;
    cpu.cycle(8);
    become stubbed_trainer_rankings_increment_asm_106142(cpu);
}

pub(crate) fn stubbed_trainer_rankings_increment_asm_106142(cpu: &mut Cpu) {
    cpu.pc = 0x6142;
    let addr = cpu.hl();
    let value = cpu.read_byte(addr);
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.write_byte(addr, result);
    cpu.pc += 1;
    cpu.cycle(12);
    if !cpu.flag_z {
        cpu.cycle(12);
        become stubbed_trainer_rankings_increment_asm_10614d(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
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
    if cpu.flag_z {
        cpu.cycle(12);
        become stubbed_trainer_rankings_increment_asm_10614d(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.c;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.c = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.cycle(12);
    become stubbed_trainer_rankings_increment_asm_106142(cpu);
}

pub(crate) fn stubbed_trainer_rankings_increment_asm_10614d(cpu: &mut Cpu) {
    cpu.pc = 0x614d;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    update_trainer_rankings_checksum(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::close_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_bc(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn update_trainer_rankings_checksum2(cpu: &mut Cpu) {
    cpu.pc = 0x6155;
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
    return;
    cpu.a = 0x05;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    update_trainer_rankings_checksum(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::close_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn update_trainer_rankings_checksum(cpu: &mut Cpu) {
    cpu.pc = 0x6162;
    cpu.push_word(cpu.de());
    cpu.pc += 1;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    calculate_trainer_rankings_checksum(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xa081);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.write_byte(cpu.hl(), cpu.d);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.e);
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.pop_word();
    cpu.set_de(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn calculate_trainer_rankings_checksum(cpu: &mut Cpu) {
    cpu.pc = 0x616e;
    cpu.push_word(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.set_hl(0xa001);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc({ const N: i32 = (0xa081 as i32) - (0xa001 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_de(0 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    become calculate_trainer_rankings_checksum_asm_106179(cpu);
}

pub(crate) fn calculate_trainer_rankings_checksum_asm_106179(cpu: &mut Cpu) {
    cpu.pc = 0x6179;
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.read_byte(cpu.hl());
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_add(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) > 0x0f;
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_c {
        cpu.cycle(12);
        become calculate_trainer_rankings_checksum_asm_10617f(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.d;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.d = result;
    cpu.pc += 1;
    cpu.cycle(4);
    become calculate_trainer_rankings_checksum_asm_10617f(cpu);
}

pub(crate) fn calculate_trainer_rankings_checksum_asm_10617f(cpu: &mut Cpu) {
    cpu.pc = 0x617f;
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_bc(cpu.bc().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.c;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(12);
        become calculate_trainer_rankings_checksum_asm_106179(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.pop_word();
    cpu.set_bc(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn backup_gs_ball_flag(cpu: &mut Cpu) {
    cpu.pc = 0x6187;
    cpu.a = 0x01;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xbe3c);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = 0x01;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte(0xbe44, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::close_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn restore_gs_ball_flag(cpu: &mut Cpu) {
    cpu.pc = 0x619d;
    cpu.a = 0x01;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xbe44);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = 0x01;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte(0xbe3c, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::close_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn verify_trainer_rankings_checksum(cpu: &mut Cpu) {
    cpu.pc = 0x61b3;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    calculate_trainer_rankings_checksum(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xa081);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.read_byte(cpu.hl());
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 1;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.read_byte(cpu.hl());
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn clear_gs_ball_flag(cpu: &mut Cpu) {
    cpu.pc = 0x61c0;
    cpu.a = 0x01;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xbe3c, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::close_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn initialize_trainer_rankings(cpu: &mut Cpu) {
    cpu.pc = 0x61cd;
    cpu.set_hl(0xa001);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc({ const N: i32 = (0xa083 as i32) - (0xa001 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::byte_fill(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xa07d);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0x3;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), 0xe8);
    cpu.pc += 2;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    update_trainer_rankings_checksum(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xa001);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xa084);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc({ const N: i32 = (0xa083 as i32) - (0xa001 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub fn _mobile_print_num(cpu: &mut Cpu) {
    cpu.pc = 0x61ef;
    cpu.push_word(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte({ const N: i32 = (0xffb3 as i32) + (0 as i32); N as u16 }, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.write_byte({ const N: i32 = (0xffb3 as i32) + (1 as i32); N as u16 }, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.write_byte({ const N: i32 = (0xffb3 as i32) + (2 as i32); N as u16 }, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = 0xf;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x1;
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
        become _mobile_print_num_one_byte(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x2;
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
        become _mobile_print_num_two_bytes(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x3;
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
        become _mobile_print_num_three_bytes(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte({ const N: i32 = (0xffb3 as i32) + (0 as i32); N as u16 }, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    become _mobile_print_num_three_bytes(cpu);
}

pub(crate) fn _mobile_print_num_three_bytes(cpu: &mut Cpu) {
    cpu.pc = 0x620a;
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte({ const N: i32 = (0xffb3 as i32) + (1 as i32); N as u16 }, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    become _mobile_print_num_two_bytes(cpu);
}

pub(crate) fn _mobile_print_num_two_bytes(cpu: &mut Cpu) {
    cpu.pc = 0x620e;
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte({ const N: i32 = (0xffb3 as i32) + (2 as i32); N as u16 }, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    become _mobile_print_num_one_byte(cpu);
}

pub(crate) fn _mobile_print_num_one_byte(cpu: &mut Cpu) {
    cpu.pc = 0x6212;
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte({ const N: i32 = (0xffb3 as i32) + (3 as i32); N as u16 }, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.push_word(cpu.de());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte({ const N: i32 = (0xffb3 as i32) + (8 as i32); N as u16 }, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte({ const N: i32 = (0xffb3 as i32) + (9 as i32); N as u16 }, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = 2;
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
        become _mobile_print_num_two_digits(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_de(0x62ae);
    cpu.pc += 3;
    cpu.cycle(12);
    let value = 3;
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
        become _mobile_print_num_three_to_nine_digits(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_de(0x62aa);
    cpu.pc += 3;
    cpu.cycle(12);
    let value = 4;
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
        become _mobile_print_num_three_to_nine_digits(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_de(0x62a6);
    cpu.pc += 3;
    cpu.cycle(12);
    let value = 5;
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
        become _mobile_print_num_three_to_nine_digits(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_de(0x62a2);
    cpu.pc += 3;
    cpu.cycle(12);
    let value = 6;
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
        become _mobile_print_num_three_to_nine_digits(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_de(0x629e);
    cpu.pc += 3;
    cpu.cycle(12);
    let value = 7;
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
        become _mobile_print_num_three_to_nine_digits(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_de(0x629a);
    cpu.pc += 3;
    cpu.cycle(12);
    let value = 8;
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
        become _mobile_print_num_three_to_nine_digits(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_de(0x6296);
    cpu.pc += 3;
    cpu.cycle(12);
    let value = 9;
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
        become _mobile_print_num_three_to_nine_digits(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_de(0x6292);
    cpu.pc += 3;
    cpu.cycle(12);
    become _mobile_print_num_three_to_nine_digits(cpu);
}

pub(crate) fn _mobile_print_num_three_to_nine_digits(cpu: &mut Cpu) {
    cpu.pc = 0x6256;
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
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
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    become _mobile_print_num_digit_loop(cpu);
}

pub(crate) fn _mobile_print_num_digit_loop(cpu: &mut Cpu) {
    cpu.pc = 0x625b;
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    _mobile_print_num_function1062b2(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    _mobile_print_num_function1062ff(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
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
        become _mobile_print_num_digit_loop(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become _mobile_print_num_two_digits(cpu);
}

pub(crate) fn _mobile_print_num_two_digits(cpu: &mut Cpu) {
    cpu.pc = 0x626a;
    cpu.c = 0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte({ const N: i32 = (0xffb3 as i32) + (3 as i32); N as u16 });
    cpu.pc += 2;
    cpu.cycle(12);
    become _mobile_print_num_mod_ten_loop(cpu);
}

pub(crate) fn _mobile_print_num_mod_ten_loop(cpu: &mut Cpu) {
    cpu.pc = 0x626e;
    let value = 10;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_c {
        cpu.cycle(12);
        become _mobile_print_num_simple_divide_done(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 10;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.c;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.c = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.cycle(12);
    become _mobile_print_num_mod_ten_loop(cpu);
}

pub(crate) fn _mobile_print_num_simple_divide_done(cpu: &mut Cpu) {
    cpu.pc = 0x6277;
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte({ const N: i32 = (0xffb3 as i32) + (8 as i32); N as u16 });
    cpu.pc += 2;
    cpu.cycle(12);
    let value = cpu.c;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte({ const N: i32 = (0xffb3 as i32) + (8 as i32); N as u16 }, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    if !cpu.flag_z {
        cpu.cycle(12);
        become _mobile_print_num_create_digit(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    _mobile_print_num_load_minus_ten_if_negative(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(12);
    become _mobile_print_num_done(cpu);
}

pub(crate) fn _mobile_print_num_create_digit(cpu: &mut Cpu) {
    cpu.pc = 0x6284;
    cpu.a = { let value = (0xf6 as i32); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.c;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_add(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) > 0x0f;
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    become _mobile_print_num_done(cpu);
}

pub(crate) fn _mobile_print_num_done(cpu: &mut Cpu) {
    cpu.pc = 0x6288;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    _mobile_print_num_function1062ff(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = { let value = (0xf6 as i32); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.b;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_add(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) > 0x0f;
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.pop_word();
    cpu.set_de(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let value = cpu.pop_word();
    cpu.set_bc(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn _mobile_print_num_function1062b2(cpu: &mut Cpu) {
    cpu.pc = 0x62b2;
    cpu.c = 0x0;
    cpu.pc += 2;
    cpu.cycle(8);
    become _mobile_print_num_asm_1062b4(cpu);
}

pub(crate) fn _mobile_print_num_asm_1062b4(cpu: &mut Cpu) {
    cpu.pc = 0x62b4;
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte({ const N: i32 = (0xffb3 as i32) + (3 as i32); N as u16 });
    cpu.pc += 2;
    cpu.cycle(12);
    let value = cpu.b;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte({ const N: i32 = (0xffb3 as i32) + (7 as i32); N as u16 }, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte({ const N: i32 = (0xffb3 as i32) + (2 as i32); N as u16 });
    cpu.pc += 2;
    cpu.cycle(12);
    let value = cpu.b;
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
    cpu.write_byte({ const N: i32 = (0xffb3 as i32) + (6 as i32); N as u16 }, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte({ const N: i32 = (0xffb3 as i32) + (1 as i32); N as u16 });
    cpu.pc += 2;
    cpu.cycle(12);
    let value = cpu.b;
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
    cpu.write_byte({ const N: i32 = (0xffb3 as i32) + (5 as i32); N as u16 }, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte({ const N: i32 = (0xffb3 as i32) + (0 as i32); N as u16 });
    cpu.pc += 2;
    cpu.cycle(12);
    let value = cpu.b;
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
    cpu.write_byte({ const N: i32 = (0xffb3 as i32) + (4 as i32); N as u16 }, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    if cpu.flag_c {
        cpu.cycle(12);
        become _mobile_print_num_asm_1062eb(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte({ const N: i32 = (0xffb3 as i32) + (4 as i32); N as u16 });
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.write_byte({ const N: i32 = (0xffb3 as i32) + (0 as i32); N as u16 }, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = cpu.read_byte({ const N: i32 = (0xffb3 as i32) + (5 as i32); N as u16 });
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.write_byte({ const N: i32 = (0xffb3 as i32) + (1 as i32); N as u16 }, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = cpu.read_byte({ const N: i32 = (0xffb3 as i32) + (6 as i32); N as u16 });
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.write_byte({ const N: i32 = (0xffb3 as i32) + (2 as i32); N as u16 }, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = cpu.read_byte({ const N: i32 = (0xffb3 as i32) + (7 as i32); N as u16 });
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.write_byte({ const N: i32 = (0xffb3 as i32) + (3 as i32); N as u16 }, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    let value = cpu.c;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.c = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.cycle(12);
    become _mobile_print_num_asm_1062b4(cpu);
}

pub(crate) fn _mobile_print_num_asm_1062eb(cpu: &mut Cpu) {
    cpu.pc = 0x62eb;
    cpu.a = cpu.read_byte({ const N: i32 = (0xffb3 as i32) + (8 as i32); N as u16 });
    cpu.pc += 2;
    cpu.cycle(12);
    let value = cpu.c;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become _mobile_print_num_load_minus_ten_if_negative(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = -10i16 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.c;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_add(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) > 0x0f;
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte({ const N: i32 = (0xffb3 as i32) + (8 as i32); N as u16 }, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn _mobile_print_num_load_minus_ten_if_negative(cpu: &mut Cpu) {
    cpu.pc = 0x62f7;
    cpu.a = cpu.read_byte({ const N: i32 = (0xffb3 as i32) + (9 as i32); N as u16 });
    cpu.pc += 2;
    cpu.cycle(12);
    let mask = 1u8 << ((7) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), -10i16 as u8);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn _mobile_print_num_function1062ff(cpu: &mut Cpu) {
    cpu.pc = 0x62ff;
    cpu.a = cpu.read_byte({ const N: i32 = (0xffb3 as i32) + (9 as i32); N as u16 });
    cpu.pc += 2;
    cpu.cycle(12);
    let mask = 1u8 << ((7) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(12);
        become _mobile_print_num_asm_10630d(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let mask = 1u8 << ((6) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become _mobile_print_num_asm_10630d(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte({ const N: i32 = (0xffb3 as i32) + (8 as i32); N as u16 });
    cpu.pc += 2;
    cpu.cycle(12);
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
    become _mobile_print_num_asm_10630d(cpu);
}

pub(crate) fn _mobile_print_num_asm_10630d(cpu: &mut Cpu) {
    cpu.pc = 0x630d;
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn check_mobile_adapter_status_special(cpu: &mut Cpu) {
    cpu.pc = 0x630f;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc2dd, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn set_mobile_adapter_status(cpu: &mut Cpu) {
    cpu.pc = 0x6314;
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
    return;
    cpu.a = 0x04;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = !cpu.a;
    cpu.flag_n = true;
    cpu.flag_h = true;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xb000, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::close_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x07;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xa800, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::close_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn check_mobile_adapter_status(cpu: &mut Cpu) {
    cpu.pc = 0x632f;
    let value = cpu.a;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
    return;
    cpu.a = 0x04;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xb000);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = !cpu.a;
    cpu.flag_n = true;
    cpu.flag_h = true;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::close_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x07;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xa800);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::close_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.b;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(12);
        become check_mobile_adapter_status_nope(cpu);
    }
    cpu.pc += 2;
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
        become check_mobile_adapter_status_nope(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0b10001111;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.c;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(12);
        become check_mobile_adapter_status_nope(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = true;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn check_mobile_adapter_status_nope(cpu: &mut Cpu) {
    cpu.pc = 0x6359;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function10635c(cpu: &mut Cpu) {
    cpu.pc = 0x635c;
    cpu.a = cpu.read_byte(0xcd25);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((crate::defs::JUMPTABLE_EXIT_F as u8) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xcd25);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0x636a);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::jump_table(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function10635c_init(cpu: &mut Cpu) {
    cpu.pc = 0x637c;
    cpu.set_de(0xcd30);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0x41 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(0x41 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = crate::defs::MOBILEAPI_20 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::mobile::mobile_api(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xcd25);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.a;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcd25, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function106392(cpu: &mut Cpu) {
    cpu.pc = 0x6392;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcf64, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xc821);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((1) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(12);
        become function106392_asm_1063a2(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let mask = 1u8 << ((0) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become function106392_asm_1063bf(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function106392_asm_1063a2(cpu: &mut Cpu) {
    cpu.pc = 0x63a2;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    check_mobile_adapter_status(cpu);
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
    if !cpu.flag_z {
        cpu.cycle(12);
        become function106392_asm_1063b4(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0xb;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcf64, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x7;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd25, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function106392_asm_1063b4(cpu: &mut Cpu) {
    cpu.pc = 0x63b4;
    cpu.a = 0x7;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcf64, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x7;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd25, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function106392_asm_1063bf(cpu: &mut Cpu) {
    cpu.pc = 0x63bf;
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcf64, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xcd25);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.a;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcd25, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function1063cc(cpu: &mut Cpu) {
    cpu.pc = 0x63cc;
    cpu.a = 0x78;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd42, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xcd25);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.a;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcd25, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    become function1063d8(cpu);
}

pub(crate) fn function1063d8(cpu: &mut Cpu) {
    cpu.pc = 0x63d8;
    cpu.set_hl(0xcd42);
    cpu.pc += 3;
    cpu.cycle(12);
    let addr = cpu.hl();
    let value = cpu.read_byte(addr);
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.write_byte(addr, result);
    cpu.pc += 1;
    cpu.cycle(12);
    if !cpu.flag_z {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xcd25);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.a;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcd25, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function1063e5(cpu: &mut Cpu) {
    cpu.pc = 0x63e5;
    cpu.a = cpu.read_byte(0xcf64);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x3;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xcd25);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.a;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcd25, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function1063f3(cpu: &mut Cpu) {
    cpu.pc = 0x63f3;
    cpu.set_de(0xcd31);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = crate::defs::MOBILEAPI_TELEPHONESTATUS as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::mobile::mobile_api(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xcd25);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.a;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcd25, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function106403(cpu: &mut Cpu) {
    cpu.pc = 0x6403;
    cpu.a = cpu.read_byte(0xc821);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((1) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(12);
        become function106403_asm_106426(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let mask = 1u8 << ((0) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become function106403_asm_10640f(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function106403_asm_10640f(cpu: &mut Cpu) {
    cpu.pc = 0x640f;
    cpu.a = cpu.read_byte(0xcd31);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x80;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xcd30);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.c;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.a;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    set_mobile_adapter_status(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xcd25);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.a;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcd25, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function106403_asm_106426(cpu: &mut Cpu) {
    cpu.pc = 0x6426;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    check_mobile_adapter_status(cpu);
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
    if cpu.flag_z {
        cpu.cycle(12);
        become function106403_asm_106435(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xcd25);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.a;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcd25, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function106403_asm_106435(cpu: &mut Cpu) {
    cpu.pc = 0x6435;
    cpu.c = 0x0;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    set_mobile_adapter_status(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xcd25);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.a;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcd25, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function106442(cpu: &mut Cpu) {
    cpu.pc = 0x6442;
    cpu.a = crate::defs::MOBILEAPI_1B as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::mobile::mobile_api(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xffe9, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.write_byte(0xffc9, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xcd25);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.a;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcd25, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    become function106453(cpu);
}

pub(crate) fn function106453(cpu: &mut Cpu) {
    cpu.pc = 0x6453;
    cpu.a = cpu.read_byte(0xcd25);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((crate::defs::JUMPTABLE_EXIT_F as u8) & 7);
    cpu.a |= mask;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd25, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = 0x4;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcf64, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn stubbed_function106462(cpu: &mut Cpu) {
    cpu.pc = 0x6462;
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
    return;
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub fn function106464(cpu: &mut Cpu) {
    cpu.pc = 0x6464;
    cpu.set_de(0x5214);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl({ const N: i32 = ((0x9000 as i32) + ((crate::defs::TILE_SIZE as u16 as i32) * (0x60 as i32))); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc({ const N: i32 = ((((0x3e as i32) & (0xff as i32)) << (8 as i32)) | ((1 as i32) & (0xff as i32))); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::gfx::get2bpp(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_de(0x5424);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl({ const N: i32 = ((0x9000 as i32) + ((crate::defs::TILE_SIZE as u16 as i32) * (0x61 as i32))); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc({ const N: i32 = ((((0x3e as i32) & (0xff as i32)) << (8 as i32)) | ((1 as i32) & (0xff as i32))); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::gfx::get2bpp(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_de(0x6514);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl({ const N: i32 = ((0x9000 as i32) + ((crate::defs::TILE_SIZE as u16 as i32) * (0x62 as i32))); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.c = 9;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.b = 0x41;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::gfx::get2bpp(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_de(0x40b0 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl({ const N: i32 = ((0x9000 as i32) + ((crate::defs::TILE_SIZE as u16 as i32) * (0x6b as i32))); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = 0x0f;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::gfx::get2bpp(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x3e;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x74cc);
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

pub(crate) fn function10649b(cpu: &mut Cpu) {
    cpu.pc = 0x649b;
    cpu.a = cpu.read_byte(0xcfce);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 7;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_bc(0x0030);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0x4800);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::array::add_n_times(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.d = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.e = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl({ const N: i32 = ((0x9000 as i32) + ((crate::defs::TILE_SIZE as u16 as i32) * (0x79 as i32))); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.c = crate::defs::TEXTBOX_FRAME_TILES as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.b = 0x3e;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function1064c3(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = ((0x9000 as i32) + ((crate::defs::TILE_SIZE as u16 as i32) * (0x7f as i32))); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0x5204);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.c = 1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.b = 0x3e;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function1064c3(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function1064c3(cpu: &mut Cpu) {
    cpu.pc = 0x64c3;
    cpu.a = cpu.read_byte(crate::defs::R_WBK as u16);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = 0x6;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.push_word(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.set_hl(0x3f88);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let value = cpu.pop_word();
    cpu.set_bc(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.cycle(12);
    become asm_1064ed(cpu);
}

pub(crate) fn function1064d8(cpu: &mut Cpu) {
    cpu.pc = 0x64d8;
    cpu.a = cpu.read_byte(crate::defs::R_WBK as u16);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = 0x6;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.push_word(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.set_hl(0x3f9f);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let value = cpu.pop_word();
    cpu.set_bc(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.cycle(12);
    become asm_1064ed(cpu);
}

pub(crate) fn asm_1064ed(cpu: &mut Cpu) {
    cpu.pc = 0x64ed;
    cpu.set_de(0xd000);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = 0x0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(crate::defs::R_WBK as u16);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = 0x6;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
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
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::gfx::get2bpp(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte(crate::defs::R_VBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function10650a(cpu: &mut Cpu) {
    cpu.pc = 0x650a;
    cpu.set_de(0x5234);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc({ const N: i32 = ((((0x3e as i32) & (0xff as i32)) << (8 as i32)) | ((17 as i32) & (0xff as i32))); N as u16 });
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

