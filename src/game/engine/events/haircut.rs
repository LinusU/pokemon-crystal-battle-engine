use crate::runtime::Cpu;

pub(crate) fn bills_grandfather(cpu: &mut Cpu) {
    cpu.pc = 0x73f7;
    cpu.a = 0x14;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x4000);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.cycle(12);
        become bills_grandfather_cancel(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xd108);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xc2dd, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xd265, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::names::get_pokemon_name(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(16);
    become copy_pokemon_name_buffer1_buffer3(cpu);
}

pub(crate) fn bills_grandfather_cancel(cpu: &mut Cpu) {
    cpu.pc = 0x740e;
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

pub(crate) fn older_haircut_brother(cpu: &mut Cpu) {
    cpu.pc = 0x7413;
    cpu.set_hl(0x7459);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(12);
    become haircut_or_grooming(cpu);
}

pub(crate) fn younger_haircut_brother(cpu: &mut Cpu) {
    cpu.pc = 0x7418;
    cpu.set_hl(0x7462);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(12);
    become haircut_or_grooming(cpu);
}

pub(crate) fn daisys_grooming(cpu: &mut Cpu) {
    cpu.pc = 0x741d;
    cpu.set_hl(0x746b);
    cpu.pc += 3;
    cpu.cycle(12);
    become haircut_or_grooming(cpu);
}

pub(crate) fn haircut_or_grooming(cpu: &mut Cpu) {
    cpu.pc = 0x7420;
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = 0x14;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x4000);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    if cpu.flag_c {
        cpu.cycle(12);
        become haircut_or_grooming_nope(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xd108);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = crate::defs::EGG as u8;
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
        become haircut_or_grooming_egg(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::pokemon::get_cur_nickname(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    copy_pokemon_name_buffer1_buffer3(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::random::random(cpu);
    assert_eq!(cpu.pc, next_pc);
    become haircut_or_grooming_loop(cpu);
}

pub(crate) fn haircut_or_grooming_loop(cpu: &mut Cpu) {
    cpu.pc = 0x743c;
    let value = cpu.read_byte(cpu.hl());
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(8);
    if cpu.flag_c {
        cpu.cycle(12);
        become haircut_or_grooming_ok(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.cycle(12);
    become haircut_or_grooming_loop(cpu);
}

pub(crate) fn haircut_or_grooming_ok(cpu: &mut Cpu) {
    cpu.pc = 0x7444;
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(0xc2dd, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.c = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::engine::events::happiness_egg::change_happiness(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn haircut_or_grooming_nope(cpu: &mut Cpu) {
    cpu.pc = 0x744e;
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

pub(crate) fn haircut_or_grooming_egg(cpu: &mut Cpu) {
    cpu.pc = 0x7453;
    cpu.a = 1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc2dd, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn copy_pokemon_name_buffer1_buffer3(cpu: &mut Cpu) {
    cpu.pc = 0x746e;
    cpu.set_hl(0xd073);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xd099);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(crate::defs::MON_NAME_LENGTH as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become crate::game::home::copy::copy_bytes(cpu);
}

pub(crate) fn dummy_predef1(cpu: &mut Cpu) {
    cpu.pc = 0x747a;
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}
