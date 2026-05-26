use crate::runtime::Cpu;

pub fn serial(cpu: &mut Cpu) {
    cpu.pc = 0x06ef;
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.push_word(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.push_word(cpu.de());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xffc9);
    cpu.pc += 2;
    cpu.cycle(12);
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
        become serial_mobile(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc2d4);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((crate::defs::PRINTER_CONNECTION_OPEN as u8) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(12);
        become serial_printer(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xffcb);
    cpu.pc += 2;
    cpu.cycle(12);
    let value = cpu.a;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become serial_establish_connection(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(crate::defs::R_SB as u16);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.write_byte(0xffce, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xffcd);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.write_byte(crate::defs::R_SB as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xffcb);
    cpu.pc += 2;
    cpu.cycle(12);
    let value = crate::defs::USING_INTERNAL_CLOCK as u8;
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
        become serial_player2_2(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = crate::defs::SC_EXTERNAL as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_SC as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = { let value = ((crate::defs::SC_START as i32) | (crate::defs::SC_EXTERNAL as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_SC as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.cycle(12);
    become serial_player2_2(cpu);
}

pub(crate) fn serial_mobile(cpu: &mut Cpu) {
    cpu.pc = 0x071c;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::mobile::mobile_receive(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(12);
    become serial_end(cpu);
}

pub(crate) fn serial_printer(cpu: &mut Cpu) {
    cpu.pc = 0x0721;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::printer::printer_receive(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(12);
    become serial_end(cpu);
}

pub(crate) fn serial_establish_connection(cpu: &mut Cpu) {
    cpu.pc = 0x0726;
    cpu.a = cpu.read_byte(crate::defs::R_SB as u16);
    cpu.pc += 2;
    cpu.cycle(12);
    let value = crate::defs::USING_EXTERNAL_CLOCK as u8;
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
        become serial_player1(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = crate::defs::USING_INTERNAL_CLOCK as u8;
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
        become serial_player2_2(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become serial_player1(cpu);
}

pub(crate) fn serial_player1(cpu: &mut Cpu) {
    cpu.pc = 0x0730;
    cpu.write_byte(0xffce, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.write_byte(0xffcb, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    let value = crate::defs::USING_INTERNAL_CLOCK as u8;
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
        become serial_player2(cpu);
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
    cpu.write_byte(crate::defs::R_SB as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = 3;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_DIV as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    become serial_delay_loop(cpu);
}

pub(crate) fn serial_delay_loop(cpu: &mut Cpu) {
    cpu.pc = 0x073f;
    cpu.a = cpu.read_byte(crate::defs::R_DIV as u16);
    cpu.pc += 2;
    cpu.cycle(12);
    let mask = 1u8 << ((7) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(12);
        become serial_delay_loop(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = crate::defs::SC_EXTERNAL as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_SC as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = { let value = ((crate::defs::SC_START as i32) | (crate::defs::SC_EXTERNAL as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_SC as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.cycle(12);
    become serial_player2_2(cpu);
}

pub(crate) fn serial_player2(cpu: &mut Cpu) {
    cpu.pc = 0x074f;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(crate::defs::R_SB as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    become serial_player2_2(cpu);
}

pub(crate) fn serial_player2_2(cpu: &mut Cpu) {
    cpu.pc = 0x0752;
    cpu.a = crate::defs::TRUE as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xffca, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = crate::defs::SERIAL_NO_DATA_BYTE as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xffcd, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    become serial_end(cpu);
}

pub(crate) fn serial_end(cpu: &mut Cpu) {
    cpu.pc = 0x075a;
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
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
    cpu.reti();
    return;
}

pub fn serial_exchange_bytes(cpu: &mut Cpu) {
    cpu.pc = 0x075f;
    cpu.a = crate::defs::TRUE as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xffcc, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    become serial_exchange_bytes_loop(cpu);
}

pub(crate) fn serial_exchange_bytes_loop(cpu: &mut Cpu) {
    cpu.pc = 0x0763;
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(0xffcd, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    serial_exchange_byte(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.push_word(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 48;
    cpu.pc += 2;
    cpu.cycle(8);
    become serial_exchange_bytes_wait(cpu);
}

pub(crate) fn serial_exchange_bytes_wait(cpu: &mut Cpu) {
    cpu.pc = 0x076e;
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
        become serial_exchange_bytes_wait(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xffcc);
    cpu.pc += 2;
    cpu.cycle(12);
    let value = cpu.a;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.pop_word();
    cpu.set_bc(value);
    cpu.pc += 1;
    cpu.cycle(12);
    if cpu.flag_z {
        cpu.cycle(12);
        become serial_exchange_bytes_load(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = crate::defs::SERIAL_PREAMBLE_BYTE as u8;
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
        become serial_exchange_bytes_loop(cpu);
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
    cpu.write_byte(0xffcc, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.cycle(12);
    become serial_exchange_bytes_loop(cpu);
}

pub(crate) fn serial_exchange_bytes_load(cpu: &mut Cpu) {
    cpu.pc = 0x0782;
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_bc(cpu.bc().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.c;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(12);
        become serial_exchange_bytes_loop(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub fn serial_exchange_byte(cpu: &mut Cpu) {
    cpu.pc = 0x078a;
    become serial_exchange_byte_timeout_loop(cpu);
}

pub(crate) fn serial_exchange_byte_timeout_loop(cpu: &mut Cpu) {
    cpu.pc = 0x078a;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xffca, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xffcb);
    cpu.pc += 2;
    cpu.cycle(12);
    let value = crate::defs::USING_INTERNAL_CLOCK as u8;
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
        become serial_exchange_byte_not_player_2(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = crate::defs::SC_INTERNAL as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_SC as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = { let value = ((crate::defs::SC_START as i32) | (crate::defs::SC_INTERNAL as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_SC as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    become serial_exchange_byte_not_player_2(cpu);
}

pub(crate) fn serial_exchange_byte_not_player_2(cpu: &mut Cpu) {
    cpu.pc = 0x079b;
    become serial_exchange_byte_loop(cpu);
}

pub(crate) fn serial_exchange_byte_loop(cpu: &mut Cpu) {
    cpu.pc = 0x079b;
    cpu.a = cpu.read_byte(0xffca);
    cpu.pc += 2;
    cpu.cycle(12);
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
        become serial_exchange_byte_await_new_data(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xffcb);
    cpu.pc += 2;
    cpu.cycle(12);
    let value = crate::defs::USING_EXTERNAL_CLOCK as u8;
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
        become serial_exchange_byte_not_player_1_or_timed_out(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    check_link_timeout_frames_nonzero(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_z {
        cpu.cycle(12);
        become serial_exchange_byte_not_player_1_or_timed_out(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    serial_exchange_byte_short_delay(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.set_hl({ const N: i32 = (0xcf5b as i32) + (1 as i32); N as u16 });
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
    if !cpu.flag_z {
        cpu.cycle(12);
        become serial_exchange_byte_no_rollover_up(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_sub(1));
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
    become serial_exchange_byte_no_rollover_up(cpu);
}

pub(crate) fn serial_exchange_byte_no_rollover_up(cpu: &mut Cpu) {
    cpu.pc = 0x07b7;
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    check_link_timeout_frames_nonzero(cpu);
    assert_eq!(cpu.pc, next_pc);
    if !cpu.flag_z {
        cpu.cycle(12);
        become serial_exchange_byte_loop(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become serial_disconnected(cpu);
}

pub(crate) fn serial_exchange_byte_not_player_1_or_timed_out(cpu: &mut Cpu) {
    cpu.pc = 0x07c0;
    cpu.a = cpu.read_byte(crate::defs::R_IE as u16);
    cpu.pc += 2;
    cpu.cycle(12);
    let value = { let value = ((((crate::defs::IE_SERIAL as i32) | (crate::defs::IE_TIMER as i32)) | (crate::defs::IE_STAT as i32)) | (crate::defs::IE_VBLANK as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = crate::defs::IE_SERIAL as u8;
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
        become serial_exchange_byte_loop(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xcf5d);
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
    cpu.write_byte(0xcf5d, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    if !cpu.flag_z {
        cpu.cycle(12);
        become serial_exchange_byte_loop(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte({ const N: i32 = (0xcf5d as i32) + (1 as i32); N as u16 });
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
    cpu.write_byte({ const N: i32 = (0xcf5d as i32) + (1 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    if !cpu.flag_z {
        cpu.cycle(12);
        become serial_exchange_byte_loop(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xffcb);
    cpu.pc += 2;
    cpu.cycle(12);
    let value = crate::defs::USING_EXTERNAL_CLOCK as u8;
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
        become serial_exchange_byte_await_new_data(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 255;
    cpu.pc += 2;
    cpu.cycle(8);
    become serial_exchange_byte_long_delay_loop(cpu);
}

pub(crate) fn serial_exchange_byte_long_delay_loop(cpu: &mut Cpu) {
    cpu.pc = 0x07e2;
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
        become serial_exchange_byte_long_delay_loop(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become serial_exchange_byte_await_new_data(cpu);
}

pub(crate) fn serial_exchange_byte_await_new_data(cpu: &mut Cpu) {
    cpu.pc = 0x07e5;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xffca, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(crate::defs::R_IE as u16);
    cpu.pc += 2;
    cpu.cycle(12);
    let value = { let value = ((((crate::defs::IE_SERIAL as i32) | (crate::defs::IE_TIMER as i32)) | (crate::defs::IE_STAT as i32)) | (crate::defs::IE_VBLANK as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = crate::defs::IE_SERIAL as u8;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(12);
        become serial_exchange_byte_non_serial_interrupts_enabled(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcf5d, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = (((crate::defs::SERIAL_LINK_BYTE_TIMEOUT as u32) >> 8) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte({ const N: i32 = (0xcf5d as i32) + (1 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    become serial_exchange_byte_non_serial_interrupts_enabled(cpu);
}

pub(crate) fn serial_exchange_byte_non_serial_interrupts_enabled(cpu: &mut Cpu) {
    cpu.pc = 0x07f8;
    cpu.a = cpu.read_byte(0xffce);
    cpu.pc += 2;
    cpu.cycle(12);
    let value = crate::defs::SERIAL_NO_DATA_BYTE as u8;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    check_link_timeout_frames_nonzero(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_z {
        cpu.cycle(12);
        become serial_exchange_byte_timed_out(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.set_hl({ const N: i32 = (0xcf5b as i32) + (1 as i32); N as u16 });
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
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(12);
        become serial_exchange_byte_no_rollover(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let addr = cpu.hl();
    let value = cpu.read_byte(addr);
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.write_byte(addr, result);
    cpu.pc += 1;
    cpu.cycle(12);
    become serial_exchange_byte_no_rollover(cpu);
}

pub(crate) fn serial_exchange_byte_no_rollover(cpu: &mut Cpu) {
    cpu.pc = 0x080d;
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    check_link_timeout_frames_nonzero(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_z {
        cpu.cycle(12);
        become serial_disconnected(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become serial_exchange_byte_timed_out(cpu);
}

pub(crate) fn serial_exchange_byte_timed_out(cpu: &mut Cpu) {
    cpu.pc = 0x0813;
    cpu.a = cpu.read_byte(crate::defs::R_IE as u16);
    cpu.pc += 2;
    cpu.cycle(12);
    let value = { let value = ((((crate::defs::IE_SERIAL as i32) | (crate::defs::IE_TIMER as i32)) | (crate::defs::IE_STAT as i32)) | (crate::defs::IE_VBLANK as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = crate::defs::IE_SERIAL as u8;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = crate::defs::SERIAL_NO_DATA_BYTE as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(0xffcd, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::delay::delay_frame(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(16);
    become serial_exchange_byte_timeout_loop(cpu);
}

pub(crate) fn serial_exchange_byte_short_delay(cpu: &mut Cpu) {
    cpu.pc = 0x0825;
    cpu.a = 15;
    cpu.pc += 2;
    cpu.cycle(8);
    become serial_exchange_byte_short_delay_loop(cpu);
}

pub(crate) fn serial_exchange_byte_short_delay_loop(cpu: &mut Cpu) {
    cpu.pc = 0x0827;
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
        become serial_exchange_byte_short_delay_loop(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub fn check_link_timeout_frames_nonzero(cpu: &mut Cpu) {
    cpu.pc = 0x082b;
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.set_hl(0xcf5b);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.read_byte(cpu.hl());
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub fn serial_disconnected(cpu: &mut Cpu) {
    cpu.pc = 0x0833;
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcf5b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte({ const N: i32 = (0xcf5b as i32) + (1 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub fn serial_exchange_sync_bytes(cpu: &mut Cpu) {
    cpu.pc = 0x083b;
    cpu.set_hl(0xcf56);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xcf51);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.c = 2;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = crate::defs::TRUE as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xffcc, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    become serial_exchange_sync_bytes_exchange(cpu);
}

pub(crate) fn serial_exchange_sync_bytes_exchange(cpu: &mut Cpu) {
    cpu.pc = 0x0847;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::delay::delay_frame(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(0xffcd, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    serial_exchange_byte(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xffcc);
    cpu.pc += 2;
    cpu.cycle(12);
    let value = cpu.a;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = crate::defs::FALSE as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xffcc, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    if !cpu.flag_z {
        cpu.cycle(12);
        become serial_exchange_sync_bytes_exchange(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
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
        become serial_exchange_sync_bytes_exchange(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub fn serial_place_waiting_text_and_sync_and_exchange_nybble(cpu: &mut Cpu) {
    cpu.pc = 0x0862;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy_tilemap::load_tilemap_to_temp_tilemap(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0x4000);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0x01;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    wait_link_transfer(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(16);
    become crate::game::home::copy_tilemap::safe_load_temp_tilemap_to_tilemap(cpu);
}

pub fn serial_sync_and_exchange_nybble(cpu: &mut Cpu) {
    cpu.pc = 0x0871;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy_tilemap::load_tilemap_to_temp_tilemap(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0x4000);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0x01;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(16);
    become wait_link_transfer(cpu);
}

pub fn wait_link_transfer(cpu: &mut Cpu) {
    cpu.pc = 0x087d;
    cpu.a = 0xff;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcf52, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    become wait_link_transfer_loop(cpu);
}

pub(crate) fn wait_link_transfer_loop(cpu: &mut Cpu) {
    cpu.pc = 0x0882;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    link_transfer(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::delay::delay_frame(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    check_link_timeout_frames_nonzero(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_z {
        cpu.cycle(12);
        become wait_link_transfer_check(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.set_hl({ const N: i32 = (0xcf5b as i32) + (1 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    let addr = cpu.hl();
    let value = cpu.read_byte(addr);
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.write_byte(addr, result);
    cpu.pc += 1;
    cpu.cycle(12);
    if !cpu.flag_z {
        cpu.cycle(12);
        become wait_link_transfer_skip(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let addr = cpu.hl();
    let value = cpu.read_byte(addr);
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.write_byte(addr, result);
    cpu.pc += 1;
    cpu.cycle(12);
    if !cpu.flag_z {
        cpu.cycle(12);
        become wait_link_transfer_skip(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.cycle(16);
    become serial_disconnected(cpu);
}

pub(crate) fn wait_link_transfer_skip(cpu: &mut Cpu) {
    cpu.pc = 0x089d;
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    become wait_link_transfer_check(cpu);
}

pub(crate) fn wait_link_transfer_check(cpu: &mut Cpu) {
    cpu.pc = 0x089e;
    cpu.a = cpu.read_byte(0xcf52);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.a;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become wait_link_transfer_loop(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.b = 10;
    cpu.pc += 2;
    cpu.cycle(8);
    become wait_link_transfer_receive(cpu);
}

pub(crate) fn wait_link_transfer_receive(cpu: &mut Cpu) {
    cpu.pc = 0x08a6;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::delay::delay_frame(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    link_transfer(cpu);
    assert_eq!(cpu.pc, next_pc);
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
        become wait_link_transfer_receive(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.b = 10;
    cpu.pc += 2;
    cpu.cycle(8);
    become wait_link_transfer_acknowledge(cpu);
}

pub(crate) fn wait_link_transfer_acknowledge(cpu: &mut Cpu) {
    cpu.pc = 0x08b1;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::delay::delay_frame(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    link_data_received(cpu);
    assert_eq!(cpu.pc, next_pc);
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
        become wait_link_transfer_acknowledge(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xcf52);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xcf51, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub fn link_transfer(cpu: &mut Cpu) {
    cpu.pc = 0x08c1;
    cpu.push_word(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.b = crate::defs::SERIAL_TIMECAPSULE as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc2dc);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = crate::defs::LINK_TIMECAPSULE as u8;
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
        become link_transfer_got_high_nybble(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.b = crate::defs::SERIAL_TIMECAPSULE as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_c {
        cpu.cycle(12);
        become link_transfer_got_high_nybble(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = crate::defs::LINK_TRADECENTER as u8;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.b = crate::defs::SERIAL_TRADECENTER as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become link_transfer_got_high_nybble(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.b = crate::defs::SERIAL_BATTLE as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    become link_transfer_got_high_nybble(cpu);
}

pub(crate) fn link_transfer_got_high_nybble(cpu: &mut Cpu) {
    cpu.pc = 0x08d7;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    link_transfer_receive(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xcf56);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.b;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_add(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) > 0x0f;
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xffcd, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xffcb);
    cpu.pc += 2;
    cpu.cycle(12);
    let value = crate::defs::USING_INTERNAL_CLOCK as u8;
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
        become link_transfer_player_1(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = crate::defs::SC_INTERNAL as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_SC as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = { let value = ((crate::defs::SC_START as i32) | (crate::defs::SC_INTERNAL as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_SC as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    become link_transfer_player_1(cpu);
}

pub(crate) fn link_transfer_player_1(cpu: &mut Cpu) {
    cpu.pc = 0x08ee;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    link_transfer_receive(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_bc(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn link_transfer_receive(cpu: &mut Cpu) {
    cpu.pc = 0x08f3;
    cpu.a = cpu.read_byte(0xffce);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.write_byte(0xcf51, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0xf0;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.b;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xffce, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xcf51);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0xf;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcf52, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub fn link_data_received(cpu: &mut Cpu) {
    cpu.pc = 0x0908;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xffcd, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xffcb);
    cpu.pc += 2;
    cpu.cycle(12);
    let value = crate::defs::USING_INTERNAL_CLOCK as u8;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = crate::defs::SC_INTERNAL as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_SC as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = { let value = ((crate::defs::SC_START as i32) | (crate::defs::SC_INTERNAL as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_SC as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub fn set_bits_for_time_capsule_request_if_not_linked(cpu: &mut Cpu) {
    cpu.pc = 0x0919;
    cpu.a = cpu.read_byte(0xc2dc);
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
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = crate::defs::USING_INTERNAL_CLOCK as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_SB as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xffce, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = crate::defs::SC_EXTERNAL as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_SC as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = { let value = ((crate::defs::SC_START as i32) | (crate::defs::SC_EXTERNAL as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_SC as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}
