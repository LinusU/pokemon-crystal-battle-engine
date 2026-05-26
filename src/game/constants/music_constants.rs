use crate::runtime::Cpu;

pub const CONST_VALUE: u8 = 0;

pub const CONST_INC: u8 = 1;

pub const MUSIC_NONE: u8 = 0;

pub const MUSIC_TITLE: u8 = 1;

pub const MUSIC_ROUTE_1: u8 = 2;

pub const MUSIC_ROUTE_3: u8 = 3;

pub const MUSIC_ROUTE_12: u8 = 4;

pub const MUSIC_MAGNET_TRAIN: u8 = 5;

pub const MUSIC_KANTO_GYM_LEADER_BATTLE: u8 = 6;

pub const MUSIC_KANTO_TRAINER_BATTLE: u8 = 7;

pub const MUSIC_KANTO_WILD_BATTLE: u8 = 8;

pub const MUSIC_POKEMON_CENTER: u8 = 9;

pub const MUSIC_HIKER_ENCOUNTER: u8 = 10;

pub const MUSIC_LASS_ENCOUNTER: u8 = 11;

pub const MUSIC_OFFICER_ENCOUNTER: u8 = 12;

pub const MUSIC_HEAL: u8 = 13;

pub const MUSIC_LAVENDER_TOWN: u8 = 14;

pub const MUSIC_ROUTE_2: u8 = 15;

pub const MUSIC_MT_MOON: u8 = 16;

pub const MUSIC_SHOW_ME_AROUND: u8 = 17;

pub const MUSIC_GAME_CORNER: u8 = 18;

pub const MUSIC_BICYCLE: u8 = 19;

pub const MUSIC_HALL_OF_FAME: u8 = 20;

pub const MUSIC_VIRIDIAN_CITY: u8 = 21;

pub const MUSIC_CELADON_CITY: u8 = 22;

pub const MUSIC_TRAINER_VICTORY: u8 = 23;

pub const MUSIC_WILD_VICTORY: u8 = 24;

pub const MUSIC_GYM_VICTORY: u8 = 25;

pub const MUSIC_MT_MOON_SQUARE: u8 = 26;

pub const MUSIC_GYM: u8 = 27;

pub const MUSIC_PALLET_TOWN: u8 = 28;

pub const MUSIC_POKEMON_TALK: u8 = 29;

pub const MUSIC_PROF_OAK: u8 = 30;

pub const MUSIC_RIVAL_ENCOUNTER: u8 = 31;

pub const MUSIC_RIVAL_AFTER: u8 = 32;

pub const MUSIC_SURF: u8 = 33;

pub const MUSIC_EVOLUTION: u8 = 34;

pub const MUSIC_NATIONAL_PARK: u8 = 35;

pub const MUSIC_CREDITS: u8 = 36;

pub const MUSIC_AZALEA_TOWN: u8 = 37;

pub const MUSIC_CHERRYGROVE_CITY: u8 = 38;

pub const MUSIC_KIMONO_ENCOUNTER: u8 = 39;

pub const MUSIC_UNION_CAVE: u8 = 40;

pub const MUSIC_JOHTO_WILD_BATTLE: u8 = 41;

pub const MUSIC_JOHTO_TRAINER_BATTLE: u8 = 42;

pub const MUSIC_ROUTE_30: u8 = 43;

pub const MUSIC_ECRUTEAK_CITY: u8 = 44;

pub const MUSIC_VIOLET_CITY: u8 = 45;

pub const MUSIC_JOHTO_GYM_LEADER_BATTLE: u8 = 46;

pub const MUSIC_CHAMPION_BATTLE: u8 = 47;

pub const MUSIC_RIVAL_BATTLE: u8 = 48;

pub const MUSIC_ROCKET_BATTLE: u8 = 49;

pub const MUSIC_PROF_ELM: u8 = 50;

pub const MUSIC_DARK_CAVE: u8 = 51;

pub const MUSIC_ROUTE_29: u8 = 52;

pub const MUSIC_ROUTE_36: u8 = 53;

pub const MUSIC_SS_AQUA: u8 = 54;

pub const MUSIC_YOUNGSTER_ENCOUNTER: u8 = 55;

pub const MUSIC_BEAUTY_ENCOUNTER: u8 = 56;

pub const MUSIC_ROCKET_ENCOUNTER: u8 = 57;

pub const MUSIC_POKEMANIAC_ENCOUNTER: u8 = 58;

pub const MUSIC_SAGE_ENCOUNTER: u8 = 59;

pub const MUSIC_NEW_BARK_TOWN: u8 = 60;

pub const MUSIC_GOLDENROD_CITY: u8 = 61;

pub const MUSIC_VERMILION_CITY: u8 = 62;

pub const MUSIC_POKEMON_CHANNEL: u8 = 63;

pub const MUSIC_POKE_FLUTE_CHANNEL: u8 = 64;

pub const MUSIC_TIN_TOWER: u8 = 65;

pub const MUSIC_SPROUT_TOWER: u8 = 66;

pub const MUSIC_BURNED_TOWER: u8 = 67;

pub const MUSIC_LIGHTHOUSE: u8 = 68;

pub const MUSIC_LAKE_OF_RAGE: u8 = 69;

pub const MUSIC_INDIGO_PLATEAU: u8 = 70;

pub const MUSIC_ROUTE_37: u8 = 71;

pub const MUSIC_ROCKET_HIDEOUT: u8 = 72;

pub const MUSIC_DRAGONS_DEN: u8 = 73;

pub const MUSIC_JOHTO_WILD_BATTLE_NIGHT: u8 = 74;

pub const MUSIC_RUINS_OF_ALPH_RADIO: u8 = 75;

pub const MUSIC_CAPTURE: u8 = 76;

pub const MUSIC_ROUTE_26: u8 = 77;

pub const MUSIC_MOM: u8 = 78;

pub const MUSIC_VICTORY_ROAD: u8 = 79;

pub const MUSIC_POKEMON_LULLABY: u8 = 80;

pub const MUSIC_POKEMON_MARCH: u8 = 81;

pub const MUSIC_GS_OPENING: u8 = 82;

pub const MUSIC_GS_OPENING_2: u8 = 83;

pub const MUSIC_MAIN_MENU: u8 = 84;

pub const MUSIC_RUINS_OF_ALPH_INTERIOR: u8 = 85;

pub const MUSIC_ROCKET_OVERTURE: u8 = 86;

pub const MUSIC_DANCING_HALL: u8 = 87;

pub const MUSIC_BUG_CATCHING_CONTEST_RANKING: u8 = 88;

pub const MUSIC_BUG_CATCHING_CONTEST: u8 = 89;

pub const MUSIC_LAKE_OF_RAGE_ROCKET_RADIO: u8 = 90;

pub const MUSIC_PRINTER: u8 = 91;

pub const MUSIC_POST_CREDITS: u8 = 92;

pub const MUSIC_CLAIR: u8 = 93;

pub const MUSIC_MOBILE_ADAPTER_MENU: u8 = 94;

pub const MUSIC_MOBILE_ADAPTER: u8 = 95;

pub const MUSIC_BUENAS_PASSWORD: u8 = 96;

pub const MUSIC_MYSTICALMAN_ENCOUNTER: u8 = 97;

pub const MUSIC_CRYSTAL_OPENING: u8 = 98;

pub const MUSIC_BATTLE_TOWER_THEME: u8 = 99;

pub const MUSIC_SUICUNE_BATTLE: u8 = 100;

pub const MUSIC_BATTLE_TOWER_LOBBY: u8 = 101;

pub const MUSIC_MOBILE_CENTER: u8 = 102;

pub const NUM_MUSIC_SONGS: u8 = 103;

pub const MUSIC_MAHOGANY_MART: u8 = 100;

pub const RESTART_MAP_MUSIC: u8 = 254;

pub const ENTER_MAP_MUSIC: u8 = 255;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const RADIO_TOWER_MUSIC: u8 = 128;

pub const RADIO_TOWER_MUSIC_F: u8 = 7;

