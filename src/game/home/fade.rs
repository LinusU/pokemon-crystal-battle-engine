use crate::runtime::Cpu;

pub fn time_of_day_fade(cpu: &mut Cpu) {
    cpu.pc = 0x048c;
    cpu.a = cpu.read_byte(0xd841);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl(0x0538);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.b;
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
    if !cpu.flag_c {
        cpu.cycle(12);
        become time_of_day_fade_okay(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.h;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.h = result;
    cpu.pc += 1;
    cpu.cycle(4);
    become time_of_day_fade_okay(cpu);
}

pub(crate) fn time_of_day_fade_okay(cpu: &mut Cpu) {
    cpu.pc = 0x0499;
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_BGP as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_OBP0 as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_OBP1 as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub fn rotate_four_palettes_right(cpu: &mut Cpu) {
    cpu.pc = 0x04a3;
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
        become rotate_four_palettes_right_dmg(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x0517);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = 4;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become rotate_palettes_right(cpu);
}

pub(crate) fn rotate_four_palettes_right_dmg(cpu: &mut Cpu) {
    cpu.pc = 0x04af;
    cpu.set_hl(0x052f);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = 4;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become rotate_palettes_right(cpu);
}

pub fn rotate_three_palettes_right(cpu: &mut Cpu) {
    cpu.pc = 0x04b6;
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
        become rotate_three_palettes_right_dmg(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x0526);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = 3;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become rotate_palettes_right(cpu);
}

pub(crate) fn rotate_three_palettes_right_dmg(cpu: &mut Cpu) {
    cpu.pc = 0x04c2;
    cpu.set_hl(0x053e);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = 3;
    cpu.pc += 2;
    cpu.cycle(8);
    become rotate_palettes_right(cpu);
}

pub fn rotate_palettes_right(cpu: &mut Cpu) {
    cpu.pc = 0x04c7;
    cpu.push_word(cpu.de());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::palettes::dmg_to_cgb_bg_pals(cpu);
    assert_eq!(cpu.pc, next_pc);
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
    crate::game::home::palettes::dmg_to_cgb_obj_pals(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.c = 8;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::delay::delay_frames(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_de(value);
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
        become rotate_palettes_right(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub fn rotate_four_palettes_left(cpu: &mut Cpu) {
    cpu.pc = 0x04dd;
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
        become rotate_four_palettes_left_dmg(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl({ const N: i32 = (0x0523 as i32) - (1 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = 4;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become rotate_palettes_left(cpu);
}

pub(crate) fn rotate_four_palettes_left_dmg(cpu: &mut Cpu) {
    cpu.pc = 0x04e9;
    cpu.set_hl({ const N: i32 = (0x053b as i32) - (1 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = 4;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become rotate_palettes_left(cpu);
}

pub fn rotate_three_palettes_left(cpu: &mut Cpu) {
    cpu.pc = 0x04f0;
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
        become rotate_three_palettes_left_dmg(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl({ const N: i32 = (0x052c as i32) - (1 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = 3;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become rotate_palettes_left(cpu);
}

pub(crate) fn rotate_three_palettes_left_dmg(cpu: &mut Cpu) {
    cpu.pc = 0x04fc;
    cpu.set_hl({ const N: i32 = (0x0544 as i32) - (1 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = 3;
    cpu.pc += 2;
    cpu.cycle(8);
    become rotate_palettes_left(cpu);
}

pub fn rotate_palettes_left(cpu: &mut Cpu) {
    cpu.pc = 0x0501;
    cpu.push_word(cpu.de());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.d = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::palettes::dmg_to_cgb_obj_pals(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::palettes::dmg_to_cgb_bg_pals(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.c = 8;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::delay::delay_frames(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_de(value);
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
        become rotate_palettes_left(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

