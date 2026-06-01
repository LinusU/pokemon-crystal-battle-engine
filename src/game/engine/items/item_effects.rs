use crate::runtime::Cpu;

pub const CURRENT_TABLE_WIDTH: u8 = 2;

pub const W: u8 = 179;

pub const X: u16 = 358;

pub const Y: i32 = 0;

// duplicate assembler constant X

pub fn _do_item_effect(cpu: &mut Cpu) {
    cpu.pc = 0x6722;
    cpu.a = cpu.read_byte(0xd106);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xd265, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::names::get_item_name(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy_name::copy_name1(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xd0ec, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xd106);
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
    cpu.set_hl(0x673c);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::jump_table(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn poke_ball_effect(cpu: &mut Cpu) {
    cpu.pc = 0x68a2;
    cpu.a = cpu.read_byte(0xd22d);
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
    if !cpu.flag_z {
        cpu.cycle(16);
        become use_ball_in_trainer_battle(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xdcd7);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = crate::defs::PARTY_LENGTH as u8;
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
        become poke_ball_effect_room_in_party(cpu);
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
    cpu.a = cpu.read_byte(0xad10);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = crate::defs::MONS_PER_BOX as u8;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::close_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_z {
        cpu.cycle(16);
        become ball_box_is_full_message(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    become poke_ball_effect_room_in_party(cpu);
}

pub(crate) fn poke_ball_effect_room_in_party(cpu: &mut Cpu) {
    cpu.pc = 0x68c0;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc64e, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xd106);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = crate::defs::PARK_BALL as u8;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        let next_pc = cpu.pc + 3;
        cpu.push_word(next_pc);
        cpu.cycle(24);
        return_to_battle_use_ball(cpu);
        assert_eq!(cpu.pc, next_pc);
    } else {
        cpu.pc += 3;
        cpu.cycle(12);
    }
    cpu.set_hl(0xcfcc);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((crate::defs::NO_TEXT_SCROLL as u8) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.set_hl(0x783d);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::print_text(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xd22b);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xd230);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = crate::defs::BATTLETYPE_TUTORIAL as u8;
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
        become poke_ball_effect_catch_without_fail(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xd106);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = crate::defs::MASTER_BALL as u8;
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
        become poke_ball_effect_catch_without_fail(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xd106);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl(0x6c0a);
    cpu.pc += 3;
    cpu.cycle(12);
    become poke_ball_effect_get_multiplier_loop(cpu);
}

pub(crate) fn poke_ball_effect_get_multiplier_loop(cpu: &mut Cpu) {
    cpu.pc = 0x68f2;
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
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
        become poke_ball_effect_skip_or_return_from_ball_fn(cpu);
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
        become poke_ball_effect_call_ball_function(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.cycle(12);
    become poke_ball_effect_get_multiplier_loop(cpu);
}

pub(crate) fn poke_ball_effect_skip_or_return_from_ball_fn(cpu: &mut Cpu) {
    cpu.pc = 0x6906;
    cpu.a = cpu.read_byte(0xd106);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = crate::defs::LEVEL_BALL as u8;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(16);
        become poke_ball_effect_skip_hp_calc(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte({ const N: i32 = (0xffb4 as i32) + (2 as i32); N as u16 }, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.set_hl(0xd216);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.c = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.d = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.e = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
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
    cpu.h = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.l = cpu.e;
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
    cpu.d = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.e = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.d;
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
        become poke_ball_effect_okay_1(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.d;
    let result = value >> 1;
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x01 != 0;
    cpu.d = result;
    cpu.pc += 2;
    cpu.cycle(8);
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
    let value = cpu.d;
    let result = value >> 1;
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x01 != 0;
    cpu.d = result;
    cpu.pc += 2;
    cpu.cycle(8);
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
    let result = value >> 1;
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x01 != 0;
    cpu.b = result;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.c;
    let carry_in = u8::from(cpu.flag_c) << 7;
    let result = (value >> 1) | carry_in;
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x01 != 0;
    cpu.c = result;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.b;
    let result = value >> 1;
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x01 != 0;
    cpu.b = result;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.c;
    let carry_in = u8::from(cpu.flag_c) << 7;
    let result = (value >> 1) | carry_in;
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x01 != 0;
    cpu.c = result;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.c;
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
        become poke_ball_effect_okay_1(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.c = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    become poke_ball_effect_okay_1(cpu);
}

pub(crate) fn poke_ball_effect_okay_1(cpu: &mut Cpu) {
    cpu.pc = 0x6940;
    cpu.b = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.push_word(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);
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
    cpu.write_byte(0xffb7, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte({ const N: i32 = (0xffb3 as i32) + (0 as i32); N as u16 }, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.write_byte({ const N: i32 = (0xffb4 as i32) + (0 as i32); N as u16 }, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.write_byte({ const N: i32 = (0xffb4 as i32) + (1 as i32); N as u16 }, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::math::multiply(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_bc(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xffb7, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.b = 4;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::math::divide(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte({ const N: i32 = (0xffb3 as i32) + (3 as i32); N as u16 });
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
        become poke_ball_effect_statuscheck(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 1;
    cpu.pc += 2;
    cpu.cycle(8);
    become poke_ball_effect_statuscheck(cpu);
}

pub(crate) fn poke_ball_effect_statuscheck(cpu: &mut Cpu) {
    cpu.pc = 0x6960;
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xd214);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = { let value = (((1 as i32) << (crate::defs::FRZ as i32)) | (crate::defs::SLP_MASK as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.c = 10;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(12);
        become poke_ball_effect_addstatus(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.a;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.c = 5;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(12);
        become poke_ball_effect_addstatus(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.c = 0;
    cpu.pc += 2;
    cpu.cycle(8);
    become poke_ball_effect_addstatus(cpu);
}

pub(crate) fn poke_ball_effect_addstatus(cpu: &mut Cpu) {
    cpu.pc = 0x6971;
    cpu.a = cpu.b;
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
    if !cpu.flag_c {
        cpu.cycle(12);
        become poke_ball_effect_max_1(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0xff;
    cpu.pc += 2;
    cpu.cycle(8);
    become poke_ball_effect_max_1(cpu);
}

pub(crate) fn poke_ball_effect_max_1(cpu: &mut Cpu) {
    cpu.pc = 0x6977;
    cpu.d = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.push_word(cpu.de());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xc62d);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x0d;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x7dd0);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = crate::defs::HELD_CATCH_CHANCE as u8;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.pop_word();
    cpu.set_de(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.a = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(12);
        become poke_ball_effect_max_2(cpu);
    }
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
    if !cpu.flag_c {
        cpu.cycle(12);
        become poke_ball_effect_max_2(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0xff;
    cpu.pc += 2;
    cpu.cycle(8);
    become poke_ball_effect_max_2(cpu);
}

pub(crate) fn poke_ball_effect_max_2(cpu: &mut Cpu) {
    cpu.pc = 0x698e;
    become poke_ball_effect_skip_hp_calc(cpu);
}

pub(crate) fn poke_ball_effect_skip_hp_calc(cpu: &mut Cpu) {
    cpu.pc = 0x698e;
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xd1ea, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::random::random(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.b;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = 0;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become poke_ball_effect_catch_without_fail(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_c {
        cpu.cycle(12);
        become poke_ball_effect_fail_to_catch(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become poke_ball_effect_catch_without_fail(cpu);
}

pub(crate) fn poke_ball_effect_catch_without_fail(cpu: &mut Cpu) {
    cpu.pc = 0x699c;
    cpu.a = cpu.read_byte(0xd206);
    cpu.pc += 3;
    cpu.cycle(16);
    become poke_ball_effect_fail_to_catch(cpu);
}

pub(crate) fn poke_ball_effect_fail_to_catch(cpu: &mut Cpu) {
    cpu.pc = 0x699f;
    cpu.write_byte(0xc64e, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.c = 20;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::delay::delay_frames(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xd106);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = { let value = ((crate::defs::POKE_BALL as i32) + (1 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
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
        become poke_ball_effect_not_kurt_ball(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = crate::defs::POKE_BALL as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    become poke_ball_effect_not_kurt_ball(cpu);
}

pub(crate) fn poke_ball_effect_not_kurt_ball(cpu: &mut Cpu) {
    cpu.pc = 0x69b0;
    cpu.write_byte(0xc689, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_de(crate::defs::ANIM_THROW_POKE_BALL as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcfc2, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte({ const N: i32 = (0xcfc2 as i32) + (1 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xffe4, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.write_byte(0xd1eb, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xcfca, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = { let value = (((0x4610 as i32) - (0x456b as i32)) / (3 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    // HARNESS-PATCH: skip the ball-throw battle animation (`predef PlayBattleAnim`
    // with `ANIM_THROW_POKE_BALL`; asm engine/items/item_effects.asm:406). Headless,
    // its OAM-update loop (`battle_anim_oam_update_loop`) never terminates — the
    // sprite slots it polls are serviced by the on-screen OAM pipeline we don't run.
    // (Ordinary move animations take a different, terminating script and run fine, so
    // this is scoped to the ball throw only, not PlayBattleAnim in general.) The
    // authoritative catch result already lives in `wWildMon` (0xc64e), written by
    // `.catch_without_fail` / `.fail_to_catch` BEFORE this call, and that is exactly
    // what the code right below reads to branch to `.caught` vs the break-free path.
    // The animation's only side effect on control flow is `wThrownBallWobbleCount`
    // (0xd1eb): the break-free path (below) reads it to pick which "broke free" flavour
    // text to print, treating 1..=4 as "escaped" and falling through to `.caught`
    // otherwise. It was just zeroed (`xor a; ld [wThrownBallWobbleCount], a` above), so
    // if we skipped the anim AND broke free it would wrongly fall through to `.caught`.
    // To keep the catch/escape decision correct we set it to 1 ("broke free") on an
    // escape; on a catch `wWildMon != 0` short-circuits to `.caught` and the value is
    // ignored. Then skip the `predef` call (don't push/invoke), leaving PC where it
    // would have returned. See docs/transpiled_code.md and bootstrap::cli.
    if cpu.read_byte(0xc64e) == 0 {
        cpu.write_byte(0xd1eb, 1);
    }
    cpu.pc = next_pc;
    cpu.cycle(24);
    cpu.a = cpu.read_byte(0xc64e);
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
        become poke_ball_effect_caught(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xd1eb);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 1;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x6db5);
    cpu.pc += 3;
    cpu.cycle(12);
    if cpu.flag_z {
        cpu.cycle(16);
        become poke_ball_effect_shake_and_break_free(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    let value = 2;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x6dba);
    cpu.pc += 3;
    cpu.cycle(12);
    if cpu.flag_z {
        cpu.cycle(16);
        become poke_ball_effect_shake_and_break_free(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    let value = 3;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x6dbf);
    cpu.pc += 3;
    cpu.cycle(12);
    if cpu.flag_z {
        cpu.cycle(16);
        become poke_ball_effect_shake_and_break_free(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    let value = 4;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x6dc4);
    cpu.pc += 3;
    cpu.cycle(12);
    if cpu.flag_z {
        cpu.cycle(16);
        become poke_ball_effect_shake_and_break_free(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    become poke_ball_effect_caught(cpu);
}

pub(crate) fn poke_ball_effect_caught(cpu: &mut Cpu) {
    cpu.pc = 0x69f5;
    cpu.set_hl(0xd214);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.set_hl(0xd207);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.set_hl(0xc671);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    let mask = 1u8 << ((crate::defs::SUBSTATUS_TRANSFORMED as u8) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value |= mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    let mask = 1u8 << ((crate::defs::SUBSTATUS_TRANSFORMED as u8) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(12);
        become poke_ball_effect_ditto(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become poke_ball_effect_not_ditto(cpu);
}

pub(crate) fn poke_ball_effect_ditto(cpu: &mut Cpu) {
    cpu.pc = 0x6a13;
    cpu.a = crate::defs::DITTO as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xd204, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(12);
    become poke_ball_effect_load_data(cpu);
}

pub(crate) fn poke_ball_effect_not_ditto(cpu: &mut Cpu) {
    cpu.pc = 0x6a1a;
    let mask = 1u8 << ((crate::defs::SUBSTATUS_TRANSFORMED as u8) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value |= mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.set_hl(0xc6f2);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xd20c);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte({ const N: i32 = (0xd20c as i32) + (1 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    become poke_ball_effect_load_data(cpu);
}

pub(crate) fn poke_ball_effect_load_data(cpu: &mut Cpu) {
    cpu.pc = 0x6a27;
    cpu.a = cpu.read_byte(0xd204);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xd108, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xd213);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xd143, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x0f;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x68eb);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte(0xc671, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(0xc671);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((crate::defs::SUBSTATUS_TRANSFORMED as u8) & 7);
    let value = cpu.read_byte(cpu.hl());
    cpu.flag_z = (value & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(12);
    if !cpu.flag_z {
        cpu.cycle(12);
        become poke_ball_effect_transformed(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xc735);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xd208);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(crate::defs::NUM_MOVES as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xc739);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xd20e);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(crate::defs::NUM_MOVES as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    become poke_ball_effect_transformed(cpu);
}

pub(crate) fn poke_ball_effect_transformed(cpu: &mut Cpu) {
    cpu.pc = 0x6a67;
    cpu.a = cpu.read_byte(0xd206);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xc64e, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xd108, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xd265, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xd230);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = crate::defs::BATTLETYPE_TUTORIAL as u8;
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
        become poke_ball_effect_finish_tutorial(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0x41;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x607f);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0x6dc9);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::print_text(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::clear_sprites::clear_sprites(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xd265);
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
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::pokedex_flags::check_caught_mon(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xd265);
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
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::pokedex_flags::set_seen_and_caught_mon(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
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
        become poke_ball_effect_skip_pokedex(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::flag::check_received_dex(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_z {
        cpu.cycle(12);
        become poke_ball_effect_skip_pokedex(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x6df0);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::print_text(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::clear_sprites::clear_sprites(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xd206);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xd265, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = { let value = (((0x4634 as i32) - (0x456b as i32)) / (3 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::predef::predef(cpu);
    assert_eq!(cpu.pc, next_pc);
    become poke_ball_effect_skip_pokedex(cpu);
}

pub(crate) fn poke_ball_effect_skip_pokedex(cpu: &mut Cpu) {
    cpu.pc = 0x6ab7;
    cpu.a = cpu.read_byte(0xd230);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = crate::defs::BATTLETYPE_CONTEST as u8;
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
        become poke_ball_effect_catch_bug_contest_mon(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    let value = crate::defs::BATTLETYPE_CELEBI as u8;
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
        become poke_ball_effect_not_celebi(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xd0ee);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((crate::defs::BATTLERESULT_CAUGHT_CELEBI as u8) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value |= mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    become poke_ball_effect_not_celebi(cpu);
}

pub(crate) fn poke_ball_effect_not_celebi(cpu: &mut Cpu) {
    cpu.pc = 0x6ac8;
    cpu.a = cpu.read_byte(0xdcd7);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = crate::defs::PARTY_LENGTH as u8;
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
        become poke_ball_effect_send_to_pc(cpu);
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
    cpu.write_byte(0xcf5f, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::clear_sprites::clear_sprites(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = { let value = (((0x457d as i32) - (0x456b as i32)) / (3 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::predef::predef(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x13;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x5b49);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xd106);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = crate::defs::FRIEND_BALL as u8;
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
        become poke_ball_effect_skip_party_mon_friend_ball(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xdcd7);
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
    cpu.set_hl(0xdcfa);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(crate::defs::PARTYMON_STRUCT_LENGTH as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::array::add_n_times(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = crate::defs::FRIEND_BALL_HAPPINESS as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    become poke_ball_effect_skip_party_mon_friend_ball(cpu);
}

pub(crate) fn poke_ball_effect_skip_party_mon_friend_ball(cpu: &mut Cpu) {
    cpu.pc = 0x6af8;
    // HARNESS-PATCH: skip the post-capture nickname prompt when running headless.
    // By the time control reaches here the caught mon is already in the party —
    // `poke_ball_effect_not_celebi` ran `TryAddMonToParty` (predef) + `SetCaughtData`
    // and the Pokédex bookkeeping (`poke_ball_effect_transformed`) before tailing in.
    // This routine is ONLY the UI tail: `AskGiveNicknameText` (PrintText) → `YesNoBox`
    // → `NamingScreen` (far_call), all in the asm at engine/items/item_effects.asm
    // `.SkipPartyMonFriendBall` (~asm:573-607). `YesNoBox`/`NamingScreen` are
    // interactive menu loops that hang/garble a headless run (the auto-A in joypad.rs
    // can't drive a free-text NamingScreen). PrintText itself is already safe headless
    // (joypad auto-A advances text waits — see the joypad.rs patches), so the only
    // thing we must avoid is the nickname menu. Jump straight to the asm's
    // `.return_from_capture`, which ends the capture and tosses the ball
    // (`TossItem`). See docs/transpiled_code.md.
    become poke_ball_effect_return_from_capture(cpu);
    cpu.set_hl(0x6df5);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::print_text(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xd108);
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
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::menu::yes_no_box(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.cycle(16);
        become poke_ball_effect_return_from_capture(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xdcd7);
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
    cpu.write_byte(0xd109, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xde41);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(crate::defs::MON_NAME_LENGTH as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::array::add_n_times(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.d = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.e = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.push_word(cpu.de());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcf5f, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.b = crate::defs::NAME_MON as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x04;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x56c1);
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
    crate::game::home::fade::rotate_three_palettes_right(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::gfx::load_standard_font(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.set_de(0xd073);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::string::init_name(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(16);
    become poke_ball_effect_return_from_capture(cpu);
}

pub(crate) fn poke_ball_effect_send_to_pc(cpu: &mut Cpu) {
    cpu.pc = 0x6b3c;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::clear_sprites::clear_sprites(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = { let value = (((0x4586 as i32) - (0x456b as i32)) / (3 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::predef::predef(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x13;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x5b83);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x01;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xad10);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = crate::defs::MONS_PER_BOX as u8;
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
        become poke_ball_effect_box_not_full_yet(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xd0ee);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((crate::defs::BATTLERESULT_BOX_FULL as u8) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value |= mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    become poke_ball_effect_box_not_full_yet(cpu);
}

pub(crate) fn poke_ball_effect_box_not_full_yet(cpu: &mut Cpu) {
    cpu.pc = 0x6b5b;
    cpu.a = cpu.read_byte(0xd106);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = crate::defs::FRIEND_BALL as u8;
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
        become poke_ball_effect_skip_box_mon_friend_ball(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = crate::defs::FRIEND_BALL_HAPPINESS as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xad41, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    become poke_ball_effect_skip_box_mon_friend_ball(cpu);
}

pub(crate) fn poke_ball_effect_skip_box_mon_friend_ball(cpu: &mut Cpu) {
    cpu.pc = 0x6b67;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::close_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    // HARNESS-PATCH: skip the post-capture nickname prompt on the box-full
    // (`.SendToPC`) path, the box analogue of the party-path skip above. The
    // caught mon is already in the box — `poke_ball_effect_send_to_pc` ran
    // `SendMonIntoBox` (predef) + `SetBoxMonCaughtData` before tailing in. The
    // `close_sram` just above balances the `OpenSRAM` that `.SendToPC` left open,
    // so we keep it; everything after it in the asm (`.SkipBoxMonNickname`,
    // ~asm:634-665) is `AskGiveNicknameText` → `YesNoBox` → `NamingScreen`, the
    // same interactive UI that hangs headless. Jump to `poke_ball_effect_skip_box_mon_nickname`
    // (the asm's `.SkipBoxMonNickname` label), which copies the default species
    // name, prints `BallSentToPCText`, and restores palettes/font — all headless-safe
    // — then falls through to `.return_from_capture`. This path is never exercised by
    // the current fixtures (party is never full), but is kept correct defensively.
    // See docs/transpiled_code.md.
    become poke_ball_effect_skip_box_mon_nickname(cpu);
    cpu.set_hl(0x6df5);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::print_text(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xd108);
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
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::menu::yes_no_box(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.cycle(12);
        become poke_ball_effect_skip_box_mon_nickname(cpu);
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
    cpu.write_byte(0xd109, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = crate::defs::BOXMON as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcf5f, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_de(0xd050);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = crate::defs::NAME_MON as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x04;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x56c1);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x01;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xd050);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xb082);
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
    cpu.set_hl(0xb082);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xd073);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::string::init_name(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::close_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    become poke_ball_effect_skip_box_mon_nickname(cpu);
}

pub(crate) fn poke_ball_effect_skip_box_mon_nickname(cpu: &mut Cpu) {
    cpu.pc = 0x6baf;
    cpu.a = 0x01;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::open_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xb082);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xd050);
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
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::sram::close_sram(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0x6deb);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::print_text(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::fade::rotate_three_palettes_right(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::gfx::load_standard_font(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(12);
    become poke_ball_effect_return_from_capture(cpu);
}

pub(crate) fn poke_ball_effect_catch_bug_contest_mon(cpu: &mut Cpu) {
    cpu.pc = 0x6bd1;
    cpu.a = 0x03;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x66ce);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(12);
    become poke_ball_effect_return_from_capture(cpu);
}

pub(crate) fn poke_ball_effect_finish_tutorial(cpu: &mut Cpu) {
    cpu.pc = 0x6bd9;
    cpu.set_hl(0x6dc9);
    cpu.pc += 3;
    cpu.cycle(12);
    become poke_ball_effect_shake_and_break_free(cpu);
}

pub(crate) fn poke_ball_effect_shake_and_break_free(cpu: &mut Cpu) {
    cpu.pc = 0x6bdc;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::print_text(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::clear_sprites::clear_sprites(cpu);
    assert_eq!(cpu.pc, next_pc);
    become poke_ball_effect_return_from_capture(cpu);
}

pub(crate) fn poke_ball_effect_return_from_capture(cpu: &mut Cpu) {
    cpu.pc = 0x6be2;
    cpu.a = cpu.read_byte(0xd230);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = crate::defs::BATTLETYPE_TUTORIAL as u8;
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
    let value = crate::defs::BATTLETYPE_DEBUG as u8;
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
    let value = crate::defs::BATTLETYPE_CONTEST as u8;
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
        become poke_ball_effect_used_park_ball(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc64e);
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
        become poke_ball_effect_toss(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::tilemap::clear_bg_palettes(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::clear_tilemap(cpu);
    assert_eq!(cpu.pc, next_pc);
    become poke_ball_effect_toss(cpu);
}

pub(crate) fn poke_ball_effect_toss(cpu: &mut Cpu) {
    cpu.pc = 0x6bfb;
    cpu.set_hl(0xd892);
    cpu.pc += 3;
    cpu.cycle(12);
    let value = cpu.a;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xd10c, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(16);
    become crate::game::home::item::toss_item(cpu);
}

pub(crate) fn poke_ball_effect_used_park_ball(cpu: &mut Cpu) {
    cpu.pc = 0x6c05;
    cpu.set_hl(0xdc79);
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

pub(crate) fn ultra_ball_multiplier(cpu: &mut Cpu) {
    cpu.pc = 0x6c29;
    let value = cpu.b;
    let result = value << 1;
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x80 != 0;
    cpu.b = result;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_c {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.b = 0xff;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn safari_ball_multiplier(cpu: &mut Cpu) {
    cpu.pc = 0x6c2f;
    become great_ball_multiplier(cpu);
}

pub(crate) fn great_ball_multiplier(cpu: &mut Cpu) {
    cpu.pc = 0x6c2f;
    become park_ball_multiplier(cpu);
}

pub(crate) fn park_ball_multiplier(cpu: &mut Cpu) {
    cpu.pc = 0x6c2f;
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
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_c {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.b = 0xff;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn heavy_ball_get_dex_entry_bank(cpu: &mut Cpu) {
    cpu.pc = 0x6c38;
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.push_word(cpu.de());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xd206);
    cpu.pc += 3;
    cpu.cycle(16);
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
    let value = 3;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x6c4c);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.d = 0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.e = cpu.a;
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
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.pop_word();
    cpu.set_de(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn heavy_ball_multiplier(cpu: &mut Cpu) {
    cpu.pc = 0x6c50;
    cpu.a = cpu.read_byte(0xd206);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0x4378);
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
    let lhs = cpu.hl();
    let rhs = cpu.de();
    let (result, carry) = lhs.overflowing_add(rhs);
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0fff) + (rhs & 0x0fff) > 0x0fff;
    cpu.flag_c = carry;
    cpu.set_hl(result);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 0x11;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::get_far_word(cpu);
    assert_eq!(cpu.pc, next_pc);
    become heavy_ball_multiplier_skip_text(cpu);
}

pub(crate) fn heavy_ball_multiplier_skip_text(cpu: &mut Cpu) {
    cpu.pc = 0x6c61;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    heavy_ball_get_dex_entry_bank(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::get_far_byte(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = { let value = (0x50 as i32); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
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
        become heavy_ball_multiplier_skip_text(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    heavy_ball_get_dex_entry_bank(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.push_word(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::get_far_word(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.h;
    let result = value >> 1;
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x01 != 0;
    cpu.h = result;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.l;
    let carry_in = u8::from(cpu.flag_c) << 7;
    let result = (value >> 1) | carry_in;
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x01 != 0;
    cpu.l = result;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.b = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.c = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.b;
    let result = value >> 1;
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x01 != 0;
    cpu.b = result;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.c;
    let carry_in = u8::from(cpu.flag_c) << 7;
    let result = (value >> 1) | carry_in;
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x01 != 0;
    cpu.c = result;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.b;
    let result = value >> 1;
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x01 != 0;
    cpu.b = result;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.c;
    let carry_in = u8::from(cpu.flag_c) << 7;
    let result = (value >> 1) | carry_in;
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x01 != 0;
    cpu.c = result;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.b;
    let result = value >> 1;
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x01 != 0;
    cpu.b = result;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.c;
    let carry_in = u8::from(cpu.flag_c) << 7;
    let result = (value >> 1) | carry_in;
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x01 != 0;
    cpu.c = result;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.b;
    let result = value >> 1;
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x01 != 0;
    cpu.b = result;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.c;
    let carry_in = u8::from(cpu.flag_c) << 7;
    let result = (value >> 1) | carry_in;
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x01 != 0;
    cpu.c = result;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    heavy_ball_multiplier_subbc(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.b;
    let result = value >> 1;
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x01 != 0;
    cpu.b = result;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.c;
    let carry_in = u8::from(cpu.flag_c) << 7;
    let result = (value >> 1) | carry_in;
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x01 != 0;
    cpu.c = result;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    heavy_ball_multiplier_subbc(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.pop_word();
    cpu.set_bc(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.cycle(12);
    become heavy_ball_multiplier_compare(cpu);
}

pub(crate) fn heavy_ball_multiplier_subbc(cpu: &mut Cpu) {
    cpu.pc = 0x6c99;
    cpu.push_word(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = !cpu.a;
    cpu.flag_n = true;
    cpu.flag_h = true;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = !cpu.a;
    cpu.flag_n = true;
    cpu.flag_h = true;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_bc(cpu.bc().wrapping_add(1));
    cpu.pc += 1;
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
    let value = cpu.pop_word();
    cpu.set_bc(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn heavy_ball_multiplier_compare(cpu: &mut Cpu) {
    cpu.pc = 0x6ca4;
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = (((1024 as u32) >> 8) & 0x00ff) as u8;
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
        become heavy_ball_multiplier_lightmon(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x6cc4);
    cpu.pc += 3;
    cpu.cycle(12);
    become heavy_ball_multiplier_lookup(cpu);
}

pub(crate) fn heavy_ball_multiplier_lookup(cpu: &mut Cpu) {
    cpu.pc = 0x6cac;
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
    if cpu.flag_c {
        cpu.cycle(12);
        become heavy_ball_multiplier_heavymon(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.cycle(12);
    become heavy_ball_multiplier_lookup(cpu);
}

pub(crate) fn heavy_ball_multiplier_heavymon(cpu: &mut Cpu) {
    cpu.pc = 0x6cb4;
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.b;
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
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_c {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.b = 0xff;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn heavy_ball_multiplier_lightmon(cpu: &mut Cpu) {
    cpu.pc = 0x6cbc;
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = 20;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_c {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.b = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn lure_ball_multiplier(cpu: &mut Cpu) {
    cpu.pc = 0x6ccc;
    cpu.a = cpu.read_byte(0xd230);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = crate::defs::BATTLETYPE_FISH as u8;
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
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.a;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_add(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) > 0x0f;
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_c {
        cpu.cycle(12);
        become lure_ball_multiplier_max(cpu);
    }
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
    if !cpu.flag_c {
        cpu.cycle(12);
        become lure_ball_multiplier_done(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become lure_ball_multiplier_max(cpu);
}

pub(crate) fn lure_ball_multiplier_max(cpu: &mut Cpu) {
    cpu.pc = 0x6cd9;
    cpu.a = 0xff;
    cpu.pc += 2;
    cpu.cycle(8);
    become lure_ball_multiplier_done(cpu);
}

pub(crate) fn lure_ball_multiplier_done(cpu: &mut Cpu) {
    cpu.pc = 0x6cdb;
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn moon_ball_multiplier(cpu: &mut Cpu) {
    cpu.pc = 0x6cdd;
    cpu.push_word(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xd204);
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
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.b = 0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x65b1);
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
    let lhs = cpu.hl();
    let rhs = cpu.bc();
    let (result, carry) = lhs.overflowing_add(rhs);
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0fff) + (rhs & 0x0fff) > 0x0fff;
    cpu.flag_c = carry;
    cpu.set_hl(result);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 0x10;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::get_far_word(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_bc(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.push_word(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = 0x10;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::get_far_byte(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = crate::defs::EVOLVE_ITEM as u8;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.pop_word();
    cpu.set_bc(value);
    cpu.pc += 1;
    cpu.cycle(12);
    if !cpu.flag_z {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
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
    cpu.push_word(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = 0x10;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::get_far_byte(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = crate::defs::MOON_STONE_RED as u8;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.pop_word();
    cpu.set_bc(value);
    cpu.pc += 1;
    cpu.cycle(12);
    if !cpu.flag_z {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
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
    if cpu.flag_c {
        cpu.cycle(12);
        become moon_ball_multiplier_max(cpu);
    }
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
    if !cpu.flag_c {
        cpu.cycle(12);
        become moon_ball_multiplier_done(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become moon_ball_multiplier_max(cpu);
}

pub(crate) fn moon_ball_multiplier_max(cpu: &mut Cpu) {
    cpu.pc = 0x6d0f;
    cpu.b = 0xff;
    cpu.pc += 2;
    cpu.cycle(8);
    become moon_ball_multiplier_done(cpu);
}

pub(crate) fn moon_ball_multiplier_done(cpu: &mut Cpu) {
    cpu.pc = 0x6d11;
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn love_ball_multiplier(cpu: &mut Cpu) {
    cpu.pc = 0x6d12;
    cpu.a = cpu.read_byte(0xd204);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xd205);
    cpu.pc += 3;
    cpu.cycle(16);
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
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.push_word(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xd205);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xd108, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcf5f, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xd0d4);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xd109, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x14;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x4bdd);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.cycle(12);
        become love_ball_multiplier_done1(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.d = 0;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(12);
        become love_ball_multiplier_got_player_gender(cpu);
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
    become love_ball_multiplier_got_player_gender(cpu);
}

pub(crate) fn love_ball_multiplier_got_player_gender(cpu: &mut Cpu) {
    cpu.pc = 0x6d39;
    cpu.push_word(cpu.de());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xd204);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xd108, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = crate::defs::WILDMON as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xcf5f, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x14;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x4bdd);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.cycle(12);
        become love_ball_multiplier_done2(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.d = 0;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(12);
        become love_ball_multiplier_got_wild_gender(cpu);
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
    become love_ball_multiplier_got_wild_gender(cpu);
}

pub(crate) fn love_ball_multiplier_got_wild_gender(cpu: &mut Cpu) {
    cpu.pc = 0x6d52;
    cpu.a = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.pop_word();
    cpu.set_de(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let value = cpu.d;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.pop_word();
    cpu.set_bc(value);
    cpu.pc += 1;
    cpu.cycle(12);
    if !cpu.flag_z {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
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
    if cpu.flag_c {
        cpu.cycle(12);
        become love_ball_multiplier_max(cpu);
    }
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
    if cpu.flag_c {
        cpu.cycle(12);
        become love_ball_multiplier_max(cpu);
    }
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
    if !cpu.flag_c {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    become love_ball_multiplier_max(cpu);
}

pub(crate) fn love_ball_multiplier_max(cpu: &mut Cpu) {
    cpu.pc = 0x6d62;
    cpu.b = 0xff;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn love_ball_multiplier_done2(cpu: &mut Cpu) {
    cpu.pc = 0x6d65;
    let value = cpu.pop_word();
    cpu.set_de(value);
    cpu.pc += 1;
    cpu.cycle(12);
    become love_ball_multiplier_done1(cpu);
}

pub(crate) fn love_ball_multiplier_done1(cpu: &mut Cpu) {
    cpu.pc = 0x6d66;
    let value = cpu.pop_word();
    cpu.set_bc(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn fast_ball_multiplier(cpu: &mut Cpu) {
    cpu.pc = 0x6d68;
    cpu.a = cpu.read_byte(0xd204);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl(0x459a);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.d = 3;
    cpu.pc += 2;
    cpu.cycle(8);
    become fast_ball_multiplier_loop(cpu);
}

pub(crate) fn fast_ball_multiplier_loop(cpu: &mut Cpu) {
    cpu.pc = 0x6d71;
    cpu.a = 0x0f;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::get_far_byte(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(cpu.hl().wrapping_add(1));
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
        become fast_ball_multiplier_next(cpu);
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
    if !cpu.flag_z {
        cpu.cycle(12);
        become fast_ball_multiplier_next(cpu);
    }
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
    if cpu.flag_c {
        cpu.cycle(12);
        become fast_ball_multiplier_max(cpu);
    }
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
    if !cpu.flag_c {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    become fast_ball_multiplier_max(cpu);
}

pub(crate) fn fast_ball_multiplier_max(cpu: &mut Cpu) {
    cpu.pc = 0x6d85;
    cpu.b = 0xff;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn fast_ball_multiplier_next(cpu: &mut Cpu) {
    cpu.pc = 0x6d88;
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
        become fast_ball_multiplier_loop(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn level_ball_multiplier(cpu: &mut Cpu) {
    cpu.pc = 0x6d8c;
    cpu.a = cpu.read_byte(0xc639);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xd213);
    cpu.pc += 3;
    cpu.cycle(16);
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
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
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
    if cpu.flag_c {
        cpu.cycle(12);
        become level_ball_multiplier_max(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.c;
    let result = value >> 1;
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x01 != 0;
    cpu.c = result;
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
    if !cpu.flag_c {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
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
    if cpu.flag_c {
        cpu.cycle(12);
        become level_ball_multiplier_max(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.c;
    let result = value >> 1;
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x01 != 0;
    cpu.c = result;
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
    if !cpu.flag_c {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
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
    if !cpu.flag_c {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    become level_ball_multiplier_max(cpu);
}

pub(crate) fn level_ball_multiplier_max(cpu: &mut Cpu) {
    cpu.pc = 0x6da8;
    cpu.b = 0xff;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn text_gotcha_mon_was_caught_text_asm_0(cpu: &mut Cpu) {
    cpu.pc = 0x6dce;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::audio::wait_sfx(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.push_word(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.set_de(crate::defs::MUSIC_NONE as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::audio::play_music(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::delay::delay_frame(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_de(crate::defs::MUSIC_CAPTURE as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::audio::play_music(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_bc(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.set_hl(0x6de6);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn return_to_battle_use_ball(cpu: &mut Cpu) {
    cpu.pc = 0x6dfa;
    // HARNESS-PATCH: no-op the post-pack-menu battle-screen redraw when running
    // headless. `poke_ball_effect` calls this (asm `.room_in_party`,
    // engine/items/item_effects.asm:235) to repaint the battle scene after the
    // bag UI closes — its body (`_ReturnToBattle_UseBall`,
    // engine/battle/returntobattle_useball.asm) is purely cosmetic
    // (ClearBGPalettes / ClearTilemap / GetBattleMonBackpic / GetEnemyMonFrontpic /
    // CloseWindow / LoadStandardMenuHeader / WaitBGMap / SetDefaultBGPAndOBP).
    // The CLI driver's `battle_menu` patch never opens the engine's pack-menu
    // window, so the `CloseWindow` inside spins forever in
    // `error_cant_exit_menu_infinite_loop` (the window stack is empty). This is
    // the same "skip UI cleanup that assumes an overlay we never opened" pattern
    // as the `offer_switch` / `try_player_switch` patches in
    // engine/battle/core.rs. Return immediately (the caller pushed our return
    // address). See docs/transpiled_code.md and bootstrap::cli.
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
    return;
    cpu.a = 0x09;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x715c);
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

pub(crate) fn town_map_effect(cpu: &mut Cpu) {
    cpu.pc = 0x6e01;
    cpu.a = 0x24;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x5ae1);
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

pub(crate) fn bicycle_effect(cpu: &mut Cpu) {
    cpu.pc = 0x6e08;
    cpu.a = 0x03;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x50b3);
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

pub(crate) fn evo_stone_effect(cpu: &mut Cpu) {
    cpu.pc = 0x6e0f;
    cpu.b = crate::defs::PARTYMENUACTION_EVO_STONE as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    use_item_select_mon(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.cycle(16);
        become evo_stone_effect_decided_not_to_use(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = crate::defs::MON_ITEM as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::battle::get_party_param_location(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = crate::defs::EVERSTONE as u8;
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
        become evo_stone_effect_no_effect(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = crate::defs::TRUE as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xd1e9, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x10;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x61d8);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xd268);
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
        become evo_stone_effect_no_effect(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become use_disposable_item(cpu);
}

pub(crate) fn evo_stone_effect_no_effect(cpu: &mut Cpu) {
    cpu.pc = 0x6e35;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    wont_have_any_effect_message(cpu);
    assert_eq!(cpu.pc, next_pc);
    become evo_stone_effect_decided_not_to_use(cpu);
}

pub(crate) fn evo_stone_effect_decided_not_to_use(cpu: &mut Cpu) {
    cpu.pc = 0x6e38;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xd0ec, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn vitamin_effect(cpu: &mut Cpu) {
    cpu.pc = 0x6e3d;
    cpu.b = crate::defs::PARTYMENUACTION_HEALING_ITEM as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    use_item_select_mon(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.cycle(16);
        become rare_candy_stat_booster_exit_menu(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    rare_candy_stat_booster_get_parameters(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    get_stat_exp_relative_pointer(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = crate::defs::MON_STAT_EXP as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::battle::get_party_param_location(cpu);
    assert_eq!(cpu.pc, next_pc);
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
    let value = 100;
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
        become no_effect_message(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = 10;
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
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    update_stats_after_item(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    get_stat_exp_relative_pointer(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0x6eab);
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
    cpu.set_de(0xd086);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(crate::defs::ITEM_NAME_LENGTH as u16);
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
    play_sfx_full_heal(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0x6ea6);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::print_text(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.c = crate::defs::HAPPINESS_USEDITEM as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x01;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x71c2);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(16);
    become use_disposable_item(cpu);
}

pub(crate) fn no_effect_message(cpu: &mut Cpu) {
    cpu.pc = 0x6e83;
    cpu.set_hl(0x781f);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::print_text(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(16);
    become crate::game::home::tilemap::clear_palettes(cpu);
}

pub(crate) fn update_stats_after_item(cpu: &mut Cpu) {
    cpu.pc = 0x6e8c;
    cpu.a = crate::defs::MON_MAXHP as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::battle::get_party_param_location(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.d = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.e = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = { let value = ((crate::defs::MON_STAT_EXP as i32) - (1 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::battle::get_party_param_location(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.b = crate::defs::TRUE as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = { let value = (((0x458f as i32) - (0x456b as i32)) / (3 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become crate::game::home::predef::predef(cpu);
}

pub(crate) fn rare_candy_stat_booster_exit_menu(cpu: &mut Cpu) {
    cpu.pc = 0x6e9f;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xd0ec, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(16);
    become crate::game::home::tilemap::clear_palettes(cpu);
}

pub(crate) fn get_stat_exp_relative_pointer(cpu: &mut Cpu) {
    cpu.pc = 0x6ed9;
    cpu.a = cpu.read_byte(0xd106);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0x6eeb);
    cpu.pc += 3;
    cpu.cycle(12);
    become get_stat_exp_relative_pointer_next(cpu);
}

pub(crate) fn get_stat_exp_relative_pointer_next(cpu: &mut Cpu) {
    cpu.pc = 0x6edf;
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
    if cpu.flag_z {
        cpu.cycle(12);
        become get_stat_exp_relative_pointer_got_it(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.cycle(12);
    become get_stat_exp_relative_pointer_next(cpu);
}

pub(crate) fn get_stat_exp_relative_pointer_got_it(cpu: &mut Cpu) {
    cpu.pc = 0x6ee6;
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.b = 0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn rare_candy_stat_booster_get_parameters(cpu: &mut Cpu) {
    cpu.pc = 0x6ef5;
    cpu.a = cpu.read_byte(0xd108);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xcf60, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xd265, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = crate::defs::MON_LEVEL as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::battle::get_party_param_location(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(0xd143, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::pokemon::get_base_data(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xd109);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xde41);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::pokemon::get_nickname(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn rare_candy_effect(cpu: &mut Cpu) {
    cpu.pc = 0x6f14;
    cpu.b = crate::defs::PARTYMENUACTION_HEALING_ITEM as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    use_item_select_mon(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.cycle(16);
        become rare_candy_stat_booster_exit_menu(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    rare_candy_stat_booster_get_parameters(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = crate::defs::MON_LEVEL as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::battle::get_party_param_location(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = crate::defs::MAX_LEVEL as u8;
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
        become no_effect_message(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
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
    cpu.write_byte(0xd143, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.push_word(cpu.de());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.d = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = 0x14;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x4e47);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_de(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.a = crate::defs::MON_EXP as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::battle::get_party_param_location(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte({ const N: i32 = (0xffb4 as i32) + (0 as i32); N as u16 });
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte({ const N: i32 = (0xffb4 as i32) + (1 as i32); N as u16 });
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte({ const N: i32 = (0xffb4 as i32) + (2 as i32); N as u16 });
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = crate::defs::MON_MAXHP as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::battle::get_party_param_location(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.c = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.push_word(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    update_stats_after_item(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = { let value = ((crate::defs::MON_MAXHP as i32) + (1 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::battle::get_party_param_location(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_bc(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_sub(1));
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
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.b;
    let carry_in = u8::from(cpu.flag_c);
    let lhs = cpu.a;
    let (partial, borrow1) = lhs.overflowing_sub(value);
    let (result, borrow2) = partial.overflowing_sub(carry_in);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < ((value & 0x0f) + carry_in);
    cpu.flag_c = borrow1 || borrow2;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl(cpu.hl().wrapping_sub(1));
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
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.b;
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
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 0x09;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x709e);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = crate::defs::PARTYMENUTEXT_LEVEL_UP as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    item_action_text(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcf5f, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = { let value = (((0x45c8 as i32) - (0x456b as i32)) / (3 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::predef::predef(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = ((((0 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (9 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = 10;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.c = 9;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::textbox(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = ((((1 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (11 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(4 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = { let value = (((0x45e3 as i32) - (0x456b as i32)) / (3 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::predef::predef(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::joypad::wait_press_aor_b_blink_cursor(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcf5f, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xd108);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xd265, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = { let value = (((0x45b9 as i32) - (0x456b as i32)) / (3 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::predef::predef(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xd1e9, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x10;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x61d8);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(16);
    become use_disposable_item(cpu);
}

pub(crate) fn heal_powder_effect(cpu: &mut Cpu) {
    cpu.pc = 0x6fad;
    cpu.b = crate::defs::PARTYMENUACTION_HEALING_ITEM as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    use_item_select_mon(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.cycle(16);
        become status_healer_exit_menu(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    use_status_healer(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = crate::defs::FALSE as u8;
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
        become heal_powder_effect_not_used(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.c = crate::defs::HAPPINESS_BITTERPOWDER as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x01;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x71c2);
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
    looks_bitter_message(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x0;
    cpu.pc += 2;
    cpu.cycle(8);
    become heal_powder_effect_not_used(cpu);
}

pub(crate) fn heal_powder_effect_not_used(cpu: &mut Cpu) {
    cpu.pc = 0x6fc9;
    cpu.cycle(16);
    become status_healer_jumptable(cpu);
}

pub(crate) fn status_healing_effect(cpu: &mut Cpu) {
    cpu.pc = 0x6fcc;
    cpu.b = crate::defs::PARTYMENUACTION_HEALING_ITEM as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    use_item_select_mon(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.cycle(16);
        become status_healer_exit_menu(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    become fully_heal_status(cpu);
}

pub(crate) fn fully_heal_status(cpu: &mut Cpu) {
    cpu.pc = 0x6fd4;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    use_status_healer(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(16);
    become status_healer_jumptable(cpu);
}

pub(crate) fn use_status_healer(cpu: &mut Cpu) {
    cpu.pc = 0x6fda;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    is_mon_fainted(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = crate::defs::TRUE as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    get_item_healing_action(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = crate::defs::MON_STATUS as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::battle::get_party_param_location(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.c;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(12);
        become use_status_healer_good(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    is_item_used_on_confused_mon(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = crate::defs::TRUE as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_c {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.b = crate::defs::PARTYMENUTEXT_HEAL_CONFUSION as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    become use_status_healer_good(cpu);
}

pub(crate) fn use_status_healer_good(cpu: &mut Cpu) {
    cpu.pc = 0x6ff4;
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
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xd141, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    heal_status(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    play_sfx_full_heal(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    item_action_text_wait_button(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    use_disposable_item(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = crate::defs::FALSE as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn is_item_used_on_confused_mon(cpu: &mut Cpu) {
    cpu.pc = 0x7009;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    is_item_used_on_battle_mon(cpu);
    assert_eq!(cpu.pc, next_pc);
    if !cpu.flag_c {
        cpu.cycle(12);
        become is_item_used_on_confused_mon_nope(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc66a);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((crate::defs::SUBSTATUS_CONFUSED as u8) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become is_item_used_on_confused_mon_nope(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.c;
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
    if !cpu.flag_z {
        cpu.cycle(12);
        become is_item_used_on_confused_mon_nope(cpu);
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

pub(crate) fn is_item_used_on_confused_mon_nope(cpu: &mut Cpu) {
    cpu.pc = 0x701c;
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

pub(crate) fn battlemon_restore_health(cpu: &mut Cpu) {
    cpu.pc = 0x701e;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    is_item_used_on_battle_mon(cpu);
    assert_eq!(cpu.pc, next_pc);
    if !cpu.flag_c {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = crate::defs::MON_HP as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::battle::get_party_param_location(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(0xc63c, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte({ const N: i32 = (0xc63c as i32) + (1 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn heal_status(cpu: &mut Cpu) {
    cpu.pc = 0x7030;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    is_item_used_on_battle_mon(cpu);
    assert_eq!(cpu.pc, next_pc);
    if !cpu.flag_c {
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
    cpu.write_byte(0xc63a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xc66c);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((crate::defs::SUBSTATUS_TOXIC as u8) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.set_hl(0xc668);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((crate::defs::SUBSTATUS_NIGHTMARE as u8) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    get_item_healing_action(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = 0b11111111;
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
        become heal_status_not_full_heal(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xc66a);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((crate::defs::SUBSTATUS_CONFUSED as u8) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    become heal_status_not_full_heal(cpu);
}

pub(crate) fn heal_status_not_full_heal(cpu: &mut Cpu) {
    cpu.pc = 0x704f;
    cpu.push_word(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = 0x0d;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x65d7);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_bc(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn get_item_healing_action(cpu: &mut Cpu) {
    cpu.pc = 0x7058;
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xd106);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0x7071);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(3 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    become get_item_healing_action_next(cpu);
}

pub(crate) fn get_item_healing_action_next(cpu: &mut Cpu) {
    cpu.pc = 0x7062;
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
        become get_item_healing_action_found_it(cpu);
    }
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
    cpu.cycle(12);
    become get_item_healing_action_next(cpu);
}

pub(crate) fn get_item_healing_action_found_it(cpu: &mut Cpu) {
    cpu.pc = 0x7068;
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.b = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = 0b11111111;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn status_healer_jumptable(cpu: &mut Cpu) {
    cpu.pc = 0x709e;
    cpu.set_hl(0x70a3);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::jump_table(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn revival_herb_effect(cpu: &mut Cpu) {
    cpu.pc = 0x70a9;
    cpu.b = crate::defs::PARTYMENUACTION_HEALING_ITEM as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    use_item_select_mon(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.cycle(16);
        become status_healer_exit_menu(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    revive_pokemon(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = crate::defs::FALSE as u8;
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
        become revival_herb_effect_not_used(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.c = crate::defs::HAPPINESS_REVIVALHERB as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x01;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x71c2);
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
    looks_bitter_message(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x0;
    cpu.pc += 2;
    cpu.cycle(8);
    become revival_herb_effect_not_used(cpu);
}

pub(crate) fn revival_herb_effect_not_used(cpu: &mut Cpu) {
    cpu.pc = 0x70c5;
    cpu.cycle(16);
    become status_healer_jumptable(cpu);
}

pub(crate) fn revive_effect(cpu: &mut Cpu) {
    cpu.pc = 0x70c8;
    cpu.b = crate::defs::PARTYMENUACTION_HEALING_ITEM as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    use_item_select_mon(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.cycle(16);
        become status_healer_exit_menu(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    revive_pokemon(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(16);
    become status_healer_jumptable(cpu);
}

pub(crate) fn revive_pokemon(cpu: &mut Cpu) {
    cpu.pc = 0x70d6;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    is_mon_fainted(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = crate::defs::TRUE as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xd22d);
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
        become revive_pokemon_skip_to_revive(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xd109);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.d = 0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xc6fc);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = crate::defs::CHECK_FLAG as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = { let value = (((0x4574 as i32) - (0x456b as i32)) / (3 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::predef::predef(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.c;
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
        become revive_pokemon_skip_to_revive(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xd109);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl(0xc664);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = crate::defs::SET_FLAG as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = { let value = (((0x4574 as i32) - (0x456b as i32)) / (3 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::predef::predef(cpu);
    assert_eq!(cpu.pc, next_pc);
    become revive_pokemon_skip_to_revive(cpu);
}

pub(crate) fn revive_pokemon_skip_to_revive(cpu: &mut Cpu) {
    cpu.pc = 0x7104;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc2a6, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xd106);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = crate::defs::REVIVE as u8;
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
        become revive_pokemon_revive_half_hp(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    revive_full_hp(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(12);
    become revive_pokemon_finish_revive(cpu);
}

pub(crate) fn revive_pokemon_revive_half_hp(cpu: &mut Cpu) {
    cpu.pc = 0x7114;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    revive_half_hp(cpu);
    assert_eq!(cpu.pc, next_pc);
    become revive_pokemon_finish_revive(cpu);
}

pub(crate) fn revive_pokemon_finish_revive(cpu: &mut Cpu) {
    cpu.pc = 0x7117;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    heal_hp_sfx_gfx(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = crate::defs::PARTYMENUTEXT_REVIVE as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xd141, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    item_action_text_wait_button(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    use_disposable_item(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = crate::defs::FALSE as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn full_restore_effect(cpu: &mut Cpu) {
    cpu.pc = 0x7128;
    cpu.b = crate::defs::PARTYMENUACTION_HEALING_ITEM as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    use_item_select_mon(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.cycle(16);
        become status_healer_exit_menu(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    is_mon_fainted(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_z {
        cpu.cycle(16);
        become status_healer_no_effect(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    is_mon_at_full_health(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.cycle(12);
        become full_restore_effect_not_at_full_health(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become fully_heal_status(cpu);
}

pub(crate) fn full_restore_effect_not_at_full_health(cpu: &mut Cpu) {
    cpu.pc = 0x713e;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    full_restore_effect_full_restore(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(16);
    become status_healer_jumptable(cpu);
}

pub(crate) fn full_restore_effect_full_restore(cpu: &mut Cpu) {
    cpu.pc = 0x7144;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc2a6, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    revive_full_hp(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = crate::defs::MON_STATUS as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::battle::get_party_param_location(cpu);
    assert_eq!(cpu.pc, next_pc);
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
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    heal_status(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    battlemon_restore_health(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    heal_hp_sfx_gfx(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = crate::defs::PARTYMENUTEXT_HEAL_HP as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xd141, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    item_action_text_wait_button(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    use_disposable_item(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn bitter_berry_effect(cpu: &mut Cpu) {
    cpu.pc = 0x716a;
    cpu.set_hl(0xc66a);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((crate::defs::SUBSTATUS_CONFUSED as u8) & 7);
    let value = cpu.read_byte(cpu.hl());
    cpu.flag_z = (value & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = 1;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become bitter_berry_effect_done(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let mask = 1u8 << ((crate::defs::SUBSTATUS_CONFUSED as u8) & 7);
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
    cpu.write_byte(0xffe4, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    use_item_text(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0x4d81);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::battle::std_battle_textbox(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0;
    cpu.pc += 2;
    cpu.cycle(8);
    become bitter_berry_effect_done(cpu);
}

pub(crate) fn bitter_berry_effect_done(cpu: &mut Cpu) {
    cpu.pc = 0x7183;
    cpu.cycle(16);
    become status_healer_jumptable(cpu);
}

pub(crate) fn restore_hp_effect(cpu: &mut Cpu) {
    cpu.pc = 0x7186;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    item_restore_hp(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(16);
    become status_healer_jumptable(cpu);
}

pub(crate) fn energypowder_effect(cpu: &mut Cpu) {
    cpu.pc = 0x718c;
    cpu.c = crate::defs::HAPPINESS_BITTERPOWDER as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become energypowder_energy_root_common(cpu);
}

pub(crate) fn energy_root_effect(cpu: &mut Cpu) {
    cpu.pc = 0x7190;
    cpu.c = crate::defs::HAPPINESS_ENERGYROOT as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    become energypowder_energy_root_common(cpu);
}

pub(crate) fn energypowder_energy_root_common(cpu: &mut Cpu) {
    cpu.pc = 0x7192;
    cpu.push_word(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    item_restore_hp(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_bc(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let value = 0;
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
        become energypowder_energy_root_common_skip_happiness(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x01;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x71c2);
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
    looks_bitter_message(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0;
    cpu.pc += 2;
    cpu.cycle(8);
    become energypowder_energy_root_common_skip_happiness(cpu);
}

pub(crate) fn energypowder_energy_root_common_skip_happiness(cpu: &mut Cpu) {
    cpu.pc = 0x71a6;
    cpu.cycle(16);
    become status_healer_jumptable(cpu);
}

pub(crate) fn item_restore_hp(cpu: &mut Cpu) {
    cpu.pc = 0x71a9;
    cpu.b = crate::defs::PARTYMENUACTION_HEALING_ITEM as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    use_item_select_mon(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 2;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_c {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    is_mon_fainted(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 1;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    is_mon_at_full_health(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 1;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_c {
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
    cpu.write_byte(0xc2a6, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    get_healing_item_amount(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    restore_health(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    battlemon_restore_health(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    heal_hp_sfx_gfx(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = crate::defs::PARTYMENUTEXT_HEAL_HP as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xd141, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    item_action_text_wait_button(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    use_disposable_item(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn heal_hp_sfx_gfx(cpu: &mut Cpu) {
    cpu.pc = 0x71db;
    cpu.push_word(cpu.de());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.set_de(crate::defs::SFX_POTION as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::audio::wait_play_sfx(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_de(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xd109);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl({ const N: i32 = ((((0 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (11 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(0x0028);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::array::add_n_times(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x2;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xd10a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = { let value = (((0x458c as i32) - (0x456b as i32)) / (3 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become crate::game::home::predef::predef(cpu);
}

pub(crate) fn use_item_select_mon(cpu: &mut Cpu) {
    cpu.pc = 0x71f9;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    use_item_select_mon_select_mon(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xd108);
    cpu.pc += 3;
    cpu.cycle(16);
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
        become use_item_select_mon_not_egg(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    cant_use_on_egg_message(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = true;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn use_item_select_mon_not_egg(cpu: &mut Cpu) {
    cpu.pc = 0x7209;
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

pub(crate) fn use_item_select_mon_select_mon(cpu: &mut Cpu) {
    cpu.pc = 0x720b;
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xd141, cpu.a);
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
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::tilemap::clear_bg_palettes(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    choose_mon_to_use_item_on(cpu);
    assert_eq!(cpu.pc, next_pc);
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
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn choose_mon_to_use_item_on(cpu: &mut Cpu) {
    cpu.pc = 0x721c;
    cpu.a = 0x14;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x404f);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x14;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x4405);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x14;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x43e0);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x14;
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
    cpu.a = 0x14;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x449a);
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
    crate::game::home::tilemap::wait_bg_map(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::tilemap::set_default_bgp_and_obp(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::delay::delay_frame(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x14;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x4457);
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

pub(crate) fn item_action_text(cpu: &mut Cpu) {
    cpu.pc = 0x724a;
    cpu.write_byte(0xd141, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xd108);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xd109);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
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
    cpu.a = 0x14;
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
    cpu.a = 0x14;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x4566);
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
    crate::game::home::tilemap::wait_bg_map(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::tilemap::set_default_bgp_and_obp(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::delay::delay_frame(cpu);
    assert_eq!(cpu.pc, next_pc);
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
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte(0xd109, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte(0xd108, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn item_action_text_wait_button(cpu: &mut Cpu) {
    cpu.pc = 0x7279;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xffd4, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.set_hl({ const N: i32 = ((((0 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (0 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc({ const N: i32 = (0xc608 as i32) - (0xc4a0 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = { let value = (0x7f as i32); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::byte_fill(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xd141);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    item_action_text(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xffd4, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.c = 50;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::delay::delay_frames(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(16);
    become crate::game::home::joypad::wait_press_aor_b_blink_cursor(cpu);
}

pub(crate) fn status_healer_no_effect(cpu: &mut Cpu) {
    cpu.pc = 0x7299;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    wont_have_any_effect_message(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(12);
    become status_healer_clear_palettes(cpu);
}

pub(crate) fn status_healer_exit_menu(cpu: &mut Cpu) {
    cpu.pc = 0x729e;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xd0ec, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    become status_healer_clear_palettes(cpu);
}

pub(crate) fn status_healer_clear_palettes(cpu: &mut Cpu) {
    cpu.pc = 0x72a2;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::tilemap::clear_palettes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn is_item_used_on_battle_mon(cpu: &mut Cpu) {
    cpu.pc = 0x72a6;
    cpu.a = cpu.read_byte(0xd22d);
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
    cpu.a = cpu.read_byte(0xd109);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.set_hl(0xd0d4);
    cpu.pc += 3;
    cpu.cycle(12);
    let value = cpu.read_byte(cpu.hl());
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    if !cpu.flag_z {
        cpu.cycle(12);
        become is_item_used_on_battle_mon_nope(cpu);
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

pub(crate) fn is_item_used_on_battle_mon_nope(cpu: &mut Cpu) {
    cpu.pc = 0x72b8;
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

pub(crate) fn revive_half_hp(cpu: &mut Cpu) {
    cpu.pc = 0x72ba;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    load_hp_from_buffer1(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.d;
    let result = value >> 1;
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x01 != 0;
    cpu.d = result;
    cpu.pc += 2;
    cpu.cycle(8);
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
    cpu.cycle(12);
    become continue_revive(cpu);
}

pub(crate) fn revive_full_hp(cpu: &mut Cpu) {
    cpu.pc = 0x72c3;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    load_hp_from_buffer1(cpu);
    assert_eq!(cpu.pc, next_pc);
    become continue_revive(cpu);
}

pub(crate) fn continue_revive(cpu: &mut Cpu) {
    cpu.pc = 0x72c6;
    cpu.a = crate::defs::MON_HP as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::battle::get_party_param_location(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.write_byte(cpu.hl(), cpu.d);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.e);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.cycle(16);
    become load_cur_hp_into_buffer3(cpu);
}

pub(crate) fn restore_health(cpu: &mut Cpu) {
    cpu.pc = 0x72d1;
    cpu.a = { let value = ((crate::defs::MON_HP as i32) + (1 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::battle::get_party_param_location(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(cpu.hl());
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
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
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
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    if cpu.flag_c {
        cpu.cycle(12);
        become restore_health_full_hp(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    load_cur_hp_into_buffer3(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = { let value = ((crate::defs::MON_HP as i32) + (1 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::battle::get_party_param_location(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.d = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.e = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = { let value = ((crate::defs::MON_MAXHP as i32) + (1 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::battle::get_party_param_location(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(cpu.de());
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
    cpu.set_de(cpu.de().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.read_byte(cpu.hl());
    let carry_in = u8::from(cpu.flag_c);
    let lhs = cpu.a;
    let (partial, borrow1) = lhs.overflowing_sub(value);
    let (result, borrow2) = partial.overflowing_sub(carry_in);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < ((value & 0x0f) + carry_in);
    cpu.flag_c = borrow1 || borrow2;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(8);
    if cpu.flag_c {
        cpu.cycle(12);
        become restore_health_finish(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become restore_health_full_hp(cpu);
}

pub(crate) fn restore_health_full_hp(cpu: &mut Cpu) {
    cpu.pc = 0x72f5;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    revive_full_hp(cpu);
    assert_eq!(cpu.pc, next_pc);
    become restore_health_finish(cpu);
}

pub(crate) fn restore_health_finish(cpu: &mut Cpu) {
    cpu.pc = 0x72f8;
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn remove_hp(cpu: &mut Cpu) {
    cpu.pc = 0x72f9;
    cpu.a = { let value = ((crate::defs::MON_HP as i32) + (1 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::battle::get_party_param_location(cpu);
    assert_eq!(cpu.pc, next_pc);
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
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.d;
    let carry_in = u8::from(cpu.flag_c);
    let lhs = cpu.a;
    let (partial, borrow1) = lhs.overflowing_sub(value);
    let (result, borrow2) = partial.overflowing_sub(carry_in);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < ((value & 0x0f) + carry_in);
    cpu.flag_c = borrow1 || borrow2;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    if !cpu.flag_c {
        cpu.cycle(12);
        become remove_hp_okay(cpu);
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
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    become remove_hp_okay(cpu);
}

pub(crate) fn remove_hp_okay(cpu: &mut Cpu) {
    cpu.pc = 0x7309;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    load_cur_hp_into_buffer3(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn is_mon_fainted(cpu: &mut Cpu) {
    cpu.pc = 0x730d;
    cpu.push_word(cpu.de());
    cpu.pc += 1;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    load_max_hp_into_buffer1(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    load_cur_hp_into_buffer2(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    load_hp_from_buffer2(cpu);
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
    let value = cpu.pop_word();
    cpu.set_de(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn is_mon_at_full_health(cpu: &mut Cpu) {
    cpu.pc = 0x731b;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    load_hp_from_buffer2(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.h = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.l = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    load_hp_from_buffer1(cpu);
    assert_eq!(cpu.pc, next_pc);
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
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.d;
    let carry_in = u8::from(cpu.flag_c);
    let lhs = cpu.a;
    let (partial, borrow1) = lhs.overflowing_sub(value);
    let (result, borrow2) = partial.overflowing_sub(carry_in);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < ((value & 0x0f) + carry_in);
    cpu.flag_c = borrow1 || borrow2;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn load_cur_hp_into_buffer3(cpu: &mut Cpu) {
    cpu.pc = 0x7328;
    cpu.a = crate::defs::MON_HP as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::battle::get_party_param_location(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte({ const N: i32 = (0xd1ee as i32) + (1 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(0xd1ee, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn load_hp_into_buffer3(cpu: &mut Cpu) {
    cpu.pc = 0x7336;
    cpu.a = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte({ const N: i32 = (0xd1ee as i32) + (1 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xd1ee, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn load_hp_from_buffer3(cpu: &mut Cpu) {
    cpu.pc = 0x733f;
    cpu.a = cpu.read_byte({ const N: i32 = (0xd1ee as i32) + (1 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.d = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xd1ee);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn load_cur_hp_into_buffer2(cpu: &mut Cpu) {
    cpu.pc = 0x7348;
    cpu.a = crate::defs::MON_HP as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::battle::get_party_param_location(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte({ const N: i32 = (0xd1ec as i32) + (1 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(0xd1ec, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn load_hp_from_buffer2(cpu: &mut Cpu) {
    cpu.pc = 0x7356;
    cpu.a = cpu.read_byte({ const N: i32 = (0xd1ec as i32) + (1 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.d = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xd1ec);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn load_max_hp_into_buffer1(cpu: &mut Cpu) {
    cpu.pc = 0x735f;
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = crate::defs::MON_MAXHP as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::battle::get_party_param_location(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte({ const N: i32 = (0xd1ea as i32) + (1 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(0xd1ea, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn load_hp_from_buffer1(cpu: &mut Cpu) {
    cpu.pc = 0x736f;
    cpu.a = cpu.read_byte({ const N: i32 = (0xd1ea as i32) + (1 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.d = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xd1ea);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn get_one_fifth_max_hp(cpu: &mut Cpu) {
    cpu.pc = 0x7378;
    cpu.push_word(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = crate::defs::MON_MAXHP as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::battle::get_party_param_location(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte({ const N: i32 = (0xffb3 as i32) + (0 as i32); N as u16 }, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte({ const N: i32 = (0xffb3 as i32) + (1 as i32); N as u16 }, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = 5;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xffb7, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.b = 2;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::math::divide(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte({ const N: i32 = (0xffb3 as i32) + (2 as i32); N as u16 });
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.d = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte({ const N: i32 = (0xffb3 as i32) + (3 as i32); N as u16 });
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.pop_word();
    cpu.set_bc(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn get_healing_item_amount(cpu: &mut Cpu) {
    cpu.pc = 0x7395;
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xd106);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0x73af);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.d = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    become get_healing_item_amount_next(cpu);
}

pub(crate) fn get_healing_item_amount_next(cpu: &mut Cpu) {
    cpu.pc = 0x739d;
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
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
        become get_healing_item_amount_not_found(cpu);
    }
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
    if cpu.flag_z {
        cpu.cycle(12);
        become get_healing_item_amount_done(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.cycle(12);
    become get_healing_item_amount_next(cpu);
}

pub(crate) fn get_healing_item_amount_not_found(cpu: &mut Cpu) {
    cpu.pc = 0x73a9;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = true;
    cpu.pc += 1;
    cpu.cycle(4);
    become get_healing_item_amount_done(cpu);
}

pub(crate) fn get_healing_item_amount_done(cpu: &mut Cpu) {
    cpu.pc = 0x73aa;
    cpu.e = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.d = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn softboiled_milk_drink_function(cpu: &mut Cpu) {
    cpu.pc = 0x73df;
    cpu.a = cpu.read_byte(0xd0d8);
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
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    softboiled_milk_drink_function_select_milk_drink_recipient(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.cycle(12);
        become softboiled_milk_drink_function_skip(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xd109, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    is_mon_fainted(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    get_one_fifth_max_hp(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    remove_hp(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.push_word(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    heal_hp_sfx_gfx(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_bc(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    get_one_fifth_max_hp(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xd109, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    is_mon_fainted(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    restore_health(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    heal_hp_sfx_gfx(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = crate::defs::PARTYMENUTEXT_HEAL_HP as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    item_action_text(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::joypad::joy_wait_aor_b(cpu);
    assert_eq!(cpu.pc, next_pc);
    become softboiled_milk_drink_function_skip(cpu);
}

pub(crate) fn softboiled_milk_drink_function_skip(cpu: &mut Cpu) {
    cpu.pc = 0x7413;
    cpu.a = cpu.b;
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
    cpu.write_byte(0xd0d8, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn softboiled_milk_drink_function_select_milk_drink_recipient(cpu: &mut Cpu) {
    cpu.pc = 0x7419;
    become softboiled_milk_drink_function_loop(cpu);
}

pub(crate) fn softboiled_milk_drink_function_loop(cpu: &mut Cpu) {
    cpu.pc = 0x7419;
    cpu.push_word(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = crate::defs::PARTYMENUACTION_HEALING_ITEM as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xd141, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    choose_mon_to_use_item_on(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_bc(value);
    cpu.pc += 1;
    cpu.cycle(12);
    if cpu.flag_c {
        cpu.cycle(12);
        become softboiled_milk_drink_function_set_carry(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xd0d8);
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
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
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
    if cpu.flag_z {
        cpu.cycle(12);
        become softboiled_milk_drink_function_cant_use(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xd109, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    is_mon_fainted(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_z {
        cpu.cycle(12);
        become softboiled_milk_drink_function_cant_use(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    is_mon_at_full_health(cpu);
    assert_eq!(cpu.pc, next_pc);
    if !cpu.flag_c {
        cpu.cycle(12);
        become softboiled_milk_drink_function_cant_use(cpu);
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
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn softboiled_milk_drink_function_set_carry(cpu: &mut Cpu) {
    cpu.pc = 0x743e;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = true;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn softboiled_milk_drink_function_cant_use(cpu: &mut Cpu) {
    cpu.pc = 0x7440;
    cpu.push_word(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.set_hl(0x744a);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::menu::menu_textbox_backup(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_bc(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.cycle(12);
    become softboiled_milk_drink_function_loop(cpu);
}

pub(crate) fn escape_rope_effect(cpu: &mut Cpu) {
    cpu.pc = 0x744f;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xd0ec, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x03;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x4b95);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xd0ec);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 1;
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
        use_disposable_item(cpu);
        assert_eq!(cpu.pc, next_pc);
    } else {
        cpu.pc += 3;
        cpu.cycle(12);
    }
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn super_repel_effect(cpu: &mut Cpu) {
    cpu.pc = 0x7462;
    cpu.b = 200;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become use_repel(cpu);
}

pub(crate) fn max_repel_effect(cpu: &mut Cpu) {
    cpu.pc = 0x7466;
    cpu.b = 250;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become use_repel(cpu);
}

pub(crate) fn repel_effect(cpu: &mut Cpu) {
    cpu.pc = 0x746a;
    cpu.b = 100;
    cpu.pc += 2;
    cpu.cycle(8);
    become use_repel(cpu);
}

pub(crate) fn use_repel(cpu: &mut Cpu) {
    cpu.pc = 0x746c;
    cpu.a = cpu.read_byte(0xdca1);
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
    cpu.set_hl(0x747d);
    cpu.pc += 3;
    cpu.cycle(12);
    if !cpu.flag_z {
        cpu.cycle(16);
        become crate::game::home::text::print_text(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xdca1, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(16);
    become use_item_text(cpu);
}

pub(crate) fn x_accuracy_effect(cpu: &mut Cpu) {
    cpu.pc = 0x7482;
    cpu.set_hl(0xc66b);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((crate::defs::SUBSTATUS_X_ACCURACY as u8) & 7);
    let value = cpu.read_byte(cpu.hl());
    cpu.flag_z = (value & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(12);
    if !cpu.flag_z {
        cpu.cycle(16);
        become wont_have_any_effect_not_used_message(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((crate::defs::SUBSTATUS_X_ACCURACY as u8) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value |= mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.cycle(16);
    become use_item_text(cpu);
}

pub(crate) fn poke_doll_effect(cpu: &mut Cpu) {
    cpu.pc = 0x748f;
    cpu.a = cpu.read_byte(0xd22d);
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
    if !cpu.flag_z {
        cpu.cycle(12);
        become poke_doll_effect_not_wild(cpu);
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
    cpu.write_byte(0xd232, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xd0ee);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = crate::defs::BATTLERESULT_BITMASK as u8;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = crate::defs::DRAW as u8;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xd0ee, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(16);
    become use_item_text(cpu);
}

pub(crate) fn poke_doll_effect_not_wild(cpu: &mut Cpu) {
    cpu.pc = 0x74a6;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xd0ec, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn guard_spec_effect(cpu: &mut Cpu) {
    cpu.pc = 0x74ab;
    cpu.set_hl(0xc66b);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((crate::defs::SUBSTATUS_MIST as u8) & 7);
    let value = cpu.read_byte(cpu.hl());
    cpu.flag_z = (value & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(12);
    if !cpu.flag_z {
        cpu.cycle(16);
        become wont_have_any_effect_not_used_message(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((crate::defs::SUBSTATUS_MIST as u8) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value |= mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.cycle(16);
    become use_item_text(cpu);
}

pub(crate) fn dire_hit_effect(cpu: &mut Cpu) {
    cpu.pc = 0x74b8;
    cpu.set_hl(0xc66b);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((crate::defs::SUBSTATUS_FOCUS_ENERGY as u8) & 7);
    let value = cpu.read_byte(cpu.hl());
    cpu.flag_z = (value & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(12);
    if !cpu.flag_z {
        cpu.cycle(16);
        become wont_have_any_effect_not_used_message(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((crate::defs::SUBSTATUS_FOCUS_ENERGY as u8) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value |= mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.cycle(16);
    become use_item_text(cpu);
}

pub(crate) fn x_item_effect(cpu: &mut Cpu) {
    cpu.pc = 0x74c5;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    use_item_text(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xd106);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0x7504);
    cpu.pc += 3;
    cpu.cycle(12);
    become x_item_effect_loop(cpu);
}

pub(crate) fn x_item_effect_loop(cpu: &mut Cpu) {
    cpu.pc = 0x74ce;
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
        become x_item_effect_got_it(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.cycle(12);
    become x_item_effect_loop(cpu);
}

pub(crate) fn x_item_effect_got_it(cpu: &mut Cpu) {
    cpu.pc = 0x74d5;
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.b = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xffe4, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.write_byte(0xc667, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xc70d, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x0d;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x61ef);
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
    crate::game::home::audio::wait_sfx(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x0d;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x63b8);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x0d;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x644c);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xd0d4);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xd109, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.c = crate::defs::HAPPINESS_USEDXITEM as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x01;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x71c2);
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

pub(crate) fn poke_flute_effect(cpu: &mut Cpu) {
    cpu.pc = 0x750c;
    cpu.a = cpu.read_byte(0xd22d);
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
        become poke_flute_effect_in_battle(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become poke_flute_effect_in_battle(cpu);
}

pub(crate) fn poke_flute_effect_in_battle(cpu: &mut Cpu) {
    cpu.pc = 0x7512;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xd002, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.b = !({ let value = ((crate::defs::SLP_MASK as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } });
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xdcff);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    poke_flute_effect_cure_sleep(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xd22d);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = crate::defs::WILD_BATTLE as u8;
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
        become poke_flute_effect_skip_otrainer(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xd2a8);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    poke_flute_effect_cure_sleep(cpu);
    assert_eq!(cpu.pc, next_pc);
    become poke_flute_effect_skip_otrainer(cpu);
}

pub(crate) fn poke_flute_effect_skip_otrainer(cpu: &mut Cpu) {
    cpu.pc = 0x752b;
    cpu.set_hl(0xc63a);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.b;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(0xd214);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.b;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xd002);
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
    cpu.set_hl(0x756c);
    cpu.pc += 3;
    cpu.cycle(12);
    if cpu.flag_z {
        cpu.cycle(16);
        become crate::game::home::text::print_text(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0x7576);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::print_text(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xc2a6);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = { let value = ((1 as i32) << (crate::defs::DANGER_ON_F as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(12);
        become poke_flute_effect_dummy(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become poke_flute_effect_dummy(cpu);
}

pub(crate) fn poke_flute_effect_dummy(cpu: &mut Cpu) {
    cpu.pc = 0x754e;
    cpu.set_hl(0x7571);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become crate::game::home::text::print_text(cpu);
}

pub(crate) fn poke_flute_effect_cure_sleep(cpu: &mut Cpu) {
    cpu.pc = 0x7554;
    cpu.set_de(crate::defs::PARTYMON_STRUCT_LENGTH as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.c = crate::defs::PARTY_LENGTH as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    become poke_flute_effect_loop(cpu);
}

pub(crate) fn poke_flute_effect_loop(cpu: &mut Cpu) {
    cpu.pc = 0x7559;
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    let value = crate::defs::SLP_MASK as u8;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become poke_flute_effect_not_asleep(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = crate::defs::TRUE as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xd002, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    become poke_flute_effect_not_asleep(cpu);
}

pub(crate) fn poke_flute_effect_not_asleep(cpu: &mut Cpu) {
    cpu.pc = 0x7564;
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let value = cpu.b;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
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
        become poke_flute_effect_loop(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn poke_flute_effect_text_asm_0(cpu: &mut Cpu) {
    cpu.pc = 0x757b;
    cpu.a = cpu.read_byte(0xd22d);
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
        become poke_flute_effect_battle(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.push_word(cpu.de());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.set_de(crate::defs::SFX_POKEFLUTE as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::audio::wait_play_sfx(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::audio::wait_sfx(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_de(value);
    cpu.pc += 1;
    cpu.cycle(12);
    become poke_flute_effect_battle(cpu);
}

pub(crate) fn poke_flute_effect_battle(cpu: &mut Cpu) {
    cpu.pc = 0x758c;
    cpu.cycle(16);
    become crate::game::home::text::poke_flute_terminator(cpu);
}

pub(crate) fn blue_card_effect(cpu: &mut Cpu) {
    cpu.pc = 0x758f;
    cpu.set_hl(0x7595);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become crate::game::home::menu::menu_textbox_wait_button(cpu);
}

pub(crate) fn coin_case_effect(cpu: &mut Cpu) {
    cpu.pc = 0x759a;
    cpu.set_hl(0x75a0);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become crate::game::home::menu::menu_textbox_wait_button(cpu);
}

pub(crate) fn old_rod_effect(cpu: &mut Cpu) {
    cpu.pc = 0x75a5;
    cpu.e = 0x0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become use_rod(cpu);
}

pub(crate) fn good_rod_effect(cpu: &mut Cpu) {
    cpu.pc = 0x75a9;
    cpu.e = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become use_rod(cpu);
}

pub(crate) fn super_rod_effect(cpu: &mut Cpu) {
    cpu.pc = 0x75ad;
    cpu.e = 0x2;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become use_rod(cpu);
}

pub(crate) fn use_rod(cpu: &mut Cpu) {
    cpu.pc = 0x75b1;
    cpu.a = 0x03;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x4f8e);
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

pub(crate) fn itemfinder_effect(cpu: &mut Cpu) {
    cpu.pc = 0x75b8;
    cpu.a = 0x04;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x6580);
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

pub(crate) fn restore_pp_effect(cpu: &mut Cpu) {
    cpu.pc = 0x75bf;
    cpu.a = cpu.read_byte(0xd106);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xd002, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    become restore_pp_effect_loop(cpu);
}

pub(crate) fn restore_pp_effect_loop(cpu: &mut Cpu) {
    cpu.pc = 0x75c5;
    cpu.b = crate::defs::PARTYMENUACTION_HEALING_ITEM as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    use_item_select_mon(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.cycle(16);
        become pp_restore_item_cancel(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    become restore_pp_effect_loop2(cpu);
}

pub(crate) fn restore_pp_effect_loop2(cpu: &mut Cpu) {
    cpu.pc = 0x75cd;
    cpu.a = cpu.read_byte(0xd002);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = crate::defs::MAX_ELIXER as u8;
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
        become elixer_restore_p_pof_all_moves(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    let value = crate::defs::ELIXER as u8;
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
        become elixer_restore_p_pof_all_moves(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0x7725);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xd002);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = crate::defs::PP_UP as u8;
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
        become restore_pp_effect_ppup(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x772a);
    cpu.pc += 3;
    cpu.cycle(12);
    become restore_pp_effect_ppup(cpu);
}

pub(crate) fn restore_pp_effect_ppup(cpu: &mut Cpu) {
    cpu.pc = 0x75e7;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::print_text(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xd0d5);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xd0d5, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x2;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xd235, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0x0f;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x64bc);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_bc(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xd0d5, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    if !cpu.flag_z {
        cpu.cycle(12);
        become restore_pp_effect_loop(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xdce1);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(crate::defs::PARTYMON_STRUCT_LENGTH as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    get_mth_move_of_nth_partymon(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(0xd265, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::names::get_move_name(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy_name::copy_name1(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xd002);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = crate::defs::PP_UP as u8;
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
        become not_pp_up(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = crate::defs::SKETCH as u8;
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
        become restore_pp_effect_cant_use_pp_up_on_sketch(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_bc({ const N: i32 = (crate::defs::MON_PP as u16 as i32) - (crate::defs::MON_MOVES as u16 as i32); N as u16 });
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
    let value = crate::defs::PP_UP_MASK as u8;
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
        become restore_pp_effect_do_ppup(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become restore_pp_effect_cant_use_pp_up_on_sketch(cpu);
}

pub(crate) fn restore_pp_effect_cant_use_pp_up_on_sketch(cpu: &mut Cpu) {
    cpu.pc = 0x762f;
    cpu.set_hl(0x772f);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::print_text(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(12);
    become restore_pp_effect_loop2(cpu);
}

pub(crate) fn restore_pp_effect_do_ppup(cpu: &mut Cpu) {
    cpu.pc = 0x7637;
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = crate::defs::PP_UP_ONE as u8;
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
    cpu.a = crate::defs::TRUE as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xd265, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    apply_pp_up(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    play_sfx_full_heal(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0x7734);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::print_text(cpu);
    assert_eq!(cpu.pc, next_pc);
    become finish_pp_restore(cpu);
}

pub(crate) fn finish_pp_restore(cpu: &mut Cpu) {
    cpu.pc = 0x764c;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::tilemap::clear_palettes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(16);
    become use_disposable_item(cpu);
}

pub(crate) fn battle_restore_pp(cpu: &mut Cpu) {
    cpu.pc = 0x7652;
    cpu.a = cpu.read_byte(0xd22d);
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
        become battle_restore_pp_not_in_battle(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xd109);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xd0d4);
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
        become battle_restore_pp_not_in_battle(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc66c);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((crate::defs::SUBSTATUS_TRANSFORMED as u8) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(12);
        become battle_restore_pp_not_in_battle(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    battle_restore_pp_update_battle_mon_pp(cpu);
    assert_eq!(cpu.pc, next_pc);
    become battle_restore_pp_not_in_battle(cpu);
}

pub(crate) fn battle_restore_pp_not_in_battle(cpu: &mut Cpu) {
    cpu.pc = 0x766c;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    play_sfx_full_heal(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0x7739);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::print_text(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(12);
    become finish_pp_restore(cpu);
}

pub(crate) fn battle_restore_pp_update_battle_mon_pp(cpu: &mut Cpu) {
    cpu.pc = 0x7677;
    cpu.a = cpu.read_byte(0xd109);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xdce1);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(crate::defs::PARTYMON_STRUCT_LENGTH as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::array::add_n_times(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_de(0xc62e);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = crate::defs::NUM_MOVES as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    become battle_restore_pp_loop(cpu);
}

pub(crate) fn battle_restore_pp_loop(cpu: &mut Cpu) {
    cpu.pc = 0x7688;
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
    if cpu.flag_z {
        cpu.cycle(12);
        become battle_restore_pp_done(cpu);
    }
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
        become battle_restore_pp_next(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.push_word(cpu.de());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.push_word(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_bc({ const N: i32 = (crate::defs::MON_PP as u16 as i32) - (crate::defs::MON_MOVES as u16 as i32); N as u16 });
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
    cpu.write_byte(cpu.de(), cpu.a);
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
    become battle_restore_pp_next(cpu);
}

pub(crate) fn battle_restore_pp_next(cpu: &mut Cpu) {
    cpu.pc = 0x76a1;
    cpu.set_hl(cpu.hl().wrapping_add(1));
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
        become battle_restore_pp_loop(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become battle_restore_pp_done(cpu);
}

pub(crate) fn battle_restore_pp_done(cpu: &mut Cpu) {
    cpu.pc = 0x76a6;
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn not_pp_up(cpu: &mut Cpu) {
    cpu.pc = 0x76a7;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    restore_pp(cpu);
    assert_eq!(cpu.pc, next_pc);
    if !cpu.flag_z {
        cpu.cycle(12);
        become battle_restore_pp(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become pp_restore_item_no_effect(cpu);
}

pub(crate) fn elixer_restore_p_pof_all_moves(cpu: &mut Cpu) {
    cpu.pc = 0x76af;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl(0xcfa9);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.b = crate::defs::NUM_MOVES as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    become elixer_restore_p_pof_all_moves_move_loop(cpu);
}

pub(crate) fn elixer_restore_p_pof_all_moves_move_loop(cpu: &mut Cpu) {
    cpu.pc = 0x76b7;
    cpu.push_word(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.set_hl(0xdce1);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(crate::defs::PARTYMON_STRUCT_LENGTH as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    get_mth_move_of_nth_partymon(cpu);
    assert_eq!(cpu.pc, next_pc);
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
        become elixer_restore_p_pof_all_moves_next(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    restore_pp(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_z {
        cpu.cycle(12);
        become elixer_restore_p_pof_all_moves_next(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xcfaa);
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
    become elixer_restore_p_pof_all_moves_next(cpu);
}

pub(crate) fn elixer_restore_p_pof_all_moves_next(cpu: &mut Cpu) {
    cpu.pc = 0x76ce;
    cpu.set_hl(0xcfa9);
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
    let value = cpu.pop_word();
    cpu.set_bc(value);
    cpu.pc += 1;
    cpu.cycle(12);
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
        become elixer_restore_p_pof_all_moves_move_loop(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xcfaa);
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
        cpu.cycle(16);
        become battle_restore_pp(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    become pp_restore_item_no_effect(cpu);
}

pub(crate) fn pp_restore_item_no_effect(cpu: &mut Cpu) {
    cpu.pc = 0x76dd;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    wont_have_any_effect_message(cpu);
    assert_eq!(cpu.pc, next_pc);
    become pp_restore_item_cancel(cpu);
}

pub(crate) fn pp_restore_item_cancel(cpu: &mut Cpu) {
    cpu.pc = 0x76e0;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::tilemap::clear_palettes(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xd0ec, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn restore_pp(cpu: &mut Cpu) {
    cpu.pc = 0x76e8;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcf5f, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    get_max_pp_of_move(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xdcf6);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(crate::defs::PARTYMON_STRUCT_LENGTH as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    get_mth_move_of_nth_partymon(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xd265);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = crate::defs::PP_MASK as u8;
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
    if !cpu.flag_c {
        cpu.cycle(12);
        become restore_pp_dont_restore(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xd002);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = crate::defs::MAX_ELIXER as u8;
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
        become restore_pp_restore_all(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = crate::defs::MAX_ETHER as u8;
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
        become restore_pp_restore_all(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.c = 5;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = crate::defs::MYSTERYBERRY as u8;
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
        become restore_pp_restore_some(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.c = 10;
    cpu.pc += 2;
    cpu.cycle(8);
    become restore_pp_restore_some(cpu);
}

pub(crate) fn restore_pp_restore_some(cpu: &mut Cpu) {
    cpu.pc = 0x7715;
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = crate::defs::PP_MASK as u8;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
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
        become restore_pp_restore_all(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    become restore_pp_restore_all(cpu);
}

pub(crate) fn restore_pp_restore_all(cpu: &mut Cpu) {
    cpu.pc = 0x771d;
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = crate::defs::PP_UP_MASK as u8;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
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
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn restore_pp_dont_restore(cpu: &mut Cpu) {
    cpu.pc = 0x7723;
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

pub(crate) fn squirtbottle_effect(cpu: &mut Cpu) {
    cpu.pc = 0x773e;
    cpu.a = 0x14;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x4730);
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

pub(crate) fn card_key_effect(cpu: &mut Cpu) {
    cpu.pc = 0x7745;
    cpu.a = 0x14;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x4779);
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

pub(crate) fn basement_key_effect(cpu: &mut Cpu) {
    cpu.pc = 0x774c;
    cpu.a = 0x14;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x47b4);
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

pub(crate) fn sacred_ash_effect(cpu: &mut Cpu) {
    cpu.pc = 0x7753;
    cpu.a = 0x14;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x47e6);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xd0ec);
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
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    use_disposable_item(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn normal_box_effect(cpu: &mut Cpu) {
    cpu.pc = 0x7763;
    cpu.c = crate::defs::DECOFLAG_SILVER_TROPHY_DOLL as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become open_box(cpu);
}

pub(crate) fn gorgeous_box_effect(cpu: &mut Cpu) {
    cpu.pc = 0x7767;
    cpu.c = crate::defs::DECOFLAG_GOLD_TROPHY_DOLL as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    become open_box(cpu);
}

pub(crate) fn open_box(cpu: &mut Cpu) {
    cpu.pc = 0x7769;
    cpu.a = 0x09;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x6f02);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0x7778);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::print_text(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(16);
    become use_disposable_item(cpu);
}

pub(crate) fn no_effect(cpu: &mut Cpu) {
    cpu.pc = 0x777d;
    cpu.cycle(16);
    become isnt_the_time_message(cpu);
}

pub(crate) fn play_sfx_full_heal(cpu: &mut Cpu) {
    cpu.pc = 0x7780;
    cpu.push_word(cpu.de());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.set_de(crate::defs::SFX_FULL_HEAL as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::audio::wait_play_sfx(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_de(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn use_item_text(cpu: &mut Cpu) {
    cpu.pc = 0x7789;
    cpu.set_hl(0x783d);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::print_text(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    play_sfx_full_heal(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::joypad::wait_press_aor_b_blink_cursor(cpu);
    assert_eq!(cpu.pc, next_pc);
    become use_disposable_item(cpu);
}

pub(crate) fn use_disposable_item(cpu: &mut Cpu) {
    cpu.pc = 0x7795;
    cpu.set_hl(0xd892);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xd10c, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(16);
    become crate::game::home::item::toss_item(cpu);
}

pub(crate) fn use_ball_in_trainer_battle(cpu: &mut Cpu) {
    cpu.pc = 0x77a0;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    return_to_battle_use_ball(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_de(crate::defs::ANIM_THROW_POKE_BALL as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcfc2, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte({ const N: i32 = (0xcfc2 as i32) + (1 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc689, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xffe4, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.write_byte(0xcfca, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = { let value = (((0x4610 as i32) - (0x456b as i32)) / (3 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::predef::predef(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0x7824);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::print_text(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0x7829);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::print_text(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(12);
    become use_disposable_item(cpu);
}

pub(crate) fn wont_have_any_effect_not_used_message(cpu: &mut Cpu) {
    cpu.pc = 0x77ca;
    cpu.set_hl(0x781f);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::print_text(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x2;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xd0ec, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn looks_bitter_message(cpu: &mut Cpu) {
    cpu.pc = 0x77d6;
    cpu.set_hl(0x780b);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become crate::game::home::text::print_text(cpu);
}

pub(crate) fn ball_box_is_full_message(cpu: &mut Cpu) {
    cpu.pc = 0x77dc;
    cpu.set_hl(0x7838);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::print_text(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x2;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xd0ec, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn cant_use_on_egg_message(cpu: &mut Cpu) {
    cpu.pc = 0x77e8;
    cpu.set_hl(0x7810);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(12);
    become cant_use_item_message(cpu);
}

pub(crate) fn isnt_the_time_message(cpu: &mut Cpu) {
    cpu.pc = 0x77ed;
    cpu.set_hl(0x7815);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(12);
    become cant_use_item_message(cpu);
}

pub(crate) fn wont_have_any_effect_message(cpu: &mut Cpu) {
    cpu.pc = 0x77f2;
    cpu.set_hl(0x781f);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(12);
    become cant_use_item_message(cpu);
}

pub(crate) fn belongs_to_someone_else_message(cpu: &mut Cpu) {
    cpu.pc = 0x77f7;
    cpu.set_hl(0x781a);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(12);
    become cant_use_item_message(cpu);
}

pub(crate) fn cycling_isnt_allowed_message(cpu: &mut Cpu) {
    cpu.pc = 0x77fc;
    cpu.set_hl(0x782e);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(12);
    become cant_use_item_message(cpu);
}

pub(crate) fn cant_get_on_your_bike_message(cpu: &mut Cpu) {
    cpu.pc = 0x7801;
    cpu.set_hl(0x7833);
    cpu.pc += 3;
    cpu.cycle(12);
    become cant_use_item_message(cpu);
}

pub(crate) fn cant_use_item_message(cpu: &mut Cpu) {
    cpu.pc = 0x7804;
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xd0ec, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(16);
    become crate::game::home::text::print_text(cpu);
}

pub(crate) fn apply_pp_up(cpu: &mut Cpu) {
    cpu.pc = 0x784c;
    cpu.a = crate::defs::MON_MOVES as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::battle::get_party_param_location(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.set_de(0xd1ea);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = { let value = (((0x457a as i32) - (0x456b as i32)) / (3 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::predef::predef(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.set_bc({ const N: i32 = (crate::defs::MON_PP as u16 as i32) - (crate::defs::MON_MOVES as u16 as i32); N as u16 });
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
    cpu.set_de(0xd1ea);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = 0;
    cpu.pc += 2;
    cpu.cycle(8);
    become apply_pp_up_loop(cpu);
}

pub(crate) fn apply_pp_up_loop(cpu: &mut Cpu) {
    cpu.pc = 0x7864;
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
    let value = { let value = ((crate::defs::NUM_MOVES as i32) + (1 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
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
    cpu.a = cpu.read_byte(0xd265);
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
    if !cpu.flag_z {
        cpu.cycle(12);
        become apply_pp_up_use(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xcfa9);
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
        become apply_pp_up_skip(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become apply_pp_up_use(cpu);
}

pub(crate) fn apply_pp_up_use(cpu: &mut Cpu) {
    cpu.pc = 0x7876;
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = crate::defs::PP_UP_MASK as u8;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    if !cpu.flag_z {
        let next_pc = cpu.pc + 3;
        cpu.push_word(next_pc);
        cpu.cycle(24);
        compute_max_pp(cpu);
        assert_eq!(cpu.pc, next_pc);
    } else {
        cpu.pc += 3;
        cpu.cycle(12);
    }
    become apply_pp_up_skip(cpu);
}

pub(crate) fn apply_pp_up_skip(cpu: &mut Cpu) {
    cpu.pc = 0x787d;
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.cycle(12);
    become apply_pp_up_loop(cpu);
}

pub(crate) fn compute_max_pp(cpu: &mut Cpu) {
    cpu.pc = 0x7881;
    cpu.push_word(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte({ const N: i32 = (0xffb3 as i32) + (3 as i32); N as u16 }, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xffb3, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.write_byte({ const N: i32 = (0xffb3 as i32) + (1 as i32); N as u16 }, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.write_byte({ const N: i32 = (0xffb3 as i32) + (2 as i32); N as u16 }, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = 5;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xffb7, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.b = 4;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::math::divide(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.b = cpu.a;
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
    let value = 0xf;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
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
    let value = cpu.a;
    let result = value >> 1;
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = value & 0x01 != 0;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.c = cpu.a;
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
        become compute_max_pp_no_pp_up(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become compute_max_pp_loop(cpu);
}

pub(crate) fn compute_max_pp_loop(cpu: &mut Cpu) {
    cpu.pc = 0x78a3;
    cpu.a = cpu.read_byte({ const N: i32 = (0xffb3 as i32) + (3 as i32); N as u16 });
    cpu.pc += 2;
    cpu.cycle(12);
    let value = 0x8;
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
        become compute_max_pp_okay(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x7;
    cpu.pc += 2;
    cpu.cycle(8);
    become compute_max_pp_okay(cpu);
}

pub(crate) fn compute_max_pp_okay(cpu: &mut Cpu) {
    cpu.pc = 0x78ab;
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
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xd265);
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
        cpu.cycle(12);
        become compute_max_pp_no_pp_up(cpu);
    }
    cpu.pc += 2;
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
        become compute_max_pp_loop(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become compute_max_pp_no_pp_up(cpu);
}

pub(crate) fn compute_max_pp_no_pp_up(cpu: &mut Cpu) {
    cpu.pc = 0x78b6;
    cpu.write_byte(cpu.hl(), cpu.b);
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.pop_word();
    cpu.set_bc(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn restore_all_pp(cpu: &mut Cpu) {
    cpu.pc = 0x78b9;
    cpu.a = crate::defs::MON_PP as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::battle::get_party_param_location(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = crate::defs::MON_MOVES as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::battle::get_party_param_location(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_de(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xcfa9, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xcf5f, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.c = crate::defs::NUM_MOVES as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    become restore_all_pp_loop(cpu);
}

pub(crate) fn restore_all_pp_loop(cpu: &mut Cpu) {
    cpu.pc = 0x78ce;
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
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.push_word(cpu.de());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.push_word(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    get_max_pp_of_move(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_bc(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let value = cpu.pop_word();
    cpu.set_de(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = crate::defs::PP_UP_MASK as u8;
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
    cpu.a = cpu.read_byte(0xd265);
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
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(0xcfa9);
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
    let value = cpu.pop_word();
    cpu.set_hl(value);
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
        become restore_all_pp_loop(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn get_max_pp_of_move(cpu: &mut Cpu) {
    cpu.pc = 0x78ec;
    cpu.a = cpu.read_byte({ const N: i32 = (0xd073 as i32) + (0 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = cpu.read_byte({ const N: i32 = (0xd073 as i32) + (1 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xcf5f);
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
    cpu.set_hl(0xdce1);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(crate::defs::PARTYMON_STRUCT_LENGTH as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    if cpu.flag_z {
        cpu.cycle(12);
        become get_max_pp_of_move_got_partymon(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xd28a);
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
        become get_max_pp_of_move_got_partymon(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xd110);
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
        become get_max_pp_of_move_got_nonpartymon(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xd110);
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
        become get_max_pp_of_move_got_nonpartymon(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xc62e);
    cpu.pc += 3;
    cpu.cycle(12);
    become get_max_pp_of_move_got_nonpartymon(cpu);
}

pub(crate) fn get_max_pp_of_move_got_nonpartymon(cpu: &mut Cpu) {
    cpu.pc = 0x7915;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    get_mth_move_of_current_mon(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(12);
    become get_max_pp_of_move_gotdatmove(cpu);
}

pub(crate) fn get_max_pp_of_move_got_partymon(cpu: &mut Cpu) {
    cpu.pc = 0x791a;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    get_mth_move_of_nth_partymon(cpu);
    assert_eq!(cpu.pc, next_pc);
    become get_max_pp_of_move_gotdatmove(cpu);
}

pub(crate) fn get_max_pp_of_move_gotdatmove(cpu: &mut Cpu) {
    cpu.pc = 0x791d;
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
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.set_hl({ const N: i32 = (0x5afb as i32) + (crate::defs::MOVE_PP as u16 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(crate::defs::MOVE_LENGTH as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::array::add_n_times(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x10;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::copy::get_far_byte(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_de(0xd073);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.push_word(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.set_bc({ const N: i32 = (crate::defs::MON_PP as u16 as i32) - (crate::defs::MON_MOVES as u16 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xcf5f);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = crate::defs::WILDMON as u8;
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
        become get_max_pp_of_move_notwild(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_bc({ const N: i32 = (0xd20e as i32) - (0xd208 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    become get_max_pp_of_move_notwild(cpu);
}

pub(crate) fn get_max_pp_of_move_notwild(cpu: &mut Cpu) {
    cpu.pc = 0x7942;
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
    let value = crate::defs::PP_UP_MASK as u8;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.pop_word();
    cpu.set_bc(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let value = cpu.b;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl({ const N: i32 = (0xd073 as i32) + (1 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
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
    cpu.write_byte(0xd265, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    compute_max_pp(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = crate::defs::PP_MASK as u8;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xd265, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte({ const N: i32 = (0xd073 as i32) + (1 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte({ const N: i32 = (0xd073 as i32) + (0 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn get_mth_move_of_nth_partymon(cpu: &mut Cpu) {
    cpu.pc = 0x7963;
    cpu.a = cpu.read_byte(0xd109);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::array::add_n_times(cpu);
    assert_eq!(cpu.pc, next_pc);
    become get_mth_move_of_current_mon(cpu);
}

pub(crate) fn get_mth_move_of_current_mon(cpu: &mut Cpu) {
    cpu.pc = 0x7969;
    cpu.a = cpu.read_byte(0xcfa9);
    cpu.pc += 3;
    cpu.cycle(16);
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
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn poke_ball_effect_call_ball_function(cpu: &mut Cpu) {
    cpu.set_de(0x6906);
    cpu.push_word(cpu.de());
    match cpu.read_byte(0xd106) {
        v if v == crate::defs::ULTRA_BALL  => ultra_ball_multiplier(cpu),
        v if v == crate::defs::GREAT_BALL  => great_ball_multiplier(cpu),
        v if v == crate::defs::SAFARI_BALL => safari_ball_multiplier(cpu),
        v if v == crate::defs::HEAVY_BALL  => heavy_ball_multiplier(cpu),
        v if v == crate::defs::LEVEL_BALL  => level_ball_multiplier(cpu),
        v if v == crate::defs::LURE_BALL   => lure_ball_multiplier(cpu),
        v if v == crate::defs::FAST_BALL   => fast_ball_multiplier(cpu),
        v if v == crate::defs::MOON_BALL   => moon_ball_multiplier(cpu),
        v if v == crate::defs::LOVE_BALL   => love_ball_multiplier(cpu),
        v if v == crate::defs::PARK_BALL   => park_ball_multiplier(cpu),
        n => panic!("unsupported ball type in BallMultiplierFunctionTable: {}", n),
    }
    assert_eq!(cpu.pc, 0x6906);
    become poke_ball_effect_skip_or_return_from_ball_fn(cpu)
}
