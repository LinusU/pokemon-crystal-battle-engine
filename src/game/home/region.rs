use crate::runtime::Cpu;

pub fn is_in_johto(cpu: &mut Cpu) {
    cpu.pc = 0x2f17;
    cpu.a = cpu.read_byte(0xdcb5);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xdcb6);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map::get_world_map_location(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = crate::defs::LANDMARK_FAST_SHIP as u8;
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
        become is_in_johto_johto(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = crate::defs::LANDMARK_SPECIAL as u8;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(12);
        become is_in_johto_check_region(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xdcad);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xdcae);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map::get_world_map_location(cpu);
    assert_eq!(cpu.pc, next_pc);
    become is_in_johto_check_region(cpu);
}

pub(crate) fn is_in_johto_check_region(cpu: &mut Cpu) {
    cpu.pc = 0x2f35;
    let value = crate::defs::KANTO_LANDMARK as u8;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_c {
        cpu.cycle(12);
        become is_in_johto_kanto(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become is_in_johto_johto(cpu);
}

pub(crate) fn is_in_johto_johto(cpu: &mut Cpu) {
    cpu.pc = 0x2f39;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn is_in_johto_kanto(cpu: &mut Cpu) {
    cpu.pc = 0x2f3b;
    cpu.a = crate::defs::KANTO_REGION as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub fn set_xy_compare_flags(cpu: &mut Cpu) {
    cpu.pc = 0x2f3e;
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}
