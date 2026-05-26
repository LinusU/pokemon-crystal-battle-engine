use crate::runtime::Cpu;

pub const CONST_VALUE: u8 = 1;

pub const CONST_INC: u8 = 1;

pub const BULBASAUR: u8 = 1;

pub const IVYSAUR: u8 = 2;

pub const VENUSAUR: u8 = 3;

pub const CHARMANDER: u8 = 4;

pub const CHARMELEON: u8 = 5;

pub const CHARIZARD: u8 = 6;

pub const SQUIRTLE: u8 = 7;

pub const WARTORTLE: u8 = 8;

pub const BLASTOISE: u8 = 9;

pub const CATERPIE: u8 = 10;

pub const METAPOD: u8 = 11;

pub const BUTTERFREE: u8 = 12;

pub const WEEDLE: u8 = 13;

pub const KAKUNA: u8 = 14;

pub const BEEDRILL: u8 = 15;

pub const PIDGEY: u8 = 16;

pub const PIDGEOTTO: u8 = 17;

pub const PIDGEOT: u8 = 18;

pub const RATTATA: u8 = 19;

pub const RATICATE: u8 = 20;

pub const SPEAROW: u8 = 21;

pub const FEAROW: u8 = 22;

pub const EKANS: u8 = 23;

pub const ARBOK: u8 = 24;

pub const PIKACHU: u8 = 25;

pub const RAICHU: u8 = 26;

pub const SANDSHREW: u8 = 27;

pub const SANDSLASH: u8 = 28;

pub const NIDORAN_F: u8 = 29;

pub const NIDORINA: u8 = 30;

pub const NIDOQUEEN: u8 = 31;

pub const NIDORAN_M: u8 = 32;

pub const NIDORINO: u8 = 33;

pub const NIDOKING: u8 = 34;

pub const CLEFAIRY: u8 = 35;

pub const CLEFABLE: u8 = 36;

pub const VULPIX: u8 = 37;

pub const NINETALES: u8 = 38;

pub const JIGGLYPUFF: u8 = 39;

pub const WIGGLYTUFF: u8 = 40;

pub const ZUBAT: u8 = 41;

pub const GOLBAT: u8 = 42;

pub const ODDISH: u8 = 43;

pub const GLOOM: u8 = 44;

pub const VILEPLUME: u8 = 45;

pub const PARAS: u8 = 46;

pub const PARASECT: u8 = 47;

pub const VENONAT: u8 = 48;

pub const VENOMOTH: u8 = 49;

pub const DIGLETT: u8 = 50;

pub const DUGTRIO: u8 = 51;

pub const MEOWTH: u8 = 52;

pub const PERSIAN: u8 = 53;

pub const PSYDUCK: u8 = 54;

pub const GOLDUCK: u8 = 55;

pub const MANKEY: u8 = 56;

pub const PRIMEAPE: u8 = 57;

pub const GROWLITHE: u8 = 58;

pub const ARCANINE: u8 = 59;

pub const POLIWAG: u8 = 60;

pub const POLIWHIRL: u8 = 61;

pub const POLIWRATH: u8 = 62;

pub const ABRA: u8 = 63;

pub const KADABRA: u8 = 64;

pub const ALAKAZAM: u8 = 65;

pub const MACHOP: u8 = 66;

pub const MACHOKE: u8 = 67;

pub const MACHAMP: u8 = 68;

pub const BELLSPROUT: u8 = 69;

pub const WEEPINBELL: u8 = 70;

pub const VICTREEBEL: u8 = 71;

pub const TENTACOOL: u8 = 72;

pub const TENTACRUEL: u8 = 73;

pub const GEODUDE: u8 = 74;

pub const GRAVELER: u8 = 75;

pub const GOLEM: u8 = 76;

pub const PONYTA: u8 = 77;

pub const RAPIDASH: u8 = 78;

pub const SLOWPOKE: u8 = 79;

pub const SLOWBRO: u8 = 80;

pub const MAGNEMITE: u8 = 81;

pub const MAGNETON: u8 = 82;

pub const FARFETCH_D: u8 = 83;

pub const DODUO: u8 = 84;

pub const DODRIO: u8 = 85;

pub const SEEL: u8 = 86;

pub const DEWGONG: u8 = 87;

pub const GRIMER: u8 = 88;

pub const MUK: u8 = 89;

pub const SHELLDER: u8 = 90;

pub const CLOYSTER: u8 = 91;

pub const GASTLY: u8 = 92;

pub const HAUNTER: u8 = 93;

pub const GENGAR: u8 = 94;

pub const ONIX: u8 = 95;

pub const DROWZEE: u8 = 96;

pub const HYPNO: u8 = 97;

pub const KRABBY: u8 = 98;

pub const KINGLER: u8 = 99;

pub const VOLTORB: u8 = 100;

pub const ELECTRODE: u8 = 101;

pub const EXEGGCUTE: u8 = 102;

pub const EXEGGUTOR: u8 = 103;

pub const CUBONE: u8 = 104;

pub const MAROWAK: u8 = 105;

pub const HITMONLEE: u8 = 106;

pub const HITMONCHAN: u8 = 107;

pub const LICKITUNG: u8 = 108;

pub const KOFFING: u8 = 109;

pub const WEEZING: u8 = 110;

pub const RHYHORN: u8 = 111;

pub const RHYDON: u8 = 112;

pub const CHANSEY: u8 = 113;

pub const TANGELA: u8 = 114;

pub const KANGASKHAN: u8 = 115;

pub const HORSEA: u8 = 116;

pub const SEADRA: u8 = 117;

pub const GOLDEEN: u8 = 118;

pub const SEAKING: u8 = 119;

pub const STARYU: u8 = 120;

pub const STARMIE: u8 = 121;

pub const MR_MIME: u8 = 122;

pub const SCYTHER: u8 = 123;

pub const JYNX: u8 = 124;

pub const ELECTABUZZ: u8 = 125;

pub const MAGMAR: u8 = 126;

pub const PINSIR: u8 = 127;

pub const TAUROS: u8 = 128;

pub const MAGIKARP: u8 = 129;

pub const GYARADOS: u8 = 130;

pub const LAPRAS: u8 = 131;

pub const DITTO: u8 = 132;

pub const EEVEE: u8 = 133;

pub const VAPOREON: u8 = 134;

pub const JOLTEON: u8 = 135;

pub const FLAREON: u8 = 136;

pub const PORYGON: u8 = 137;

pub const OMANYTE: u8 = 138;

pub const OMASTAR: u8 = 139;

pub const KABUTO: u8 = 140;

pub const KABUTOPS: u8 = 141;

pub const AERODACTYL: u8 = 142;

pub const SNORLAX: u8 = 143;

pub const ARTICUNO: u8 = 144;

pub const ZAPDOS: u8 = 145;

pub const MOLTRES: u8 = 146;

pub const DRATINI: u8 = 147;

pub const DRAGONAIR: u8 = 148;

pub const DRAGONITE: u8 = 149;

pub const MEWTWO: u8 = 150;

pub const MEW: u8 = 151;

pub const JOHTO_POKEMON: u8 = 152;

pub const CHIKORITA: u8 = 152;

pub const BAYLEEF: u8 = 153;

pub const MEGANIUM: u8 = 154;

pub const CYNDAQUIL: u8 = 155;

pub const QUILAVA: u8 = 156;

pub const TYPHLOSION: u8 = 157;

pub const TOTODILE: u8 = 158;

pub const CROCONAW: u8 = 159;

pub const FERALIGATR: u8 = 160;

pub const SENTRET: u8 = 161;

pub const FURRET: u8 = 162;

pub const HOOTHOOT: u8 = 163;

pub const NOCTOWL: u8 = 164;

pub const LEDYBA: u8 = 165;

pub const LEDIAN: u8 = 166;

pub const SPINARAK: u8 = 167;

pub const ARIADOS: u8 = 168;

pub const CROBAT: u8 = 169;

pub const CHINCHOU: u8 = 170;

pub const LANTURN: u8 = 171;

pub const PICHU: u8 = 172;

pub const CLEFFA: u8 = 173;

pub const IGGLYBUFF: u8 = 174;

pub const TOGEPI: u8 = 175;

pub const TOGETIC: u8 = 176;

pub const NATU: u8 = 177;

pub const XATU: u8 = 178;

pub const MAREEP: u8 = 179;

pub const FLAAFFY: u8 = 180;

pub const AMPHAROS: u8 = 181;

pub const BELLOSSOM: u8 = 182;

pub const MARILL: u8 = 183;

pub const AZUMARILL: u8 = 184;

pub const SUDOWOODO: u8 = 185;

pub const POLITOED: u8 = 186;

pub const HOPPIP: u8 = 187;

pub const SKIPLOOM: u8 = 188;

pub const JUMPLUFF: u8 = 189;

pub const AIPOM: u8 = 190;

pub const SUNKERN: u8 = 191;

pub const SUNFLORA: u8 = 192;

pub const YANMA: u8 = 193;

pub const WOOPER: u8 = 194;

pub const QUAGSIRE: u8 = 195;

pub const ESPEON: u8 = 196;

pub const UMBREON: u8 = 197;

pub const MURKROW: u8 = 198;

pub const SLOWKING: u8 = 199;

pub const MISDREAVUS: u8 = 200;

pub const UNOWN: u8 = 201;

pub const WOBBUFFET: u8 = 202;

pub const GIRAFARIG: u8 = 203;

pub const PINECO: u8 = 204;

pub const FORRETRESS: u8 = 205;

pub const DUNSPARCE: u8 = 206;

pub const GLIGAR: u8 = 207;

pub const STEELIX: u8 = 208;

pub const SNUBBULL: u8 = 209;

pub const GRANBULL: u8 = 210;

pub const QWILFISH: u8 = 211;

pub const SCIZOR: u8 = 212;

pub const SHUCKLE: u8 = 213;

pub const HERACROSS: u8 = 214;

pub const SNEASEL: u8 = 215;

pub const TEDDIURSA: u8 = 216;

pub const URSARING: u8 = 217;

pub const SLUGMA: u8 = 218;

pub const MAGCARGO: u8 = 219;

pub const SWINUB: u8 = 220;

pub const PILOSWINE: u8 = 221;

pub const CORSOLA: u8 = 222;

pub const REMORAID: u8 = 223;

pub const OCTILLERY: u8 = 224;

pub const DELIBIRD: u8 = 225;

pub const MANTINE: u8 = 226;

pub const SKARMORY: u8 = 227;

pub const HOUNDOUR: u8 = 228;

pub const HOUNDOOM: u8 = 229;

pub const KINGDRA: u8 = 230;

pub const PHANPY: u8 = 231;

pub const DONPHAN: u8 = 232;

pub const PORYGON2: u8 = 233;

pub const STANTLER: u8 = 234;

pub const SMEARGLE: u8 = 235;

pub const TYROGUE: u8 = 236;

pub const HITMONTOP: u8 = 237;

pub const SMOOCHUM: u8 = 238;

pub const ELEKID: u8 = 239;

pub const MAGBY: u8 = 240;

pub const MILTANK: u8 = 241;

pub const BLISSEY: u8 = 242;

pub const RAIKOU: u8 = 243;

pub const ENTEI: u8 = 244;

pub const SUICUNE: u8 = 245;

pub const LARVITAR: u8 = 246;

pub const PUPITAR: u8 = 247;

pub const TYRANITAR: u8 = 248;

pub const LUGIA: u8 = 249;

pub const HO_OH: u8 = 250;

pub const CELEBI: u8 = 251;

pub const NUM_POKEMON: u8 = 251;

pub const EGG: u8 = 253;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const UNOWN_A: u8 = 1;

pub const UNOWN_B: u8 = 2;

pub const UNOWN_C: u8 = 3;

pub const UNOWN_D: u8 = 4;

pub const UNOWN_E: u8 = 5;

pub const UNOWN_F: u8 = 6;

pub const UNOWN_G: u8 = 7;

pub const UNOWN_H: u8 = 8;

pub const UNOWN_I: u8 = 9;

pub const UNOWN_J: u8 = 10;

pub const UNOWN_K: u8 = 11;

pub const UNOWN_L: u8 = 12;

pub const UNOWN_M: u8 = 13;

pub const UNOWN_N: u8 = 14;

pub const UNOWN_O: u8 = 15;

pub const UNOWN_P: u8 = 16;

pub const UNOWN_Q: u8 = 17;

pub const UNOWN_R: u8 = 18;

pub const UNOWN_S: u8 = 19;

pub const UNOWN_T: u8 = 20;

pub const UNOWN_U: u8 = 21;

pub const UNOWN_V: u8 = 22;

pub const UNOWN_W: u8 = 23;

pub const UNOWN_X: u8 = 24;

pub const UNOWN_Y: u8 = 25;

pub const UNOWN_Z: u8 = 26;

pub const NUM_UNOWN: u8 = 26;

