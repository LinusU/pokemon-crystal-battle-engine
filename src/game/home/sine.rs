use crate::runtime::Cpu;

pub fn cosine(cpu: &mut Cpu) {
    cpu.pc = 0x1b0f;
    let value = 0b010000;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_add(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) > 0x0f;
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    become sine(cpu);
}

pub fn sine(cpu: &mut Cpu) {
    cpu.pc = 0x1b11;
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xff9d);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = 0x02;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::bankswitch(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::engine::math::sine::_sine(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::bankswitch(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}
