use crate::runtime::Cpu;

pub fn print_bcd_number(cpu: &mut Cpu) {
    cpu.pc = 0x38bb;
    cpu.b = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    let mask = 1u8 << ((crate::defs::PRINTNUM_LEADINGZEROS_F as u8) & 7);
    cpu.c &= !mask;
    cpu.pc += 2;
    cpu.cycle(8);
    let mask = 1u8 << ((crate::defs::PRINTNUM_LEFTALIGN_F as u8) & 7);
    cpu.c &= !mask;
    cpu.pc += 2;
    cpu.cycle(8);
    let mask = 1u8 << ((crate::defs::PRINTNUM_MONEY_F as u8) & 7);
    cpu.c &= !mask;
    cpu.pc += 2;
    cpu.cycle(8);
    let mask = 1u8 << ((crate::defs::PRINTNUM_MONEY_F as u8) & 7);
    cpu.flag_z = (cpu.b & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become print_bcd_number_loop(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let mask = 1u8 << ((crate::defs::PRINTNUM_LEADINGZEROS_F as u8) & 7);
    cpu.flag_z = (cpu.b & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(12);
        become print_bcd_number_loop(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), { let value = (0xf0 as i32); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } });
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    become print_bcd_number_loop(cpu);
}

pub(crate) fn print_bcd_number_loop(cpu: &mut Cpu) {
    cpu.pc = 0x38cd;
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value.rotate_left(4);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    print_bcd_digit(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    print_bcd_digit(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_de(cpu.de().wrapping_add(1));
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
    if !cpu.flag_z {
        cpu.cycle(12);
        become print_bcd_number_loop(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let mask = 1u8 << ((crate::defs::PRINTNUM_LEADINGZEROS_F as u8) & 7);
    cpu.flag_z = (cpu.b & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become print_bcd_number_done(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let mask = 1u8 << ((crate::defs::PRINTNUM_LEFTALIGN_F as u8) & 7);
    cpu.flag_z = (cpu.b & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(12);
        become print_bcd_number_skip_left_alignment_adjustment(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    become print_bcd_number_skip_left_alignment_adjustment(cpu);
}

pub(crate) fn print_bcd_number_skip_left_alignment_adjustment(cpu: &mut Cpu) {
    cpu.pc = 0x38e4;
    let mask = 1u8 << ((crate::defs::PRINTNUM_MONEY_F as u8) & 7);
    cpu.flag_z = (cpu.b & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become print_bcd_number_skip_currency_symbol(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), { let value = (0xf0 as i32); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } });
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    become print_bcd_number_skip_currency_symbol(cpu);
}

pub(crate) fn print_bcd_number_skip_currency_symbol(cpu: &mut Cpu) {
    cpu.pc = 0x38eb;
    cpu.write_byte(cpu.hl(), { let value = (0xf6 as i32); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } });
    cpu.pc += 2;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::print_text::print_letter_delay(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    become print_bcd_number_done(cpu);
}

pub(crate) fn print_bcd_number_done(cpu: &mut Cpu) {
    cpu.pc = 0x38f1;
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub fn print_bcd_digit(cpu: &mut Cpu) {
    cpu.pc = 0x38f2;
    let value = 0b00001111;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
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
        become print_bcd_digit_zero_digit(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let mask = 1u8 << ((crate::defs::PRINTNUM_LEADINGZEROS_F as u8) & 7);
    cpu.flag_z = (cpu.b & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become print_bcd_digit_output_digit(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let mask = 1u8 << ((crate::defs::PRINTNUM_MONEY_F as u8) & 7);
    cpu.flag_z = (cpu.b & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become print_bcd_digit_skip_currency_symbol(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), { let value = (0xf0 as i32); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } });
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let mask = 1u8 << ((crate::defs::PRINTNUM_MONEY_F as u8) & 7);
    cpu.b &= !mask;
    cpu.pc += 2;
    cpu.cycle(8);
    become print_bcd_digit_skip_currency_symbol(cpu);
}

pub(crate) fn print_bcd_digit_skip_currency_symbol(cpu: &mut Cpu) {
    cpu.pc = 0x3904;
    let mask = 1u8 << ((crate::defs::PRINTNUM_LEADINGZEROS_F as u8) & 7);
    cpu.b &= !mask;
    cpu.pc += 2;
    cpu.cycle(8);
    become print_bcd_digit_output_digit(cpu);
}

pub(crate) fn print_bcd_digit_output_digit(cpu: &mut Cpu) {
    cpu.pc = 0x3906;
    let value = { let value = (0xf6 as i32); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_add(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) > 0x0f;
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.cycle(16);
    become crate::game::home::print_text::print_letter_delay(cpu);
}

pub(crate) fn print_bcd_digit_zero_digit(cpu: &mut Cpu) {
    cpu.pc = 0x390c;
    let mask = 1u8 << ((crate::defs::PRINTNUM_LEADINGZEROS_F as u8) & 7);
    cpu.flag_z = (cpu.b & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become print_bcd_digit_output_digit(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let mask = 1u8 << ((crate::defs::PRINTNUM_LEFTALIGN_F as u8) & 7);
    cpu.flag_z = (cpu.b & mask) == 0;
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
    cpu.a = { let value = (0x7f as i32); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}
