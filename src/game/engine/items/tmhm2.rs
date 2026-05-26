use crate::runtime::Cpu;

pub(crate) fn can_learn_tmhm_move(cpu: &mut Cpu) {
    cpu.pc = 0x5639;
    cpu.a = cpu.read_byte(0xd108);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xcf60, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::pokemon::get_base_data(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xd24e);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xd262);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.c = 0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x567a);
    cpu.pc += 3;
    cpu.cycle(12);
    become can_learn_tmhm_move_loop(cpu);
}

pub(crate) fn can_learn_tmhm_move_loop(cpu: &mut Cpu) {
    cpu.pc = 0x564f;
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
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
        become can_learn_tmhm_move_end(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.b;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become can_learn_tmhm_move_found(cpu);
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
    cpu.cycle(12);
    become can_learn_tmhm_move_loop(cpu);
}

pub(crate) fn can_learn_tmhm_move_found(cpu: &mut Cpu) {
    cpu.pc = 0x5659;
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.b = crate::defs::CHECK_FLAG as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.push_word(cpu.de());
    cpu.pc += 1;
    cpu.cycle(16);
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
    let value = cpu.pop_word();
    cpu.set_de(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn can_learn_tmhm_move_end(cpu: &mut Cpu) {
    cpu.pc = 0x5666;
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.c = 0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn get_tmhm_move(cpu: &mut Cpu) {
    cpu.pc = 0x566a;
    cpu.a = cpu.read_byte(0xd265);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl(0x567a);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = 0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
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
    cpu.write_byte(0xd265, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}
