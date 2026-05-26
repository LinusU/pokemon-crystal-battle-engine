use crate::runtime::Cpu;

pub const CONST_VALUE: u8 = 0;

pub const CONST_INC: u8 = 1;

pub const MENU_RESTORE_TILES: u8 = 1;

pub const MENU_RESTORE_TILES_F: u8 = 0;

pub const MENU_NO_CLICK_SFX: u8 = 8;

pub const MENU_NO_CLICK_SFX_F: u8 = 3;

pub const MENU_SPRITE_ANIMS: u8 = 16;

pub const MENU_SPRITE_ANIMS_F: u8 = 4;

pub const MENU_UNUSED: u8 = 32;

pub const MENU_UNUSED_F: u8 = 5;

pub const MENU_BACKUP_TILES: u8 = 64;

pub const MENU_BACKUP_TILES_F: u8 = 6;

pub const MENU_BACKUP_TILES_2: u8 = 128;

pub const MENU_BACKUP_TILES_2_F: u8 = 7;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const STATICMENU_DISABLE_B: u8 = 1;

pub const STATICMENU_DISABLE_B_F: u8 = 0;

pub const STATICMENU_ENABLE_SELECT: u8 = 2;

pub const STATICMENU_ENABLE_SELECT_F: u8 = 1;

pub const STATICMENU_ENABLE_LEFT_RIGHT: u8 = 4;

pub const STATICMENU_ENABLE_LEFT_RIGHT_F: u8 = 2;

pub const STATICMENU_ENABLE_START: u8 = 8;

pub const STATICMENU_ENABLE_START_F: u8 = 3;

pub const STATICMENU_PLACE_TITLE: u8 = 16;

pub const STATICMENU_PLACE_TITLE_F: u8 = 4;

pub const STATICMENU_WRAP: u8 = 32;

pub const STATICMENU_WRAP_F: u8 = 5;

pub const STATICMENU_NO_TOP_SPACING: u8 = 64;

pub const STATICMENU_NO_TOP_SPACING_F: u8 = 6;

pub const STATICMENU_CURSOR: u8 = 128;

pub const STATICMENU_CURSOR_F: u8 = 7;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const SCROLLINGMENU_CALL_FUNCTION1_CANCEL: u8 = 1;

pub const SCROLLINGMENU_CALL_FUNCTION1_CANCEL_F: u8 = 0;

pub const SCROLLINGMENU_CALL_FUNCTION3_NO_SWITCH: u8 = 2;

pub const SCROLLINGMENU_CALL_FUNCTION3_NO_SWITCH_F: u8 = 1;

pub const SCROLLINGMENU_ENABLE_RIGHT: u8 = 4;

pub const SCROLLINGMENU_ENABLE_RIGHT_F: u8 = 2;

pub const SCROLLINGMENU_ENABLE_LEFT: u8 = 8;

pub const SCROLLINGMENU_ENABLE_LEFT_F: u8 = 3;

pub const SCROLLINGMENU_DISPLAY_ARROWS: u8 = 16;

pub const SCROLLINGMENU_DISPLAY_ARROWS_F: u8 = 4;

pub const SCROLLINGMENU_ENABLE_FUNCTION3: u8 = 32;

pub const SCROLLINGMENU_ENABLE_FUNCTION3_F: u8 = 5;

pub const SCROLLINGMENU_ENABLE_START: u8 = 64;

pub const SCROLLINGMENU_ENABLE_START_F: u8 = 6;

pub const SCROLLINGMENU_ENABLE_SELECT: u8 = 128;

pub const SCROLLINGMENU_ENABLE_SELECT_F: u8 = 7;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const SCROLLINGMENU_ITEMS_NORMAL: u8 = 1;

pub const SCROLLINGMENU_ITEMS_QUANTITY: u8 = 2;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const _2DMENU_EXIT_RIGHT: u8 = 1;

pub const _2DMENU_EXIT_RIGHT_F: u8 = 0;

pub const _2DMENU_EXIT_LEFT: u8 = 2;

pub const _2DMENU_EXIT_LEFT_F: u8 = 1;

pub const _2DMENU_EXIT_UP: u8 = 4;

pub const _2DMENU_EXIT_UP_F: u8 = 2;

pub const _2DMENU_EXIT_DOWN: u8 = 8;

pub const _2DMENU_EXIT_DOWN_F: u8 = 3;

pub const _2DMENU_WRAP_LEFT_RIGHT: u8 = 16;

pub const _2DMENU_WRAP_LEFT_RIGHT_F: u8 = 4;

pub const _2DMENU_WRAP_UP_DOWN: u8 = 32;

pub const _2DMENU_WRAP_UP_DOWN_F: u8 = 5;

pub const _2DMENU_ENABLE_SPRITE_ANIMS: u8 = 64;

pub const _2DMENU_ENABLE_SPRITE_ANIMS_F: u8 = 6;

pub const _2DMENU_DISABLE_JOYPAD_FILTER: u8 = 128;

pub const _2DMENU_DISABLE_JOYPAD_FILTER_F: u8 = 7;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const _2DMENU_EXITING: u8 = 128;

pub const _2DMENU_EXITING_F: u8 = 7;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const MONMENUITEM_CUT: u8 = 1;

pub const MONMENUITEM_FLY: u8 = 2;

pub const MONMENUITEM_SURF: u8 = 3;

pub const MONMENUITEM_STRENGTH: u8 = 4;

pub const MONMENUITEM_WATERFALL: u8 = 5;

pub const MONMENUITEM_FLASH: u8 = 6;

pub const MONMENUITEM_WHIRLPOOL: u8 = 7;

pub const MONMENUITEM_DIG: u8 = 8;

pub const MONMENUITEM_TELEPORT: u8 = 9;

pub const MONMENUITEM_SOFTBOILED: u8 = 10;

pub const MONMENUITEM_HEADBUTT: u8 = 11;

pub const MONMENUITEM_ROCKSMASH: u8 = 12;

pub const MONMENUITEM_MILKDRINK: u8 = 13;

pub const MONMENUITEM_SWEETSCENT: u8 = 14;

pub const MONMENUITEM_STATS: u8 = 15;

pub const MONMENUITEM_SWITCH: u8 = 16;

pub const MONMENUITEM_ITEM: u8 = 17;

pub const MONMENUITEM_CANCEL: u8 = 18;

pub const MONMENUITEM_MOVE: u8 = 19;

pub const MONMENUITEM_MAIL: u8 = 20;

pub const MONMENUITEM_ERROR: u8 = 21;

pub const NUM_MONMENUITEMS: u8 = 21;

pub const MONMENU_FIELD_MOVE: u8 = 0;

pub const MONMENU_MENUOPTION: u8 = 1;

pub const NUM_MONMENU_ITEMS: u8 = 8;

pub const HMENURETURN_SCRIPT: u8 = 128;

pub const HMENURETURN_ASM: u8 = 255;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const PARTYMENUACTION_CHOOSE_POKEMON: u8 = 0;

pub const PARTYMENUACTION_HEALING_ITEM: u8 = 1;

pub const PARTYMENUACTION_SWITCH: u8 = 2;

pub const PARTYMENUACTION_TEACH_TMHM: u8 = 3;

pub const PARTYMENUACTION_MOVE: u8 = 4;

pub const PARTYMENUACTION_EVO_STONE: u8 = 5;

pub const PARTYMENUACTION_GIVE_MON: u8 = 6;

pub const PARTYMENUACTION_GIVE_MON_FEMALE: u8 = 7;

pub const PARTYMENUACTION_GIVE_ITEM: u8 = 8;

pub const PARTYMENUACTION_MOBILE: u8 = 9;

pub const NUM_PARTYMENUACTIONS: u8 = 10;

// duplicate assembler constant CONST_VALUE

pub const PARTYMENUTEXT_HEAL_PSN: u8 = 240;

pub const PARTYMENUTEXT_HEAL_BRN: u8 = 241;

pub const PARTYMENUTEXT_HEAL_FRZ: u8 = 242;

pub const PARTYMENUTEXT_HEAL_SLP: u8 = 243;

pub const PARTYMENUTEXT_HEAL_PAR: u8 = 244;

pub const PARTYMENUTEXT_HEAL_HP: u8 = 245;

pub const PARTYMENUTEXT_HEAL_ALL: u8 = 246;

pub const PARTYMENUTEXT_REVIVE: u8 = 247;

pub const PARTYMENUTEXT_LEVEL_UP: u8 = 248;

pub const PARTYMENUTEXT_HEAL_CONFUSION: u8 = 249;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const NAME_MON: u8 = 0;

pub const NAME_PLAYER: u8 = 1;

pub const NAME_RIVAL: u8 = 2;

pub const NAME_MOM: u8 = 3;

pub const NAME_BOX: u8 = 4;

pub const NAME_FRIEND: u8 = 5;

pub const NAME_6: u8 = 6;

pub const NAME_7: u8 = 7;

pub const NUM_NAMING_SCREEN_TYPES: u8 = 8;

