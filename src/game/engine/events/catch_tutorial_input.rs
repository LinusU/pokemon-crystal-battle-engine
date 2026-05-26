use crate::runtime::Cpu;

pub fn _dude_auto_input_a(cpu: &mut Cpu) {
    cpu.pc = 0x628a;
    cpu.set_hl(0x629f);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(12);
    become _dude_auto_input(cpu);
}

pub(crate) fn _dude_auto_input_right_a(cpu: &mut Cpu) {
    cpu.pc = 0x628f;
    cpu.set_hl(0x62a5);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(12);
    become _dude_auto_input(cpu);
}

pub(crate) fn _dude_auto_input_down_a(cpu: &mut Cpu) {
    cpu.pc = 0x6294;
    cpu.set_hl(0x62af);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(12);
    become _dude_auto_input(cpu);
}

pub(crate) fn _dude_auto_input(cpu: &mut Cpu) {
    cpu.pc = 0x6299;
    cpu.a = 0x77;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::joypad::start_auto_input(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

