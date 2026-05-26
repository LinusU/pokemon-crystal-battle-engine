use crate::runtime::Cpu;

pub const AUTO_INPUT: u8 = 255;

pub const CONST_VALUE: u8 = 0;

pub const CONST_INC: u8 = 1;

pub const DEBUG_BATTLE_F: u8 = 0;

pub const DEBUG_FIELD_F: u8 = 1;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const DEXMODE_NEW: u8 = 0;

pub const DEXMODE_OLD: u8 = 1;

pub const DEXMODE_ABC: u8 = 2;

pub const DEXMODE_UNOWN: u8 = 3;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const PARTYMON: u8 = 0;

pub const OTPARTYMON: u8 = 1;

pub const BOXMON: u8 = 2;

pub const TEMPMON: u8 = 3;

pub const WILDMON: u8 = 4;

pub const JUMPTABLE_INDEX_MASK: u8 = 127;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const JUMPTABLE_EXIT: u8 = 128;

pub const JUMPTABLE_EXIT_F: u8 = 7;

pub const GAME_TIMER_COUNTING_F: u8 = 0;

pub const GAME_TIMER_MOBILE_F: u8 = 7;

pub const JOYPAD_DISABLE_MON_FAINT_F: u8 = 6;

pub const JOYPAD_DISABLE_SGB_TRANSFER_F: u8 = 7;

pub const IN_BATTLE_TOWER_BATTLE_F: u8 = 0;

pub const TEXT_DELAY_MASK: u8 = 7;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const NO_TEXT_SCROLL: u8 = 4;

pub const STEREO: u8 = 5;

pub const BATTLE_SHIFT: u8 = 6;

pub const BATTLE_SCENE: u8 = 7;

pub const TEXT_DELAY_FAST: u8 = 1;

pub const TEXT_DELAY_MED: u8 = 3;

pub const TEXT_DELAY_SLOW: u8 = 5;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const FRAME_1: u8 = 0;

pub const FRAME_2: u8 = 1;

pub const FRAME_3: u8 = 2;

pub const FRAME_4: u8 = 3;

pub const FRAME_5: u8 = 4;

pub const FRAME_6: u8 = 5;

pub const FRAME_7: u8 = 6;

pub const FRAME_8: u8 = 7;

pub const NUM_FRAMES: u8 = 8;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const FAST_TEXT_DELAY_F: u8 = 0;

pub const TEXT_DELAY_F: u8 = 1;

pub const GBPRINTER_LIGHTEST: u8 = 0;

pub const GBPRINTER_LIGHTER: u8 = 32;

pub const GBPRINTER_NORMAL: u8 = 64;

pub const GBPRINTER_DARKER: u8 = 96;

pub const GBPRINTER_DARKEST: u8 = 127;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const MENU_ACCOUNT: u8 = 0;

pub const DST_F: u8 = 7;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const STANDING: i16 = -1;

pub const DOWN: u8 = 0;

pub const UP: u8 = 1;

pub const LEFT: u8 = 2;

pub const RIGHT: u8 = 3;

pub const NUM_DIRECTIONS: u8 = 4;

pub const DOWN_MASK: u8 = 1;

pub const UP_MASK: u8 = 2;

pub const LEFT_MASK: u8 = 4;

pub const RIGHT_MASK: u8 = 8;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const FACE_DOWN: u8 = 8;

pub const FACE_DOWN_F: u8 = 3;

pub const FACE_UP: u8 = 4;

pub const FACE_UP_F: u8 = 2;

pub const FACE_LEFT: u8 = 2;

pub const FACE_LEFT_F: u8 = 1;

pub const FACE_RIGHT: u8 = 1;

pub const FACE_RIGHT_F: u8 = 0;

pub const FACE_CURRENT: u8 = 0;

pub const SPRITE_UPDATES_DISABLED_F: u8 = 0;

pub const LAST_12_SPRITE_OAM_STRUCTS_RESERVED_F: u8 = 1;

pub const TEXT_STATE_F: u8 = 6;

pub const SCRIPTED_MOVEMENT_STATE_F: u8 = 7;

pub const SPRITES_VRAM_BANK_0_F: u8 = 5;

pub const SPRITES_SKIP_WALKING_GFX_F: u8 = 6;

pub const SPRITES_SKIP_STANDING_GFX_F: u8 = 7;

pub const PC_WITHDRAW: u8 = 0;

pub const PC_DEPOSIT: u8 = 1;

pub const REMOVE_PARTY: u8 = 0;

pub const REMOVE_BOX: u8 = 1;

pub const DAY_CARE_WITHDRAW: u8 = 2;

pub const DAY_CARE_DEPOSIT: u8 = 3;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const PLAYERSTEP_MIDAIR_F: u8 = 4;

pub const PLAYERSTEP_CONTINUE_F: u8 = 5;

pub const PLAYERSTEP_STOP_F: u8 = 6;

pub const PLAYERSTEP_START_F: u8 = 7;

pub const INIT_ENEMYOT_LIST: u8 = 1;

pub const INIT_BAG_ITEM_LIST: u8 = 2;

pub const INIT_OTHER_ITEM_LIST: u8 = 3;

pub const INIT_PLAYEROT_LIST: u8 = 4;

pub const INIT_MON_LIST: u8 = 5;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const SHOWN_MAP_NAME_SIGN: u8 = 1;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const MORN: u8 = 1;

pub const MORN_F: u8 = 0;

pub const DAY: u8 = 2;

pub const DAY_F: u8 = 1;

pub const NITE: u8 = 4;

pub const NITE_F: u8 = 2;

pub const DARKNESS: u8 = 8;

pub const DARKNESS_F: u8 = 3;

pub const NUM_DAYTIMES: u8 = 4;

pub const ANYTIME: u8 = 7;

pub const FORCED_PALSET_F: u8 = 7;

pub const DARKNESS_PALSET: u8 = 255;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const BATTLEANIM_STOP_F: u8 = 0;

pub const BATTLEANIM_IN_SUBROUTINE_F: u8 = 1;

pub const BATTLEANIM_IN_LOOP_F: u8 = 2;

pub const BATTLEANIM_KEEPSPRITES_F: u8 = 3;

pub const BATTLESCRIPT_WILD_F: u8 = 0;

pub const BATTLESCRIPT_SCRIPTED_F: u8 = 7;

pub const PLAYERSPRITESETUP_FACING_MASK: u8 = 3;

pub const PLAYERSPRITESETUP_FEMALE_TO_MALE_F: u8 = 2;

pub const PLAYERSPRITESETUP_CUSTOM_FACING_F: u8 = 5;

pub const PLAYERSPRITESETUP_SKIP_RELOAD_GFX_F: u8 = 6;

pub const PLAYERSPRITESETUP_RESET_ACTION_F: u8 = 7;

pub const PLAYERGENDER_FEMALE_F: u8 = 0;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const MAPSTATUS_START: u8 = 0;

pub const MAPSTATUS_ENTER: u8 = 1;

pub const MAPSTATUS_HANDLE: u8 = 2;

pub const MAPSTATUS_DONE: u8 = 3;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const MAPEVENTS_ON: u8 = 0;

pub const MAPEVENTS_OFF: u8 = 1;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const UNUSED_SCRIPT_FLAG_0: u8 = 0;

pub const UNUSED_SCRIPT_FLAG_1: u8 = 1;

pub const SCRIPT_RUNNING: u8 = 2;

pub const RUN_DEFERRED_SCRIPT: u8 = 3;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const PLAYEREVENTS_COUNT_STEPS: u8 = 0;

pub const PLAYEREVENTS_COORD_EVENTS: u8 = 1;

pub const PLAYEREVENTS_WARPS_AND_CONNECTIONS: u8 = 2;

pub const PLAYEREVENTS_WILD_ENCOUNTERS: u8 = 4;

pub const PLAYEREVENTS_UNUSED: u8 = 5;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const SCRIPT_OFF: u8 = 0;

pub const SCRIPT_READ: u8 = 1;

pub const SCRIPT_WAIT_MOVEMENT: u8 = 2;

pub const SCRIPT_WAIT: u8 = 3;

pub const SPAWN_LANCE: u8 = 1;

pub const SPAWN_RED: u8 = 2;

pub const GAME_TIME_CAPPED: u8 = 0;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const SUNDAY: u8 = 0;

pub const MONDAY: u8 = 1;

pub const TUESDAY: u8 = 2;

pub const WEDNESDAY: u8 = 3;

pub const THURSDAY: u8 = 4;

pub const FRIDAY: u8 = 5;

pub const SATURDAY: u8 = 6;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const STATUSFLAGS_POKEDEX_F: u8 = 0;

pub const STATUSFLAGS_UNOWN_DEX_F: u8 = 1;

pub const STATUSFLAGS_FLASH_F: u8 = 2;

pub const STATUSFLAGS_CAUGHT_POKERUS_F: u8 = 3;

pub const STATUSFLAGS_ROCKET_SIGNAL_F: u8 = 4;

pub const STATUSFLAGS_NO_WILD_ENCOUNTERS_F: u8 = 5;

pub const STATUSFLAGS_HALL_OF_FAME_F: u8 = 6;

pub const STATUSFLAGS_MAIN_MENU_MOBILE_CHOICES_F: u8 = 7;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const STATUSFLAGS2_ROCKETS_IN_RADIO_TOWER_F: u8 = 0;

pub const STATUSFLAGS2_SAFARI_GAME_F: u8 = 1;

pub const STATUSFLAGS2_BUG_CONTEST_TIMER_F: u8 = 2;

pub const STATUSFLAGS2_UNUSED_3_F: u8 = 3;

pub const STATUSFLAGS2_BIKE_SHOP_CALL_F: u8 = 4;

pub const STATUSFLAGS2_UNUSED_5_F: u8 = 5;

pub const STATUSFLAGS2_REACHED_GOLDENROD_F: u8 = 6;

pub const STATUSFLAGS2_ROCKETS_IN_MAHOGANY_F: u8 = 7;

pub const MOM_SAVING_SOME_MONEY_F: u8 = 0;

pub const MOM_SAVING_HALF_MONEY_F: u8 = 1;

pub const MOM_SAVING_ALL_MONEY_F: u8 = 2;

pub const MOM_ACTIVE_F: u8 = 7;

pub const MOM_SAVING_MONEY_MASK: u8 = 7;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const ZEPHYRBADGE: u8 = 0;

pub const HIVEBADGE: u8 = 1;

pub const PLAINBADGE: u8 = 2;

pub const FOGBADGE: u8 = 3;

pub const MINERALBADGE: u8 = 4;

pub const STORMBADGE: u8 = 5;

pub const GLACIERBADGE: u8 = 6;

pub const RISINGBADGE: u8 = 7;

pub const NUM_JOHTO_BADGES: u8 = 8;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const BOULDERBADGE: u8 = 0;

pub const CASCADEBADGE: u8 = 1;

pub const THUNDERBADGE: u8 = 2;

pub const RAINBOWBADGE: u8 = 3;

pub const SOULBADGE: u8 = 4;

pub const MARSHBADGE: u8 = 5;

pub const VOLCANOBADGE: u8 = 6;

pub const EARTHBADGE: u8 = 7;

pub const NUM_KANTO_BADGES: u8 = 8;

pub const NUM_BADGES: u8 = 16;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const POKEGEAR_MAP_CARD_F: u8 = 0;

pub const POKEGEAR_RADIO_CARD_F: u8 = 1;

pub const POKEGEAR_PHONE_CARD_F: u8 = 2;

pub const POKEGEAR_EXPN_CARD_F: u8 = 3;

pub const POKEGEAR_OBTAINED_F: u8 = 7;

pub const REGISTERED_POCKET: u8 = 192;

pub const REGISTERED_NUMBER: u8 = 63;

pub const PLAYER_NORMAL: u8 = 0;

pub const PLAYER_BIKE: u8 = 1;

pub const PLAYER_SKATE: u8 = 2;

pub const PLAYER_SURF: u8 = 4;

pub const PLAYER_SURF_PIKA: u8 = 8;

pub const CELEBIEVENT_FOREST_IS_RESTLESS_F: u8 = 2;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const BIKEFLAGS_STRENGTH_ACTIVE_F: u8 = 0;

pub const BIKEFLAGS_ALWAYS_ON_BIKE_F: u8 = 1;

pub const BIKEFLAGS_DOWNHILL_F: u8 = 2;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const DAILYFLAGS1_KURT_MAKING_BALLS_F: u8 = 0;

pub const DAILYFLAGS1_BUG_CONTEST_F: u8 = 1;

pub const DAILYFLAGS1_FISH_SWARM_F: u8 = 2;

pub const DAILYFLAGS1_TIME_CAPSULE_F: u8 = 3;

pub const DAILYFLAGS1_ALL_FRUIT_TREES_F: u8 = 4;

pub const DAILYFLAGS1_GOT_SHUCKIE_TODAY_F: u8 = 5;

pub const DAILYFLAGS1_GOLDENROD_UNDERGROUND_BARGAIN_F: u8 = 6;

pub const DAILYFLAGS1_TRAINER_HOUSE_F: u8 = 7;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const DAILYFLAGS2_MT_MOON_SQUARE_CLEFAIRY_F: u8 = 0;

pub const DAILYFLAGS2_UNION_CAVE_LAPRAS_F: u8 = 1;

pub const DAILYFLAGS2_GOLDENROD_UNDERGROUND_GOT_HAIRCUT_F: u8 = 2;

pub const DAILYFLAGS2_GOLDENROD_DEPT_STORE_TM27_RETURN_F: u8 = 3;

pub const DAILYFLAGS2_DAISYS_GROOMING_F: u8 = 4;

pub const DAILYFLAGS2_INDIGO_PLATEAU_RIVAL_FIGHT_F: u8 = 5;

pub const DAILYFLAGS2_MOVE_TUTOR_F: u8 = 6;

pub const DAILYFLAGS2_BUENAS_PASSWORD_F: u8 = 7;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const SWARMFLAGS_BUENAS_PASSWORD_F: u8 = 0;

pub const SWARMFLAGS_GOLDENROD_DEPT_STORE_SALE_F: u8 = 1;

pub const SWARMFLAGS_DUNSPARCE_SWARM_F: u8 = 2;

pub const SWARMFLAGS_YANMA_SWARM_F: u8 = 3;

pub const SWARMFLAGS_MOBILE_4_F: u8 = 4;

pub const LUCKYNUMBERSHOW_GAME_OVER_F: u8 = 0;

pub const DAYCAREMAN_HAS_MON_F: u8 = 0;

pub const DAYCAREMAN_MONS_COMPATIBLE_F: u8 = 5;

pub const DAYCAREMAN_HAS_EGG_F: u8 = 6;

pub const DAYCAREMAN_ACTIVE_F: u8 = 7;

pub const DAYCARELADY_HAS_MON_F: u8 = 0;

pub const DAYCARELADY_ACTIVE_F: u8 = 7;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const UNLOCKED_UNOWNS_A_TO_K_F: u8 = 0;

pub const UNLOCKED_UNOWNS_L_TO_R_F: u8 = 1;

pub const UNLOCKED_UNOWNS_S_TO_W_F: u8 = 2;

pub const UNLOCKED_UNOWNS_X_TO_Z_F: u8 = 3;

pub const NUM_UNLOCKED_UNOWN_SETS: u8 = 4;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const RTC_DAYS_EXCEED_139: u8 = 32;

pub const RTC_DAYS_EXCEED_139_F: u8 = 5;

pub const RTC_DAYS_EXCEED_255: u8 = 64;

pub const RTC_DAYS_EXCEED_255_F: u8 = 6;

pub const RTC_RESET: u8 = 128;

pub const RTC_RESET_F: u8 = 7;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const VBLANK_NORMAL: u8 = 0;

pub const VBLANK_CUTSCENE: u8 = 1;

pub const VBLANK_SOUND_ONLY: u8 = 2;

pub const VBLANK_CUTSCENE_CGB: u8 = 3;

pub const VBLANK_SERIAL: u8 = 4;

pub const VBLANK_CREDITS: u8 = 5;

pub const VBLANK_DMA_TRANSFER: u8 = 6;

pub const VBLANK_UNUSED: u8 = 7;

pub const NUM_VBLANK_HANDLERS: u8 = 8;

pub const NUM_SRAM_BANKS: u8 = 4;

pub const LCDC_DEFAULT: u8 = 227;

pub const IE_DEFAULT: u8 = 15;

