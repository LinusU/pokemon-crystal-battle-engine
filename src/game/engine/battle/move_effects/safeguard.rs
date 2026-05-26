use crate::runtime::Cpu;

pub(crate) fn battle_command_safeguard(cpu: &mut Cpu) {
    cpu.pc = 0x7939;
    cpu.set_hl(0xc6ff);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xc701);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xffe4);
    cpu.pc += 2;
    cpu.cycle(12);
    let value = cpu.a;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become battle_command_safeguard_ok(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xc700);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xc705);
    cpu.pc += 3;
    cpu.cycle(12);
    become battle_command_safeguard_ok(cpu);
}

pub(crate) fn battle_command_safeguard_ok(cpu: &mut Cpu) {
    cpu.pc = 0x794a;
    let mask = 1u8 << ((crate::defs::SCREENS_SAFEGUARD as u8) & 7);
    let value = cpu.read_byte(cpu.hl());
    cpu.flag_z = (value & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(12);
    if !cpu.flag_z {
        cpu.cycle(12);
        become battle_command_safeguard_failed(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let mask = 1u8 << ((crate::defs::SCREENS_SAFEGUARD as u8) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value |= mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.a = 5;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::engine::battle::effect_commands::animate_current_move(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0x571c);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become crate::game::home::battle::std_battle_textbox(cpu);
}

pub(crate) fn battle_command_safeguard_failed(cpu: &mut Cpu) {
    cpu.pc = 0x795c;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::engine::battle::effect_commands::animate_failed_move(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(16);
    become crate::game::engine::battle::effect_commands::print_but_it_failed(cpu);
}
