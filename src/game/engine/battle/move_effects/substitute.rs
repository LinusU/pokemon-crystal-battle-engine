use crate::runtime::Cpu;

pub(crate) fn battle_command_substitute(cpu: &mut Cpu) {
    cpu.pc = 0x6e7c;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::engine::battle::effect_commands::battle_command_move_delay(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xc63e);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xc6df);
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
        become battle_command_substitute_got_hp(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xd218);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xc6e0);
    cpu.pc += 3;
    cpu.cycle(12);
    become battle_command_substitute_got_hp(cpu);
}

pub(crate) fn battle_command_substitute_got_hp(cpu: &mut Cpu) {
    cpu.pc = 0x6e90;
    cpu.a = crate::defs::BATTLE_VARS_SUBSTATUS4 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::battle_vars::get_battle_var(cpu);
    assert_eq!(cpu.pc, next_pc);
    let mask = 1u8 << ((crate::defs::SUBSTATUS_SUBSTITUTE as u8) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(12);
        become battle_command_substitute_already_has_sub(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.b = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value >> 1;
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x01 != 0;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.b;
    let carry_in = u8::from(cpu.flag_c) << 7;
    let result = (value >> 1) | carry_in;
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x01 != 0;
    cpu.b = result;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value >> 1;
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x01 != 0;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.b;
    let carry_in = u8::from(cpu.flag_c) << 7;
    let result = (value >> 1) | carry_in;
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x01 != 0;
    cpu.b = result;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
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
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0;
    let carry_in = u8::from(cpu.flag_c);
    let lhs = cpu.a;
    let (partial, borrow1) = lhs.overflowing_sub(value);
    let (result, borrow2) = partial.overflowing_sub(carry_in);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < ((value & 0x0f) + carry_in);
    cpu.flag_c = borrow1 || borrow2;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.d = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_c {
        cpu.cycle(12);
        become battle_command_substitute_too_weak_to_sub(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.e;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become battle_command_substitute_too_weak_to_sub(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.d);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.e);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = crate::defs::BATTLE_VARS_SUBSTATUS4 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::battle_vars::get_battle_var_addr(cpu);
    assert_eq!(cpu.pc, next_pc);
    let mask = 1u8 << ((crate::defs::SUBSTATUS_SUBSTITUTE as u8) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value |= mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.set_hl(0xc730);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xc72e);
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
        become battle_command_substitute_player(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xc731);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xc72f);
    cpu.pc += 3;
    cpu.cycle(12);
    become battle_command_substitute_player(cpu);
}

pub(crate) fn battle_command_substitute_player(cpu: &mut Cpu) {
    cpu.pc = 0x6ecf;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::engine::battle::effect_commands::_check_battle_scene(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.cycle(12);
        become battle_command_substitute_no_anim(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcfca, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte({ const N: i32 = (0xcfc2 as i32) + (1 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xc689, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = crate::defs::SUBSTITUTE as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::engine::battle::effect_commands::load_anim(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(12);
    become battle_command_substitute_finish(cpu);
}

pub(crate) fn battle_command_substitute_no_anim(cpu: &mut Cpu) {
    cpu.pc = 0x6ee8;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::engine::battle::effect_commands::battle_command_raise_sub_no_anim(cpu);
    assert_eq!(cpu.pc, next_pc);
    become battle_command_substitute_finish(cpu);
}

pub(crate) fn battle_command_substitute_finish(cpu: &mut Cpu) {
    cpu.pc = 0x6eeb;
    cpu.set_hl(0x5362);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::battle::std_battle_textbox(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(16);
    become crate::game::home::battle::refresh_battle_huds(cpu);
}

pub(crate) fn battle_command_substitute_already_has_sub(cpu: &mut Cpu) {
    cpu.pc = 0x6ef4;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::engine::battle::effect_commands::check_user_is_charging(cpu);
    assert_eq!(cpu.pc, next_pc);
    if !cpu.flag_z {
        let next_pc = cpu.pc + 3;
        cpu.push_word(next_pc);
        cpu.cycle(24);
        crate::game::engine::battle::effect_commands::battle_command_raise_sub(cpu);
        assert_eq!(cpu.pc, next_pc);
    } else {
        cpu.pc += 3;
        cpu.cycle(12);
    }
    cpu.set_hl(0x5378);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(12);
    become battle_command_substitute_jp_stdbattletextbox(cpu);
}

pub(crate) fn battle_command_substitute_too_weak_to_sub(cpu: &mut Cpu) {
    cpu.pc = 0x6eff;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::engine::battle::effect_commands::check_user_is_charging(cpu);
    assert_eq!(cpu.pc, next_pc);
    if !cpu.flag_z {
        let next_pc = cpu.pc + 3;
        cpu.push_word(next_pc);
        cpu.cycle(24);
        crate::game::engine::battle::effect_commands::battle_command_raise_sub(cpu);
        assert_eq!(cpu.pc, next_pc);
    } else {
        cpu.pc += 3;
        cpu.cycle(12);
    }
    cpu.set_hl(0x538d);
    cpu.pc += 3;
    cpu.cycle(12);
    become battle_command_substitute_jp_stdbattletextbox(cpu);
}

pub(crate) fn battle_command_substitute_jp_stdbattletextbox(cpu: &mut Cpu) {
    cpu.pc = 0x6f08;
    cpu.cycle(16);
    become crate::game::home::battle::std_battle_textbox(cpu);
}
