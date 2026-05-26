use crate::runtime::Cpu;

pub const OBJECT_LENGTH: u8 = 40;

pub const NUM_OBJECT_STRUCTS: u8 = 13;

pub const OW_DOWN: u8 = 0;

pub const OW_UP: u8 = 4;

pub const OW_LEFT: u8 = 8;

pub const OW_RIGHT: u8 = 12;

pub const CONST_VALUE: u8 = 0;

pub const CONST_INC: u8 = 1;

pub const INVISIBLE: u8 = 1;

pub const INVISIBLE_F: u8 = 0;

pub const WONT_DELETE: u8 = 2;

pub const WONT_DELETE_F: u8 = 1;

pub const FIXED_FACING: u8 = 4;

pub const FIXED_FACING_F: u8 = 2;

pub const SLIDING: u8 = 8;

pub const SLIDING_F: u8 = 3;

pub const NOCLIP_TILES: u8 = 16;

pub const NOCLIP_TILES_F: u8 = 4;

pub const MOVE_ANYWHERE: u8 = 32;

pub const MOVE_ANYWHERE_F: u8 = 5;

pub const NOCLIP_OBJS: u8 = 64;

pub const NOCLIP_OBJS_F: u8 = 6;

pub const EMOTE_OBJECT: u8 = 128;

pub const EMOTE_OBJECT_F: u8 = 7;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const LOW_PRIORITY: u8 = 1;

pub const LOW_PRIORITY_F: u8 = 0;

pub const HIGH_PRIORITY: u8 = 2;

pub const HIGH_PRIORITY_F: u8 = 1;

pub const BOULDER_MOVING: u8 = 4;

pub const BOULDER_MOVING_F: u8 = 2;

pub const IN_GRASS: u8 = 8;

pub const IN_GRASS_F: u8 = 3;

pub const USE_OBP1: u8 = 16;

pub const USE_OBP1_F: u8 = 4;

pub const FROZEN: u8 = 32;

pub const FROZEN_F: u8 = 5;

pub const OFF_SCREEN: u8 = 64;

pub const OFF_SCREEN_F: u8 = 6;

pub const UNDER_TILES: u8 = 128;

pub const UNDER_TILES_F: u8 = 7;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const SWIMMING: u8 = 32;

pub const SWIMMING_F: u8 = 5;

pub const STRENGTH_BOULDER: u8 = 64;

pub const STRENGTH_BOULDER_F: u8 = 6;

pub const BIG_OBJECT: u8 = 128;

pub const BIG_OBJECT_F: u8 = 7;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const RELATIVE_ATTRIBUTES: u8 = 2;

pub const RELATIVE_ATTRIBUTES_F: u8 = 1;

pub const ABSOLUTE_TILE_ID: u8 = 4;

pub const ABSOLUTE_TILE_ID_F: u8 = 2;

pub const MAPOBJECT_LENGTH: u8 = 16;

pub const NUM_OBJECTS: u8 = 16;

pub const PLAYER_OBJECT: u8 = 0;

pub const MAPOBJECT_PALETTE_MASK: u8 = 240;

pub const MAPOBJECT_TYPE_MASK: u8 = 15;

pub const NUM_SPRITEMOVEDATA_FIELDS: u8 = 6;

pub const MAPOBJECT_SCREEN_WIDTH: u8 = 12;

pub const MAPOBJECT_SCREEN_HEIGHT: u8 = 11;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const SPRITEMOVEDATA_00: u8 = 0;

pub const SPRITEMOVEDATA_STILL: u8 = 1;

pub const SPRITEMOVEDATA_WANDER: u8 = 2;

pub const SPRITEMOVEDATA_SPINRANDOM_SLOW: u8 = 3;

pub const SPRITEMOVEDATA_WALK_UP_DOWN: u8 = 4;

pub const SPRITEMOVEDATA_WALK_LEFT_RIGHT: u8 = 5;

pub const SPRITEMOVEDATA_STANDING_DOWN: u8 = 6;

pub const SPRITEMOVEDATA_STANDING_UP: u8 = 7;

pub const SPRITEMOVEDATA_STANDING_LEFT: u8 = 8;

pub const SPRITEMOVEDATA_STANDING_RIGHT: u8 = 9;

pub const SPRITEMOVEDATA_SPINRANDOM_FAST: u8 = 10;

pub const SPRITEMOVEDATA_PLAYER: u8 = 11;

pub const SPRITEMOVEDATA_INDEXED_1: u8 = 12;

pub const SPRITEMOVEDATA_INDEXED_2: u8 = 13;

pub const SPRITEMOVEDATA_0E: u8 = 14;

pub const SPRITEMOVEDATA_0F: u8 = 15;

pub const SPRITEMOVEDATA_10: u8 = 16;

pub const SPRITEMOVEDATA_11: u8 = 17;

pub const SPRITEMOVEDATA_12: u8 = 18;

pub const SPRITEMOVEDATA_FOLLOWING: u8 = 19;

pub const SPRITEMOVEDATA_SCRIPTED: u8 = 20;

pub const SPRITEMOVEDATA_BIGDOLLSYM: u8 = 21;

pub const SPRITEMOVEDATA_POKEMON: u8 = 22;

pub const SPRITEMOVEDATA_SUDOWOODO: u8 = 23;

pub const SPRITEMOVEDATA_SMASHABLE_ROCK: u8 = 24;

pub const SPRITEMOVEDATA_STRENGTH_BOULDER: u8 = 25;

pub const SPRITEMOVEDATA_FOLLOWNOTEXACT: u8 = 26;

pub const SPRITEMOVEDATA_SHADOW: u8 = 27;

pub const SPRITEMOVEDATA_EMOTE: u8 = 28;

pub const SPRITEMOVEDATA_SCREENSHAKE: u8 = 29;

pub const SPRITEMOVEDATA_SPINCOUNTERCLOCKWISE: u8 = 30;

pub const SPRITEMOVEDATA_SPINCLOCKWISE: u8 = 31;

pub const SPRITEMOVEDATA_BIGDOLLASYM: u8 = 32;

pub const SPRITEMOVEDATA_BIGDOLL: u8 = 33;

pub const SPRITEMOVEDATA_BOULDERDUST: u8 = 34;

pub const SPRITEMOVEDATA_GRASS: u8 = 35;

pub const SPRITEMOVEDATA_SWIM_WANDER: u8 = 36;

pub const NUM_SPRITEMOVEDATA: u8 = 37;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const SPRITEMOVEFN_00: u8 = 0;

pub const SPRITEMOVEFN_RANDOM_WALK_Y: u8 = 1;

pub const SPRITEMOVEFN_RANDOM_WALK_X: u8 = 2;

pub const SPRITEMOVEFN_RANDOM_WALK_XY: u8 = 3;

pub const SPRITEMOVEFN_SLOW_RANDOM_SPIN: u8 = 4;

pub const SPRITEMOVEFN_FAST_RANDOM_SPIN: u8 = 5;

pub const SPRITEMOVEFN_STANDING: u8 = 6;

pub const SPRITEMOVEFN_OBEY_DPAD: u8 = 7;

pub const SPRITEMOVEFN_INDEXED_1: u8 = 8;

pub const SPRITEMOVEFN_INDEXED_2: u8 = 9;

pub const SPRITEMOVEFN_0A: u8 = 10;

pub const SPRITEMOVEFN_0B: u8 = 11;

pub const SPRITEMOVEFN_0C: u8 = 12;

pub const SPRITEMOVEFN_0D: u8 = 13;

pub const SPRITEMOVEFN_0E: u8 = 14;

pub const SPRITEMOVEFN_FOLLOW: u8 = 15;

pub const SPRITEMOVEFN_SCRIPTED: u8 = 16;

pub const SPRITEMOVEFN_STRENGTH: u8 = 17;

pub const SPRITEMOVEFN_FOLLOWNOTEXACT: u8 = 18;

pub const SPRITEMOVEFN_SHADOW: u8 = 19;

pub const SPRITEMOVEFN_EMOTE: u8 = 20;

pub const SPRITEMOVEFN_BIG_SNORLAX: u8 = 21;

pub const SPRITEMOVEFN_BOUNCE: u8 = 22;

pub const SPRITEMOVEFN_SCREENSHAKE: u8 = 23;

pub const SPRITEMOVEFN_SPIN_CLOCKWISE: u8 = 24;

pub const SPRITEMOVEFN_SPIN_COUNTERCLOCKWISE: u8 = 25;

pub const SPRITEMOVEFN_BOULDERDUST: u8 = 26;

pub const SPRITEMOVEFN_GRASS: u8 = 27;

pub const NUM_SPRITEMOVEFN: u8 = 28;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const STEP_TYPE_RESET: u8 = 0;

pub const STEP_TYPE_FROM_MOVEMENT: u8 = 1;

pub const STEP_TYPE_NPC_WALK: u8 = 2;

pub const STEP_TYPE_SLEEP: u8 = 3;

pub const STEP_TYPE_STANDING: u8 = 4;

pub const STEP_TYPE_RESTORE: u8 = 5;

pub const STEP_TYPE_PLAYER_WALK: u8 = 6;

pub const STEP_TYPE_CONTINUE_WALK: u8 = 7;

pub const STEP_TYPE_NPC_JUMP: u8 = 8;

pub const STEP_TYPE_PLAYER_JUMP: u8 = 9;

pub const STEP_TYPE_TURN: u8 = 10;

pub const STEP_TYPE_BUMP: u8 = 11;

pub const STEP_TYPE_TELEPORT_FROM: u8 = 12;

pub const STEP_TYPE_TELEPORT_TO: u8 = 13;

pub const STEP_TYPE_SKYFALL: u8 = 14;

pub const STEP_TYPE_STRENGTH_BOULDER: u8 = 15;

pub const STEP_TYPE_GOT_BITE: u8 = 16;

pub const STEP_TYPE_ROCK_SMASH: u8 = 17;

pub const STEP_TYPE_RETURN_DIG: u8 = 18;

pub const STEP_TYPE_TRACKING_OBJECT: u8 = 19;

pub const STEP_TYPE_14: u8 = 20;

pub const STEP_TYPE_SCREENSHAKE: u8 = 21;

pub const STEP_TYPE_16: u8 = 22;

pub const STEP_TYPE_17: u8 = 23;

pub const STEP_TYPE_DELETE: u8 = 24;

pub const STEP_TYPE_SKYFALL_TOP: u8 = 25;

pub const NUM_STEP_TYPES: u8 = 26;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const OBJECT_ACTION_00: u8 = 0;

pub const OBJECT_ACTION_STAND: u8 = 1;

pub const OBJECT_ACTION_STEP: u8 = 2;

pub const OBJECT_ACTION_BUMP: u8 = 3;

pub const OBJECT_ACTION_SPIN: u8 = 4;

pub const OBJECT_ACTION_SPIN_FLICKER: u8 = 5;

pub const OBJECT_ACTION_FISHING: u8 = 6;

pub const OBJECT_ACTION_SHADOW: u8 = 7;

pub const OBJECT_ACTION_EMOTE: u8 = 8;

pub const OBJECT_ACTION_BIG_DOLL_SYM: u8 = 9;

pub const OBJECT_ACTION_BOUNCE: u8 = 10;

pub const OBJECT_ACTION_WEIRD_TREE: u8 = 11;

pub const OBJECT_ACTION_BIG_DOLL_ASYM: u8 = 12;

pub const OBJECT_ACTION_BIG_DOLL: u8 = 13;

pub const OBJECT_ACTION_BOULDER_DUST: u8 = 14;

pub const OBJECT_ACTION_GRASS_SHAKE: u8 = 15;

pub const OBJECT_ACTION_SKYFALL: u8 = 16;

pub const NUM_OBJECT_ACTIONS: u8 = 17;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const FACING_STEP_DOWN_0: u8 = 0;

pub const FACING_STEP_DOWN_1: u8 = 1;

pub const FACING_STEP_DOWN_2: u8 = 2;

pub const FACING_STEP_DOWN_3: u8 = 3;

pub const FACING_STEP_UP_0: u8 = 4;

pub const FACING_STEP_UP_1: u8 = 5;

pub const FACING_STEP_UP_2: u8 = 6;

pub const FACING_STEP_UP_3: u8 = 7;

pub const FACING_STEP_LEFT_0: u8 = 8;

pub const FACING_STEP_LEFT_1: u8 = 9;

pub const FACING_STEP_LEFT_2: u8 = 10;

pub const FACING_STEP_LEFT_3: u8 = 11;

pub const FACING_STEP_RIGHT_0: u8 = 12;

pub const FACING_STEP_RIGHT_1: u8 = 13;

pub const FACING_STEP_RIGHT_2: u8 = 14;

pub const FACING_STEP_RIGHT_3: u8 = 15;

pub const FACING_FISH_DOWN: u8 = 16;

pub const FACING_FISH_UP: u8 = 17;

pub const FACING_FISH_LEFT: u8 = 18;

pub const FACING_FISH_RIGHT: u8 = 19;

pub const FACING_EMOTE: u8 = 20;

pub const FACING_SHADOW: u8 = 21;

pub const FACING_BIG_DOLL_ASYM: u8 = 22;

pub const FACING_BIG_DOLL_SYM: u8 = 23;

pub const FACING_WEIRD_TREE_0: u8 = 24;

pub const FACING_WEIRD_TREE_1: u8 = 25;

pub const FACING_WEIRD_TREE_2: u8 = 26;

pub const FACING_WEIRD_TREE_3: u8 = 27;

pub const FACING_BOULDER_DUST_1: u8 = 28;

pub const FACING_BOULDER_DUST_2: u8 = 29;

pub const FACING_GRASS_1: u8 = 30;

pub const FACING_GRASS_2: u8 = 31;

pub const NUM_FACINGS: u8 = 32;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const STEP_SLOW: u8 = 0;

pub const STEP_WALK: u8 = 1;

pub const STEP_BIKE: u8 = 2;

pub const STEP_LEDGE: u8 = 3;

pub const STEP_ICE: u8 = 4;

pub const STEP_TURN: u8 = 5;

pub const STEP_BACK_LEDGE: u8 = 6;

pub const STEP_WALK_IN_PLACE: u8 = 7;

pub const NUM_STEPS: u8 = 8;

