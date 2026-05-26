use crate::runtime::Cpu;

pub const CONST_VALUE: u8 = 0;

pub const CONST_INC: u8 = 1;

pub const OAKS_POKEMON_TALK: u8 = 0;

pub const POKEDEX_SHOW: u8 = 1;

pub const POKEMON_MUSIC: u8 = 2;

pub const LUCKY_CHANNEL: u8 = 3;

pub const BUENAS_PASSWORD: u8 = 4;

pub const PLACES_AND_PEOPLE: u8 = 5;

pub const LETS_ALL_SING: u8 = 6;

pub const ROCKET_RADIO: u8 = 7;

pub const POKE_FLUTE_RADIO: u8 = 8;

pub const UNOWN_RADIO: u8 = 9;

pub const EVOLUTION_RADIO: u8 = 10;

pub const NUM_RADIO_CHANNELS: u8 = 11;

pub const OAKS_POKEMON_TALK_2: u8 = 11;

pub const OAKS_POKEMON_TALK_3: u8 = 12;

pub const OAKS_POKEMON_TALK_4: u8 = 13;

pub const OAKS_POKEMON_TALK_5: u8 = 14;

pub const OAKS_POKEMON_TALK_6: u8 = 15;

pub const OAKS_POKEMON_TALK_7: u8 = 16;

pub const OAKS_POKEMON_TALK_8: u8 = 17;

pub const OAKS_POKEMON_TALK_9: u8 = 18;

pub const POKEDEX_SHOW_2: u8 = 19;

pub const POKEDEX_SHOW_3: u8 = 20;

pub const POKEDEX_SHOW_4: u8 = 21;

pub const POKEDEX_SHOW_5: u8 = 22;

pub const POKEMON_MUSIC_2: u8 = 23;

pub const POKEMON_MUSIC_3: u8 = 24;

pub const POKEMON_MUSIC_4: u8 = 25;

pub const POKEMON_MUSIC_5: u8 = 26;

pub const POKEMON_MUSIC_6: u8 = 27;

pub const POKEMON_MUSIC_7: u8 = 28;

pub const LETS_ALL_SING_2: u8 = 29;

pub const LUCKY_NUMBER_SHOW_2: u8 = 30;

pub const LUCKY_NUMBER_SHOW_3: u8 = 31;

pub const LUCKY_NUMBER_SHOW_4: u8 = 32;

pub const LUCKY_NUMBER_SHOW_5: u8 = 33;

pub const LUCKY_NUMBER_SHOW_6: u8 = 34;

pub const LUCKY_NUMBER_SHOW_7: u8 = 35;

pub const LUCKY_NUMBER_SHOW_8: u8 = 36;

pub const LUCKY_NUMBER_SHOW_9: u8 = 37;

pub const LUCKY_NUMBER_SHOW_10: u8 = 38;

pub const LUCKY_NUMBER_SHOW_11: u8 = 39;

pub const LUCKY_NUMBER_SHOW_12: u8 = 40;

pub const LUCKY_NUMBER_SHOW_13: u8 = 41;

pub const LUCKY_NUMBER_SHOW_14: u8 = 42;

pub const LUCKY_NUMBER_SHOW_15: u8 = 43;

pub const PLACES_AND_PEOPLE_2: u8 = 44;

pub const PLACES_AND_PEOPLE_3: u8 = 45;

pub const PLACES_AND_PEOPLE_4: u8 = 46;

pub const PLACES_AND_PEOPLE_5: u8 = 47;

pub const PLACES_AND_PEOPLE_6: u8 = 48;

pub const PLACES_AND_PEOPLE_7: u8 = 49;

pub const ROCKET_RADIO_2: u8 = 50;

pub const ROCKET_RADIO_3: u8 = 51;

pub const ROCKET_RADIO_4: u8 = 52;

pub const ROCKET_RADIO_5: u8 = 53;

pub const ROCKET_RADIO_6: u8 = 54;

pub const ROCKET_RADIO_7: u8 = 55;

pub const ROCKET_RADIO_8: u8 = 56;

pub const ROCKET_RADIO_9: u8 = 57;

pub const ROCKET_RADIO_10: u8 = 58;

pub const OAKS_POKEMON_TALK_10: u8 = 59;

pub const OAKS_POKEMON_TALK_11: u8 = 60;

pub const OAKS_POKEMON_TALK_12: u8 = 61;

pub const OAKS_POKEMON_TALK_13: u8 = 62;

pub const OAKS_POKEMON_TALK_14: u8 = 63;

pub const BUENAS_PASSWORD_2: u8 = 64;

pub const BUENAS_PASSWORD_3: u8 = 65;

pub const BUENAS_PASSWORD_4: u8 = 66;

pub const BUENAS_PASSWORD_5: u8 = 67;

pub const BUENAS_PASSWORD_6: u8 = 68;

pub const BUENAS_PASSWORD_7: u8 = 69;

pub const BUENAS_PASSWORD_8: u8 = 70;

pub const BUENAS_PASSWORD_9: u8 = 71;

pub const BUENAS_PASSWORD_10: u8 = 72;

pub const BUENAS_PASSWORD_11: u8 = 73;

pub const BUENAS_PASSWORD_12: u8 = 74;

pub const BUENAS_PASSWORD_13: u8 = 75;

pub const BUENAS_PASSWORD_14: u8 = 76;

pub const BUENAS_PASSWORD_15: u8 = 77;

pub const BUENAS_PASSWORD_16: u8 = 78;

pub const BUENAS_PASSWORD_17: u8 = 79;

pub const BUENAS_PASSWORD_18: u8 = 80;

pub const BUENAS_PASSWORD_19: u8 = 81;

pub const BUENAS_PASSWORD_20: u8 = 82;

pub const BUENAS_PASSWORD_21: u8 = 83;

pub const RADIO_SCROLL: u8 = 84;

pub const POKEDEX_SHOW_6: u8 = 85;

pub const POKEDEX_SHOW_7: u8 = 86;

pub const POKEDEX_SHOW_8: u8 = 87;

pub const NUM_RADIO_SEGMENTS: u8 = 88;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const MAPRADIO_POKEMON_CHANNEL: u8 = 0;

pub const MAPRADIO_OAKS_POKEMON_TALK: u8 = 1;

pub const MAPRADIO_POKEDEX_SHOW: u8 = 2;

pub const MAPRADIO_POKEMON_MUSIC: u8 = 3;

pub const MAPRADIO_LUCKY_CHANNEL: u8 = 4;

pub const MAPRADIO_UNOWN: u8 = 5;

pub const MAPRADIO_PLACES_PEOPLE: u8 = 6;

pub const MAPRADIO_LETS_ALL_SING: u8 = 7;

pub const MAPRADIO_ROCKET: u8 = 8;

pub const NUM_MAP_RADIO_STATIONS: u8 = 9;

pub const NUM_OAKS_POKEMON_TALK_ADVERBS: u8 = 16;

pub const NUM_OAKS_POKEMON_TALK_ADJECTIVES: u8 = 16;

pub const NUM_PNP_PEOPLE_ADJECTIVES: u8 = 16;

pub const NUM_PNP_PLACES_ADJECTIVES: u8 = 16;

pub const NUM_PASSWORD_CATEGORIES: u8 = 11;

pub const NUM_PASSWORDS_PER_CATEGORY: u8 = 3;

pub const NUM_BUENA_PRIZES: u8 = 9;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const BUENA_MON: u8 = 0;

pub const BUENA_ITEM: u8 = 1;

pub const BUENA_MOVE: u8 = 2;

pub const BUENA_STRING: u8 = 3;

pub const NUM_BUENA_FUNCTIONS: u8 = 4;

