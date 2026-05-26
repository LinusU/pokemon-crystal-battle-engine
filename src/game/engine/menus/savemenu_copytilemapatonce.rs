use crate::runtime::Cpu;

pub(crate) fn save_menu_copy_tilemap_at_once(cpu: &mut Cpu) {
    cpu.pc = 0x4f45;
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
        cpu.cycle(16);
        become crate::game::home::tilemap::wait_bg_map(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
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
    cpu.a = cpu.read_byte(0xffde);
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
    cpu.write_byte(0xffde, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    become save_menu_copy_tilemap_at_once_wait(cpu);
}

pub(crate) fn save_menu_copy_tilemap_at_once_wait(cpu: &mut Cpu) {
    cpu.pc = 0x4f57;
    cpu.a = cpu.read_byte(crate::defs::R_LY as u16);
    cpu.pc += 2;
    cpu.cycle(12);
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
        become save_menu_copy_tilemap_at_once_wait(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.disable_interrupts_after_next_instruction();
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = 0x01;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_VBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.set_hl({ const N: i32 = ((((0 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (0 as i32)) + (0xcdd9 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    save_menu_copy_tilemap_at_once_copy_bg_map_via_stack(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x00;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_VBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.set_hl({ const N: i32 = ((((0 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (0 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    save_menu_copy_tilemap_at_once_copy_bg_map_via_stack(cpu);
    assert_eq!(cpu.pc, next_pc);
    become save_menu_copy_tilemap_at_once_wait2(cpu);
}

pub(crate) fn save_menu_copy_tilemap_at_once_wait2(cpu: &mut Cpu) {
    cpu.pc = 0x4f72;
    cpu.a = cpu.read_byte(crate::defs::R_LY as u16);
    cpu.pc += 2;
    cpu.cycle(12);
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
        become save_menu_copy_tilemap_at_once_wait2(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.enable_interrupts_after_next_instruction();
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte(0xffde, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte(0xffd4, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn save_menu_copy_tilemap_at_once_copy_bg_map_via_stack(cpu: &mut Cpu) {
    cpu.pc = 0x4f80;
    cpu.write_word(0xffd9, cpu.sp);
    cpu.pc += 3;
    cpu.cycle(20);
    cpu.sp = cpu.hl();
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte({ const N: i32 = (0xffd6 as i32) + (1 as i32); N as u16 });
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.h = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.l = 0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = crate::defs::SCREEN_HEIGHT as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xffd3, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.b = crate::defs::STAT_BUSY as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.c = ((crate::defs::R_STAT as u32) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    become save_menu_copy_tilemap_at_once_loop(cpu);
}

pub(crate) fn save_menu_copy_tilemap_at_once_loop(cpu: &mut Cpu) {
    cpu.pc = 0x4f91;
    let value = cpu.pop_word();
    cpu.set_de(value);
    cpu.pc += 1;
    cpu.cycle(12);
    become save_menu_copy_tilemap_at_once_loop_u152(cpu);
}

pub(crate) fn save_menu_copy_tilemap_at_once_loop_u152(cpu: &mut Cpu) {
    cpu.pc = 0x4f92;
    cpu.a = cpu.read_byte(0xff00u16 + u16::from(cpu.c));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.b;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(12);
        become save_menu_copy_tilemap_at_once_loop_u152(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.e);
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.l;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.l = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.d);
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.l;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.l = result;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.pop_word();
    cpu.set_de(value);
    cpu.pc += 1;
    cpu.cycle(12);
    become save_menu_copy_tilemap_at_once_loop_u153(cpu);
}

pub(crate) fn save_menu_copy_tilemap_at_once_loop_u153(cpu: &mut Cpu) {
    cpu.pc = 0x4f9b;
    cpu.a = cpu.read_byte(0xff00u16 + u16::from(cpu.c));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.b;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(12);
        become save_menu_copy_tilemap_at_once_loop_u153(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.e);
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.l;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.l = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.d);
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.l;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.l = result;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.pop_word();
    cpu.set_de(value);
    cpu.pc += 1;
    cpu.cycle(12);
    become save_menu_copy_tilemap_at_once_loop_u154(cpu);
}

pub(crate) fn save_menu_copy_tilemap_at_once_loop_u154(cpu: &mut Cpu) {
    cpu.pc = 0x4fa4;
    cpu.a = cpu.read_byte(0xff00u16 + u16::from(cpu.c));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.b;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(12);
        become save_menu_copy_tilemap_at_once_loop_u154(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.e);
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.l;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.l = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.d);
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.l;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.l = result;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.pop_word();
    cpu.set_de(value);
    cpu.pc += 1;
    cpu.cycle(12);
    become save_menu_copy_tilemap_at_once_loop_u155(cpu);
}

pub(crate) fn save_menu_copy_tilemap_at_once_loop_u155(cpu: &mut Cpu) {
    cpu.pc = 0x4fad;
    cpu.a = cpu.read_byte(0xff00u16 + u16::from(cpu.c));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.b;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(12);
        become save_menu_copy_tilemap_at_once_loop_u155(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.e);
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.l;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.l = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.d);
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.l;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.l = result;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.pop_word();
    cpu.set_de(value);
    cpu.pc += 1;
    cpu.cycle(12);
    become save_menu_copy_tilemap_at_once_loop_u156(cpu);
}

pub(crate) fn save_menu_copy_tilemap_at_once_loop_u156(cpu: &mut Cpu) {
    cpu.pc = 0x4fb6;
    cpu.a = cpu.read_byte(0xff00u16 + u16::from(cpu.c));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.b;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(12);
        become save_menu_copy_tilemap_at_once_loop_u156(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.e);
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.l;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.l = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.d);
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.l;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.l = result;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.pop_word();
    cpu.set_de(value);
    cpu.pc += 1;
    cpu.cycle(12);
    become save_menu_copy_tilemap_at_once_loop_u157(cpu);
}

pub(crate) fn save_menu_copy_tilemap_at_once_loop_u157(cpu: &mut Cpu) {
    cpu.pc = 0x4fbf;
    cpu.a = cpu.read_byte(0xff00u16 + u16::from(cpu.c));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.b;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(12);
        become save_menu_copy_tilemap_at_once_loop_u157(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.e);
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.l;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.l = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.d);
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.l;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.l = result;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.pop_word();
    cpu.set_de(value);
    cpu.pc += 1;
    cpu.cycle(12);
    become save_menu_copy_tilemap_at_once_loop_u158(cpu);
}

pub(crate) fn save_menu_copy_tilemap_at_once_loop_u158(cpu: &mut Cpu) {
    cpu.pc = 0x4fc8;
    cpu.a = cpu.read_byte(0xff00u16 + u16::from(cpu.c));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.b;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(12);
        become save_menu_copy_tilemap_at_once_loop_u158(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.e);
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.l;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.l = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.d);
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.l;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.l = result;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.pop_word();
    cpu.set_de(value);
    cpu.pc += 1;
    cpu.cycle(12);
    become save_menu_copy_tilemap_at_once_loop_u159(cpu);
}

pub(crate) fn save_menu_copy_tilemap_at_once_loop_u159(cpu: &mut Cpu) {
    cpu.pc = 0x4fd1;
    cpu.a = cpu.read_byte(0xff00u16 + u16::from(cpu.c));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.b;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(12);
        become save_menu_copy_tilemap_at_once_loop_u159(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.e);
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.l;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.l = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.d);
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.l;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.l = result;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.pop_word();
    cpu.set_de(value);
    cpu.pc += 1;
    cpu.cycle(12);
    become save_menu_copy_tilemap_at_once_loop_u160(cpu);
}

pub(crate) fn save_menu_copy_tilemap_at_once_loop_u160(cpu: &mut Cpu) {
    cpu.pc = 0x4fda;
    cpu.a = cpu.read_byte(0xff00u16 + u16::from(cpu.c));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.b;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(12);
        become save_menu_copy_tilemap_at_once_loop_u160(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.e);
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.l;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.l = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.d);
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.l;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.l = result;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.pop_word();
    cpu.set_de(value);
    cpu.pc += 1;
    cpu.cycle(12);
    become save_menu_copy_tilemap_at_once_loop_u161(cpu);
}

pub(crate) fn save_menu_copy_tilemap_at_once_loop_u161(cpu: &mut Cpu) {
    cpu.pc = 0x4fe3;
    cpu.a = cpu.read_byte(0xff00u16 + u16::from(cpu.c));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.b;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(12);
        become save_menu_copy_tilemap_at_once_loop_u161(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.e);
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.l;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.l = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.d);
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.l;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.l = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_de({ const N: i32 = (crate::defs::TILEMAP_WIDTH as u16 as i32) - (crate::defs::SCREEN_WIDTH as u16 as i32); N as u16 });
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
    cpu.a = cpu.read_byte(0xffd3);
    cpu.pc += 2;
    cpu.cycle(12);
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xffd3, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    if !cpu.flag_z {
        cpu.cycle(12);
        become save_menu_copy_tilemap_at_once_loop(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xffd9);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte({ const N: i32 = (0xffd9 as i32) + (1 as i32); N as u16 });
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.h = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.sp = cpu.hl();
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}
