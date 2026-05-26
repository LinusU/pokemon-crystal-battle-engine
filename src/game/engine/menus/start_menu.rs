use crate::runtime::Cpu;

pub const CONST_VALUE: u8 = 0;

pub const CONST_INC: u8 = 1;

pub const STARTMENUITEM_POKEDEX: u8 = 0;

pub const STARTMENUITEM_POKEMON: u8 = 1;

pub const STARTMENUITEM_PACK: u8 = 2;

pub const STARTMENUITEM_STATUS: u8 = 3;

pub const STARTMENUITEM_SAVE: u8 = 4;

pub const STARTMENUITEM_OPTION: u8 = 5;

pub const STARTMENUITEM_EXIT: u8 = 6;

pub const STARTMENUITEM_POKEGEAR: u8 = 7;

pub const STARTMENUITEM_QUIT: u8 = 8;

pub fn start_menu(cpu: &mut Cpu) {
    cpu.pc = 0x65cd;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::menu::clear_window_data(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_de(crate::defs::SFX_MENU as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::audio::play_sfx(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x01;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x6454);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xd84d);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((crate::defs::STATUSFLAGS2_BUG_CONTEST_TIMER_F as u8) & 7);
    let value = cpu.read_byte(cpu.hl());
    cpu.flag_z = (value & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.set_hl(0x66d3);
    cpu.pc += 3;
    cpu.cycle(12);
    if cpu.flag_z {
        cpu.cycle(12);
        become start_menu_got_menu_data(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x66db);
    cpu.pc += 3;
    cpu.cycle(12);
    become start_menu_got_menu_data(cpu);
}

pub(crate) fn start_menu_got_menu_data(cpu: &mut Cpu) {
    cpu.pc = 0x65e9;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::menu::load_menu_header(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    start_menu_set_up_menu_items(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xd0d2);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xcf88, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    start_menu_draw_menu_account(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::menu::draw_variable_length_menu_box(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    start_menu_draw_bug_contest_status_box(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::window::safe_update_sprites(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::window::hdma_transfer_tilemap_and_attrmap_menu(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x01;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x64bf);
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
    start_menu_draw_bug_contest_status(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::time_palettes::update_time_pals(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(12);
    become start_menu_select(cpu);
}

pub(crate) fn start_menu_reopen(cpu: &mut Cpu) {
    cpu.pc = 0x6612;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map_objects::update_sprites(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::time_palettes::update_time_pals(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    start_menu_set_up_menu_items(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xd0d2);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xcf88, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    become start_menu_select(cpu);
}

pub(crate) fn start_menu_exit(cpu: &mut Cpu) {
    cpu.pc = 0x6652;
    cpu.a = cpu.read_byte(0xffd8);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = 1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xffd8, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::gfx::load_fonts_extra(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte(0xffd8, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    become start_menu_return_end(cpu);
}

pub(crate) fn start_menu_return_end(cpu: &mut Cpu) {
    cpu.pc = 0x665f;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::menu::exit_menu(cpu);
    assert_eq!(cpu.pc, next_pc);
    become start_menu_return_end2(cpu);
}

pub(crate) fn start_menu_return_end2(cpu: &mut Cpu) {
    cpu.pc = 0x6662;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::window::close_text(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::time_palettes::update_time_pals(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn start_menu_get_input(cpu: &mut Cpu) {
    cpu.pc = 0x6669;
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
    start_menu_draw_menu_account_2(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::menu::set_up_menu(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0xff;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcf74, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    become start_menu_loop(cpu);
}

pub(crate) fn start_menu_loop(cpu: &mut Cpu) {
    cpu.pc = 0x6677;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    start_menu_print_menu_account(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::menu::get_scrolling_menu_joypad(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xcf73);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = crate::defs::PAD_B as u8;
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
        become start_menu_b(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = crate::defs::PAD_A as u8;
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
        become start_menu_a(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become start_menu_loop(cpu);
}

pub(crate) fn start_menu_a(cpu: &mut Cpu) {
    cpu.pc = 0x668a;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::menu::play_click_sfx(cpu);
    assert_eq!(cpu.pc, next_pc);
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

pub(crate) fn start_menu_b(cpu: &mut Cpu) {
    cpu.pc = 0x668f;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = true;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn start_menu_exit_menu_run_script(cpu: &mut Cpu) {
    cpu.pc = 0x6691;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::menu::exit_menu(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = crate::defs::HMENURETURN_SCRIPT as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xffa0, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn start_menu_exit_menu_run_script_close_text(cpu: &mut Cpu) {
    cpu.pc = 0x6699;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::menu::exit_menu(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = crate::defs::HMENURETURN_SCRIPT as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xffa0, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.cycle(12);
    become start_menu_return_end2(cpu);
}

pub(crate) fn start_menu_exit_menu_call_func_close_text(cpu: &mut Cpu) {
    cpu.pc = 0x66a2;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::menu::exit_menu(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xd0e9);
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
    cpu.a = cpu.read_byte(0xd0e8);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(12);
    become start_menu_return_end2(cpu);
}

pub(crate) fn start_menu_return_redraw(cpu: &mut Cpu) {
    cpu.pc = 0x66b1;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    start_menu_clear(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(16);
    become start_menu_reopen(cpu);
}

pub(crate) fn start_menu_clear(cpu: &mut Cpu) {
    cpu.pc = 0x66b7;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::tilemap::clear_bg_palettes(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::menu::call_exit_menu(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map::reload_tileset_and_palettes(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    start_menu_draw_menu_account(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::menu::draw_variable_length_menu_box(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    start_menu_draw_bug_contest_status(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map_objects::update_sprites(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::palettes::gs_reload_palettes(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map::finish_exit_menu(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn start_menu_menu_string(cpu: &mut Cpu) {
    cpu.pc = 0x67ef;
    cpu.push_word(cpu.de());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xcf74);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    start_menu_get_menu_account_text_pointer(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.d = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::place_string(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn start_menu_menu_desc(cpu: &mut Cpu) {
    cpu.pc = 0x6800;
    cpu.push_word(cpu.de());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xcf74);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0xff;
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
        become start_menu_none(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    start_menu_get_menu_account_text_pointer(cpu);
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
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.d = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::place_string(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn start_menu_none(cpu: &mut Cpu) {
    cpu.pc = 0x6817;
    let value = cpu.pop_word();
    cpu.set_de(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn start_menu_get_menu_account_text_pointer(cpu: &mut Cpu) {
    cpu.pc = 0x6819;
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.d = 0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xcf97);
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
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn start_menu_set_up_menu_items(cpu: &mut Cpu) {
    cpu.pc = 0x6829;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcf76, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    start_menu_fill_menu_list(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xd84c);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((crate::defs::STATUSFLAGS_POKEDEX_F as u8) & 7);
    let value = cpu.read_byte(cpu.hl());
    cpu.flag_z = (value & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(12);
    if cpu.flag_z {
        cpu.cycle(12);
        become start_menu_no_pokedex(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = STARTMENUITEM_POKEDEX as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    start_menu_append_menu_list(cpu);
    assert_eq!(cpu.pc, next_pc);
    become start_menu_no_pokedex(cpu);
}

pub(crate) fn start_menu_no_pokedex(cpu: &mut Cpu) {
    cpu.pc = 0x683c;
    cpu.a = cpu.read_byte(0xdcd7);
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
        become start_menu_no_pokemon(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = STARTMENUITEM_POKEMON as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    start_menu_append_menu_list(cpu);
    assert_eq!(cpu.pc, next_pc);
    become start_menu_no_pokemon(cpu);
}

pub(crate) fn start_menu_no_pokemon(cpu: &mut Cpu) {
    cpu.pc = 0x6847;
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
        become start_menu_no_pack(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xd84d);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((crate::defs::STATUSFLAGS2_BUG_CONTEST_TIMER_F as u8) & 7);
    let value = cpu.read_byte(cpu.hl());
    cpu.flag_z = (value & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(12);
    if !cpu.flag_z {
        cpu.cycle(12);
        become start_menu_no_pack(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = STARTMENUITEM_PACK as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    start_menu_append_menu_list(cpu);
    assert_eq!(cpu.pc, next_pc);
    become start_menu_no_pack(cpu);
}

pub(crate) fn start_menu_no_pack(cpu: &mut Cpu) {
    cpu.pc = 0x6859;
    cpu.set_hl(0xd957);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((crate::defs::POKEGEAR_OBTAINED_F as u8) & 7);
    let value = cpu.read_byte(cpu.hl());
    cpu.flag_z = (value & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(12);
    if cpu.flag_z {
        cpu.cycle(12);
        become start_menu_no_pokegear(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = STARTMENUITEM_POKEGEAR as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    start_menu_append_menu_list(cpu);
    assert_eq!(cpu.pc, next_pc);
    become start_menu_no_pokegear(cpu);
}

pub(crate) fn start_menu_no_pokegear(cpu: &mut Cpu) {
    cpu.pc = 0x6865;
    cpu.a = STARTMENUITEM_STATUS as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    start_menu_append_menu_list(cpu);
    assert_eq!(cpu.pc, next_pc);
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
        become start_menu_no_save(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xd84d);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((crate::defs::STATUSFLAGS2_BUG_CONTEST_TIMER_F as u8) & 7);
    let value = cpu.read_byte(cpu.hl());
    cpu.flag_z = (value & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = STARTMENUITEM_QUIT as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(12);
        become start_menu_write(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = STARTMENUITEM_SAVE as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    become start_menu_write(cpu);
}

pub(crate) fn start_menu_write(cpu: &mut Cpu) {
    cpu.pc = 0x687b;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    start_menu_append_menu_list(cpu);
    assert_eq!(cpu.pc, next_pc);
    become start_menu_no_save(cpu);
}

pub(crate) fn start_menu_no_save(cpu: &mut Cpu) {
    cpu.pc = 0x687e;
    cpu.a = STARTMENUITEM_OPTION as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    start_menu_append_menu_list(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = STARTMENUITEM_EXIT as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    start_menu_append_menu_list(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xd03e, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn start_menu_fill_menu_list(cpu: &mut Cpu) {
    cpu.pc = 0x688d;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl(0xd03e);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = -1i16 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_bc({ const N: i32 = ((0xd04e as i32) - ((0xd03e as i32) + (1 as i32))); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::byte_fill(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_de({ const N: i32 = (0xd03e as i32) + (1 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.c = 0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn start_menu_append_menu_list(cpu: &mut Cpu) {
    cpu.pc = 0x68a0;
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.c;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.c = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn start_menu_draw_menu_account(cpu: &mut Cpu) {
    cpu.pc = 0x68a4;
    cpu.cycle(16);
    become start_menu_draw_menu_account_2(cpu);
}

pub(crate) fn start_menu_print_menu_account(cpu: &mut Cpu) {
    cpu.pc = 0x68a7;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    start_menu_is_menu_account_on(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_z {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    start_menu_draw_menu_account_2(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_de({ const N: i32 = ((((14 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (0 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become start_menu_menu_desc(cpu);
}

pub(crate) fn start_menu_draw_menu_account_2(cpu: &mut Cpu) {
    cpu.pc = 0x68b4;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    start_menu_is_menu_account_on(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_z {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl({ const N: i32 = ((((13 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (0 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(0x050a);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::clear_box(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = ((((13 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (0 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = 3;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.c = 8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become crate::game::home::text::textbox_palette(cpu);
}

pub(crate) fn start_menu_is_menu_account_on(cpu: &mut Cpu) {
    cpu.pc = 0x68cb;
    cpu.a = cpu.read_byte(0xcfd1);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = { let value = ((1 as i32) << (crate::defs::MENU_ACCOUNT as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn start_menu_draw_bug_contest_status_box(cpu: &mut Cpu) {
    cpu.pc = 0x68d1;
    cpu.set_hl(0xd84d);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((crate::defs::STATUSFLAGS2_BUG_CONTEST_TIMER_F as u8) & 7);
    let value = cpu.read_byte(cpu.hl());
    cpu.flag_z = (value & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(12);
    if cpu.flag_z {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 0x09;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x4bdc);
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

pub(crate) fn start_menu_draw_bug_contest_status(cpu: &mut Cpu) {
    cpu.pc = 0x68de;
    cpu.set_hl(0xd84d);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((crate::defs::STATUSFLAGS2_BUG_CONTEST_TIMER_F as u8) & 7);
    let value = cpu.read_byte(cpu.hl());
    cpu.flag_z = (value & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(12);
    if !cpu.flag_z {
        cpu.cycle(12);
        become start_menu_contest(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn start_menu_contest(cpu: &mut Cpu) {
    cpu.pc = 0x68e6;
    cpu.a = 0x09;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x4be7);
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

pub(crate) fn start_menu_exit_2(cpu: &mut Cpu) {
    cpu.pc = 0x68ed;
    cpu.a = 1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn start_menu_quit(cpu: &mut Cpu) {
    cpu.pc = 0x68f0;
    cpu.set_hl(0x6906);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::engine::pokemon::mon_menu::start_menu_yes_no(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.cycle(12);
        become start_menu_quit_dont_end_contest(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x04;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x760b);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::queue_script::far_queue_script(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 4;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn start_menu_quit_dont_end_contest(cpu: &mut Cpu) {
    cpu.pc = 0x6903;
    cpu.a = 0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn start_menu_save(cpu: &mut Cpu) {
    cpu.pc = 0x690b;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map::buffer_screen(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x05;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x4a1a);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    if !cpu.flag_c {
        cpu.cycle(12);
        become start_menu_save_saved(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn start_menu_save_saved(cpu: &mut Cpu) {
    cpu.pc = 0x6919;
    cpu.a = 1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn start_menu_option(cpu: &mut Cpu) {
    cpu.pc = 0x691c;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map::fade_to_menu(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x01;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x5b64);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 6;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn start_menu_status(cpu: &mut Cpu) {
    cpu.pc = 0x6928;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map::fade_to_menu(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x09;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x5105);
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
    crate::game::home::map::close_submenu(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn start_menu_pokedex(cpu: &mut Cpu) {
    cpu.pc = 0x6937;
    cpu.a = cpu.read_byte(0xdcd7);
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
        become start_menu_pokedex_empty(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map::fade_to_menu(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x10;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x4000);
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
    crate::game::home::map::close_submenu(cpu);
    assert_eq!(cpu.pc, next_pc);
    become start_menu_pokedex_empty(cpu);
}

pub(crate) fn start_menu_pokedex_empty(cpu: &mut Cpu) {
    cpu.pc = 0x6949;
    cpu.a = 0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn start_menu_pokegear(cpu: &mut Cpu) {
    cpu.pc = 0x694c;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map::fade_to_menu(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x24;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x4b8d);
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
    crate::game::home::map::close_submenu(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn start_menu_pack(cpu: &mut Cpu) {
    cpu.pc = 0x695b;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map::fade_to_menu(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x04;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x4000);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xcf66);
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
        become start_menu_pack_used_item(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map::close_submenu(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn start_menu_pack_used_item(cpu: &mut Cpu) {
    cpu.pc = 0x6970;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map::exit_all_menus(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 4;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn start_menu_pokemon(cpu: &mut Cpu) {
    cpu.pc = 0x6976;
    cpu.a = cpu.read_byte(0xdcd7);
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
        become start_menu_pokemon_return(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map::fade_to_menu(cpu);
    assert_eq!(cpu.pc, next_pc);
    become start_menu_pokemon_choosemenu(cpu);
}

pub(crate) fn start_menu_pokemon_choosemenu(cpu: &mut Cpu) {
    cpu.pc = 0x697f;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xd141, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::tilemap::clear_bg_palettes(cpu);
    assert_eq!(cpu.pc, next_pc);
    become start_menu_pokemon_menu(cpu);
}

pub(crate) fn start_menu_pokemon_menu(cpu: &mut Cpu) {
    cpu.pc = 0x6986;
    cpu.a = 0x14;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x404f);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x14;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x4405);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x14;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x43e0);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    become start_menu_pokemon_menunoreload(cpu);
}

pub(crate) fn start_menu_pokemon_menunoreload(cpu: &mut Cpu) {
    cpu.pc = 0x6998;
    cpu.a = 0x14;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x405f);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x14;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x449a);
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
    crate::game::home::tilemap::wait_bg_map(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::tilemap::set_default_bgp_and_obp(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::delay::delay_frame(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x14;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x4457);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.cycle(12);
        become start_menu_pokemon_return(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::engine::pokemon::mon_menu::pokemon_action_submenu(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = 3;
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
        become start_menu_pokemon_menu(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0;
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
        become start_menu_pokemon_choosemenu(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 1;
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
        become start_menu_pokemon_menunoreload(cpu);
    }
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
    if cpu.flag_z {
        cpu.cycle(12);
        become start_menu_pokemon_quit(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become start_menu_pokemon_return(cpu);
}

pub(crate) fn start_menu_pokemon_return(cpu: &mut Cpu) {
    cpu.pc = 0x69c8;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map::close_submenu(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn start_menu_pokemon_quit(cpu: &mut Cpu) {
    cpu.pc = 0x69ce;
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map::exit_all_menus(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn start_menu_open_menu(cpu: &mut Cpu) {
    cpu.a = cpu.read_byte(0xcf74); // wMenuSelection
    match cpu.a {
        0 => become start_menu_pokedex(cpu),
        1 => become start_menu_pokemon(cpu),
        2 => become start_menu_pack(cpu),
        3 => become start_menu_status(cpu),
        4 => become start_menu_save(cpu),
        5 => become start_menu_option(cpu),
        6 => become start_menu_exit_2(cpu),
        7 => become start_menu_pokegear(cpu),
        8 => become start_menu_quit(cpu),
        n => panic!("unsupported OpenMenu index: {}", n),
    }
}

pub(crate) fn start_menu_select(cpu: &mut Cpu) {
    cpu.pc = 0x6621;

    cpu.push_word(0x6624);
    start_menu_get_input(cpu);

    if cpu.flag_c {
        become start_menu_exit(cpu);
    }

    cpu.push_word(0x6629);
    start_menu_draw_menu_account_2(cpu);

    cpu.a = cpu.read_byte(0xcf88); // wMenuCursorPosition
    cpu.write_byte(0xd0d2, cpu.a); // wBattleMenuCursorPosition

    cpu.push_word(0x6632);
    crate::game::home::menu::play_click_sfx(cpu);

    cpu.push_word(0x6635);
    crate::game::home::menu::place_hollow_cursor(cpu);

    cpu.push_word(0x6638);
    start_menu_open_menu(cpu);

    match cpu.a {
        0 => become start_menu_reopen(cpu),
        1 => become start_menu_exit(cpu),
        2 => become start_menu_exit_menu_call_func_close_text(cpu),
        3 => become start_menu_exit_menu_run_script_close_text(cpu),
        4 => become start_menu_exit_menu_run_script(cpu),
        5 => become start_menu_return_end(cpu),
        6 => become start_menu_return_redraw(cpu),
        n => panic!("unsupported MenuReturns index: {}", n),
    }
}
