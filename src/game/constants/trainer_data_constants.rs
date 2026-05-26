use crate::runtime::Cpu;

pub const NUM_TRAINER_ATTRIBUTES: u8 = 7;

pub const CONST_VALUE: u8 = 0;

pub const CONST_INC: u8 = 1;

pub const AI_BASIC: u8 = 1;

pub const AI_BASIC_F: u8 = 0;

pub const AI_SETUP: u8 = 2;

pub const AI_SETUP_F: u8 = 1;

pub const AI_TYPES: u8 = 4;

pub const AI_TYPES_F: u8 = 2;

pub const AI_OFFENSIVE: u8 = 8;

pub const AI_OFFENSIVE_F: u8 = 3;

pub const AI_SMART: u8 = 16;

pub const AI_SMART_F: u8 = 4;

pub const AI_OPPORTUNIST: u8 = 32;

pub const AI_OPPORTUNIST_F: u8 = 5;

pub const AI_AGGRESSIVE: u8 = 64;

pub const AI_AGGRESSIVE_F: u8 = 6;

pub const AI_CAUTIOUS: u8 = 128;

pub const AI_CAUTIOUS_F: u8 = 7;

pub const AI_STATUS: u16 = 256;

pub const AI_STATUS_F: u8 = 8;

pub const AI_RISKY: u16 = 512;

pub const AI_RISKY_F: u8 = 9;

pub const NO_AI: u8 = 0;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const SWITCH_OFTEN: u8 = 1;

pub const SWITCH_OFTEN_F: u8 = 0;

pub const SWITCH_RARELY: u8 = 2;

pub const SWITCH_RARELY_F: u8 = 1;

pub const SWITCH_SOMETIMES: u8 = 4;

pub const SWITCH_SOMETIMES_F: u8 = 2;

pub const ALWAYS_USE: u8 = 16;

pub const ALWAYS_USE_F: u8 = 4;

pub const UNKNOWN_USE: u8 = 32;

pub const UNKNOWN_USE_F: u8 = 5;

pub const CONTEXT_USE: u8 = 64;

pub const CONTEXT_USE_F: u8 = 6;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const TRAINERTYPE_NORMAL: u8 = 0;

pub const TRAINERTYPE_MOVES: u8 = 1;

pub const TRAINERTYPE_ITEM: u8 = 2;

pub const TRAINERTYPE_ITEM_MOVES: u8 = 3;

