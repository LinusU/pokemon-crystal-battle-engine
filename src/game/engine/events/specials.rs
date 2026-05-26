use crate::runtime::Cpu;

pub fn special(cpu: &mut Cpu) {
    cpu.pc = 0x401b;
    cpu.set_hl(0x4029);
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
    let lhs = cpu.hl();
    let rhs = cpu.de();
    let (result, carry) = lhs.overflowing_add(rhs);
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0fff) + (rhs & 0x0fff) > 0x0fff;
    cpu.flag_c = carry;
    cpu.set_hl(result);
    cpu.pc += 1;
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
    cpu.b = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.h = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn unused_dummy_special(cpu: &mut Cpu) {
    cpu.pc = 0x4224;
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn set_player_palette(cpu: &mut Cpu) {
    cpu.pc = 0x4225;
    cpu.a = cpu.read_byte(0xc2dd);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.d = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = 0x01;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x57e2);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn game_corner_prize_mon_check_dex(cpu: &mut Cpu) {
    cpu.pc = 0x4230;
    cpu.a = cpu.read_byte(0xc2dd);
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
    if !cpu.flag_z {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc2dd);
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
    crate::game::home::pokedex_flags::set_seen_and_caught_mon(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map::fade_to_menu(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xc2dd);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xd265, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x3e;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x7877);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map::exit_all_menus(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn unused_set_seen_mon(cpu: &mut Cpu) {
    cpu.pc = 0x4252;
    cpu.a = cpu.read_byte(0xc2dd);
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
    crate::game::home::pokedex_flags::set_seen_mon(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn find_party_mon_above_level(cpu: &mut Cpu) {
    cpu.pc = 0x425a;
    cpu.a = cpu.read_byte(0xc2dd);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = 0x13;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x5bd2);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_z {
        cpu.cycle(12);
        become found_none(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become found_one(cpu);
}

pub(crate) fn find_party_mon_at_least_that_happy(cpu: &mut Cpu) {
    cpu.pc = 0x4268;
    cpu.a = cpu.read_byte(0xc2dd);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = 0x13;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x5bd9);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_z {
        cpu.cycle(12);
        become found_none(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become found_one(cpu);
}

pub(crate) fn find_party_mon_that_species(cpu: &mut Cpu) {
    cpu.pc = 0x4276;
    cpu.a = cpu.read_byte(0xc2dd);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = 0x13;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x5be0);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_z {
        cpu.cycle(12);
        become found_none(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become found_one(cpu);
}

pub(crate) fn find_party_mon_that_species_your_trainer_id(cpu: &mut Cpu) {
    cpu.pc = 0x4284;
    cpu.a = cpu.read_byte(0xc2dd);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = 0x13;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x5be6);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_z {
        cpu.cycle(12);
        become found_none(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become found_one(cpu);
}

pub(crate) fn found_one(cpu: &mut Cpu) {
    cpu.pc = 0x4292;
    cpu.a = crate::defs::TRUE as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc2dd, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn found_none(cpu: &mut Cpu) {
    cpu.pc = 0x4298;
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

pub(crate) fn name_rival(cpu: &mut Cpu) {
    cpu.pc = 0x429d;
    cpu.b = crate::defs::NAME_RIVAL as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_de(0xd493);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0x04;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x56b7);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xd493);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0x42b2);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::string::init_name(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn name_rater(cpu: &mut Cpu) {
    cpu.pc = 0x42b9;
    cpu.a = 0x3e;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x76ed);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn overworld_town_map(cpu: &mut Cpu) {
    cpu.pc = 0x42c0;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map::fade_to_menu(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x24;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x591c);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map::exit_all_menus(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn unown_printer(cpu: &mut Cpu) {
    cpu.pc = 0x42cd;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map::fade_to_menu(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x05;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x6be4);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map::exit_all_menus(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn display_link_record(cpu: &mut Cpu) {
    cpu.pc = 0x42da;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map::fade_to_menu(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x0f;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x7836);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map::exit_all_menus(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn players_house_pc(cpu: &mut Cpu) {
    cpu.pc = 0x42e7;
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
    cpu.a = 0x05;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x56d9);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc2dd, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn check_mystery_gift(cpu: &mut Cpu) {
    cpu.pc = 0x42f6;
    cpu.a = 0x00;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xabe2);
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
        cpu.cycle(12);
        become check_mystery_gift_no(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    become check_mystery_gift_no(cpu);
}

pub(crate) fn check_mystery_gift_no(cpu: &mut Cpu) {
    cpu.pc = 0x4302;
    cpu.write_byte(0xc2dd, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::close_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn get_mystery_gift_item(cpu: &mut Cpu) {
    cpu.pc = 0x4309;
    cpu.a = 0x00;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xabe2);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xd106, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xd10c, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xd892);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::item::receive_item(cpu);
    assert_eq!(cpu.pc, next_pc);
    if !cpu.flag_c {
        cpu.cycle(12);
        become get_mystery_gift_item_no_room(cpu);
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
    cpu.write_byte(0xabe2, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::close_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xd106);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xd265, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::names::get_item_name(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0x4345);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::print_text(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = crate::defs::TRUE as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc2dd, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn get_mystery_gift_item_no_room(cpu: &mut Cpu) {
    cpu.pc = 0x433d;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::close_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
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

pub(crate) fn bug_contest_judging(cpu: &mut Cpu) {
    cpu.pc = 0x434a;
    cpu.a = 0x04;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x769d);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc2dd, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn map_radio(cpu: &mut Cpu) {
    cpu.pc = 0x4355;
    cpu.a = cpu.read_byte(0xc2dd);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = 0x24;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x5a53);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn unown_puzzle(cpu: &mut Cpu) {
    cpu.pc = 0x4360;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map::fade_to_menu(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x38;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x5190);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xd0ec);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xc2dd, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map::exit_all_menus(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn slot_machine(cpu: &mut Cpu) {
    cpu.pc = 0x4373;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    check_coins_and_coin_case(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 0x24;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x66c7);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    start_game_corner_game(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn card_flip(cpu: &mut Cpu) {
    cpu.pc = 0x4380;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    check_coins_and_coin_case(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 0x38;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x40ee);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    start_game_corner_game(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn unused_memory_game(cpu: &mut Cpu) {
    cpu.pc = 0x438d;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    check_coins_and_coin_case(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 0x38;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x5e5b);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    start_game_corner_game(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn start_game_corner_game(cpu: &mut Cpu) {
    cpu.pc = 0x439a;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::queue_script::far_queue_script(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map::fade_to_menu(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xd0e8);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.h = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map::exit_all_menus(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn check_coins_and_coin_case(cpu: &mut Cpu) {
    cpu.pc = 0x43ae;
    cpu.set_hl(0xd855);
    cpu.pc += 3;
    cpu.cycle(12);
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
        cpu.cycle(12);
        become check_coins_and_coin_case_no_coins(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = crate::defs::COIN_CASE as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xd106, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xd892);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::item::check_item(cpu);
    assert_eq!(cpu.pc, next_pc);
    if !cpu.flag_c {
        cpu.cycle(12);
        become check_coins_and_coin_case_no_coin_case(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.a;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn check_coins_and_coin_case_no_coins(cpu: &mut Cpu) {
    cpu.pc = 0x43c4;
    cpu.set_hl(0x43d1);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(12);
    become check_coins_and_coin_case_print(cpu);
}

pub(crate) fn check_coins_and_coin_case_no_coin_case(cpu: &mut Cpu) {
    cpu.pc = 0x43c9;
    cpu.set_hl(0x43d6);
    cpu.pc += 3;
    cpu.cycle(12);
    become check_coins_and_coin_case_print(cpu);
}

pub(crate) fn check_coins_and_coin_case_print(cpu: &mut Cpu) {
    cpu.pc = 0x43cc;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::print_text(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = true;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn clear_bg_palettes_buffer_screen(cpu: &mut Cpu) {
    cpu.pc = 0x43db;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::tilemap::clear_bg_palettes(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map::buffer_screen(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn script_return_carry(cpu: &mut Cpu) {
    cpu.pc = 0x43e2;
    if cpu.flag_c {
        cpu.cycle(12);
        become script_return_carry_carry(cpu);
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
    cpu.write_byte(0xc2dd, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn script_return_carry_carry(cpu: &mut Cpu) {
    cpu.pc = 0x43e9;
    cpu.a = 1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc2dd, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn unused_check_unused_two_day_timer(cpu: &mut Cpu) {
    cpu.pc = 0x43ef;
    cpu.a = 0x04;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x550c);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xdc3a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xc2dd, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn activate_fishing_swarm(cpu: &mut Cpu) {
    cpu.pc = 0x43fc;
    cpu.a = cpu.read_byte(0xc2dd);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xdfce, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub fn store_swarm_map_indices(cpu: &mut Cpu) {
    cpu.pc = 0x4403;
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
    if !cpu.flag_z {
        cpu.cycle(12);
        become store_swarm_map_indices_yanma(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xdfcc, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xdfcd, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn store_swarm_map_indices_yanma(cpu: &mut Cpu) {
    cpu.pc = 0x4410;
    cpu.a = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xdc5a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xdc5b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn check_pokerus(cpu: &mut Cpu) {
    cpu.pc = 0x4419;
    cpu.a = 0x13;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x5860);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(16);
    become script_return_carry(cpu);
}

pub(crate) fn reset_lucky_number_show_flag(cpu: &mut Cpu) {
    cpu.pc = 0x4422;
    cpu.a = 0x04;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x552b);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xdc9d);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((crate::defs::LUCKYNUMBERSHOW_GAME_OVER_F as u8) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.a = 0x01;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x5d33);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn check_lucky_number_show_flag(cpu: &mut Cpu) {
    cpu.pc = 0x4434;
    cpu.a = 0x04;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x5542);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(16);
    become script_return_carry(cpu);
}

pub(crate) fn snorlax_awake(cpu: &mut Cpu) {
    cpu.pc = 0x443d;
    cpu.a = cpu.read_byte(0xc2c0);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = crate::defs::MUSIC_POKE_FLUTE_CHANNEL as u8;
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
        become snorlax_awake_nope(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xdcb8);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xdcb7);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl(0x4467);
    cpu.pc += 3;
    cpu.cycle(12);
    become snorlax_awake_loop(cpu);
}

pub(crate) fn snorlax_awake_loop(cpu: &mut Cpu) {
    cpu.pc = 0x444f;
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = -1i16 as u8;
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
        become snorlax_awake_nope(cpu);
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
    if !cpu.flag_z {
        cpu.cycle(12);
        become snorlax_awake_nextcoord(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.c;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(12);
        become snorlax_awake_loop(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = crate::defs::TRUE as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become snorlax_awake_done(cpu);
}

pub(crate) fn snorlax_awake_nextcoord(cpu: &mut Cpu) {
    cpu.pc = 0x445f;
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.cycle(12);
    become snorlax_awake_loop(cpu);
}

pub(crate) fn snorlax_awake_nope(cpu: &mut Cpu) {
    cpu.pc = 0x4462;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    become snorlax_awake_done(cpu);
}

pub(crate) fn snorlax_awake_done(cpu: &mut Cpu) {
    cpu.pc = 0x4463;
    cpu.write_byte(0xc2dd, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn play_cur_mon_cry(cpu: &mut Cpu) {
    cpu.pc = 0x4472;
    cpu.a = cpu.read_byte(0xd108);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(16);
    become crate::game::home::pokemon::play_mon_cry(cpu);
}

pub(crate) fn gameboy_check(cpu: &mut Cpu) {
    cpu.pc = 0x4478;
    cpu.a = cpu.read_byte(0xffe6);
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
        become gameboy_check_cgb(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xffe7);
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
        become gameboy_check_sgb(cpu);
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
    cpu.cycle(12);
    become gameboy_check_done(cpu);
}

pub(crate) fn gameboy_check_sgb(cpu: &mut Cpu) {
    cpu.pc = 0x4485;
    cpu.a = crate::defs::GBCHECK_SGB as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become gameboy_check_done(cpu);
}

pub(crate) fn gameboy_check_cgb(cpu: &mut Cpu) {
    cpu.pc = 0x4489;
    cpu.a = crate::defs::GBCHECK_CGB as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    become gameboy_check_done(cpu);
}

pub(crate) fn gameboy_check_done(cpu: &mut Cpu) {
    cpu.pc = 0x448b;
    cpu.write_byte(0xc2dd, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn fade_out_music(cpu: &mut Cpu) {
    cpu.pc = 0x448f;
    cpu.a = ((crate::defs::MUSIC_NONE as u32) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc2a9, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = (((crate::defs::MUSIC_NONE as u32) >> 8) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte({ const N: i32 = (0xc2a9 as i32) + (1 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x2;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc2a7, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn diploma(cpu: &mut Cpu) {
    cpu.pc = 0x449f;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map::fade_to_menu(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x77;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x5702);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map::exit_all_menus(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn print_diploma(cpu: &mut Cpu) {
    cpu.pc = 0x44ac;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map::fade_to_menu(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x21;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x4688);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map::exit_all_menus(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn trainer_house(cpu: &mut Cpu) {
    cpu.pc = 0x44b9;
    cpu.a = 0x00;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xabfd);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xc2dd, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(16);
    become crate::game::home::sram::close_sram(cpu);
}
