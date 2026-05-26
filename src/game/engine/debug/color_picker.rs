use crate::runtime::Cpu;

pub const CONST_VALUE: u8 = 106;

pub const CONST_INC: u8 = 1;

pub const DEBUGTEST_TICKS_1: u8 = 106;

pub const DEBUGTEST_TICKS_2: u8 = 107;

pub const DEBUGTEST_WHITE: u8 = 108;

pub const DEBUGTEST_LIGHT: u8 = 109;

pub const DEBUGTEST_DARK: u8 = 110;

pub const DEBUGTEST_BLACK: u8 = 111;

pub const DEBUGTEST_0: u8 = 112;

pub const DEBUGTEST_1: u8 = 113;

pub const DEBUGTEST_2: u8 = 114;

pub const DEBUGTEST_3: u8 = 115;

pub const DEBUGTEST_4: u8 = 116;

pub const DEBUGTEST_5: u8 = 117;

pub const DEBUGTEST_6: u8 = 118;

pub const DEBUGTEST_7: u8 = 119;

pub const DEBUGTEST_8: u8 = 120;

pub const DEBUGTEST_9: u8 = 121;

pub const DEBUGTEST_A: u8 = 122;

pub const DEBUGTEST_B: u8 = 123;

pub const DEBUGTEST_C: u8 = 124;

pub const DEBUGTEST_D: u8 = 125;

pub const DEBUGTEST_E: u8 = 126;

pub const DEBUGTEST_F: u8 = 127;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const DEBUGCOLORMAIN_INITSCREEN: u8 = 0;

pub const DEBUGCOLORMAIN_UPDATESCREEN: u8 = 1;

pub const DEBUGCOLORMAIN_UPDATEPALETTES: u8 = 2;

pub const DEBUGCOLORMAIN_JOYPAD: u8 = 3;

pub const DEBUGCOLORMAIN_INITTMHM: u8 = 4;

pub const DEBUGCOLORMAIN_TMHMJOYPAD: u8 = 5;

pub const X: u8 = 3;

// duplicate assembler constant X

// duplicate assembler constant X

pub(crate) fn debug_color_picker(cpu: &mut Cpu) {
    cpu.pc = 0x58ac;
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
        become debug_color_picker_cgb(cpu);
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
    if cpu.flag_z {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    become debug_color_picker_cgb(cpu);
}

pub(crate) fn debug_color_picker_cgb(cpu: &mut Cpu) {
    cpu.pc = 0x58b5;
    cpu.a = cpu.read_byte(0xffaa);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = crate::defs::TRUE as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xffaa, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::lcd::disable_lcd(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    debug_color_init_vram(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    debug_color_load_gfx(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    debug_color_init_palettes(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    debug_color_init_mon_or_trainer_color(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::lcd::enable_lcd(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_de(crate::defs::MUSIC_NONE as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::audio::play_music(cpu);
    assert_eq!(cpu.pc, next_pc);
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
    cpu.write_byte(0xcf66, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xd003, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    become debug_color_picker_loop(cpu);
}

pub(crate) fn debug_color_picker_loop(cpu: &mut Cpu) {
    cpu.pc = 0x58de;
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
        become debug_color_picker_exit(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    debug_color_main(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    debug_color_place_cursor(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::delay::delay_frame(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(12);
    become debug_color_picker_loop(cpu);
}

pub(crate) fn debug_color_picker_exit(cpu: &mut Cpu) {
    cpu.pc = 0x58f0;
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

pub(crate) fn debug_color_init_mon_or_trainer_color(cpu: &mut Cpu) {
    cpu.pc = 0x58f4;
    cpu.a = cpu.read_byte(0xd002);
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
        become debug_color_init_trainer_color(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x68ce);
    cpu.pc += 3;
    cpu.cycle(12);
    become debug_color_init_mon_color(cpu);
}

pub(crate) fn debug_color_init_mon_color(cpu: &mut Cpu) {
    cpu.pc = 0x58fd;
    cpu.set_de(0xc800);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.c = { let value = ((crate::defs::NUM_POKEMON as i32) + (1 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    become debug_color_init_mon_color_loop(cpu);
}

pub(crate) fn debug_color_init_mon_color_loop(cpu: &mut Cpu) {
    cpu.pc = 0x5902;
    cpu.push_word(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    debug_color_init_color(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.set_bc(8 as u16);
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
    if !cpu.flag_z {
        cpu.cycle(12);
        become debug_color_init_mon_color_loop(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn debug_color_init_trainer_color(cpu: &mut Cpu) {
    cpu.pc = 0x5911;
    cpu.set_hl(0x70ce);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xc800);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.c = { let value = ((crate::defs::NUM_TRAINER_CLASSES as i32) + (1 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    become debug_color_init_trainer_color_loop(cpu);
}

pub(crate) fn debug_color_init_trainer_color_loop(cpu: &mut Cpu) {
    cpu.pc = 0x5919;
    cpu.push_word(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    debug_color_init_color(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.set_bc(4 as u16);
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
    if !cpu.flag_z {
        cpu.cycle(12);
        become debug_color_init_trainer_color_loop(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn debug_color_init_color(cpu: &mut Cpu) {
    cpu.pc = 0x5928;
    cpu.a = 0x02;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::get_far_byte(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 0x02;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::get_far_byte(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 0x02;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::get_far_byte(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 0x02;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::get_far_byte(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn debug_color_init_vram(cpu: &mut Cpu) {
    cpu.pc = 0x5948;
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_VBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.set_hl(0x8000);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(0x2000);
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
    cpu.a = 0x0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_VBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.set_hl(0x8000);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(0x2000);
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
    cpu.set_hl({ const N: i32 = ((((0 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (0 as i32)) + (0xcdd9 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(crate::defs::SCREEN_AREA as u16);
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
    cpu.set_hl({ const N: i32 = ((((0 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (0 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(crate::defs::SCREEN_AREA as u16);
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
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::clear_sprites::clear_sprites(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn debug_color_load_gfx(cpu: &mut Cpu) {
    cpu.pc = 0x597c;
    cpu.set_hl(0x5ff3);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de({ const N: i32 = ((0x9000 as i32) + ((crate::defs::TILE_SIZE as u16 as i32) * (DEBUGTEST_TICKS_1 as u16 as i32))); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(0x0160);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0x5fe3);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0x8000);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(0x0010);
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
    crate::game::home::gfx::load_standard_font(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0x8800);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(0x0800);
    cpu.pc += 3;
    cpu.cycle(12);
    become debug_color_load_gfx_loop(cpu);
}

pub(crate) fn debug_color_load_gfx_loop(cpu: &mut Cpu) {
    cpu.pc = 0x599d;
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0xff;
    cpu.a ^= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_bc(cpu.bc().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.b;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(12);
        become debug_color_load_gfx_loop(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn debug_color_init_palettes(cpu: &mut Cpu) {
    cpu.pc = 0x59a7;
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
    if cpu.flag_z {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(crate::defs::R_WBK as u16);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = 0x05;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.set_hl(0x59f4);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xd080);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(0x0080);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = crate::defs::BGPI_AUTOINC as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_BGPI as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.set_hl(0x59f4);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.c = { let value = ((8 as i32) * (crate::defs::PAL_SIZE as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    become debug_color_init_palettes_bg_loop(cpu);
}

pub(crate) fn debug_color_init_palettes_bg_loop(cpu: &mut Cpu) {
    cpu.pc = 0x59c8;
    cpu.write_byte(crate::defs::R_BGPD as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
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
        become debug_color_init_palettes_bg_loop(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = crate::defs::OBPI_AUTOINC as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_OBPI as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.set_hl(0x5a34);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.c = { let value = ((8 as i32) * (crate::defs::PAL_SIZE as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    become debug_color_init_palettes_ob_loop(cpu);
}

pub(crate) fn debug_color_init_palettes_ob_loop(cpu: &mut Cpu) {
    cpu.pc = 0x59d6;
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_OBPD as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
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
        become debug_color_init_palettes_ob_loop(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = (((((((1 as u32) << (crate::defs::B_COLOR_RED as u32)) * (20 as u32)) + (((1 as u32) << (crate::defs::B_COLOR_GREEN as u32)) * (20 as u32))) + (((1 as u32) << (crate::defs::B_COLOR_BLUE as u32)) * (20 as u32)))) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte({ const N: i32 = (0xc608 as i32) + (0 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = ((((((((1 as u32) << (crate::defs::B_COLOR_RED as u32)) * (20 as u32)) + (((1 as u32) << (crate::defs::B_COLOR_GREEN as u32)) * (20 as u32))) + (((1 as u32) << (crate::defs::B_COLOR_BLUE as u32)) * (20 as u32)))) >> 8) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte({ const N: i32 = (0xc608 as i32) + (1 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = (((((((1 as u32) << (crate::defs::B_COLOR_RED as u32)) * (10 as u32)) + (((1 as u32) << (crate::defs::B_COLOR_GREEN as u32)) * (10 as u32))) + (((1 as u32) << (crate::defs::B_COLOR_BLUE as u32)) * (10 as u32)))) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte({ const N: i32 = (0xc60a as i32) + (0 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = ((((((((1 as u32) << (crate::defs::B_COLOR_RED as u32)) * (10 as u32)) + (((1 as u32) << (crate::defs::B_COLOR_GREEN as u32)) * (10 as u32))) + (((1 as u32) << (crate::defs::B_COLOR_BLUE as u32)) * (10 as u32)))) >> 8) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte({ const N: i32 = (0xc60a as i32) + (1 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn palette_debug_bg(cpu: &mut Cpu) {
    cpu.pc = 0x59f4;
    become palette_debug_ob(cpu);
}

pub(crate) fn palette_debug_ob(cpu: &mut Cpu) {
    cpu.pc = 0x5a34;
    become debug_color_main(cpu);
}

pub(crate) fn debug_color_main(cpu: &mut Cpu) {
    cpu.pc = 0x5a74;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::joypad::joy_text_delay(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xcf63);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = DEBUGCOLORMAIN_INITTMHM as u8;
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
        become debug_color_main_no_start_select(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xffa9);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = crate::defs::PAD_SELECT as u8;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(12);
        become debug_color_main_next_mon(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = crate::defs::PAD_START as u8;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(12);
        become debug_color_main_previous_mon(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become debug_color_main_no_start_select(cpu);
}

pub(crate) fn debug_color_main_next_mon(cpu: &mut Cpu) {
    cpu.pc = 0x5a9a;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    debug_color_backup_sprite_colors(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    debug_color_main_set_max_num(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xcf66);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.a;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.e;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_c {
        cpu.cycle(12);
        become debug_color_main_switch_mon(cpu);
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
    become debug_color_main_switch_mon(cpu);
}

pub(crate) fn debug_color_main_previous_mon(cpu: &mut Cpu) {
    cpu.pc = 0x5aab;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    debug_color_backup_sprite_colors(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xcf66);
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
    let value = -1i16 as u8;
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
        become debug_color_main_switch_mon(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    debug_color_main_set_max_num(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    become debug_color_main_switch_mon(cpu);
}

pub(crate) fn debug_color_main_switch_mon(cpu: &mut Cpu) {
    cpu.pc = 0x5aba;
    cpu.write_byte(0xcf66, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = DEBUGCOLORMAIN_INITSCREEN as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcf63, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn debug_color_main_set_max_num(cpu: &mut Cpu) {
    cpu.pc = 0x5ac3;
    cpu.a = cpu.read_byte(0xd002);
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
        become debug_color_main_trainer(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = crate::defs::NUM_POKEMON as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn debug_color_main_trainer(cpu: &mut Cpu) {
    cpu.pc = 0x5acc;
    cpu.a = crate::defs::NUM_TRAINER_CLASSES as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn debug_color_init_screen(cpu: &mut Cpu) {
    cpu.pc = 0x5adb;
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
    cpu.set_bc(crate::defs::SCREEN_AREA as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = DEBUGTEST_BLACK as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::byte_fill(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = ((((3 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (1 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(0x0712);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = DEBUGTEST_WHITE as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    debug_color_fill_box_with_byte(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = ((((0 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (11 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(0x0203);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = DEBUGTEST_LIGHT as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    debug_color_fill_box_with_byte(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = ((((0 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (16 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(0x0203);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = DEBUGTEST_DARK as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    debug_color_fill_box_with_byte(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    debug_color_load_rgb_meter(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    debug_color_set_rgb_meter(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xcf66);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.a;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xd108, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xd265, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl({ const N: i32 = ((((1 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (0 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xd265);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(0x8103);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::print_text::print_num(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xd002);
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
        become debug_color_init_screen_trainer(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = crate::defs::UNOWN_A as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xd234, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::names::get_pokemon_name(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = ((((1 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (4 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::place_string(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc2c6, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl({ const N: i32 = ((((3 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (12 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::pokemon::_prep_mon_frontpic(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_de({ const N: i32 = ((0x9000 as i32) + ((crate::defs::TILE_SIZE as u16 as i32) * (0x31 as i32))); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = { let value = (((0x4622 as i32) - (0x456b as i32)) / (3 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::predef::predef(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x31;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xffad, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.set_hl({ const N: i32 = ((((4 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (2 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(0x0606);
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
    cpu.a = cpu.read_byte(0xd003);
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
        become debug_color_init_screen_normal(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_de(0x5baf);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(12);
    become debug_color_init_screen_place_text(cpu);
}

pub(crate) fn debug_color_init_screen_normal(cpu: &mut Cpu) {
    cpu.pc = 0x5b66;
    cpu.set_de(0x5bb4);
    cpu.pc += 3;
    cpu.cycle(12);
    become debug_color_init_screen_place_text(cpu);
}

pub(crate) fn debug_color_init_screen_place_text(cpu: &mut Cpu) {
    cpu.pc = 0x5b69;
    cpu.set_hl({ const N: i32 = ((((17 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (7 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::place_string(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = ((((17 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (0 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0x5bb9);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::place_string(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(12);
    become debug_color_init_screen_done(cpu);
}

pub(crate) fn debug_color_init_screen_trainer(cpu: &mut Cpu) {
    cpu.pc = 0x5b7a;
    cpu.a = cpu.read_byte(0xd265);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xd233, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0x557b);
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
    cpu.set_hl({ const N: i32 = ((((1 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (4 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::place_string(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_de(0x9000);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0x520d);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0x14;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xd204, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xffad, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.set_hl({ const N: i32 = ((((3 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (2 as i32)) + (0xc4a0 as i32)); N as u16 });
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
    become debug_color_init_screen_done(cpu);
}

pub(crate) fn debug_color_init_screen_done(cpu: &mut Cpu) {
    cpu.pc = 0x5ba9;
    cpu.a = DEBUGCOLORMAIN_UPDATESCREEN as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcf63, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn debug_color_load_rgb_meter(cpu: &mut Cpu) {
    cpu.pc = 0x5bc0;
    cpu.set_de({ const N: i32 = ((((11 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (0 as i32)) + (0xcdd9 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl({ const N: i32 = ((((11 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (2 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 1;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    debug_color_load_rgb_meter_load_meter(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_de({ const N: i32 = ((((13 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (0 as i32)) + (0xcdd9 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl({ const N: i32 = ((((13 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (2 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 2;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    debug_color_load_rgb_meter_load_meter(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_de({ const N: i32 = ((((15 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (0 as i32)) + (0xcdd9 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl({ const N: i32 = ((((15 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (2 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 3;
    cpu.pc += 2;
    cpu.cycle(8);
    become debug_color_load_rgb_meter_load_meter(cpu);
}

pub(crate) fn debug_color_load_rgb_meter_load_meter(cpu: &mut Cpu) {
    cpu.pc = 0x5bde;
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = DEBUGTEST_TICKS_1 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_bc(15 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = DEBUGTEST_TICKS_2 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::byte_fill(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.l = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.h = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.set_bc(0x0028);
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

pub(crate) fn debug_color_set_rgb_meter(cpu: &mut Cpu) {
    cpu.pc = 0x5bf4;
    cpu.a = cpu.read_byte(0xcf66);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.a;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.h = 0;
    cpu.pc += 2;
    cpu.cycle(8);
    let lhs = cpu.hl();
    let rhs = cpu.hl();
    let (result, carry) = lhs.overflowing_add(rhs);
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0fff) + (rhs & 0x0fff) > 0x0fff;
    cpu.flag_c = carry;
    cpu.set_hl(result);
    cpu.pc += 1;
    cpu.cycle(8);
    let lhs = cpu.hl();
    let rhs = cpu.hl();
    let (result, carry) = lhs.overflowing_add(rhs);
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0fff) + (rhs & 0x0fff) > 0x0fff;
    cpu.flag_c = carry;
    cpu.set_hl(result);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(0xc800);
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
    cpu.set_de(0xc608);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(4 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcf64, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xcf65, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_de(0xc608);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    debug_color_calculate_rgb(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn debug_color_update_screen(cpu: &mut Cpu) {
    cpu.pc = 0x5c18;
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
    if cpu.flag_z {
        cpu.cycle(12);
        become debug_color_update_screen_sgb(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 2;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xffd4, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::delay::delay_frame(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::delay::delay_frame(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::delay::delay_frame(cpu);
    assert_eq!(cpu.pc, next_pc);
    become debug_color_update_screen_sgb(cpu);
}

pub(crate) fn debug_color_update_screen_sgb(cpu: &mut Cpu) {
    cpu.pc = 0x5c2a;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::tilemap::wait_bg_map(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = DEBUGCOLORMAIN_UPDATEPALETTES as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcf63, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn debug_color_update_palettes(cpu: &mut Cpu) {
    cpu.pc = 0x5c33;
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
    if cpu.flag_z {
        cpu.cycle(12);
        become debug_color_update_palettes_sgb(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(crate::defs::R_WBK as u16);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = 0x05;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.set_hl(0xd080);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xc608);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.c = 1;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    debug_color_load_palettes_white_col1_col2_black(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = ((((2 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (10 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xc608);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    debug_color_print_hex_color(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = ((((2 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (15 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xc60a);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    debug_color_print_hex_color(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = crate::defs::TRUE as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xffe5, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = DEBUGCOLORMAIN_JOYPAD as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcf63, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn debug_color_update_palettes_sgb(cpu: &mut Cpu) {
    cpu.pc = 0x5c69;
    cpu.set_hl(0xcda9);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = ((crate::defs::PALRGB_WHITE as u32) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = (((crate::defs::PALRGB_WHITE as u32) >> 8) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte({ const N: i32 = (0xc608 as i32) + (0 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte({ const N: i32 = (0xc608 as i32) + (1 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte({ const N: i32 = (0xc60a as i32) + (0 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte({ const N: i32 = (0xc60a as i32) + (1 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
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
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(0xcda9);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    debug_color_push_sgb_pals(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = ((((2 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (10 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xc608);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    debug_color_print_hex_color(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = ((((2 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (15 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xc60a);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    debug_color_print_hex_color(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = DEBUGCOLORMAIN_JOYPAD as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcf63, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn debug_color_print_hex_color(cpu: &mut Cpu) {
    cpu.pc = 0x5ca7;
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    debug_color_print_hex_color_place_tile(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
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
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    debug_color_print_hex_color_place_tile(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    debug_color_print_hex_color_place_tile(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
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
    become debug_color_print_hex_color_place_tile(cpu);
}

pub(crate) fn debug_color_print_hex_color_place_tile(cpu: &mut Cpu) {
    cpu.pc = 0x5cbc;
    let value = 0xf;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = DEBUGTEST_0 as u8;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_add(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) > 0x0f;
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn debug_color_joypad_tmhm(cpu: &mut Cpu) {
    cpu.pc = 0x5cdf;
    cpu.a = DEBUGCOLORMAIN_INITTMHM as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcf63, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn debug_color_joypad_toggle_shiny(cpu: &mut Cpu) {
    cpu.pc = 0x5ce5;
    cpu.a = cpu.read_byte(0xd002);
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
    cpu.a = cpu.read_byte(0xd003);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0b00000100;
    cpu.a ^= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xd003, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.b = 0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x68ce);
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
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    debug_color_init_mon_color(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = DEBUGCOLORMAIN_INITSCREEN as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcf63, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn debug_color_select_color_box(cpu: &mut Cpu) {
    cpu.pc = 0x5d0a;
    cpu.set_hl(0xffa9);
    cpu.pc += 3;
    cpu.cycle(12);
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
        become debug_color_next_rgb_color(cpu);
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
        become debug_color_select_color_box_light(cpu);
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
        become debug_color_select_color_box_dark(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn debug_color_select_color_box_light(cpu: &mut Cpu) {
    cpu.pc = 0x5d1d;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcf65, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_de(0xc608);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    debug_color_calculate_rgb(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn debug_color_select_color_box_dark(cpu: &mut Cpu) {
    cpu.pc = 0x5d28;
    cpu.a = crate::defs::TRUE as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcf65, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_de(0xc60a);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    debug_color_calculate_rgb(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn debug_color_change_red_value(cpu: &mut Cpu) {
    cpu.pc = 0x5d34;
    cpu.set_hl(0xffa9);
    cpu.pc += 3;
    cpu.cycle(12);
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
        become debug_color_next_rgb_color(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
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
        become debug_color_previous_rgb_color(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xc612);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(12);
    become debug_color_update_rgb_color(cpu);
}

pub(crate) fn debug_color_change_green_value(cpu: &mut Cpu) {
    cpu.pc = 0x5d46;
    cpu.set_hl(0xffa9);
    cpu.pc += 3;
    cpu.cycle(12);
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
        become debug_color_next_rgb_color(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
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
        become debug_color_previous_rgb_color(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xc613);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(12);
    become debug_color_update_rgb_color(cpu);
}

pub(crate) fn debug_color_change_blue_value(cpu: &mut Cpu) {
    cpu.pc = 0x5d58;
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
        become debug_color_previous_rgb_color(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xc614);
    cpu.pc += 3;
    cpu.cycle(12);
    become debug_color_update_rgb_color(cpu);
}

pub(crate) fn debug_color_update_rgb_color(cpu: &mut Cpu) {
    cpu.pc = 0x5d63;
    cpu.a = cpu.read_byte(0xffa9);
    cpu.pc += 2;
    cpu.cycle(12);
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
        become debug_color_update_rgb_color_increment(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xffa9);
    cpu.pc += 2;
    cpu.cycle(12);
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
        become debug_color_update_rgb_color_decrement(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn debug_color_update_rgb_color_increment(cpu: &mut Cpu) {
    cpu.pc = 0x5d70;
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 31;
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
    cpu.cycle(12);
    become debug_color_update_rgb_color_done(cpu);
}

pub(crate) fn debug_color_update_rgb_color_decrement(cpu: &mut Cpu) {
    cpu.pc = 0x5d77;
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
    become debug_color_update_rgb_color_done(cpu);
}

pub(crate) fn debug_color_update_rgb_color_done(cpu: &mut Cpu) {
    cpu.pc = 0x5d7b;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    debug_color_calculate_palette(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = DEBUGCOLORMAIN_UPDATEPALETTES as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcf63, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn debug_color_previous_rgb_color(cpu: &mut Cpu) {
    cpu.pc = 0x5d84;
    cpu.set_hl(0xcf64);
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
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn debug_color_next_rgb_color(cpu: &mut Cpu) {
    cpu.pc = 0x5d89;
    cpu.set_hl(0xcf64);
    cpu.pc += 3;
    cpu.cycle(12);
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

pub(crate) fn debug_color_init_tmhm(cpu: &mut Cpu) {
    cpu.pc = 0x5d8e;
    cpu.set_hl({ const N: i32 = ((((10 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (0 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(0x00a0);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = DEBUGTEST_BLACK as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::byte_fill(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = ((((12 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (2 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0x5fcd);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::place_string(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xd004, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    debug_color_print_tmhm_move(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = DEBUGCOLORMAIN_TMHMJOYPAD as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcf63, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn debug_color_tmhm_joypad(cpu: &mut Cpu) {
    cpu.pc = 0x5daf;
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
        become debug_color_tmhm_joypad_cancel(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    debug_color_tmhm_joypad_scroll(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn debug_color_tmhm_joypad_cancel(cpu: &mut Cpu) {
    cpu.pc = 0x5dbb;
    cpu.a = DEBUGCOLORMAIN_INITSCREEN as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcf63, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn debug_color_tmhm_joypad_exit(cpu: &mut Cpu) {
    cpu.pc = 0x5dc1;
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

pub(crate) fn debug_color_tmhm_joypad_scroll(cpu: &mut Cpu) {
    cpu.pc = 0x5dc7;
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
        become debug_color_tmhm_joypad_up(cpu);
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
        become debug_color_tmhm_joypad_down(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn debug_color_tmhm_joypad_up(cpu: &mut Cpu) {
    cpu.pc = 0x5dd5;
    cpu.a = cpu.read_byte(0xd004);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = { let value = ((crate::defs::NUM_TM_HM_TUTOR as i32) - (1 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
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
        become debug_color_tmhm_joypad_wrap_down(cpu);
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
    cpu.cycle(12);
    become debug_color_tmhm_joypad_done(cpu);
}

pub(crate) fn debug_color_tmhm_joypad_wrap_down(cpu: &mut Cpu) {
    cpu.pc = 0x5ddf;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.cycle(12);
    become debug_color_tmhm_joypad_done(cpu);
}

pub(crate) fn debug_color_tmhm_joypad_down(cpu: &mut Cpu) {
    cpu.pc = 0x5de2;
    cpu.a = cpu.read_byte(0xd004);
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
        become debug_color_tmhm_joypad_wrap_up(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.cycle(12);
    become debug_color_tmhm_joypad_done(cpu);
}

pub(crate) fn debug_color_tmhm_joypad_wrap_up(cpu: &mut Cpu) {
    cpu.pc = 0x5deb;
    cpu.a = { let value = ((crate::defs::NUM_TM_HM_TUTOR as i32) - (1 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    become debug_color_tmhm_joypad_done(cpu);
}

pub(crate) fn debug_color_tmhm_joypad_done(cpu: &mut Cpu) {
    cpu.pc = 0x5ded;
    cpu.write_byte(0xd004, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    debug_color_print_tmhm_move(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn debug_color_print_tmhm_move(cpu: &mut Cpu) {
    cpu.pc = 0x5df4;
    cpu.set_hl({ const N: i32 = ((((11 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (10 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    debug_color_print_tmhm_move_clear_row(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = ((((12 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (10 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    debug_color_print_tmhm_move_clear_row(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = ((((13 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (10 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    debug_color_print_tmhm_move_clear_row(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = ((((14 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (10 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    debug_color_print_tmhm_move_clear_row(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xd004);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.a;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xd265, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = { let value = (((0x4598 as i32) - (0x456b as i32)) / (3 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::predef::predef(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xd265);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xd262, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::names::get_move_name(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = ((((12 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (10 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::place_string(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xd004);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    debug_color_print_tmhm_move_get_numbered_tmhm(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.write_byte(0xd106, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = { let value = (((0x4595 as i32) - (0x456b as i32)) / (3 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::predef::predef(cpu);
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
    cpu.set_de(0x5e46);
    cpu.pc += 3;
    cpu.cycle(12);
    if !cpu.flag_z {
        cpu.cycle(12);
        become debug_color_print_tmhm_move_place_string(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_de(0x5e4d);
    cpu.pc += 3;
    cpu.cycle(12);
    become debug_color_print_tmhm_move_place_string(cpu);
}

pub(crate) fn debug_color_print_tmhm_move_place_string(cpu: &mut Cpu) {
    cpu.pc = 0x5e3f;
    cpu.set_hl({ const N: i32 = ((((14 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (10 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::place_string(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn debug_color_print_tmhm_move_get_numbered_tmhm(cpu: &mut Cpu) {
    cpu.pc = 0x5e55;
    let value = crate::defs::NUM_TMS as u8;
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
        become debug_color_print_tmhm_move_tm(cpu);
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
    let value = cpu.a;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    become debug_color_print_tmhm_move_tm(cpu);
}

pub(crate) fn debug_color_print_tmhm_move_tm(cpu: &mut Cpu) {
    cpu.pc = 0x5e5b;
    let value = crate::defs::TM01 as u8;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_add(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) > 0x0f;
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn debug_color_print_tmhm_move_clear_row(cpu: &mut Cpu) {
    cpu.pc = 0x5e5e;
    cpu.set_bc(10 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = DEBUGTEST_BLACK as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::byte_fill(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn debug_color_calculate_palette(cpu: &mut Cpu) {
    cpu.pc = 0x5e67;
    cpu.a = cpu.read_byte(0xc612);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0b00011111;
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
    cpu.a = cpu.read_byte(0xc613);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0b00000111;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value << 1;
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x80 != 0;
    cpu.a = result;
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
    let value = cpu.e;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xc613);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0b00011000;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value << 1;
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x80 != 0;
    cpu.a = result;
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
    cpu.d = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xc614);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0b00011111;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value << 1;
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x80 != 0;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value << 1;
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x80 != 0;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.d;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.d = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xcf65);
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
        become debug_color_calculate_palette_light(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte({ const N: i32 = (0xc60a as i32) + (0 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte({ const N: i32 = (0xc60a as i32) + (1 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn debug_color_calculate_palette_light(cpu: &mut Cpu) {
    cpu.pc = 0x5e9c;
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte({ const N: i32 = (0xc608 as i32) + (0 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte({ const N: i32 = (0xc608 as i32) + (1 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn debug_color_calculate_rgb(cpu: &mut Cpu) {
    cpu.pc = 0x5ea5;
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0b00011111;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc612, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0b11100000;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
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
    let value = cpu.a;
    let result = value >> 1;
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x01 != 0;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0b00000011;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
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
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc613, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0b01111100;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
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
    let value = cpu.a;
    let result = value >> 1;
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x01 != 0;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc614, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn debug_color_backup_sprite_colors(cpu: &mut Cpu) {
    cpu.pc = 0x5eca;
    cpu.a = cpu.read_byte(0xcf66);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.a;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.h = 0;
    cpu.pc += 2;
    cpu.cycle(8);
    let lhs = cpu.hl();
    let rhs = cpu.hl();
    let (result, carry) = lhs.overflowing_add(rhs);
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0fff) + (rhs & 0x0fff) > 0x0fff;
    cpu.flag_c = carry;
    cpu.set_hl(result);
    cpu.pc += 1;
    cpu.cycle(8);
    let lhs = cpu.hl();
    let rhs = cpu.hl();
    let (result, carry) = lhs.overflowing_add(rhs);
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0fff) + (rhs & 0x0fff) > 0x0fff;
    cpu.flag_c = carry;
    cpu.set_hl(result);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(0xc800);
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
    cpu.e = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.d = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl(0xc608);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(4 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn debug_color_load_palettes_white_col1_col2_black(cpu: &mut Cpu) {
    cpu.pc = 0x5ee3;
    become debug_color_load_palettes_white_col1_col2_black_loop(cpu);
}

pub(crate) fn debug_color_load_palettes_white_col1_col2_black_loop(cpu: &mut Cpu) {
    cpu.pc = 0x5ee3;
    cpu.a = ((crate::defs::PALRGB_WHITE as u32) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = (((crate::defs::PALRGB_WHITE as u32) >> 8) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
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
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
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
        become debug_color_load_palettes_white_col1_col2_black_loop(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn debug_color_fill_box_with_byte(cpu: &mut Cpu) {
    cpu.pc = 0x5efc;
    become debug_color_fill_box_with_byte_row(cpu);
}

pub(crate) fn debug_color_fill_box_with_byte_row(cpu: &mut Cpu) {
    cpu.pc = 0x5efc;
    cpu.push_word(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    become debug_color_fill_box_with_byte_col(cpu);
}

pub(crate) fn debug_color_fill_box_with_byte_col(cpu: &mut Cpu) {
    cpu.pc = 0x5efe;
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
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
        become debug_color_fill_box_with_byte_col(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
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
    let value = cpu.pop_word();
    cpu.set_bc(value);
    cpu.pc += 1;
    cpu.cycle(12);
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
        become debug_color_fill_box_with_byte_row(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn debug_color_push_sgb_pals(cpu: &mut Cpu) {
    cpu.pc = 0x5f0c;
    cpu.a = cpu.read_byte(0xcfbe);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    let mask = 1u8 << ((crate::defs::JOYPAD_DISABLE_SGB_TRANSFER_F as u8) & 7);
    cpu.a |= mask;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcfbe, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    _debug_color_push_sgb_pals(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte(0xcfbe, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn _debug_color_push_sgb_pals(cpu: &mut Cpu) {
    cpu.pc = 0x5f1d;
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0x7;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    become _debug_color_push_sgb_pals_loop(cpu);
}

pub(crate) fn _debug_color_push_sgb_pals_loop(cpu: &mut Cpu) {
    cpu.pc = 0x5f22;
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
    cpu.write_byte(crate::defs::R_JOYP as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = crate::defs::JOYP_SGB_FINISH as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_JOYP as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.b = crate::defs::JOYP_SGB_ONE as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    become _debug_color_push_sgb_pals_loop2(cpu);
}

pub(crate) fn _debug_color_push_sgb_pals_loop2(cpu: &mut Cpu) {
    cpu.pc = 0x5f2c;
    cpu.e = 8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.d = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    become _debug_color_push_sgb_pals_loop3(cpu);
}

pub(crate) fn _debug_color_push_sgb_pals_loop3(cpu: &mut Cpu) {
    cpu.pc = 0x5f30;
    let mask = 1u8 << ((0) & 7);
    cpu.flag_z = (cpu.d & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = crate::defs::JOYP_SGB_ONE as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(12);
        become _debug_color_push_sgb_pals_okay(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = crate::defs::JOYP_SGB_ZERO as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    become _debug_color_push_sgb_pals_okay(cpu);
}

pub(crate) fn _debug_color_push_sgb_pals_okay(cpu: &mut Cpu) {
    cpu.pc = 0x5f38;
    cpu.write_byte(crate::defs::R_JOYP as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = crate::defs::JOYP_SGB_FINISH as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_JOYP as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    let value = cpu.d;
    let carry_in = u8::from(cpu.flag_c) << 7;
    let result = (value >> 1) | carry_in;
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x01 != 0;
    cpu.d = result;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.e;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.e = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(12);
        become _debug_color_push_sgb_pals_loop3(cpu);
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
        become _debug_color_push_sgb_pals_loop2(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = crate::defs::JOYP_SGB_ZERO as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_JOYP as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = crate::defs::JOYP_SGB_FINISH as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_JOYP as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.set_de(7000 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    become _debug_color_push_sgb_pals_wait(cpu);
}

pub(crate) fn _debug_color_push_sgb_pals_wait(cpu: &mut Cpu) {
    cpu.pc = 0x5f51;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_de(cpu.de().wrapping_sub(1));
    cpu.pc += 1;
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
    if !cpu.flag_z {
        cpu.cycle(12);
        become _debug_color_push_sgb_pals_wait(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.pop_word();
    cpu.set_bc(value);
    cpu.pc += 1;
    cpu.cycle(12);
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
        become _debug_color_push_sgb_pals_loop(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn debug_color_place_cursor(cpu: &mut Cpu) {
    cpu.pc = 0x5f5e;
    cpu.a = DEBUGTEST_BLACK as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl({ const N: i32 = ((((0 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (10 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl({ const N: i32 = ((((0 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (15 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl({ const N: i32 = ((((11 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (1 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl({ const N: i32 = ((((13 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (1 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl({ const N: i32 = ((((15 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (1 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xcf63);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = DEBUGCOLORMAIN_JOYPAD as u8;
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
        become debug_color_place_cursor_clearsprites(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xcf64);
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
        become debug_color_place_cursor_place_cursor(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl({ const N: i32 = ((((11 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (1 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(0x0028);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::array::add_n_times(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.write_byte(cpu.hl(), { let value = (0xed as i32); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } });
    cpu.pc += 2;
    cpu.cycle(12);
    become debug_color_place_cursor_place_cursor(cpu);
}

pub(crate) fn debug_color_place_cursor_place_cursor(cpu: &mut Cpu) {
    cpu.pc = 0x5f8d;
    cpu.a = cpu.read_byte(0xcf65);
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
        become debug_color_place_cursor_light(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl({ const N: i32 = ((((0 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (15 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(12);
    become debug_color_place_cursor_place(cpu);
}

pub(crate) fn debug_color_place_cursor_light(cpu: &mut Cpu) {
    cpu.pc = 0x5f98;
    cpu.set_hl({ const N: i32 = ((((0 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (10 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    become debug_color_place_cursor_place(cpu);
}

pub(crate) fn debug_color_place_cursor_place(cpu: &mut Cpu) {
    cpu.pc = 0x5f9b;
    cpu.write_byte(cpu.hl(), { let value = (0xed as i32); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } });
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.b = 0x70;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.c = 5;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xc400);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xc612);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    debug_color_place_cursor_placesprite(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_de(0xc613);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    debug_color_place_cursor_placesprite(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_de(0xc614);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    debug_color_place_cursor_placesprite(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn debug_color_place_cursor_placesprite(cpu: &mut Cpu) {
    cpu.pc = 0x5fb7;
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.a;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_add(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) > 0x0f;
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.a;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_add(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) > 0x0f;
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = { let value = ((3 as i32) * (crate::defs::TILE_WIDTH as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_add(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) > 0x0f;
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
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
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = { let value = ((2 as i32) * (crate::defs::TILE_WIDTH as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
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
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
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

pub(crate) fn debug_color_place_cursor_clearsprites(cpu: &mut Cpu) {
    cpu.pc = 0x5fc9;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::clear_sprites::clear_sprites(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn tileset_color_picker(cpu: &mut Cpu) {
    cpu.pc = 0x6153;
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
    return;
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
    cpu.write_byte(0xcf64, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xcf65, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xcf66, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xffde, cpu.a);
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
    crate::game::home::map::load_overworld_tilemap_and_attrmap_pals(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::tilemap::wait_bg_map2(cpu);
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
    cpu.set_de(0x5ff3);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl({ const N: i32 = ((0x9000 as i32) + ((crate::defs::TILE_SIZE as u16 as i32) * (DEBUGTEST_TICKS_1 as u16 as i32))); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc({ const N: i32 = ((((0x20 as i32) & (0xff as i32)) << (8 as i32)) | ((22 as i32) & (0xff as i32))); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::gfx::request2bpp(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_de(0x5fe3);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0x8800);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc({ const N: i32 = ((((0x20 as i32) & (0xff as i32)) << (8 as i32)) | ((1 as i32) & (0xff as i32))); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::gfx::request2bpp(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = (((0x9c00u32) >> 8) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte({ const N: i32 = (0xffd6 as i32) + (1 as i32); N as u16 }, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.set_hl({ const N: i32 = ((((0 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (0 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(crate::defs::SCREEN_AREA as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = DEBUGTEST_BLACK as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::byte_fill(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = ((((0 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (0 as i32)) + (0xcdd9 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(crate::defs::SCREEN_AREA as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = crate::defs::PAL_BG_TEXT as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::byte_fill(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_de(0x0015);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = DEBUGTEST_WHITE as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    debug_tileset_draw_color_swatch(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_de(0x001a);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = DEBUGTEST_LIGHT as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    debug_tileset_draw_color_swatch(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_de(0x001f);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = DEBUGTEST_DARK as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    debug_tileset_draw_color_swatch(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_de(0x0024);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = DEBUGTEST_BLACK as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    debug_tileset_draw_color_swatch(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    debug_tileset_load_rgb_meter(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    debug_tileset_load_palettes(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::tilemap::wait_bg_map2(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.write_byte(0xcf63, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x40;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xffd2, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn debug_tileset_draw_color_swatch(cpu: &mut Cpu) {
    cpu.pc = 0x61d2;
    cpu.set_hl({ const N: i32 = ((((0 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (0 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    _debug_color_draw_swatch(cpu);
    assert_eq!(cpu.pc, next_pc);
    become debug_color_draw_attribute_swatch(cpu);
}

pub(crate) fn debug_color_draw_attribute_swatch(cpu: &mut Cpu) {
    cpu.pc = 0x61d8;
    cpu.a = cpu.read_byte(0xcf64);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl({ const N: i32 = ((((0 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (0 as i32)) + (0xcdd9 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    become _debug_color_draw_swatch(cpu);
}

pub(crate) fn _debug_color_draw_swatch(cpu: &mut Cpu) {
    cpu.pc = 0x61de;
    let lhs = cpu.hl();
    let rhs = cpu.de();
    let (result, carry) = lhs.overflowing_add(rhs);
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0fff) + (rhs & 0x0fff) > 0x0fff;
    cpu.flag_c = carry;
    cpu.set_hl(result);
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
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_bc({ const N: i32 = (crate::defs::SCREEN_WIDTH as u16 as i32) - (4 as i32); N as u16 });
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
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_bc({ const N: i32 = (crate::defs::SCREEN_WIDTH as u16 as i32) - (4 as i32); N as u16 });
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
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn debug_tileset_load_rgb_meter(cpu: &mut Cpu) {
    cpu.pc = 0x61f4;
    cpu.set_hl({ const N: i32 = ((((4 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (2 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    debug_tileset_load_rgb_meter_place(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = ((((6 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (2 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    debug_tileset_load_rgb_meter_place(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = ((((8 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (2 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    become debug_tileset_load_rgb_meter_place(cpu);
}

pub(crate) fn debug_tileset_load_rgb_meter_place(cpu: &mut Cpu) {
    cpu.pc = 0x6203;
    cpu.a = DEBUGTEST_TICKS_1 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_bc(15 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = DEBUGTEST_TICKS_2 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::byte_fill(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn debug_tileset_load_palettes(cpu: &mut Cpu) {
    cpu.pc = 0x620f;
    cpu.a = cpu.read_byte(crate::defs::R_WBK as u16);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = 0x05;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xcf64);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.h = 0;
    cpu.pc += 2;
    cpu.cycle(8);
    let lhs = cpu.hl();
    let rhs = cpu.hl();
    let (result, carry) = lhs.overflowing_add(rhs);
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0fff) + (rhs & 0x0fff) > 0x0fff;
    cpu.flag_c = carry;
    cpu.set_hl(result);
    cpu.pc += 1;
    cpu.cycle(8);
    let lhs = cpu.hl();
    let rhs = cpu.hl();
    let (result, carry) = lhs.overflowing_add(rhs);
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0fff) + (rhs & 0x0fff) > 0x0fff;
    cpu.flag_c = carry;
    cpu.set_hl(result);
    cpu.pc += 1;
    cpu.cycle(8);
    let lhs = cpu.hl();
    let rhs = cpu.hl();
    let (result, carry) = lhs.overflowing_add(rhs);
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0fff) + (rhs & 0x0fff) > 0x0fff;
    cpu.flag_c = carry;
    cpu.set_hl(result);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(0xd000);
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
    cpu.set_de(0xc608);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(0x0008);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_de(0xc608);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    debug_color_calculate_rgb(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn debug_color_main2(cpu: &mut Cpu) {
    cpu.pc = 0x6236;
    cpu.set_hl(0xffa9);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = crate::defs::PAD_SELECT as u8;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(12);
        become debug_color_main2_next_palette(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
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
        become debug_color_main2_cancel(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    debug_tileset_joypad(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn debug_color_main2_next_palette(cpu: &mut Cpu) {
    cpu.pc = 0x6247;
    cpu.set_hl(0xcf64);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = crate::defs::OAM_PALETTE as u8;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = crate::defs::PAL_BG_TEXT as u8;
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
        become debug_color_main2_palette_ok(cpu);
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
    become debug_color_main2_palette_ok(cpu);
}

pub(crate) fn debug_color_main2_palette_ok(cpu: &mut Cpu) {
    cpu.pc = 0x6253;
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(0x0015);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    debug_color_draw_attribute_swatch(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_de(0x001a);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    debug_color_draw_attribute_swatch(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_de(0x001f);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    debug_color_draw_attribute_swatch(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_de(0x0024);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    debug_color_draw_attribute_swatch(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(crate::defs::R_WBK as u16);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = 0x05;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.set_hl(0xd080);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xcf64);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_bc(0x0008);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::array::add_n_times(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_de(0xc608);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(0x0008);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = 2;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xffd4, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.c = 3;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::delay::delay_frames(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xffd4, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn debug_color_main2_cancel(cpu: &mut Cpu) {
    cpu.pc = 0x6299;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::clear_sprites::clear_sprites(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xffd2);
    cpu.pc += 2;
    cpu.cycle(12);
    let value = 0b11010000;
    cpu.a ^= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xffd2, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn debug_tileset_update_palettes(cpu: &mut Cpu) {
    cpu.pc = 0x62a3;
    cpu.a = cpu.read_byte(crate::defs::R_WBK as u16);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = 0x05;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.set_hl(0xd080);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xcf64);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_bc(0x0008);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::array::add_n_times(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.e = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.d = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl(0xc608);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(0x0008);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = ((((0 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (1 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xc608);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    debug_color_print_hex_color(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = ((((0 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (6 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xc60a);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    debug_color_print_hex_color(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = ((((0 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (11 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xc60c);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    debug_color_print_hex_color(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = ((((0 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (16 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xc60e);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    debug_color_print_hex_color(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = crate::defs::TRUE as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xffe5, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::delay::delay_frame(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn debug_tileset_select_color_box(cpu: &mut Cpu) {
    cpu.pc = 0x6309;
    cpu.set_hl(0xffa9);
    cpu.pc += 3;
    cpu.cycle(12);
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
        become debug_tileset_next_rgb_color(cpu);
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
        become debug_tileset_select_color_box_left(cpu);
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
        become debug_tileset_select_color_box_right(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn debug_tileset_select_color_box_left(cpu: &mut Cpu) {
    cpu.pc = 0x631c;
    cpu.a = cpu.read_byte(0xcf66);
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
    cpu.cycle(12);
    become debug_tileset_select_color_box_done(cpu);
}

pub(crate) fn debug_tileset_select_color_box_right(cpu: &mut Cpu) {
    cpu.pc = 0x6322;
    cpu.a = cpu.read_byte(0xcf66);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.a;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    become debug_tileset_select_color_box_done(cpu);
}

pub(crate) fn debug_tileset_select_color_box_done(cpu: &mut Cpu) {
    cpu.pc = 0x6326;
    let value = 3;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcf66, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.d = 0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xc608);
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
    cpu.e = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.d = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    debug_color_calculate_rgb(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn debug_tileset_change_red_value(cpu: &mut Cpu) {
    cpu.pc = 0x6339;
    cpu.set_hl(0xffa9);
    cpu.pc += 3;
    cpu.cycle(12);
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
        become debug_tileset_next_rgb_color(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
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
        become debug_tileset_previous_rgb_color(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xc612);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(12);
    become debug_tileset_update_rgb_color(cpu);
}

pub(crate) fn debug_tileset_change_green_value(cpu: &mut Cpu) {
    cpu.pc = 0x634b;
    cpu.set_hl(0xffa9);
    cpu.pc += 3;
    cpu.cycle(12);
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
        become debug_tileset_next_rgb_color(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
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
        become debug_tileset_previous_rgb_color(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xc613);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(12);
    become debug_tileset_update_rgb_color(cpu);
}

pub(crate) fn debug_tileset_change_blue_value(cpu: &mut Cpu) {
    cpu.pc = 0x635d;
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
        become debug_tileset_previous_rgb_color(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xc614);
    cpu.pc += 3;
    cpu.cycle(12);
    become debug_tileset_update_rgb_color(cpu);
}

pub(crate) fn debug_tileset_update_rgb_color(cpu: &mut Cpu) {
    cpu.pc = 0x6368;
    cpu.a = cpu.read_byte(0xffa9);
    cpu.pc += 2;
    cpu.cycle(12);
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
        become debug_tileset_update_rgb_color_increment(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xffa9);
    cpu.pc += 2;
    cpu.cycle(12);
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
        become debug_tileset_update_rgb_color_decrement(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn debug_tileset_update_rgb_color_increment(cpu: &mut Cpu) {
    cpu.pc = 0x6375;
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 31;
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
    cpu.cycle(12);
    become debug_tileset_update_rgb_color_done(cpu);
}

pub(crate) fn debug_tileset_update_rgb_color_decrement(cpu: &mut Cpu) {
    cpu.pc = 0x637c;
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
    become debug_tileset_update_rgb_color_done(cpu);
}

pub(crate) fn debug_tileset_update_rgb_color_done(cpu: &mut Cpu) {
    cpu.pc = 0x6380;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    debug_tileset_calculate_palette(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    debug_tileset_update_palettes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn debug_tileset_previous_rgb_color(cpu: &mut Cpu) {
    cpu.pc = 0x6387;
    cpu.set_hl(0xcf65);
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
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn debug_tileset_next_rgb_color(cpu: &mut Cpu) {
    cpu.pc = 0x638c;
    cpu.set_hl(0xcf65);
    cpu.pc += 3;
    cpu.cycle(12);
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

pub(crate) fn debug_tileset_calculate_palette(cpu: &mut Cpu) {
    cpu.pc = 0x6391;
    cpu.a = cpu.read_byte(0xc612);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0b00011111;
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
    cpu.a = cpu.read_byte(0xc613);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0b0000111;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value << 1;
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x80 != 0;
    cpu.a = result;
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
    let value = cpu.e;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xc613);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0b00011000;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value << 1;
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x80 != 0;
    cpu.a = result;
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
    cpu.d = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xc614);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0b00011111;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value << 1;
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x80 != 0;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value << 1;
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x80 != 0;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.d;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.d = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xcf66);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.b = 0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xc608);
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
    let lhs = cpu.hl();
    let rhs = cpu.bc();
    let (result, carry) = lhs.overflowing_add(rhs);
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0fff) + (rhs & 0x0fff) > 0x0fff;
    cpu.flag_c = carry;
    cpu.set_hl(result);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.d);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn debug_tileset_calculate_palette_dummy1(cpu: &mut Cpu) {
    cpu.pc = 0x63c6;
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn debug_tileset_calculate_palette_dummy2(cpu: &mut Cpu) {
    cpu.pc = 0x63c7;
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn debug_color_joypad(cpu: &mut Cpu) {
    cpu.a = cpu.read_byte(0xffa9);
    cpu.a &= crate::defs::PAD_B as u8;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    if !cpu.flag_z {
        become debug_color_joypad_tmhm(cpu);
    }

    cpu.a = cpu.read_byte(0xffa9);
    cpu.a &= crate::defs::PAD_A as u8;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    if !cpu.flag_z {
        become debug_color_joypad_toggle_shiny(cpu);
    }

    match cpu.read_byte(0xcf64) & 3 {
        0 => become debug_color_select_color_box(cpu),
        1 => become debug_color_change_red_value(cpu),
        2 => become debug_color_change_green_value(cpu),
        3 => become debug_color_change_blue_value(cpu),
        n => panic!("unsupported debug color RGB step: {}", n),
    }
}

pub(crate) fn debug_color_main_no_start_select(cpu: &mut Cpu) {
    match cpu.read_byte(0xcf63) {
        0 => become debug_color_init_screen(cpu),
        1 => become debug_color_update_screen(cpu),
        2 => become debug_color_update_palettes(cpu),
        3 => become debug_color_joypad(cpu),
        4 => become debug_color_init_tmhm(cpu),
        5 => become debug_color_tmhm_joypad(cpu),
        n => panic!("unsupported debug color step: {}", n),
    };
}

pub(crate) fn debug_tileset_joypad(cpu: &mut Cpu) {
    match cpu.read_byte(0xcf65) & 3 {
        0 => become debug_tileset_select_color_box(cpu),
        1 => become debug_tileset_change_red_value(cpu),
        2 => become debug_tileset_change_green_value(cpu),
        3 => become debug_tileset_change_blue_value(cpu),
        n => panic!("unsupported debug tileset RGB step: {}", n),
    }
}
