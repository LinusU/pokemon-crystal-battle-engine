use crate::runtime::Cpu;

pub(crate) fn battle_command_start_sandstorm(cpu: &mut Cpu) {
    cpu.pc = 0x76f8;
    cpu.a = cpu.read_byte(0xc70a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = crate::defs::WEATHER_SANDSTORM as u8;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become battle_command_start_sandstorm_failed(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = crate::defs::WEATHER_SANDSTORM as u8;
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
    cpu.set_hl(0x56e4);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become crate::game::home::battle::std_battle_textbox(cpu);
}

pub(crate) fn battle_command_start_sandstorm_failed(cpu: &mut Cpu) {
    cpu.pc = 0x7712;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::engine::battle::effect_commands::animate_failed_move(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(16);
    become crate::game::engine::battle::effect_commands::print_but_it_failed(cpu);
}
