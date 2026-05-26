use crate::runtime::Cpu;

pub(crate) fn kurt_select_quantity_interpret_joypad(cpu: &mut Cpu) {
    cpu.pc = 0x7a28;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::engine::items::buy_sell_toss::buy_sell_toss_interpret_joypad(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}
