use crate::runtime::Cpu;

pub fn open_sram(cpu: &mut Cpu) {
    cpu.pc = 0x2fcb;
    let value = crate::defs::NUM_SRAM_BANKS as u8;
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
        become open_sram_valid(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become close_sram(cpu);
}

pub(crate) fn open_sram_valid(cpu: &mut Cpu) {
    cpu.pc = 0x2fd1;
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = 1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_RTCLATCH as u16, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = crate::defs::RAMG_SRAM_ENABLE as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_RAMG as u16, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte(crate::defs::R_RAMB as u16, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub fn close_sram(cpu: &mut Cpu) {
    cpu.pc = 0x2fe1;
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = crate::defs::RAMG_SRAM_DISABLE as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_RTCLATCH as u16, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(crate::defs::R_RAMG as u16, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}
