use crate::runtime::Cpu;

pub const N: u8 = 6;

pub(crate) fn battle_tower_room_menu(cpu: &mut Cpu) {
    cpu.pc = 0x40b0;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    init_battle_tower_challenge_ram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x46;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x4121);
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

pub(crate) fn function1700ba(cpu: &mut Cpu) {
    cpu.pc = 0x40ba;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    init_battle_tower_challenge_ram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x46;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x411a);
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

pub(crate) fn function1700c4(cpu: &mut Cpu) {
    cpu.pc = 0x40c4;
    cpu.a = cpu.read_byte(crate::defs::R_WBK as u16);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = 0x03;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function17042c(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x05;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xbe45, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xbe46, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xdffc);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xaa41);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(4 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xd202);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xaa8e);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(0x0594);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xaa5d);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let addr = cpu.hl();
    let value = cpu.read_byte(addr);
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.write_byte(addr, result);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value << 1;
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x80 != 0;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value << 1;
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x80 != 0;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.d = 0;
    cpu.pc += 2;
    cpu.cycle(8);
    let lhs = cpu.hl();
    let rhs = cpu.de();
    let (result, carry) = lhs.overflowing_add(rhs);
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0fff) + (rhs & 0x0fff) > 0x0fff;
    cpu.flag_c = carry;
    cpu.set_hl(result);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.e = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.d = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl(0xdffc);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(4 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::close_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function170114(cpu: &mut Cpu) {
    cpu.pc = 0x4114;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    init_battle_tower_challenge_ram(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function170114_function170121(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x46;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x405f);
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

pub(crate) fn function170114_function170121(cpu: &mut Cpu) {
    cpu.pc = 0x4121;
    cpu.a = 0x05;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xa948);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xc608);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(246 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::close_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::mobile::mobile_5c::function170c8b(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function170139(cpu: &mut Cpu) {
    cpu.pc = 0x4139;
    cpu.a = 0x05;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_de(0xaa41);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.h = 0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.l = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_bc(1000 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function170139_dec_to_bin(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_bc(100 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function170139_dec_to_bin(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_bc(10 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function170139_dec_to_bin(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.b = 0;
    cpu.pc += 2;
    cpu.cycle(8);
    let lhs = cpu.hl();
    let rhs = cpu.bc();
    let (result, carry) = lhs.overflowing_add(rhs);
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0fff) + (rhs & 0x0fff) > 0x0fff;
    cpu.flag_c = carry;
    cpu.set_hl(result);
    cpu.pc += 1;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::close_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc608, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte({ const N: i32 = (0xc608 as i32) + (1 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xc628);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xd47b);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte({ const N: i32 = (0xd47b as i32) + (1 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xd84a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte({ const N: i32 = (0xd84a as i32) + (1 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.e = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.d = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl(0xd47d);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc({ const N: i32 = (crate::defs::NAME_LENGTH_JAPANESE as u16 as i32) - (1 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_bc(0xd47b);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xd472);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0x13;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x6929);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_de(0xc631);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = ((0xdcdfu32) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd49, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = (((0xdcdfu32) >> 8) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd4a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = ((0xde41u32) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd4b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = (((0xde41u32) >> 8) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd4c, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 3;
    cpu.pc += 2;
    cpu.cycle(8);
    become function170139_copy_loop(cpu);
}

pub(crate) fn function170139_copy_loop(cpu: &mut Cpu) {
    cpu.pc = 0x41ac;
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xcd49);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xcd4a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.h = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_bc(crate::defs::PARTYMON_STRUCT_LENGTH as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcd49, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcd4a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xcd4b);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xcd4c);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.h = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_bc(6 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcd4b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcd4c, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(12);
        become function170139_copy_loop(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x04;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xa013);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(0x0024);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::close_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x05;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xa894);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(6 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xc608);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xa948);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(246 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::close_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function170139_dec_to_bin(cpu: &mut Cpu) {
    cpu.pc = 0x420c;
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.a;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    become function170139_digit_loop(cpu);
}

pub(crate) fn function170139_digit_loop(cpu: &mut Cpu) {
    cpu.pc = 0x4210;
    let lhs = cpu.hl();
    let rhs = cpu.bc();
    let (result, carry) = lhs.overflowing_add(rhs);
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0fff) + (rhs & 0x0fff) > 0x0fff;
    cpu.flag_c = carry;
    cpu.set_hl(result);
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(12);
        become function170139_digit_loop(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn battle_tower_battle(cpu: &mut Cpu) {
    cpu.pc = 0x4215;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcf63, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    _battle_tower_battle(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn unused_battle_tower_dummy_special1(cpu: &mut Cpu) {
    cpu.pc = 0x421d;
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn init_battle_tower_challenge_ram(cpu: &mut Cpu) {
    cpu.pc = 0x421e;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcf63, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xcf64, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xcf65, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xcf66, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn _battle_tower_battle(cpu: &mut Cpu) {
    cpu.pc = 0x422c;
    become _battle_tower_battle_loop(cpu);
}

pub(crate) fn _battle_tower_battle_loop(cpu: &mut Cpu) {
    cpu.pc = 0x422c;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    _battle_tower_battle_do_dw(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::delay::delay_frame(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xcf63);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = crate::defs::TRUE as u8;
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
        become _battle_tower_battle_loop(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn run_battle_tower_trainer(cpu: &mut Cpu) {
    cpu.pc = 0x424d;
    cpu.a = cpu.read_byte(0xcfcc);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.set_hl(0xcfcc);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((crate::defs::BATTLE_SHIFT as u8) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value |= mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xcfc0);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    let value = { let value = ((1 as i32) << (crate::defs::IN_BATTLE_TOWER_BATTLE_F as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcfc0, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc2dc, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x41;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x60a2);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x03;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x4658);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    read_bt_trainer_party(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::mobile::mobile_5c::clears5_a89a(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = { let value = (((0x45ad as i32) - (0x456b as i32)) / (3 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::predef::predef(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x05;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x500c);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x03;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x4658);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xd0ee);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xc2dd, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.a;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(12);
        become run_battle_tower_trainer_lost(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x01;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xbe46);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xcf64, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::close_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xd099);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xcf64);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = { let value = (0xf7 as i32); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_add(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) > 0x0f;
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = { let value = (0x50 as i32); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    become run_battle_tower_trainer_lost(cpu);
}

pub(crate) fn run_battle_tower_trainer_lost(cpu: &mut Cpu) {
    cpu.pc = 0x42a9;
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte(0xcfc0, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte(0xcfcc, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = crate::defs::TRUE as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcf63, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn read_bt_trainer_party(cpu: &mut Cpu) {
    cpu.pc = 0x42b7;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    copy_bt_trainer_from_bt_ot_tow_bt_ot_temp(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_de(0xc643);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.c = crate::defs::MON_NAME_LENGTH as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x5f;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x5073);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    if !cpu.flag_c {
        cpu.cycle(12);
        become read_bt_trainer_party_skip_mon_1(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc613);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xd265, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::names::get_pokemon_name(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.l = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.h = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_de(0xc643);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(crate::defs::MON_NAME_LENGTH as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    become read_bt_trainer_party_skip_mon_1(cpu);
}

pub(crate) fn read_bt_trainer_party_skip_mon_1(cpu: &mut Cpu) {
    cpu.pc = 0x42db;
    cpu.set_de(0xc67e);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.c = crate::defs::MON_NAME_LENGTH as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x5f;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x5073);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    if !cpu.flag_c {
        cpu.cycle(12);
        become read_bt_trainer_party_skip_mon_2(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc64e);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xd265, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::names::get_pokemon_name(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.l = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.h = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_de(0xc67e);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(crate::defs::MON_NAME_LENGTH as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    become read_bt_trainer_party_skip_mon_2(cpu);
}

pub(crate) fn read_bt_trainer_party_skip_mon_2(cpu: &mut Cpu) {
    cpu.pc = 0x42fc;
    cpu.set_de(0xc6b9);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.c = crate::defs::MON_NAME_LENGTH as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x5f;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x5073);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    if !cpu.flag_c {
        cpu.cycle(12);
        become read_bt_trainer_party_skip_mon_3(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc689);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xd265, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::names::get_pokemon_name(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.l = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.h = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_de(0xc6b9);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(crate::defs::MON_NAME_LENGTH as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    become read_bt_trainer_party_skip_mon_3(cpu);
}

pub(crate) fn read_bt_trainer_party_skip_mon_3(cpu: &mut Cpu) {
    cpu.pc = 0x431d;
    cpu.a = { let value = (0x50 as i32); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte({ const N: i32 = (0xc643 as i32) + (crate::defs::MON_NAME_LENGTH as u16 as i32) - (1 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte({ const N: i32 = (0xc67e as i32) + (crate::defs::MON_NAME_LENGTH as u16 as i32) - (1 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte({ const N: i32 = (0xc6b9 as i32) + (crate::defs::MON_NAME_LENGTH as u16 as i32) - (1 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::mobile::mobile_5c::check_bt_mon_moves_for_errors(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_de(0xc608);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.c = { let value = ((crate::defs::NAME_LENGTH as i32) - (1 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x5f;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x5073);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    if !cpu.flag_c {
        cpu.cycle(12);
        become read_bt_trainer_party_trainer_name_okay(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x4426);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(12);
    become read_bt_trainer_party_done_trainer_name(cpu);
}

pub(crate) fn read_bt_trainer_party_trainer_name_okay(cpu: &mut Cpu) {
    cpu.pc = 0x433d;
    cpu.set_hl(0xc608);
    cpu.pc += 3;
    cpu.cycle(12);
    become read_bt_trainer_party_done_trainer_name(cpu);
}

pub(crate) fn read_bt_trainer_party_done_trainer_name(cpu: &mut Cpu) {
    cpu.pc = 0x4340;
    cpu.set_de(0xd26b);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc({ const N: i32 = (crate::defs::NAME_LENGTH as u16 as i32) - (1 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = { let value = (0x50 as i32); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(0xc612);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(0xd22f, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = ((0xd3eau32) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd20, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = (((0xd3eau32) >> 8) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte({ const N: i32 = (0xcd20 as i32) + (1 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_de(0xd288);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(0xd280);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = crate::defs::BATTLETOWER_PARTY_LENGTH as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.bc(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_bc(cpu.bc().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    become read_bt_trainer_party_otpartymon_loop(cpu);
}

pub(crate) fn read_bt_trainer_party_otpartymon_loop(cpu: &mut Cpu) {
    cpu.pc = 0x4367;
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.bc(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_bc(cpu.bc().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.push_word(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.set_bc(crate::defs::PARTYMON_STRUCT_LENGTH as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.push_word(cpu.de());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xcd20);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte({ const N: i32 = (0xcd20 as i32) + (1 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.d = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_bc(crate::defs::MON_NAME_LENGTH as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcd20, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte({ const N: i32 = (0xcd20 as i32) + (1 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.pop_word();
    cpu.set_de(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let value = cpu.pop_word();
    cpu.set_bc(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
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
    if !cpu.flag_z {
        cpu.cycle(12);
        become read_bt_trainer_party_otpartymon_loop(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = -1i16 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.bc(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn validate_bt_party(cpu: &mut Cpu) {
    cpu.pc = 0x4394;
    cpu.set_hl(0xc613);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.d = crate::defs::BATTLETOWER_PARTY_LENGTH as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    become validate_bt_party_pkmn_loop(cpu);
}

pub(crate) fn validate_bt_party_pkmn_loop(cpu: &mut Cpu) {
    cpu.pc = 0x4399;
    cpu.push_word(cpu.de());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.b = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.c = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
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
        become validate_bt_party_invalid(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 255;
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
        become validate_bt_party_invalid(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 254;
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
        become validate_bt_party_invalid(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 253;
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
        become validate_bt_party_invalid(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 252;
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
        become validate_bt_party_valid(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become validate_bt_party_invalid(cpu);
}

pub(crate) fn validate_bt_party_invalid(cpu: &mut Cpu) {
    cpu.pc = 0x43b1;
    cpu.a = crate::defs::SMEARGLE as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    become validate_bt_party_valid(cpu);
}

pub(crate) fn validate_bt_party_valid(cpu: &mut Cpu) {
    cpu.pc = 0x43b4;
    cpu.write_byte(0xcf60, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::pokemon::get_base_data(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x05;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xb2fb);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::close_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl(crate::defs::MON_LEVEL as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let lhs = cpu.hl();
    let rhs = cpu.bc();
    let (result, carry) = lhs.overflowing_add(rhs);
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0fff) + (rhs & 0x0fff) > 0x0fff;
    cpu.flag_c = carry;
    cpu.set_hl(result);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = crate::defs::MIN_LEVEL as u8;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = crate::defs::MIN_LEVEL as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_c {
        cpu.cycle(12);
        become validate_bt_party_load(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.e;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_c {
        cpu.cycle(12);
        become validate_bt_party_dont_load(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    become validate_bt_party_load(cpu);
}

pub(crate) fn validate_bt_party_load(cpu: &mut Cpu) {
    cpu.pc = 0x43d6;
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    become validate_bt_party_dont_load(cpu);
}

pub(crate) fn validate_bt_party_dont_load(cpu: &mut Cpu) {
    cpu.pc = 0x43d7;
    cpu.write_byte(0xd143, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(crate::defs::MON_MOVES as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let lhs = cpu.hl();
    let rhs = cpu.bc();
    let (result, carry) = lhs.overflowing_add(rhs);
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0fff) + (rhs & 0x0fff) > 0x0fff;
    cpu.flag_c = carry;
    cpu.set_hl(result);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.d = { let value = ((crate::defs::NUM_MOVES as i32) - (1 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
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
        become validate_bt_party_not_move(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = { let value = ((crate::defs::NUM_ATTACKS as i32) + (1 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
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
        become validate_bt_party_not_move(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become validate_bt_party_valid_move(cpu);
}

pub(crate) fn validate_bt_party_not_move(cpu: &mut Cpu) {
    cpu.pc = 0x43ea;
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = crate::defs::POUND as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.cycle(12);
    become validate_bt_party_done_moves(cpu);
}

pub(crate) fn validate_bt_party_valid_move(cpu: &mut Cpu) {
    cpu.pc = 0x43f4;
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = { let value = ((crate::defs::NUM_ATTACKS as i32) + (1 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
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
        become validate_bt_party_next(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), 0x0);
    cpu.pc += 2;
    cpu.cycle(12);
    become validate_bt_party_next(cpu);
}

pub(crate) fn validate_bt_party_next(cpu: &mut Cpu) {
    cpu.pc = 0x43fb;
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.d;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.d = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(12);
        become validate_bt_party_valid_move(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become validate_bt_party_done_moves(cpu);
}

pub(crate) fn validate_bt_party_done_moves(cpu: &mut Cpu) {
    cpu.pc = 0x43ff;
    cpu.set_hl(crate::defs::MON_MAXHP as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let lhs = cpu.hl();
    let rhs = cpu.bc();
    let (result, carry) = lhs.overflowing_add(rhs);
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0fff) + (rhs & 0x0fff) > 0x0fff;
    cpu.flag_c = carry;
    cpu.set_hl(result);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.d = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.e = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.push_word(cpu.de());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.set_hl({ const N: i32 = (crate::defs::MON_STAT_EXP as u16 as i32) - (1 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    let lhs = cpu.hl();
    let rhs = cpu.bc();
    let (result, carry) = lhs.overflowing_add(rhs);
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0fff) + (rhs & 0x0fff) > 0x0fff;
    cpu.flag_c = carry;
    cpu.set_hl(result);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.b = crate::defs::TRUE as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = { let value = (((0x458f as i32) - (0x456b as i32)) / (3 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::predef::predef(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_de(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.set_de(cpu.de().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.set_bc(crate::defs::NICKNAMED_MON_STRUCT_LENGTH as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let lhs = cpu.hl();
    let rhs = cpu.bc();
    let (result, carry) = lhs.overflowing_add(rhs);
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0fff) + (rhs & 0x0fff) > 0x0fff;
    cpu.flag_c = carry;
    cpu.set_hl(result);
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.pop_word();
    cpu.set_de(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let value = cpu.d;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.d = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(16);
        become validate_bt_party_pkmn_loop(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function17042c(cpu: &mut Cpu) {
    cpu.pc = 0x442c;
    cpu.set_hl(0xd2be);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = crate::defs::BATTLETOWER_STREAK_LENGTH as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    become function17042c_loop(cpu);
}

pub(crate) fn function17042c_loop(cpu: &mut Cpu) {
    cpu.pc = 0x4431;
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.c = { let value = ((crate::defs::BATTLETOWER_TRAINERDATALENGTH as i32) / (2 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    become function17042c_loop2(cpu);
}

pub(crate) fn function17042c_loop2(cpu: &mut Cpu) {
    cpu.pc = 0x4435;
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
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
        become function17042c_empty(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = { let value = (((0x447e as i32) - (0x4470 as i32)) + (1 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
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
        become function17042c_copy_data(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.set_hl(0x4470);
    cpu.pc += 3;
    cpu.cycle(12);
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.d = 0;
    cpu.pc += 2;
    cpu.cycle(8);
    let lhs = cpu.hl();
    let rhs = cpu.de();
    let (result, carry) = lhs.overflowing_add(rhs);
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0fff) + (rhs & 0x0fff) > 0x0fff;
    cpu.flag_c = carry;
    cpu.set_hl(result);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let value = cpu.b;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_c {
        cpu.cycle(12);
        become function17042c_copy_data(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become function17042c_copy_data(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become function17042c_next_iteration(cpu);
}

pub(crate) fn function17042c_empty(cpu: &mut Cpu) {
    cpu.pc = 0x4451;
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = { let value = ((crate::defs::NUM_POKEMON as i32) + (1 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
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
        become function17042c_copy_data(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become function17042c_next_iteration(cpu);
}

pub(crate) fn function17042c_next_iteration(cpu: &mut Cpu) {
    cpu.pc = 0x4456;
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
        become function17042c_loop2(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become function17042c_next_trainer(cpu);
}

pub(crate) fn function17042c_copy_data(cpu: &mut Cpu) {
    cpu.pc = 0x445b;
    let value = cpu.pop_word();
    cpu.set_de(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.push_word(cpu.de());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.set_hl(0x447e);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(crate::defs::BATTLETOWER_TRAINERDATALENGTH as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    become function17042c_next_trainer(cpu);
}

pub(crate) fn function17042c_next_trainer(cpu: &mut Cpu) {
    cpu.pc = 0x4466;
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.set_de(crate::defs::BATTLE_TOWER_STRUCT_LENGTH as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let lhs = cpu.hl();
    let rhs = cpu.de();
    let (result, carry) = lhs.overflowing_add(rhs);
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0fff) + (rhs & 0x0fff) > 0x0fff;
    cpu.flag_c = carry;
    cpu.set_hl(result);
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(12);
        become function17042c_loop(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn copy_bt_trainer_from_bt_ot_tow_bt_ot_temp(cpu: &mut Cpu) {
    cpu.pc = 0x44a2;
    cpu.a = cpu.read_byte(crate::defs::R_WBK as u16);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = 0x03;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.set_hl(0xd100);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xc608);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(crate::defs::BATTLE_TOWER_STRUCT_LENGTH as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = 0x01;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = crate::defs::BATTLETOWER_CHALLENGE_IN_PROGRESS as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xbe45, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xbe46);
    cpu.pc += 3;
    cpu.cycle(12);
    let addr = cpu.hl();
    let value = cpu.read_byte(addr);
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.write_byte(addr, result);
    cpu.pc += 1;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::close_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    become skip_battle_tower_trainer(cpu);
}

pub(crate) fn skip_battle_tower_trainer(cpu: &mut Cpu) {
    cpu.pc = 0x44c9;
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function1704ca(cpu: &mut Cpu) {
    cpu.pc = 0x44ca;
    cpu.a = cpu.read_byte(0xbe46);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = crate::defs::BATTLETOWER_STREAK_LENGTH as u8;
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
        become function1704ca_not_max(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = { let value = ((crate::defs::BATTLETOWER_STREAK_LENGTH as i32) - (1 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    become function1704ca_not_max(cpu);
}

pub(crate) fn function1704ca_not_max(cpu: &mut Cpu) {
    cpu.pc = 0x44d3;
    cpu.set_hl({ const N: i32 = ((0xaa8e as i32) + ((crate::defs::BATTLE_TOWER_STRUCT_LENGTH as u16 as i32) * ((crate::defs::BATTLETOWER_STREAK_LENGTH as u16 as i32) - (1 as i32)))); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xff20);
    cpu.pc += 3;
    cpu.cycle(12);
    become function1704ca_loop(cpu);
}

pub(crate) fn function1704ca_loop(cpu: &mut Cpu) {
    cpu.pc = 0x44d9;
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
        become function1704ca_done(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let lhs = cpu.hl();
    let rhs = cpu.de();
    let (result, carry) = lhs.overflowing_add(rhs);
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0fff) + (rhs & 0x0fff) > 0x0fff;
    cpu.flag_c = carry;
    cpu.set_hl(result);
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.cycle(12);
    become function1704ca_loop(cpu);
}

pub(crate) fn function1704ca_done(cpu: &mut Cpu) {
    cpu.pc = 0x44e0;
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function1704e1(cpu: &mut Cpu) {
    cpu.pc = 0x44e1;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::speech_textbox(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map::fade_to_menu(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    init_battle_tower_challenge_ram(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function1704e1_jumptable_loop(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map::close_submenu(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function1704e1_jumptable_loop(cpu: &mut Cpu) {
    cpu.pc = 0x44f1;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::tilemap::clear_bg_palettes(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::clear_sprites::clear_sprites(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::clear_screen(cpu);
    assert_eq!(cpu.pc, next_pc);
    become function1704e1_loop(cpu);
}

pub(crate) fn function1704e1_loop(cpu: &mut Cpu) {
    cpu.pc = 0x44fa;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::joypad::joy_text_delay(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xcf63);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((crate::defs::JUMPTABLE_EXIT_F as u8) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(12);
        become function1704e1_done(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function1704e1_do_jumptable(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x41;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x4061);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(12);
    become function1704e1_loop(cpu);
}

pub(crate) fn function1704e1_done(cpu: &mut Cpu) {
    cpu.pc = 0x450f;
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function1704e1_jumptable_0(cpu: &mut Cpu) {
    cpu.pc = 0x4525;
    cpu.a = 0x05;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xa89c);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xd099);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(22 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xa8b2);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xc608);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(150 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::close_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = ((((1 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (1 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xd099);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::place_string(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = ((((3 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (1 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0x4676);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::place_string(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = ((((3 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (4 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xd0ac);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::place_string(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = ((((3 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (8 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0x467a);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::place_string(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function1704e1_draw_border(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function1704e1_place_text_items(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(12);
    become function1704e1_next_jumptable_function(cpu);
}

pub(crate) fn function1704e1_jumptable_1(cpu: &mut Cpu) {
    cpu.pc = 0x4571;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::tilemap::set_default_bgp_and_obp(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function1704e1_next_jumptable_function(cpu);
    assert_eq!(cpu.pc, next_pc);
    become function1704e1_jumptable_2(cpu);
}

pub(crate) fn function1704e1_jumptable_2(cpu: &mut Cpu) {
    cpu.pc = 0x4577;
    cpu.set_hl(0xffa7);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = crate::defs::PAD_A as u8;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(12);
        become function1704e1_pressed_a_or_b(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = crate::defs::PAD_B as u8;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(12);
        become function1704e1_pressed_a_or_b(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = crate::defs::PAD_UP as u8;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(12);
        become function1704e1_pressed_up(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = crate::defs::PAD_DOWN as u8;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(12);
        become function1704e1_pressed_down(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function1704e1_pressed_up(cpu: &mut Cpu) {
    cpu.pc = 0x458f;
    cpu.a = cpu.read_byte(0xcf64);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.a;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 15;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcf64, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function1704e1_place_text_items(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function1704e1_pressed_down(cpu: &mut Cpu) {
    cpu.pc = 0x459d;
    cpu.a = cpu.read_byte(0xcf64);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 60;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 15;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_add(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) > 0x0f;
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcf64, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function1704e1_place_text_items(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function1704e1_pressed_a_or_b(cpu: &mut Cpu) {
    cpu.pc = 0x45ac;
    cpu.set_hl(0xcf63);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((crate::defs::JUMPTABLE_EXIT_F as u8) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value |= mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function1704e1_next_jumptable_function(cpu: &mut Cpu) {
    cpu.pc = 0x45b2;
    cpu.set_hl(0xcf63);
    cpu.pc += 3;
    cpu.cycle(12);
    let addr = cpu.hl();
    let value = cpu.read_byte(addr);
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.write_byte(addr, result);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function1704e1_draw_border(cpu: &mut Cpu) {
    cpu.pc = 0x45b7;
    cpu.set_hl({ const N: i32 = ((((4 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (0 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = { let value = (0x79 as i32); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.c = { let value = ((crate::defs::SCREEN_WIDTH as i32) - (2 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    become function1704e1_top_border_loop(cpu);
}

pub(crate) fn function1704e1_top_border_loop(cpu: &mut Cpu) {
    cpu.pc = 0x45bf;
    cpu.a = { let value = (0x7a as i32); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
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
        become function1704e1_top_border_loop(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = { let value = (0x7b as i32); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(crate::defs::SCREEN_WIDTH as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.c = 12;
    cpu.pc += 2;
    cpu.cycle(8);
    become function1704e1_left_border_loop(cpu);
}

pub(crate) fn function1704e1_left_border_loop(cpu: &mut Cpu) {
    cpu.pc = 0x45cd;
    cpu.a = { let value = (0x7c as i32); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    let lhs = cpu.hl();
    let rhs = cpu.de();
    let (result, carry) = lhs.overflowing_add(rhs);
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0fff) + (rhs & 0x0fff) > 0x0fff;
    cpu.flag_c = carry;
    cpu.set_hl(result);
    cpu.pc += 1;
    cpu.cycle(8);
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
        become function1704e1_left_border_loop(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = { let value = (0x7d as i32); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.c = { let value = ((crate::defs::SCREEN_WIDTH as i32) - (2 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    become function1704e1_bottom_border_loop(cpu);
}

pub(crate) fn function1704e1_bottom_border_loop(cpu: &mut Cpu) {
    cpu.pc = 0x45d9;
    cpu.a = { let value = (0x7a as i32); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
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
        become function1704e1_bottom_border_loop(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = { let value = (0x7e as i32); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(0xffec);
    cpu.pc += 3;
    cpu.cycle(12);
    let lhs = cpu.hl();
    let rhs = cpu.de();
    let (result, carry) = lhs.overflowing_add(rhs);
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0fff) + (rhs & 0x0fff) > 0x0fff;
    cpu.flag_c = carry;
    cpu.set_hl(result);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.c = 12;
    cpu.pc += 2;
    cpu.cycle(8);
    become function1704e1_right_border_loop(cpu);
}

pub(crate) fn function1704e1_right_border_loop(cpu: &mut Cpu) {
    cpu.pc = 0x45e8;
    cpu.a = { let value = (0x7c as i32); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    let lhs = cpu.hl();
    let rhs = cpu.de();
    let (result, carry) = lhs.overflowing_add(rhs);
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0fff) + (rhs & 0x0fff) > 0x0fff;
    cpu.flag_c = carry;
    cpu.set_hl(result);
    cpu.pc += 1;
    cpu.cycle(8);
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
        become function1704e1_right_border_loop(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function1704e1_place_text_items(cpu: &mut Cpu) {
    cpu.pc = 0x45f0;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function1704e1_clear_box(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function1704e1_place_up_down_arrows(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x50;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcd4e, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xc608);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xcf64);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    let lhs = cpu.hl();
    let rhs = cpu.bc();
    let (result, carry) = lhs.overflowing_add(rhs);
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0fff) + (rhs & 0x0fff) > 0x0fff;
    cpu.flag_c = carry;
    cpu.set_hl(result);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    let value = cpu.pop_word();
    cpu.set_bc(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.set_hl({ const N: i32 = ((((6 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (1 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 6;
    cpu.pc += 2;
    cpu.cycle(8);
    become function1704e1_loop1(cpu);
}

pub(crate) fn function1704e1_loop1(cpu: &mut Cpu) {
    cpu.pc = 0x460c;
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = 3;
    cpu.pc += 2;
    cpu.cycle(8);
    become function1704e1_loop2(cpu);
}

pub(crate) fn function1704e1_loop2(cpu: &mut Cpu) {
    cpu.pc = 0x4610;
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.set_de(0xcd49);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(8);
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
        become function1704e1_fill_with_e3(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 5;
    cpu.pc += 2;
    cpu.cycle(8);
    become function1704e1_loop3a(cpu);
}

pub(crate) fn function1704e1_loop3a(cpu: &mut Cpu) {
    cpu.pc = 0x461a;
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_bc(cpu.bc().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(12);
        become function1704e1_loop3a(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become function1704e1_rejoin(cpu);
}

pub(crate) fn function1704e1_fill_with_e3(cpu: &mut Cpu) {
    cpu.pc = 0x4625;
    cpu.a = 5;
    cpu.pc += 2;
    cpu.cycle(8);
    become function1704e1_loop3b(cpu);
}

pub(crate) fn function1704e1_loop3b(cpu: &mut Cpu) {
    cpu.pc = 0x4627;
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = 0xe3;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_bc(cpu.bc().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(12);
        become function1704e1_loop3b(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become function1704e1_rejoin(cpu);
}

pub(crate) fn function1704e1_rejoin(cpu: &mut Cpu) {
    cpu.pc = 0x4631;
    cpu.set_de(0xcd49);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.push_word(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::place_string(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_de(crate::defs::NAME_LENGTH_JAPANESE as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let lhs = cpu.hl();
    let rhs = cpu.de();
    let (result, carry) = lhs.overflowing_add(rhs);
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0fff) + (rhs & 0x0fff) > 0x0fff;
    cpu.flag_c = carry;
    cpu.set_hl(result);
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.pop_word();
    cpu.set_bc(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(12);
        become function1704e1_loop2(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.set_de(0x28 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let lhs = cpu.hl();
    let rhs = cpu.de();
    let (result, carry) = lhs.overflowing_add(rhs);
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0fff) + (rhs & 0x0fff) > 0x0fff;
    cpu.flag_c = carry;
    cpu.set_hl(result);
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(12);
        become function1704e1_loop1(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function1704e1_clear_box(cpu: &mut Cpu) {
    cpu.pc = 0x464b;
    cpu.set_hl({ const N: i32 = ((((5 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (1 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.b = 12;
    cpu.pc += 2;
    cpu.cycle(8);
    become function1704e1_clearbox_row(cpu);
}

pub(crate) fn function1704e1_clearbox_row(cpu: &mut Cpu) {
    cpu.pc = 0x4651;
    cpu.c = { let value = ((crate::defs::SCREEN_WIDTH as i32) - (2 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    become function1704e1_clearbox_column(cpu);
}

pub(crate) fn function1704e1_clearbox_column(cpu: &mut Cpu) {
    cpu.pc = 0x4653;
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
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
        become function1704e1_clearbox_column(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.b;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.b = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(12);
        become function1704e1_clearbox_row(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function1704e1_place_up_down_arrows(cpu: &mut Cpu) {
    cpu.pc = 0x465d;
    cpu.a = cpu.read_byte(0xcf64);
    cpu.pc += 3;
    cpu.cycle(16);
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
        become function1704e1_nope(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl({ const N: i32 = ((((5 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (18 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = { let value = (0x61 as i32); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    become function1704e1_nope(cpu);
}

pub(crate) fn function1704e1_nope(cpu: &mut Cpu) {
    cpu.pc = 0x4669;
    cpu.a = cpu.read_byte(0xcf64);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 60;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl({ const N: i32 = ((((16 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (18 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = { let value = (0xee as i32); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn reset_battle_tower_trainers_sram(cpu: &mut Cpu) {
    cpu.pc = 0x46d6;
    cpu.a = 0x01;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0xff;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xbe48);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(crate::defs::BATTLETOWER_STREAK_LENGTH as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::byte_fill(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xbe46, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::close_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn battle_tower_give_reward(cpu: &mut Cpu) {
    cpu.pc = 0x46ee;
    cpu.a = 0x01;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xbe50);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::close_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.write_byte(0xc2dd, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xd892);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = crate::defs::MAX_ITEMS as u8;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_c {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.b = crate::defs::MAX_ITEMS as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc2dd);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    become battle_tower_give_reward_loop(cpu);
}

pub(crate) fn battle_tower_give_reward_loop(cpu: &mut Cpu) {
    cpu.pc = 0x4709;
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.c;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(12);
        become battle_tower_give_reward_next(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 95;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_c {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    become battle_tower_give_reward_next(cpu);
}

pub(crate) fn battle_tower_give_reward_next(cpu: &mut Cpu) {
    cpu.pc = 0x4711;
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.b;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.b = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(12);
        become battle_tower_give_reward_loop(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = crate::defs::POTION as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc2dd, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn battle_tower_action_1c(cpu: &mut Cpu) {
    cpu.pc = 0x471b;
    cpu.a = 0x01;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = crate::defs::BATTLETOWER_WON_CHALLENGE as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xbe45, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::close_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn battle_tower_action_1d(cpu: &mut Cpu) {
    cpu.pc = 0x4729;
    cpu.a = 0x01;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = crate::defs::BATTLETOWER_RECEIVED_REWARD as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xbe45, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::close_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn battle_tower_save_options(cpu: &mut Cpu) {
    cpu.pc = 0x4737;
    cpu.a = 0x05;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x4dbb);
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

pub(crate) fn battle_tower_randomly_choose_reward(cpu: &mut Cpu) {
    cpu.pc = 0x473e;
    become battle_tower_randomly_choose_reward_loop(cpu);
}

pub(crate) fn battle_tower_randomly_choose_reward_loop(cpu: &mut Cpu) {
    cpu.pc = 0x473e;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::random::random(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xffe1);
    cpu.pc += 2;
    cpu.cycle(12);
    let value = 0x7;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 6;
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
        become battle_tower_randomly_choose_reward_okay(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 6;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    become battle_tower_randomly_choose_reward_okay(cpu);
}

pub(crate) fn battle_tower_randomly_choose_reward_okay(cpu: &mut Cpu) {
    cpu.pc = 0x474b;
    let value = crate::defs::HP_UP as u8;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_add(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) > 0x0f;
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = crate::defs::LUCKY_PUNCH as u8;
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
        become battle_tower_randomly_choose_reward_loop(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = 0x01;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte(0xbe50, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::close_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn battle_tower_action_check_explanation_read(cpu: &mut Cpu) {
    cpu.pc = 0x475f;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    battle_tower_check_save_file_exists_and_is_yours(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xc2dd);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.a;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 0x01;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xbe4f);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 2;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc2dd, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::close_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn battle_tower_action_get_challenge_state(cpu: &mut Cpu) {
    cpu.pc = 0x4778;
    cpu.set_hl(0xbe45);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0x01;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(0xc2dd, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::close_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn battle_tower_action_set_explanation_read(cpu: &mut Cpu) {
    cpu.pc = 0x4788;
    cpu.a = 0x01;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xbe4f);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 2;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xbe4f, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::close_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn battle_tower_action_set_byte_to_quick_save_challenge(cpu: &mut Cpu) {
    cpu.pc = 0x4799;
    cpu.c = crate::defs::BATTLETOWER_SAVED_AND_LEFT as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become set_battle_tower_challenge_state(cpu);
}

pub(crate) fn battle_tower_action_set_byte_to_cancel_challenge(cpu: &mut Cpu) {
    cpu.pc = 0x479d;
    cpu.c = crate::defs::BATTLETOWER_NO_CHALLENGE as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    become set_battle_tower_challenge_state(cpu);
}

pub(crate) fn set_battle_tower_challenge_state(cpu: &mut Cpu) {
    cpu.pc = 0x479f;
    cpu.a = 0x01;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xbe45, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::close_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn battle_tower_action_05(cpu: &mut Cpu) {
    cpu.pc = 0x47ac;
    cpu.a = 0x05;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xaa8c);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xbe46);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xc2dd, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::close_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.a;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = 2;
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
        become battle_tower_action_05_asm_1707ef(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.push_word(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::time::update_time(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_bc(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.a = 0x05;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xaa8b);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::close_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xd4cb);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.c;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_c {
        cpu.cycle(12);
        become battle_tower_action_05_asm_1707e5(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 8;
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
        become battle_tower_action_05_asm_1707ef(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
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
    if !cpu.flag_z {
        cpu.cycle(12);
        become battle_tower_action_05_asm_1707ef(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn battle_tower_action_05_asm_1707e5(cpu: &mut Cpu) {
    cpu.pc = 0x47e5;
    cpu.set_hl(0xd4cb);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0x8c;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.c;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.read_byte(cpu.hl());
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_add(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) > 0x0f;
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 8;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_c {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    become battle_tower_action_05_asm_1707ef(cpu);
}

pub(crate) fn battle_tower_action_05_asm_1707ef(cpu: &mut Cpu) {
    cpu.pc = 0x47ef;
    cpu.a = 8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc2dd, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    become battle_tower_action_06(cpu);
}

pub(crate) fn battle_tower_action_06(cpu: &mut Cpu) {
    cpu.pc = 0x47f4;
    cpu.a = 0x05;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xbe46, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xaa8b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xaa8c, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::close_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn battle_tower_action_16(cpu: &mut Cpu) {
    cpu.pc = 0x4807;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::time::update_time(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x05;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xd4cb);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xb2f9, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xb2fa, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::close_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn battle_tower_action_17(cpu: &mut Cpu) {
    cpu.pc = 0x481d;
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
    cpu.a = 0x05;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xb2f9);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xb2fa);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::close_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = 2;
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
        become battle_tower_action_17_asm_170853(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.push_word(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::time::update_time(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_bc(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xd4cb);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.c;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_c {
        cpu.cycle(12);
        become battle_tower_action_17_asm_170849(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 11;
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
        become battle_tower_action_17_asm_170853(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
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
    if !cpu.flag_z {
        cpu.cycle(12);
        become battle_tower_action_17_asm_170853(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn battle_tower_action_17_asm_170849(cpu: &mut Cpu) {
    cpu.pc = 0x4849;
    cpu.set_hl(0xd4cb);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 140;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.c;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.read_byte(cpu.hl());
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_add(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) > 0x0f;
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 11;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_c {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    become battle_tower_action_17_asm_170853(cpu);
}

pub(crate) fn battle_tower_action_17_asm_170853(cpu: &mut Cpu) {
    cpu.pc = 0x4853;
    cpu.a = 1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc2dd, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x05;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xb2f9, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xb2fa, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::close_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn save_battle_tower_level_group(cpu: &mut Cpu) {
    cpu.pc = 0x4868;
    cpu.a = 0x01;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(crate::defs::R_WBK as u16);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = 0x03;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xd800);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xbe47, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::close_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn load_battle_tower_level_group(cpu: &mut Cpu) {
    cpu.pc = 0x4881;
    cpu.a = 0x01;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(crate::defs::R_WBK as u16);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = 0x03;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xbe47);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xd800, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::close_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn battle_tower_check_save_file_exists_and_is_yours(cpu: &mut Cpu) {
    cpu.pc = 0x489a;
    cpu.a = cpu.read_byte(0xcfcd);
    cpu.pc += 3;
    cpu.cycle(16);
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
        become battle_tower_check_save_file_exists_and_is_yours_nope(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x05;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x4bcb);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_z {
        cpu.cycle(12);
        become battle_tower_check_save_file_exists_and_is_yours_yes(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.cycle(12);
    become battle_tower_check_save_file_exists_and_is_yours_nope(cpu);
}

pub(crate) fn battle_tower_check_save_file_exists_and_is_yours_yes(cpu: &mut Cpu) {
    cpu.pc = 0x48ab;
    cpu.a = crate::defs::TRUE as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    become battle_tower_check_save_file_exists_and_is_yours_nope(cpu);
}

pub(crate) fn battle_tower_check_save_file_exists_and_is_yours_nope(cpu: &mut Cpu) {
    cpu.pc = 0x48ad;
    cpu.write_byte(0xc2dd, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn battle_tower_action_0a(cpu: &mut Cpu) {
    cpu.pc = 0x48b1;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc2a7, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::audio::max_volume(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn battle_tower_action_gs_ball(cpu: &mut Cpu) {
    cpu.pc = 0x48b9;
    cpu.a = 0x01;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xbe3c);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xc2dd, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::close_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn battle_tower_action_0c(cpu: &mut Cpu) {
    cpu.pc = 0x48c8;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::time::update_time(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x05;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xd4cb);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xaa8b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xaa8c, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xaa5d);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 2;
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
        become battle_tower_action_0c_asm_1708ec(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xd4cb);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xaa48, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xaa47, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    become battle_tower_action_0c_asm_1708ec(cpu);
}

pub(crate) fn battle_tower_action_0c_asm_1708ec(cpu: &mut Cpu) {
    cpu.pc = 0x48ec;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::close_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn battle_tower_action_0d(cpu: &mut Cpu) {
    cpu.pc = 0x48f0;
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
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::time::update_time(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x05;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xaa48);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xaa47);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::close_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.a;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(0xd4cb);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.read_byte(cpu.hl());
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 1;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(12);
        become function170923(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x05;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xaa5d);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::close_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = 5;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_c {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = crate::defs::TRUE as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc2dd, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function170923(cpu: &mut Cpu) {
    cpu.pc = 0x4923;
    cpu.a = 0x05;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xaa48, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xaa47, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xaa5d);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(crate::defs::MOBILE_LOGIN_PASSWORD_LENGTH as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::byte_fill(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::close_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn battle_tower_action_egg_ticket(cpu: &mut Cpu) {
    cpu.pc = 0x493c;
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
    cpu.a = crate::defs::EGG_TICKET as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xd106, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xd892);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::item::check_item(cpu);
    assert_eq!(cpu.pc, next_pc);
    if !cpu.flag_c {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xdcd7);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.b = 0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl(0xdcd8);
    cpu.pc += 3;
    cpu.cycle(12);
    become battle_tower_action_egg_ticket_loop(cpu);
}

pub(crate) fn battle_tower_action_egg_ticket_loop(cpu: &mut Cpu) {
    cpu.pc = 0x4955;
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = crate::defs::EGG as u8;
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
        become battle_tower_action_egg_ticket_not_egg(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.set_hl(0xddff);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(crate::defs::NAME_LENGTH_JAPANESE as u16);
    cpu.pc += 3;
    cpu.cycle(12);
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
        become battle_tower_action_egg_ticket_skip(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become battle_tower_action_egg_ticket_loop2(cpu);
}

pub(crate) fn battle_tower_action_egg_ticket_loop2(cpu: &mut Cpu) {
    cpu.pc = 0x4965;
    let lhs = cpu.hl();
    let rhs = cpu.de();
    let (result, carry) = lhs.overflowing_add(rhs);
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0fff) + (rhs & 0x0fff) > 0x0fff;
    cpu.flag_c = carry;
    cpu.set_hl(result);
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(12);
        become battle_tower_action_egg_ticket_loop2(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become battle_tower_action_egg_ticket_skip(cpu);
}

pub(crate) fn battle_tower_action_egg_ticket_skip(cpu: &mut Cpu) {
    cpu.pc = 0x4969;
    cpu.set_de(0x49a4);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = crate::defs::NAME_LENGTH_JAPANESE as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    become battle_tower_action_egg_ticket_compare_loop(cpu);
}

pub(crate) fn battle_tower_action_egg_ticket_compare_loop(cpu: &mut Cpu) {
    cpu.pc = 0x496e;
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.read_byte(cpu.hl());
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(12);
        become battle_tower_action_egg_ticket_different(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(12);
        become battle_tower_action_egg_ticket_compare_loop(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = { let value = (0x50 as i32); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.a = crate::defs::EGG_TICKET as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xd106, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xd10c, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = -1i16 as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xd107, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xd892);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::item::toss_item(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = crate::defs::TRUE as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc2dd, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn battle_tower_action_egg_ticket_different(cpu: &mut Cpu) {
    cpu.pc = 0x499d;
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    become battle_tower_action_egg_ticket_not_egg(cpu);
}

pub(crate) fn battle_tower_action_egg_ticket_not_egg(cpu: &mut Cpu) {
    cpu.pc = 0x499f;
    let value = cpu.b;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.b = result;
    cpu.pc += 1;
    cpu.cycle(4);
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
        become battle_tower_action_egg_ticket_loop(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn battle_tower_action_0f(cpu: &mut Cpu) {
    cpu.pc = 0x49aa;
    cpu.a = cpu.read_byte(crate::defs::R_WBK as u16);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = 0x03;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xd090);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xc2dd, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn battle_tower_action_10_invalid(cpu: &mut Cpu) {
    cpu.pc = 0x49da;
    cpu.a = 0x05;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xa800, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::close_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn battle_tower_action_10_do_action1(cpu: &mut Cpu) {
    cpu.pc = 0x49f3;
    cpu.a = 0x05;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xa800, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::close_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    become battle_tower_action_10_no_action(cpu);
}

pub(crate) fn battle_tower_action_10_no_action(cpu: &mut Cpu) {
    cpu.pc = 0x4a00;
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn battle_tower_action_10_action4(cpu: &mut Cpu) {
    cpu.pc = 0x4a01;
    cpu.a = 0x05;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xb023);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xc608);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(105 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xa825);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xcd30, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xa826);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xcd31, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::close_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x46;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x76b4);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x5f;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x50f3);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = crate::defs::TRUE as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc2dd, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn battle_tower_action_10_action5(cpu: &mut Cpu) {
    cpu.pc = 0x4a33;
    cpu.a = 0;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xd4ba);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xc608);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(4 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::close_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x05;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xb08c);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xc608);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.c = 4;
    cpu.pc += 2;
    cpu.cycle(8);
    become battle_tower_action_10_compare_loop(cpu);
}

pub(crate) fn battle_tower_action_10_compare_loop(cpu: &mut Cpu) {
    cpu.pc = 0x4a54;
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.read_byte(cpu.hl());
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 1;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(12);
        become battle_tower_action_10_different(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
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
        become battle_tower_action_10_compare_loop(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::close_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
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
    crate::game::home::map::get_map_scene_id(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.e;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become battle_tower_action_10_no_scene(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.a;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    become battle_tower_action_10_no_scene(cpu);
}

pub(crate) fn battle_tower_action_10_no_scene(cpu: &mut Cpu) {
    cpu.pc = 0x4a72;
    cpu.a = crate::defs::TRUE as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc2dd, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn battle_tower_action_10_different(cpu: &mut Cpu) {
    cpu.pc = 0x4a78;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::close_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x05;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xa800, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::close_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.write_byte(0xc2dd, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
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
    crate::game::home::map::get_map_scene_id(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.e;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become battle_tower_action_10_no_scene_2(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    become battle_tower_action_10_no_scene_2(cpu);
}

pub(crate) fn battle_tower_action_10_no_scene_2(cpu: &mut Cpu) {
    cpu.pc = 0x4a9b;
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn battle_tower_action_11(cpu: &mut Cpu) {
    cpu.pc = 0x4a9c;
    cpu.c = crate::defs::FALSE as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become set_s5_aa8d(cpu);
}

pub(crate) fn battle_tower_action_12(cpu: &mut Cpu) {
    cpu.pc = 0x4aa0;
    cpu.c = crate::defs::TRUE as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    become set_s5_aa8d(cpu);
}

pub(crate) fn set_s5_aa8d(cpu: &mut Cpu) {
    cpu.pc = 0x4aa2;
    cpu.a = 0x05;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xaa8d, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::close_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn battle_tower_action_13(cpu: &mut Cpu) {
    cpu.pc = 0x4aaf;
    cpu.a = 0x05;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xaa8d);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xc2dd, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::close_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn battle_tower_action_14(cpu: &mut Cpu) {
    cpu.pc = 0x4abe;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    battle_tower_check_save_file_exists_and_is_yours(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xc2dd);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.a;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 0x01;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xbe4f);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 1;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc2dd, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::close_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn battle_tower_action_15(cpu: &mut Cpu) {
    cpu.pc = 0x4ad7;
    cpu.a = 0x01;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xbe4f);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 1;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xbe4f, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::close_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn battle_tower_action_level_check(cpu: &mut Cpu) {
    cpu.pc = 0x4ae8;
    cpu.a = 0x05;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xb2fb);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::close_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.c = 10;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::math::simple_divide(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcd4f, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
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
    cpu.a = 0x46;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x5d93);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    if !cpu.flag_c {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 0x05;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xb2fb);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::close_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.write_byte(0xc2dd, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn battle_tower_action_ubers_check(cpu: &mut Cpu) {
    cpu.pc = 0x4b16;
    cpu.a = 0x05;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xb2fb);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::close_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.c = 10;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::math::simple_divide(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcd4f, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
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
    cpu.a = 0x46;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x5dd1);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    if !cpu.flag_c {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 0x05;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xb2fb);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::close_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.write_byte(0xc2dd, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn load_opponent_trainer_and_pokemon_with_ot_sprite(cpu: &mut Cpu) {
    cpu.pc = 0x4b44;
    cpu.a = 0x7e;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x4000);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(crate::defs::R_WBK as u16);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = 0x03;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.set_hl(0xd10a);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.b = 0;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.set_hl(0x4b90);
    cpu.pc += 3;
    cpu.cycle(12);
    let lhs = cpu.hl();
    let rhs = cpu.bc();
    let (result, carry) = lhs.overflowing_add(rhs);
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0fff) + (rhs & 0x0fff) > 0x0fff;
    cpu.flag_c = carry;
    cpu.set_hl(result);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(0xcd49, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xc2dd);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.a;
    let result = value << 1;
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x80 != 0;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.a;
    let result = value << 1;
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x80 != 0;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value << 1;
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x80 != 0;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value << 1;
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x80 != 0;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.b = 0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.d = 0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xd71e);
    cpu.pc += 3;
    cpu.cycle(12);
    let lhs = cpu.hl();
    let rhs = cpu.bc();
    let (result, carry) = lhs.overflowing_add(rhs);
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0fff) + (rhs & 0x0fff) > 0x0fff;
    cpu.flag_c = carry;
    cpu.set_hl(result);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xcd49);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(0xd154);
    cpu.pc += 3;
    cpu.cycle(12);
    let lhs = cpu.hl();
    let rhs = cpu.de();
    let (result, carry) = lhs.overflowing_add(rhs);
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0fff) + (rhs & 0x0fff) > 0x0fff;
    cpu.flag_c = carry;
    cpu.set_hl(result);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(0xffbd, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(0xffbe, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = 0x05;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x43c8);
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

pub(crate) fn unused_battle_tower_dummy_special2(cpu: &mut Cpu) {
    cpu.pc = 0x4bd2;
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn check_for_battle_tower_rules(cpu: &mut Cpu) {
    cpu.pc = 0x4bd3;
    cpu.a = 0x22;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x7201);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.cycle(12);
        become check_for_battle_tower_rules_ready(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.cycle(12);
    become check_for_battle_tower_rules_end(cpu);
}

pub(crate) fn check_for_battle_tower_rules_ready(cpu: &mut Cpu) {
    cpu.pc = 0x4bde;
    cpu.a = crate::defs::TRUE as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    become check_for_battle_tower_rules_end(cpu);
}

pub(crate) fn check_for_battle_tower_rules_end(cpu: &mut Cpu) {
    cpu.pc = 0x4be0;
    cpu.write_byte(0xc2dd, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn _battle_tower_battle_do_dw(cpu: &mut Cpu) {
    match cpu.read_byte(0xcf63) {
        0 => become run_battle_tower_trainer(cpu),
        1 => become skip_battle_tower_trainer(cpu),
        n => panic!("unsupported _BattleTowerBattle jumptable index: {}", n),
    }
}

pub(crate) fn battle_tower_action(cpu: &mut Cpu) {
    match cpu.read_byte(0xc2dd) {
        0x00 => become battle_tower_action_check_explanation_read(cpu),
        0x01 => become battle_tower_action_set_explanation_read(cpu),
        0x02 => become battle_tower_action_get_challenge_state(cpu),
        0x03 => become battle_tower_action_set_byte_to_quick_save_challenge(cpu),
        0x04 => become battle_tower_action_set_byte_to_cancel_challenge(cpu),
        0x05 => become battle_tower_action_05(cpu),
        0x06 => become battle_tower_action_06(cpu),
        0x07 => become save_battle_tower_level_group(cpu),
        0x08 => become load_battle_tower_level_group(cpu),
        0x09 => become battle_tower_check_save_file_exists_and_is_yours(cpu),
        0x0a => become battle_tower_action_0a(cpu),
        0x0b => become battle_tower_action_gs_ball(cpu),
        0x0c => become battle_tower_action_0c(cpu),
        0x0d => become battle_tower_action_0d(cpu),
        0x0e => become battle_tower_action_egg_ticket(cpu),
        0x0f => become battle_tower_action_0f(cpu),
        0x10 => become battle_tower_action_10(cpu),
        0x11 => become battle_tower_action_11(cpu),
        0x12 => become battle_tower_action_12(cpu),
        0x13 => become battle_tower_action_13(cpu),
        0x14 => become battle_tower_action_14(cpu),
        0x15 => become battle_tower_action_15(cpu),
        0x16 => become battle_tower_action_16(cpu),
        0x17 => become battle_tower_action_17(cpu),
        0x18 => become battle_tower_action_level_check(cpu),
        0x19 => become battle_tower_action_ubers_check(cpu),
        0x1a => become reset_battle_tower_trainers_sram(cpu),
        0x1b => become battle_tower_give_reward(cpu),
        0x1c => become battle_tower_action_1c(cpu),
        0x1d => become battle_tower_action_1d(cpu),
        0x1e => become battle_tower_randomly_choose_reward(cpu),
        0x1f => become battle_tower_save_options(cpu),
        n => panic!("unsupported BattleTowerAction jumptable index: {}", n),
    }
}

pub(crate) fn battle_tower_action_10(cpu: &mut Cpu) {
    let index = cpu.read_byte(0xa800);
    match index {
        0 | 1 => become battle_tower_action_10_no_action(cpu),
        2 | 3 => become battle_tower_action_10_do_action1(cpu),
        4 => become battle_tower_action_10_action4(cpu),
        5 => become battle_tower_action_10_action5(cpu),
        _ => become battle_tower_action_10_invalid(cpu),
    }
}

pub(crate) fn function1704e1_do_jumptable(cpu: &mut Cpu) {
    match cpu.read_byte(0xcf63) {
        0 => become function1704e1_jumptable_0(cpu),
        1 => become function1704e1_jumptable_1(cpu),
        2 => become function1704e1_jumptable_2(cpu),
        n => panic!("unsupported Function1704e1 jumptable index: {}", n),
    }
}
