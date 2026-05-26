use crate::runtime::Cpu;

pub const ITEMATTR_STRUCT_LENGTH: u8 = 7;

pub const CONST_VALUE: u8 = 1;

pub const CONST_INC: u8 = 1;

pub const ITEM: u8 = 1;

pub const KEY_ITEM: u8 = 2;

pub const BALL: u8 = 3;

pub const TM_HM: u8 = 4;

pub const NUM_ITEM_TYPES: u8 = 4;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const ITEMMENU_NOUSE: u8 = 0;

pub const ITEMMENU_CURRENT: u8 = 4;

pub const ITEMMENU_PARTY: u8 = 5;

pub const ITEMMENU_CLOSE: u8 = 6;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const CANT_SELECT: u8 = 64;

pub const CANT_SELECT_F: u8 = 6;

pub const CANT_TOSS: u8 = 128;

pub const CANT_TOSS_F: u8 = 7;

pub const NO_LIMITS: u8 = 0;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const ITEM_POCKET: u8 = 0;

pub const BALL_POCKET: u8 = 1;

pub const KEY_ITEM_POCKET: u8 = 2;

pub const TM_HM_POCKET: u8 = 3;

pub const NUM_POCKETS: u8 = 4;

pub const MAX_ITEMS: u8 = 20;

pub const MAX_BALLS: u8 = 12;

pub const MAX_KEY_ITEMS: u8 = 25;

pub const MAX_PC_ITEMS: u8 = 50;

pub const MAX_ITEM_STACK: u8 = 99;

pub const MAIL_LINE_LENGTH: u8 = 16;

pub const MAIL_MSG_LENGTH: u8 = 32;

pub const MAILBOX_CAPACITY: u8 = 10;

pub const MAIL_STRUCT_LENGTH: u8 = 47;

pub const MAIL_STRUCT_LENGTH_JP: u8 = 42;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const MAIL_LANG_ENGLISH: u8 = 0;

pub const MAIL_LANG_FRENCH: u8 = 1;

pub const MAIL_LANG_GERMAN: u8 = 2;

pub const MAIL_LANG_ITALIAN: u8 = 3;

pub const MAIL_LANG_SPANISH: u8 = 4;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const HELD_NONE: u8 = 0;

pub const HELD_BERRY: u8 = 1;

pub const HELD_2: u8 = 2;

pub const HELD_LEFTOVERS: u8 = 3;

pub const HELD_5: u8 = 5;

pub const HELD_RESTORE_PP: u8 = 6;

pub const HELD_CLEANSE_TAG: u8 = 8;

// duplicate assembler constant CONST_VALUE

pub const HELD_HEAL_POISON: u8 = 10;

pub const HELD_HEAL_FREEZE: u8 = 11;

pub const HELD_HEAL_BURN: u8 = 12;

pub const HELD_HEAL_SLEEP: u8 = 13;

pub const HELD_HEAL_PARALYZE: u8 = 14;

pub const HELD_HEAL_STATUS: u8 = 15;

pub const HELD_HEAL_CONFUSION: u8 = 16;

// duplicate assembler constant CONST_VALUE

pub const HELD_PREVENT_POISON: u8 = 20;

pub const HELD_PREVENT_BURN: u8 = 21;

pub const HELD_PREVENT_FREEZE: u8 = 22;

pub const HELD_PREVENT_SLEEP: u8 = 23;

pub const HELD_PREVENT_PARALYZE: u8 = 24;

pub const HELD_PREVENT_CONFUSE: u8 = 25;

// duplicate assembler constant CONST_VALUE

pub const HELD_30: u8 = 30;

pub const HELD_ATTACK_UP: u8 = 31;

pub const HELD_DEFENSE_UP: u8 = 32;

pub const HELD_SPEED_UP: u8 = 33;

pub const HELD_SP_ATTACK_UP: u8 = 34;

pub const HELD_SP_DEFENSE_UP: u8 = 35;

pub const HELD_ACCURACY_UP: u8 = 36;

pub const HELD_EVASION_UP: u8 = 37;

pub const HELD_38: u8 = 38;

// duplicate assembler constant CONST_VALUE

pub const HELD_METAL_POWDER: u8 = 42;

// duplicate assembler constant CONST_VALUE

pub const HELD_NORMAL_BOOST: u8 = 50;

pub const HELD_FIGHTING_BOOST: u8 = 51;

pub const HELD_FLYING_BOOST: u8 = 52;

pub const HELD_POISON_BOOST: u8 = 53;

pub const HELD_GROUND_BOOST: u8 = 54;

pub const HELD_ROCK_BOOST: u8 = 55;

pub const HELD_BUG_BOOST: u8 = 56;

pub const HELD_GHOST_BOOST: u8 = 57;

pub const HELD_FIRE_BOOST: u8 = 58;

pub const HELD_WATER_BOOST: u8 = 59;

pub const HELD_GRASS_BOOST: u8 = 60;

pub const HELD_ELECTRIC_BOOST: u8 = 61;

pub const HELD_PSYCHIC_BOOST: u8 = 62;

pub const HELD_ICE_BOOST: u8 = 63;

pub const HELD_DRAGON_BOOST: u8 = 64;

pub const HELD_DARK_BOOST: u8 = 65;

pub const HELD_STEEL_BOOST: u8 = 66;

// duplicate assembler constant CONST_VALUE

pub const HELD_CATCH_CHANCE: u8 = 70;

pub const HELD_71: u8 = 71;

pub const HELD_ESCAPE: u8 = 72;

pub const HELD_CRITICAL_UP: u8 = 73;

pub const HELD_QUICK_CLAW: u8 = 74;

pub const HELD_FLINCH: u8 = 75;

pub const HELD_AMULET_COIN: u8 = 76;

pub const HELD_BRIGHTPOWDER: u8 = 77;

pub const HELD_FOCUS_BAND: u8 = 79;

