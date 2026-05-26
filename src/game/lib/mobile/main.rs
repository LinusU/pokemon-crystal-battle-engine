use crate::runtime::Cpu;

pub const MOBILE_COMMAND_BEGIN_SESSION: u8 = 16;

pub const MOBILE_COMMAND_END_SESSION: u8 = 17;

pub const MOBILE_COMMAND_DIAL_TELEPHONE: u8 = 18;

pub const MOBILE_COMMAND_HANG_UP_TELEPHONE: u8 = 19;

pub const MOBILE_COMMAND_WAIT_FOR_TELEPHONE_CALL: u8 = 20;

pub const MOBILE_COMMAND_TRANSFER_DATA: u8 = 21;

pub const MOBILE_COMMAND_TELEPHONE_STATUS: u8 = 23;

pub const MOBILE_COMMAND_READ_CONFIGURATION_DATA: u8 = 25;

pub const MOBILE_COMMAND_WRITE_CONFIGURATION_DATA: u8 = 26;

pub const MOBILE_COMMAND_TRANSFER_DATA_END: u8 = 31;

pub const MOBILE_COMMAND_ISP_LOGIN: u8 = 33;

pub const MOBILE_COMMAND_ISP_LOGOUT: u8 = 34;

pub const MOBILE_COMMAND_OPEN_TCP_CONNECTION: u8 = 35;

pub const MOBILE_COMMAND_CLOSE_TCP_CONNECTION: u8 = 36;

pub const MOBILE_COMMAND_DNS_QUERY: u8 = 40;

pub const MOBILE_COMMAND_ERROR: u8 = 110;

pub(crate) fn mobile_sdk_copy_bytes(cpu: &mut Cpu) {
    cpu.pc = 0x4000;
    become mobile_sdk_copy_bytes_loop(cpu);
}

pub(crate) fn mobile_sdk_copy_bytes_loop(cpu: &mut Cpu) {
    cpu.pc = 0x4000;
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
        become mobile_sdk_copy_bytes_loop(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn mobile_sdk_copy_string(cpu: &mut Cpu) {
    cpu.pc = 0x4007;
    become mobile_sdk_copy_string_loop(cpu);
}

pub(crate) fn mobile_sdk_copy_string_loop(cpu: &mut Cpu) {
    cpu.pc = 0x4007;
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.a;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
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
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_bc(cpu.bc().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.cycle(12);
    become mobile_sdk_copy_string_loop(cpu);
}

pub(crate) fn mobile_sdk_copy_string_len(cpu: &mut Cpu) {
    cpu.pc = 0x400f;
    cpu.push_word(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.c = 0x0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.b;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.b = result;
    cpu.pc += 1;
    cpu.cycle(4);
    become mobile_sdk_copy_string_len_loop(cpu);
}

pub(crate) fn mobile_sdk_copy_string_len_loop(cpu: &mut Cpu) {
    cpu.pc = 0x4014;
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.a;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become mobile_sdk_copy_string_len_done(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.c;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.c = result;
    cpu.pc += 1;
    cpu.cycle(4);
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
        become mobile_sdk_copy_string_len_loop(cpu);
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
    become mobile_sdk_copy_string_len_done(cpu);
}

pub(crate) fn mobile_sdk_copy_string_len_done(cpu: &mut Cpu) {
    cpu.pc = 0x4020;
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.pop_word();
    cpu.set_bc(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let value = cpu.c;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_add(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) > 0x0f;
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = 0;
    let carry_in = u8::from(cpu.flag_c);
    let lhs = cpu.a;
    let (partial, carry1) = lhs.overflowing_add(value);
    let (result, carry2) = partial.overflowing_add(carry_in);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) + carry_in > 0x0f;
    cpu.flag_c = carry1 || carry2;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn reset_receive_packet_buffer(cpu: &mut Cpu) {
    cpu.pc = 0x4029;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl(0xca3a);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub fn _mobile_api(cpu: &mut Cpu) {
    cpu.pc = 0x4030;
    cpu.push_word(cpu.de());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xc988);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = crate::defs::MOBILEAPI_06 as u8;
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
        become _mobile_api_noreset(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = crate::defs::MOBILEAPI_07 as u8;
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
        become _mobile_api_noreset(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = crate::defs::MOBILEAPI_08 as u8;
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
        become _mobile_api_noreset(cpu);
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
    cpu.write_byte(0xc835, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xc988);
    cpu.pc += 3;
    cpu.cycle(16);
    become _mobile_api_noreset(cpu);
}

pub(crate) fn _mobile_api_noreset(cpu: &mut Cpu) {
    cpu.pc = 0x4047;
    cpu.d = 0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl(0x4070);
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
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(0xc988, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.pop_word();
    cpu.set_de(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.set_hl(0x3e60);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.h = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xc988);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = ((0x4236u32) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.l;
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
        become _mobile_api_okay(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = (((0x4236u32) >> 8) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.h;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 1;
    cpu.cycle(4);
    become _mobile_api_okay(cpu);
}

pub(crate) fn _mobile_api_okay(cpu: &mut Cpu) {
    cpu.pc = 0x4066;
    if !cpu.flag_z {
        let next_pc = cpu.pc + 3;
        cpu.push_word(next_pc);
        cpu.cycle(24);
        function1100b4(cpu);
        assert_eq!(cpu.pc, next_pc);
    } else {
        cpu.pc += 3;
        cpu.cycle(12);
    }
    cpu.set_hl(0xc986);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.h = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function1100b4(cpu: &mut Cpu) {
    cpu.pc = 0x40b4;
    cpu.push_word(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);
    become function1100b4_loop(cpu);
}

pub(crate) fn function1100b4_loop(cpu: &mut Cpu) {
    cpu.pc = 0x40b5;
    cpu.disable_interrupts_after_next_instruction();
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xc800);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xc80b);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xc822);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.enable_interrupts_after_next_instruction();
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.a;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    let mask = 1u8 << ((0) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become function1100b4_done(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.a;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(12);
        become function1100b4_loop(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = 0x4;
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
        become function1100b4_loop(cpu);
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
    cpu.write_byte(0xc80f, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xc821);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((1) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value |= mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = true;
    cpu.pc += 1;
    cpu.cycle(4);
    become function1100b4_done(cpu);
}

pub(crate) fn function1100b4_done(cpu: &mut Cpu) {
    cpu.pc = 0x40da;
    let value = cpu.pop_word();
    cpu.set_bc(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn mobile_api_set_timer(cpu: &mut Cpu) {
    cpu.pc = 0x40dc;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(crate::defs::R_TAC as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.e = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl(0x6089);
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
    cpu.c = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(crate::defs::R_SPD as u16);
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
        become mobile_api_set_timer_asm_1100f9(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.c;
    let result = (value >> 1) | (value & 0x80);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x01 != 0;
    cpu.c = result;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = 0x4;
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
        become mobile_api_set_timer_asm_1100f9(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_de(0x000f as u16);
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
    become mobile_api_set_timer_asm_1100f9(cpu);
}

pub(crate) fn mobile_api_set_timer_asm_1100f9(cpu: &mut Cpu) {
    cpu.pc = 0x40f9;
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(crate::defs::R_TMA as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.write_byte(crate::defs::R_TIMA as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(0xc81f, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xc816, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(0xc820, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xc815, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.c = ((crate::defs::R_TAC as u32) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = crate::defs::TAC_65KHZ as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xff00u16 + u16::from(cpu.c), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = { let value = ((crate::defs::TAC_START as i32) | (crate::defs::TAC_65KHZ as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xff00u16 + u16::from(cpu.c), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function110115(cpu: &mut Cpu) {
    cpu.pc = 0x4115;
    cpu.set_hl(0xc821);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((1) & 7);
    let value = cpu.read_byte(cpu.hl());
    cpu.flag_z = (value & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(12);
    if !cpu.flag_z {
        cpu.cycle(12);
        become function110115_asm_110120(cpu);
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
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.h = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function110115_asm_110120(cpu: &mut Cpu) {
    cpu.pc = 0x4120;
    let mask = 1u8 << ((1) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xc80f);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = 0x22;
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
        become function110115_asm_11016a(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x23;
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
        become function110115_asm_11016a(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x25;
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
        become function110115_asm_11016a(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x26;
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
        become function110115_asm_11018e(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x24;
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
        become function110115_asm_1101a4(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x30;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(16);
        become function110115_asm_1101f8(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    let value = 0x31;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(16);
        become function110115_asm_11020d(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    let value = 0x32;
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
        become function110115_asm_1101a4(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x33;
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
        become function110115_asm_1101a4(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value.rotate_left(4);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0xf;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x1;
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
        become function110115_asm_11016a(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x0;
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
        become function110115_asm_11015d(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become function110115_asm_110158(cpu);
}

pub(crate) fn function110115_asm_110158(cpu: &mut Cpu) {
    cpu.pc = 0x4158;
    cpu.set_hl(0 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    become function110115_asm_11015b(cpu);
}

pub(crate) fn function110115_asm_11015b(cpu: &mut Cpu) {
    cpu.pc = 0x415b;
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function110115_asm_11015d(cpu: &mut Cpu) {
    cpu.pc = 0x415d;
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = 0x15;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_add(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) > 0x0f;
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl(0xc810);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(0xc821);
    cpu.pc += 3;
    cpu.cycle(12);
    become function110115_asm_11016a(cpu);
}

pub(crate) fn function110115_asm_11016a(cpu: &mut Cpu) {
    cpu.pc = 0x416a;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc86d, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(0xc807, cpu.a);
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
    cpu.write_byte(0xc86a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xc822);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((0) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    let mask = 1u8 << ((5) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.set_hl(0xcb47);
    cpu.pc += 3;
    cpu.cycle(12);
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
    let value = cpu.a;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function111686(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x15;
    cpu.pc += 2;
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
    if !cpu.flag_z {
        cpu.cycle(12);
        become function110115_asm_110158(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become function110115_asm_1101d7(cpu);
}

pub(crate) fn function110115_asm_11018e(cpu: &mut Cpu) {
    cpu.pc = 0x418e;
    cpu.a = cpu.read_byte(0xc821);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((4) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become function110115_asm_11016a(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x2;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc86a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xc805);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xc807, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(12);
    become function110115_asm_110158(cpu);
}

pub(crate) fn function110115_asm_1101a4(cpu: &mut Cpu) {
    cpu.pc = 0x41a4;
    let mask = 1u8 << ((0) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.set_hl(0xc822);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((5) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.set_hl(0xc821);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((7) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    let mask = 1u8 << ((6) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    let mask = 1u8 << ((5) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value |= mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc86d, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xc9af, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x2;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc86a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x4;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc807, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = 0x32;
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
        become function110115_asm_1101d7(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x33;
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
        become function110115_asm_1101d7(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x30;
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
        become function110115_asm_1101d7(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x31;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(16);
        become function110115_asm_110158(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    become function110115_asm_1101d7(cpu);
}

pub(crate) fn function110115_asm_1101d7(cpu: &mut Cpu) {
    cpu.pc = 0x41d7;
    cpu.set_hl(0xc810);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.h = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = 0x32;
    cpu.pc += 2;
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
    if !cpu.flag_z {
        cpu.cycle(16);
        become function110115_asm_11015b(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0x3;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.h;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(16);
        become function110115_asm_11015b(cpu);
    }
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
    let value = cpu.l;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function110115_asm_1101f2(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.l;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(16);
        become function110115_asm_11015b(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    become function110115_asm_1101f2(cpu);
}

pub(crate) fn function110115_asm_1101f2(cpu: &mut Cpu) {
    cpu.pc = 0x41f2;
    cpu.set_bc(0xc880);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become function110115_asm_11015b(cpu);
}

pub(crate) fn function110115_asm_1101f8(cpu: &mut Cpu) {
    cpu.pc = 0x41f8;
    cpu.a = cpu.read_byte(0xca3c);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0xa4;
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
        become function110115_asm_1101a4(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x3;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc86a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xc810);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.h = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.cycle(16);
    become function110115_asm_11015b(cpu);
}

pub(crate) fn function110115_asm_11020d(cpu: &mut Cpu) {
    cpu.pc = 0x420d;
    cpu.a = cpu.read_byte(0xc810);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x2;
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
        become function110115_asm_1101a4(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x3;
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
        become function110115_asm_1101a4(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x4;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc86a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xc810);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.h = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.cycle(16);
    become function110115_asm_11015b(cpu);
}

pub(crate) fn function110226(cpu: &mut Cpu) {
    cpu.pc = 0x4226;
    cpu.a = 0x21;
    cpu.pc += 2;
    cpu.cycle(8);
    become function110228(cpu);
}

pub(crate) fn function110228(cpu: &mut Cpu) {
    cpu.pc = 0x4228;
    cpu.write_byte(0xc80f, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xc821);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((1) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value |= mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function110231(cpu: &mut Cpu) {
    cpu.pc = 0x4231;
    cpu.a = 0x20;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become function110228(cpu);
}

pub(crate) fn function110235(cpu: &mut Cpu) {
    cpu.pc = 0x4235;
    cpu.pc += 1;
    cpu.cycle(4);
    become function110236(cpu);
}

pub(crate) fn function110236(cpu: &mut Cpu) {
    cpu.pc = 0x4236;
    cpu.a = cpu.read_byte(0xc988);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.push_word(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(crate::defs::R_TAC as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(crate::defs::R_IF as u16);
    cpu.pc += 2;
    cpu.cycle(12);
    let value = 0x1b;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_IF as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    reset_receive_packet_buffer(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_bc(0x0452 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0xc800);
    cpu.pc += 3;
    cpu.cycle(12);
    become function110236_asm_11024e(cpu);
}

pub(crate) fn function110236_asm_11024e(cpu: &mut Cpu) {
    cpu.pc = 0x424e;
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
    cpu.set_bc(cpu.bc().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.b;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(12);
        become function110236_asm_11024e(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc822);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((6) & 7);
    cpu.a |= mask;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc822, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.a = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc981, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc982, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.pop_word();
    cpu.set_bc(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.set_hl(0xc983);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(0xc86e);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.d);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc819, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.c = 0xc;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_api_set_timer(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function1104b0(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let value = 0x35;
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
        become function110236_asm_110289(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x2b;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become function110236_asm_11028b(cpu);
}

pub(crate) fn function110236_asm_110289(cpu: &mut Cpu) {
    cpu.pc = 0x4289;
    cpu.a = 0xa;
    cpu.pc += 2;
    cpu.cycle(8);
    become function110236_asm_11028b(cpu);
}

pub(crate) fn function110236_asm_11028b(cpu: &mut Cpu) {
    cpu.pc = 0x428b;
    cpu.write_byte(0xc86a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(16);
    become function110432(cpu);
}

pub(crate) fn function110291(cpu: &mut Cpu) {
    cpu.pc = 0x4291;
    cpu.a = cpu.read_byte(0xc821);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((1) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become function110291_asm_1102a6(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc80f);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x14;
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
        become function110291_asm_1102b3(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x25;
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
        become function110291_asm_1102b3(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc821);
    cpu.pc += 3;
    cpu.cycle(16);
    become function110291_asm_1102a6(cpu);
}

pub(crate) fn function110291_asm_1102a6(cpu: &mut Cpu) {
    cpu.pc = 0x42a6;
    let mask = 1u8 << ((0) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(16);
        become function110226(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xc86a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x1;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(16);
        become function110226(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    become function110291_asm_1102b3(cpu);
}

pub(crate) fn function110291_asm_1102b3(cpu: &mut Cpu) {
    cpu.pc = 0x42b3;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(crate::defs::R_TAC as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc819, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.b = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl(0xc880);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc870);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_api_set_timer(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xc829);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = ((0xc872u32) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = (((0xc872u32) >> 8) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(0xcb47);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = { let value = ((0x6063 as i32) - (0x605e as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x605e);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xc882);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.a;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function110291_asm_1102f2(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x80;
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
        become function110291_asm_1102f2(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.c = 0x80;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become function110291_asm_1102f4(cpu);
}

pub(crate) fn function110291_asm_1102f2(cpu: &mut Cpu) {
    cpu.pc = 0x42f2;
    cpu.a = 0x80;
    cpu.pc += 2;
    cpu.cycle(8);
    become function110291_asm_1102f4(cpu);
}

pub(crate) fn function110291_asm_1102f4(cpu: &mut Cpu) {
    cpu.pc = 0x42f4;
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.a;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 0x80;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.c;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_add(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) > 0x0f;
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl(0xc882);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0x80;
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
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(0xc880);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.h = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.c = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc880, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte({ const N: i32 = (0xc880 as i32) + (1 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.b = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.b;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.b = result;
    cpu.pc += 1;
    cpu.cycle(4);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function111f63(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function1104b0(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x2e;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc86a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xc821);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((1) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    let mask = 1u8 << ((0) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value |= mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function11032c(cpu: &mut Cpu) {
    cpu.pc = 0x432c;
    cpu.a = cpu.read_byte(0xc821);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((1) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(16);
        become function110226(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((0) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(16);
        become function110226(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xc86a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x1;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(16);
        become function110226(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(crate::defs::R_TAC as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.write_byte(0xc819, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xc880);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(0xc829);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc870);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_api_set_timer(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_de(0xcb47);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = 6;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x6046);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xc883);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc882);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.a;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function11032c_asm_11037f(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x80;
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
        become function11032c_asm_11037f(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.c = 0x80;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become function11032c_asm_110381(cpu);
}

pub(crate) fn function11032c_asm_11037f(cpu: &mut Cpu) {
    cpu.pc = 0x437f;
    cpu.a = 0x80;
    cpu.pc += 2;
    cpu.cycle(8);
    become function11032c_asm_110381(cpu);
}

pub(crate) fn function11032c_asm_110381(cpu: &mut Cpu) {
    cpu.pc = 0x4381;
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.b = 0x2;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function111f63(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function1104b0(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x2d;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc86a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(16);
    become function110432(cpu);
}

pub(crate) fn function110393(cpu: &mut Cpu) {
    cpu.pc = 0x4393;
    cpu.c = ((crate::defs::R_IE as u32) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xff00u16 + u16::from(cpu.c));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = { let value = ((crate::defs::IE_SERIAL as i32) | (crate::defs::IE_TIMER as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xff00u16 + u16::from(cpu.c), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function11039a(cpu: &mut Cpu) {
    cpu.pc = 0x439a;
    cpu.b = 0x0;
    cpu.pc += 2;
    cpu.cycle(8);
    become function11039a_asm_11039c(cpu);
}

pub(crate) fn function11039a_asm_11039c(cpu: &mut Cpu) {
    cpu.pc = 0x439c;
    let value = cpu.b;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.b = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function11039a_asm_1103a3(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.a;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(12);
        become function11039a_asm_11039c(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become function11039a_asm_1103a3(cpu);
}

pub(crate) fn function11039a_asm_1103a3(cpu: &mut Cpu) {
    cpu.pc = 0x43a3;
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.c;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_c {
        cpu.cycle(12);
        become function11039a_asm_1103aa(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x2;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function11039a_asm_1103aa(cpu: &mut Cpu) {
    cpu.pc = 0x43aa;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = true;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function1103ac(cpu: &mut Cpu) {
    cpu.pc = 0x43ac;
    cpu.a = cpu.read_byte(0xc821);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((0) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(16);
        become function110226(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xc86a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x1;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(16);
        become function110226(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.c = 0x15;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11039a(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.cycle(12);
        become function1103ac_asm_1103d2(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.c = 0x22;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11039a(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.cycle(12);
        become function1103ac_asm_1103d2(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.c = 0x12;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11039a(cpu);
    assert_eq!(cpu.pc, next_pc);
    if !cpu.flag_c {
        cpu.cycle(12);
        become function1103ac_isp_login(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become function1103ac_asm_1103d2(cpu);
}

pub(crate) fn function1103ac_asm_1103d2(cpu: &mut Cpu) {
    cpu.pc = 0x43d2;
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.cycle(16);
    become function110231(cpu);
}

pub(crate) fn function1103ac_isp_login(cpu: &mut Cpu) {
    cpu.pc = 0x43d6;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(crate::defs::R_TAC as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.write_byte(0xc86d, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xc97a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xc870);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_api_set_timer(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xc829);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = ((0xc880u32) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = (((0xc880u32) >> 8) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_dial_telephone(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function111f63(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.b = { let value = ((0x603c as i32) - (0x6037 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x6037);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de({ const N: i32 = (0xcb47 as i32) + (45 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.set_bc(0 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_string(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte({ const N: i32 = (0xcb47 as i32) + (51 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xc86b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.push_word(cpu.de());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_bc(0 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0x20;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_string_len(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.l = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.h = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.pop_word();
    cpu.set_de(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc86b);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.c;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_add(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) > 0x0f;
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = 0xa;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_add(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) > 0x0f;
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte({ const N: i32 = (0xcb47 as i32) + (50 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function1104b0(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0xb;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc86a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    become function110432(cpu);
}

pub(crate) fn function110432(cpu: &mut Cpu) {
    cpu.pc = 0x4432;
    cpu.set_hl(0xc821);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((0) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value |= mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function110438(cpu: &mut Cpu) {
    cpu.pc = 0x4438;
    cpu.a = cpu.read_byte(0xc821);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((0) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(16);
        become function110226(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xc86a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x1;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(16);
        become function110226(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.c = 0x15;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11039a(cpu);
    assert_eq!(cpu.pc, next_pc);
    if !cpu.flag_c {
        cpu.cycle(12);
        become function110438_asm_110454(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.cycle(16);
    become function110231(cpu);
}

pub(crate) fn function110438_asm_110454(cpu: &mut Cpu) {
    cpu.pc = 0x4454;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(crate::defs::R_TAC as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.write_byte(0xc97a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xc870);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_api_set_timer(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xc98f);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = ((((0xc880 as u32) + (1 as u32))) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = (((((0xc880 as u32) + (1 as u32))) >> 8) & 0x00ff) as u8;
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
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 0xff;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc86e, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_dial_telephone(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function111f63(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function1104b0(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0xc;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc86a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(12);
    become function110432(cpu);
}

pub(crate) fn mobile_dial_telephone(cpu: &mut Cpu) {
    cpu.pc = 0x4485;
    cpu.set_de(0xcb47);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0x601d);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = { let value = ((0x6023 as i32) - (0x601d as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_bc(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.push_word(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xc818);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x8c;
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
        become mobile_dial_telephone_asm_11049e(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x3;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become mobile_dial_telephone_asm_1104a1(cpu);
}

pub(crate) fn mobile_dial_telephone_asm_11049e(cpu: &mut Cpu) {
    cpu.pc = 0x449e;
    cpu.a = cpu.read_byte(0xc871);
    cpu.pc += 3;
    cpu.cycle(16);
    become mobile_dial_telephone_asm_1104a1(cpu);
}

pub(crate) fn mobile_dial_telephone_asm_1104a1(cpu: &mut Cpu) {
    cpu.pc = 0x44a1;
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_bc(0x0001 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = crate::defs::MOBILE_PHONE_NUMBER_LENGTH as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_string_len(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte({ const N: i32 = (0xcb47 as i32) + (5 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function1104b0(cpu: &mut Cpu) {
    cpu.pc = 0x44b0;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc81e, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function110393(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc86b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_de({ const N: i32 = (0x6001 as i32) - (0x6000 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0x6000);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = 1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become packet_send_bytes(cpu);
}

pub(crate) fn function1104c6(cpu: &mut Cpu) {
    cpu.pc = 0x44c6;
    cpu.a = cpu.read_byte(0xc821);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((0) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(16);
        become function110226(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xc86a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x4;
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
        become function1104c6_asm_110526(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x3;
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
        become function1104c6_asm_110526(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x2;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(16);
        become function110226(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0xc822);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((4) & 7);
    let value = cpu.read_byte(cpu.hl());
    cpu.flag_z = (value & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(12);
    if !cpu.flag_z {
        cpu.cycle(12);
        become function1104c6_asm_110507(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x2;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc86b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = { let value = ((MOBILE_COMMAND_ISP_LOGOUT as i32) | (0x80 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc81e, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_de({ const N: i32 = (0x6046 as i32) - (0x603c as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0x603c);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = 0x5;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    packet_send_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    become function1104c6_asm_1104fa(cpu);
}

pub(crate) fn function1104c6_asm_1104fa(cpu: &mut Cpu) {
    cpu.pc = 0x44fa;
    cpu.a = 0xe;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc86a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xc821);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((0) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value |= mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    let mask = 1u8 << ((3) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function1104c6_asm_110507(cpu: &mut Cpu) {
    cpu.pc = 0x4507;
    cpu.a = cpu.read_byte(0xc807);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.a;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(12);
        become function1104c6_asm_11051f(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc86a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xc822);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((4) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.set_hl(0xc821);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0x17;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function1104c6_asm_11051f(cpu: &mut Cpu) {
    cpu.pc = 0x451f;
    cpu.a = 0x2;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc86b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(12);
    become function1104c6_asm_1104fa(cpu);
}

pub(crate) fn function1104c6_asm_110526(cpu: &mut Cpu) {
    cpu.pc = 0x4526;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function112724(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc86b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_de({ const N: i32 = (0xcb47 as i32) + (32 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0x6072);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = 0x6;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xc86c);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.b = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function111f63(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_de(0xcb47);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0x6072);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = 0x5;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x7;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc86c);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_bc(0x0001 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0x60c1);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_string(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.b = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function111f63(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = { let value = ((MOBILE_COMMAND_TRANSFER_DATA as i32) | (0x80 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc81e, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xcb47);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = 0x5;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    packet_send_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0xe;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc86a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(16);
    become function110432(cpu);
}

pub(crate) fn function110578(cpu: &mut Cpu) {
    cpu.pc = 0x4578;
    cpu.b = 0x25;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function110596(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.a;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(16);
        become function1135ba(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function110582(cpu: &mut Cpu) {
    cpu.pc = 0x4582;
    cpu.b = 0x26;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function110596(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.a;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(16);
        become function11359d(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function11058c(cpu: &mut Cpu) {
    cpu.pc = 0x458c;
    cpu.b = 0x27;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function110596(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.a;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(16);
        become function1135ad(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function110596(cpu: &mut Cpu) {
    cpu.pc = 0x4596;
    cpu.a = cpu.read_byte(0xc821);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((0) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(12);
        become function110596_asm_1105d9(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc86a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x1;
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
        become function110596_asm_1105d9(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc835);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.a;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
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
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcb36, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(crate::defs::R_TAC as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc86e, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte({ const N: i32 = (0xc86e as i32) + (1 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc819, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xc870);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_api_set_timer(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xc829);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = ((0xc880u32) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = (((0xc880u32) >> 8) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function1104b0(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xcb36);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xc86a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.cycle(16);
    become function110432(cpu);
}

pub(crate) fn function110596_asm_1105d9(cpu: &mut Cpu) {
    cpu.pc = 0x45d9;
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.cycle(16);
    become function110226(cpu);
}

pub(crate) fn function1105dd(cpu: &mut Cpu) {
    cpu.pc = 0x45dd;
    cpu.a = cpu.read_byte(0xc821);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((0) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(16);
        become function110226(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xc86a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x1;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(16);
        become function110226(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(crate::defs::R_TAC as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xc870);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_api_set_timer(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xc98f);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = ((((0xc880 as u32) + (1 as u32))) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = (((((0xc880 as u32) + (1 as u32))) >> 8) & 0x00ff) as u8;
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
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 0xff;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc86e, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function1104b0(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0xd;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc86a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(16);
    become function110432(cpu);
}

pub(crate) fn function110615(cpu: &mut Cpu) {
    cpu.pc = 0x4615;
    cpu.b = 0x15;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc86e, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.a;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function110615_asm_110625(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function110615_asm_11062c(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(16);
        become function110615_asm_1106c1(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function110615_asm_110625(cpu: &mut Cpu) {
    cpu.pc = 0x4625;
    cpu.a = 0x19;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xc83e);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(12);
    become function110615_asm_110631(cpu);
}

pub(crate) fn function110615_asm_11062c(cpu: &mut Cpu) {
    cpu.pc = 0x462c;
    cpu.a = 0x6e;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xc852);
    cpu.pc += 3;
    cpu.cycle(12);
    become function110615_asm_110631(cpu);
}

pub(crate) fn function110615_asm_110631(cpu: &mut Cpu) {
    cpu.pc = 0x4631;
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.push_word(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.write_byte({ const N: i32 = (0xcb47 as i32) + (91 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xc829);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = ((((0xcb47 as u32) + (86 as u32))) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = (((((0xcb47 as u32) + (86 as u32))) >> 8) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte({ const N: i32 = (0xcb47 as i32) + (90 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xc86b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xc9af, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_de({ const N: i32 = (0xcb47 as i32) + (80 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0x607d);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = { let value = ((0x6083 as i32) - (0x607d as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_de(0xcb47);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0x6063);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = { let value = ((0x6068 as i32) - (0x6063 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_bc(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.push_word(cpu.de());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_bc(0 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_string_len(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.b = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function111f63(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xc86e);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x2;
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
        become function110615_asm_1106ac(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte({ const N: i32 = (0xca3c as i32) + (128 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.a;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function110615_asm_1106ac(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xc995);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0x99;
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
        become function110615_asm_1106ac(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0x66;
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
        become function110615_asm_1106ac(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0x23;
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
        become function110615_asm_1106ac(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x2;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc86e, cpu.a);
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
    cpu.write_byte(0xc86b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0xa3;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_de(0x0010 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0xc995);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function111f02(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0xf;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc86a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(16);
    become function110432(cpu);
}

pub(crate) fn function110615_asm_1106ac(cpu: &mut Cpu) {
    cpu.pc = 0x46ac;
    cpu.set_hl(0xcb47);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = { let value = ((MOBILE_COMMAND_DNS_QUERY as i32) | (0x80 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc81e, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.b = 0x5;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    packet_send_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0xf;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc86a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(16);
    become function110432(cpu);
}

pub(crate) fn function110615_asm_1106c1(cpu: &mut Cpu) {
    cpu.pc = 0x46c1;
    cpu.b = 0x50;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xc876);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.h = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_de(0x0007 as u16);
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
    cpu.set_de(0xc8ff);
    cpu.pc += 3;
    cpu.cycle(12);
    become function110615_asm_1106d0(cpu);
}

pub(crate) fn function110615_asm_1106d0(cpu: &mut Cpu) {
    cpu.pc = 0x46d0;
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0x2f;
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
        become function110615_asm_1106da(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
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
        become function110615_asm_1106d0(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become function110615_asm_1106da(cpu);
}

pub(crate) fn function110615_asm_1106da(cpu: &mut Cpu) {
    cpu.pc = 0x46da;
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
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc876, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc877, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xc8ff);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0x50;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.b = 0x40;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become function110615_asm_110631(cpu);
}

pub(crate) fn function1106ef(cpu: &mut Cpu) {
    cpu.pc = 0x46ef;
    cpu.a = cpu.read_byte(0xc821);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((0) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(16);
        become function110226(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xc86a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x2;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(16);
        become function110226(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xc86d);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.a;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(16);
        become function110226(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.c = 0x20;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11039a(cpu);
    assert_eq!(cpu.pc, next_pc);
    if !cpu.flag_c {
        cpu.cycle(12);
        become function1106ef_asm_110712(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.cycle(16);
    become function110231(cpu);
}

pub(crate) fn function1106ef_asm_110712(cpu: &mut Cpu) {
    cpu.pc = 0x4712;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc86b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_de({ const N: i32 = (0xcb47 as i32) + (96 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0x6072);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = 0x6;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_de({ const N: i32 = (0xcb47 as i32) + (112 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0x6072);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = 0x5;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_bc(0x0001 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0x609e);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_string(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.b = 0xff;
    cpu.pc += 2;
    cpu.cycle(8);
    become function1106ef_asm_11073b(cpu);
}

pub(crate) fn function1106ef_asm_11073b(cpu: &mut Cpu) {
    cpu.pc = 0x473b;
    let value = cpu.b;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.b = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.a;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function1106ef_asm_110744(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x40;
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
        become function1106ef_asm_11073b(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become function1106ef_asm_110744(cpu);
}

pub(crate) fn function1106ef_asm_110744(cpu: &mut Cpu) {
    cpu.pc = 0x4744;
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
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
    let value = 0x2;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_add(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) > 0x0f;
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte({ const N: i32 = (0xcb47 as i32) + (117 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11295e(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become function110615(cpu);
}

pub(crate) fn function110757(cpu: &mut Cpu) {
    cpu.pc = 0x4757;
    cpu.a = cpu.read_byte(0xc821);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((0) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(16);
        become function110226(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xc86a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x3;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(16);
        become function110226(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xc98a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.a;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(16);
        become function110226(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    become function110757_asm_11076f(cpu);
}

pub(crate) fn function110757_asm_11076f(cpu: &mut Cpu) {
    cpu.pc = 0x476f;
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.a;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(12);
        become function110757_asm_11076f(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.a;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(16);
        become function110757_asm_1107fb(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.c = 0x20;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11039a(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.cycle(12);
        become function110757_asm_1107fb(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become function110757_asm_110781(cpu);
}

pub(crate) fn function110757_asm_110781(cpu: &mut Cpu) {
    cpu.pc = 0x4781;
    cpu.c = 0x81;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11039a(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.cycle(12);
        become function110757_asm_1107fb(cpu);
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
        become function110757_asm_110781(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function112724(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc86b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_de(0xcb47);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0x6072);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = 0x6;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xc86c);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.b = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function111f63(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_de({ const N: i32 = (0xcb47 as i32) + (12 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0x6072);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = 0x5;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_de({ const N: i32 = (0xcb47 as i32) + (18 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xc86c);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_bc(0x0001 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de({ const N: i32 = (0xcb47 as i32) + (19 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0x60a4);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_string(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_string(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x3e;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.c;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.c = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc87c, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc87d, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11295e(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte({ const N: i32 = (0xcb47 as i32) + (17 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.b = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function111f63(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = { let value = ((MOBILE_COMMAND_TRANSFER_DATA as i32) | (0x80 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc81e, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl({ const N: i32 = (0xcb47 as i32) + (12 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.d = 0x0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.e = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.b = 0x5;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    packet_send_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x15;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc86a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(16);
    become function110432(cpu);
}

pub(crate) fn function110757_asm_1107fb(cpu: &mut Cpu) {
    cpu.pc = 0x47fb;
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.cycle(16);
    become function110231(cpu);
}

pub(crate) fn function1107ff(cpu: &mut Cpu) {
    cpu.pc = 0x47ff;
    cpu.a = cpu.read_byte(0xc821);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((0) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(16);
        become function110226(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xc86a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x3;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(16);
        become function110226(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xc98a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.a;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(16);
        become function110226(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.b;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(16);
        become function110231(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc87c, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc87d, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xc87e);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte({ const N: i32 = (0xc86e as i32) + (1 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function112724(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xc98a);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0x1;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x1;
    cpu.a ^= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc86b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let addr = cpu.hl();
    let value = cpu.read_byte(addr);
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.write_byte(addr, result);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.set_de(0xcb47);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0x6072);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = 0x6;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_de({ const N: i32 = (0xcb47 as i32) + (6 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xc86c);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.b = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function111f63(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_de({ const N: i32 = (0xcb47 as i32) + (150 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0x6072);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = 0x5;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_de({ const N: i32 = (0xcb47 as i32) + (156 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xc86c);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc86b);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.a;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(12);
        become function1107ff_asm_110891(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_bc(0x0001 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de({ const N: i32 = (0xcb47 as i32) + (157 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0x60ba);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_string(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte({ const N: i32 = (0xcb47 as i32) + (155 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.b = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function111f63(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = { let value = ((MOBILE_COMMAND_TRANSFER_DATA as i32) | (0x80 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc81e, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_de(0x0011 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl({ const N: i32 = (0xcb47 as i32) + (150 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = 0x5;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    packet_send_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    become function1107ff_asm_110891(cpu);
}

pub(crate) fn function1107ff_asm_110891(cpu: &mut Cpu) {
    cpu.pc = 0x4891;
    cpu.a = 0x16;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc86a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(16);
    become function110432(cpu);
}

pub(crate) fn function110899(cpu: &mut Cpu) {
    cpu.pc = 0x4899;
    cpu.a = cpu.read_byte(0xc86a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x3;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(16);
        become function110226(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(12);
    become function1108ab(cpu);
}

pub(crate) fn function1108a3(cpu: &mut Cpu) {
    cpu.pc = 0x48a3;
    cpu.a = cpu.read_byte(0xc86a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x4;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(16);
        become function110226(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    become function1108ab(cpu);
}

pub(crate) fn function1108ab(cpu: &mut Cpu) {
    cpu.pc = 0x48ab;
    cpu.set_hl(0xc821);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((0) & 7);
    let value = cpu.read_byte(cpu.hl());
    cpu.flag_z = (value & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(12);
    if !cpu.flag_z {
        cpu.cycle(16);
        become function110226(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function112724(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc86b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_de({ const N: i32 = (0xcb47 as i32) + (32 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0x6072);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = 0x6;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xc86c);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.b = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function111f63(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_de(0xcb47);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0x6072);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = 0x5;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x7;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc86c);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_bc(0x0001 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0x60c1);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_string(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.b = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function111f63(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = { let value = ((MOBILE_COMMAND_TRANSFER_DATA as i32) | (0x80 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc81e, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xcb47);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = 0x5;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    packet_send_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x17;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc86a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(16);
    become function110432(cpu);
}

pub(crate) fn function110905(cpu: &mut Cpu) {
    cpu.pc = 0x4905;
    cpu.a = cpu.read_byte(0xc821);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((0) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(16);
        become function110226(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xc86a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x2;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(16);
        become function110226(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xc86d);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.a;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(16);
        become function110226(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc86b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.c = 0x20;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11039a(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.cycle(12);
        become function110905_asm_11092f(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.c = 0x22;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11039a(cpu);
    assert_eq!(cpu.pc, next_pc);
    if !cpu.flag_c {
        cpu.cycle(12);
        become function110905_asm_110933(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become function110905_asm_11092f(cpu);
}

pub(crate) fn function110905_asm_11092f(cpu: &mut Cpu) {
    cpu.pc = 0x492f;
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.cycle(16);
    become function110231(cpu);
}

pub(crate) fn function110905_asm_110933(cpu: &mut Cpu) {
    cpu.pc = 0x4933;
    cpu.set_de({ const N: i32 = (0xcb47 as i32) + (96 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0x6072);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = 0x5;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(0x60c8);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_string(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.b = 0xff;
    cpu.pc += 2;
    cpu.cycle(8);
    become function110905_asm_11094a(cpu);
}

pub(crate) fn function110905_asm_11094a(cpu: &mut Cpu) {
    cpu.pc = 0x494a;
    let value = cpu.b;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.b = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.a;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function110905_asm_110953(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x40;
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
        become function110905_asm_11094a(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become function110905_asm_110953(cpu);
}

pub(crate) fn function110905_asm_110953(cpu: &mut Cpu) {
    cpu.pc = 0x4953;
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = 0x6;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_add(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) > 0x0f;
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte({ const N: i32 = (0xcb47 as i32) + (101 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.set_de({ const N: i32 = (0xcb47 as i32) + (108 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    become function110905_asm_110961(cpu);
}

pub(crate) fn function110905_asm_110961(cpu: &mut Cpu) {
    cpu.pc = 0x4961;
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.a;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(12);
        become function110905_asm_110961(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11295e(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte({ const N: i32 = (0xcb47 as i32) + (101 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_bc(0x0006 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de({ const N: i32 = (0xcb47 as i32) + (172 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0x20;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_string_len(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11295e(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte({ const N: i32 = (0xcb47 as i32) + (165 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_de({ const N: i32 = (0xcb47 as i32) + (160 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0x6072);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = 0x5;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_de({ const N: i32 = (0xcb47 as i32) + (167 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0x60ce);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = 0x5;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_de({ const N: i32 = (0xcb47 as i32) + (128 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0x6072);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = 0x6;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become function110615(cpu);
}

pub(crate) fn function1109a4(cpu: &mut Cpu) {
    cpu.pc = 0x49a4;
    cpu.set_hl(0xc821);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((0) & 7);
    let value = cpu.read_byte(cpu.hl());
    cpu.flag_z = (value & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(12);
    if !cpu.flag_z {
        cpu.cycle(16);
        become function110226(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xc86a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x4;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(16);
        become function110226(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc86e, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte({ const N: i32 = (0xc86e as i32) + (1 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc86b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function112729(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_de(0xcb47);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0x6072);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = 0x5;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x7;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc86c);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_bc(0x0001 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0x60d4);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_string(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.b = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function111f63(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = { let value = ((MOBILE_COMMAND_TRANSFER_DATA as i32) | (0x80 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc81e, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xcb47);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = 0x5;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    packet_send_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x18;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc86a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(16);
    become function110432(cpu);
}

pub(crate) fn function1109f9(cpu: &mut Cpu) {
    cpu.pc = 0x49f9;
    cpu.a = cpu.read_byte(0xc821);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((0) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(16);
        become function110226(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xc86a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x4;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(16);
        become function110226(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc86b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc86e, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte({ const N: i32 = (0xc86e as i32) + (1 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.h;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(16);
        become function110231(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function112729(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_de(0xcb47);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0x6072);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = 0x5;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0xd;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc86c);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_bc(0x0001 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0x60db);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_string(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_de({ const N: i32 = (0xcb47 as i32) + (12 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function110d37(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.b = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function111f63(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = { let value = ((MOBILE_COMMAND_TRANSFER_DATA as i32) | (0x80 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc81e, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xcb47);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = 0x5;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    packet_send_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x1d;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc86a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(16);
    become function110432(cpu);
}

pub(crate) fn function110a5b(cpu: &mut Cpu) {
    cpu.pc = 0x4a5b;
    cpu.a = cpu.read_byte(0xc821);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((2) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become function110a5b_asm_110a6d(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc86a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x1a;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(16);
        become function110226(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become function110af4(cpu);
}

pub(crate) fn function110a5b_asm_110a6d(cpu: &mut Cpu) {
    cpu.pc = 0x4a6d;
    let mask = 1u8 << ((0) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(16);
        become function110226(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xc86a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x4;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(16);
        become function110226(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.h;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(16);
        become function110231(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc86e, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte({ const N: i32 = (0xc86e as i32) + (1 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xc827);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_bc(cpu.bc().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_bc(cpu.bc().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(0xc98f);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(0xc829);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = ((0xc880u32) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = (((0xc880u32) >> 8) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 0x80;
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
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc86b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_de(0xcb47);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0x6072);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = 0x5;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0xd;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc86c);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_bc(0x0001 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0x60e8);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_string(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_de({ const N: i32 = (0xcb47 as i32) + (12 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0xc86e);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.h = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function110d37(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.b = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function111f63(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = { let value = ((MOBILE_COMMAND_TRANSFER_DATA as i32) | (0x80 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc81e, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xcb47);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = 0x5;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    packet_send_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x1a;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc86a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(16);
    become function110432(cpu);
}

pub(crate) fn function110af4(cpu: &mut Cpu) {
    cpu.pc = 0x4af4;
    cpu.set_hl(0xc827);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.e = cpu.read_byte(cpu.hl());
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
    cpu.write_byte(0xc86e, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte({ const N: i32 = (0xc86e as i32) + (1 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    if cpu.flag_z {
        cpu.cycle(12);
        become function110af4_asm_110b5c(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_bc(cpu.bc().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_bc(cpu.bc().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc993);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.a;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(16);
        become function110af4_asm_110bd5(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xc994);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.a;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function110af4_asm_110b1c(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    become function110af4_asm_110b1c(cpu);
}

pub(crate) fn function110af4_asm_110b1c(cpu: &mut Cpu) {
    cpu.pc = 0x4b1c;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc994, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
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
        cpu.cycle(12);
        become function110af4_asm_110b5c(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.c;
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
        become function110af4_asm_110b5c(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.push_word(cpu.bc());
    cpu.pc += 1;
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
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.b = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl(0xc82d);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xc993);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.c;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_add(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) > 0x0f;
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = 0;
    let carry_in = u8::from(cpu.flag_c);
    let lhs = cpu.a;
    let (partial, carry1) = lhs.overflowing_add(value);
    let (result, carry2) = partial.overflowing_add(carry_in);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) + carry_in > 0x0f;
    cpu.flag_c = carry1 || carry2;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc993, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl({ const N: i32 = (0xca3c as i32) + (3 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.e;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
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
    cpu.a = cpu.read_byte(0xc829);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte({ const N: i32 = (0xc829 as i32) + (1 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.d = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_bc(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.set_hl(0xc827);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.h = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.b);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function110af4_asm_110b5c(cpu: &mut Cpu) {
    cpu.pc = 0x4b5c;
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.e;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = 0x0;
    let carry_in = u8::from(cpu.flag_c);
    let lhs = cpu.a;
    let (partial, borrow1) = lhs.overflowing_sub(value);
    let (result, borrow2) = partial.overflowing_sub(carry_in);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < ((value & 0x0f) + carry_in);
    cpu.flag_c = borrow1 || borrow2;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.b);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(0xc82d);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xc993);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.e;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_add(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) > 0x0f;
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 0;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0;
    let carry_in = u8::from(cpu.flag_c);
    let lhs = cpu.a;
    let (partial, carry1) = lhs.overflowing_add(value);
    let (result, carry2) = partial.overflowing_add(carry_in);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) + carry_in > 0x0f;
    cpu.flag_c = carry1 || carry2;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc993, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xc86e);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.a;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function110af4_asm_110b9b(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.b = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl({ const N: i32 = (0xca3c as i32) + (3 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.e;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
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
    cpu.a = cpu.read_byte(0xc829);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte({ const N: i32 = (0xc829 as i32) + (1 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.d = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xc829);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    become function110af4_asm_110b9b(cpu);
}

pub(crate) fn function110af4_asm_110b9b(cpu: &mut Cpu) {
    cpu.pc = 0x4b9b;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function1127f3(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_z {
        cpu.cycle(12);
        become function110af4_asm_110bbb(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.disable_interrupts_after_next_instruction();
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl(0xc821);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((2) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc86b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_de(0x000b as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = { let value = ((MOBILE_COMMAND_TRANSFER_DATA as i32) | (0x80 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc81e, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl({ const N: i32 = (0xcb47 as i32) + (128 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = 0x5;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become packet_send_bytes(cpu);
}

pub(crate) fn function110af4_asm_110bbb(cpu: &mut Cpu) {
    cpu.pc = 0x4bbb;
    cpu.a = 0x4;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc86a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xc821);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((0) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    let mask = 1u8 << ((2) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.set_hl(0xc827);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.d = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(0xc82d);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = 0x2;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become mobile_sdk_copy_bytes(cpu);
}

pub(crate) fn function110af4_asm_110bd5(cpu: &mut Cpu) {
    cpu.pc = 0x4bd5;
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
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
        cpu.cycle(12);
        become function110af4_asm_110c05(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.c;
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
        become function110af4_asm_110c05(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.b = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl(0xc993);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
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
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 0x80;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.e;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.d = 0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xc880);
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
    cpu.a = cpu.read_byte(0xc829);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte({ const N: i32 = (0xc829 as i32) + (1 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.d = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xc827);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.h = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
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
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function110af4_asm_110c05(cpu: &mut Cpu) {
    cpu.pc = 0x4c05;
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.push_word(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xc993);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = 0x80;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.e;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.d = 0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xc880);
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
    cpu.a = cpu.read_byte(0xc829);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte({ const N: i32 = (0xc829 as i32) + (1 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.d = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc829, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte({ const N: i32 = (0xc829 as i32) + (1 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.pop_word();
    cpu.set_bc(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xc993);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.e;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = 0x0;
    let carry_in = u8::from(cpu.flag_c);
    let lhs = cpu.a;
    let (partial, borrow1) = lhs.overflowing_sub(value);
    let (result, borrow2) = partial.overflowing_sub(carry_in);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < ((value & 0x0f) + carry_in);
    cpu.flag_c = borrow1 || borrow2;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xc994);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.cycle(16);
    become function110af4_asm_110b1c(cpu);
}

pub(crate) fn function110c3c(cpu: &mut Cpu) {
    cpu.pc = 0x4c3c;
    cpu.a = cpu.read_byte(0xc821);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((0) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(16);
        become function110226(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xc86a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x4;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(16);
        become function110226(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.h;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(16);
        become function110231(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc86e, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte({ const N: i32 = (0xc86e as i32) + (1 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function112729(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_de(0xcb47);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0x6072);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = 0x5;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0xd;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc86c);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_bc(0x0001 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0x60f5);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_string(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_de({ const N: i32 = (0xcb47 as i32) + (12 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0xc86e);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.h = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function110d37(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.b = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function111f63(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = { let value = ((MOBILE_COMMAND_TRANSFER_DATA as i32) | (0x80 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc81e, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xcb47);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = 0x5;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    packet_send_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x1b;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc86a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(16);
    become function110432(cpu);
}

pub(crate) fn function110c9e(cpu: &mut Cpu) {
    cpu.pc = 0x4c9e;
    cpu.a = cpu.read_byte(0xc821);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((2) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become function110c9e_asm_110cb0(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc86a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x1c;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(16);
        become function110226(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become function110af4(cpu);
}

pub(crate) fn function110c9e_asm_110cb0(cpu: &mut Cpu) {
    cpu.pc = 0x4cb0;
    let mask = 1u8 << ((0) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(16);
        become function110226(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xc86a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x4;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(16);
        become function110226(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.h;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(16);
        become function110231(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc86e, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte({ const N: i32 = (0xc86e as i32) + (1 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xc827);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_bc(cpu.bc().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_bc(cpu.bc().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(0xc98f);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(0xc829);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = ((0xc880u32) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = (((0xc880u32) >> 8) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 0x80;
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
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc86b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_de(0xcb47);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0x6072);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = 0x5;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0xe;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc86c);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_bc(0x0001 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0x6102);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_string(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_de({ const N: i32 = (0xcb47 as i32) + (11 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0xc86e);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.h = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function110d37(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.b = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function111f63(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = { let value = ((MOBILE_COMMAND_TRANSFER_DATA as i32) | (0x80 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc81e, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xcb47);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = 0x5;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    packet_send_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x1c;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc86a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(16);
    become function110432(cpu);
}

pub(crate) fn function110d37(cpu: &mut Cpu) {
    cpu.pc = 0x4d37;
    cpu.push_word(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.push_word(cpu.de());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.b = 0;
    cpu.pc += 2;
    cpu.cycle(8);
    become function110d37_check_under_10k(cpu);
}

pub(crate) fn function110d37_check_under_10k(cpu: &mut Cpu) {
    cpu.pc = 0x4d3b;
    cpu.a = (((10000 as u32) >> 8) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.h;
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
        become function110d37_subtract_10k(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(12);
        become function110d37_done_10k(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = ((10000 as u32) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.l;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function110d37_subtract_10k(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_c {
        cpu.cycle(12);
        become function110d37_done_10k(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become function110d37_subtract_10k(cpu);
}

pub(crate) fn function110d37_subtract_10k(cpu: &mut Cpu) {
    cpu.pc = 0x4d49;
    let value = cpu.b;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.b = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_bc(-10000i32 as u16);
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
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.cycle(12);
    become function110d37_check_under_10k(cpu);
}

pub(crate) fn function110d37_done_10k(cpu: &mut Cpu) {
    cpu.pc = 0x4d52;
    cpu.a = 0x30;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.b;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.b = 0;
    cpu.pc += 2;
    cpu.cycle(8);
    become function110d37_check_under_1k(cpu);
}

pub(crate) fn function110d37_check_under_1k(cpu: &mut Cpu) {
    cpu.pc = 0x4d59;
    cpu.a = (((1000 as u32) >> 8) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.h;
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
        become function110d37_subtract_1k(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(12);
        become function110d37_done_1k(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = ((1000 as u32) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.l;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function110d37_subtract_1k(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_c {
        cpu.cycle(12);
        become function110d37_done_1k(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become function110d37_subtract_1k(cpu);
}

pub(crate) fn function110d37_subtract_1k(cpu: &mut Cpu) {
    cpu.pc = 0x4d67;
    let value = cpu.b;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.b = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_bc(-1000i32 as u16);
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
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.cycle(12);
    become function110d37_check_under_1k(cpu);
}

pub(crate) fn function110d37_done_1k(cpu: &mut Cpu) {
    cpu.pc = 0x4d70;
    cpu.a = 0x30;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.b;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.b = 0;
    cpu.pc += 2;
    cpu.cycle(8);
    become function110d37_check_under_100(cpu);
}

pub(crate) fn function110d37_check_under_100(cpu: &mut Cpu) {
    cpu.pc = 0x4d77;
    cpu.a = (((100 as u32) >> 8) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.h;
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
        become function110d37_subtract_100(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = ((100 as u32) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.l;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function110d37_subtract_100(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_c {
        cpu.cycle(12);
        become function110d37_check_under_10(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become function110d37_subtract_100(cpu);
}

pub(crate) fn function110d37_subtract_100(cpu: &mut Cpu) {
    cpu.pc = 0x4d83;
    let value = cpu.b;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.b = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_bc(-100i32 as u16);
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
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.cycle(12);
    become function110d37_check_under_100(cpu);
}

pub(crate) fn function110d37_check_under_10(cpu: &mut Cpu) {
    cpu.pc = 0x4d8c;
    cpu.a = 0x30;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.b;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.b = 0x0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    become function110d37_subtract_10(cpu);
}

pub(crate) fn function110d37_subtract_10(cpu: &mut Cpu) {
    cpu.pc = 0x4d94;
    let value = 10;
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
        become function110d37_done_10(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 10;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.b;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.b = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.cycle(12);
    become function110d37_subtract_10(cpu);
}

pub(crate) fn function110d37_done_10(cpu: &mut Cpu) {
    cpu.pc = 0x4d9d;
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = 0x30;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.b;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 0x30;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.l;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.pop_word();
    cpu.set_de(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.l = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.h = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.b = 0x5;
    cpu.pc += 2;
    cpu.cycle(8);
    become function110d37_find_first_digit(cpu);
}

pub(crate) fn function110d37_find_first_digit(cpu: &mut Cpu) {
    cpu.pc = 0x4dac;
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0x30;
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
        become function110d37_found_first_digit(cpu);
    }
    cpu.pc += 2;
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
        become function110d37_find_first_digit(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become function110d37_done(cpu);
}

pub(crate) fn function110d37_found_first_digit(cpu: &mut Cpu) {
    cpu.pc = 0x4db7;
    cpu.a = 0x5;
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
    if cpu.flag_z {
        cpu.cycle(12);
        become function110d37_done(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.b;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte({ const N: i32 = (0xcb47 as i32) + (5 as i32); N as u16 });
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
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte({ const N: i32 = (0xcb47 as i32) + (5 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.b = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    become function110d37_penultimate_loop(cpu);
}

pub(crate) fn function110d37_penultimate_loop(cpu: &mut Cpu) {
    cpu.pc = 0x4dc9;
    let value = cpu.b;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.b = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0xd;
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
        become function110d37_penultimate_loop(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function110d37_done(cpu: &mut Cpu) {
    cpu.pc = 0x4dd5;
    let value = cpu.pop_word();
    cpu.set_bc(value);
    cpu.pc += 1;
    cpu.cycle(12);
    become function110d37_last_loop(cpu);
}

pub(crate) fn function110d37_last_loop(cpu: &mut Cpu) {
    cpu.pc = 0x4dd6;
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0xa;
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
        become function110d37_last_loop(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function110ddd(cpu: &mut Cpu) {
    cpu.pc = 0x4ddd;
    cpu.a = cpu.read_byte(0xc821);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((2) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc86a);
    cpu.pc += 3;
    cpu.cycle(16);
    if cpu.flag_z {
        cpu.cycle(12);
        become function110ddd_asm_110e00(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x13;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(16);
        become function111044(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    let value = 0x1f;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(16);
        become function111044(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    let value = 0x21;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(16);
        become function111044(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become function110226(cpu);
}

pub(crate) fn function110ddd_asm_110df9(cpu: &mut Cpu) {
    cpu.pc = 0x4df9;
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    become function110ddd_asm_110dfa(cpu);
}

pub(crate) fn function110ddd_asm_110dfa(cpu: &mut Cpu) {
    cpu.pc = 0x4dfa;
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    become function110ddd_asm_110dfd(cpu);
}

pub(crate) fn function110ddd_asm_110dfd(cpu: &mut Cpu) {
    cpu.pc = 0x4dfd;
    cpu.cycle(16);
    become function110231(cpu);
}

pub(crate) fn function110ddd_asm_110e00(cpu: &mut Cpu) {
    cpu.pc = 0x4e00;
    let value = 0x2;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(16);
        become function110226(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xc821);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((0) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(16);
        become function110226(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xc86d);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.a;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(16);
        become function110226(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc9b5, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc9b6, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc989, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xc9a5, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xc98a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xc993, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(0xc833, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(0xc834, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc97f, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc980, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.h = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = ((0xc880u32) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.l;
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
        become function110ddd_asm_110e4a(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = (((0xc880u32) >> 8) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.h;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function110ddd_asm_110dfd(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become function110ddd_asm_110e4a(cpu);
}

pub(crate) fn function110ddd_asm_110e4a(cpu: &mut Cpu) {
    cpu.pc = 0x4e4a;
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.push_word(cpu.de());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.push_word(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.b = { let value = ((0x4fb4 as i32) - (0x4fad as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_de(0x4fad);
    cpu.pc += 3;
    cpu.cycle(12);
    become function110ddd_asm_110e53(cpu);
}

pub(crate) fn function110ddd_asm_110e53(cpu: &mut Cpu) {
    cpu.pc = 0x4e53;
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
        become function110ddd_asm_110df9(cpu);
    }
    cpu.pc += 2;
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
        become function110ddd_asm_110e53(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.b = { let value = ((0x4ffc as i32) - (0x4fd9 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.c = 0x0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_de(0x4fd9);
    cpu.pc += 3;
    cpu.cycle(12);
    become function110ddd_asm_110e64(cpu);
}

pub(crate) fn function110ddd_asm_110e64(cpu: &mut Cpu) {
    cpu.pc = 0x4e64;
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
        become function110ddd_asm_110e70(cpu);
    }
    cpu.pc += 2;
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
        become function110ddd_asm_110e64(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.cycle(12);
    become function110ddd_asm_110df9(cpu);
}

pub(crate) fn function110ddd_asm_110e70(cpu: &mut Cpu) {
    cpu.pc = 0x4e70;
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.b = { let value = ((0x5044 as i32) - (0x5020 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.c = 0x0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_de(0x5020);
    cpu.pc += 3;
    cpu.cycle(12);
    become function110ddd_asm_110e79(cpu);
}

pub(crate) fn function110ddd_asm_110e79(cpu: &mut Cpu) {
    cpu.pc = 0x4e79;
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
        become function110ddd_asm_110e86(cpu);
    }
    cpu.pc += 2;
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
        become function110ddd_asm_110e79(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.cycle(16);
    become function110ddd_asm_110df9(cpu);
}

pub(crate) fn function110ddd_asm_110e86(cpu: &mut Cpu) {
    cpu.pc = 0x4e86;
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.b = { let value = ((0x5020 as i32) - (0x4ffc as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.c = 0x0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_de(0x4ffc);
    cpu.pc += 3;
    cpu.cycle(12);
    become function110ddd_asm_110e8f(cpu);
}

pub(crate) fn function110ddd_asm_110e8f(cpu: &mut Cpu) {
    cpu.pc = 0x4e8f;
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
        become function110ddd_asm_110ea2(cpu);
    }
    cpu.pc += 2;
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
        become function110ddd_asm_110e8f(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc98a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.c = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become function110ddd_asm_110eb3(cpu);
}

pub(crate) fn function110ddd_asm_110ea2(cpu: &mut Cpu) {
    cpu.pc = 0x4ea2;
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.b = { let value = ((0x4fd9 as i32) - (0x4fb4 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.c = 0x0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_de(0x4fb4);
    cpu.pc += 3;
    cpu.cycle(12);
    become function110ddd_asm_110eaa(cpu);
}

pub(crate) fn function110ddd_asm_110eaa(cpu: &mut Cpu) {
    cpu.pc = 0x4eaa;
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
        become function110ddd_asm_110ecb(cpu);
    }
    cpu.pc += 2;
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
        become function110ddd_asm_110eaa(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become function110ddd_asm_110eb3(cpu);
}

pub(crate) fn function110ddd_asm_110eb3(cpu: &mut Cpu) {
    cpu.pc = 0x4eb3;
    cpu.set_hl(0xc97f);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.h = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.c = 0x12;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11039a(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.cycle(16);
        become function110ddd_asm_110df9(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.c = 0x12;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11039a(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.cycle(16);
        become function110ddd_asm_110df9(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.c = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    become function110ddd_asm_110ecb(cpu);
}

pub(crate) fn function110ddd_asm_110ecb(cpu: &mut Cpu) {
    cpu.pc = 0x4ecb;
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc98f, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte({ const N: i32 = (0xca3c as i32) + (128 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function1111d7(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = 0x4;
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
        become function110ddd_asm_110ee3(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(16);
        become function110ddd_asm_110dfa(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
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
        cpu.cycle(16);
        become function110ddd_asm_110dfa(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    become function110ddd_asm_110ee3(cpu);
}

pub(crate) fn function110ddd_asm_110ee3(cpu: &mut Cpu) {
    cpu.pc = 0x4ee3;
    cpu.set_hl(0xc98b);
    cpu.pc += 3;
    cpu.cycle(12);
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
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.pop_word();
    cpu.set_bc(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let value = cpu.pop_word();
    cpu.set_de(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.a = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc876, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc877, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xc872);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
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
    cpu.write_byte(0xc994, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    become function110f07(cpu);
}

pub(crate) fn function110f07(cpu: &mut Cpu) {
    cpu.pc = 0x4f07;
    cpu.set_hl(0xc833);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.h = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.h;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function110f07_asm_110f12(cpu);
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
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    become function110f07_asm_110f12(cpu);
}

pub(crate) fn function110f07_asm_110f12(cpu: &mut Cpu) {
    cpu.pc = 0x4f12;
    cpu.set_hl(0xc991);
    cpu.pc += 3;
    cpu.cycle(12);
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
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(0xc866);
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
    if !cpu.flag_z {
        cpu.cycle(12);
        become function110f07_asm_110f28(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x2;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become function110615(cpu);
}

pub(crate) fn function110f07_asm_110f28(cpu: &mut Cpu) {
    cpu.pc = 0x4f28;
    cpu.a = 0x2;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc86e, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x1f;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte({ const N: i32 = (0xcb47 as i32) + (10 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x90;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte({ const N: i32 = (0xcb47 as i32) + (11 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc86b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_de(0xcb47);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0x607d);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = 0x6;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xc866);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = 0x4;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.b = 0x6;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function111f63(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte({ const N: i32 = (0xca3c as i32) + (128 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.a;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function110f07_asm_110f95(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xc995);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0x99;
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
        become function110f07_asm_110f8a(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0x66;
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
        become function110f07_asm_110f8a(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0x23;
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
        become function110f07_asm_110f8a(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x2;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc86e, cpu.a);
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
    cpu.write_byte(0xc86b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0xa3;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_de(0x0010 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0xc995);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function111f02(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0xf;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc86a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(16);
    become function110432(cpu);
}

pub(crate) fn function110f07_asm_110f8a(cpu: &mut Cpu) {
    cpu.pc = 0x4f8a;
    cpu.set_hl(0xcb47);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xc995);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = 0x10;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    become function110f07_asm_110f95(cpu);
}

pub(crate) fn function110f07_asm_110f95(cpu: &mut Cpu) {
    cpu.pc = 0x4f95;
    cpu.set_de(0x0010 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0xcb47);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = { let value = ((MOBILE_COMMAND_OPEN_TCP_CONNECTION as i32) | (0x80 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc81e, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.b = 0x5;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    packet_send_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0xf;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc86a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(16);
    become function110432(cpu);
}

pub(crate) fn http_ranking_url_end(cpu: &mut Cpu) {
    cpu.pc = 0x5044;
    become function111044(cpu);
}

pub(crate) fn function111044(cpu: &mut Cpu) {
    cpu.pc = 0x5044;
    cpu.set_hl(0xc827);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.e = cpu.read_byte(cpu.hl());
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
    cpu.write_byte(0xc86e, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte({ const N: i32 = (0xc86e as i32) + (1 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_bc(cpu.bc().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_bc(cpu.bc().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(16);
        become function1111ca(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xc991);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.a;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        let next_pc = cpu.pc + 3;
        cpu.push_word(next_pc);
        cpu.cycle(24);
        function11115f(cpu);
        assert_eq!(cpu.pc, next_pc);
    } else {
        cpu.pc += 3;
        cpu.cycle(12);
    }
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.e;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(16);
        become function111044_asm_1110eb(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
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
        cpu.cycle(12);
        become function111044_asm_1110ac(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.c;
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
        become function111044_asm_1110ac(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.push_word(cpu.bc());
    cpu.pc += 1;
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
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.b = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl(0xc82d);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
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
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl({ const N: i32 = (0xca3c as i32) + (3 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.e;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
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
    cpu.a = cpu.read_byte(0xc829);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte({ const N: i32 = (0xc829 as i32) + (1 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.d = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_bc(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xc991);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.h = 0;
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
    cpu.c = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.b = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc991, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xc827);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.h = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.b);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function111044_asm_1110ac(cpu: &mut Cpu) {
    cpu.pc = 0x50ac;
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.e;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = 0x0;
    let carry_in = u8::from(cpu.flag_c);
    let lhs = cpu.a;
    let (partial, borrow1) = lhs.overflowing_sub(value);
    let (result, borrow2) = partial.overflowing_sub(carry_in);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < ((value & 0x0f) + carry_in);
    cpu.flag_c = borrow1 || borrow2;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.b);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(0xc82d);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xc991);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.e;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_add(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) > 0x0f;
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 0;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0;
    let carry_in = u8::from(cpu.flag_c);
    let lhs = cpu.a;
    let (partial, carry1) = lhs.overflowing_add(value);
    let (result, carry2) = partial.overflowing_add(carry_in);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) + carry_in > 0x0f;
    cpu.flag_c = carry1 || carry2;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc991, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xc86e);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.a;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function111044_asm_1110eb(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.b = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl({ const N: i32 = (0xca3c as i32) + (3 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.e;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
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
    cpu.a = cpu.read_byte(0xc829);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte({ const N: i32 = (0xc829 as i32) + (1 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.d = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xc829);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    become function111044_asm_1110eb(cpu);
}

pub(crate) fn function111044_asm_1110eb(cpu: &mut Cpu) {
    cpu.pc = 0x50eb;
    cpu.disable_interrupts_after_next_instruction();
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = 0x2;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc989, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xc821);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((2) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xca3c);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = { let value = ((MOBILE_COMMAND_TRANSFER_DATA_END as i32) | (0x80 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
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
        become function111044_asm_111144(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_de(0x000b as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = { let value = ((MOBILE_COMMAND_TRANSFER_DATA as i32) | (0x80 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc81e, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xcb47);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = 0x5;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    packet_send_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc86b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
    return;
    cpu.set_hl(0xc827);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.h = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xc82d);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc82e);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(0xc98f);
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
    cpu.a = 0xf;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc86a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc86b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xc86d);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xc86e, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc989, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0xa3;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_de(0x0010 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0xc995);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become function111f02(cpu);
}

pub(crate) fn function111044_asm_111144(cpu: &mut Cpu) {
    cpu.pc = 0x5144;
    let mask = 1u8 << ((0) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.set_hl(0xc827);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.h = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xc82d);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc82e);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 0x2;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc86a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc86d, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.enable_interrupts_after_next_instruction();
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function11115f(cpu: &mut Cpu) {
    cpu.pc = 0x515f;
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
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
        cpu.cycle(12);
        become function11115f_asm_111168(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
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
        become function11115f_asm_1111a2(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become function11115f_asm_111168(cpu);
}

pub(crate) fn function11115f_asm_111168(cpu: &mut Cpu) {
    cpu.pc = 0x5168;
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.push_word(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.b = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.c = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xc993);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.e;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.d = 0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xc880);
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
    cpu.a = cpu.read_byte(0xc829);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte({ const N: i32 = (0xc829 as i32) + (1 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.d = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xc829);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.e = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl(0xc82d);
    cpu.pc += 3;
    cpu.cycle(12);
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
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.pop_word();
    cpu.set_bc(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.e;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = 0x0;
    let carry_in = u8::from(cpu.flag_c);
    let lhs = cpu.a;
    let (partial, borrow1) = lhs.overflowing_sub(value);
    let (result, borrow2) = partial.overflowing_sub(carry_in);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < ((value & 0x0f) + carry_in);
    cpu.flag_c = borrow1 || borrow2;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xc992);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xc82b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function11115f_asm_1111a2(cpu: &mut Cpu) {
    cpu.pc = 0x51a2;
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
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
    cpu.write_byte(0xc991, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xc993);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.e;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.d = 0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xc880);
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
    cpu.a = cpu.read_byte(0xc829);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte({ const N: i32 = (0xc829 as i32) + (1 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.d = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.b = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xc827);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.h = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
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
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function1111ca(cpu: &mut Cpu) {
    cpu.pc = 0x51ca;
    cpu.set_hl(0xc821);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((2) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.a = 0x6;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc86b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(16);
    become function112430(cpu);
}

pub(crate) fn function1111d7(cpu: &mut Cpu) {
    cpu.pc = 0x51d7;
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.set_hl(0xc866);
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
    if !cpu.flag_z {
        cpu.cycle(12);
        become function1111d7_asm_1111ee(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_de(0x0007 as u16);
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
    become function1111d7_asm_1111e8(cpu);
}

pub(crate) fn function1111d7_asm_1111e8(cpu: &mut Cpu) {
    cpu.pc = 0x51e8;
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0x2f;
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
        become function1111d7_asm_1111e8(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    become function1111d7_asm_1111ee(cpu);
}

pub(crate) fn function1111d7_asm_1111ee(cpu: &mut Cpu) {
    cpu.pc = 0x51ee;
    cpu.set_bc(-1i32 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    become function1111d7_asm_1111f1(cpu);
}

pub(crate) fn function1111d7_asm_1111f1(cpu: &mut Cpu) {
    cpu.pc = 0x51f1;
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_bc(cpu.bc().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.a;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(12);
        become function1111d7_asm_1111f1(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xc87a);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function1111fe(cpu: &mut Cpu) {
    cpu.pc = 0x51fe;
    cpu.a = cpu.read_byte(0xc821);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((2) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc86a);
    cpu.pc += 3;
    cpu.cycle(16);
    if !cpu.flag_z {
        cpu.cycle(16);
        become function1113ea(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    let value = 0x2;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(16);
        become function110226(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xc821);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((0) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(16);
        become function110226(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xc86d);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.a;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(16);
        become function110226(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc989, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xc98a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xc993, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.push_word(cpu.de());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.push_word(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(0xc833, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(0xc834, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc97f, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc980, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.h = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = ((0xc880u32) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.l;
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
        become function1111fe_asm_111251(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = (((0xc880u32) >> 8) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.h;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(16);
        become function1113f7(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    become function1111fe_asm_111251(cpu);
}

pub(crate) fn function1111fe_asm_111251(cpu: &mut Cpu) {
    cpu.pc = 0x5251;
    cpu.b = { let value = ((0x4fb4 as i32) - (0x4fad as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_de(0x4fad);
    cpu.pc += 3;
    cpu.cycle(12);
    become function1111fe_asm_111256(cpu);
}

pub(crate) fn function1111fe_asm_111256(cpu: &mut Cpu) {
    cpu.pc = 0x5256;
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
        cpu.cycle(16);
        become function1113f7(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
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
        become function1111fe_asm_111256(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.b = { let value = ((0x4fd9 as i32) - (0x4fb4 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.c = 0x0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_de(0x4fb4);
    cpu.pc += 3;
    cpu.cycle(12);
    become function1111fe_asm_111268(cpu);
}

pub(crate) fn function1111fe_asm_111268(cpu: &mut Cpu) {
    cpu.pc = 0x5268;
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
        become function1111fe_asm_111275(cpu);
    }
    cpu.pc += 2;
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
        become function1111fe_asm_111268(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.cycle(16);
    become function1113f7(cpu);
}

pub(crate) fn function1111fe_asm_111275(cpu: &mut Cpu) {
    cpu.pc = 0x5275;
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.b = { let value = ((0x5044 as i32) - (0x5020 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.c = 0x0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_de(0x5020);
    cpu.pc += 3;
    cpu.cycle(12);
    become function1111fe_asm_11127e(cpu);
}

pub(crate) fn function1111fe_asm_11127e(cpu: &mut Cpu) {
    cpu.pc = 0x527e;
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
        become function1111fe_asm_11128f(cpu);
    }
    cpu.pc += 2;
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
        become function1111fe_asm_11127e(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x2;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc98a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.cycle(12);
    become function1111fe_asm_1112a0(cpu);
}

pub(crate) fn function1111fe_asm_11128f(cpu: &mut Cpu) {
    cpu.pc = 0x528f;
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.b = { let value = ((0x4ffc as i32) - (0x4fd9 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.c = 0x0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_de(0x4fd9);
    cpu.pc += 3;
    cpu.cycle(12);
    become function1111fe_asm_111297(cpu);
}

pub(crate) fn function1111fe_asm_111297(cpu: &mut Cpu) {
    cpu.pc = 0x5297;
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
        become function1111fe_asm_1112cc(cpu);
    }
    cpu.pc += 2;
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
        become function1111fe_asm_111297(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become function1111fe_asm_1112a0(cpu);
}

pub(crate) fn function1111fe_asm_1112a0(cpu: &mut Cpu) {
    cpu.pc = 0x52a0;
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.a;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(12);
        become function1111fe_asm_1112a0(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become function1111fe_asm_1112a4(cpu);
}

pub(crate) fn function1111fe_asm_1112a4(cpu: &mut Cpu) {
    cpu.pc = 0x52a4;
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0x2f;
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
        become function1111fe_asm_1112a4(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0x30;
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
        become function1111fe_asm_1112cc(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x3a;
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
        become function1111fe_asm_1112cc(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xc97f);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.h = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.c = 0x12;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11039a(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.cycle(16);
        become function1113f7(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.c = 0x12;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11039a(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.cycle(16);
        become function1113f7(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.c = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    become function1111fe_asm_1112cc(cpu);
}

pub(crate) fn function1111fe_asm_1112cc(cpu: &mut Cpu) {
    cpu.pc = 0x52cc;
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc98f, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte({ const N: i32 = (0xca3c as i32) + (128 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.set_de(0x0006 as u16);
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
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.h = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function1111d7(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = 0x4;
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
        become function1111fe_asm_1112eb(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(16);
        become function1113f8(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
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
        cpu.cycle(16);
        become function1113f8(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    become function1111fe_asm_1112eb(cpu);
}

pub(crate) fn function1111fe_asm_1112eb(cpu: &mut Cpu) {
    cpu.pc = 0x52eb;
    let value = cpu.pop_word();
    cpu.set_bc(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let value = cpu.pop_word();
    cpu.set_de(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.a = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc876, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc877, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xc872);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function111335(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xc876);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.h = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(0xc9aa, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(0xc9ab, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(0xc9ac, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(0xc9ad, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(0xc876, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(0xc877, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xc98f);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x1;
    cpu.a ^= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc994, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(16);
    become function110f07(cpu);
}

pub(crate) fn function111335(cpu: &mut Cpu) {
    cpu.pc = 0x5335;
    cpu.set_hl(0xc876);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.h = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.h = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc8c9, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    become function111335_asm_111344(cpu);
}

pub(crate) fn function111335_asm_111344(cpu: &mut Cpu) {
    cpu.pc = 0x5344;
    cpu.set_de(0x8ad0 as u16);
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
    if !cpu.flag_c {
        cpu.cycle(12);
        become function111335_asm_11134e(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x3;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_add(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) > 0x0f;
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become function111335_asm_111344(cpu);
}

pub(crate) fn function111335_asm_11134e(cpu: &mut Cpu) {
    cpu.pc = 0x534e;
    cpu.set_de(0x7530 as u16);
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
    become function111335_asm_111352(cpu);
}

pub(crate) fn function111335_asm_111352(cpu: &mut Cpu) {
    cpu.pc = 0x5352;
    cpu.set_de(0xd8f0 as u16);
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
    if !cpu.flag_c {
        cpu.cycle(12);
        become function111335_asm_11135b(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.cycle(12);
    become function111335_asm_111352(cpu);
}

pub(crate) fn function111335_asm_11135b(cpu: &mut Cpu) {
    cpu.pc = 0x535b;
    cpu.set_de(0x2710 as u16);
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
    cpu.write_byte(0xc8c6, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    become function111335_asm_111363(cpu);
}

pub(crate) fn function111335_asm_111363(cpu: &mut Cpu) {
    cpu.pc = 0x5363;
    cpu.set_de(0xf448 as u16);
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
    if !cpu.flag_c {
        cpu.cycle(12);
        become function111335_asm_11136d(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x30;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_add(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) > 0x0f;
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become function111335_asm_111363(cpu);
}

pub(crate) fn function111335_asm_11136d(cpu: &mut Cpu) {
    cpu.pc = 0x536d;
    cpu.set_de(0x0bb8 as u16);
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
    become function111335_asm_111371(cpu);
}

pub(crate) fn function111335_asm_111371(cpu: &mut Cpu) {
    cpu.pc = 0x5371;
    cpu.set_de(0xfc18 as u16);
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
    if !cpu.flag_c {
        cpu.cycle(12);
        become function111335_asm_11137b(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x10;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_add(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) > 0x0f;
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become function111335_asm_111371(cpu);
}

pub(crate) fn function111335_asm_11137b(cpu: &mut Cpu) {
    cpu.pc = 0x537b;
    cpu.set_de(0x03e8 as u16);
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
    become function111335_asm_11137f(cpu);
}

pub(crate) fn function111335_asm_11137f(cpu: &mut Cpu) {
    cpu.pc = 0x537f;
    cpu.set_de(0xfed4 as u16);
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
    if !cpu.flag_c {
        cpu.cycle(12);
        become function111335_asm_111389(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x3;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_add(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) > 0x0f;
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become function111335_asm_11137f(cpu);
}

pub(crate) fn function111335_asm_111389(cpu: &mut Cpu) {
    cpu.pc = 0x5389;
    cpu.set_de(0x012c as u16);
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
    become function111335_asm_11138d(cpu);
}

pub(crate) fn function111335_asm_11138d(cpu: &mut Cpu) {
    cpu.pc = 0x538d;
    cpu.set_de(0xff9c as u16);
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
    if !cpu.flag_c {
        cpu.cycle(12);
        become function111335_asm_111396(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.cycle(12);
    become function111335_asm_11138d(cpu);
}

pub(crate) fn function111335_asm_111396(cpu: &mut Cpu) {
    cpu.pc = 0x5396;
    cpu.set_de(0x0064 as u16);
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
    cpu.write_byte(0xc8c7, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    become function111335_asm_11139e(cpu);
}

pub(crate) fn function111335_asm_11139e(cpu: &mut Cpu) {
    cpu.pc = 0x539e;
    cpu.set_de(0xffe2 as u16);
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
    if !cpu.flag_c {
        cpu.cycle(12);
        become function111335_asm_1113a8(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x30;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_add(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) > 0x0f;
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become function111335_asm_11139e(cpu);
}

pub(crate) fn function111335_asm_1113a8(cpu: &mut Cpu) {
    cpu.pc = 0x53a8;
    cpu.set_de(0x001e as u16);
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
    become function111335_asm_1113ac(cpu);
}

pub(crate) fn function111335_asm_1113ac(cpu: &mut Cpu) {
    cpu.pc = 0x53ac;
    cpu.set_de(0xfff6 as u16);
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
    if !cpu.flag_c {
        cpu.cycle(12);
        become function111335_asm_1113b6(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x10;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_add(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) > 0x0f;
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become function111335_asm_1113ac(cpu);
}

pub(crate) fn function111335_asm_1113b6(cpu: &mut Cpu) {
    cpu.pc = 0x53b6;
    cpu.set_de(0x000a as u16);
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
    let value = cpu.l;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_add(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) > 0x0f;
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc8c8, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_de(0xc9a5);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0xc8c6);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0x30;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 2;
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
    let value = cpu.a;
    let result = value.rotate_left(4);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0xf;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x30;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0xf;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x30;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 2;
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
    let value = cpu.a;
    let result = value.rotate_left(4);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0xf;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x30;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 2;
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
    let value = 0xf;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x30;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function1113ea(cpu: &mut Cpu) {
    cpu.pc = 0x53ea;
    let value = 0x14;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(16);
        become function111044(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    let value = 0x24;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(16);
        become function111044(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become function110226(cpu);
}

pub(crate) fn function1113f7(cpu: &mut Cpu) {
    cpu.pc = 0x53f7;
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    become function1113f8(cpu);
}

pub(crate) fn function1113f8(cpu: &mut Cpu) {
    cpu.pc = 0x53f8;
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.cycle(16);
    become function110231(cpu);
}

pub(crate) fn function1113fe(cpu: &mut Cpu) {
    cpu.pc = 0x53fe;
    cpu.a = cpu.read_byte(0xc822);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((4) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(16);
        become function1113fe_asm_11147f(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((7) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(16);
        become function1113fe_asm_11147f(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xc821);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((0) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(16);
        become function1113fe_asm_11147f(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    become function1113fe_asm_111413(cpu);
}

pub(crate) fn function1113fe_asm_111413(cpu: &mut Cpu) {
    cpu.pc = 0x5413;
    cpu.a = cpu.read_byte(0xc800);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.a;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(12);
        become function1113fe_asm_111413(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.disable_interrupts_after_next_instruction();
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xc821);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((3) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(16);
        become function1113fe_asm_11147b(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xc807);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.a;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(12);
        become function1113fe_asm_111436(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xc821);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((1) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value |= mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.a = 0x23;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc80f, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0xff;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.enable_interrupts_after_next_instruction();
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function1113fe_asm_111436(cpu: &mut Cpu) {
    cpu.pc = 0x5436;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc86b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.set_hl(0xc829);
    cpu.pc += 3;
    cpu.cycle(12);
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
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(0xcb47);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0x6072);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = 0x5;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.a;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function1113fe_asm_111485(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x81;
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
        become function1113fe_asm_111485(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.a;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.a;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 0xff;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.b = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.b = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.b;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.b = result;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.b;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.b = result;
    cpu.pc += 1;
    cpu.cycle(4);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function111f63(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xc822);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((7) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value |= mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.set_hl(0xc821);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((0) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value |= mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.a = 0x0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.enable_interrupts_after_next_instruction();
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function1113fe_asm_11147b(cpu: &mut Cpu) {
    cpu.pc = 0x547b;
    cpu.enable_interrupts_after_next_instruction();
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function1113fe_asm_11147f(cpu: &mut Cpu) {
    cpu.pc = 0x547f;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function110226(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0xff;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function1113fe_asm_111485(cpu: &mut Cpu) {
    cpu.pc = 0x5485;
    cpu.enable_interrupts_after_next_instruction();
    cpu.pc += 1;
    cpu.cycle(4);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function110231(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0xff;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function11148c(cpu: &mut Cpu) {
    cpu.pc = 0x548c;
    cpu.a = cpu.read_byte(0xc822);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((4) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(16);
        become function110226(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xc821);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((0) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(16);
        become function110226(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((3) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(16);
        become function110226(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.e = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.d = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xc992);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.a;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(12);
        become function11148c_asm_111507(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc993);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.b = 0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl({ const N: i32 = (0xca3c as i32) + (4 as i32); N as u16 });
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
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.a;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function11148c_asm_1114bb(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x81;
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
        become function11148c_asm_1114bd(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become function11148c_asm_1114bb(cpu);
}

pub(crate) fn function11148c_asm_1114bb(cpu: &mut Cpu) {
    cpu.pc = 0x54bb;
    cpu.a = 0x80;
    cpu.pc += 2;
    cpu.cycle(8);
    become function11148c_asm_1114bd(cpu);
}

pub(crate) fn function11148c_asm_1114bd(cpu: &mut Cpu) {
    cpu.pc = 0x54bd;
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.c;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.c = result;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.c;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_add(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) > 0x0f;
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc993, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xc994);
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
    let value = cpu.b;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc994, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    become function11148c_asm_1114d2(cpu);
}

pub(crate) fn function11148c_asm_1114d2(cpu: &mut Cpu) {
    cpu.pc = 0x54d2;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
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
        become function11148c_asm_1114dc(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xc821);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((3) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function11148c_asm_1114dc(cpu: &mut Cpu) {
    cpu.pc = 0x54dc;
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.a;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function11148c_asm_1114e4(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x81;
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
        become function11148c_asm_1114e6(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become function11148c_asm_1114e4(cpu);
}

pub(crate) fn function11148c_asm_1114e4(cpu: &mut Cpu) {
    cpu.pc = 0x54e4;
    cpu.a = 0x80;
    cpu.pc += 2;
    cpu.cycle(8);
    become function11148c_asm_1114e6(cpu);
}

pub(crate) fn function11148c_asm_1114e6(cpu: &mut Cpu) {
    cpu.pc = 0x54e6;
    let value = cpu.c;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_c {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(0xc991, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.c;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.c = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.a;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function11148c_asm_111500(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc992, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_de(0xc880);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    become function11148c_asm_1114fa(cpu);
}

pub(crate) fn function11148c_asm_1114fa(cpu: &mut Cpu) {
    cpu.pc = 0x54fa;
    cpu.set_hl(0xc821);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((3) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function11148c_asm_111500(cpu: &mut Cpu) {
    cpu.pc = 0x5500;
    cpu.a = 0xff;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc992, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(12);
    become function11148c_asm_1114fa(cpu);
}

pub(crate) fn function11148c_asm_111507(cpu: &mut Cpu) {
    cpu.pc = 0x5507;
    let value = 0xff;
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
        become function11148c_asm_11150c(cpu);
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
    become function11148c_asm_11150c(cpu);
}

pub(crate) fn function11148c_asm_11150c(cpu: &mut Cpu) {
    cpu.pc = 0x550c;
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xc991);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.b;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl(0xc880);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xc991);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.a;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function11148c_asm_111521(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    become function11148c_asm_111521(cpu);
}

pub(crate) fn function11148c_asm_111521(cpu: &mut Cpu) {
    cpu.pc = 0x5521;
    cpu.set_hl({ const N: i32 = (0xca3c as i32) + (5 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.a;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc993, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte({ const N: i32 = (0xca3c as i32) + (3 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.b;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc994, cpu.a);
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
    cpu.set_hl(0xc991);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.cycle(12);
    become function11148c_asm_1114d2(cpu);
}

pub(crate) fn function111540(cpu: &mut Cpu) {
    cpu.pc = 0x5540;
    cpu.pc += 1;
    cpu.cycle(4);
    become mobile_api_telephone_status(cpu);
}

pub(crate) fn mobile_api_telephone_status(cpu: &mut Cpu) {
    cpu.pc = 0x5541;
    cpu.set_hl(0xc821);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((0) & 7);
    let value = cpu.read_byte(cpu.hl());
    cpu.flag_z = (value & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(12);
    if !cpu.flag_z {
        cpu.cycle(16);
        become function110226(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xc86a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x5;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_c {
        cpu.cycle(16);
        become function110226(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.write_byte(0xc985, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc86e, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte({ const N: i32 = (0xc86e as i32) + (1 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xc807);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x2;
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
        become mobile_api_telephone_status_asm_111582(cpu);
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
    cpu.write_byte(0xc86b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = { let value = ((MOBILE_COMMAND_TELEPHONE_STATUS as i32) | (0x80 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x602d);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    packet_send_empty_body(cpu);
    assert_eq!(cpu.pc, next_pc);
    become mobile_api_telephone_status_asm_11156f(cpu);
}

pub(crate) fn mobile_api_telephone_status_asm_11156f(cpu: &mut Cpu) {
    cpu.pc = 0x556f;
    cpu.a = cpu.read_byte(0xc988);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x40;
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
        become mobile_api_telephone_status_asm_11157a(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x2c;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become mobile_api_telephone_status_asm_11157c(cpu);
}

pub(crate) fn mobile_api_telephone_status_asm_11157a(cpu: &mut Cpu) {
    cpu.pc = 0x557a;
    cpu.a = 0x1e;
    cpu.pc += 2;
    cpu.cycle(8);
    become mobile_api_telephone_status_asm_11157c(cpu);
}

pub(crate) fn mobile_api_telephone_status_asm_11157c(cpu: &mut Cpu) {
    cpu.pc = 0x557c;
    cpu.write_byte(0xc86a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(16);
    become function110432(cpu);
}

pub(crate) fn mobile_api_telephone_status_asm_111582(cpu: &mut Cpu) {
    cpu.pc = 0x5582;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(crate::defs::R_TAC as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xc870);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_api_set_timer(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function1104b0(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc86b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(12);
    become mobile_api_telephone_status_asm_11156f(cpu);
}

pub(crate) fn function111596(cpu: &mut Cpu) {
    cpu.pc = 0x5596;
    cpu.set_hl(0xc86a);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0x1;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(16);
        become function110226(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    let value = 0x2a;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(16);
        become function110226(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xc800);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((1) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(12);
        become function111596_asm_1115af(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x2a;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become function1115e4(cpu);
}

pub(crate) fn function111596_asm_1115af(cpu: &mut Cpu) {
    cpu.pc = 0x55af;
    cpu.a = cpu.read_byte(0xc81e);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x92;
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
        become function111596_asm_1115dd(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x2a;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.b = 0x0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.disable_interrupts_after_next_instruction();
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.b);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(0xc822);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((5) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    let mask = 1u8 << ((0) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc80b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xc800, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc807, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    reset_receive_packet_buffer(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11164f(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xc821);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((0) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value |= mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.enable_interrupts_after_next_instruction();
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function111596_asm_1115dd(cpu: &mut Cpu) {
    cpu.pc = 0x55dd;
    cpu.a = 0x2a;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function1115e4(cpu: &mut Cpu) {
    cpu.pc = 0x55e4;
    cpu.disable_interrupts_after_next_instruction();
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.set_hl(0xc821);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((0) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value |= mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc86b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xc86d);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.a;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xca3c);
    cpu.pc += 3;
    cpu.cycle(16);
    if cpu.flag_z {
        cpu.cycle(12);
        become function1115e4_asm_111609(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = { let value = ((MOBILE_COMMAND_TRANSFER_DATA_END as i32) | (0x80 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
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
        become function1115e4_asm_11160d(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = { let value = ((MOBILE_COMMAND_CLOSE_TCP_CONNECTION as i32) | (0x80 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
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
        become function1115e4_asm_11160d(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become function1115e4_asm_111601(cpu);
}

pub(crate) fn function1115e4_asm_111601(cpu: &mut Cpu) {
    cpu.pc = 0x5601;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function112430(cpu);
    assert_eq!(cpu.pc, next_pc);
    become function1115e4_asm_111604(cpu);
}

pub(crate) fn function1115e4_asm_111604(cpu: &mut Cpu) {
    cpu.pc = 0x5604;
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte(0xc86a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function1115e4_asm_111609(cpu: &mut Cpu) {
    cpu.pc = 0x5609;
    let value = 0xa3;
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
        become function1115e4_asm_111601(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become function1115e4_asm_11160d(cpu);
}

pub(crate) fn function1115e4_asm_11160d(cpu: &mut Cpu) {
    cpu.pc = 0x560d;
    cpu.enable_interrupts_after_next_instruction();
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.cycle(12);
    become function1115e4_asm_111604(cpu);
}

pub(crate) fn function111610(cpu: &mut Cpu) {
    cpu.pc = 0x5610;
    cpu.set_hl(0xc86a);
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
    if cpu.flag_z {
        cpu.cycle(16);
        become function110226(cpu);
    }
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
    if cpu.flag_z {
        cpu.cycle(16);
        become function110226(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xc800);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.a;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(12);
        become function111610_asm_111626(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x28;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become function1115e4(cpu);
}

pub(crate) fn function111610_asm_111626(cpu: &mut Cpu) {
    cpu.pc = 0x5626;
    cpu.a = 0x28;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.b = 0x2;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.b);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function11162d(cpu: &mut Cpu) {
    cpu.pc = 0x562d;
    cpu.a = cpu.read_byte(0xc86a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x1;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(16);
        become function110226(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl(0xcb47);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function111686(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    reset_receive_packet_buffer(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_bc(0x0452 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0xc800);
    cpu.pc += 3;
    cpu.cycle(12);
    become function11162d_asm_111647(cpu);
}

pub(crate) fn function11162d_asm_111647(cpu: &mut Cpu) {
    cpu.pc = 0x5647;
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
    cpu.set_bc(cpu.bc().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.b;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(12);
        become function11162d_asm_111647(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function11164f(cpu: &mut Cpu) {
    cpu.pc = 0x564f;
    cpu.set_hl(0xc815);
    cpu.pc += 3;
    cpu.cycle(12);
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
    cpu.a = cpu.read_byte(0xc81f);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xc818);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.a;
    let result = value >> 1;
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x01 != 0;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value >> 1;
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x01 != 0;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
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
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function111664(cpu: &mut Cpu) {
    cpu.pc = 0x5664;
    cpu.set_hl(0xca3a);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.d = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xc81e);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0xff;
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
        become function111664_asm_111679(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc822);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((0) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become function111664_asm_11167c(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become function111664_asm_111679(cpu);
}

pub(crate) fn function111664_asm_111679(cpu: &mut Cpu) {
    cpu.pc = 0x5679;
    cpu.set_hl(0xca2f);
    cpu.pc += 3;
    cpu.cycle(12);
    become function111664_asm_11167c(cpu);
}

pub(crate) fn function111664_asm_11167c(cpu: &mut Cpu) {
    cpu.pc = 0x567c;
    let lhs = cpu.hl();
    let rhs = cpu.de();
    let (result, carry) = lhs.overflowing_add(rhs);
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0fff) + (rhs & 0x0fff) > 0x0fff;
    cpu.flag_c = carry;
    cpu.set_hl(result);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.c);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(0xca3a);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.d);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function111686(cpu: &mut Cpu) {
    cpu.pc = 0x5686;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(crate::defs::R_TAC as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.c = ((crate::defs::R_IE as u32) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xff00u16 + u16::from(cpu.c));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = !({ let value = (((crate::defs::IE_SERIAL as i32) | (crate::defs::IE_TIMER as i32))); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } });
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xff00u16 + u16::from(cpu.c), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte({ const N: i32 = (0xcb47 as i32) + (1 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xc86a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xcb47);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl(0xc821);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.c;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function1116a0(cpu: &mut Cpu) {
    cpu.pc = 0x56a0;
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become function1116a9(cpu);
}

pub(crate) fn function1116a4(cpu: &mut Cpu) {
    cpu.pc = 0x56a4;
    let mask = 1u8 << ((1) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value |= mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xc86a);
    cpu.pc += 3;
    cpu.cycle(16);
    become function1116a9(cpu);
}

pub(crate) fn function1116a9(cpu: &mut Cpu) {
    cpu.pc = 0x56a9;
    cpu.write_byte({ const N: i32 = (0xcb47 as i32) + (1 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xc815);
    cpu.pc += 3;
    cpu.cycle(12);
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
    cpu.a = cpu.read_byte(0xc81f);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.a;
    let carry_in = u8::from(cpu.flag_c);
    let result = (value << 1) | carry_in;
    cpu.flag_z = false;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x80 != 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(0xc821);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = 0xd;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 0x2;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.b;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcb47, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub fn _mobile_receive(cpu: &mut Cpu) {
    cpu.pc = 0x56c5;
    cpu.a = cpu.read_byte(0xc800);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.a;
    let result = value.rotate_right(1);
    cpu.flag_z = false;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x01 != 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_c {
        cpu.cycle(16);
        become function1118bc(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    let value = cpu.a;
    let result = value.rotate_right(1);
    cpu.flag_z = false;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x01 != 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_c {
        cpu.cycle(16);
        become function1117e7(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0xc801);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.d = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_de(cpu.de().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0x2;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_c {
        cpu.cycle(16);
        become function1118bc(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.a;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(16);
        become function1118bc(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0xc808);
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
    cpu.a = cpu.read_byte(crate::defs::R_SB as u16);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 0x8;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.l;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(16);
        become function1118bc(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xc81e);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0xff;
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
        become _mobile_receive_asm_111716(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0xf2;
    cpu.pc += 2;
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
    if cpu.flag_z {
        cpu.cycle(16);
        become function111796(cpu);
    }
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
    let value = cpu.read_byte(cpu.hl());
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 1;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(16);
        become function1117a0(cpu);
    }
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
    let value = cpu.read_byte(cpu.hl());
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 1;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(16);
        become function1117a0(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xc807);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x1;
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
        become _mobile_receive_asm_111716(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc806);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.a;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become _mobile_receive_asm_111778(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become _mobile_receive_asm_111716(cpu);
}

pub(crate) fn _mobile_receive_asm_111716(cpu: &mut Cpu) {
    cpu.pc = 0x5716;
    cpu.a = cpu.read_byte(0xc81e);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = -1i16 as u8;
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
        become _mobile_receive_asm_111730(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = { let value = ((MOBILE_COMMAND_ERROR as i32) | (0x80 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
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
        become _mobile_receive_asm_111727(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = { let value = ((MOBILE_COMMAND_TRANSFER_DATA_END as i32) | (0x80 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
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
        become _mobile_receive_asm_111727(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = { let value = ((MOBILE_COMMAND_TRANSFER_DATA as i32) | (0x80 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    become _mobile_receive_asm_111727(cpu);
}

pub(crate) fn _mobile_receive_asm_111727(cpu: &mut Cpu) {
    cpu.pc = 0x5727;
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
        become asm_11179a(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc818);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.a;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become _mobile_receive_asm_111730(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become _mobile_receive_asm_111730(cpu);
}

pub(crate) fn _mobile_receive_asm_111730(cpu: &mut Cpu) {
    cpu.pc = 0x5730;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc819, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x3;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc800, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl(0xc80a);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(0xc81f);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(0xc815);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc822);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((0) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become _mobile_receive_asm_111757(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0xb;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become _mobile_receive_asm_111774(cpu);
}

pub(crate) fn _mobile_receive_asm_111757(cpu: &mut Cpu) {
    cpu.pc = 0x5757;
    cpu.a = cpu.read_byte(0xc81e);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = -1i16 as u8;
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
        become _mobile_receive_asm_11176e(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = { let value = ((MOBILE_COMMAND_DIAL_TELEPHONE as i32) | (0x80 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
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
        become _mobile_receive_asm_111772(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = { let value = ((MOBILE_COMMAND_OPEN_TCP_CONNECTION as i32) | (0x80 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
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
        become _mobile_receive_asm_111772(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = { let value = ((MOBILE_COMMAND_DNS_QUERY as i32) | (0x80 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
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
        become _mobile_receive_asm_111772(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x20;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become _mobile_receive_asm_111774(cpu);
}

pub(crate) fn _mobile_receive_asm_11176e(cpu: &mut Cpu) {
    cpu.pc = 0x576e;
    cpu.a = 0x3;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become _mobile_receive_asm_111774(cpu);
}

pub(crate) fn _mobile_receive_asm_111772(cpu: &mut Cpu) {
    cpu.pc = 0x5772;
    cpu.a = 0x60;
    cpu.pc += 2;
    cpu.cycle(8);
    become _mobile_receive_asm_111774(cpu);
}

pub(crate) fn _mobile_receive_asm_111774(cpu: &mut Cpu) {
    cpu.pc = 0x5774;
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.cycle(16);
    become function1118bc(cpu);
}

pub(crate) fn _mobile_receive_asm_111778(cpu: &mut Cpu) {
    cpu.pc = 0x5778;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc800, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    become function11177c(cpu);
}

pub(crate) fn function11177c(cpu: &mut Cpu) {
    cpu.pc = 0x577c;
    cpu.set_hl(0xc820);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
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
    cpu.b = 0x3;
    cpu.pc += 2;
    cpu.cycle(8);
    become function11177c_asm_111785(cpu);
}

pub(crate) fn function11177c_asm_111785(cpu: &mut Cpu) {
    cpu.pc = 0x5785;
    let value = cpu.a;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.a;
    let carry_in = u8::from(cpu.flag_c) << 7;
    let result = (value >> 1) | carry_in;
    cpu.flag_z = false;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x01 != 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.e;
    let carry_in = u8::from(cpu.flag_c) << 7;
    let result = (value >> 1) | carry_in;
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x01 != 0;
    cpu.e = result;
    cpu.pc += 2;
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
        become function11177c_asm_111785(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.a;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.a;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl(0xc816);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.e);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.cycle(16);
    become function1118bc(cpu);
}

pub(crate) fn function111796(cpu: &mut Cpu) {
    cpu.pc = 0x5796;
    cpu.b = 0xa;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become asm_1117a2(cpu);
}

pub(crate) fn asm_11179a(cpu: &mut Cpu) {
    cpu.pc = 0x579a;
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
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.cycle(16);
    become function1118bc(cpu);
}

pub(crate) fn function1117a0(cpu: &mut Cpu) {
    cpu.pc = 0x57a0;
    cpu.b = 0x3;
    cpu.pc += 2;
    cpu.cycle(8);
    become asm_1117a2(cpu);
}

pub(crate) fn asm_1117a2(cpu: &mut Cpu) {
    cpu.pc = 0x57a2;
    cpu.set_hl(0xc822);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((3) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value |= mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.set_hl(0xc815);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xc820);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc81f);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc800, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xc819);
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
    cpu.a = cpu.b;
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
    if !cpu.flag_c {
        cpu.cycle(16);
        become function1118bc(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl(0xc806);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(0xc800, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x6;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(0xc821);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((1) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value |= mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.a = 0x15;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc80f, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xc810);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xc808);
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
    let value = 0x2;
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
        become asm_1117a2_asm_1117e1(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    become asm_1117a2_asm_1117e1(cpu);
}

pub(crate) fn asm_1117a2_asm_1117e1(cpu: &mut Cpu) {
    cpu.pc = 0x57e1;
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
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.cycle(16);
    become function1118bc(cpu);
}

pub(crate) fn function1117e7(cpu: &mut Cpu) {
    cpu.pc = 0x57e7;
    cpu.a = cpu.read_byte(0xc80b);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.a;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function1117e7_asm_1117f8(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(16);
        become function11186e(cpu);
    }
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
    if cpu.flag_z {
        cpu.cycle(16);
        become function111884(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become function111892(cpu);
}

pub(crate) fn function1117e7_asm_1117f8(cpu: &mut Cpu) {
    cpu.pc = 0x57f8;
    cpu.set_hl(0xc80a);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.a;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(12);
        become function1117e7_asm_111803(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.b = 0x99;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become function1117e7_asm_111805(cpu);
}

pub(crate) fn function1117e7_asm_111803(cpu: &mut Cpu) {
    cpu.pc = 0x5803;
    cpu.b = 0x66;
    cpu.pc += 2;
    cpu.cycle(8);
    become function1117e7_asm_111805(cpu);
}

pub(crate) fn function1117e7_asm_111805(cpu: &mut Cpu) {
    cpu.pc = 0x5805;
    cpu.a = cpu.read_byte(crate::defs::R_SB as u16);
    cpu.pc += 2;
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
    if cpu.flag_z {
        cpu.cycle(12);
        become function1117e7_asm_111840(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0xd2;
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
        become function1117e7_asm_111817(cpu);
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
    cpu.write_byte(0xc9ae, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    become function1117e7_asm_111812(cpu);
}

pub(crate) fn function1117e7_asm_111812(cpu: &mut Cpu) {
    cpu.pc = 0x5812;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.cycle(16);
    become function1118bc(cpu);
}

pub(crate) fn function1117e7_asm_111817(cpu: &mut Cpu) {
    cpu.pc = 0x5817;
    cpu.a = cpu.read_byte(0xc9ae);
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
    cpu.write_byte(0xc9ae, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x14;
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
        become function1117e7_asm_111812(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x6;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc807, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x10;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc80f, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc800, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xc822);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((0) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.set_hl(0xc821);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let mask = 1u8 << ((1) & 7);
    cpu.a |= mask;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0xf;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.cycle(12);
    become function1118bc(cpu);
}

pub(crate) fn function1117e7_asm_111840(cpu: &mut Cpu) {
    cpu.pc = 0x5840;
    let addr = cpu.hl();
    let value = cpu.read_byte(addr);
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.write_byte(addr, result);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.a = 0x2;
    cpu.pc += 2;
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
        become function1118bc(cpu);
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
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
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
    cpu.set_hl(0xc812);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = 0x3;
    cpu.pc += 2;
    cpu.cycle(8);
    become function1117e7_asm_11184e(cpu);
}

pub(crate) fn function1117e7_asm_11184e(cpu: &mut Cpu) {
    cpu.pc = 0x584e;
    cpu.write_byte(cpu.hl(), cpu.a);
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
        become function1117e7_asm_11184e(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc822);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((4) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become function1117e7_asm_111864(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xc821);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((3) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(12);
        become function1117e7_asm_111864(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become function11177c(cpu);
}

pub(crate) fn function1117e7_asm_111864(cpu: &mut Cpu) {
    cpu.pc = 0x5864;
    cpu.a = cpu.read_byte(0xc820);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc81f);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.cycle(12);
    become function1118bc(cpu);
}

pub(crate) fn function11186e(cpu: &mut Cpu) {
    cpu.pc = 0x586e;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function1118c2(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x4;
    cpu.pc += 2;
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
        become function1118bc(cpu);
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
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(crate::defs::R_SB as u16);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.write_byte(0xc80c, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let addr = cpu.hl();
    let value = cpu.read_byte(addr);
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.write_byte(addr, result);
    cpu.pc += 1;
    cpu.cycle(12);
    let value = cpu.a;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(12);
        become function1118bc(cpu);
    }
    cpu.pc += 2;
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
    cpu.cycle(12);
    become function1118bc(cpu);
}

pub(crate) fn function111884(cpu: &mut Cpu) {
    cpu.pc = 0x5884;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function1118c2(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xc80c);
    cpu.pc += 3;
    cpu.cycle(16);
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
        become function1118bc(cpu);
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
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
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
    cpu.cycle(12);
    become function1118bc(cpu);
}

pub(crate) fn function111892(cpu: &mut Cpu) {
    cpu.pc = 0x5892;
    cpu.a = cpu.read_byte(crate::defs::R_SB as u16);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function111664(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xc80a);
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
    cpu.a = 0x2;
    cpu.pc += 2;
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
    if cpu.flag_c {
        cpu.cycle(12);
        become function111892_asm_1118b4(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc80a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x11;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_add(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) > 0x0f;
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.d = 0xc8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.de());
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
    if cpu.flag_z {
        cpu.cycle(12);
        become function1118bc(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc814, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(12);
    become function1118bc(cpu);
}

pub(crate) fn function111892_asm_1118b4(cpu: &mut Cpu) {
    cpu.pc = 0x58b4;
    cpu.a = 0x4;
    cpu.pc += 2;
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
        become function1118bc(cpu);
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
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
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
    become function1118bc(cpu);
}

pub(crate) fn function1118bc(cpu: &mut Cpu) {
    cpu.pc = 0x58bc;
    cpu.set_hl(0xc822);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((1) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function1118c2(cpu: &mut Cpu) {
    cpu.pc = 0x58c2;
    cpu.a = cpu.read_byte(crate::defs::R_SB as u16);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.b = 0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xc812);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.l = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.h = cpu.a;
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
    cpu.a = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc812, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte({ const N: i32 = (0xc812 as i32) + (1 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function111664(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xc80a);
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

pub fn _timer(cpu: &mut Cpu) {
    cpu.pc = 0x58de;
    cpu.a = cpu.read_byte(0xc80b);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x4;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        let next_pc = cpu.pc + 3;
        cpu.push_word(next_pc);
        cpu.cycle(24);
        function111b3c(cpu);
        assert_eq!(cpu.pc, next_pc);
    } else {
        cpu.pc += 3;
        cpu.cycle(12);
    }
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11214e(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xc807);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0x2;
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
        become _timer_asm_111927(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.b;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = 0xff;
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
        become _timer_asm_1118fe(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.b;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(12);
        become _timer_asm_111927(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become _timer_asm_1118fe(cpu);
}

pub(crate) fn _timer_asm_1118fe(cpu: &mut Cpu) {
    cpu.pc = 0x58fe;
    cpu.set_hl(0xc807);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0x6;
    cpu.pc += 2;
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
    if cpu.flag_z {
        cpu.cycle(16);
        become function111b3b(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 0x10;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc80f, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc800, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xc822);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((0) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.set_hl(0xc821);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0xf;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x2;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 0x10;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc80f, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(16);
    become function111b3b(cpu);
}

pub(crate) fn _timer_asm_111927(cpu: &mut Cpu) {
    cpu.pc = 0x5927;
    cpu.a = cpu.read_byte(0xc800);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x1;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(16);
        become function111b21(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    let value = 0x3;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(16);
        become function111a2a(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xc807);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x1;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_c {
        cpu.cycle(16);
        become function111b3b(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0xc815);
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
        cpu.cycle(16);
        become function111b3b(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(cpu.hl().wrapping_add(1));
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
        cpu.cycle(16);
        become function111b3b(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0xc807);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xc822);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((3) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(16);
        become function111a0b(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((4) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(12);
        become _timer_asm_11199c(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0x1;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(16);
        become function1119f0_begin_session(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    let value = 0xa;
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
        become _timer_asm_111984(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x8;
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
        become _timer_asm_11197d(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc86a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x2a;
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
        become _timer_asm_111991(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0xd;
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
        become _timer_asm_111977(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc86b);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x4;
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
        become _timer_asm_11199c(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become _timer_asm_111977(cpu);
}

pub(crate) fn _timer_asm_111977(cpu: &mut Cpu) {
    cpu.pc = 0x5977;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function111f97(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(16);
    become function111b3b(cpu);
}

pub(crate) fn _timer_asm_11197d(cpu: &mut Cpu) {
    cpu.pc = 0x597d;
    cpu.a = cpu.read_byte(0xc805);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.cycle(16);
    become function111b3b(cpu);
}

pub(crate) fn _timer_asm_111984(cpu: &mut Cpu) {
    cpu.pc = 0x5984;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(0xc821);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((0) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function111686(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(16);
    become function111b3b(cpu);
}

pub(crate) fn _timer_asm_111991(cpu: &mut Cpu) {
    cpu.pc = 0x5991;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(0xc821, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function111686(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(16);
    become function111b3b(cpu);
}

pub(crate) fn _timer_asm_11199c(cpu: &mut Cpu) {
    cpu.pc = 0x599c;
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.a;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(16);
        become function111b3b(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xc822);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((7) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(12);
        become _timer_asm_1119be(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become _timer_asm_1119a9(cpu);
}

pub(crate) fn _timer_asm_1119a9(cpu: &mut Cpu) {
    cpu.pc = 0x59a9;
    cpu.a = cpu.read_byte(0xc821);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((3) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(12);
        become _timer_asm_111977(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_de({ const N: i32 = (0x607d as i32) - (0x6072 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0x6072);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0x95;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function111f02(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(16);
    become function111b3b(cpu);
}

pub(crate) fn _timer_asm_1119be(cpu: &mut Cpu) {
    cpu.pc = 0x59be;
    cpu.a = cpu.read_byte(0xc821);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((3) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(12);
        become _timer_asm_1119dd(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte({ const N: i32 = (0xcb47 as i32) + (5 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0xa;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_add(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) > 0x0f;
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.d = 0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = { let value = ((MOBILE_COMMAND_TRANSFER_DATA as i32) | (0x80 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc81e, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xcb47);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = 0x5;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    packet_send_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(16);
    become function111b3b(cpu);
}

pub(crate) fn _timer_asm_1119dd(cpu: &mut Cpu) {
    cpu.pc = 0x59dd;
    cpu.set_hl(0xc821);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((1) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value |= mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    let mask = 1u8 << ((0) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.set_hl(0xc822);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((7) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.a = 0x21;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc80f, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(12);
    become _timer_asm_1119a9(cpu);
}

pub(crate) fn function1119f0_begin_session(cpu: &mut Cpu) {
    cpu.pc = 0x59f0;
    cpu.a = { let value = ((MOBILE_COMMAND_BEGIN_SESSION as i32) | (0x80 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc81e, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xc808, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.b = 0x5;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_de({ const N: i32 = (0x6013 as i32) - (0x6001 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0x6001);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    packet_send_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc806, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(16);
    become function111b3b(cpu);
}

pub(crate) fn function111a0b(cpu: &mut Cpu) {
    cpu.pc = 0x5a0b;
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0x6;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(16);
        become function111b3b(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0xc822);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((3) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    let mask = 1u8 << ((0) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.set_hl(0xc81a);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.d = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.h = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.b = 0x5;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    packet_send_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(16);
    become function111b3b(cpu);
}

pub(crate) fn function111a2a(cpu: &mut Cpu) {
    cpu.pc = 0x5a2a;
    cpu.set_hl(0xc80b);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.a;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become asm_111a47(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x3;
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
        become asm_111a40(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0x2;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(16);
        become function111ab9(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    let value = 0x3;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(16);
        become function111abd(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    become asm_111a40(cpu);
}

pub(crate) fn asm_111a40(cpu: &mut Cpu) {
    cpu.pc = 0x5a40;
    cpu.a = 0x4b;
    cpu.pc += 2;
    cpu.cycle(8);
    become function111a42(cpu);
}

pub(crate) fn function111a42(cpu: &mut Cpu) {
    cpu.pc = 0x5a42;
    cpu.write_byte(crate::defs::R_SB as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.cycle(16);
    become function111b2e(cpu);
}

pub(crate) fn asm_111a47(cpu: &mut Cpu) {
    cpu.pc = 0x5a47;
    cpu.set_hl(0xc815);
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
        become asm_111a40(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
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
        become asm_111a40(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
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
    if cpu.flag_z {
        cpu.cycle(12);
        become asm_111a47_asm_111a63(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xc81f);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.d = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(0xc815);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.cycle(12);
    become asm_111a40(cpu);
}

pub(crate) fn asm_111a47_asm_111a63(cpu: &mut Cpu) {
    cpu.pc = 0x5a63;
    cpu.disable_interrupts_after_next_instruction();
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xc86a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x2a;
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
        become asm_111a47_asm_111aa8(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xc9b2);
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
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0x1;
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
        become asm_111a47_asm_111a91(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xc822);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((5) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    let mask = 1u8 << ((0) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.set_hl(0xc821);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((4) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.a = 0x0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc805, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x29;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc86a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc806, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(12);
    become asm_111a47_asm_111aa8(cpu);
}

pub(crate) fn asm_111a47_asm_111a91(cpu: &mut Cpu) {
    cpu.pc = 0x5a91;
    cpu.a = 0x29;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc86a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc806, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xc86b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xc80b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xc800, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc807, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    become asm_111a47_asm_111aa8(cpu);
}

pub(crate) fn asm_111a47_asm_111aa8(cpu: &mut Cpu) {
    cpu.pc = 0x5aa8;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    reset_receive_packet_buffer(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11164f(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xc822);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((5) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    let mask = 1u8 << ((0) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.enable_interrupts_after_next_instruction();
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.cycle(16);
    become function111b3b(cpu);
}

pub(crate) fn function111ab9(cpu: &mut Cpu) {
    cpu.pc = 0x5ab9;
    cpu.a = 0x80;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become function111a42(cpu);
}

pub(crate) fn function111abd(cpu: &mut Cpu) {
    cpu.pc = 0x5abd;
    cpu.a = cpu.read_byte(0xc814);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.a;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(12);
        become function111abd_asm_111acb(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xca3c);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x80;
    cpu.a ^= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become function111a42(cpu);
}

pub(crate) fn function111abd_asm_111acb(cpu: &mut Cpu) {
    cpu.pc = 0x5acb;
    cpu.set_hl(0xc819);
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
    cpu.a = 0x3;
    cpu.pc += 2;
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
    if cpu.flag_z {
        cpu.cycle(12);
        become function111abd_asm_111afe(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    reset_receive_packet_buffer(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x3;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc800, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl(0xc80a);
    cpu.pc += 3;
    cpu.cycle(12);
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
    cpu.set_hl(0xc815);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xc820);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc81f);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc822);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((0) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become function111abd_asm_111af9(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0xb;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become function111abd_asm_111afb(cpu);
}

pub(crate) fn function111abd_asm_111af9(cpu: &mut Cpu) {
    cpu.pc = 0x5af9;
    cpu.a = 0x20;
    cpu.pc += 2;
    cpu.cycle(8);
    become function111abd_asm_111afb(cpu);
}

pub(crate) fn function111abd_asm_111afb(cpu: &mut Cpu) {
    cpu.pc = 0x5afb;
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.cycle(12);
    become function111abd_asm_111b1c(cpu);
}

pub(crate) fn function111abd_asm_111afe(cpu: &mut Cpu) {
    cpu.pc = 0x5afe;
    cpu.set_hl(0xc806);
    cpu.pc += 3;
    cpu.cycle(12);
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
    cpu.write_byte(0xc800, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x6;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(0xc821);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((1) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value |= mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.a = 0x15;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc80f, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x2;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc810, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte({ const N: i32 = (0xc810 as i32) + (1 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    become function111abd_asm_111b1c(cpu);
}

pub(crate) fn function111abd_asm_111b1c(cpu: &mut Cpu) {
    cpu.pc = 0x5b1c;
    cpu.a = 0xf1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become function111a42(cpu);
}

pub(crate) fn function111b21(cpu: &mut Cpu) {
    cpu.pc = 0x5b21;
    cpu.set_hl(0xc803);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.d = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_SB as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.e);
    cpu.pc += 1;
    cpu.cycle(8);
    become function111b2e(cpu);
}

pub(crate) fn function111b2e(cpu: &mut Cpu) {
    cpu.pc = 0x5b2e;
    cpu.set_hl(0xc822);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((1) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value |= mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.a = { let value = ((crate::defs::SC_FAST as i32) | (crate::defs::SC_INTERNAL as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_SC as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = { let value = (((crate::defs::SC_START as i32) | (crate::defs::SC_FAST as i32)) | (crate::defs::SC_INTERNAL as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_SC as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    become function111b3b(cpu);
}

pub(crate) fn function111b3b(cpu: &mut Cpu) {
    cpu.pc = 0x5b3b;
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function111b3c(cpu: &mut Cpu) {
    cpu.pc = 0x5b3c;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc819, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xc80b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xc9b1);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(0xc800, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xc822);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((5) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    let mask = 1u8 << ((0) & 7);
    let value = cpu.read_byte(cpu.hl());
    cpu.flag_z = (value & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(12);
    if cpu.flag_z {
        cpu.cycle(12);
        become function111b3c_asm_111b59(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xca2f);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(12);
    become function111b3c_asm_111b5c(cpu);
}

pub(crate) fn function111b3c_asm_111b59(cpu: &mut Cpu) {
    cpu.pc = 0x5b59;
    cpu.a = cpu.read_byte(0xca3c);
    cpu.pc += 3;
    cpu.cycle(16);
    become function111b3c_asm_111b5c(cpu);
}

pub(crate) fn function111b3c_asm_111b5c(cpu: &mut Cpu) {
    cpu.pc = 0x5b5c;
    let value = { let value = ((MOBILE_COMMAND_TRANSFER_DATA_END as i32) | (0x80 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
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
        become function111b3c_asm_111b62(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = { let value = ((MOBILE_COMMAND_TRANSFER_DATA as i32) | (0x80 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    become function111b3c_asm_111b62(cpu);
}

pub(crate) fn function111b3c_asm_111b62(cpu: &mut Cpu) {
    cpu.pc = 0x5b62;
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl(0x5e28);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    let value = { let value = ((MOBILE_COMMAND_ERROR as i32) | (0x80 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(16);
        become get_error_code(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xc81e);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = -1i16 as u8;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(16);
        become function111ef8(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    let value = { let value = ((MOBILE_COMMAND_TRANSFER_DATA as i32) | (0x80 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(16);
        become function111c17(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    let value = { let value = ((MOBILE_COMMAND_DNS_QUERY as i32) | (0x80 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(16);
        become function111d23(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    let value = { let value = ((MOBILE_COMMAND_OPEN_TCP_CONNECTION as i32) | (0x80 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
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
        become function111b3c_asm_111bbe(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = { let value = ((MOBILE_COMMAND_CLOSE_TCP_CONNECTION as i32) | (0x80 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
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
        become function111b3c_asm_111bbe(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = { let value = ((MOBILE_COMMAND_HANG_UP_TELEPHONE as i32) | (0x80 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
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
        become function111b3c_asm_111be0(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = { let value = ((MOBILE_COMMAND_READ_CONFIGURATION_DATA as i32) | (0x80 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
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
        become function111b3c_asm_111bf0(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = { let value = ((MOBILE_COMMAND_WRITE_CONFIGURATION_DATA as i32) | (0x80 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(16);
        become function111c06(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    let value = { let value = ((MOBILE_COMMAND_TELEPHONE_STATUS as i32) | (0x80 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(16);
        become function111d70(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    let value = { let value = ((MOBILE_COMMAND_ISP_LOGIN as i32) | (0x80 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
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
        become function111b3c_asm_111bd0(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = { let value = ((MOBILE_COMMAND_ISP_LOGOUT as i32) | (0x80 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
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
        become function111b3c_asm_111bca(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = { let value = ((MOBILE_COMMAND_BEGIN_SESSION as i32) | (0x80 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(16);
        become parse_response_begin_session(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    let value = { let value = ((MOBILE_COMMAND_WAIT_FOR_TELEPHONE_CALL as i32) | (0x80 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(16);
        become function111d65(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    let value = { let value = ((MOBILE_COMMAND_DIAL_TELEPHONE as i32) | (0x80 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(16);
        become function111d65(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0xc822);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((0) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.a = 0xa;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc807, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc800, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function111b3c_asm_111bbe(cpu: &mut Cpu) {
    cpu.pc = 0x5bbe;
    cpu.a = cpu.read_byte({ const N: i32 = (0xca3c as i32) + (4 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xc86c, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x4;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc807, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function111b3c_asm_111bca(cpu: &mut Cpu) {
    cpu.pc = 0x5bca;
    cpu.a = 0x3;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc807, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function111b3c_asm_111bd0(cpu: &mut Cpu) {
    cpu.pc = 0x5bd0;
    cpu.a = 0x4;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc807, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_de(0xc823);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl({ const N: i32 = (0xca3c as i32) + (4 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = 0x4;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become mobile_sdk_copy_bytes(cpu);
}

pub(crate) fn function111b3c_asm_111be0(cpu: &mut Cpu) {
    cpu.pc = 0x5be0;
    cpu.a = 0x2;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc807, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xc822);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((4) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.set_hl(0xc821);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((4) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function111b3c_asm_111bf0(cpu: &mut Cpu) {
    cpu.pc = 0x5bf0;
    cpu.set_hl(0xc829);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.d = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl({ const N: i32 = (0xca3c as i32) + (3 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
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
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x2;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc807, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function111c06(cpu: &mut Cpu) {
    cpu.pc = 0x5c06;
    cpu.set_de(0xc872);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl({ const N: i32 = (0xca3c as i32) + (4 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = 0x2;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x2;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc807, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function111c17(cpu: &mut Cpu) {
    cpu.pc = 0x5c17;
    cpu.a = cpu.read_byte(0xca3c);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = { let value = ((MOBILE_COMMAND_TRANSFER_DATA_END as i32) | (0x80 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(16);
        become function111d07(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte({ const N: i32 = (0xc86e as i32) + (1 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xc86e);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.b;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(16);
        become function111d07(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0xc82b);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.d = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte({ const N: i32 = (0xca3c as i32) + (3 as i32); N as u16 });
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
    if cpu.flag_z {
        cpu.cycle(16);
        become function111d07(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xc822);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((4) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(16);
        become function111cc2(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xc992);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.a;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(12);
        become function111c17_asm_111c89(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte({ const N: i32 = (0xca3c as i32) + (5 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.a;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function111c17_asm_111c50(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x81;
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
        become function111c17_asm_111c52(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become function111c17_asm_111c50(cpu);
}

pub(crate) fn function111c17_asm_111c50(cpu: &mut Cpu) {
    cpu.pc = 0x5c50;
    cpu.a = 0x80;
    cpu.pc += 2;
    cpu.cycle(8);
    become function111c17_asm_111c52(cpu);
}

pub(crate) fn function111c17_asm_111c52(cpu: &mut Cpu) {
    cpu.pc = 0x5c52;
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte({ const N: i32 = (0xca3c as i32) + (3 as i32); N as u16 });
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
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
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
        become function111c17_asm_111c6e(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become function111c17_asm_111c5b(cpu);
}

pub(crate) fn function111c17_asm_111c5b(cpu: &mut Cpu) {
    cpu.pc = 0x5c5b;
    cpu.set_hl(0xc821);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((3) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value |= mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.set_hl(0xc993);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte({ const N: i32 = (0xca3c as i32) + (3 as i32); N as u16 });
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
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.cycle(16);
    become function111d07(cpu);
}

pub(crate) fn function111c17_asm_111c6e(cpu: &mut Cpu) {
    cpu.pc = 0x5c6e;
    cpu.set_hl(0xc992);
    cpu.pc += 3;
    cpu.cycle(12);
    let value = cpu.a;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function111c17_asm_111c83(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.b);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl({ const N: i32 = (0xca3c as i32) + (6 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xc880);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(16);
    become function111d07(cpu);
}

pub(crate) fn function111c17_asm_111c83(cpu: &mut Cpu) {
    cpu.pc = 0x5c83;
    cpu.a = 0xff;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.b);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.cycle(12);
    become function111d07(cpu);
}

pub(crate) fn function111c17_asm_111c89(cpu: &mut Cpu) {
    cpu.pc = 0x5c89;
    let value = 0xff;
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
        become function111c17_asm_111c9d(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xc991);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte({ const N: i32 = (0xca3c as i32) + (3 as i32); N as u16 });
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
    let value = cpu.b;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_c {
        cpu.cycle(12);
        become function111c17_asm_111c5b(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become function111c17_asm_111c5b(cpu);
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
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    become function111c17_asm_111c9d(cpu);
}

pub(crate) fn function111c17_asm_111c9d(cpu: &mut Cpu) {
    cpu.pc = 0x5c9d;
    cpu.set_hl(0xc991);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.read_byte(cpu.hl());
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte({ const N: i32 = (0xca3c as i32) + (3 as i32); N as u16 });
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
    let value = cpu.b;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_c {
        cpu.cycle(12);
        become function111c17_asm_111c5b(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become function111c17_asm_111c5b(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.l = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.h = 0x0;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.l;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_add(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) > 0x0f;
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc992, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_de(0xc880);
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
    cpu.e = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.d = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl({ const N: i32 = (0xca3c as i32) + (5 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(12);
    become function111d07(cpu);
}

pub(crate) fn function111cc2(cpu: &mut Cpu) {
    cpu.pc = 0x5cc2;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.d;
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
        become function111cc2_asm_111cda(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
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
        become function111cc2_asm_111cda(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become function111cc2_asm_111cda(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc821);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((2) & 7);
    cpu.a |= mask;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc821, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.e;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.c = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.cycle(12);
    become function111cc2_asm_111ce1(cpu);
}

pub(crate) fn function111cc2_asm_111cda(cpu: &mut Cpu) {
    cpu.pc = 0x5cda;
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
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
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = 0x0;
    let carry_in = u8::from(cpu.flag_c);
    let lhs = cpu.a;
    let (partial, borrow1) = lhs.overflowing_sub(value);
    let (result, borrow2) = partial.overflowing_sub(carry_in);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < ((value & 0x0f) + carry_in);
    cpu.flag_c = borrow1 || borrow2;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.d = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    become function111cc2_asm_111ce1(cpu);
}

pub(crate) fn function111cc2_asm_111ce1(cpu: &mut Cpu) {
    cpu.pc = 0x5ce1;
    cpu.a = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.e);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc829);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte({ const N: i32 = (0xc829 as i32) + (1 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.d = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl({ const N: i32 = (0xca3c as i32) + (5 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.a;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function111d07(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xc829);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.d);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(0x3 as u16);
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
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.c;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_add(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) > 0x0f;
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    if !cpu.flag_c {
        cpu.cycle(12);
        become function111d07(cpu);
    }
    cpu.pc += 2;
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
    become function111d07(cpu);
}

pub(crate) fn function111d07(cpu: &mut Cpu) {
    cpu.pc = 0x5d07;
    cpu.a = cpu.read_byte(0xc822);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((4) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become function111d07_asm_111d1c(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let mask = 1u8 << ((7) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become function111d07_asm_111d1c(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xc822);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((7) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.set_hl(0xc821);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((0) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    become function111d07_asm_111d1c(cpu);
}

pub(crate) fn function111d07_asm_111d1c(cpu: &mut Cpu) {
    cpu.pc = 0x5d1c;
    cpu.a = cpu.read_byte(0xc805);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xc807, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function111d23(cpu: &mut Cpu) {
    cpu.pc = 0x5d23;
    cpu.a = cpu.read_byte(0xc829);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte({ const N: i32 = (0xc829 as i32) + (1 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.d = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl({ const N: i32 = (0xca3c as i32) + (4 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = 0x4;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x4;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc807, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn parse_response_begin_session(cpu: &mut Cpu) {
    cpu.pc = 0x5d39;
    cpu.set_de({ const N: i32 = (0xca3c as i32) + (3 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl({ const N: i32 = (0x6001 as i32) + (5 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = { let value = ((1 as i32) + (8 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    become parse_response_begin_session_check_loop(cpu);
}

pub(crate) fn parse_response_begin_session_check_loop(cpu: &mut Cpu) {
    cpu.pc = 0x5d41;
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
        become parse_response_begin_session_check_done(cpu);
    }
    cpu.pc += 2;
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
        become parse_response_begin_session_check_loop(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become parse_response_begin_session_check_done(cpu);
}

pub(crate) fn parse_response_begin_session_check_done(cpu: &mut Cpu) {
    cpu.pc = 0x5d4a;
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.a;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(12);
        become parse_response_begin_session_done(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte({ const N: i32 = (0xca3c as i32) + (14 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x80;
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
        become parse_response_begin_session_fail(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x90;
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
        become parse_response_begin_session_fail(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become parse_response_begin_session_done(cpu);
}

pub(crate) fn parse_response_begin_session_done(cpu: &mut Cpu) {
    cpu.pc = 0x5d59;
    cpu.write_byte(0xc818, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x2;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc807, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn parse_response_begin_session_fail(cpu: &mut Cpu) {
    cpu.pc = 0x5d62;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.cycle(12);
    become parse_response_begin_session_done(cpu);
}

pub(crate) fn function111d65(cpu: &mut Cpu) {
    cpu.pc = 0x5d65;
    cpu.a = 0x3;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc807, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xc821);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((4) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value |= mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function111d70(cpu: &mut Cpu) {
    cpu.pc = 0x5d70;
    cpu.set_hl(0xc822);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((0) & 7);
    let value = cpu.read_byte(cpu.hl());
    cpu.flag_z = (value & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(12);
    if cpu.flag_z {
        cpu.cycle(12);
        become function111d70_asm_111dc0(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc805);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xc807, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte({ const N: i32 = (0xca2f as i32) + (4 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function111dd9(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function111e15(cpu);
    assert_eq!(cpu.pc, next_pc);
    let mask = 1u8 << ((0) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = 0x7;
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
        become function111d70_asm_111da9(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.a;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
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
    cpu.set_hl(0xc821);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((4) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    let mask = 1u8 << ((1) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value |= mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xc822);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((4) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(12);
        become function111d70_asm_111dbb(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x23;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc80f, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x6;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc807, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function111d70_asm_111da9(cpu: &mut Cpu) {
    cpu.pc = 0x5da9;
    cpu.set_hl(0xc821);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((4) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    let mask = 1u8 << ((1) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value |= mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.a = 0x11;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc80f, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x6;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc807, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function111d70_asm_111dbb(cpu: &mut Cpu) {
    cpu.pc = 0x5dbb;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc807, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function111d70_asm_111dc0(cpu: &mut Cpu) {
    cpu.pc = 0x5dc0;
    cpu.set_hl(0xc86e);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.h = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte({ const N: i32 = (0xca3c as i32) + (4 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function111dd9(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function111e15(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc805);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xc807, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function111dd9(cpu: &mut Cpu) {
    cpu.pc = 0x5dd9;
    let value = 0xff;
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
        become function111dd9_asm_111de7(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.a;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
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
    let value = 0x4;
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
        become function111dd9_asm_111dea(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x5;
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
        become function111dd9_asm_111e12(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become function111dd9_asm_111de7(cpu);
}

pub(crate) fn function111dd9_asm_111de7(cpu: &mut Cpu) {
    cpu.pc = 0x5de7;
    cpu.b = 0x7;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function111dd9_asm_111dea(cpu: &mut Cpu) {
    cpu.pc = 0x5dea;
    cpu.b = 0x5;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc822);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((0) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become function111dd9_asm_111df8(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc86a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(12);
    become function111dd9_asm_111dfb(cpu);
}

pub(crate) fn function111dd9_asm_111df8(cpu: &mut Cpu) {
    cpu.pc = 0x5df8;
    cpu.a = cpu.read_byte(0xc985);
    cpu.pc += 3;
    cpu.cycle(16);
    become function111dd9_asm_111dfb(cpu);
}

pub(crate) fn function111dd9_asm_111dfb(cpu: &mut Cpu) {
    cpu.pc = 0x5dfb;
    let value = 0x4;
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
    let value = 0x1c;
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
    let value = 0x1a;
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
    let value = cpu.b;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.b = result;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = 0x3;
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
    cpu.b = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc822);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((4) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.b;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.b = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function111dd9_asm_111e12(cpu: &mut Cpu) {
    cpu.pc = 0x5e12;
    cpu.b = 0x3;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function111e15(cpu: &mut Cpu) {
    cpu.pc = 0x5e15;
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = 0x7;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value.rotate_right(1);
    cpu.flag_z = false;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x01 != 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.a;
    let result = value.rotate_right(1);
    cpu.flag_z = false;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x01 != 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.a;
    let result = value.rotate_right(1);
    cpu.flag_z = false;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x01 != 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xc821);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x1f;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.l;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc821, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function111e28(cpu: &mut Cpu) {
    cpu.pc = 0x5e28;
    cpu.cycle(16);
    become reset_receive_packet_buffer(cpu);
}

pub(crate) fn get_error_code(cpu: &mut Cpu) {
    cpu.pc = 0x5e2b;
    cpu.a = cpu.read_byte(0xc81e);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = -1i16 as u8;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(16);
        become function111ef8(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xc86a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0xd;
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
        become get_error_code_asm_111e48(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x2a;
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
        become get_error_code_asm_111e48(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x6;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc807, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xc821);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((1) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value |= mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    become get_error_code_asm_111e48(cpu);
}

pub(crate) fn get_error_code_asm_111e48(cpu: &mut Cpu) {
    cpu.pc = 0x5e48;
    cpu.a = cpu.read_byte(0xc822);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((0) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become get_error_code_asm_111e54(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl({ const N: i32 = (0xca2f as i32) + (4 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(12);
    become get_error_code_asm_111e57(cpu);
}

pub(crate) fn get_error_code_asm_111e54(cpu: &mut Cpu) {
    cpu.pc = 0x5e54;
    cpu.set_hl({ const N: i32 = (0xca3c as i32) + (4 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    become get_error_code_asm_111e57(cpu);
}

pub(crate) fn get_error_code_asm_111e57(cpu: &mut Cpu) {
    cpu.pc = 0x5e57;
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(0xc80e, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = MOBILE_COMMAND_BEGIN_SESSION as u8;
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
        become get_error_code_adapter_not_plugged_in(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = MOBILE_COMMAND_DIAL_TELEPHONE as u8;
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
        become get_error_code_dial_telephone(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = MOBILE_COMMAND_HANG_UP_TELEPHONE as u8;
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
        become get_error_code_hang_up_logout(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = MOBILE_COMMAND_TRANSFER_DATA as u8;
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
        become get_error_code_transfer_data(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = MOBILE_COMMAND_READ_CONFIGURATION_DATA as u8;
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
        become get_error_code_read_configuration_data(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = MOBILE_COMMAND_ISP_LOGIN as u8;
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
        become get_error_code_isp_login(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = MOBILE_COMMAND_ISP_LOGOUT as u8;
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
        become get_error_code_hang_up_logout(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = MOBILE_COMMAND_OPEN_TCP_CONNECTION as u8;
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
        become get_error_code_open_tcp_connection(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = MOBILE_COMMAND_CLOSE_TCP_CONNECTION as u8;
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
        become get_error_code_close_tcp_connection(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = MOBILE_COMMAND_DNS_QUERY as u8;
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
        become get_error_code_dns_query(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    become get_error_code_store_error_code(cpu);
}

pub(crate) fn get_error_code_store_error_code(cpu: &mut Cpu) {
    cpu.pc = 0x5e84;
    cpu.write_byte(0xc80f, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn get_error_code_adapter_not_plugged_in(cpu: &mut Cpu) {
    cpu.pc = 0x5e88;
    cpu.a = 0x10;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become get_error_code_store_error_code(cpu);
}

pub(crate) fn get_error_code_dial_telephone(cpu: &mut Cpu) {
    cpu.pc = 0x5e8c;
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0x0;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become get_error_code_asm_111e9d(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x2;
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
        become get_error_code_asm_111e99(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x13;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become get_error_code_store_error_code(cpu);
}

pub(crate) fn get_error_code_asm_111e99(cpu: &mut Cpu) {
    cpu.pc = 0x5e99;
    cpu.a = 0x17;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become get_error_code_store_error_code(cpu);
}

pub(crate) fn get_error_code_asm_111e9d(cpu: &mut Cpu) {
    cpu.pc = 0x5e9d;
    cpu.a = 0x12;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become get_error_code_store_error_code(cpu);
}

pub(crate) fn get_error_code_hang_up_logout(cpu: &mut Cpu) {
    cpu.pc = 0x5ea1;
    cpu.set_hl(0xc821);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((1) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    let mask = 1u8 << ((4) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.a = 0x2;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc807, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn get_error_code_transfer_data(cpu: &mut Cpu) {
    cpu.pc = 0x5eae;
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0x1;
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
        become get_error_code_asm_111ed3(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc822);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((4) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become get_error_code_asm_111ed3(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let mask = 1u8 << ((4) & 7);
    cpu.a &= !mask;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc822, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xc821);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0xf;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x2;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 0x23;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc80f, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x6;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc807, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn get_error_code_asm_111ed3(cpu: &mut Cpu) {
    cpu.pc = 0x5ed3;
    cpu.set_hl(0xc822);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((5) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.a = 0x24;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become get_error_code_store_error_code(cpu);
}

pub(crate) fn get_error_code_read_configuration_data(cpu: &mut Cpu) {
    cpu.pc = 0x5edc;
    cpu.a = 0x14;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become get_error_code_store_error_code(cpu);
}

pub(crate) fn get_error_code_isp_login(cpu: &mut Cpu) {
    cpu.pc = 0x5ee0;
    cpu.a = 0x22;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become get_error_code_store_error_code(cpu);
}

pub(crate) fn get_error_code_open_tcp_connection(cpu: &mut Cpu) {
    cpu.pc = 0x5ee4;
    cpu.set_hl(0xc821);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((1) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    become get_error_code_dns_query(cpu);
}

pub(crate) fn get_error_code_dns_query(cpu: &mut Cpu) {
    cpu.pc = 0x5ee9;
    cpu.a = 0x24;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become get_error_code_store_error_code(cpu);
}

pub(crate) fn get_error_code_close_tcp_connection(cpu: &mut Cpu) {
    cpu.pc = 0x5eed;
    cpu.set_hl(0xc821);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((1) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.a = 0x3;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc807, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function111ef8(cpu: &mut Cpu) {
    cpu.pc = 0x5ef8;
    cpu.a = cpu.read_byte(0xc805);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xc807, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn packet_send_empty_body(cpu: &mut Cpu) {
    cpu.pc = 0x5eff;
    cpu.set_de(10 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    become function111f02(cpu);
}

pub(crate) fn function111f02(cpu: &mut Cpu) {
    cpu.pc = 0x5f02;
    cpu.write_byte(0xc81e, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.b = 0x5;
    cpu.pc += 2;
    cpu.cycle(8);
    become packet_send_bytes(cpu);
}

pub(crate) fn packet_send_bytes(cpu: &mut Cpu) {
    cpu.pc = 0x5f07;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function1100b4(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc800);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x0;
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
        become packet_send_bytes_asm_111f17(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function110226(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = true;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn packet_send_bytes_asm_111f17(cpu: &mut Cpu) {
    cpu.pc = 0x5f17;
    cpu.a = cpu.read_byte(crate::defs::R_SC as u16);
    cpu.pc += 2;
    cpu.cycle(12);
    let value = crate::defs::SC_START as u8;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(12);
        become packet_send_bytes_asm_111f17(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.disable_interrupts_after_next_instruction();
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xc81e);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0xff;
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
        become packet_send_bytes_asm_111f35(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc81c, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc81d, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc81a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc81b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    become packet_send_bytes_asm_111f35(cpu);
}

pub(crate) fn packet_send_bytes_asm_111f35(cpu: &mut Cpu) {
    cpu.pc = 0x5f35;
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc801, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc802, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc803, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc804, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xc807);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
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
    if cpu.flag_z {
        cpu.cycle(12);
        become packet_send_bytes_asm_111f4f(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc805, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    become packet_send_bytes_asm_111f4f(cpu);
}

pub(crate) fn packet_send_bytes_asm_111f4f(cpu: &mut Cpu) {
    cpu.pc = 0x5f4f;
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc807, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc806, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc800, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xc822);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((5) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value |= mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.enable_interrupts_after_next_instruction();
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function111f63(cpu: &mut Cpu) {
    cpu.pc = 0x5f63;
    cpu.push_word(cpu.de());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.set_hl(0 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.c = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.b;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function111f63_asm_111f71(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become function111f63_asm_111f6c(cpu);
}

pub(crate) fn function111f63_asm_111f6c(cpu: &mut Cpu) {
    cpu.pc = 0x5f6c;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function111f8d(cpu);
    assert_eq!(cpu.pc, next_pc);
    if !cpu.flag_z {
        cpu.cycle(12);
        become function111f63_asm_111f6c(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become function111f63_asm_111f71(cpu);
}

pub(crate) fn function111f63_asm_111f71(cpu: &mut Cpu) {
    cpu.pc = 0x5f71;
    cpu.b = 0x4;
    cpu.pc += 2;
    cpu.cycle(8);
    become function111f63_asm_111f73(cpu);
}

pub(crate) fn function111f63_asm_111f73(cpu: &mut Cpu) {
    cpu.pc = 0x5f73;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function111f8d(cpu);
    assert_eq!(cpu.pc, next_pc);
    if !cpu.flag_z {
        cpu.cycle(12);
        become function111f63_asm_111f73(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.e = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.d = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl(0x000a as u16);
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
    cpu.c = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.b = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.a = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 0x80;
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
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.e = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.d = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function111f8d(cpu: &mut Cpu) {
    cpu.pc = 0x5f8d;
    cpu.set_de(cpu.de().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.l;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_add(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) > 0x0f;
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = 0x0;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.h;
    let carry_in = u8::from(cpu.flag_c);
    let lhs = cpu.a;
    let (partial, carry1) = lhs.overflowing_add(value);
    let (result, carry2) = partial.overflowing_add(carry_in);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) + carry_in > 0x0f;
    cpu.flag_c = carry1 || carry2;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.h = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.b;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.b = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function111f97(cpu: &mut Cpu) {
    cpu.pc = 0x5f97;
    cpu.set_hl(0xc822);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((0) & 7);
    let value = cpu.read_byte(cpu.hl());
    cpu.flag_z = (value & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(12);
    if !cpu.flag_z {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc807);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x2;
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
        become function111f97_done(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x5;
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
        become function111f97_done(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x6;
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
        become function111f97_hang_up(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc80f);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x22;
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
        become function111f97_done(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x23;
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
        become function111f97_done(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x26;
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
        become function111f97_done(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value.rotate_left(4);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0xf;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x1;
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
        become function111f97_done(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x0;
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
        become function111f97_done(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x8;
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
        become function111f97_hang_up(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become function111f97_done(cpu);
}

pub(crate) fn function111f97_done(cpu: &mut Cpu) {
    cpu.pc = 0x5fcb;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = true;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function111f97_hang_up(cpu: &mut Cpu) {
    cpu.pc = 0x5fcd;
    cpu.b = 0x5;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xc81e);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = -1i16 as u8;
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
        become function111f97_begin_session(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = { let value = ((MOBILE_COMMAND_TELEPHONE_STATUS as i32) | (0x80 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(0x602d);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de({ const N: i32 = (0x6037 as i32) - (0x602d as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    packet_send_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xc822);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((0) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value |= mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function111f97_begin_session(cpu: &mut Cpu) {
    cpu.pc = 0x5fe9;
    cpu.set_hl(0x6001);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de({ const N: i32 = (0x6013 as i32) - (0x6001 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become packet_send_bytes(cpu);
}

pub(crate) fn function11214e(cpu: &mut Cpu) {
    cpu.pc = 0x614e;
    cpu.a = cpu.read_byte(0xc822);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((5) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc86a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0xa;
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
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = 0xd;
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
        become function11214e_asm_112187(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0xf;
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
        become function11214e_asm_112196(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x29;
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
        become function11214e_asm_112175(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x2a;
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
        become function11214e_asm_112175(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x28;
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
        become function11214e_asm_112175(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become function11214e_asm_11216f(cpu);
}

pub(crate) fn function11214e_asm_11216f(cpu: &mut Cpu) {
    cpu.pc = 0x616f;
    cpu.a = cpu.read_byte(0xc807);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x6;
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
    become function11214e_asm_112175(cpu);
}

pub(crate) fn function11214e_asm_112175(cpu: &mut Cpu) {
    cpu.pc = 0x6175;
    cpu.b = 0x0;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.c;
    let result = value << 1;
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x80 != 0;
    cpu.c = result;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl({ const N: i32 = ((0x61ac as i32) - ((2 as i32) * (0xa as i32))); N as u16 });
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
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.h = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.set_hl(0xc86b);
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
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function11214e_asm_112187(cpu: &mut Cpu) {
    cpu.pc = 0x6187;
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xc86b);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x1;
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
        become function11214e_asm_11216f(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xc821);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((1) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.cycle(12);
    become function11214e_asm_112175(cpu);
}

pub(crate) fn function11214e_asm_112196(cpu: &mut Cpu) {
    cpu.pc = 0x6196;
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xc80f);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x24;
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
        become function11214e_asm_11216f(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc86b);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x1;
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
        become function11214e_asm_11216f(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xc821);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((1) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.cycle(12);
    become function11214e_asm_112175(cpu);
}

pub(crate) fn function1121f6(cpu: &mut Cpu) {
    cpu.pc = 0x61f6;
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function1121f6_asm_1121fe(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function1121f6_asm_112210(cpu);
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
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function1121f6_asm_1121fe(cpu: &mut Cpu) {
    cpu.pc = 0x61fe;
    cpu.a = cpu.read_byte(0xc818);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.a;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function1121f6_asm_112206(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become mobile_end_session(cpu);
}

pub(crate) fn function1121f6_asm_112206(cpu: &mut Cpu) {
    cpu.pc = 0x6206;
    cpu.a = 0x10;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11225d(cpu);
    assert_eq!(cpu.pc, next_pc);
    let mask = 1u8 << ((0) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    let mask = 1u8 << ((1) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value |= mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function1121f6_asm_112210(cpu: &mut Cpu) {
    cpu.pc = 0x6210;
    cpu.set_hl(0xc86e);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.h = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xc818);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x88;
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
        become function1121f6_asm_112249(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x88;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0x4;
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
        become function1121f6_asm_112226(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x3;
    cpu.pc += 2;
    cpu.cycle(8);
    become function1121f6_asm_112226(cpu);
}

pub(crate) fn function1121f6_asm_112226(cpu: &mut Cpu) {
    cpu.pc = 0x6226;
    let value = 0x3;
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
        become function1121f6_asm_11222b(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    become function1121f6_asm_11222b(cpu);
}

pub(crate) fn function1121f6_asm_11222b(cpu: &mut Cpu) {
    cpu.pc = 0x622b;
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = 0x4;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.b;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.d = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.a;
    let result = value.rotate_left(1);
    cpu.flag_z = false;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x80 != 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.d;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_add(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) > 0x0f;
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
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
    let value = cpu.b;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function1121f6_asm_11223a(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x3;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.b;
    cpu.a ^= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    become function1121f6_asm_11223a(cpu);
}

pub(crate) fn function1121f6_asm_11223a(cpu: &mut Cpu) {
    cpu.pc = 0x623a;
    cpu.set_hl(0xc871);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.c);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc86a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0xa;
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
        become function112251(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become function1116a0(cpu);
}

pub(crate) fn function1121f6_asm_112249(cpu: &mut Cpu) {
    cpu.pc = 0x6249;
    cpu.a = 0x10;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11225d(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(16);
    become function1116a4(cpu);
}

pub(crate) fn function112251(cpu: &mut Cpu) {
    cpu.pc = 0x6251;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc821, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xc807, cpu.a);
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
    cpu.write_byte(0xc86a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function11225d(cpu: &mut Cpu) {
    cpu.pc = 0x625d;
    cpu.write_byte(0xc80f, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x5;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc86a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xc821);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn mobile_end_session(cpu: &mut Cpu) {
    cpu.pc = 0x6269;
    cpu.a = { let value = ((MOBILE_COMMAND_END_SESSION as i32) | (0x80 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x6013);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become packet_send_empty_body(cpu);
}

pub(crate) fn function112271(cpu: &mut Cpu) {
    cpu.pc = 0x6271;
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function112271_asm_11228c(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function112271_asm_112292(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function112271_asm_1122a1(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(16);
        become function112271_asm_112309(cpu);
    }
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
    if cpu.flag_z {
        cpu.cycle(16);
        become function112271_asm_112326(cpu);
    }
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
    if cpu.flag_z {
        cpu.cycle(16);
        become function112271_asm_112335(cpu);
    }
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
    if cpu.flag_z {
        cpu.cycle(16);
        become function112271_asm_112342(cpu);
    }
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
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function112271_asm_11228c(cpu: &mut Cpu) {
    cpu.pc = 0x628c;
    cpu.set_hl(0x6046);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become function11236b(cpu);
}

pub(crate) fn function112271_asm_112292(cpu: &mut Cpu) {
    cpu.pc = 0x6292;
    cpu.set_hl(0xc829);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = ((((0xc8ca as u32) + (22 as u32))) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = (((((0xc8ca as u32) + (22 as u32))) >> 8) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(0x6052);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become function11236b(cpu);
}

pub(crate) fn function112271_asm_1122a1(cpu: &mut Cpu) {
    cpu.pc = 0x62a1;
    cpu.set_hl(0xc880);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = { let value = (0x8c as i32); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
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
        become function112271_asm_1122f5(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = { let value = (0x80 as i32); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
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
        become function112271_asm_1122f5(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.b = 0xbe;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_de(0 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    become function112271_asm_1122b3(cpu);
}

pub(crate) fn function112271_asm_1122b3(cpu: &mut Cpu) {
    cpu.pc = 0x62b3;
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.e;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_add(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) > 0x0f;
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = 0x0;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.d;
    let carry_in = u8::from(cpu.flag_c);
    let lhs = cpu.a;
    let (partial, carry1) = lhs.overflowing_add(value);
    let (result, carry2) = partial.overflowing_add(carry_in);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) + carry_in > 0x0f;
    cpu.flag_c = carry1 || carry2;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.d = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
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
        become function112271_asm_1122b3(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.d;
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
        become function112271_asm_1122fc(cpu);
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
    if !cpu.flag_z {
        cpu.cycle(12);
        become function112271_asm_1122fc(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xc884);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xc836);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = 0x8;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xc8ca);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = 0x2c;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte({ const N: i32 = (0xcb47 as i32) + (50 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = 0x8;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.d = 0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl({ const N: i32 = (0xcb47 as i32) + (51 as i32); N as u16 });
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
    cpu.e = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.d = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl(0xc836);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = 0x8;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.b = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function111f63(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(12);
    become function11235a(cpu);
}

pub(crate) fn function112271_asm_1122f5(cpu: &mut Cpu) {
    cpu.pc = 0x62f5;
    cpu.a = 0x25;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc872, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(12);
    become function112271_asm_112301(cpu);
}

pub(crate) fn function112271_asm_1122fc(cpu: &mut Cpu) {
    cpu.pc = 0x62fc;
    cpu.a = 0x14;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc872, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    become function112271_asm_112301(cpu);
}

pub(crate) fn function112271_asm_112301(cpu: &mut Cpu) {
    cpu.pc = 0x6301;
    cpu.a = 0x6;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc86b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(16);
    become mobile_end_session(cpu);
}

pub(crate) fn function112271_asm_112309(cpu: &mut Cpu) {
    cpu.pc = 0x6309;
    cpu.a = cpu.read_byte(0xc821);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0xe0;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(12);
        become function112271_asm_112314(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.b = 0x92;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become function11234b(cpu);
}

pub(crate) fn function112271_asm_112314(cpu: &mut Cpu) {
    cpu.pc = 0x6314;
    let value = 0xe0;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x11;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become function112271_asm_11231b(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    become function112271_asm_11231b(cpu);
}

pub(crate) fn function112271_asm_11231b(cpu: &mut Cpu) {
    cpu.pc = 0x631b;
    cpu.write_byte(0xc872, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x6;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc86b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(16);
    become mobile_end_session(cpu);
}

pub(crate) fn function112271_asm_112326(cpu: &mut Cpu) {
    cpu.pc = 0x6326;
    cpu.d = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte({ const N: i32 = (0xcb47 as i32) + (50 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0xa;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_add(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) > 0x0f;
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl({ const N: i32 = (0xcb47 as i32) + (45 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0xa1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become function111f02(cpu);
}

pub(crate) fn function112271_asm_112335(cpu: &mut Cpu) {
    cpu.pc = 0x6335;
    cpu.a = 0x2;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc86a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xc821);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((0) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    let mask = 1u8 << ((5) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value |= mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function112271_asm_112342(cpu: &mut Cpu) {
    cpu.pc = 0x6342;
    cpu.a = cpu.read_byte(0xc872);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11225d(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(16);
    become function1116a4(cpu);
}

pub(crate) fn function11234b(cpu: &mut Cpu) {
    cpu.pc = 0x634b;
    cpu.a = cpu.read_byte({ const N: i32 = (0xcb47 as i32) + (5 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0xa;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_add(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) > 0x0f;
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.d = 0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xcb47);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.cycle(16);
    become function111f02(cpu);
}

pub(crate) fn function11235a(cpu: &mut Cpu) {
    cpu.pc = 0x635a;
    cpu.set_hl(0xc86e);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = ((0xc880u32) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = (((0xc880u32) >> 8) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = { let value = ((MOBILE_COMMAND_TELEPHONE_STATUS as i32) | (0x80 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x602d);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become packet_send_empty_body(cpu);
}

pub(crate) fn function11236b(cpu: &mut Cpu) {
    cpu.pc = 0x636b;
    cpu.a = 0x99;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_de(0x000c as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become function111f02(cpu);
}

pub(crate) fn function112373(cpu: &mut Cpu) {
    cpu.pc = 0x6373;
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function11235a(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function112373_asm_112381(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function112373_asm_11239b(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function112373_asm_1123ad(cpu);
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
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function112373_asm_112381(cpu: &mut Cpu) {
    cpu.pc = 0x6381;
    cpu.a = cpu.read_byte(0xc821);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0xe0;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(12);
        become function112373_asm_11238c(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.b = 0x92;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become function11234b(cpu);
}

pub(crate) fn function112373_asm_11238c(cpu: &mut Cpu) {
    cpu.pc = 0x638c;
    let value = 0xe0;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x11;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become function112373_asm_112393(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    become function112373_asm_112393(cpu);
}

pub(crate) fn function112373_asm_112393(cpu: &mut Cpu) {
    cpu.pc = 0x6393;
    cpu.a = 0x3;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc86b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(16);
    become mobile_end_session(cpu);
}

pub(crate) fn function112373_asm_11239b(cpu: &mut Cpu) {
    cpu.pc = 0x639b;
    cpu.set_hl(0xc822);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((4) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value |= mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.a = 0x2;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc86a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xc821);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((0) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    let mask = 1u8 << ((6) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value |= mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function112373_asm_1123ad(cpu: &mut Cpu) {
    cpu.pc = 0x63ad;
    cpu.a = cpu.read_byte(0xc872);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11225d(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(16);
    become function1116a4(cpu);
}

pub(crate) fn function1123b6(cpu: &mut Cpu) {
    cpu.pc = 0x63b6;
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function1123b6_wait_for_call(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function1123b6_asm_1123c6(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function1123b6_asm_1123bd(cpu: &mut Cpu) {
    cpu.pc = 0x63bd;
    let addr = cpu.hl();
    let value = cpu.read_byte(addr);
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.write_byte(addr, result);
    cpu.pc += 1;
    cpu.cycle(12);
    become function1123b6_wait_for_call(cpu);
}

pub(crate) fn function1123b6_wait_for_call(cpu: &mut Cpu) {
    cpu.pc = 0x63be;
    cpu.a = { let value = ((MOBILE_COMMAND_WAIT_FOR_TELEPHONE_CALL as i32) | (0x80 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x6068);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become packet_send_empty_body(cpu);
}

pub(crate) fn function1123b6_asm_1123c6(cpu: &mut Cpu) {
    cpu.pc = 0x63c6;
    cpu.a = cpu.read_byte(0xca3c);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0xee;
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
        become function1123b6_asm_1123bd(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xc822);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((4) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value |= mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.a = 0x2;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc86a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xc821);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((0) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    let mask = 1u8 << ((6) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value |= mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    let mask = 1u8 << ((5) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value |= mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function1123e1(cpu: &mut Cpu) {
    cpu.pc = 0x63e1;
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function1123e1_asm_1123f2(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function1123e1_asm_11240a(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function1123e1_asm_112416(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function1123e1_asm_11241e(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function1123e1_asm_112421(cpu);
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
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function1123e1_asm_1123f2(cpu: &mut Cpu) {
    cpu.pc = 0x63f2;
    cpu.a = cpu.read_byte(0xca3c);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x9f;
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
        become function1123e1_asm_112408(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function1127e1(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_z {
        cpu.cycle(12);
        become function1123e1_asm_112408(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xc86b);
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
    cpu.set_hl({ const N: i32 = (0xcb47 as i32) + (32 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become function1127c5(cpu);
}

pub(crate) fn function1123e1_asm_112408(cpu: &mut Cpu) {
    cpu.pc = 0x6408;
    cpu.cycle(12);
    become function112430(cpu);
}

pub(crate) fn function1123e1_asm_11240a(cpu: &mut Cpu) {
    cpu.pc = 0x640a;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc86d, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = { let value = ((MOBILE_COMMAND_ISP_LOGOUT as i32) | (0x80 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x603c);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become packet_send_empty_body(cpu);
}

pub(crate) fn function1123e1_asm_112416(cpu: &mut Cpu) {
    cpu.pc = 0x6416;
    cpu.a = { let value = ((MOBILE_COMMAND_HANG_UP_TELEPHONE as i32) | (0x80 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x6023);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become packet_send_empty_body(cpu);
}

pub(crate) fn function1123e1_asm_11241e(cpu: &mut Cpu) {
    cpu.pc = 0x641e;
    cpu.cycle(16);
    become mobile_end_session(cpu);
}

pub(crate) fn function1123e1_asm_112421(cpu: &mut Cpu) {
    cpu.pc = 0x6421;
    cpu.set_hl(0xc822);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((4) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.set_hl(0xc821);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0xf;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.cycle(16);
    become function1116a0(cpu);
}

pub(crate) fn function112430(cpu: &mut Cpu) {
    cpu.pc = 0x6430;
    cpu.a = 0x3;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc807, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_de(0xcb47);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0x6083);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = { let value = ((0x6089 as i32) - (0x6083 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xc86c);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.b;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.b = result;
    cpu.pc += 1;
    cpu.cycle(4);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function111f63(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0xa4;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xcb47);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become function111f02(cpu);
}

pub(crate) fn function112451(cpu: &mut Cpu) {
    cpu.pc = 0x6451;
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function112451_asm_112458(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function112451_asm_112496(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function112451_asm_112458(cpu: &mut Cpu) {
    cpu.pc = 0x6458;
    cpu.b = 0x6;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_de({ const N: i32 = (0xcb47 as i32) + (92 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function111f63(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xc86e);
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
    let value = 0x3;
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
        become function112451_asm_11248b(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte({ const N: i32 = (0xca3c as i32) + (128 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.a;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function112451_asm_11248b(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xc995);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0x99;
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
        become function112451_asm_112480(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0x66;
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
        become function112451_asm_112480(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0x23;
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
        become function112451_asm_11248b(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become function112451_asm_112480(cpu);
}

pub(crate) fn function112451_asm_112480(cpu: &mut Cpu) {
    cpu.pc = 0x6480;
    cpu.set_hl({ const N: i32 = (0xcb47 as i32) + (80 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xc995);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = 0x10;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    become function112451_asm_11248b(cpu);
}

pub(crate) fn function112451_asm_11248b(cpu: &mut Cpu) {
    cpu.pc = 0x648b;
    cpu.a = 0xa3;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_de(0x0010 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl({ const N: i32 = (0xcb47 as i32) + (80 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become function111f02(cpu);
}

pub(crate) fn function112451_asm_112496(cpu: &mut Cpu) {
    cpu.pc = 0x6496;
    cpu.a = cpu.read_byte(0xca3c);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0xa3;
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
        become function112451_asm_1124ce(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc822);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((3) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become function112451_asm_1124ab(cpu);
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
    cpu.a = 0x3;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc807, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function112451_asm_1124ab(cpu: &mut Cpu) {
    cpu.pc = 0x64ab;
    cpu.a = cpu.read_byte(0xc9af);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x5;
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
        become function112451_asm_1124b8(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xc821);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((1) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value |= mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function112451_asm_1124b8(cpu: &mut Cpu) {
    cpu.pc = 0x64b8;
    let addr = cpu.hl();
    let value = cpu.read_byte(addr);
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.write_byte(addr, result);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.set_hl(0xc9af);
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
    cpu.set_hl(0xc822);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((3) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value |= mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.set_hl(0xc815);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xc820);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc81f);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function112451_asm_1124ce(cpu: &mut Cpu) {
    cpu.pc = 0x64ce;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc9af, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xc86e);
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
    cpu.write_byte(0xc86d, cpu.a);
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
    if cpu.flag_z {
        cpu.cycle(16);
        become function11261c(cpu);
    }
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
    if cpu.flag_z {
        cpu.cycle(16);
        become function112654(cpu);
    }
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
    if cpu.flag_z {
        cpu.cycle(16);
        become function112597(cpu);
    }
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
    if cpu.flag_z {
        cpu.cycle(16);
        become function112566(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function1125c7(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.push_word(cpu.de());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.set_de(0xc880);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0xc827);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc86e, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0xfa;
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
    let value = cpu.pop_word();
    cpu.set_de(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc994, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11269b(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x5;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc86b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function112534(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xc9a5);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.a;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function112451_asm_112521(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    become function112451_asm_112521(cpu);
}

pub(crate) fn function112451_asm_112521(cpu: &mut Cpu) {
    cpu.pc = 0x6521;
    let value = 0x23;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_add(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) > 0x0f;
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc86a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xc98a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x2;
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
        become function112451_asm_112531(cpu);
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
    cpu.write_byte(0xc9a5, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    become function112451_asm_112531(cpu);
}

pub(crate) fn function112451_asm_112531(cpu: &mut Cpu) {
    cpu.pc = 0x6531;
    cpu.cycle(16);
    become function1125bf(cpu);
}

pub(crate) fn function112534(cpu: &mut Cpu) {
    cpu.pc = 0x6534;
    cpu.b = 0xfa;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xc880);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    become function112534_asm_11253a(cpu);
}

pub(crate) fn function112534_asm_11253a(cpu: &mut Cpu) {
    cpu.pc = 0x653a;
    cpu.write_byte(cpu.hl(), cpu.a);
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
        become function112534_asm_11253a(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc876);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xc87c, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xc877);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xc87d, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xc87a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xc87e, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xc87b);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xc87f, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte({ const N: i32 = (0xcb47 as i32) + (17 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.b = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function111f63(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x95;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl({ const N: i32 = (0xcb47 as i32) + (12 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become function111f02(cpu);
}

pub(crate) fn function112566(cpu: &mut Cpu) {
    cpu.pc = 0x6566;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function1125c7(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte({ const N: i32 = (0xcb47 as i32) + (19 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x1;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.a;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(12);
        become function112566_asm_11257d(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc98a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x2;
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
        become function112566_asm_11257d(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc994, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    become function112566_asm_11257d(cpu);
}

pub(crate) fn function112566_asm_11257d(cpu: &mut Cpu) {
    cpu.pc = 0x657d;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11269b(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x5;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc86b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function112534(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xc9a5);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.a;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function112566_asm_112590(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    become function112566_asm_112590(cpu);
}

pub(crate) fn function112566_asm_112590(cpu: &mut Cpu) {
    cpu.pc = 0x6590;
    let value = 0x21;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_add(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) > 0x0f;
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc86a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(12);
    become function1125bf(cpu);
}

pub(crate) fn function112597(cpu: &mut Cpu) {
    cpu.pc = 0x6597;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function1125c7(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11269b(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x5;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc86b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function112534(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xc98f);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xc994);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x1;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x13;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_add(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) > 0x0f;
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    let mask = 1u8 << ((0) & 7);
    cpu.flag_z = (cpu.b & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become function112597_asm_1125bc(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x13;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x1f;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_add(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) > 0x0f;
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.b;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.b = result;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.b;
    let result = value << 1;
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x80 != 0;
    cpu.b = result;
    cpu.pc += 2;
    cpu.cycle(8);
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
    become function112597_asm_1125bc(cpu);
}

pub(crate) fn function112597_asm_1125bc(cpu: &mut Cpu) {
    cpu.pc = 0x65bc;
    cpu.write_byte(0xc86a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    become function1125bf(cpu);
}

pub(crate) fn function1125bf(cpu: &mut Cpu) {
    cpu.pc = 0x65bf;
    cpu.set_hl(0xc821);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((0) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value |= mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    let mask = 1u8 << ((2) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function1125c7(cpu: &mut Cpu) {
    cpu.pc = 0x65c7;
    cpu.set_hl(0xc872);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
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
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.d = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc994);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x1;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x1;
    cpu.a ^= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc86b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xc827);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = ((0xc880u32) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = (((0xc880u32) >> 8) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_bc(cpu.bc().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_bc(cpu.bc().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 0xfa;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 0x0;
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
    cpu.set_de(0xcb47);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0x6072);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = 0x6;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xc86c);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.b = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function111f63(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_de({ const N: i32 = (0xcb47 as i32) + (12 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0x6072);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = 0x5;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc86c);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function11261c(cpu: &mut Cpu) {
    cpu.pc = 0x661c;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc86b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xc86c);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte({ const N: i32 = (0xcb47 as i32) + (118 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_de({ const N: i32 = (0xcb47 as i32) + (102 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.b = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function111f63(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function112724(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte({ const N: i32 = (0xcb47 as i32) + (117 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_de({ const N: i32 = (0xcb47 as i32) + (118 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    let value = cpu.e;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_add(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) > 0x0f;
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = 0x0;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.d;
    let carry_in = u8::from(cpu.flag_c);
    let lhs = cpu.a;
    let (partial, carry1) = lhs.overflowing_add(value);
    let (result, carry2) = partial.overflowing_add(carry_in);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) + carry_in > 0x0f;
    cpu.flag_c = carry1 || carry2;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.d = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function111f63(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = (0xcb47 as i32) + (96 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function1127c5(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x11;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc86a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    become function11264e(cpu);
}

pub(crate) fn function11264e(cpu: &mut Cpu) {
    cpu.pc = 0x664e;
    cpu.set_hl(0xc821);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((0) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value |= mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function112654(cpu: &mut Cpu) {
    cpu.pc = 0x6654;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc86b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xc86c);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte({ const N: i32 = (0xcb47 as i32) + (102 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte({ const N: i32 = (0xcb47 as i32) + (166 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_de({ const N: i32 = (0xcb47 as i32) + (134 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.b = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function111f63(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function112724(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte({ const N: i32 = (0xcb47 as i32) + (165 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_de({ const N: i32 = (0xcb47 as i32) + (166 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    let value = cpu.e;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_add(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) > 0x0f;
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = 0x0;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.d;
    let carry_in = u8::from(cpu.flag_c);
    let lhs = cpu.a;
    let (partial, carry1) = lhs.overflowing_add(value);
    let (result, carry2) = partial.overflowing_add(carry_in);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) + carry_in > 0x0f;
    cpu.flag_c = carry1 || carry2;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.d = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function111f63(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte({ const N: i32 = (0xcb47 as i32) + (101 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_de({ const N: i32 = (0xcb47 as i32) + (102 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    let value = cpu.e;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_add(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) > 0x0f;
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = 0x0;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.d;
    let carry_in = u8::from(cpu.flag_c);
    let lhs = cpu.a;
    let (partial, carry1) = lhs.overflowing_add(value);
    let (result, carry2) = partial.overflowing_add(carry_in);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) + carry_in > 0x0f;
    cpu.flag_c = carry1 || carry2;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.d = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function111f63(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = (0xcb47 as i32) + (128 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function1127c5(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x12;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc86a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(12);
    become function11264e(cpu);
}

pub(crate) fn function11269b(cpu: &mut Cpu) {
    cpu.pc = 0x669b;
    cpu.set_bc(0x0001 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0x6110);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xc994);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.a;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        let next_pc = cpu.pc + 3;
        cpu.push_word(next_pc);
        cpu.cycle(24);
        function1126ac(cpu);
        assert_eq!(cpu.pc, next_pc);
    } else {
        cpu.pc += 3;
        cpu.cycle(12);
    }
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_string(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function1126ac(cpu: &mut Cpu) {
    cpu.pc = 0x66ac;
    cpu.set_hl(0x6137);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function1126b0(cpu: &mut Cpu) {
    cpu.pc = 0x66b0;
    cpu.set_hl(0x6115);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become mobile_sdk_copy_string(cpu);
}

pub(crate) fn function1126b6(cpu: &mut Cpu) {
    cpu.pc = 0x66b6;
    cpu.set_hl(0x6121);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_string(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0x013f as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = 0x4;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x2d;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0x014c as u16);
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
    let value = cpu.a;
    let result = value.rotate_left(4);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x30;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0x014c as u16);
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
    let value = 0x30;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 0x7;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.c;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_add(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) > 0x0f;
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl(0x6132);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become mobile_sdk_copy_string(cpu);
}

pub(crate) fn function1126e6(cpu: &mut Cpu) {
    cpu.pc = 0x66e6;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc86b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0x613d);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_string(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xc9a5);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = 0x5;
    cpu.pc += 2;
    cpu.cycle(8);
    become function1126e6_asm_1126f5(cpu);
}

pub(crate) fn function1126e6_asm_1126f5(cpu: &mut Cpu) {
    cpu.pc = 0x66f5;
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0x30;
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
        become function1126e6_asm_112701(cpu);
    }
    cpu.pc += 2;
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
    cpu.a = 0x1;
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
        cpu.cycle(12);
        become function1126e6_asm_1126f5(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become function1126e6_asm_112701(cpu);
}

pub(crate) fn function1126e6_asm_112701(cpu: &mut Cpu) {
    cpu.pc = 0x6701;
    cpu.push_word(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0xd;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 0xa;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.pop_word();
    cpu.set_bc(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = 0x2;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_add(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) > 0x0f;
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.c;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_add(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) > 0x0f;
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.c = cpu.a;
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
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function112715(cpu: &mut Cpu) {
    cpu.pc = 0x6715;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc86c, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x2;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc86a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xc821);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((0) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function112724(cpu: &mut Cpu) {
    cpu.pc = 0x6724;
    cpu.a = 0xff;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc86e, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    become function112729(cpu);
}

pub(crate) fn function112729(cpu: &mut Cpu) {
    cpu.pc = 0x6729;
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.set_hl(0xc82c);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 0xff;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = (((0xc880u32) >> 8) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = ((0xc880u32) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function11273a(cpu: &mut Cpu) {
    cpu.pc = 0x673a;
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function11273a_asm_112752(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function11273a_asm_11278f(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function11273a_asm_112744(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function11273a_asm_112744(cpu: &mut Cpu) {
    cpu.pc = 0x6744;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc86d, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x30;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11225d(cpu);
    assert_eq!(cpu.pc, next_pc);
    let mask = 1u8 << ((1) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value |= mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    let mask = 1u8 << ((0) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function11273a_asm_112752(cpu: &mut Cpu) {
    cpu.pc = 0x6752;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function1127e1(cpu);
    assert_eq!(cpu.pc, next_pc);
    if !cpu.flag_z {
        cpu.cycle(12);
        become function11273a_asm_11277a(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xc880);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function112b11(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x2;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.d;
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
        become function11273a_asm_1127b7(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x20;
    cpu.pc += 2;
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
    if !cpu.flag_z {
        cpu.cycle(12);
        become function11273a_asm_1127b7(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function112724(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte({ const N: i32 = (0xcb47 as i32) + (117 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0xa;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_add(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) > 0x0f;
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.d = 0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x95;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl({ const N: i32 = (0xcb47 as i32) + (112 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become function111f02(cpu);
}

pub(crate) fn function11273a_asm_11277a(cpu: &mut Cpu) {
    cpu.pc = 0x677a;
    cpu.a = cpu.read_byte(0xca3c);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x9f;
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
        become function1127cd(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xc86b);
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
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte({ const N: i32 = (0xca3c as i32) + (3 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl({ const N: i32 = (0xcb47 as i32) + (96 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become function1127c5(cpu);
}

pub(crate) fn function11273a_asm_11278f(cpu: &mut Cpu) {
    cpu.pc = 0x678f;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function1127e1(cpu);
    assert_eq!(cpu.pc, next_pc);
    if !cpu.flag_z {
        cpu.cycle(12);
        become function11273a_asm_11277a(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xc880);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function112b11(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x2;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.d;
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
        become function11273a_asm_1127b7(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x50;
    cpu.pc += 2;
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
    if !cpu.flag_z {
        cpu.cycle(12);
        become function11273a_asm_1127b7(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x3;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc86a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xc821);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0xd6;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x80;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc98a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function11273a_asm_1127b7(cpu: &mut Cpu) {
    cpu.pc = 0x67b7;
    cpu.set_hl(0xc810);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.d);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 0x2;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc86b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(16);
    become function112430(cpu);
}

pub(crate) fn function1127c5(cpu: &mut Cpu) {
    cpu.pc = 0x67c5;
    cpu.set_de(0x000b as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0x95;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become function111f02(cpu);
}

pub(crate) fn function1127cd(cpu: &mut Cpu) {
    cpu.pc = 0x67cd;
    cpu.set_hl(0xc810);
    cpu.pc += 3;
    cpu.cycle(12);
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
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc86d, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x30;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11225d(cpu);
    assert_eq!(cpu.pc, next_pc);
    let mask = 1u8 << ((1) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value |= mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    let mask = 1u8 << ((0) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function1127e1(cpu: &mut Cpu) {
    cpu.pc = 0x67e1;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function112807(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xc832);
    cpu.pc += 3;
    cpu.cycle(12);
    become function1127e7(cpu);
}

pub(crate) fn function1127e7(cpu: &mut Cpu) {
    cpu.pc = 0x67e7;
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0xd;
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
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0xa;
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
    cpu.a = 0x20;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function1127f3(cpu: &mut Cpu) {
    cpu.pc = 0x67f3;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function112807(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xc82f);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0xd;
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
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0xa;
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
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0x2e;
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
    cpu.cycle(12);
    become function1127e7(cpu);
}

pub(crate) fn function112807(cpu: &mut Cpu) {
    cpu.pc = 0x6807;
    cpu.push_word(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.push_word(cpu.de());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.set_hl({ const N: i32 = (0xca3c as i32) + (3 as i32); N as u16 });
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
    if cpu.flag_z {
        cpu.cycle(12);
        become function112807_asm_11282d(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = 0x5;
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
        become function112807_asm_112830(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x5;
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
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.e = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.d = 0x0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xc82f);
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
    cpu.set_de(0xc82f);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = (0xca3c as i32) + (5 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    become function112807_asm_11282a(cpu);
}

pub(crate) fn function112807_asm_11282a(cpu: &mut Cpu) {
    cpu.pc = 0x682a;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    become function112807_asm_11282d(cpu);
}

pub(crate) fn function112807_asm_11282d(cpu: &mut Cpu) {
    cpu.pc = 0x682d;
    let value = cpu.pop_word();
    cpu.set_de(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let value = cpu.pop_word();
    cpu.set_bc(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function112807_asm_112830(cpu: &mut Cpu) {
    cpu.pc = 0x6830;
    let value = 0x5;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.b = 0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl({ const N: i32 = (0xca3c as i32) + (5 as i32); N as u16 });
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
    cpu.b = 0x5;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_de(0xc82f);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(12);
    become function112807_asm_11282a(cpu);
}

pub(crate) fn function112840(cpu: &mut Cpu) {
    cpu.pc = 0x6840;
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function112840_asm_112844(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function112840_asm_112844(cpu: &mut Cpu) {
    cpu.pc = 0x6844;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function1127e1(cpu);
    assert_eq!(cpu.pc, next_pc);
    if !cpu.flag_z {
        cpu.cycle(12);
        become function112840_asm_1128ab(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xc880);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0x32;
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
        become function1128bd(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0x35;
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
        become function1128bd(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function112724(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xc87c);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.h = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.a;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function112840_asm_11289d(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.set_hl(0xc86b);
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
    cpu.set_bc(0x0001 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de({ const N: i32 = (0xcb47 as i32) + (19 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0x60b0);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_string(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.a = 0x80;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_string_len(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x3e;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.c;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.c = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc87c, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc87d, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11295e(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte({ const N: i32 = (0xcb47 as i32) + (17 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.b = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function111f63(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = (0xcb47 as i32) + (12 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.d = 0x0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.e = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = 0x95;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become function111f02(cpu);
}

pub(crate) fn function112840_asm_11289d(cpu: &mut Cpu) {
    cpu.pc = 0x689d;
    cpu.a = 0x3;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc86a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function1128d3(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc98a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function112840_asm_1128ab(cpu: &mut Cpu) {
    cpu.pc = 0x68ab;
    cpu.a = cpu.read_byte(0xca3c);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x9f;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(16);
        become function1127cd(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0xc86b);
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
    cpu.set_hl(0xcb47);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become function1127c5(cpu);
}

pub(crate) fn function1128bd(cpu: &mut Cpu) {
    cpu.pc = 0x68bd;
    cpu.set_hl(0xc880);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function112b11(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xc810);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.d);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 0x30;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11225d(cpu);
    assert_eq!(cpu.pc, next_pc);
    let mask = 1u8 << ((1) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value |= mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    let mask = 1u8 << ((0) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function1128d3(cpu: &mut Cpu) {
    cpu.pc = 0x68d3;
    cpu.set_hl(0xc821);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((0) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    let mask = 1u8 << ((2) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function1128db(cpu: &mut Cpu) {
    cpu.pc = 0x68db;
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function1128db_asm_112947(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function1128db_asm_1128e5(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function1128db_asm_112913(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function1128db_asm_1128e5(cpu: &mut Cpu) {
    cpu.pc = 0x68e5;
    cpu.a = cpu.read_byte(0xca3c);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x9f;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(16);
        become function1127cd(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function113482(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte({ const N: i32 = (0xc86e as i32) + (1 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.a;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(12);
        become function1128db_asm_112901(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x3;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc86a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xc821);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((0) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function1128db_asm_112901(cpu: &mut Cpu) {
    cpu.pc = 0x6901;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function112724(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_de({ const N: i32 = (0xcb47 as i32) + (5 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.b = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function111f63(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(12);
    become function1128db_asm_112941(cpu);
}

pub(crate) fn function1128db_asm_112913(cpu: &mut Cpu) {
    cpu.pc = 0x6913;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function1127e1(cpu);
    assert_eq!(cpu.pc, next_pc);
    if !cpu.flag_z {
        cpu.cycle(12);
        become function1128db_asm_11293d(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xca3c);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x9f;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(16);
        become function1127cd(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0xc880);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function112b11(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = 0x2;
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
        become function1128db_asm_11295b(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = 0x50;
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
        become function1128db_asm_11295b(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x3;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc86a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function1128d3(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc98a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function1128db_asm_11293d(cpu: &mut Cpu) {
    cpu.pc = 0x693d;
    cpu.set_hl(0xc86b);
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
    become function1128db_asm_112941(cpu);
}

pub(crate) fn function1128db_asm_112941(cpu: &mut Cpu) {
    cpu.pc = 0x6941;
    cpu.set_hl(0xcb47);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become function1127c5(cpu);
}

pub(crate) fn function1128db_asm_112947(cpu: &mut Cpu) {
    cpu.pc = 0x6947;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function1127e1(cpu);
    assert_eq!(cpu.pc, next_pc);
    if !cpu.flag_z {
        cpu.cycle(12);
        become function1128db_asm_11293d(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xc880);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function112b11(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = 0x3;
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
        become function1128db_asm_11295b(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = 0x54;
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
    become function1128db_asm_11295b(cpu);
}

pub(crate) fn function1128db_asm_11295b(cpu: &mut Cpu) {
    cpu.pc = 0x695b;
    cpu.cycle(16);
    become function1128bd(cpu);
}

pub(crate) fn function11295e(cpu: &mut Cpu) {
    cpu.pc = 0x695e;
    cpu.a = 0xd;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.c;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.c = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = 0xa;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.c;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.c = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function112969(cpu: &mut Cpu) {
    cpu.pc = 0x6969;
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function112969_asm_112970(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function112969_asm_112989(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function112969_asm_112970(cpu: &mut Cpu) {
    cpu.pc = 0x6970;
    cpu.a = cpu.read_byte(0xca3c);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x9f;
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
        become function112969_asm_112986(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function1127e1(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_z {
        cpu.cycle(12);
        become function112969_asm_112986(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xc86b);
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
    cpu.set_hl({ const N: i32 = (0xcb47 as i32) + (32 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become function1127c5(cpu);
}

pub(crate) fn function112969_asm_112986(cpu: &mut Cpu) {
    cpu.pc = 0x6986;
    cpu.cycle(16);
    become function112430(cpu);
}

pub(crate) fn function112969_asm_112989(cpu: &mut Cpu) {
    cpu.pc = 0x6989;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc86d, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x2;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc86a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xc821);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((0) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    let mask = 1u8 << ((7) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    let mask = 1u8 << ((5) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value |= mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function11299c(cpu: &mut Cpu) {
    cpu.pc = 0x699c;
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function11299c_asm_1129aa(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function11299c_asm_1129c9(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function11299c_asm_1129e7(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(16);
        become function11299c_asm_112a1d(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function11299c_asm_1129aa(cpu: &mut Cpu) {
    cpu.pc = 0x69aa;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function1127e1(cpu);
    assert_eq!(cpu.pc, next_pc);
    if !cpu.flag_z {
        cpu.cycle(12);
        become function11299c_asm_1129fe(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc880);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x2b;
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
        become function11299c_asm_112a0f(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function112724(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte({ const N: i32 = (0xcb47 as i32) + (101 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0xa;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_add(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) > 0x0f;
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.d = 0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x95;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl({ const N: i32 = (0xcb47 as i32) + (96 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become function111f02(cpu);
}

pub(crate) fn function11299c_asm_1129c9(cpu: &mut Cpu) {
    cpu.pc = 0x69c9;
    cpu.d = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function1127e1(cpu);
    assert_eq!(cpu.pc, next_pc);
    if !cpu.flag_z {
        cpu.cycle(12);
        become function11299c_asm_1129fe(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc880);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x2b;
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
        become function11299c_asm_112a0f(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function112724(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte({ const N: i32 = (0xcb47 as i32) + (165 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0xa;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_add(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) > 0x0f;
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = 0x95;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl({ const N: i32 = (0xcb47 as i32) + (160 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become function111f02(cpu);
}

pub(crate) fn function11299c_asm_1129e7(cpu: &mut Cpu) {
    cpu.pc = 0x69e7;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function1127e1(cpu);
    assert_eq!(cpu.pc, next_pc);
    if !cpu.flag_z {
        cpu.cycle(12);
        become function11299c_asm_1129fe(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc880);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x2b;
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
        become function11299c_asm_112a0f(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x4;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc86a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function1128d3(cpu);
    assert_eq!(cpu.pc, next_pc);
    let mask = 1u8 << ((7) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value |= mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function11299c_asm_1129fe(cpu: &mut Cpu) {
    cpu.pc = 0x69fe;
    cpu.a = cpu.read_byte(0xca3c);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x9f;
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
        become function112a42(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xc86b);
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
    cpu.set_hl({ const N: i32 = (0xcb47 as i32) + (128 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become function1127c5(cpu);
}

pub(crate) fn function11299c_asm_112a0f(cpu: &mut Cpu) {
    cpu.pc = 0x6a0f;
    cpu.a = cpu.read_byte(0xc86b);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte({ const N: i32 = (0xcb47 as i32) + (32 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x3;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc86b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(16);
    become function112430(cpu);
}

pub(crate) fn function11299c_asm_112a1d(cpu: &mut Cpu) {
    cpu.pc = 0x6a1d;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc86d, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_de(0x0002 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte({ const N: i32 = (0xcb47 as i32) + (32 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x1;
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
        become function11299c_asm_112a2c(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    become function11299c_asm_112a2c(cpu);
}

pub(crate) fn function11299c_asm_112a2c(cpu: &mut Cpu) {
    cpu.pc = 0x6a2c;
    become function112a2c(cpu);
}

pub(crate) fn function112a2c(cpu: &mut Cpu) {
    cpu.pc = 0x6a2c;
    cpu.set_hl(0xc821);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((1) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value |= mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    let mask = 1u8 << ((0) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.set_hl(0xc80f);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0x31;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.d);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 0x5;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc86a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function112a42(cpu: &mut Cpu) {
    cpu.pc = 0x6a42;
    cpu.set_hl(0xc810);
    cpu.pc += 3;
    cpu.cycle(12);
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
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc86d, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x31;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11225d(cpu);
    assert_eq!(cpu.pc, next_pc);
    let mask = 1u8 << ((1) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value |= mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    let mask = 1u8 << ((0) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function112a56(cpu: &mut Cpu) {
    cpu.pc = 0x6a56;
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function112a56_asm_112a5a(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function112a56_asm_112a5a(cpu: &mut Cpu) {
    cpu.pc = 0x6a5a;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function1127e1(cpu);
    assert_eq!(cpu.pc, next_pc);
    if !cpu.flag_z {
        cpu.cycle(12);
        become function112a56_asm_112a95(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xc880);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0x2b;
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
        become function112a56_asm_112aa6(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become function112a56_asm_112a67(cpu);
}

pub(crate) fn function112a56_asm_112a67(cpu: &mut Cpu) {
    cpu.pc = 0x6a67;
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0x20;
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
        become function112a56_asm_112a67(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function112aac(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xc86e);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte({ const N: i32 = (0xc86e as i32) + (1 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.bc(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_bc(cpu.bc().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.bc(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function112aac(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xc86e);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.h = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 0x4;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc86a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(16);
    become function1128d3(cpu);
}

pub(crate) fn function112a56_asm_112a95(cpu: &mut Cpu) {
    cpu.pc = 0x6a95;
    cpu.a = cpu.read_byte(0xca3c);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x9f;
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
        become function112a42(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xc86b);
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
    cpu.set_hl({ const N: i32 = (0xcb47 as i32) + (128 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become function1127c5(cpu);
}

pub(crate) fn function112a56_asm_112aa6(cpu: &mut Cpu) {
    cpu.pc = 0x6aa6;
    cpu.set_de(0x0005 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become function112a2c(cpu);
}

pub(crate) fn function112aac(cpu: &mut Cpu) {
    cpu.pc = 0x6aac;
    cpu.a = cpu.read_byte(0xc872);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xc873);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xc874);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.set_bc(0 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    become function112aac_asm_112abe(cpu);
}

pub(crate) fn function112aac_asm_112abe(cpu: &mut Cpu) {
    cpu.pc = 0x6abe;
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0xd;
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
        become function112aac_asm_112b04(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x20;
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
        become function112aac_asm_112b04(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0xf;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.e;
    let result = value << 1;
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x80 != 0;
    cpu.e = result;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.d;
    let carry_in = u8::from(cpu.flag_c);
    let result = (value << 1) | carry_in;
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x80 != 0;
    cpu.d = result;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.c;
    let carry_in = u8::from(cpu.flag_c);
    let result = (value << 1) | carry_in;
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x80 != 0;
    cpu.c = result;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc872, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc873, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc874, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.e;
    let result = value << 1;
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x80 != 0;
    cpu.e = result;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.d;
    let carry_in = u8::from(cpu.flag_c);
    let result = (value << 1) | carry_in;
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x80 != 0;
    cpu.d = result;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.c;
    let carry_in = u8::from(cpu.flag_c);
    let result = (value << 1) | carry_in;
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x80 != 0;
    cpu.c = result;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.e;
    let result = value << 1;
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x80 != 0;
    cpu.e = result;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.d;
    let carry_in = u8::from(cpu.flag_c);
    let result = (value << 1) | carry_in;
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x80 != 0;
    cpu.d = result;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.c;
    let carry_in = u8::from(cpu.flag_c);
    let result = (value << 1) | carry_in;
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x80 != 0;
    cpu.c = result;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc872);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.e;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_add(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) > 0x0f;
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xc873);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.d;
    let carry_in = u8::from(cpu.flag_c);
    let lhs = cpu.a;
    let (partial, carry1) = lhs.overflowing_add(value);
    let (result, carry2) = partial.overflowing_add(carry_in);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) + carry_in > 0x0f;
    cpu.flag_c = carry1 || carry2;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.d = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xc874);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.c;
    let carry_in = u8::from(cpu.flag_c);
    let lhs = cpu.a;
    let (partial, carry1) = lhs.overflowing_add(value);
    let (result, carry2) = partial.overflowing_add(carry_in);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) + carry_in > 0x0f;
    cpu.flag_c = carry1 || carry2;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.e;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_add(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) > 0x0f;
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = 0x0;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.d;
    let carry_in = u8::from(cpu.flag_c);
    let lhs = cpu.a;
    let (partial, carry1) = lhs.overflowing_add(value);
    let (result, carry2) = partial.overflowing_add(carry_in);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) + carry_in > 0x0f;
    cpu.flag_c = carry1 || carry2;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.d = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = 0x0;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.c;
    let carry_in = u8::from(cpu.flag_c);
    let lhs = cpu.a;
    let (partial, carry1) = lhs.overflowing_add(value);
    let (result, carry2) = partial.overflowing_add(carry_in);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) + carry_in > 0x0f;
    cpu.flag_c = carry1 || carry2;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.cycle(12);
    become function112aac_asm_112abe(cpu);
}

pub(crate) fn function112aac_asm_112b04(cpu: &mut Cpu) {
    cpu.pc = 0x6b04;
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte(0xc874, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte(0xc873, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte(0xc872, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function112b11(cpu: &mut Cpu) {
    cpu.pc = 0x6b11;
    cpu.a = cpu.read_byte(0xc872);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xc873);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xc874);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.set_bc(0x0300 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xc872);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function112b11_asm_112b60(cpu);
    assert_eq!(cpu.pc, next_pc);
    if !cpu.flag_c {
        let next_pc = cpu.pc + 3;
        cpu.push_word(next_pc);
        cpu.cycle(24);
        function112b11_asm_112b60(cpu);
        assert_eq!(cpu.pc, next_pc);
    } else {
        cpu.pc += 3;
        cpu.cycle(12);
    }
    if !cpu.flag_c {
        let next_pc = cpu.pc + 3;
        cpu.push_word(next_pc);
        cpu.cycle(24);
        function112b11_asm_112b60(cpu);
        assert_eq!(cpu.pc, next_pc);
    } else {
        cpu.pc += 3;
        cpu.cycle(12);
    }
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    become function112b11_asm_112b2d(cpu);
}

pub(crate) fn function112b11_asm_112b2d(cpu: &mut Cpu) {
    cpu.pc = 0x6b2d;
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0x0d;
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
        become function112b11_asm_112b36(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = { let value = (0x7f as i32); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
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
        become function112b11_asm_112b2d(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become function112b11_asm_112b36(cpu);
}

pub(crate) fn function112b11_asm_112b36(cpu: &mut Cpu) {
    cpu.pc = 0x6b36;
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.set_hl(0xc872);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.a;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function112b11_asm_112b49(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function112b11_asm_112b4b(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function112b11_asm_112b4f(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become function112b11_asm_112b52(cpu);
}

pub(crate) fn function112b11_asm_112b49(cpu: &mut Cpu) {
    cpu.pc = 0x6b49;
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.d = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    become function112b11_asm_112b4b(cpu);
}

pub(crate) fn function112b11_asm_112b4b(cpu: &mut Cpu) {
    cpu.pc = 0x6b4b;
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value.rotate_left(4);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    become function112b11_asm_112b4f(cpu);
}

pub(crate) fn function112b11_asm_112b4f(cpu: &mut Cpu) {
    cpu.pc = 0x6b4f;
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.e;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    become function112b11_asm_112b52(cpu);
}

pub(crate) fn function112b11_asm_112b52(cpu: &mut Cpu) {
    cpu.pc = 0x6b52;
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte(0xc874, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte(0xc873, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte(0xc872, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function112b11_asm_112b60(cpu: &mut Cpu) {
    cpu.pc = 0x6b60;
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = { let value = (0xf6 as i32); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
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
        become function112b11_asm_112b6f(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = { let value = ((0xff as i32) + (1 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
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
        become function112b11_asm_112b6f(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0xf;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
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
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function112b11_asm_112b6f(cpu: &mut Cpu) {
    cpu.pc = 0x6b6f;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = true;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function112b71(cpu: &mut Cpu) {
    cpu.pc = 0x6b71;
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function112b71_asm_112b75(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function112b71_asm_112b75(cpu: &mut Cpu) {
    cpu.pc = 0x6b75;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function1127e1(cpu);
    assert_eq!(cpu.pc, next_pc);
    if !cpu.flag_z {
        cpu.cycle(12);
        become function112b71_asm_112ba3(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xc880);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0x2b;
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
        become function112b71_asm_112bb5(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become function112b71_asm_112b82(cpu);
}

pub(crate) fn function112b71_asm_112b82(cpu: &mut Cpu) {
    cpu.pc = 0x6b82;
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0x20;
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
        become function112b71_asm_112b82(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become function112b71_asm_112b87(cpu);
}

pub(crate) fn function112b71_asm_112b87(cpu: &mut Cpu) {
    cpu.pc = 0x6b87;
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0x20;
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
        become function112b71_asm_112b87(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function112aac(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xc86e);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.h = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 0x4;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc86a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(16);
    become function1128d3(cpu);
}

pub(crate) fn function112b71_asm_112ba3(cpu: &mut Cpu) {
    cpu.pc = 0x6ba3;
    cpu.a = cpu.read_byte(0xca3c);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x9f;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(16);
        become function112a42(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0xc86b);
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
    cpu.set_hl({ const N: i32 = (0xcb47 as i32) + (128 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become function1127c5(cpu);
}

pub(crate) fn function112b71_asm_112bb5(cpu: &mut Cpu) {
    cpu.pc = 0x6bb5;
    cpu.set_de(0x0004 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become function112a2c(cpu);
}

pub(crate) fn function112bbb(cpu: &mut Cpu) {
    cpu.pc = 0x6bbb;
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function112bbb_asm_112bbf(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function112bbb_asm_112bbf(cpu: &mut Cpu) {
    cpu.pc = 0x6bbf;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function1127e1(cpu);
    assert_eq!(cpu.pc, next_pc);
    if !cpu.flag_z {
        cpu.cycle(12);
        become function112bbb_asm_112bd4(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xc880);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0x2b;
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
        become function112bbb_asm_112be6(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x4;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc86a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(16);
    become function1128d3(cpu);
}

pub(crate) fn function112bbb_asm_112bd4(cpu: &mut Cpu) {
    cpu.pc = 0x6bd4;
    cpu.a = cpu.read_byte(0xca3c);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x9f;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(16);
        become function112a42(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0xc86b);
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
    cpu.set_hl({ const N: i32 = (0xcb47 as i32) + (128 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become function1127c5(cpu);
}

pub(crate) fn function112bbb_asm_112be6(cpu: &mut Cpu) {
    cpu.pc = 0x6be6;
    cpu.set_de(0x0004 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become function112a2c(cpu);
}

pub(crate) fn function112bec(cpu: &mut Cpu) {
    cpu.pc = 0x6bec;
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function112bec_asm_112bf7(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(16);
        become function112bec_asm_112cdb(cpu);
    }
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
    if !cpu.flag_z {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
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
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function112bec_asm_112bf7(cpu: &mut Cpu) {
    cpu.pc = 0x6bf7;
    cpu.a = cpu.read_byte(0xc880);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x2d;
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
        become function112bec_asm_112c03(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function1127e1(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_z {
        cpu.cycle(12);
        become function112bec_asm_112c0b(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become function112bec_asm_112c03(cpu);
}

pub(crate) fn function112bec_asm_112c03(cpu: &mut Cpu) {
    cpu.pc = 0x6c03;
    cpu.a = cpu.read_byte(0xc821);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((2) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(16);
        become function112bec_asm_112cef(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    become function112bec_asm_112c0b(cpu);
}

pub(crate) fn function112bec_asm_112c0b(cpu: &mut Cpu) {
    cpu.pc = 0x6c0b;
    cpu.set_hl(0xc86b);
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
    cpu.set_hl(0xc880);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0x2b;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(16);
        become function112d20(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = 0x7f;
    cpu.pc += 2;
    cpu.cycle(8);
    become function112bec_asm_112c1a(cpu);
}

pub(crate) fn function112bec_asm_112c1a(cpu: &mut Cpu) {
    cpu.pc = 0x6c1a;
    cpu.a = cpu.read_byte(cpu.hl());
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
    let value = 0xa;
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
        become function112bec_asm_112c1a(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.set_hl(0xc98f);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.d = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc82d, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.h = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.b;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = 0x0;
    let carry_in = u8::from(cpu.flag_c);
    let lhs = cpu.a;
    let (partial, borrow1) = lhs.overflowing_sub(value);
    let (result, borrow2) = partial.overflowing_sub(carry_in);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < ((value & 0x0f) + carry_in);
    cpu.flag_c = borrow1 || borrow2;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.h = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_c {
        cpu.cycle(12);
        become function112bec_asm_112c56(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0xff;
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
        become function112bec_asm_112c56(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xc991);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
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
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc82b);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(0xc827);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.h = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
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
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.b = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.cycle(16);
    become mobile_sdk_copy_bytes(cpu);
}

pub(crate) fn function112bec_asm_112c56(cpu: &mut Cpu) {
    cpu.pc = 0x6c56;
    cpu.write_byte(0xc993, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xc82b);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc994, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
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
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = 0x0;
    let carry_in = u8::from(cpu.flag_c);
    let lhs = cpu.a;
    let (partial, borrow1) = lhs.overflowing_sub(value);
    let (result, borrow2) = partial.overflowing_sub(carry_in);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < ((value & 0x0f) + carry_in);
    cpu.flag_c = borrow1 || borrow2;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.h = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_c {
        cpu.cycle(12);
        become function112bec_asm_112c9f(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0xff;
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
        become function112bec_asm_112c9f(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte({ const N: i32 = (0xca3c as i32) + (1 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte({ const N: i32 = (0xca3c as i32) + (3 as i32); N as u16 });
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
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.c = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.push_word(cpu.de());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.set_hl({ const N: i32 = (0xca3c as i32) + (4 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
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
    let value = cpu.pop_word();
    cpu.set_de(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.b = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte({ const N: i32 = (0xca3c as i32) + (1 as i32); N as u16 });
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
    cpu.write_byte(0xc994, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xc827);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.h = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xc991);
    cpu.pc += 3;
    cpu.cycle(16);
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
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function112bec_asm_112c9f(cpu: &mut Cpu) {
    cpu.pc = 0x6c9f;
    cpu.write_byte(0xc994, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc82b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc82c, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte({ const N: i32 = (0xca3c as i32) + (3 as i32); N as u16 });
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
    cpu.push_word(cpu.de());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.set_hl({ const N: i32 = (0xca3c as i32) + (4 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
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
    let value = cpu.pop_word();
    cpu.set_de(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.b = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xc82d);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.c;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_add(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) > 0x0f;
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc82d, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xc82e);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0;
    let carry_in = u8::from(cpu.flag_c);
    let lhs = cpu.a;
    let (partial, carry1) = lhs.overflowing_add(value);
    let (result, carry2) = partial.overflowing_add(carry_in);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) + carry_in > 0x0f;
    cpu.flag_c = carry1 || carry2;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc82e, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xc829);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(0xc821);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((2) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    become function112bec_asm_112cdb(cpu);
}

pub(crate) fn function112bec_asm_112cdb(cpu: &mut Cpu) {
    cpu.pc = 0x6cdb;
    cpu.a = cpu.read_byte(0xc821);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((2) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become function112bec_asm_112cea(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x2;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc86b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(16);
    become function112bec_asm_112d09(cpu);
}

pub(crate) fn function112bec_asm_112cea(cpu: &mut Cpu) {
    cpu.pc = 0x6cea;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function1127f3(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_z {
        cpu.cycle(12);
        become function112bec_asm_112d01(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become function112bec_asm_112cef(cpu);
}

pub(crate) fn function112bec_asm_112cef(cpu: &mut Cpu) {
    cpu.pc = 0x6cef;
    cpu.a = cpu.read_byte(0xca3c);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x9f;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(16);
        become function112a42(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0xc86b);
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
    cpu.set_hl({ const N: i32 = (0xcb47 as i32) + (128 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become function1127c5(cpu);
}

pub(crate) fn function112bec_asm_112d01(cpu: &mut Cpu) {
    cpu.pc = 0x6d01;
    cpu.a = 0x4;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc86a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function1128d3(cpu);
    assert_eq!(cpu.pc, next_pc);
    become function112bec_asm_112d09(cpu);
}

pub(crate) fn function112bec_asm_112d09(cpu: &mut Cpu) {
    cpu.pc = 0x6d09;
    cpu.a = cpu.read_byte(0xc86e);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte({ const N: i32 = (0xc86e as i32) + (1 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.l;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
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
    cpu.set_hl(0xc827);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.d = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(0xc82d);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = 0x2;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become mobile_sdk_copy_bytes(cpu);
}

pub(crate) fn function112d20(cpu: &mut Cpu) {
    cpu.pc = 0x6d20;
    cpu.a = cpu.read_byte(0xc86a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x1a;
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
        become function112d20_asm_112d2d(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_de(0x0004 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become function112a2c(cpu);
}

pub(crate) fn function112d20_asm_112d2d(cpu: &mut Cpu) {
    cpu.pc = 0x6d2d;
    cpu.set_de(0x0004 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become function112a2c(cpu);
}

pub(crate) fn function112d33(cpu: &mut Cpu) {
    cpu.pc = 0x6d33;
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function112d33_asm_112d87(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function112d33_asm_112d9f(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(16);
        become function112d33_asm_112e46(cpu);
    }
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
    if cpu.flag_z {
        cpu.cycle(12);
        become function112d33_asm_112d4d(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(16);
        become function113317(cpu);
    }
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
    if cpu.flag_z {
        cpu.cycle(16);
        become function113386(cpu);
    }
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
    cpu.cycle(16);
    become function112d33_asm_112e46(cpu);
}

pub(crate) fn function112d33_asm_112d4c(cpu: &mut Cpu) {
    cpu.pc = 0x6d4c;
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function112d33_asm_112d4d(cpu: &mut Cpu) {
    cpu.pc = 0x6d4d;
    cpu.a = cpu.read_byte(0xc86a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x23;
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
        become function112d33_asm_112d6d(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x1f;
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
        become function112d33_asm_112d60(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x20;
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
        become function112d33_asm_112d6d(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x22;
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
        become function112d33_asm_112d82(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become function112d33_asm_112d60(cpu);
}

pub(crate) fn function112d33_asm_112d60(cpu: &mut Cpu) {
    cpu.pc = 0x6d60;
    cpu.set_hl(0xc98b);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0x1;
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
        become function112d33_asm_112d82(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0x4;
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
        become function112d33_asm_112d82(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become function112d33_asm_112d6d(cpu);
}

pub(crate) fn function112d33_asm_112d6d(cpu: &mut Cpu) {
    cpu.pc = 0x6d6d;
    cpu.set_hl(0xc86e);
    cpu.pc += 3;
    cpu.cycle(12);
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
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(0xc82b);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(0xc821);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((2) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.set_hl(0xc86b);
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
    let addr = cpu.hl();
    let value = cpu.read_byte(addr);
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.write_byte(addr, result);
    cpu.pc += 1;
    cpu.cycle(12);
    become function112d33_asm_112d82(cpu);
}

pub(crate) fn function112d33_asm_112d82(cpu: &mut Cpu) {
    cpu.pc = 0x6d82;
    cpu.set_hl(0xc86b);
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
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function112d33_asm_112d87(cpu: &mut Cpu) {
    cpu.pc = 0x6d87;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function113482(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_de(0xcb47);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0x6072);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = 0x6;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xc86c);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.b = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function111f63(cpu);
    assert_eq!(cpu.pc, next_pc);
    become function112d33_asm_112d9f(cpu);
}

pub(crate) fn function112d33_asm_112d9f(cpu: &mut Cpu) {
    cpu.pc = 0x6d9f;
    cpu.a = cpu.read_byte(0xc821);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((2) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become function112d33_asm_112dab(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x3;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.cycle(12);
    become function112d33_asm_112dc1(cpu);
}

pub(crate) fn function112d33_asm_112dab(cpu: &mut Cpu) {
    cpu.pc = 0x6dab;
    cpu.a = cpu.read_byte(0xca3c);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = { let value = ((MOBILE_COMMAND_TRANSFER_DATA_END as i32) | (0x80 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
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
        become function112d33_asm_112dc1(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xc86b);
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
    cpu.set_de(0x000b as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0xcb47);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = 0x5;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become packet_send_bytes(cpu);
}

pub(crate) fn function112d33_asm_112dc1(cpu: &mut Cpu) {
    cpu.pc = 0x6dc1;
    cpu.a = cpu.read_byte(0xc989);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x2;
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
        become function112d33_asm_112df2(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function112f61(cpu);
    assert_eq!(cpu.pc, next_pc);
    let mask = 1u8 << ((2) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0x3;
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
        become function112d33_asm_112e38(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x1;
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
        become function112d33_asm_112df2(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc86a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x1f;
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
        become function112d33_asm_112de1(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x20;
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
        become function112d33_asm_112df2(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become function112d33_asm_112de1(cpu);
}

pub(crate) fn function112d33_asm_112de1(cpu: &mut Cpu) {
    cpu.pc = 0x6de1;
    cpu.set_hl(0xc98b);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0x1;
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
        become function112d33_asm_112df2(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x4;
    cpu.pc += 2;
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
        become function112d33_asm_112df2(cpu);
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
    cpu.write_byte(0xc990, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    become function112d33_asm_112df2(cpu);
}

pub(crate) fn function112d33_asm_112df2(cpu: &mut Cpu) {
    cpu.pc = 0x6df2;
    cpu.a = cpu.read_byte(0xc86e);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte({ const N: i32 = (0xc86e as i32) + (1 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.l;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
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
    cpu.a = cpu.read_byte(0xc86a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x13;
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
        become function112d33_asm_112e21(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x14;
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
        become function112d33_asm_112e21(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x20;
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
    let value = 0x22;
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
    let value = 0x23;
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
    let value = 0x1f;
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
        become function112d33_asm_112e21(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xc98b);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0x0;
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
    cpu.a = 0x2;
    cpu.pc += 2;
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
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc86a);
    cpu.pc += 3;
    cpu.cycle(16);
    become function112d33_asm_112e21(cpu);
}

pub(crate) fn function112d33_asm_112e21(cpu: &mut Cpu) {
    cpu.pc = 0x6e21;
    let value = 0x24;
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
        become function112d33_asm_112e2a(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xc878);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(12);
    become function112d33_asm_112e2d(cpu);
}

pub(crate) fn function112d33_asm_112e2a(cpu: &mut Cpu) {
    cpu.pc = 0x6e2a;
    cpu.set_hl(0xc827);
    cpu.pc += 3;
    cpu.cycle(12);
    become function112d33_asm_112e2d(cpu);
}

pub(crate) fn function112d33_asm_112e2d(cpu: &mut Cpu) {
    cpu.pc = 0x6e2d;
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.d = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(0xc82d);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = 0x2;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become mobile_sdk_copy_bytes(cpu);
}

pub(crate) fn function112d33_asm_112e38(cpu: &mut Cpu) {
    cpu.pc = 0x6e38;
    cpu.set_hl(0xc821);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((1) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value |= mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    let mask = 1u8 << ((0) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.set_de(0xc98b);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0x24;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become function112d33_asm_112e95(cpu);
}

pub(crate) fn function112d33_asm_112e46(cpu: &mut Cpu) {
    cpu.pc = 0x6e46;
    cpu.a = cpu.read_byte(0xc86a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x1f;
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
        become function112d33_asm_112ea6(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x20;
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
        become function112d33_asm_112ea6(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc98a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x1;
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
        become function112d33_asm_112e65(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc86a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x21;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(16);
        become function112d33_asm_112eea(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    let value = 0x22;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(16);
        become function112d33_asm_112eea(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    become function112d33_asm_112e65(cpu);
}

pub(crate) fn function112d33_asm_112e65(cpu: &mut Cpu) {
    cpu.pc = 0x6e65;
    cpu.a = cpu.read_byte(0xc990);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.a;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(16);
        become function112d33_asm_112f3d(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    become function112d33_asm_112e6c(cpu);
}

pub(crate) fn function112d33_asm_112e6c(cpu: &mut Cpu) {
    cpu.pc = 0x6e6c;
    cpu.set_hl(0xc98c);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0x3;
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
        become function112d33_asm_112e7f(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.a;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function112d33_asm_112e7f(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x3;
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
        become function112d33_asm_112e7f(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function1133fe(cpu);
    assert_eq!(cpu.pc, next_pc);
    become function112d33_asm_112e7f(cpu);
}

pub(crate) fn function112d33_asm_112e7f(cpu: &mut Cpu) {
    cpu.pc = 0x6e7f;
    cpu.set_hl(0xc821);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((1) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value |= mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    let mask = 1u8 << ((0) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.set_de(0xc98b);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xc990);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x1;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x32;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become function112d33_asm_112e95(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
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
    become function112d33_asm_112e95(cpu);
}

pub(crate) fn function112d33_asm_112e95(cpu: &mut Cpu) {
    cpu.pc = 0x6e95;
    cpu.write_byte(0xc80f, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xc810);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 0x5;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc86a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function112d33_asm_112ea6(cpu: &mut Cpu) {
    cpu.pc = 0x6ea6;
    cpu.set_hl(0xc98b);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.h = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = 0x0;
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
        become function112d33_asm_112ec1(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x2;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.h;
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
        become function112d33_asm_112ec1(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc98d);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xc98e);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.b;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(12);
        become function112d33_asm_112e6c(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become function112d33_asm_112f3d(cpu);
}

pub(crate) fn function112d33_asm_112ec1(cpu: &mut Cpu) {
    cpu.pc = 0x6ec1;
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.l;
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
        become function112d33_asm_112e6c(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x4;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.h;
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
        become function112d33_asm_112e6c(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc9a5);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.a;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(12);
        become function112d33_asm_112efb(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc86e);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte({ const N: i32 = (0xc86e as i32) + (1 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.l;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(12);
        become function112d33_asm_112efb(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x2;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc86a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc86d, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xc821);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((0) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function112d33_asm_112eea(cpu: &mut Cpu) {
    cpu.pc = 0x6eea;
    cpu.set_hl(0xc98b);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.h = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = 0x0;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(16);
        become function112d33_asm_112e6c(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0x2;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.h;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(16);
        become function112d33_asm_112e6c(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    become function112d33_asm_112efb(cpu);
}

pub(crate) fn function112d33_asm_112efb(cpu: &mut Cpu) {
    cpu.pc = 0x6efb;
    cpu.a = cpu.read_byte(0xc98d);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xc98e);
    cpu.pc += 3;
    cpu.cycle(16);
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
        cpu.cycle(16);
        become function112d33_asm_112e6c(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    let value = cpu.a;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function112d33_asm_112f13(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x1;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(16);
        become function112d33_asm_112e6c(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc993, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    become function112d33_asm_112f13(cpu);
}

pub(crate) fn function112d33_asm_112f13(cpu: &mut Cpu) {
    cpu.pc = 0x6f13;
    cpu.a = cpu.read_byte(0xc86b);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x7;
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
        become function112d33_asm_112f3d(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xc98f);
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
    cpu.a = 0xf;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc86a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc86b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xc86d);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xc86e, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc989, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0xa3;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_de(0x0010 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0xc995);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become function111f02(cpu);
}

pub(crate) fn function112d33_asm_112f3d(cpu: &mut Cpu) {
    cpu.pc = 0x6f3d;
    cpu.a = cpu.read_byte(0xc993);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x1;
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
        become function112d33_asm_112f52(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x2;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc990, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xc98d);
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
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.cycle(16);
    become function112d33_asm_112e7f(cpu);
}

pub(crate) fn function112d33_asm_112f52(cpu: &mut Cpu) {
    cpu.pc = 0x6f52;
    cpu.a = 0x2;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc86a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc86d, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xc821);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((0) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function112f61(cpu: &mut Cpu) {
    cpu.pc = 0x6f61;
    cpu.set_hl(0xc989);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.a;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(12);
        become function112f61_asm_112f8a(cpu);
    }
    cpu.pc += 2;
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
    cpu.set_hl(0xc880);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0x0008 as u16);
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
    become function112f61_asm_112f70(cpu);
}

pub(crate) fn function112f61_asm_112f70(cpu: &mut Cpu) {
    cpu.pc = 0x6f70;
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0x20;
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
        become function112f61_asm_112f70(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.d = 0x0;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x32;
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
        become function112f61_asm_112f7d(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.d;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.d = result;
    cpu.pc += 1;
    cpu.cycle(4);
    become function112f61_asm_112f7d(cpu);
}

pub(crate) fn function112f61_asm_112f7d(cpu: &mut Cpu) {
    cpu.pc = 0x6f7d;
    cpu.a = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc990, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function112b11(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xc98b);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.d);
    cpu.pc += 1;
    cpu.cycle(8);
    become function112f61_asm_112f8a(cpu);
}

pub(crate) fn function112f61_asm_112f8a(cpu: &mut Cpu) {
    cpu.pc = 0x6f8a;
    cpu.set_hl(0xc880);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xc82d);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.a;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(12);
        become function112f61_asm_112fa1(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xc98b);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0x0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc990, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function112f61_asm_112fa1(cpu: &mut Cpu) {
    cpu.pc = 0x6fa1;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function112fd5(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function113008(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function113026(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function113054(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function113167(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function113180(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function113268(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.cycle(12);
        become function112f61_asm_112fc9(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0xd;
    cpu.pc += 2;
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
    if cpu.flag_z {
        cpu.cycle(12);
        become function112f61_asm_112fc2(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0xa;
    cpu.pc += 2;
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
        become function112f61_asm_112fa1(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become function112f61_asm_112fc2(cpu);
}

pub(crate) fn function112f61_asm_112fc2(cpu: &mut Cpu) {
    cpu.pc = 0x6fc2;
    cpu.set_hl(0xc990);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((2) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.cycle(12);
    become function112f61_asm_112fce(cpu);
}

pub(crate) fn function112f61_asm_112fc9(cpu: &mut Cpu) {
    cpu.pc = 0x6fc9;
    cpu.set_hl(0xc990);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((2) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value |= mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    become function112f61_asm_112fce(cpu);
}

pub(crate) fn function112f61_asm_112fce(cpu: &mut Cpu) {
    cpu.pc = 0x6fce;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11306b(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xc990);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function112fd5(cpu: &mut Cpu) {
    cpu.pc = 0x6fd5;
    cpu.set_de(0x7001);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function113281(cpu);
    assert_eq!(cpu.pc, next_pc);
    if !cpu.flag_c {
        cpu.cycle(12);
        become function112fd5_asm_112fe0(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function112fd5_asm_112fe0(cpu: &mut Cpu) {
    cpu.pc = 0x6fe0;
    let value = cpu.pop_word();
    cpu.set_de(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.push_word(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.push_word(cpu.de());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.b = 0x0;
    cpu.pc += 2;
    cpu.cycle(8);
    become function112fd5_asm_112fe6(cpu);
}

pub(crate) fn function112fd5_asm_112fe6(cpu: &mut Cpu) {
    cpu.pc = 0x6fe6;
    let value = cpu.b;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.b = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0xa;
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
        become function112fd5_asm_112fe6(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.c = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xc833);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xc834);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.d = cpu.a;
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
        become function112fd5_asm_112ffe(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
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
    become function112fd5_asm_112ffe(cpu);
}

pub(crate) fn function112fd5_asm_112ffe(cpu: &mut Cpu) {
    cpu.pc = 0x6ffe;
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let value = cpu.pop_word();
    cpu.set_bc(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function113008(cpu: &mut Cpu) {
    cpu.pc = 0x7008;
    cpu.set_de(0x72a6);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function113273(cpu);
    assert_eq!(cpu.pc, next_pc);
    if !cpu.flag_c {
        cpu.cycle(12);
        become function113008_asm_113013(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function113008_asm_113013(cpu: &mut Cpu) {
    cpu.pc = 0x7013;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function112b11(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xc98d);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.d);
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
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
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 0x2;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc990, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function113026(cpu: &mut Cpu) {
    cpu.pc = 0x7026;
    cpu.set_de(0x72b2);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function113273(cpu);
    assert_eq!(cpu.pc, next_pc);
    if !cpu.flag_c {
        cpu.cycle(12);
        become function113026_asm_113031(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function113026_asm_113031(cpu: &mut Cpu) {
    cpu.pc = 0x7031;
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.push_word(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.b = 0x0;
    cpu.pc += 2;
    cpu.cycle(8);
    become function113026_asm_113037(cpu);
}

pub(crate) fn function113026_asm_113037(cpu: &mut Cpu) {
    cpu.pc = 0x7037;
    let value = cpu.b;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.b = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0xa;
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
        become function113026_asm_113037(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.c = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_de({ const N: i32 = (0xcb47 as i32) + (18 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = (0xcb47 as i32) + (18 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xc9b5);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
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
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let value = cpu.pop_word();
    cpu.set_bc(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function113054(cpu: &mut Cpu) {
    cpu.pc = 0x7054;
    cpu.set_de(0x72bf);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function113273(cpu);
    assert_eq!(cpu.pc, next_pc);
    if !cpu.flag_c {
        cpu.cycle(12);
        become function113054_asm_11305f(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function113054_asm_11305f(cpu: &mut Cpu) {
    cpu.pc = 0x705f;
    cpu.push_word(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.set_de(0xc9b5);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = 0x30;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function1136c1(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_bc(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function11306b(cpu: &mut Cpu) {
    cpu.pc = 0x706b;
    cpu.set_hl(0xc880);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xc82d);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    become function113072(cpu);
}

pub(crate) fn function113072(cpu: &mut Cpu) {
    cpu.pc = 0x7072;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function113268(cpu);
    assert_eq!(cpu.pc, next_pc);
    if !cpu.flag_c {
        cpu.cycle(16);
        become function113089(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xca3c);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x9f;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(16);
        become function1131a9(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.set_hl(0xc990);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((2) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.cycle(12);
    become function113095(cpu);
}

pub(crate) fn function113089(cpu: &mut Cpu) {
    cpu.pc = 0x7089;
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0xd;
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
        become function113089_asm_113094(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0xa;
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
        become function113095(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become function113072(cpu);
}

pub(crate) fn function113089_asm_113094(cpu: &mut Cpu) {
    cpu.pc = 0x7094;
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    become function113095(cpu);
}

pub(crate) fn function113095(cpu: &mut Cpu) {
    cpu.pc = 0x7095;
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.push_word(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xc872);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xc873);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.b;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.pop_word();
    cpu.set_bc(value);
    cpu.pc += 1;
    cpu.cycle(12);
    if cpu.flag_z {
        cpu.cycle(12);
        become function113095_asm_1130b3(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc86a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x23;
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
        become function113095_asm_1130b3(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x20;
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
        become function113095_asm_1130b3(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x22;
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
        become function113095_asm_1130b3(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become function113095_asm_1130d6(cpu);
}

pub(crate) fn function113095_asm_1130b3(cpu: &mut Cpu) {
    cpu.pc = 0x70b3;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl(0xc86e);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(0xc821);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((2) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xc86a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x13;
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
        become function113095_asm_1130c8(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x14;
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
    become function113095_asm_1130c8(cpu);
}

pub(crate) fn function113095_asm_1130c8(cpu: &mut Cpu) {
    cpu.pc = 0x70c8;
    cpu.a = 0x6;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc86b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xca3c);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x9f;
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
    cpu.cycle(16);
    become function112430(cpu);
}

pub(crate) fn function113095_asm_1130d6(cpu: &mut Cpu) {
    cpu.pc = 0x70d6;
    cpu.a = cpu.read_byte(0xc82b);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.b;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.b = result;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.b;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.b = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc82d, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    if cpu.flag_z {
        cpu.cycle(12);
        become function113095_asm_11310d(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc873);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.d = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xc872);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_de(cpu.de().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.d;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(12);
        become function113095_asm_1130f5(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
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
        cpu.cycle(16);
        become function113206(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    become function113095_asm_1130f5(cpu);
}

pub(crate) fn function113095_asm_1130f5(cpu: &mut Cpu) {
    cpu.pc = 0x70f5;
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.b;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc82b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = 0x0;
    let carry_in = u8::from(cpu.flag_c);
    let lhs = cpu.a;
    let (partial, borrow1) = lhs.overflowing_sub(value);
    let (result, borrow2) = partial.overflowing_sub(carry_in);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < ((value & 0x0f) + carry_in);
    cpu.flag_c = borrow1 || borrow2;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc82c, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xc874);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xc875);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.d = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    become function113095_asm_11310d(cpu);
}

pub(crate) fn function113095_asm_11310d(cpu: &mut Cpu) {
    cpu.pc = 0x710d;
    cpu.a = cpu.read_byte(0xca3c);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x9f;
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
        become function113095_asm_113150(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte({ const N: i32 = (0xca3c as i32) + (3 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.a;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function113095_asm_113150(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.l = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
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
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl({ const N: i32 = (0xca3c as i32) + (4 as i32); N as u16 });
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
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.push_word(cpu.de());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xc82b);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xc82c);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.d = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.d;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(12);
        become function113095_asm_113135(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
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
        cpu.cycle(16);
        become function113245(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    become function113095_asm_113135(cpu);
}

pub(crate) fn function113095_asm_113135(cpu: &mut Cpu) {
    cpu.pc = 0x7135;
    let value = cpu.pop_word();
    cpu.set_de(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.set_hl(0xc82d);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
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
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0;
    let carry_in = u8::from(cpu.flag_c);
    let lhs = cpu.a;
    let (partial, carry1) = lhs.overflowing_add(value);
    let (result, carry2) = partial.overflowing_add(carry_in);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) + carry_in > 0x0f;
    cpu.flag_c = carry1 || carry2;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.c = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xc82b);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
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
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0x0;
    let carry_in = u8::from(cpu.flag_c);
    let lhs = cpu.a;
    let (partial, borrow1) = lhs.overflowing_sub(value);
    let (result, borrow2) = partial.overflowing_sub(carry_in);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < ((value & 0x0f) + carry_in);
    cpu.flag_c = borrow1 || borrow2;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    become function113095_asm_113150(cpu);
}

pub(crate) fn function113095_asm_113150(cpu: &mut Cpu) {
    cpu.pc = 0x7150;
    cpu.set_hl(0xc829);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(0xc821);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((2) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc86b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x2;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc989, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function113167(cpu: &mut Cpu) {
    cpu.pc = 0x7167;
    cpu.set_de(0x72ff);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function113273(cpu);
    assert_eq!(cpu.pc, next_pc);
    if !cpu.flag_c {
        cpu.cycle(12);
        become function113167_asm_113172(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function113167_asm_113172(cpu: &mut Cpu) {
    cpu.pc = 0x7172;
    let value = cpu.pop_word();
    cpu.set_de(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.push_word(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.push_word(cpu.de());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.b = 0x0;
    cpu.pc += 2;
    cpu.cycle(8);
    become function113167_asm_113178(cpu);
}

pub(crate) fn function113167_asm_113178(cpu: &mut Cpu) {
    cpu.pc = 0x7178;
    let value = cpu.b;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.b = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0xa;
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
        become function113167_asm_113178(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become function113197(cpu);
}

pub(crate) fn function113180(cpu: &mut Cpu) {
    cpu.pc = 0x7180;
    cpu.set_de(0x730c);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function113273(cpu);
    assert_eq!(cpu.pc, next_pc);
    if !cpu.flag_c {
        cpu.cycle(12);
        become function113180_asm_11318b(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function113180_asm_11318b(cpu: &mut Cpu) {
    cpu.pc = 0x718b;
    let value = cpu.pop_word();
    cpu.set_de(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.push_word(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.push_word(cpu.de());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.b = 0x0;
    cpu.pc += 2;
    cpu.cycle(8);
    become function113180_asm_113191(cpu);
}

pub(crate) fn function113180_asm_113191(cpu: &mut Cpu) {
    cpu.pc = 0x7191;
    let value = cpu.b;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.b = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0xa;
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
        become function113180_asm_113191(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become function113197(cpu);
}

pub(crate) fn function113197(cpu: &mut Cpu) {
    cpu.pc = 0x7197;
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.c = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_de({ const N: i32 = (0xcb47 as i32) + (16 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
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
    let value = cpu.b;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.b = result;
    cpu.pc += 1;
    cpu.cycle(4);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
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
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let value = cpu.pop_word();
    cpu.set_bc(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function1131a9(cpu: &mut Cpu) {
    cpu.pc = 0x71a9;
    cpu.set_hl(0xc979);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xc880);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = 0x0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.c = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0xa;
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
        become function1131a9_asm_1131c4(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become function1131a9_asm_1131b7(cpu);
}

pub(crate) fn function1131a9_asm_1131b7(cpu: &mut Cpu) {
    cpu.pc = 0x71b7;
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.b;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.b = result;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = 0xa;
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
        become function1131a9_asm_1131b7(cpu);
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
    cpu.c = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    become function1131a9_asm_1131c4(cpu);
}

pub(crate) fn function1131a9_asm_1131c4(cpu: &mut Cpu) {
    cpu.pc = 0x71c4;
    cpu.a = cpu.read_byte(0xc82b);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.c;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_add(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) > 0x0f;
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.push_word(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = 0xff;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.b;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.b = 0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl({ const N: i32 = (0xca3c as i32) + (4 as i32); N as u16 });
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
    cpu.set_bc(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc82d, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0xfa;
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
    cpu.write_byte(0xc82b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xc829);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.l = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.h = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_de(0xc97a);
    cpu.pc += 3;
    cpu.cycle(12);
    become function1131a9_asm_1131ef(cpu);
}

pub(crate) fn function1131a9_asm_1131ef(cpu: &mut Cpu) {
    cpu.pc = 0x71ef;
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
    cpu.a = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.e;
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
        become function1131a9_asm_1131ef(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.h;
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
        become function1131a9_asm_1131ef(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xc821);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((2) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.set_hl(0xc86b);
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
    let addr = cpu.hl();
    let value = cpu.read_byte(addr);
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.write_byte(addr, result);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.a = 0x4;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function113206(cpu: &mut Cpu) {
    cpu.pc = 0x7206;
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.e;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc991, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xc821);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((2) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(12);
        become function113206_asm_113214(cpu);
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
    become function113206_asm_113214(cpu);
}

pub(crate) fn function113206_asm_113214(cpu: &mut Cpu) {
    cpu.pc = 0x7214;
    cpu.write_byte(0xc992, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.b = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.c = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xc874);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xc875);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.d = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xc991);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xc993, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_de(0xc880);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xc82d);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
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
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(0xc821);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((2) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value |= mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.a = 0x3;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc86b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function113245(cpu: &mut Cpu) {
    cpu.pc = 0x7245;
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.e;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc992, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xc82b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.b = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.c = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.pop_word();
    cpu.set_de(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xc82d);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.c;
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
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 0x0;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.read_byte(cpu.hl());
    let carry_in = u8::from(cpu.flag_c);
    let lhs = cpu.a;
    let (partial, carry1) = lhs.overflowing_add(value);
    let (result, carry2) = partial.overflowing_add(carry_in);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) + carry_in > 0x0f;
    cpu.flag_c = carry1 || carry2;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(0xc821);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((2) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value |= mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.a = 0x3;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc86b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function113268(cpu: &mut Cpu) {
    cpu.pc = 0x7268;
    become function113268_asm_113268(cpu);
}

pub(crate) fn function113268_asm_113268(cpu: &mut Cpu) {
    cpu.pc = 0x7268;
    let value = cpu.b;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.b = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0xa;
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
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.b;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(12);
        become function113268_asm_113268(cpu);
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

pub(crate) fn function113273(cpu: &mut Cpu) {
    cpu.pc = 0x7273;
    cpu.c = 0x0;
    cpu.pc += 2;
    cpu.cycle(8);
    become function113273_asm_113275(cpu);
}

pub(crate) fn function113273_asm_113275(cpu: &mut Cpu) {
    cpu.pc = 0x7275;
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.a;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
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
    let value = cpu.read_byte(cpu.hl());
    cpu.a ^= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.c;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function113273_asm_113275(cpu);
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

pub(crate) fn function113281(cpu: &mut Cpu) {
    cpu.pc = 0x7281;
    cpu.c = 0x0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.l = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.h = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.pop_word();
    cpu.set_de(value);
    cpu.pc += 1;
    cpu.cycle(12);
    become function113281_asm_113287(cpu);
}

pub(crate) fn function113281_asm_113287(cpu: &mut Cpu) {
    cpu.pc = 0x7287;
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11329d(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.read_byte(cpu.hl());
    cpu.a ^= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.c;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
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
    let value = cpu.read_byte(cpu.hl());
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 1;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become function113281_asm_113298(cpu);
    }
    cpu.pc += 2;
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
    if cpu.flag_z {
        cpu.cycle(12);
        become function113281_asm_113287(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = true;
    cpu.pc += 1;
    cpu.cycle(4);
    become function113281_asm_113298(cpu);
}

pub(crate) fn function113281_asm_113298(cpu: &mut Cpu) {
    cpu.pc = 0x7298;
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.l = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.h = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.pop_word();
    cpu.set_de(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function11329d(cpu: &mut Cpu) {
    cpu.pc = 0x729d;
    let value = 0x41;
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
    let value = 0x5b;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_c {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0x20;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function113317(cpu: &mut Cpu) {
    cpu.pc = 0x7317;
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc86b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_de({ const N: i32 = (0xcb47 as i32) + (18 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xc86c);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_bc(0x0001 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function1126b0(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0x72dd);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xc9a5);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.a;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        let next_pc = cpu.pc + 3;
        cpu.push_word(next_pc);
        cpu.cycle(24);
        mobile_sdk_copy_string(cpu);
        assert_eq!(cpu.pc, next_pc);
    } else {
        cpu.pc += 3;
        cpu.cycle(12);
    }
    cpu.a = cpu.read_byte(0xc86a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x22;
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
        become function113317_asm_113344(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc98a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x2;
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
        become function113317_asm_113351(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become function113317_asm_113348(cpu);
}

pub(crate) fn function113317_asm_113344(cpu: &mut Cpu) {
    cpu.pc = 0x7344;
    let value = 0x24;
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
        become function113317_asm_113351(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become function113317_asm_113348(cpu);
}

pub(crate) fn function113317_asm_113348(cpu: &mut Cpu) {
    cpu.pc = 0x7348;
    cpu.a = cpu.read_byte(0xc9a5);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.a;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function113317_asm_11336a(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function1133de(cpu);
    assert_eq!(cpu.pc, next_pc);
    become function113317_asm_113351(cpu);
}

pub(crate) fn function113317_asm_113351(cpu: &mut Cpu) {
    cpu.pc = 0x7351;
    cpu.set_hl(0xc9b5);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_string(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function1126b6(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte({ const N: i32 = (0xcb47 as i32) + (17 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.b = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function111f63(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x95;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl({ const N: i32 = (0xcb47 as i32) + (12 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become function111f02(cpu);
}

pub(crate) fn function113317_asm_11336a(cpu: &mut Cpu) {
    cpu.pc = 0x736a;
    cpu.set_hl(0x7372);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_string(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(12);
    become function113317_asm_113351(cpu);
}

pub(crate) fn function113386(cpu: &mut Cpu) {
    cpu.pc = 0x7386;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function113482(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc86b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_de(0xcb47);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0x6072);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = 0x6;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xc86c);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.b = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function111f63(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_de({ const N: i32 = (0xcb47 as i32) + (12 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0x6072);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = 0x6;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xc86d);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x3;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(16);
        become function113317(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de({ const N: i32 = (0xcb47 as i32) + (18 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xc86c);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_bc(0x0001 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function1126b0(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xc994);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.a;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        let next_pc = cpu.pc + 3;
        cpu.push_word(next_pc);
        cpu.cycle(24);
        function1133de(cpu);
        assert_eq!(cpu.pc, next_pc);
    } else {
        cpu.pc += 3;
        cpu.cycle(12);
    }
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function1126b6(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte({ const N: i32 = (0xcb47 as i32) + (17 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.b = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function111f63(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x95;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl({ const N: i32 = (0xcb47 as i32) + (12 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become function111f02(cpu);
}

pub(crate) fn function1133de(cpu: &mut Cpu) {
    cpu.pc = 0x73de;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function1126e6(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc86b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xc9aa);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xc87c, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xc9ab);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xc87d, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xc9ac);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xc87e, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xc9ad);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xc87f, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function1133fe(cpu: &mut Cpu) {
    cpu.pc = 0x73fe;
    cpu.set_hl({ const N: i32 = (0xcb47 as i32) + (17 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = { let value = (0xa7 as i32); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
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
        become function1133fe_asm_113432(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = { let value = (0xb3 as i32); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
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
        become function1133fe_asm_113432(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = { let value = (0xb3 as i32); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
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
        become function1133fe_asm_113432(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = { let value = (0xaf as i32); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
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
        become function1133fe_asm_113432(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = { let value = (0x9c as i32); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
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
        become function1133fe_asm_113432(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = { let value = (0xf3 as i32); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
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
        become function1133fe_asm_113432(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = { let value = (0xf3 as i32); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
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
        become function1133fe_asm_113432(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl({ const N: i32 = (0xcb47 as i32) + (16 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xc880);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
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
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function1133fe_asm_113432(cpu: &mut Cpu) {
    cpu.pc = 0x7432;
    cpu.a = cpu.read_byte({ const N: i32 = (0xcb47 as i32) + (17 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    let value = { let value = (0xf3 as i32); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
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
        become function1133fe_asm_113460(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_de(0xc880);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0xc9b5);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.h = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.h = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_string(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.l = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.h = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    become function1133fe_asm_11344c(cpu);
}

pub(crate) fn function1133fe_asm_11344c(cpu: &mut Cpu) {
    cpu.pc = 0x744c;
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = { let value = (0xf3 as i32); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
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
        become function1133fe_asm_11344c(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.e = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.d = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    become function1133fe_asm_113455(cpu);
}

pub(crate) fn function1133fe_asm_113455(cpu: &mut Cpu) {
    cpu.pc = 0x7455;
    cpu.set_hl({ const N: i32 = (0xcb47 as i32) + (16 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
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
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function1133fe_asm_113460(cpu: &mut Cpu) {
    cpu.pc = 0x7460;
    cpu.set_de(0xc880);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0xc9b5);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.h = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.h = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = { let value = (0xf3 as i32); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
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
        become function1133fe_asm_113478(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.b = 0x7;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    become function1133fe_asm_113478(cpu);
}

pub(crate) fn function1133fe_asm_113478(cpu: &mut Cpu) {
    cpu.pc = 0x7478;
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
    let value = { let value = (0xf3 as i32); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
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
        become function1133fe_asm_113478(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.cycle(12);
    become function1133fe_asm_113455(cpu);
}

pub(crate) fn function113482(cpu: &mut Cpu) {
    cpu.pc = 0x7482;
    cpu.set_hl(0xc87f);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
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
    if cpu.flag_z {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.set_hl(crate::defs::R_SC as u16);
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
    if cpu.flag_c {
        cpu.cycle(12);
        become function113482_asm_113496(cpu);
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
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.h = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    become function113482_asm_113496(cpu);
}

pub(crate) fn function113482_asm_113496(cpu: &mut Cpu) {
    cpu.pc = 0x7496;
    cpu.e = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.d = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl(0xc87f);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    if !cpu.flag_c {
        cpu.cycle(12);
        become function113482_asm_1134a3(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.c = 0xfe;
    cpu.pc += 2;
    cpu.cycle(8);
    become function113482_asm_1134a3(cpu);
}

pub(crate) fn function113482_asm_1134a3(cpu: &mut Cpu) {
    cpu.pc = 0x74a3;
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.l = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.h = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.a;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte({ const N: i32 = (0xcb47 as i32) + (5 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_de({ const N: i32 = (0xcb47 as i32) + (7 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc87c, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc87d, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.b = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.b;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.b = result;
    cpu.pc += 1;
    cpu.cycle(4);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function111f63(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xc86b);
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
    cpu.set_hl(0xcb47);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0x95;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become function111f02(cpu);
}

pub(crate) fn function1134cb(cpu: &mut Cpu) {
    cpu.pc = 0x74cb;
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function1134cb_asm_1134d9(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function1134cb_asm_1134f4(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function1134cb_asm_1134fc(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function1134cb_asm_11350e(cpu);
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
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function1134cb_asm_1134d9(cpu: &mut Cpu) {
    cpu.pc = 0x74d9;
    cpu.a = cpu.read_byte({ const N: i32 = (0xca3c as i32) + (4 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x0;
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
        become function1134cb_asm_1134f0(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0xff;
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
        become function1134cb_asm_1134f0(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc985);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xc86a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xc821);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((0) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function1134cb_asm_1134f0(cpu: &mut Cpu) {
    cpu.pc = 0x74f0;
    let addr = cpu.hl();
    let value = cpu.read_byte(addr);
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.write_byte(addr, result);
    cpu.pc += 1;
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
    cpu.cycle(12);
    become function1134cb_asm_1134fc(cpu);
}

pub(crate) fn function1134cb_asm_1134f4(cpu: &mut Cpu) {
    cpu.pc = 0x74f4;
    cpu.a = { let value = ((MOBILE_COMMAND_TELEPHONE_STATUS as i32) | (0x80 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x602d);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become packet_send_empty_body(cpu);
}

pub(crate) fn function1134cb_asm_1134fc(cpu: &mut Cpu) {
    cpu.pc = 0x74fc;
    cpu.set_hl(0xc86e);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.h = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte({ const N: i32 = (0xca3c as i32) + (6 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0xf0;
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
        become function1134cb_asm_11350b(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let mask = 1u8 << ((7) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value |= mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    become function1134cb_asm_11350b(cpu);
}

pub(crate) fn function1134cb_asm_11350b(cpu: &mut Cpu) {
    cpu.pc = 0x750b;
    cpu.cycle(16);
    become mobile_end_session(cpu);
}

pub(crate) fn function1134cb_asm_11350e(cpu: &mut Cpu) {
    cpu.pc = 0x750e;
    cpu.a = cpu.read_byte(0xc86a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x1e;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(16);
        become function112251(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become function1116a0(cpu);
}

pub(crate) fn function113519(cpu: &mut Cpu) {
    cpu.pc = 0x7519;
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function113519_asm_113527(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function113519_asm_11352d(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function113519_asm_11353c(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function113519_asm_11353f(cpu);
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
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function113519_asm_113527(cpu: &mut Cpu) {
    cpu.pc = 0x7527;
    cpu.set_hl(0x6046);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become function11236b(cpu);
}

pub(crate) fn function113519_asm_11352d(cpu: &mut Cpu) {
    cpu.pc = 0x752d;
    cpu.set_hl(0xc829);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = ((((0xc8ca as u32) + (22 as u32))) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = (((((0xc8ca as u32) + (22 as u32))) >> 8) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(0x6052);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become function11236b(cpu);
}

pub(crate) fn function113519_asm_11353c(cpu: &mut Cpu) {
    cpu.pc = 0x753c;
    cpu.cycle(16);
    become mobile_end_session(cpu);
}

pub(crate) fn function113519_asm_11353f(cpu: &mut Cpu) {
    cpu.pc = 0x753f;
    cpu.set_hl(0xc880);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = { let value = (0x8c as i32); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
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
        become function113519_asm_113586(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = { let value = (0x80 as i32); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
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
        become function113519_asm_113586(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.b = 0xbe;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_de(0 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    become function113519_asm_113551(cpu);
}

pub(crate) fn function113519_asm_113551(cpu: &mut Cpu) {
    cpu.pc = 0x7551;
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.e;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_add(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) > 0x0f;
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = 0x0;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.d;
    let carry_in = u8::from(cpu.flag_c);
    let lhs = cpu.a;
    let (partial, carry1) = lhs.overflowing_add(value);
    let (result, carry2) = partial.overflowing_add(carry_in);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) + carry_in > 0x0f;
    cpu.flag_c = carry1 || carry2;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.d = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
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
        become function113519_asm_113551(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.d;
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
        become function113519_asm_11358a(cpu);
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
    if !cpu.flag_z {
        cpu.cycle(12);
        become function113519_asm_11358a(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc86e);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte({ const N: i32 = (0xc86e as i32) + (1 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.d = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl(0x757e);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xc86a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x25;
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
        become function1135ba(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x26;
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
        become function11359d(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x27;
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
        become function1135ad(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become function113519_asm_11357e(cpu);
}

pub(crate) fn function113519_asm_11357e(cpu: &mut Cpu) {
    cpu.pc = 0x757e;
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc835, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(16);
    become function1116a0(cpu);
}

pub(crate) fn function113519_asm_113586(cpu: &mut Cpu) {
    cpu.pc = 0x7586;
    cpu.a = 0x25;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become function113519_asm_11358c(cpu);
}

pub(crate) fn function113519_asm_11358a(cpu: &mut Cpu) {
    cpu.pc = 0x758a;
    cpu.a = 0x14;
    cpu.pc += 2;
    cpu.cycle(8);
    become function113519_asm_11358c(cpu);
}

pub(crate) fn function113519_asm_11358c(cpu: &mut Cpu) {
    cpu.pc = 0x758c;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11225d(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(16);
    become function1116a4(cpu);
}

pub(crate) fn function113592(cpu: &mut Cpu) {
    cpu.pc = 0x7592;
    cpu.push_word(cpu.de());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.l = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.h = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_de(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function11359d(cpu: &mut Cpu) {
    cpu.pc = 0x759d;
    cpu.b = 0x20;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function113592(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x21;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xc88c);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_string_len(cpu);
    assert_eq!(cpu.pc, next_pc);
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
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function1135ad(cpu: &mut Cpu) {
    cpu.pc = 0x75ad;
    cpu.b = 0x1e;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function113592(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x1f;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xc8ac);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become mobile_sdk_copy_string_len(cpu);
}

pub(crate) fn function1135ba(cpu: &mut Cpu) {
    cpu.pc = 0x75ba;
    cpu.b = 0x65;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function113592(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xc8f6);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function1135eb(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x11;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xc8fe);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_string_len(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(0xc90e);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function1135eb(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x11;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xc916);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_string_len(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(0xc926);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function1135eb(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x11;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xc92e);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become mobile_sdk_copy_string_len(cpu);
}

pub(crate) fn function1135eb(cpu: &mut Cpu) {
    cpu.pc = 0x75eb;
    cpu.b = 0x8;
    cpu.pc += 2;
    cpu.cycle(8);
    become function1135eb_asm_1135ed(cpu);
}

pub(crate) fn function1135eb_asm_1135ed(cpu: &mut Cpu) {
    cpu.pc = 0x75ed;
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value.rotate_left(4);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0xf;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0xf;
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
        become function1135eb_asm_11361c(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x30;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x3a;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        let next_pc = cpu.pc + 3;
        cpu.push_word(next_pc);
        cpu.cycle(24);
        function113620(cpu);
        assert_eq!(cpu.pc, next_pc);
    } else {
        cpu.pc += 3;
        cpu.cycle(12);
    }
    let value = 0x3b;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        let next_pc = cpu.pc + 3;
        cpu.push_word(next_pc);
        cpu.cycle(24);
        function113623(cpu);
        assert_eq!(cpu.pc, next_pc);
    } else {
        cpu.pc += 3;
        cpu.cycle(12);
    }
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0xf;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0xf;
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
        become function1135eb_asm_11361c(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x30;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x3a;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        let next_pc = cpu.pc + 3;
        cpu.push_word(next_pc);
        cpu.cycle(24);
        function113620(cpu);
        assert_eq!(cpu.pc, next_pc);
    } else {
        cpu.pc += 3;
        cpu.cycle(12);
    }
    let value = 0x3b;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        let next_pc = cpu.pc + 3;
        cpu.push_word(next_pc);
        cpu.cycle(24);
        function113623(cpu);
        assert_eq!(cpu.pc, next_pc);
    } else {
        cpu.pc += 3;
        cpu.cycle(12);
    }
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
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
        become function1135eb_asm_1135ed(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become function1135eb_asm_11361c(cpu);
}

pub(crate) fn function1135eb_asm_11361c(cpu: &mut Cpu) {
    cpu.pc = 0x761c;
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
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function113620(cpu: &mut Cpu) {
    cpu.pc = 0x7620;
    cpu.a = 0x23;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function113623(cpu: &mut Cpu) {
    cpu.pc = 0x7623;
    cpu.a = 0x2a;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function113626(cpu: &mut Cpu) {
    cpu.pc = 0x7626;
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function113626_asm_113634(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function113626_asm_113639(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function113626_asm_11366c(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function113626_asm_11366f(cpu);
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
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function113626_asm_113634(cpu: &mut Cpu) {
    cpu.pc = 0x7634;
    cpu.b = 0x9a;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become function11234b(cpu);
}

pub(crate) fn function113626_asm_113639(cpu: &mut Cpu) {
    cpu.pc = 0x7639;
    cpu.a = cpu.read_byte(0xc882);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.a;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(12);
        become function113626_asm_113642(cpu);
    }
    cpu.pc += 2;
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
    cpu.cycle(12);
    become function113626_asm_11366c(cpu);
}

pub(crate) fn function113626_asm_113642(cpu: &mut Cpu) {
    cpu.pc = 0x7642;
    cpu.set_de({ const N: i32 = (0xcb47 as i32) + (5 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.a;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 0x80;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(0xc880);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.h = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.b = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.b = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.b;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.b = result;
    cpu.pc += 1;
    cpu.cycle(4);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function111f63(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte({ const N: i32 = (0xcb47 as i32) + (5 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0xa;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_add(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) > 0x0f;
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.d = 0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x9a;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xcb47);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become function111f02(cpu);
}

pub(crate) fn function113626_asm_11366c(cpu: &mut Cpu) {
    cpu.pc = 0x766c;
    cpu.cycle(16);
    become mobile_end_session(cpu);
}

pub(crate) fn function113626_asm_11366f(cpu: &mut Cpu) {
    cpu.pc = 0x766f;
    cpu.cycle(16);
    become function1116a0(cpu);
}

pub(crate) fn function113672(cpu: &mut Cpu) {
    cpu.pc = 0x7672;
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function113672_asm_113680(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function113672_asm_113686(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function113672_asm_1136bb(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function113672_asm_1136be(cpu);
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
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function113672_asm_113680(cpu: &mut Cpu) {
    cpu.pc = 0x7680;
    cpu.set_hl(0xcb47);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become function11236b(cpu);
}

pub(crate) fn function113672_asm_113686(cpu: &mut Cpu) {
    cpu.pc = 0x7686;
    cpu.a = cpu.read_byte(0xc882);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.a;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function113672_asm_113693(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x81;
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
        become function113672_asm_113693(cpu);
    }
    cpu.pc += 2;
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
    cpu.cycle(12);
    become function113672_asm_1136bb(cpu);
}

pub(crate) fn function113672_asm_113693(cpu: &mut Cpu) {
    cpu.pc = 0x7693;
    cpu.set_hl({ const N: i32 = (0xcb47 as i32) + (7 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    let value = 0x80;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 0x80;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de({ const N: i32 = (0xcb47 as i32) + (8 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = 0x2;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function111f63(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xc880);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.h = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_de(0x0080 as u16);
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
    cpu.e = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl(0xc829);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.e);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(0xcb47);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become function11236b(cpu);
}

pub(crate) fn function113672_asm_1136bb(cpu: &mut Cpu) {
    cpu.pc = 0x76bb;
    cpu.cycle(16);
    become mobile_end_session(cpu);
}

pub(crate) fn function113672_asm_1136be(cpu: &mut Cpu) {
    cpu.pc = 0x76be;
    cpu.cycle(16);
    become function1116a0(cpu);
}

pub(crate) fn function1136c1(cpu: &mut Cpu) {
    cpu.pc = 0x76c1;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte({ const N: i32 = (0xcb47 as i32) + (225 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte({ const N: i32 = (0xcb47 as i32) + (192 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte({ const N: i32 = (0xcb47 as i32) + (193 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl({ const N: i32 = (0xcb47 as i32) + (194 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl({ const N: i32 = (0xcb47 as i32) + (192 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.h = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_de({ const N: i32 = (0xcb47 as i32) + (32 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = 0x30;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.c = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xc97f);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.h = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    become function1136c1_asm_1136eb(cpu);
}

pub(crate) fn function1136c1_asm_1136eb(cpu: &mut Cpu) {
    cpu.pc = 0x76eb;
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.a;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(12);
        become function1136c1_asm_1136eb(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_string(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x37;
    cpu.pc += 2;
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
    let value = cpu.a;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_c {
        cpu.cycle(12);
        become function1136c1_asm_1136ff(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x2;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte({ const N: i32 = (0xcb47 as i32) + (225 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x78;
    cpu.pc += 2;
    cpu.cycle(8);
    become function1136c1_asm_1136ff(cpu);
}

pub(crate) fn function1136c1_asm_1136ff(cpu: &mut Cpu) {
    cpu.pc = 0x76ff;
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
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = 0x80;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    become function1136c1_asm_113706(cpu);
}

pub(crate) fn function1136c1_asm_113706(cpu: &mut Cpu) {
    cpu.pc = 0x7706;
    let value = cpu.b;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.b = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function1136c1_asm_11370d(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.cycle(12);
    become function1136c1_asm_113706(cpu);
}

pub(crate) fn function1136c1_asm_11370d(cpu: &mut Cpu) {
    cpu.pc = 0x770d;
    let value = cpu.a;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.c;
    let result = value << 1;
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x80 != 0;
    cpu.c = result;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.b;
    let carry_in = u8::from(cpu.flag_c);
    let result = (value << 1) | carry_in;
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x80 != 0;
    cpu.b = result;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.c;
    let result = value << 1;
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x80 != 0;
    cpu.c = result;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.b;
    let carry_in = u8::from(cpu.flag_c);
    let result = (value << 1) | carry_in;
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x80 != 0;
    cpu.b = result;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.c;
    let result = value << 1;
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x80 != 0;
    cpu.c = result;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.b;
    let carry_in = u8::from(cpu.flag_c);
    let result = (value << 1) | carry_in;
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x80 != 0;
    cpu.b = result;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
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
    cpu.l = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.h = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.b = 0x6;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    become function1136c1_asm_113725(cpu);
}

pub(crate) fn function1136c1_asm_113725(cpu: &mut Cpu) {
    cpu.pc = 0x7725;
    cpu.write_byte(cpu.hl(), cpu.a);
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
        become function1136c1_asm_113725(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_de({ const N: i32 = (0xcb47 as i32) + (160 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0x7b7e);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = 0x10;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    become function1136c1_asm_113734(cpu);
}

pub(crate) fn function1136c1_asm_113734(cpu: &mut Cpu) {
    cpu.pc = 0x7734;
    cpu.set_hl({ const N: i32 = (0xcb47 as i32) + (197 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = ((0x7b8eu32) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = (((0x7b8eu32) >> 8) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl({ const N: i32 = (0xcb47 as i32) + (199 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = ((0x7a70u32) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = (((0x7a70u32) >> 8) & 0x00ff) as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl({ const N: i32 = (0xcb47 as i32) + (160 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de({ const N: i32 = (0xcb47 as i32) + (209 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = 0x10;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    become function1136c1_asm_113751(cpu);
}

pub(crate) fn function1136c1_asm_113751(cpu: &mut Cpu) {
    cpu.pc = 0x7751;
    cpu.set_hl({ const N: i32 = (0xcb47 as i32) + (199 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.h = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function113909(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = (0xcb47 as i32) + (176 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.d = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl({ const N: i32 = (0xcb47 as i32) + (184 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function113a32(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.d = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = cpu.read_byte({ const N: i32 = (0xcb47 as i32) + (225 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((0) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become function1136c1_asm_11377c(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x0040 as u16);
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
    cpu.e = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.d = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    become function1136c1_asm_11377c(cpu);
}

pub(crate) fn function1136c1_asm_11377c(cpu: &mut Cpu) {
    cpu.pc = 0x777c;
    cpu.set_hl({ const N: i32 = (0xcb47 as i32) + (32 as i32); N as u16 });
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
    cpu.e = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.d = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl({ const N: i32 = (0xcb47 as i32) + (184 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function113a32(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = (0xcb47 as i32) + (197 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.d = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl({ const N: i32 = (0xcb47 as i32) + (184 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function113a32(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte({ const N: i32 = (0xcb47 as i32) + (199 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte({ const N: i32 = (0xcb47 as i32) + (200 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl({ const N: i32 = (0xcb47 as i32) + (184 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function113a40(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = (0xcb47 as i32) + (178 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.d = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl({ const N: i32 = (0xcb47 as i32) + (184 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function113a32(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = (0xcb47 as i32) + (176 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.d = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl({ const N: i32 = (0xcb47 as i32) + (184 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = 0x4;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = (0xcb47 as i32) + (197 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.h = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte({ const N: i32 = (0xcb47 as i32) + (198 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte({ const N: i32 = (0xcb47 as i32) + (197 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x8e;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(16);
        become function1136c1_asm_113751(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de({ const N: i32 = (0xcb47 as i32) + (209 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl({ const N: i32 = (0xcb47 as i32) + (160 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function113a32(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_de({ const N: i32 = (0xcb47 as i32) + (213 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function113a32(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_de({ const N: i32 = (0xcb47 as i32) + (217 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function113a32(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_de({ const N: i32 = (0xcb47 as i32) + (221 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function113a32(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = (0xcb47 as i32) + (225 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((1) & 7);
    let value = cpu.read_byte(cpu.hl());
    cpu.flag_z = (value & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(12);
    if cpu.flag_z {
        cpu.cycle(12);
        become function1136c1_asm_1137fc(cpu);
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
    cpu.cycle(16);
    become function1136c1_asm_113734(cpu);
}

pub(crate) fn function1136c1_asm_1137fc(cpu: &mut Cpu) {
    cpu.pc = 0x77fc;
    cpu.set_hl({ const N: i32 = (0xcb47 as i32) + (32 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de({ const N: i32 = (0xcb47 as i32) + (80 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(0x0030 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function113d66(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = (0xcb47 as i32) + (194 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.d = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl(0x7a55);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_string(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = (0xcb47 as i32) + (80 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(0x0020 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function113c8e(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte({ const N: i32 = (0xcb47 as i32) + (194 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte({ const N: i32 = (0xcb47 as i32) + (195 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.b = 0x12;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl({ const N: i32 = (0xcb47 as i32) + (80 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de({ const N: i32 = (0xcb47 as i32) + (32 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    become function1136c1_asm_11382d(cpu);
}

pub(crate) fn function1136c1_asm_11382d(cpu: &mut Cpu) {
    cpu.pc = 0x782d;
    cpu.a = 0x40;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.read_byte(cpu.hl());
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value.rotate_left(1);
    cpu.flag_z = false;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x80 != 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let mask = 1u8 << ((4) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become function1136c1_asm_113839(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let mask = 1u8 << ((6) & 7);
    cpu.c |= mask;
    cpu.pc += 2;
    cpu.cycle(8);
    become function1136c1_asm_113839(cpu);
}

pub(crate) fn function1136c1_asm_113839(cpu: &mut Cpu) {
    cpu.pc = 0x7839;
    let mask = 1u8 << ((2) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become function1136c1_asm_11383f(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let mask = 1u8 << ((5) & 7);
    cpu.c |= mask;
    cpu.pc += 2;
    cpu.cycle(8);
    become function1136c1_asm_11383f(cpu);
}

pub(crate) fn function1136c1_asm_11383f(cpu: &mut Cpu) {
    cpu.pc = 0x783f;
    let mask = 1u8 << ((0) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become function1136c1_asm_113845(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let mask = 1u8 << ((4) & 7);
    cpu.c |= mask;
    cpu.pc += 2;
    cpu.cycle(8);
    become function1136c1_asm_113845(cpu);
}

pub(crate) fn function1136c1_asm_113845(cpu: &mut Cpu) {
    cpu.pc = 0x7845;
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let mask = 1u8 << ((6) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become function1136c1_asm_11384c(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let mask = 1u8 << ((3) & 7);
    cpu.c |= mask;
    cpu.pc += 2;
    cpu.cycle(8);
    become function1136c1_asm_11384c(cpu);
}

pub(crate) fn function1136c1_asm_11384c(cpu: &mut Cpu) {
    cpu.pc = 0x784c;
    let mask = 1u8 << ((4) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become function1136c1_asm_113852(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let mask = 1u8 << ((2) & 7);
    cpu.c |= mask;
    cpu.pc += 2;
    cpu.cycle(8);
    become function1136c1_asm_113852(cpu);
}

pub(crate) fn function1136c1_asm_113852(cpu: &mut Cpu) {
    cpu.pc = 0x7852;
    let mask = 1u8 << ((2) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become function1136c1_asm_113858(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let mask = 1u8 << ((1) & 7);
    cpu.c |= mask;
    cpu.pc += 2;
    cpu.cycle(8);
    become function1136c1_asm_113858(cpu);
}

pub(crate) fn function1136c1_asm_113858(cpu: &mut Cpu) {
    cpu.pc = 0x7858;
    let mask = 1u8 << ((0) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become function1136c1_asm_11385e(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let mask = 1u8 << ((0) & 7);
    cpu.c |= mask;
    cpu.pc += 2;
    cpu.cycle(8);
    become function1136c1_asm_11385e(cpu);
}

pub(crate) fn function1136c1_asm_11385e(cpu: &mut Cpu) {
    cpu.pc = 0x785e;
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
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
        become function1136c1_asm_11382d(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.b = 0x12;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl({ const N: i32 = (0xcb47 as i32) + (115 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de({ const N: i32 = (0xcb47 as i32) + (67 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    become function1136c1_asm_11386c(cpu);
}

pub(crate) fn function1136c1_asm_11386c(cpu: &mut Cpu) {
    cpu.pc = 0x786c;
    cpu.a = 0x2;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.read_byte(cpu.hl());
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value.rotate_right(1);
    cpu.flag_z = false;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x01 != 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let mask = 1u8 << ((3) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become function1136c1_asm_113878(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let mask = 1u8 << ((1) & 7);
    cpu.c |= mask;
    cpu.pc += 2;
    cpu.cycle(8);
    become function1136c1_asm_113878(cpu);
}

pub(crate) fn function1136c1_asm_113878(cpu: &mut Cpu) {
    cpu.pc = 0x7878;
    let mask = 1u8 << ((5) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become function1136c1_asm_11387e(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let mask = 1u8 << ((2) & 7);
    cpu.c |= mask;
    cpu.pc += 2;
    cpu.cycle(8);
    become function1136c1_asm_11387e(cpu);
}

pub(crate) fn function1136c1_asm_11387e(cpu: &mut Cpu) {
    cpu.pc = 0x787e;
    let mask = 1u8 << ((7) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become function1136c1_asm_113884(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let mask = 1u8 << ((3) & 7);
    cpu.c |= mask;
    cpu.pc += 2;
    cpu.cycle(8);
    become function1136c1_asm_113884(cpu);
}

pub(crate) fn function1136c1_asm_113884(cpu: &mut Cpu) {
    cpu.pc = 0x7884;
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let mask = 1u8 << ((1) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become function1136c1_asm_11388b(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let mask = 1u8 << ((4) & 7);
    cpu.c |= mask;
    cpu.pc += 2;
    cpu.cycle(8);
    become function1136c1_asm_11388b(cpu);
}

pub(crate) fn function1136c1_asm_11388b(cpu: &mut Cpu) {
    cpu.pc = 0x788b;
    let mask = 1u8 << ((3) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become function1136c1_asm_113891(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let mask = 1u8 << ((5) & 7);
    cpu.c |= mask;
    cpu.pc += 2;
    cpu.cycle(8);
    become function1136c1_asm_113891(cpu);
}

pub(crate) fn function1136c1_asm_113891(cpu: &mut Cpu) {
    cpu.pc = 0x7891;
    let mask = 1u8 << ((5) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become function1136c1_asm_113897(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let mask = 1u8 << ((6) & 7);
    cpu.c |= mask;
    cpu.pc += 2;
    cpu.cycle(8);
    become function1136c1_asm_113897(cpu);
}

pub(crate) fn function1136c1_asm_113897(cpu: &mut Cpu) {
    cpu.pc = 0x7897;
    let mask = 1u8 << ((7) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become function1136c1_asm_11389d(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let mask = 1u8 << ((7) & 7);
    cpu.c |= mask;
    cpu.pc += 2;
    cpu.cycle(8);
    become function1136c1_asm_11389d(cpu);
}

pub(crate) fn function1136c1_asm_11389d(cpu: &mut Cpu) {
    cpu.pc = 0x789d;
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_sub(1));
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
        become function1136c1_asm_11386c(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.b = 0x10;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_de({ const N: i32 = (0xcb47 as i32) + (80 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl({ const N: i32 = (0xcb47 as i32) + (160 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_bc(0x0010 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0xc97f);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.h = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_string(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x24;
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
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.l = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.h = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = 0xff;
    cpu.pc += 2;
    cpu.cycle(8);
    become function1136c1_asm_1138c2(cpu);
}

pub(crate) fn function1136c1_asm_1138c2(cpu: &mut Cpu) {
    cpu.pc = 0x78c2;
    cpu.write_byte(cpu.hl(), cpu.a);
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
        become function1136c1_asm_1138c2(cpu);
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
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.b = 0x24;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl({ const N: i32 = (0xcb47 as i32) + (32 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de({ const N: i32 = (0xcb47 as i32) + (80 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    become function1136c1_asm_1138d0(cpu);
}

pub(crate) fn function1136c1_asm_1138d0(cpu: &mut Cpu) {
    cpu.pc = 0x78d0;
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.read_byte(cpu.hl());
    cpu.a ^= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.c = 0x0;
    cpu.pc += 2;
    cpu.cycle(8);
    let mask = 1u8 << ((0) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become function1136c1_asm_1138db(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let mask = 1u8 << ((3) & 7);
    cpu.c |= mask;
    cpu.pc += 2;
    cpu.cycle(8);
    become function1136c1_asm_1138db(cpu);
}

pub(crate) fn function1136c1_asm_1138db(cpu: &mut Cpu) {
    cpu.pc = 0x78db;
    let mask = 1u8 << ((3) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become function1136c1_asm_1138e1(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let mask = 1u8 << ((6) & 7);
    cpu.c |= mask;
    cpu.pc += 2;
    cpu.cycle(8);
    become function1136c1_asm_1138e1(cpu);
}

pub(crate) fn function1136c1_asm_1138e1(cpu: &mut Cpu) {
    cpu.pc = 0x78e1;
    let mask = 1u8 << ((6) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become function1136c1_asm_1138e7(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let mask = 1u8 << ((0) & 7);
    cpu.c |= mask;
    cpu.pc += 2;
    cpu.cycle(8);
    become function1136c1_asm_1138e7(cpu);
}

pub(crate) fn function1136c1_asm_1138e7(cpu: &mut Cpu) {
    cpu.pc = 0x78e7;
    let value = 0xb6;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.c;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
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
        become function1136c1_asm_1138d0(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl({ const N: i32 = (0xcb47 as i32) + (194 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.d = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl({ const N: i32 = (0xcb47 as i32) + (32 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(0x0024 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function113c8e(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x22;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 0xd;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 0xa;
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
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function113909(cpu: &mut Cpu) {
    cpu.pc = 0x7909;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11391e(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = 0xf0;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value.rotate_left(4);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.a;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function11392f(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function113973(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(16);
        become function1139b7(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become function1139de(cpu);
}

pub(crate) fn function11391e(cpu: &mut Cpu) {
    cpu.pc = 0x791e;
    let value = 0xf;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.d = 0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x7b70);
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
    cpu.set_de({ const N: i32 = (0xcb47 as i32) + (176 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = 0x8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become mobile_sdk_copy_bytes(cpu);
}

pub(crate) fn function11392f(cpu: &mut Cpu) {
    cpu.pc = 0x792f;
    cpu.set_hl({ const N: i32 = (0xcb47 as i32) + (178 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.h = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_de({ const N: i32 = (0xcb47 as i32) + (184 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = 0x4;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = (0xcb47 as i32) + (180 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.d = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl({ const N: i32 = (0xcb47 as i32) + (184 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function113a0b(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = (0xcb47 as i32) + (178 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.h = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_de({ const N: i32 = (0xcb47 as i32) + (188 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = 0x4;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = (0xcb47 as i32) + (188 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function113a1f(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = (0xcb47 as i32) + (182 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.d = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl({ const N: i32 = (0xcb47 as i32) + (188 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function113a0b(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = (0xcb47 as i32) + (184 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de({ const N: i32 = (0xcb47 as i32) + (188 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function113a15(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function113973(cpu: &mut Cpu) {
    cpu.pc = 0x7973;
    cpu.set_hl({ const N: i32 = (0xcb47 as i32) + (178 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.h = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_de({ const N: i32 = (0xcb47 as i32) + (184 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = 0x4;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = (0xcb47 as i32) + (182 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.d = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl({ const N: i32 = (0xcb47 as i32) + (184 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function113a0b(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = (0xcb47 as i32) + (182 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.h = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_de({ const N: i32 = (0xcb47 as i32) + (188 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = 0x4;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = (0xcb47 as i32) + (188 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function113a1f(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = (0xcb47 as i32) + (180 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.d = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl({ const N: i32 = (0xcb47 as i32) + (188 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function113a0b(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = (0xcb47 as i32) + (184 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de({ const N: i32 = (0xcb47 as i32) + (188 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function113a15(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function1139b7(cpu: &mut Cpu) {
    cpu.pc = 0x79b7;
    cpu.set_hl({ const N: i32 = (0xcb47 as i32) + (178 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.h = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_de({ const N: i32 = (0xcb47 as i32) + (184 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = 0x4;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = (0xcb47 as i32) + (180 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.d = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl({ const N: i32 = (0xcb47 as i32) + (184 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function113a28(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = (0xcb47 as i32) + (182 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.d = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl({ const N: i32 = (0xcb47 as i32) + (184 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function113a28(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function1139de(cpu: &mut Cpu) {
    cpu.pc = 0x79de;
    cpu.set_hl({ const N: i32 = (0xcb47 as i32) + (182 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.h = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_de({ const N: i32 = (0xcb47 as i32) + (184 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = 0x4;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    mobile_sdk_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = (0xcb47 as i32) + (184 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function113a1f(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = (0xcb47 as i32) + (178 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.d = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl({ const N: i32 = (0xcb47 as i32) + (184 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function113a15(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = (0xcb47 as i32) + (180 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.d = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl({ const N: i32 = (0xcb47 as i32) + (184 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function113a28(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function113a0b(cpu: &mut Cpu) {
    cpu.pc = 0x7a0b;
    cpu.b = 0x4;
    cpu.pc += 2;
    cpu.cycle(8);
    become function113a0b_asm_113a0d(cpu);
}

pub(crate) fn function113a0b_asm_113a0d(cpu: &mut Cpu) {
    cpu.pc = 0x7a0d;
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.read_byte(cpu.hl());
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
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
        become function113a0b_asm_113a0d(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function113a15(cpu: &mut Cpu) {
    cpu.pc = 0x7a15;
    cpu.b = 0x4;
    cpu.pc += 2;
    cpu.cycle(8);
    become function113a15_asm_113a17(cpu);
}

pub(crate) fn function113a15_asm_113a17(cpu: &mut Cpu) {
    cpu.pc = 0x7a17;
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
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
    cpu.write_byte(cpu.hl(), cpu.a);
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
        become function113a15_asm_113a17(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function113a1f(cpu: &mut Cpu) {
    cpu.pc = 0x7a1f;
    cpu.b = 0x4;
    cpu.pc += 2;
    cpu.cycle(8);
    become function113a1f_asm_113a21(cpu);
}

pub(crate) fn function113a1f_asm_113a21(cpu: &mut Cpu) {
    cpu.pc = 0x7a21;
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = !cpu.a;
    cpu.flag_n = true;
    cpu.flag_h = true;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
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
        become function113a1f_asm_113a21(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function113a28(cpu: &mut Cpu) {
    cpu.pc = 0x7a28;
    cpu.b = 0x4;
    cpu.pc += 2;
    cpu.cycle(8);
    become function113a28_asm_113a2a(cpu);
}

pub(crate) fn function113a28_asm_113a2a(cpu: &mut Cpu) {
    cpu.pc = 0x7a2a;
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.read_byte(cpu.hl());
    cpu.a ^= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
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
        become function113a28_asm_113a2a(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function113a32(cpu: &mut Cpu) {
    cpu.pc = 0x7a32;
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
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
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.b = 0x3;
    cpu.pc += 2;
    cpu.cycle(8);
    become function113a32_asm_113a38(cpu);
}

pub(crate) fn function113a32_asm_113a38(cpu: &mut Cpu) {
    cpu.pc = 0x7a38;
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.read_byte(cpu.hl());
    let carry_in = u8::from(cpu.flag_c);
    let lhs = cpu.a;
    let (partial, carry1) = lhs.overflowing_add(value);
    let (result, carry2) = partial.overflowing_add(carry_in);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) + carry_in > 0x0f;
    cpu.flag_c = carry1 || carry2;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
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
        become function113a32_asm_113a38(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function113a40(cpu: &mut Cpu) {
    cpu.pc = 0x7a40;
    become function113a40_asm_113a40(cpu);
}

pub(crate) fn function113a40_asm_113a40(cpu: &mut Cpu) {
    cpu.pc = 0x7a40;
    let value = cpu.a;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.a;
    let carry_in = u8::from(cpu.flag_c);
    let result = (value << 1) | carry_in;
    cpu.flag_z = false;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x80 != 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.a;
    let carry_in = u8::from(cpu.flag_c);
    let result = (value << 1) | carry_in;
    cpu.flag_z = false;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x80 != 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.a;
    let carry_in = u8::from(cpu.flag_c);
    let result = (value << 1) | carry_in;
    cpu.flag_z = false;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x80 != 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.a;
    let carry_in = u8::from(cpu.flag_c);
    let result = (value << 1) | carry_in;
    cpu.flag_z = false;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x80 != 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.a;
    let carry_in = u8::from(cpu.flag_c);
    let result = (value << 1) | carry_in;
    cpu.flag_z = false;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x80 != 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
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
        become function113a40_asm_113a40(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function113c8e(cpu: &mut Cpu) {
    cpu.pc = 0x7c8e;
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte({ const N: i32 = (0xcb47 as i32) + (201 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte({ const N: i32 = (0xcb47 as i32) + (202 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.c = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.b = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.e = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.d = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.l = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.h = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte({ const N: i32 = (0xcb47 as i32) + (207 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    become function113c8e_asm_113ca0(cpu);
}

pub(crate) fn function113c8e_asm_113ca0(cpu: &mut Cpu) {
    cpu.pc = 0x7ca0;
    cpu.b = 0x3;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.set_hl({ const N: i32 = (0xcb47 as i32) + (203 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    become function113c8e_asm_113ca6(cpu);
}

pub(crate) fn function113c8e_asm_113ca6(cpu: &mut Cpu) {
    cpu.pc = 0x7ca6;
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
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
        become function113c8e_asm_113ca6(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte({ const N: i32 = (0xcb47 as i32) + (201 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte({ const N: i32 = (0xcb47 as i32) + (202 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.b;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(12);
        become function113c8e_asm_113ccf(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x2;
    cpu.pc += 2;
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
    if cpu.flag_c {
        cpu.cycle(12);
        become function113c8e_asm_113ccf(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte({ const N: i32 = (0xcb47 as i32) + (207 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    become function113c8e_asm_113cc3(cpu);
}

pub(crate) fn function113c8e_asm_113cc3(cpu: &mut Cpu) {
    cpu.pc = 0x7cc3;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.c;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.c = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = 0x3;
    cpu.pc += 2;
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
        become function113c8e_asm_113cc3(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.set_bc(0x0003 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    become function113c8e_asm_113ccf(cpu);
}

pub(crate) fn function113c8e_asm_113ccf(cpu: &mut Cpu) {
    cpu.pc = 0x7ccf;
    cpu.set_bc(cpu.bc().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_bc(cpu.bc().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_bc(cpu.bc().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte({ const N: i32 = (0xcb47 as i32) + (201 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte({ const N: i32 = (0xcb47 as i32) + (202 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.push_word(cpu.de());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.c = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.b = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.d = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.a;
    let result = value >> 1;
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x01 != 0;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value >> 1;
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x01 != 0;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 0x3;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.d;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.d = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = 0xf0;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.b;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.d;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.a;
    let result = value.rotate_left(4);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 0xf;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.b;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.d = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = 0xc0;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.d;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.a;
    let result = value.rotate_left(1);
    cpu.flag_z = false;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x80 != 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.a;
    let result = value.rotate_left(1);
    cpu.flag_z = false;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x80 != 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 0x3f;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.c;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.pop_word();
    cpu.set_de(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.b = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.c = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_bc(cpu.bc().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function113d47(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_bc(cpu.bc().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function113d47(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_bc(cpu.bc().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function113d47(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_bc(cpu.bc().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function113d47(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte({ const N: i32 = (0xcb47 as i32) + (201 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x0;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(16);
        become function113c8e_asm_113ca0(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte({ const N: i32 = (0xcb47 as i32) + (202 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x0;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(16);
        become function113c8e_asm_113ca0(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte({ const N: i32 = (0xcb47 as i32) + (207 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x0;
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
        become function113c8e_asm_113d43(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    become function113c8e_asm_113d39(cpu);
}

pub(crate) fn function113c8e_asm_113d39(cpu: &mut Cpu) {
    cpu.pc = 0x7d39;
    cpu.a = 0x3d;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.b;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.b = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = 0x3;
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
        cpu.cycle(12);
        become function113c8e_asm_113d39(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    become function113c8e_asm_113d43(cpu);
}

pub(crate) fn function113c8e_asm_113d43(cpu: &mut Cpu) {
    cpu.pc = 0x7d43;
    cpu.a = 0x0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function113d47(cpu: &mut Cpu) {
    cpu.pc = 0x7d47;
    let value = 0x1a;
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
        become function113d47_asm_113d5a(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x34;
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
        become function113d47_asm_113d5d(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x3e;
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
        become function113d47_asm_113d60(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x3e;
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
        become function113d47_asm_113d63(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x2f;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function113d47_asm_113d5a(cpu: &mut Cpu) {
    cpu.pc = 0x7d5a;
    let value = 0x41;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_add(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) > 0x0f;
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function113d47_asm_113d5d(cpu: &mut Cpu) {
    cpu.pc = 0x7d5d;
    let value = 0x47;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_add(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) > 0x0f;
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function113d47_asm_113d60(cpu: &mut Cpu) {
    cpu.pc = 0x7d60;
    let value = 0x4;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function113d47_asm_113d63(cpu: &mut Cpu) {
    cpu.pc = 0x7d63;
    cpu.a = 0x2b;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function113d66(cpu: &mut Cpu) {
    cpu.pc = 0x7d66;
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte({ const N: i32 = (0xcb47 as i32) + (201 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte({ const N: i32 = (0xcb47 as i32) + (202 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.c = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.b = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.e = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.d = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.l = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.h = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    become function113d66_asm_113d74(cpu);
}

pub(crate) fn function113d66_asm_113d74(cpu: &mut Cpu) {
    cpu.pc = 0x7d74;
    cpu.a = cpu.read_byte({ const N: i32 = (0xcb47 as i32) + (202 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.a;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(12);
        become function113d66_asm_113d82(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte({ const N: i32 = (0xcb47 as i32) + (201 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x4;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_c {
        cpu.cycle(16);
        become function113d66_asm_113e26(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    become function113d66_asm_113d82(cpu);
}

pub(crate) fn function113d66_asm_113d82(cpu: &mut Cpu) {
    cpu.pc = 0x7d82;
    cpu.b = 0x4;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.set_hl({ const N: i32 = (0xcb47 as i32) + (203 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    become function113d66_asm_113d88(cpu);
}

pub(crate) fn function113d66_asm_113d88(cpu: &mut Cpu) {
    cpu.pc = 0x7d88;
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function113d66_decode_base64_character(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.write_byte(cpu.hl(), cpu.a);
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
        become function113d66_asm_113d88(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte({ const N: i32 = (0xcb47 as i32) + (201 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte({ const N: i32 = (0xcb47 as i32) + (202 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_bc(cpu.bc().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_bc(cpu.bc().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_bc(cpu.bc().wrapping_sub(1));
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
    if cpu.flag_z {
        cpu.cycle(12);
        become function113d66_asm_113dae(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become function113d66_asm_113da1(cpu);
}

pub(crate) fn function113d66_asm_113da1(cpu: &mut Cpu) {
    cpu.pc = 0x7da1;
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0xd;
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
        become function113d66_asm_113daa(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0xa;
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
        become function113d66_asm_113dae(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become function113d66_asm_113daa(cpu);
}

pub(crate) fn function113d66_asm_113daa(cpu: &mut Cpu) {
    cpu.pc = 0x7daa;
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_bc(cpu.bc().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.cycle(12);
    become function113d66_asm_113da1(cpu);
}

pub(crate) fn function113d66_asm_113dae(cpu: &mut Cpu) {
    cpu.pc = 0x7dae;
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte({ const N: i32 = (0xcb47 as i32) + (201 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte({ const N: i32 = (0xcb47 as i32) + (202 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.push_word(cpu.de());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.d = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.c = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.b = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.b;
    let result = value << 1;
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x80 != 0;
    cpu.b = result;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.b;
    let result = value << 1;
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x80 != 0;
    cpu.b = result;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.b;
    let result = value << 1;
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x80 != 0;
    cpu.b = result;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.a;
    let carry_in = u8::from(cpu.flag_c);
    let result = (value << 1) | carry_in;
    cpu.flag_z = false;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x80 != 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.b;
    let result = value << 1;
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x80 != 0;
    cpu.b = result;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.a;
    let carry_in = u8::from(cpu.flag_c);
    let result = (value << 1) | carry_in;
    cpu.flag_z = false;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x80 != 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.b);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.c;
    let result = value.rotate_right(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x01 != 0;
    cpu.c = result;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.c;
    let result = value.rotate_right(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x01 != 0;
    cpu.c = result;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.c);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 0xf;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.c;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.read_byte(cpu.hl());
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0xc0;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.read_byte(cpu.hl());
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.pop_word();
    cpu.set_de(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.b = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.c = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_bc(cpu.bc().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_bc(cpu.bc().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte({ const N: i32 = (0xcb47 as i32) + (201 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.a;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(12);
        become function113d66_asm_113d74(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte({ const N: i32 = (0xcb47 as i32) + (202 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.a;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(16);
        become function113d66_asm_113d74(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function113d66_decode_base64_character(cpu: &mut Cpu) {
    cpu.pc = 0x7dfa;
    let value = 0x2b;
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
        become function113d66_asm_113e24(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become function113d66_asm_113e31(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = { let value = (0xf3 as i32); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
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
        become function113d66_asm_113e24(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become function113d66_asm_113e34(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = { let value = (0xf6 as i32); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
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
        become function113d66_asm_113e24(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = { let value = ((0xff as i32) + (1 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
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
        become function113d66_asm_113e37(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x3d;
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
        become function113d66_asm_113e24(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become function113d66_asm_113e3a(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = { let value = (0x80 as i32); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
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
        become function113d66_asm_113e24(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = { let value = ((0x99 as i32) + (1 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
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
        become function113d66_asm_113e3c(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = { let value = (0xa0 as i32); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
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
        become function113d66_asm_113e24(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = { let value = ((0xb9 as i32) + (1 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
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
        become function113d66_asm_113e3f(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become function113d66_asm_113e24(cpu);
}

pub(crate) fn function113d66_asm_113e24(cpu: &mut Cpu) {
    cpu.pc = 0x7e24;
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    become function113d66_asm_113e26(cpu);
}

pub(crate) fn function113d66_asm_113e26(cpu: &mut Cpu) {
    cpu.pc = 0x7e26;
    cpu.set_hl(0xc821);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((1) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value |= mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.a = 0x20;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc80f, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function113d66_asm_113e31(cpu: &mut Cpu) {
    cpu.pc = 0x7e31;
    cpu.a = 0x3e;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function113d66_asm_113e34(cpu: &mut Cpu) {
    cpu.pc = 0x7e34;
    cpu.a = 0x3f;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function113d66_asm_113e37(cpu: &mut Cpu) {
    cpu.pc = 0x7e37;
    let value = 0x4;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_add(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) > 0x0f;
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function113d66_asm_113e3a(cpu: &mut Cpu) {
    cpu.pc = 0x7e3a;
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

pub(crate) fn function113d66_asm_113e3c(cpu: &mut Cpu) {
    cpu.pc = 0x7e3c;
    let value = 0x41;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function113d66_asm_113e3f(cpu: &mut Cpu) {
    cpu.pc = 0x7e3f;
    let value = 0x47;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function113e42(cpu: &mut Cpu) {
    cpu.pc = 0x7e42;
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function113e42_asm_113e4f(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function113e42_asm_113e85(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function113e42_asm_113ea8(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function113e42_asm_113eb0(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function113e42_asm_113e4f(cpu: &mut Cpu) {
    cpu.pc = 0x7e4f;
    cpu.a = cpu.read_byte(0xc807);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x8;
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
        become function113e42_asm_113e58(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become function113e42_asm_113e56(cpu);
}

pub(crate) fn function113e42_asm_113e56(cpu: &mut Cpu) {
    cpu.pc = 0x7e56;
    let addr = cpu.hl();
    let value = cpu.read_byte(addr);
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.write_byte(addr, result);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function113e42_asm_113e58(cpu: &mut Cpu) {
    cpu.pc = 0x7e58;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc86d, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x2;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc86a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xc821);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0x10;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    let mask = 1u8 << ((5) & 7);
    cpu.a |= mask;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.cycle(16);
    become function113eb8(cpu);
}

pub(crate) fn function113e42_asm_113e6d(cpu: &mut Cpu) {
    cpu.pc = 0x7e6d;
    cpu.a = cpu.read_byte(0xc86d);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.a;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xca3c);
    cpu.pc += 3;
    cpu.cycle(16);
    if cpu.flag_z {
        cpu.cycle(12);
        become function113e42_asm_113e81(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0x9f;
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
        become function113e42_asm_113e85(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 0xa4;
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
        become function113e42_asm_113e85(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become function113e42_asm_113e7e(cpu);
}

pub(crate) fn function113e42_asm_113e7e(cpu: &mut Cpu) {
    cpu.pc = 0x7e7e;
    cpu.cycle(16);
    become function112430(cpu);
}

pub(crate) fn function113e42_asm_113e81(cpu: &mut Cpu) {
    cpu.pc = 0x7e81;
    let value = 0xa3;
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
        become function113e42_asm_113e7e(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become function113e42_asm_113e85(cpu);
}

pub(crate) fn function113e42_asm_113e85(cpu: &mut Cpu) {
    cpu.pc = 0x7e85;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc86d, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xc81e, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x2;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc86a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x3;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc807, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xc821);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0x10;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    let mask = 1u8 << ((5) & 7);
    cpu.a |= mask;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(0xc822);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((0) & 7);
    let value = cpu.read_byte(cpu.hl());
    cpu.flag_z = (value & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(12);
    if cpu.flag_z {
        let next_pc = cpu.pc + 3;
        cpu.push_word(next_pc);
        cpu.cycle(24);
        function111f97(cpu);
        assert_eq!(cpu.pc, next_pc);
    } else {
        cpu.pc += 3;
        cpu.cycle(12);
    }
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function113e42_asm_113ea8(cpu: &mut Cpu) {
    cpu.pc = 0x7ea8;
    cpu.a = cpu.read_byte(0xc807);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x8;
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
        become function113e42_asm_113e56(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function113e42_asm_113eb0(cpu: &mut Cpu) {
    cpu.pc = 0x7eb0;
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc86b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(16);
    become function113e42_asm_113e6d(cpu);
}

pub(crate) fn function113eb8(cpu: &mut Cpu) {
    cpu.pc = 0x7eb8;
    cpu.a = 0xff;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc81e, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xc822);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((5) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    let mask = 1u8 << ((0) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.cycle(16);
    become function111f97(cpu);
}

pub(crate) fn function113ec7(cpu: &mut Cpu) {
    cpu.pc = 0x7ec7;
    cpu.set_hl(0xc822);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    let mask = 1u8 << ((3) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    let mask = 1u8 << ((0) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.set_hl(0xc81a);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.d = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.h = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = 0x80;
    cpu.a ^= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc81e, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.b = 0x5;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    packet_send_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let mask = 1u8 << ((0) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(0xc822);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((0) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value |= mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function113ef2(cpu: &mut Cpu) {
    cpu.pc = 0x7ef2;
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function113ef2_asm_113efa(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function113ef2_asm_113f03(cpu);
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
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function113ef2_asm_113efa(cpu: &mut Cpu) {
    cpu.pc = 0x7efa;
    cpu.a = cpu.read_byte(0xc807);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x8;
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
        become function113eb8(cpu);
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
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function113ef2_asm_113f03(cpu: &mut Cpu) {
    cpu.pc = 0x7f03;
    cpu.a = 0x26;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    function11225d(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x2a;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc86a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xc820);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.h = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.e = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.d = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    let lhs = cpu.hl();
    let rhs = cpu.de();
    let (result, carry) = lhs.overflowing_add(rhs);
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0fff) + (rhs & 0x0fff) > 0x0fff;
    cpu.flag_c = carry;
    cpu.set_hl(result);
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
    cpu.e = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.d = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl(0xc815);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc800, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xcb47);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0x2;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
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
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function113f2d(cpu: &mut Cpu) {
    cpu.pc = 0x7f2d;
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function113f2d_asm_113f35(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.a;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become function113f2d_asm_113f4f(cpu);
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
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function113f2d_asm_113f35(cpu: &mut Cpu) {
    cpu.pc = 0x7f35;
    cpu.a = cpu.read_byte(0xc807);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x8;
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
        become function113f2d_asm_113f3e(cpu);
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
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function113f2d_asm_113f3e(cpu: &mut Cpu) {
    cpu.pc = 0x7f3e;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xca3c, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xca2f, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xc81e);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 0x91;
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
        become function113f2d_asm_113f4f(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become function113eb8(cpu);
}

pub(crate) fn function113f2d_asm_113f4f(cpu: &mut Cpu) {
    cpu.pc = 0x7f4f;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc86d, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xc821);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((0) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value |= mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.set_hl(0xc822);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc80b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcb47, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xc820);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.h = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.e = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.d = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    let lhs = cpu.hl();
    let rhs = cpu.de();
    let (result, carry) = lhs.overflowing_add(rhs);
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0fff) + (rhs & 0x0fff) > 0x0fff;
    cpu.flag_c = carry;
    cpu.set_hl(result);
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
    cpu.e = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.d = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl(0xc815);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc800, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xcb47);
    cpu.pc += 3;
    cpu.cycle(12);
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
    let value = cpu.a;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}
