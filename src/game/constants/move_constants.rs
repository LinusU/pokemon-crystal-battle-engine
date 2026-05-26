use crate::runtime::Cpu;

pub const CONST_VALUE: u8 = 0;

pub const CONST_INC: u8 = 1;

pub const NO_MOVE: u8 = 0;

pub const POUND: u8 = 1;

pub const KARATE_CHOP: u8 = 2;

pub const DOUBLESLAP: u8 = 3;

pub const COMET_PUNCH: u8 = 4;

pub const MEGA_PUNCH: u8 = 5;

pub const PAY_DAY: u8 = 6;

pub const FIRE_PUNCH: u8 = 7;

pub const ICE_PUNCH: u8 = 8;

pub const THUNDERPUNCH: u8 = 9;

pub const SCRATCH: u8 = 10;

pub const VICEGRIP: u8 = 11;

pub const GUILLOTINE: u8 = 12;

pub const RAZOR_WIND: u8 = 13;

pub const SWORDS_DANCE: u8 = 14;

pub const CUT: u8 = 15;

pub const GUST: u8 = 16;

pub const WING_ATTACK: u8 = 17;

pub const WHIRLWIND: u8 = 18;

pub const FLY: u8 = 19;

pub const BIND: u8 = 20;

pub const SLAM: u8 = 21;

pub const VINE_WHIP: u8 = 22;

pub const STOMP: u8 = 23;

pub const DOUBLE_KICK: u8 = 24;

pub const MEGA_KICK: u8 = 25;

pub const JUMP_KICK: u8 = 26;

pub const ROLLING_KICK: u8 = 27;

pub const SAND_ATTACK: u8 = 28;

pub const HEADBUTT: u8 = 29;

pub const HORN_ATTACK: u8 = 30;

pub const FURY_ATTACK: u8 = 31;

pub const HORN_DRILL: u8 = 32;

pub const TACKLE: u8 = 33;

pub const BODY_SLAM: u8 = 34;

pub const WRAP: u8 = 35;

pub const TAKE_DOWN: u8 = 36;

pub const THRASH: u8 = 37;

pub const DOUBLE_EDGE: u8 = 38;

pub const TAIL_WHIP: u8 = 39;

pub const POISON_STING: u8 = 40;

pub const TWINEEDLE: u8 = 41;

pub const PIN_MISSILE: u8 = 42;

pub const LEER: u8 = 43;

pub const BITE: u8 = 44;

pub const GROWL: u8 = 45;

pub const ROAR: u8 = 46;

pub const SING: u8 = 47;

pub const SUPERSONIC: u8 = 48;

pub const SONICBOOM: u8 = 49;

pub const DISABLE: u8 = 50;

pub const ACID: u8 = 51;

pub const EMBER: u8 = 52;

pub const FLAMETHROWER: u8 = 53;

pub const MIST: u8 = 54;

pub const WATER_GUN: u8 = 55;

pub const HYDRO_PUMP: u8 = 56;

pub const SURF: u8 = 57;

pub const ICE_BEAM: u8 = 58;

pub const BLIZZARD: u8 = 59;

pub const PSYBEAM: u8 = 60;

pub const BUBBLEBEAM: u8 = 61;

pub const AURORA_BEAM: u8 = 62;

pub const HYPER_BEAM: u8 = 63;

pub const PECK: u8 = 64;

pub const DRILL_PECK: u8 = 65;

pub const SUBMISSION: u8 = 66;

pub const LOW_KICK: u8 = 67;

pub const COUNTER: u8 = 68;

pub const SEISMIC_TOSS: u8 = 69;

pub const STRENGTH: u8 = 70;

pub const ABSORB: u8 = 71;

pub const MEGA_DRAIN: u8 = 72;

pub const LEECH_SEED: u8 = 73;

pub const GROWTH: u8 = 74;

pub const RAZOR_LEAF: u8 = 75;

pub const SOLARBEAM: u8 = 76;

pub const POISONPOWDER: u8 = 77;

pub const STUN_SPORE: u8 = 78;

pub const SLEEP_POWDER: u8 = 79;

pub const PETAL_DANCE: u8 = 80;

pub const STRING_SHOT: u8 = 81;

pub const DRAGON_RAGE: u8 = 82;

pub const FIRE_SPIN: u8 = 83;

pub const THUNDERSHOCK: u8 = 84;

pub const THUNDERBOLT: u8 = 85;

pub const THUNDER_WAVE: u8 = 86;

pub const THUNDER: u8 = 87;

pub const ROCK_THROW: u8 = 88;

pub const EARTHQUAKE: u8 = 89;

pub const FISSURE: u8 = 90;

pub const DIG: u8 = 91;

pub const TOXIC: u8 = 92;

pub const CONFUSION: u8 = 93;

pub const PSYCHIC_M: u8 = 94;

pub const HYPNOSIS: u8 = 95;

pub const MEDITATE: u8 = 96;

pub const AGILITY: u8 = 97;

pub const QUICK_ATTACK: u8 = 98;

pub const RAGE: u8 = 99;

pub const TELEPORT: u8 = 100;

pub const NIGHT_SHADE: u8 = 101;

pub const MIMIC: u8 = 102;

pub const SCREECH: u8 = 103;

pub const DOUBLE_TEAM: u8 = 104;

pub const RECOVER: u8 = 105;

pub const HARDEN: u8 = 106;

pub const MINIMIZE: u8 = 107;

pub const SMOKESCREEN: u8 = 108;

pub const CONFUSE_RAY: u8 = 109;

pub const WITHDRAW: u8 = 110;

pub const DEFENSE_CURL: u8 = 111;

pub const BARRIER: u8 = 112;

pub const LIGHT_SCREEN: u8 = 113;

pub const HAZE: u8 = 114;

pub const REFLECT: u8 = 115;

pub const FOCUS_ENERGY: u8 = 116;

pub const BIDE: u8 = 117;

pub const METRONOME: u8 = 118;

pub const MIRROR_MOVE: u8 = 119;

pub const SELFDESTRUCT: u8 = 120;

pub const EGG_BOMB: u8 = 121;

pub const LICK: u8 = 122;

pub const SMOG: u8 = 123;

pub const SLUDGE: u8 = 124;

pub const BONE_CLUB: u8 = 125;

pub const FIRE_BLAST: u8 = 126;

pub const WATERFALL: u8 = 127;

pub const CLAMP: u8 = 128;

pub const SWIFT: u8 = 129;

pub const SKULL_BASH: u8 = 130;

pub const SPIKE_CANNON: u8 = 131;

pub const CONSTRICT: u8 = 132;

pub const AMNESIA: u8 = 133;

pub const KINESIS: u8 = 134;

pub const SOFTBOILED: u8 = 135;

pub const HI_JUMP_KICK: u8 = 136;

pub const GLARE: u8 = 137;

pub const DREAM_EATER: u8 = 138;

pub const POISON_GAS: u8 = 139;

pub const BARRAGE: u8 = 140;

pub const LEECH_LIFE: u8 = 141;

pub const LOVELY_KISS: u8 = 142;

pub const SKY_ATTACK: u8 = 143;

pub const TRANSFORM: u8 = 144;

pub const BUBBLE: u8 = 145;

pub const DIZZY_PUNCH: u8 = 146;

pub const SPORE: u8 = 147;

pub const FLASH: u8 = 148;

pub const PSYWAVE: u8 = 149;

pub const SPLASH: u8 = 150;

pub const ACID_ARMOR: u8 = 151;

pub const CRABHAMMER: u8 = 152;

pub const EXPLOSION: u8 = 153;

pub const FURY_SWIPES: u8 = 154;

pub const BONEMERANG: u8 = 155;

pub const REST: u8 = 156;

pub const ROCK_SLIDE: u8 = 157;

pub const HYPER_FANG: u8 = 158;

pub const SHARPEN: u8 = 159;

pub const CONVERSION: u8 = 160;

pub const TRI_ATTACK: u8 = 161;

pub const SUPER_FANG: u8 = 162;

pub const SLASH: u8 = 163;

pub const SUBSTITUTE: u8 = 164;

pub const STRUGGLE: u8 = 165;

pub const SKETCH: u8 = 166;

pub const TRIPLE_KICK: u8 = 167;

pub const THIEF: u8 = 168;

pub const SPIDER_WEB: u8 = 169;

pub const MIND_READER: u8 = 170;

pub const NIGHTMARE: u8 = 171;

pub const FLAME_WHEEL: u8 = 172;

pub const SNORE: u8 = 173;

pub const CURSE: u8 = 174;

pub const FLAIL: u8 = 175;

pub const CONVERSION2: u8 = 176;

pub const AEROBLAST: u8 = 177;

pub const COTTON_SPORE: u8 = 178;

pub const REVERSAL: u8 = 179;

pub const SPITE: u8 = 180;

pub const POWDER_SNOW: u8 = 181;

pub const PROTECT: u8 = 182;

pub const MACH_PUNCH: u8 = 183;

pub const SCARY_FACE: u8 = 184;

pub const FAINT_ATTACK: u8 = 185;

pub const SWEET_KISS: u8 = 186;

pub const BELLY_DRUM: u8 = 187;

pub const SLUDGE_BOMB: u8 = 188;

pub const MUD_SLAP: u8 = 189;

pub const OCTAZOOKA: u8 = 190;

pub const SPIKES: u8 = 191;

pub const ZAP_CANNON: u8 = 192;

pub const FORESIGHT: u8 = 193;

pub const DESTINY_BOND: u8 = 194;

pub const PERISH_SONG: u8 = 195;

pub const ICY_WIND: u8 = 196;

pub const DETECT: u8 = 197;

pub const BONE_RUSH: u8 = 198;

pub const LOCK_ON: u8 = 199;

pub const OUTRAGE: u8 = 200;

pub const SANDSTORM: u8 = 201;

pub const GIGA_DRAIN: u8 = 202;

pub const ENDURE: u8 = 203;

pub const CHARM: u8 = 204;

pub const ROLLOUT: u8 = 205;

pub const FALSE_SWIPE: u8 = 206;

pub const SWAGGER: u8 = 207;

pub const MILK_DRINK: u8 = 208;

pub const SPARK: u8 = 209;

pub const FURY_CUTTER: u8 = 210;

pub const STEEL_WING: u8 = 211;

pub const MEAN_LOOK: u8 = 212;

pub const ATTRACT: u8 = 213;

pub const SLEEP_TALK: u8 = 214;

pub const HEAL_BELL: u8 = 215;

pub const RETURN: u8 = 216;

pub const PRESENT: u8 = 217;

pub const FRUSTRATION: u8 = 218;

pub const SAFEGUARD: u8 = 219;

pub const PAIN_SPLIT: u8 = 220;

pub const SACRED_FIRE: u8 = 221;

pub const MAGNITUDE: u8 = 222;

pub const DYNAMICPUNCH: u8 = 223;

pub const MEGAHORN: u8 = 224;

pub const DRAGONBREATH: u8 = 225;

pub const BATON_PASS: u8 = 226;

pub const ENCORE: u8 = 227;

pub const PURSUIT: u8 = 228;

pub const RAPID_SPIN: u8 = 229;

pub const SWEET_SCENT: u8 = 230;

pub const IRON_TAIL: u8 = 231;

pub const METAL_CLAW: u8 = 232;

pub const VITAL_THROW: u8 = 233;

pub const MORNING_SUN: u8 = 234;

pub const SYNTHESIS: u8 = 235;

pub const MOONLIGHT: u8 = 236;

pub const HIDDEN_POWER: u8 = 237;

pub const CROSS_CHOP: u8 = 238;

pub const TWISTER: u8 = 239;

pub const RAIN_DANCE: u8 = 240;

pub const SUNNY_DAY: u8 = 241;

pub const CRUNCH: u8 = 242;

pub const MIRROR_COAT: u8 = 243;

pub const PSYCH_UP: u8 = 244;

pub const EXTREMESPEED: u8 = 245;

pub const ANCIENTPOWER: u8 = 246;

pub const SHADOW_BALL: u8 = 247;

pub const FUTURE_SIGHT: u8 = 248;

pub const ROCK_SMASH: u8 = 249;

pub const WHIRLPOOL: u8 = 250;

pub const BEAT_UP: u8 = 251;

pub const NUM_ATTACKS: u8 = 251;

pub const CANNOT_MOVE: u8 = 255;

// duplicate assembler constant CONST_VALUE

pub const ANIM_SWEET_SCENT_2: u8 = 255;

pub const ANIM_THROW_POKE_BALL: u16 = 256;

pub const ANIM_SEND_OUT_MON: u16 = 257;

pub const ANIM_RETURN_MON: u16 = 258;

pub const ANIM_CONFUSED: u16 = 259;

pub const ANIM_SLP: u16 = 260;

pub const ANIM_BRN: u16 = 261;

pub const ANIM_PSN: u16 = 262;

pub const ANIM_SAP: u16 = 263;

pub const ANIM_FRZ: u16 = 264;

pub const ANIM_PAR: u16 = 265;

pub const ANIM_IN_LOVE: u16 = 266;

pub const ANIM_IN_SANDSTORM: u16 = 267;

pub const ANIM_IN_NIGHTMARE: u16 = 268;

pub const ANIM_IN_WHIRLPOOL: u16 = 269;

pub const BATTLE_AFTERANIMS: u16 = 270;

pub const ANIM_MISS: u16 = 270;

pub const ANIM_ENEMY_DAMAGE: u16 = 271;

pub const ANIM_ENEMY_STAT_DOWN: u16 = 272;

pub const ANIM_PLAYER_STAT_DOWN: u16 = 273;

pub const ANIM_PLAYER_DAMAGE: u16 = 274;

pub const ANIM_WOBBLE: u16 = 275;

pub const ANIM_SHAKE: u16 = 276;

pub const ANIM_HIT_CONFUSION: u16 = 277;

pub const NUM_BATTLE_ANIMS: u16 = 277;

