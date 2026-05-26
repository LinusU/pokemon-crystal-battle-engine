use crate::runtime::Cpu;

pub const CONST_VALUE: u8 = 0;

pub const CONST_INC: u8 = 1;

pub const SATOSHI_TAJIRI: u8 = 0;

pub const JUNICHI_MASUDA: u8 = 1;

pub const TETSUYA_WATANABE: u8 = 2;

pub const SHIGEKI_MORIMOTO: u8 = 3;

pub const SOUSUKE_TAMADA: u8 = 4;

pub const TAKENORI_OOTA: u8 = 5;

pub const KEN_SUGIMORI: u8 = 6;

pub const MOTOFUMI_FUJIWARA: u8 = 7;

pub const ATSUKO_NISHIDA: u8 = 8;

pub const MUNEO_SAITO: u8 = 9;

pub const SATOSHI_OOTA: u8 = 10;

pub const RENA_YOSHIKAWA: u8 = 11;

pub const JUN_OKUTANI: u8 = 12;

pub const HIRONOBU_YOSHIDA: u8 = 13;

pub const ASUKA_IWASHITA: u8 = 14;

pub const GO_ICHINOSE: u8 = 15;

pub const MORIKAZU_AOKI: u8 = 16;

pub const KOHJI_NISHINO: u8 = 17;

pub const KENJI_MATSUSHIMA: u8 = 18;

pub const TOSHINOBU_MATSUMIYA: u8 = 19;

pub const SATORU_IWATA: u8 = 20;

pub const NOBUHIRO_SEYA: u8 = 21;

pub const KAZUHITO_SEKINE: u8 = 22;

pub const TETSUJI_OOTA: u8 = 23;

pub const NCL_SUPER_MARIO_CLUB: u8 = 24;

pub const SARUGAKUCHO: u8 = 25;

pub const AKITO_MORI: u8 = 26;

pub const TAKAHIRO_HARADA: u8 = 27;

pub const TOHRU_HASHIMOTO: u8 = 28;

pub const NOBORU_MATSUMOTO: u8 = 29;

pub const TAKEHIRO_IZUSHI: u8 = 30;

pub const TAKASHI_KAWAGUCHI: u8 = 31;

pub const TSUNEKAZU_ISHIHARA: u8 = 32;

pub const HIROSHI_YAMAUCHI: u8 = 33;

pub const KENJI_SAIKI: u8 = 34;

pub const ATSUSHI_TADA: u8 = 35;

pub const NAOKO_KAWAKAMI: u8 = 36;

pub const HIROYUKI_ZINNAI: u8 = 37;

pub const KUNIMI_KAWAMURA: u8 = 38;

pub const HISASHI_SOGABE: u8 = 39;

pub const KEITA_KAGAYA: u8 = 40;

pub const YOSHINORI_MATSUDA: u8 = 41;

pub const HITOMI_SATO: u8 = 42;

pub const TORU_OSAWA: u8 = 43;

pub const TAKAO_OHARA: u8 = 44;

pub const YUICHIRO_ITO: u8 = 45;

pub const TAKAO_SHIMIZU: u8 = 46;

pub const PLANNING: u8 = 47;

pub const KEITA_NAKAMURA: u8 = 48;

pub const HIROTAKA_UEMURA: u8 = 49;

pub const HIROAKI_TAMURA: u8 = 50;

pub const NORIAKI_SAKAGUCHI: u8 = 51;

pub const MIYUKI_SATO: u8 = 52;

pub const GAKUZI_NOMOTO: u8 = 53;

pub const AI_MASHIMA: u8 = 54;

pub const MIKIHIRO_ISHIKAWA: u8 = 55;

pub const HIDEYUKI_HASHIMOTO: u8 = 56;

pub const SATOSHI_YAMATO: u8 = 57;

pub const SHIGERU_MIYAMOTO: u8 = 58;

pub const GAIL_TILDEN: u8 = 59;

pub const NOB_OGASAWARA: u8 = 60;

pub const SETH_MCMAHILL: u8 = 61;

pub const HIROTO_ALEXANDER: u8 = 62;

pub const TERESA_LILLYGREN: u8 = 63;

pub const THOMAS_HERTZOG: u8 = 64;

pub const ERIK_JOHNSON: u8 = 65;

pub const HIRO_NAKAMURA: u8 = 66;

pub const TERUKI_MURAKAWA: u8 = 67;

pub const KAZUYOSHI_OSAWA: u8 = 68;

pub const KIMIKO_NAKAMICHI: u8 = 69;

pub const CREDIT_END: u8 = 70;

pub const CREDIT_UNKNOWN: u8 = 71;

pub const STAFF: u8 = 72;

pub const DIRECTOR: u8 = 73;

pub const CODIRECTOR: u8 = 74;

pub const PROGRAMMERS: u8 = 75;

pub const GRAPHICS_DIRECTOR: u8 = 76;

pub const MONSTER_DESIGN: u8 = 77;

pub const GRAPHICS_DESIGN: u8 = 78;

pub const CREDIT_MUSIC: u8 = 79;

pub const CREDIT_SOUND_EFFECTS: u8 = 80;

pub const GAME_DESIGN: u8 = 81;

pub const GAME_SCENARIO: u8 = 82;

pub const TOOL_PROGRAMMING: u8 = 83;

pub const PARAMETRIC_DESIGN: u8 = 84;

pub const SCRIPT_DESIGN: u8 = 85;

pub const MAP_DATA_DESIGN: u8 = 86;

pub const MAP_DESIGN: u8 = 87;

pub const PRODUCT_TESTING: u8 = 88;

pub const SPECIAL_THANKS: u8 = 89;

pub const PRODUCERS: u8 = 90;

pub const EXECUTIVE_PRODUCER: u8 = 91;

pub const POKEMON_ANIMATION: u8 = 92;

pub const POKEDEX_TEXT: u8 = 93;

pub const MOBILE_PRJ_LEADER: u8 = 94;

pub const MOBILE_SYSTEM_AD: u8 = 95;

pub const MOBILE_STADIUM_DIR: u8 = 96;

pub const COORDINATION: u8 = 97;

pub const COPYRIGHT: u8 = 98;

pub const US_VERSION_STAFF: u8 = 99;

pub const US_COORDINATION: u8 = 100;

pub const TEXT_TRANSLATION: u8 = 101;

pub const PAAD_TESTING: u8 = 102;

pub const NUM_CREDITS_STRINGS: u8 = 103;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const CREDITS_END: i16 = -1;

pub const CREDITS_WAIT: i16 = -2;

pub const CREDITS_SCENE: i16 = -3;

pub const CREDITS_CLEAR: i16 = -4;

pub const CREDITS_MUSIC: i16 = -5;

pub const CREDITS_WAIT2: i16 = -6;

pub const CREDITS_THEEND: i16 = -7;

