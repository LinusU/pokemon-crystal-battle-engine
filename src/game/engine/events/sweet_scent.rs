use crate::runtime::Cpu;

pub(crate) fn sweet_scent_from_menu(cpu: &mut Cpu) {
    cpu.pc = 0x46bc;
    cpu.set_hl(0x46c8);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::queue_script::queue_script(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xd0ec, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn sweet_scent_encounter(cpu: &mut Cpu) {
    cpu.pc = 0x46ef;
    cpu.a = 0x25;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x7cfd);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    if !cpu.flag_c {
        cpu.cycle(12);
        become sweet_scent_encounter_no_battle(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xd84d);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((crate::defs::STATUSFLAGS2_BUG_CONTEST_TIMER_F as u8) & 7);
    let value = cpu.read_byte(cpu.hl());
    cpu.flag_z = (value & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(12);
    if !cpu.flag_z {
        cpu.cycle(12);
        become sweet_scent_encounter_in_bug_contest(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x0a;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x6111);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
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
        become sweet_scent_encounter_no_battle(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x0a;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x614f);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    if !cpu.flag_z {
        cpu.cycle(12);
        become sweet_scent_encounter_no_battle(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become sweet_scent_encounter_start_battle(cpu);
}

pub(crate) fn sweet_scent_encounter_in_bug_contest(cpu: &mut Cpu) {
    cpu.pc = 0x4712;
    cpu.a = 0x25;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x7d31);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    become sweet_scent_encounter_start_battle(cpu);
}

pub(crate) fn sweet_scent_encounter_start_battle(cpu: &mut Cpu) {
    cpu.pc = 0x4718;
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc2dd, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn sweet_scent_encounter_no_battle(cpu: &mut Cpu) {
    cpu.pc = 0x471e;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc2dd, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xd230, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

