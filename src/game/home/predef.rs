use crate::runtime::Cpu;


pub(crate) fn predef(cpu: &mut Cpu) {
    cpu.write_byte(0xcfb4, cpu.a);

    let saved_bank = cpu.read_byte(0xff9d);
    let saved_flags = cpu.flags();
    cpu.push_word(u16::from_be_bytes([saved_bank, saved_flags]));

    cpu.a = 0x02;
    let next_pc = 0x2d94;
    cpu.push_word(next_pc);
    crate::game::home::header::bankswitch(cpu);
    assert_eq!(cpu.pc, next_pc);

    let next_pc = 0x2d9a;
    cpu.push_word(next_pc);
    crate::game::engine::predef::get_predef_pointer(cpu);
    assert_eq!(cpu.pc, next_pc);

    let target_bank = cpu.a;
    let target_addr = u16::from_be_bytes([cpu.read_byte(0xcfb7), cpu.read_byte(0xcfb8)]);

    let next_pc = 0x2d9b;
    cpu.push_word(next_pc);
    crate::game::home::header::bankswitch(cpu);
    assert_eq!(cpu.pc, next_pc);

    cpu.h = cpu.read_byte(0xcfb5);
    cpu.l = cpu.read_byte(0xcfb6);

    let next_pc = 0x2da6;
    cpu.push_word(next_pc);
    (crate::dispatch::resolve(target_bank, target_addr))(cpu);
    assert_eq!(cpu.pc, next_pc);

    become predef_return(cpu);
}

pub(crate) fn predef_return(cpu: &mut Cpu) {
    cpu.write_byte(0xcfb5, cpu.h);
    cpu.write_byte(0xcfb6, cpu.l);

    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.a = cpu.h;

    let next_pc = 0x2db1;
    cpu.push_word(next_pc);
    crate::game::home::header::bankswitch(cpu);
    assert_eq!(cpu.pc, next_pc);

    cpu.h = cpu.read_byte(0xcfb5);
    cpu.l = cpu.read_byte(0xcfb6);
    cpu.a = cpu.read_byte(0xcfb6);

    cpu.pc = cpu.pop_word();
}
