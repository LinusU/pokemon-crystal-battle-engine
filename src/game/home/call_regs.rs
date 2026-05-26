use crate::runtime::Cpu;


pub(crate) fn _de_(cpu: &mut Cpu) {
    let bank = cpu.read_byte(0xff9d);
    let addr = cpu.de();

    become (crate::dispatch::resolve(bank, addr))(cpu);
}

pub(crate) fn _hl_(cpu: &mut Cpu) {
    let bank = cpu.read_byte(0xff9d);
    let addr = cpu.hl();

    become (crate::dispatch::resolve(bank, addr))(cpu);
}
