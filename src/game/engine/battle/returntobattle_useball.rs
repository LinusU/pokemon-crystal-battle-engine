use crate::runtime::Cpu;

pub(crate) fn _return_to_battle_use_ball(cpu: &mut Cpu) {
    cpu.pc = 0x715c;
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
        cpu.cycle(12);
        become _return_to_battle_use_ball_gettutorialbackpic(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = 0x0f;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x743d);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(12);
    become _return_to_battle_use_ball_continue(cpu);
}

pub(crate) fn _return_to_battle_use_ball_gettutorialbackpic(cpu: &mut Cpu) {
    cpu.pc = 0x7171;
    cpu.a = 0x0f;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x7bff);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 1;
    cpu.push_word(next_pc);
    cpu.cycle(16);
    crate::game::home::header::far_call(cpu);
    assert_eq!(cpu.pc, next_pc);
    become _return_to_battle_use_ball_continue(cpu);
}

pub(crate) fn _return_to_battle_use_ball_continue(cpu: &mut Cpu) {
    cpu.pc = 0x7177;
    cpu.a = 0x0f;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0x747c);
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
    cpu.set_hl(0x6d9f);
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
    crate::game::home::tilemap::get_mem_sgb_layout(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::menu::close_window(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::menu::load_standard_menu_header(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::tilemap::wait_bg_map(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.cycle(16);
    become crate::game::home::tilemap::set_default_bgp_and_obp(cpu);
}
