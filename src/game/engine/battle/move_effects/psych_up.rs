use crate::runtime::Cpu;

pub(crate) fn battle_command_psych_up(cpu: &mut Cpu) {
    cpu.pc = 0x7c55;
    cpu.set_hl(0xc6d4);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xc6cc);
    cpu.pc += 3;
    cpu.cycle(12);
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
    if cpu.flag_z {
        cpu.cycle(12);
        become battle_command_psych_up_pointers_correct(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.h = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.l = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.pop_word();
    cpu.set_de(value);
    cpu.pc += 1;
    cpu.cycle(12);
    become battle_command_psych_up_pointers_correct(cpu);
}

pub(crate) fn battle_command_psych_up_pointers_correct(cpu: &mut Cpu) {
    cpu.pc = 0x7c64;
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.b = crate::defs::NUM_LEVEL_STATS as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    become battle_command_psych_up_loop(cpu);
}

pub(crate) fn battle_command_psych_up_loop(cpu: &mut Cpu) {
    cpu.pc = 0x7c67;
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = crate::defs::BASE_STAT_LEVEL as u8;
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
        become battle_command_psych_up_break(cpu);
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
        become battle_command_psych_up_loop(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::engine::battle::effect_commands::animate_failed_move(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(16);
    become crate::game::engine::battle::effect_commands::print_but_it_failed(cpu);
}

pub(crate) fn battle_command_psych_up_break(cpu: &mut Cpu) {
    cpu.pc = 0x7c76;
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.b = crate::defs::NUM_LEVEL_STATS as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    become battle_command_psych_up_loop2(cpu);
}

pub(crate) fn battle_command_psych_up_loop2(cpu: &mut Cpu) {
    cpu.pc = 0x7c79;
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
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
        become battle_command_psych_up_loop2(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
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
        cpu.cycle(12);
        become battle_command_psych_up_calc_enemy_stats(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::engine::battle::effect_commands::calc_player_stats(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(12);
    become battle_command_psych_up_merge(cpu);
}

pub(crate) fn battle_command_psych_up_calc_enemy_stats(cpu: &mut Cpu) {
    cpu.pc = 0x7c89;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::engine::battle::effect_commands::calc_enemy_stats(cpu);
    assert_eq!(cpu.pc, next_pc);
    become battle_command_psych_up_merge(cpu);
}

pub(crate) fn battle_command_psych_up_merge(cpu: &mut Cpu) {
    cpu.pc = 0x7c8c;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::engine::battle::effect_commands::animate_current_move(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0x57f6);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become crate::game::home::battle::std_battle_textbox(cpu);
}
