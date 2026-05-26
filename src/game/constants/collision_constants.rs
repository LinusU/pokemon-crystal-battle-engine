use crate::runtime::Cpu;

pub const LAND_TILE: u8 = 0;

pub const WATER_TILE: u8 = 1;

pub const WALL_TILE: u8 = 15;

pub const TALK: u8 = 16;

pub const COLL_FLOOR: u8 = 0;

pub const COLL_01: u8 = 1;

pub const COLL_03: u8 = 3;

pub const COLL_04: u8 = 4;

pub const COLL_WALL: u8 = 7;

pub const COLL_CUT_08: u8 = 8;

pub const COLL_TALL_GRASS_10: u8 = 16;

pub const COLL_CUT_TREE: u8 = 18;

pub const COLL_LONG_GRASS: u8 = 20;

pub const COLL_HEADBUTT_TREE: u8 = 21;

pub const COLL_TALL_GRASS: u8 = 24;

pub const COLL_CUT_TREE_1A: u8 = 26;

pub const COLL_LONG_GRASS_1C: u8 = 28;

pub const COLL_HEADBUTT_TREE_1D: u8 = 29;

pub const COLL_WATER_21: u8 = 33;

pub const COLL_ICE: u8 = 35;

pub const COLL_WHIRLPOOL: u8 = 36;

pub const COLL_BUOY: u8 = 39;

pub const COLL_CUT_28: u8 = 40;

pub const COLL_WATER: u8 = 41;

pub const COLL_ICE_2B: u8 = 43;

pub const COLL_WHIRLPOOL_2C: u8 = 44;

pub const COLL_WATERFALL_RIGHT: u8 = 48;

pub const COLL_WATERFALL_LEFT: u8 = 49;

pub const COLL_WATERFALL_UP: u8 = 50;

pub const COLL_WATERFALL: u8 = 51;

pub const COLL_CURRENT_RIGHT: u8 = 56;

pub const COLL_CURRENT_LEFT: u8 = 57;

pub const COLL_CURRENT_UP: u8 = 58;

pub const COLL_CURRENT_DOWN: u8 = 59;

pub const COLL_BRAKE: u8 = 64;

pub const COLL_WALK_RIGHT: u8 = 65;

pub const COLL_WALK_LEFT: u8 = 66;

pub const COLL_WALK_UP: u8 = 67;

pub const COLL_WALK_DOWN: u8 = 68;

pub const COLL_BRAKE_45: u8 = 69;

pub const COLL_BRAKE_46: u8 = 70;

pub const COLL_BRAKE_47: u8 = 71;

pub const COLL_GRASS_48: u8 = 72;

pub const COLL_GRASS_49: u8 = 73;

pub const COLL_GRASS_4A: u8 = 74;

pub const COLL_GRASS_4B: u8 = 75;

pub const COLL_GRASS_4C: u8 = 76;

pub const COLL_WALK_RIGHT_ALT: u8 = 80;

pub const COLL_WALK_LEFT_ALT: u8 = 81;

pub const COLL_WALK_UP_ALT: u8 = 82;

pub const COLL_WALK_DOWN_ALT: u8 = 83;

pub const COLL_BRAKE_ALT: u8 = 84;

pub const COLL_BRAKE_55: u8 = 85;

pub const COLL_BRAKE_56: u8 = 86;

pub const COLL_BRAKE_57: u8 = 87;

pub const COLL_5B: u8 = 91;

pub const COLL_PIT: u8 = 96;

pub const COLL_VIRTUAL_BOY: u8 = 97;

pub const COLL_64: u8 = 100;

pub const COLL_65: u8 = 101;

pub const COLL_PIT_68: u8 = 104;

pub const COLL_WARP_CARPET_DOWN: u8 = 112;

pub const COLL_DOOR: u8 = 113;

pub const COLL_LADDER: u8 = 114;

pub const COLL_STAIRCASE_73: u8 = 115;

pub const COLL_CAVE_74: u8 = 116;

pub const COLL_DOOR_75: u8 = 117;

pub const COLL_WARP_CARPET_LEFT: u8 = 118;

pub const COLL_WARP_77: u8 = 119;

pub const COLL_WARP_CARPET_UP: u8 = 120;

pub const COLL_DOOR_79: u8 = 121;

pub const COLL_STAIRCASE: u8 = 122;

pub const COLL_CAVE: u8 = 123;

pub const COLL_WARP_PANEL: u8 = 124;

pub const COLL_DOOR_7D: u8 = 125;

pub const COLL_WARP_CARPET_RIGHT: u8 = 126;

pub const COLL_WARP_7F: u8 = 127;

pub const COLL_COUNTER: u8 = 144;

pub const COLL_BOOKSHELF: u8 = 145;

pub const COLL_PC: u8 = 147;

pub const COLL_RADIO: u8 = 148;

pub const COLL_TOWN_MAP: u8 = 149;

pub const COLL_MART_SHELF: u8 = 150;

pub const COLL_TV: u8 = 151;

pub const COLL_COUNTER_98: u8 = 152;

pub const COLL_9C: u8 = 156;

pub const COLL_WINDOW: u8 = 157;

pub const COLL_INCENSE_BURNER: u8 = 159;

pub const COLL_HOP_RIGHT: u8 = 160;

pub const COLL_HOP_LEFT: u8 = 161;

pub const COLL_HOP_UP: u8 = 162;

pub const COLL_HOP_DOWN: u8 = 163;

pub const COLL_HOP_DOWN_RIGHT: u8 = 164;

pub const COLL_HOP_DOWN_LEFT: u8 = 165;

pub const COLL_HOP_UP_RIGHT: u8 = 166;

pub const COLL_HOP_UP_LEFT: u8 = 167;

pub const COLL_RIGHT_WALL: u8 = 176;

pub const COLL_LEFT_WALL: u8 = 177;

pub const COLL_UP_WALL: u8 = 178;

pub const COLL_DOWN_WALL: u8 = 179;

pub const COLL_DOWN_RIGHT_WALL: u8 = 180;

pub const COLL_DOWN_LEFT_WALL: u8 = 181;

pub const COLL_UP_RIGHT_WALL: u8 = 182;

pub const COLL_UP_LEFT_WALL: u8 = 183;

pub const COLL_RIGHT_BUOY: u8 = 192;

pub const COLL_LEFT_BUOY: u8 = 193;

pub const COLL_UP_BUOY: u8 = 194;

pub const COLL_DOWN_BUOY: u8 = 195;

pub const COLL_DOWN_RIGHT_BUOY: u8 = 196;

pub const COLL_DOWN_LEFT_BUOY: u8 = 197;

pub const COLL_UP_RIGHT_BUOY: u8 = 198;

pub const COLL_UP_LEFT_BUOY: u8 = 199;

pub const COLL_FF: u8 = 255;

pub const LO_NYBBLE_GRASS: u8 = 7;

pub const HI_NYBBLE_TALL_GRASS: u8 = 16;

pub const HI_NYBBLE_WATER: u8 = 32;

pub const HI_NYBBLE_CURRENT: u8 = 48;

pub const HI_NYBBLE_WALK: u8 = 64;

pub const HI_NYBBLE_WALK_ALT: u8 = 80;

pub const HI_NYBBLE_WARPS: u8 = 112;

pub const HI_NYBBLE_LEDGES: u8 = 160;

pub const HI_NYBBLE_SIDE_WALLS: u8 = 176;

pub const HI_NYBBLE_SIDE_BUOYS: u8 = 192;

