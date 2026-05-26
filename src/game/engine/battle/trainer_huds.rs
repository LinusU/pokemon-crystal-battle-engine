use crate::runtime::Cpu;

pub(crate) fn battle_start_trainer_huds(cpu: &mut Cpu) {
    cpu.pc = 0x4000;
    cpu.a = 0xe4;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_OBP0 as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    load_ball_icon_gfx(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    show_player_mons_remaining(cpu);
    assert_eq!(cpu.pc, next_pc);
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
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.cycle(16);
    become show_ot_trainer_mons_remaining(cpu);
}

pub(crate) fn enemy_switch_trainer_hud(cpu: &mut Cpu) {
    cpu.pc = 0x4012;
    cpu.a = 0xe4;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_OBP0 as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    load_ball_icon_gfx(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(16);
    become show_ot_trainer_mons_remaining(cpu);
}

pub(crate) fn show_player_mons_remaining(cpu: &mut Cpu) {
    cpu.pc = 0x401c;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    draw_player_party_icon_hud_border(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xdd01);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xdcd7);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    stage_ball_tiles_data(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = { let value = ((12 as i32) * (crate::defs::TILE_WIDTH as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xcfc4);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = crate::defs::TILE_WIDTH as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xd003, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xc400);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become load_trainer_hud_oam(cpu);
}

pub(crate) fn show_ot_trainer_mons_remaining(cpu: &mut Cpu) {
    cpu.pc = 0x403a;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    draw_enemy_hud_border(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xd2aa);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xd280);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    stage_ball_tiles_data(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xcfc4);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = { let value = ((9 as i32) * (crate::defs::TILE_WIDTH as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), { let value = ((4 as i32) * (crate::defs::TILE_WIDTH as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } });
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = { let value = (-(crate::defs::TILE_WIDTH as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xd003, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl({ const N: i32 = ((0xc400 as i32) + ((crate::defs::PARTY_LENGTH as u16 as i32) * (crate::defs::OBJ_SIZE as u16 as i32))); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become load_trainer_hud_oam(cpu);
}

pub(crate) fn stage_ball_tiles_data(cpu: &mut Cpu) {
    cpu.pc = 0x4059;
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.set_de(0xd1ea);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.c = crate::defs::PARTY_LENGTH as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x34;
    cpu.pc += 2;
    cpu.cycle(8);
    become stage_ball_tiles_data_loop1(cpu);
}

pub(crate) fn stage_ball_tiles_data_loop1(cpu: &mut Cpu) {
    cpu.pc = 0x4062;
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
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
        become stage_ball_tiles_data_loop1(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.set_de(0xd1ea);
    cpu.pc += 3;
    cpu.cycle(12);
    become stage_ball_tiles_data_loop2(cpu);
}

pub(crate) fn stage_ball_tiles_data_loop2(cpu: &mut Cpu) {
    cpu.pc = 0x406b;
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    stage_ball_tiles_data_get_hud_tile(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(12);
        become stage_ball_tiles_data_loop2(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn stage_ball_tiles_data_get_hud_tile(cpu: &mut Cpu) {
    cpu.pc = 0x4075;
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
    if !cpu.flag_z {
        cpu.cycle(12);
        become stage_ball_tiles_data_got_hp(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
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
    cpu.b = 0x33;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become stage_ball_tiles_data_fainted(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become stage_ball_tiles_data_got_hp(cpu);
}

pub(crate) fn stage_ball_tiles_data_got_hp(cpu: &mut Cpu) {
    cpu.pc = 0x407f;
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
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
    cpu.b = 0x32;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(12);
        become stage_ball_tiles_data_load(cpu);
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
    cpu.cycle(12);
    become stage_ball_tiles_data_load(cpu);
}

pub(crate) fn stage_ball_tiles_data_fainted(cpu: &mut Cpu) {
    cpu.pc = 0x408b;
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    become stage_ball_tiles_data_load(cpu);
}

pub(crate) fn stage_ball_tiles_data_load(cpu: &mut Cpu) {
    cpu.pc = 0x408e;
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_bc({ const N: i32 = (crate::defs::PARTYMON_STRUCT_LENGTH as u16 as i32) + (crate::defs::MON_HP as u16 as i32) - (crate::defs::MON_STATUS as u16 as i32); N as u16 });
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
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn draw_player_hud_border(cpu: &mut Cpu) {
    cpu.pc = 0x4095;
    cpu.set_hl(0x40a9);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xd004);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc({ const N: i32 = (0x40ad as i32) - (0x40a9 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = ((((10 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (18 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(-1i32 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(12);
    become place_hud_border_tiles(cpu);
}

pub(crate) fn draw_player_hud_border_tiles_end(cpu: &mut Cpu) {
    cpu.pc = 0x40ad;
    become draw_player_party_icon_hud_border(cpu);
}

pub(crate) fn draw_player_party_icon_hud_border(cpu: &mut Cpu) {
    cpu.pc = 0x40ad;
    cpu.set_hl(0x40c1);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xd004);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc({ const N: i32 = (0x40c5 as i32) - (0x40c1 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = ((((10 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (18 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(-1i32 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(12);
    become place_hud_border_tiles(cpu);
}

pub(crate) fn draw_player_party_icon_hud_border_tiles_end(cpu: &mut Cpu) {
    cpu.pc = 0x40c5;
    become draw_enemy_hud_border(cpu);
}

pub(crate) fn draw_enemy_hud_border(cpu: &mut Cpu) {
    cpu.pc = 0x40c5;
    cpu.set_hl(0x40ed);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xd004);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc({ const N: i32 = (0x40f1 as i32) - (0x40ed as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = ((((2 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (1 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(1 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    place_hud_border_tiles(cpu);
    assert_eq!(cpu.pc, next_pc);
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
    if !cpu.flag_z {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xd204);
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
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::pokedex_flags::check_caught_mon(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_z {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl({ const N: i32 = ((((1 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (1 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.write_byte(cpu.hl(), 0x5d);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn draw_enemy_hud_border_tiles_end(cpu: &mut Cpu) {
    cpu.pc = 0x40f1;
    become place_hud_border_tiles(cpu);
}

pub(crate) fn place_hud_border_tiles(cpu: &mut Cpu) {
    cpu.pc = 0x40f1;
    cpu.a = cpu.read_byte({ const N: i32 = (0xd004 as i32) + (0 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_bc(crate::defs::SCREEN_WIDTH as u16);
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
    cpu.a = cpu.read_byte({ const N: i32 = (0xd004 as i32) + (1 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.b = 8;
    cpu.pc += 2;
    cpu.cycle(8);
    become place_hud_border_tiles_loop(cpu);
}

pub(crate) fn place_hud_border_tiles_loop(cpu: &mut Cpu) {
    cpu.pc = 0x40ff;
    let lhs = cpu.hl();
    let rhs = cpu.de();
    let (result, carry) = lhs.overflowing_add(rhs);
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0fff) + (rhs & 0x0fff) > 0x0fff;
    cpu.flag_c = carry;
    cpu.set_hl(result);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte({ const N: i32 = (0xd004 as i32) + (3 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(cpu.hl(), cpu.a);
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
        become place_hud_border_tiles_loop(cpu);
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
    cpu.a = cpu.read_byte({ const N: i32 = (0xd004 as i32) + (2 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn link_battle_trainer_huds(cpu: &mut Cpu) {
    cpu.pc = 0x410d;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    load_ball_icon_gfx(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xdd01);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xdcd7);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    stage_ball_tiles_data(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xcfc4);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = { let value = ((10 as i32) * (crate::defs::TILE_WIDTH as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), { let value = ((8 as i32) * (crate::defs::TILE_WIDTH as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } });
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = crate::defs::TILE_WIDTH as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xd003, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xc400);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    load_trainer_hud_oam(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xd2aa);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xd280);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    stage_ball_tiles_data(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xcfc4);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = { let value = ((10 as i32) * (crate::defs::TILE_WIDTH as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), { let value = ((13 as i32) * (crate::defs::TILE_WIDTH as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } });
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.set_hl({ const N: i32 = ((0xc400 as i32) + ((crate::defs::PARTY_LENGTH as u16 as i32) * (crate::defs::OBJ_SIZE as u16 as i32))); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become load_trainer_hud_oam(cpu);
}

pub(crate) fn load_trainer_hud_oam(cpu: &mut Cpu) {
    cpu.pc = 0x4143;
    cpu.set_de(0xd1ea);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.c = crate::defs::PARTY_LENGTH as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    become load_trainer_hud_oam_loop(cpu);
}

pub(crate) fn load_trainer_hud_oam_loop(cpu: &mut Cpu) {
    cpu.pc = 0x4148;
    cpu.a = cpu.read_byte(0xcfc5);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xcfc4);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = crate::defs::PAL_BATTLE_OB_YELLOW as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xcfc4);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xd003);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.b;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_add(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) > 0x0f;
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcfc4, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
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
        become load_trainer_hud_oam_loop(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn load_ball_icon_gfx(cpu: &mut Cpu) {
    cpu.pc = 0x4165;
    cpu.set_de(0x4172);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl({ const N: i32 = ((0x8000 as i32) + ((crate::defs::TILE_SIZE as u16 as i32) * (0x31 as i32))); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc({ const N: i32 = ((((0x0b as i32) & (0xff as i32)) << (8 as i32)) | ((4 as i32) & (0xff as i32))); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::gfx::get2bpp_via_hdma(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn _show_link_battle_participants(cpu: &mut Cpu) {
    cpu.pc = 0x41b2;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::tilemap::clear_bg_palettes(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::gfx::load_fonts_extra(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = ((((3 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (2 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = 9;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.c = 14;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::textbox(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = ((((5 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (4 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xd47d);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::place_string(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = ((((10 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (4 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xd26b);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::place_string(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = ((((8 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (9 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = { let value = (0x69 as i32); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), { let value = (0x6a as i32); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } });
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = 0x0b;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x410d);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.b = crate::defs::SCGB_DIPLOMA as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::tilemap::get_sgb_layout(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::tilemap::set_default_bgp_and_obp(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0xe4;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_OBP0 as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}
