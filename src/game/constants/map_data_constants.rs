use crate::runtime::Cpu;

pub const MAPGROUP_N_A: i16 = -1;

pub const GROUP_N_A: i16 = -1;

pub const MAP_N_A: i16 = -1;

pub const MAPGROUP_NONE: u8 = 0;

pub const GROUP_NONE: u8 = 0;

pub const MAP_NONE: u8 = 0;

pub const MAP_LENGTH: u8 = 9;

pub const CONST_VALUE: u8 = 1;

pub const CONST_INC: u8 = 1;

pub const TOWN: u8 = 1;

pub const ROUTE: u8 = 2;

pub const INDOOR: u8 = 3;

pub const CAVE: u8 = 4;

pub const ENVIRONMENT_5: u8 = 5;

pub const GATE: u8 = 6;

pub const DUNGEON: u8 = 7;

pub const NUM_ENVIRONMENTS: u8 = 7;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const PALETTE_AUTO: u8 = 0;

pub const PALETTE_DAY: u8 = 1;

pub const PALETTE_NITE: u8 = 2;

pub const PALETTE_MORN: u8 = 3;

pub const PALETTE_DARK: u8 = 4;

pub const NUM_MAP_PALETTES: u8 = 5;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const FISHGROUP_NONE: u8 = 0;

pub const FISHGROUP_SHORE: u8 = 1;

pub const FISHGROUP_OCEAN: u8 = 2;

pub const FISHGROUP_LAKE: u8 = 3;

pub const FISHGROUP_POND: u8 = 4;

pub const FISHGROUP_DRATINI: u8 = 5;

pub const FISHGROUP_QWILFISH_SWARM: u8 = 6;

pub const FISHGROUP_REMORAID_SWARM: u8 = 7;

pub const FISHGROUP_GYARADOS: u8 = 8;

pub const FISHGROUP_DRATINI_2: u8 = 9;

pub const FISHGROUP_WHIRL_ISLANDS: u8 = 10;

pub const FISHGROUP_QWILFISH: u8 = 11;

pub const FISHGROUP_REMORAID: u8 = 12;

pub const FISHGROUP_QWILFISH_NO_SWARM: u8 = 13;

pub const NUM_FISHGROUPS: u8 = 13;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const EAST: u8 = 1;

pub const EAST_F: u8 = 0;

pub const WEST: u8 = 2;

pub const WEST_F: u8 = 1;

pub const SOUTH: u8 = 4;

pub const SOUTH_F: u8 = 2;

pub const NORTH: u8 = 8;

pub const NORTH_F: u8 = 3;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const SPAWN_HOME: u8 = 0;

pub const SPAWN_DEBUG: u8 = 1;

pub const SPAWN_PALLET: u8 = 2;

pub const SPAWN_VIRIDIAN: u8 = 3;

pub const SPAWN_PEWTER: u8 = 4;

pub const SPAWN_CERULEAN: u8 = 5;

pub const SPAWN_ROCK_TUNNEL: u8 = 6;

pub const SPAWN_VERMILION: u8 = 7;

pub const SPAWN_LAVENDER: u8 = 8;

pub const SPAWN_SAFFRON: u8 = 9;

pub const SPAWN_CELADON: u8 = 10;

pub const SPAWN_FUCHSIA: u8 = 11;

pub const SPAWN_CINNABAR: u8 = 12;

pub const SPAWN_INDIGO: u8 = 13;

pub const SPAWN_NEW_BARK: u8 = 14;

pub const SPAWN_CHERRYGROVE: u8 = 15;

pub const SPAWN_VIOLET: u8 = 16;

pub const SPAWN_UNION_CAVE: u8 = 17;

pub const SPAWN_AZALEA: u8 = 18;

pub const SPAWN_CIANWOOD: u8 = 19;

pub const SPAWN_GOLDENROD: u8 = 20;

pub const SPAWN_OLIVINE: u8 = 21;

pub const SPAWN_ECRUTEAK: u8 = 22;

pub const SPAWN_MAHOGANY: u8 = 23;

pub const SPAWN_LAKE_OF_RAGE: u8 = 24;

pub const SPAWN_BLACKTHORN: u8 = 25;

pub const SPAWN_MT_SILVER: u8 = 26;

pub const SPAWN_FAST_SHIP: u8 = 27;

pub const NUM_SPAWNS: u8 = 28;

pub const SPAWN_N_A: i16 = -1;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const JOHTO_FLYPOINT: u8 = 0;

pub const FLY_NEW_BARK: u8 = 0;

pub const FLY_CHERRYGROVE: u8 = 1;

pub const FLY_VIOLET: u8 = 2;

pub const FLY_AZALEA: u8 = 3;

pub const FLY_GOLDENROD: u8 = 4;

pub const FLY_ECRUTEAK: u8 = 5;

pub const FLY_OLIVINE: u8 = 6;

pub const FLY_CIANWOOD: u8 = 7;

pub const FLY_MAHOGANY: u8 = 8;

pub const FLY_LAKE_OF_RAGE: u8 = 9;

pub const FLY_BLACKTHORN: u8 = 10;

pub const FLY_MT_SILVER: u8 = 11;

pub const KANTO_FLYPOINT: u8 = 12;

pub const FLY_PALLET: u8 = 12;

pub const FLY_VIRIDIAN: u8 = 13;

pub const FLY_PEWTER: u8 = 14;

pub const FLY_CERULEAN: u8 = 15;

pub const FLY_VERMILION: u8 = 16;

pub const FLY_ROCK_TUNNEL: u8 = 17;

pub const FLY_LAVENDER: u8 = 18;

pub const FLY_CELADON: u8 = 19;

pub const FLY_SAFFRON: u8 = 20;

pub const FLY_FUCHSIA: u8 = 21;

pub const FLY_CINNABAR: u8 = 22;

pub const FLY_INDIGO: u8 = 23;

pub const NUM_FLYPOINTS: u8 = 24;

pub const MAX_OUTDOOR_SPRITES: u8 = 23;

