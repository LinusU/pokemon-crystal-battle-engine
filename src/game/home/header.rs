use crate::runtime::Cpu;

pub(crate) fn __top_level(cpu: &mut Cpu) {
    cpu.disable_interrupts_after_next_instruction();
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.cycle(16);
    become start(cpu);
}

pub fn far_call(cpu: &mut Cpu) {
    cpu.pc = 0x0008;
    cpu.cycle(16);
    become crate::game::home::farcall::far_call_hl(cpu);
}

pub fn bankswitch(cpu: &mut Cpu) {
    cpu.pc = 0x0010;
    cpu.write_byte(0xff9d, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.write_byte(crate::defs::R_ROMB as u16, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
    return;
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    (crate::dispatch::resolve(0, 0x38 as u16))(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    (crate::dispatch::resolve(0, 0x38 as u16))(cpu);
    assert_eq!(cpu.pc, next_pc);
    become jump_table(cpu);
}

pub fn start(cpu: &mut Cpu) {
    cpu.pc = 0x0100;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.cycle(16);
    become crate::game::home::init::_start(cpu);
}

pub(crate) fn jump_table(cpu: &mut Cpu) {
    let index = cpu.a as u16;
    let table = cpu.hl();

    let bank = cpu.read_byte(0xff9d);

    let addr = u16::from_le_bytes([
        cpu.read_byte(table + index * 2),
        cpu.read_byte(table + index * 2 + 1),
    ]);

    become (crate::dispatch::resolve(bank, addr))(cpu);
}
