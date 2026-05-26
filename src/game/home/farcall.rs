use crate::runtime::Cpu;

pub fn return_far_call(cpu: &mut Cpu) {
    cpu.pc = 0x2d6e;
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcfb9, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte({ const N: i32 = (0xcfb9 as i32) + (1 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.pop_word();
    cpu.set_bc(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::bankswitch(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xcfb9);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte({ const N: i32 = (0xcfb9 as i32) + (1 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}


pub(crate) fn far_call_de(cpu: &mut Cpu) {
    let target_bank = cpu.a;
    let target_addr = cpu.de();

    cpu.write_byte(0xff8b, target_bank);
    cpu.a = cpu.read_byte(0xff9d);
    cpu.push_word(cpu.af());
    cpu.a = cpu.read_byte(0xff8b);

    let next_pc = 0x2d5c;
    cpu.push_word(next_pc);
    crate::game::home::header::bankswitch(cpu);
    assert_eq!(cpu.pc, next_pc);

    let next_pc = 0x2d5f;
    cpu.push_word(next_pc);
    (crate::dispatch::resolve(target_bank, target_addr))(cpu);
    assert_eq!(cpu.pc, next_pc);

    become return_far_call(cpu);
}

pub(crate) fn far_call_hl(cpu: &mut Cpu) {
    let target_bank = cpu.a;
    let target_addr = cpu.hl();

    cpu.write_byte(0xff8b, target_bank);
    cpu.a = cpu.read_byte(0xff9d);
    cpu.push_word(cpu.af());
    cpu.a = cpu.read_byte(0xff8b);

    let next_pc = 0x2d6b;
    cpu.push_word(next_pc);
    crate::game::home::header::bankswitch(cpu);
    assert_eq!(cpu.pc, next_pc);

    let next_pc = 0x2d82;
    cpu.push_word(next_pc);
    (crate::dispatch::resolve(target_bank, target_addr))(cpu);
    assert_eq!(cpu.pc, next_pc);

    become return_far_call(cpu);
}
