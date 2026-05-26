use crate::runtime::Cpu;

pub(crate) fn ai_redundant_light_screen(cpu: &mut Cpu) {
    cpu.pc = 0x4487;
    cpu.a = cpu.read_byte(0xc700);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((crate::defs::SCREENS_LIGHT_SCREEN as u8) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn ai_redundant_mist(cpu: &mut Cpu) {
    cpu.pc = 0x448d;
    cpu.a = cpu.read_byte(0xc670);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((crate::defs::SUBSTATUS_MIST as u8) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn ai_redundant_focus_energy(cpu: &mut Cpu) {
    cpu.pc = 0x4493;
    cpu.a = cpu.read_byte(0xc670);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((crate::defs::SUBSTATUS_FOCUS_ENERGY as u8) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn ai_redundant_confuse(cpu: &mut Cpu) {
    cpu.pc = 0x4499;
    cpu.a = cpu.read_byte(0xc66a);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((crate::defs::SUBSTATUS_CONFUSED as u8) & 7);
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
    cpu.a = cpu.read_byte(0xc6ff);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((crate::defs::SCREENS_SAFEGUARD as u8) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn ai_redundant_transform(cpu: &mut Cpu) {
    cpu.pc = 0x44a5;
    cpu.a = cpu.read_byte(0xc671);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((crate::defs::SUBSTATUS_TRANSFORMED as u8) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn ai_redundant_reflect(cpu: &mut Cpu) {
    cpu.pc = 0x44ab;
    cpu.a = cpu.read_byte(0xc700);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((crate::defs::SCREENS_REFLECT as u8) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn ai_redundant_substitute(cpu: &mut Cpu) {
    cpu.pc = 0x44b1;
    cpu.a = cpu.read_byte(0xc670);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((crate::defs::SUBSTATUS_SUBSTITUTE as u8) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn ai_redundant_leech_seed(cpu: &mut Cpu) {
    cpu.pc = 0x44b7;
    cpu.a = cpu.read_byte(0xc66b);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((crate::defs::SUBSTATUS_LEECH_SEED as u8) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn ai_redundant_disable(cpu: &mut Cpu) {
    cpu.pc = 0x44bd;
    cpu.a = cpu.read_byte(0xc675);
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
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn ai_redundant_encore(cpu: &mut Cpu) {
    cpu.pc = 0x44c2;
    cpu.a = cpu.read_byte(0xc66c);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((crate::defs::SUBSTATUS_ENCORED as u8) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn ai_redundant_snore(cpu: &mut Cpu) {
    cpu.pc = 0x44c8;
    become ai_redundant_sleep_talk(cpu);
}

pub(crate) fn ai_redundant_sleep_talk(cpu: &mut Cpu) {
    cpu.pc = 0x44c8;
    cpu.a = cpu.read_byte(0xd214);
    cpu.pc += 3;
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
        become ai_redundant_redundant(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become ai_redundant_not_redundant(cpu);
}

pub(crate) fn ai_redundant_mean_look(cpu: &mut Cpu) {
    cpu.pc = 0x44d1;
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

pub(crate) fn ai_redundant_nightmare(cpu: &mut Cpu) {
    cpu.pc = 0x44d7;
    cpu.a = cpu.read_byte(0xc63a);
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
        become ai_redundant_redundant(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc668);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((crate::defs::SUBSTATUS_NIGHTMARE as u8) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn ai_redundant_spikes(cpu: &mut Cpu) {
    cpu.pc = 0x44e3;
    cpu.a = cpu.read_byte(0xc6ff);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((crate::defs::SCREENS_SPIKES as u8) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn ai_redundant_foresight(cpu: &mut Cpu) {
    cpu.pc = 0x44e9;
    cpu.a = cpu.read_byte(0xc668);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((crate::defs::SUBSTATUS_IDENTIFIED as u8) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn ai_redundant_perish_song(cpu: &mut Cpu) {
    cpu.pc = 0x44ef;
    cpu.a = cpu.read_byte(0xc668);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((crate::defs::SUBSTATUS_PERISH as u8) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn ai_redundant_sandstorm(cpu: &mut Cpu) {
    cpu.pc = 0x44f5;
    cpu.a = cpu.read_byte(0xc70a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = crate::defs::WEATHER_SANDSTORM as u8;
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
        become ai_redundant_redundant(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become ai_redundant_not_redundant(cpu);
}

pub(crate) fn ai_redundant_attract(cpu: &mut Cpu) {
    cpu.pc = 0x44fe;
    cpu.a = 0x0d;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x77f5);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    if cpu.flag_c {
        cpu.cycle(12);
        become ai_redundant_redundant(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xc668);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((crate::defs::SUBSTATUS_IN_LOVE as u8) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn ai_redundant_safeguard(cpu: &mut Cpu) {
    cpu.pc = 0x450c;
    cpu.a = cpu.read_byte(0xc700);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((crate::defs::SCREENS_SAFEGUARD as u8) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn ai_redundant_rain_dance(cpu: &mut Cpu) {
    cpu.pc = 0x4512;
    cpu.a = cpu.read_byte(0xc70a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = crate::defs::WEATHER_RAIN as u8;
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
        become ai_redundant_redundant(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become ai_redundant_not_redundant(cpu);
}

pub(crate) fn ai_redundant_sunny_day(cpu: &mut Cpu) {
    cpu.pc = 0x451b;
    cpu.a = cpu.read_byte(0xc70a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = crate::defs::WEATHER_SUN as u8;
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
        become ai_redundant_redundant(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become ai_redundant_not_redundant(cpu);
}

pub(crate) fn ai_redundant_dream_eater(cpu: &mut Cpu) {
    cpu.pc = 0x4524;
    cpu.a = cpu.read_byte(0xc63a);
    cpu.pc += 3;
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
        become ai_redundant_redundant(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become ai_redundant_not_redundant(cpu);
}

pub(crate) fn ai_redundant_swagger(cpu: &mut Cpu) {
    cpu.pc = 0x452d;
    cpu.a = cpu.read_byte(0xc66a);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((crate::defs::SUBSTATUS_CONFUSED as u8) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn ai_redundant_future_sight(cpu: &mut Cpu) {
    cpu.pc = 0x4533;
    cpu.a = cpu.read_byte(0xc700);
    cpu.pc += 3;
    cpu.cycle(16);
    let mask = 1u8 << ((crate::defs::SCREENS_UNUSED as u8) & 7);
    cpu.flag_z = (cpu.a & mask) == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn ai_redundant_heal(cpu: &mut Cpu) {
    cpu.pc = 0x4539;
    become ai_redundant_morning_sun(cpu);
}

pub(crate) fn ai_redundant_morning_sun(cpu: &mut Cpu) {
    cpu.pc = 0x4539;
    become ai_redundant_synthesis(cpu);
}

pub(crate) fn ai_redundant_synthesis(cpu: &mut Cpu) {
    cpu.pc = 0x4539;
    become ai_redundant_moonlight(cpu);
}

pub(crate) fn ai_redundant_moonlight(cpu: &mut Cpu) {
    cpu.pc = 0x4539;
    cpu.a = 0x0e;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x5251);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    if !cpu.flag_c {
        cpu.cycle(12);
        become ai_redundant_not_redundant(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become ai_redundant_teleport(cpu);
}

pub(crate) fn ai_redundant_teleport(cpu: &mut Cpu) {
    cpu.pc = 0x4541;
    become ai_redundant_redundant(cpu);
}

pub(crate) fn ai_redundant_redundant(cpu: &mut Cpu) {
    cpu.pc = 0x4541;
    cpu.a = 1;
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
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn ai_redundant_not_redundant(cpu: &mut Cpu) {
    cpu.pc = 0x4545;
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

pub(crate) fn ai_redundant(cpu: &mut Cpu) {
    match cpu.c {
        crate::defs::EFFECT_DREAM_EATER => become ai_redundant_dream_eater(cpu),
        crate::defs::EFFECT_HEAL => become ai_redundant_heal(cpu),
        crate::defs::EFFECT_LIGHT_SCREEN => become ai_redundant_light_screen(cpu),
        crate::defs::EFFECT_MIST => become ai_redundant_mist(cpu),
        crate::defs::EFFECT_FOCUS_ENERGY => become ai_redundant_focus_energy(cpu),
        crate::defs::EFFECT_CONFUSE => become ai_redundant_confuse(cpu),
        crate::defs::EFFECT_TRANSFORM => become ai_redundant_transform(cpu),
        crate::defs::EFFECT_REFLECT => become ai_redundant_reflect(cpu),
        crate::defs::EFFECT_SUBSTITUTE => become ai_redundant_substitute(cpu),
        crate::defs::EFFECT_LEECH_SEED => become ai_redundant_leech_seed(cpu),
        crate::defs::EFFECT_DISABLE => become ai_redundant_disable(cpu),
        crate::defs::EFFECT_ENCORE => become ai_redundant_encore(cpu),
        crate::defs::EFFECT_SNORE => become ai_redundant_snore(cpu),
        crate::defs::EFFECT_SLEEP_TALK => become ai_redundant_sleep_talk(cpu),
        crate::defs::EFFECT_MEAN_LOOK => become ai_redundant_mean_look(cpu),
        crate::defs::EFFECT_NIGHTMARE => become ai_redundant_nightmare(cpu),
        crate::defs::EFFECT_SPIKES => become ai_redundant_spikes(cpu),
        crate::defs::EFFECT_FORESIGHT => become ai_redundant_foresight(cpu),
        crate::defs::EFFECT_PERISH_SONG => become ai_redundant_perish_song(cpu),
        crate::defs::EFFECT_SANDSTORM => become ai_redundant_sandstorm(cpu),
        crate::defs::EFFECT_ATTRACT => become ai_redundant_attract(cpu),
        crate::defs::EFFECT_SAFEGUARD => become ai_redundant_safeguard(cpu),
        crate::defs::EFFECT_RAIN_DANCE => become ai_redundant_rain_dance(cpu),
        crate::defs::EFFECT_SUNNY_DAY => become ai_redundant_sunny_day(cpu),
        crate::defs::EFFECT_TELEPORT => become ai_redundant_teleport(cpu),
        crate::defs::EFFECT_MORNING_SUN => become ai_redundant_morning_sun(cpu),
        crate::defs::EFFECT_SYNTHESIS => become ai_redundant_synthesis(cpu),
        crate::defs::EFFECT_MOONLIGHT => become ai_redundant_moonlight(cpu),
        crate::defs::EFFECT_SWAGGER => become ai_redundant_swagger(cpu),
        crate::defs::EFFECT_FUTURE_SIGHT => become ai_redundant_future_sight(cpu),
        _ => become ai_redundant_not_redundant(cpu),
    }
}
