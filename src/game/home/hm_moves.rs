use crate::runtime::Cpu;

pub fn is_hm(cpu: &mut Cpu) {
    cpu.pc = 0x34df;
    let value = crate::defs::HM01 as u8;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_c {
        cpu.cycle(12);
        become is_hm_not_hm(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = true;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn is_hm_not_hm(cpu: &mut Cpu) {
    cpu.pc = 0x34e5;
    let value = cpu.a;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub fn is_hm_move(cpu: &mut Cpu) {
    cpu.pc = 0x34e7;
    cpu.set_hl(0x34f0);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(1 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become crate::game::home::array::is_in_array(cpu);
}

