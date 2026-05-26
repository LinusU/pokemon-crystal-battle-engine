use crate::runtime::Cpu;

pub(crate) fn beta_load_player_trainer_class(cpu: &mut Cpu) {
    cpu.pc = 0x4248;
    cpu.c = crate::defs::CAL as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xd472);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((crate::defs::PLAYERGENDER_FEMALE_F as u8) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become beta_load_player_trainer_class_got_class(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.c = crate::defs::KAREN as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    become beta_load_player_trainer_class_got_class(cpu);
}

pub(crate) fn beta_load_player_trainer_class_got_class(cpu: &mut Cpu) {
    cpu.pc = 0x4253;
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xd233, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn move_player_pic_right(cpu: &mut Cpu) {
    cpu.pc = 0x4258;
    cpu.set_hl({ const N: i32 = ((((4 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (6 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(1 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(12);
    become move_player_pic(cpu);
}

pub(crate) fn move_player_pic_left(cpu: &mut Cpu) {
    cpu.pc = 0x4260;
    cpu.set_hl({ const N: i32 = ((((4 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (13 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(-1i32 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    become move_player_pic(cpu);
}

pub(crate) fn move_player_pic(cpu: &mut Cpu) {
    cpu.pc = 0x4266;
    cpu.c = 0x8;
    cpu.pc += 2;
    cpu.cycle(8);
    become move_player_pic_loop(cpu);
}

pub(crate) fn move_player_pic_loop(cpu: &mut Cpu) {
    cpu.pc = 0x4268;
    cpu.push_word(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.push_word(cpu.de());
    cpu.pc += 1;
    cpu.cycle(16);
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
    cpu.set_bc(0x0707);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = { let value = (((0x45a4 as i32) - (0x456b as i32)) / (3 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::predef::predef(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xffd5, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::tilemap::wait_bg_map(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::delay::delay_frame(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_de(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
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
    let value = cpu.pop_word();
    cpu.set_bc(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let value = cpu.c;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.c = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.push_word(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.e;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.d;
    let carry_in = u8::from(cpu.flag_c);
    let lhs = cpu.a;
    let (partial, borrow1) = lhs.overflowing_sub(value);
    let (result, borrow2) = partial.overflowing_sub(carry_in);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < ((value & 0x0f) + carry_in);
    cpu.flag_c = borrow1 || borrow2;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.h = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_bc(0x0707);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::clear_box(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_bc(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.cycle(12);
    become move_player_pic_loop(cpu);
}

pub(crate) fn show_player_naming_choices(cpu: &mut Cpu) {
    cpu.pc = 0x4297;
    cpu.set_hl(0x42b5);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xd472);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((crate::defs::PLAYERGENDER_FEMALE_F as u8) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become show_player_naming_choices_got_header(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x42e5);
    cpu.pc += 3;
    cpu.cycle(12);
    become show_player_naming_choices_got_header(cpu);
}

pub(crate) fn show_player_naming_choices_got_header(cpu: &mut Cpu) {
    cpu.pc = 0x42a4;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::menu::load_menu_header(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::menu::vertical_menu(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xcfa9);
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
    crate::game::home::menu::copy_name_from_menu(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::menu::close_window(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn get_player_name_array(cpu: &mut Cpu) {
    cpu.pc = 0x4318;
    cpu.set_hl(0xd47d);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0x42c9);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xd472);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((crate::defs::PLAYERGENDER_FEMALE_F as u8) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become get_player_name_array_got_array(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_de(0x42f9);
    cpu.pc += 3;
    cpu.cycle(12);
    become get_player_name_array_got_array(cpu);
}

pub(crate) fn get_player_name_array_got_array(cpu: &mut Cpu) {
    cpu.pc = 0x4328;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::string::init_name(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn get_player_icon(cpu: &mut Cpu) {
    cpu.pc = 0x432c;
    cpu.set_de(0x4000);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = 0x30;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xd472);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((crate::defs::PLAYERGENDER_FEMALE_F as u8) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become get_player_icon_got_gfx(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_de(0x7a40);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = 0x31;
    cpu.pc += 2;
    cpu.cycle(8);
    become get_player_icon_got_gfx(cpu);
}

pub(crate) fn get_player_icon_got_gfx(cpu: &mut Cpu) {
    cpu.pc = 0x433d;
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn get_card_pic(cpu: &mut Cpu) {
    cpu.pc = 0x433e;
    cpu.set_hl(0x4365);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xd472);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((crate::defs::PLAYERGENDER_FEMALE_F as u8) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become get_card_pic_got_pic(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x4595);
    cpu.pc += 3;
    cpu.cycle(12);
    become get_card_pic_got_pic(cpu);
}

pub(crate) fn get_card_pic_got_pic(cpu: &mut Cpu) {
    cpu.pc = 0x434b;
    cpu.set_de({ const N: i32 = ((0x9000 as i32) + ((crate::defs::TILE_SIZE as u16 as i32) * (0x00 as i32))); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(0x0230);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0x22;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::gfx::far_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0x47c5);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de({ const N: i32 = ((0x9000 as i32) + ((crate::defs::TILE_SIZE as u16 as i32) * (0x23 as i32))); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(0x0060);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0x22;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::gfx::far_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn get_player_backpic(cpu: &mut Cpu) {
    cpu.pc = 0x4825;
    cpu.a = cpu.read_byte(0xd472);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((crate::defs::PLAYERGENDER_FEMALE_F as u8) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become get_chris_backpic(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    get_kris_backpic(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn get_chris_backpic(cpu: &mut Cpu) {
    cpu.pc = 0x4830;
    cpu.set_hl(0x7a1a);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = 0x0a;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_de({ const N: i32 = ((0x9000 as i32) + ((crate::defs::TILE_SIZE as u16 as i32) * (0x31 as i32))); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.c = { let value = ((7 as i32) * (7 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = { let value = (((0x462b as i32) - (0x456b as i32)) / (3 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::predef::predef(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn hof_load_trainer_frontpic(cpu: &mut Cpu) {
    cpu.pc = 0x4840;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::tilemap::wait_bg_map(cpu);
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
    cpu.e = crate::defs::CHRIS as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xd472);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((crate::defs::PLAYERGENDER_FEMALE_F as u8) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become hof_load_trainer_frontpic_got_class(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.e = crate::defs::KRIS as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    become hof_load_trainer_frontpic_got_class(cpu);
}

pub(crate) fn hof_load_trainer_frontpic_got_class(cpu: &mut Cpu) {
    cpu.pc = 0x4851;
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xd233, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_de(0x48a9);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xd472);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((crate::defs::PLAYERGENDER_FEMALE_F as u8) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become hof_load_trainer_frontpic_got_pic(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_de(0x4bb9);
    cpu.pc += 3;
    cpu.cycle(12);
    become hof_load_trainer_frontpic_got_pic(cpu);
}

pub(crate) fn hof_load_trainer_frontpic_got_pic(cpu: &mut Cpu) {
    cpu.pc = 0x4862;
    cpu.set_hl(0x9000);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = 0x22;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.c = { let value = ((7 as i32) * (7 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::gfx::get2bpp(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::tilemap::wait_bg_map(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xffd4, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn draw_intro_player_pic(cpu: &mut Cpu) {
    cpu.pc = 0x4874;
    cpu.e = crate::defs::CHRIS as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xd472);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((crate::defs::PLAYERGENDER_FEMALE_F as u8) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become draw_intro_player_pic_got_class(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.e = crate::defs::KRIS as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    become draw_intro_player_pic_got_class(cpu);
}

pub(crate) fn draw_intro_player_pic_got_class(cpu: &mut Cpu) {
    cpu.pc = 0x487f;
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xd233, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_de(0x48a9);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xd472);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((crate::defs::PLAYERGENDER_FEMALE_F as u8) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become draw_intro_player_pic_got_pic(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_de(0x4bb9);
    cpu.pc += 3;
    cpu.cycle(12);
    become draw_intro_player_pic_got_pic(cpu);
}

pub(crate) fn draw_intro_player_pic_got_pic(cpu: &mut Cpu) {
    cpu.pc = 0x4890;
    cpu.set_hl(0x9000);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = 0x22;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.c = { let value = ((7 as i32) * (7 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::gfx::get2bpp(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xffad, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.set_hl({ const N: i32 = ((((4 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (6 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(0x0707);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = { let value = (((0x45a4 as i32) - (0x456b as i32)) / (3 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::predef::predef(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn get_kris_backpic(cpu: &mut Cpu) {
    cpu.pc = 0x4ec9;
    cpu.set_de(0x4ed6);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl({ const N: i32 = ((0x9000 as i32) + ((crate::defs::TILE_SIZE as u16 as i32) * (0x31 as i32))); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc({ const N: i32 = ((((0x22 as i32) & (0xff as i32)) << (8 as i32)) | (((7 as i32) * (7 as i32)) & (0xff as i32))); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::gfx::get2bpp(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

