use crate::runtime::Cpu;

pub(crate) fn battle_command_perish_song(cpu: &mut Cpu) {
    cpu.pc = 0x76c2;
    cpu.set_hl(0xc668);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xc66d);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((crate::defs::SUBSTATUS_PERISH as u8) & 7);
    let value = cpu.read_byte(cpu.hl());
    cpu.flag_z = (value & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(12);
    if cpu.flag_z {
        cpu.cycle(12);
        become battle_command_perish_song_ok(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    let mask = 1u8 << ((crate::defs::SUBSTATUS_PERISH as u8) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(12);
        become battle_command_perish_song_failed(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become battle_command_perish_song_ok(cpu);
}

pub(crate) fn battle_command_perish_song_ok(cpu: &mut Cpu) {
    cpu.pc = 0x76d1;
    let mask = 1u8 << ((crate::defs::SUBSTATUS_PERISH as u8) & 7);
    let value = cpu.read_byte(cpu.hl());
    cpu.flag_z = (value & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(12);
    if !cpu.flag_z {
        cpu.cycle(12);
        become battle_command_perish_song_enemy(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let mask = 1u8 << ((crate::defs::SUBSTATUS_PERISH as u8) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value |= mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.a = 4;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc677, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    become battle_command_perish_song_enemy(cpu);
}

pub(crate) fn battle_command_perish_song_enemy(cpu: &mut Cpu) {
    cpu.pc = 0x76dc;
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    let mask = 1u8 << ((crate::defs::SUBSTATUS_PERISH as u8) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(12);
        become battle_command_perish_song_done(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let mask = 1u8 << ((crate::defs::SUBSTATUS_PERISH as u8) & 7);
    cpu.a |= mask;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 4;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc67f, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    become battle_command_perish_song_done(cpu);
}

pub(crate) fn battle_command_perish_song_done(cpu: &mut Cpu) {
    cpu.pc = 0x76e9;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::engine::battle::effect_commands::animate_current_move(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0x56c2);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become crate::game::home::battle::std_battle_textbox(cpu);
}

pub(crate) fn battle_command_perish_song_failed(cpu: &mut Cpu) {
    cpu.pc = 0x76f2;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::engine::battle::effect_commands::animate_failed_move(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(16);
    become crate::game::engine::battle::effect_commands::print_but_it_failed(cpu);
}
