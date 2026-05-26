use crate::runtime::Cpu;

pub(crate) fn field_move_jumptable_reset(cpu: &mut Cpu) {
    cpu.pc = 0x46ea;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl(0xd1ea);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc({ const N: i32 = (0xd1f1 as i32) - (0xd1ea as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::byte_fill(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn field_move_jumptable(cpu: &mut Cpu) {
    cpu.pc = 0x46f5;
    cpu.a = cpu.read_byte(0xd1ea);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::jump_table(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.write_byte(0xd1ea, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((crate::defs::JUMPTABLE_EXIT_F as u8) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(12);
        become field_move_jumptable_okay(cpu);
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

pub(crate) fn field_move_jumptable_okay(cpu: &mut Cpu) {
    cpu.pc = 0x4702;
    let value = crate::defs::JUMPTABLE_INDEX_MASK as u8;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = true;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn get_party_nickname(cpu: &mut Cpu) {
    cpu.pc = 0x4706;
    cpu.set_hl(0xde41);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = crate::defs::BOXMON as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcf5f, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xd109);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::pokemon::get_nickname(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy_name::copy_name1(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_de(0xd086);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0xd099);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy_name::copy_name2(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn check_engine_flag(cpu: &mut Cpu) {
    cpu.pc = 0x4721;
    cpu.b = crate::defs::CHECK_FLAG as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x20;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x4430);
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
        become check_engine_flag_isset(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = true;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn check_engine_flag_isset(cpu: &mut Cpu) {
    cpu.pc = 0x472f;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn check_badge(cpu: &mut Cpu) {
    cpu.pc = 0x4731;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    check_engine_flag(cpu);
    assert_eq!(cpu.pc, next_pc);
    if !cpu.flag_c {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(0x473d);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::menu::menu_textbox_backup(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = true;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn check_party_move(cpu: &mut Cpu) {
    cpu.pc = 0x4742;
    cpu.e = 0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xd109, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    become check_party_move_loop(cpu);
}

pub(crate) fn check_party_move_loop(cpu: &mut Cpu) {
    cpu.pc = 0x4748;
    cpu.c = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.b = 0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xdcd8);
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
        become check_party_move_no(cpu);
    }
    cpu.pc += 2;
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
        become check_party_move_no(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
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
        become check_party_move_next(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_bc(crate::defs::PARTYMON_STRUCT_LENGTH as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0xdce1);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::array::add_n_times(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.b = crate::defs::NUM_MOVES as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    become check_party_move_check(cpu);
}

pub(crate) fn check_party_move_check(cpu: &mut Cpu) {
    cpu.pc = 0x4767;
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.d;
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
        become check_party_move_yes(cpu);
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
        become check_party_move_check(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become check_party_move_next(cpu);
}

pub(crate) fn check_party_move_next(cpu: &mut Cpu) {
    cpu.pc = 0x476e;
    let value = cpu.e;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.e = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.cycle(12);
    become check_party_move_loop(cpu);
}

pub(crate) fn check_party_move_yes(cpu: &mut Cpu) {
    cpu.pc = 0x4771;
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xd109, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn check_party_move_no(cpu: &mut Cpu) {
    cpu.pc = 0x4777;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = true;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn field_move_failed(cpu: &mut Cpu) {
    cpu.pc = 0x4779;
    cpu.set_hl(0x4780);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::menu::menu_textbox_backup(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn cut_function(cpu: &mut Cpu) {
    cpu.pc = 0x4785;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    field_move_jumptable_reset(cpu);
    assert_eq!(cpu.pc, next_pc);
    become cut_function_loop(cpu);
}

pub(crate) fn cut_function_loop(cpu: &mut Cpu) {
    cpu.pc = 0x4788;
    cpu.set_hl(0x4796);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    field_move_jumptable(cpu);
    assert_eq!(cpu.pc, next_pc);
    if !cpu.flag_c {
        cpu.cycle(12);
        become cut_function_loop(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = crate::defs::JUMPTABLE_INDEX_MASK as u8;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xd0ec, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn cut_function_check_able(cpu: &mut Cpu) {
    cpu.pc = 0x479c;
    cpu.set_de(crate::defs::ENGINE_HIVEBADGE as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    check_badge(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.cycle(12);
        become cut_function_nohivebadge(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    check_map_for_something_to_cut(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.cycle(12);
        become cut_function_nothingtocut(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn cut_function_nohivebadge(cpu: &mut Cpu) {
    cpu.pc = 0x47ac;
    cpu.a = crate::defs::JUMPTABLE_EXIT as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn cut_function_nothingtocut(cpu: &mut Cpu) {
    cpu.pc = 0x47af;
    cpu.a = 0x2;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn cut_function_do_cut(cpu: &mut Cpu) {
    cpu.pc = 0x47b2;
    cpu.set_hl(0x47fe);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::queue_script::queue_script(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = { let value = ((crate::defs::JUMPTABLE_EXIT as i32) | (0x1 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn cut_function_fail_cut(cpu: &mut Cpu) {
    cpu.pc = 0x47bb;
    cpu.set_hl(0x47c9);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::menu::menu_textbox_backup(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = crate::defs::JUMPTABLE_EXIT as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn check_map_for_something_to_cut(cpu: &mut Cpu) {
    cpu.pc = 0x47ce;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map::get_facing_tile_coord(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.push_word(cpu.de());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = 0x05;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x49f5);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_de(value);
    cpu.pc += 1;
    cpu.cycle(12);
    if !cpu.flag_c {
        cpu.cycle(12);
        become check_map_for_something_to_cut_fail(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map::get_block_location(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.c = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.set_hl(0x4862);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    check_overworld_tile_arrays(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    if !cpu.flag_c {
        cpu.cycle(12);
        become check_map_for_something_to_cut_fail(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xd1ec, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte({ const N: i32 = (0xd1ec as i32) + (1 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xd1ee, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xd1ef, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn check_map_for_something_to_cut_fail(cpu: &mut Cpu) {
    cpu.pc = 0x47fc;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = true;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn cut_down_tree_or_grass(cpu: &mut Cpu) {
    cpu.pc = 0x4810;
    cpu.set_hl(0xd1ec);
    cpu.pc += 3;
    cpu.cycle(12);
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
    cpu.a = cpu.read_byte(0xd1ee);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
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
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map::load_overworld_tilemap_and_attrmap_pals(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map_objects::update_sprites(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::delay::delay_frame(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xd1ef);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = 0x23;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x4940);
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
    crate::game::home::map::buffer_screen(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map::get_movement_permissions(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map_objects::update_sprites(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::delay::delay_frame(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::gfx::load_standard_font(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn check_overworld_tile_arrays(cpu: &mut Cpu) {
    cpu.pc = 0x4840;
    cpu.push_word(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xd199);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_de(3 as u16);
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
    if !cpu.flag_c {
        cpu.cycle(12);
        become check_overworld_tile_arrays_nope(cpu);
    }
    cpu.pc += 2;
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
    cpu.set_de(3 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::array::is_in_array(cpu);
    assert_eq!(cpu.pc, next_pc);
    if !cpu.flag_c {
        cpu.cycle(12);
        become check_overworld_tile_arrays_nope(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
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
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = true;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn check_overworld_tile_arrays_nope(cpu: &mut Cpu) {
    cpu.pc = 0x4860;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn flash_function(cpu: &mut Cpu) {
    cpu.pc = 0x48ac;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    flash_function_check_use_flash(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = crate::defs::JUMPTABLE_INDEX_MASK as u8;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xd0ec, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn flash_function_check_use_flash(cpu: &mut Cpu) {
    cpu.pc = 0x48b5;
    cpu.set_de(crate::defs::ENGINE_ZEPHYRBADGE as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0x03;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x4731);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.cycle(12);
        become flash_function_nozephyrbadge(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = 0x22;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x6e30);
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
        become flash_function_useflash(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xd847);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = crate::defs::DARKNESS_PALSET as u8;
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
        become flash_function_notadarkcave(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become flash_function_useflash(cpu);
}

pub(crate) fn flash_function_useflash(cpu: &mut Cpu) {
    cpu.pc = 0x48d1;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    use_flash(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = { let value = ((crate::defs::JUMPTABLE_EXIT as i32) | (0x1 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn flash_function_notadarkcave(cpu: &mut Cpu) {
    cpu.pc = 0x48d7;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    field_move_failed(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = crate::defs::JUMPTABLE_EXIT as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn flash_function_nozephyrbadge(cpu: &mut Cpu) {
    cpu.pc = 0x48dd;
    cpu.a = crate::defs::JUMPTABLE_EXIT as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn use_flash(cpu: &mut Cpu) {
    cpu.pc = 0x48e0;
    cpu.set_hl(0x48e6);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become crate::game::home::queue_script::queue_script(cpu);
}

pub(crate) fn use_flash_text_script_text_asm_0(cpu: &mut Cpu) {
    cpu.pc = 0x48f8;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::audio::wait_sfx(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_de(crate::defs::SFX_FLASH as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::audio::play_sfx(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::audio::wait_sfx(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0x4908);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn surf_function(cpu: &mut Cpu) {
    cpu.pc = 0x4909;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    field_move_jumptable_reset(cpu);
    assert_eq!(cpu.pc, next_pc);
    become surf_function_loop(cpu);
}

pub(crate) fn surf_function_loop(cpu: &mut Cpu) {
    cpu.pc = 0x490c;
    cpu.set_hl(0x491a);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    field_move_jumptable(cpu);
    assert_eq!(cpu.pc, next_pc);
    if !cpu.flag_c {
        cpu.cycle(12);
        become surf_function_loop(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = crate::defs::JUMPTABLE_INDEX_MASK as u8;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xd0ec, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn surf_function_try_surf(cpu: &mut Cpu) {
    cpu.pc = 0x4922;
    cpu.set_de(crate::defs::ENGINE_FOGBADGE as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    check_badge(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.cycle(12);
        become surf_function_nofogbadge(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xdbf5);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((crate::defs::BIKEFLAGS_ALWAYS_ON_BIKE_F as u8) & 7);
    let value = cpu.read_byte(cpu.hl());
    cpu.flag_z = (value & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(12);
    if !cpu.flag_z {
        cpu.cycle(12);
        become surf_function_cannotsurf(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xd95d);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = crate::defs::PLAYER_SURF as u8;
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
        become surf_function_alreadyfail(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = crate::defs::PLAYER_SURF_PIKA as u8;
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
        become surf_function_alreadyfail(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map::get_facing_tile_coord(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map_objects::get_tile_permission(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = crate::defs::WATER_TILE as u8;
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
        become surf_function_cannotsurf(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    check_direction(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.cycle(12);
        become surf_function_cannotsurf(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x01;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x6fd9);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.cycle(12);
        become surf_function_cannotsurf(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn surf_function_nofogbadge(cpu: &mut Cpu) {
    cpu.pc = 0x4956;
    cpu.a = crate::defs::JUMPTABLE_EXIT as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn surf_function_alreadyfail(cpu: &mut Cpu) {
    cpu.pc = 0x4959;
    cpu.a = 0x3;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn surf_function_cannotsurf(cpu: &mut Cpu) {
    cpu.pc = 0x495c;
    cpu.a = 0x2;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn surf_function_do_surf(cpu: &mut Cpu) {
    cpu.pc = 0x495f;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    get_surf_type(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.write_byte(0xd1eb, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    get_party_nickname(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0x4983);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::queue_script::queue_script(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = { let value = ((crate::defs::JUMPTABLE_EXIT as i32) | (0x1 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn surf_function_fail_surf(cpu: &mut Cpu) {
    cpu.pc = 0x4971;
    cpu.set_hl(0x49ae);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::menu::menu_textbox_backup(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = crate::defs::JUMPTABLE_EXIT as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn surf_function_already_surfing(cpu: &mut Cpu) {
    cpu.pc = 0x497a;
    cpu.set_hl(0x49b3);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::menu::menu_textbox_backup(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = crate::defs::JUMPTABLE_EXIT as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn used_surf_script_stubbed_fn(cpu: &mut Cpu) {
    cpu.pc = 0x49a2;
    cpu.a = 0x41;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x60bb);
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

pub(crate) fn get_surf_type(cpu: &mut Cpu) {
    cpu.pc = 0x49b8;
    cpu.a = cpu.read_byte(0xd109);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.d = 0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xdcd8);
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
    let value = crate::defs::PIKACHU as u8;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = crate::defs::PLAYER_SURF_PIKA as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = crate::defs::PLAYER_SURF as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn check_direction(cpu: &mut Cpu) {
    cpu.pc = 0x49cb;
    cpu.a = cpu.read_byte(0xd4de);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0b00001100;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value.rotate_right(1);
    cpu.flag_z = false;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x01 != 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.a;
    let result = value.rotate_right(1);
    cpu.flag_z = false;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x01 != 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.d = 0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x49e3);
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
    cpu.a = cpu.read_byte(0xc2fe);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.read_byte(cpu.hl());
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(12);
        become check_direction_quit(cpu);
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
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn check_direction_quit(cpu: &mut Cpu) {
    cpu.pc = 0x49e1;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = true;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub fn try_surf_ow(cpu: &mut Cpu) {
    cpu.pc = 0x49e7;
    cpu.a = cpu.read_byte(0xd95d);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = crate::defs::PLAYER_SURF_PIKA as u8;
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
        become try_surf_ow_quit(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = crate::defs::PLAYER_SURF as u8;
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
        become try_surf_ow_quit(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xd03e);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map_objects::get_tile_permission(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = crate::defs::WATER_TILE as u8;
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
        become try_surf_ow_quit(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    check_direction(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.cycle(12);
        become try_surf_ow_quit(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_de(crate::defs::ENGINE_FOGBADGE as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    check_engine_flag(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.cycle(12);
        become try_surf_ow_quit(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.d = crate::defs::SURF as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    check_party_move(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.cycle(12);
        become try_surf_ow_quit(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xdbf5);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((crate::defs::BIKEFLAGS_ALWAYS_ON_BIKE_F as u8) & 7);
    let value = cpu.read_byte(cpu.hl());
    cpu.flag_z = (value & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(12);
    if !cpu.flag_z {
        cpu.cycle(12);
        become try_surf_ow_quit(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    get_surf_type(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.write_byte(0xd1eb, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    get_party_nickname(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x03;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x4a2c);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map::call_script(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = true;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn try_surf_ow_quit(cpu: &mut Cpu) {
    cpu.pc = 0x4a2a;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn fly_function(cpu: &mut Cpu) {
    cpu.pc = 0x4a3b;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    field_move_jumptable_reset(cpu);
    assert_eq!(cpu.pc, next_pc);
    become fly_function_loop(cpu);
}

pub(crate) fn fly_function_loop(cpu: &mut Cpu) {
    cpu.pc = 0x4a3e;
    cpu.set_hl(0x4a4c);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    field_move_jumptable(cpu);
    assert_eq!(cpu.pc, next_pc);
    if !cpu.flag_c {
        cpu.cycle(12);
        become fly_function_loop(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = crate::defs::JUMPTABLE_INDEX_MASK as u8;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xd0ec, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn fly_function_try_fly(cpu: &mut Cpu) {
    cpu.pc = 0x4a52;
    cpu.set_de(crate::defs::ENGINE_STORMBADGE as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    check_badge(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.cycle(12);
        become fly_function_nostormbadge(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map::get_map_environment(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map::check_outdoor_map(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_z {
        cpu.cycle(12);
        become fly_function_outdoors(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become fly_function_indoors(cpu);
}

pub(crate) fn fly_function_outdoors(cpu: &mut Cpu) {
    cpu.pc = 0x4a64;
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
    crate::game::home::menu::load_standard_menu_header(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::clear_sprites::clear_sprites(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x24;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x5af3);
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
        become fly_function_illegal(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = crate::defs::NUM_SPAWNS as u8;
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
        become fly_function_illegal(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xd001, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::menu::close_window(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn fly_function_nostormbadge(cpu: &mut Cpu) {
    cpu.pc = 0x4a85;
    cpu.a = { let value = ((crate::defs::JUMPTABLE_EXIT as i32) | (0x2 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn fly_function_indoors(cpu: &mut Cpu) {
    cpu.pc = 0x4a88;
    cpu.a = 0x2;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn fly_function_illegal(cpu: &mut Cpu) {
    cpu.pc = 0x4a8b;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::menu::close_window(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::tilemap::wait_bg_map(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = crate::defs::JUMPTABLE_EXIT as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn fly_function_do_fly(cpu: &mut Cpu) {
    cpu.pc = 0x4a94;
    cpu.set_hl(0x4aa3);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::queue_script::queue_script(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = { let value = ((crate::defs::JUMPTABLE_EXIT as i32) | (0x1 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn fly_function_fail_fly(cpu: &mut Cpu) {
    cpu.pc = 0x4a9d;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    field_move_failed(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = { let value = ((crate::defs::JUMPTABLE_EXIT as i32) | (0x2 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn fly_function_return_from_fly(cpu: &mut Cpu) {
    cpu.pc = 0x4acb;
    cpu.a = 0x01;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x561d);
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
    crate::game::home::delay::delay_frame(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::gfx::update_player_sprite(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x41;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x6594);
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

pub(crate) fn waterfall_function(cpu: &mut Cpu) {
    cpu.pc = 0x4ade;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    waterfall_function_try_waterfall(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = crate::defs::JUMPTABLE_INDEX_MASK as u8;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xd0ec, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn waterfall_function_try_waterfall(cpu: &mut Cpu) {
    cpu.pc = 0x4ae7;
    cpu.set_de(crate::defs::ENGINE_RISINGBADGE as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0x03;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x4731);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = crate::defs::JUMPTABLE_EXIT as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_c {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    check_map_can_waterfall(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.cycle(12);
        become waterfall_function_failed(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x4b1c);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::queue_script::queue_script(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = { let value = ((crate::defs::JUMPTABLE_EXIT as i32) | (0x1 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn waterfall_function_failed(cpu: &mut Cpu) {
    cpu.pc = 0x4b01;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    field_move_failed(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = crate::defs::JUMPTABLE_EXIT as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn check_map_can_waterfall(cpu: &mut Cpu) {
    cpu.pc = 0x4b07;
    cpu.a = cpu.read_byte(0xd4de);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0xc;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = crate::defs::FACE_UP as u8;
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
        become check_map_can_waterfall_failed(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc2fb);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map_objects::check_waterfall_tile(cpu);
    assert_eq!(cpu.pc, next_pc);
    if !cpu.flag_z {
        cpu.cycle(12);
        become check_map_can_waterfall_failed(cpu);
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
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn check_map_can_waterfall_failed(cpu: &mut Cpu) {
    cpu.pc = 0x4b1a;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = true;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn script_used_waterfall_check_continue_waterfall(cpu: &mut Cpu) {
    cpu.pc = 0x4b38;
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
    cpu.a = cpu.read_byte(0xd4e4);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map_objects::check_waterfall_tile(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_z {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 0x41;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x60c1);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc2dd, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub fn try_waterfall_ow(cpu: &mut Cpu) {
    cpu.pc = 0x4b56;
    cpu.d = crate::defs::WATERFALL as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    check_party_move(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.cycle(12);
        become try_waterfall_ow_failed(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_de(crate::defs::ENGINE_RISINGBADGE as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    check_engine_flag(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.cycle(12);
        become try_waterfall_ow_failed(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    check_map_can_waterfall(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.cycle(12);
        become try_waterfall_ow_failed(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x03;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x4b86);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map::call_script(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = true;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn try_waterfall_ow_failed(cpu: &mut Cpu) {
    cpu.pc = 0x4b74;
    cpu.a = 0x03;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x4b7e);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map::call_script(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = true;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn escape_rope_function(cpu: &mut Cpu) {
    cpu.pc = 0x4b95;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    field_move_jumptable_reset(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become escape_rope_or_dig(cpu);
}

pub(crate) fn dig_function(cpu: &mut Cpu) {
    cpu.pc = 0x4b9c;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    field_move_jumptable_reset(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x2;
    cpu.pc += 2;
    cpu.cycle(8);
    become escape_rope_or_dig(cpu);
}

pub(crate) fn escape_rope_or_dig(cpu: &mut Cpu) {
    cpu.pc = 0x4ba1;
    cpu.write_byte(0xd1eb, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    become escape_rope_or_dig_loop(cpu);
}

pub(crate) fn escape_rope_or_dig_loop(cpu: &mut Cpu) {
    cpu.pc = 0x4ba4;
    cpu.set_hl(0x4bb2);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    field_move_jumptable(cpu);
    assert_eq!(cpu.pc, next_pc);
    if !cpu.flag_c {
        cpu.cycle(12);
        become escape_rope_or_dig_loop(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = crate::defs::JUMPTABLE_INDEX_MASK as u8;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xd0ec, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn escape_rope_or_dig_check_can_dig(cpu: &mut Cpu) {
    cpu.pc = 0x4bb8;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map::get_map_environment(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = crate::defs::CAVE as u8;
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
        become escape_rope_or_dig_incave(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = crate::defs::DUNGEON as u8;
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
        become escape_rope_or_dig_incave(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become escape_rope_or_dig_fail(cpu);
}

pub(crate) fn escape_rope_or_dig_fail(cpu: &mut Cpu) {
    cpu.pc = 0x4bc3;
    cpu.a = 0x2;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn escape_rope_or_dig_incave(cpu: &mut Cpu) {
    cpu.pc = 0x4bc6;
    cpu.set_hl(0xdca9);
    cpu.pc += 3;
    cpu.cycle(12);
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
    if cpu.flag_z {
        cpu.cycle(12);
        become escape_rope_or_dig_fail(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
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
    if cpu.flag_z {
        cpu.cycle(12);
        become escape_rope_or_dig_fail(cpu);
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
    if cpu.flag_z {
        cpu.cycle(12);
        become escape_rope_or_dig_fail(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn escape_rope_or_dig_do_dig(cpu: &mut Cpu) {
    cpu.pc = 0x4bd8;
    cpu.set_hl(0xdca9);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xd146);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(3 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    get_party_nickname(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xd1eb);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x2;
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
        become escape_rope_or_dig_escaperope(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x4c35);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::queue_script::queue_script(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = { let value = ((crate::defs::JUMPTABLE_EXIT as i32) | (0x1 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn escape_rope_or_dig_escaperope(cpu: &mut Cpu) {
    cpu.pc = 0x4bf7;
    cpu.a = 0x22;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x6e4e);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0x4c2b);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::queue_script::queue_script(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = { let value = ((crate::defs::JUMPTABLE_EXIT as i32) | (0x1 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn escape_rope_or_dig_fail_dig(cpu: &mut Cpu) {
    cpu.pc = 0x4c06;
    cpu.a = cpu.read_byte(0xd1eb);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x2;
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
        become escape_rope_or_dig_failescaperope(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x4c26);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::menu::menu_textbox(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::joypad::wait_press_aor_b_blink_cursor(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::menu::close_window(cpu);
    assert_eq!(cpu.pc, next_pc);
    become escape_rope_or_dig_failescaperope(cpu);
}

pub(crate) fn escape_rope_or_dig_failescaperope(cpu: &mut Cpu) {
    cpu.pc = 0x4c19;
    cpu.a = crate::defs::JUMPTABLE_EXIT as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn teleport_function(cpu: &mut Cpu) {
    cpu.pc = 0x4c61;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    field_move_jumptable_reset(cpu);
    assert_eq!(cpu.pc, next_pc);
    become teleport_function_loop(cpu);
}

pub(crate) fn teleport_function_loop(cpu: &mut Cpu) {
    cpu.pc = 0x4c64;
    cpu.set_hl(0x4c72);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    field_move_jumptable(cpu);
    assert_eq!(cpu.pc, next_pc);
    if !cpu.flag_c {
        cpu.cycle(12);
        become teleport_function_loop(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = crate::defs::JUMPTABLE_INDEX_MASK as u8;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xd0ec, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn teleport_function_try_teleport(cpu: &mut Cpu) {
    cpu.pc = 0x4c78;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map::get_map_environment(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map::check_outdoor_map(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_z {
        cpu.cycle(12);
        become teleport_function_check_if_spawn_point(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become teleport_function_nope(cpu);
}

pub(crate) fn teleport_function_check_if_spawn_point(cpu: &mut Cpu) {
    cpu.pc = 0x4c82;
    cpu.a = cpu.read_byte(0xdcb2);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.d = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xdcb3);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = 0x05;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x5344);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    if !cpu.flag_c {
        cpu.cycle(12);
        become teleport_function_nope(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xd001, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn teleport_function_nope(cpu: &mut Cpu) {
    cpu.pc = 0x4c99;
    cpu.a = 0x2;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn teleport_function_do_teleport(cpu: &mut Cpu) {
    cpu.pc = 0x4c9c;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    get_party_nickname(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0x4cbb);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::queue_script::queue_script(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = { let value = ((crate::defs::JUMPTABLE_EXIT as i32) | (0x1 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn teleport_function_fail_teleport(cpu: &mut Cpu) {
    cpu.pc = 0x4ca8;
    cpu.set_hl(0x4cb6);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::menu::menu_textbox_backup(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = crate::defs::JUMPTABLE_EXIT as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn strength_function(cpu: &mut Cpu) {
    cpu.pc = 0x4ce5;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    strength_function_try_strength(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = crate::defs::JUMPTABLE_INDEX_MASK as u8;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xd0ec, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn strength_function_try_strength(cpu: &mut Cpu) {
    cpu.pc = 0x4cee;
    cpu.set_de(crate::defs::ENGINE_PLAINBADGE as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    check_badge(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.cycle(12);
        become strength_function_failed(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become strength_function_use_strength(cpu);
}

pub(crate) fn strength_function_already_using_strength(cpu: &mut Cpu) {
    cpu.pc = 0x4cf8;
    cpu.set_hl(0x4d01);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::menu::menu_textbox_backup(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = crate::defs::JUMPTABLE_EXIT as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn strength_function_failed(cpu: &mut Cpu) {
    cpu.pc = 0x4d06;
    cpu.a = crate::defs::JUMPTABLE_EXIT as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn strength_function_use_strength(cpu: &mut Cpu) {
    cpu.pc = 0x4d09;
    cpu.set_hl(0x4d29);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::queue_script::queue_script(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = { let value = ((crate::defs::JUMPTABLE_EXIT as i32) | (0x1 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn set_strength_flag(cpu: &mut Cpu) {
    cpu.pc = 0x4d12;
    cpu.set_hl(0xdbf5);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((crate::defs::BIKEFLAGS_STRENGTH_ACTIVE_F as u8) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value |= mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xd109);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.d = 0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xdcd8);
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
    cpu.write_byte(0xd1ef, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    get_party_nickname(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn try_strength_ow(cpu: &mut Cpu) {
    cpu.pc = 0x4d78;
    cpu.d = crate::defs::STRENGTH as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    check_party_move(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.cycle(12);
        become try_strength_ow_nope(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_de(crate::defs::ENGINE_PLAINBADGE as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    check_engine_flag(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.cycle(12);
        become try_strength_ow_nope(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xdbf5);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((crate::defs::BIKEFLAGS_STRENGTH_ACTIVE_F as u8) & 7);
    let value = cpu.read_byte(cpu.hl());
    cpu.flag_z = (value & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(12);
    if cpu.flag_z {
        cpu.cycle(12);
        become try_strength_ow_already_using(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 2;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become try_strength_ow_done(cpu);
}

pub(crate) fn try_strength_ow_nope(cpu: &mut Cpu) {
    cpu.pc = 0x4d92;
    cpu.a = 1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become try_strength_ow_done(cpu);
}

pub(crate) fn try_strength_ow_already_using(cpu: &mut Cpu) {
    cpu.pc = 0x4d96;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.cycle(12);
    become try_strength_ow_done(cpu);
}

pub(crate) fn try_strength_ow_done(cpu: &mut Cpu) {
    cpu.pc = 0x4d99;
    cpu.write_byte(0xc2dd, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn whirlpool_function(cpu: &mut Cpu) {
    cpu.pc = 0x4d9d;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    field_move_jumptable_reset(cpu);
    assert_eq!(cpu.pc, next_pc);
    become whirlpool_function_loop(cpu);
}

pub(crate) fn whirlpool_function_loop(cpu: &mut Cpu) {
    cpu.pc = 0x4da0;
    cpu.set_hl(0x4dae);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    field_move_jumptable(cpu);
    assert_eq!(cpu.pc, next_pc);
    if !cpu.flag_c {
        cpu.cycle(12);
        become whirlpool_function_loop(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = crate::defs::JUMPTABLE_INDEX_MASK as u8;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xd0ec, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn whirlpool_function_try_whirlpool(cpu: &mut Cpu) {
    cpu.pc = 0x4db4;
    cpu.set_de(crate::defs::ENGINE_GLACIERBADGE as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    check_badge(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.cycle(12);
        become whirlpool_function_noglacierbadge(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    try_whirlpool_menu(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.cycle(12);
        become whirlpool_function_failed(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn whirlpool_function_failed(cpu: &mut Cpu) {
    cpu.pc = 0x4dc4;
    cpu.a = 0x2;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn whirlpool_function_noglacierbadge(cpu: &mut Cpu) {
    cpu.pc = 0x4dc7;
    cpu.a = crate::defs::JUMPTABLE_EXIT as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn whirlpool_function_do_whirlpool(cpu: &mut Cpu) {
    cpu.pc = 0x4dca;
    cpu.set_hl(0x4e0b);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::queue_script::queue_script(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = { let value = ((crate::defs::JUMPTABLE_EXIT as i32) | (0x1 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn whirlpool_function_fail_whirlpool(cpu: &mut Cpu) {
    cpu.pc = 0x4dd3;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    field_move_failed(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = crate::defs::JUMPTABLE_EXIT as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn try_whirlpool_menu(cpu: &mut Cpu) {
    cpu.pc = 0x4dde;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map::get_facing_tile_coord(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.push_word(cpu.de());
    cpu.pc += 1;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map_objects::check_whirlpool_tile(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_de(value);
    cpu.pc += 1;
    cpu.cycle(12);
    if cpu.flag_c {
        cpu.cycle(12);
        become try_whirlpool_menu_failed(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map::get_block_location(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.c = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.set_hl(0x48a4);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    check_overworld_tile_arrays(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    if !cpu.flag_c {
        cpu.cycle(12);
        become try_whirlpool_menu_failed(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xd1ec, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte({ const N: i32 = (0xd1ec as i32) + (1 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xd1ee, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xd1ef, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn try_whirlpool_menu_failed(cpu: &mut Cpu) {
    cpu.pc = 0x4e09;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = true;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn disappear_whirlpool(cpu: &mut Cpu) {
    cpu.pc = 0x4e1d;
    cpu.set_hl(0xd1ec);
    cpu.pc += 3;
    cpu.cycle(12);
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
    cpu.a = cpu.read_byte(0xd1ee);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
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
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map::load_overworld_tilemap_and_attrmap_pals(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xd1ef);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = 0x23;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x47d4);
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
    crate::game::home::map::buffer_screen(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map::get_movement_permissions(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub fn try_whirlpool_ow(cpu: &mut Cpu) {
    cpu.pc = 0x4e3e;
    cpu.d = crate::defs::WHIRLPOOL as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    check_party_move(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.cycle(12);
        become try_whirlpool_ow_failed(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_de(crate::defs::ENGINE_GLACIERBADGE as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    check_engine_flag(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.cycle(12);
        become try_whirlpool_ow_failed(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    try_whirlpool_menu(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.cycle(12);
        become try_whirlpool_ow_failed(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x03;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x4e6e);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map::call_script(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = true;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn try_whirlpool_ow_failed(cpu: &mut Cpu) {
    cpu.pc = 0x4e5c;
    cpu.a = 0x03;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x4e66);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map::call_script(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = true;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn headbutt_function(cpu: &mut Cpu) {
    cpu.pc = 0x4e7d;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    try_headbutt_from_menu(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = crate::defs::JUMPTABLE_INDEX_MASK as u8;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xd0ec, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn try_headbutt_from_menu(cpu: &mut Cpu) {
    cpu.pc = 0x4e86;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map::get_facing_tile_coord(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map_objects::check_headbutt_tree_tile(cpu);
    assert_eq!(cpu.pc, next_pc);
    if !cpu.flag_z {
        cpu.cycle(12);
        become try_headbutt_from_menu_no_tree(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x4ea7);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::queue_script::queue_script(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = { let value = ((crate::defs::JUMPTABLE_EXIT as i32) | (0x1 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn try_headbutt_from_menu_no_tree(cpu: &mut Cpu) {
    cpu.pc = 0x4e97;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    field_move_failed(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = crate::defs::JUMPTABLE_EXIT as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub fn try_headbutt_ow(cpu: &mut Cpu) {
    cpu.pc = 0x4ec9;
    cpu.d = crate::defs::HEADBUTT as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    check_party_move(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.cycle(12);
        become try_headbutt_ow_no(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x03;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x4edc);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map::call_script(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = true;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn try_headbutt_ow_no(cpu: &mut Cpu) {
    cpu.pc = 0x4eda;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn rock_smash_function(cpu: &mut Cpu) {
    cpu.pc = 0x4eeb;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    try_rock_smash_from_menu(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = crate::defs::JUMPTABLE_INDEX_MASK as u8;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xd0ec, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn try_rock_smash_from_menu(cpu: &mut Cpu) {
    cpu.pc = 0x4ef4;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    get_facing_object(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.cycle(12);
        become try_rock_smash_from_menu_no_rock(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = crate::defs::SPRITEMOVEDATA_SMASHABLE_ROCK as u8;
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
        become try_rock_smash_from_menu_no_rock(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x4f2e);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::queue_script::queue_script(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = { let value = ((crate::defs::JUMPTABLE_EXIT as i32) | (0x1 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn try_rock_smash_from_menu_no_rock(cpu: &mut Cpu) {
    cpu.pc = 0x4f07;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    field_move_failed(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = crate::defs::JUMPTABLE_EXIT as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn get_facing_object(cpu: &mut Cpu) {
    cpu.pc = 0x4f0d;
    cpu.a = 0x01;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x6fd9);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    if !cpu.flag_c {
        cpu.cycle(12);
        become get_facing_object_fail(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xffb0);
    cpu.pc += 2;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map_objects::get_object_struct(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(crate::defs::OBJECT_MAP_OBJECT_INDEX as u16);
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
    cpu.write_byte(0xffe0, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map_objects::get_map_object(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(crate::defs::MAPOBJECT_MOVEMENT as u16);
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
    cpu.d = cpu.a;
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
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn get_facing_object_fail(cpu: &mut Cpu) {
    cpu.pc = 0x4f2c;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = true;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn has_rock_smash(cpu: &mut Cpu) {
    cpu.pc = 0x4f7c;
    cpu.d = crate::defs::ROCK_SMASH as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    check_party_move(cpu);
    assert_eq!(cpu.pc, next_pc);
    if !cpu.flag_c {
        cpu.cycle(12);
        become has_rock_smash_yes(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become has_rock_smash_done(cpu);
}

pub(crate) fn has_rock_smash_yes(cpu: &mut Cpu) {
    cpu.pc = 0x4f87;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.cycle(12);
    become has_rock_smash_done(cpu);
}

pub(crate) fn has_rock_smash_done(cpu: &mut Cpu) {
    cpu.pc = 0x4f8a;
    cpu.write_byte(0xc2dd, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn fish_function(cpu: &mut Cpu) {
    cpu.pc = 0x4f8e;
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    field_move_jumptable_reset(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte(0xd1eb, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    become fish_function_loop(cpu);
}

pub(crate) fn fish_function_loop(cpu: &mut Cpu) {
    cpu.pc = 0x4f97;
    cpu.set_hl(0x4fa5);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    field_move_jumptable(cpu);
    assert_eq!(cpu.pc, next_pc);
    if !cpu.flag_c {
        cpu.cycle(12);
        become fish_function_loop(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = crate::defs::JUMPTABLE_INDEX_MASK as u8;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xd0ec, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn fish_function_try_fish(cpu: &mut Cpu) {
    cpu.pc = 0x4faf;
    cpu.a = cpu.read_byte(0xd95d);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = crate::defs::PLAYER_SURF as u8;
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
        become fish_function_fail(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = crate::defs::PLAYER_SURF_PIKA as u8;
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
        become fish_function_fail(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map::get_facing_tile_coord(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map_objects::get_tile_permission(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = crate::defs::WATER_TILE as u8;
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
        become fish_function_facingwater(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become fish_function_fail(cpu);
}

pub(crate) fn fish_function_fail(cpu: &mut Cpu) {
    cpu.pc = 0x4fc4;
    cpu.a = 0x3;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn fish_function_facingwater(cpu: &mut Cpu) {
    cpu.pc = 0x4fc7;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map::get_fishing_group(cpu);
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
        become fish_function_goodtofish(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x4;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn fish_function_goodtofish(cpu: &mut Cpu) {
    cpu.pc = 0x4fd0;
    cpu.d = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xd1eb);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = 0x24;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x6402);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.d;
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
        become fish_function_nonibble(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xd22e, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xd143, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = crate::defs::BATTLETYPE_FISH as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xd230, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x2;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn fish_function_nonibble(cpu: &mut Cpu) {
    cpu.pc = 0x4fee;
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn fish_function_fail_fish(cpu: &mut Cpu) {
    cpu.pc = 0x4ff1;
    cpu.a = crate::defs::JUMPTABLE_EXIT as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn fish_function_fish_got_something(cpu: &mut Cpu) {
    cpu.pc = 0x4ff4;
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xd1ef, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0x5035);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::queue_script::queue_script(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = { let value = ((crate::defs::JUMPTABLE_EXIT as i32) | (0x1 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn fish_function_fish_no_bite(cpu: &mut Cpu) {
    cpu.pc = 0x5002;
    cpu.a = 0x2;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xd1ef, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0x501e);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::queue_script::queue_script(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = { let value = ((crate::defs::JUMPTABLE_EXIT as i32) | (0x1 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn fish_function_fish_no_fish(cpu: &mut Cpu) {
    cpu.pc = 0x5010;
    cpu.a = 0x0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xd1ef, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0x5027);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::queue_script::queue_script(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = { let value = ((crate::defs::JUMPTABLE_EXIT as i32) | (0x1 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn fishing_check_facing_up(cpu: &mut Cpu) {
    cpu.pc = 0x506c;
    cpu.a = cpu.read_byte(0xd4de);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0xc;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = crate::defs::OW_UP as u8;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become fishing_check_facing_up_up(cpu);
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
    become fishing_check_facing_up_up(cpu);
}

pub(crate) fn fishing_check_facing_up_up(cpu: &mut Cpu) {
    cpu.pc = 0x5078;
    cpu.write_byte(0xc2dd, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn put_the_rod_away(cpu: &mut Cpu) {
    cpu.pc = 0x5095;
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
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xd4e1, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map_objects::update_sprites(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::gfx::update_player_sprite(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn bike_function(cpu: &mut Cpu) {
    cpu.pc = 0x50b3;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    bike_function_try_bike(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = crate::defs::JUMPTABLE_INDEX_MASK as u8;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xd0ec, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn bike_function_try_bike(cpu: &mut Cpu) {
    cpu.pc = 0x50bc;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    bike_function_check_environment(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.cycle(12);
        become bike_function_cannot_use_bike(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xd95d);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = crate::defs::PLAYER_NORMAL as u8;
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
        become bike_function_get_on_bike(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = crate::defs::PLAYER_BIKE as u8;
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
        become bike_function_get_off_bike(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become bike_function_cannot_use_bike(cpu);
}

pub(crate) fn bike_function_get_on_bike(cpu: &mut Cpu) {
    cpu.pc = 0x50ce;
    cpu.set_hl(0x513e);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0x514e);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    bike_function_check_if_registered(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::queue_script::queue_script(cpu);
    assert_eq!(cpu.pc, next_pc);
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
    cpu.set_de(crate::defs::MUSIC_BICYCLE as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc2c0, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::audio::play_music(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn bike_function_get_off_bike(cpu: &mut Cpu) {
    cpu.pc = 0x50f7;
    cpu.set_hl(0xdbf5);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((crate::defs::BIKEFLAGS_ALWAYS_ON_BIKE_F as u8) & 7);
    let value = cpu.read_byte(cpu.hl());
    cpu.flag_z = (value & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(12);
    if !cpu.flag_z {
        cpu.cycle(12);
        become bike_function_cant_get_off_bike(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x5158);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0x516b);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    bike_function_check_if_registered(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x03;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become bike_function_done(cpu);
}

pub(crate) fn bike_function_cant_get_off_bike(cpu: &mut Cpu) {
    cpu.pc = 0x510b;
    cpu.set_hl(0x5171);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(12);
    become bike_function_done(cpu);
}

pub(crate) fn bike_function_cannot_use_bike(cpu: &mut Cpu) {
    cpu.pc = 0x5110;
    cpu.a = 0x0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn bike_function_done(cpu: &mut Cpu) {
    cpu.pc = 0x5113;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::queue_script::queue_script(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn bike_function_check_if_registered(cpu: &mut Cpu) {
    cpu.pc = 0x5119;
    cpu.a = cpu.read_byte(0xd0ef);
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
    cpu.h = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.l = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn bike_function_check_environment(cpu: &mut Cpu) {
    cpu.pc = 0x5121;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map::get_map_environment(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map::check_outdoor_map(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_z {
        cpu.cycle(12);
        become bike_function_ok(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = crate::defs::CAVE as u8;
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
        become bike_function_ok(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = crate::defs::GATE as u8;
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
        become bike_function_ok(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become bike_function_nope(cpu);
}

pub(crate) fn bike_function_ok(cpu: &mut Cpu) {
    cpu.pc = 0x5133;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map_objects::get_player_tile_permission(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = 0xf;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(12);
        become bike_function_nope(cpu);
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
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn bike_function_nope(cpu: &mut Cpu) {
    cpu.pc = 0x513c;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = true;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn overworld_dummy_function(cpu: &mut Cpu) {
    cpu.pc = 0x5156;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub fn try_cut_ow(cpu: &mut Cpu) {
    cpu.pc = 0x5186;
    cpu.d = crate::defs::CUT as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    check_party_move(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.cycle(12);
        become try_cut_ow_cant_cut(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_de(crate::defs::ENGINE_HIVEBADGE as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    check_engine_flag(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.cycle(12);
        become try_cut_ow_cant_cut(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x03;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x51a9);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map::call_script(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = true;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn try_cut_ow_cant_cut(cpu: &mut Cpu) {
    cpu.pc = 0x519f;
    cpu.a = 0x03;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x51cd);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map::call_script(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = true;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn ask_cut_script_check_map(cpu: &mut Cpu) {
    cpu.pc = 0x51ba;
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
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    check_map_for_something_to_cut(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = crate::defs::TRUE as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc2dd, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

