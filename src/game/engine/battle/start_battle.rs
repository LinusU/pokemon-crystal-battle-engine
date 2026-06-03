use crate::runtime::Cpu;

pub(crate) fn show_link_battle_participants(cpu: &mut Cpu) {
    cpu.pc = 0x6e18;
    // HARNESS-PATCH: skip the cosmetic "<player> vs <rival>" participant screen
    // in a headless link battle. It renders the two link players' names, but the
    // link name buffers (wOTPlayerName etc.) aren't seeded headless, so the text
    // engine writes its glyphs to a stale destination and scribbles over battle
    // WRAM (notably wCurOTMon at 0xc663). There's no screen to show it on anyway.
    // Mirrors the wLinkMode == 0 early return just below. See docs/transpiled_code.md.
    if cpu.link_battle {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
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
    if cpu.flag_z {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 0x0b;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x41b2);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.c = 150;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::delay::delay_frames(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::clear_tilemap(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::clear_sprites::clear_sprites(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn find_first_alive_mon_and_start_battle(cpu: &mut Cpu) {
    cpu.pc = 0x6e2f;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xffde, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::delay::delay_frame(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.b = crate::defs::PARTY_LENGTH as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xdd01);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de({ const N: i32 = (crate::defs::PARTYMON_STRUCT_LENGTH as u16 as i32) - (1 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    become find_first_alive_mon_and_start_battle_loop(cpu);
}

pub(crate) fn find_first_alive_mon_and_start_battle_loop(cpu: &mut Cpu) {
    cpu.pc = 0x6e3d;
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
    if !cpu.flag_z {
        cpu.cycle(12);
        become find_first_alive_mon_and_start_battle_okay(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let lhs = cpu.hl();
    let rhs = cpu.de();
    let (result, carry) = lhs.overflowing_add(rhs);
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0fff) + (rhs & 0x0fff) > 0x0fff;
    cpu.flag_c = carry;
    cpu.set_hl(result);
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
        become find_first_alive_mon_and_start_battle_loop(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become find_first_alive_mon_and_start_battle_okay(cpu);
}

pub(crate) fn find_first_alive_mon_and_start_battle_okay(cpu: &mut Cpu) {
    cpu.pc = 0x6e45;
    cpu.set_de({ const N: i32 = (crate::defs::MON_LEVEL as u16 as i32) - (crate::defs::MON_HP as u16 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    let lhs = cpu.hl();
    let rhs = cpu.de();
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
    cpu.write_byte(0xc639, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = { let value = (((0x4607 as i32) - (0x456b as i32)) / (3 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::predef::predef(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x0f;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x6d9f);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xffd4, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::clear_sprites::clear_sprites(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::clear_tilemap(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xffd4, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.write_byte(0xffd2, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.write_byte(crate::defs::R_WY as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.write_byte(0xffde, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn play_battle_music(cpu: &mut Cpu) {
    cpu.pc = 0x6e6c;
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.push_word(cpu.de());
    cpu.pc += 1;
    cpu.cycle(16);
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
    cpu.write_byte(0xc2a7, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_de(crate::defs::MUSIC_NONE as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::audio::play_music(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::delay::delay_frame(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::audio::max_volume(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xd230);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = crate::defs::BATTLETYPE_SUICUNE as u8;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_de(crate::defs::MUSIC_SUICUNE_BATTLE as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    if cpu.flag_z {
        cpu.cycle(16);
        become play_battle_music_done(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    let value = crate::defs::BATTLETYPE_ROAMING as u8;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(16);
        become play_battle_music_done(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xd22f);
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
        become play_battle_music_trainermusic(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x72;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x6ea1);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.e;
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
        become play_battle_music_kantowild(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_de(crate::defs::MUSIC_JOHTO_WILD_BATTLE as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xd269);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = crate::defs::NITE_F as u8;
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
        become play_battle_music_done(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_de(crate::defs::MUSIC_JOHTO_WILD_BATTLE_NIGHT as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(12);
    become play_battle_music_done(cpu);
}

pub(crate) fn play_battle_music_kantowild(cpu: &mut Cpu) {
    cpu.pc = 0x6eae;
    cpu.set_de(crate::defs::MUSIC_KANTO_WILD_BATTLE as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(12);
    become play_battle_music_done(cpu);
}

pub(crate) fn play_battle_music_trainermusic(cpu: &mut Cpu) {
    cpu.pc = 0x6eb3;
    cpu.set_de(crate::defs::MUSIC_CHAMPION_BATTLE as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let value = crate::defs::CHAMPION as u8;
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
        become play_battle_music_done(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = crate::defs::RED as u8;
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
        become play_battle_music_done(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_de(crate::defs::MUSIC_ROCKET_BATTLE as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let value = crate::defs::GRUNTM as u8;
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
        become play_battle_music_done(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = crate::defs::GRUNTF as u8;
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
        become play_battle_music_done(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_de(crate::defs::MUSIC_KANTO_GYM_LEADER_BATTLE as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0x0f;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x5123);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.cycle(12);
        become play_battle_music_done(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_de(crate::defs::MUSIC_JOHTO_GYM_LEADER_BATTLE as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0x0f;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x5128);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.cycle(12);
        become play_battle_music_done(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_de(crate::defs::MUSIC_RIVAL_BATTLE as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xd22f);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = crate::defs::RIVAL1 as u8;
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
        become play_battle_music_done(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = crate::defs::RIVAL2 as u8;
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
        become play_battle_music_othertrainer(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xd231);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = crate::defs::RIVAL2_2_CHIKORITA as u8;
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
        become play_battle_music_done(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_de(crate::defs::MUSIC_CHAMPION_BATTLE as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(12);
    become play_battle_music_done(cpu);
}

pub(crate) fn play_battle_music_othertrainer(cpu: &mut Cpu) {
    cpu.pc = 0x6ef9;
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
        become play_battle_music_johtotrainer(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x72;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x6ea1);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.e;
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
        become play_battle_music_kantotrainer(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become play_battle_music_johtotrainer(cpu);
}

pub(crate) fn play_battle_music_johtotrainer(cpu: &mut Cpu) {
    cpu.pc = 0x6f09;
    cpu.set_de(crate::defs::MUSIC_JOHTO_TRAINER_BATTLE as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(12);
    become play_battle_music_done(cpu);
}

pub(crate) fn play_battle_music_kantotrainer(cpu: &mut Cpu) {
    cpu.pc = 0x6f0e;
    cpu.set_de(crate::defs::MUSIC_KANTO_TRAINER_BATTLE as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    become play_battle_music_done(cpu);
}

pub(crate) fn play_battle_music_done(cpu: &mut Cpu) {
    cpu.pc = 0x6f11;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::audio::play_music(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_bc(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let value = cpu.pop_word();
    cpu.set_de(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn clear_battle_ram(cpu: &mut Cpu) {
    cpu.pc = 0x6f18;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xd0ec, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xd0ee, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xd0d8);
    cpu.pc += 3;
    cpu.cycle(12);
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
    cpu.write_byte(0xd0e4, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xc666, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xc62c, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xc664, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xd0d4, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xd232, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xd841, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xc6dd, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xc6dc, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xd1e8, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xcd99);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(0xc632);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(0xd20c);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(0xc608);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc({ const N: i32 = (0xc741 as i32) - (0xc608 as i32); N as u16 });
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
    cpu.set_hl(0x5867);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0x0f;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::menu::clear_window_data(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xffd6);
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
    cpu.write_byte(cpu.hl(), (((0x9800u32) >> 8) & 0x00ff) as u8);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}
