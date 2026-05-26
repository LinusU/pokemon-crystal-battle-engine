use crate::runtime::Cpu;

pub const MAX_LEVEL: u8 = 100;

pub const MIN_LEVEL: u8 = 2;

pub const EGG_LEVEL: u8 = 5;

pub const NUM_MOVES: u8 = 4;

pub const BASE_STAT_LEVEL: u8 = 7;

pub const MAX_STAT_LEVEL: u8 = 13;

pub const REST_SLEEP_TURNS: u8 = 2;

pub const TREEMON_SLEEP_TURNS: u8 = 7;

pub const BASE_PRIORITY: u8 = 1;

pub const SUPER_EFFECTIVE: u8 = 20;

pub const MORE_EFFECTIVE: u8 = 15;

pub const EFFECTIVE: u8 = 10;

pub const NOT_VERY_EFFECTIVE: u8 = 5;

pub const NO_EFFECT: u8 = 0;

pub const EFFECTIVENESS_MASK: u8 = 127;

pub const CONST_VALUE: u8 = 7;

pub const CONST_INC: u8 = 1;

pub const STAB_DAMAGE: u8 = 128;

pub const STAB_DAMAGE_F: u8 = 7;

pub const BASE_AI_SWITCH_SCORE: u8 = 10;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const ATTACK: u8 = 0;

pub const DEFENSE: u8 = 1;

pub const SPEED: u8 = 2;

pub const SP_ATTACK: u8 = 3;

pub const SP_DEFENSE: u8 = 4;

pub const ACCURACY: u8 = 5;

pub const EVASION: u8 = 6;

pub const ABILITY: u8 = 7;

pub const NUM_LEVEL_STATS: u8 = 8;

pub const MOVE_LENGTH: u8 = 7;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const STAT_HP: u8 = 1;

pub const STAT_ATK: u8 = 2;

pub const STAT_DEF: u8 = 3;

pub const STAT_SPD: u8 = 4;

pub const STAT_SATK: u8 = 5;

pub const NUM_EXP_STATS: u8 = 5;

pub const STAT_SDEF: u8 = 6;

pub const NUM_STATS: u8 = 6;

pub const NUM_BATTLE_STATS: u8 = 5;

pub const STAT_MIN_NORMAL: u8 = 5;

pub const STAT_MIN_HP: u8 = 10;

pub const MAX_STAT_VALUE: u16 = 999;

pub const ATKDEFDV_SHINY: u8 = 234;

pub const SPDSPCDV_SHINY: u8 = 170;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const WILD_BATTLE: u8 = 1;

pub const TRAINER_BATTLE: u8 = 2;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const BATTLETYPE_NORMAL: u8 = 0;

pub const BATTLETYPE_CANLOSE: u8 = 1;

pub const BATTLETYPE_DEBUG: u8 = 2;

pub const BATTLETYPE_TUTORIAL: u8 = 3;

pub const BATTLETYPE_FISH: u8 = 4;

pub const BATTLETYPE_ROAMING: u8 = 5;

pub const BATTLETYPE_CONTEST: u8 = 6;

pub const BATTLETYPE_FORCESHINY: u8 = 7;

pub const BATTLETYPE_TREE: u8 = 8;

pub const BATTLETYPE_TRAP: u8 = 9;

pub const BATTLETYPE_FORCEITEM: u8 = 10;

pub const BATTLETYPE_CELEBI: u8 = 11;

pub const BATTLETYPE_SUICUNE: u8 = 12;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const BATTLE_VARS_SUBSTATUS1: u8 = 0;

pub const BATTLE_VARS_SUBSTATUS2: u8 = 1;

pub const BATTLE_VARS_SUBSTATUS3: u8 = 2;

pub const BATTLE_VARS_SUBSTATUS4: u8 = 3;

pub const BATTLE_VARS_SUBSTATUS5: u8 = 4;

pub const BATTLE_VARS_SUBSTATUS1_OPP: u8 = 5;

pub const BATTLE_VARS_SUBSTATUS2_OPP: u8 = 6;

pub const BATTLE_VARS_SUBSTATUS3_OPP: u8 = 7;

pub const BATTLE_VARS_SUBSTATUS4_OPP: u8 = 8;

pub const BATTLE_VARS_SUBSTATUS5_OPP: u8 = 9;

pub const BATTLE_VARS_STATUS: u8 = 10;

pub const BATTLE_VARS_STATUS_OPP: u8 = 11;

pub const BATTLE_VARS_MOVE_ANIM: u8 = 12;

pub const BATTLE_VARS_MOVE_EFFECT: u8 = 13;

pub const BATTLE_VARS_MOVE_POWER: u8 = 14;

pub const BATTLE_VARS_MOVE_TYPE: u8 = 15;

pub const BATTLE_VARS_MOVE: u8 = 16;

pub const BATTLE_VARS_LAST_COUNTER_MOVE: u8 = 17;

pub const BATTLE_VARS_LAST_COUNTER_MOVE_OPP: u8 = 18;

pub const BATTLE_VARS_LAST_MOVE: u8 = 19;

pub const BATTLE_VARS_LAST_MOVE_OPP: u8 = 20;

pub const NUM_BATTLE_VARS: u8 = 21;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const PLAYER_SUBSTATUS_1: u8 = 0;

pub const ENEMY_SUBSTATUS_1: u8 = 1;

pub const PLAYER_SUBSTATUS_2: u8 = 2;

pub const ENEMY_SUBSTATUS_2: u8 = 3;

pub const PLAYER_SUBSTATUS_3: u8 = 4;

pub const ENEMY_SUBSTATUS_3: u8 = 5;

pub const PLAYER_SUBSTATUS_4: u8 = 6;

pub const ENEMY_SUBSTATUS_4: u8 = 7;

pub const PLAYER_SUBSTATUS_5: u8 = 8;

pub const ENEMY_SUBSTATUS_5: u8 = 9;

pub const PLAYER_STATUS: u8 = 10;

pub const ENEMY_STATUS: u8 = 11;

pub const PLAYER_MOVE_ANIMATION: u8 = 12;

pub const ENEMY_MOVE_ANIMATION: u8 = 13;

pub const PLAYER_MOVE_EFFECT: u8 = 14;

pub const ENEMY_MOVE_EFFECT: u8 = 15;

pub const PLAYER_MOVE_POWER: u8 = 16;

pub const ENEMY_MOVE_POWER: u8 = 17;

pub const PLAYER_MOVE_TYPE: u8 = 18;

pub const ENEMY_MOVE_TYPE: u8 = 19;

pub const PLAYER_CUR_MOVE: u8 = 20;

pub const ENEMY_CUR_MOVE: u8 = 21;

pub const PLAYER_COUNTER_MOVE: u8 = 22;

pub const ENEMY_COUNTER_MOVE: u8 = 23;

pub const PLAYER_LAST_MOVE: u8 = 24;

pub const ENEMY_LAST_MOVE: u8 = 25;

pub const NUM_BATTLE_VAR_LOCATION_PAIRS: u8 = 13;

pub const SLP_MASK: u8 = 7;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const PSN: u8 = 3;

pub const BRN: u8 = 4;

pub const FRZ: u8 = 5;

pub const PAR: u8 = 6;

pub const ALL_STATUS: u8 = 127;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const SUBSTATUS_NIGHTMARE: u8 = 0;

pub const SUBSTATUS_CURSE: u8 = 1;

pub const SUBSTATUS_PROTECT: u8 = 2;

pub const SUBSTATUS_IDENTIFIED: u8 = 3;

pub const SUBSTATUS_PERISH: u8 = 4;

pub const SUBSTATUS_ENDURE: u8 = 5;

pub const SUBSTATUS_ROLLOUT: u8 = 6;

pub const SUBSTATUS_IN_LOVE: u8 = 7;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const SUBSTATUS_CURLED: u8 = 0;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const SUBSTATUS_BIDE: u8 = 0;

pub const SUBSTATUS_RAMPAGE: u8 = 1;

pub const SUBSTATUS_IN_LOOP: u8 = 2;

pub const SUBSTATUS_FLINCHED: u8 = 3;

pub const SUBSTATUS_CHARGED: u8 = 4;

pub const SUBSTATUS_UNDERGROUND: u8 = 5;

pub const SUBSTATUS_FLYING: u8 = 6;

pub const SUBSTATUS_CONFUSED: u8 = 7;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const SUBSTATUS_X_ACCURACY: u8 = 0;

pub const SUBSTATUS_MIST: u8 = 1;

pub const SUBSTATUS_FOCUS_ENERGY: u8 = 2;

pub const SUBSTATUS_SUBSTITUTE: u8 = 4;

pub const SUBSTATUS_RECHARGE: u8 = 5;

pub const SUBSTATUS_RAGE: u8 = 6;

pub const SUBSTATUS_LEECH_SEED: u8 = 7;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const SUBSTATUS_TOXIC: u8 = 0;

pub const SUBSTATUS_TRANSFORMED: u8 = 3;

pub const SUBSTATUS_ENCORED: u8 = 4;

pub const SUBSTATUS_LOCK_ON: u8 = 5;

pub const SUBSTATUS_DESTINY_BOND: u8 = 6;

pub const SUBSTATUS_CANT_RUN: u8 = 7;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const SCREENS_SPIKES: u8 = 0;

pub const SCREENS_SAFEGUARD: u8 = 2;

pub const SCREENS_LIGHT_SCREEN: u8 = 3;

pub const SCREENS_REFLECT: u8 = 4;

pub const SCREENS_UNUSED: u8 = 5;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const WEATHER_NONE: u8 = 0;

pub const WEATHER_RAIN: u8 = 1;

pub const WEATHER_SUN: u8 = 2;

pub const WEATHER_SANDSTORM: u8 = 3;

pub const WEATHER_RAIN_END: u8 = 4;

pub const WEATHER_SUN_END: u8 = 5;

pub const WEATHER_SANDSTORM_END: u8 = 6;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const BATTLEACTION_MOVE1: u8 = 0;

pub const BATTLEACTION_MOVE2: u8 = 1;

pub const BATTLEACTION_MOVE3: u8 = 2;

pub const BATTLEACTION_MOVE4: u8 = 3;

pub const BATTLEACTION_SWITCH1: u8 = 4;

pub const BATTLEACTION_SWITCH2: u8 = 5;

pub const BATTLEACTION_SWITCH3: u8 = 6;

pub const BATTLEACTION_SWITCH4: u8 = 7;

pub const BATTLEACTION_SWITCH5: u8 = 8;

pub const BATTLEACTION_SWITCH6: u8 = 9;

pub const BATTLEACTION_SKIPTURN: u8 = 13;

pub const BATTLEACTION_STRUGGLE: u8 = 14;

pub const BATTLEACTION_FORFEIT: u8 = 15;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const BATTLEPLAYERACTION_USEMOVE: u8 = 0;

pub const BATTLEPLAYERACTION_USEITEM: u8 = 1;

pub const BATTLEPLAYERACTION_SWITCH: u8 = 2;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const WIN: u8 = 0;

pub const LOSE: u8 = 1;

pub const DRAW: u8 = 2;

pub const BATTLERESULT_CAUGHT_CELEBI: u8 = 6;

pub const BATTLERESULT_BOX_FULL: u8 = 7;

pub const BATTLERESULT_BITMASK: u8 = 192;

pub const LINK_BATTLE_RECORD_LENGTH: u8 = 18;

pub const NUM_LINK_BATTLE_RECORDS: u8 = 5;

