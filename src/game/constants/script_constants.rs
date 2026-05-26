use crate::runtime::Cpu;

pub const PLAYER: u8 = 0;

pub const LAST_TALKED: i16 = -2;

pub const CONST_VALUE: u8 = 0;

pub const CONST_INC: u8 = 1;

pub const STRING_BUFFER_3: u8 = 0;

pub const STRING_BUFFER_4: u8 = 1;

pub const STRING_BUFFER_5: u8 = 2;

pub const NUM_STRING_BUFFERS: u8 = 3;

pub const STRING_BUFFER_LENGTH: u8 = 19;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const YOUR_MONEY: u8 = 0;

pub const MOMS_MONEY: u8 = 1;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const HAVE_MORE: u8 = 0;

pub const HAVE_AMOUNT: u8 = 1;

pub const HAVE_LESS: u8 = 2;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const POKEMAIL_WRONG_MAIL: u8 = 0;

pub const POKEMAIL_CORRECT: u8 = 1;

pub const POKEMAIL_REFUSED: u8 = 2;

pub const POKEMAIL_NO_MAIL: u8 = 3;

pub const POKEMAIL_LAST_MON: u8 = 4;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const PHONE_CONTACT_GOT: u8 = 0;

pub const PHONE_CONTACTS_FULL: u8 = 1;

pub const PHONE_CONTACT_REFUSED: u8 = 2;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const TRAINERTEXT_SEEN: u8 = 0;

pub const TRAINERTEXT_WIN: u8 = 1;

pub const TRAINERTEXT_LOSS: u8 = 2;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const VAR_STRINGBUFFER2: u8 = 0;

pub const VAR_PARTYCOUNT: u8 = 1;

pub const VAR_BATTLERESULT: u8 = 2;

pub const VAR_BATTLETYPE: u8 = 3;

pub const VAR_TIMEOFDAY: u8 = 4;

pub const VAR_DEXCAUGHT: u8 = 5;

pub const VAR_DEXSEEN: u8 = 6;

pub const VAR_BADGES: u8 = 7;

pub const VAR_MOVEMENT: u8 = 8;

pub const VAR_FACING: u8 = 9;

pub const VAR_HOUR: u8 = 10;

pub const VAR_WEEKDAY: u8 = 11;

pub const VAR_MAPGROUP: u8 = 12;

pub const VAR_MAPNUMBER: u8 = 13;

pub const VAR_UNOWNCOUNT: u8 = 14;

pub const VAR_ENVIRONMENT: u8 = 15;

pub const VAR_BOXSPACE: u8 = 16;

pub const VAR_CONTESTMINUTES: u8 = 17;

pub const VAR_XCOORD: u8 = 18;

pub const VAR_YCOORD: u8 = 19;

pub const VAR_SPECIALPHONECALL: u8 = 20;

pub const VAR_BT_WIN_STREAK: u8 = 21;

pub const VAR_KURT_APRICORNS: u8 = 22;

pub const VAR_CALLERID: u8 = 23;

pub const VAR_BLUECARDBALANCE: u8 = 24;

pub const VAR_BUENASPASSWORD: u8 = 25;

pub const VAR_KENJI_BREAK: u8 = 26;

pub const NUM_VARS: u8 = 27;

pub const RETVAR_STRBUF2: u8 = 0;

pub const RETVAR_ADDR_DE: u8 = 64;

pub const RETVAR_EXECUTE: u8 = 128;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const PLAYEREVENT_MAPSCRIPT: i16 = -1;

pub const PLAYEREVENT_NONE: u8 = 0;

pub const PLAYEREVENT_SEENBYTRAINER: u8 = 1;

pub const PLAYEREVENT_TALKTOTRAINER: u8 = 2;

pub const PLAYEREVENT_ITEMBALL: u8 = 3;

pub const PLAYEREVENT_CONNECTION: u8 = 4;

pub const PLAYEREVENT_WARP: u8 = 5;

pub const PLAYEREVENT_FALL: u8 = 6;

pub const PLAYEREVENT_WHITEOUT: u8 = 7;

pub const PLAYEREVENT_HATCH: u8 = 8;

pub const PLAYEREVENT_JOYCHANGEFACING: u8 = 9;

pub const NUM_PLAYER_EVENTS: u8 = 10;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const PLAYERMOVEMENT_NORMAL: u8 = 0;

pub const PLAYERMOVEMENT_WARP: u8 = 1;

pub const PLAYERMOVEMENT_TURN: u8 = 2;

pub const PLAYERMOVEMENT_FORCE_TURN: u8 = 3;

pub const PLAYERMOVEMENT_FINISH: u8 = 4;

pub const PLAYERMOVEMENT_CONTINUE: u8 = 5;

pub const PLAYERMOVEMENT_EXIT_WATER: u8 = 6;

pub const PLAYERMOVEMENT_JUMP: u8 = 7;

pub const NUM_PLAYER_MOVEMENTS: u8 = 8;

pub const SCENE_SCRIPT_SIZE: u8 = 4;

pub const CALLBACK_SIZE: u8 = 3;

pub const WARP_EVENT_SIZE: u8 = 5;

pub const COORD_EVENT_SIZE: u8 = 8;

pub const BG_EVENT_SIZE: u8 = 5;

pub const OBJECT_EVENT_SIZE: u8 = 13;

pub const SCENE_ALWAYS: i16 = -1;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const BGEVENT_READ: u8 = 0;

pub const BGEVENT_UP: u8 = 1;

pub const BGEVENT_DOWN: u8 = 2;

pub const BGEVENT_RIGHT: u8 = 3;

pub const BGEVENT_LEFT: u8 = 4;

pub const BGEVENT_IFSET: u8 = 5;

pub const BGEVENT_IFNOTSET: u8 = 6;

pub const BGEVENT_ITEM: u8 = 7;

pub const BGEVENT_COPY: u8 = 8;

pub const NUM_BGEVENTS: u8 = 9;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const OBJECTTYPE_SCRIPT: u8 = 0;

pub const OBJECTTYPE_ITEMBALL: u8 = 1;

pub const OBJECTTYPE_TRAINER: u8 = 2;

pub const OBJECTTYPE_3: u8 = 3;

pub const OBJECTTYPE_4: u8 = 4;

pub const OBJECTTYPE_5: u8 = 5;

pub const OBJECTTYPE_6: u8 = 6;

pub const NUM_OBJECT_TYPES: u8 = 7;

pub const CMDQUEUE_ENTRY_SIZE: u8 = 6;

pub const CMDQUEUE_CAPACITY: u8 = 4;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const CMDQUEUE_NULL: u8 = 0;

pub const CMDQUEUE_TYPE1: u8 = 1;

pub const CMDQUEUE_STONETABLE: u8 = 2;

pub const CMDQUEUE_TYPE3: u8 = 3;

pub const CMDQUEUE_TYPE4: u8 = 4;

pub const NUM_CMDQUEUE_TYPES: u8 = 5;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const FLOOR_B4F: u8 = 0;

pub const FLOOR_B3F: u8 = 1;

pub const FLOOR_B2F: u8 = 2;

pub const FLOOR_B1F: u8 = 3;

pub const FLOOR_1F: u8 = 4;

pub const FLOOR_2F: u8 = 5;

pub const FLOOR_3F: u8 = 6;

pub const FLOOR_4F: u8 = 7;

pub const FLOOR_5F: u8 = 8;

pub const FLOOR_6F: u8 = 9;

pub const FLOOR_7F: u8 = 10;

pub const FLOOR_8F: u8 = 11;

pub const FLOOR_9F: u8 = 12;

pub const FLOOR_10F: u8 = 13;

pub const FLOOR_11F: u8 = 14;

pub const FLOOR_ROOF: u8 = 15;

pub const NUM_FLOORS: u8 = 16;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const EMOTE_SHOCK: u8 = 0;

pub const EMOTE_QUESTION: u8 = 1;

pub const EMOTE_HAPPY: u8 = 2;

pub const EMOTE_SAD: u8 = 3;

pub const EMOTE_HEART: u8 = 4;

pub const EMOTE_BOLT: u8 = 5;

pub const EMOTE_SLEEP: u8 = 6;

pub const EMOTE_FISH: u8 = 7;

pub const EMOTE_SHADOW: u8 = 8;

pub const EMOTE_ROD: u8 = 9;

pub const EMOTE_BOULDER_DUST: u8 = 10;

pub const EMOTE_GRASS_RUSTLE: u8 = 11;

pub const NUM_EMOTES: u8 = 12;

pub const EMOTE_FROM_MEM: i16 = -1;

pub const EMOTE_LENGTH: u8 = 6;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const FRUITTREE_ROUTE_29: u8 = 1;

pub const FRUITTREE_ROUTE_30_1: u8 = 2;

pub const FRUITTREE_ROUTE_38: u8 = 3;

pub const FRUITTREE_ROUTE_46_1: u8 = 4;

pub const FRUITTREE_ROUTE_30_2: u8 = 5;

pub const FRUITTREE_ROUTE_33: u8 = 6;

pub const FRUITTREE_ROUTE_31: u8 = 7;

pub const FRUITTREE_ROUTE_43: u8 = 8;

pub const FRUITTREE_VIOLET_CITY: u8 = 9;

pub const FRUITTREE_ROUTE_46_2: u8 = 10;

pub const FRUITTREE_ROUTE_35: u8 = 11;

pub const FRUITTREE_ROUTE_45: u8 = 12;

pub const FRUITTREE_ROUTE_36: u8 = 13;

pub const FRUITTREE_ROUTE_26: u8 = 14;

pub const FRUITTREE_ROUTE_39: u8 = 15;

pub const FRUITTREE_ROUTE_44: u8 = 16;

pub const FRUITTREE_ROUTE_37_1: u8 = 17;

pub const FRUITTREE_ROUTE_37_2: u8 = 18;

pub const FRUITTREE_ROUTE_37_3: u8 = 19;

pub const FRUITTREE_AZALEA_TOWN: u8 = 20;

pub const FRUITTREE_ROUTE_42_1: u8 = 21;

pub const FRUITTREE_ROUTE_42_2: u8 = 22;

pub const FRUITTREE_ROUTE_42_3: u8 = 23;

pub const FRUITTREE_ROUTE_11: u8 = 24;

pub const FRUITTREE_ROUTE_2: u8 = 25;

pub const FRUITTREE_ROUTE_1: u8 = 26;

pub const FRUITTREE_ROUTE_8: u8 = 27;

pub const FRUITTREE_PEWTER_CITY_1: u8 = 28;

pub const FRUITTREE_PEWTER_CITY_2: u8 = 29;

pub const FRUITTREE_FUCHSIA_CITY: u8 = 30;

pub const NUM_FRUIT_TREES: u8 = 30;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const DECODESC_POSTER: u8 = 0;

pub const DECODESC_LEFT_DOLL: u8 = 1;

pub const DECODESC_RIGHT_DOLL: u8 = 2;

pub const DECODESC_BIG_DOLL: u8 = 3;

pub const DECODESC_CONSOLE: u8 = 4;

pub const NUM_DECODESCS: u8 = 5;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const SWARM_DUNSPARCE: u8 = 0;

pub const SWARM_YANMA: u8 = 1;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const FISHSWARM_NONE: u8 = 0;

pub const FISHSWARM_QWILFISH: u8 = 1;

pub const FISHSWARM_REMORAID: u8 = 2;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const GBCHECK_GB: u8 = 0;

pub const GBCHECK_SGB: u8 = 1;

pub const GBCHECK_CGB: u8 = 2;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const MAGIKARPLENGTH_NOT_MAGIKARP: u8 = 0;

pub const MAGIKARPLENGTH_REFUSED: u8 = 1;

pub const MAGIKARPLENGTH_TOO_SHORT: u8 = 2;

pub const MAGIKARPLENGTH_BEAT_RECORD: u8 = 3;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const SHUCKIE_WRONG_MON: u8 = 0;

pub const SHUCKIE_REFUSED: u8 = 1;

pub const SHUCKIE_RETURNED: u8 = 2;

pub const SHUCKIE_HAPPY: u8 = 3;

pub const SHUCKIE_FAINTED: u8 = 4;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const BUGCONTEST_CAUGHT_MON: u8 = 0;

pub const BUGCONTEST_BOXED_MON: u8 = 1;

pub const BUGCONTEST_NO_CATCH: u8 = 2;

pub const BUG_CONTEST_BALLS: u8 = 20;

pub const BUG_CONTEST_MINUTES: u8 = 20;

pub const BUG_CONTEST_SECONDS: u8 = 0;

pub const BUG_CONTEST_PLAYER: u8 = 1;

pub const NUM_BUG_CONTESTANTS: u8 = 10;

pub const BUG_CONTESTANT_SIZE: u8 = 4;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const HEALMACHINE_POKECENTER: u8 = 0;

pub const HEALMACHINE_ELMS_LAB: u8 = 1;

pub const HEALMACHINE_HALL_OF_FAME: u8 = 2;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const UNOWNPUZZLE_KABUTO: u8 = 0;

pub const UNOWNPUZZLE_OMANYTE: u8 = 1;

pub const UNOWNPUZZLE_AERODACTYL: u8 = 2;

pub const UNOWNPUZZLE_HO_OH: u8 = 3;

pub const NUM_UNOWN_PUZZLES: u8 = 4;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const UNOWNWORDS_ESCAPE: u8 = 0;

pub const UNOWNWORDS_LIGHT: u8 = 1;

pub const UNOWNWORDS_WATER: u8 = 2;

pub const UNOWNWORDS_HO_OH: u8 = 3;

pub const NUM_UNOWN_WALLS: u8 = 4;

pub const UNOWN_WALL_MENU_HEADER_SIZE: u8 = 5;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const MOVETUTOR_FLAMETHROWER: u8 = 1;

pub const MOVETUTOR_THUNDERBOLT: u8 = 2;

pub const MOVETUTOR_ICE_BEAM: u8 = 3;

