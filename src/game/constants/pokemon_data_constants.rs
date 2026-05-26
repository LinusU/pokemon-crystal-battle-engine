use crate::runtime::Cpu;

pub const BASE_DATA_SIZE: u8 = 32;

pub const GENDER_F0: u8 = 0;

pub const GENDER_F12_5: u8 = 31;

pub const GENDER_F25: u8 = 63;

pub const GENDER_F50: u8 = 127;

pub const GENDER_F75: u8 = 191;

pub const GENDER_F100: u8 = 254;

pub const GENDER_UNKNOWN: i16 = -1;

pub const CONST_VALUE: u8 = 0;

pub const CONST_INC: u8 = 1;

pub const GROWTH_MEDIUM_FAST: u8 = 0;

pub const GROWTH_SLIGHTLY_FAST: u8 = 1;

pub const GROWTH_SLIGHTLY_SLOW: u8 = 2;

pub const GROWTH_MEDIUM_SLOW: u8 = 3;

pub const GROWTH_FAST: u8 = 4;

pub const GROWTH_SLOW: u8 = 5;

pub const NUM_GROWTH_RATES: u8 = 6;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const EGG_MONSTER: u8 = 1;

pub const EGG_WATER_1: u8 = 2;

pub const EGG_BUG: u8 = 3;

pub const EGG_FLYING: u8 = 4;

pub const EGG_GROUND: u8 = 5;

pub const EGG_FAIRY: u8 = 6;

pub const EGG_PLANT: u8 = 7;

pub const EGG_HUMANSHAPE: u8 = 8;

pub const EGG_WATER_3: u8 = 9;

pub const EGG_MINERAL: u8 = 10;

pub const EGG_INDETERMINATE: u8 = 11;

pub const EGG_WATER_2: u8 = 12;

pub const EGG_DITTO: u8 = 13;

pub const EGG_DRAGON: u8 = 14;

pub const EGG_NONE: u8 = 15;

pub const NUM_DEX_ENTRY_BANKS: u8 = 4;

pub const BOXMON_STRUCT_LENGTH: u8 = 32;

pub const PARTYMON_STRUCT_LENGTH: u8 = 48;

pub const NICKNAMED_MON_STRUCT_LENGTH: u8 = 59;

pub const REDMON_STRUCT_LENGTH: u8 = 44;

pub const CAUGHT_TIME_MASK: u8 = 192;

pub const CAUGHT_LEVEL_MASK: u8 = 63;

pub const CAUGHT_GENDER_MASK: u8 = 128;

pub const CAUGHT_LOCATION_MASK: u8 = 127;

pub const CAUGHT_BY_UNKNOWN: u8 = 0;

pub const CAUGHT_BY_GIRL: u8 = 1;

pub const CAUGHT_BY_BOY: u8 = 2;

pub const CAUGHT_EGG_LEVEL: u8 = 1;

pub const MON_CRY_LENGTH: u8 = 6;

pub const PARTY_LENGTH: u8 = 6;

pub const MONS_PER_BOX: u8 = 20;

pub const MONS_PER_BOX_JP: u8 = 30;

pub const BOX_LENGTH: u16 = 1104;

pub const NUM_BOXES: u8 = 14;

pub const BOXLIST_SIZE: u8 = 3;

pub const HOF_MON_LENGTH: u8 = 16;

pub const HOF_LENGTH: u8 = 98;

pub const NUM_HOF_TEAMS: u8 = 30;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const EVOLVE_LEVEL: u8 = 1;

pub const EVOLVE_ITEM: u8 = 2;

pub const EVOLVE_TRADE: u8 = 3;

pub const EVOLVE_HAPPINESS: u8 = 4;

pub const EVOLVE_STAT: u8 = 5;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const TR_ANYTIME: u8 = 1;

pub const TR_MORNDAY: u8 = 2;

pub const TR_NITE: u8 = 3;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const ATK_GT_DEF: u8 = 1;

pub const ATK_LT_DEF: u8 = 2;

pub const ATK_EQ_DEF: u8 = 3;

pub const NUM_GRASSMON: u8 = 7;

pub const NUM_WATERMON: u8 = 3;

pub const GRASS_WILDDATA_LENGTH: u8 = 47;

pub const WATER_WILDDATA_LENGTH: u8 = 9;

pub const FISHGROUP_DATA_LENGTH: u8 = 7;

pub const NUM_ROAMMON_MAPS: u8 = 16;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const TREEMON_SET_NONE: u8 = 0;

pub const TREEMON_SET_CANYON: u8 = 1;

pub const TREEMON_SET_TOWN: u8 = 2;

pub const TREEMON_SET_ROUTE: u8 = 3;

pub const TREEMON_SET_KANTO: u8 = 4;

pub const TREEMON_SET_LAKE: u8 = 5;

pub const TREEMON_SET_FOREST: u8 = 6;

pub const TREEMON_SET_ROCK: u8 = 7;

pub const NUM_TREEMON_SETS: u8 = 8;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const TREEMON_SCORE_BAD: u8 = 0;

pub const TREEMON_SCORE_GOOD: u8 = 1;

pub const TREEMON_SCORE_RARE: u8 = 2;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const HAPPINESS_GAINLEVEL: u8 = 1;

pub const HAPPINESS_USEDITEM: u8 = 2;

pub const HAPPINESS_USEDXITEM: u8 = 3;

pub const HAPPINESS_GYMBATTLE: u8 = 4;

pub const HAPPINESS_LEARNMOVE: u8 = 5;

pub const HAPPINESS_FAINTED: u8 = 6;

pub const HAPPINESS_POISONFAINT: u8 = 7;

pub const HAPPINESS_BEATENBYSTRONGFOE: u8 = 8;

pub const HAPPINESS_OLDERCUT1: u8 = 9;

pub const HAPPINESS_OLDERCUT2: u8 = 10;

pub const HAPPINESS_OLDERCUT3: u8 = 11;

pub const HAPPINESS_YOUNGCUT1: u8 = 12;

pub const HAPPINESS_YOUNGCUT2: u8 = 13;

pub const HAPPINESS_YOUNGCUT3: u8 = 14;

pub const HAPPINESS_BITTERPOWDER: u8 = 15;

pub const HAPPINESS_ENERGYROOT: u8 = 16;

pub const HAPPINESS_REVIVALHERB: u8 = 17;

pub const HAPPINESS_GROOMING: u8 = 18;

pub const HAPPINESS_GAINLEVELATHOME: u8 = 19;

pub const NUM_HAPPINESS_CHANGES: u8 = 19;

pub const BASE_HAPPINESS: u8 = 70;

pub const FRIEND_BALL_HAPPINESS: u8 = 200;

pub const HAPPINESS_TO_EVOLVE: u8 = 220;

pub const HAPPINESS_THRESHOLD_1: u8 = 100;

pub const HAPPINESS_THRESHOLD_2: u8 = 200;

pub const PP_UP_MASK: u8 = 192;

pub const PP_UP_ONE: u8 = 64;

pub const PP_MASK: u8 = 63;

