use crate::runtime::Cpu;

pub fn function547(cpu: &mut Cpu) {
    cpu.pc = 0x0547;
    cpu.a = cpu.read_byte(0xffc6);
    cpu.pc += 2;
    cpu.cycle(12);
    let value = ((crate::defs::R_SCX as u32) & 0x00ff) as u8;
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
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xd100);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xff00u16 + u16::from(cpu.c), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub fn lcd(cpu: &mut Cpu) {
    cpu.pc = 0x0552;
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xffc6);
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
        become lcd_done(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.push_word(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(crate::defs::R_LY as u16);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.b = (((0xd100u32) >> 8) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xffc6);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xff00u16 + u16::from(cpu.c), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.pop_word();
    cpu.set_bc(value);
    cpu.pc += 1;
    cpu.cycle(12);
    become lcd_done(cpu);
}

pub(crate) fn lcd_done(cpu: &mut Cpu) {
    cpu.pc = 0x0566;
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.reti();
    return;
}

pub fn disable_lcd(cpu: &mut Cpu) {
    cpu.pc = 0x0568;
    cpu.a = cpu.read_byte(crate::defs::R_LCDC as u16);
    cpu.pc += 2;
    cpu.cycle(12);
    let mask = 1u8 << ((crate::defs::B_LCDC_ENABLE as u8) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
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
    cpu.a = cpu.read_byte(crate::defs::R_IE as u16);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    let mask = 1u8 << ((crate::defs::B_IE_VBLANK as u8) & 7);
    cpu.a &= !mask;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_IE as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    become disable_lcd_wait(cpu);
}

pub(crate) fn disable_lcd_wait(cpu: &mut Cpu) {
    cpu.pc = 0x0577;
    cpu.a = cpu.read_byte(crate::defs::R_LY as u16);
    cpu.pc += 2;
    cpu.cycle(12);
    let value = { let value = ((crate::defs::LY_VBLANK as i32) + (1 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
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
        become disable_lcd_wait(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(crate::defs::R_LCDC as u16);
    cpu.pc += 2;
    cpu.cycle(12);
    let value = !({ let value = ((crate::defs::LCDC_ON as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } });
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_LCDC as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
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
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(crate::defs::R_IE as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub fn enable_lcd(cpu: &mut Cpu) {
    cpu.pc = 0x058a;
    cpu.a = cpu.read_byte(crate::defs::R_LCDC as u16);
    cpu.pc += 2;
    cpu.cycle(12);
    let mask = 1u8 << ((crate::defs::B_LCDC_ENABLE as u8) & 7);
    cpu.a |= mask;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_LCDC as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}
