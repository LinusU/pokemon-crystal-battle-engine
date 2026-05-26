use crate::runtime::Cpu;

pub(crate) fn battle_command_mist(cpu: &mut Cpu) {
    cpu.pc = 0x6c7e;
    cpu.a = crate::defs::BATTLE_VARS_SUBSTATUS4 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::battle_vars::get_battle_var_addr(cpu);
    assert_eq!(cpu.pc, next_pc);
    let mask = 1u8 << ((crate::defs::SUBSTATUS_MIST as u8) & 7);
    let value = cpu.read_byte(cpu.hl());
    cpu.flag_z = (value & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(12);
    if !cpu.flag_z {
        cpu.cycle(12);
        become battle_command_mist_already_mist(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let mask = 1u8 << ((crate::defs::SUBSTATUS_MIST as u8) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value |= mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::engine::battle::effect_commands::animate_current_move(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0x530b);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become crate::game::home::battle::std_battle_textbox(cpu);
}

pub(crate) fn battle_command_mist_already_mist(cpu: &mut Cpu) {
    cpu.pc = 0x6c92;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::engine::battle::effect_commands::animate_failed_move(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(16);
    become crate::game::engine::battle::effect_commands::print_but_it_failed(cpu);
}
