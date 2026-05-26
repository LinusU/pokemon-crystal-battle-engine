use crate::runtime::Cpu;

pub(crate) fn battle_command_metronome(cpu: &mut Cpu) {
    cpu.pc = 0x7418;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::engine::battle::effect_commands::clear_last_move(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::engine::battle::effect_commands::check_user_is_charging(cpu);
    assert_eq!(cpu.pc, next_pc);
    if !cpu.flag_z {
        cpu.cycle(12);
        become battle_command_metronome_charging(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc689);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::engine::battle::effect_commands::battle_command_lower_sub(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte(0xc689, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    become battle_command_metronome_charging(cpu);
}

pub(crate) fn battle_command_metronome_charging(cpu: &mut Cpu) {
    cpu.pc = 0x742b;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::engine::battle::effect_commands::load_move_anim(cpu);
    assert_eq!(cpu.pc, next_pc);
    become battle_command_metronome_get_move(cpu);
}

pub(crate) fn battle_command_metronome_get_move(cpu: &mut Cpu) {
    cpu.pc = 0x742e;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::random::battle_random(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = { let value = ((crate::defs::NUM_ATTACKS as i32) + (1 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
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
        become battle_command_metronome_get_move(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.set_de(1 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0x7454);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::array::is_in_array(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_bc(value);
    cpu.pc += 1;
    cpu.cycle(12);
    if cpu.flag_c {
        cpu.cycle(12);
        become battle_command_metronome_get_move(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::engine::battle::effect_commands::check_user_move(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_z {
        cpu.cycle(12);
        become battle_command_metronome_get_move(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = crate::defs::BATTLE_VARS_MOVE as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::battle_vars::get_battle_var_addr(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.write_byte(cpu.hl(), cpu.b);
    cpu.pc += 1;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::engine::battle::effect_commands::update_move_data(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(16);
    become crate::game::engine::battle::effect_commands::reset_turn(cpu);
}
