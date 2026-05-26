use crate::runtime::Cpu;

pub(crate) fn bank_of_mom(cpu: &mut Cpu) {
    cpu.pc = 0x6218;
    cpu.a = cpu.read_byte(0xffaa);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xffaa, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcf63, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    become bank_of_mom_loop(cpu);
}

pub(crate) fn bank_of_mom_loop(cpu: &mut Cpu) {
    cpu.pc = 0x6223;
    cpu.a = cpu.read_byte(0xcf63);
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
        become bank_of_mom_done(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    bank_of_mom_run_jumptable(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(12);
    become bank_of_mom_loop(cpu);
}

pub(crate) fn bank_of_mom_done(cpu: &mut Cpu) {
    cpu.pc = 0x622f;
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte(0xffaa, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn bank_of_mom_check_if_bank_initialized(cpu: &mut Cpu) {
    cpu.pc = 0x6254;
    cpu.a = cpu.read_byte(0xd854);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((crate::defs::MOM_ACTIVE_F as u8) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(12);
        become bank_of_mom_savingmoneyalready(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let mask = 1u8 << ((crate::defs::MOM_ACTIVE_F as u8) & 7);
    cpu.a |= mask;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xd854, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become bank_of_mom_done_0(cpu);
}

pub(crate) fn bank_of_mom_savingmoneyalready(cpu: &mut Cpu) {
    cpu.pc = 0x6264;
    cpu.a = 0x2;
    cpu.pc += 2;
    cpu.cycle(8);
    become bank_of_mom_done_0(cpu);
}

pub(crate) fn bank_of_mom_done_0(cpu: &mut Cpu) {
    cpu.pc = 0x6266;
    cpu.write_byte(0xcf63, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn bank_of_mom_initialize_bank(cpu: &mut Cpu) {
    cpu.pc = 0x626a;
    cpu.set_hl(0x6649);
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
    crate::game::home::menu::yes_no_box(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.cycle(12);
        become bank_of_mom_dont_save_money(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x664e);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::print_text(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = { let value = (((1 as i32) << (crate::defs::MOM_ACTIVE_F as i32)) | ((1 as i32) << (crate::defs::MOM_SAVING_SOME_MONEY_F as i32))); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become bank_of_mom_done_1(cpu);
}

pub(crate) fn bank_of_mom_dont_save_money(cpu: &mut Cpu) {
    cpu.pc = 0x627f;
    cpu.a = { let value = ((1 as i32) << (crate::defs::MOM_ACTIVE_F as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    become bank_of_mom_done_1(cpu);
}

pub(crate) fn bank_of_mom_done_1(cpu: &mut Cpu) {
    cpu.pc = 0x6281;
    cpu.write_byte(0xd854, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0x6653);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::print_text(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcf63, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn bank_of_mom_is_this_about_your_money(cpu: &mut Cpu) {
    cpu.pc = 0x6290;
    cpu.set_hl(0x6658);
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
    crate::game::home::menu::yes_no_box(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.cycle(12);
        become bank_of_mom_nope(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x3;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become bank_of_mom_done_2(cpu);
}

pub(crate) fn bank_of_mom_nope(cpu: &mut Cpu) {
    cpu.pc = 0x629f;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    dst_checks(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x7;
    cpu.pc += 2;
    cpu.cycle(8);
    become bank_of_mom_done_2(cpu);
}

pub(crate) fn bank_of_mom_done_2(cpu: &mut Cpu) {
    cpu.pc = 0x62a4;
    cpu.write_byte(0xcf63, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn bank_of_mom_access_bank_of_mom(cpu: &mut Cpu) {
    cpu.pc = 0x62a8;
    cpu.set_hl(0x665d);
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
    crate::game::home::menu::load_standard_menu_header(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0x66b5);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::menu::copy_menu_header(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::menu::vertical_menu(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::menu::close_window(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.cycle(12);
        become bank_of_mom_cancel(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xcfa9);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x1;
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
        become bank_of_mom_withdraw(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x2;
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
        become bank_of_mom_deposit(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x3;
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
        become bank_of_mom_stopsaving(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become bank_of_mom_cancel(cpu);
}

pub(crate) fn bank_of_mom_cancel(cpu: &mut Cpu) {
    cpu.pc = 0x62ce;
    cpu.a = 0x7;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become bank_of_mom_done_3(cpu);
}

pub(crate) fn bank_of_mom_withdraw(cpu: &mut Cpu) {
    cpu.pc = 0x62d2;
    cpu.a = 0x5;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become bank_of_mom_done_3(cpu);
}

pub(crate) fn bank_of_mom_deposit(cpu: &mut Cpu) {
    cpu.pc = 0x62d6;
    cpu.a = 0x4;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become bank_of_mom_done_3(cpu);
}

pub(crate) fn bank_of_mom_stopsaving(cpu: &mut Cpu) {
    cpu.pc = 0x62da;
    cpu.a = 0x6;
    cpu.pc += 2;
    cpu.cycle(8);
    become bank_of_mom_done_3(cpu);
}

pub(crate) fn bank_of_mom_done_3(cpu: &mut Cpu) {
    cpu.pc = 0x62dc;
    cpu.write_byte(0xcf63, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn bank_of_mom_store_money(cpu: &mut Cpu) {
    cpu.pc = 0x62e0;
    cpu.set_hl(0x6662);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::print_text(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl(0xd086);
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
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 5;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcf64, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::menu::load_standard_menu_header(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mom_set_up_deposit_menu(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mom_wait10_frames(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mom_withdraw_deposit_menu_joypad(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::menu::close_window(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.cycle(12);
        become bank_of_mom_cancel_deposit(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xd086);
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
        become bank_of_mom_cancel_deposit(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_de(0xd84e);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(0xd086);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0x05;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x600b);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.cycle(12);
        become bank_of_mom_insufficient_funds_in_wallet(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xd086);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de({ const N: i32 = (0xd086 as i32) + (3 as i32); N as u16 });
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
    cpu.set_bc(0xd851);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xd086);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0x05;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x5fd7);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.cycle(12);
        become bank_of_mom_not_enough_room_in_bank(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_bc({ const N: i32 = (0xd086 as i32) + (3 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xd84e);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0x05;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x5ffa);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xd086);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xd851);
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
    cpu.set_de(crate::defs::SFX_TRANSACTION as u16);
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
    cpu.set_hl(0x668a);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::print_text(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become bank_of_mom_done_4(cpu);
}

pub(crate) fn bank_of_mom_insufficient_funds_in_wallet(cpu: &mut Cpu) {
    cpu.pc = 0x635f;
    cpu.set_hl(0x667b);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::print_text(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn bank_of_mom_not_enough_room_in_bank(cpu: &mut Cpu) {
    cpu.pc = 0x6366;
    cpu.set_hl(0x6680);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::print_text(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn bank_of_mom_cancel_deposit(cpu: &mut Cpu) {
    cpu.pc = 0x636d;
    cpu.a = 0x7;
    cpu.pc += 2;
    cpu.cycle(8);
    become bank_of_mom_done_4(cpu);
}

pub(crate) fn bank_of_mom_done_4(cpu: &mut Cpu) {
    cpu.pc = 0x636f;
    cpu.write_byte(0xcf63, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn bank_of_mom_take_money(cpu: &mut Cpu) {
    cpu.pc = 0x6373;
    cpu.set_hl(0x6667);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::print_text(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl(0xd086);
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
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 5;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcf64, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::menu::load_standard_menu_header(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mom_set_up_withdraw_menu(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mom_wait10_frames(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mom_withdraw_deposit_menu_joypad(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::menu::close_window(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.cycle(12);
        become bank_of_mom_cancel_withdraw(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xd086);
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
        become bank_of_mom_cancel_withdraw(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xd086);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de({ const N: i32 = (0xd086 as i32) + (3 as i32); N as u16 });
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
    cpu.set_de(0xd851);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(0xd086);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0x05;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x600b);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.cycle(12);
        become bank_of_mom_insufficient_funds_in_bank(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_bc(0xd84e);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xd086);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0x05;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x5fd7);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.cycle(12);
        become bank_of_mom_not_enough_room_in_wallet(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_bc({ const N: i32 = (0xd086 as i32) + (3 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xd851);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0x05;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x5ffa);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xd086);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xd84e);
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
    cpu.set_de(crate::defs::SFX_TRANSACTION as u16);
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
    cpu.set_hl(0x668f);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::print_text(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become bank_of_mom_done_5(cpu);
}

pub(crate) fn bank_of_mom_insufficient_funds_in_bank(cpu: &mut Cpu) {
    cpu.pc = 0x63f2;
    cpu.set_hl(0x6671);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::print_text(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn bank_of_mom_not_enough_room_in_wallet(cpu: &mut Cpu) {
    cpu.pc = 0x63f9;
    cpu.set_hl(0x6676);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::print_text(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn bank_of_mom_cancel_withdraw(cpu: &mut Cpu) {
    cpu.pc = 0x6400;
    cpu.a = 0x7;
    cpu.pc += 2;
    cpu.cycle(8);
    become bank_of_mom_done_5(cpu);
}

pub(crate) fn bank_of_mom_done_5(cpu: &mut Cpu) {
    cpu.pc = 0x6402;
    cpu.write_byte(0xcf63, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn bank_of_mom_stop_or_start_saving_money(cpu: &mut Cpu) {
    cpu.pc = 0x6406;
    cpu.set_hl(0x666c);
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
    crate::game::home::menu::yes_no_box(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.cycle(12);
        become bank_of_mom_stop_saving_money(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = { let value = (((1 as i32) << (crate::defs::MOM_ACTIVE_F as i32)) | ((1 as i32) << (crate::defs::MOM_SAVING_SOME_MONEY_F as i32))); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xd854, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0x6685);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::print_text(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcf63, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn bank_of_mom_stop_saving_money(cpu: &mut Cpu) {
    cpu.pc = 0x6422;
    cpu.a = { let value = ((1 as i32) << (crate::defs::MOM_ACTIVE_F as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xd854, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x7;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcf63, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn bank_of_mom_just_do_what_you_can(cpu: &mut Cpu) {
    cpu.pc = 0x642d;
    cpu.set_hl(0x6694);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::print_text(cpu);
    assert_eq!(cpu.pc, next_pc);
    become bank_of_mom_ask_dst(cpu);
}

pub(crate) fn bank_of_mom_ask_dst(cpu: &mut Cpu) {
    cpu.pc = 0x6433;
    cpu.set_hl(0xcf63);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((crate::defs::JUMPTABLE_EXIT_F as u8) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value |= mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn dst_checks(cpu: &mut Cpu) {
    cpu.pc = 0x6439;
    cpu.a = cpu.read_byte(0xd4c2);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((crate::defs::DST_F as u8) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xff94);
    cpu.pc += 2;
    cpu.cycle(12);
    if cpu.flag_z {
        cpu.cycle(12);
        become dst_checks_not_dst(cpu);
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
    if cpu.flag_z {
        cpu.cycle(12);
        become dst_checks_lost_booklet(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become dst_checks_loop(cpu);
}

pub(crate) fn dst_checks_not_dst(cpu: &mut Cpu) {
    cpu.pc = 0x6447;
    let value = 23;
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
        become dst_checks_loop(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become dst_checks_lost_booklet(cpu);
}

pub(crate) fn dst_checks_lost_booklet(cpu: &mut Cpu) {
    cpu.pc = 0x644b;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    dst_checks_clear_box(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_bc({ const N: i32 = ((((14 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (1 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0x64f4);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::print_textbox_text_at(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::menu::yes_no_box(cpu);
    assert_eq!(cpu.pc, next_pc);
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
    dst_checks_clear_box(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_bc({ const N: i32 = ((((14 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (1 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0x64f9);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::print_textbox_text_at(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn dst_checks_loop(cpu: &mut Cpu) {
    cpu.pc = 0x6468;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    dst_checks_clear_box(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_bc({ const N: i32 = ((((14 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (1 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xd4c2);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((crate::defs::DST_F as u8) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become dst_checks_set_dst(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x6508);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::print_textbox_text_at(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::menu::yes_no_box(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xd4c2);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((crate::defs::DST_F as u8) & 7);
    cpu.a &= !mask;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xd4c2, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    dst_checks_set_clock_back(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    dst_checks_clear_box(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_bc({ const N: i32 = ((((14 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (1 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0x650d);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::print_textbox_text_at(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn dst_checks_set_dst(cpu: &mut Cpu) {
    cpu.pc = 0x6497;
    cpu.set_hl(0x64fe);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::print_textbox_text_at(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::menu::yes_no_box(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xd4c2);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((crate::defs::DST_F as u8) & 7);
    cpu.a |= mask;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xd4c2, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    dst_checks_set_clock_forward(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    dst_checks_clear_box(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_bc({ const N: i32 = ((((14 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (1 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0x6503);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::print_textbox_text_at(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn dst_checks_set_clock_forward(cpu: &mut Cpu) {
    cpu.pc = 0x64b9;
    cpu.a = cpu.read_byte(0xd4b7);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 1;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_add(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) > 0x0f;
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 24;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_c {
        cpu.cycle(12);
        become dst_checks_dont_loop_hour_forward(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 24;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_add(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) > 0x0f;
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    become dst_checks_dont_loop_hour_forward(cpu);
}

pub(crate) fn dst_checks_dont_loop_hour_forward(cpu: &mut Cpu) {
    cpu.pc = 0x64c4;
    cpu.write_byte(0xd4b7, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = !cpu.flag_c;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xd4b6);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0;
    let carry_in = u8::from(cpu.flag_c);
    let lhs = cpu.a;
    let (partial, carry1) = lhs.overflowing_add(value);
    let (result, carry2) = partial.overflowing_add(carry_in);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) + carry_in > 0x0f;
    cpu.flag_c = carry1 || carry2;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xd4b6, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn dst_checks_set_clock_back(cpu: &mut Cpu) {
    cpu.pc = 0x64d1;
    cpu.a = cpu.read_byte(0xd4b7);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 1;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_c {
        cpu.cycle(12);
        become dst_checks_dont_loop_hour_back(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 24;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_add(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) > 0x0f;
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    become dst_checks_dont_loop_hour_back(cpu);
}

pub(crate) fn dst_checks_dont_loop_hour_back(cpu: &mut Cpu) {
    cpu.pc = 0x64da;
    cpu.write_byte(0xd4b7, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xd4b6);
    cpu.pc += 3;
    cpu.cycle(16);
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
    if !cpu.flag_c {
        cpu.cycle(12);
        become dst_checks_dont_loop_day_back(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 7;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_add(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) > 0x0f;
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    become dst_checks_dont_loop_day_back(cpu);
}

pub(crate) fn dst_checks_dont_loop_day_back(cpu: &mut Cpu) {
    cpu.pc = 0x64e6;
    cpu.write_byte(0xd4b6, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn dst_checks_clear_box(cpu: &mut Cpu) {
    cpu.pc = 0x64ea;
    cpu.set_hl({ const N: i32 = ((((14 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (1 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(0x0312);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::clear_box(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn mom_set_up_withdraw_menu(cpu: &mut Cpu) {
    cpu.pc = 0x6512;
    cpu.set_de(0x669f);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(12);
    become mom_continue_menu_setup(cpu);
}

pub(crate) fn mom_set_up_deposit_menu(cpu: &mut Cpu) {
    cpu.pc = 0x6517;
    cpu.set_de(0x66a8);
    cpu.pc += 3;
    cpu.cycle(12);
    become mom_continue_menu_setup(cpu);
}

pub(crate) fn mom_continue_menu_setup(cpu: &mut Cpu) {
    cpu.pc = 0x651a;
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
    cpu.set_hl({ const N: i32 = ((((0 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (0 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(0x0612);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::textbox(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = ((((2 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (1 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0x6699);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::place_string(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = ((((2 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (12 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xd851);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(0x2306);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::print_text::print_num(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = ((((4 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (1 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0x66b0);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::place_string(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = ((((4 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (12 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xd84e);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(0x2306);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::print_text::print_num(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = ((((6 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (1 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    let value = cpu.pop_word();
    cpu.set_de(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::place_string(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = ((((6 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (12 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xd086);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(0xa306);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::print_text::print_num(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map_objects::update_sprites(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::tilemap::cgb_only_copy_tilemap_at_once(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn mom_wait10_frames(cpu: &mut Cpu) {
    cpu.pc = 0x656b;
    cpu.c = 10;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::delay::delay_frames(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn mom_withdraw_deposit_menu_joypad(cpu: &mut Cpu) {
    cpu.pc = 0x6571;
    become mom_withdraw_deposit_menu_joypad_loop(cpu);
}

pub(crate) fn mom_withdraw_deposit_menu_joypad_loop(cpu: &mut Cpu) {
    cpu.pc = 0x6571;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::joypad::joy_text_delay(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xffa7);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = crate::defs::PAD_B as u8;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(12);
        become mom_withdraw_deposit_menu_joypad_pressed_b(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = crate::defs::PAD_A as u8;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(12);
        become mom_withdraw_deposit_menu_joypad_pressed_a(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mom_withdraw_deposit_menu_joypad_dpadaction(cpu);
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
    cpu.set_hl({ const N: i32 = ((((6 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (12 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(7 as u16);
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
    cpu.set_hl({ const N: i32 = ((((6 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (12 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xd086);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(0xa306);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::print_text::print_num(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xff9b);
    cpu.pc += 2;
    cpu.cycle(12);
    let value = 0x10;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(12);
        become mom_withdraw_deposit_menu_joypad_skip(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl({ const N: i32 = ((((6 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (13 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xcf64);
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
    cpu.write_byte(cpu.hl(), { let value = (0x7f as i32); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } });
    cpu.pc += 2;
    cpu.cycle(12);
    become mom_withdraw_deposit_menu_joypad_skip(cpu);
}

pub(crate) fn mom_withdraw_deposit_menu_joypad_skip(cpu: &mut Cpu) {
    cpu.pc = 0x65b0;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::tilemap::wait_bg_map(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(12);
    become mom_withdraw_deposit_menu_joypad_loop(cpu);
}

pub(crate) fn mom_withdraw_deposit_menu_joypad_pressed_b(cpu: &mut Cpu) {
    cpu.pc = 0x65b5;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = true;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn mom_withdraw_deposit_menu_joypad_pressed_a(cpu: &mut Cpu) {
    cpu.pc = 0x65b7;
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

pub(crate) fn mom_withdraw_deposit_menu_joypad_dpadaction(cpu: &mut Cpu) {
    cpu.pc = 0x65b9;
    cpu.set_hl(0xffa9);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = crate::defs::PAD_UP as u8;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(12);
        become mom_withdraw_deposit_menu_joypad_incrementdigit(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = crate::defs::PAD_DOWN as u8;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(12);
        become mom_withdraw_deposit_menu_joypad_decrementdigit(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = crate::defs::PAD_LEFT as u8;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(12);
        become mom_withdraw_deposit_menu_joypad_movecursorleft(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = crate::defs::PAD_RIGHT as u8;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(12);
        become mom_withdraw_deposit_menu_joypad_movecursorright(cpu);
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

pub(crate) fn mom_withdraw_deposit_menu_joypad_movecursorleft(cpu: &mut Cpu) {
    cpu.pc = 0x65d2;
    cpu.set_hl(0xcf64);
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
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
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

pub(crate) fn mom_withdraw_deposit_menu_joypad_movecursorright(cpu: &mut Cpu) {
    cpu.pc = 0x65da;
    cpu.set_hl(0xcf64);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
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
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    let addr = cpu.hl();
    let value = cpu.read_byte(addr);
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.write_byte(addr, result);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn mom_withdraw_deposit_menu_joypad_incrementdigit(cpu: &mut Cpu) {
    cpu.pc = 0x65e3;
    cpu.set_hl(0x6613);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mom_withdraw_deposit_menu_joypad_getdigitquantity(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.c = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.b = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_de(0xd086);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0x05;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x5fd7);
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

pub(crate) fn mom_withdraw_deposit_menu_joypad_decrementdigit(cpu: &mut Cpu) {
    cpu.pc = 0x65f5;
    cpu.set_hl(0x6613);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mom_withdraw_deposit_menu_joypad_getdigitquantity(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.c = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.b = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_de(0xd086);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0x05;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x5ffa);
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

pub(crate) fn mom_withdraw_deposit_menu_joypad_getdigitquantity(cpu: &mut Cpu) {
    cpu.pc = 0x6607;
    cpu.a = cpu.read_byte(0xcf64);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.push_word(cpu.de());
    cpu.pc += 1;
    cpu.cycle(16);
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
    cpu.set_de(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}


pub(crate) fn bank_of_mom_run_jumptable(cpu: &mut Cpu) {
    match cpu.read_byte(0xcf63) {
        0 => become bank_of_mom_check_if_bank_initialized(cpu),
        1 => become bank_of_mom_initialize_bank(cpu),
        2 => become bank_of_mom_is_this_about_your_money(cpu),
        3 => become bank_of_mom_access_bank_of_mom(cpu),
        4 => become bank_of_mom_store_money(cpu),
        5 => become bank_of_mom_take_money(cpu),
        6 => become bank_of_mom_stop_or_start_saving_money(cpu),
        7 => become bank_of_mom_just_do_what_you_can(cpu),
        8 => become bank_of_mom_ask_dst(cpu),
        n => panic!("unsupported mom menu step: {}", n),
    };
}
