use crate::runtime::Cpu;

pub const _TRAINER_CLASS_: u8 = 0;

pub const CHRIS: u8 = 0;

pub const TRAINER_NONE: u8 = 0;

pub const CONST_VALUE: u8 = 1;

pub const CONST_INC: u8 = 1;

pub const PHONECONTACT_MOM: u8 = 1;

pub const PHONECONTACT_BIKESHOP: u8 = 2;

pub const PHONECONTACT_BILL: u8 = 3;

pub const PHONECONTACT_ELM: u8 = 4;

pub const PHONECONTACT_BUENA: u8 = 5;

pub const NUM_NONTRAINER_PHONECONTACTS: u8 = 5;

pub const KRIS: u8 = 1;

pub const FALKNER: u8 = 1;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const FALKNER1: u8 = 1;

pub const WHITNEY: u8 = 2;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const WHITNEY1: u8 = 1;

pub const BUGSY: u8 = 3;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const BUGSY1: u8 = 1;

pub const MORTY: u8 = 4;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const MORTY1: u8 = 1;

pub const PRYCE: u8 = 5;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const PRYCE1: u8 = 1;

pub const JASMINE: u8 = 6;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const JASMINE1: u8 = 1;

pub const CHUCK: u8 = 7;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const CHUCK1: u8 = 1;

pub const CLAIR: u8 = 8;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const CLAIR1: u8 = 1;

pub const RIVAL1: u8 = 9;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const RIVAL1_1_CHIKORITA: u8 = 1;

pub const RIVAL1_1_CYNDAQUIL: u8 = 2;

pub const RIVAL1_1_TOTODILE: u8 = 3;

pub const RIVAL1_2_CHIKORITA: u8 = 4;

pub const RIVAL1_2_CYNDAQUIL: u8 = 5;

pub const RIVAL1_2_TOTODILE: u8 = 6;

pub const RIVAL1_3_CHIKORITA: u8 = 7;

pub const RIVAL1_3_CYNDAQUIL: u8 = 8;

pub const RIVAL1_3_TOTODILE: u8 = 9;

pub const RIVAL1_4_CHIKORITA: u8 = 10;

pub const RIVAL1_4_CYNDAQUIL: u8 = 11;

pub const RIVAL1_4_TOTODILE: u8 = 12;

pub const RIVAL1_5_CHIKORITA: u8 = 13;

pub const RIVAL1_5_CYNDAQUIL: u8 = 14;

pub const RIVAL1_5_TOTODILE: u8 = 15;

pub const POKEMON_PROF: u8 = 10;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const WILL: u8 = 11;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const WILL1: u8 = 1;

pub const CAL: u8 = 12;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const CAL1: u8 = 1;

pub const CAL2: u8 = 2;

pub const CAL3: u8 = 3;

pub const BRUNO: u8 = 13;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const BRUNO1: u8 = 1;

pub const KAREN: u8 = 14;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const KAREN1: u8 = 1;

pub const KOGA: u8 = 15;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const KOGA1: u8 = 1;

pub const CHAMPION: u8 = 16;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const LANCE: u8 = 1;

pub const BROCK: u8 = 17;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const BROCK1: u8 = 1;

pub const MISTY: u8 = 18;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const MISTY1: u8 = 1;

pub const LT_SURGE: u8 = 19;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const LT_SURGE1: u8 = 1;

pub const SCIENTIST: u8 = 20;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const ROSS: u8 = 1;

pub const MITCH: u8 = 2;

pub const JED: u8 = 3;

pub const MARC: u8 = 4;

pub const RICH: u8 = 5;

pub const ERIKA: u8 = 21;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const ERIKA1: u8 = 1;

pub const YOUNGSTER: u8 = 22;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const JOEY1: u8 = 1;

pub const MIKEY: u8 = 2;

pub const ALBERT: u8 = 3;

pub const GORDON: u8 = 4;

pub const SAMUEL: u8 = 5;

pub const IAN: u8 = 6;

pub const JOEY2: u8 = 7;

pub const JOEY3: u8 = 8;

pub const WARREN: u8 = 9;

pub const JIMMY: u8 = 10;

pub const OWEN: u8 = 11;

pub const JASON: u8 = 12;

pub const JOEY4: u8 = 13;

pub const JOEY5: u8 = 14;

pub const SCHOOLBOY: u8 = 23;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const JACK1: u8 = 1;

pub const KIPP: u8 = 2;

pub const ALAN1: u8 = 3;

pub const JOHNNY: u8 = 4;

pub const DANNY: u8 = 5;

pub const TOMMY: u8 = 6;

pub const DUDLEY: u8 = 7;

pub const JOE: u8 = 8;

pub const BILLY: u8 = 9;

pub const CHAD1: u8 = 10;

pub const NATE: u8 = 11;

pub const RICKY: u8 = 12;

pub const JACK2: u8 = 13;

pub const JACK3: u8 = 14;

pub const ALAN2: u8 = 15;

pub const ALAN3: u8 = 16;

pub const CHAD2: u8 = 17;

pub const CHAD3: u8 = 18;

pub const JACK4: u8 = 19;

pub const JACK5: u8 = 20;

pub const ALAN4: u8 = 21;

pub const ALAN5: u8 = 22;

pub const CHAD4: u8 = 23;

pub const CHAD5: u8 = 24;

pub const BIRD_KEEPER: u8 = 24;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const ROD: u8 = 1;

pub const ABE: u8 = 2;

pub const BRYAN: u8 = 3;

pub const THEO: u8 = 4;

pub const TOBY: u8 = 5;

pub const DENIS: u8 = 6;

pub const VANCE1: u8 = 7;

pub const HANK: u8 = 8;

pub const ROY: u8 = 9;

pub const BORIS: u8 = 10;

pub const BOB: u8 = 11;

pub const JOSE1: u8 = 12;

pub const PETER: u8 = 13;

pub const JOSE2: u8 = 14;

pub const PERRY: u8 = 15;

pub const BRET: u8 = 16;

pub const JOSE3: u8 = 17;

pub const VANCE2: u8 = 18;

pub const VANCE3: u8 = 19;

pub const LASS: u8 = 25;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const CARRIE: u8 = 1;

pub const BRIDGET: u8 = 2;

pub const ALICE: u8 = 3;

pub const KRISE: u8 = 4;

pub const CONNIE1: u8 = 5;

pub const LINDA: u8 = 6;

pub const LAURA: u8 = 7;

pub const SHANNON: u8 = 8;

pub const MICHELLE: u8 = 9;

pub const DANA1: u8 = 10;

pub const ELLEN: u8 = 11;

pub const CONNIE2: u8 = 12;

pub const CONNIE3: u8 = 13;

pub const DANA2: u8 = 14;

pub const DANA3: u8 = 15;

pub const DANA4: u8 = 16;

pub const DANA5: u8 = 17;

pub const JANINE: u8 = 26;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const JANINE1: u8 = 1;

pub const COOLTRAINERM: u8 = 27;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const NICK: u8 = 1;

pub const AARON: u8 = 2;

pub const PAUL: u8 = 3;

pub const CODY: u8 = 4;

pub const MIKE: u8 = 5;

pub const GAVEN1: u8 = 6;

pub const GAVEN2: u8 = 7;

pub const RYAN: u8 = 8;

pub const JAKE: u8 = 9;

pub const GAVEN3: u8 = 10;

pub const BLAKE: u8 = 11;

pub const BRIAN: u8 = 12;

pub const ERICK: u8 = 13;

pub const ANDY: u8 = 14;

pub const TYLER: u8 = 15;

pub const SEAN: u8 = 16;

pub const KEVIN: u8 = 17;

pub const STEVE: u8 = 18;

pub const ALLEN: u8 = 19;

pub const DARIN: u8 = 20;

pub const COOLTRAINERF: u8 = 28;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const GWEN: u8 = 1;

pub const LOIS: u8 = 2;

pub const FRAN: u8 = 3;

pub const LOLA: u8 = 4;

pub const KATE: u8 = 5;

pub const IRENE: u8 = 6;

pub const KELLY: u8 = 7;

pub const JOYCE: u8 = 8;

pub const BETH1: u8 = 9;

pub const REENA1: u8 = 10;

pub const MEGAN: u8 = 11;

pub const BETH2: u8 = 12;

pub const CAROL: u8 = 13;

pub const QUINN: u8 = 14;

pub const EMMA: u8 = 15;

pub const CYBIL: u8 = 16;

pub const JENN: u8 = 17;

pub const BETH3: u8 = 18;

pub const REENA2: u8 = 19;

pub const REENA3: u8 = 20;

pub const CARA: u8 = 21;

pub const BEAUTY: u8 = 29;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const VICTORIA: u8 = 1;

pub const SAMANTHA: u8 = 2;

pub const JULIE: u8 = 3;

pub const JACLYN: u8 = 4;

pub const BRENDA: u8 = 5;

pub const CASSIE: u8 = 6;

pub const CAROLINE: u8 = 7;

pub const CARLENE: u8 = 8;

pub const JESSICA: u8 = 9;

pub const RACHAEL: u8 = 10;

pub const ANGELICA: u8 = 11;

pub const KENDRA: u8 = 12;

pub const VERONICA: u8 = 13;

pub const JULIA: u8 = 14;

pub const THERESA: u8 = 15;

pub const VALERIE: u8 = 16;

pub const OLIVIA: u8 = 17;

pub const POKEMANIAC: u8 = 30;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const LARRY: u8 = 1;

pub const ANDREW: u8 = 2;

pub const CALVIN: u8 = 3;

pub const SHANE: u8 = 4;

pub const BEN: u8 = 5;

pub const BRENT1: u8 = 6;

pub const RON: u8 = 7;

pub const ETHAN: u8 = 8;

pub const BRENT2: u8 = 9;

pub const BRENT3: u8 = 10;

pub const ISSAC: u8 = 11;

pub const DONALD: u8 = 12;

pub const ZACH: u8 = 13;

pub const BRENT4: u8 = 14;

pub const MILLER: u8 = 15;

pub const GRUNTM: u8 = 31;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const GRUNTM_1: u8 = 1;

pub const GRUNTM_2: u8 = 2;

pub const GRUNTM_3: u8 = 3;

pub const GRUNTM_4: u8 = 4;

pub const GRUNTM_5: u8 = 5;

pub const GRUNTM_6: u8 = 6;

pub const GRUNTM_7: u8 = 7;

pub const GRUNTM_8: u8 = 8;

pub const GRUNTM_9: u8 = 9;

pub const GRUNTM_10: u8 = 10;

pub const GRUNTM_11: u8 = 11;

pub const GRUNTM_12: u8 = 12;

pub const GRUNTM_13: u8 = 13;

pub const GRUNTM_14: u8 = 14;

pub const GRUNTM_15: u8 = 15;

pub const GRUNTM_16: u8 = 16;

pub const GRUNTM_17: u8 = 17;

pub const GRUNTM_18: u8 = 18;

pub const GRUNTM_19: u8 = 19;

pub const GRUNTM_20: u8 = 20;

pub const GRUNTM_21: u8 = 21;

pub const GRUNTM_22: u8 = 22;

pub const GRUNTM_23: u8 = 23;

pub const GRUNTM_24: u8 = 24;

pub const GRUNTM_25: u8 = 25;

pub const GRUNTM_26: u8 = 26;

pub const GRUNTM_27: u8 = 27;

pub const GRUNTM_28: u8 = 28;

pub const GRUNTM_29: u8 = 29;

pub const GRUNTM_30: u8 = 30;

pub const GRUNTM_31: u8 = 31;

pub const GENTLEMAN: u8 = 32;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const PRESTON: u8 = 1;

pub const EDWARD: u8 = 2;

pub const GREGORY: u8 = 3;

pub const VIRGIL: u8 = 4;

pub const ALFRED: u8 = 5;

pub const SKIER: u8 = 33;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const ROXANNE: u8 = 1;

pub const CLARISSA: u8 = 2;

pub const TEACHER: u8 = 34;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const COLETTE: u8 = 1;

pub const HILLARY: u8 = 2;

pub const SHIRLEY: u8 = 3;

pub const SABRINA: u8 = 35;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const SABRINA1: u8 = 1;

pub const BUG_CATCHER: u8 = 36;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const DON: u8 = 1;

pub const ROB: u8 = 2;

pub const ED: u8 = 3;

pub const WADE1: u8 = 4;

pub const BUG_CATCHER_BENNY: u8 = 5;

pub const AL: u8 = 6;

pub const JOSH: u8 = 7;

pub const ARNIE1: u8 = 8;

pub const KEN: u8 = 9;

pub const WADE2: u8 = 10;

pub const WADE3: u8 = 11;

pub const DOUG: u8 = 12;

pub const ARNIE2: u8 = 13;

pub const ARNIE3: u8 = 14;

pub const WADE4: u8 = 15;

pub const WADE5: u8 = 16;

pub const ARNIE4: u8 = 17;

pub const ARNIE5: u8 = 18;

pub const WAYNE: u8 = 19;

pub const FISHER: u8 = 37;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const JUSTIN: u8 = 1;

pub const RALPH1: u8 = 2;

pub const ARNOLD: u8 = 3;

pub const KYLE: u8 = 4;

pub const HENRY: u8 = 5;

pub const MARVIN: u8 = 6;

pub const TULLY1: u8 = 7;

pub const ANDRE: u8 = 8;

pub const RAYMOND: u8 = 9;

pub const WILTON1: u8 = 10;

pub const EDGAR: u8 = 11;

pub const JONAH: u8 = 12;

pub const MARTIN: u8 = 13;

pub const STEPHEN: u8 = 14;

pub const BARNEY: u8 = 15;

pub const RALPH2: u8 = 16;

pub const RALPH3: u8 = 17;

pub const TULLY2: u8 = 18;

pub const TULLY3: u8 = 19;

pub const WILTON2: u8 = 20;

pub const SCOTT: u8 = 21;

pub const WILTON3: u8 = 22;

pub const RALPH4: u8 = 23;

pub const RALPH5: u8 = 24;

pub const TULLY4: u8 = 25;

pub const SWIMMERM: u8 = 38;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const HAROLD: u8 = 1;

pub const SIMON: u8 = 2;

pub const RANDALL: u8 = 3;

pub const CHARLIE: u8 = 4;

pub const GEORGE: u8 = 5;

pub const BERKE: u8 = 6;

pub const KIRK: u8 = 7;

pub const MATHEW: u8 = 8;

pub const HAL: u8 = 9;

pub const PATON: u8 = 10;

pub const DARYL: u8 = 11;

pub const WALTER: u8 = 12;

pub const TONY: u8 = 13;

pub const JEROME: u8 = 14;

pub const TUCKER: u8 = 15;

pub const RICK: u8 = 16;

pub const CAMERON: u8 = 17;

pub const SETH: u8 = 18;

pub const JAMES: u8 = 19;

pub const LEWIS: u8 = 20;

pub const PARKER: u8 = 21;

pub const SWIMMERF: u8 = 39;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const ELAINE: u8 = 1;

pub const PAULA: u8 = 2;

pub const KAYLEE: u8 = 3;

pub const SUSIE: u8 = 4;

pub const DENISE: u8 = 5;

pub const KARA: u8 = 6;

pub const WENDY: u8 = 7;

pub const LISA: u8 = 8;

pub const JILL: u8 = 9;

pub const MARY: u8 = 10;

pub const KATIE: u8 = 11;

pub const DAWN: u8 = 12;

pub const TARA: u8 = 13;

pub const NICOLE: u8 = 14;

pub const LORI: u8 = 15;

pub const JODY: u8 = 16;

pub const NIKKI: u8 = 17;

pub const DIANA: u8 = 18;

pub const BRIANA: u8 = 19;

pub const SAILOR: u8 = 40;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const EUGENE: u8 = 1;

pub const HUEY1: u8 = 2;

pub const TERRELL: u8 = 3;

pub const KENT: u8 = 4;

pub const ERNEST: u8 = 5;

pub const JEFF: u8 = 6;

pub const GARRETT: u8 = 7;

pub const KENNETH: u8 = 8;

pub const STANLY: u8 = 9;

pub const HARRY: u8 = 10;

pub const HUEY2: u8 = 11;

pub const HUEY3: u8 = 12;

pub const HUEY4: u8 = 13;

pub const SUPER_NERD: u8 = 41;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const STAN: u8 = 1;

pub const ERIC: u8 = 2;

pub const GREGG: u8 = 3;

pub const JAY: u8 = 4;

pub const DAVE: u8 = 5;

pub const SAM: u8 = 6;

pub const TOM: u8 = 7;

pub const PAT: u8 = 8;

pub const SHAWN: u8 = 9;

pub const TERU: u8 = 10;

pub const RUSS: u8 = 11;

pub const NORTON: u8 = 12;

pub const HUGH: u8 = 13;

pub const MARKUS: u8 = 14;

pub const RIVAL2: u8 = 42;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const RIVAL2_1_CHIKORITA: u8 = 1;

pub const RIVAL2_1_CYNDAQUIL: u8 = 2;

pub const RIVAL2_1_TOTODILE: u8 = 3;

pub const RIVAL2_2_CHIKORITA: u8 = 4;

pub const RIVAL2_2_CYNDAQUIL: u8 = 5;

pub const RIVAL2_2_TOTODILE: u8 = 6;

pub const GUITARIST: u8 = 43;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const CLYDE: u8 = 1;

pub const VINCENT: u8 = 2;

pub const HIKER: u8 = 44;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const ANTHONY1: u8 = 1;

pub const RUSSELL: u8 = 2;

pub const PHILLIP: u8 = 3;

pub const LEONARD: u8 = 4;

pub const ANTHONY2: u8 = 5;

pub const BENJAMIN: u8 = 6;

pub const ERIK: u8 = 7;

pub const MICHAEL: u8 = 8;

pub const PARRY1: u8 = 9;

pub const TIMOTHY: u8 = 10;

pub const BAILEY: u8 = 11;

pub const ANTHONY3: u8 = 12;

pub const TIM: u8 = 13;

pub const NOLAND: u8 = 14;

pub const SIDNEY: u8 = 15;

pub const KENNY: u8 = 16;

pub const JIM: u8 = 17;

pub const DANIEL: u8 = 18;

pub const PARRY2: u8 = 19;

pub const PARRY3: u8 = 20;

pub const ANTHONY4: u8 = 21;

pub const ANTHONY5: u8 = 22;

pub const BIKER: u8 = 45;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const BIKER_BENNY: u8 = 1;

pub const KAZU: u8 = 2;

pub const DWAYNE: u8 = 3;

pub const HARRIS: u8 = 4;

pub const ZEKE: u8 = 5;

pub const CHARLES: u8 = 6;

pub const RILEY: u8 = 7;

pub const JOEL: u8 = 8;

pub const GLENN: u8 = 9;

pub const BLAINE: u8 = 46;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const BLAINE1: u8 = 1;

pub const BURGLAR: u8 = 47;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const DUNCAN: u8 = 1;

pub const EDDIE: u8 = 2;

pub const COREY: u8 = 3;

pub const FIREBREATHER: u8 = 48;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const OTIS: u8 = 1;

pub const DICK: u8 = 2;

pub const NED: u8 = 3;

pub const BURT: u8 = 4;

pub const BILL: u8 = 5;

pub const WALT: u8 = 6;

pub const RAY: u8 = 7;

pub const LYLE: u8 = 8;

pub const JUGGLER: u8 = 49;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const IRWIN1: u8 = 1;

pub const FRITZ: u8 = 2;

pub const HORTON: u8 = 3;

pub const IRWIN2: u8 = 4;

pub const IRWIN3: u8 = 5;

pub const IRWIN4: u8 = 6;

pub const BLACKBELT_T: u8 = 50;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const KENJI1: u8 = 1;

pub const YOSHI: u8 = 2;

pub const KENJI2: u8 = 3;

pub const LAO: u8 = 4;

pub const NOB: u8 = 5;

pub const KIYO: u8 = 6;

pub const LUNG: u8 = 7;

pub const KENJI3: u8 = 8;

pub const WAI: u8 = 9;

pub const EXECUTIVEM: u8 = 51;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const EXECUTIVEM_1: u8 = 1;

pub const EXECUTIVEM_2: u8 = 2;

pub const EXECUTIVEM_3: u8 = 3;

pub const EXECUTIVEM_4: u8 = 4;

pub const PSYCHIC_T: u8 = 52;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const NATHAN: u8 = 1;

pub const FRANKLIN: u8 = 2;

pub const HERMAN: u8 = 3;

pub const FIDEL: u8 = 4;

pub const GREG: u8 = 5;

pub const NORMAN: u8 = 6;

pub const MARK: u8 = 7;

pub const PHIL: u8 = 8;

pub const RICHARD: u8 = 9;

pub const GILBERT: u8 = 10;

pub const JARED: u8 = 11;

pub const RODNEY: u8 = 12;

pub const PICNICKER: u8 = 53;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const LIZ1: u8 = 1;

pub const GINA1: u8 = 2;

pub const BROOKE: u8 = 3;

pub const KIM: u8 = 4;

pub const CINDY: u8 = 5;

pub const HOPE: u8 = 6;

pub const SHARON: u8 = 7;

pub const DEBRA: u8 = 8;

pub const GINA2: u8 = 9;

pub const ERIN1: u8 = 10;

pub const LIZ2: u8 = 11;

pub const LIZ3: u8 = 12;

pub const HEIDI: u8 = 13;

pub const EDNA: u8 = 14;

pub const GINA3: u8 = 15;

pub const TIFFANY1: u8 = 16;

pub const TIFFANY2: u8 = 17;

pub const ERIN2: u8 = 18;

pub const TANYA: u8 = 19;

pub const TIFFANY3: u8 = 20;

pub const ERIN3: u8 = 21;

pub const LIZ4: u8 = 22;

pub const LIZ5: u8 = 23;

pub const GINA4: u8 = 24;

pub const GINA5: u8 = 25;

pub const TIFFANY4: u8 = 26;

pub const CAMPER: u8 = 54;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const ROLAND: u8 = 1;

pub const TODD1: u8 = 2;

pub const IVAN: u8 = 3;

pub const ELLIOT: u8 = 4;

pub const BARRY: u8 = 5;

pub const LLOYD: u8 = 6;

pub const DEAN: u8 = 7;

pub const SID: u8 = 8;

pub const HARVEY: u8 = 9;

pub const DALE: u8 = 10;

pub const TED: u8 = 11;

pub const TODD2: u8 = 12;

pub const TODD3: u8 = 13;

pub const THOMAS: u8 = 14;

pub const LEROY: u8 = 15;

pub const DAVID: u8 = 16;

pub const JOHN: u8 = 17;

pub const JERRY: u8 = 18;

pub const SPENCER: u8 = 19;

pub const TODD4: u8 = 20;

pub const TODD5: u8 = 21;

pub const QUENTIN: u8 = 22;

pub const EXECUTIVEF: u8 = 55;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const EXECUTIVEF_1: u8 = 1;

pub const EXECUTIVEF_2: u8 = 2;

pub const SAGE: u8 = 56;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const CHOW: u8 = 1;

pub const NICO: u8 = 2;

pub const JIN: u8 = 3;

pub const TROY: u8 = 4;

pub const JEFFREY: u8 = 5;

pub const PING: u8 = 6;

pub const EDMOND: u8 = 7;

pub const NEAL: u8 = 8;

pub const LI: u8 = 9;

pub const GAKU: u8 = 10;

pub const MASA: u8 = 11;

pub const KOJI: u8 = 12;

pub const MEDIUM: u8 = 57;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const MARTHA: u8 = 1;

pub const GRACE: u8 = 2;

pub const BETHANY: u8 = 3;

pub const MARGRET: u8 = 4;

pub const ETHEL: u8 = 5;

pub const REBECCA: u8 = 6;

pub const DORIS: u8 = 7;

pub const BOARDER: u8 = 58;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const RONALD: u8 = 1;

pub const BRAD: u8 = 2;

pub const DOUGLAS: u8 = 3;

pub const POKEFANM: u8 = 59;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const WILLIAM: u8 = 1;

pub const DEREK1: u8 = 2;

pub const ROBERT: u8 = 3;

pub const JOSHUA: u8 = 4;

pub const CARTER: u8 = 5;

pub const TREVOR: u8 = 6;

pub const BRANDON: u8 = 7;

pub const JEREMY: u8 = 8;

pub const COLIN: u8 = 9;

pub const DEREK2: u8 = 10;

pub const DEREK3: u8 = 11;

pub const ALEX: u8 = 12;

pub const REX: u8 = 13;

pub const ALLAN: u8 = 14;

pub const KIMONO_GIRL: u8 = 60;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const NAOKO_UNUSED: u8 = 1;

pub const NAOKO: u8 = 2;

pub const SAYO: u8 = 3;

pub const ZUKI: u8 = 4;

pub const KUNI: u8 = 5;

pub const MIKI: u8 = 6;

pub const TWINS: u8 = 61;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const AMYANDMAY1: u8 = 1;

pub const ANNANDANNE1: u8 = 2;

pub const ANNANDANNE2: u8 = 3;

pub const AMYANDMAY2: u8 = 4;

pub const JOANDZOE1: u8 = 5;

pub const JOANDZOE2: u8 = 6;

pub const MEGANDPEG1: u8 = 7;

pub const MEGANDPEG2: u8 = 8;

pub const LEAANDPIA1: u8 = 9;

pub const LEAANDPIA2: u8 = 10;

pub const POKEFANF: u8 = 62;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const BEVERLY1: u8 = 1;

pub const RUTH: u8 = 2;

pub const BEVERLY2: u8 = 3;

pub const BEVERLY3: u8 = 4;

pub const GEORGIA: u8 = 5;

pub const JAIME: u8 = 6;

pub const RED: u8 = 63;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const RED1: u8 = 1;

pub const BLUE: u8 = 64;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const BLUE1: u8 = 1;

pub const OFFICER: u8 = 65;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const KEITH: u8 = 1;

pub const DIRK: u8 = 2;

pub const GRUNTF: u8 = 66;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const GRUNTF_1: u8 = 1;

pub const GRUNTF_2: u8 = 2;

pub const GRUNTF_3: u8 = 3;

pub const GRUNTF_4: u8 = 4;

pub const GRUNTF_5: u8 = 5;

pub const MYSTICALMAN: u8 = 67;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const EUSINE: u8 = 1;

pub const NUM_TRAINER_CLASSES: u8 = 67;

