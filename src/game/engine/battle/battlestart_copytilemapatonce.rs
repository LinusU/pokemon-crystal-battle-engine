use crate::runtime::Cpu;

pub(crate) fn battle_start_copy_tilemap_at_once(cpu: &mut Cpu) {
    cpu.pc = 0x4f4f;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::tilemap::cgb_only_copy_tilemap_at_once(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}
