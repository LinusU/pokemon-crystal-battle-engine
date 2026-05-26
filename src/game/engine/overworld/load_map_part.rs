use crate::runtime::Cpu;

pub fn _load_overworld_tilemap(cpu: &mut Cpu) {
    cpu.pc = 0x515b;
    cpu.set_hl(0xc608);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xd196);
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
        become _load_overworld_tilemap_top_row(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_bc(0x0030);
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
    become _load_overworld_tilemap_top_row(cpu);
}

pub(crate) fn _load_overworld_tilemap_top_row(cpu: &mut Cpu) {
    cpu.pc = 0x5168;
    cpu.a = cpu.read_byte(0xd197);
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
        become _load_overworld_tilemap_left_column(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    become _load_overworld_tilemap_left_column(cpu);
}

pub(crate) fn _load_overworld_tilemap_left_column(cpu: &mut Cpu) {
    cpu.pc = 0x5170;
    cpu.set_de({ const N: i32 = ((((0 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (0 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.b = crate::defs::SCREEN_HEIGHT as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    become _load_overworld_tilemap_loop(cpu);
}

pub(crate) fn _load_overworld_tilemap_loop(cpu: &mut Cpu) {
    cpu.pc = 0x5175;
    cpu.c = crate::defs::SCREEN_WIDTH as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    become _load_overworld_tilemap_loop2(cpu);
}

pub(crate) fn _load_overworld_tilemap_loop2(cpu: &mut Cpu) {
    cpu.pc = 0x5177;
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
        become _load_overworld_tilemap_loop2(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = crate::defs::METATILE_WIDTH as u8;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_add(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) > 0x0f;
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_c {
        cpu.cycle(12);
        become _load_overworld_tilemap_carry(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.h;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.h = result;
    cpu.pc += 1;
    cpu.cycle(4);
    become _load_overworld_tilemap_carry(cpu);
}

pub(crate) fn _load_overworld_tilemap_carry(cpu: &mut Cpu) {
    cpu.pc = 0x5184;
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
        become _load_overworld_tilemap_loop(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}
