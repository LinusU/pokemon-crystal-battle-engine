use crate::runtime::Cpu;

pub fn queue_script(cpu: &mut Cpu) {
    cpu.pc = 0x31cd;
    cpu.a = cpu.read_byte(0xff9d);
    cpu.pc += 2;
    cpu.cycle(12);
    become far_queue_script(cpu);
}

pub fn far_queue_script(cpu: &mut Cpu) {
    cpu.pc = 0x31cf;
    cpu.write_byte(0xd0e8, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xd0e9, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte({ const N: i32 = (0xd0e9 as i32) + (1 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}
