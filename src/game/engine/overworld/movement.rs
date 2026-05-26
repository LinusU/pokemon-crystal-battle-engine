use crate::runtime::Cpu;

pub const CURRENT_TABLE_WIDTH: u8 = 2;

pub const W: u8 = 90;

pub const X: u8 = 180;

pub const Y: i32 = 0;

pub(crate) fn movement_teleport_from(cpu: &mut Cpu) {
    cpu.pc = 0x5129;
    cpu.set_hl(crate::defs::OBJECT_STEP_TYPE as u16);
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
    cpu.write_byte(cpu.hl(), crate::defs::STEP_TYPE_TELEPORT_FROM as u8);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn movement_teleport_to(cpu: &mut Cpu) {
    cpu.pc = 0x5130;
    cpu.set_hl(crate::defs::OBJECT_STEP_TYPE as u16);
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
    cpu.write_byte(cpu.hl(), crate::defs::STEP_TYPE_TELEPORT_TO as u8);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn movement_skyfall(cpu: &mut Cpu) {
    cpu.pc = 0x5137;
    cpu.set_hl(crate::defs::OBJECT_STEP_TYPE as u16);
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
    cpu.write_byte(cpu.hl(), crate::defs::STEP_TYPE_SKYFALL as u8);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn movement_skyfall_top(cpu: &mut Cpu) {
    cpu.pc = 0x513e;
    cpu.set_hl(crate::defs::OBJECT_STEP_TYPE as u16);
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
    cpu.write_byte(cpu.hl(), crate::defs::STEP_TYPE_SKYFALL_TOP as u8);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn movement_step_dig(cpu: &mut Cpu) {
    cpu.pc = 0x5145;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map_objects::get_sprite_direction(cpu);
    assert_eq!(cpu.pc, next_pc);
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
    cpu.set_hl(crate::defs::OBJECT_STEP_FRAME as u16);
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
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(crate::defs::OBJECT_ACTION as u16);
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
    cpu.write_byte(cpu.hl(), crate::defs::OBJECT_ACTION_SPIN as u8);
    cpu.pc += 2;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::engine::overworld::map_objects::jump_movement_pointer(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(crate::defs::OBJECT_STEP_DURATION as u16);
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
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(crate::defs::OBJECT_STEP_TYPE as u16);
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
    cpu.write_byte(cpu.hl(), crate::defs::STEP_TYPE_SLEEP as u8);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.set_hl(crate::defs::OBJECT_WALKING as u16);
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
    cpu.write_byte(cpu.hl(), crate::defs::STANDING as u8);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn movement_return_dig(cpu: &mut Cpu) {
    cpu.pc = 0x516a;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map_objects::get_sprite_direction(cpu);
    assert_eq!(cpu.pc, next_pc);
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
    cpu.set_hl(crate::defs::OBJECT_STEP_FRAME as u16);
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
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::engine::overworld::map_objects::jump_movement_pointer(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(crate::defs::OBJECT_STEP_DURATION as u16);
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
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(crate::defs::OBJECT_WALKING as u16);
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
    cpu.write_byte(cpu.hl(), crate::defs::STANDING as u8);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.set_hl(crate::defs::OBJECT_STEP_TYPE as u16);
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
    cpu.write_byte(cpu.hl(), crate::defs::STEP_TYPE_RETURN_DIG as u8);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn movement_fish_got_bite(cpu: &mut Cpu) {
    cpu.pc = 0x5189;
    cpu.set_hl(crate::defs::OBJECT_ACTION as u16);
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
    cpu.write_byte(cpu.hl(), crate::defs::OBJECT_ACTION_FISHING as u8);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.set_hl(crate::defs::OBJECT_STEP_TYPE as u16);
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
    cpu.write_byte(cpu.hl(), crate::defs::STEP_TYPE_GOT_BITE as u8);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn movement_rock_smash(cpu: &mut Cpu) {
    cpu.pc = 0x5196;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::engine::overworld::map_objects::jump_movement_pointer(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(crate::defs::OBJECT_STEP_DURATION as u16);
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
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(crate::defs::OBJECT_ACTION as u16);
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
    cpu.write_byte(cpu.hl(), crate::defs::OBJECT_ACTION_STAND as u8);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.set_hl(crate::defs::OBJECT_STEP_TYPE as u16);
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
    cpu.write_byte(cpu.hl(), crate::defs::STEP_TYPE_ROCK_SMASH as u8);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn movement_fish_cast_rod(cpu: &mut Cpu) {
    cpu.pc = 0x51ab;
    cpu.set_hl(crate::defs::OBJECT_ACTION as u16);
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
    cpu.write_byte(cpu.hl(), crate::defs::OBJECT_ACTION_FISHING as u8);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.set_hl(crate::defs::OBJECT_STEP_TYPE as u16);
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
    cpu.write_byte(cpu.hl(), crate::defs::STEP_TYPE_FROM_MOVEMENT as u8);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn movement_step_loop(cpu: &mut Cpu) {
    cpu.pc = 0x51b8;
    cpu.set_hl(crate::defs::OBJECT_MOVEMENT_INDEX as u16);
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
    cpu.write_byte(cpu.hl(), 0x0);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.cycle(16);
    become crate::game::engine::overworld::map_objects::continue_reading_movement(cpu);
}

pub(crate) fn movement_step_end(cpu: &mut Cpu) {
    cpu.pc = 0x51c1;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::engine::overworld::map_objects::restore_default_movement(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(crate::defs::OBJECT_MOVEMENT_TYPE as u16);
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
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(crate::defs::OBJECT_MOVEMENT_INDEX as u16);
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
    cpu.write_byte(cpu.hl(), 0x0);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.set_hl(0xd0ed);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((crate::defs::SCRIPTED_MOVEMENT_STATE_F as u8) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.set_hl(crate::defs::OBJECT_STEP_TYPE as u16);
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
    cpu.write_byte(cpu.hl(), crate::defs::STEP_TYPE_FROM_MOVEMENT as u8);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn movement_step_wait_end(cpu: &mut Cpu) {
    cpu.pc = 0x51db;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::engine::overworld::map_objects::restore_default_movement(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(crate::defs::OBJECT_MOVEMENT_TYPE as u16);
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
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(crate::defs::OBJECT_MOVEMENT_INDEX as u16);
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
    cpu.write_byte(cpu.hl(), 0x0);
    cpu.pc += 2;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::engine::overworld::map_objects::jump_movement_pointer(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(crate::defs::OBJECT_STEP_DURATION as u16);
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
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(crate::defs::OBJECT_STEP_TYPE as u16);
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
    cpu.write_byte(cpu.hl(), crate::defs::STEP_TYPE_SLEEP as u8);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.set_hl(0xd0ed);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((crate::defs::SCRIPTED_MOVEMENT_STATE_F as u8) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn movement_remove_object(cpu: &mut Cpu) {
    cpu.pc = 0x51fd;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::engine::overworld::map_objects::delete_map_object(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xd4cd);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xffaf);
    cpu.pc += 2;
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
    if !cpu.flag_z {
        cpu.cycle(12);
        become movement_remove_object_not_leading(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), -1i16 as u8);
    cpu.pc += 2;
    cpu.cycle(12);
    become movement_remove_object_not_leading(cpu);
}

pub(crate) fn movement_remove_object_not_leading(cpu: &mut Cpu) {
    cpu.pc = 0x520a;
    cpu.set_hl(0xd0ed);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((crate::defs::SCRIPTED_MOVEMENT_STATE_F as u8) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn movement_stop(cpu: &mut Cpu) {
    cpu.pc = 0x5210;
    cpu.set_hl(crate::defs::OBJECT_ACTION as u16);
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
    cpu.write_byte(cpu.hl(), crate::defs::OBJECT_ACTION_STAND as u8);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.set_hl(crate::defs::OBJECT_STEP_TYPE as u16);
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
    cpu.write_byte(cpu.hl(), crate::defs::STEP_TYPE_STANDING as u8);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.set_hl(0xd0ed);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((crate::defs::SCRIPTED_MOVEMENT_STATE_F as u8) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn movement_step_sleep_1(cpu: &mut Cpu) {
    cpu.pc = 0x5222;
    cpu.a = 1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become movement_step_sleep_common(cpu);
}

pub(crate) fn movement_step_sleep_2(cpu: &mut Cpu) {
    cpu.pc = 0x5226;
    cpu.a = 2;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become movement_step_sleep_common(cpu);
}

pub(crate) fn movement_step_sleep_3(cpu: &mut Cpu) {
    cpu.pc = 0x522a;
    cpu.a = 3;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become movement_step_sleep_common(cpu);
}

pub(crate) fn movement_step_sleep_4(cpu: &mut Cpu) {
    cpu.pc = 0x522e;
    cpu.a = 4;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become movement_step_sleep_common(cpu);
}

pub(crate) fn movement_step_sleep_5(cpu: &mut Cpu) {
    cpu.pc = 0x5232;
    cpu.a = 5;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become movement_step_sleep_common(cpu);
}

pub(crate) fn movement_step_sleep_6(cpu: &mut Cpu) {
    cpu.pc = 0x5236;
    cpu.a = 6;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become movement_step_sleep_common(cpu);
}

pub(crate) fn movement_step_sleep_7(cpu: &mut Cpu) {
    cpu.pc = 0x523a;
    cpu.a = 7;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become movement_step_sleep_common(cpu);
}

pub(crate) fn movement_step_sleep_8(cpu: &mut Cpu) {
    cpu.pc = 0x523e;
    cpu.a = 8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become movement_step_sleep_common(cpu);
}

pub(crate) fn movement_step_sleep(cpu: &mut Cpu) {
    cpu.pc = 0x5242;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::engine::overworld::map_objects::jump_movement_pointer(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(12);
    become movement_step_sleep_common(cpu);
}

pub(crate) fn movement_step_sleep_common(cpu: &mut Cpu) {
    cpu.pc = 0x5247;
    cpu.set_hl(crate::defs::OBJECT_STEP_DURATION as u16);
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
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(crate::defs::OBJECT_STEP_TYPE as u16);
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
    cpu.write_byte(cpu.hl(), crate::defs::STEP_TYPE_SLEEP as u8);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.set_hl(crate::defs::OBJECT_ACTION as u16);
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
    cpu.write_byte(cpu.hl(), crate::defs::OBJECT_ACTION_STAND as u8);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.set_hl(crate::defs::OBJECT_WALKING as u16);
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
    cpu.write_byte(cpu.hl(), crate::defs::STANDING as u8);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn movement_step_bump(cpu: &mut Cpu) {
    cpu.pc = 0x525f;
    cpu.a = 1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(crate::defs::OBJECT_STEP_DURATION as u16);
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
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(crate::defs::OBJECT_STEP_TYPE as u16);
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
    cpu.write_byte(cpu.hl(), crate::defs::STEP_TYPE_BUMP as u8);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.set_hl(crate::defs::OBJECT_ACTION as u16);
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
    cpu.write_byte(cpu.hl(), crate::defs::OBJECT_ACTION_BUMP as u8);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.set_hl(crate::defs::OBJECT_WALKING as u16);
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
    cpu.write_byte(cpu.hl(), crate::defs::STANDING as u8);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn movement_tree_shake(cpu: &mut Cpu) {
    cpu.pc = 0x5279;
    cpu.a = 24;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(crate::defs::OBJECT_STEP_DURATION as u16);
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
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(crate::defs::OBJECT_STEP_TYPE as u16);
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
    cpu.write_byte(cpu.hl(), crate::defs::STEP_TYPE_SLEEP as u8);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.set_hl(crate::defs::OBJECT_ACTION as u16);
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
    cpu.write_byte(cpu.hl(), crate::defs::OBJECT_ACTION_WEIRD_TREE as u8);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.set_hl(crate::defs::OBJECT_WALKING as u16);
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
    cpu.write_byte(cpu.hl(), crate::defs::STANDING as u8);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn movement_remove_sliding(cpu: &mut Cpu) {
    cpu.pc = 0x5293;
    cpu.set_hl(crate::defs::OBJECT_FLAGS1 as u16);
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
    let mask = 1u8 << ((crate::defs::SLIDING_F as u8) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.cycle(16);
    become crate::game::engine::overworld::map_objects::continue_reading_movement(cpu);
}

pub(crate) fn movement_set_sliding(cpu: &mut Cpu) {
    cpu.pc = 0x529c;
    cpu.set_hl(crate::defs::OBJECT_FLAGS1 as u16);
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
    let mask = 1u8 << ((crate::defs::SLIDING_F as u8) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value |= mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.cycle(16);
    become crate::game::engine::overworld::map_objects::continue_reading_movement(cpu);
}

pub(crate) fn movement_remove_fixed_facing(cpu: &mut Cpu) {
    cpu.pc = 0x52a5;
    cpu.set_hl(crate::defs::OBJECT_FLAGS1 as u16);
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
    let mask = 1u8 << ((crate::defs::FIXED_FACING_F as u8) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.cycle(16);
    become crate::game::engine::overworld::map_objects::continue_reading_movement(cpu);
}

pub(crate) fn movement_fix_facing(cpu: &mut Cpu) {
    cpu.pc = 0x52ae;
    cpu.set_hl(crate::defs::OBJECT_FLAGS1 as u16);
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
    let mask = 1u8 << ((crate::defs::FIXED_FACING_F as u8) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value |= mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.cycle(16);
    become crate::game::engine::overworld::map_objects::continue_reading_movement(cpu);
}

pub(crate) fn movement_show_object(cpu: &mut Cpu) {
    cpu.pc = 0x52b7;
    cpu.set_hl(crate::defs::OBJECT_FLAGS1 as u16);
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
    let mask = 1u8 << ((crate::defs::INVISIBLE_F as u8) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.cycle(16);
    become crate::game::engine::overworld::map_objects::continue_reading_movement(cpu);
}

pub(crate) fn movement_hide_object(cpu: &mut Cpu) {
    cpu.pc = 0x52c0;
    cpu.set_hl(crate::defs::OBJECT_FLAGS1 as u16);
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
    let mask = 1u8 << ((crate::defs::INVISIBLE_F as u8) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value |= mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.cycle(16);
    become crate::game::engine::overworld::map_objects::continue_reading_movement(cpu);
}

pub(crate) fn movement_hide_emote(cpu: &mut Cpu) {
    cpu.pc = 0x52c9;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::engine::overworld::map_objects::despawn_emote(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(16);
    become crate::game::engine::overworld::map_objects::continue_reading_movement(cpu);
}

pub(crate) fn movement_show_emote(cpu: &mut Cpu) {
    cpu.pc = 0x52cf;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::engine::overworld::map_objects::spawn_emote(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(16);
    become crate::game::engine::overworld::map_objects::continue_reading_movement(cpu);
}

pub(crate) fn movement_step_shake(cpu: &mut Cpu) {
    cpu.pc = 0x52d5;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::engine::overworld::map_objects::jump_movement_pointer(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::engine::overworld::map_objects::shake_screen(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(16);
    become crate::game::engine::overworld::map_objects::continue_reading_movement(cpu);
}

pub(crate) fn movement_turn_head_down(cpu: &mut Cpu) {
    cpu.pc = 0x52de;
    cpu.a = crate::defs::OW_DOWN as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become turn_head(cpu);
}

pub(crate) fn movement_turn_head_up(cpu: &mut Cpu) {
    cpu.pc = 0x52e2;
    cpu.a = crate::defs::OW_UP as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become turn_head(cpu);
}

pub(crate) fn movement_turn_head_left(cpu: &mut Cpu) {
    cpu.pc = 0x52e6;
    cpu.a = crate::defs::OW_LEFT as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become turn_head(cpu);
}

pub(crate) fn movement_turn_head_right(cpu: &mut Cpu) {
    cpu.pc = 0x52ea;
    cpu.a = crate::defs::OW_RIGHT as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become turn_head(cpu);
}

pub(crate) fn turn_head(cpu: &mut Cpu) {
    cpu.pc = 0x52ee;
    cpu.set_hl(crate::defs::OBJECT_DIRECTION as u16);
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
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(crate::defs::OBJECT_ACTION as u16);
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
    cpu.write_byte(cpu.hl(), crate::defs::OBJECT_ACTION_STAND as u8);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.set_hl(crate::defs::OBJECT_WALKING as u16);
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
    cpu.write_byte(cpu.hl(), crate::defs::STANDING as u8);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn movement_slow_step_down(cpu: &mut Cpu) {
    cpu.pc = 0x5300;
    cpu.a = { let value = (((crate::defs::STEP_SLOW as i32) << (2 as i32)) | (crate::defs::DOWN as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become normal_step(cpu);
}

pub(crate) fn movement_slow_step_up(cpu: &mut Cpu) {
    cpu.pc = 0x5305;
    cpu.a = { let value = (((crate::defs::STEP_SLOW as i32) << (2 as i32)) | (crate::defs::UP as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become normal_step(cpu);
}

pub(crate) fn movement_slow_step_left(cpu: &mut Cpu) {
    cpu.pc = 0x530a;
    cpu.a = { let value = (((crate::defs::STEP_SLOW as i32) << (2 as i32)) | (crate::defs::LEFT as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become normal_step(cpu);
}

pub(crate) fn movement_slow_step_right(cpu: &mut Cpu) {
    cpu.pc = 0x530f;
    cpu.a = { let value = (((crate::defs::STEP_SLOW as i32) << (2 as i32)) | (crate::defs::RIGHT as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become normal_step(cpu);
}

pub(crate) fn movement_step_down(cpu: &mut Cpu) {
    cpu.pc = 0x5314;
    cpu.a = { let value = (((crate::defs::STEP_WALK as i32) << (2 as i32)) | (crate::defs::DOWN as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become normal_step(cpu);
}

pub(crate) fn movement_step_up(cpu: &mut Cpu) {
    cpu.pc = 0x5319;
    cpu.a = { let value = (((crate::defs::STEP_WALK as i32) << (2 as i32)) | (crate::defs::UP as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become normal_step(cpu);
}

pub(crate) fn movement_step_left(cpu: &mut Cpu) {
    cpu.pc = 0x531e;
    cpu.a = { let value = (((crate::defs::STEP_WALK as i32) << (2 as i32)) | (crate::defs::LEFT as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become normal_step(cpu);
}

pub(crate) fn movement_step_right(cpu: &mut Cpu) {
    cpu.pc = 0x5323;
    cpu.a = { let value = (((crate::defs::STEP_WALK as i32) << (2 as i32)) | (crate::defs::RIGHT as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become normal_step(cpu);
}

pub(crate) fn movement_big_step_down(cpu: &mut Cpu) {
    cpu.pc = 0x5328;
    cpu.a = { let value = (((crate::defs::STEP_BIKE as i32) << (2 as i32)) | (crate::defs::DOWN as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become normal_step(cpu);
}

pub(crate) fn movement_big_step_up(cpu: &mut Cpu) {
    cpu.pc = 0x532d;
    cpu.a = { let value = (((crate::defs::STEP_BIKE as i32) << (2 as i32)) | (crate::defs::UP as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become normal_step(cpu);
}

pub(crate) fn movement_big_step_left(cpu: &mut Cpu) {
    cpu.pc = 0x5332;
    cpu.a = { let value = (((crate::defs::STEP_BIKE as i32) << (2 as i32)) | (crate::defs::LEFT as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become normal_step(cpu);
}

pub(crate) fn movement_big_step_right(cpu: &mut Cpu) {
    cpu.pc = 0x5337;
    cpu.a = { let value = (((crate::defs::STEP_BIKE as i32) << (2 as i32)) | (crate::defs::RIGHT as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become normal_step(cpu);
}

pub(crate) fn movement_turn_away_down(cpu: &mut Cpu) {
    cpu.pc = 0x533c;
    cpu.a = { let value = (((crate::defs::STEP_SLOW as i32) << (2 as i32)) | (crate::defs::DOWN as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become turning_step(cpu);
}

pub(crate) fn movement_turn_away_up(cpu: &mut Cpu) {
    cpu.pc = 0x5341;
    cpu.a = { let value = (((crate::defs::STEP_SLOW as i32) << (2 as i32)) | (crate::defs::UP as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become turning_step(cpu);
}

pub(crate) fn movement_turn_away_left(cpu: &mut Cpu) {
    cpu.pc = 0x5346;
    cpu.a = { let value = (((crate::defs::STEP_SLOW as i32) << (2 as i32)) | (crate::defs::LEFT as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become turning_step(cpu);
}

pub(crate) fn movement_turn_away_right(cpu: &mut Cpu) {
    cpu.pc = 0x534b;
    cpu.a = { let value = (((crate::defs::STEP_SLOW as i32) << (2 as i32)) | (crate::defs::RIGHT as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become turning_step(cpu);
}

pub(crate) fn movement_turn_in_down(cpu: &mut Cpu) {
    cpu.pc = 0x5350;
    cpu.a = { let value = (((crate::defs::STEP_WALK as i32) << (2 as i32)) | (crate::defs::DOWN as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become turning_step(cpu);
}

pub(crate) fn movement_turn_in_up(cpu: &mut Cpu) {
    cpu.pc = 0x5355;
    cpu.a = { let value = (((crate::defs::STEP_WALK as i32) << (2 as i32)) | (crate::defs::UP as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become turning_step(cpu);
}

pub(crate) fn movement_turn_in_left(cpu: &mut Cpu) {
    cpu.pc = 0x535a;
    cpu.a = { let value = (((crate::defs::STEP_WALK as i32) << (2 as i32)) | (crate::defs::LEFT as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become turning_step(cpu);
}

pub(crate) fn movement_turn_in_right(cpu: &mut Cpu) {
    cpu.pc = 0x535f;
    cpu.a = { let value = (((crate::defs::STEP_WALK as i32) << (2 as i32)) | (crate::defs::RIGHT as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become turning_step(cpu);
}

pub(crate) fn movement_turn_waterfall_down(cpu: &mut Cpu) {
    cpu.pc = 0x5364;
    cpu.a = { let value = (((crate::defs::STEP_BIKE as i32) << (2 as i32)) | (crate::defs::DOWN as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become turning_step(cpu);
}

pub(crate) fn movement_turn_waterfall_up(cpu: &mut Cpu) {
    cpu.pc = 0x5369;
    cpu.a = { let value = (((crate::defs::STEP_BIKE as i32) << (2 as i32)) | (crate::defs::UP as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become turning_step(cpu);
}

pub(crate) fn movement_turn_waterfall_left(cpu: &mut Cpu) {
    cpu.pc = 0x536e;
    cpu.a = { let value = (((crate::defs::STEP_BIKE as i32) << (2 as i32)) | (crate::defs::LEFT as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become turning_step(cpu);
}

pub(crate) fn movement_turn_waterfall_right(cpu: &mut Cpu) {
    cpu.pc = 0x5373;
    cpu.a = { let value = (((crate::defs::STEP_BIKE as i32) << (2 as i32)) | (crate::defs::RIGHT as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become turning_step(cpu);
}

pub(crate) fn movement_slow_slide_step_down(cpu: &mut Cpu) {
    cpu.pc = 0x5378;
    cpu.a = { let value = (((crate::defs::STEP_SLOW as i32) << (2 as i32)) | (crate::defs::DOWN as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become slide_step(cpu);
}

pub(crate) fn movement_slow_slide_step_up(cpu: &mut Cpu) {
    cpu.pc = 0x537d;
    cpu.a = { let value = (((crate::defs::STEP_SLOW as i32) << (2 as i32)) | (crate::defs::UP as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become slide_step(cpu);
}

pub(crate) fn movement_slow_slide_step_left(cpu: &mut Cpu) {
    cpu.pc = 0x5382;
    cpu.a = { let value = (((crate::defs::STEP_SLOW as i32) << (2 as i32)) | (crate::defs::LEFT as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become slide_step(cpu);
}

pub(crate) fn movement_slow_slide_step_right(cpu: &mut Cpu) {
    cpu.pc = 0x5387;
    cpu.a = { let value = (((crate::defs::STEP_SLOW as i32) << (2 as i32)) | (crate::defs::RIGHT as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become slide_step(cpu);
}

pub(crate) fn movement_slide_step_down(cpu: &mut Cpu) {
    cpu.pc = 0x538c;
    cpu.a = { let value = (((crate::defs::STEP_WALK as i32) << (2 as i32)) | (crate::defs::DOWN as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become slide_step(cpu);
}

pub(crate) fn movement_slide_step_up(cpu: &mut Cpu) {
    cpu.pc = 0x5391;
    cpu.a = { let value = (((crate::defs::STEP_WALK as i32) << (2 as i32)) | (crate::defs::UP as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become slide_step(cpu);
}

pub(crate) fn movement_slide_step_left(cpu: &mut Cpu) {
    cpu.pc = 0x5396;
    cpu.a = { let value = (((crate::defs::STEP_WALK as i32) << (2 as i32)) | (crate::defs::LEFT as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become slide_step(cpu);
}

pub(crate) fn movement_slide_step_right(cpu: &mut Cpu) {
    cpu.pc = 0x539b;
    cpu.a = { let value = (((crate::defs::STEP_WALK as i32) << (2 as i32)) | (crate::defs::RIGHT as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become slide_step(cpu);
}

pub(crate) fn movement_fast_slide_step_down(cpu: &mut Cpu) {
    cpu.pc = 0x53a0;
    cpu.a = { let value = (((crate::defs::STEP_BIKE as i32) << (2 as i32)) | (crate::defs::DOWN as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become slide_step(cpu);
}

pub(crate) fn movement_fast_slide_step_up(cpu: &mut Cpu) {
    cpu.pc = 0x53a5;
    cpu.a = { let value = (((crate::defs::STEP_BIKE as i32) << (2 as i32)) | (crate::defs::UP as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become slide_step(cpu);
}

pub(crate) fn movement_fast_slide_step_left(cpu: &mut Cpu) {
    cpu.pc = 0x53aa;
    cpu.a = { let value = (((crate::defs::STEP_BIKE as i32) << (2 as i32)) | (crate::defs::LEFT as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become slide_step(cpu);
}

pub(crate) fn movement_fast_slide_step_right(cpu: &mut Cpu) {
    cpu.pc = 0x53af;
    cpu.a = { let value = (((crate::defs::STEP_BIKE as i32) << (2 as i32)) | (crate::defs::RIGHT as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become slide_step(cpu);
}

pub(crate) fn movement_slow_jump_step_down(cpu: &mut Cpu) {
    cpu.pc = 0x53b4;
    cpu.a = { let value = (((crate::defs::STEP_SLOW as i32) << (2 as i32)) | (crate::defs::DOWN as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become jump_step(cpu);
}

pub(crate) fn movement_slow_jump_step_up(cpu: &mut Cpu) {
    cpu.pc = 0x53b9;
    cpu.a = { let value = (((crate::defs::STEP_SLOW as i32) << (2 as i32)) | (crate::defs::UP as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become jump_step(cpu);
}

pub(crate) fn movement_slow_jump_step_left(cpu: &mut Cpu) {
    cpu.pc = 0x53be;
    cpu.a = { let value = (((crate::defs::STEP_SLOW as i32) << (2 as i32)) | (crate::defs::LEFT as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become jump_step(cpu);
}

pub(crate) fn movement_slow_jump_step_right(cpu: &mut Cpu) {
    cpu.pc = 0x53c3;
    cpu.a = { let value = (((crate::defs::STEP_SLOW as i32) << (2 as i32)) | (crate::defs::RIGHT as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become jump_step(cpu);
}

pub(crate) fn movement_jump_step_down(cpu: &mut Cpu) {
    cpu.pc = 0x53c8;
    cpu.a = { let value = (((crate::defs::STEP_WALK as i32) << (2 as i32)) | (crate::defs::DOWN as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become jump_step(cpu);
}

pub(crate) fn movement_jump_step_up(cpu: &mut Cpu) {
    cpu.pc = 0x53cd;
    cpu.a = { let value = (((crate::defs::STEP_WALK as i32) << (2 as i32)) | (crate::defs::UP as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become jump_step(cpu);
}

pub(crate) fn movement_jump_step_left(cpu: &mut Cpu) {
    cpu.pc = 0x53d2;
    cpu.a = { let value = (((crate::defs::STEP_WALK as i32) << (2 as i32)) | (crate::defs::LEFT as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become jump_step(cpu);
}

pub(crate) fn movement_jump_step_right(cpu: &mut Cpu) {
    cpu.pc = 0x53d7;
    cpu.a = { let value = (((crate::defs::STEP_WALK as i32) << (2 as i32)) | (crate::defs::RIGHT as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become jump_step(cpu);
}

pub(crate) fn movement_fast_jump_step_down(cpu: &mut Cpu) {
    cpu.pc = 0x53dc;
    cpu.a = { let value = (((crate::defs::STEP_BIKE as i32) << (2 as i32)) | (crate::defs::DOWN as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become jump_step(cpu);
}

pub(crate) fn movement_fast_jump_step_up(cpu: &mut Cpu) {
    cpu.pc = 0x53e1;
    cpu.a = { let value = (((crate::defs::STEP_BIKE as i32) << (2 as i32)) | (crate::defs::UP as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become jump_step(cpu);
}

pub(crate) fn movement_fast_jump_step_left(cpu: &mut Cpu) {
    cpu.pc = 0x53e6;
    cpu.a = { let value = (((crate::defs::STEP_BIKE as i32) << (2 as i32)) | (crate::defs::LEFT as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become jump_step(cpu);
}

pub(crate) fn movement_fast_jump_step_right(cpu: &mut Cpu) {
    cpu.pc = 0x53eb;
    cpu.a = { let value = (((crate::defs::STEP_BIKE as i32) << (2 as i32)) | (crate::defs::RIGHT as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become jump_step(cpu);
}

pub(crate) fn movement_turn_step_down(cpu: &mut Cpu) {
    cpu.pc = 0x53f0;
    cpu.a = crate::defs::OW_DOWN as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become turn_step(cpu);
}

pub(crate) fn movement_turn_step_up(cpu: &mut Cpu) {
    cpu.pc = 0x53f4;
    cpu.a = crate::defs::OW_UP as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become turn_step(cpu);
}

pub(crate) fn movement_turn_step_left(cpu: &mut Cpu) {
    cpu.pc = 0x53f8;
    cpu.a = crate::defs::OW_LEFT as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become turn_step(cpu);
}

pub(crate) fn movement_turn_step_right(cpu: &mut Cpu) {
    cpu.pc = 0x53fc;
    cpu.a = crate::defs::OW_RIGHT as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become turn_step(cpu);
}

pub(crate) fn turn_step(cpu: &mut Cpu) {
    cpu.pc = 0x5400;
    cpu.set_hl(crate::defs::OBJECT_1D as u16);
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
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(crate::defs::OBJECT_ACTION as u16);
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
    cpu.write_byte(cpu.hl(), crate::defs::OBJECT_ACTION_STEP as u8);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.set_hl(crate::defs::OBJECT_STEP_TYPE as u16);
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
    cpu.write_byte(cpu.hl(), crate::defs::STEP_TYPE_TURN as u8);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn normal_step(cpu: &mut Cpu) {
    cpu.pc = 0x5412;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::engine::overworld::map_objects::init_step(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::engine::overworld::map_objects::update_tall_grass_flags(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(crate::defs::OBJECT_ACTION as u16);
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
    cpu.write_byte(cpu.hl(), crate::defs::OBJECT_ACTION_STEP as u8);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.set_hl(crate::defs::OBJECT_TILE_COLLISION as u16);
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
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map_objects::check_super_tall_grass_tile(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_z {
        cpu.cycle(12);
        become normal_step_shake_grass(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::map_objects::check_grass_tile(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.cycle(12);
        become normal_step_skip_grass(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become normal_step_shake_grass(cpu);
}

pub(crate) fn normal_step_shake_grass(cpu: &mut Cpu) {
    cpu.pc = 0x542d;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::engine::overworld::map_objects::shake_grass(cpu);
    assert_eq!(cpu.pc, next_pc);
    become normal_step_skip_grass(cpu);
}

pub(crate) fn normal_step_skip_grass(cpu: &mut Cpu) {
    cpu.pc = 0x5430;
    cpu.set_hl(0xd4cf);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xffaf);
    cpu.pc += 2;
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
    if cpu.flag_z {
        cpu.cycle(12);
        become normal_step_player(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(crate::defs::OBJECT_STEP_TYPE as u16);
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
    cpu.write_byte(cpu.hl(), crate::defs::STEP_TYPE_NPC_WALK as u8);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn normal_step_player(cpu: &mut Cpu) {
    cpu.pc = 0x543f;
    cpu.set_hl(crate::defs::OBJECT_STEP_TYPE as u16);
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
    cpu.write_byte(cpu.hl(), crate::defs::STEP_TYPE_PLAYER_WALK as u8);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn turning_step(cpu: &mut Cpu) {
    cpu.pc = 0x5446;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::engine::overworld::map_objects::init_step(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::engine::overworld::map_objects::update_tall_grass_flags(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(crate::defs::OBJECT_ACTION as u16);
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
    cpu.write_byte(cpu.hl(), crate::defs::OBJECT_ACTION_SPIN as u8);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.set_hl(0xd4cf);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xffaf);
    cpu.pc += 2;
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
    if cpu.flag_z {
        cpu.cycle(12);
        become turning_step_player(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(crate::defs::OBJECT_STEP_TYPE as u16);
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
    cpu.write_byte(cpu.hl(), crate::defs::STEP_TYPE_NPC_WALK as u8);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn turning_step_player(cpu: &mut Cpu) {
    cpu.pc = 0x5461;
    cpu.set_hl(crate::defs::OBJECT_STEP_TYPE as u16);
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
    cpu.write_byte(cpu.hl(), crate::defs::STEP_TYPE_PLAYER_WALK as u8);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn slide_step(cpu: &mut Cpu) {
    cpu.pc = 0x5468;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::engine::overworld::map_objects::init_step(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::engine::overworld::map_objects::update_tall_grass_flags(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(crate::defs::OBJECT_ACTION as u16);
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
    cpu.write_byte(cpu.hl(), crate::defs::OBJECT_ACTION_STAND as u8);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.set_hl(0xd4cf);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xffaf);
    cpu.pc += 2;
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
    if cpu.flag_z {
        cpu.cycle(12);
        become slide_step_player(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(crate::defs::OBJECT_STEP_TYPE as u16);
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
    cpu.write_byte(cpu.hl(), crate::defs::STEP_TYPE_NPC_WALK as u8);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn slide_step_player(cpu: &mut Cpu) {
    cpu.pc = 0x5483;
    cpu.set_hl(crate::defs::OBJECT_STEP_TYPE as u16);
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
    cpu.write_byte(cpu.hl(), crate::defs::STEP_TYPE_PLAYER_WALK as u8);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn jump_step(cpu: &mut Cpu) {
    cpu.pc = 0x548a;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::engine::overworld::map_objects::init_step(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(crate::defs::OBJECT_JUMP_HEIGHT as u16);
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
    cpu.write_byte(cpu.hl(), 0x0);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.set_hl(crate::defs::OBJECT_FLAGS2 as u16);
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
    let mask = 1u8 << ((crate::defs::IN_GRASS_F as u8) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.set_hl(crate::defs::OBJECT_ACTION as u16);
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
    cpu.write_byte(cpu.hl(), crate::defs::OBJECT_ACTION_STEP as u8);
    cpu.pc += 2;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::engine::overworld::map_objects::spawn_shadow(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xd4cf);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xffaf);
    cpu.pc += 2;
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
    if cpu.flag_z {
        cpu.cycle(12);
        become jump_step_player(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(crate::defs::OBJECT_STEP_TYPE as u16);
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
    cpu.write_byte(cpu.hl(), crate::defs::STEP_TYPE_NPC_JUMP as u8);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn jump_step_player(cpu: &mut Cpu) {
    cpu.pc = 0x54b1;
    cpu.set_hl(crate::defs::OBJECT_STEP_TYPE as u16);
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
    cpu.write_byte(cpu.hl(), crate::defs::STEP_TYPE_PLAYER_JUMP as u8);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}
