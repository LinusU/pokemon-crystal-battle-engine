use crate::runtime::Cpu;

pub const CONST_VALUE: u8 = 0;

pub const CONST_INC: u8 = 1;

pub const SFX_DEX_FANFARE_50_79: u8 = 0;

pub const SFX_ITEM: u8 = 1;

pub const SFX_CAUGHT_MON: u8 = 2;

pub const SFX_POKEBALLS_PLACED_ON_TABLE: u8 = 3;

pub const SFX_POTION: u8 = 4;

pub const SFX_FULL_HEAL: u8 = 5;

pub const SFX_MENU: u8 = 6;

pub const SFX_READ_TEXT: u8 = 7;

pub const SFX_READ_TEXT_2: u8 = 8;

pub const SFX_DEX_FANFARE_20_49: u8 = 9;

pub const SFX_DEX_FANFARE_80_109: u8 = 10;

pub const SFX_POISON: u8 = 11;

pub const SFX_GOT_SAFARI_BALLS: u8 = 12;

pub const SFX_BOOT_PC: u8 = 13;

pub const SFX_SHUT_DOWN_PC: u8 = 14;

pub const SFX_CHOOSE_PC_OPTION: u8 = 15;

pub const SFX_ESCAPE_ROPE: u8 = 16;

pub const SFX_PUSH_BUTTON: u8 = 17;

pub const SFX_SECOND_PART_OF_ITEMFINDER: u8 = 18;

pub const SFX_WARP_TO: u8 = 19;

pub const SFX_WARP_FROM: u8 = 20;

pub const SFX_CHANGE_DEX_MODE: u8 = 21;

pub const SFX_JUMP_OVER_LEDGE: u8 = 22;

pub const SFX_GRASS_RUSTLE: u8 = 23;

pub const SFX_FLY: u8 = 24;

pub const SFX_WRONG: u8 = 25;

pub const SFX_SQUEAK: u8 = 26;

pub const SFX_STRENGTH: u8 = 27;

pub const SFX_BOAT: u8 = 28;

pub const SFX_WALL_OPEN: u8 = 29;

pub const SFX_PLACE_PUZZLE_PIECE_DOWN: u8 = 30;

pub const SFX_ENTER_DOOR: u8 = 31;

pub const SFX_SWITCH_POKEMON: u8 = 32;

pub const SFX_TALLY: u8 = 33;

pub const SFX_TRANSACTION: u8 = 34;

pub const SFX_EXIT_BUILDING: u8 = 35;

pub const SFX_BUMP: u8 = 36;

pub const SFX_SAVE: u8 = 37;

pub const SFX_POKEFLUTE: u8 = 38;

pub const SFX_ELEVATOR_END: u8 = 39;

pub const SFX_THROW_BALL: u8 = 40;

pub const SFX_BALL_POOF: u8 = 41;

pub const SFX_FAINT: u8 = 42;

pub const SFX_RUN: u8 = 43;

pub const SFX_SLOT_MACHINE_START: u8 = 44;

pub const SFX_FANFARE: u8 = 45;

pub const SFX_PECK: u8 = 46;

pub const SFX_KINESIS: u8 = 47;

pub const SFX_LICK: u8 = 48;

pub const SFX_POUND: u8 = 49;

pub const SFX_MOVE_PUZZLE_PIECE: u8 = 50;

pub const SFX_COMET_PUNCH: u8 = 51;

pub const SFX_MEGA_PUNCH: u8 = 52;

pub const SFX_SCRATCH: u8 = 53;

pub const SFX_VICEGRIP: u8 = 54;

pub const SFX_RAZOR_WIND: u8 = 55;

pub const SFX_CUT: u8 = 56;

pub const SFX_WING_ATTACK: u8 = 57;

pub const SFX_WHIRLWIND: u8 = 58;

pub const SFX_BIND: u8 = 59;

pub const SFX_VINE_WHIP: u8 = 60;

pub const SFX_DOUBLE_KICK: u8 = 61;

pub const SFX_MEGA_KICK: u8 = 62;

pub const SFX_HEADBUTT: u8 = 63;

pub const SFX_HORN_ATTACK: u8 = 64;

pub const SFX_TACKLE: u8 = 65;

pub const SFX_POISON_STING: u8 = 66;

pub const SFX_POWDER: u8 = 67;

pub const SFX_DOUBLESLAP: u8 = 68;

pub const SFX_BITE: u8 = 69;

pub const SFX_JUMP_KICK: u8 = 70;

pub const SFX_STOMP: u8 = 71;

pub const SFX_TAIL_WHIP: u8 = 72;

pub const SFX_KARATE_CHOP: u8 = 73;

pub const SFX_SUBMISSION: u8 = 74;

pub const SFX_WATER_GUN: u8 = 75;

pub const SFX_SWORDS_DANCE: u8 = 76;

pub const SFX_THUNDER: u8 = 77;

pub const SFX_SUPERSONIC: u8 = 78;

pub const SFX_LEER: u8 = 79;

pub const SFX_EMBER: u8 = 80;

pub const SFX_BUBBLEBEAM: u8 = 81;

pub const SFX_HYDRO_PUMP: u8 = 82;

pub const SFX_SURF: u8 = 83;

pub const SFX_PSYBEAM: u8 = 84;

pub const SFX_CHARGE: u8 = 85;

pub const SFX_THUNDERSHOCK: u8 = 86;

pub const SFX_PSYCHIC: u8 = 87;

pub const SFX_SCREECH: u8 = 88;

pub const SFX_BONE_CLUB: u8 = 89;

pub const SFX_SHARPEN: u8 = 90;

pub const SFX_EGG_BOMB: u8 = 91;

pub const SFX_SING: u8 = 92;

pub const SFX_HYPER_BEAM: u8 = 93;

pub const SFX_SHINE: u8 = 94;

pub const SFX_UNKNOWN_5F: u8 = 95;

pub const SFX_UNKNOWN_60: u8 = 96;

pub const SFX_UNKNOWN_61: u8 = 97;

pub const SFX_SWITCH_POCKETS: u8 = 98;

pub const SFX_UNKNOWN_63: u8 = 99;

pub const SFX_BURN: u8 = 100;

pub const SFX_TITLE_SCREEN_ENTRANCE: u8 = 101;

pub const SFX_UNKNOWN_66: u8 = 102;

pub const SFX_GET_COIN_FROM_SLOTS: u8 = 103;

pub const SFX_PAY_DAY: u8 = 104;

pub const SFX_METRONOME: u8 = 105;

pub const SFX_CALL: u8 = 106;

pub const SFX_HANG_UP: u8 = 107;

pub const SFX_NO_SIGNAL: u8 = 108;

pub const SFX_SANDSTORM: u8 = 109;

pub const SFX_ELEVATOR: u8 = 110;

pub const SFX_PROTECT: u8 = 111;

pub const SFX_SKETCH: u8 = 112;

pub const SFX_RAIN_DANCE: u8 = 113;

pub const SFX_AEROBLAST: u8 = 114;

pub const SFX_SPARK: u8 = 115;

pub const SFX_CURSE: u8 = 116;

pub const SFX_RAGE: u8 = 117;

pub const SFX_THIEF: u8 = 118;

pub const SFX_THIEF_2: u8 = 119;

pub const SFX_SPIDER_WEB: u8 = 120;

pub const SFX_MIND_READER: u8 = 121;

pub const SFX_NIGHTMARE: u8 = 122;

pub const SFX_SNORE: u8 = 123;

pub const SFX_SWEET_KISS: u8 = 124;

pub const SFX_SWEET_KISS_2: u8 = 125;

pub const SFX_BELLY_DRUM: u8 = 126;

pub const SFX_TOXIC: u8 = 127;

pub const SFX_SLUDGE_BOMB: u8 = 128;

pub const SFX_FORESIGHT: u8 = 129;

pub const SFX_SPITE: u8 = 130;

pub const SFX_OUTRAGE: u8 = 131;

pub const SFX_PERISH_SONG: u8 = 132;

pub const SFX_GIGA_DRAIN: u8 = 133;

pub const SFX_ATTRACT: u8 = 134;

pub const SFX_KINESIS_2: u8 = 135;

pub const SFX_ZAP_CANNON: u8 = 136;

pub const SFX_MEAN_LOOK: u8 = 137;

pub const SFX_HEAL_BELL: u8 = 138;

pub const SFX_RETURN: u8 = 139;

pub const SFX_EXP_BAR: u8 = 140;

pub const SFX_MILK_DRINK: u8 = 141;

pub const SFX_PRESENT: u8 = 142;

pub const SFX_MORNING_SUN: u8 = 143;

pub const SFX_LEVEL_UP: u8 = 144;

pub const SFX_KEY_ITEM: u8 = 145;

pub const SFX_FANFARE_2: u8 = 146;

pub const SFX_REGISTER_PHONE_NUMBER: u8 = 147;

pub const SFX_3RD_PLACE: u8 = 148;

pub const SFX_GET_EGG_UNUSED: u8 = 149;

pub const SFX_GET_EGG: u8 = 150;

pub const SFX_MOVE_DELETED: u8 = 151;

pub const SFX_2ND_PLACE: u8 = 152;

pub const SFX_1ST_PLACE: u8 = 153;

pub const SFX_CHOOSE_A_CARD: u8 = 154;

pub const SFX_GET_TM: u8 = 155;

pub const SFX_GET_BADGE: u8 = 156;

pub const SFX_QUIT_SLOTS: u8 = 157;

pub const SFX_EGG_CRACK: u8 = 158;

pub const SFX_DEX_FANFARE_LESS_THAN_20: u8 = 159;

pub const SFX_DEX_FANFARE_140_169: u8 = 160;

pub const SFX_DEX_FANFARE_170_199: u8 = 161;

pub const SFX_DEX_FANFARE_200_229: u8 = 162;

pub const SFX_DEX_FANFARE_230_PLUS: u8 = 163;

pub const SFX_EVOLVED: u8 = 164;

pub const SFX_MASTER_BALL: u8 = 165;

pub const SFX_EGG_HATCH: u8 = 166;

pub const SFX_GS_INTRO_CHARIZARD_FIREBALL: u8 = 167;

pub const SFX_GS_INTRO_POKEMON_APPEARS: u8 = 168;

pub const SFX_FLASH: u8 = 169;

pub const SFX_GAME_FREAK_LOGO_GS: u8 = 170;

pub const SFX_NOT_VERY_EFFECTIVE: u8 = 171;

pub const SFX_DAMAGE: u8 = 172;

pub const SFX_SUPER_EFFECTIVE: u8 = 173;

pub const SFX_BALL_BOUNCE: u8 = 174;

pub const SFX_MOONLIGHT: u8 = 175;

pub const SFX_ENCORE: u8 = 176;

pub const SFX_BEAT_UP: u8 = 177;

pub const SFX_BATON_PASS: u8 = 178;

pub const SFX_BALL_WOBBLE: u8 = 179;

pub const SFX_SWEET_SCENT: u8 = 180;

pub const SFX_SWEET_SCENT_2: u8 = 181;

pub const SFX_HIT_END_OF_EXP_BAR: u8 = 182;

pub const SFX_GIVE_TRADEMON: u8 = 183;

pub const SFX_GET_TRADEMON: u8 = 184;

pub const SFX_TRAIN_ARRIVED: u8 = 185;

pub const SFX_STOP_SLOT: u8 = 186;

pub const SFX_2_BOOPS: u8 = 187;

pub const SFX_GLASS_TING: u8 = 188;

pub const SFX_GLASS_TING_2: u8 = 189;

pub const SFX_INTRO_UNOWN_1: u8 = 190;

pub const SFX_INTRO_UNOWN_2: u8 = 191;

pub const SFX_INTRO_UNOWN_3: u8 = 192;

pub const SFX_DITTO_POP_UP: u8 = 193;

pub const SFX_DITTO_TRANSFORM: u8 = 194;

pub const SFX_INTRO_SUICUNE_1: u8 = 195;

pub const SFX_INTRO_PICHU: u8 = 196;

pub const SFX_INTRO_SUICUNE_2: u8 = 197;

pub const SFX_INTRO_SUICUNE_3: u8 = 198;

pub const SFX_DITTO_BOUNCE: u8 = 199;

pub const SFX_INTRO_SUICUNE_4: u8 = 200;

pub const SFX_GAME_FREAK_PRESENTS: u8 = 201;

pub const SFX_TINGLE: u8 = 202;

pub const SFX_INTRO_WHOOSH: u8 = 203;

pub const SFX_TWO_PC_BEEPS: u8 = 204;

pub const SFX_4_NOTE_DITTY: u8 = 205;

pub const SFX_TWINKLE: u8 = 206;

pub const NUM_SFX: u8 = 207;

