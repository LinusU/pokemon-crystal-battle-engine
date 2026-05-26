use crate::runtime::Cpu;

pub const CONST_VALUE: u8 = 241;

pub const CONST_INC: u8 = 1;

pub const MAPSETUP_WARP: u8 = 241;

pub const MAPSETUP_CONTINUE: u8 = 242;

pub const MAPSETUP_RELOADMAP: u8 = 243;

pub const MAPSETUP_TELEPORT: u8 = 244;

pub const MAPSETUP_DOOR: u8 = 245;

pub const MAPSETUP_FALL: u8 = 246;

pub const MAPSETUP_CONNECTION: u8 = 247;

pub const MAPSETUP_LINKRETURN: u8 = 248;

pub const MAPSETUP_TRAIN: u8 = 249;

pub const MAPSETUP_SUBMENU: u8 = 250;

pub const MAPSETUP_BADWARP: u8 = 251;

pub const MAPSETUP_FLY: u8 = 252;

pub const NUM_MAPSETUP_SCRIPTS: u8 = 12;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const MAPCALLBACK_TILES: u8 = 1;

pub const MAPCALLBACK_OBJECTS: u8 = 2;

pub const MAPCALLBACK_CMDQUEUE: u8 = 3;

pub const MAPCALLBACK_SPRITES: u8 = 4;

pub const MAPCALLBACK_NEWMAP: u8 = 5;

pub const NUM_MAPCALLBACK_TYPES: u8 = 5;

pub const MAPSETUPSCRIPT_HAS_PARAM_F: u8 = 7;

