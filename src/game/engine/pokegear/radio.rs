use crate::runtime::Cpu;

pub const CURRENT_TABLE_WIDTH: u8 = 2;

pub const W: u8 = 11;

pub const X: u8 = 22;

pub const Y: i32 = 0;

// duplicate assembler constant W

// duplicate assembler constant X

// duplicate assembler constant Y

// duplicate assembler constant X

// duplicate assembler constant X

// duplicate assembler constant X

// duplicate assembler constant CURRENT_TABLE_WIDTH

// duplicate assembler constant W

// duplicate assembler constant X

// duplicate assembler constant Y

// duplicate assembler constant X

// duplicate assembler constant CURRENT_TABLE_WIDTH

// duplicate assembler constant W

// duplicate assembler constant X

// duplicate assembler constant Y

// duplicate assembler constant X

// duplicate assembler constant X

// duplicate assembler constant X

// duplicate assembler constant CURRENT_TABLE_WIDTH

// duplicate assembler constant W

// duplicate assembler constant X

// duplicate assembler constant Y

// duplicate assembler constant X

// duplicate assembler constant CURRENT_TABLE_WIDTH

// duplicate assembler constant W

// duplicate assembler constant X

// duplicate assembler constant Y

// duplicate assembler constant X

// duplicate assembler constant X

// duplicate assembler constant CURRENT_TABLE_WIDTH

// duplicate assembler constant W

// duplicate assembler constant X

// duplicate assembler constant Y

pub(crate) fn play_radio_show(cpu: &mut Cpu) {
    cpu.pc = 0x4612;
    cpu.a = cpu.read_byte(0xd002);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = crate::defs::POKE_FLUTE_RADIO as u8;
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
        become play_radio_show_ok(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xd84d);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((crate::defs::STATUSFLAGS2_ROCKETS_IN_RADIO_TOWER_F as u8) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become play_radio_show_ok(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::region::is_in_johto(cpu);
    assert_eq!(cpu.pc, next_pc);
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
        become play_radio_show_ok(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = crate::defs::ROCKET_RADIO as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xd002, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    become play_radio_show_ok(cpu);
}

pub(crate) fn print_radio_line(cpu: &mut Cpu) {
    cpu.pc = 0x46ea;
    cpu.write_byte(0xd003, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xd00c);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xd005);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 2;
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
        become print_radio_line_print(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), crate::defs::TX_START as u8);
    cpu.pc += 2;
    cpu.cycle(12);
    let value = cpu.a;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xd005, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 2;
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
        become print_radio_line_print(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_bc({ const N: i32 = ((((16 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (1 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::print_textbox_text_at(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(12);
    become print_radio_line_skip(cpu);
}

pub(crate) fn print_radio_line_print(cpu: &mut Cpu) {
    cpu.pc = 0x470a;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::print_textbox_text(cpu);
    assert_eq!(cpu.pc, next_pc);
    become print_radio_line_skip(cpu);
}

pub(crate) fn print_radio_line_skip(cpu: &mut Cpu) {
    cpu.pc = 0x470d;
    cpu.a = crate::defs::RADIO_SCROLL as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xd002, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 100;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xd004, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn replace_periods_with_spaces(cpu: &mut Cpu) {
    cpu.pc = 0x4718;
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.b = { let value = ((crate::defs::SCREEN_WIDTH as i32) * (2 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    become replace_periods_with_spaces_loop(cpu);
}

pub(crate) fn replace_periods_with_spaces_loop(cpu: &mut Cpu) {
    cpu.pc = 0x471b;
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = { let value = (0xe8 as i32); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
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
        become replace_periods_with_spaces_next(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), { let value = (0x7f as i32); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } });
    cpu.pc += 2;
    cpu.cycle(12);
    become replace_periods_with_spaces_next(cpu);
}

pub(crate) fn replace_periods_with_spaces_next(cpu: &mut Cpu) {
    cpu.pc = 0x4722;
    cpu.set_hl(cpu.hl().wrapping_add(1));
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
        become replace_periods_with_spaces_loop(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn radio_scroll(cpu: &mut Cpu) {
    cpu.pc = 0x4728;
    cpu.set_hl(0xd004);
    cpu.pc += 3;
    cpu.cycle(12);
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
    if cpu.flag_z {
        cpu.cycle(12);
        become radio_scroll_proceed(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let addr = cpu.hl();
    let value = cpu.read_byte(addr);
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.write_byte(addr, result);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn radio_scroll_proceed(cpu: &mut Cpu) {
    cpu.pc = 0x4731;
    cpu.a = cpu.read_byte(0xd003);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xd002, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xd005);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 1;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        let next_pc = cpu.pc + 3;
        cpu.push_word(next_pc);
        cpu.cycle(24);
        copy_bottom_line_to_top_line(cpu);
        assert_eq!(cpu.pc, next_pc);
    } else {
        cpu.pc += 3;
        cpu.cycle(12);
    }
    cpu.cycle(16);
    become clear_bottom_line(cpu);
}

pub(crate) fn oaks_pkmn_talk1(cpu: &mut Cpu) {
    cpu.pc = 0x4742;
    cpu.a = 5;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xd006, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    start_radio_station(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0x4820);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = crate::defs::OAKS_POKEMON_TALK_2 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become next_radio_line(cpu);
}

pub(crate) fn oaks_pkmn_talk2(cpu: &mut Cpu) {
    cpu.pc = 0x4752;
    cpu.set_hl(0x4825);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = crate::defs::OAKS_POKEMON_TALK_3 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become next_radio_line(cpu);
}

pub(crate) fn oaks_pkmn_talk3(cpu: &mut Cpu) {
    cpu.pc = 0x475a;
    cpu.set_hl(0x482a);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = crate::defs::OAKS_POKEMON_TALK_4 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become next_radio_line(cpu);
}

pub(crate) fn oaks_pkmn_talk4(cpu: &mut Cpu) {
    cpu.pc = 0x4762;
    become oaks_pkmn_talk4_sample(cpu);
}

pub(crate) fn oaks_pkmn_talk4_sample(cpu: &mut Cpu) {
    cpu.pc = 0x4762;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::random::random(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = 0b11111;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = { let value = (((0x4810 as i32) - (0x47f2 as i32)) / (2 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
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
        become oaks_pkmn_talk4_sample(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x47f2);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.b = 0;
    cpu.pc += 2;
    cpu.cycle(8);
    let lhs = cpu.hl();
    let rhs = cpu.bc();
    let (result, carry) = lhs.overflowing_add(rhs);
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0fff) + (rhs & 0x0fff) > 0x0fff;
    cpu.flag_c = carry;
    cpu.set_hl(result);
    cpu.pc += 1;
    cpu.cycle(8);
    let lhs = cpu.hl();
    let rhs = cpu.bc();
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
    cpu.c = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.push_word(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.set_hl(0x65e9);
    cpu.pc += 3;
    cpu.cycle(12);
    become oaks_pkmn_talk4_loop(cpu);
}

pub(crate) fn oaks_pkmn_talk4_loop(cpu: &mut Cpu) {
    cpu.pc = 0x477a;
    cpu.a = 0x0a;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::get_far_byte(cpu);
    assert_eq!(cpu.pc, next_pc);
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
        become oaks_pkmn_talk4_overflow(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
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
        become oaks_pkmn_talk4_next(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x0a;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::get_far_byte(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.c;
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
        become oaks_pkmn_talk4_done(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become oaks_pkmn_talk4_next(cpu);
}

pub(crate) fn oaks_pkmn_talk4_next(cpu: &mut Cpu) {
    cpu.pc = 0x478f;
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(crate::defs::GRASS_WILDDATA_LENGTH as u16);
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
    cpu.cycle(12);
    become oaks_pkmn_talk4_loop(cpu);
}

pub(crate) fn oaks_pkmn_talk4_done(cpu: &mut Cpu) {
    cpu.pc = 0x4796;
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
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
    become oaks_pkmn_talk4_loop2(cpu);
}

pub(crate) fn oaks_pkmn_talk4_loop2(cpu: &mut Cpu) {
    cpu.pc = 0x479a;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::random::random(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = 3;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = crate::defs::DARKNESS_F as u8;
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
        become oaks_pkmn_talk4_loop2(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_bc(0x000e);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::array::add_n_times(cpu);
    assert_eq!(cpu.pc, next_pc);
    become oaks_pkmn_talk4_loop3(cpu);
}

pub(crate) fn oaks_pkmn_talk4_loop3(cpu: &mut Cpu) {
    cpu.pc = 0x47a9;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::random::random(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = 7;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 2;
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
        become oaks_pkmn_talk4_loop3(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 5;
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
        become oaks_pkmn_talk4_loop3(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.d = 0;
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
    let lhs = cpu.hl();
    let rhs = cpu.de();
    let (result, carry) = lhs.overflowing_add(rhs);
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0fff) + (rhs & 0x0fff) > 0x0fff;
    cpu.flag_c = carry;
    cpu.set_hl(result);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 0x0a;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::get_far_byte(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.write_byte(0xd265, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xd108, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::names::get_pokemon_name(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xd073);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xd050);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(crate::defs::MON_NAME_LENGTH as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_bc(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map::get_world_map_location(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = 0x72;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x68a5);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0x482f);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    copy_radio_text_to_ram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = crate::defs::OAKS_POKEMON_TALK_5 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become print_radio_line(cpu);
}

pub(crate) fn oaks_pkmn_talk4_overflow(cpu: &mut Cpu) {
    cpu.pc = 0x47ec;
    let value = cpu.pop_word();
    cpu.set_bc(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.a = crate::defs::OAKS_POKEMON_TALK as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become print_radio_line(cpu);
}

pub(crate) fn oaks_pkmn_talk5(cpu: &mut Cpu) {
    cpu.pc = 0x4810;
    cpu.set_hl(0x4834);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = crate::defs::OAKS_POKEMON_TALK_6 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become next_radio_line(cpu);
}

pub(crate) fn oaks_pkmn_talk6(cpu: &mut Cpu) {
    cpu.pc = 0x4818;
    cpu.set_hl(0x4839);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = crate::defs::OAKS_POKEMON_TALK_7 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become next_radio_line(cpu);
}

pub(crate) fn oaks_pkmn_talk7(cpu: &mut Cpu) {
    cpu.pc = 0x483e;
    cpu.a = cpu.read_byte(0xd108);
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
    cpu.set_hl(0x484f);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = crate::defs::OAKS_POKEMON_TALK_8 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become next_radio_line(cpu);
}

pub(crate) fn oaks_pkmn_talk8(cpu: &mut Cpu) {
    cpu.pc = 0x4854;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::random::random(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = 15;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.d = 0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x4869);
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
    cpu.a = crate::defs::OAKS_POKEMON_TALK_9 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become next_radio_line(cpu);
}

pub(crate) fn oaks_pkmn_talk9(cpu: &mut Cpu) {
    cpu.pc = 0x48d9;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::random::random(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = 15;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.d = 0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x48fe);
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
    cpu.a = cpu.read_byte(0xd006);
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
    cpu.write_byte(0xd006, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = crate::defs::OAKS_POKEMON_TALK_4 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(12);
        become oaks_pkmn_talk9_ok(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 5;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xd006, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = crate::defs::OAKS_POKEMON_TALK_10 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    become oaks_pkmn_talk9_ok(cpu);
}

pub(crate) fn oaks_pkmn_talk9_ok(cpu: &mut Cpu) {
    cpu.pc = 0x48fb;
    cpu.cycle(16);
    become next_radio_line(cpu);
}

pub(crate) fn oaks_pkmn_talk10(cpu: &mut Cpu) {
    cpu.pc = 0x496e;
    cpu.a = 0x24;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x5868);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0x4993);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::print_text(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::tilemap::wait_bg_map(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0x498e);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::print_text(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = crate::defs::OAKS_POKEMON_TALK_11 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xd002, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 100;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xd004, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn oaks_pkmn_talk11(cpu: &mut Cpu) {
    cpu.pc = 0x4994;
    cpu.set_hl(0xd004);
    cpu.pc += 3;
    cpu.cycle(12);
    let addr = cpu.hl();
    let value = cpu.read_byte(addr);
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.write_byte(addr, result);
    cpu.pc += 1;
    cpu.cycle(12);
    if !cpu.flag_z {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl({ const N: i32 = ((((14 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (9 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0x49a4);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = crate::defs::OAKS_POKEMON_TALK_12 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become place_radio_string(cpu);
}

pub(crate) fn oaks_pkmn_talk12(cpu: &mut Cpu) {
    cpu.pc = 0x49a9;
    cpu.set_hl(0xd004);
    cpu.pc += 3;
    cpu.cycle(12);
    let addr = cpu.hl();
    let value = cpu.read_byte(addr);
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.write_byte(addr, result);
    cpu.pc += 1;
    cpu.cycle(12);
    if !cpu.flag_z {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl({ const N: i32 = ((((16 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (1 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0x49b9);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = crate::defs::OAKS_POKEMON_TALK_13 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become place_radio_string(cpu);
}

pub(crate) fn oaks_pkmn_talk13(cpu: &mut Cpu) {
    cpu.pc = 0x49c6;
    cpu.set_hl(0xd004);
    cpu.pc += 3;
    cpu.cycle(12);
    let addr = cpu.hl();
    let value = cpu.read_byte(addr);
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.write_byte(addr, result);
    cpu.pc += 1;
    cpu.cycle(12);
    if !cpu.flag_z {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl({ const N: i32 = ((((16 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (12 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0x49d6);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = crate::defs::OAKS_POKEMON_TALK_14 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become place_radio_string(cpu);
}

pub(crate) fn oaks_pkmn_talk14(cpu: &mut Cpu) {
    cpu.pc = 0x49d7;
    cpu.set_hl(0xd004);
    cpu.pc += 3;
    cpu.cycle(12);
    let addr = cpu.hl();
    let value = cpu.read_byte(addr);
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.write_byte(addr, result);
    cpu.pc += 1;
    cpu.cycle(12);
    if !cpu.flag_z {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(crate::defs::MUSIC_POKEMON_TALK as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0x5854);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0x24;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0x49ff);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::print_text(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = crate::defs::OAKS_POKEMON_TALK_4 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xd003, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xd005, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = crate::defs::RADIO_SCROLL as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xd002, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 10;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xd004, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn place_radio_string(cpu: &mut Cpu) {
    cpu.pc = 0x4a00;
    cpu.write_byte(0xd002, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 100;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xd004, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(16);
    become crate::game::home::text::place_string(cpu);
}

pub(crate) fn copy_bottom_line_to_top_line(cpu: &mut Cpu) {
    cpu.pc = 0x4a0b;
    cpu.set_hl({ const N: i32 = ((((15 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (0 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de({ const N: i32 = ((((13 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (0 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(0x0028);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become crate::game::home::copy::copy_bytes(cpu);
}

pub(crate) fn clear_bottom_line(cpu: &mut Cpu) {
    cpu.pc = 0x4a17;
    cpu.set_hl({ const N: i32 = ((((15 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (1 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc({ const N: i32 = (crate::defs::SCREEN_WIDTH as u16 as i32) - (2 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = { let value = (0x7f as i32); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::byte_fill(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = ((((16 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (1 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc({ const N: i32 = (crate::defs::SCREEN_WIDTH as u16 as i32) - (2 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = { let value = (0x7f as i32); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become crate::game::home::copy::byte_fill(cpu);
}

pub(crate) fn pokedex_show_get_dex_entry_bank(cpu: &mut Cpu) {
    cpu.pc = 0x4a2d;
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.push_word(cpu.de());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xd108);
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
    let value = cpu.a;
    let result = value.rotate_left(1);
    cpu.flag_z = false;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x80 != 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.a;
    let result = value.rotate_left(1);
    cpu.flag_z = false;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x80 != 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = 3;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x4a42);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.d = 0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
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

pub(crate) fn pokedex_show1(cpu: &mut Cpu) {
    cpu.pc = 0x4a46;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    start_radio_station(cpu);
    assert_eq!(cpu.pc, next_pc);
    become pokedex_show1_loop(cpu);
}

pub(crate) fn pokedex_show1_loop(cpu: &mut Cpu) {
    cpu.pc = 0x4a49;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::random::random(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = crate::defs::NUM_POKEMON as u8;
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
        become pokedex_show1_loop(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.push_word(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::pokedex_flags::check_caught_mon(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_bc(value);
    cpu.pc += 1;
    cpu.cycle(12);
    if cpu.flag_z {
        cpu.cycle(12);
        become pokedex_show1_loop(cpu);
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
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xd108, cpu.a);
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
    cpu.set_hl(0x4b30);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = crate::defs::POKEDEX_SHOW_2 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become next_radio_line(cpu);
}

pub(crate) fn pokedex_show2(cpu: &mut Cpu) {
    cpu.pc = 0x4a6c;
    cpu.a = cpu.read_byte(0xd108);
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
    cpu.set_hl(0x4378);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.b = 0;
    cpu.pc += 2;
    cpu.cycle(8);
    let lhs = cpu.hl();
    let rhs = cpu.bc();
    let (result, carry) = lhs.overflowing_add(rhs);
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0fff) + (rhs & 0x0fff) > 0x0fff;
    cpu.flag_c = carry;
    cpu.set_hl(result);
    cpu.pc += 1;
    cpu.cycle(8);
    let lhs = cpu.hl();
    let rhs = cpu.bc();
    let (result, carry) = lhs.overflowing_add(rhs);
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0fff) + (rhs & 0x0fff) > 0x0fff;
    cpu.flag_c = carry;
    cpu.set_hl(result);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 0x11;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::get_far_word(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    pokedex_show_get_dex_entry_bank(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    copy_dex_entry_part1(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), { let value = (0x57 as i32); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } });
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.set_hl(0xd26b);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    copy_radio_text_to_ram(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    copy_dex_entry_part2(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
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
    cpu.a = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xd26b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte({ const N: i32 = (0xd26b as i32) + (1 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = crate::defs::POKEDEX_SHOW_3 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become print_radio_line(cpu);
}

pub(crate) fn pokedex_show3(cpu: &mut Cpu) {
    cpu.pc = 0x4aa4;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    copy_dex_entry(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = crate::defs::POKEDEX_SHOW_4 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become print_radio_line(cpu);
}

pub(crate) fn pokedex_show4(cpu: &mut Cpu) {
    cpu.pc = 0x4aac;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    copy_dex_entry(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = crate::defs::POKEDEX_SHOW_5 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become print_radio_line(cpu);
}

pub(crate) fn pokedex_show5(cpu: &mut Cpu) {
    cpu.pc = 0x4ab4;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    copy_dex_entry(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = crate::defs::POKEDEX_SHOW_6 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become print_radio_line(cpu);
}

pub(crate) fn pokedex_show6(cpu: &mut Cpu) {
    cpu.pc = 0x4abc;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    copy_dex_entry(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = crate::defs::POKEDEX_SHOW_7 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become print_radio_line(cpu);
}

pub(crate) fn pokedex_show7(cpu: &mut Cpu) {
    cpu.pc = 0x4ac4;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    copy_dex_entry(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = crate::defs::POKEDEX_SHOW_8 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become print_radio_line(cpu);
}

pub(crate) fn pokedex_show8(cpu: &mut Cpu) {
    cpu.pc = 0x4acc;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    copy_dex_entry(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = crate::defs::POKEDEX_SHOW as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become print_radio_line(cpu);
}

pub(crate) fn copy_dex_entry(cpu: &mut Cpu) {
    cpu.pc = 0x4ad4;
    cpu.a = cpu.read_byte(0xd26b);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte({ const N: i32 = (0xd26b as i32) + (1 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.h = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xd26d);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    copy_dex_entry_part1(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), { let value = (0x57 as i32); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } });
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.set_hl(0xd26b);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    copy_radio_text_to_ram(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    copy_dex_entry_part2(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn copy_dex_entry_part1(cpu: &mut Cpu) {
    cpu.pc = 0x4af3;
    cpu.set_de(0xd26d);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc({ const N: i32 = (crate::defs::SCREEN_WIDTH as u16 as i32) - (1 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::gfx::far_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xd26b);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.write_byte(cpu.hl(), crate::defs::TX_START as u8);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), { let value = (0x4f as i32); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } });
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    become copy_dex_entry_part1_loop(cpu);
}

pub(crate) fn copy_dex_entry_part1_loop(cpu: &mut Cpu) {
    cpu.pc = 0x4b05;
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = { let value = (0x50 as i32); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    let value = { let value = (0x4e as i32); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    let value = { let value = (0x5f as i32); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.cycle(12);
    become copy_dex_entry_part1_loop(cpu);
}

pub(crate) fn copy_dex_entry_part2(cpu: &mut Cpu) {
    cpu.pc = 0x4b11;
    cpu.d = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    become copy_dex_entry_part2_loop(cpu);
}

pub(crate) fn copy_dex_entry_part2_loop(cpu: &mut Cpu) {
    cpu.pc = 0x4b12;
    cpu.a = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::get_far_byte(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = { let value = (0x50 as i32); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
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
        become copy_dex_entry_part2_okay(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = { let value = (0x4e as i32); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
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
        become copy_dex_entry_part2_okay(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = { let value = (0x5f as i32); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
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
        become copy_dex_entry_part2_loop(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become copy_dex_entry_part2_okay(cpu);
}

pub(crate) fn copy_dex_entry_part2_okay(cpu: &mut Cpu) {
    cpu.pc = 0x4b23;
    cpu.a = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xd26b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte({ const N: i32 = (0xd26b as i32) + (1 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xd26d, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn ben_mon_music1(cpu: &mut Cpu) {
    cpu.pc = 0x4b35;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    start_pokemon_music_channel(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0x4baa);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = crate::defs::POKEMON_MUSIC_2 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become next_radio_line(cpu);
}

pub(crate) fn ben_mon_music2(cpu: &mut Cpu) {
    cpu.pc = 0x4b40;
    cpu.set_hl(0x4baf);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = crate::defs::POKEMON_MUSIC_3 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become next_radio_line(cpu);
}

pub(crate) fn ben_mon_music3(cpu: &mut Cpu) {
    cpu.pc = 0x4b48;
    cpu.set_hl(0x4bb4);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = crate::defs::POKEMON_MUSIC_4 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become next_radio_line(cpu);
}

pub(crate) fn fern_mon_music1(cpu: &mut Cpu) {
    cpu.pc = 0x4b50;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    start_pokemon_music_channel(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0x4bb9);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = crate::defs::LETS_ALL_SING_2 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become next_radio_line(cpu);
}

pub(crate) fn fern_mon_music2(cpu: &mut Cpu) {
    cpu.pc = 0x4b5b;
    cpu.set_hl(0x4bbe);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = crate::defs::POKEMON_MUSIC_4 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become next_radio_line(cpu);
}

pub(crate) fn ben_fern_music4(cpu: &mut Cpu) {
    cpu.pc = 0x4b63;
    cpu.set_hl(0x4bc3);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = crate::defs::POKEMON_MUSIC_5 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become next_radio_line(cpu);
}

pub(crate) fn ben_fern_music5(cpu: &mut Cpu) {
    cpu.pc = 0x4b6b;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::pokedex_flags::get_weekday(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = 1;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x4bc8);
    cpu.pc += 3;
    cpu.cycle(12);
    if cpu.flag_z {
        cpu.cycle(12);
        become ben_fern_music5_sun_tue_thur_sun(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x4bcd);
    cpu.pc += 3;
    cpu.cycle(12);
    become ben_fern_music5_sun_tue_thur_sun(cpu);
}

pub(crate) fn ben_fern_music5_sun_tue_thur_sun(cpu: &mut Cpu) {
    cpu.pc = 0x4b78;
    cpu.a = crate::defs::POKEMON_MUSIC_6 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become next_radio_line(cpu);
}

pub(crate) fn ben_fern_music6(cpu: &mut Cpu) {
    cpu.pc = 0x4b7d;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::pokedex_flags::get_weekday(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = 1;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x4bd2);
    cpu.pc += 3;
    cpu.cycle(12);
    if cpu.flag_z {
        cpu.cycle(12);
        become ben_fern_music6_sun_tue_thur_sun(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x4bd7);
    cpu.pc += 3;
    cpu.cycle(12);
    become ben_fern_music6_sun_tue_thur_sun(cpu);
}

pub(crate) fn ben_fern_music6_sun_tue_thur_sun(cpu: &mut Cpu) {
    cpu.pc = 0x4b8a;
    cpu.a = crate::defs::POKEMON_MUSIC_7 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become next_radio_line(cpu);
}

pub(crate) fn ben_fern_music7(cpu: &mut Cpu) {
    cpu.pc = 0x4b8f;
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn start_pokemon_music_channel(cpu: &mut Cpu) {
    cpu.pc = 0x4b90;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::radio_terminator(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::print_text(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_de(crate::defs::MUSIC_POKEMON_MARCH as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::pokedex_flags::get_weekday(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = 1;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become start_pokemon_music_channel_sun_tue_thur_sun(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_de(crate::defs::MUSIC_POKEMON_LULLABY as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    become start_pokemon_music_channel_sun_tue_thur_sun(cpu);
}

pub(crate) fn start_pokemon_music_channel_sun_tue_thur_sun(cpu: &mut Cpu) {
    cpu.pc = 0x4ba3;
    cpu.set_hl(0x5854);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0x24;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn lucky_number_show1(cpu: &mut Cpu) {
    cpu.pc = 0x4bdc;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    start_radio_station(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0x4434);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0x03;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    if !cpu.flag_c {
        cpu.cycle(12);
        become lucky_number_show1_dontreset(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x4422);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0x03;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    become lucky_number_show1_dontreset(cpu);
}

pub(crate) fn lucky_number_show1_dontreset(cpu: &mut Cpu) {
    cpu.pc = 0x4bed;
    cpu.set_hl(0x4c7e);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = crate::defs::LUCKY_NUMBER_SHOW_2 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become next_radio_line(cpu);
}

pub(crate) fn lucky_number_show2(cpu: &mut Cpu) {
    cpu.pc = 0x4bf5;
    cpu.set_hl(0x4c83);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = crate::defs::LUCKY_NUMBER_SHOW_3 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become next_radio_line(cpu);
}

pub(crate) fn lucky_number_show3(cpu: &mut Cpu) {
    cpu.pc = 0x4bfd;
    cpu.set_hl(0x4c88);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = crate::defs::LUCKY_NUMBER_SHOW_4 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become next_radio_line(cpu);
}

pub(crate) fn lucky_number_show4(cpu: &mut Cpu) {
    cpu.pc = 0x4c05;
    cpu.set_hl(0x4c8d);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = crate::defs::LUCKY_NUMBER_SHOW_5 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become next_radio_line(cpu);
}

pub(crate) fn lucky_number_show5(cpu: &mut Cpu) {
    cpu.pc = 0x4c0d;
    cpu.set_hl(0x4c92);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = crate::defs::LUCKY_NUMBER_SHOW_6 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become next_radio_line(cpu);
}

pub(crate) fn lucky_number_show6(cpu: &mut Cpu) {
    cpu.pc = 0x4c15;
    cpu.set_hl(0x4c97);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = crate::defs::LUCKY_NUMBER_SHOW_7 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become next_radio_line(cpu);
}

pub(crate) fn lucky_number_show7(cpu: &mut Cpu) {
    cpu.pc = 0x4c1d;
    cpu.set_hl(0x4c9c);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = crate::defs::LUCKY_NUMBER_SHOW_8 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become next_radio_line(cpu);
}

pub(crate) fn lucky_number_show8(cpu: &mut Cpu) {
    cpu.pc = 0x4c25;
    cpu.set_hl(0xd073);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xdc9f);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(0x8205);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::print_text::print_num(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = { let value = (0x50 as i32); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte({ const N: i32 = (0xd073 as i32) + (5 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0x4ca1);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = crate::defs::LUCKY_NUMBER_SHOW_9 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become next_radio_line(cpu);
}

pub(crate) fn lucky_number_show9(cpu: &mut Cpu) {
    cpu.pc = 0x4c3e;
    cpu.set_hl(0x4ca6);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = crate::defs::LUCKY_NUMBER_SHOW_10 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become next_radio_line(cpu);
}

pub(crate) fn lucky_number_show10(cpu: &mut Cpu) {
    cpu.pc = 0x4c46;
    cpu.set_hl(0x4c9c);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = crate::defs::LUCKY_NUMBER_SHOW_11 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become next_radio_line(cpu);
}

pub(crate) fn lucky_number_show11(cpu: &mut Cpu) {
    cpu.pc = 0x4c4e;
    cpu.set_hl(0x4ca1);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = crate::defs::LUCKY_NUMBER_SHOW_12 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become next_radio_line(cpu);
}

pub(crate) fn lucky_number_show12(cpu: &mut Cpu) {
    cpu.pc = 0x4c56;
    cpu.set_hl(0x4cab);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = crate::defs::LUCKY_NUMBER_SHOW_13 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become next_radio_line(cpu);
}

pub(crate) fn lucky_number_show13(cpu: &mut Cpu) {
    cpu.pc = 0x4c5e;
    cpu.set_hl(0x4cb0);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::random::random(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.a;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = crate::defs::LUCKY_CHANNEL as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(12);
        become lucky_number_show13_okay(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = crate::defs::LUCKY_NUMBER_SHOW_14 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    become lucky_number_show13_okay(cpu);
}

pub(crate) fn lucky_number_show13_okay(cpu: &mut Cpu) {
    cpu.pc = 0x4c6b;
    cpu.cycle(16);
    become next_radio_line(cpu);
}

pub(crate) fn lucky_number_show14(cpu: &mut Cpu) {
    cpu.pc = 0x4c6e;
    cpu.set_hl(0x4cb5);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = crate::defs::LUCKY_NUMBER_SHOW_15 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become next_radio_line(cpu);
}

pub(crate) fn lucky_number_show15(cpu: &mut Cpu) {
    cpu.pc = 0x4c76;
    cpu.set_hl(0x4cba);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = crate::defs::LUCKY_CHANNEL as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become next_radio_line(cpu);
}

pub(crate) fn people_places1(cpu: &mut Cpu) {
    cpu.pc = 0x4cbf;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    start_radio_station(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0x4ce3);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = crate::defs::PLACES_AND_PEOPLE_2 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become next_radio_line(cpu);
}

pub(crate) fn people_places2(cpu: &mut Cpu) {
    cpu.pc = 0x4cca;
    cpu.set_hl(0x4ce8);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = crate::defs::PLACES_AND_PEOPLE_3 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become next_radio_line(cpu);
}

pub(crate) fn people_places3(cpu: &mut Cpu) {
    cpu.pc = 0x4cd2;
    cpu.set_hl(0x4ced);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::random::random(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = { let value = ((((49 as i32) * (0xff as i32)) / (100 as i32)) - (1 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = crate::defs::PLACES_AND_PEOPLE_4 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_c {
        cpu.cycle(12);
        become people_places3_ok(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = crate::defs::PLACES_AND_PEOPLE_6 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    become people_places3_ok(cpu);
}

pub(crate) fn people_places3_ok(cpu: &mut Cpu) {
    cpu.pc = 0x4ce0;
    cpu.cycle(16);
    become next_radio_line(cpu);
}

pub(crate) fn people_places4(cpu: &mut Cpu) {
    cpu.pc = 0x4cf2;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::random::random(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = 127;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
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
    let value = crate::defs::NUM_TRAINER_CLASSES as u8;
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
        become people_places4(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.set_hl(0x4d3e);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xd84c);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((crate::defs::STATUSFLAGS_HALL_OF_FAME_F as u8) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become people_places4_ok(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x4d43);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xd858);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0b11111111;
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
        become people_places4_ok(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x4d4b);
    cpu.pc += 3;
    cpu.cycle(12);
    become people_places4_ok(cpu);
}

pub(crate) fn people_places4_ok(cpu: &mut Cpu) {
    cpu.pc = 0x4d14;
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_de(1 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.push_word(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);
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
        become people_places4(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.push_word(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.set_hl(0x552d);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0x0e;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_de(0xd073);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy_name::copy_name1(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_bc(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.b = 1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x594c);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0x0e;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0x4d51);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = crate::defs::PLACES_AND_PEOPLE_5 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become next_radio_line(cpu);
}

pub(crate) fn people_places5(cpu: &mut Cpu) {
    cpu.pc = 0x4d56;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::random::random(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = 15;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.d = 0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x4d7d);
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
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::random::random(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = { let value = (((4 as i32) * (0xff as i32)) / (100 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = crate::defs::PLACES_AND_PEOPLE as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_c {
        cpu.cycle(12);
        become people_places5_ok(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::random::random(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = { let value = ((((49 as i32) * (0xff as i32)) / (100 as i32)) - (1 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = crate::defs::PLACES_AND_PEOPLE_4 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_c {
        cpu.cycle(12);
        become people_places5_ok(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = crate::defs::PLACES_AND_PEOPLE_6 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    become people_places5_ok(cpu);
}

pub(crate) fn people_places5_ok(cpu: &mut Cpu) {
    cpu.pc = 0x4d7a;
    cpu.cycle(16);
    become next_radio_line(cpu);
}

pub(crate) fn people_places6(cpu: &mut Cpu) {
    cpu.pc = 0x4ded;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::random::random(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = { let value = (((0x4e23 as i32) - (0x4e11 as i32)) / (2 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
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
        become people_places6(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x4e11);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.b = 0;
    cpu.pc += 2;
    cpu.cycle(8);
    let lhs = cpu.hl();
    let rhs = cpu.bc();
    let (result, carry) = lhs.overflowing_add(rhs);
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0fff) + (rhs & 0x0fff) > 0x0fff;
    cpu.flag_c = carry;
    cpu.set_hl(result);
    cpu.pc += 1;
    cpu.cycle(8);
    let lhs = cpu.hl();
    let rhs = cpu.bc();
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
    cpu.c = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map::get_world_map_location(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = 0x72;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x68a5);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0x4e23);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = crate::defs::PLACES_AND_PEOPLE_7 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become next_radio_line(cpu);
}

pub(crate) fn people_places7(cpu: &mut Cpu) {
    cpu.pc = 0x4e28;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::random::random(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = 15;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.d = 0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x4e52);
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
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    copy_radio_text_to_ram(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::random::random(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = { let value = (((4 as i32) * (0xff as i32)) / (100 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = crate::defs::PLACES_AND_PEOPLE as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_c {
        cpu.cycle(12);
        become people_places7_ok(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::random::random(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = { let value = ((((49 as i32) * (0xff as i32)) / (100 as i32)) - (1 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = crate::defs::PLACES_AND_PEOPLE_4 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_c {
        cpu.cycle(12);
        become people_places7_ok(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = crate::defs::PLACES_AND_PEOPLE_6 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    become people_places7_ok(cpu);
}

pub(crate) fn people_places7_ok(cpu: &mut Cpu) {
    cpu.pc = 0x4e4f;
    cpu.cycle(16);
    become print_radio_line(cpu);
}

pub(crate) fn rocket_radio1(cpu: &mut Cpu) {
    cpu.pc = 0x4e72;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    start_radio_station(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0x4ec5);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = crate::defs::ROCKET_RADIO_2 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become next_radio_line(cpu);
}

pub(crate) fn rocket_radio2(cpu: &mut Cpu) {
    cpu.pc = 0x4e7d;
    cpu.set_hl(0x4eca);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = crate::defs::ROCKET_RADIO_3 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become next_radio_line(cpu);
}

pub(crate) fn rocket_radio3(cpu: &mut Cpu) {
    cpu.pc = 0x4e85;
    cpu.set_hl(0x4ecf);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = crate::defs::ROCKET_RADIO_4 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become next_radio_line(cpu);
}

pub(crate) fn rocket_radio4(cpu: &mut Cpu) {
    cpu.pc = 0x4e8d;
    cpu.set_hl(0x4ed4);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = crate::defs::ROCKET_RADIO_5 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become next_radio_line(cpu);
}

pub(crate) fn rocket_radio5(cpu: &mut Cpu) {
    cpu.pc = 0x4e95;
    cpu.set_hl(0x4ed9);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = crate::defs::ROCKET_RADIO_6 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become next_radio_line(cpu);
}

pub(crate) fn rocket_radio6(cpu: &mut Cpu) {
    cpu.pc = 0x4e9d;
    cpu.set_hl(0x4ede);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = crate::defs::ROCKET_RADIO_7 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become next_radio_line(cpu);
}

pub(crate) fn rocket_radio7(cpu: &mut Cpu) {
    cpu.pc = 0x4ea5;
    cpu.set_hl(0x4ee3);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = crate::defs::ROCKET_RADIO_8 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become next_radio_line(cpu);
}

pub(crate) fn rocket_radio8(cpu: &mut Cpu) {
    cpu.pc = 0x4ead;
    cpu.set_hl(0x4ee8);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = crate::defs::ROCKET_RADIO_9 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become next_radio_line(cpu);
}

pub(crate) fn rocket_radio9(cpu: &mut Cpu) {
    cpu.pc = 0x4eb5;
    cpu.set_hl(0x4eed);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = crate::defs::ROCKET_RADIO_10 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become next_radio_line(cpu);
}

pub(crate) fn rocket_radio10(cpu: &mut Cpu) {
    cpu.pc = 0x4ebd;
    cpu.set_hl(0x4ef2);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = crate::defs::ROCKET_RADIO as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become next_radio_line(cpu);
}

pub(crate) fn poke_flute_radio(cpu: &mut Cpu) {
    cpu.pc = 0x4ef7;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    start_radio_station(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xd005, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn unown_radio(cpu: &mut Cpu) {
    cpu.pc = 0x4f00;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    start_radio_station(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xd005, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn evolution_radio(cpu: &mut Cpu) {
    cpu.pc = 0x4f09;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    start_radio_station(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xd005, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn buenas_password1(cpu: &mut Cpu) {
    cpu.pc = 0x4f12;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    buenas_password_check_time(cpu);
    assert_eq!(cpu.pc, next_pc);
    if !cpu.flag_c {
        cpu.cycle(16);
        become buenas_password1_play_password(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xd005);
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
        cpu.cycle(16);
        become buenas_password20(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become buenas_password8(cpu);
}

pub(crate) fn buenas_password1_play_password(cpu: &mut Cpu) {
    cpu.pc = 0x4f22;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    start_radio_station(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xffd4);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.push_word(cpu.af());
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
    cpu.set_de(0x5171);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl({ const N: i32 = ((((9 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (2 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::place_string(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte(0xffd4, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.set_hl(0x5182);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = crate::defs::BUENAS_PASSWORD_2 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become next_radio_line(cpu);
}

pub(crate) fn buenas_password2(cpu: &mut Cpu) {
    cpu.pc = 0x4f3f;
    cpu.set_hl(0x5187);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = crate::defs::BUENAS_PASSWORD_3 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become next_radio_line(cpu);
}

pub(crate) fn buenas_password3(cpu: &mut Cpu) {
    cpu.pc = 0x4f47;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    buenas_password_check_time(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0x518c);
    cpu.pc += 3;
    cpu.cycle(12);
    if cpu.flag_c {
        cpu.cycle(16);
        become buenas_password_after_midnight(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = crate::defs::BUENAS_PASSWORD_4 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become next_radio_line(cpu);
}

pub(crate) fn buenas_password4(cpu: &mut Cpu) {
    cpu.pc = 0x4f55;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    buenas_password_check_time(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.cycle(16);
        become buenas_password8(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xdc4a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xdc1f);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((crate::defs::DAILYFLAGS2_BUENAS_PASSWORD_F as u8) & 7);
    let value = cpu.read_byte(cpu.hl());
    cpu.flag_z = (value & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(12);
    if !cpu.flag_z {
        cpu.cycle(12);
        become buenas_password4_already_got_it(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become buenas_password4_greater_than_11(cpu);
}

pub(crate) fn buenas_password4_greater_than_11(cpu: &mut Cpu) {
    cpu.pc = 0x4f65;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::random::random(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = 15;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = crate::defs::NUM_PASSWORD_CATEGORIES as u8;
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
        become buenas_password4_greater_than_11(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value.rotate_left(4);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    become buenas_password4_greater_than_three(cpu);
}

pub(crate) fn buenas_password4_greater_than_three(cpu: &mut Cpu) {
    cpu.pc = 0x4f71;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::random::random(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = 3;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = crate::defs::NUM_PASSWORDS_PER_CATEGORY as u8;
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
        become buenas_password4_greater_than_three(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.e;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_add(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) > 0x0f;
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xdc4a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xdc1f);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((crate::defs::DAILYFLAGS2_BUENAS_PASSWORD_F as u8) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value |= mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    become buenas_password4_already_got_it(cpu);
}

pub(crate) fn buenas_password4_already_got_it(cpu: &mut Cpu) {
    cpu.pc = 0x4f83;
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    get_buenas_password(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0x5191);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = crate::defs::BUENAS_PASSWORD_5 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become next_radio_line(cpu);
}

pub(crate) fn get_buenas_password(cpu: &mut Cpu) {
    cpu.pc = 0x4f8f;
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.a;
    let result = value.rotate_left(4);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0xf;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x4ff9);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.d = 0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
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
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = 0xf;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.set_hl(0x4fb8);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.e = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
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
    cpu.set_de(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::call_regs::_hl_(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.c = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn get_buenas_password_mon(cpu: &mut Cpu) {
    cpu.pc = 0x4fc0;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    get_buenas_password_get_the_index(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::names::get_pokemon_name(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn get_buenas_password_item(cpu: &mut Cpu) {
    cpu.pc = 0x4fc7;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    get_buenas_password_get_the_index(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::names::get_item_name(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn get_buenas_password_move(cpu: &mut Cpu) {
    cpu.pc = 0x4fce;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    get_buenas_password_get_the_index(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::names::get_move_name(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn get_buenas_password_get_the_index(cpu: &mut Cpu) {
    cpu.pc = 0x4fd5;
    cpu.h = 0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.l = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
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
    cpu.write_byte(0xd265, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn get_buenas_password_raw_string(cpu: &mut Cpu) {
    cpu.pc = 0x4fde;
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
    if cpu.flag_z {
        cpu.cycle(12);
        become get_buenas_password_skip(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become get_buenas_password_read_loop(cpu);
}

pub(crate) fn get_buenas_password_read_loop(cpu: &mut Cpu) {
    cpu.pc = 0x4fe2;
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = { let value = (0x50 as i32); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
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
        become get_buenas_password_read_loop(cpu);
    }
    cpu.pc += 2;
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
        become get_buenas_password_read_loop(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become get_buenas_password_skip(cpu);
}

pub(crate) fn get_buenas_password_skip(cpu: &mut Cpu) {
    cpu.pc = 0x4feb;
    cpu.set_hl(0xd073);
    cpu.pc += 3;
    cpu.cycle(12);
    become get_buenas_password_copy_loop(cpu);
}

pub(crate) fn get_buenas_password_copy_loop(cpu: &mut Cpu) {
    cpu.pc = 0x4fee;
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = { let value = (0x50 as i32); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
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
        become get_buenas_password_copy_loop(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_de(0xd073);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn buenas_password5(cpu: &mut Cpu) {
    cpu.pc = 0x509c;
    cpu.set_hl(0x5196);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = crate::defs::BUENAS_PASSWORD_6 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become next_radio_line(cpu);
}

pub(crate) fn buenas_password6(cpu: &mut Cpu) {
    cpu.pc = 0x50a4;
    cpu.set_hl(0x519b);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = crate::defs::BUENAS_PASSWORD_7 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become next_radio_line(cpu);
}

pub(crate) fn buenas_password7(cpu: &mut Cpu) {
    cpu.pc = 0x50ac;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    buenas_password_check_time(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0x51a0);
    cpu.pc += 3;
    cpu.cycle(12);
    if cpu.flag_c {
        cpu.cycle(12);
        become buenas_password_after_midnight(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = crate::defs::BUENAS_PASSWORD as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become next_radio_line(cpu);
}

pub(crate) fn buenas_password_after_midnight(cpu: &mut Cpu) {
    cpu.pc = 0x50b9;
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.set_hl(0xdc1f);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((crate::defs::DAILYFLAGS2_BUENAS_PASSWORD_F as u8) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.a = crate::defs::BUENAS_PASSWORD_8 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become next_radio_line(cpu);
}

pub(crate) fn buenas_password8(cpu: &mut Cpu) {
    cpu.pc = 0x50c5;
    cpu.set_hl(0xdc1f);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((crate::defs::DAILYFLAGS2_BUENAS_PASSWORD_F as u8) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.set_hl(0x51d2);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = crate::defs::BUENAS_PASSWORD_9 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become next_radio_line(cpu);
}

pub(crate) fn buenas_password9(cpu: &mut Cpu) {
    cpu.pc = 0x50d2;
    cpu.set_hl(0x51a5);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = crate::defs::BUENAS_PASSWORD_10 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become next_radio_line(cpu);
}

pub(crate) fn buenas_password10(cpu: &mut Cpu) {
    cpu.pc = 0x50da;
    cpu.set_hl(0x51aa);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = crate::defs::BUENAS_PASSWORD_11 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become next_radio_line(cpu);
}

pub(crate) fn buenas_password11(cpu: &mut Cpu) {
    cpu.pc = 0x50e2;
    cpu.set_hl(0x51af);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = crate::defs::BUENAS_PASSWORD_12 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become next_radio_line(cpu);
}

pub(crate) fn buenas_password12(cpu: &mut Cpu) {
    cpu.pc = 0x50ea;
    cpu.set_hl(0x51b4);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = crate::defs::BUENAS_PASSWORD_13 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become next_radio_line(cpu);
}

pub(crate) fn buenas_password13(cpu: &mut Cpu) {
    cpu.pc = 0x50f2;
    cpu.set_hl(0x51b9);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = crate::defs::BUENAS_PASSWORD_14 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become next_radio_line(cpu);
}

pub(crate) fn buenas_password14(cpu: &mut Cpu) {
    cpu.pc = 0x50fa;
    cpu.set_hl(0x51be);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = crate::defs::BUENAS_PASSWORD_15 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become next_radio_line(cpu);
}

pub(crate) fn buenas_password15(cpu: &mut Cpu) {
    cpu.pc = 0x5102;
    cpu.set_hl(0x51c3);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = crate::defs::BUENAS_PASSWORD_16 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become next_radio_line(cpu);
}

pub(crate) fn buenas_password16(cpu: &mut Cpu) {
    cpu.pc = 0x510a;
    cpu.set_hl(0x51c8);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = crate::defs::BUENAS_PASSWORD_17 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become next_radio_line(cpu);
}

pub(crate) fn buenas_password17(cpu: &mut Cpu) {
    cpu.pc = 0x5112;
    cpu.set_hl(0x51cd);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = crate::defs::BUENAS_PASSWORD_18 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become next_radio_line(cpu);
}

pub(crate) fn buenas_password18(cpu: &mut Cpu) {
    cpu.pc = 0x511a;
    cpu.set_hl(0x51d2);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = crate::defs::BUENAS_PASSWORD_19 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become next_radio_line(cpu);
}

pub(crate) fn buenas_password19(cpu: &mut Cpu) {
    cpu.pc = 0x5122;
    cpu.set_hl(0x51d2);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = crate::defs::BUENAS_PASSWORD_20 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become next_radio_line(cpu);
}

pub(crate) fn buenas_password20(cpu: &mut Cpu) {
    cpu.pc = 0x512a;
    cpu.a = cpu.read_byte(0xffd4);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = 0x24;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x589d);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x24;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x58a9);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte(0xffd4, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.set_hl(0xdc1f);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((crate::defs::DAILYFLAGS2_BUENAS_PASSWORD_F as u8) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.a = crate::defs::BUENAS_PASSWORD as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xd002, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xd005, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0x51d7);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = crate::defs::BUENAS_PASSWORD_21 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become next_radio_line(cpu);
}

pub(crate) fn buenas_password21(cpu: &mut Cpu) {
    cpu.pc = 0x5152;
    cpu.a = crate::defs::BUENAS_PASSWORD as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xd002, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xd005, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    buenas_password_check_time(cpu);
    assert_eq!(cpu.pc, next_pc);
    if !cpu.flag_c {
        cpu.cycle(16);
        become buenas_password1(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0x51d7);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = crate::defs::BUENAS_PASSWORD_21 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become next_radio_line(cpu);
}

pub(crate) fn buenas_password_check_time(cpu: &mut Cpu) {
    cpu.pc = 0x5169;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::time::update_time(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xff94);
    cpu.pc += 2;
    cpu.cycle(12);
    let value = crate::defs::NITE_HOUR as u8;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn copy_radio_text_to_ram(cpu: &mut Cpu) {
    cpu.pc = 0x51dc;
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = crate::defs::TX_FAR as u8;
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
        become crate::game::home::battle::far_copy_radio_text(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xd00c);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(0x0028);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become crate::game::home::copy::copy_bytes(cpu);
}

pub(crate) fn start_radio_station(cpu: &mut Cpu) {
    cpu.pc = 0x51eb;
    cpu.a = cpu.read_byte(0xd005);
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
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::radio_terminator(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::print_text(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0x520b);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xd002);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.b = 0;
    cpu.pc += 2;
    cpu.cycle(8);
    let lhs = cpu.hl();
    let rhs = cpu.bc();
    let (result, carry) = lhs.overflowing_add(rhs);
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0fff) + (rhs & 0x0fff) > 0x0fff;
    cpu.flag_c = carry;
    cpu.set_hl(result);
    cpu.pc += 1;
    cpu.cycle(8);
    let lhs = cpu.hl();
    let rhs = cpu.bc();
    let (result, carry) = lhs.overflowing_add(rhs);
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0fff) + (rhs & 0x0fff) > 0x0fff;
    cpu.flag_c = carry;
    cpu.set_hl(result);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.e = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.d = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(0x5854);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0x24;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn next_radio_line(cpu: &mut Cpu) {
    cpu.pc = 0x5221;
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    copy_radio_text_to_ram(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.cycle(16);
    become print_radio_line(cpu);
}

pub(crate) fn play_radio_show_ok(cpu: &mut Cpu) {
    match cpu.read_byte(0xd002) {
        0x00 => become oaks_pkmn_talk1(cpu),
        0x01 => become pokedex_show1(cpu),
        0x02 => become ben_mon_music1(cpu),
        0x03 => become lucky_number_show1(cpu),
        0x04 => become buenas_password1(cpu),
        0x05 => become people_places1(cpu),
        0x06 => become fern_mon_music1(cpu),
        0x07 => become rocket_radio1(cpu),
        0x08 => become poke_flute_radio(cpu),
        0x09 => become unown_radio(cpu),
        0x0a => become evolution_radio(cpu),
        0x0b => become oaks_pkmn_talk2(cpu),
        0x0c => become oaks_pkmn_talk3(cpu),
        0x0d => become oaks_pkmn_talk4(cpu),
        0x0e => become oaks_pkmn_talk5(cpu),
        0x0f => become oaks_pkmn_talk6(cpu),
        0x10 => become oaks_pkmn_talk7(cpu),
        0x11 => become oaks_pkmn_talk8(cpu),
        0x12 => become oaks_pkmn_talk9(cpu),
        0x13 => become pokedex_show2(cpu),
        0x14 => become pokedex_show3(cpu),
        0x15 => become pokedex_show4(cpu),
        0x16 => become pokedex_show5(cpu),
        0x17 => become ben_mon_music2(cpu),
        0x18 => become ben_mon_music3(cpu),
        0x19 => become ben_fern_music4(cpu),
        0x1a => become ben_fern_music5(cpu),
        0x1b => become ben_fern_music6(cpu),
        0x1c => become ben_fern_music7(cpu),
        0x1d => become fern_mon_music2(cpu),
        0x1e => become lucky_number_show2(cpu),
        0x1f => become lucky_number_show3(cpu),
        0x20 => become lucky_number_show4(cpu),
        0x21 => become lucky_number_show5(cpu),
        0x22 => become lucky_number_show6(cpu),
        0x23 => become lucky_number_show7(cpu),
        0x24 => become lucky_number_show8(cpu),
        0x25 => become lucky_number_show9(cpu),
        0x26 => become lucky_number_show10(cpu),
        0x27 => become lucky_number_show11(cpu),
        0x28 => become lucky_number_show12(cpu),
        0x29 => become lucky_number_show13(cpu),
        0x2a => become lucky_number_show14(cpu),
        0x2b => become lucky_number_show15(cpu),
        0x2c => become people_places2(cpu),
        0x2d => become people_places3(cpu),
        0x2e => become people_places4(cpu),
        0x2f => become people_places5(cpu),
        0x30 => become people_places6(cpu),
        0x31 => become people_places7(cpu),
        0x32 => become rocket_radio2(cpu),
        0x33 => become rocket_radio3(cpu),
        0x34 => become rocket_radio4(cpu),
        0x35 => become rocket_radio5(cpu),
        0x36 => become rocket_radio6(cpu),
        0x37 => become rocket_radio7(cpu),
        0x38 => become rocket_radio8(cpu),
        0x39 => become rocket_radio9(cpu),
        0x3a => become rocket_radio10(cpu),
        0x3b => become oaks_pkmn_talk10(cpu),
        0x3c => become oaks_pkmn_talk11(cpu),
        0x3d => become oaks_pkmn_talk12(cpu),
        0x3e => become oaks_pkmn_talk13(cpu),
        0x3f => become oaks_pkmn_talk14(cpu),
        0x40 => become buenas_password2(cpu),
        0x41 => become buenas_password3(cpu),
        0x42 => become buenas_password4(cpu),
        0x43 => become buenas_password5(cpu),
        0x44 => become buenas_password6(cpu),
        0x45 => become buenas_password7(cpu),
        0x46 => become buenas_password8(cpu),
        0x47 => become buenas_password9(cpu),
        0x48 => become buenas_password10(cpu),
        0x49 => become buenas_password11(cpu),
        0x4a => become buenas_password12(cpu),
        0x4b => become buenas_password13(cpu),
        0x4c => become buenas_password14(cpu),
        0x4d => become buenas_password15(cpu),
        0x4e => become buenas_password16(cpu),
        0x4f => become buenas_password17(cpu),
        0x50 => become buenas_password18(cpu),
        0x51 => become buenas_password19(cpu),
        0x52 => become buenas_password20(cpu),
        0x53 => become buenas_password21(cpu),
        0x54 => become radio_scroll(cpu),
        0x55 => become pokedex_show6(cpu),
        0x56 => become pokedex_show7(cpu),
        0x57 => become pokedex_show8(cpu),
        n => panic!("unsupported radio line: {}", n),
    }
}
