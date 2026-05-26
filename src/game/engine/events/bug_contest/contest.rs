use crate::runtime::Cpu;

pub(crate) fn give_park_balls(cpu: &mut Cpu) {
    cpu.pc = 0x75db;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xdf9c, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = crate::defs::BUG_CONTEST_BALLS as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xdc79, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x04;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x5490);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

