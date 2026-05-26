use crate::runtime::Cpu;

pub(crate) fn battle_command_start_rain(cpu: &mut Cpu) {
    cpu.pc = 0x7bf4;
    cpu.a = crate::defs::WEATHER_RAIN as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc70a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 5;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc70b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::engine::battle::effect_commands::animate_current_move(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0x57a3);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become crate::game::home::battle::std_battle_textbox(cpu);
}
