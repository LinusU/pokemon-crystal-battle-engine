use crate::runtime::Cpu;

pub const DECOATTR_STRUCT_LENGTH: u8 = 6;

pub const CONST_VALUE: u8 = 1;

pub const CONST_INC: u8 = 1;

pub const DECO_PLANT: u8 = 1;

pub const DECO_BED: u8 = 2;

pub const DECO_CARPET: u8 = 3;

pub const DECO_POSTER: u8 = 4;

pub const DECO_DOLL: u8 = 5;

pub const DECO_BIGDOLL: u8 = 6;

pub const NUM_DECO_TYPES: u8 = 6;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const CANCEL_DECO: u8 = 0;

pub const PUT_IT_AWAY: u8 = 1;

pub const MAGNAPLANT: u8 = 2;

pub const TROPICPLANT: u8 = 3;

pub const JUMBOPLANT: u8 = 4;

pub const TOWN_MAP_POSTER: u8 = 5;

pub const FAMICOM: u8 = 6;

pub const SUPER_NES: u8 = 7;

pub const NINTENDO_64: u8 = 8;

pub const VIRTUAL_BOY: u8 = 9;

pub const GOLD_TROPHY: u8 = 10;

pub const SILVER_TROPHY: u8 = 11;

pub const SURF_PIKA_DOLL: u8 = 12;

pub const _BED: u8 = 13;

pub const _CARPET: u8 = 14;

pub const _POSTER: u8 = 15;

pub const _DOLL: u8 = 16;

pub const BIG_: u8 = 17;

pub const FEATHERY_BED: u8 = 18;

pub const PIKACHU_BED: u8 = 19;

pub const PINK_BED: u8 = 20;

pub const POLKADOT_BED: u8 = 21;

pub const RED_CARPET: u8 = 22;

pub const BLUE_CARPET: u8 = 23;

pub const YELLOW_CARPET: u8 = 24;

pub const GREEN_CARPET: u8 = 25;

pub const NUM_DECO_NAMES: u8 = 26;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const SET_UP_BED: u8 = 1;

pub const PUT_AWAY_BED: u8 = 2;

pub const SET_UP_CARPET: u8 = 3;

pub const PUT_AWAY_CARPET: u8 = 4;

pub const SET_UP_PLANT: u8 = 5;

pub const PUT_AWAY_PLANT: u8 = 6;

pub const SET_UP_POSTER: u8 = 7;

pub const PUT_AWAY_POSTER: u8 = 8;

pub const SET_UP_CONSOLE: u8 = 9;

pub const PUT_AWAY_CONSOLE: u8 = 10;

pub const SET_UP_BIG_DOLL: u8 = 11;

pub const PUT_AWAY_BIG_DOLL: u8 = 12;

pub const SET_UP_DOLL: u8 = 13;

pub const PUT_AWAY_DOLL: u8 = 14;

pub const NUM_DECO_ACTIONS: u8 = 14;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const DECOFLAG_FEATHERY_BED: u8 = 0;

pub const DECOFLAG_PINK_BED: u8 = 1;

pub const DECOFLAG_POLKADOT_BED: u8 = 2;

pub const DECOFLAG_PIKACHU_BED: u8 = 3;

pub const DECOFLAG_RED_CARPET: u8 = 4;

pub const DECOFLAG_BLUE_CARPET: u8 = 5;

pub const DECOFLAG_YELLOW_CARPET: u8 = 6;

pub const DECOFLAG_GREEN_CARPET: u8 = 7;

pub const DECOFLAG_MAGNAPLANT: u8 = 8;

pub const DECOFLAG_TROPICPLANT: u8 = 9;

pub const DECOFLAG_JUMBOPLANT: u8 = 10;

pub const DECOFLAG_TOWN_MAP: u8 = 11;

pub const DECOFLAG_PIKACHU_POSTER: u8 = 12;

pub const DECOFLAG_CLEFAIRY_POSTER: u8 = 13;

pub const DECOFLAG_JIGGLYPUFF_POSTER: u8 = 14;

pub const DECOFLAG_FAMICOM: u8 = 15;

pub const DECOFLAG_SNES: u8 = 16;

pub const DECOFLAG_N64: u8 = 17;

pub const DECOFLAG_VIRTUAL_BOY: u8 = 18;

pub const DECOFLAG_PIKACHU_DOLL: u8 = 19;

pub const DECOFLAG_SURF_PIKACHU_DOLL: u8 = 20;

pub const DECOFLAG_CLEFAIRY_DOLL: u8 = 21;

pub const DECOFLAG_JIGGLYPUFF_DOLL: u8 = 22;

pub const DECOFLAG_BULBASAUR_DOLL: u8 = 23;

pub const DECOFLAG_CHARMANDER_DOLL: u8 = 24;

pub const DECOFLAG_SQUIRTLE_DOLL: u8 = 25;

pub const DECOFLAG_POLIWAG_DOLL: u8 = 26;

pub const DECOFLAG_DIGLETT_DOLL: u8 = 27;

pub const DECOFLAG_STARYU_DOLL: u8 = 28;

pub const DECOFLAG_MAGIKARP_DOLL: u8 = 29;

pub const DECOFLAG_ODDISH_DOLL: u8 = 30;

pub const DECOFLAG_GENGAR_DOLL: u8 = 31;

pub const DECOFLAG_SHELLDER_DOLL: u8 = 32;

pub const DECOFLAG_GRIMER_DOLL: u8 = 33;

pub const DECOFLAG_VOLTORB_DOLL: u8 = 34;

pub const DECOFLAG_WEEDLE_DOLL: u8 = 35;

pub const DECOFLAG_UNOWN_DOLL: u8 = 36;

pub const DECOFLAG_GEODUDE_DOLL: u8 = 37;

pub const DECOFLAG_MACHOP_DOLL: u8 = 38;

pub const DECOFLAG_TENTACOOL_DOLL: u8 = 39;

pub const DECOFLAG_BIG_SNORLAX_DOLL: u8 = 40;

pub const DECOFLAG_BIG_ONIX_DOLL: u8 = 41;

pub const DECOFLAG_BIG_LAPRAS_DOLL: u8 = 42;

pub const NUM_NON_TROPHY_DECOS: u8 = 43;

pub const DECOFLAG_GOLD_TROPHY_DOLL: u8 = 43;

pub const DECOFLAG_SILVER_TROPHY_DOLL: u8 = 44;

pub const NUM_DECOS: u8 = 45;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const BEDS: u8 = 1;

pub const DECO_FEATHERY_BED: u8 = 2;

pub const DECO_PINK_BED: u8 = 3;

pub const DECO_POLKADOT_BED: u8 = 4;

pub const DECO_PIKACHU_BED: u8 = 5;

pub const CARPETS: u8 = 6;

pub const DECO_RED_CARPET: u8 = 7;

pub const DECO_BLUE_CARPET: u8 = 8;

pub const DECO_YELLOW_CARPET: u8 = 9;

pub const DECO_GREEN_CARPET: u8 = 10;

pub const PLANTS: u8 = 11;

pub const DECO_MAGNAPLANT: u8 = 12;

pub const DECO_TROPICPLANT: u8 = 13;

pub const DECO_JUMBOPLANT: u8 = 14;

pub const POSTERS: u8 = 15;

pub const DECO_TOWN_MAP: u8 = 16;

pub const DECO_PIKACHU_POSTER: u8 = 17;

pub const DECO_CLEFAIRY_POSTER: u8 = 18;

pub const DECO_JIGGLYPUFF_POSTER: u8 = 19;

pub const CONSOLES: u8 = 20;

pub const DECO_FAMICOM: u8 = 21;

pub const DECO_SNES: u8 = 22;

pub const DECO_N64: u8 = 23;

pub const DECO_VIRTUAL_BOY: u8 = 24;

pub const BIG_DOLLS: u8 = 25;

pub const DECO_BIG_SNORLAX_DOLL: u8 = 26;

pub const DECO_BIG_ONIX_DOLL: u8 = 27;

pub const DECO_BIG_LAPRAS_DOLL: u8 = 28;

pub const DOLLS: u8 = 29;

pub const DECO_PIKACHU_DOLL: u8 = 30;

pub const DECO_SURF_PIKACHU_DOLL: u8 = 31;

pub const DECO_CLEFAIRY_DOLL: u8 = 32;

pub const DECO_JIGGLYPUFF_DOLL: u8 = 33;

pub const DECO_BULBASAUR_DOLL: u8 = 34;

pub const DECO_CHARMANDER_DOLL: u8 = 35;

pub const DECO_SQUIRTLE_DOLL: u8 = 36;

pub const DECO_POLIWAG_DOLL: u8 = 37;

pub const DECO_DIGLETT_DOLL: u8 = 38;

pub const DECO_STARYU_DOLL: u8 = 39;

pub const DECO_MAGIKARP_DOLL: u8 = 40;

pub const DECO_ODDISH_DOLL: u8 = 41;

pub const DECO_GENGAR_DOLL: u8 = 42;

pub const DECO_SHELLDER_DOLL: u8 = 43;

pub const DECO_GRIMER_DOLL: u8 = 44;

pub const DECO_VOLTORB_DOLL: u8 = 45;

pub const DECO_WEEDLE_DOLL: u8 = 46;

pub const DECO_UNOWN_DOLL: u8 = 47;

pub const DECO_GEODUDE_DOLL: u8 = 48;

pub const DECO_MACHOP_DOLL: u8 = 49;

pub const DECO_TENTACOOL_DOLL: u8 = 50;

pub const DECO_GOLD_TROPHY_DOLL: u8 = 51;

pub const DECO_SILVER_TROPHY_DOLL: u8 = 52;

pub const NUM_DECO_CATEGORIES: u8 = 7;

