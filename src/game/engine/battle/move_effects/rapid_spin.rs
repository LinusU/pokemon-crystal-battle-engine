use crate::runtime::Cpu;

pub(crate) fn battle_command_clear_hazards(cpu: &mut Cpu) {
    cpu.pc = 0x7b39;
    cpu.a = crate::defs::BATTLE_VARS_SUBSTATUS4 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::battle_vars::get_battle_var_addr(cpu);
    assert_eq!(cpu.pc, next_pc);
    let mask = 1u8 << ((crate::defs::SUBSTATUS_LEECH_SEED as u8) & 7);
    let value = cpu.read_byte(cpu.hl());
    cpu.flag_z = (value & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(12);
    if cpu.flag_z {
        cpu.cycle(12);
        become battle_command_clear_hazards_not_leeched(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let mask = 1u8 << ((crate::defs::SUBSTATUS_LEECH_SEED as u8) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.set_hl(0x577a);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::battle::std_battle_textbox(cpu);
    assert_eq!(cpu.pc, next_pc);
    become battle_command_clear_hazards_not_leeched(cpu);
}

pub(crate) fn battle_command_clear_hazards_not_leeched(cpu: &mut Cpu) {
    cpu.pc = 0x7b4a;
    cpu.set_hl(0xc6ff);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xc730);
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
        become battle_command_clear_hazards_got_screens_wrap(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xc700);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xc731);
    cpu.pc += 3;
    cpu.cycle(12);
    become battle_command_clear_hazards_got_screens_wrap(cpu);
}

pub(crate) fn battle_command_clear_hazards_got_screens_wrap(cpu: &mut Cpu) {
    cpu.pc = 0x7b5b;
    let mask = 1u8 << ((crate::defs::SCREENS_SPIKES as u8) & 7);
    let value = cpu.read_byte(cpu.hl());
    cpu.flag_z = (value & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(12);
    if cpu.flag_z {
        cpu.cycle(12);
        become battle_command_clear_hazards_no_spikes(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let mask = 1u8 << ((crate::defs::SCREENS_SPIKES as u8) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.set_hl(0x578e);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.push_word(cpu.de());
    cpu.pc += 1;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::battle::std_battle_textbox(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_de(value);
    cpu.pc += 1;
    cpu.cycle(12);
    become battle_command_clear_hazards_no_spikes(cpu);
}

pub(crate) fn battle_command_clear_hazards_no_spikes(cpu: &mut Cpu) {
    cpu.pc = 0x7b69;
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.a;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(0x5764);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become crate::game::home::battle::std_battle_textbox(cpu);
}
