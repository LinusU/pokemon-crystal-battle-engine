use crate::runtime::Cpu;

pub(crate) fn get_mobile_ot_trainer_class(cpu: &mut Cpu) {
    cpu.pc = 0x6929;
    cpu.h = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.l = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    get_mobile_ot_trainer_class_get_mobile_ot_trainer_class(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn get_mobile_ot_trainer_class_get_mobile_ot_trainer_class(cpu: &mut Cpu) {
    cpu.pc = 0x6930;
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.read_byte(cpu.hl());
    cpu.a ^= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become get_mobile_ot_trainer_class_skip_male_trainers(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.c;
    let result = value >> 1;
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x01 != 0;
    cpu.c = result;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.c;
    let result = value >> 1;
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x01 != 0;
    cpu.c = result;
    cpu.pc += 2;
    cpu.cycle(8);
    become get_mobile_ot_trainer_class_male_trainer_loop(cpu);
}

pub(crate) fn get_mobile_ot_trainer_class_male_trainer_loop(cpu: &mut Cpu) {
    cpu.pc = 0x6939;
    let value = cpu.c;
    let result = value >> 1;
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x01 != 0;
    cpu.c = result;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = { let value = (((0x6976 as i32) - (0x695d as i32)) - (1 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_c {
        cpu.cycle(12);
        become get_mobile_ot_trainer_class_male_trainer_loop(cpu);
    }
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
    become get_mobile_ot_trainer_class_skip_male_trainers(cpu);
}

pub(crate) fn get_mobile_ot_trainer_class_skip_male_trainers(cpu: &mut Cpu) {
    cpu.pc = 0x6941;
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = crate::defs::FEMALE as u8;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x695d);
    cpu.pc += 3;
    cpu.cycle(12);
    if !cpu.flag_z {
        cpu.cycle(12);
        become get_mobile_ot_trainer_class_finished(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x6976);
    cpu.pc += 3;
    cpu.cycle(12);
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
        become get_mobile_ot_trainer_class_finished(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become get_mobile_ot_trainer_class_female_trainer_loop(cpu);
}

pub(crate) fn get_mobile_ot_trainer_class_female_trainer_loop(cpu: &mut Cpu) {
    cpu.pc = 0x6950;
    let value = cpu.c;
    let result = value >> 1;
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x01 != 0;
    cpu.c = result;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = { let value = (((0x6980 as i32) - (0x6976 as i32)) - (1 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_c {
        cpu.cycle(12);
        become get_mobile_ot_trainer_class_female_trainer_loop(cpu);
    }
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
    become get_mobile_ot_trainer_class_finished(cpu);
}

pub(crate) fn get_mobile_ot_trainer_class_finished(cpu: &mut Cpu) {
    cpu.pc = 0x6958;
    cpu.b = 0x0;
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
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}
