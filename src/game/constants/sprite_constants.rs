use crate::runtime::Cpu;

pub const CONST_VALUE: u8 = 0;

pub const CONST_INC: u8 = 1;

pub const SPRITE_NONE: u8 = 0;

pub const SPRITE_CHRIS: u8 = 1;

pub const SPRITE_CHRIS_BIKE: u8 = 2;

pub const SPRITE_GAMEBOY_KID: u8 = 3;

pub const SPRITE_RIVAL: u8 = 4;

pub const SPRITE_OAK: u8 = 5;

pub const SPRITE_RED: u8 = 6;

pub const SPRITE_BLUE: u8 = 7;

pub const SPRITE_BILL: u8 = 8;

pub const SPRITE_ELDER: u8 = 9;

pub const SPRITE_JANINE: u8 = 10;

pub const SPRITE_KURT: u8 = 11;

pub const SPRITE_MOM: u8 = 12;

pub const SPRITE_BLAINE: u8 = 13;

pub const SPRITE_REDS_MOM: u8 = 14;

pub const SPRITE_DAISY: u8 = 15;

pub const SPRITE_ELM: u8 = 16;

pub const SPRITE_WILL: u8 = 17;

pub const SPRITE_FALKNER: u8 = 18;

pub const SPRITE_WHITNEY: u8 = 19;

pub const SPRITE_BUGSY: u8 = 20;

pub const SPRITE_MORTY: u8 = 21;

pub const SPRITE_CHUCK: u8 = 22;

pub const SPRITE_JASMINE: u8 = 23;

pub const SPRITE_PRYCE: u8 = 24;

pub const SPRITE_CLAIR: u8 = 25;

pub const SPRITE_BROCK: u8 = 26;

pub const SPRITE_KAREN: u8 = 27;

pub const SPRITE_BRUNO: u8 = 28;

pub const SPRITE_MISTY: u8 = 29;

pub const SPRITE_LANCE: u8 = 30;

pub const SPRITE_SURGE: u8 = 31;

pub const SPRITE_ERIKA: u8 = 32;

pub const SPRITE_KOGA: u8 = 33;

pub const SPRITE_SABRINA: u8 = 34;

pub const SPRITE_COOLTRAINER_M: u8 = 35;

pub const SPRITE_COOLTRAINER_F: u8 = 36;

pub const SPRITE_BUG_CATCHER: u8 = 37;

pub const SPRITE_TWIN: u8 = 38;

pub const SPRITE_YOUNGSTER: u8 = 39;

pub const SPRITE_LASS: u8 = 40;

pub const SPRITE_TEACHER: u8 = 41;

pub const SPRITE_BEAUTY: u8 = 42;

pub const SPRITE_SUPER_NERD: u8 = 43;

pub const SPRITE_ROCKER: u8 = 44;

pub const SPRITE_POKEFAN_M: u8 = 45;

pub const SPRITE_POKEFAN_F: u8 = 46;

pub const SPRITE_GRAMPS: u8 = 47;

pub const SPRITE_GRANNY: u8 = 48;

pub const SPRITE_SWIMMER_GUY: u8 = 49;

pub const SPRITE_SWIMMER_GIRL: u8 = 50;

pub const SPRITE_BIG_SNORLAX: u8 = 51;

pub const SPRITE_SURFING_PIKACHU: u8 = 52;

pub const SPRITE_ROCKET: u8 = 53;

pub const SPRITE_ROCKET_GIRL: u8 = 54;

pub const SPRITE_NURSE: u8 = 55;

pub const SPRITE_LINK_RECEPTIONIST: u8 = 56;

pub const SPRITE_CLERK: u8 = 57;

pub const SPRITE_FISHER: u8 = 58;

pub const SPRITE_FISHING_GURU: u8 = 59;

pub const SPRITE_SCIENTIST: u8 = 60;

pub const SPRITE_KIMONO_GIRL: u8 = 61;

pub const SPRITE_SAGE: u8 = 62;

pub const SPRITE_UNUSED_GUY: u8 = 63;

pub const SPRITE_GENTLEMAN: u8 = 64;

pub const SPRITE_BLACK_BELT: u8 = 65;

pub const SPRITE_RECEPTIONIST: u8 = 66;

pub const SPRITE_OFFICER: u8 = 67;

pub const SPRITE_CAL: u8 = 68;

pub const SPRITE_SLOWPOKE: u8 = 69;

pub const SPRITE_CAPTAIN: u8 = 70;

pub const SPRITE_BIG_LAPRAS: u8 = 71;

pub const SPRITE_GYM_GUIDE: u8 = 72;

pub const SPRITE_SAILOR: u8 = 73;

pub const SPRITE_BIKER: u8 = 74;

pub const SPRITE_PHARMACIST: u8 = 75;

pub const SPRITE_MONSTER: u8 = 76;

pub const SPRITE_FAIRY: u8 = 77;

pub const SPRITE_BIRD: u8 = 78;

pub const SPRITE_DRAGON: u8 = 79;

pub const SPRITE_BIG_ONIX: u8 = 80;

pub const SPRITE_N64: u8 = 81;

pub const SPRITE_SUDOWOODO: u8 = 82;

pub const SPRITE_SURF: u8 = 83;

pub const SPRITE_POKE_BALL: u8 = 84;

pub const SPRITE_POKEDEX: u8 = 85;

pub const SPRITE_PAPER: u8 = 86;

pub const SPRITE_VIRTUAL_BOY: u8 = 87;

pub const SPRITE_OLD_LINK_RECEPTIONIST: u8 = 88;

pub const SPRITE_ROCK: u8 = 89;

pub const SPRITE_BOULDER: u8 = 90;

pub const SPRITE_SNES: u8 = 91;

pub const SPRITE_FAMICOM: u8 = 92;

pub const SPRITE_FRUIT_TREE: u8 = 93;

pub const SPRITE_GOLD_TROPHY: u8 = 94;

pub const SPRITE_SILVER_TROPHY: u8 = 95;

pub const SPRITE_KRIS: u8 = 96;

pub const SPRITE_KRIS_BIKE: u8 = 97;

pub const SPRITE_KURT_OUTSIDE: u8 = 98;

pub const SPRITE_SUICUNE: u8 = 99;

pub const SPRITE_ENTEI: u8 = 100;

pub const SPRITE_RAIKOU: u8 = 101;

pub const SPRITE_STANDING_YOUNGSTER: u8 = 102;

pub const NUM_OVERWORLD_SPRITES: u8 = 102;

// duplicate assembler constant CONST_VALUE

pub const SPRITE_POKEMON: u8 = 128;

pub const SPRITE_UNOWN: u8 = 128;

pub const SPRITE_GEODUDE: u8 = 129;

pub const SPRITE_GROWLITHE: u8 = 130;

pub const SPRITE_WEEDLE: u8 = 131;

pub const SPRITE_SHELLDER: u8 = 132;

pub const SPRITE_ODDISH: u8 = 133;

pub const SPRITE_GENGAR: u8 = 134;

pub const SPRITE_ZUBAT: u8 = 135;

pub const SPRITE_MAGIKARP: u8 = 136;

pub const SPRITE_SQUIRTLE: u8 = 137;

pub const SPRITE_TOGEPI: u8 = 138;

pub const SPRITE_BUTTERFREE: u8 = 139;

pub const SPRITE_DIGLETT: u8 = 140;

pub const SPRITE_POLIWAG: u8 = 141;

pub const SPRITE_PIKACHU: u8 = 142;

pub const SPRITE_CLEFAIRY: u8 = 143;

pub const SPRITE_CHARMANDER: u8 = 144;

pub const SPRITE_JYNX: u8 = 145;

pub const SPRITE_STARMIE: u8 = 146;

pub const SPRITE_BULBASAUR: u8 = 147;

pub const SPRITE_JIGGLYPUFF: u8 = 148;

pub const SPRITE_GRIMER: u8 = 149;

pub const SPRITE_EKANS: u8 = 150;

pub const SPRITE_PARAS: u8 = 151;

pub const SPRITE_TENTACOOL: u8 = 152;

pub const SPRITE_TAUROS: u8 = 153;

pub const SPRITE_MACHOP: u8 = 154;

pub const SPRITE_VOLTORB: u8 = 155;

pub const SPRITE_LAPRAS: u8 = 156;

pub const SPRITE_RHYDON: u8 = 157;

pub const SPRITE_MOLTRES: u8 = 158;

pub const SPRITE_SNORLAX: u8 = 159;

pub const SPRITE_GYARADOS: u8 = 160;

pub const SPRITE_LUGIA: u8 = 161;

pub const SPRITE_HO_OH: u8 = 162;

pub const NUM_POKEMON_SPRITES: u8 = 35;

// duplicate assembler constant CONST_VALUE

pub const SPRITE_DAY_CARE_MON_1: u8 = 224;

pub const SPRITE_DAY_CARE_MON_2: u8 = 225;

// duplicate assembler constant CONST_VALUE

pub const SPRITE_VARS: u8 = 240;

pub const SPRITE_CONSOLE: u8 = 240;

pub const SPRITE_DOLL_1: u8 = 241;

pub const SPRITE_DOLL_2: u8 = 242;

pub const SPRITE_BIG_DOLL: u8 = 243;

pub const SPRITE_WEIRD_TREE: u8 = 244;

pub const SPRITE_OLIVINE_RIVAL: u8 = 245;

pub const SPRITE_AZALEA_ROCKET: u8 = 246;

pub const SPRITE_FUCHSIA_GYM_1: u8 = 247;

pub const SPRITE_FUCHSIA_GYM_2: u8 = 248;

pub const SPRITE_FUCHSIA_GYM_3: u8 = 249;

pub const SPRITE_FUCHSIA_GYM_4: u8 = 250;

pub const SPRITE_COPYCAT: u8 = 251;

pub const SPRITE_JANINE_IMPERSONATOR: u8 = 252;

