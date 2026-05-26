use crate::runtime::Cpu;

pub const CONST_VALUE: u8 = 0;

pub const CONST_INC: u8 = 1;

pub const SCGB_BATTLE_GRAYSCALE: u8 = 0;

pub const SCGB_BATTLE_COLORS: u8 = 1;

pub const SCGB_POKEGEAR_PALS: u8 = 2;

pub const SCGB_STATS_SCREEN_HP_PALS: u8 = 3;

pub const SCGB_POKEDEX: u8 = 4;

pub const SCGB_SLOT_MACHINE: u8 = 5;

pub const SCGB_BETA_TITLE_SCREEN: u8 = 6;

pub const SCGB_GS_INTRO: u8 = 7;

pub const SCGB_DIPLOMA: u8 = 8;

pub const SCGB_MAPPALS: u8 = 9;

pub const SCGB_PARTY_MENU: u8 = 10;

pub const SCGB_EVOLUTION: u8 = 11;

pub const SCGB_GS_TITLE_SCREEN: u8 = 12;

pub const SCGB_0D: u8 = 13;

pub const SCGB_MOVE_LIST: u8 = 14;

pub const SCGB_BETA_PIKACHU_MINIGAME: u8 = 15;

pub const SCGB_POKEDEX_SEARCH_OPTION: u8 = 16;

pub const SCGB_BETA_POKER: u8 = 17;

pub const SCGB_POKEPIC: u8 = 18;

pub const SCGB_MAGNET_TRAIN: u8 = 19;

pub const SCGB_PACKPALS: u8 = 20;

pub const SCGB_TRAINER_CARD: u8 = 21;

pub const SCGB_POKEDEX_UNOWN_MODE: u8 = 22;

pub const SCGB_BILLS_PC: u8 = 23;

pub const SCGB_UNOWN_PUZZLE: u8 = 24;

pub const SCGB_GAMEFREAK_LOGO: u8 = 25;

pub const SCGB_PLAYER_OR_MON_FRONTPIC_PALS: u8 = 26;

pub const SCGB_TRADE_TUBE: u8 = 27;

pub const SCGB_TRAINER_OR_MON_FRONTPIC_PALS: u8 = 28;

pub const SCGB_MYSTERY_GIFT: u8 = 29;

pub const SCGB_1E: u8 = 30;

pub const NUM_SCGB_LAYOUTS: u8 = 31;

pub const SCGB_PARTY_MENU_HP_BARS: u8 = 252;

pub const SCGB_DEFAULT: u8 = 255;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const CRYSTAL_CGB_MOBILE_0: u8 = 0;

pub const CRYSTAL_CGB_MOBILE_1: u8 = 1;

pub const CRYSTAL_CGB_NAME_CARD: u8 = 2;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const PREDEFPAL_ROUTES: u8 = 0;

pub const PREDEFPAL_PALLET: u8 = 1;

pub const PREDEFPAL_VIRIDIAN: u8 = 2;

pub const PREDEFPAL_PEWTER: u8 = 3;

pub const PREDEFPAL_CERULEAN: u8 = 4;

pub const PREDEFPAL_LAVENDER: u8 = 5;

pub const PREDEFPAL_VERMILION: u8 = 6;

pub const PREDEFPAL_CELADON: u8 = 7;

pub const PREDEFPAL_FUCHSIA: u8 = 8;

pub const PREDEFPAL_CINNABAR: u8 = 9;

pub const PREDEFPAL_SAFFRON: u8 = 10;

pub const PREDEFPAL_INDIGO: u8 = 11;

pub const PREDEFPAL_NEW_BARK: u8 = 12;

pub const PREDEFPAL_CHERRYGROVE: u8 = 13;

pub const PREDEFPAL_VIOLET: u8 = 14;

pub const PREDEFPAL_AZALEA: u8 = 15;

pub const PREDEFPAL_GOLDENROD: u8 = 16;

pub const PREDEFPAL_ECRUTEAK: u8 = 17;

pub const PREDEFPAL_OLIVINE: u8 = 18;

pub const PREDEFPAL_CIANWOOD: u8 = 19;

pub const PREDEFPAL_MAHOGANY: u8 = 20;

pub const PREDEFPAL_BLACKTHORN: u8 = 21;

pub const PREDEFPAL_LAKE_OF_RAGE: u8 = 22;

pub const PREDEFPAL_SILVER_CAVE: u8 = 23;

pub const PREDEFPAL_DUNGEONS: u8 = 24;

pub const PREDEFPAL_NITE: u8 = 25;

pub const PREDEFPAL_BLACKOUT: u8 = 26;

pub const PREDEFPAL_DIPLOMA: u8 = 27;

pub const PREDEFPAL_TRADE_TUBE: u8 = 28;

pub const PREDEFPAL_POKEDEX: u8 = 29;

pub const PREDEFPAL_RB_CYANMON: u8 = 30;

pub const PREDEFPAL_RB_PURPLEMON: u8 = 31;

pub const PREDEFPAL_RB_BROWNMON: u8 = 32;

pub const PREDEFPAL_RB_GREENMON: u8 = 33;

pub const PREDEFPAL_RB_PINKMON: u8 = 34;

pub const PREDEFPAL_RB_YELLOWMON: u8 = 35;

pub const PREDEFPAL_CGB_BADGE: u8 = 36;

pub const PREDEFPAL_BETA_SHINY_MEWMON: u8 = 37;

pub const PREDEFPAL_BETA_SHINY_BLUEMON: u8 = 38;

pub const PREDEFPAL_BETA_SHINY_REDMON: u8 = 39;

pub const PREDEFPAL_BETA_SHINY_CYANMON: u8 = 40;

pub const PREDEFPAL_BETA_SHINY_PURPLEMON: u8 = 41;

pub const PREDEFPAL_BETA_SHINY_BROWNMON: u8 = 42;

pub const PREDEFPAL_BETA_SHINY_GREENMON: u8 = 43;

pub const PREDEFPAL_BETA_SHINY_PINKMON: u8 = 44;

pub const PREDEFPAL_BETA_SHINY_YELLOWMON: u8 = 45;

pub const PREDEFPAL_PARTY_ICON: u8 = 46;

pub const PREDEFPAL_HP_GREEN: u8 = 47;

pub const PREDEFPAL_HP_YELLOW: u8 = 48;

pub const PREDEFPAL_HP_RED: u8 = 49;

pub const PREDEFPAL_POKEGEAR: u8 = 50;

pub const PREDEFPAL_BETA_LOGO_1: u8 = 51;

pub const PREDEFPAL_BETA_LOGO_2: u8 = 52;

pub const PREDEFPAL_GS_INTRO_GAMEFREAK_LOGO: u8 = 53;

pub const PREDEFPAL_GS_INTRO_SHELLDER_LAPRAS: u8 = 54;

pub const PREDEFPAL_BETA_INTRO_LAPRAS: u8 = 55;

pub const PREDEFPAL_GS_INTRO_JIGGLYPUFF_PIKACHU_BG: u8 = 56;

pub const PREDEFPAL_GS_INTRO_JIGGLYPUFF_PIKACHU_OB: u8 = 57;

pub const PREDEFPAL_GS_INTRO_STARTERS_TRANSITION: u8 = 58;

pub const PREDEFPAL_BETA_INTRO_VENUSAUR: u8 = 59;

pub const PREDEFPAL_PACK: u8 = 60;

pub const PREDEFPAL_SLOT_MACHINE_0: u8 = 61;

pub const PREDEFPAL_SLOT_MACHINE_1: u8 = 62;

pub const PREDEFPAL_SLOT_MACHINE_2: u8 = 63;

pub const PREDEFPAL_SLOT_MACHINE_3: u8 = 64;

pub const PREDEFPAL_BETA_POKER_0: u8 = 65;

pub const PREDEFPAL_BETA_POKER_1: u8 = 66;

pub const PREDEFPAL_BETA_POKER_2: u8 = 67;

pub const PREDEFPAL_BETA_POKER_3: u8 = 68;

pub const PREDEFPAL_BETA_RADIO: u8 = 69;

pub const PREDEFPAL_BETA_POKEGEAR: u8 = 70;

pub const PREDEFPAL_47: u8 = 71;

pub const PREDEFPAL_GS_TITLE_SCREEN_0: u8 = 72;

pub const PREDEFPAL_GS_TITLE_SCREEN_1: u8 = 73;

pub const PREDEFPAL_GS_TITLE_SCREEN_2: u8 = 74;

pub const PREDEFPAL_GS_TITLE_SCREEN_3: u8 = 75;

pub const PREDEFPAL_UNOWN_PUZZLE: u8 = 76;

pub const PREDEFPAL_GAMEFREAK_LOGO_OB: u8 = 77;

pub const PREDEFPAL_GAMEFREAK_LOGO_BG: u8 = 78;

pub const NUM_PREDEF_PALS: u8 = 79;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const SGB_PAL01: u8 = 0;

pub const SGB_PAL23: u8 = 1;

pub const SGB_PAL03: u8 = 2;

pub const SGB_PAL12: u8 = 3;

pub const SGB_ATTR_BLK: u8 = 4;

pub const SGB_ATTR_LIN: u8 = 5;

pub const SGB_ATTR_DIV: u8 = 6;

pub const SGB_ATTR_CHR: u8 = 7;

pub const SGB_SOUND: u8 = 8;

pub const SGB_SOU_TRN: u8 = 9;

pub const SGB_PAL_SET: u8 = 10;

pub const SGB_PAL_TRN: u8 = 11;

pub const SGB_ATRC_EN: u8 = 12;

pub const SGB_TEST_EN: u8 = 13;

pub const SGB_ICON_EN: u8 = 14;

pub const SGB_DATA_SND: u8 = 15;

pub const SGB_DATA_TRN: u8 = 16;

pub const SGB_MLT_REQ: u8 = 17;

pub const SGB_JUMP: u8 = 18;

pub const SGB_CHR_TRN: u8 = 19;

pub const SGB_PCT_TRN: u8 = 20;

pub const SGB_ATTR_TRN: u8 = 21;

pub const SGB_ATTR_SET: u8 = 22;

pub const SGB_MASK_EN: u8 = 23;

pub const SGB_OBJ_TRN: u8 = 24;

pub const PALPACKET_LENGTH: u8 = 16;

