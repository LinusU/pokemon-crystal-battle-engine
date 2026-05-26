use crate::runtime::Cpu;

pub fn copy_name1(cpu: &mut Cpu) {
    cpu.pc = 0x30d6;
    cpu.set_hl(0xd086);
    cpu.pc += 3;
    cpu.cycle(12);
    become copy_name2(cpu);
}

pub fn copy_name2(cpu: &mut Cpu) {
    cpu.pc = 0x30d9;
    become copy_name2_loop(cpu);
}

pub(crate) fn copy_name2_loop(cpu: &mut Cpu) {
    cpu.pc = 0x30d9;
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
    let value = { let value = (0x50 as i32); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
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
        become copy_name2_loop(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}
