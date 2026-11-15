use crate::runtime::Cpu;

pub(crate) fn ai_switch_or_try_item(cpu: &mut Cpu) {
    cpu.pc = 0x4000;
    // HARNESS-PATCH: an externally controlled trainer carries out the switch
    // or item it picked at AIChooseMove (see the patch there) instead of
    // deciding here. Only trainer battles set `external_enemy`, so the wild /
    // link early returns don't apply; CheckEnemyLockedIn still does. A switch
    // runs AI_Switch with wEnemySwitchMonIndex set as AI_TrySwitch sets it; an
    // item runs its EnemyUsed* routine and AI_TryItem's `.used_item`
    // bookkeeping. See docs/transpiled_code.md.
    if cpu.external_enemy {
        use crate::bootstrap::cli::{ExternalEnemyTurn, enemy_locked_in, external_enemy_switch_or_item};
        let turn = if enemy_locked_in(cpu) { ExternalEnemyTurn::Attack } else { external_enemy_switch_or_item(cpu) };
        match turn {
            ExternalEnemyTurn::Attack => {
                cpu.flag_c = false;
                cpu.pc = cpu.pop_word();
                cpu.cycle(16);
                return;
            }
            ExternalEnemyTurn::Switch(slot) => {
                cpu.write_byte(0xc718, slot + 1); // wEnemySwitchMonIndex
                become ai_switch(cpu);
            }
            ExternalEnemyTurn::Item(id) => {
                let routine: fn(&mut Cpu) = match id {
                    crate::defs::FULL_RESTORE => enemy_used_full_restore,
                    crate::defs::MAX_POTION => enemy_used_max_potion,
                    crate::defs::HYPER_POTION => enemy_used_hyper_potion,
                    crate::defs::SUPER_POTION => enemy_used_super_potion,
                    crate::defs::POTION => enemy_used_potion,
                    crate::defs::FULL_HEAL => enemy_used_full_heal,
                    crate::defs::X_ACCURACY => enemy_used_x_accuracy,
                    crate::defs::GUARD_SPEC => enemy_used_guard_spec,
                    crate::defs::DIRE_HIT => enemy_used_dire_hit,
                    crate::defs::X_ATTACK => enemy_used_x_attack,
                    crate::defs::X_DEFEND => enemy_used_x_defend,
                    crate::defs::X_SPEED => enemy_used_x_speed,
                    crate::defs::X_SPECIAL => enemy_used_x_special,
                    _ => unreachable!("not an AI item: {id}"),
                };
                let next_pc = cpu.pc + 1;
                cpu.push_word(next_pc);
                routine(cpu);
                assert_eq!(cpu.pc, next_pc);
                crate::bootstrap::cli::external_enemy_item_used(cpu, id);
                cpu.flag_c = true;
                cpu.pc = cpu.pop_word();
                cpu.cycle(16);
                return;
            }
        }
    }
    let value = cpu.a;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
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
    if cpu.flag_z {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
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
    cpu.a = 0x0f;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x68d1);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    if !cpu.flag_z {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc66c);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((crate::defs::SUBSTATUS_CANT_RUN as u8) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(12);
        become dont_switch(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc731);
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
        become dont_switch(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl({ const N: i32 = (0x559c as i32) + (crate::defs::TRNATTR_AI_ITEM_SWITCH as u16 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xcfc0);
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
        become ai_switch_or_try_item_ok(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xd233);
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
    cpu.set_bc(crate::defs::NUM_TRAINER_ATTRIBUTES as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::array::add_n_times(cpu);
    assert_eq!(cpu.pc, next_pc);
    become ai_switch_or_try_item_ok(cpu);
}

pub(crate) fn ai_switch_or_try_item_ok(cpu: &mut Cpu) {
    cpu.pc = 0x4032;
    let mask = 1u8 << ((crate::defs::SWITCH_OFTEN_F as u8) & 7);
    let value = cpu.read_byte(cpu.hl());
    cpu.flag_z = (value & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(12);
    if !cpu.flag_z {
        cpu.cycle(16);
        become switch_often(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((crate::defs::SWITCH_RARELY_F as u8) & 7);
    let value = cpu.read_byte(cpu.hl());
    cpu.flag_z = (value & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(12);
    if !cpu.flag_z {
        cpu.cycle(16);
        become switch_rarely(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((crate::defs::SWITCH_SOMETIMES_F as u8) & 7);
    let value = cpu.read_byte(cpu.hl());
    cpu.flag_z = (value & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(12);
    if !cpu.flag_z {
        cpu.cycle(16);
        become switch_sometimes(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    become dont_switch(cpu);
}

pub(crate) fn dont_switch(cpu: &mut Cpu) {
    cpu.pc = 0x4041;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    ai_try_item(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn switch_often(cpu: &mut Cpu) {
    cpu.pc = 0x4045;
    cpu.set_hl(0x4941);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0x0d;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xc717);
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
    if cpu.flag_z {
        cpu.cycle(16);
        become dont_switch(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    let value = 0x10;
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
        become switch_often_not_10(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::random::random(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = { let value = ((((50 as i32) * (0xff as i32)) / (100 as i32)) + (1 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
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
        become switch_often_switch(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become dont_switch(cpu);
}

pub(crate) fn switch_often_not_10(cpu: &mut Cpu) {
    cpu.pc = 0x4061;
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
        become switch_often_not_20(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::random::random(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = { let value = ((((79 as i32) * (0xff as i32)) / (100 as i32)) - (1 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
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
        become switch_often_switch(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become dont_switch(cpu);
}

pub(crate) fn switch_often_not_20(cpu: &mut Cpu) {
    cpu.pc = 0x406f;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::random::random(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = { let value = (((4 as i32) * (0xff as i32)) / (100 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
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
        become dont_switch(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    become switch_often_switch(cpu);
}

pub(crate) fn switch_often_switch(cpu: &mut Cpu) {
    cpu.pc = 0x4077;
    cpu.a = cpu.read_byte(0xc717);
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
    let value = cpu.a;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc718, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(16);
    become ai_try_switch(cpu);
}

pub(crate) fn switch_rarely(cpu: &mut Cpu) {
    cpu.pc = 0x4083;
    cpu.set_hl(0x4941);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0x0d;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xc717);
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
    if cpu.flag_z {
        cpu.cycle(16);
        become dont_switch(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    let value = 0x10;
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
        become switch_rarely_not_10(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::random::random(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = { let value = (((8 as i32) * (0xff as i32)) / (100 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
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
        become switch_rarely_switch(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become dont_switch(cpu);
}

pub(crate) fn switch_rarely_not_10(cpu: &mut Cpu) {
    cpu.pc = 0x409f;
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
        become switch_rarely_not_20(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::random::random(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = { let value = (((12 as i32) * (0xff as i32)) / (100 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
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
        become switch_rarely_switch(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become dont_switch(cpu);
}

pub(crate) fn switch_rarely_not_20(cpu: &mut Cpu) {
    cpu.pc = 0x40ad;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::random::random(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = { let value = ((((79 as i32) * (0xff as i32)) / (100 as i32)) - (1 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
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
        become dont_switch(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    become switch_rarely_switch(cpu);
}

pub(crate) fn switch_rarely_switch(cpu: &mut Cpu) {
    cpu.pc = 0x40b5;
    cpu.a = cpu.read_byte(0xc717);
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
    let value = cpu.a;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc718, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(16);
    become ai_try_switch(cpu);
}

pub(crate) fn switch_sometimes(cpu: &mut Cpu) {
    cpu.pc = 0x40c1;
    cpu.set_hl(0x4941);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0x0d;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xc717);
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
    if cpu.flag_z {
        cpu.cycle(16);
        become dont_switch(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    let value = 0x10;
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
        become switch_sometimes_not_10(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::random::random(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = { let value = ((((20 as i32) * (0xff as i32)) / (100 as i32)) - (1 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
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
        become switch_sometimes_switch(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become dont_switch(cpu);
}

pub(crate) fn switch_sometimes_not_10(cpu: &mut Cpu) {
    cpu.pc = 0x40dd;
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
        become switch_sometimes_not_20(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::random::random(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = { let value = ((((50 as i32) * (0xff as i32)) / (100 as i32)) + (1 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
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
        become switch_sometimes_switch(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become dont_switch(cpu);
}

pub(crate) fn switch_sometimes_not_20(cpu: &mut Cpu) {
    cpu.pc = 0x40eb;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::random::random(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = { let value = ((((20 as i32) * (0xff as i32)) / (100 as i32)) - (1 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
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
        become dont_switch(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    become switch_sometimes_switch(cpu);
}

pub(crate) fn switch_sometimes_switch(cpu: &mut Cpu) {
    cpu.pc = 0x40f3;
    cpu.a = cpu.read_byte(0xc717);
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
    let value = cpu.a;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc718, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(16);
    become ai_try_switch(cpu);
}

pub(crate) fn check_substatus_cant_run(cpu: &mut Cpu) {
    cpu.pc = 0x40ff;
    cpu.a = cpu.read_byte(0xc671);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((crate::defs::SUBSTATUS_CANT_RUN as u8) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn ai_try_item(cpu: &mut Cpu) {
    cpu.pc = 0x4105;
    cpu.a = cpu.read_byte(0xcfc0);
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
    cpu.a = cpu.read_byte(0xc650);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xc651);
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
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    ai_try_item_is_highest_level(cpu);
    assert_eq!(cpu.pc, next_pc);
    if !cpu.flag_c {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xd233);
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
    cpu.set_hl({ const N: i32 = (0x559c as i32) + (crate::defs::TRNATTR_AI_ITEM_SWITCH as u16 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(crate::defs::NUM_TRAINER_ATTRIBUTES as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::array::add_n_times(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.b = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.c = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl(0x4196);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xc650);
    cpu.pc += 3;
    cpu.cycle(12);
    become ai_try_item_loop(cpu);
}

pub(crate) fn ai_try_item_loop(cpu: &mut Cpu) {
    cpu.pc = 0x412c;
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
    let value = cpu.a;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
    cpu.cycle(8);
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
    cpu.pc += 1;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become ai_try_item_has_item(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
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
    cpu.pc += 1;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become ai_try_item_has_item(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_sub(1));
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
    cpu.cycle(12);
    become ai_try_item_loop(cpu);
}

pub(crate) fn ai_try_item_callback(cpu: &mut Cpu) {
    cpu.pc = 0x414a;
    let value = cpu.pop_word();
    cpu.set_de(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    if cpu.flag_c {
        cpu.cycle(12);
        become ai_try_item_loop(cpu);
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
    let value = cpu.a;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xc70f, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xc66f);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((crate::defs::SUBSTATUS_BIDE as u8) & 7);
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
    cpu.write_byte(0xc680, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xc681, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xc72c, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xc670);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((crate::defs::SUBSTATUS_RAGE as u8) & 7);
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
    cpu.write_byte(0xc6f9, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = true;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn ai_try_item_is_highest_level(cpu: &mut Cpu) {
    cpu.pc = 0x4170;
    cpu.a = cpu.read_byte(0xd280);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.d = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.e = 0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xd2a7);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc(crate::defs::PARTYMON_STRUCT_LENGTH as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    become ai_try_item_next(cpu);
}

pub(crate) fn ai_try_item_next(cpu: &mut Cpu) {
    cpu.pc = 0x417c;
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
        become ai_try_item_ok(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    become ai_try_item_ok(cpu);
}

pub(crate) fn ai_try_item_ok(cpu: &mut Cpu) {
    cpu.pc = 0x4181;
    let lhs = cpu.hl();
    let rhs = cpu.bc();
    let (result, carry) = lhs.overflowing_add(rhs);
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0fff) + (rhs & 0x0fff) > 0x0fff;
    cpu.flag_c = carry;
    cpu.set_hl(result);
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
        become ai_try_item_next(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc663);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xd2a7);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::array::add_n_times(cpu);
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
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_c {
        cpu.cycle(12);
        become ai_try_item_yes(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become ai_try_item_no(cpu);
}

pub(crate) fn ai_try_item_no(cpu: &mut Cpu) {
    cpu.pc = 0x4192;
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

pub(crate) fn ai_try_item_yes(cpu: &mut Cpu) {
    cpu.pc = 0x4194;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = true;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn ai_items_full_heal(cpu: &mut Cpu) {
    cpu.pc = 0x41be;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    ai_items_status(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.cycle(16);
        become ai_items_dont_use_2(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    enemy_used_full_heal(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(16);
    become ai_items_use(cpu);
}

pub(crate) fn ai_items_status(cpu: &mut Cpu) {
    cpu.pc = 0x41ca;
    cpu.a = cpu.read_byte(0xd214);
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
        cpu.cycle(16);
        become ai_items_dont_use_2(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(8);
    let mask = 1u8 << ((crate::defs::CONTEXT_USE_F as u8) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(12);
        become ai_items_status_check_context(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(8);
    let mask = 1u8 << ((crate::defs::ALWAYS_USE_F as u8) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(16);
        become ai_items_use(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::random::random(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = { let value = ((((20 as i32) * (0xff as i32)) / (100 as i32)) - (1 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
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
        become ai_items_use(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become ai_items_dont_use_2(cpu);
}

pub(crate) fn ai_items_status_check_context(cpu: &mut Cpu) {
    cpu.pc = 0x41e7;
    cpu.a = cpu.read_byte(0xc671);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((crate::defs::SUBSTATUS_TOXIC as u8) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become ai_items_fail_toxic_check(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc67c);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = 4;
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
        become ai_items_fail_toxic_check(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::random::random(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = { let value = ((((50 as i32) * (0xff as i32)) / (100 as i32)) + (1 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
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
        become ai_items_use(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    become ai_items_fail_toxic_check(cpu);
}

pub(crate) fn ai_items_fail_toxic_check(cpu: &mut Cpu) {
    cpu.pc = 0x41fd;
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
    if cpu.flag_z {
        cpu.cycle(16);
        become ai_items_dont_use_2(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become ai_items_use(cpu);
}

pub(crate) fn ai_items_full_restore(cpu: &mut Cpu) {
    cpu.pc = 0x4208;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    ai_items_heal_item(cpu);
    assert_eq!(cpu.pc, next_pc);
    if !cpu.flag_c {
        cpu.cycle(16);
        become ai_items_use_full_restore(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(8);
    let mask = 1u8 << ((crate::defs::CONTEXT_USE_F as u8) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(16);
        become ai_items_dont_use_2(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    ai_items_status(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.cycle(16);
        become ai_items_dont_use_2(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    become ai_items_use_full_restore(cpu);
}

pub(crate) fn ai_items_use_full_restore(cpu: &mut Cpu) {
    cpu.pc = 0x421a;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    enemy_used_full_restore(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(16);
    become ai_items_use(cpu);
}

pub(crate) fn ai_items_max_potion(cpu: &mut Cpu) {
    cpu.pc = 0x4220;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    ai_items_heal_item(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.cycle(16);
        become ai_items_dont_use_2(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    enemy_used_max_potion(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(16);
    become ai_items_use(cpu);
}

pub(crate) fn ai_items_heal_item(cpu: &mut Cpu) {
    cpu.pc = 0x422c;
    cpu.a = cpu.read_byte(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(8);
    let mask = 1u8 << ((crate::defs::CONTEXT_USE_F as u8) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(12);
        become ai_items_check_half_or_quarter_hp(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x5281);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0x0e;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.cycle(16);
        become ai_items_dont_use_2(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(8);
    let mask = 1u8 << ((crate::defs::UNKNOWN_USE_F as u8) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(16);
        become ai_items_check_quarter_hp(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0x5298);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0x0e;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    if !cpu.flag_c {
        cpu.cycle(16);
        become ai_items_use_heal_item(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::random::random(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = { let value = ((((50 as i32) * (0xff as i32)) / (100 as i32)) + (1 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
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
        become ai_items_use_heal_item(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become ai_items_dont_use_2(cpu);
}

pub(crate) fn ai_items_check_quarter_hp(cpu: &mut Cpu) {
    cpu.pc = 0x4254;
    cpu.set_hl(0x5298);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0x0e;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.cycle(16);
        become ai_items_dont_use_2(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::random::random(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = { let value = ((((20 as i32) * (0xff as i32)) / (100 as i32)) - (1 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
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
        become ai_items_dont_use_2(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(12);
    become ai_items_use_heal_item(cpu);
}

pub(crate) fn ai_items_check_half_or_quarter_hp(cpu: &mut Cpu) {
    cpu.pc = 0x4267;
    cpu.set_hl(0x5281);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0x0e;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.cycle(16);
        become ai_items_dont_use_2(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0x5298);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0x0e;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    if !cpu.flag_c {
        cpu.cycle(16);
        become ai_items_use_heal_item(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::random::random(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = { let value = ((((20 as i32) * (0xff as i32)) / (100 as i32)) - (1 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
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
        become ai_items_dont_use_2(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    become ai_items_use_heal_item(cpu);
}

pub(crate) fn ai_items_use_heal_item(cpu: &mut Cpu) {
    cpu.pc = 0x4281;
    cpu.cycle(16);
    become ai_items_use(cpu);
}

pub(crate) fn ai_items_hyper_potion(cpu: &mut Cpu) {
    cpu.pc = 0x4284;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    ai_items_heal_item(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.cycle(16);
        become ai_items_dont_use_2(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = 200;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    enemy_used_hyper_potion(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(16);
    become ai_items_use(cpu);
}

pub(crate) fn ai_items_super_potion(cpu: &mut Cpu) {
    cpu.pc = 0x4292;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    ai_items_heal_item(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.cycle(16);
        become ai_items_dont_use_2(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = 50;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    enemy_used_super_potion(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(16);
    become ai_items_use(cpu);
}

pub(crate) fn ai_items_potion(cpu: &mut Cpu) {
    cpu.pc = 0x42a0;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    ai_items_heal_item(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.cycle(16);
        become ai_items_dont_use_2(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = 20;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    enemy_used_potion(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(16);
    become ai_items_use(cpu);
}

pub(crate) fn ai_items_unused_heal_item(cpu: &mut Cpu) {
    cpu.pc = 0x42ae;
    cpu.set_hl(0x5251);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0x0e;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.cycle(12);
        become ai_items_dont_use(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.push_word(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.set_de({ const N: i32 = (0xd218 as i32) + (1 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl({ const N: i32 = (0xd216 as i32) + (1 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
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
    if cpu.flag_z {
        cpu.cycle(12);
        become ai_items_check_40_percent(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
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
        become ai_items_check_40_percent(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.c;
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
        become ai_items_check_50_percent(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0x5298);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0x0e;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.cycle(12);
        become ai_items_check_40_percent(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become ai_items_check_50_percent(cpu);
}

pub(crate) fn ai_items_check_50_percent(cpu: &mut Cpu) {
    cpu.pc = 0x42d5;
    let value = cpu.pop_word();
    cpu.set_bc(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(8);
    let mask = 1u8 << ((crate::defs::UNKNOWN_USE_F as u8) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(16);
        become ai_items_use(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::random::random(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = { let value = ((((50 as i32) * (0xff as i32)) / (100 as i32)) + (1 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
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
        become ai_items_use(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    become ai_items_dont_use(cpu);
}

pub(crate) fn ai_items_dont_use(cpu: &mut Cpu) {
    cpu.pc = 0x42e4;
    cpu.cycle(16);
    become ai_items_dont_use_2(cpu);
}

pub(crate) fn ai_items_check_40_percent(cpu: &mut Cpu) {
    cpu.pc = 0x42e7;
    let value = cpu.pop_word();
    cpu.set_bc(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(8);
    let mask = 1u8 << ((crate::defs::UNKNOWN_USE_F as u8) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(16);
        become ai_items_dont_use_2(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::random::random(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = { let value = ((((39 as i32) * (0xff as i32)) / (100 as i32)) + (1 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
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
        become ai_items_use(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become ai_items_dont_use_2(cpu);
}

pub(crate) fn ai_items_x_accuracy(cpu: &mut Cpu) {
    cpu.pc = 0x42f9;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    ai_items_x_item(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.cycle(16);
        become ai_items_dont_use_2(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    enemy_used_x_accuracy(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(16);
    become ai_items_use(cpu);
}

pub(crate) fn ai_items_guard_spec(cpu: &mut Cpu) {
    cpu.pc = 0x4305;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    ai_items_x_item(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.cycle(16);
        become ai_items_dont_use_2(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    enemy_used_guard_spec(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(16);
    become ai_items_use(cpu);
}

pub(crate) fn ai_items_dire_hit(cpu: &mut Cpu) {
    cpu.pc = 0x4311;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    ai_items_x_item(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.cycle(16);
        become ai_items_dont_use_2(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    enemy_used_dire_hit(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(16);
    become ai_items_use(cpu);
}

pub(crate) fn ai_items_x_attack(cpu: &mut Cpu) {
    cpu.pc = 0x431d;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    ai_items_x_item(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.cycle(16);
        become ai_items_dont_use_2(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    enemy_used_x_attack(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(16);
    become ai_items_use(cpu);
}

pub(crate) fn ai_items_x_defend(cpu: &mut Cpu) {
    cpu.pc = 0x4329;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    ai_items_x_item(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.cycle(16);
        become ai_items_dont_use_2(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    enemy_used_x_defend(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(16);
    become ai_items_use(cpu);
}

pub(crate) fn ai_items_x_speed(cpu: &mut Cpu) {
    cpu.pc = 0x4335;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    ai_items_x_item(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.cycle(16);
        become ai_items_dont_use_2(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    enemy_used_x_speed(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(16);
    become ai_items_use(cpu);
}

pub(crate) fn ai_items_x_special(cpu: &mut Cpu) {
    cpu.pc = 0x4341;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    ai_items_x_item(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.cycle(16);
        become ai_items_dont_use_2(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    enemy_used_x_special(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(16);
    become ai_items_use(cpu);
}

pub(crate) fn ai_items_x_item(cpu: &mut Cpu) {
    cpu.pc = 0x434d;
    cpu.a = cpu.read_byte(0xc6dc);
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
        become ai_items_notfirstturnout(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(8);
    let mask = 1u8 << ((crate::defs::ALWAYS_USE_F as u8) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(16);
        become ai_items_use(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::random::random(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = { let value = ((((50 as i32) * (0xff as i32)) / (100 as i32)) + (1 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
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
        become ai_items_dont_use_2(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(8);
    let mask = 1u8 << ((crate::defs::CONTEXT_USE_F as u8) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(16);
        become ai_items_use(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::random::random(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = { let value = ((((50 as i32) * (0xff as i32)) / (100 as i32)) + (1 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
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
        become ai_items_dont_use_2(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become ai_items_use(cpu);
}

pub(crate) fn ai_items_notfirstturnout(cpu: &mut Cpu) {
    cpu.pc = 0x4372;
    cpu.a = cpu.read_byte(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(8);
    let mask = 1u8 << ((crate::defs::ALWAYS_USE_F as u8) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(16);
        become ai_items_dont_use_2(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::random::random(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = { let value = ((((20 as i32) * (0xff as i32)) / (100 as i32)) - (1 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
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
        become ai_items_dont_use_2(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become ai_items_use(cpu);
}

pub(crate) fn ai_items_dont_use_2(cpu: &mut Cpu) {
    cpu.pc = 0x4383;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = true;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn ai_items_use(cpu: &mut Cpu) {
    cpu.pc = 0x4385;
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

pub(crate) fn ai_update_hud(cpu: &mut Cpu) {
    cpu.pc = 0x4387;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::battle::update_enemy_mon_in_party(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x0f;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x6036);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xffd4, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.set_hl(0xc6e6);
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
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = true;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn ai_used_item_sound(cpu: &mut Cpu) {
    cpu.pc = 0x439a;
    cpu.push_word(cpu.de());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.set_de(crate::defs::SFX_FULL_HEAL as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::audio::play_sfx(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_de(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn enemy_used_full_heal(cpu: &mut Cpu) {
    cpu.pc = 0x43a3;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    ai_used_item_sound(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    ai_heal_status(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = crate::defs::FULL_HEAL as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become print_text_used_item_on_and_ai_update_hud(cpu);
}

pub(crate) fn enemy_used_max_potion(cpu: &mut Cpu) {
    cpu.pc = 0x43ae;
    cpu.a = crate::defs::MAX_POTION as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xd1f1, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(12);
    become full_restore_continue(cpu);
}

pub(crate) fn enemy_used_full_restore(cpu: &mut Cpu) {
    cpu.pc = 0x43b5;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    ai_heal_status(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = crate::defs::FULL_RESTORE as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xd1f1, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xc66f);
    cpu.pc += 3;
    cpu.cycle(12);
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
    cpu.write_byte(0xc67b, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    become full_restore_continue(cpu);
}

pub(crate) fn full_restore_continue(cpu: &mut Cpu) {
    cpu.pc = 0x43c6;
    cpu.set_de(0xd1ec);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl({ const N: i32 = (0xd216 as i32) + (1 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_sub(1));
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
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl({ const N: i32 = (0xd218 as i32) + (1 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(0xd1ea, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte({ const N: i32 = (0xd216 as i32) + (1 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte({ const N: i32 = (0xd1ea as i32) + (1 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xd216, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.cycle(12);
    become enemy_potion_finish(cpu);
}

pub(crate) fn enemy_used_potion(cpu: &mut Cpu) {
    cpu.pc = 0x43e8;
    cpu.a = crate::defs::POTION as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.b = 20;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become enemy_potion_continue(cpu);
}

pub(crate) fn enemy_used_super_potion(cpu: &mut Cpu) {
    cpu.pc = 0x43ee;
    cpu.a = crate::defs::SUPER_POTION as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.b = 50;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become enemy_potion_continue(cpu);
}

pub(crate) fn enemy_used_hyper_potion(cpu: &mut Cpu) {
    cpu.pc = 0x43f4;
    cpu.a = crate::defs::HYPER_POTION as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.b = 200;
    cpu.pc += 2;
    cpu.cycle(8);
    become enemy_potion_continue(cpu);
}

pub(crate) fn enemy_potion_continue(cpu: &mut Cpu) {
    cpu.pc = 0x43f8;
    cpu.write_byte(0xd1f1, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl({ const N: i32 = (0xd216 as i32) + (1 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(0xd1ec, cpu.a);
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
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(0xd1ee, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte({ const N: i32 = (0xd1ec as i32) + (1 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte({ const N: i32 = (0xd1ee as i32) + (1 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    if !cpu.flag_c {
        cpu.cycle(12);
        become enemy_potion_continue_ok(cpu);
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
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte({ const N: i32 = (0xd1ee as i32) + (1 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    become enemy_potion_continue_ok(cpu);
}

pub(crate) fn enemy_potion_continue_ok(cpu: &mut Cpu) {
    cpu.pc = 0x4415;
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_de({ const N: i32 = (0xd218 as i32) + (1 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(0xd1ea, cpu.a);
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
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte({ const N: i32 = (0xd1ea as i32) + (1 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
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
    if !cpu.flag_c {
        cpu.cycle(12);
        become enemy_potion_finish(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(0xd1ee, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte({ const N: i32 = (0xd1ee as i32) + (1 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    become enemy_potion_finish(cpu);
}

pub(crate) fn enemy_potion_finish(cpu: &mut Cpu) {
    cpu.pc = 0x4436;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    print_text_used_item_on(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = ((((2 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (2 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xd10a, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    ai_used_item_sound(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = { let value = (((0x458c as i32) - (0x456b as i32)) / (3 as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::predef::predef(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(16);
    become ai_update_hud(cpu);
}

pub(crate) fn ai_try_switch(cpu: &mut Cpu) {
    cpu.pc = 0x444b;
    cpu.a = cpu.read_byte(0xd280);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl(0xd2aa);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.d = 0;
    cpu.pc += 2;
    cpu.cycle(8);
    become ai_try_switch_switch_loop(cpu);
}

pub(crate) fn ai_try_switch_switch_loop(cpu: &mut Cpu) {
    cpu.pc = 0x4454;
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.b;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if cpu.flag_z {
        cpu.cycle(12);
        become ai_try_switch_fainted(cpu);
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
    become ai_try_switch_fainted(cpu);
}

pub(crate) fn ai_try_switch_fainted(cpu: &mut Cpu) {
    cpu.pc = 0x445b;
    cpu.push_word(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.set_bc(crate::defs::PARTYMON_STRUCT_LENGTH as u16);
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
        become ai_try_switch_switch_loop(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.d;
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
        cpu.cycle(16);
        become ai_switch(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
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

pub(crate) fn ai_switch(cpu: &mut Cpu) {
    cpu.pc = 0x446c;
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xc711, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xc70f, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xc670);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((crate::defs::SUBSTATUS_RAGE as u8) & 7);
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
    cpu.set_hl(0x5c5b);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0x0f;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xc663);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xd2a8);
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
    cpu.d = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.e = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl(0xd214);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_bc({ const N: i32 = (crate::defs::MON_MAXHP as u16 as i32) - (crate::defs::MON_STATUS as u16 as i32); N as u16 });
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
    if cpu.flag_c {
        cpu.cycle(12);
        become ai_switch_skiptext(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x44d0);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::text::print_text(cpu);
    assert_eq!(cpu.pc, next_pc);
    become ai_switch_skiptext(cpu);
}

pub(crate) fn ai_switch_skiptext(cpu: &mut Cpu) {
    cpu.pc = 0x44a3;
    cpu.a = 1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xd264, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0x5834);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0x0f;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0x5867);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = 0x0f;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xc668);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((crate::defs::SUBSTATUS_IN_LOVE as u8) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.a = 0x0f;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x54e1);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x0f;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x557a);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xd264, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
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
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = true;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn enemy_used_full_heal_red(cpu: &mut Cpu) {
    cpu.pc = 0x44d5;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    ai_used_item_sound(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    ai_heal_status(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = crate::defs::FULL_HEAL_RED as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become print_text_used_item_on_and_ai_update_hud(cpu);
}

pub(crate) fn ai_heal_status(cpu: &mut Cpu) {
    cpu.pc = 0x44e0;
    cpu.a = cpu.read_byte(0xc663);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xd2a8);
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
    cpu.write_byte(0xd214, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_hl(0xc671);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((crate::defs::SUBSTATUS_TOXIC as u8) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn enemy_used_x_accuracy(cpu: &mut Cpu) {
    cpu.pc = 0x44f7;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    ai_used_item_sound(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xc670);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((crate::defs::SUBSTATUS_X_ACCURACY as u8) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value |= mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.a = crate::defs::X_ACCURACY as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become print_text_used_item_on_and_ai_update_hud(cpu);
}

pub(crate) fn enemy_used_guard_spec(cpu: &mut Cpu) {
    cpu.pc = 0x4504;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    ai_used_item_sound(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xc670);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((crate::defs::SUBSTATUS_MIST as u8) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value |= mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.a = crate::defs::GUARD_SPEC as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become print_text_used_item_on_and_ai_update_hud(cpu);
}

pub(crate) fn enemy_used_dire_hit(cpu: &mut Cpu) {
    cpu.pc = 0x4511;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    ai_used_item_sound(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl(0xc670);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((crate::defs::SUBSTATUS_FOCUS_ENERGY as u8) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value |= mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.a = crate::defs::DIRE_HIT as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(16);
    become print_text_used_item_on_and_ai_update_hud(cpu);
}

pub(crate) fn ai_check_enemy_fraction_max_hp(cpu: &mut Cpu) {
    cpu.pc = 0x451e;
    cpu.write_byte(0xffb7, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.set_hl(0xd218);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(0xffb3, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte({ const N: i32 = (0xffb3 as i32) + (1 as i32); N as u16 }, cpu.a);
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
    cpu.a = cpu.read_byte({ const N: i32 = (0xffb3 as i32) + (3 as i32); N as u16 });
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte({ const N: i32 = (0xffb3 as i32) + (2 as i32); N as u16 });
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl({ const N: i32 = (0xd216 as i32) + (1 as i32); N as u16 });
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
    cpu.d = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.d;
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
    if !cpu.flag_z {
        cpu.pc = cpu.pop_word();
        cpu.cycle(20);
        return;
    }
    cpu.pc += 1;
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
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn enemy_used_x_attack(cpu: &mut Cpu) {
    cpu.pc = 0x4541;
    cpu.b = crate::defs::ATTACK as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = crate::defs::X_ATTACK as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become enemy_used_x_item(cpu);
}

pub(crate) fn enemy_used_x_defend(cpu: &mut Cpu) {
    cpu.pc = 0x4547;
    cpu.b = crate::defs::DEFENSE as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = crate::defs::X_DEFEND as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become enemy_used_x_item(cpu);
}

pub(crate) fn enemy_used_x_speed(cpu: &mut Cpu) {
    cpu.pc = 0x454d;
    cpu.b = crate::defs::SPEED as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = crate::defs::X_SPEED as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become enemy_used_x_item(cpu);
}

pub(crate) fn enemy_used_x_special(cpu: &mut Cpu) {
    cpu.pc = 0x4553;
    cpu.b = crate::defs::SP_ATTACK as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = crate::defs::X_SPECIAL as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    become enemy_used_x_item(cpu);
}

pub(crate) fn enemy_used_x_item(cpu: &mut Cpu) {
    cpu.pc = 0x4557;
    cpu.write_byte(0xd1f1, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.push_word(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    print_text_used_item_on(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_bc(value);
    cpu.pc += 1;
    cpu.cycle(12);
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
    cpu.cycle(16);
    become ai_update_hud(cpu);
}

pub(crate) fn print_text_used_item_on_and_ai_update_hud(cpu: &mut Cpu) {
    cpu.pc = 0x4568;
    cpu.write_byte(0xd1f1, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    print_text_used_item_on(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(16);
    become ai_update_hud(cpu);
}

pub(crate) fn print_text_used_item_on(cpu: &mut Cpu) {
    cpu.pc = 0x4571;
    cpu.a = cpu.read_byte(0xd1f1);
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
    cpu.set_hl(0xd073);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xd050);
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
    cpu.set_hl(0x458c);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become crate::game::home::text::print_text(cpu);
}


pub(crate) fn ai_try_item_has_item(cpu: &mut Cpu) {
    cpu.set_hl(cpu.hl() + 1);

    cpu.push_word(cpu.hl());
    cpu.push_word(cpu.de());

    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.h = cpu.read_byte(cpu.hl());
    cpu.l = cpu.a;

    cpu.set_de(0x414a);
    cpu.push_word(0x414a);

    match cpu.hl() {
        0x41be => ai_items_full_heal(cpu),
        0x4208 => ai_items_full_restore(cpu),
        0x4220 => ai_items_max_potion(cpu),
        0x4284 => ai_items_hyper_potion(cpu),
        0x4292 => ai_items_super_potion(cpu),
        0x42a0 => ai_items_potion(cpu),
        0x42f9 => ai_items_x_accuracy(cpu),
        0x4305 => ai_items_guard_spec(cpu),
        0x4311 => ai_items_dire_hit(cpu),
        0x431d => ai_items_x_attack(cpu),
        0x4329 => ai_items_x_defend(cpu),
        0x4335 => ai_items_x_speed(cpu),
        0x4341 => ai_items_x_special(cpu),
        n => panic!("Unknown item addr: {:04x}", n),
    }

    assert_eq!(cpu.pc, 0x414a);

    become ai_try_item_callback(cpu);
}
