use crate::runtime::Cpu;

pub(crate) fn battle_command_beat_up(cpu: &mut Cpu) {
    cpu.pc = 0x5461;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::battle::reset_damage(cpu);
    assert_eq!(cpu.pc, next_pc);
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
        cpu.cycle(16);
        become battle_command_beat_up_enemy_beats_up(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xc66a);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((crate::defs::SUBSTATUS_IN_LOOP as u8) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(12);
        become battle_command_beat_up_next_mon(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.c = 20;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::delay::delay_frames(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc672, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xd002, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xc72d, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(12);
    become battle_command_beat_up_got_mon(cpu);
}

pub(crate) fn battle_command_beat_up_next_mon(cpu: &mut Cpu) {
    cpu.pc = 0x5482;
    cpu.a = cpu.read_byte(0xc672);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xdcd7);
    cpu.pc += 3;
    cpu.cycle(16);
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
    cpu.write_byte(0xd002, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    become battle_command_beat_up_got_mon(cpu);
}

pub(crate) fn battle_command_beat_up_got_mon(cpu: &mut Cpu) {
    cpu.pc = 0x548d;
    cpu.a = cpu.read_byte(0xd002);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xde41);
    cpu.pc += 3;
    cpu.cycle(12);
    // HARNESS-PATCH: a stale loop counter runs wCurBeatUpPartyMon past the
    // party; a nickname that would be read from state the hand-written engine
    // doesn't keep (live stack, sound, graphics) is read from zeros instead.
    // hl and a are dead after GetNickname. See docs/transpiled_code.md.
    let name = 0xde41u16.wrapping_add(u16::from(cpu.a) * 11);
    if !crate::engine::mem::glitch_range_kept(name, 11) {
        cpu.set_hl(crate::engine::mem::GLITCH_ZERO);
        cpu.a = 0;
    }
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::pokemon::get_nickname(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = crate::defs::MON_HP as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    get_beatup_mon_location(cpu);
    assert_eq!(cpu.pc, next_pc);
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
    if cpu.flag_z {
        cpu.cycle(16);
        become battle_command_beat_up_beatup_fail(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xd002);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xd0d4);
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
    cpu.set_hl(0xc63a);
    cpu.pc += 3;
    cpu.cycle(12);
    if cpu.flag_z {
        cpu.cycle(12);
        become battle_command_beat_up_active_mon(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = crate::defs::MON_STATUS as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    get_beatup_mon_location(cpu);
    assert_eq!(cpu.pc, next_pc);
    become battle_command_beat_up_active_mon(cpu);
}

pub(crate) fn battle_command_beat_up_active_mon(cpu: &mut Cpu) {
    cpu.pc = 0x54b2;
    cpu.a = cpu.read_byte(cpu.hl());
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
    if !cpu.flag_z {
        cpu.cycle(16);
        become battle_command_beat_up_beatup_fail(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc72d, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0x582d);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::battle::std_battle_textbox(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xd206);
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
    cpu.a = cpu.read_byte(0xd239);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.push_word(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = crate::defs::MON_SPECIES as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    get_beatup_mon_location(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(0xcf60, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::pokemon::get_base_data(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xd238);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.pop_word();
    cpu.set_bc(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.push_word(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = crate::defs::MON_LEVEL as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    get_beatup_mon_location(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.pop_word();
    cpu.set_bc(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xc611);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.d = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn battle_command_beat_up_enemy_beats_up(cpu: &mut Cpu) {
    cpu.pc = 0x54ef;
    cpu.a = cpu.read_byte(0xc66f);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((crate::defs::SUBSTATUS_IN_LOOP as u8) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(12);
        become battle_command_beat_up_enemy_next_mon(cpu);
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
    cpu.write_byte(0xc67a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xd002, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xc72d, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(12);
    become battle_command_beat_up_enemy_got_mon(cpu);
}

pub(crate) fn battle_command_beat_up_enemy_next_mon(cpu: &mut Cpu) {
    cpu.pc = 0x5502;
    cpu.a = cpu.read_byte(0xc67a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xd280);
    cpu.pc += 3;
    cpu.cycle(16);
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
    cpu.write_byte(0xd002, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    become battle_command_beat_up_enemy_got_mon(cpu);
}

pub(crate) fn battle_command_beat_up_enemy_got_mon(cpu: &mut Cpu) {
    cpu.pc = 0x550d;
    cpu.a = cpu.read_byte(0xd22d);
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
    if cpu.flag_z {
        cpu.cycle(12);
        become battle_command_beat_up_wild(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc2dc);
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
    if !cpu.flag_z {
        cpu.cycle(12);
        become battle_command_beat_up_link_or_tower(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xcfc0);
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
    if !cpu.flag_z {
        cpu.cycle(12);
        become battle_command_beat_up_link_or_tower(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xd002);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.b = 0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xd281);
    cpu.pc += 3;
    cpu.cycle(12);
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
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::names::get_pokemon_name(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(12);
    become battle_command_beat_up_got_enemy_nick(cpu);
}

pub(crate) fn battle_command_beat_up_link_or_tower(cpu: &mut Cpu) {
    cpu.pc = 0x5532;
    cpu.a = cpu.read_byte(0xd002);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xd3ea);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(crate::defs::NAME_LENGTH as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::array::add_n_times(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_de(0xd073);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    become battle_command_beat_up_got_enemy_nick(cpu);
}

pub(crate) fn battle_command_beat_up_got_enemy_nick(cpu: &mut Cpu) {
    cpu.pc = 0x5544;
    cpu.a = crate::defs::MON_HP as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    get_beatup_mon_location(cpu);
    assert_eq!(cpu.pc, next_pc);
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
    if cpu.flag_z {
        cpu.cycle(16);
        become battle_command_beat_up_beatup_fail(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xd002);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xc663);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.b;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl(0xd214);
    cpu.pc += 3;
    cpu.cycle(12);
    if cpu.flag_z {
        cpu.cycle(12);
        become battle_command_beat_up_active_enemy(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = crate::defs::MON_STATUS as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    get_beatup_mon_location(cpu);
    assert_eq!(cpu.pc, next_pc);
    become battle_command_beat_up_active_enemy(cpu);
}

pub(crate) fn battle_command_beat_up_active_enemy(cpu: &mut Cpu) {
    cpu.pc = 0x5560;
    cpu.a = cpu.read_byte(cpu.hl());
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
    if !cpu.flag_z {
        cpu.cycle(12);
        become battle_command_beat_up_beatup_fail(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc72d, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(12);
    become battle_command_beat_up_finish_beatup(cpu);
}

pub(crate) fn battle_command_beat_up_wild(cpu: &mut Cpu) {
    cpu.pc = 0x556b;
    cpu.a = cpu.read_byte(0xd206);
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
    cpu.set_hl(0x582d);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::battle::std_battle_textbox(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(16);
    become crate::game::engine::battle::effect_commands::enemy_attack_damage(cpu);
}

pub(crate) fn battle_command_beat_up_finish_beatup(cpu: &mut Cpu) {
    cpu.pc = 0x557d;
    cpu.set_hl(0x582d);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::battle::std_battle_textbox(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xc62c);
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
    cpu.a = cpu.read_byte(0xd239);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.push_word(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = crate::defs::MON_SPECIES as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    get_beatup_mon_location(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(0xcf60, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::pokemon::get_base_data(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xd238);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.pop_word();
    cpu.set_bc(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.push_word(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = crate::defs::MON_LEVEL as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    get_beatup_mon_location(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.pop_word();
    cpu.set_bc(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xc60a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.d = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn battle_command_beat_up_beatup_fail(cpu: &mut Cpu) {
    cpu.pc = 0x55b0;
    cpu.b = crate::defs::BUILDOPPONENTRAGE_COMMAND as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become crate::game::engine::battle::effect_commands::skip_to_battle_command(cpu);
}

pub(crate) fn battle_command_beat_up_fail_text(cpu: &mut Cpu) {
    cpu.pc = 0x55b5;
    cpu.a = cpu.read_byte(0xc72d);
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
    if !cpu.flag_z {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.cycle(16);
    become crate::game::engine::battle::effect_commands::print_but_it_failed(cpu);
}

pub(crate) fn get_beatup_mon_location(cpu: &mut Cpu) {
    cpu.pc = 0x55bd;
    cpu.push_word(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.b = 0;
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
    cpu.set_hl(0xdcdf);
    cpu.pc += 3;
    cpu.cycle(12);
    if cpu.flag_z {
        cpu.cycle(12);
        become get_beatup_mon_location_got_species(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xd288);
    cpu.pc += 3;
    cpu.cycle(12);
    become get_beatup_mon_location_got_species(cpu);
}

pub(crate) fn get_beatup_mon_location_got_species(cpu: &mut Cpu) {
    cpu.pc = 0x55cc;
    cpu.a = cpu.read_byte(0xd002);
    cpu.pc += 3;
    cpu.cycle(16);
    let lhs = cpu.hl();
    let rhs = cpu.bc();
    let (result, carry) = lhs.overflowing_add(rhs);
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0fff) + (rhs & 0x0fff) > 0x0fff;
    cpu.flag_c = carry;
    cpu.set_hl(result);
    cpu.pc += 1;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::battle::get_party_location(cpu);
    assert_eq!(cpu.pc, next_pc);
    // HARNESS-PATCH: Beat Up's party index past the party (stale loop
    // counter) can point at state the hand-written engine doesn't keep (live
    // stack, sound, graphics, OAM, I/O); read zeros there instead. See
    // docs/transpiled_code.md.
    if !crate::engine::mem::glitch_range_kept(cpu.hl(), 2) {
        cpu.set_hl(crate::engine::mem::GLITCH_ZERO);
    }
    let value = cpu.pop_word();
    cpu.set_bc(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}
