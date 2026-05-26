use crate::runtime::Cpu;

// HARNESS-PATCH: replace DIV-driven entropy with a seedable xorshift64 stream
// (`Cpu::next_rng_byte`). The original routine xors `hRandomAdd`/`hRandomSub`
// (0xffe1/0xffe2) with successive reads of `rDIV` (0xff04). The same stir is
// also done from the VBlank handler every frame (see vblank.rs), so by the
// time a caller invokes Random the base values have already been perturbed
// by however long the frame loop spent waiting for keypresses. We overwrite
// both bytes wholesale with fresh PRNG output — no read-modify-write — so
// the values callers see don't depend on prior contents at all. PC/cycle
// bookkeeping and `bc` push/pop are kept to match the original; flags after
// return mirror the original `sbc` semantics on the new value.
// `_battle_random` falls through here whenever `wLinkMode == 0`, which is
// always true in the harness. Reseed via `Cpu::seed_rng`. See
// docs/transpiled_code.md.
pub fn random(cpu: &mut Cpu) {
    cpu.pc = 0x2f8c;
    cpu.push_word(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);

    let add_byte = cpu.next_rng_byte();
    cpu.write_byte(0xffe1, add_byte);
    cpu.b = add_byte;
    cpu.flag_z = add_byte == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.a = add_byte;
    cpu.cycle(12 + 4 + 12 + 4 + 12);
    cpu.pc += 2 + 1 + 2 + 1 + 2;

    let sub_byte = cpu.next_rng_byte();
    cpu.write_byte(0xffe2, sub_byte);
    cpu.b = sub_byte;
    cpu.flag_z = sub_byte == 0;
    cpu.flag_n = true;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.a = sub_byte;
    cpu.cycle(12 + 4 + 12 + 4 + 12);
    cpu.pc += 2 + 1 + 2 + 1 + 2;

    let value = cpu.pop_word();
    cpu.set_bc(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub fn battle_random(cpu: &mut Cpu) {
    cpu.pc = 0x2f9f;
    cpu.a = cpu.read_byte(0xff9d);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = 0x0f;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::bankswitch(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::engine::battle::core::_battle_random(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.write_byte({ const N: i32 = (0xcfb5 as i32) + (1 as i32); N as u16 }, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::bankswitch(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte({ const N: i32 = (0xcfb5 as i32) + (1 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub fn random_range(cpu: &mut Cpu) {
    cpu.pc = 0x2fb1;
    cpu.push_word(cpu.bc());
    cpu.pc += 1;
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
    become random_range_mod(cpu);
}

pub(crate) fn random_range_mod(cpu: &mut Cpu) {
    cpu.pc = 0x2fb5;
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
    if !cpu.flag_c {
        cpu.cycle(12);
        become random_range_mod(cpu);
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
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.push_word(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);
    become random_range_loop(cpu);
}

pub(crate) fn random_range_loop(cpu: &mut Cpu) {
    cpu.pc = 0x2fbb;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    random(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xffe1);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.c = cpu.a;
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
    if cpu.flag_c {
        cpu.cycle(12);
        become random_range_loop(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.pop_word();
    cpu.set_bc(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::math::simple_divide(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_bc(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}
