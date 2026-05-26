use crate::runtime::Cpu;

pub(crate) fn init_list(cpu: &mut Cpu) {
    cpu.pc = 0x4db9;
    cpu.a = cpu.read_byte(0xd263);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = crate::defs::INIT_ENEMYOT_LIST as u8;
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
        become init_list_check_party_ot_name(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xd280);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xd3a8);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = crate::defs::ENEMY_OT_NAME as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become init_list_done(cpu);
}

pub(crate) fn init_list_check_party_ot_name(cpu: &mut Cpu) {
    cpu.pc = 0x4dca;
    let value = crate::defs::INIT_PLAYEROT_LIST as u8;
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
        become init_list_check_mon_name(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xdcd7);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0xddff);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = crate::defs::PARTY_OT_NAME as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become init_list_done(cpu);
}

pub(crate) fn init_list_check_mon_name(cpu: &mut Cpu) {
    cpu.pc = 0x4dd8;
    let value = crate::defs::INIT_MON_LIST as u8;
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
        become init_list_check_item_name(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xd0f0);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0x7384);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = crate::defs::MON_NAME as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become init_list_done(cpu);
}

pub(crate) fn init_list_check_item_name(cpu: &mut Cpu) {
    cpu.pc = 0x4de6;
    let value = crate::defs::INIT_BAG_ITEM_LIST as u8;
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
        become init_list_check_ob_item_name(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xd892);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0x4000);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = crate::defs::ITEM_NAME as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become init_list_done(cpu);
}

pub(crate) fn init_list_check_ob_item_name(cpu: &mut Cpu) {
    cpu.pc = 0x4df4;
    cpu.set_hl(0xd0f0);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de(0x4000);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = crate::defs::ITEM_NAME as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    become init_list_done(cpu);
}

pub(crate) fn init_list_done(cpu: &mut Cpu) {
    cpu.pc = 0x4dfc;
    cpu.write_byte(0xcf61, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xd100, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte({ const N: i32 = (0xd100 as i32) + (1 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xd102, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte({ const N: i32 = (0xd102 as i32) + (1 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.set_bc(0x67c1);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xd104, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte({ const N: i32 = (0xd104 as i32) + (1 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}
