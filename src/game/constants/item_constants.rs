use crate::runtime::Cpu;

pub const CONST_VALUE: u8 = 0;

pub const CONST_INC: u8 = 1;

pub const NO_ITEM: u8 = 0;

pub const MASTER_BALL: u8 = 1;

pub const ULTRA_BALL: u8 = 2;

pub const BRIGHTPOWDER: u8 = 3;

pub const GREAT_BALL: u8 = 4;

pub const POKE_BALL: u8 = 5;

pub const TOWN_MAP: u8 = 6;

pub const BICYCLE: u8 = 7;

pub const MOON_STONE: u8 = 8;

pub const ANTIDOTE: u8 = 9;

pub const BURN_HEAL: u8 = 10;

pub const ICE_HEAL: u8 = 11;

pub const AWAKENING: u8 = 12;

pub const PARLYZ_HEAL: u8 = 13;

pub const FULL_RESTORE: u8 = 14;

pub const MAX_POTION: u8 = 15;

pub const HYPER_POTION: u8 = 16;

pub const SUPER_POTION: u8 = 17;

pub const POTION: u8 = 18;

pub const ESCAPE_ROPE: u8 = 19;

pub const REPEL: u8 = 20;

pub const MAX_ELIXER: u8 = 21;

pub const FIRE_STONE: u8 = 22;

pub const THUNDERSTONE: u8 = 23;

pub const WATER_STONE: u8 = 24;

pub const ITEM_19: u8 = 25;

pub const HP_UP: u8 = 26;

pub const PROTEIN: u8 = 27;

pub const IRON: u8 = 28;

pub const CARBOS: u8 = 29;

pub const LUCKY_PUNCH: u8 = 30;

pub const CALCIUM: u8 = 31;

pub const RARE_CANDY: u8 = 32;

pub const X_ACCURACY: u8 = 33;

pub const LEAF_STONE: u8 = 34;

pub const METAL_POWDER: u8 = 35;

pub const NUGGET: u8 = 36;

pub const POKE_DOLL: u8 = 37;

pub const FULL_HEAL: u8 = 38;

pub const REVIVE: u8 = 39;

pub const MAX_REVIVE: u8 = 40;

pub const GUARD_SPEC: u8 = 41;

pub const SUPER_REPEL: u8 = 42;

pub const MAX_REPEL: u8 = 43;

pub const DIRE_HIT: u8 = 44;

pub const ITEM_2D: u8 = 45;

pub const FRESH_WATER: u8 = 46;

pub const SODA_POP: u8 = 47;

pub const LEMONADE: u8 = 48;

pub const X_ATTACK: u8 = 49;

pub const ITEM_32: u8 = 50;

pub const X_DEFEND: u8 = 51;

pub const X_SPEED: u8 = 52;

pub const X_SPECIAL: u8 = 53;

pub const COIN_CASE: u8 = 54;

pub const ITEMFINDER: u8 = 55;

pub const POKE_FLUTE: u8 = 56;

pub const EXP_SHARE: u8 = 57;

pub const OLD_ROD: u8 = 58;

pub const GOOD_ROD: u8 = 59;

pub const SILVER_LEAF: u8 = 60;

pub const SUPER_ROD: u8 = 61;

pub const PP_UP: u8 = 62;

pub const ETHER: u8 = 63;

pub const MAX_ETHER: u8 = 64;

pub const ELIXER: u8 = 65;

pub const RED_SCALE: u8 = 66;

pub const SECRETPOTION: u8 = 67;

pub const S_S_TICKET: u8 = 68;

pub const MYSTERY_EGG: u8 = 69;

pub const CLEAR_BELL: u8 = 70;

pub const SILVER_WING: u8 = 71;

pub const MOOMOO_MILK: u8 = 72;

pub const QUICK_CLAW: u8 = 73;

pub const PSNCUREBERRY: u8 = 74;

pub const GOLD_LEAF: u8 = 75;

pub const SOFT_SAND: u8 = 76;

pub const SHARP_BEAK: u8 = 77;

pub const PRZCUREBERRY: u8 = 78;

pub const BURNT_BERRY: u8 = 79;

pub const ICE_BERRY: u8 = 80;

pub const POISON_BARB: u8 = 81;

pub const KINGS_ROCK: u8 = 82;

pub const BITTER_BERRY: u8 = 83;

pub const MINT_BERRY: u8 = 84;

pub const RED_APRICORN: u8 = 85;

pub const TINYMUSHROOM: u8 = 86;

pub const BIG_MUSHROOM: u8 = 87;

pub const SILVERPOWDER: u8 = 88;

pub const BLU_APRICORN: u8 = 89;

pub const ITEM_5A: u8 = 90;

pub const AMULET_COIN: u8 = 91;

pub const YLW_APRICORN: u8 = 92;

pub const GRN_APRICORN: u8 = 93;

pub const CLEANSE_TAG: u8 = 94;

pub const MYSTIC_WATER: u8 = 95;

pub const TWISTEDSPOON: u8 = 96;

pub const WHT_APRICORN: u8 = 97;

pub const BLACKBELT_I: u8 = 98;

pub const BLK_APRICORN: u8 = 99;

pub const ITEM_64: u8 = 100;

pub const PNK_APRICORN: u8 = 101;

pub const BLACKGLASSES: u8 = 102;

pub const SLOWPOKETAIL: u8 = 103;

pub const PINK_BOW: u8 = 104;

pub const STICK: u8 = 105;

pub const SMOKE_BALL: u8 = 106;

pub const NEVERMELTICE: u8 = 107;

pub const MAGNET: u8 = 108;

pub const MIRACLEBERRY: u8 = 109;

pub const PEARL: u8 = 110;

pub const BIG_PEARL: u8 = 111;

pub const EVERSTONE: u8 = 112;

pub const SPELL_TAG: u8 = 113;

pub const RAGECANDYBAR: u8 = 114;

pub const GS_BALL: u8 = 115;

pub const BLUE_CARD: u8 = 116;

pub const MIRACLE_SEED: u8 = 117;

pub const THICK_CLUB: u8 = 118;

pub const FOCUS_BAND: u8 = 119;

pub const ITEM_78: u8 = 120;

pub const ENERGYPOWDER: u8 = 121;

pub const ENERGY_ROOT: u8 = 122;

pub const HEAL_POWDER: u8 = 123;

pub const REVIVAL_HERB: u8 = 124;

pub const HARD_STONE: u8 = 125;

pub const LUCKY_EGG: u8 = 126;

pub const CARD_KEY: u8 = 127;

pub const MACHINE_PART: u8 = 128;

pub const EGG_TICKET: u8 = 129;

pub const LOST_ITEM: u8 = 130;

pub const STARDUST: u8 = 131;

pub const STAR_PIECE: u8 = 132;

pub const BASEMENT_KEY: u8 = 133;

pub const PASS: u8 = 134;

pub const ITEM_87: u8 = 135;

pub const ITEM_88: u8 = 136;

pub const ITEM_89: u8 = 137;

pub const CHARCOAL: u8 = 138;

pub const BERRY_JUICE: u8 = 139;

pub const SCOPE_LENS: u8 = 140;

pub const ITEM_8D: u8 = 141;

pub const ITEM_8E: u8 = 142;

pub const METAL_COAT: u8 = 143;

pub const DRAGON_FANG: u8 = 144;

pub const ITEM_91: u8 = 145;

pub const LEFTOVERS: u8 = 146;

pub const ITEM_93: u8 = 147;

pub const ITEM_94: u8 = 148;

pub const ITEM_95: u8 = 149;

pub const MYSTERYBERRY: u8 = 150;

pub const DRAGON_SCALE: u8 = 151;

pub const BERSERK_GENE: u8 = 152;

pub const ITEM_99: u8 = 153;

pub const ITEM_9A: u8 = 154;

pub const ITEM_9B: u8 = 155;

pub const SACRED_ASH: u8 = 156;

pub const HEAVY_BALL: u8 = 157;

pub const FLOWER_MAIL: u8 = 158;

pub const LEVEL_BALL: u8 = 159;

pub const LURE_BALL: u8 = 160;

pub const FAST_BALL: u8 = 161;

pub const ITEM_A2: u8 = 162;

pub const LIGHT_BALL: u8 = 163;

pub const FRIEND_BALL: u8 = 164;

pub const MOON_BALL: u8 = 165;

pub const LOVE_BALL: u8 = 166;

pub const NORMAL_BOX: u8 = 167;

pub const GORGEOUS_BOX: u8 = 168;

pub const SUN_STONE: u8 = 169;

pub const POLKADOT_BOW: u8 = 170;

pub const ITEM_AB: u8 = 171;

pub const UP_GRADE: u8 = 172;

pub const BERRY: u8 = 173;

pub const GOLD_BERRY: u8 = 174;

pub const SQUIRTBOTTLE: u8 = 175;

pub const ITEM_B0: u8 = 176;

pub const PARK_BALL: u8 = 177;

pub const RAINBOW_WING: u8 = 178;

pub const ITEM_B3: u8 = 179;

pub const BRICK_PIECE: u8 = 180;

pub const SURF_MAIL: u8 = 181;

pub const LITEBLUEMAIL: u8 = 182;

pub const PORTRAITMAIL: u8 = 183;

pub const LOVELY_MAIL: u8 = 184;

pub const EON_MAIL: u8 = 185;

pub const MORPH_MAIL: u8 = 186;

pub const BLUESKY_MAIL: u8 = 187;

pub const MUSIC_MAIL: u8 = 188;

pub const MIRAGE_MAIL: u8 = 189;

pub const ITEM_BE: u8 = 190;

pub const NUM_ITEMS: u8 = 190;

pub const _TMHM_VALUE_: u8 = 1;

pub const TM01: u8 = 191;

pub const TM_DYNAMICPUNCH: u8 = 191;

pub const TM01_MOVE: u8 = 223;

pub const DYNAMICPUNCH_TMNUM: u8 = 1;

pub const TM_HEADBUTT: u8 = 192;

pub const TM02_MOVE: u8 = 29;

pub const HEADBUTT_TMNUM: u8 = 2;

pub const TM_CURSE: u8 = 193;

pub const TM03_MOVE: u8 = 174;

pub const CURSE_TMNUM: u8 = 3;

pub const TM_ROLLOUT: u8 = 194;

pub const TM04_MOVE: u8 = 205;

pub const ROLLOUT_TMNUM: u8 = 4;

pub const ITEM_C3: u8 = 195;

pub const TM_ROAR: u8 = 196;

pub const TM05_MOVE: u8 = 46;

pub const ROAR_TMNUM: u8 = 5;

pub const TM_TOXIC: u8 = 197;

pub const TM06_MOVE: u8 = 92;

pub const TOXIC_TMNUM: u8 = 6;

pub const TM_ZAP_CANNON: u8 = 198;

pub const TM07_MOVE: u8 = 192;

pub const ZAP_CANNON_TMNUM: u8 = 7;

pub const TM_ROCK_SMASH: u8 = 199;

pub const TM08_MOVE: u8 = 249;

pub const ROCK_SMASH_TMNUM: u8 = 8;

pub const TM_PSYCH_UP: u8 = 200;

pub const TM09_MOVE: u8 = 244;

pub const PSYCH_UP_TMNUM: u8 = 9;

pub const TM_HIDDEN_POWER: u8 = 201;

pub const TM10_MOVE: u8 = 237;

pub const HIDDEN_POWER_TMNUM: u8 = 10;

pub const TM_SUNNY_DAY: u8 = 202;

pub const TM11_MOVE: u8 = 241;

pub const SUNNY_DAY_TMNUM: u8 = 11;

pub const TM_SWEET_SCENT: u8 = 203;

pub const TM12_MOVE: u8 = 230;

pub const SWEET_SCENT_TMNUM: u8 = 12;

pub const TM_SNORE: u8 = 204;

pub const TM13_MOVE: u8 = 173;

pub const SNORE_TMNUM: u8 = 13;

pub const TM_BLIZZARD: u8 = 205;

pub const TM14_MOVE: u8 = 59;

pub const BLIZZARD_TMNUM: u8 = 14;

pub const TM_HYPER_BEAM: u8 = 206;

pub const TM15_MOVE: u8 = 63;

pub const HYPER_BEAM_TMNUM: u8 = 15;

pub const TM_ICY_WIND: u8 = 207;

pub const TM16_MOVE: u8 = 196;

pub const ICY_WIND_TMNUM: u8 = 16;

pub const TM_PROTECT: u8 = 208;

pub const TM17_MOVE: u8 = 182;

pub const PROTECT_TMNUM: u8 = 17;

pub const TM_RAIN_DANCE: u8 = 209;

pub const TM18_MOVE: u8 = 240;

pub const RAIN_DANCE_TMNUM: u8 = 18;

pub const TM_GIGA_DRAIN: u8 = 210;

pub const TM19_MOVE: u8 = 202;

pub const GIGA_DRAIN_TMNUM: u8 = 19;

pub const TM_ENDURE: u8 = 211;

pub const TM20_MOVE: u8 = 203;

pub const ENDURE_TMNUM: u8 = 20;

pub const TM_FRUSTRATION: u8 = 212;

pub const TM21_MOVE: u8 = 218;

pub const FRUSTRATION_TMNUM: u8 = 21;

pub const TM_SOLARBEAM: u8 = 213;

pub const TM22_MOVE: u8 = 76;

pub const SOLARBEAM_TMNUM: u8 = 22;

pub const TM_IRON_TAIL: u8 = 214;

pub const TM23_MOVE: u8 = 231;

pub const IRON_TAIL_TMNUM: u8 = 23;

pub const TM_DRAGONBREATH: u8 = 215;

pub const TM24_MOVE: u8 = 225;

pub const DRAGONBREATH_TMNUM: u8 = 24;

pub const TM_THUNDER: u8 = 216;

pub const TM25_MOVE: u8 = 87;

pub const THUNDER_TMNUM: u8 = 25;

pub const TM_EARTHQUAKE: u8 = 217;

pub const TM26_MOVE: u8 = 89;

pub const EARTHQUAKE_TMNUM: u8 = 26;

pub const TM_RETURN: u8 = 218;

pub const TM27_MOVE: u8 = 216;

pub const RETURN_TMNUM: u8 = 27;

pub const TM_DIG: u8 = 219;

pub const TM28_MOVE: u8 = 91;

pub const DIG_TMNUM: u8 = 28;

pub const ITEM_DC: u8 = 220;

pub const TM_PSYCHIC_M: u8 = 221;

pub const TM29_MOVE: u8 = 94;

pub const PSYCHIC_M_TMNUM: u8 = 29;

pub const TM_SHADOW_BALL: u8 = 222;

pub const TM30_MOVE: u8 = 247;

pub const SHADOW_BALL_TMNUM: u8 = 30;

pub const TM_MUD_SLAP: u8 = 223;

pub const TM31_MOVE: u8 = 189;

pub const MUD_SLAP_TMNUM: u8 = 31;

pub const TM_DOUBLE_TEAM: u8 = 224;

pub const TM32_MOVE: u8 = 104;

pub const DOUBLE_TEAM_TMNUM: u8 = 32;

pub const TM_ICE_PUNCH: u8 = 225;

pub const TM33_MOVE: u8 = 8;

pub const ICE_PUNCH_TMNUM: u8 = 33;

pub const TM_SWAGGER: u8 = 226;

pub const TM34_MOVE: u8 = 207;

pub const SWAGGER_TMNUM: u8 = 34;

pub const TM_SLEEP_TALK: u8 = 227;

pub const TM35_MOVE: u8 = 214;

pub const SLEEP_TALK_TMNUM: u8 = 35;

pub const TM_SLUDGE_BOMB: u8 = 228;

pub const TM36_MOVE: u8 = 188;

pub const SLUDGE_BOMB_TMNUM: u8 = 36;

pub const TM_SANDSTORM: u8 = 229;

pub const TM37_MOVE: u8 = 201;

pub const SANDSTORM_TMNUM: u8 = 37;

pub const TM_FIRE_BLAST: u8 = 230;

pub const TM38_MOVE: u8 = 126;

pub const FIRE_BLAST_TMNUM: u8 = 38;

pub const TM_SWIFT: u8 = 231;

pub const TM39_MOVE: u8 = 129;

pub const SWIFT_TMNUM: u8 = 39;

pub const TM_DEFENSE_CURL: u8 = 232;

pub const TM40_MOVE: u8 = 111;

pub const DEFENSE_CURL_TMNUM: u8 = 40;

pub const TM_THUNDERPUNCH: u8 = 233;

pub const TM41_MOVE: u8 = 9;

pub const THUNDERPUNCH_TMNUM: u8 = 41;

pub const TM_DREAM_EATER: u8 = 234;

pub const TM42_MOVE: u8 = 138;

pub const DREAM_EATER_TMNUM: u8 = 42;

pub const TM_DETECT: u8 = 235;

pub const TM43_MOVE: u8 = 197;

pub const DETECT_TMNUM: u8 = 43;

pub const TM_REST: u8 = 236;

pub const TM44_MOVE: u8 = 156;

pub const REST_TMNUM: u8 = 44;

pub const TM_ATTRACT: u8 = 237;

pub const TM45_MOVE: u8 = 213;

pub const ATTRACT_TMNUM: u8 = 45;

pub const TM_THIEF: u8 = 238;

pub const TM46_MOVE: u8 = 168;

pub const THIEF_TMNUM: u8 = 46;

pub const TM_STEEL_WING: u8 = 239;

pub const TM47_MOVE: u8 = 211;

pub const STEEL_WING_TMNUM: u8 = 47;

pub const TM_FIRE_PUNCH: u8 = 240;

pub const TM48_MOVE: u8 = 7;

pub const FIRE_PUNCH_TMNUM: u8 = 48;

pub const TM_FURY_CUTTER: u8 = 241;

pub const TM49_MOVE: u8 = 210;

pub const FURY_CUTTER_TMNUM: u8 = 49;

pub const TM_NIGHTMARE: u8 = 242;

pub const TM50_MOVE: u8 = 171;

pub const NIGHTMARE_TMNUM: u8 = 50;

pub const NUM_TMS: u8 = 50;

pub const HM01: u8 = 243;

pub const HM_CUT: u8 = 243;

pub const HM_VALUE: u8 = 1;

pub const HM01_MOVE: u8 = 15;

pub const CUT_TMNUM: u8 = 51;

pub const HM_FLY: u8 = 244;

// duplicate assembler constant HM_VALUE

pub const HM02_MOVE: u8 = 19;

pub const FLY_TMNUM: u8 = 52;

pub const HM_SURF: u8 = 245;

// duplicate assembler constant HM_VALUE

pub const HM03_MOVE: u8 = 57;

pub const SURF_TMNUM: u8 = 53;

pub const HM_STRENGTH: u8 = 246;

// duplicate assembler constant HM_VALUE

pub const HM04_MOVE: u8 = 70;

pub const STRENGTH_TMNUM: u8 = 54;

pub const HM_FLASH: u8 = 247;

// duplicate assembler constant HM_VALUE

pub const HM05_MOVE: u8 = 148;

pub const FLASH_TMNUM: u8 = 55;

pub const HM_WHIRLPOOL: u8 = 248;

// duplicate assembler constant HM_VALUE

pub const HM06_MOVE: u8 = 250;

pub const WHIRLPOOL_TMNUM: u8 = 56;

pub const HM_WATERFALL: u8 = 249;

// duplicate assembler constant HM_VALUE

pub const HM07_MOVE: u8 = 127;

pub const WATERFALL_TMNUM: u8 = 57;

pub const NUM_HMS: u8 = 7;

pub const MT01: u8 = 250;

pub const MT_VALUE: u8 = 1;

pub const MT01_MOVE: u8 = 53;

pub const FLAMETHROWER_TMNUM: u8 = 58;

// duplicate assembler constant MT_VALUE

pub const MT02_MOVE: u8 = 85;

pub const THUNDERBOLT_TMNUM: u8 = 59;

// duplicate assembler constant MT_VALUE

pub const MT03_MOVE: u8 = 58;

pub const ICE_BEAM_TMNUM: u8 = 60;

pub const NUM_TUTORS: u8 = 3;

pub const NUM_TM_HM_TUTOR: u8 = 60;

pub const ITEM_FA: u8 = 250;

pub const USE_SCRIPT_VAR: u8 = 0;

pub const ITEM_FROM_MEM: u8 = 255;

pub const SAFARI_BALL: u8 = 8;

pub const MOON_STONE_RED: u8 = 10;

pub const FULL_HEAL_RED: u8 = 52;

