use crate::runtime::Cpu;

pub const CONST_VALUE: u8 = 0;

pub const CONST_INC: u8 = 1;

pub const LANDMARK_SPECIAL: u8 = 0;

pub const JOHTO_LANDMARK: u8 = 1;

pub const LANDMARK_NEW_BARK_TOWN: u8 = 1;

pub const LANDMARK_ROUTE_29: u8 = 2;

pub const LANDMARK_CHERRYGROVE_CITY: u8 = 3;

pub const LANDMARK_ROUTE_30: u8 = 4;

pub const LANDMARK_ROUTE_31: u8 = 5;

pub const LANDMARK_VIOLET_CITY: u8 = 6;

pub const LANDMARK_SPROUT_TOWER: u8 = 7;

pub const LANDMARK_ROUTE_32: u8 = 8;

pub const LANDMARK_RUINS_OF_ALPH: u8 = 9;

pub const LANDMARK_UNION_CAVE: u8 = 10;

pub const LANDMARK_ROUTE_33: u8 = 11;

pub const LANDMARK_AZALEA_TOWN: u8 = 12;

pub const LANDMARK_SLOWPOKE_WELL: u8 = 13;

pub const LANDMARK_ILEX_FOREST: u8 = 14;

pub const LANDMARK_ROUTE_34: u8 = 15;

pub const LANDMARK_GOLDENROD_CITY: u8 = 16;

pub const LANDMARK_RADIO_TOWER: u8 = 17;

pub const LANDMARK_ROUTE_35: u8 = 18;

pub const LANDMARK_NATIONAL_PARK: u8 = 19;

pub const LANDMARK_ROUTE_36: u8 = 20;

pub const LANDMARK_ROUTE_37: u8 = 21;

pub const LANDMARK_ECRUTEAK_CITY: u8 = 22;

pub const LANDMARK_TIN_TOWER: u8 = 23;

pub const LANDMARK_BURNED_TOWER: u8 = 24;

pub const LANDMARK_ROUTE_38: u8 = 25;

pub const LANDMARK_ROUTE_39: u8 = 26;

pub const LANDMARK_OLIVINE_CITY: u8 = 27;

pub const LANDMARK_LIGHTHOUSE: u8 = 28;

pub const LANDMARK_BATTLE_TOWER: u8 = 29;

pub const LANDMARK_ROUTE_40: u8 = 30;

pub const LANDMARK_WHIRL_ISLANDS: u8 = 31;

pub const LANDMARK_ROUTE_41: u8 = 32;

pub const LANDMARK_CIANWOOD_CITY: u8 = 33;

pub const LANDMARK_ROUTE_42: u8 = 34;

pub const LANDMARK_MT_MORTAR: u8 = 35;

pub const LANDMARK_MAHOGANY_TOWN: u8 = 36;

pub const LANDMARK_ROUTE_43: u8 = 37;

pub const LANDMARK_LAKE_OF_RAGE: u8 = 38;

pub const LANDMARK_ROUTE_44: u8 = 39;

pub const LANDMARK_ICE_PATH: u8 = 40;

pub const LANDMARK_BLACKTHORN_CITY: u8 = 41;

pub const LANDMARK_DRAGONS_DEN: u8 = 42;

pub const LANDMARK_ROUTE_45: u8 = 43;

pub const LANDMARK_DARK_CAVE: u8 = 44;

pub const LANDMARK_ROUTE_46: u8 = 45;

pub const LANDMARK_SILVER_CAVE: u8 = 46;

pub const JOHTO_LANDMARK_LAST: u8 = 46;

pub const KANTO_LANDMARK: u8 = 47;

pub const LANDMARK_PALLET_TOWN: u8 = 47;

pub const LANDMARK_ROUTE_1: u8 = 48;

pub const LANDMARK_VIRIDIAN_CITY: u8 = 49;

pub const LANDMARK_ROUTE_2: u8 = 50;

pub const LANDMARK_PEWTER_CITY: u8 = 51;

pub const LANDMARK_ROUTE_3: u8 = 52;

pub const LANDMARK_MT_MOON: u8 = 53;

pub const LANDMARK_ROUTE_4: u8 = 54;

pub const LANDMARK_CERULEAN_CITY: u8 = 55;

pub const LANDMARK_ROUTE_24: u8 = 56;

pub const LANDMARK_ROUTE_25: u8 = 57;

pub const LANDMARK_ROUTE_5: u8 = 58;

pub const LANDMARK_UNDERGROUND_PATH: u8 = 59;

pub const LANDMARK_ROUTE_6: u8 = 60;

pub const LANDMARK_VERMILION_CITY: u8 = 61;

pub const LANDMARK_DIGLETTS_CAVE: u8 = 62;

pub const LANDMARK_ROUTE_7: u8 = 63;

pub const LANDMARK_ROUTE_8: u8 = 64;

pub const LANDMARK_ROUTE_9: u8 = 65;

pub const LANDMARK_ROCK_TUNNEL: u8 = 66;

pub const LANDMARK_ROUTE_10: u8 = 67;

pub const LANDMARK_POWER_PLANT: u8 = 68;

pub const LANDMARK_LAVENDER_TOWN: u8 = 69;

pub const LANDMARK_LAV_RADIO_TOWER: u8 = 70;

pub const LANDMARK_CELADON_CITY: u8 = 71;

pub const LANDMARK_SAFFRON_CITY: u8 = 72;

pub const LANDMARK_ROUTE_11: u8 = 73;

pub const LANDMARK_ROUTE_12: u8 = 74;

pub const LANDMARK_ROUTE_13: u8 = 75;

pub const LANDMARK_ROUTE_14: u8 = 76;

pub const LANDMARK_ROUTE_15: u8 = 77;

pub const LANDMARK_ROUTE_16: u8 = 78;

pub const LANDMARK_ROUTE_17: u8 = 79;

pub const LANDMARK_ROUTE_18: u8 = 80;

pub const LANDMARK_FUCHSIA_CITY: u8 = 81;

pub const LANDMARK_ROUTE_19: u8 = 82;

pub const LANDMARK_ROUTE_20: u8 = 83;

pub const LANDMARK_SEAFOAM_ISLANDS: u8 = 84;

pub const LANDMARK_CINNABAR_ISLAND: u8 = 85;

pub const LANDMARK_ROUTE_21: u8 = 86;

pub const LANDMARK_ROUTE_22: u8 = 87;

pub const LANDMARK_VICTORY_ROAD: u8 = 88;

pub const LANDMARK_ROUTE_23: u8 = 89;

pub const LANDMARK_INDIGO_PLATEAU: u8 = 90;

pub const LANDMARK_ROUTE_26: u8 = 91;

pub const LANDMARK_ROUTE_27: u8 = 92;

pub const LANDMARK_TOHJO_FALLS: u8 = 93;

pub const LANDMARK_ROUTE_28: u8 = 94;

pub const KANTO_LANDMARK_LAST: u8 = 94;

pub const OTHER_LANDMARK: u8 = 95;

pub const LANDMARK_FAST_SHIP: u8 = 95;

pub const NUM_LANDMARKS: u8 = 96;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const LANDMARK_EVENT: u8 = 127;

pub const LANDMARK_GIFT: u8 = 126;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const JOHTO_REGION: u8 = 0;

pub const KANTO_REGION: u8 = 1;

pub const NUM_REGIONS: u8 = 2;

