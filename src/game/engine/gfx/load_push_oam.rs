use crate::runtime::Cpu;

pub fn write_oamdma_code_to_hram(cpu: &mut Cpu) {
    cpu.pc = 0x4031;
    cpu.c = ((0xff80u32) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.b = { let value = ((0x4049 as i32) - (0x403f as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x403f);
    cpu.pc += 3;
    cpu.cycle(12);
    become write_oamdma_code_to_hram_copy(cpu);
}

pub(crate) fn write_oamdma_code_to_hram_copy(cpu: &mut Cpu) {
    cpu.pc = 0x4038;
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(0xff00u16 + u16::from(cpu.c), cpu.a);
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
        become write_oamdma_code_to_hram_copy(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn oamdma_code(cpu: &mut Cpu) {
    cpu.pc = 0x403f;
    become h_transfer_shadow_oam(cpu);
}

pub fn h_transfer_shadow_oam(cpu: &mut Cpu) {
    cpu.pc = 0xff80;
    cpu.a = (((0xc400u32) >> 8) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_DMA as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = crate::defs::OAM_COUNT as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    become h_transfer_shadow_oam_wait(cpu);
}

pub(crate) fn h_transfer_shadow_oam_wait(cpu: &mut Cpu) {
    cpu.pc = 0xff86;
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
        become h_transfer_shadow_oam_wait(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
    return;
}

