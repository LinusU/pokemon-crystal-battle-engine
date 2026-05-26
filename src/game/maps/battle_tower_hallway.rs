use crate::runtime::Cpu;

pub const CONST_VALUE: u8 = 2;

pub const CONST_INC: u8 = 1;

pub const BATTLETOWERHALLWAY_RECEPTIONIST: u8 = 2;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const _NUM_SCENE_SCRIPTS_U313: u8 = 0;

pub const SCENE_BATTLETOWERHALLWAY_ENTER: u8 = 0;

pub const SCENE_BATTLETOWERHALLWAY_NOOP: u8 = 1;

pub const _NUM_CALLBACKS_U314: u8 = 0;

pub const MAP_CALLBACK_1: u8 = 0;

pub const MAP_CALLBACK_2: u8 = 0;

pub const MAP_CALLBACK_3: u8 = 0;

pub const MAP_CALLBACK_4: u8 = 0;

pub const MAP_CALLBACK_5: u8 = 0;

pub const _NUM_WARP_EVENTS_U315: u8 = 0;

pub const _NUM_COORD_EVENTS_U316: u8 = 0;

pub const _NUM_BG_EVENTS_U317: u8 = 0;

pub const _NUM_OBJECT_EVENTS_U318: u8 = 0;

pub(crate) fn battle_tower_hallway_choose_battle_room_script_asm_load_battle_room(cpu: &mut Cpu) {
    cpu.pc = 0x75cb;
    cpu.a = cpu.read_byte(crate::defs::R_WBK as u16);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = 0x03;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xd800);
    cpu.pc += 3;
    cpu.cycle(16);
    cpu.write_byte(0xc2dd, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

