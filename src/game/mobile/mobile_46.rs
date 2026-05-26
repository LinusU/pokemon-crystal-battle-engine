use crate::runtime::Cpu;

pub(crate) fn function118000(cpu: &mut Cpu) {
    cpu.pc = 0x4000;
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd38, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(12);
    become asm_11800b(cpu);
}

pub(crate) fn function118007(cpu: &mut Cpu) {
    cpu.pc = 0x4007;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcd38, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    become asm_11800b(cpu);
}

pub(crate) fn asm_11800b(cpu: &mut Cpu) {
    cpu.pc = 0x400b;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    battle_tower_room_menu_init_ram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x18;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd33, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x19;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd34, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x4;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc3f0, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(crate::defs::R_WBK as u16);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = 0x3;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    become asm_11800b_loop(cpu);
}

pub(crate) fn asm_11800b_loop(cpu: &mut Cpu) {
    cpu.pc = 0x4024;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::joypad::joy_text_delay(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function118473(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xcf66);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x1b;
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
        become asm_11800b_skip(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xcd34);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xcf66, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    become asm_11800b_skip(cpu);
}

pub(crate) fn asm_11800b_skip(cpu: &mut Cpu) {
    cpu.pc = 0x4037;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function1184a5(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    battle_tower_room_menu_write_message(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x45;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x5dd3);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x45;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x619d);
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
    cpu.a = cpu.read_byte(0xcf66);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xcd33);
    cpu.pc += 3;
    cpu.cycle(12);
    let value = cpu.read_byte(cpu.hl());
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 1;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(12);
        become asm_11800b_loop(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    battle_tower_room_menu_cleanup(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map::return_to_map_from_submenu(cpu);
    assert_eq!(cpu.pc, next_pc);
    become battle_tower_room_menu_do_nothing(cpu);
}

pub(crate) fn battle_tower_room_menu_do_nothing(cpu: &mut Cpu) {
    cpu.pc = 0x405e;
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function11805f(cpu: &mut Cpu) {
    cpu.pc = 0x405f;
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd38, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    battle_tower_room_menu_init_ram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x18;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd33, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x19;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd34, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x4;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc3f0, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(crate::defs::R_WBK as u16);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = 0x3;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    become function11805f_asm_11807d(cpu);
}

pub(crate) fn function11805f_asm_11807d(cpu: &mut Cpu) {
    cpu.pc = 0x407d;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::joypad::joy_text_delay(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function118473(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xcf66);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x1b;
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
        become function11805f_asm_118090(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xcd34);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xcf66, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    become function11805f_asm_118090(cpu);
}

pub(crate) fn function11805f_asm_118090(cpu: &mut Cpu) {
    cpu.pc = 0x4090;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11857c(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    battle_tower_room_menu_write_message(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x45;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x5dd3);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x45;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x619d);
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
    cpu.a = cpu.read_byte(0xcf66);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xcd33);
    cpu.pc += 3;
    cpu.cycle(12);
    let value = cpu.read_byte(cpu.hl());
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 1;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(12);
        become function11805f_asm_11807d(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    battle_tower_room_menu_cleanup(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map::return_to_map_from_submenu(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function1180b8(cpu: &mut Cpu) {
    cpu.pc = 0x40b8;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    battle_tower_room_menu_init_ram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x22;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd33, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x23;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd34, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x4;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc3f0, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(crate::defs::R_WBK as u16);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = 0x3;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    become function1180b8_asm_1180d1(cpu);
}

pub(crate) fn function1180b8_asm_1180d1(cpu: &mut Cpu) {
    cpu.pc = 0x40d1;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::joypad::joy_text_delay(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function118473(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xcf66);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x28;
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
        become function1180b8_asm_1180e4(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xcd34);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xcf66, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    become function1180b8_asm_1180e4(cpu);
}

pub(crate) fn function1180b8_asm_1180e4(cpu: &mut Cpu) {
    cpu.pc = 0x40e4;
    cpu.a = cpu.read_byte(0xcf66);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x10;
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
        become function1180b8_asm_1180f2(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x16;
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
        become function1180b8_asm_1180f2(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11884c(cpu);
    assert_eq!(cpu.pc, next_pc);
    become function1180b8_asm_1180f2(cpu);
}

pub(crate) fn function1180b8_asm_1180f2(cpu: &mut Cpu) {
    cpu.pc = 0x40f2;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function1184ec(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    battle_tower_room_menu_write_message(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x45;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x5dd3);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x45;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x619d);
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
    cpu.a = cpu.read_byte(0xcf66);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xcd33);
    cpu.pc += 3;
    cpu.cycle(12);
    let value = cpu.read_byte(cpu.hl());
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 1;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(12);
        become function1180b8_asm_1180d1(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    battle_tower_room_menu_cleanup(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map::return_to_map_from_submenu(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function11811a(cpu: &mut Cpu) {
    cpu.pc = 0x411a;
    cpu.a = 1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd38, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(12);
    become function118125(cpu);
}

pub(crate) fn _battle_tower_room_menu(cpu: &mut Cpu) {
    cpu.pc = 0x4121;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcd38, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    become function118125(cpu);
}

pub(crate) fn function118125(cpu: &mut Cpu) {
    cpu.pc = 0x4125;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    battle_tower_room_menu_init_ram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x3;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd33, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0xd;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd34, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x4;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc3f0, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(crate::defs::R_WBK as u16);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = 0x3;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    become function118125_loop(cpu);
}

pub(crate) fn function118125_loop(cpu: &mut Cpu) {
    cpu.pc = 0x413e;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::joypad::joy_text_delay(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function118473(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xcf66);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0xf;
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
        become function118125_skip(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xcd34);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xcf66, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    become function118125_skip(cpu);
}

pub(crate) fn function118125_skip(cpu: &mut Cpu) {
    cpu.pc = 0x4151;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    battle_tower_room_menu_jumptable(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    battle_tower_room_menu_write_message(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x45;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x5dd3);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x45;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x619d);
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
    cpu.a = cpu.read_byte(0xcf66);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xcd33);
    cpu.pc += 3;
    cpu.cycle(12);
    let value = cpu.read_byte(cpu.hl());
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 1;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(12);
        become function118125_loop(cpu);
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
    cpu.write_byte(0xd000, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    battle_tower_room_menu_cleanup(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function118180(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map::return_to_map_from_submenu(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function118180(cpu: &mut Cpu) {
    cpu.pc = 0x4180;
    cpu.a = cpu.read_byte(0xc2dd);
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
    cpu.a = cpu.read_byte(0xcd38);
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
    cpu.a = 0x05;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xcd69);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xa89c);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(22 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(crate::defs::R_WBK as u16);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = 0x03;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.set_de(0xd202);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.c = 0x96;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x5f;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x50b3);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.cycle(12);
        become function118180_return_d3(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_de(0xd202);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(0x0196);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0x5f;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x51e1);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.cycle(12);
        become function118180_return_d3(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xd202);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xa8b2);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(150 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    become function118180_reset_banks(cpu);
}

pub(crate) fn function118180_reset_banks(cpu: &mut Cpu) {
    cpu.pc = 0x41c9;
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::close_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function118180_return_d3(cpu: &mut Cpu) {
    cpu.pc = 0x41d0;
    cpu.a = 0xd3;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc300, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xc2dd, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(12);
    become function118180_reset_banks(cpu);
}

pub(crate) fn function1181da(cpu: &mut Cpu) {
    cpu.pc = 0x41da;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    battle_tower_room_menu_init_ram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x2;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd38, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x21;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd33, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x22;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd34, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x4;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc3f0, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(crate::defs::R_WBK as u16);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = 0x3;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    become function1181da_asm_1181f8(cpu);
}

pub(crate) fn function1181da_asm_1181f8(cpu: &mut Cpu) {
    cpu.pc = 0x41f8;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::joypad::joy_text_delay(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function118473(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xcf66);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x28;
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
        become function1181da_asm_11820b(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xcd34);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xcf66, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    become function1181da_asm_11820b(cpu);
}

pub(crate) fn function1181da_asm_11820b(cpu: &mut Cpu) {
    cpu.pc = 0x420b;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function1185c3(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    battle_tower_room_menu_write_message(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x45;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x5dd3);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x45;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x619d);
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
    cpu.a = cpu.read_byte(0xcf66);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xcd33);
    cpu.pc += 3;
    cpu.cycle(12);
    let value = cpu.read_byte(cpu.hl());
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 1;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(12);
        become function1181da_asm_1181f8(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    battle_tower_room_menu_cleanup(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map::return_to_map_from_submenu(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function118233(cpu: &mut Cpu) {
    cpu.pc = 0x4233;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    battle_tower_room_menu_init_ram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x1b;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd33, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x1c;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd34, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x6;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc3f0, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(crate::defs::R_WBK as u16);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = 0x3;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    become function118233_asm_11824c(cpu);
}

pub(crate) fn function118233_asm_11824c(cpu: &mut Cpu) {
    cpu.pc = 0x424c;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::joypad::joy_text_delay(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function118473(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xcf66);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x1e;
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
        become function118233_asm_11825f(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xcd34);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xcf66, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    become function118233_asm_11825f(cpu);
}

pub(crate) fn function118233_asm_11825f(cpu: &mut Cpu) {
    cpu.pc = 0x425f;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function118624(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    battle_tower_room_menu_write_message(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x45;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x5dd3);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x45;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x619d);
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
    cpu.a = cpu.read_byte(0xcf66);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xcd33);
    cpu.pc += 3;
    cpu.cycle(12);
    let value = cpu.read_byte(cpu.hl());
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 1;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(12);
        become function118233_asm_11824c(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    battle_tower_room_menu_cleanup(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function118284(cpu: &mut Cpu) {
    cpu.pc = 0x4284;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    battle_tower_room_menu_init_ram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x19;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd33, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x1e;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd34, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x5;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc3f0, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(crate::defs::R_WBK as u16);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = 0x3;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    become function118284_asm_11829d(cpu);
}

pub(crate) fn function118284_asm_11829d(cpu: &mut Cpu) {
    cpu.pc = 0x429d;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::joypad::joy_text_delay(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function118473(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xcf66);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x20;
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
        become function118284_asm_1182b0(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xcd34);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xcf66, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    become function118284_asm_1182b0(cpu);
}

pub(crate) fn function118284_asm_1182b0(cpu: &mut Cpu) {
    cpu.pc = 0x42b0;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function1186f5(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    battle_tower_room_menu_write_message(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x45;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x5dd3);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x45;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x619d);
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
    cpu.a = cpu.read_byte(0xcf66);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xcd33);
    cpu.pc += 3;
    cpu.cycle(12);
    let value = cpu.read_byte(cpu.hl());
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 1;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(12);
        become function118284_asm_11829d(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    battle_tower_room_menu_cleanup(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function1182d5(cpu: &mut Cpu) {
    cpu.pc = 0x42d5;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    battle_tower_room_menu_init_ram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x18;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd33, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x19;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd34, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x4;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc3f0, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(crate::defs::R_WBK as u16);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = 0x3;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    become function1182d5_asm_1182ee(cpu);
}

pub(crate) fn function1182d5_asm_1182ee(cpu: &mut Cpu) {
    cpu.pc = 0x42ee;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::joypad::joy_text_delay(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function118473(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xcf66);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x1b;
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
        become function1182d5_asm_118301(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xcd34);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xcf66, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    become function1182d5_asm_118301(cpu);
}

pub(crate) fn function1182d5_asm_118301(cpu: &mut Cpu) {
    cpu.pc = 0x4301;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function118746(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    battle_tower_room_menu_write_message(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x45;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x5dd3);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x45;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x619d);
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
    cpu.a = cpu.read_byte(0xcf66);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xcd33);
    cpu.pc += 3;
    cpu.cycle(12);
    let value = cpu.read_byte(cpu.hl());
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 1;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(12);
        become function1182d5_asm_1182ee(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    battle_tower_room_menu_cleanup(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map::return_to_map_from_submenu(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function118329(cpu: &mut Cpu) {
    cpu.pc = 0x4329;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    battle_tower_room_menu_init_ram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x15;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd33, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x16;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd34, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x6;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc3f0, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(crate::defs::R_WBK as u16);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = 0x3;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    become function118329_asm_118342(cpu);
}

pub(crate) fn function118329_asm_118342(cpu: &mut Cpu) {
    cpu.pc = 0x4342;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::joypad::joy_text_delay(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function118473(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xcf66);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x18;
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
        become function118329_asm_118355(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xcd34);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xcf66, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    become function118329_asm_118355(cpu);
}

pub(crate) fn function118329_asm_118355(cpu: &mut Cpu) {
    cpu.pc = 0x4355;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function118671(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    battle_tower_room_menu_write_message(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x45;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x5dd3);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x45;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x619d);
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
    cpu.a = cpu.read_byte(0xcf66);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xcd33);
    cpu.pc += 3;
    cpu.cycle(12);
    let value = cpu.read_byte(cpu.hl());
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 1;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(12);
        become function118329_asm_118342(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    battle_tower_room_menu_cleanup(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function11837a(cpu: &mut Cpu) {
    cpu.pc = 0x437a;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    battle_tower_room_menu_init_ram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x16;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd33, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x17;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd34, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x6;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc3f0, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(crate::defs::R_WBK as u16);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = 0x3;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    become function11837a_asm_118393(cpu);
}

pub(crate) fn function11837a_asm_118393(cpu: &mut Cpu) {
    cpu.pc = 0x4393;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::joypad::joy_text_delay(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function118473(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xcf66);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x19;
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
        become function11837a_asm_1183a6(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xcd34);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xcf66, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    become function11837a_asm_1183a6(cpu);
}

pub(crate) fn function11837a_asm_1183a6(cpu: &mut Cpu) {
    cpu.pc = 0x43a6;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function1186b2(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    battle_tower_room_menu_write_message(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x45;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x5dd3);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x45;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x619d);
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
    cpu.a = cpu.read_byte(0xcf66);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xcd33);
    cpu.pc += 3;
    cpu.cycle(12);
    let value = cpu.read_byte(cpu.hl());
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 1;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(12);
        become function11837a_asm_118393(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    battle_tower_room_menu_cleanup(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn battle_tower_room_menu_init_ram(cpu: &mut Cpu) {
    cpu.pc = 0x43cb;
    cpu.disable_interrupts_after_next_instruction();
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(crate::defs::R_IE as u16);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.write_byte(0xcd32, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::double_speed::double_speed(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(crate::defs::R_IF as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.write_byte(0xc300, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte({ const N: i32 = (0xc300 as i32) + (1 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte({ const N: i32 = (0xc300 as i32) + (2 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xcd80, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xcd65, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xcd66, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xcd67, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xcd68, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xc31a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xcd89, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xcd8a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xcd8b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xc3ec, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xc3ed, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xc3ee, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xc3ef, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xd0ed);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(0xcd7f, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((crate::defs::LAST_12_SPRITE_OAM_STRUCTS_RESERVED_F as u8) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value |= mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.a = { let value = ((((crate::defs::IE_SERIAL as i32) | (crate::defs::IE_TIMER as i32)) | (crate::defs::IE_STAT as i32)) | (crate::defs::IE_VBLANK as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_IE as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xffc9, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.write_byte(0xffe9, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.enable_interrupts_after_next_instruction();
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = 0x41;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x6462);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x41;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x6464);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x45;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x5d99);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x45;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x615a);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x05;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xbfff, cpu.a);
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

pub(crate) fn function118440(cpu: &mut Cpu) {
    cpu.pc = 0x4440;
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = 0x05;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xbfff);
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
    cpu.write_byte(0xbfff, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::close_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn battle_tower_room_menu_cleanup(cpu: &mut Cpu) {
    cpu.pc = 0x4452;
    cpu.disable_interrupts_after_next_instruction();
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xffc9, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.write_byte(0xffe9, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.write_byte(0xff9e, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::double_speed::normal_speed(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(crate::defs::R_IF as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xcd32);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(crate::defs::R_IE as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.enable_interrupts_after_next_instruction();
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xcd7f);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xd0ed, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xc300);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xc2dd, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function118473(cpu: &mut Cpu) {
    cpu.pc = 0x4473;
    cpu.a = cpu.read_byte(0xcd65);
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
    cpu.a = cpu.read_byte(0xcd66);
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
    cpu.write_byte(0xcd66, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 60;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcd66, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xcd67);
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
    cpu.write_byte(0xcd67, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 60;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xcd68);
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
    cpu.write_byte(0xcd68, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 99;
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
        become function118473_ninety_nine(cpu);
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
    cpu.write_byte(0xcd67, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function118473_ninety_nine(cpu: &mut Cpu) {
    cpu.pc = 0x44a0;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcd65, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function11878d(cpu: &mut Cpu) {
    cpu.pc = 0x478d;
    cpu.a = cpu.read_byte(0xc821);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((1) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(12);
        become function11878d_asm_1187af(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let mask = 1u8 << ((2) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(12);
        become function11878d_asm_1187d1(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let mask = 1u8 << ((0) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(12);
        become function11878d_asm_1187aa(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xcd89);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x1;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become function11878d_asm_1187a7(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x3;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    become function11878d_asm_1187a7(cpu);
}

pub(crate) fn function11878d_asm_1187a7(cpu: &mut Cpu) {
    cpu.pc = 0x47a7;
    cpu.cycle(16);
    become battle_tower_room_menu_increment_jumptable(cpu);
}

pub(crate) fn function11878d_asm_1187aa(cpu: &mut Cpu) {
    cpu.pc = 0x47aa;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function118821(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function11878d_asm_1187af(cpu: &mut Cpu) {
    cpu.pc = 0x47af;
    cpu.a = crate::defs::MOBILEAPI_00 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::mobile::mobile_api(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.write_byte(0xc300, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte({ const N: i32 = (0xc300 as i32) + (1 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte({ const N: i32 = (0xc300 as i32) + (2 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = crate::defs::MOBILEAPI_05 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::mobile::mobile_api(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xc3f0);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xc319, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xcd34);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xcf66, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function11878d_asm_1187d1(cpu: &mut Cpu) {
    cpu.pc = 0x47d1;
    cpu.set_hl(0xcd89);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((0) & 7);
    let value = cpu.read_byte(cpu.hl());
    cpu.flag_z = (value & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(12);
    if !cpu.flag_z {
        cpu.cycle(12);
        become function11878d_asm_118803(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let mask = 1u8 << ((0) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value |= mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.a = 0x06;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.set_de(0xd000);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(0x1000 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
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
    if cpu.flag_c {
        cpu.cycle(12);
        become function11878d_asm_1187f9(cpu);
    }
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
    if cpu.flag_c {
        cpu.cycle(12);
        become function11878d_asm_1187fd(cpu);
    }
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
    if cpu.flag_c {
        cpu.cycle(12);
        become function11878d_asm_1187f5(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = crate::defs::MOBILEAPI_12 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become function11878d_asm_1187ff(cpu);
}

pub(crate) fn function11878d_asm_1187f5(cpu: &mut Cpu) {
    cpu.pc = 0x47f5;
    cpu.a = crate::defs::MOBILEAPI_14 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become function11878d_asm_1187ff(cpu);
}

pub(crate) fn function11878d_asm_1187f9(cpu: &mut Cpu) {
    cpu.pc = 0x47f9;
    cpu.a = crate::defs::MOBILEAPI_15 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become function11878d_asm_1187ff(cpu);
}

pub(crate) fn function11878d_asm_1187fd(cpu: &mut Cpu) {
    cpu.pc = 0x47fd;
    cpu.a = crate::defs::MOBILEAPI_16 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    become function11878d_asm_1187ff(cpu);
}

pub(crate) fn function11878d_asm_1187ff(cpu: &mut Cpu) {
    cpu.pc = 0x47ff;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::mobile::mobile_api(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function11878d_asm_118803(cpu: &mut Cpu) {
    cpu.pc = 0x4803;
    cpu.a = 0xd3;
    cpu.pc += 2;
    cpu.cycle(8);
    become set_mobile_error_code(cpu);
}

pub(crate) fn set_mobile_error_code(cpu: &mut Cpu) {
    cpu.pc = 0x4805;
    cpu.write_byte(0xc300, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte({ const N: i32 = (0xc300 as i32) + (1 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte({ const N: i32 = (0xc300 as i32) + (2 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = crate::defs::MOBILEAPI_05 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::mobile::mobile_api(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xc3f0);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xc319, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xcd34);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xcf66, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function118821(cpu: &mut Cpu) {
    cpu.pc = 0x4821;
    cpu.a = cpu.read_byte(0xc319);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x3;
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
        become function118821_asm_11884a(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x4;
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
        become function118821_asm_11884a(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xffa8);
    cpu.pc += 2;
    cpu.cycle(12);
    let value = 0x5;
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
        become function118821_asm_11884a(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = crate::defs::MOBILEAPI_05 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::mobile::mobile_api(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0xa;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc300, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xc3f0);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xc319, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xcd34);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xcf66, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = true;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function118821_asm_11884a(cpu: &mut Cpu) {
    cpu.pc = 0x484a;
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

pub(crate) fn function11884c(cpu: &mut Cpu) {
    cpu.pc = 0x484c;
    cpu.a = cpu.read_byte(0xffa8);
    cpu.pc += 2;
    cpu.cycle(12);
    let value = 0x5;
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
        become function11884c_asm_118864(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = crate::defs::MOBILEAPI_05 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::mobile::mobile_api(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0xa;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc300, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xcd34);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xcf66, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = true;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function11884c_asm_118864(cpu: &mut Cpu) {
    cpu.pc = 0x4864;
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

pub(crate) fn function118866(cpu: &mut Cpu) {
    cpu.pc = 0x4866;
    cpu.a = 2;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become asm_11886f(cpu);
}

pub(crate) fn function11886a(cpu: &mut Cpu) {
    cpu.pc = 0x486a;
    cpu.a = 1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become asm_11886f(cpu);
}

pub(crate) fn function11886e(cpu: &mut Cpu) {
    cpu.pc = 0x486e;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    become asm_11886f(cpu);
}

pub(crate) fn asm_11886f(cpu: &mut Cpu) {
    cpu.pc = 0x486f;
    cpu.write_byte(0xcd48, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd3c, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    battle_tower_room_menu_increment_jumptable(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xcd33);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xcd45, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    become function118880(cpu);
}

pub(crate) fn function118880(cpu: &mut Cpu) {
    cpu.pc = 0x4880;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    battle_tower_room_menu2(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
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
    cpu.write_byte(0xc807, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_de(0xcd81);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0x46 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = crate::defs::MOBILEAPI_01 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become function119e2b(cpu);
}

pub(crate) fn function118896(cpu: &mut Cpu) {
    cpu.pc = 0x4896;
    cpu.a = cpu.read_byte(0xc821);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((1) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(12);
        become function118896_asm_1188a5(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let mask = 1u8 << ((2) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(12);
        become function118896_asm_1188a5(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let mask = 1u8 << ((0) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become function118896_asm_1188aa(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become function118896_asm_1188a5(cpu);
}

pub(crate) fn function118896_asm_1188a5(cpu: &mut Cpu) {
    cpu.pc = 0x48a5;
    cpu.a = crate::defs::MOBILEAPI_1A as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become function119e2b(cpu);
}

pub(crate) fn function118896_asm_1188aa(cpu: &mut Cpu) {
    cpu.pc = 0x48aa;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    battle_tower_room_menu_increment_jumptable(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(16);
    become battle_tower_room_menu_increment_jumptable(cpu);
}

pub(crate) fn function1188b0(cpu: &mut Cpu) {
    cpu.pc = 0x48b0;
    cpu.set_de(0xc346);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = crate::defs::MOBILEAPI_06 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become function119e2b(cpu);
}

pub(crate) fn function1188b8(cpu: &mut Cpu) {
    cpu.pc = 0x48b8;
    cpu.set_de(0xc3ac);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = crate::defs::MOBILEAPI_07 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become function119e2b(cpu);
}

pub(crate) fn function1188c0(cpu: &mut Cpu) {
    cpu.pc = 0x48c0;
    cpu.set_de(0xc3cd);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = crate::defs::MOBILEAPI_08 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become function119e2b(cpu);
}

pub(crate) fn function1188c8(cpu: &mut Cpu) {
    cpu.pc = 0x48c8;
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd65, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function1188e7(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xc708);
    cpu.pc += 3;
    cpu.cycle(12);
    become function1188c8_asm_1188d3(cpu);
}

pub(crate) fn function1188c8_asm_1188d3(cpu: &mut Cpu) {
    cpu.pc = 0x48d3;
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
        become function1188c8_asm_1188d3(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function119eb4(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function119ec2(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xc708);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = crate::defs::MOBILEAPI_03 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become function119e2b(cpu);
}

pub(crate) fn function1188e7(cpu: &mut Cpu) {
    cpu.pc = 0x48e7;
    cpu.set_de(0xc346);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0x05;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xaa4a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::close_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
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
    let value = cpu.a;
    let result = value << 1;
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x80 != 0;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    become function1188e7_asm_1188fa(cpu);
}

pub(crate) fn function1188e7_asm_1188fa(cpu: &mut Cpu) {
    cpu.pc = 0x48fa;
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
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
        become function1188e7_asm_1188fa(cpu);
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
        become function1188e7_asm_1188fa(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function118903(cpu: &mut Cpu) {
    cpu.pc = 0x4903;
    cpu.a = cpu.read_byte(0xc3f0);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xc319, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.c = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x45;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x5e18);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd3c, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    battle_tower_room_menu2(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(16);
    become battle_tower_room_menu_increment_jumptable(cpu);
}

pub(crate) fn function11891c(cpu: &mut Cpu) {
    cpu.pc = 0x491c;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function118b42(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(16);
    become battle_tower_room_menu_increment_jumptable(cpu);
}

pub(crate) fn battle_tower_room_menu_pick_level_message(cpu: &mut Cpu) {
    cpu.pc = 0x4922;
    cpu.a = cpu.read_byte(0xcd38);
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
        become battle_tower_room_menu_pick_level_message_asm_11892d(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x6ba5);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(12);
    become battle_tower_room_menu_pick_level_message_asm_118930(cpu);
}

pub(crate) fn battle_tower_room_menu_pick_level_message_asm_11892d(cpu: &mut Cpu) {
    cpu.pc = 0x492d;
    cpu.set_hl(0x6bcb);
    cpu.pc += 3;
    cpu.cycle(12);
    become battle_tower_room_menu_pick_level_message_asm_118930(cpu);
}

pub(crate) fn battle_tower_room_menu_pick_level_message_asm_118930(cpu: &mut Cpu) {
    cpu.pc = 0x4930;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    battle_tower_room_menu_set_message(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    battle_tower_room_menu_increment_jumptable(cpu);
    assert_eq!(cpu.pc, next_pc);
    become battle_tower_room_menu_place_pick_level_menu(cpu);
}

pub(crate) fn battle_tower_room_menu_place_pick_level_menu(cpu: &mut Cpu) {
    cpu.pc = 0x4936;
    cpu.a = cpu.read_byte(0xc31a);
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
    cpu.set_hl(0x5cf7);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::menu::load_menu_header(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::menu::menu_box(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::menu::menu_box_coord2_tile(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::tilemap::apply_tilemap(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = ((((8 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (16 as i32)) + (0xcdd9 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0x40;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.read_byte(cpu.hl());
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::tilemap::wait_bg_map2(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd4f, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
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
    if !cpu.flag_z {
        cpu.cycle(12);
        become battle_tower_room_menu_place_pick_level_menu_asm_11896b(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x5d64);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 5;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become battle_tower_room_menu_place_pick_level_menu_asm_118970(cpu);
}

pub(crate) fn battle_tower_room_menu_place_pick_level_menu_asm_11896b(cpu: &mut Cpu) {
    cpu.pc = 0x496b;
    cpu.set_hl(0x5d0c);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 11;
    cpu.pc += 2;
    cpu.cycle(8);
    become battle_tower_room_menu_place_pick_level_menu_asm_118970(cpu);
}

pub(crate) fn battle_tower_room_menu_place_pick_level_menu_asm_118970(cpu: &mut Cpu) {
    cpu.pc = 0x4970;
    cpu.write_byte(0xcd4a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcd4b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcd4c, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x3;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    battle_tower_room_menu_increment_jumptable(cpu);
    assert_eq!(cpu.pc, next_pc);
    become battle_tower_room_menu_update_pick_level_menu(cpu);
}

pub(crate) fn battle_tower_room_menu_update_pick_level_menu(cpu: &mut Cpu) {
    cpu.pc = 0x4982;
    cpu.set_hl({ const N: i32 = ((((8 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (13 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0x5d07);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::place_string(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = ((((10 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (13 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0x5d07);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::place_string(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xcd4b);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xcd4c);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.h = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.d = 0x0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xcd4f);
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
    let value = cpu.a;
    let result = value.rotate_left(1);
    cpu.flag_z = false;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x80 != 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
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
    cpu.a = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.d = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(crate::defs::R_WBK as u16);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.set_bc(0xd099);
    cpu.pc += 3;
    cpu.cycle(12);
    become battle_tower_room_menu_update_pick_level_menu_asm_1189b5(cpu);
}

pub(crate) fn battle_tower_room_menu_update_pick_level_menu_asm_1189b5(cpu: &mut Cpu) {
    cpu.pc = 0x49b5;
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0x50;
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
        become battle_tower_room_menu_update_pick_level_menu_asm_1189c4(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x7f;
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
        become battle_tower_room_menu_update_pick_level_menu_asm_1189c2(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.bc(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_bc(cpu.bc().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.cycle(12);
    become battle_tower_room_menu_update_pick_level_menu_asm_1189b5(cpu);
}

pub(crate) fn battle_tower_room_menu_update_pick_level_menu_asm_1189c2(cpu: &mut Cpu) {
    cpu.pc = 0x49c2;
    cpu.a = 0x50;
    cpu.pc += 2;
    cpu.cycle(8);
    become battle_tower_room_menu_update_pick_level_menu_asm_1189c4(cpu);
}

pub(crate) fn battle_tower_room_menu_update_pick_level_menu_asm_1189c4(cpu: &mut Cpu) {
    cpu.pc = 0x49c4;
    cpu.write_byte(cpu.bc(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.set_hl({ const N: i32 = ((((9 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (13 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::place_string(cpu);
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
        become battle_tower_room_menu_update_pick_level_menu_b_button(cpu);
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
        become battle_tower_room_menu_update_pick_level_menu_a_button(cpu);
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
        become battle_tower_room_menu_update_pick_level_menu_d_down(cpu);
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
        become battle_tower_room_menu_update_pick_level_menu_d_up(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become battle_tower_room_menu_update_pick_level_menu_asm_1189e5(cpu);
}

pub(crate) fn battle_tower_room_menu_update_pick_level_menu_asm_1189e5(cpu: &mut Cpu) {
    cpu.pc = 0x49e5;
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn battle_tower_room_menu_update_pick_level_menu_d_down(cpu: &mut Cpu) {
    cpu.pc = 0x49e6;
    cpu.set_hl(0xcd4f);
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
        cpu.cycle(12);
        become battle_tower_room_menu_update_pick_level_menu_asm_1189e5(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xcd4a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.cycle(12);
    become battle_tower_room_menu_update_pick_level_menu_asm_1189e5(cpu);
}

pub(crate) fn battle_tower_room_menu_update_pick_level_menu_d_up(cpu: &mut Cpu) {
    cpu.pc = 0x49f2;
    cpu.a = cpu.read_byte(0xcd4a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xcd4f);
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
    let value = cpu.read_byte(cpu.hl());
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 1;
    cpu.cycle(8);
    if !cpu.flag_c {
        cpu.cycle(12);
        become battle_tower_room_menu_update_pick_level_menu_asm_1189e5(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.cycle(12);
    become battle_tower_room_menu_update_pick_level_menu_asm_1189e5(cpu);
}

pub(crate) fn battle_tower_room_menu_update_pick_level_menu_a_button(cpu: &mut Cpu) {
    cpu.pc = 0x4a01;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::menu::play_click_sfx(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xcd4f);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xcd4a);
    cpu.pc += 3;
    cpu.cycle(12);
    let value = cpu.read_byte(cpu.hl());
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 1;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become battle_tower_room_menu_update_pick_level_menu_asm_118a3c(cpu);
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
    let value = 0xfe;
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
    cpu.write_byte(0xcf65, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(crate::defs::R_WBK as u16);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::menu::close_window(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xcd38);
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
        become battle_tower_room_menu_update_pick_level_menu_asm_118a30(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    battle_tower_level_check(cpu);
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
    battle_tower_ubers_check(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    become battle_tower_room_menu_update_pick_level_menu_asm_118a30(cpu);
}

pub(crate) fn battle_tower_room_menu_update_pick_level_menu_asm_118a30(cpu: &mut Cpu) {
    cpu.pc = 0x4a30;
    cpu.a = cpu.read_byte(0xcd4f);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xd800, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(16);
    become battle_tower_room_menu_increment_jumptable(cpu);
}

pub(crate) fn battle_tower_room_menu_update_pick_level_menu_b_button(cpu: &mut Cpu) {
    cpu.pc = 0x4a39;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::menu::play_click_sfx(cpu);
    assert_eq!(cpu.pc, next_pc);
    become battle_tower_room_menu_update_pick_level_menu_asm_118a3c(cpu);
}

pub(crate) fn battle_tower_room_menu_update_pick_level_menu_asm_118a3c(cpu: &mut Cpu) {
    cpu.pc = 0x4a3c;
    cpu.a = cpu.read_byte(crate::defs::R_WBK as u16);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::menu::close_window(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = 0x7;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcf66, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd46, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function118a54(cpu: &mut Cpu) {
    cpu.pc = 0x4a54;
    cpu.a = cpu.read_byte(0xcd55);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xcd56);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.h = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_de(0xc3ec);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(0x0004 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become function118ae4(cpu);
}

pub(crate) fn function118a65(cpu: &mut Cpu) {
    cpu.pc = 0x4a65;
    cpu.set_hl(0x4bf7);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xcc60);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(0x80 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_de(0xd000);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(0x1000 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become function118b10(cpu);
}

pub(crate) fn set_battle_download_url(cpu: &mut Cpu) {
    cpu.pc = 0x4a7a;
    cpu.set_hl(0x4bf7);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xcc60);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(0x80 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_de(0xd000);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(0x1000 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become function118b10(cpu);
}

pub(crate) fn set_exchange_download_url(cpu: &mut Cpu) {
    cpu.pc = 0x4a8f;
    cpu.set_hl(0x4ba5);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xcc60);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(0x80 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_de(0xd000);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(0x1000 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become function118b10(cpu);
}

pub(crate) fn set_news_download_url(cpu: &mut Cpu) {
    cpu.pc = 0x4aa4;
    cpu.set_hl(0x4c47);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xcc60);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(0x80 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x5;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.set_de(0xd100);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(0xe00 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(12);
    become function118b10(cpu);
}

pub(crate) fn set_stadium_download_url(cpu: &mut Cpu) {
    cpu.pc = 0x4abc;
    cpu.set_hl(0x4c95);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xcc60);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(0x80 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_de(0xd000);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(0x1000 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(12);
    become function118b10(cpu);
}

pub(crate) fn set_odd_egg_download_url(cpu: &mut Cpu) {
    cpu.pc = 0x4ad0;
    cpu.set_hl(0x4ce5);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xcc60);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(0x80 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_de(0xd000);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(0x1000 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(12);
    become function118b10(cpu);
}

pub(crate) fn function118ae4(cpu: &mut Cpu) {
    cpu.pc = 0x4ae4;
    cpu.push_word(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.push_word(cpu.de());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = 0x8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd3c, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    battle_tower_room_menu2(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.c = 0x0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_de(0xcc60);
    cpu.pc += 3;
    cpu.cycle(12);
    become function118ae4_asm_118af5(cpu);
}

pub(crate) fn function118ae4_asm_118af5(cpu: &mut Cpu) {
    cpu.pc = 0x4af5;
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
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
        become function118ae4_asm_118b06(cpu);
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
    let value = 0xa6;
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
        become function118ae4_asm_118af5(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0xda;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become set_mobile_error_code(cpu);
}

pub(crate) fn function118ae4_asm_118b06(cpu: &mut Cpu) {
    cpu.pc = 0x4b06;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function118b24(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_de(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let value = cpu.pop_word();
    cpu.set_bc(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.a = crate::defs::MOBILEAPI_15 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become function119e2b(cpu);
}

pub(crate) fn function118b10(cpu: &mut Cpu) {
    cpu.pc = 0x4b10;
    cpu.push_word(cpu.de());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.push_word(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = 0x8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd3c, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    battle_tower_room_menu2(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function118b24(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_bc(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let value = cpu.pop_word();
    cpu.set_de(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.a = crate::defs::MOBILEAPI_15 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become function119e2b(cpu);
}

pub(crate) fn function118b24(cpu: &mut Cpu) {
    cpu.pc = 0x4b24;
    cpu.set_hl(0xc346);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = ((0xc708u32) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = (((0xc708u32) >> 8) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = ((0xcc60u32) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = (((0xcc60u32) >> 8) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function119eb4(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function119ec2(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x80;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd89, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xc346);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function118b42(cpu: &mut Cpu) {
    cpu.pc = 0x4b42;
    cpu.set_hl(0xd002);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcd51, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcd52, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function118b8c(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcd55, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xcd59, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcd56, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xcd5a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function118b8c(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcd53, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xcd5d, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcd54, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xcd5e, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function118b8c(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcd57, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xcd5b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcd58, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xcd5c, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function118b8c(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcd5f, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcd60, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function118b8c(cpu: &mut Cpu) {
    cpu.pc = 0x4b8c;
    become function118b8c_asm_118b8c(cpu);
}

pub(crate) fn function118b8c_asm_118b8c(cpu: &mut Cpu) {
    cpu.pc = 0x4b8c;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function118b9a(cpu);
    assert_eq!(cpu.pc, next_pc);
    if !cpu.flag_c {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0xd;
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
        become function118b8c_asm_118b8c(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_sub(1));
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
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function118b9a(cpu: &mut Cpu) {
    cpu.pc = 0x4b9a;
    cpu.a = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = 0xe0;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_c {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 0xd3;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    set_mobile_error_code(cpu);
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

pub(crate) fn function118d35(cpu: &mut Cpu) {
    cpu.pc = 0x4d35;
    cpu.set_hl(0xd200 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xcd38);
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
        become function118d35_asm_118d6e(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0x94;
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
        become function118d35_asm_118d7b(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0x5;
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
        become function118d35_asm_118d7b(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xcd4f);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.a;
    let result = value << 1;
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x80 != 0;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
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
    cpu.a = 0x05;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xb2fb, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::close_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x5c;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x4be4);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x5c;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x40c4);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(12);
    become function118d35_asm_118d78(cpu);
}

pub(crate) fn function118d35_asm_118d6e(cpu: &mut Cpu) {
    cpu.pc = 0x4d6e;
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0x96;
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
        become function118d35_asm_118d7b(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0x0;
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
        become function118d35_asm_118d7b(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become function118d35_asm_118d78(cpu);
}

pub(crate) fn function118d35_asm_118d78(cpu: &mut Cpu) {
    cpu.pc = 0x4d78;
    cpu.cycle(16);
    become battle_tower_room_menu_increment_jumptable(cpu);
}

pub(crate) fn function118d35_asm_118d7b(cpu: &mut Cpu) {
    cpu.pc = 0x4d7b;
    cpu.a = 0xd3;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become set_mobile_error_code(cpu);
}

pub(crate) fn function118d80(cpu: &mut Cpu) {
    cpu.pc = 0x4d80;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function118e06(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xcd38);
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
        become function118d80_asm_118d8e(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    battle_tower_room_menu_increment_jumptable(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(12);
    become asm_118d9f(cpu);
}

pub(crate) fn function118d80_asm_118d8e(cpu: &mut Cpu) {
    cpu.pc = 0x4d8e;
    cpu.a = 0x9;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd3c, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x12;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd45, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    battle_tower_room_menu_increment_jumptable(cpu);
    assert_eq!(cpu.pc, next_pc);
    become function118d9b(cpu);
}

pub(crate) fn function118d9b(cpu: &mut Cpu) {
    cpu.pc = 0x4d9b;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    battle_tower_room_menu2(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    become asm_118d9f(cpu);
}

pub(crate) fn asm_118d9f(cpu: &mut Cpu) {
    cpu.pc = 0x4d9f;
    cpu.set_hl(0xc608);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function119940(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xcd38);
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
        become asm_118d9f_asm_118db2(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = crate::defs::TRADE_CORNER_REQUEST_LENGTH as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd3b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(12);
    become asm_118d9f_asm_118db7(cpu);
}

pub(crate) fn asm_118d9f_asm_118db2(cpu: &mut Cpu) {
    cpu.pc = 0x4db2;
    cpu.a = 0x26;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd3b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    become asm_118d9f_asm_118db7(cpu);
}

pub(crate) fn asm_118d9f_asm_118db7(cpu: &mut Cpu) {
    cpu.pc = 0x4db7;
    cpu.set_hl(0xd800);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = ((0xc608u32) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = (((0xc608u32) >> 8) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xcd3b);
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
    cpu.a = ((0xc708u32) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = (((0xc708u32) >> 8) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xcd39);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xcd3a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function119eb4(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function119ec2(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x40;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd89, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xd800);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xde00);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(0x200 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = crate::defs::MOBILEAPI_16 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become function119e2b(cpu);
}

pub(crate) fn function118ded(cpu: &mut Cpu) {
    cpu.pc = 0x4ded;
    cpu.a = cpu.read_byte(0xcd38);
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
        become function118ded_asm_118e03(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(crate::defs::R_WBK as u16);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = 0x46;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x793b);
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
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    become function118ded_asm_118e03(cpu);
}

pub(crate) fn function118ded_asm_118e03(cpu: &mut Cpu) {
    cpu.pc = 0x4e03;
    cpu.cycle(16);
    become battle_tower_room_menu_increment_jumptable(cpu);
}

pub(crate) fn function118e06(cpu: &mut Cpu) {
    cpu.pc = 0x4e06;
    cpu.set_hl(0xd002);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xcd38);
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
        become function118e06_asm_118e1d(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become function118e06_asm_118e0f(cpu);
}

pub(crate) fn function118e06_asm_118e0f(cpu: &mut Cpu) {
    cpu.pc = 0x4e0f;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function118b9a(cpu);
    assert_eq!(cpu.pc, next_pc);
    if !cpu.flag_c {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0xd;
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
        become function118e06_asm_118e0f(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0xa;
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
        become function118e06_asm_118e0f(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become function118e06_asm_118e1d(cpu);
}

pub(crate) fn function118e06_asm_118e1d(cpu: &mut Cpu) {
    cpu.pc = 0x4e1d;
    cpu.a = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcd39, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcd3a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    become function118e06_asm_118e25(cpu);
}

pub(crate) fn function118e06_asm_118e25(cpu: &mut Cpu) {
    cpu.pc = 0x4e25;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function118b9a(cpu);
    assert_eq!(cpu.pc, next_pc);
    if !cpu.flag_c {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0xd;
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
        become function118e06_asm_118e25(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0xa;
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
        become function118e06_asm_118e25(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_sub(1));
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
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.cycle(12);
    become asm_118e3e(cpu);
}

pub(crate) fn function118e39(cpu: &mut Cpu) {
    cpu.pc = 0x4e39;
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
        become function118e39(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    become asm_118e3e(cpu);
}

pub(crate) fn asm_118e3e(cpu: &mut Cpu) {
    cpu.pc = 0x4e3e;
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0x2f;
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
        become asm_118e3e(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(0xcd85);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.c = 0x4;
    cpu.pc += 2;
    cpu.cycle(8);
    become asm_118e3e_asm_118e4a(cpu);
}

pub(crate) fn asm_118e3e_asm_118e4a(cpu: &mut Cpu) {
    cpu.pc = 0x4e4a;
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0x2e;
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
        become asm_118e3e_asm_118e63(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x30;
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
        become asm_118e3e_asm_118e67(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x3a;
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
        become asm_118e3e_asm_118e67(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x30;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0xf6;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_add(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) > 0x0f;
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
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
        become asm_118e3e_asm_118e4a(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_de(0xcd85);
    cpu.pc += 3;
    cpu.cycle(12);
    become asm_118e3e_asm_118e63(cpu);
}

pub(crate) fn asm_118e3e_asm_118e63(cpu: &mut Cpu) {
    cpu.pc = 0x4e63;
    cpu.a = 0x50;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn asm_118e3e_asm_118e67(cpu: &mut Cpu) {
    cpu.pc = 0x4e67;
    cpu.a = 0xf3;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.cycle(12);
    become asm_118e3e_asm_118e63(cpu);
}

pub(crate) fn function118e6d(cpu: &mut Cpu) {
    cpu.pc = 0x4e6d;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcd65, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = crate::defs::MOBILEAPI_05 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become function119e2b(cpu);
}

pub(crate) fn function118e76(cpu: &mut Cpu) {
    cpu.pc = 0x4e76;
    cpu.a = 0xc;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd3c, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(16);
    become battle_tower_room_menu_increment_jumptable(cpu);
}

pub(crate) fn function118e7e(cpu: &mut Cpu) {
    cpu.pc = 0x4e7e;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    battle_tower_room_menu2(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = crate::defs::MOBILEAPI_1B as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become function119e2b(cpu);
}

pub(crate) fn battle_tower_room_menu_call_room_menu2(cpu: &mut Cpu) {
    cpu.pc = 0x4e87;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    battle_tower_room_menu2(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xcd33);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xcf66, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function118e92(cpu: &mut Cpu) {
    cpu.pc = 0x4e92;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function118440(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function1191d3(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xcd53);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xcd54);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.h = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_de(0xcc60);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function1191ad(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(0xd800);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(0x0800 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become function118b10(cpu);
}

pub(crate) fn function118eb0(cpu: &mut Cpu) {
    cpu.pc = 0x4eb0;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function118440(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xd802);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xcd20);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(0x000c as u16);
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
    function1192cc(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.cycle(16);
    become battle_tower_room_menu_increment_jumptable(cpu);
}

pub(crate) fn function118ec6(cpu: &mut Cpu) {
    cpu.pc = 0x4ec6;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function118440(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::speech_textbox(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xd80e);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xc320);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(0x0026 as u16);
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
    cpu.write_byte(0xc31f, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = ((0xc320u32) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc31b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = (((0xc320u32) >> 8) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc31c, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl({ const N: i32 = ((((14 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (1 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc31d, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc31e, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x2;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc31a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x1d;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd3c, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x24;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd45, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x11;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd46, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x1c;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd47, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(16);
    become battle_tower_room_menu_increment_jumptable(cpu);
}

pub(crate) fn function118f0d(cpu: &mut Cpu) {
    cpu.pc = 0x4f0d;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    battle_tower_room_menu2(cpu);
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
    function118440(cpu);
    assert_eq!(cpu.pc, next_pc);
    become function118f14(cpu);
}

pub(crate) fn function118f14(cpu: &mut Cpu) {
    cpu.pc = 0x4f14;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function118440(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xcd51);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xcd52);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.h = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_de(0xcc60);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function1191ad(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xcc60);
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
        become function118f14_dont_send_save_file(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x6a13);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    battle_tower_room_menu_set_message(cpu);
    assert_eq!(cpu.pc, next_pc);
    become function118f14_dont_send_save_file(cpu);
}

pub(crate) fn function118f14_dont_send_save_file(cpu: &mut Cpu) {
    cpu.pc = 0x4f32;
    cpu.a = cpu.read_byte(0xcd57);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xcd58);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.h = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_de(0xcc60);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function1191ad(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(0xcc60);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function118e39(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x9;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd3c, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x24;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd45, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x13;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd46, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x1c;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd47, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(16);
    become battle_tower_room_menu_increment_jumptable(cpu);
}

pub(crate) fn function118f5e(cpu: &mut Cpu) {
    cpu.pc = 0x4f5e;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    battle_tower_room_menu2(cpu);
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
    function118440(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::delay::delay_frame(cpu);
    assert_eq!(cpu.pc, next_pc);
    become function118f68(cpu);
}

pub(crate) fn function118f68(cpu: &mut Cpu) {
    cpu.pc = 0x4f68;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function119223(cpu);
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
    function118440(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xcd51);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xcd52);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.h = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_de(0xcc60);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function1191ad(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xcc60);
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
        become function118f68_asm_118fba(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xc346);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = ((0xc608u32) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = (((0xc608u32) >> 8) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xcd4b);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xcd4c);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = ((0xc708u32) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = (((0xc708u32) >> 8) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = ((0xcc60u32) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = (((0xcc60u32) >> 8) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function119eb4(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function119ec2(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x40;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd89, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xc346);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xde00);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(0x200 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = crate::defs::MOBILEAPI_16 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become function119e2b(cpu);
}

pub(crate) fn function118f68_asm_118fba(cpu: &mut Cpu) {
    cpu.pc = 0x4fba;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    battle_tower_room_menu_increment_jumptable(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(16);
    become battle_tower_room_menu_increment_jumptable(cpu);
}

pub(crate) fn function118fc0(cpu: &mut Cpu) {
    cpu.pc = 0x4fc0;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function118440(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xcd55);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xcd56);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.h = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_de(0xcc60);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function1191ad(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xcc60);
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
        become function118fc0_asm_118ffa(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xcd51);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xcd52);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.h = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_de(0xcc60);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function1191ad(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xcc60);
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
        become function118fc0_asm_118ff2(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x6a2c);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(12);
    become function118fc0_asm_118ff5(cpu);
}

pub(crate) fn function118fc0_asm_118ff2(cpu: &mut Cpu) {
    cpu.pc = 0x4ff2;
    cpu.set_hl(0x6a4b);
    cpu.pc += 3;
    cpu.cycle(12);
    become function118fc0_asm_118ff5(cpu);
}

pub(crate) fn function118fc0_asm_118ff5(cpu: &mut Cpu) {
    cpu.pc = 0x4ff5;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    battle_tower_room_menu_set_message(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(12);
    become function119009(cpu);
}

pub(crate) fn function118fc0_asm_118ffa(cpu: &mut Cpu) {
    cpu.pc = 0x4ffa;
    cpu.set_hl(0x6a4b);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    battle_tower_room_menu_set_message(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    battle_tower_room_menu_increment_jumptable(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    battle_tower_room_menu_increment_jumptable(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(16);
    become battle_tower_room_menu_increment_jumptable(cpu);
}

pub(crate) fn function119009(cpu: &mut Cpu) {
    cpu.pc = 0x5009;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function118440(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function119300(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xcd55);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xcd56);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.h = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_de(0xcc60);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function1191ad(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(0xc346);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = ((0xc608u32) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = (((0xc608u32) >> 8) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xcd4b);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xcd4c);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = ((0xc708u32) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = (((0xc708u32) >> 8) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = ((0xcc60u32) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = (((0xcc60u32) >> 8) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function119eb4(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function119ec2(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x40;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd89, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xc346);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xd000);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(0x1000 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = crate::defs::MOBILEAPI_16 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become function119e2b(cpu);
}

pub(crate) fn function119054(cpu: &mut Cpu) {
    cpu.pc = 0x5054;
    cpu.a = 0x6;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xd002);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xcd4f);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xcd50);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.d = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xd000);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte({ const N: i32 = (0xd000 as i32) + (1 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function119192(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xcd89);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x1;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become function119054_asm_11908a(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x6;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.set_hl(0xd002);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xd000);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte({ const N: i32 = (0xd000 as i32) + (1 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function119192(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    become function119054_asm_11908a(cpu);
}

pub(crate) fn function119054_asm_11908a(cpu: &mut Cpu) {
    cpu.pc = 0x508a;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::close_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x3;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = 0x05;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xcd4f);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xb1b3, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xcd50);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xb1b4, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xcd20);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xaa7f);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(12 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(crate::defs::R_WBK as u16);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = 0x01;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xd474);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xb2f3, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xd475);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xb2f4);
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
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::close_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(16);
    become battle_tower_room_menu_increment_jumptable(cpu);
}

pub(crate) fn function1190d0(cpu: &mut Cpu) {
    cpu.pc = 0x50d0;
    cpu.a = 0x03;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xcd57);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xcd58);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.h = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_de(0xcc60);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function1191ad(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(0xd000);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(0x1000 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become function118b10(cpu);
}

pub(crate) fn function1190ec(cpu: &mut Cpu) {
    cpu.pc = 0x50ec;
    cpu.a = 0x05;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xcd20);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xaa73);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(12 as u16);
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
    crate::game::home::sram::close_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x05;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xaa72, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::close_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x06;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xd000);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte({ const N: i32 = (0xd000 as i32) + (1 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl(0xd002);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xa000);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function119192(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xcd89);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x1;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become function1190ec_asm_11913e(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x06;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xd000);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte({ const N: i32 = (0xd000 as i32) + (1 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl({ const N: i32 = (0xd000 as i32) + (2 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function119192(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    become function1190ec_asm_11913e(cpu);
}

pub(crate) fn function1190ec_asm_11913e(cpu: &mut Cpu) {
    cpu.pc = 0x513e;
    cpu.a = 0x7f;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::close_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0x6a5a);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    battle_tower_room_menu_set_message(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(16);
    become battle_tower_room_menu_increment_jumptable(cpu);
}

pub(crate) fn function11914e(cpu: &mut Cpu) {
    cpu.pc = 0x514e;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    battle_tower_room_menu2(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 0x1c;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcf66, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0xa;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc300, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function11915d(cpu: &mut Cpu) {
    cpu.pc = 0x515d;
    cpu.set_hl(0xd802);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xcd20);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(12 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x05;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xcd20);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xaa7f);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.c = 12;
    cpu.pc += 2;
    cpu.cycle(8);
    become function11915d_asm_119176(cpu);
}

pub(crate) fn function11915d_asm_119176(cpu: &mut Cpu) {
    cpu.pc = 0x5176;
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.read_byte(cpu.hl());
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 1;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(12);
        become function11915d_asm_119184(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
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
        become function11915d_asm_119176(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    battle_tower_room_menu_increment_jumptable(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(12);
    become function11915d_asm_11918e(cpu);
}

pub(crate) fn function11915d_asm_119184(cpu: &mut Cpu) {
    cpu.pc = 0x5184;
    cpu.a = 0x16;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcf66, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0xb;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc300, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    become function11915d_asm_11918e(cpu);
}

pub(crate) fn function11915d_asm_11918e(cpu: &mut Cpu) {
    cpu.pc = 0x518e;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::close_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function119192(cpu: &mut Cpu) {
    cpu.pc = 0x5192;
    let value = cpu.b;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.b = result;
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
    cpu.cycle(12);
    become function119192_asm_11919e(cpu);
}

pub(crate) fn function119192_asm_119196(cpu: &mut Cpu) {
    cpu.pc = 0x5196;
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 0xbf;
    cpu.pc += 2;
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
    if cpu.flag_c {
        cpu.cycle(12);
        become function119192_asm_1191a6(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become function119192_asm_11919e(cpu);
}

pub(crate) fn function119192_asm_11919e(cpu: &mut Cpu) {
    cpu.pc = 0x519e;
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
        become function119192_asm_119196(cpu);
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
        become function119192_asm_119196(cpu);
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

pub(crate) fn function119192_asm_1191a6(cpu: &mut Cpu) {
    cpu.pc = 0x51a6;
    cpu.a = 0xd3;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    set_mobile_error_code(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = true;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function1191ad(cpu: &mut Cpu) {
    cpu.pc = 0x51ad;
    cpu.push_word(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.c = 0x0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x5;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    become function1191ad_asm_1191b4(cpu);
}

pub(crate) fn function1191ad_asm_1191b4(cpu: &mut Cpu) {
    cpu.pc = 0x51b4;
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
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
        become function1191ad_asm_1191cc(cpu);
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
    let value = 0xa6;
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
        become function1191ad_asm_1191b4(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0xda;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    set_mobile_error_code(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x7f;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    let value = cpu.pop_word();
    cpu.set_bc(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = true;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function1191ad_asm_1191cc(cpu: &mut Cpu) {
    cpu.pc = 0x51cc;
    cpu.a = 0x7f;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    let value = cpu.pop_word();
    cpu.set_bc(value);
    cpu.pc += 1;
    cpu.cycle(12);
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

pub(crate) fn function1191d3(cpu: &mut Cpu) {
    cpu.pc = 0x51d3;
    cpu.set_hl({ const N: i32 = (0xd100 as i32) + (2 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcd53, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcd54, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11920f(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcd51, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xcd4a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcd52, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11920f(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcd55, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xcd4a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcd56, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11920f(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xcd49);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcd57, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xcd4a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcd58, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11920f(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function11920f(cpu: &mut Cpu) {
    cpu.pc = 0x520f;
    become function11920f_asm_11920f(cpu);
}

pub(crate) fn function11920f_asm_11920f(cpu: &mut Cpu) {
    cpu.pc = 0x520f;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function118b9a(cpu);
    assert_eq!(cpu.pc, next_pc);
    if !cpu.flag_c {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0xd;
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
        become function11920f_asm_11920f(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0xa;
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
        become function11920f_asm_11920f(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_sub(1));
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
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function119223(cpu: &mut Cpu) {
    cpu.pc = 0x5223;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcd4b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xcd4c, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x05;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xc3cd);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xb092);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(31 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_de(cpu.de().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(0xd810);
    cpu.pc += 3;
    cpu.cycle(12);
    become function119223_asm_119241(cpu);
}

pub(crate) fn function119223_asm_119241(cpu: &mut Cpu) {
    cpu.pc = 0x5241;
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0x50;
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
        become function119223_asm_119241(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(0xcd4f, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(0xcd50, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(0xb1b1, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(0xb1b2, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_de(0xb1d3);
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
    crate::game::home::sram::close_sram(cpu);
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
    become function119223_asm_119266(cpu);
}

pub(crate) fn function119223_asm_119266(cpu: &mut Cpu) {
    cpu.pc = 0x5266;
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
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
        become function119223_asm_1192c2(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0xfe;
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
        become function119223_asm_1192a5(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.push_word(cpu.de());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xcd4b);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xcd4c);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.d = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    become function119223_asm_119286(cpu);
}

pub(crate) fn function119223_asm_119286(cpu: &mut Cpu) {
    cpu.pc = 0x5286;
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_bc(cpu.bc().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
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
        become function119223_asm_119286(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::close_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcd4b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcd4c, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.pop_word();
    cpu.set_de(value);
    cpu.pc += 1;
    cpu.cycle(12);
    become function119223_asm_11929b(cpu);
}

pub(crate) fn function119223_asm_11929b(cpu: &mut Cpu) {
    cpu.pc = 0x529b;
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
        become function119223_asm_119266(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0xd3;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    set_mobile_error_code(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = true;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function119223_asm_1192a5(cpu: &mut Cpu) {
    cpu.pc = 0x52a5;
    cpu.a = cpu.read_byte(0xcd4b);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xcd4c);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    become function119223_asm_1192af(cpu);
}

pub(crate) fn function119223_asm_1192af(cpu: &mut Cpu) {
    cpu.pc = 0x52af;
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
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
    cpu.set_bc(cpu.bc().wrapping_add(1));
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
        become function119223_asm_1192af(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcd4b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcd4c, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(12);
    become function119223_asm_11929b(cpu);
}

pub(crate) fn function119223_asm_1192c2(cpu: &mut Cpu) {
    cpu.pc = 0x52c2;
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcd4d, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcd4e, cpu.a);
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
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function1192cc(cpu: &mut Cpu) {
    cpu.pc = 0x52cc;
    cpu.a = 0x05;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xaa73);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xc608);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(12 as u16);
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
    crate::game::home::sram::close_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xc608);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xcd20);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.c = 0xc;
    cpu.pc += 2;
    cpu.cycle(8);
    become function1192cc_asm_1192e8(cpu);
}

pub(crate) fn function1192cc_asm_1192e8(cpu: &mut Cpu) {
    cpu.pc = 0x52e8;
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(cpu.hl());
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
        become function1192cc_asm_1192fe(cpu);
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
        become function1192cc_asm_1192e8(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x1f;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd3c, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x27;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcf66, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = true;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function1192cc_asm_1192fe(cpu: &mut Cpu) {
    cpu.pc = 0x52fe;
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

pub(crate) fn function119300(cpu: &mut Cpu) {
    cpu.pc = 0x5300;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcd4b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xcd4c, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xcd4d);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xcd4e);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.d = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl(0xc608);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xcd4b);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xcd4c);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    become function119300_asm_11931a(cpu);
}

pub(crate) fn function119300_asm_11931a(cpu: &mut Cpu) {
    cpu.pc = 0x531a;
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0x50;
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
        become function119300_asm_119324(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_bc(cpu.bc().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.cycle(12);
    become function119300_asm_11931a(cpu);
}

pub(crate) fn function119300_asm_119324(cpu: &mut Cpu) {
    cpu.pc = 0x5324;
    cpu.a = 0x3d;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_bc(cpu.bc().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcd4b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcd4c, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.push_word(cpu.de());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xcd4b);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xcd4c);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.d = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    become function119300_asm_119348(cpu);
}

pub(crate) fn function119300_asm_119348(cpu: &mut Cpu) {
    cpu.pc = 0x5348;
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0xf0;
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
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function1197bf(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_bc(cpu.bc().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0xf;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function1197bf(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
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
        become function119300_asm_119348(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::close_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcd4b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcd4c, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.pop_word();
    cpu.set_de(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0x50;
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
        become function119300_asm_11937f(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xcd4b);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xcd4c);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = 0x26;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_bc(cpu.bc().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.cycle(12);
    become function119300_asm_11931a(cpu);
}

pub(crate) fn function119300_asm_11937f(cpu: &mut Cpu) {
    cpu.pc = 0x537f;
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function119380(cpu: &mut Cpu) {
    cpu.pc = 0x5380;
    cpu.a = 0x80;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd49, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(16);
    become battle_tower_room_menu_increment_jumptable(cpu);
}

pub(crate) fn function119388(cpu: &mut Cpu) {
    cpu.pc = 0x5388;
    cpu.set_hl(0xcd49);
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
    cpu.set_hl(0xcc60);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function118e39(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x9;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd3c, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x10;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd45, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    battle_tower_room_menu_increment_jumptable(cpu);
    assert_eq!(cpu.pc, next_pc);
    become function1193a0(cpu);
}

pub(crate) fn function1193a0(cpu: &mut Cpu) {
    cpu.pc = 0x53a0;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    battle_tower_room_menu2(cpu);
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
    crate::game::home::delay::delay_frame(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd3c, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    battle_tower_room_menu2(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xc346);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = ((0xd000u32) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = (((0xd000u32) >> 8) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xcd3b);
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
    cpu.a = ((0xc708u32) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = (((0xc708u32) >> 8) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = ((0xcc60u32) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = (((0xcc60u32) >> 8) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function119eb4(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function119ec2(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x40;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd89, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xc346);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xde00);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(0x200 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = crate::defs::MOBILEAPI_16 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become function119e2b(cpu);
}

pub(crate) fn function1193e3(cpu: &mut Cpu) {
    cpu.pc = 0x53e3;
    cpu.set_hl(0xcd49);
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
    cpu.set_hl(0xcc60);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function118e39(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x9;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd3c, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x11;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd45, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    battle_tower_room_menu_increment_jumptable(cpu);
    assert_eq!(cpu.pc, next_pc);
    become function1193fb(cpu);
}

pub(crate) fn function1193fb(cpu: &mut Cpu) {
    cpu.pc = 0x53fb;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    battle_tower_room_menu2(cpu);
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
    crate::game::home::delay::delay_frame(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd3c, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    battle_tower_room_menu2(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_de(0xd000);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(0x1000 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become function118b10(cpu);
}

pub(crate) fn function119413(cpu: &mut Cpu) {
    cpu.pc = 0x5413;
    cpu.a = 0x6;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xd000);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte({ const N: i32 = (0xd000 as i32) + (1 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_bc(cpu.bc().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_bc(cpu.bc().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(0xd002);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.d = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function119192(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xcd89);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x1;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become function119413_asm_119447(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x06;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xd000);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte({ const N: i32 = (0xd000 as i32) + (1 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl({ const N: i32 = (0xd000 as i32) + (2 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function119192(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    become function119413_asm_119447(cpu);
}

pub(crate) fn function119413_asm_119447(cpu: &mut Cpu) {
    cpu.pc = 0x5447;
    cpu.a = 0x7f;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::close_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(16);
    become battle_tower_room_menu_increment_jumptable(cpu);
}

pub(crate) fn function119451(cpu: &mut Cpu) {
    cpu.pc = 0x5451;
    cpu.a = cpu.read_byte(0xcd89);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x1;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become function119451_asm_11945d(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0xd3;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become set_mobile_error_code(cpu);
}

pub(crate) fn function119451_asm_11945d(cpu: &mut Cpu) {
    cpu.pc = 0x545d;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcd50, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function119694(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcd49, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function1196cd(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xd002);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xd003);
    cpu.pc += 3;
    cpu.cycle(12);
    become function119471(cpu);
}

pub(crate) fn function119471(cpu: &mut Cpu) {
    cpu.pc = 0x5471;
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(0xc608, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte({ const N: i32 = (0xc608 as i32) + (3 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte({ const N: i32 = (0xc608 as i32) + (1 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte({ const N: i32 = (0xc608 as i32) + (2 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte({ const N: i32 = (0xc608 as i32) + (4 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte({ const N: i32 = (0xc608 as i32) + (5 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xc608);
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
        become function119471_asm_1194a7(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte({ const N: i32 = (0xc608 as i32) + (2 as i32); N as u16 });
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
        become function119471_asm_1194a7(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte({ const N: i32 = (0xc608 as i32) + (1 as i32); N as u16 });
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
    if !cpu.flag_z {
        cpu.cycle(12);
        become function119471_asm_1194a7(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function119584(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.cycle(12);
        become function119471_asm_11950c(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become function119471_asm_1194f0(cpu);
}

pub(crate) fn function119471_asm_1194a7(cpu: &mut Cpu) {
    cpu.pc = 0x54a7;
    cpu.set_hl(0xc608);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de({ const N: i32 = (0xc608 as i32) + (3 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.c = 0x3;
    cpu.pc += 2;
    cpu.cycle(8);
    become function119471_asm_1194af(cpu);
}

pub(crate) fn function119471_asm_1194af(cpu: &mut Cpu) {
    cpu.pc = 0x54af;
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.read_byte(cpu.hl());
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    if cpu.flag_c {
        cpu.cycle(12);
        become function119471_asm_1194f3(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become function119471_asm_1194b9(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_c {
        cpu.cycle(12);
        become function119471_asm_1194bc(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become function119471_asm_1194b9(cpu);
}

pub(crate) fn function119471_asm_1194b9(cpu: &mut Cpu) {
    cpu.pc = 0x54b9;
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
        become function119471_asm_1194af(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become function119471_asm_1194bc(cpu);
}

pub(crate) fn function119471_asm_1194bc(cpu: &mut Cpu) {
    cpu.pc = 0x54bc;
    cpu.c = 0x3;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xcd49);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xc608);
    cpu.pc += 3;
    cpu.cycle(12);
    become function119471_asm_1194c4(cpu);
}

pub(crate) fn function119471_asm_1194c4(cpu: &mut Cpu) {
    cpu.pc = 0x54c4;
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
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
        become function119471_asm_1194d1(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.read_byte(cpu.hl());
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 1;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become function119471_asm_1194d1(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_c {
        cpu.cycle(12);
        become function119471_asm_1194d5(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_c {
        cpu.cycle(12);
        become function119471_asm_1194f0(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become function119471_asm_1194d1(cpu);
}

pub(crate) fn function119471_asm_1194d1(cpu: &mut Cpu) {
    cpu.pc = 0x54d1;
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
        become function119471_asm_1194c4(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become function119471_asm_1194d5(cpu);
}

pub(crate) fn function119471_asm_1194d5(cpu: &mut Cpu) {
    cpu.pc = 0x54d5;
    cpu.c = 0x3;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xcd49);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de({ const N: i32 = (0xc608 as i32) + (3 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    become function119471_asm_1194dd(cpu);
}

pub(crate) fn function119471_asm_1194dd(cpu: &mut Cpu) {
    cpu.pc = 0x54dd;
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
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
        become function119471_asm_1194ea(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.read_byte(cpu.hl());
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 1;
    cpu.cycle(8);
    if cpu.flag_c {
        cpu.cycle(12);
        become function119471_asm_1194f0(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become function119471_asm_1194ea(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_c {
        cpu.cycle(12);
        become function119471_asm_11950c(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become function119471_asm_1194ea(cpu);
}

pub(crate) fn function119471_asm_1194ea(cpu: &mut Cpu) {
    cpu.pc = 0x54ea;
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
        become function119471_asm_1194dd(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become function119471_asm_11950c(cpu);
}

pub(crate) fn function119471_asm_1194f0(cpu: &mut Cpu) {
    cpu.pc = 0x54f0;
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.cycle(12);
    become function119471_asm_119557(cpu);
}

pub(crate) fn function119471_asm_1194f3(cpu: &mut Cpu) {
    cpu.pc = 0x54f3;
    cpu.c = 0x3;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xcd49);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xc608);
    cpu.pc += 3;
    cpu.cycle(12);
    become function119471_asm_1194fb(cpu);
}

pub(crate) fn function119471_asm_1194fb(cpu: &mut Cpu) {
    cpu.pc = 0x54fb;
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
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
        become function119471_asm_119508(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.read_byte(cpu.hl());
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 1;
    cpu.cycle(8);
    if cpu.flag_c {
        cpu.cycle(12);
        become function119471_asm_11950c(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become function119471_asm_119508(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_c {
        cpu.cycle(12);
        become function119471_asm_1194d5(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become function119471_asm_119508(cpu);
}

pub(crate) fn function119471_asm_119508(cpu: &mut Cpu) {
    cpu.pc = 0x5508;
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
        become function119471_asm_1194fb(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become function119471_asm_11950c(cpu);
}

pub(crate) fn function119471_asm_11950c(cpu: &mut Cpu) {
    cpu.pc = 0x550c;
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd50, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc608, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte({ const N: i32 = (0xc608 as i32) + (1 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_de(0xcd69);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.c = 0x10;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.b = 0x0;
    cpu.pc += 2;
    cpu.cycle(8);
    become function119471_asm_119521(cpu);
}

pub(crate) fn function119471_asm_119521(cpu: &mut Cpu) {
    cpu.pc = 0x5521;
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.read_byte(cpu.hl());
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(12);
        become function119471_asm_119528(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.b;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.b = result;
    cpu.pc += 1;
    cpu.cycle(4);
    become function119471_asm_119528(cpu);
}

pub(crate) fn function119471_asm_119528(cpu: &mut Cpu) {
    cpu.pc = 0x5528;
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
        become function119471_asm_119521(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x10;
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
    if cpu.flag_z {
        cpu.cycle(12);
        become function119471_asm_119536(cpu);
    }
    cpu.pc += 2;
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
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.cycle(12);
    become function119471_asm_11957a(cpu);
}

pub(crate) fn function119471_asm_119536(cpu: &mut Cpu) {
    cpu.pc = 0x5536;
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0x50;
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
        become function119471_asm_119552(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0x33;
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
        become function119471_asm_119553(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xcd83);
    cpu.pc += 3;
    cpu.cycle(16);
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
        become function119471_asm_119576(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xcd84);
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
    if !cpu.flag_z {
        cpu.cycle(12);
        become function119471_asm_119576(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become function119471_asm_11955b(cpu);
}

pub(crate) fn function119471_asm_119552(cpu: &mut Cpu) {
    cpu.pc = 0x5552;
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    become function119471_asm_119553(cpu);
}

pub(crate) fn function119471_asm_119553(cpu: &mut Cpu) {
    cpu.pc = 0x5553;
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.cycle(12);
    become function119471_asm_11955b(cpu);
}

pub(crate) fn function119471_asm_119557(cpu: &mut Cpu) {
    cpu.pc = 0x5557;
    cpu.set_de(0x14 as u16);
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
    become function119471_asm_11955b(cpu);
}

pub(crate) fn function119471_asm_11955b(cpu: &mut Cpu) {
    cpu.pc = 0x555b;
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.d = cpu.a;
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
        cpu.cycle(16);
        become function119471(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xcd50);
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
        become function119471_asm_119571(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x1a;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcf66, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function119471_asm_119571(cpu: &mut Cpu) {
    cpu.pc = 0x5571;
    cpu.a = 0xd8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become set_mobile_error_code(cpu);
}

pub(crate) fn function119471_asm_119576(cpu: &mut Cpu) {
    cpu.pc = 0x5576;
    cpu.a = 0x10;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become function119471_asm_11957c(cpu);
}

pub(crate) fn function119471_asm_11957a(cpu: &mut Cpu) {
    cpu.pc = 0x557a;
    cpu.a = 0xf;
    cpu.pc += 2;
    cpu.cycle(8);
    become function119471_asm_11957c(cpu);
}

pub(crate) fn function119471_asm_11957c(cpu: &mut Cpu) {
    cpu.pc = 0x557c;
    cpu.write_byte(0xcf66, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function1195c4(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function119584(cpu: &mut Cpu) {
    cpu.pc = 0x5584;
    cpu.a = cpu.read_byte(0xc608);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte({ const N: i32 = (0xc608 as i32) + (3 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.b;
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
        become function119584_asm_11959c(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xcd49);
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
    if cpu.flag_c {
        cpu.cycle(12);
        become function119584_asm_1195c2(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become function119584_asm_119595(cpu);
}

pub(crate) fn function119584_asm_119595(cpu: &mut Cpu) {
    cpu.pc = 0x5595;
    let value = cpu.c;
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
        become function119584_asm_1195a2(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become function119584_asm_1195a2(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become function119584_asm_1195c2(cpu);
}

pub(crate) fn function119584_asm_11959c(cpu: &mut Cpu) {
    cpu.pc = 0x559c;
    cpu.a = cpu.read_byte(0xcd49);
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
    if cpu.flag_c {
        cpu.cycle(12);
        become function119584_asm_119595(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become function119584_asm_1195a2(cpu);
}

pub(crate) fn function119584_asm_1195a2(cpu: &mut Cpu) {
    cpu.pc = 0x55a2;
    cpu.a = cpu.read_byte({ const N: i32 = (0xc608 as i32) + (2 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte({ const N: i32 = (0xc608 as i32) + (5 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.b;
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
        become function119584_asm_1195ba(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xcd4b);
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
    if cpu.flag_c {
        cpu.cycle(12);
        become function119584_asm_1195c2(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become function119584_asm_1195b3(cpu);
}

pub(crate) fn function119584_asm_1195b3(cpu: &mut Cpu) {
    cpu.pc = 0x55b3;
    let value = cpu.c;
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
        become function119584_asm_1195c0(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become function119584_asm_1195c0(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become function119584_asm_1195c2(cpu);
}

pub(crate) fn function119584_asm_1195ba(cpu: &mut Cpu) {
    cpu.pc = 0x55ba;
    cpu.a = cpu.read_byte(0xcd4b);
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
    if cpu.flag_c {
        cpu.cycle(12);
        become function119584_asm_1195b3(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become function119584_asm_1195c0(cpu);
}

pub(crate) fn function119584_asm_1195c0(cpu: &mut Cpu) {
    cpu.pc = 0x55c0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = true;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function119584_asm_1195c2(cpu: &mut Cpu) {
    cpu.pc = 0x55c2;
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

pub(crate) fn function1195c4(cpu: &mut Cpu) {
    cpu.pc = 0x55c4;
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_de(0xa5 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.d;
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
        become function1195c4_asm_1195d9(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become function1195c4_asm_1195d3(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_c {
        cpu.cycle(12);
        become function1195c4_asm_1195f3(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become function1195c4_asm_1195d3(cpu);
}

pub(crate) fn function1195c4_asm_1195d3(cpu: &mut Cpu) {
    cpu.pc = 0x55d3;
    cpu.a = cpu.c;
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
    if cpu.flag_z {
        cpu.cycle(12);
        become function1195c4_asm_1195d9(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_c {
        cpu.cycle(12);
        become function1195c4_asm_1195f3(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become function1195c4_asm_1195d9(cpu);
}

pub(crate) fn function1195c4_asm_1195d9(cpu: &mut Cpu) {
    cpu.pc = 0x55d9;
    cpu.set_de(0xcc60);
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
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc608);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte({ const N: i32 = (0xc608 as i32) + (1 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.h = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_de(0xcd69);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(0x10 as u16);
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

pub(crate) fn function1195c4_asm_1195f3(cpu: &mut Cpu) {
    cpu.pc = 0x55f3;
    cpu.a = 0xd8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become set_mobile_error_code(cpu);
}

pub(crate) fn function1195f8(cpu: &mut Cpu) {
    cpu.pc = 0x55f8;
    cpu.a = 0x11;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd3c, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x1c;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd45, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0xf;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd46, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x14;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd47, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    battle_tower_room_menu_increment_jumptable(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(16);
    become battle_tower_room_menu_increment_jumptable(cpu);
}

pub(crate) fn function119612(cpu: &mut Cpu) {
    cpu.pc = 0x5612;
    cpu.a = 0x14;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd3c, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x1c;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd45, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x10;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd46, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x14;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd47, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(16);
    become battle_tower_room_menu_increment_jumptable(cpu);
}

pub(crate) fn function119629(cpu: &mut Cpu) {
    cpu.pc = 0x5629;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    battle_tower_room_menu2(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = ((0xcc60u32) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = (((0xcc60u32) >> 8) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.h = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function118e39(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x9;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd3c, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x1c;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd45, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x14;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd47, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    battle_tower_room_menu_increment_jumptable(cpu);
    assert_eq!(cpu.pc, next_pc);
    become function119648(cpu);
}

pub(crate) fn function119648(cpu: &mut Cpu) {
    cpu.pc = 0x5648;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    battle_tower_room_menu2(cpu);
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
    crate::game::home::delay::delay_frame(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd3c, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    battle_tower_room_menu2(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function118b24(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_de(0xd000);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(0x1000 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = crate::defs::MOBILEAPI_15 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become function119e2b(cpu);
}

pub(crate) fn function119665(cpu: &mut Cpu) {
    cpu.pc = 0x5665;
    cpu.a = 0x1a;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd3c, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    battle_tower_room_menu_increment_jumptable(cpu);
    assert_eq!(cpu.pc, next_pc);
    become function11966d(cpu);
}

pub(crate) fn function11966d(cpu: &mut Cpu) {
    cpu.pc = 0x566d;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    battle_tower_room_menu2(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xcd47);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xcf66, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0xa;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc300, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function11967d(cpu: &mut Cpu) {
    cpu.pc = 0x567d;
    cpu.a = 0x18;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd3c, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    battle_tower_room_menu_increment_jumptable(cpu);
    assert_eq!(cpu.pc, next_pc);
    become function119685(cpu);
}

pub(crate) fn function119685(cpu: &mut Cpu) {
    cpu.pc = 0x5685;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    battle_tower_room_menu2(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 0x14;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcf66, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0xa;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc300, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function119694(cpu: &mut Cpu) {
    cpu.pc = 0x5694;
    cpu.b = 0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x56b8);
    cpu.pc += 3;
    cpu.cycle(12);
    become function119694_asm_119699(cpu);
}

pub(crate) fn function119694_asm_119699(cpu: &mut Cpu) {
    cpu.pc = 0x5699;
    cpu.set_de(0xc708);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.read_byte(cpu.hl());
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(12);
        become function119694_asm_1196af(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.read_byte(cpu.hl());
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(12);
        become function119694_asm_1196b0(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.read_byte(cpu.hl());
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(12);
        become function119694_asm_1196b1(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function119694_asm_1196af(cpu: &mut Cpu) {
    cpu.pc = 0x56af;
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    become function119694_asm_1196b0(cpu);
}

pub(crate) fn function119694_asm_1196b0(cpu: &mut Cpu) {
    cpu.pc = 0x56b0;
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    become function119694_asm_1196b1(cpu);
}

pub(crate) fn function119694_asm_1196b1(cpu: &mut Cpu) {
    cpu.pc = 0x56b1;
    let value = cpu.b;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.b = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = 0x7;
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
        become function119694_asm_119699(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function1196cd(cpu: &mut Cpu) {
    cpu.pc = 0x56cd;
    cpu.set_de(0xc719);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function1196de(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.write_byte(0xcd4a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function1196de(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.write_byte(0xcd4b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function1196de(cpu: &mut Cpu) {
    cpu.pc = 0x56de;
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0x30;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
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
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
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
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    let lhs = cpu.hl();
    let rhs = cpu.bc();
    let (result, carry) = lhs.overflowing_add(rhs);
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0fff) + (rhs & 0x0fff) > 0x0fff;
    cpu.flag_c = carry;
    cpu.set_hl(result);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0x30;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.c;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_add(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) > 0x0f;
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function1196f2(cpu: &mut Cpu) {
    cpu.pc = 0x56f2;
    cpu.set_hl(0xd002);
    cpu.pc += 3;
    cpu.cycle(12);
    become function1196f2_asm_1196f5(cpu);
}

pub(crate) fn function1196f2_asm_1196f5(cpu: &mut Cpu) {
    cpu.pc = 0x56f5;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function118b9a(cpu);
    assert_eq!(cpu.pc, next_pc);
    if !cpu.flag_c {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0xd;
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
        become function1196f2_asm_1196f5(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0xa;
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
        become function1196f2_asm_1196f5(cpu);
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
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcd5b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcd5c, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.e = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.d = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0xd;
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
        become function1196f2_asm_119722(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0xa;
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
        become function1196f2_asm_119722(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0xb;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become set_mobile_error_code(cpu);
}

pub(crate) fn function1196f2_asm_119722(cpu: &mut Cpu) {
    cpu.pc = 0x5722;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::random::random(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.c = 0x0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.b = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    become function1196f2_asm_119728(cpu);
}

pub(crate) fn function1196f2_asm_119728(cpu: &mut Cpu) {
    cpu.pc = 0x5728;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function119798(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = 0xff;
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
        become function1196f2_asm_119735(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
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
        become function1196f2_asm_11974c(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become function1196f2_asm_119735(cpu);
}

pub(crate) fn function1196f2_asm_119735(cpu: &mut Cpu) {
    cpu.pc = 0x5735;
    cpu.a = cpu.read_byte(0xffe2);
    cpu.pc += 2;
    cpu.cycle(12);
    let value = cpu.d;
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
        become function1196f2_asm_11974c(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become function1196f2_asm_11973e(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become function1196f2_asm_119745(cpu);
}

pub(crate) fn function1196f2_asm_11973e(cpu: &mut Cpu) {
    cpu.pc = 0x573e;
    cpu.a = cpu.read_byte(0xffe1);
    cpu.pc += 2;
    cpu.cycle(12);
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
        become function1196f2_asm_11974c(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become function1196f2_asm_11974c(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become function1196f2_asm_119745(cpu);
}

pub(crate) fn function1196f2_asm_119745(cpu: &mut Cpu) {
    cpu.pc = 0x5745;
    cpu.set_bc(cpu.bc().wrapping_add(1));
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
    if cpu.flag_z {
        cpu.cycle(12);
        become function1196f2_asm_119770(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become function1196f2_asm_119728(cpu);
}

pub(crate) fn function1196f2_asm_11974c(cpu: &mut Cpu) {
    cpu.pc = 0x574c;
    cpu.a = cpu.read_byte(0xcd5b);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xcd5c);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.h = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    become function1196f2_asm_119754(cpu);
}

pub(crate) fn function1196f2_asm_119754(cpu: &mut Cpu) {
    cpu.pc = 0x5754;
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0x58;
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
        become function1196f2_asm_119754(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.d = 0x0;
    cpu.pc += 2;
    cpu.cycle(8);
    become function1196f2_asm_11975b(cpu);
}

pub(crate) fn function1196f2_asm_11975b(cpu: &mut Cpu) {
    cpu.pc = 0x575b;
    let value = cpu.d;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.d = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0x58;
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
        become function1196f2_asm_11975b(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
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
        become function1196f2_asm_11978e(cpu);
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
    if cpu.flag_z {
        cpu.cycle(12);
        become function1196f2_asm_119785(cpu);
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
    if cpu.flag_z {
        cpu.cycle(12);
        become function1196f2_asm_11977e(cpu);
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
    if cpu.flag_z {
        cpu.cycle(12);
        become function1196f2_asm_119775(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become function1196f2_asm_119770(cpu);
}

pub(crate) fn function1196f2_asm_119770(cpu: &mut Cpu) {
    cpu.pc = 0x5770;
    cpu.a = 0xd3;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become set_mobile_error_code(cpu);
}

pub(crate) fn function1196f2_asm_119775(cpu: &mut Cpu) {
    cpu.pc = 0x5775;
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = 0xf0;
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
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function1197bf(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    become function1196f2_asm_11977e(cpu);
}

pub(crate) fn function1196f2_asm_11977e(cpu: &mut Cpu) {
    cpu.pc = 0x577e;
    cpu.a = cpu.b;
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
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function1197bf(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    become function1196f2_asm_119785(cpu);
}

pub(crate) fn function1196f2_asm_119785(cpu: &mut Cpu) {
    cpu.pc = 0x5785;
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = 0xf0;
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
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function1197bf(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    become function1196f2_asm_11978e(cpu);
}

pub(crate) fn function1196f2_asm_11978e(cpu: &mut Cpu) {
    cpu.pc = 0x578e;
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
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function1197bf(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.cycle(16);
    become battle_tower_room_menu_increment_jumptable(cpu);
}

pub(crate) fn function119798(cpu: &mut Cpu) {
    cpu.pc = 0x5798;
    cpu.d = 0x0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.e = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function1197b4(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.a;
    let result = value.rotate_left(4);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
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
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function1197b4(cpu);
    assert_eq!(cpu.pc, next_pc);
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
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function1197b4(cpu);
    assert_eq!(cpu.pc, next_pc);
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
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function1197b4(cpu);
    assert_eq!(cpu.pc, next_pc);
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
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function1197b4(cpu: &mut Cpu) {
    cpu.pc = 0x57b4;
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0x61;
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
        become function1197b4_asm_1197bc(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x30;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function1197b4_asm_1197bc(cpu: &mut Cpu) {
    cpu.pc = 0x57bc;
    let value = 0x57;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function1197bf(cpu: &mut Cpu) {
    cpu.pc = 0x57bf;
    let value = 0xa;
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
        become function1197bf_asm_1197c6(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x30;
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

pub(crate) fn function1197bf_asm_1197c6(cpu: &mut Cpu) {
    cpu.pc = 0x57c6;
    let value = 0x57;
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

pub(crate) fn function1197c9(cpu: &mut Cpu) {
    cpu.pc = 0x57c9;
    cpu.set_hl(0xd002);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function118e39(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x9;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd3c, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x12;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd45, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    battle_tower_room_menu_increment_jumptable(cpu);
    assert_eq!(cpu.pc, next_pc);
    become function1197dc(cpu);
}

pub(crate) fn function1197dc(cpu: &mut Cpu) {
    cpu.pc = 0x57dc;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    battle_tower_room_menu2(cpu);
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
    crate::game::home::delay::delay_frame(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xd002);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xcc60);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(0x0080 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_de(cpu.de().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function118b24(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_de(0xd000);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(0x1000 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = crate::defs::MOBILEAPI_15 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become function119e2b(cpu);
}

pub(crate) fn function119800(cpu: &mut Cpu) {
    cpu.pc = 0x5800;
    cpu.a = 0xfd;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc6d0, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xc702, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xcd81);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xc74e, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xcf63);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xcf64);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xcf65);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xcf66);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map::fade_to_menu(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x42;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x403d);
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
    function11a9ce(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::audio::restart_map_music(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x7f;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte(0xcf66, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte(0xcf65, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte(0xcf64, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte(0xcf63, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x45;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x5dc3);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(16);
    become battle_tower_room_menu_increment_jumptable(cpu);
}

pub(crate) fn function11984e(cpu: &mut Cpu) {
    cpu.pc = 0x584e;
    cpu.a = cpu.read_byte(0xcd80);
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
        become function11984e_asm_1198a0(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xcd38);
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
        become function11984e_asm_1198a8(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x5c;
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
    cpu.a = cpu.read_byte(0xcf63);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xcf64);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xcf65);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xcf66);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map::fade_to_menu(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x42;
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
    function11a9ce(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::audio::restart_map_music(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x7f;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte(0xcf66, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte(0xcf65, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte(0xcf64, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte(0xcf63, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x45;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x5dc3);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(16);
    become battle_tower_room_menu_increment_jumptable(cpu);
}

pub(crate) fn function11984e_asm_1198a0(cpu: &mut Cpu) {
    cpu.pc = 0x58a0;
    cpu.a = 0xa;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc300, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(16);
    become battle_tower_room_menu_increment_jumptable(cpu);
}

pub(crate) fn function11984e_asm_1198a8(cpu: &mut Cpu) {
    cpu.pc = 0x58a8;
    cpu.a = 0x5c;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x405a);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xcf63);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xcf64);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xcf65);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xcf66);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map::fade_to_menu(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x42;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x4012);
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
    function11a9ce(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::audio::restart_map_music(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x7f;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte(0xcf66, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte(0xcf65, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte(0xcf64, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte(0xcf63, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x45;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x5dc3);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(16);
    become battle_tower_room_menu_increment_jumptable(cpu);
}

pub(crate) fn function1198ee(cpu: &mut Cpu) {
    cpu.pc = 0x58ee;
    cpu.set_hl(0x6ab2);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    battle_tower_room_menu_set_message(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    battle_tower_room_menu_increment_jumptable(cpu);
    assert_eq!(cpu.pc, next_pc);
    become function1198f7(cpu);
}

pub(crate) fn function1198f7(cpu: &mut Cpu) {
    cpu.pc = 0x58f7;
    cpu.a = cpu.read_byte(0xc31a);
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
    cpu.set_hl({ const N: i32 = (0xc608 as i32) + (2 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function119940(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xd800);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = ((0xc608u32) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = (((0xc608u32) >> 8) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 0xf6;
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
    cpu.a = ((0xc708u32) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = (((0xc708u32) >> 8) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xcd51);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xcd52);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function119eb4(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function119ec2(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x40;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd89, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xd800);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xde00);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(0x200 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = crate::defs::MOBILEAPI_16 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become function119e2b(cpu);
}

pub(crate) fn function119937(cpu: &mut Cpu) {
    cpu.pc = 0x5937;
    cpu.a = 0x5c;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x47f4);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(16);
    become battle_tower_room_menu_increment_jumptable(cpu);
}

pub(crate) fn function119940(cpu: &mut Cpu) {
    cpu.pc = 0x5940;
    cpu.set_de(0xc3cd);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.c = crate::defs::MOBILE_EMAIL_LENGTH as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    become function119940_asm_119945(cpu);
}

pub(crate) fn function119940_asm_119945(cpu: &mut Cpu) {
    cpu.pc = 0x5945;
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
    let value = cpu.c;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.c = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function119940_asm_119953(cpu);
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
    if !cpu.flag_z {
        cpu.cycle(12);
        become function119940_asm_119945(cpu);
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
    become function119940_asm_11994f(cpu);
}

pub(crate) fn function119940_asm_11994f(cpu: &mut Cpu) {
    cpu.pc = 0x594f;
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
        become function119940_asm_11994f(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become function119940_asm_119953(cpu);
}

pub(crate) fn function119940_asm_119953(cpu: &mut Cpu) {
    cpu.pc = 0x5953;
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function119954(cpu: &mut Cpu) {
    cpu.pc = 0x5954;
    cpu.a = 0x1c;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd3c, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    battle_tower_room_menu2(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xc608);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xc3cd);
    cpu.pc += 3;
    cpu.cycle(12);
    become function119954_asm_119962(cpu);
}

pub(crate) fn function119954_asm_119962(cpu: &mut Cpu) {
    cpu.pc = 0x5962;
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
        become function119954_asm_119962(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function119ec2(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xc608);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = crate::defs::MOBILEAPI_0F as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become function119e2b(cpu);
}

pub(crate) fn function119973(cpu: &mut Cpu) {
    cpu.pc = 0x5973;
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcf64, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
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
    cpu.write_byte(0xd090, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_de(0xd000);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = crate::defs::MOBILEAPI_10 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become function119e2b(cpu);
}

pub(crate) fn function119987(cpu: &mut Cpu) {
    cpu.pc = 0x5987;
    cpu.set_hl({ const N: i32 = (0xd000 as i32) + (1 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xd000);
    cpu.pc += 3;
    cpu.cycle(16);
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
        become function119987_asm_1199a0(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xcf64);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xcf65);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.h = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_de(0xd080);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = crate::defs::MOBILEAPI_11 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become function119e2b(cpu);
}

pub(crate) fn function119987_asm_1199a0(cpu: &mut Cpu) {
    cpu.pc = 0x59a0;
    cpu.a = cpu.read_byte(0xd090);
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
        become function119987_asm_1199ae(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x16;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcf66, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(16);
    become function119b0d(cpu);
}

pub(crate) fn function119987_asm_1199ae(cpu: &mut Cpu) {
    cpu.pc = 0x59ae;
    cpu.a = 0x1b;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcf66, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function1199b4(cpu: &mut Cpu) {
    cpu.pc = 0x59b4;
    cpu.a = cpu.read_byte({ const N: i32 = (0xd081 as i32) + (1 as i32); N as u16 });
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
        become function1199b4_asm_1199c7(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xd081);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x7;
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
        become function1199b4_asm_1199c7(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    battle_tower_room_menu_increment_jumptable(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(16);
    become function1199ca(cpu);
}

pub(crate) fn function1199b4_asm_1199c7(cpu: &mut Cpu) {
    cpu.pc = 0x59c7;
    cpu.cycle(16);
    become function119ac9(cpu);
}

pub(crate) fn function1199ca(cpu: &mut Cpu) {
    cpu.pc = 0x59ca;
    cpu.a = 0x20;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd89, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xcf64);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xcf65);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.h = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_de(0xd100);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(0x0700 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = crate::defs::MOBILEAPI_14 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become function119e2b(cpu);
}

pub(crate) fn function1199e2(cpu: &mut Cpu) {
    cpu.pc = 0x59e2;
    cpu.c = 0xc;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_de(0x5e33);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function119e4f(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.cycle(16);
        become function119ac9(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = 0x1;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(16);
        become function119ac9(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0xd880);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(0x5af1);
    cpu.pc += 3;
    cpu.cycle(12);
    become function1199e2_loop(cpu);
}

pub(crate) fn function1199e2_loop(cpu: &mut Cpu) {
    cpu.pc = 0x59f9;
    cpu.a = cpu.read_byte(cpu.bc());
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
        become function1199e2_game_result_prefix(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.read_byte(cpu.hl());
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 1;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(16);
        become function119ac9(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(cpu.bc().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.cycle(12);
    become function1199e2_loop(cpu);
}

pub(crate) fn function1199e2_game_result_prefix(cpu: &mut Cpu) {
    cpu.pc = 0x5a05;
    cpu.c = 0x17;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_de(0x5e40);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function119e4f(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.cycle(16);
        become function1199e2_asm_119aa7(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = 0x1;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(16);
        become function1199e2_asm_119aa7(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xd880);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x31;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(16);
        become function1199e2_asm_119aa7(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xd881);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x20;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(16);
        become function1199e2_asm_119aa7(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xd88a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x20;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(16);
        become function1199e2_asm_119aa7(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xd894);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x20;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(16);
        become function1199e2_asm_119aa7(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xd8a0, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xd8a1, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xd8a2, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xd8a3, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xd8a0);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(0xd889);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function119e98(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function119e98(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xd8a0);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xcd2d);
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
    if !cpu.flag_z {
        cpu.cycle(12);
        become function119ac9(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xcd2c);
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
    if !cpu.flag_z {
        cpu.cycle(12);
        become function119ac9(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xcd2b);
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
    if !cpu.flag_z {
        cpu.cycle(12);
        become function119ac9(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xcd2a);
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
    if !cpu.flag_z {
        cpu.cycle(12);
        become function119ac9(cpu);
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
    cpu.write_byte(0xd8a0, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xd8a1, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xd8a2, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xd8a3, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xd8a0);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(0xd88e);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function119e98(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_bc(0xd893);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function119e98(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xd8a0);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xcd2f);
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
    if !cpu.flag_z {
        cpu.cycle(12);
        become function1199e2_asm_119aa7(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xcd2e);
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
    if !cpu.flag_z {
        cpu.cycle(12);
        become function1199e2_asm_119aa7(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xcd31);
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
    if !cpu.flag_z {
        cpu.cycle(12);
        become function1199e2_asm_119aa7(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xcd30);
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
    if cpu.flag_z {
        cpu.cycle(12);
        become function1199e2_asm_119aaf(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become function1199e2_asm_119aa7(cpu);
}

pub(crate) fn function1199e2_asm_119aa7(cpu: &mut Cpu) {
    cpu.pc = 0x5aa7;
    cpu.a = 0x25;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcf66, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(16);
    become function119b45(cpu);
}

pub(crate) fn function1199e2_asm_119aaf(cpu: &mut Cpu) {
    cpu.pc = 0x5aaf;
    cpu.a = cpu.read_byte(0xd090);
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
        become function1199e2_asm_119aa7(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xd895);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x30;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xd090, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xcf64);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte({ const N: i32 = (0xd090 as i32) + (1 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xcf65);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte({ const N: i32 = (0xd090 as i32) + (2 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    become function119ac9(cpu);
}

pub(crate) fn function119ac9(cpu: &mut Cpu) {
    cpu.pc = 0x5ac9;
    cpu.a = cpu.read_byte(0xd000);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte({ const N: i32 = (0xd000 as i32) + (1 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.h = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xd000, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte({ const N: i32 = (0xd000 as i32) + (1 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xcf64);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xcf65);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.h = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcf64, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcf65, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x10;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcf66, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function119b0d(cpu: &mut Cpu) {
    cpu.pc = 0x5b0d;
    cpu.a = 0x8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd3c, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    battle_tower_room_menu2(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xd090);
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
        become function119b0d_asm_119b23(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x19;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcf66, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(12);
    become function119b3b(cpu);
}

pub(crate) fn function119b0d_asm_119b23(cpu: &mut Cpu) {
    cpu.pc = 0x5b23;
    cpu.a = 0x10;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd89, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte({ const N: i32 = (0xd090 as i32) + (1 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte({ const N: i32 = (0xd090 as i32) + (2 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.h = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_de(0xd100);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(0x0700 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = crate::defs::MOBILEAPI_12 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become function119e2b(cpu);
}

pub(crate) fn function119b3b(cpu: &mut Cpu) {
    cpu.pc = 0x5b3b;
    cpu.a = cpu.read_byte({ const N: i32 = (0xd090 as i32) + (1 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte({ const N: i32 = (0xd090 as i32) + (2 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.h = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.cycle(12);
    become asm_119b4d(cpu);
}

pub(crate) fn function119b45(cpu: &mut Cpu) {
    cpu.pc = 0x5b45;
    cpu.a = cpu.read_byte(0xcf64);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xcf65);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.h = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    become asm_119b4d(cpu);
}

pub(crate) fn asm_119b4d(cpu: &mut Cpu) {
    cpu.pc = 0x5b4d;
    cpu.a = crate::defs::MOBILEAPI_13 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become function119e2b(cpu);
}

pub(crate) fn function119b52(cpu: &mut Cpu) {
    cpu.pc = 0x5b52;
    cpu.a = cpu.read_byte(0xd090);
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
    if !cpu.flag_z {
        cpu.cycle(12);
        become function119b52_asm_119b66(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x05;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x4;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xa800, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::close_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    become function119b52_asm_119b66(cpu);
}

pub(crate) fn function119b52_asm_119b66(cpu: &mut Cpu) {
    cpu.pc = 0x5b66;
    cpu.a = crate::defs::MOBILEAPI_0E as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become function119e2b(cpu);
}

pub(crate) fn function119b6b(cpu: &mut Cpu) {
    cpu.pc = 0x5b6b;
    cpu.a = cpu.read_byte(0xd090);
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
        become function119b6b_asm_119b75(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become battle_tower_room_menu_increment_jumptable(cpu);
}

pub(crate) fn function119b6b_asm_119b75(cpu: &mut Cpu) {
    cpu.pc = 0x5b75;
    cpu.a = cpu.read_byte(0xd100);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte({ const N: i32 = (0xd100 as i32) + (1 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.b;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function119b6b_asm_119be3(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xd800);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de({ const N: i32 = (0xd100 as i32) + (2 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    become function119b6b_asm_119b85(cpu);
}

pub(crate) fn function119b6b_asm_119b85(cpu: &mut Cpu) {
    cpu.pc = 0x5b85;
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0xd;
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
        become function119b6b_asm_119b85(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0xd;
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
        become function119b6b_asm_119b85(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    become function119b6b_asm_119b93(cpu);
}

pub(crate) fn function119b6b_asm_119b93(cpu: &mut Cpu) {
    cpu.pc = 0x5b93;
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0xd;
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
        become function119b6b_asm_119bfa(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function119b6b_decode_base64_character(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
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
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function119b6b_decode_base64_character(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
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
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function119b6b_decode_base64_character(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
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
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function119b6b_decode_base64_character(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.push_word(cpu.de());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.d = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.c = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.b = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.b;
    let result = value << 1;
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x80 != 0;
    cpu.b = result;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.b;
    let result = value << 1;
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x80 != 0;
    cpu.b = result;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.b;
    let result = value << 1;
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x80 != 0;
    cpu.b = result;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.a;
    let carry_in = u8::from(cpu.flag_c);
    let result = (value << 1) | carry_in;
    cpu.flag_z = false;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x80 != 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.b;
    let result = value << 1;
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x80 != 0;
    cpu.b = result;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.a;
    let carry_in = u8::from(cpu.flag_c);
    let result = (value << 1) | carry_in;
    cpu.flag_z = false;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x80 != 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.b);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.c;
    let result = value.rotate_right(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x01 != 0;
    cpu.c = result;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.c;
    let result = value.rotate_right(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x01 != 0;
    cpu.c = result;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.c);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 0xf;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.c;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.read_byte(cpu.hl());
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0xc0;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.read_byte(cpu.hl());
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.pop_word();
    cpu.set_de(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = 0xe0;
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
        become function119b6b_asm_119b93(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become function119b6b_asm_119be3(cpu);
}

pub(crate) fn function119b6b_asm_119be3(cpu: &mut Cpu) {
    cpu.pc = 0x5be3;
    cpu.a = 0x19;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcf66, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x05;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xa800, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
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
    cpu.write_byte(0xd090, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function119b6b_asm_119bfa(cpu: &mut Cpu) {
    cpu.pc = 0x5bfa;
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0xd;
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
        become function119b6b_asm_119b93(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = ((0xd869u32) & 0x00ff) as u8;
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
        become function119b6b_asm_119be3(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = (((0xd869u32) >> 8) & 0x00ff) as u8;
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
        become function119b6b_asm_119be3(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x05;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xcf64);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xb090, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xcf65);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xb091, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xd800);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xb023);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(105 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x3;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xa800, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::close_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xd800);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xc608);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(105 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(16);
    become battle_tower_room_menu_increment_jumptable(cpu);
}

pub(crate) fn function119b6b_decode_base64_character(cpu: &mut Cpu) {
    cpu.pc = 0x5c3e;
    let value = 0x2b;
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
        become function119b6b_asm_119c68(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become function119b6b_asm_119c80(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = { let value = (0xf3 as i32); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
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
        become function119b6b_asm_119c68(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become function119b6b_asm_119c84(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = { let value = (0xf6 as i32); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
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
        become function119b6b_asm_119c68(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = { let value = ((0xff as i32) + (1 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
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
        become function119b6b_asm_119c88(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x3d;
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
        become function119b6b_asm_119c68(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become function119b6b_asm_119c8c(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = { let value = (0x80 as i32); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
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
        become function119b6b_asm_119c68(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = { let value = ((0x99 as i32) + (1 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
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
        become function119b6b_asm_119c8f(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = { let value = (0xa0 as i32); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
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
        become function119b6b_asm_119c68(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = { let value = ((0xb9 as i32) + (1 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
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
        become function119b6b_asm_119c93(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become function119b6b_asm_119c68(cpu);
}

pub(crate) fn function119b6b_asm_119c68(cpu: &mut Cpu) {
    cpu.pc = 0x5c68;
    cpu.a = 0x19;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcf66, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x05;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xa800, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
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
    cpu.write_byte(0xd090, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = true;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function119b6b_asm_119c80(cpu: &mut Cpu) {
    cpu.pc = 0x5c80;
    cpu.a = 0x3e;
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

pub(crate) fn function119b6b_asm_119c84(cpu: &mut Cpu) {
    cpu.pc = 0x5c84;
    cpu.a = 0x3f;
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

pub(crate) fn function119b6b_asm_119c88(cpu: &mut Cpu) {
    cpu.pc = 0x5c88;
    let value = 0x4;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_add(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) > 0x0f;
    cpu.flag_c = carry;
    cpu.a = result;
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

pub(crate) fn function119b6b_asm_119c8c(cpu: &mut Cpu) {
    cpu.pc = 0x5c8c;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
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

pub(crate) fn function119b6b_asm_119c8f(cpu: &mut Cpu) {
    cpu.pc = 0x5c8f;
    let value = 0x41;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.a = result;
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

pub(crate) fn function119b6b_asm_119c93(cpu: &mut Cpu) {
    cpu.pc = 0x5c93;
    let value = 0x47;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.a = result;
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

pub(crate) fn battle_tower_room_menu_uber_restriction_message(cpu: &mut Cpu) {
    cpu.pc = 0x5c97;
    cpu.set_hl(0x6b0f);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    battle_tower_room_menu_set_message(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    battle_tower_room_menu_increment_jumptable(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(12);
    become battle_tower_room_menu_wait_for_message(cpu);
}

pub(crate) fn battle_tower_room_menu_party_mon_tops_this_level_message(cpu: &mut Cpu) {
    cpu.pc = 0x5ca2;
    cpu.set_hl(0x6af0);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    battle_tower_room_menu_set_message(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    battle_tower_room_menu_increment_jumptable(cpu);
    assert_eq!(cpu.pc, next_pc);
    become battle_tower_room_menu_wait_for_message(cpu);
}

pub(crate) fn battle_tower_room_menu_wait_for_message(cpu: &mut Cpu) {
    cpu.pc = 0x5cab;
    cpu.a = cpu.read_byte(0xc31a);
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
    cpu.a = 0x80;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd50, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    battle_tower_room_menu_increment_jumptable(cpu);
    assert_eq!(cpu.pc, next_pc);
    become battle_tower_room_menu_delay_restart_menu(cpu);
}

pub(crate) fn battle_tower_room_menu_delay_restart_menu(cpu: &mut Cpu) {
    cpu.pc = 0x5cb8;
    cpu.set_hl(0xcd50);
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
    cpu.a = 0x0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcf66, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn battle_tower_room_menu_quit_message(cpu: &mut Cpu) {
    cpu.pc = 0x5cc3;
    cpu.a = cpu.read_byte(0xcd38);
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
        become battle_tower_room_menu_quit_message_asm_119cd1(cpu);
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
    if cpu.flag_z {
        cpu.cycle(12);
        become battle_tower_room_menu_quit_message_asm_119cd6(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x6a6a);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(12);
    become battle_tower_room_menu_quit_message_asm_119cd9(cpu);
}

pub(crate) fn battle_tower_room_menu_quit_message_asm_119cd1(cpu: &mut Cpu) {
    cpu.pc = 0x5cd1;
    cpu.set_hl(0x6b4a);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(12);
    become battle_tower_room_menu_quit_message_asm_119cd9(cpu);
}

pub(crate) fn battle_tower_room_menu_quit_message_asm_119cd6(cpu: &mut Cpu) {
    cpu.pc = 0x5cd6;
    cpu.set_hl(0x6b6e);
    cpu.pc += 3;
    cpu.cycle(12);
    become battle_tower_room_menu_quit_message_asm_119cd9(cpu);
}

pub(crate) fn battle_tower_room_menu_quit_message_asm_119cd9(cpu: &mut Cpu) {
    cpu.pc = 0x5cd9;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    battle_tower_room_menu_set_message(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    battle_tower_room_menu_increment_jumptable(cpu);
    assert_eq!(cpu.pc, next_pc);
    become battle_tower_room_menu_place_yes_no_menu(cpu);
}

pub(crate) fn battle_tower_room_menu_place_yes_no_menu(cpu: &mut Cpu) {
    cpu.pc = 0x5cdf;
    cpu.a = cpu.read_byte(0xc31a);
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
    cpu.a = 0xf;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd3c, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    battle_tower_room_menu_increment_jumptable(cpu);
    assert_eq!(cpu.pc, next_pc);
    become battle_tower_room_menu_update_yes_no_menu(cpu);
}

pub(crate) fn battle_tower_room_menu_update_yes_no_menu(cpu: &mut Cpu) {
    cpu.pc = 0x5cec;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    battle_tower_room_menu2(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xcd46);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xcf66, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn battle_tower_level_check(cpu: &mut Cpu) {
    cpu.pc = 0x5d93;
    cpu.a = cpu.read_byte(crate::defs::R_WBK as u16);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = 0x01;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xcd4f);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.c = 10;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::math::simple_multiply(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xcd50);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_bc(crate::defs::PARTYMON_STRUCT_LENGTH as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xdcfe);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xdcd7);
    cpu.pc += 3;
    cpu.cycle(16);
    become battle_tower_level_check_party_loop(cpu);
}

pub(crate) fn battle_tower_level_check_party_loop(cpu: &mut Cpu) {
    cpu.pc = 0x5daf;
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.push_word(cpu.de());
    cpu.pc += 1;
    cpu.cycle(16);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
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
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    let value = cpu.pop_word();
    cpu.set_de(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let value = cpu.read_byte(cpu.hl());
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 1;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become battle_tower_level_check_equal(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_c {
        cpu.cycle(12);
        become battle_tower_level_check_exceeds(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become battle_tower_level_check_equal(cpu);
}

pub(crate) fn battle_tower_level_check_equal(cpu: &mut Cpu) {
    cpu.pc = 0x5dbd;
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
        become battle_tower_level_check_party_loop(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
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
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn battle_tower_level_check_exceeds(cpu: &mut Cpu) {
    cpu.pc = 0x5dc6;
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.a = 0x4;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcf66, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = true;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn battle_tower_ubers_check(cpu: &mut Cpu) {
    cpu.pc = 0x5dd1;
    cpu.a = cpu.read_byte(crate::defs::R_WBK as u16);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xcd4f);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = { let value = ((70 as i32) / (10 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
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
        become battle_tower_ubers_check_level_70_or_more(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x01;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.set_hl(0xdcfe);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(crate::defs::PARTYMON_STRUCT_LENGTH as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xdcd8);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xdcd7);
    cpu.pc += 3;
    cpu.cycle(16);
    become battle_tower_ubers_check_loop(cpu);
}

pub(crate) fn battle_tower_ubers_check_loop(cpu: &mut Cpu) {
    cpu.pc = 0x5deb;
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = crate::defs::MEWTWO as u8;
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
        become battle_tower_ubers_check_uber(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = crate::defs::MEW as u8;
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
        become battle_tower_ubers_check_uber(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = crate::defs::LUGIA as u8;
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
        become battle_tower_ubers_check_next(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = { let value = ((crate::defs::NUM_POKEMON as i32) + (1 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
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
        become battle_tower_ubers_check_next(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become battle_tower_ubers_check_uber(cpu);
}

pub(crate) fn battle_tower_ubers_check_uber(cpu: &mut Cpu) {
    cpu.pc = 0x5dfd;
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 70;
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
        become battle_tower_ubers_check_uber_under_70(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become battle_tower_ubers_check_next(cpu);
}

pub(crate) fn battle_tower_ubers_check_next(cpu: &mut Cpu) {
    cpu.pc = 0x5e02;
    let lhs = cpu.hl();
    let rhs = cpu.bc();
    let (result, carry) = lhs.overflowing_add(rhs);
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0fff) + (rhs & 0x0fff) > 0x0fff;
    cpu.flag_c = carry;
    cpu.set_hl(result);
    cpu.pc += 1;
    cpu.cycle(8);
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
        become battle_tower_ubers_check_loop(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become battle_tower_ubers_check_level_70_or_more(cpu);
}

pub(crate) fn battle_tower_ubers_check_level_70_or_more(cpu: &mut Cpu) {
    cpu.pc = 0x5e08;
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
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
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn battle_tower_ubers_check_uber_under_70(cpu: &mut Cpu) {
    cpu.pc = 0x5e0d;
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.de());
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
    cpu.set_hl(0xd073);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xcd49);
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
    cpu.a = 0xa;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcf66, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = true;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function119e2b(cpu: &mut Cpu) {
    cpu.pc = 0x5e2b;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::mobile::mobile_api(cpu);
    assert_eq!(cpu.pc, next_pc);
    become battle_tower_room_menu_increment_jumptable(cpu);
}

pub(crate) fn battle_tower_room_menu_increment_jumptable(cpu: &mut Cpu) {
    cpu.pc = 0x5e2e;
    cpu.set_hl(0xcf66);
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

pub(crate) fn function119e4f(cpu: &mut Cpu) {
    cpu.pc = 0x5e4f;
    cpu.push_word(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.set_hl(0xd100);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    become function119e4f_loop(cpu);
}

pub(crate) fn function119e4f_loop(cpu: &mut Cpu) {
    cpu.pc = 0x5e57;
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.read_byte(cpu.hl());
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 1;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become function119e4f_found_equality(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become function119e4f_next(cpu);
}

pub(crate) fn function119e4f_next(cpu: &mut Cpu) {
    cpu.pc = 0x5e5b;
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_bc(cpu.bc().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.c;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(12);
        become function119e4f_loop(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become function119e4f_return_carry(cpu);
}

pub(crate) fn function119e4f_return_carry(cpu: &mut Cpu) {
    cpu.pc = 0x5e61;
    let value = cpu.pop_word();
    cpu.set_bc(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = true;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function119e4f_found_equality(cpu: &mut Cpu) {
    cpu.pc = 0x5e64;
    cpu.push_word(cpu.de());
    cpu.pc += 1;
    cpu.cycle(16);
    become function119e4f_loop2(cpu);
}

pub(crate) fn function119e4f_loop2(cpu: &mut Cpu) {
    cpu.pc = 0x5e65;
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0x0a;
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
        become function119e4f_newline(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.read_byte(cpu.hl());
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 1;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(12);
        become function119e4f_unequal(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_bc(cpu.bc().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.c;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(12);
        become function119e4f_loop2(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.pop_word();
    cpu.set_de(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.cycle(12);
    become function119e4f_return_carry(cpu);
}

pub(crate) fn function119e4f_unequal(cpu: &mut Cpu) {
    cpu.pc = 0x5e77;
    let value = cpu.pop_word();
    cpu.set_de(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.cycle(12);
    become function119e4f_next(cpu);
}

pub(crate) fn function119e4f_newline(cpu: &mut Cpu) {
    cpu.pc = 0x5e7a;
    let value = cpu.pop_word();
    cpu.set_de(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let value = cpu.pop_word();
    cpu.set_bc(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(0xd880);
    cpu.pc += 3;
    cpu.cycle(12);
    become function119e4f_loop3(cpu);
}

pub(crate) fn function119e4f_loop3(cpu: &mut Cpu) {
    cpu.pc = 0x5e80;
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0x0d;
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
        become function119e4f_finish(cpu);
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
        become function119e4f_loop3(cpu);
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

pub(crate) fn function119e4f_finish(cpu: &mut Cpu) {
    cpu.pc = 0x5e8c;
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

pub(crate) fn function119e8e(cpu: &mut Cpu) {
    cpu.pc = 0x5e8e;
    let value = 0x60;
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
        become function119e8e_less_than_0x60(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x57;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function119e8e_less_than_0x60(cpu: &mut Cpu) {
    cpu.pc = 0x5e95;
    let value = 0x30;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function119e98(cpu: &mut Cpu) {
    cpu.pc = 0x5e98;
    cpu.a = 0x2;
    cpu.pc += 2;
    cpu.cycle(8);
    become function119e98_loop(cpu);
}

pub(crate) fn function119e98_loop(cpu: &mut Cpu) {
    cpu.pc = 0x5e9a;
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_bc(cpu.bc().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function119e8e(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.read_byte(cpu.hl());
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_bc(cpu.bc().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function119e8e(cpu);
    assert_eq!(cpu.pc, next_pc);
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
    let value = cpu.read_byte(cpu.hl());
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
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
        become function119e98_loop(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function119eb4(cpu: &mut Cpu) {
    cpu.pc = 0x5eb4;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc3cc, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_de(0xc3ac);
    cpu.pc += 3;
    cpu.cycle(12);
    become function119eb4_loop(cpu);
}

pub(crate) fn function119eb4_loop(cpu: &mut Cpu) {
    cpu.pc = 0x5ebb;
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
        become function119eb4_loop(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function119ec2(cpu: &mut Cpu) {
    cpu.pc = 0x5ec2;
    cpu.a = 0x05;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte({ const N: i32 = (0xaa4b as i32) + (crate::defs::MOBILE_LOGIN_PASSWORD_LENGTH as u16 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_de({ const N: i32 = (0xaa4b as i32) + (1 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    become function119ec2_loop(cpu);
}

pub(crate) fn function119ec2_loop(cpu: &mut Cpu) {
    cpu.pc = 0x5ece;
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
        become function119ec2_loop(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::close_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn battle_tower_room_menu2(cpu: &mut Cpu) {
    cpu.pc = 0x5ed8;
    cpu.a = cpu.read_byte(crate::defs::R_WBK as u16);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.write_byte(0xcd8c, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    battle_tower_room_menu2_run_jumptable(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xcd8c);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xffd4, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function119f3f(cpu: &mut Cpu) {
    cpu.pc = 0x5f3f;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11a5b9(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(16);
    become battle_tower_room_menu2_increment_jumptable(cpu);
}

pub(crate) fn function119f45(cpu: &mut Cpu) {
    cpu.pc = 0x5f45;
    cpu.set_hl({ const N: i32 = ((((2 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (4 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0x6661);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::place_string(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x80;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd44, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(16);
    become battle_tower_room_menu2_increment_jumptable(cpu);
}

pub(crate) fn function119f56(cpu: &mut Cpu) {
    cpu.pc = 0x5f56;
    cpu.a = cpu.read_byte(0xcd44);
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
        become function119f56_asm_119f62(cpu);
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
    cpu.write_byte(0xcd44, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = true;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function119f56_asm_119f62(cpu: &mut Cpu) {
    cpu.pc = 0x5f62;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11a63c(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = ((((2 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (4 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0x66aa);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::place_string(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x80;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd44, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(16);
    become battle_tower_room_menu2_increment_jumptable(cpu);
}

pub(crate) fn function119f76(cpu: &mut Cpu) {
    cpu.pc = 0x5f76;
    cpu.a = cpu.read_byte(0xcd44);
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
        become function119f76_asm_119f82(cpu);
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
    cpu.write_byte(0xcd44, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = true;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function119f76_asm_119f82(cpu: &mut Cpu) {
    cpu.pc = 0x5f82;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11a63c(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = ((((2 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (4 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0x6679);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::place_string(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11a5f5(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcd44, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(16);
    become battle_tower_room_menu2_increment_jumptable(cpu);
}

pub(crate) fn function119f98(cpu: &mut Cpu) {
    cpu.pc = 0x5f98;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11a536(cpu);
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
    crate::game::home::menu::play_click_sfx(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xcd44);
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
        become function119f98_asm_119fef(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::menu::exit_menu(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11a63c(cpu);
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
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11a00e(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xc2dd);
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
        become function119f98_asm_119fd4(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::menu::exit_menu(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x41;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x4061);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x45;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x5dc3);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xcd33);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xcf66, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0xa;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc300, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = true;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function119f98_asm_119fd4(cpu: &mut Cpu) {
    cpu.pc = 0x5fd4;
    cpu.set_hl({ const N: i32 = ((((2 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (4 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0x6692);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::place_string(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc30d, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc314, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x41;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x4061);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
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

pub(crate) fn function119f98_asm_119fef(cpu: &mut Cpu) {
    cpu.pc = 0x5fef;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::menu::exit_menu(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::menu::exit_menu(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x41;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x4061);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xcd45);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xcf66, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x45;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x5dc3);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0xa;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc300, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = true;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function11a00e(cpu: &mut Cpu) {
    cpu.pc = 0x600e;
    cpu.a = 0x05;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xaa4b);
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
        become function11a00e_asm_11a02a(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte({ const N: i32 = (0xaa4b as i32) + (1 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::close_sram(cpu);
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
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 0x05;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xaa4b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    become function11a00e_asm_11a02a(cpu);
}

pub(crate) fn function11a00e_asm_11a02a(cpu: &mut Cpu) {
    cpu.pc = 0x602a;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::close_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xcd48);
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
        become function11a00e_asm_11a039(cpu);
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
    if cpu.flag_z {
        cpu.cycle(12);
        become function11a00e_asm_11a081(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become function11a0ca(cpu);
}

pub(crate) fn function11a00e_asm_11a039(cpu: &mut Cpu) {
    cpu.pc = 0x6039;
    cpu.a = 0x03;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.set_hl(0xc608);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xd800);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(246 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map::fade_to_menu(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x45;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x765d);
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
    function11a9ce(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x03;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.set_hl(0xd800);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xc608);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(246 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = 0x45;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x5d99);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.c = 0x0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x45;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x5e18);
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
    cpu.write_byte(0xc305, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function11a00e_asm_11a081(cpu: &mut Cpu) {
    cpu.pc = 0x6081;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcf83, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xcf82, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x13;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcf85, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x5;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcf84, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::menu::push_window(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x45;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x765d);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x45;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x7ab4);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x41;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x6462);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x41;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x6464);
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
    crate::game::home::menu::exit_menu(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x41;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x4061);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x45;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x5d99);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.c = 0x0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x45;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x5e18);
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
    cpu.write_byte(0xc305, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function11a0ca(cpu: &mut Cpu) {
    cpu.pc = 0x60ca;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcf83, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xcf82, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x13;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcf85, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x11;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcf84, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::menu::push_window(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x45;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x765d);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x5f;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x53f6);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x41;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x6462);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x41;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x6464);
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
    crate::game::home::menu::exit_menu(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x41;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x4061);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x45;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x5d99);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.c = 0x0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x45;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x5e18);
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
    cpu.write_byte(0xc305, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function11a113(cpu: &mut Cpu) {
    cpu.pc = 0x6113;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11a63c(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.c = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x45;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x5e18);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = ((((2 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (4 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0x66c8);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::place_string(cpu);
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

pub(crate) fn function11a129(cpu: &mut Cpu) {
    cpu.pc = 0x6129;
    cpu.a = 0x80;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd44, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(16);
    become battle_tower_room_menu2_increment_jumptable(cpu);
}

pub(crate) fn function11a131(cpu: &mut Cpu) {
    cpu.pc = 0x6131;
    cpu.set_hl(0xcd44);
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
    cpu.a = cpu.read_byte(0xcd3c);
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
    cpu.write_byte(0xcd3c, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    become function11a13d(cpu);
}

pub(crate) fn function11a13d(cpu: &mut Cpu) {
    cpu.pc = 0x613d;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11a63c(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = ((((2 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (4 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0x66d2);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::place_string(cpu);
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

pub(crate) fn function11a14b(cpu: &mut Cpu) {
    cpu.pc = 0x614b;
    cpu.set_hl(0xcd85);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0xf3;
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
        become function11a14b_asm_11a155(cpu);
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

pub(crate) fn function11a14b_asm_11a155(cpu: &mut Cpu) {
    cpu.pc = 0x6155;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11a1d6(cpu);
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
    function11a63c(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = ((((2 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (4 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0x66f1);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::place_string(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x80;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd44, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(16);
    become battle_tower_room_menu2_increment_jumptable(cpu);
}

pub(crate) fn function11a16d(cpu: &mut Cpu) {
    cpu.pc = 0x616d;
    cpu.a = cpu.read_byte(0xcd44);
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
        become function11a16d_asm_11a179(cpu);
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
    cpu.write_byte(0xcd44, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = true;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function11a16d_asm_11a179(cpu: &mut Cpu) {
    cpu.pc = 0x6179;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11a63c(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11a1e6(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = ((((2 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (4 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xc346);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::place_string(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11a5f5(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcd44, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(16);
    become battle_tower_room_menu2_increment_jumptable(cpu);
}

pub(crate) fn function11a192(cpu: &mut Cpu) {
    cpu.pc = 0x6192;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11a536(cpu);
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
    crate::game::home::menu::play_click_sfx(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xcd44);
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
        become function11a192_asm_11a1b6(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::menu::exit_menu(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x41;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x4061);
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
    function11a63c(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = ((((2 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (4 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0x66d2);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::place_string(cpu);
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

pub(crate) fn function11a192_asm_11a1b6(cpu: &mut Cpu) {
    cpu.pc = 0x61b6;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::menu::exit_menu(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x41;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x4061);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xcd45);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xcf66, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xcd80, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11a63c(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = ((((2 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (4 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0x66d2);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::place_string(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = true;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function11a1d6(cpu: &mut Cpu) {
    cpu.pc = 0x61d6;
    cpu.a = cpu.read_byte(0xcd85);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x50;
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
        become function11a1d6_asm_11a1e4(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0xd3;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    set_mobile_error_code(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = true;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function11a1d6_asm_11a1e4(cpu: &mut Cpu) {
    cpu.pc = 0x61e4;
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

pub(crate) fn function11a1e6(cpu: &mut Cpu) {
    cpu.pc = 0x61e6;
    cpu.set_hl(0x6706);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xc346);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11a1ff(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xcd85);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11a1ff(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0x670b);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11a1ff(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x50;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function11a1ff(cpu: &mut Cpu) {
    cpu.pc = 0x61ff;
    become function11a1ff_asm_11a1ff(cpu);
}

pub(crate) fn function11a1ff_asm_11a1ff(cpu: &mut Cpu) {
    cpu.pc = 0x61ff;
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0x50;
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
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.cycle(12);
    become function11a1ff_asm_11a1ff(cpu);
}

pub(crate) fn battle_tower_room_menu2_place_yes_no_menu(cpu: &mut Cpu) {
    cpu.pc = 0x6207;
    cpu.set_hl(0x62de);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::menu::load_menu_header(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::menu::menu_box(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::menu::menu_box_coord2_tile(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::tilemap::apply_tilemap(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = ((((8 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (16 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0x62cf);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::place_string(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = ((((10 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (16 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0x62d3);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::place_string(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = ((((8 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (15 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0xed;
    cpu.pc += 2;
    cpu.cycle(8);
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
    cpu.write_byte(0xcd44, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(16);
    become battle_tower_room_menu2_increment_jumptable(cpu);
}

pub(crate) fn battle_tower_room_menu2_update_yes_no_menu(cpu: &mut Cpu) {
    cpu.pc = 0x6235;
    cpu.set_hl(0xffa7);
    cpu.pc += 3;
    cpu.cycle(12);
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
        become battle_tower_room_menu2_update_yes_no_menu_a_button(cpu);
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
        become battle_tower_room_menu2_update_yes_no_menu_b_button(cpu);
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
        become battle_tower_room_menu2_update_yes_no_menu_d_up(cpu);
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
        become battle_tower_room_menu2_update_yes_no_menu_d_down(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become battle_tower_room_menu2_update_yes_no_menu_asm_11a24c(cpu);
}

pub(crate) fn battle_tower_room_menu2_update_yes_no_menu_asm_11a24c(cpu: &mut Cpu) {
    cpu.pc = 0x624c;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11a9f0(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = true;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn battle_tower_room_menu2_update_yes_no_menu_d_up(cpu: &mut Cpu) {
    cpu.pc = 0x6251;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcd8a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xcd8b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xcd44);
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
        become battle_tower_room_menu2_update_yes_no_menu_asm_11a24c(cpu);
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
    cpu.write_byte(0xcd44, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl({ const N: i32 = ((((8 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (15 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0xed;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl({ const N: i32 = ((((10 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (15 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0x7f;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.cycle(12);
    become battle_tower_room_menu2_update_yes_no_menu_asm_11a24c(cpu);
}

pub(crate) fn battle_tower_room_menu2_update_yes_no_menu_d_down(cpu: &mut Cpu) {
    cpu.pc = 0x6270;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcd8a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xcd8b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xcd44);
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
        become battle_tower_room_menu2_update_yes_no_menu_asm_11a24c(cpu);
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
    cpu.write_byte(0xcd44, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl({ const N: i32 = ((((8 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (15 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0x7f;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl({ const N: i32 = ((((10 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (15 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0xed;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.cycle(12);
    become battle_tower_room_menu2_update_yes_no_menu_asm_11a24c(cpu);
}

pub(crate) fn battle_tower_room_menu2_update_yes_no_menu_a_button(cpu: &mut Cpu) {
    cpu.pc = 0x628f;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcd8a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xcd8b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::menu::play_click_sfx(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xcd44);
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
        become battle_tower_room_menu2_update_yes_no_menu_exit_no_carry(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::menu::exit_menu(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x41;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x4061);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xcd46);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x0;
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
        become battle_tower_room_menu2_update_yes_no_menu_asm_11a2b4(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xcd47);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(12);
    become battle_tower_room_menu2_update_yes_no_menu_exit_carry(cpu);
}

pub(crate) fn battle_tower_room_menu2_update_yes_no_menu_asm_11a2b4(cpu: &mut Cpu) {
    cpu.pc = 0x62b4;
    cpu.a = cpu.read_byte(0xcd33);
    cpu.pc += 3;
    cpu.cycle(16);
    become battle_tower_room_menu2_update_yes_no_menu_exit_carry(cpu);
}

pub(crate) fn battle_tower_room_menu2_update_yes_no_menu_exit_carry(cpu: &mut Cpu) {
    cpu.pc = 0x62b7;
    cpu.write_byte(0xcf66, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0xa;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc300, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = true;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn battle_tower_room_menu2_update_yes_no_menu_b_button(cpu: &mut Cpu) {
    cpu.pc = 0x62c1;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::menu::play_click_sfx(cpu);
    assert_eq!(cpu.pc, next_pc);
    become battle_tower_room_menu2_update_yes_no_menu_exit_no_carry(cpu);
}

pub(crate) fn battle_tower_room_menu2_update_yes_no_menu_exit_no_carry(cpu: &mut Cpu) {
    cpu.pc = 0x62c4;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::menu::exit_menu(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x41;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x4061);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
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

pub(crate) fn function11a2e6(cpu: &mut Cpu) {
    cpu.pc = 0x62e6;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11a63c(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.c = 0x2;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x45;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x5e18);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = ((((2 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (4 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0x671e);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::place_string(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x80;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd44, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(16);
    become battle_tower_room_menu2_increment_jumptable(cpu);
}

pub(crate) fn function11a302(cpu: &mut Cpu) {
    cpu.pc = 0x6302;
    cpu.a = cpu.read_byte(0xcd44);
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
        become function11a302_asm_11a30e(cpu);
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
    cpu.write_byte(0xcd44, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = true;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function11a302_asm_11a30e(cpu: &mut Cpu) {
    cpu.pc = 0x630e;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11a63c(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = ((((2 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (4 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0x672a);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::place_string(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = ((((4 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (9 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xcd68);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(0x8102);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::print_text::print_num(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = ((((4 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (14 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xcd67);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(0x8102);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::print_text::print_num(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x80;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd44, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(16);
    become battle_tower_room_menu2_increment_jumptable(cpu);
}

pub(crate) fn function11a33a(cpu: &mut Cpu) {
    cpu.pc = 0x633a;
    cpu.a = cpu.read_byte(0xcd44);
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
        become function11a33a_asm_11a346(cpu);
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
    cpu.write_byte(0xcd44, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = true;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function11a33a_asm_11a346(cpu: &mut Cpu) {
    cpu.pc = 0x6346;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::menu::exit_menu(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x41;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x4061);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x45;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x5dc3);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
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

pub(crate) fn function11a357(cpu: &mut Cpu) {
    cpu.pc = 0x6357;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11a63c(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = ((((2 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (4 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0x6743);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::place_string(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x80;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd44, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(16);
    become battle_tower_room_menu2_increment_jumptable(cpu);
}

pub(crate) fn function11a36b(cpu: &mut Cpu) {
    cpu.pc = 0x636b;
    cpu.a = cpu.read_byte(0xcd44);
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
        become function11a36b_asm_11a377(cpu);
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
    cpu.write_byte(0xcd44, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = true;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function11a36b_asm_11a377(cpu: &mut Cpu) {
    cpu.pc = 0x6377;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11a63c(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = ((((2 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (4 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0x6755);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::place_string(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11a5f5(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcd44, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(16);
    become battle_tower_room_menu2_increment_jumptable(cpu);
}

pub(crate) fn function11a38d(cpu: &mut Cpu) {
    cpu.pc = 0x638d;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11a536(cpu);
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
    crate::game::home::menu::play_click_sfx(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xcd44);
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
        become function11a38d_asm_11a3b1(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::menu::exit_menu(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x41;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x4061);
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
    function11a63c(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = ((((2 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (4 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0x66d2);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::place_string(cpu);
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

pub(crate) fn function11a38d_asm_11a3b1(cpu: &mut Cpu) {
    cpu.pc = 0x63b1;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::menu::exit_menu(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x41;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x4061);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xcd45);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xcf66, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xcd80, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = true;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function11a3c5(cpu: &mut Cpu) {
    cpu.pc = 0x63c5;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11a63c(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = ((((2 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (4 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0x6762);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::place_string(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x80;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd44, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(16);
    become battle_tower_room_menu2_increment_jumptable(cpu);
}

pub(crate) fn function11a3d9(cpu: &mut Cpu) {
    cpu.pc = 0x63d9;
    cpu.a = cpu.read_byte(0xcd44);
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
        become function11a3d9_asm_11a3e5(cpu);
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
    cpu.write_byte(0xcd44, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = true;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function11a3d9_asm_11a3e5(cpu: &mut Cpu) {
    cpu.pc = 0x63e5;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11a63c(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = ((((2 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (4 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0x6779);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::place_string(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x80;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd44, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(16);
    become battle_tower_room_menu2_increment_jumptable(cpu);
}

pub(crate) fn function11a3f9(cpu: &mut Cpu) {
    cpu.pc = 0x63f9;
    cpu.a = cpu.read_byte(0xcd44);
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
        become function11a3f9_asm_11a405(cpu);
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
    cpu.write_byte(0xcd44, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = true;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function11a3f9_asm_11a405(cpu: &mut Cpu) {
    cpu.pc = 0x6405;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11a63c(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = ((((2 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (4 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0x6755);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::place_string(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11a5f5(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcd44, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(16);
    become battle_tower_room_menu2_increment_jumptable(cpu);
}

pub(crate) fn function11a41b(cpu: &mut Cpu) {
    cpu.pc = 0x641b;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11a536(cpu);
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
    crate::game::home::menu::play_click_sfx(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xcd44);
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
        become function11a41b_asm_11a43f(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::menu::exit_menu(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x41;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x4061);
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
    function11a63c(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = ((((2 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (4 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0x66d2);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::place_string(cpu);
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

pub(crate) fn function11a41b_asm_11a43f(cpu: &mut Cpu) {
    cpu.pc = 0x643f;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::menu::exit_menu(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x41;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x4061);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x1c;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcf66, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xcd80, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = true;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function11a452(cpu: &mut Cpu) {
    cpu.pc = 0x6452;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11a63c(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = ((((2 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (4 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0x6791);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::place_string(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x80;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd44, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(16);
    become battle_tower_room_menu2_increment_jumptable(cpu);
}

pub(crate) fn function11a466(cpu: &mut Cpu) {
    cpu.pc = 0x6466;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11a63c(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = ((((2 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (4 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0x67c1);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::place_string(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x80;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd44, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(16);
    become battle_tower_room_menu2_increment_jumptable(cpu);
}

pub(crate) fn function11a47a(cpu: &mut Cpu) {
    cpu.pc = 0x647a;
    cpu.a = cpu.read_byte(0xcd44);
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
        become function11a47a_asm_11a486(cpu);
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
    cpu.write_byte(0xcd44, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = true;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function11a47a_asm_11a486(cpu: &mut Cpu) {
    cpu.pc = 0x6486;
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

pub(crate) fn function11a488(cpu: &mut Cpu) {
    cpu.pc = 0x6488;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11a63c(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = ((((2 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (4 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0x67ac);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::place_string(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11a5f5(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcd44, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(16);
    become battle_tower_room_menu2_increment_jumptable(cpu);
}

pub(crate) fn function11a49e(cpu: &mut Cpu) {
    cpu.pc = 0x649e;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11a536(cpu);
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
    crate::game::home::menu::play_click_sfx(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xcd44);
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
        become function11a49e_asm_11a4c7(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::menu::exit_menu(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x41;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x4061);
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
    function11a63c(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = ((((2 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (4 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0x66d2);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::place_string(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x14;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcf66, cpu.a);
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
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function11a49e_asm_11a4c7(cpu: &mut Cpu) {
    cpu.pc = 0x64c7;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::menu::exit_menu(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x41;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x4061);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xcd46);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xcf66, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xcd80, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = true;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function11a4db(cpu: &mut Cpu) {
    cpu.pc = 0x64db;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11a63c(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_de(0x66db);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl({ const N: i32 = ((((2 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (4 as i32)) + (0xc4a0 as i32)); N as u16 });
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

pub(crate) fn function11a4e8(cpu: &mut Cpu) {
    cpu.pc = 0x64e8;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11a63c(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = ((((2 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (4 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0x67d7);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::place_string(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11a5f5(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcd44, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(16);
    become battle_tower_room_menu2_increment_jumptable(cpu);
}

pub(crate) fn function11a4fe(cpu: &mut Cpu) {
    cpu.pc = 0x64fe;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11a536(cpu);
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
    crate::game::home::menu::play_click_sfx(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xcd44);
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
        become function11a4fe_asm_11a522(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::menu::exit_menu(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x41;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x4061);
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
    function11a63c(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = ((((2 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (4 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0x66d2);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::place_string(cpu);
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

pub(crate) fn function11a4fe_asm_11a522(cpu: &mut Cpu) {
    cpu.pc = 0x6522;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::menu::exit_menu(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x41;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x4061);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xcd45);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xcf66, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xcd80, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = true;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function11a536(cpu: &mut Cpu) {
    cpu.pc = 0x6536;
    cpu.set_hl(0xffa7);
    cpu.pc += 3;
    cpu.cycle(12);
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
        become function11a536_asm_11a5a7(cpu);
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
        become function11a536_asm_11a5a2(cpu);
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
        become function11a536_asm_11a564(cpu);
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
        become function11a536_asm_11a583(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become function11a536_asm_11a54d(cpu);
}

pub(crate) fn function11a536_asm_11a54d(cpu: &mut Cpu) {
    cpu.pc = 0x654d;
    cpu.a = cpu.read_byte(0xcd3c);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x4;
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
        become function11a536_asm_11a562(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11a9f0(cpu);
    assert_eq!(cpu.pc, next_pc);
    if !cpu.flag_z {
        cpu.cycle(12);
        become function11a536_asm_11a562(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::menu::exit_menu(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x41;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x4061);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    become function11a536_asm_11a562(cpu);
}

pub(crate) fn function11a536_asm_11a562(cpu: &mut Cpu) {
    cpu.pc = 0x6562;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = true;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function11a536_asm_11a564(cpu: &mut Cpu) {
    cpu.pc = 0x6564;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcd8a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xcd8b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xcd44);
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
        become function11a536_asm_11a54d(cpu);
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
    cpu.write_byte(0xcd44, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl({ const N: i32 = ((((7 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (15 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0xed;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl({ const N: i32 = ((((9 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (15 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0x7f;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.cycle(12);
    become function11a536_asm_11a54d(cpu);
}

pub(crate) fn function11a536_asm_11a583(cpu: &mut Cpu) {
    cpu.pc = 0x6583;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcd8a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xcd8b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xcd44);
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
        become function11a536_asm_11a54d(cpu);
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
    cpu.write_byte(0xcd44, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl({ const N: i32 = ((((7 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (15 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0x7f;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl({ const N: i32 = ((((9 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (15 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0xed;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.cycle(12);
    become function11a536_asm_11a54d(cpu);
}

pub(crate) fn function11a536_asm_11a5a2(cpu: &mut Cpu) {
    cpu.pc = 0x65a2;
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd44, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    become function11a536_asm_11a5a7(cpu);
}

pub(crate) fn function11a536_asm_11a5a7(cpu: &mut Cpu) {
    cpu.pc = 0x65a7;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcd8a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xcd8b, cpu.a);
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
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn battle_tower_room_menu2_increment_jumptable(cpu: &mut Cpu) {
    cpu.pc = 0x65b0;
    cpu.a = cpu.read_byte(0xcd3c);
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
    cpu.write_byte(0xcd3c, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = true;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function11a5b9(cpu: &mut Cpu) {
    cpu.pc = 0x65b9;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcf83, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xcf82, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x13;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcf85, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x5;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcf84, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::menu::push_window(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = ((((0 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (0 as i32)) + (0xcdd9 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = 0x6;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.c = 0x14;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl({ const N: i32 = ((((0 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (0 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = 0x4;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.c = 0x12;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::mobile::function3eea(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x41;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x4061);
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
    crate::game::home::map_objects::update_sprites(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.c = 0x0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x45;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x5e18);
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
    cpu.write_byte(0xc305, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function11a5f5(cpu: &mut Cpu) {
    cpu.pc = 0x65f5;
    cpu.a = 0xe;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcf83, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x13;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcf85, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x6;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcf82, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0xa;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcf84, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::menu::push_window(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = ((((6 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (14 as i32)) + (0xcdd9 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = 0x5;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.c = 0x6;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl({ const N: i32 = ((((6 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (14 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = 0x3;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.c = 0x4;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::mobile::function3eea(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = ((((7 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (16 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0x62cf);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::place_string(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = ((((9 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (16 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0x62d3);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::place_string(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = ((((7 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (15 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0xed;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 0x41;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x4061);
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

pub(crate) fn function11a63c(cpu: &mut Cpu) {
    cpu.pc = 0x663c;
    cpu.set_hl({ const N: i32 = ((((1 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (4 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0x67f4);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::place_string(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = ((((2 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (4 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0x67f4);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::place_string(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = ((((3 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (4 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0x67f4);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::place_string(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = ((((4 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (4 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0x67f4);
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

pub(crate) fn function11a80c(cpu: &mut Cpu) {
    cpu.pc = 0x680c;
    cpu.set_de(0xffb7);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(0xffb3);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0x689a);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11a88c(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_bc({ const N: i32 = (0xffb3 as i32) + (1 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0x68ba);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11a88c(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_bc({ const N: i32 = (0xffb3 as i32) + (2 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0x68da);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11a88c(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xffb7);
    cpu.pc += 2;
    cpu.cycle(12);
    let value = 0xf;
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
    cpu.a = cpu.read_byte({ const N: i32 = (0xffb3 as i32) + (6 as i32); N as u16 });
    cpu.pc += 2;
    cpu.cycle(12);
    let value = 0xf;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11a884(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte({ const N: i32 = (0xffb3 as i32) + (8 as i32); N as u16 });
    cpu.pc += 2;
    cpu.cycle(12);
    let value = 0xf;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11a884(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.write_byte(0xcd62, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.e = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xffb7);
    cpu.pc += 2;
    cpu.cycle(12);
    let value = 0xf0;
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
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11a884(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte({ const N: i32 = (0xffb3 as i32) + (6 as i32); N as u16 });
    cpu.pc += 2;
    cpu.cycle(12);
    let value = 0xf0;
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
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11a884(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte({ const N: i32 = (0xffb3 as i32) + (8 as i32); N as u16 });
    cpu.pc += 2;
    cpu.cycle(12);
    let value = 0xf0;
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
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11a884(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.write_byte(0xcd63, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.e = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xffb8);
    cpu.pc += 2;
    cpu.cycle(12);
    let value = 0xf;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11a884(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte({ const N: i32 = (0xffb3 as i32) + (7 as i32); N as u16 });
    cpu.pc += 2;
    cpu.cycle(12);
    let value = 0xf;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11a884(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte({ const N: i32 = (0xffb3 as i32) + (9 as i32); N as u16 });
    cpu.pc += 2;
    cpu.cycle(12);
    let value = 0xf;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11a884(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.write_byte(0xcd64, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function11a884(cpu: &mut Cpu) {
    cpu.pc = 0x6884;
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
    let value = 0xa;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_c {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0xa;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.b;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.b = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function11a88c(cpu: &mut Cpu) {
    cpu.pc = 0x688c;
    cpu.a = cpu.read_byte(cpu.bc());
    cpu.pc += 1;
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
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
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
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function11a90f(cpu: &mut Cpu) {
    cpu.pc = 0x690f;
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::speech_textbox(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x50;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xc320);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(0x008c as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::byte_fill(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xc31b);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xc31c);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.h = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_de(0xc320);
    cpu.pc += 3;
    cpu.cycle(12);
    become function11a90f_asm_11a92c(cpu);
}

pub(crate) fn function11a90f_asm_11a92c(cpu: &mut Cpu) {
    cpu.pc = 0x692c;
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0x57;
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
        become function11a90f_asm_11a94f(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x0;
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
        become function11a90f_asm_11a92c(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x50;
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
        become function11a90f_asm_11a92c(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
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
        become function11a90f_asm_11a941(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.cycle(12);
    become function11a90f_asm_11a92c(cpu);
}

pub(crate) fn function11a90f_asm_11a941(cpu: &mut Cpu) {
    cpu.pc = 0x6941;
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    become function11a90f_asm_11a945(cpu);
}

pub(crate) fn function11a90f_asm_11a945(cpu: &mut Cpu) {
    cpu.pc = 0x6945;
    cpu.a = cpu.read_byte(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_bc(cpu.bc().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0x50;
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
        become function11a90f_asm_11a92c(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.cycle(12);
    become function11a90f_asm_11a945(cpu);
}

pub(crate) fn function11a90f_asm_11a94f(cpu: &mut Cpu) {
    cpu.pc = 0x694f;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc31f, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = ((0xc320u32) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc31b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = (((0xc320u32) >> 8) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc31c, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl({ const N: i32 = ((((14 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (1 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc31d, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc31e, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xc31a);
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
    cpu.a = 0x3;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    become battle_tower_room_menu_write_message_do_nothing(cpu);
}

pub(crate) fn battle_tower_room_menu_write_message_do_nothing(cpu: &mut Cpu) {
    cpu.pc = 0x6970;
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function11a971(cpu: &mut Cpu) {
    cpu.pc = 0x6971;
    cpu.set_hl(0xc31f);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xffa8);
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
        become function11a971_asm_11a97f(cpu);
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
        become function11a971_asm_11a97f(cpu);
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

pub(crate) fn function11a971_asm_11a97f(cpu: &mut Cpu) {
    cpu.pc = 0x697f;
    cpu.a = cpu.read_byte(0xcfcc);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x7;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(0xcd8d);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xc31b);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xc31c);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.d = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
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
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc31b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc31c, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x50;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc31d);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xc31e);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.h = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_de(0xcd8d);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::place_string(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc31d, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc31e, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xcd8d);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x50;
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
        become function11a971_asm_11a9bf(cpu);
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
    cpu.write_byte(0xc31a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    become function11a971_asm_11a9bf(cpu);
}

pub(crate) fn function11a971_asm_11a9bf(cpu: &mut Cpu) {
    cpu.pc = 0x69bf;
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn battle_tower_room_menu_set_message(cpu: &mut Cpu) {
    cpu.pc = 0x69c0;
    cpu.a = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc31b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc31c, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc31a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function11a9ce(cpu: &mut Cpu) {
    cpu.pc = 0x69ce;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::tilemap::clear_bg_palettes(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map::reload_tileset_and_palettes(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::menu::call_exit_menu(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x41;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x6462);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x41;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x6464);
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
    crate::game::home::palettes::gs_reload_palettes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x00;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x2b5c);
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
    crate::game::home::map_objects::update_sprites(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function11a9f0(cpu: &mut Cpu) {
    cpu.pc = 0x69f0;
    cpu.a = 0x1;
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

pub(crate) fn function11a9f4(cpu: &mut Cpu) {
    cpu.pc = 0x69f4;
    cpu.a = cpu.read_byte(0xcd8a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xcd8b);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.h = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcd8a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcd8b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_de(0xd5d0 as u16);
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
    let mask = 1u8 << ((7) & 7);
    cpu.flag_z = (cpu.h & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 0xd6;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    set_mobile_error_code(cpu);
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

pub(crate) fn function11ac3e(cpu: &mut Cpu) {
    cpu.pc = 0x6c3e;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::speech_textbox(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map::fade_to_menu(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0x6814);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0x23;
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
    function11ac51(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map::close_submenu(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function11ac51(cpu: &mut Cpu) {
    cpu.pc = 0x6c51;
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
    cpu.set_hl(0xcfcc);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    let mask = 1u8 << ((4) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value |= mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xd0ed);
    cpu.pc += 3;
    cpu.cycle(16);
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
    cpu.write_byte(0xd0ed, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
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
    cpu.write_byte(0xffde, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.write_byte(0xcd49, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xcd4a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xcd4c, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xcd4d, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xcd4e, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11ad1b(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::delay::delay_frame(cpu);
    assert_eq!(cpu.pc, next_pc);
    become function11ac51_loop(cpu);
}

pub(crate) fn function11ac51_loop(cpu: &mut Cpu) {
    cpu.pc = 0x6c82;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::joypad::joy_text_delay(cpu);
    assert_eq!(cpu.pc, next_pc);
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
        become function11ac51_asm_11aca8(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11b314(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11acb7(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11ad6e(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = { let value = ((30 as i32) * (crate::defs::OBJ_SIZE as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc3b5, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x23;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x4f7a);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x41;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x4061);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(12);
    become function11ac51_loop(cpu);
}

pub(crate) fn function11ac51_asm_11aca8(cpu: &mut Cpu) {
    cpu.pc = 0x6ca8;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::clear_sprites::clear_sprites(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte(0xffaa, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte(0xd0ed, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte(0xcfcc, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function11acb7(cpu: &mut Cpu) {
    cpu.pc = 0x6cb7;
    cpu.set_hl(0x7a44);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xcd49);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.b = 0;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.c;
    let result = value << 1;
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x80 != 0;
    cpu.c = result;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.b;
    let carry_in = u8::from(cpu.flag_c);
    let result = (value << 1) | carry_in;
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x80 != 0;
    cpu.b = result;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.c;
    let result = value << 1;
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x80 != 0;
    cpu.c = result;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.b;
    let carry_in = u8::from(cpu.flag_c);
    let result = (value << 1) | carry_in;
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x80 != 0;
    cpu.b = result;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.c;
    let result = value << 1;
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x80 != 0;
    cpu.c = result;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.b;
    let carry_in = u8::from(cpu.flag_c);
    let result = (value << 1) | carry_in;
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x80 != 0;
    cpu.b = result;
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
    cpu.set_de({ const N: i32 = ((((6 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (6 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de({ const N: i32 = ((((7 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (0 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(7 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xcd49);
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
    cpu.write_byte(0xcd49, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0xff;
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
        become function11acb7_get_the_other(cpu);
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
    cpu.write_byte(0xcd49, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    become function11acb7_get_the_other(cpu);
}

pub(crate) fn function11acb7_get_the_other(cpu: &mut Cpu) {
    cpu.pc = 0x6ceb;
    cpu.set_hl(0x7b7d);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xcd4a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.b = 0;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.c;
    let result = value << 1;
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x80 != 0;
    cpu.c = result;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.b;
    let carry_in = u8::from(cpu.flag_c);
    let result = (value << 1) | carry_in;
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x80 != 0;
    cpu.b = result;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.c;
    let result = value << 1;
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x80 != 0;
    cpu.c = result;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.b;
    let carry_in = u8::from(cpu.flag_c);
    let result = (value << 1) | carry_in;
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x80 != 0;
    cpu.b = result;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.c;
    let result = value << 1;
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x80 != 0;
    cpu.c = result;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.b;
    let carry_in = u8::from(cpu.flag_c);
    let result = (value << 1) | carry_in;
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x80 != 0;
    cpu.b = result;
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
    cpu.set_de({ const N: i32 = ((((9 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (3 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(7 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xcd4a);
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
    cpu.write_byte(0xcd4a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0xff;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcd4a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function11ad1b(cpu: &mut Cpu) {
    cpu.pc = 0x6d1b;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::tilemap::clear_bg_palettes(cpu);
    assert_eq!(cpu.pc, next_pc);
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
    cpu.a = 0x5f;
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
    cpu.a = cpu.read_byte(0xcfa9);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xcd82, cpu.a);
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
    cpu.write_byte(0xffb0, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = 0x10;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc3b7, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0x683f);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0x23;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.e = crate::defs::MONICON_MOBILE1 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0x683f);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0x23;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.e = crate::defs::MONICON_MOBILE2 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xc6d0);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(0x0115 as u16);
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
    cpu.write_byte(0xcd30, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = crate::defs::DEXMODE_ABC as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc7d4, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x10;
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

pub(crate) fn mobile_inc_jumptable_index(cpu: &mut Cpu) {
    cpu.pc = 0x6d8a;
    cpu.set_hl(0xcf63);
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

pub(crate) fn function11ad8f(cpu: &mut Cpu) {
    cpu.pc = 0x6d8f;
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

pub(crate) fn function11ad95(cpu: &mut Cpu) {
    cpu.pc = 0x6d95;
    cpu.set_hl(0x6e38);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::menu::load_menu_header(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::menu::menu_box(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = ((((12 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (12 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0x6e40);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::place_string(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = ((((10 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (10 as i32)) + (0xcdd9 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(0x0808);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11afd6(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x41;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x4061);
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
    mobile_inc_jumptable_index(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcfa9, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0x6fcc);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11afb7(cpu);
    assert_eq!(cpu.pc, next_pc);
    become function11adc4(cpu);
}

pub(crate) fn function11adc4(cpu: &mut Cpu) {
    cpu.pc = 0x6dc4;
    cpu.set_hl(0xffa7);
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
        become function11adc4_asm_11ade6(cpu);
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
        become function11adc4_asm_11aded(cpu);
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
        become function11adc4_asm_11ae06(cpu);
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
    crate::game::home::menu::play_click_sfx(cpu);
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
    cpu.cycle(12);
    become function11adc4_asm_11ae2e(cpu);
}

pub(crate) fn function11adc4_asm_11ade6(cpu: &mut Cpu) {
    cpu.pc = 0x6de6;
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
    if cpu.flag_z {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.cycle(12);
    become function11adc4_asm_11adf4(cpu);
}

pub(crate) fn function11adc4_asm_11aded(cpu: &mut Cpu) {
    cpu.pc = 0x6ded;
    cpu.a = cpu.read_byte(0xcfa9);
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
    let value = 0x4;
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
    become function11adc4_asm_11adf4(cpu);
}

pub(crate) fn function11adc4_asm_11adf4(cpu: &mut Cpu) {
    cpu.pc = 0x6df4;
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.set_hl(0x6fcc);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11afbb(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte(0xcfa9, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0x6fcc);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11afb7(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function11adc4_asm_11ae06(cpu: &mut Cpu) {
    cpu.pc = 0x6e06;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::menu::play_click_sfx(cpu);
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
    cpu.set_hl(0xcd30);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.write_byte(cpu.hl(), cpu.a);
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
        become function11adc4_asm_11ae28(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl({ const N: i32 = ((((14 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (2 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xcfa9);
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
    if cpu.flag_z {
        cpu.cycle(12);
        become function11adc4_asm_11ae23(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11b272(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(12);
    become function11adc4_asm_11ae2b(cpu);
}

pub(crate) fn function11adc4_asm_11ae23(cpu: &mut Cpu) {
    cpu.pc = 0x6e23;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11b267(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(12);
    become function11adc4_asm_11ae2b(cpu);
}

pub(crate) fn function11adc4_asm_11ae28(cpu: &mut Cpu) {
    cpu.pc = 0x6e28;
    cpu.a = 0x3;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    become function11adc4_asm_11ae2b(cpu);
}

pub(crate) fn function11adc4_asm_11ae2b(cpu: &mut Cpu) {
    cpu.pc = 0x6e2b;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_inc_jumptable_index(cpu);
    assert_eq!(cpu.pc, next_pc);
    become function11adc4_asm_11ae2e(cpu);
}

pub(crate) fn function11adc4_asm_11ae2e(cpu: &mut Cpu) {
    cpu.pc = 0x6e2e;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::menu::exit_menu(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x41;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x4061);
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

pub(crate) fn function11ae4e(cpu: &mut Cpu) {
    cpu.pc = 0x6e4e;
    cpu.set_hl(0x6fe8);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::menu::load_menu_header(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::menu::menu_box(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = ((((14 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (10 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0x6ff0);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::place_string(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0x7013);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::menu::load_menu_header(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::menu::menu_box(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = ((((8 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (16 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0x701b);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::place_string(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = ((((7 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (14 as i32)) + (0xcdd9 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(0x0506);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11afd6(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = ((((12 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (9 as i32)) + (0xcdd9 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(0x060b);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11afd6(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x41;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x4061);
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
    mobile_inc_jumptable_index(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcfa9, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0x6fd2);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11afb7(cpu);
    assert_eq!(cpu.pc, next_pc);
    become function11ae98(cpu);
}

pub(crate) fn function11ae98(cpu: &mut Cpu) {
    cpu.pc = 0x6e98;
    cpu.set_hl(0xffa7);
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
        become function11ae98_asm_11aec1(cpu);
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
        become function11ae98_asm_11aec8(cpu);
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
        become function11ae98_asm_11aee1(cpu);
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
    crate::game::home::menu::play_click_sfx(cpu);
    assert_eq!(cpu.pc, next_pc);
    become function11ae98_asm_11aeb4(cpu);
}

pub(crate) fn function11ae98_asm_11aeb4(cpu: &mut Cpu) {
    cpu.pc = 0x6eb4;
    cpu.set_hl({ const N: i32 = ((((14 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (2 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0x7f;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcf63, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(12);
    become function11ae98_asm_11aef7(cpu);
}

pub(crate) fn function11ae98_asm_11aec1(cpu: &mut Cpu) {
    cpu.pc = 0x6ec1;
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
    if cpu.flag_z {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.cycle(12);
    become function11ae98_asm_11aecf(cpu);
}

pub(crate) fn function11ae98_asm_11aec8(cpu: &mut Cpu) {
    cpu.pc = 0x6ec8;
    cpu.a = cpu.read_byte(0xcfa9);
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
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    become function11ae98_asm_11aecf(cpu);
}

pub(crate) fn function11ae98_asm_11aecf(cpu: &mut Cpu) {
    cpu.pc = 0x6ecf;
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.set_hl(0x6fd2);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11afbb(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte(0xcfa9, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0x6fd2);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11afb7(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function11ae98_asm_11aee1(cpu: &mut Cpu) {
    cpu.pc = 0x6ee1;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::menu::play_click_sfx(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xcfa9);
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
    if cpu.flag_z {
        cpu.cycle(12);
        become function11ae98_asm_11aeb4(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xcd4b);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xc2dd, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11b022(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_inc_jumptable_index(cpu);
    assert_eq!(cpu.pc, next_pc);
    become function11ae98_asm_11aef7(cpu);
}

pub(crate) fn function11ae98_asm_11aef7(cpu: &mut Cpu) {
    cpu.pc = 0x6ef7;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::menu::exit_menu(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::menu::exit_menu(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x41;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x4061);
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

pub(crate) fn function11af04(cpu: &mut Cpu) {
    cpu.pc = 0x6f04;
    cpu.set_hl(0x6fe8);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::menu::load_menu_header(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::menu::menu_box(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = ((((14 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (10 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0x7003);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::place_string(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0x7013);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::menu::load_menu_header(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::menu::menu_box(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = ((((8 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (16 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0x701b);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::place_string(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = ((((7 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (14 as i32)) + (0xcdd9 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(0x0506);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11afd6(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = ((((12 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (9 as i32)) + (0xcdd9 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(0x060b);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11afd6(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x41;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x4061);
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
    mobile_inc_jumptable_index(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x2;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcfa9, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0x6fd2);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11afb7(cpu);
    assert_eq!(cpu.pc, next_pc);
    become function11af4e(cpu);
}

pub(crate) fn function11af4e(cpu: &mut Cpu) {
    cpu.pc = 0x6f4e;
    cpu.set_hl(0xffa7);
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
        become function11af4e_asm_11af77(cpu);
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
        become function11af4e_asm_11af7e(cpu);
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
        become function11af4e_asm_11af97(cpu);
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
    crate::game::home::menu::play_click_sfx(cpu);
    assert_eq!(cpu.pc, next_pc);
    become function11af4e_asm_11af6a(cpu);
}

pub(crate) fn function11af4e_asm_11af6a(cpu: &mut Cpu) {
    cpu.pc = 0x6f6a;
    cpu.set_hl({ const N: i32 = ((((14 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (2 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0x7f;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcf63, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(12);
    become function11af4e_asm_11afaa(cpu);
}

pub(crate) fn function11af4e_asm_11af77(cpu: &mut Cpu) {
    cpu.pc = 0x6f77;
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
    if cpu.flag_z {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.cycle(12);
    become function11af4e_asm_11af85(cpu);
}

pub(crate) fn function11af4e_asm_11af7e(cpu: &mut Cpu) {
    cpu.pc = 0x6f7e;
    cpu.a = cpu.read_byte(0xcfa9);
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
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    become function11af4e_asm_11af85(cpu);
}

pub(crate) fn function11af4e_asm_11af85(cpu: &mut Cpu) {
    cpu.pc = 0x6f85;
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.set_hl(0x6fd2);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11afbb(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte(0xcfa9, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0x6fd2);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11afb7(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function11af4e_asm_11af97(cpu: &mut Cpu) {
    cpu.pc = 0x6f97;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::menu::play_click_sfx(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xcfa9);
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
    if cpu.flag_z {
        cpu.cycle(12);
        become function11af4e_asm_11af6a(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x6;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcf63, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
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
    become function11af4e_asm_11afaa(cpu);
}

pub(crate) fn function11af4e_asm_11afaa(cpu: &mut Cpu) {
    cpu.pc = 0x6faa;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::menu::exit_menu(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::menu::exit_menu(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x41;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x4061);
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

pub(crate) fn function11afb7(cpu: &mut Cpu) {
    cpu.pc = 0x6fb7;
    cpu.e = 0xed;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become asm_11afbd(cpu);
}

pub(crate) fn function11afbb(cpu: &mut Cpu) {
    cpu.pc = 0x6fbb;
    cpu.e = 0x7f;
    cpu.pc += 2;
    cpu.cycle(8);
    become asm_11afbd(cpu);
}

pub(crate) fn asm_11afbd(cpu: &mut Cpu) {
    cpu.pc = 0x6fbd;
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
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.e = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.d = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function11afd6(cpu: &mut Cpu) {
    cpu.pc = 0x6fd6;
    cpu.set_de(crate::defs::SCREEN_WIDTH as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0x3;
    cpu.pc += 2;
    cpu.cycle(8);
    become function11afd6_row(cpu);
}

pub(crate) fn function11afd6_row(cpu: &mut Cpu) {
    cpu.pc = 0x6fdb;
    cpu.push_word(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    become function11afd6_col(cpu);
}

pub(crate) fn function11afd6_col(cpu: &mut Cpu) {
    cpu.pc = 0x6fdd;
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
        become function11afd6_col(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
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
        become function11afd6_row(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function11b022(cpu: &mut Cpu) {
    cpu.pc = 0x7022;
    cpu.a = cpu.read_byte(0xcd2e);
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
        become function11b022_asm_11b02e(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xd099);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11b03d(cpu);
    assert_eq!(cpu.pc, next_pc);
    become function11b022_asm_11b02e(cpu);
}

pub(crate) fn function11b022_asm_11b02e(cpu: &mut Cpu) {
    cpu.pc = 0x702e;
    cpu.a = cpu.read_byte(0xcd30);
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
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(0xd0ac);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11b03d(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function11b03d(cpu: &mut Cpu) {
    cpu.pc = 0x703d;
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.c = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    become function11b03d_loop(cpu);
}

pub(crate) fn function11b03d_loop(cpu: &mut Cpu) {
    cpu.pc = 0x7041;
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = { let value = (0xef as i32); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
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
        become function11b03d_gender(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = { let value = (0xf5 as i32); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
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
        become function11b03d_gender(cpu);
    }
    cpu.pc += 2;
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
        become function11b03d_done(cpu);
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
    cpu.cycle(12);
    become function11b03d_loop(cpu);
}

pub(crate) fn function11b03d_gender(cpu: &mut Cpu) {
    cpu.pc = 0x7051;
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = { let value = (0x50 as i32); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    become function11b03d_done(cpu);
}

pub(crate) fn function11b03d_done(cpu: &mut Cpu) {
    cpu.pc = 0x7055;
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.e = 4;
    cpu.pc += 2;
    cpu.cycle(8);
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
    cpu.e = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.d = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    become function11b03d_loop2(cpu);
}

pub(crate) fn function11b03d_loop2(cpu: &mut Cpu) {
    cpu.pc = 0x705f;
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_sub(1));
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
        become function11b03d_loop2(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let value = cpu.pop_word();
    cpu.set_de(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let value = 0x1;
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
        become function11b03d_female(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x707a);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(12);
    become function11b03d_got_string(cpu);
}

pub(crate) fn function11b03d_female(cpu: &mut Cpu) {
    cpu.pc = 0x7070;
    cpu.set_hl(0x707e);
    cpu.pc += 3;
    cpu.cycle(12);
    become function11b03d_got_string(cpu);
}

pub(crate) fn function11b03d_got_string(cpu: &mut Cpu) {
    cpu.pc = 0x7073;
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
    return;
}

pub(crate) fn function11b082(cpu: &mut Cpu) {
    cpu.pc = 0x7082;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11b242(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x7;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc7d3, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11b099(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11b295(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11b275(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::tilemap::set_default_bgp_and_obp(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(16);
    become mobile_inc_jumptable_index(cpu);
}

pub(crate) fn function11b099(cpu: &mut Cpu) {
    cpu.pc = 0x7099;
    cpu.c = 0x6;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl({ const N: i32 = ((((1 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (11 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xc7d3);
    cpu.pc += 3;
    cpu.cycle(16);
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
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
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
    function11b236(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xc7d0);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.d = 0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xc6d0);
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
    cpu.set_hl({ const N: i32 = ((((2 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (11 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xc7d3);
    cpu.pc += 3;
    cpu.cycle(16);
    become function11b099_loop(cpu);
}

pub(crate) fn function11b099_loop(cpu: &mut Cpu) {
    cpu.pc = 0x70b9;
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(0xd265, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.push_word(cpu.de());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11b099_place_mon_name_or_placeholder_string(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.set_de(0x0028);
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
    let value = cpu.pop_word();
    cpu.set_de(value);
    cpu.pc += 1;
    cpu.cycle(12);
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
        become function11b099_loop(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function11b099_place_mon_name_or_placeholder_string(cpu: &mut Cpu) {
    cpu.pc = 0x70cf;
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
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11b099_check_seen_flag(cpu);
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
    function11b099_set_caught_flag(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::names::get_pokemon_name(cpu);
    assert_eq!(cpu.pc, next_pc);
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

pub(crate) fn function11b099_set_caught_flag(cpu: &mut Cpu) {
    cpu.pc = 0x70e1;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    check_caught_mem_mon(cpu);
    assert_eq!(cpu.pc, next_pc);
    if !cpu.flag_z {
        cpu.cycle(12);
        become function11b099_okay(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function11b099_okay(cpu: &mut Cpu) {
    cpu.pc = 0x70e8;
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function11b099_check_seen_flag(cpu: &mut Cpu) {
    cpu.pc = 0x70ec;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    check_seen_mem_mon(cpu);
    assert_eq!(cpu.pc, next_pc);
    if !cpu.flag_z {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(0x70f9);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::place_string(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = true;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function11b0ff(cpu: &mut Cpu) {
    cpu.pc = 0x70ff;
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
        become function11b0ff_asm_11b141(cpu);
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
        become function11b0ff_asm_11b131(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11b175(cpu);
    assert_eq!(cpu.pc, next_pc);
    if !cpu.flag_c {
        cpu.cycle(12);
        become function11b0ff_asm_11b125(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xcd4c);
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
    let value = 0x3;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd4c, cpu.a);
    cpu.pc += 3;
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
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11b099(cpu);
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

pub(crate) fn function11b0ff_asm_11b125(cpu: &mut Cpu) {
    cpu.pc = 0x7125;
    cpu.a = cpu.read_byte(0xcd4c);
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
    let value = cpu.a;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = 0x3;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd4c, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function11b0ff_asm_11b131(cpu: &mut Cpu) {
    cpu.pc = 0x7131;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11b20b(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    check_seen_mem_mon(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_z {
        cpu.cycle(12);
        become function11b0ff_asm_11b13d(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become function11b0ff_asm_11b148(cpu);
}

pub(crate) fn function11b0ff_asm_11b13d(cpu: &mut Cpu) {
    cpu.pc = 0x713d;
    cpu.a = 0x2;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become function11b0ff_asm_11b148(cpu);
}

pub(crate) fn function11b0ff_asm_11b141(cpu: &mut Cpu) {
    cpu.pc = 0x7141;
    cpu.set_hl(0xcf63);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0x7;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function11b0ff_asm_11b148(cpu: &mut Cpu) {
    cpu.pc = 0x7148;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::menu::play_click_sfx(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.write_byte(0xcd4b, cpu.a);
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
        become function11b0ff_asm_11b16c(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xcf65);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x0;
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
        become function11b0ff_asm_11b163(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0xfe;
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
        become function11b0ff_asm_11b167(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
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
        become function11b0ff_asm_11b16b(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become mobile_inc_jumptable_index(cpu);
}

pub(crate) fn function11b0ff_asm_11b163(cpu: &mut Cpu) {
    cpu.pc = 0x7163;
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become function11b0ff_asm_11b16c(cpu);
}

pub(crate) fn function11b0ff_asm_11b167(cpu: &mut Cpu) {
    cpu.pc = 0x7167;
    cpu.a = 0x2;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become function11b0ff_asm_11b16c(cpu);
}

pub(crate) fn function11b0ff_asm_11b16b(cpu: &mut Cpu) {
    cpu.pc = 0x716b;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    become function11b0ff_asm_11b16c(cpu);
}

pub(crate) fn function11b0ff_asm_11b16c(cpu: &mut Cpu) {
    cpu.pc = 0x716c;
    cpu.write_byte(0xcd30, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x4;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcf63, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function11b175(cpu: &mut Cpu) {
    cpu.pc = 0x7175;
    cpu.a = cpu.read_byte(0xc7d3);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.d = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xc7d2);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
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
        become function11b175_asm_11b19a(cpu);
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
        become function11b175_asm_11b1ae(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.d;
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
    if !cpu.flag_c {
        cpu.cycle(12);
        become function11b175_asm_11b1ed(cpu);
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
        become function11b175_asm_11b1c6(cpu);
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
        become function11b175_asm_11b1d8(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become function11b175_asm_11b1ed(cpu);
}

pub(crate) fn function11b175_asm_11b19a(cpu: &mut Cpu) {
    cpu.pc = 0x719a;
    cpu.set_hl(0xc7d1);
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
        become function11b175_asm_11b1a4(cpu);
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
    cpu.cycle(12);
    become function11b175_asm_11b1ef(cpu);
}

pub(crate) fn function11b175_asm_11b1a4(cpu: &mut Cpu) {
    cpu.pc = 0x71a4;
    cpu.set_hl(0xc7d0);
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
        become function11b175_asm_11b1ed(cpu);
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
    cpu.cycle(12);
    become function11b175_asm_11b1ef(cpu);
}

pub(crate) fn function11b175_asm_11b1ae(cpu: &mut Cpu) {
    cpu.pc = 0x71ae;
    cpu.set_hl(0xc7d1);
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
    let value = cpu.e;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_c {
        cpu.cycle(12);
        become function11b175_asm_11b1ed(cpu);
    }
    cpu.pc += 2;
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
    if !cpu.flag_c {
        cpu.cycle(12);
        become function11b175_asm_11b1bc(cpu);
    }
    cpu.pc += 2;
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
    become function11b175_asm_11b1ef(cpu);
}

pub(crate) fn function11b175_asm_11b1bc(cpu: &mut Cpu) {
    cpu.pc = 0x71bc;
    cpu.set_hl(0xc7d0);
    cpu.pc += 3;
    cpu.cycle(12);
    let value = cpu.read_byte(cpu.hl());
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_add(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) > 0x0f;
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.e;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_c {
        cpu.cycle(12);
        become function11b175_asm_11b1ed(cpu);
    }
    cpu.pc += 2;
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
    become function11b175_asm_11b1ef(cpu);
}

pub(crate) fn function11b175_asm_11b1c6(cpu: &mut Cpu) {
    cpu.pc = 0x71c6;
    cpu.set_hl(0xc7d0);
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
        become function11b175_asm_11b1ed(cpu);
    }
    cpu.pc += 2;
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
    if !cpu.flag_c {
        cpu.cycle(12);
        become function11b175_asm_11b1d4(cpu);
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
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.cycle(12);
    become function11b175_asm_11b1ef(cpu);
}

pub(crate) fn function11b175_asm_11b1d4(cpu: &mut Cpu) {
    cpu.pc = 0x71d4;
    let value = cpu.d;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.cycle(12);
    become function11b175_asm_11b1ef(cpu);
}

pub(crate) fn function11b175_asm_11b1d8(cpu: &mut Cpu) {
    cpu.pc = 0x71d8;
    cpu.set_hl(0xc7d0);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.d;
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
    let value = cpu.read_byte(cpu.hl());
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_add(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) > 0x0f;
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(8);
    if cpu.flag_c {
        cpu.cycle(12);
        become function11b175_asm_11b1e3(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
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
        become function11b175_asm_11b1e8(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become function11b175_asm_11b1e3(cpu);
}

pub(crate) fn function11b175_asm_11b1e3(cpu: &mut Cpu) {
    cpu.pc = 0x71e3;
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.d;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.cycle(12);
    become function11b175_asm_11b1ef(cpu);
}

pub(crate) fn function11b175_asm_11b1e8(cpu: &mut Cpu) {
    cpu.pc = 0x71e8;
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.d;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_add(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) > 0x0f;
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.cycle(12);
    become function11b175_asm_11b1ef(cpu);
}

pub(crate) fn function11b175_asm_11b1ed(cpu: &mut Cpu) {
    cpu.pc = 0x71ed;
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

pub(crate) fn function11b175_asm_11b1ef(cpu: &mut Cpu) {
    cpu.pc = 0x71ef;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11b295(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11b275(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = true;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn fill_screen_with_tile32(cpu: &mut Cpu) {
    cpu.pc = 0x71f7;
    cpu.set_hl({ const N: i32 = ((((0 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (0 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0x32;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_bc(crate::defs::SCREEN_AREA as u16);
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

pub(crate) fn copy_data_until_ff(cpu: &mut Cpu) {
    cpu.pc = 0x7203;
    become copy_data_until_ff_loop(cpu);
}

pub(crate) fn copy_data_until_ff_loop(cpu: &mut Cpu) {
    cpu.pc = 0x7203;
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
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
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.cycle(12);
    become copy_data_until_ff_loop(cpu);
}

pub(crate) fn function11b20b(cpu: &mut Cpu) {
    cpu.pc = 0x720b;
    cpu.a = cpu.read_byte(0xc7d1);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xc7d0);
    cpu.pc += 3;
    cpu.cycle(12);
    let value = cpu.read_byte(cpu.hl());
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_add(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) > 0x0f;
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.d = 0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xc6d0);
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
    cpu.write_byte(0xd265, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn check_caught_mem_mon(cpu: &mut Cpu) {
    cpu.pc = 0x721e;
    cpu.push_word(cpu.de());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xd265);
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
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let value = cpu.pop_word();
    cpu.set_de(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn check_seen_mem_mon(cpu: &mut Cpu) {
    cpu.pc = 0x722a;
    cpu.push_word(cpu.de());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xd265);
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
    crate::game::home::pokedex_flags::check_seen_mon(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let value = cpu.pop_word();
    cpu.set_de(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function11b236(cpu: &mut Cpu) {
    cpu.pc = 0x7236;
    cpu.cycle(16);
    become crate::game::home::text::fill_box_with_byte(cpu);
}

pub(crate) fn function11b239(cpu: &mut Cpu) {
    cpu.pc = 0x7239;
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
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function11b242(cpu: &mut Cpu) {
    cpu.pc = 0x7242;
    cpu.set_hl({ const N: i32 = ((((4 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (3 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xd099);
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
    cpu.write_byte(0xcf5f, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x14;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x4bdd);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = ((((4 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (1 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xd108);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_bc(0xcd2f);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.write_byte(cpu.bc(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_bc(cpu.bc().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    if cpu.flag_c {
        cpu.cycle(12);
        become asm_11b26a(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become asm_11b26f(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.bc(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    become function11b267(cpu);
}

pub(crate) fn function11b267(cpu: &mut Cpu) {
    cpu.pc = 0x7267;
    cpu.write_byte(cpu.hl(), 0xef);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn asm_11b26a(cpu: &mut Cpu) {
    cpu.pc = 0x726a;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.bc(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), 0x7f);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn asm_11b26f(cpu: &mut Cpu) {
    cpu.pc = 0x726f;
    cpu.a = 0x2;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.bc(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    become function11b272(cpu);
}

pub(crate) fn function11b272(cpu: &mut Cpu) {
    cpu.pc = 0x7272;
    cpu.write_byte(cpu.hl(), 0xf5);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function11b275(cpu: &mut Cpu) {
    cpu.pc = 0x7275;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11b279(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function11b279(cpu: &mut Cpu) {
    cpu.pc = 0x7279;
    cpu.a = cpu.read_byte(0xd265);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xcf60, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    check_seen_mem_mon(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_z {
        cpu.cycle(12);
        become function11b279_asm_11b28f(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::pokemon::get_base_data(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xd243);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xcf65, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(12);
    become function11b279_asm_11b294(cpu);
}

pub(crate) fn function11b279_asm_11b28f(cpu: &mut Cpu) {
    cpu.pc = 0x728f;
    cpu.a = 0xff;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcf65, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    become function11b279_asm_11b294(cpu);
}

pub(crate) fn function11b279_asm_11b294(cpu: &mut Cpu) {
    cpu.pc = 0x7294;
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function11b295(cpu: &mut Cpu) {
    cpu.pc = 0x7295;
    cpu.set_hl({ const N: i32 = ((((13 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (4 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0x7308);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::place_string(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = ((((14 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (4 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0x7308);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::place_string(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11b20b(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    check_seen_mem_mon(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_z {
        cpu.cycle(12);
        become function11b295_asm_11b2d1(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc608);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte({ const N: i32 = (0xc608 as i32) + (1 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl(0x0007 as u16);
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
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(0x0003 as u16);
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
    cpu.e = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 0x23;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x69bc);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = ((((14 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (4 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::names::get_pokemon_name(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(12);
    become function11b295_asm_11b2e7(cpu);
}

pub(crate) fn function11b295_asm_11b2d1(cpu: &mut Cpu) {
    cpu.pc = 0x72d1;
    cpu.a = cpu.read_byte(0xc608);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte({ const N: i32 = (0xc608 as i32) + (1 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl(0x0007 as u16);
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
    cpu.a = 0x50;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl({ const N: i32 = ((((13 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (4 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.set_de(0x730e);
    cpu.pc += 3;
    cpu.cycle(12);
    become function11b295_asm_11b2e7(cpu);
}

pub(crate) fn function11b295_asm_11b2e7(cpu: &mut Cpu) {
    cpu.pc = 0x72e7;
    cpu.a = crate::defs::NAME_LENGTH_JAPANESE as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_bc(0xd0ac);
    cpu.pc += 3;
    cpu.cycle(12);
    become function11b295_asm_11b2ec(cpu);
}

pub(crate) fn function11b295_asm_11b2ec(cpu: &mut Cpu) {
    cpu.pc = 0x72ec;
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.bc(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_bc(cpu.bc().wrapping_add(1));
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
        become function11b295_asm_11b2ec(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.set_de(0xd0ac);
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

pub(crate) fn function11b314(cpu: &mut Cpu) {
    cpu.pc = 0x7314;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11b31b(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11b3d9(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function11b31b(cpu: &mut Cpu) {
    cpu.pc = 0x731b;
    cpu.set_hl(0x7350);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xcf63);
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
    if cpu.flag_c {
        cpu.cycle(12);
        become function11b31b_tilemap_1(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc7d1);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 4;
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
        become function11b31b_tilemap_3(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 3;
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
        become function11b31b_tilemap_1(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xcf63);
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
    if cpu.flag_z {
        cpu.cycle(12);
        become function11b31b_tilemap_1(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
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
        become function11b31b_tilemap_1(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 6;
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
        become function11b31b_tilemap_1(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_bc(0x737b);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(12);
    become function11b31b_load_sprites(cpu);
}

pub(crate) fn function11b31b_tilemap_3(cpu: &mut Cpu) {
    cpu.pc = 0x7344;
    cpu.set_bc(0x7389);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(12);
    become function11b31b_load_sprites(cpu);
}

pub(crate) fn function11b31b_tilemap_1(cpu: &mut Cpu) {
    cpu.pc = 0x7349;
    cpu.set_bc(0x736d);
    cpu.pc += 3;
    cpu.cycle(12);
    become function11b31b_load_sprites(cpu);
}

pub(crate) fn function11b31b_load_sprites(cpu: &mut Cpu) {
    cpu.pc = 0x734c;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11b397(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function11b397(cpu: &mut Cpu) {
    cpu.pc = 0x7397;
    cpu.set_de(0xc400);
    cpu.pc += 3;
    cpu.cycle(12);
    become function11b397_loop(cpu);
}

pub(crate) fn function11b397_loop(cpu: &mut Cpu) {
    cpu.pc = 0x739a;
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
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
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc7d1);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x7;
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
    let value = cpu.read_byte(cpu.hl());
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_add(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) > 0x0f;
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_bc(cpu.bc().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 0x5;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.cycle(12);
    become function11b397_loop(cpu);
}

pub(crate) fn function11b3b6(cpu: &mut Cpu) {
    cpu.pc = 0x73b6;
    become function11b3b6_loop(cpu);
}

pub(crate) fn function11b3b6_loop(cpu: &mut Cpu) {
    cpu.pc = 0x73b6;
    cpu.a = cpu.read_byte(cpu.hl());
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
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xcd4d);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x7;
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
    let value = cpu.read_byte(cpu.hl());
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_add(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) > 0x0f;
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.l = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.h = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xcd4e);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.read_byte(cpu.hl());
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_add(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) > 0x0f;
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_bc(cpu.bc().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.a = 0x5;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.cycle(12);
    become function11b3b6_loop(cpu);
}

pub(crate) fn function11b3d9(cpu: &mut Cpu) {
    cpu.pc = 0x73d9;
    cpu.set_de(0xc470);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.push_word(cpu.de());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xc7d2);
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
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xc7d1);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xc7d0);
    cpu.pc += 3;
    cpu.cycle(12);
    let value = cpu.read_byte(cpu.hl());
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_add(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) > 0x0f;
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.e;
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
        become function11b3d9_skip(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(0x70 as u16);
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
    cpu.b = 0;
    cpu.pc += 2;
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
    if cpu.flag_z {
        cpu.cycle(12);
        become function11b3d9_load_sprites(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc7d2);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    become function11b3d9_loop1(cpu);
}

pub(crate) fn function11b3d9_loop1(cpu: &mut Cpu) {
    cpu.pc = 0x7401;
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.c;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = 0x0;
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
    cpu.d = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_c {
        cpu.cycle(12);
        become function11b3d9_load_sprites(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.b;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.b = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.cycle(12);
    become function11b3d9_loop1(cpu);
}

pub(crate) fn function11b3d9_skip(cpu: &mut Cpu) {
    cpu.pc = 0x740d;
    cpu.b = { let value = ((14 as i32) * (crate::defs::TILE_WIDTH as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    become function11b3d9_load_sprites(cpu);
}

pub(crate) fn function11b3d9_load_sprites(cpu: &mut Cpu) {
    cpu.pc = 0x740f;
    cpu.a = { let value = (((2 as i32) * (crate::defs::TILE_WIDTH as i32)) + (5 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
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
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0x41;
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
        become function11b3d9_version1(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xcf63);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 4;
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
        become function11b3d9_version2(cpu);
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
    if cpu.flag_z {
        cpu.cycle(12);
        become function11b3d9_version2(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 7;
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
        become function11b3d9_version2(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 8;
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
        become function11b3d9_version2(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become function11b3d9_version1(cpu);
}

pub(crate) fn function11b3d9_version1(cpu: &mut Cpu) {
    cpu.pc = 0x742b;
    cpu.a = { let value = (((19 as i32) * (crate::defs::TILE_WIDTH as i32)) + (3 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xcd4c);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x3c;
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
    cpu.a = cpu.read_byte(0xcd4c);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x1;
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
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function11b3d9_version2(cpu: &mut Cpu) {
    cpu.pc = 0x743b;
    cpu.a = { let value = (((19 as i32) * (crate::defs::TILE_WIDTH as i32)) + (3 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 0x39;
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
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn trade_corner_hold_mon(cpu: &mut Cpu) {
    cpu.pc = 0x7444;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile46_init_jumptable(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    trade_corner_hold_mon_run_jumptable(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn mobile46_init_jumptable(cpu: &mut Cpu) {
    cpu.pc = 0x744b;
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
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::time::update_time(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn trade_corner_hold_mon_run_jumptable(cpu: &mut Cpu) {
    cpu.pc = 0x745c;
    become trade_corner_hold_mon_run_jumptable_loop(cpu);
}

pub(crate) fn trade_corner_hold_mon_run_jumptable_loop(cpu: &mut Cpu) {
    cpu.pc = 0x745c;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    trade_corner_hold_mon_run_jumptable_iterate_jumptable(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::delay::delay_frame(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xcf63);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 4;
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
        become trade_corner_hold_mon_run_jumptable_loop(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn trade_corner_hold_mon_prepare_for_upload(cpu: &mut Cpu) {
    cpu.pc = 0x7483;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    trade_corner_hold_mon_prepare_for_upload_init_ram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xd47d);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = { let value = ((crate::defs::NAME_LENGTH_JAPANESE as i32) - (1 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    become trade_corner_hold_mon_prepare_for_upload_get_char(cpu);
}

pub(crate) fn trade_corner_hold_mon_prepare_for_upload_get_char(cpu: &mut Cpu) {
    cpu.pc = 0x748b;
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.bc(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_bc(cpu.bc().wrapping_add(1));
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
        become trade_corner_hold_mon_prepare_for_upload_get_char(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_de(crate::defs::PARTYMON_STRUCT_LENGTH as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0xdcdf);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xcd82);
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
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    become trade_corner_hold_mon_prepare_for_upload_get_next_party_mon(cpu);
}

pub(crate) fn trade_corner_hold_mon_prepare_for_upload_get_next_party_mon(cpu: &mut Cpu) {
    cpu.pc = 0x749f;
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
        become trade_corner_hold_mon_prepare_for_upload_got_selected_mon(cpu);
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
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.cycle(12);
    become trade_corner_hold_mon_prepare_for_upload_get_next_party_mon(cpu);
}

pub(crate) fn trade_corner_hold_mon_prepare_for_upload_got_selected_mon(cpu: &mut Cpu) {
    cpu.pc = 0x74a6;
    cpu.push_word(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = crate::defs::PARTYMON_STRUCT_LENGTH as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    become trade_corner_hold_mon_prepare_for_upload_copy_mon_byte(cpu);
}

pub(crate) fn trade_corner_hold_mon_prepare_for_upload_copy_mon_byte(cpu: &mut Cpu) {
    cpu.pc = 0x74a9;
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.bc(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_bc(cpu.bc().wrapping_add(1));
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
        become trade_corner_hold_mon_prepare_for_upload_copy_mon_byte(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.pop_word();
    cpu.set_de(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.push_word(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(cpu.de());
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
    cpu.set_hl(crate::defs::MON_LEVEL as u16);
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
    cpu.write_byte(0xd143, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(crate::defs::MON_MAXHP as u16);
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
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.set_hl({ const N: i32 = (crate::defs::MON_STAT_EXP as u16 as i32) - (1 as i32); N as u16 });
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
    let value = cpu.pop_word();
    cpu.set_de(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.push_word(cpu.de());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.b = crate::defs::TRUE as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = { let value = (((0x458f as i32) - (0x456b as i32)) / (3 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::predef::predef(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_de(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.h = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.l = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.pop_word();
    cpu.set_bc(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.set_de(crate::defs::NAME_LENGTH as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0xddff);
    cpu.pc += 3;
    cpu.cycle(12);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    become trade_corner_hold_mon_prepare_for_upload_loop4(cpu);
}

pub(crate) fn trade_corner_hold_mon_prepare_for_upload_loop4(cpu: &mut Cpu) {
    cpu.pc = 0x74e8;
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
        become trade_corner_hold_mon_prepare_for_upload_okay2(cpu);
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
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.cycle(12);
    become trade_corner_hold_mon_prepare_for_upload_loop4(cpu);
}

pub(crate) fn trade_corner_hold_mon_prepare_for_upload_okay2(cpu: &mut Cpu) {
    cpu.pc = 0x74ef;
    cpu.a = { let value = ((crate::defs::NAME_LENGTH as i32) - (1 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    become trade_corner_hold_mon_prepare_for_upload_loop5(cpu);
}

pub(crate) fn trade_corner_hold_mon_prepare_for_upload_loop5(cpu: &mut Cpu) {
    cpu.pc = 0x74f1;
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.bc(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_bc(cpu.bc().wrapping_add(1));
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
        become trade_corner_hold_mon_prepare_for_upload_loop5(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_de(crate::defs::NAME_LENGTH as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0xde41);
    cpu.pc += 3;
    cpu.cycle(12);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    become trade_corner_hold_mon_prepare_for_upload_loop6(cpu);
}

pub(crate) fn trade_corner_hold_mon_prepare_for_upload_loop6(cpu: &mut Cpu) {
    cpu.pc = 0x7502;
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
        become trade_corner_hold_mon_prepare_for_upload_okay3(cpu);
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
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.cycle(12);
    become trade_corner_hold_mon_prepare_for_upload_loop6(cpu);
}

pub(crate) fn trade_corner_hold_mon_prepare_for_upload_okay3(cpu: &mut Cpu) {
    cpu.pc = 0x7509;
    cpu.a = { let value = ((crate::defs::NAME_LENGTH as i32) - (1 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    become trade_corner_hold_mon_prepare_for_upload_loop7(cpu);
}

pub(crate) fn trade_corner_hold_mon_prepare_for_upload_loop7(cpu: &mut Cpu) {
    cpu.pc = 0x750b;
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.bc(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_bc(cpu.bc().wrapping_add(1));
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
        become trade_corner_hold_mon_prepare_for_upload_loop7(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_de(crate::defs::MAIL_STRUCT_LENGTH as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0xa600);
    cpu.pc += 3;
    cpu.cycle(12);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    become trade_corner_hold_mon_prepare_for_upload_loop8(cpu);
}

pub(crate) fn trade_corner_hold_mon_prepare_for_upload_loop8(cpu: &mut Cpu) {
    cpu.pc = 0x751b;
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
        become trade_corner_hold_mon_prepare_for_upload_okay4(cpu);
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
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.cycle(12);
    become trade_corner_hold_mon_prepare_for_upload_loop8(cpu);
}

pub(crate) fn trade_corner_hold_mon_prepare_for_upload_okay4(cpu: &mut Cpu) {
    cpu.pc = 0x7522;
    cpu.a = 0x00;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = crate::defs::MAIL_STRUCT_LENGTH as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    become trade_corner_hold_mon_prepare_for_upload_loop9(cpu);
}

pub(crate) fn trade_corner_hold_mon_prepare_for_upload_loop9(cpu: &mut Cpu) {
    cpu.pc = 0x7529;
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.bc(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_bc(cpu.bc().wrapping_add(1));
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
        become trade_corner_hold_mon_prepare_for_upload_loop9(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::close_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(16);
    become mobile_inc_jumptable_index(cpu);
}

pub(crate) fn trade_corner_hold_mon_prepare_for_upload_init_ram(cpu: &mut Cpu) {
    cpu.pc = 0x7538;
    cpu.set_bc(0xc626);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xd47b);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xcd2a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(cpu.bc(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_bc(cpu.bc().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte({ const N: i32 = (0xd47b as i32) + (1 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xcd2b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(cpu.bc(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_bc(cpu.bc().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xd84a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xcd2c, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(cpu.bc(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_bc(cpu.bc().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte({ const N: i32 = (0xd84a as i32) + (1 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xcd2d, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(cpu.bc(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_bc(cpu.bc().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xcd2e);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(cpu.bc(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_bc(cpu.bc().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xcd2f);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(cpu.bc(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_bc(cpu.bc().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xcd30);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(cpu.bc(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_bc(cpu.bc().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xd265);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(cpu.bc(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_bc(cpu.bc().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function11b570(cpu: &mut Cpu) {
    cpu.pc = 0x7570;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function118007(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xc2dd);
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
        become function11b570_exit(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11b570_save_data(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(16);
    become mobile_inc_jumptable_index(cpu);
}

pub(crate) fn function11b570_exit(cpu: &mut Cpu) {
    cpu.pc = 0x757f;
    cpu.a = 0x4;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcf63, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function11b570_save_data(cpu: &mut Cpu) {
    cpu.pc = 0x7585;
    cpu.a = 0x3;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.set_hl(0xd800);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xc608);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc({ const N: i32 = (0xd88f as i32) - (0xd800 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = 0x05;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_de(0xa800);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(0xc608);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc({ const N: i32 = (0xd88f as i32) - (0xd800 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.push_word(cpu.de());
    cpu.pc += 1;
    cpu.cycle(16);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xff90);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xff8f);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xff8e);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xff8d);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::close_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn trade_corner_hold_mon_remove_from_party(cpu: &mut Cpu) {
    cpu.pc = 0x75c0;
    cpu.a = cpu.read_byte(0xcd82);
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
    cpu.write_byte(0xd10b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x03;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x6039);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x5c;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x4807);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x05;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x4a58);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(16);
    become mobile_inc_jumptable_index(cpu);
}

pub(crate) fn trade_corner_hold_mon_success(cpu: &mut Cpu) {
    cpu.pc = 0x75e0;
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
    cpu.cycle(16);
    become mobile_inc_jumptable_index(cpu);
}

pub(crate) fn trade_corner_hold_mon_noop(cpu: &mut Cpu) {
    cpu.pc = 0x75e7;
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function11b5e8(cpu: &mut Cpu) {
    cpu.pc = 0x75e8;
    cpu.a = 0x0;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xd4ba);
    cpu.pc += 3;
    cpu.cycle(12);
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
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::close_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x5;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xc608);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xb08c as u16);
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
    cpu.a = 0x2;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xa800 as u16, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xa81f as u16);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xcd2a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xa820 as u16);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xcd2b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xa821 as u16);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xcd2c, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xa822 as u16);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xcd2d, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xa823 as u16);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xcd2e, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xa824 as u16);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xcd2f, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xa825 as u16);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xcd30, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xa826 as u16);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xcd31, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::close_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile46_init_jumptable(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11b5e8_loop(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function11b5e8_loop(cpu: &mut Cpu) {
    cpu.pc = 0x764c;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11b5e8_run_jumptable(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::delay::delay_frame(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xcf63);
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
    if !cpu.flag_z {
        cpu.cycle(12);
        become function11b5e8_loop(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function11b66d(cpu: &mut Cpu) {
    cpu.pc = 0x766d;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function1180b8(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xc2dd);
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
        become function11b66d_asm_11b6b0(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(crate::defs::R_WBK as u16);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = 0x3;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xd090);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = cpu.b;
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
        become function11b66d_asm_11b691(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x1;
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
        become function11b66d_asm_11b6b0(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11b6b4(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(12);
    become function11b66d_asm_11b6b0(cpu);
}

pub(crate) fn function11b66d_asm_11b691(cpu: &mut Cpu) {
    cpu.pc = 0x7691;
    cpu.a = 0x5c;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x481d);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xc2dd);
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
        become function11b66d_asm_11b6b0(cpu);
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
    cpu.a = cpu.read_byte(crate::defs::R_WBK as u16);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = 0x3;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = 0x2;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xd090, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    become function11b66d_asm_11b6b0(cpu);
}

pub(crate) fn function11b66d_asm_11b6b0(cpu: &mut Cpu) {
    cpu.pc = 0x76b0;
    cpu.cycle(16);
    become mobile_inc_jumptable_index(cpu);
}

pub(crate) fn function11b6b3(cpu: &mut Cpu) {
    cpu.pc = 0x76b3;
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function11b6b4(cpu: &mut Cpu) {
    cpu.pc = 0x76b4;
    cpu.a = 0x5;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xcd30);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xc708, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xcd31);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xc709, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = ((0xc708u32) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd20, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = (((0xc708u32) >> 8) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte({ const N: i32 = (0xcd20 as i32) + (1 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = ((0xc60du32) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd22, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = (((0xc60du32) >> 8) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte({ const N: i32 = (0xcd22 as i32) + (1 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = ((0xc63du32) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd24, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = (((0xc63du32) >> 8) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte({ const N: i32 = (0xcd24 as i32) + (1 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = ((0xc642u32) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd26, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = (((0xc642u32) >> 8) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte({ const N: i32 = (0xcd26 as i32) + (1 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = ((0xc647u32) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd28, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = (((0xc647u32) >> 8) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte({ const N: i32 = (0xcd28 as i32) + (1 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = crate::defs::BASE_HAPPINESS as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc628, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_de(0xc63d);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.c = { let value = ((crate::defs::NAME_LENGTH_JAPANESE as i32) - (1 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x5f;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x5073);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    if !cpu.flag_c {
        cpu.cycle(12);
        become function11b6b4_length_check_ot(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x5f;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x5187);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    become function11b6b4_length_check_ot(cpu);
}

pub(crate) fn function11b6b4_length_check_ot(cpu: &mut Cpu) {
    cpu.pc = 0x770f;
    cpu.set_de(0xc63d);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(0x0105);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0x5f;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x51e1);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    if !cpu.flag_c {
        cpu.cycle(12);
        become function11b6b4_error_check_nick(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x5f;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x5187);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    become function11b6b4_error_check_nick(cpu);
}

pub(crate) fn function11b6b4_error_check_nick(cpu: &mut Cpu) {
    cpu.pc = 0x7723;
    cpu.set_de(0xc642);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.c = { let value = ((crate::defs::NAME_LENGTH_JAPANESE as i32) - (1 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x5f;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x5073);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    if !cpu.flag_c {
        cpu.cycle(12);
        become function11b6b4_length_check_nick(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x5f;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x5199);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    become function11b6b4_length_check_nick(cpu);
}

pub(crate) fn function11b6b4_length_check_nick(cpu: &mut Cpu) {
    cpu.pc = 0x7736;
    cpu.set_de(0xc642);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(0x0105);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0x5f;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x51e1);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    if !cpu.flag_c {
        cpu.cycle(12);
        become function11b6b4_error_check_mail(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x5f;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x5199);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    become function11b6b4_error_check_mail(cpu);
}

pub(crate) fn function11b6b4_error_check_mail(cpu: &mut Cpu) {
    cpu.pc = 0x774a;
    cpu.set_de(0xc647);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.c = { let value = ((crate::defs::MAIL_MSG_LENGTH as i32) + (1 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x5f;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x5073);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    if !cpu.flag_c {
        cpu.cycle(12);
        become function11b6b4_length_check_mail(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x5f;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x51ab);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    become function11b6b4_length_check_mail(cpu);
}

pub(crate) fn function11b6b4_length_check_mail(cpu: &mut Cpu) {
    cpu.pc = 0x775d;
    cpu.set_de(0xc647);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(0x0221);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0x5f;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x51e1);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.cycle(12);
        become function11b6b4_fix_mail(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
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
        become function11b6b4_mail_ok(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become function11b6b4_fix_mail(cpu);
}

pub(crate) fn function11b6b4_fix_mail(cpu: &mut Cpu) {
    cpu.pc = 0x7770;
    cpu.a = 0x5f;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x51ab);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    become function11b6b4_mail_ok(cpu);
}

pub(crate) fn function11b6b4_mail_ok(cpu: &mut Cpu) {
    cpu.pc = 0x7776;
    cpu.set_de(0xc668);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.c = { let value = ((crate::defs::NAME_LENGTH_JAPANESE as i32) - (1 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x5f;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x5073);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    if !cpu.flag_c {
        cpu.cycle(12);
        become function11b6b4_length_check_author(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x5f;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x51c9);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    become function11b6b4_length_check_author(cpu);
}

pub(crate) fn function11b6b4_length_check_author(cpu: &mut Cpu) {
    cpu.pc = 0x7789;
    cpu.set_de(0xc668);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(0x0105);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0x5f;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x51e1);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    if !cpu.flag_c {
        cpu.cycle(12);
        become function11b6b4_author_okay(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x5f;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x51c9);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    become function11b6b4_author_okay(cpu);
}

pub(crate) fn function11b6b4_author_okay(cpu: &mut Cpu) {
    cpu.pc = 0x779d;
    cpu.a = cpu.read_byte(0xc60e);
    cpu.pc += 3;
    cpu.cycle(16);
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
        become function11b6b4_item_okay(cpu);
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
    cpu.write_byte(0xc60e, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    become function11b6b4_item_okay(cpu);
}

pub(crate) fn function11b6b4_item_okay(cpu: &mut Cpu) {
    cpu.pc = 0x77a8;
    cpu.a = cpu.read_byte(0xcd31);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xc60d, cpu.a);
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
    cpu.set_hl(0xc62c);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = crate::defs::MIN_LEVEL as u8;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = crate::defs::MIN_LEVEL as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_c {
        cpu.cycle(12);
        become function11b6b4_replace_level(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = crate::defs::MAX_LEVEL as u8;
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
        become function11b6b4_done_level(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = crate::defs::MAX_LEVEL as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    become function11b6b4_replace_level(cpu);
}

pub(crate) fn function11b6b4_replace_level(cpu: &mut Cpu) {
    cpu.pc = 0x77c5;
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    become function11b6b4_done_level(cpu);
}

pub(crate) fn function11b6b4_done_level(cpu: &mut Cpu) {
    cpu.pc = 0x77c6;
    cpu.write_byte(0xd143, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl({ const N: i32 = (0xc615 as i32) + (2 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xc631);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = crate::defs::TRUE as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = { let value = (((0x458f as i32) - (0x456b as i32)) / (3 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::predef::predef(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_de(0xc631);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0xc62f);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    add_mobile_mon_to_party(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function11b7e5(cpu: &mut Cpu) {
    cpu.pc = 0x77e5;
    cpu.a = cpu.read_byte(0xc60d);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xc702, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xd108, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xcd81);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xc74e, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xc63d);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xc724);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(5 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = { let value = (0x50 as i32); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc613);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xc731, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte({ const N: i32 = (0xc613 as i32) + (1 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte({ const N: i32 = (0xc731 as i32) + (1 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xc622);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(0xc72f, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte({ const N: i32 = (0xc72f as i32) + (1 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_bc(0xc60d);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0x13;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x7301);
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
    cpu.write_byte(0xc733, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::speech_textbox(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map::fade_to_menu(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x42;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x4016);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x5f;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x51f1);
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
    cpu.write_byte(0xd1e9, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = crate::defs::LINK_TRADECENTER as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc2dc, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x10;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x61d8);
    cpu.pc += 3;
    cpu.cycle(12);
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
    cpu.write_byte(0xc2dc, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x05;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x4a58);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x05;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x5;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xa800, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::close_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xdcb5);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xdcb6);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map::get_map_scene_id(cpu);
    assert_eq!(cpu.pc, next_pc);
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
    if cpu.flag_z {
        cpu.cycle(12);
        become function11b7e5_asm_11b872(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    become function11b7e5_asm_11b872(cpu);
}

pub(crate) fn function11b7e5_asm_11b872(cpu: &mut Cpu) {
    cpu.pc = 0x7872;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map::close_submenu(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::audio::restart_map_music(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function11b879(cpu: &mut Cpu) {
    cpu.pc = 0x7879;
    cpu.a = 0x5c;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x489a);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xc2dd);
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
    cpu.a = 0x05;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xa800);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xc2dd, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xa890);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xcd49, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xa891);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xcd4a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xa892);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xcd4b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xa893);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xcd4c, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::close_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xc2dd);
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
    cpu.set_hl(0xcd4c);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xff8d);
    cpu.pc += 2;
    cpu.cycle(12);
    let value = cpu.read_byte(cpu.hl());
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 1;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xff8e);
    cpu.pc += 2;
    cpu.cycle(12);
    let value = cpu.read_byte(cpu.hl());
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 1;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(0xcd4a);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xff8f);
    cpu.pc += 2;
    cpu.cycle(12);
    let value = cpu.read_byte(cpu.hl());
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 1;
    cpu.cycle(8);
    if !cpu.flag_c {
        cpu.cycle(12);
        become function11b879_asm_11b8d8(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x18;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.read_byte(cpu.hl());
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(0xff8f);
    cpu.pc += 3;
    cpu.cycle(12);
    let value = cpu.read_byte(cpu.hl());
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_add(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) > 0x0f;
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(0xcd4c, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xff90);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.write_byte(0xcd4b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcd4a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(12);
    become function11b879_asm_11b8e2(cpu);
}

pub(crate) fn function11b879_asm_11b8d8(cpu: &mut Cpu) {
    cpu.pc = 0x78d8;
    cpu.a = cpu.read_byte(0xff90);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.write_byte(0xcd4b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xff8f);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.write_byte(0xcd4c, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    become function11b879_asm_11b8e2(cpu);
}

pub(crate) fn function11b879_asm_11b8e2(cpu: &mut Cpu) {
    cpu.pc = 0x78e2;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.h = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.d = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xcd4b);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xcd4c);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.c = 0x3c;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::array::add_n_times(cpu);
    assert_eq!(cpu.pc, next_pc);
    let lhs = cpu.hl();
    let rhs = cpu.de();
    let (result, carry) = lhs.overflowing_add(rhs);
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0fff) + (rhs & 0x0fff) > 0x0fff;
    cpu.flag_c = carry;
    cpu.set_hl(result);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.h = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.d = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xcd49);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xcd4a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.c = 0x3c;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::array::add_n_times(cpu);
    assert_eq!(cpu.pc, next_pc);
    let lhs = cpu.hl();
    let rhs = cpu.de();
    let (result, carry) = lhs.overflowing_add(rhs);
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0fff) + (rhs & 0x0fff) > 0x0fff;
    cpu.flag_c = carry;
    cpu.set_hl(result);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = !cpu.a;
    cpu.flag_n = true;
    cpu.flag_h = true;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = 0x1;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_add(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) > 0x0f;
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = !cpu.a;
    cpu.flag_n = true;
    cpu.flag_h = true;
    cpu.pc += 1;
    cpu.cycle(4);
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
    cpu.d = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
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
    cpu.set_de(0xff88 as u16);
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
    let mask = 1u8 << ((7) & 7);
    cpu.flag_z = (cpu.h & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 0x2;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc2dd, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function11b920(cpu: &mut Cpu) {
    cpu.pc = 0x7920;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile46_init_jumptable(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x05;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xa81f);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xc626);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(8 as u16);
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
    crate::game::home::sram::close_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function118000(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function11b93b(cpu: &mut Cpu) {
    cpu.pc = 0x793b;
    cpu.a = 0x05;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xa800, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xa823);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xc608);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(crate::defs::TRADE_CORNER_REQUEST_LENGTH as u16);
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
    crate::game::home::sram::close_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = ((0xc608u32) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd20, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = (((0xc608u32) >> 8) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte({ const N: i32 = (0xcd20 as i32) + (1 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = ((0xc611u32) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd22, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = (((0xc611u32) >> 8) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte({ const N: i32 = (0xcd22 as i32) + (1 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = ((0xc641u32) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd24, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = (((0xc641u32) >> 8) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte({ const N: i32 = (0xcd24 as i32) + (1 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = ((0xc646u32) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd26, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = (((0xc646u32) >> 8) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte({ const N: i32 = (0xcd26 as i32) + (1 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = ((0xc64bu32) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd28, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = (((0xc64bu32) >> 8) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte({ const N: i32 = (0xcd28 as i32) + (1 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    add_mobile_mon_to_party(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x05;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x4a58);
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

pub(crate) fn add_mobile_mon_to_party(cpu: &mut Cpu) {
    cpu.pc = 0x798f;
    cpu.set_hl(0xdcd7);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    let addr = cpu.hl();
    let value = cpu.read_byte(addr);
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.write_byte(addr, result);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xcd20);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte({ const N: i32 = (0xcd20 as i32) + (1 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.h = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_bc(0xdcd8);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.d = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    become add_mobile_mon_to_party_loop1(cpu);
}

pub(crate) fn add_mobile_mon_to_party_loop1(cpu: &mut Cpu) {
    cpu.pc = 0x79a2;
    cpu.set_bc(cpu.bc().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.d;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.d = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(12);
        become add_mobile_mon_to_party_loop1(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xd109, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.bc(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_bc(cpu.bc().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = -1i16 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.bc(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(0xdcdf);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(crate::defs::PARTYMON_STRUCT_LENGTH as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcd2a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    become add_mobile_mon_to_party_loop2(cpu);
}

pub(crate) fn add_mobile_mon_to_party_loop2(cpu: &mut Cpu) {
    cpu.pc = 0x79ba;
    let lhs = cpu.hl();
    let rhs = cpu.bc();
    let (result, carry) = lhs.overflowing_add(rhs);
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0fff) + (rhs & 0x0fff) > 0x0fff;
    cpu.flag_c = carry;
    cpu.set_hl(result);
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
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
        become add_mobile_mon_to_party_loop2(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.e = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.d = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xcd22);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte({ const N: i32 = (0xcd22 as i32) + (1 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.h = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_bc(crate::defs::PARTYMON_STRUCT_LENGTH as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xddff);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(crate::defs::NAME_LENGTH as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xcd2a);
    cpu.pc += 3;
    cpu.cycle(16);
    become add_mobile_mon_to_party_loop3(cpu);
}

pub(crate) fn add_mobile_mon_to_party_loop3(cpu: &mut Cpu) {
    cpu.pc = 0x79d8;
    let lhs = cpu.hl();
    let rhs = cpu.bc();
    let (result, carry) = lhs.overflowing_add(rhs);
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0fff) + (rhs & 0x0fff) > 0x0fff;
    cpu.flag_c = carry;
    cpu.set_hl(result);
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
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
        become add_mobile_mon_to_party_loop3(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.e = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.d = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xcd24);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte({ const N: i32 = (0xcd24 as i32) + (1 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.h = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_bc({ const N: i32 = (crate::defs::MON_NAME_LENGTH as u16 as i32) - (1 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = { let value = (0x50 as i32); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(0xde41);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(crate::defs::MON_NAME_LENGTH as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xcd2a);
    cpu.pc += 3;
    cpu.cycle(16);
    become add_mobile_mon_to_party_loop4(cpu);
}

pub(crate) fn add_mobile_mon_to_party_loop4(cpu: &mut Cpu) {
    cpu.pc = 0x79f9;
    let lhs = cpu.hl();
    let rhs = cpu.bc();
    let (result, carry) = lhs.overflowing_add(rhs);
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0fff) + (rhs & 0x0fff) > 0x0fff;
    cpu.flag_c = carry;
    cpu.set_hl(result);
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
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
        become add_mobile_mon_to_party_loop4(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.e = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.d = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xcd26);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte({ const N: i32 = (0xcd26 as i32) + (1 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.h = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_bc({ const N: i32 = (crate::defs::MON_NAME_LENGTH as u16 as i32) - (1 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = { let value = (0x50 as i32); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(0xa600);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(crate::defs::MAIL_STRUCT_LENGTH as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xcd2a);
    cpu.pc += 3;
    cpu.cycle(16);
    become add_mobile_mon_to_party_loop5(cpu);
}

pub(crate) fn add_mobile_mon_to_party_loop5(cpu: &mut Cpu) {
    cpu.pc = 0x7a1a;
    let lhs = cpu.hl();
    let rhs = cpu.bc();
    let (result, carry) = lhs.overflowing_add(rhs);
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0fff) + (rhs & 0x0fff) > 0x0fff;
    cpu.flag_c = carry;
    cpu.set_hl(result);
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
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
        become add_mobile_mon_to_party_loop5(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x00;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.e = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.d = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xcd28);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte({ const N: i32 = (0xcd28 as i32) + (1 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.h = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_bc(crate::defs::MAIL_STRUCT_LENGTH as u16);
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
    crate::game::home::sram::close_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function11ba38(cpu: &mut Cpu) {
    cpu.pc = 0x7a38;
    cpu.a = 0x03;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x6538);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
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


pub(crate) fn battle_tower_room_menu2_run_jumptable(cpu: &mut Cpu) {
    match cpu.read_byte(0xcd3c) {
        0 => become crate::game::mobile::mobile_46::function119f3f(cpu),
        1 => become crate::game::mobile::mobile_46::function119f45(cpu),
        2 => become crate::game::mobile::mobile_46::function119f56(cpu),
        3 => become crate::game::mobile::mobile_46::function119f76(cpu),
        4 => become crate::game::mobile::mobile_46::function119f98(cpu),
        5 => become crate::game::mobile::mobile_46::function11a113(cpu),
        6 => become crate::game::mobile::mobile_46::function11a129(cpu),
        7 => become crate::game::mobile::mobile_46::function11a131(cpu),
        8 => become crate::game::mobile::mobile_46::function11a13d(cpu),
        9 => become crate::game::mobile::mobile_46::function11a14b(cpu),
        10 => become crate::game::mobile::mobile_46::function11a16d(cpu),
        11 => become crate::game::mobile::mobile_46::function11a192(cpu),
        12 => become crate::game::mobile::mobile_46::function11a2e6(cpu),
        13 => become crate::game::mobile::mobile_46::function11a302(cpu),
        14 => become crate::game::mobile::mobile_46::function11a33a(cpu),
        15 => become crate::game::mobile::mobile_46::battle_tower_room_menu2_place_yes_no_menu(cpu),
        16 => become crate::game::mobile::mobile_46::battle_tower_room_menu2_update_yes_no_menu(cpu),
        17 => become crate::game::mobile::mobile_46::function11a357(cpu),
        18 => become crate::game::mobile::mobile_46::function11a36b(cpu),
        19 => become crate::game::mobile::mobile_46::function11a38d(cpu),
        20 => become crate::game::mobile::mobile_46::function11a3c5(cpu),
        21 => become crate::game::mobile::mobile_46::function11a3d9(cpu),
        22 => become crate::game::mobile::mobile_46::function11a3f9(cpu),
        23 => become crate::game::mobile::mobile_46::function11a41b(cpu),
        24 => become crate::game::mobile::mobile_46::function11a452(cpu),
        25 => become crate::game::mobile::mobile_46::function11a47a(cpu),
        26 => become crate::game::mobile::mobile_46::function11a488(cpu),
        27 => become crate::game::mobile::mobile_46::function11a49e(cpu),
        28 => become crate::game::mobile::mobile_46::function11a4db(cpu),
        29 => become crate::game::mobile::mobile_46::function11a4e8(cpu),
        30 => become crate::game::mobile::mobile_46::function11a4fe(cpu),
        31 => become crate::game::mobile::mobile_46::function11a466(cpu),
        32 => become crate::game::mobile::mobile_46::function11a47a(cpu),
        n => panic!("unsupported wBattleTowerRoomMenu2JumptableIndex state in battle_tower_room_menu2_run_jumptable: {}", n),
    }
}

pub(crate) fn battle_tower_room_menu_jumptable(cpu: &mut Cpu) {
    match cpu.read_byte(0xcf66) {
        0 => become crate::game::mobile::mobile_46::battle_tower_room_menu_pick_level_message(cpu),
        1 => become crate::game::mobile::mobile_46::battle_tower_room_menu_place_pick_level_menu(cpu),
        2 => become crate::game::mobile::mobile_46::battle_tower_room_menu_update_pick_level_menu(cpu),
        3 => become crate::game::mobile::mobile_46::battle_tower_room_menu_do_nothing(cpu),
        4 => become crate::game::mobile::mobile_46::battle_tower_room_menu_party_mon_tops_this_level_message(cpu),
        5 => become crate::game::mobile::mobile_46::battle_tower_room_menu_wait_for_message(cpu),
        6 => become crate::game::mobile::mobile_46::battle_tower_room_menu_delay_restart_menu(cpu),
        7 => become crate::game::mobile::mobile_46::battle_tower_room_menu_quit_message(cpu),
        8 => become crate::game::mobile::mobile_46::battle_tower_room_menu_place_yes_no_menu(cpu),
        9 => become crate::game::mobile::mobile_46::battle_tower_room_menu_update_yes_no_menu(cpu),
        10 => become crate::game::mobile::mobile_46::battle_tower_room_menu_uber_restriction_message(cpu),
        11 => become crate::game::mobile::mobile_46::battle_tower_room_menu_wait_for_message(cpu),
        12 => become crate::game::mobile::mobile_46::battle_tower_room_menu_delay_restart_menu(cpu),
        13 => become crate::game::mobile::mobile_46::function118e76(cpu),
        14 => become crate::game::mobile::mobile_46::battle_tower_room_menu_call_room_menu2(cpu),
        15 => become crate::game::mobile::mobile_46::function118e76(cpu),
        n => panic!("unsupported wBattleTowerRoomMenuJumptableIndex state in battle_tower_room_menu_jumptable: {}", n),
    }
}

pub(crate) fn battle_tower_room_menu_write_message(cpu: &mut Cpu) {
    match cpu.read_byte(0xc31a) {
        0 => become crate::game::mobile::mobile_46::battle_tower_room_menu_write_message_do_nothing(cpu),
        1 => become crate::game::mobile::mobile_46::function11a90f(cpu),
        2 => become crate::game::mobile::mobile_46::function11a971(cpu),
        n => panic!("unsupported wc31a state in battle_tower_room_menu_write_message: {}", n),
    }
}

pub(crate) fn function1184a5(cpu: &mut Cpu) {
    match cpu.read_byte(0xcf66) {
        0 => become crate::game::mobile::mobile_46::function11886e(cpu),
        1 => become crate::game::mobile::mobile_46::function118880(cpu),
        2 => become crate::game::mobile::mobile_46::function11878d(cpu),
        3 => become crate::game::mobile::mobile_46::function1188b0(cpu),
        4 => become crate::game::mobile::mobile_46::function11878d(cpu),
        5 => become crate::game::mobile::mobile_46::function1188b8(cpu),
        6 => become crate::game::mobile::mobile_46::function11878d(cpu),
        7 => become crate::game::mobile::mobile_46::function1188c0(cpu),
        8 => become crate::game::mobile::mobile_46::function11878d(cpu),
        9 => become crate::game::mobile::mobile_46::function1188c8(cpu),
        10 => become crate::game::mobile::mobile_46::function11878d(cpu),
        11 => become crate::game::mobile::mobile_46::function118903(cpu),
        12 => become crate::game::mobile::mobile_46::set_exchange_download_url(cpu),
        13 => become crate::game::mobile::mobile_46::function11878d(cpu),
        14 => become crate::game::mobile::mobile_46::function118d80(cpu),
        15 => become crate::game::mobile::mobile_46::function118d9b(cpu),
        16 => become crate::game::mobile::mobile_46::function11878d(cpu),
        17 => become crate::game::mobile::mobile_46::function118ded(cpu),
        18 => become crate::game::mobile::mobile_46::function118e6d(cpu),
        19 => become crate::game::mobile::mobile_46::function11984e(cpu),
        20 => become crate::game::mobile::mobile_46::function118e76(cpu),
        21 => become crate::game::mobile::mobile_46::function118e7e(cpu),
        22 => become crate::game::mobile::mobile_46::function11878d(cpu),
        23 => become crate::game::mobile::mobile_46::battle_tower_room_menu_do_nothing(cpu),
        24 => become crate::game::mobile::mobile_46::function118e76(cpu),
        25 => become crate::game::mobile::mobile_46::battle_tower_room_menu_call_room_menu2(cpu),
        26 => become crate::game::mobile::mobile_46::function118e76(cpu),
        n => panic!("unsupported wBattleTowerRoomMenuJumptableIndex state in function1184a5: {}", n),
    }
}

pub(crate) fn function1184ec(cpu: &mut Cpu) {
    match cpu.read_byte(0xcf66) {
        0 => become crate::game::mobile::mobile_46::function11886e(cpu),
        1 => become crate::game::mobile::mobile_46::function118880(cpu),
        2 => become crate::game::mobile::mobile_46::function11878d(cpu),
        3 => become crate::game::mobile::mobile_46::function1188b0(cpu),
        4 => become crate::game::mobile::mobile_46::function11878d(cpu),
        5 => become crate::game::mobile::mobile_46::function1188b8(cpu),
        6 => become crate::game::mobile::mobile_46::function11878d(cpu),
        7 => become crate::game::mobile::mobile_46::function1188c0(cpu),
        8 => become crate::game::mobile::mobile_46::function11878d(cpu),
        9 => become crate::game::mobile::mobile_46::function1188c8(cpu),
        10 => become crate::game::mobile::mobile_46::function11878d(cpu),
        11 => become crate::game::mobile::mobile_46::function118903(cpu),
        12 => become crate::game::mobile::mobile_46::function119954(cpu),
        13 => become crate::game::mobile::mobile_46::function11878d(cpu),
        14 => become crate::game::mobile::mobile_46::function119973(cpu),
        15 => become crate::game::mobile::mobile_46::function11878d(cpu),
        16 => become crate::game::mobile::mobile_46::function119987(cpu),
        17 => become crate::game::mobile::mobile_46::function11878d(cpu),
        18 => become crate::game::mobile::mobile_46::function1199b4(cpu),
        19 => become crate::game::mobile::mobile_46::function1199ca(cpu),
        20 => become crate::game::mobile::mobile_46::function11878d(cpu),
        21 => become crate::game::mobile::mobile_46::function1199e2(cpu),
        22 => become crate::game::mobile::mobile_46::function119b0d(cpu),
        23 => become crate::game::mobile::mobile_46::function11878d(cpu),
        24 => become crate::game::mobile::mobile_46::function119b6b(cpu),
        25 => become crate::game::mobile::mobile_46::function119b3b(cpu),
        26 => become crate::game::mobile::mobile_46::function11878d(cpu),
        27 => become crate::game::mobile::mobile_46::function119b52(cpu),
        28 => become crate::game::mobile::mobile_46::function11878d(cpu),
        29 => become crate::game::mobile::mobile_46::function118e6d(cpu),
        30 => become crate::game::mobile::mobile_46::function11878d(cpu),
        31 => become crate::game::mobile::mobile_46::function118e76(cpu),
        32 => become crate::game::mobile::mobile_46::function118e7e(cpu),
        33 => become crate::game::mobile::mobile_46::function11878d(cpu),
        34 => become crate::game::mobile::mobile_46::battle_tower_room_menu_do_nothing(cpu),
        35 => become crate::game::mobile::mobile_46::function118e76(cpu),
        36 => become crate::game::mobile::mobile_46::battle_tower_room_menu_call_room_menu2(cpu),
        37 => become crate::game::mobile::mobile_46::function119b45(cpu),
        38 => become crate::game::mobile::mobile_46::function11878d(cpu),
        39 => become crate::game::mobile::mobile_46::function119ac9(cpu),
        40 => become crate::game::mobile::mobile_46::function118e76(cpu),
        n => panic!("unsupported wBattleTowerRoomMenuJumptableIndex state in function1184ec: {}", n),
    }
}

pub(crate) fn function11857c(cpu: &mut Cpu) {
    match cpu.read_byte(0xcf66) {
        0 => become crate::game::mobile::mobile_46::function11886e(cpu),
        1 => become crate::game::mobile::mobile_46::function118880(cpu),
        2 => become crate::game::mobile::mobile_46::function11878d(cpu),
        3 => become crate::game::mobile::mobile_46::function1188b0(cpu),
        4 => become crate::game::mobile::mobile_46::function11878d(cpu),
        5 => become crate::game::mobile::mobile_46::function1188b8(cpu),
        6 => become crate::game::mobile::mobile_46::function11878d(cpu),
        7 => become crate::game::mobile::mobile_46::function1188c0(cpu),
        8 => become crate::game::mobile::mobile_46::function11878d(cpu),
        9 => become crate::game::mobile::mobile_46::function1188c8(cpu),
        10 => become crate::game::mobile::mobile_46::function11878d(cpu),
        11 => become crate::game::mobile::mobile_46::function118903(cpu),
        12 => become crate::game::mobile::mobile_46::set_battle_download_url(cpu),
        13 => become crate::game::mobile::mobile_46::function11878d(cpu),
        14 => become crate::game::mobile::mobile_46::function11891c(cpu),
        15 => become crate::game::mobile::mobile_46::function1198ee(cpu),
        16 => become crate::game::mobile::mobile_46::function1198f7(cpu),
        17 => become crate::game::mobile::mobile_46::function11878d(cpu),
        18 => become crate::game::mobile::mobile_46::function119937(cpu),
        19 => become crate::game::mobile::mobile_46::function118e6d(cpu),
        20 => become crate::game::mobile::mobile_46::function11878d(cpu),
        21 => become crate::game::mobile::mobile_46::function118e76(cpu),
        22 => become crate::game::mobile::mobile_46::function118e7e(cpu),
        23 => become crate::game::mobile::mobile_46::function11878d(cpu),
        24 => become crate::game::mobile::mobile_46::battle_tower_room_menu_do_nothing(cpu),
        25 => become crate::game::mobile::mobile_46::function118e76(cpu),
        26 => become crate::game::mobile::mobile_46::battle_tower_room_menu_call_room_menu2(cpu),
        27 => become crate::game::mobile::mobile_46::function118e76(cpu),
        n => panic!("unsupported wBattleTowerRoomMenuJumptableIndex state in function11857c: {}", n),
    }
}

pub(crate) fn function1185c3(cpu: &mut Cpu) {
    match cpu.read_byte(0xcf66) {
        0 => become crate::game::mobile::mobile_46::function11886e(cpu),
        1 => become crate::game::mobile::mobile_46::function118880(cpu),
        2 => become crate::game::mobile::mobile_46::function11878d(cpu),
        3 => become crate::game::mobile::mobile_46::function1188b0(cpu),
        4 => become crate::game::mobile::mobile_46::function11878d(cpu),
        5 => become crate::game::mobile::mobile_46::function1188b8(cpu),
        6 => become crate::game::mobile::mobile_46::function11878d(cpu),
        7 => become crate::game::mobile::mobile_46::function1188c0(cpu),
        8 => become crate::game::mobile::mobile_46::function11878d(cpu),
        9 => become crate::game::mobile::mobile_46::function1188c8(cpu),
        10 => become crate::game::mobile::mobile_46::function11878d(cpu),
        11 => become crate::game::mobile::mobile_46::function118903(cpu),
        12 => become crate::game::mobile::mobile_46::set_news_download_url(cpu),
        13 => become crate::game::mobile::mobile_46::function11878d(cpu),
        14 => become crate::game::mobile::mobile_46::function118e92(cpu),
        15 => become crate::game::mobile::mobile_46::function11878d(cpu),
        16 => become crate::game::mobile::mobile_46::function118eb0(cpu),
        17 => become crate::game::mobile::mobile_46::function118ec6(cpu),
        18 => become crate::game::mobile::mobile_46::function118f0d(cpu),
        19 => become crate::game::mobile::mobile_46::function118f14(cpu),
        20 => become crate::game::mobile::mobile_46::function118f5e(cpu),
        21 => become crate::game::mobile::mobile_46::function11878d(cpu),
        22 => become crate::game::mobile::mobile_46::function118fc0(cpu),
        23 => become crate::game::mobile::mobile_46::function11878d(cpu),
        24 => become crate::game::mobile::mobile_46::function119054(cpu),
        25 => become crate::game::mobile::mobile_46::function1190d0(cpu),
        26 => become crate::game::mobile::mobile_46::function11878d(cpu),
        27 => become crate::game::mobile::mobile_46::function1190ec(cpu),
        28 => become crate::game::mobile::mobile_46::function118e6d(cpu),
        29 => become crate::game::mobile::mobile_46::function11878d(cpu),
        30 => become crate::game::mobile::mobile_46::function118e76(cpu),
        31 => become crate::game::mobile::mobile_46::function118e7e(cpu),
        32 => become crate::game::mobile::mobile_46::function11878d(cpu),
        33 => become crate::game::mobile::mobile_46::battle_tower_room_menu_do_nothing(cpu),
        34 => become crate::game::mobile::mobile_46::function118e76(cpu),
        35 => become crate::game::mobile::mobile_46::battle_tower_room_menu_call_room_menu2(cpu),
        36 => become crate::game::mobile::mobile_46::battle_tower_room_menu_quit_message(cpu),
        37 => become crate::game::mobile::mobile_46::battle_tower_room_menu_place_yes_no_menu(cpu),
        38 => become crate::game::mobile::mobile_46::battle_tower_room_menu_update_yes_no_menu(cpu),
        39 => become crate::game::mobile::mobile_46::function11914e(cpu),
        40 => become crate::game::mobile::mobile_46::function118e76(cpu),
        n => panic!("unsupported wBattleTowerRoomMenuJumptableIndex state in function1185c3: {}", n),
    }
}

pub(crate) fn function118624(cpu: &mut Cpu) {
    match cpu.read_byte(0xcf66) {
        0 => become crate::game::mobile::mobile_46::function118866(cpu),
        1 => become crate::game::mobile::mobile_46::function118880(cpu),
        2 => become crate::game::mobile::mobile_46::function11878d(cpu),
        3 => become crate::game::mobile::mobile_46::function1188b0(cpu),
        4 => become crate::game::mobile::mobile_46::function11878d(cpu),
        5 => become crate::game::mobile::mobile_46::function1188b8(cpu),
        6 => become crate::game::mobile::mobile_46::function11878d(cpu),
        7 => become crate::game::mobile::mobile_46::function1188c0(cpu),
        8 => become crate::game::mobile::mobile_46::function11878d(cpu),
        9 => become crate::game::mobile::mobile_46::function1188c8(cpu),
        10 => become crate::game::mobile::mobile_46::function11878d(cpu),
        11 => become crate::game::mobile::mobile_46::function118903(cpu),
        12 => become crate::game::mobile::mobile_46::set_news_download_url(cpu),
        13 => become crate::game::mobile::mobile_46::function11878d(cpu),
        14 => become crate::game::mobile::mobile_46::function118e92(cpu),
        15 => become crate::game::mobile::mobile_46::function11878d(cpu),
        16 => become crate::game::mobile::mobile_46::function11915d(cpu),
        17 => become crate::game::mobile::mobile_46::function118f68(cpu),
        18 => become crate::game::mobile::mobile_46::function11878d(cpu),
        19 => become crate::game::mobile::mobile_46::function119009(cpu),
        20 => become crate::game::mobile::mobile_46::function11878d(cpu),
        21 => become crate::game::mobile::mobile_46::function119054(cpu),
        22 => become crate::game::mobile::mobile_46::function118e6d(cpu),
        23 => become crate::game::mobile::mobile_46::function11878d(cpu),
        24 => become crate::game::mobile::mobile_46::function118e76(cpu),
        25 => become crate::game::mobile::mobile_46::function118e7e(cpu),
        26 => become crate::game::mobile::mobile_46::function11878d(cpu),
        27 => become crate::game::mobile::mobile_46::battle_tower_room_menu_do_nothing(cpu),
        28 => become crate::game::mobile::mobile_46::function118e76(cpu),
        29 => become crate::game::mobile::mobile_46::battle_tower_room_menu_call_room_menu2(cpu),
        30 => become crate::game::mobile::mobile_46::function118e76(cpu),
        n => panic!("unsupported wBattleTowerRoomMenuJumptableIndex state in function118624: {}", n),
    }
}

pub(crate) fn function118671(cpu: &mut Cpu) {
    match cpu.read_byte(0xcf66) {
        0 => become crate::game::mobile::mobile_46::function118866(cpu),
        1 => become crate::game::mobile::mobile_46::function118880(cpu),
        2 => become crate::game::mobile::mobile_46::function11878d(cpu),
        3 => become crate::game::mobile::mobile_46::function1188b0(cpu),
        4 => become crate::game::mobile::mobile_46::function11878d(cpu),
        5 => become crate::game::mobile::mobile_46::function1188b8(cpu),
        6 => become crate::game::mobile::mobile_46::function11878d(cpu),
        7 => become crate::game::mobile::mobile_46::function1188c0(cpu),
        8 => become crate::game::mobile::mobile_46::function11878d(cpu),
        9 => become crate::game::mobile::mobile_46::function1188c8(cpu),
        10 => become crate::game::mobile::mobile_46::function11878d(cpu),
        11 => become crate::game::mobile::mobile_46::function118903(cpu),
        12 => become crate::game::mobile::mobile_46::function119380(cpu),
        13 => become crate::game::mobile::mobile_46::function119388(cpu),
        14 => become crate::game::mobile::mobile_46::function1193a0(cpu),
        15 => become crate::game::mobile::mobile_46::function11878d(cpu),
        16 => become crate::game::mobile::mobile_46::function118e6d(cpu),
        17 => become crate::game::mobile::mobile_46::function11878d(cpu),
        18 => become crate::game::mobile::mobile_46::function118e76(cpu),
        19 => become crate::game::mobile::mobile_46::function118e7e(cpu),
        20 => become crate::game::mobile::mobile_46::function11878d(cpu),
        21 => become crate::game::mobile::mobile_46::battle_tower_room_menu_do_nothing(cpu),
        22 => become crate::game::mobile::mobile_46::function118e76(cpu),
        23 => become crate::game::mobile::mobile_46::battle_tower_room_menu_call_room_menu2(cpu),
        24 => become crate::game::mobile::mobile_46::function118e76(cpu),
        n => panic!("unsupported wBattleTowerRoomMenuJumptableIndex state in function118671: {}", n),
    }
}

pub(crate) fn function1186b2(cpu: &mut Cpu) {
    match cpu.read_byte(0xcf66) {
        0 => become crate::game::mobile::mobile_46::function118866(cpu),
        1 => become crate::game::mobile::mobile_46::function118880(cpu),
        2 => become crate::game::mobile::mobile_46::function11878d(cpu),
        3 => become crate::game::mobile::mobile_46::function1188b0(cpu),
        4 => become crate::game::mobile::mobile_46::function11878d(cpu),
        5 => become crate::game::mobile::mobile_46::function1188b8(cpu),
        6 => become crate::game::mobile::mobile_46::function11878d(cpu),
        7 => become crate::game::mobile::mobile_46::function1188c0(cpu),
        8 => become crate::game::mobile::mobile_46::function11878d(cpu),
        9 => become crate::game::mobile::mobile_46::function1188c8(cpu),
        10 => become crate::game::mobile::mobile_46::function11878d(cpu),
        11 => become crate::game::mobile::mobile_46::function118903(cpu),
        12 => become crate::game::mobile::mobile_46::function119380(cpu),
        13 => become crate::game::mobile::mobile_46::function1193e3(cpu),
        14 => become crate::game::mobile::mobile_46::function1193fb(cpu),
        15 => become crate::game::mobile::mobile_46::function11878d(cpu),
        16 => become crate::game::mobile::mobile_46::function119413(cpu),
        17 => become crate::game::mobile::mobile_46::function118e6d(cpu),
        18 => become crate::game::mobile::mobile_46::function11878d(cpu),
        19 => become crate::game::mobile::mobile_46::function118e76(cpu),
        20 => become crate::game::mobile::mobile_46::function118e7e(cpu),
        21 => become crate::game::mobile::mobile_46::function11878d(cpu),
        22 => become crate::game::mobile::mobile_46::battle_tower_room_menu_do_nothing(cpu),
        23 => become crate::game::mobile::mobile_46::function118e76(cpu),
        24 => become crate::game::mobile::mobile_46::battle_tower_room_menu_call_room_menu2(cpu),
        25 => become crate::game::mobile::mobile_46::function118e76(cpu),
        n => panic!("unsupported wBattleTowerRoomMenuJumptableIndex state in function1186b2: {}", n),
    }
}

pub(crate) fn function1186f5(cpu: &mut Cpu) {
    match cpu.read_byte(0xcf66) {
        0 => become crate::game::mobile::mobile_46::function11886a(cpu),
        1 => become crate::game::mobile::mobile_46::function118880(cpu),
        2 => become crate::game::mobile::mobile_46::function11878d(cpu),
        3 => become crate::game::mobile::mobile_46::function1188b0(cpu),
        4 => become crate::game::mobile::mobile_46::function11878d(cpu),
        5 => become crate::game::mobile::mobile_46::function1188b8(cpu),
        6 => become crate::game::mobile::mobile_46::function11878d(cpu),
        7 => become crate::game::mobile::mobile_46::function1188c0(cpu),
        8 => become crate::game::mobile::mobile_46::function11878d(cpu),
        9 => become crate::game::mobile::mobile_46::function1188c8(cpu),
        10 => become crate::game::mobile::mobile_46::function11878d(cpu),
        11 => become crate::game::mobile::mobile_46::function118903(cpu),
        12 => become crate::game::mobile::mobile_46::set_stadium_download_url(cpu),
        13 => become crate::game::mobile::mobile_46::function11878d(cpu),
        14 => become crate::game::mobile::mobile_46::function119451(cpu),
        15 => become crate::game::mobile::mobile_46::function1195f8(cpu),
        16 => become crate::game::mobile::mobile_46::function119612(cpu),
        17 => become crate::game::mobile::mobile_46::function119629(cpu),
        18 => become crate::game::mobile::mobile_46::function119648(cpu),
        19 => become crate::game::mobile::mobile_46::function11878d(cpu),
        20 => become crate::game::mobile::mobile_46::function118e6d(cpu),
        21 => become crate::game::mobile::mobile_46::function11878d(cpu),
        22 => become crate::game::mobile::mobile_46::function118e76(cpu),
        23 => become crate::game::mobile::mobile_46::function118e7e(cpu),
        24 => become crate::game::mobile::mobile_46::function11878d(cpu),
        25 => become crate::game::mobile::mobile_46::battle_tower_room_menu_do_nothing(cpu),
        26 => become crate::game::mobile::mobile_46::function11967d(cpu),
        27 => become crate::game::mobile::mobile_46::function119685(cpu),
        28 => become crate::game::mobile::mobile_46::function119665(cpu),
        29 => become crate::game::mobile::mobile_46::function11966d(cpu),
        30 => become crate::game::mobile::mobile_46::function118e76(cpu),
        31 => become crate::game::mobile::mobile_46::battle_tower_room_menu_call_room_menu2(cpu),
        32 => become crate::game::mobile::mobile_46::function118e76(cpu),
        n => panic!("unsupported wBattleTowerRoomMenuJumptableIndex state in function1186f5: {}", n),
    }
}

pub(crate) fn function118746(cpu: &mut Cpu) {
    match cpu.read_byte(0xcf66) {
        0 => become crate::game::mobile::mobile_46::function11886e(cpu),
        1 => become crate::game::mobile::mobile_46::function118880(cpu),
        2 => become crate::game::mobile::mobile_46::function11878d(cpu),
        3 => become crate::game::mobile::mobile_46::function1188b0(cpu),
        4 => become crate::game::mobile::mobile_46::function11878d(cpu),
        5 => become crate::game::mobile::mobile_46::function1188b8(cpu),
        6 => become crate::game::mobile::mobile_46::function11878d(cpu),
        7 => become crate::game::mobile::mobile_46::function1188c0(cpu),
        8 => become crate::game::mobile::mobile_46::function11878d(cpu),
        9 => become crate::game::mobile::mobile_46::function1188c8(cpu),
        10 => become crate::game::mobile::mobile_46::function11878d(cpu),
        11 => become crate::game::mobile::mobile_46::function118903(cpu),
        12 => become crate::game::mobile::mobile_46::set_odd_egg_download_url(cpu),
        13 => become crate::game::mobile::mobile_46::function11878d(cpu),
        14 => become crate::game::mobile::mobile_46::function1196f2(cpu),
        15 => become crate::game::mobile::mobile_46::function1197c9(cpu),
        16 => become crate::game::mobile::mobile_46::function1197dc(cpu),
        17 => become crate::game::mobile::mobile_46::function11878d(cpu),
        18 => become crate::game::mobile::mobile_46::function118e6d(cpu),
        19 => become crate::game::mobile::mobile_46::function11878d(cpu),
        20 => become crate::game::mobile::mobile_46::function119800(cpu),
        21 => become crate::game::mobile::mobile_46::function118e76(cpu),
        22 => become crate::game::mobile::mobile_46::function118e7e(cpu),
        23 => become crate::game::mobile::mobile_46::function11878d(cpu),
        24 => become crate::game::mobile::mobile_46::battle_tower_room_menu_do_nothing(cpu),
        25 => become crate::game::mobile::mobile_46::function118e76(cpu),
        26 => become crate::game::mobile::mobile_46::battle_tower_room_menu_call_room_menu2(cpu),
        27 => become crate::game::mobile::mobile_46::function118e76(cpu),
        n => panic!("unsupported wBattleTowerRoomMenuJumptableIndex state in function118746: {}", n),
    }
}

pub(crate) fn function11ad6e(cpu: &mut Cpu) {
    match cpu.read_byte(0xcf63) {
        0 => become crate::game::mobile::mobile_46::function11b082(cpu),
        1 => become crate::game::mobile::mobile_46::function11b0ff(cpu),
        2 => become crate::game::mobile::mobile_46::function11ad95(cpu),
        3 => become crate::game::mobile::mobile_46::function11adc4(cpu),
        4 => become crate::game::mobile::mobile_46::function11ae4e(cpu),
        5 => become crate::game::mobile::mobile_46::function11ae98(cpu),
        6 => become crate::game::mobile::mobile_46::function11ad8f(cpu),
        7 => become crate::game::mobile::mobile_46::function11af04(cpu),
        8 => become crate::game::mobile::mobile_46::function11af4e(cpu),
        n => panic!("unsupported wJumptableIndex state in function11ad6e: {}", n),
    }
}

pub(crate) fn function11b5e8_run_jumptable(cpu: &mut Cpu) {
    match cpu.read_byte(0xcf63) {
        0 => become crate::game::mobile::mobile_46::function11b66d(cpu),
        1 => become crate::game::mobile::mobile_46::function11b6b3(cpu),
        n => panic!("unsupported wJumptableIndex state in function11b5e8_run_jumptable: {}", n),
    }
}

pub(crate) fn trade_corner_hold_mon_run_jumptable_iterate_jumptable(cpu: &mut Cpu) {
    match cpu.read_byte(0xcf63) {
        0 => become crate::game::mobile::mobile_46::trade_corner_hold_mon_prepare_for_upload(cpu),
        1 => become crate::game::mobile::mobile_46::function11b570(cpu),
        2 => become crate::game::mobile::mobile_46::trade_corner_hold_mon_remove_from_party(cpu),
        3 => become crate::game::mobile::mobile_46::trade_corner_hold_mon_success(cpu),
        4 => become crate::game::mobile::mobile_46::trade_corner_hold_mon_noop(cpu),
        n => panic!("unsupported wJumptableIndex state in trade_corner_hold_mon_run_jumptable_iterate_jumptable: {}", n),
    }
}
