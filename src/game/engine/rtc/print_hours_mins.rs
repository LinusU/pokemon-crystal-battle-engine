use crate::runtime::Cpu;

pub(crate) fn print_five_digit_number(cpu: &mut Cpu) {
    cpu.pc = 0x56a9;
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.b = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.push_word(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.push_word(cpu.de());
    cpu.pc += 1;
    cpu.cycle(16);
    let offset = (2 as u8) as i8;
    let unsigned_offset = offset as u8;
    let lhs = cpu.sp;
    let result = lhs.wrapping_add_signed(i16::from(offset));
    cpu.flag_z = false;
    cpu.flag_n = false;
    cpu.flag_h = ((lhs & 0x000f) + u16::from(unsigned_offset & 0x0f)) > 0x000f;
    cpu.flag_c = ((lhs & 0x00ff) + u16::from(unsigned_offset)) > 0x00ff;
    cpu.set_hl(result);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.d = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.e = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.set_bc(0x8205);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::print_text::print_num(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_bc(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn print_hours_mins(cpu: &mut Cpu) {
    cpu.pc = 0x56bb;
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = 12;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    if cpu.flag_c {
        cpu.cycle(12);
        become print_hours_mins_am(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become print_hours_mins_pm(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 12;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become print_hours_mins_pm(cpu);
}

pub(crate) fn print_hours_mins_am(cpu: &mut Cpu) {
    cpu.pc = 0x56c7;
    let value = cpu.a;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(12);
        become print_hours_mins_pm(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 12;
    cpu.pc += 2;
    cpu.cycle(8);
    become print_hours_mins_pm(cpu);
}

pub(crate) fn print_hours_mins_pm(cpu: &mut Cpu) {
    cpu.pc = 0x56cc;
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.push_word(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);
    let offset = (1 as u8) as i8;
    let unsigned_offset = offset as u8;
    let lhs = cpu.sp;
    let result = lhs.wrapping_add_signed(i16::from(offset));
    cpu.flag_z = false;
    cpu.flag_n = false;
    cpu.flag_h = ((lhs & 0x000f) + u16::from(unsigned_offset & 0x0f)) > 0x000f;
    cpu.flag_c = ((lhs & 0x00ff) + u16::from(unsigned_offset)) > 0x00ff;
    cpu.set_hl(result);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.push_word(cpu.de());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    let value = cpu.pop_word();
    cpu.set_de(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte(cpu.hl(), { let value = (0x7f as i32); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } });
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.set_bc(0x0102);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::print_text::print_num(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.write_byte(cpu.hl(), { let value = (0x9c as i32); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } });
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.d = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.e = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    let offset = (0 as u8) as i8;
    let unsigned_offset = offset as u8;
    let lhs = cpu.sp;
    let result = lhs.wrapping_add_signed(i16::from(offset));
    cpu.flag_z = false;
    cpu.flag_n = false;
    cpu.flag_h = ((lhs & 0x000f) + u16::from(unsigned_offset & 0x0f)) > 0x000f;
    cpu.flag_c = ((lhs & 0x00ff) + u16::from(unsigned_offset)) > 0x00ff;
    cpu.set_hl(result);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.push_word(cpu.de());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    let value = cpu.pop_word();
    cpu.set_de(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.set_bc(0x8102);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::print_text::print_num(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_bc(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.set_de(0x56fc);
    cpu.pc += 3;
    cpu.cycle(12);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    if cpu.flag_c {
        cpu.cycle(12);
        become print_hours_mins_place_am_pm(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_de(0x56ff);
    cpu.pc += 3;
    cpu.cycle(12);
    become print_hours_mins_place_am_pm(cpu);
}

pub(crate) fn print_hours_mins_place_am_pm(cpu: &mut Cpu) {
    cpu.pc = 0x56f7;
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::place_string(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

