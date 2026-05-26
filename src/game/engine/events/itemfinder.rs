use crate::runtime::Cpu;

pub(crate) fn item_finder(cpu: &mut Cpu) {
    cpu.pc = 0x6580;
    cpu.a = 0x2e;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x4172);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.cycle(12);
        become item_finder_found_something(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x65ba);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(12);
    become item_finder_resume(cpu);
}

pub(crate) fn item_finder_found_something(cpu: &mut Cpu) {
    cpu.pc = 0x658d;
    cpu.set_hl(0x65ad);
    cpu.pc += 3;
    cpu.cycle(12);
    become item_finder_resume(cpu);
}

pub(crate) fn item_finder_resume(cpu: &mut Cpu) {
    cpu.pc = 0x6590;
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

pub(crate) fn item_finder_itemfinder_sound(cpu: &mut Cpu) {
    cpu.pc = 0x6599;
    cpu.c = 4;
    cpu.pc += 2;
    cpu.cycle(8);
    become item_finder_sfx_loop(cpu);
}

pub(crate) fn item_finder_sfx_loop(cpu: &mut Cpu) {
    cpu.pc = 0x659b;
    cpu.push_word(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.set_de(crate::defs::SFX_SECOND_PART_OF_ITEMFINDER as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::audio::wait_play_sfx(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_de(crate::defs::SFX_TRANSACTION as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::audio::wait_play_sfx(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_bc(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let value = cpu.c;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.c = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(12);
        become item_finder_sfx_loop(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

