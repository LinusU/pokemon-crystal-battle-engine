use crate::runtime::Cpu;

pub const CONST_VALUE: u8 = 0;

pub const CONST_INC: u8 = 1;

pub const EFFECT_NORMAL_HIT: u8 = 0;

pub const EFFECT_SLEEP: u8 = 1;

pub const EFFECT_POISON_HIT: u8 = 2;

pub const EFFECT_LEECH_HIT: u8 = 3;

pub const EFFECT_BURN_HIT: u8 = 4;

pub const EFFECT_FREEZE_HIT: u8 = 5;

pub const EFFECT_PARALYZE_HIT: u8 = 6;

pub const EFFECT_SELFDESTRUCT: u8 = 7;

pub const EFFECT_DREAM_EATER: u8 = 8;

pub const EFFECT_MIRROR_MOVE: u8 = 9;

pub const EFFECT_ATTACK_UP: u8 = 10;

pub const EFFECT_DEFENSE_UP: u8 = 11;

pub const EFFECT_SPEED_UP: u8 = 12;

pub const EFFECT_SP_ATK_UP: u8 = 13;

pub const EFFECT_SP_DEF_UP: u8 = 14;

pub const EFFECT_ACCURACY_UP: u8 = 15;

pub const EFFECT_EVASION_UP: u8 = 16;

pub const EFFECT_ALWAYS_HIT: u8 = 17;

pub const EFFECT_ATTACK_DOWN: u8 = 18;

pub const EFFECT_DEFENSE_DOWN: u8 = 19;

pub const EFFECT_SPEED_DOWN: u8 = 20;

pub const EFFECT_SP_ATK_DOWN: u8 = 21;

pub const EFFECT_SP_DEF_DOWN: u8 = 22;

pub const EFFECT_ACCURACY_DOWN: u8 = 23;

pub const EFFECT_EVASION_DOWN: u8 = 24;

pub const EFFECT_RESET_STATS: u8 = 25;

pub const EFFECT_BIDE: u8 = 26;

pub const EFFECT_RAMPAGE: u8 = 27;

pub const EFFECT_FORCE_SWITCH: u8 = 28;

pub const EFFECT_MULTI_HIT: u8 = 29;

pub const EFFECT_CONVERSION: u8 = 30;

pub const EFFECT_FLINCH_HIT: u8 = 31;

pub const EFFECT_HEAL: u8 = 32;

pub const EFFECT_TOXIC: u8 = 33;

pub const EFFECT_PAY_DAY: u8 = 34;

pub const EFFECT_LIGHT_SCREEN: u8 = 35;

pub const EFFECT_TRI_ATTACK: u8 = 36;

pub const EFFECT_UNUSED_25: u8 = 37;

pub const EFFECT_OHKO: u8 = 38;

pub const EFFECT_RAZOR_WIND: u8 = 39;

pub const EFFECT_SUPER_FANG: u8 = 40;

pub const EFFECT_STATIC_DAMAGE: u8 = 41;

pub const EFFECT_TRAP_TARGET: u8 = 42;

pub const EFFECT_UNUSED_2B: u8 = 43;

pub const EFFECT_DOUBLE_HIT: u8 = 44;

pub const EFFECT_JUMP_KICK: u8 = 45;

pub const EFFECT_MIST: u8 = 46;

pub const EFFECT_FOCUS_ENERGY: u8 = 47;

pub const EFFECT_RECOIL_HIT: u8 = 48;

pub const EFFECT_CONFUSE: u8 = 49;

pub const EFFECT_ATTACK_UP_2: u8 = 50;

pub const EFFECT_DEFENSE_UP_2: u8 = 51;

pub const EFFECT_SPEED_UP_2: u8 = 52;

pub const EFFECT_SP_ATK_UP_2: u8 = 53;

pub const EFFECT_SP_DEF_UP_2: u8 = 54;

pub const EFFECT_ACCURACY_UP_2: u8 = 55;

pub const EFFECT_EVASION_UP_2: u8 = 56;

pub const EFFECT_TRANSFORM: u8 = 57;

pub const EFFECT_ATTACK_DOWN_2: u8 = 58;

pub const EFFECT_DEFENSE_DOWN_2: u8 = 59;

pub const EFFECT_SPEED_DOWN_2: u8 = 60;

pub const EFFECT_SP_ATK_DOWN_2: u8 = 61;

pub const EFFECT_SP_DEF_DOWN_2: u8 = 62;

pub const EFFECT_ACCURACY_DOWN_2: u8 = 63;

pub const EFFECT_EVASION_DOWN_2: u8 = 64;

pub const EFFECT_REFLECT: u8 = 65;

pub const EFFECT_POISON: u8 = 66;

pub const EFFECT_PARALYZE: u8 = 67;

pub const EFFECT_ATTACK_DOWN_HIT: u8 = 68;

pub const EFFECT_DEFENSE_DOWN_HIT: u8 = 69;

pub const EFFECT_SPEED_DOWN_HIT: u8 = 70;

pub const EFFECT_SP_ATK_DOWN_HIT: u8 = 71;

pub const EFFECT_SP_DEF_DOWN_HIT: u8 = 72;

pub const EFFECT_ACCURACY_DOWN_HIT: u8 = 73;

pub const EFFECT_EVASION_DOWN_HIT: u8 = 74;

pub const EFFECT_SKY_ATTACK: u8 = 75;

pub const EFFECT_CONFUSE_HIT: u8 = 76;

pub const EFFECT_POISON_MULTI_HIT: u8 = 77;

pub const EFFECT_UNUSED_4E: u8 = 78;

pub const EFFECT_SUBSTITUTE: u8 = 79;

pub const EFFECT_HYPER_BEAM: u8 = 80;

pub const EFFECT_RAGE: u8 = 81;

pub const EFFECT_MIMIC: u8 = 82;

pub const EFFECT_METRONOME: u8 = 83;

pub const EFFECT_LEECH_SEED: u8 = 84;

pub const EFFECT_SPLASH: u8 = 85;

pub const EFFECT_DISABLE: u8 = 86;

pub const EFFECT_LEVEL_DAMAGE: u8 = 87;

pub const EFFECT_PSYWAVE: u8 = 88;

pub const EFFECT_COUNTER: u8 = 89;

pub const EFFECT_ENCORE: u8 = 90;

pub const EFFECT_PAIN_SPLIT: u8 = 91;

pub const EFFECT_SNORE: u8 = 92;

pub const EFFECT_CONVERSION2: u8 = 93;

pub const EFFECT_LOCK_ON: u8 = 94;

pub const EFFECT_SKETCH: u8 = 95;

pub const EFFECT_DEFROST_OPPONENT: u8 = 96;

pub const EFFECT_SLEEP_TALK: u8 = 97;

pub const EFFECT_DESTINY_BOND: u8 = 98;

pub const EFFECT_REVERSAL: u8 = 99;

pub const EFFECT_SPITE: u8 = 100;

pub const EFFECT_FALSE_SWIPE: u8 = 101;

pub const EFFECT_HEAL_BELL: u8 = 102;

pub const EFFECT_PRIORITY_HIT: u8 = 103;

pub const EFFECT_TRIPLE_KICK: u8 = 104;

pub const EFFECT_THIEF: u8 = 105;

pub const EFFECT_MEAN_LOOK: u8 = 106;

pub const EFFECT_NIGHTMARE: u8 = 107;

pub const EFFECT_FLAME_WHEEL: u8 = 108;

pub const EFFECT_CURSE: u8 = 109;

pub const EFFECT_UNUSED_6E: u8 = 110;

pub const EFFECT_PROTECT: u8 = 111;

pub const EFFECT_SPIKES: u8 = 112;

pub const EFFECT_FORESIGHT: u8 = 113;

pub const EFFECT_PERISH_SONG: u8 = 114;

pub const EFFECT_SANDSTORM: u8 = 115;

pub const EFFECT_ENDURE: u8 = 116;

pub const EFFECT_ROLLOUT: u8 = 117;

pub const EFFECT_SWAGGER: u8 = 118;

pub const EFFECT_FURY_CUTTER: u8 = 119;

pub const EFFECT_ATTRACT: u8 = 120;

pub const EFFECT_RETURN: u8 = 121;

pub const EFFECT_PRESENT: u8 = 122;

pub const EFFECT_FRUSTRATION: u8 = 123;

pub const EFFECT_SAFEGUARD: u8 = 124;

pub const EFFECT_SACRED_FIRE: u8 = 125;

pub const EFFECT_MAGNITUDE: u8 = 126;

pub const EFFECT_BATON_PASS: u8 = 127;

pub const EFFECT_PURSUIT: u8 = 128;

pub const EFFECT_RAPID_SPIN: u8 = 129;

pub const EFFECT_UNUSED_82: u8 = 130;

pub const EFFECT_UNUSED_83: u8 = 131;

pub const EFFECT_MORNING_SUN: u8 = 132;

pub const EFFECT_SYNTHESIS: u8 = 133;

pub const EFFECT_MOONLIGHT: u8 = 134;

pub const EFFECT_HIDDEN_POWER: u8 = 135;

pub const EFFECT_RAIN_DANCE: u8 = 136;

pub const EFFECT_SUNNY_DAY: u8 = 137;

pub const EFFECT_DEFENSE_UP_HIT: u8 = 138;

pub const EFFECT_ATTACK_UP_HIT: u8 = 139;

pub const EFFECT_ALL_UP_HIT: u8 = 140;

pub const EFFECT_FAKE_OUT: u8 = 141;

pub const EFFECT_BELLY_DRUM: u8 = 142;

pub const EFFECT_PSYCH_UP: u8 = 143;

pub const EFFECT_MIRROR_COAT: u8 = 144;

pub const EFFECT_SKULL_BASH: u8 = 145;

pub const EFFECT_TWISTER: u8 = 146;

pub const EFFECT_EARTHQUAKE: u8 = 147;

pub const EFFECT_FUTURE_SIGHT: u8 = 148;

pub const EFFECT_GUST: u8 = 149;

pub const EFFECT_STOMP: u8 = 150;

pub const EFFECT_SOLARBEAM: u8 = 151;

pub const EFFECT_THUNDER: u8 = 152;

pub const EFFECT_TELEPORT: u8 = 153;

pub const EFFECT_BEAT_UP: u8 = 154;

pub const EFFECT_FLY: u8 = 155;

pub const EFFECT_DEFENSE_CURL: u8 = 156;

pub const NUM_MOVE_EFFECTS: u8 = 157;

