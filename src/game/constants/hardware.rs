use crate::runtime::Cpu;

pub const HARDWARE_INC: u8 = 1;

pub const R_JOYP: u16 = 65280;

pub const B_JOYP_GET_BUTTONS: u8 = 5;

pub const B_JOYP_GET_CTRL_PAD: u8 = 4;

pub const JOYP_GET: u8 = 48;

pub const JOYP_GET_BUTTONS: u8 = 16;

pub const JOYP_GET_CTRL_PAD: u8 = 32;

pub const JOYP_GET_NONE: u8 = 48;

pub const B_JOYP_START: u8 = 3;

pub const B_JOYP_SELECT: u8 = 2;

pub const B_JOYP_B: u8 = 1;

pub const B_JOYP_A: u8 = 0;

pub const B_JOYP_DOWN: u8 = 3;

pub const B_JOYP_UP: u8 = 2;

pub const B_JOYP_LEFT: u8 = 1;

pub const B_JOYP_RIGHT: u8 = 0;

pub const JOYP_INPUTS: u8 = 15;

pub const JOYP_START: u8 = 8;

pub const JOYP_SELECT: u8 = 4;

pub const JOYP_B: u8 = 2;

pub const JOYP_A: u8 = 1;

pub const JOYP_DOWN: u8 = 8;

pub const JOYP_UP: u8 = 4;

pub const JOYP_LEFT: u8 = 2;

pub const JOYP_RIGHT: u8 = 1;

pub const B_JOYP_SGB_ONE: u8 = 5;

pub const B_JOYP_SGB_ZERO: u8 = 4;

pub const JOYP_SGB_START: u8 = 0;

pub const JOYP_SGB_ONE: u8 = 16;

pub const JOYP_SGB_ZERO: u8 = 32;

pub const JOYP_SGB_FINISH: u8 = 48;

pub const B_PAD_DOWN: u8 = 7;

pub const B_PAD_UP: u8 = 6;

pub const B_PAD_LEFT: u8 = 5;

pub const B_PAD_RIGHT: u8 = 4;

pub const B_PAD_START: u8 = 3;

pub const B_PAD_SELECT: u8 = 2;

pub const B_PAD_B: u8 = 1;

pub const B_PAD_A: u8 = 0;

pub const PAD_CTRL_PAD: u8 = 240;

pub const PAD_BUTTONS: u8 = 15;

pub const PAD_DOWN: u8 = 128;

pub const PAD_UP: u8 = 64;

pub const PAD_LEFT: u8 = 32;

pub const PAD_RIGHT: u8 = 16;

pub const PAD_START: u8 = 8;

pub const PAD_SELECT: u8 = 4;

pub const PAD_B: u8 = 2;

pub const PAD_A: u8 = 1;

pub const B_PAD_SWAP_START: u8 = 7;

pub const B_PAD_SWAP_SELECT: u8 = 6;

pub const B_PAD_SWAP_B: u8 = 5;

pub const B_PAD_SWAP_A: u8 = 4;

pub const B_PAD_SWAP_DOWN: u8 = 3;

pub const B_PAD_SWAP_UP: u8 = 2;

pub const B_PAD_SWAP_LEFT: u8 = 1;

pub const B_PAD_SWAP_RIGHT: u8 = 0;

pub const PAD_SWAP_CTRL_PAD: u8 = 15;

pub const PAD_SWAP_BUTTONS: u8 = 240;

pub const PAD_SWAP_START: u8 = 128;

pub const PAD_SWAP_SELECT: u8 = 64;

pub const PAD_SWAP_B: u8 = 32;

pub const PAD_SWAP_A: u8 = 16;

pub const PAD_SWAP_DOWN: u8 = 8;

pub const PAD_SWAP_UP: u8 = 4;

pub const PAD_SWAP_LEFT: u8 = 2;

pub const PAD_SWAP_RIGHT: u8 = 1;

pub const R_SB: u16 = 65281;

pub const R_SC: u16 = 65282;

pub const B_SC_START: u8 = 7;

pub const B_SC_SPEED: u8 = 1;

pub const B_SC_SOURCE: u8 = 0;

pub const SC_START: u8 = 128;

pub const SC_SPEED: u8 = 2;

pub const SC_SLOW: u8 = 0;

pub const SC_FAST: u8 = 2;

pub const SC_SOURCE: u8 = 1;

pub const SC_EXTERNAL: u8 = 0;

pub const SC_INTERNAL: u8 = 1;

pub const R_DIV: u16 = 65284;

pub const R_TIMA: u16 = 65285;

pub const R_TMA: u16 = 65286;

pub const R_TAC: u16 = 65287;

pub const B_TAC_START: u8 = 2;

pub const TAC_STOP: u8 = 0;

pub const TAC_START: u8 = 4;

pub const TAC_CLOCK: u8 = 3;

pub const TAC_4KHZ: u8 = 0;

pub const TAC_262KHZ: u8 = 1;

pub const TAC_65KHZ: u8 = 2;

pub const TAC_16KHZ: u8 = 3;

pub const R_IF: u16 = 65295;

pub const B_IF_JOYPAD: u8 = 4;

pub const B_IF_SERIAL: u8 = 3;

pub const B_IF_TIMER: u8 = 2;

pub const B_IF_STAT: u8 = 1;

pub const B_IF_VBLANK: u8 = 0;

pub const IF_JOYPAD: u8 = 16;

pub const IF_SERIAL: u8 = 8;

pub const IF_TIMER: u8 = 4;

pub const IF_STAT: u8 = 2;

pub const IF_VBLANK: u8 = 1;

pub const R_AUD1SWEEP: u16 = 65296;

pub const AUD1SWEEP_TIME: u8 = 112;

pub const B_AUD1SWEEP_DIR: u8 = 3;

pub const AUD1SWEEP_DIR: u8 = 8;

pub const AUD1SWEEP_UP: u8 = 0;

pub const AUD1SWEEP_DOWN: u8 = 8;

pub const AUD1SWEEP_SHIFT: u8 = 7;

pub const R_AUD1LEN: u16 = 65297;

pub const AUD1LEN_DUTY: u8 = 192;

pub const AUD1LEN_DUTY_12_5: u8 = 0;

pub const AUD1LEN_DUTY_25: u8 = 64;

pub const AUD1LEN_DUTY_50: u8 = 128;

pub const AUD1LEN_DUTY_75: u8 = 192;

pub const AUD1LEN_TIMER: u8 = 63;

pub const R_AUD1ENV: u16 = 65298;

pub const AUD1ENV_INIT_VOLUME: u8 = 240;

pub const B_AUD1ENV_DIR: u8 = 3;

pub const AUD1ENV_DIR: u8 = 8;

pub const AUD1ENV_DOWN: u8 = 0;

pub const AUD1ENV_UP: u8 = 8;

pub const AUD1ENV_PACE: u8 = 7;

pub const R_AUD1LOW: u16 = 65299;

pub const R_AUD1HIGH: u16 = 65300;

pub const B_AUD1HIGH_RESTART: u8 = 7;

pub const B_AUD1HIGH_LEN_ENABLE: u8 = 6;

pub const AUD1HIGH_RESTART: u8 = 128;

pub const AUD1HIGH_LENGTH_OFF: u8 = 0;

pub const AUD1HIGH_LENGTH_ON: u8 = 64;

pub const AUD1HIGH_PERIOD_HIGH: u8 = 7;

pub const R_AUD2LEN: u16 = 65302;

pub const AUD2LEN_DUTY: u8 = 192;

pub const AUD2LEN_DUTY_12_5: u8 = 0;

pub const AUD2LEN_DUTY_25: u8 = 64;

pub const AUD2LEN_DUTY_50: u8 = 128;

pub const AUD2LEN_DUTY_75: u8 = 192;

pub const AUD2LEN_TIMER: u8 = 63;

pub const R_AUD2ENV: u16 = 65303;

pub const AUD2ENV_INIT_VOLUME: u8 = 240;

pub const B_AUD2ENV_DIR: u8 = 3;

pub const AUD2ENV_DIR: u8 = 8;

pub const AUD2ENV_DOWN: u8 = 0;

pub const AUD2ENV_UP: u8 = 8;

pub const AUD2ENV_PACE: u8 = 7;

pub const R_AUD2LOW: u16 = 65304;

pub const R_AUD2HIGH: u16 = 65305;

pub const B_AUD2HIGH_RESTART: u8 = 7;

pub const B_AUD2HIGH_LEN_ENABLE: u8 = 6;

pub const AUD2HIGH_RESTART: u8 = 128;

pub const AUD2HIGH_LENGTH_OFF: u8 = 0;

pub const AUD2HIGH_LENGTH_ON: u8 = 64;

pub const AUD2HIGH_PERIOD_HIGH: u8 = 7;

pub const R_AUD3ENA: u16 = 65306;

pub const B_AUD3ENA_ENABLE: u8 = 7;

pub const AUD3ENA_OFF: u8 = 0;

pub const AUD3ENA_ON: u8 = 128;

pub const R_AUD3LEN: u16 = 65307;

pub const R_AUD3LEVEL: u16 = 65308;

pub const AUD3LEVEL_VOLUME: u8 = 96;

pub const AUD3LEVEL_MUTE: u8 = 0;

pub const AUD3LEVEL_100: u8 = 32;

pub const AUD3LEVEL_50: u8 = 64;

pub const AUD3LEVEL_25: u8 = 96;

pub const R_AUD3LOW: u16 = 65309;

pub const R_AUD3HIGH: u16 = 65310;

pub const B_AUD3HIGH_RESTART: u8 = 7;

pub const B_AUD3HIGH_LEN_ENABLE: u8 = 6;

pub const AUD3HIGH_RESTART: u8 = 128;

pub const AUD3HIGH_LENGTH_OFF: u8 = 0;

pub const AUD3HIGH_LENGTH_ON: u8 = 64;

pub const AUD3HIGH_PERIOD_HIGH: u8 = 7;

pub const R_AUD4LEN: u16 = 65312;

pub const AUD4LEN_TIMER: u8 = 63;

pub const R_AUD4ENV: u16 = 65313;

pub const AUD4ENV_INIT_VOLUME: u8 = 240;

pub const B_AUD4ENV_DIR: u8 = 3;

pub const AUD4ENV_DIR: u8 = 8;

pub const AUD4ENV_DOWN: u8 = 0;

pub const AUD4ENV_UP: u8 = 8;

pub const AUD4ENV_PACE: u8 = 7;

pub const R_AUD4POLY: u16 = 65314;

pub const AUD4POLY_SHIFT: u8 = 240;

pub const B_AUD4POLY_WIDTH: u8 = 3;

pub const AUD4POLY_15STEP: u8 = 0;

pub const AUD4POLY_7STEP: u8 = 8;

pub const AUD4POLY_DIV: u8 = 7;

pub const R_AUD4GO: u16 = 65315;

pub const B_AUD4GO_RESTART: u8 = 7;

pub const B_AUD4GO_LEN_ENABLE: u8 = 6;

pub const AUD4GO_RESTART: u8 = 128;

pub const AUD4GO_LENGTH_OFF: u8 = 0;

pub const AUD4GO_LENGTH_ON: u8 = 64;

pub const R_AUDVOL: u16 = 65316;

pub const B_AUDVOL_VIN_LEFT: u8 = 7;

pub const AUDVOL_VIN_LEFT: u8 = 128;

pub const AUDVOL_LEFT: u8 = 112;

pub const B_AUDVOL_VIN_RIGHT: u8 = 3;

pub const AUDVOL_VIN_RIGHT: u8 = 8;

pub const AUDVOL_RIGHT: u8 = 7;

pub const R_AUDTERM: u16 = 65317;

pub const B_AUDTERM_4_LEFT: u8 = 7;

pub const B_AUDTERM_3_LEFT: u8 = 6;

pub const B_AUDTERM_2_LEFT: u8 = 5;

pub const B_AUDTERM_1_LEFT: u8 = 4;

pub const B_AUDTERM_4_RIGHT: u8 = 3;

pub const B_AUDTERM_3_RIGHT: u8 = 2;

pub const B_AUDTERM_2_RIGHT: u8 = 1;

pub const B_AUDTERM_1_RIGHT: u8 = 0;

pub const AUDTERM_4_LEFT: u8 = 128;

pub const AUDTERM_3_LEFT: u8 = 64;

pub const AUDTERM_2_LEFT: u8 = 32;

pub const AUDTERM_1_LEFT: u8 = 16;

pub const AUDTERM_4_RIGHT: u8 = 8;

pub const AUDTERM_3_RIGHT: u8 = 4;

pub const AUDTERM_2_RIGHT: u8 = 2;

pub const AUDTERM_1_RIGHT: u8 = 1;

pub const R_AUDENA: u16 = 65318;

pub const B_AUDENA_ENABLE: u8 = 7;

pub const B_AUDENA_ENABLE_CH4: u8 = 3;

pub const B_AUDENA_ENABLE_CH3: u8 = 2;

pub const B_AUDENA_ENABLE_CH2: u8 = 1;

pub const B_AUDENA_ENABLE_CH1: u8 = 0;

pub const AUDENA_OFF: u8 = 0;

pub const AUDENA_ON: u8 = 128;

pub const AUDENA_CH4_OFF: u8 = 0;

pub const AUDENA_CH4_ON: u8 = 8;

pub const AUDENA_CH3_OFF: u8 = 0;

pub const AUDENA_CH3_ON: u8 = 4;

pub const AUDENA_CH2_OFF: u8 = 0;

pub const AUDENA_CH2_ON: u8 = 2;

pub const AUDENA_CH1_OFF: u8 = 0;

pub const AUDENA_CH1_ON: u8 = 1;

pub const R_AUD3WAVE_0: u16 = 65328;

pub const R_AUD3WAVE_1: u16 = 65329;

pub const R_AUD3WAVE_2: u16 = 65330;

pub const R_AUD3WAVE_3: u16 = 65331;

pub const R_AUD3WAVE_4: u16 = 65332;

pub const R_AUD3WAVE_5: u16 = 65333;

pub const R_AUD3WAVE_6: u16 = 65334;

pub const R_AUD3WAVE_7: u16 = 65335;

pub const R_AUD3WAVE_8: u16 = 65336;

pub const R_AUD3WAVE_9: u16 = 65337;

pub const R_AUD3WAVE_A: u16 = 65338;

pub const R_AUD3WAVE_B: u16 = 65339;

pub const R_AUD3WAVE_C: u16 = 65340;

pub const R_AUD3WAVE_D: u16 = 65341;

pub const R_AUD3WAVE_E: u16 = 65342;

pub const R_AUD3WAVE_F: u16 = 65343;

pub const R_LCDC: u16 = 65344;

pub const B_LCDC_ENABLE: u8 = 7;

pub const B_LCDC_WIN_MAP: u8 = 6;

pub const B_LCDC_WINDOW: u8 = 5;

pub const B_LCDC_BLOCKS: u8 = 4;

pub const B_LCDC_BG_MAP: u8 = 3;

pub const B_LCDC_OBJ_SIZE: u8 = 2;

pub const B_LCDC_OBJS: u8 = 1;

pub const B_LCDC_BG: u8 = 0;

pub const B_LCDC_PRIO: u8 = 0;

pub const LCDC_ENABLE: u8 = 128;

pub const LCDC_OFF: u8 = 0;

pub const LCDC_ON: u8 = 128;

pub const LCDC_WIN_MAP: u8 = 64;

pub const LCDC_WIN_9800: u8 = 0;

pub const LCDC_WIN_9C00: u8 = 64;

pub const LCDC_WINDOW: u8 = 32;

pub const LCDC_WIN_OFF: u8 = 0;

pub const LCDC_WIN_ON: u8 = 32;

pub const LCDC_BLOCKS: u8 = 16;

pub const LCDC_BLOCK21: u8 = 0;

pub const LCDC_BLOCK01: u8 = 16;

pub const LCDC_BG_MAP: u8 = 8;

pub const LCDC_BG_9800: u8 = 0;

pub const LCDC_BG_9C00: u8 = 8;

pub const LCDC_OBJ_SIZE: u8 = 4;

pub const LCDC_OBJ_8: u8 = 0;

pub const LCDC_OBJ_16: u8 = 4;

pub const LCDC_OBJS: u8 = 2;

pub const LCDC_OBJ_OFF: u8 = 0;

pub const LCDC_OBJ_ON: u8 = 2;

pub const LCDC_BG: u8 = 1;

pub const LCDC_BG_OFF: u8 = 0;

pub const LCDC_BG_ON: u8 = 1;

pub const LCDC_PRIO: u8 = 1;

pub const LCDC_PRIO_OFF: u8 = 0;

pub const LCDC_PRIO_ON: u8 = 1;

pub const R_STAT: u16 = 65345;

pub const B_STAT_LYC: u8 = 6;

pub const B_STAT_MODE_2: u8 = 5;

pub const B_STAT_MODE_1: u8 = 4;

pub const B_STAT_MODE_0: u8 = 3;

pub const B_STAT_LYCF: u8 = 2;

pub const B_STAT_BUSY: u8 = 1;

pub const STAT_LYC: u8 = 64;

pub const STAT_MODE_2: u8 = 32;

pub const STAT_MODE_1: u8 = 16;

pub const STAT_MODE_0: u8 = 8;

pub const STAT_LYCF: u8 = 4;

pub const STAT_BUSY: u8 = 2;

pub const STAT_MODE: u8 = 3;

pub const STAT_HBLANK: u8 = 0;

pub const STAT_VBLANK: u8 = 1;

pub const STAT_OAM: u8 = 2;

pub const STAT_LCD: u8 = 3;

pub const R_SCY: u16 = 65346;

pub const R_SCX: u16 = 65347;

pub const R_LY: u16 = 65348;

pub const LY_VBLANK: u8 = 144;

pub const R_LYC: u16 = 65349;

pub const R_DMA: u16 = 65350;

pub const R_BGP: u16 = 65351;

pub const BGP_SGB_TRANSFER: u8 = 228;

pub const R_OBP0: u16 = 65352;

pub const R_OBP1: u16 = 65353;

pub const R_WY: u16 = 65354;

pub const R_WX: u16 = 65355;

pub const WX_OFS: u8 = 7;

pub const R_SYS: u16 = 65356;

pub const SYS_MODE: u8 = 12;

pub const SYS_CGB: u8 = 0;

pub const SYS_DMG: u8 = 4;

pub const SYS_PGB1: u8 = 8;

pub const SYS_PGB2: u8 = 12;

pub const R_SPD: u16 = 65357;

pub const B_SPD_DOUBLE: u8 = 7;

pub const B_SPD_PREPARE: u8 = 0;

pub const SPD_SINGLE: u8 = 0;

pub const SPD_DOUBLE: u8 = 128;

pub const SPD_PREPARE: u8 = 1;

pub const R_VBK: u16 = 65359;

pub const VBK_BANK: u8 = 1;

pub const R_BANK: u16 = 65360;

pub const B_BANK_ON: u8 = 0;

pub const BANK_ON: u8 = 0;

pub const BANK_OFF: u8 = 1;

pub const R_VDMA_SRC_HIGH: u16 = 65361;

pub const R_VDMA_SRC_LOW: u16 = 65362;

pub const R_VDMA_DEST_HIGH: u16 = 65363;

pub const R_VDMA_DEST_LOW: u16 = 65364;

pub const R_VDMA_LEN: u16 = 65365;

pub const B_VDMA_LEN_MODE: u8 = 7;

pub const VDMA_LEN_MODE: u8 = 128;

pub const VDMA_LEN_MODE_GENERAL: u8 = 0;

pub const VDMA_LEN_MODE_HBLANK: u8 = 128;

pub const B_VDMA_LEN_BUSY: u8 = 7;

pub const VDMA_LEN_BUSY: u8 = 128;

pub const VDMA_LEN_NO: u8 = 0;

pub const VDMA_LEN_YES: u8 = 128;

pub const VDMA_LEN_SIZE: u8 = 127;

pub const R_RP: u16 = 65366;

pub const RP_READ: u8 = 192;

pub const RP_DISABLE: u8 = 0;

pub const RP_ENABLE: u8 = 192;

pub const B_RP_DATA_IN: u8 = 1;

pub const B_RP_LED_ON: u8 = 0;

pub const RP_DATA_IN: u8 = 2;

pub const RP_LED_ON: u8 = 1;

pub const RP_WRITE_LOW: u8 = 0;

pub const RP_WRITE_HIGH: u8 = 1;

pub const R_BGPI: u16 = 65384;

pub const B_BGPI_AUTOINC: u8 = 7;

pub const BGPI_AUTOINC: u8 = 128;

pub const BGPI_INDEX: u8 = 63;

pub const R_BGPD: u16 = 65385;

pub const R_OBPI: u16 = 65386;

pub const B_OBPI_AUTOINC: u8 = 7;

pub const OBPI_AUTOINC: u8 = 128;

pub const OBPI_INDEX: u8 = 63;

pub const R_OBPD: u16 = 65387;

pub const R_OPRI: u16 = 65388;

pub const B_OPRI_PRIORITY: u8 = 0;

pub const OPRI_PRIORITY: u8 = 1;

pub const OPRI_OAM: u8 = 0;

pub const OPRI_COORD: u8 = 1;

pub const R_WBK: u16 = 65392;

pub const WBK_BANK: u8 = 7;

pub const R_PCM12: u16 = 65398;

pub const PCM12_CH2: u8 = 240;

pub const PCM12_CH1: u8 = 15;

pub const R_PCM34: u16 = 65399;

pub const PCM34_CH4: u8 = 240;

pub const PCM34_CH3: u8 = 15;

pub const R_IE: u16 = 65535;

pub const B_IE_JOYPAD: u8 = 4;

pub const B_IE_SERIAL: u8 = 3;

pub const B_IE_TIMER: u8 = 2;

pub const B_IE_STAT: u8 = 1;

pub const B_IE_VBLANK: u8 = 0;

pub const IE_JOYPAD: u8 = 16;

pub const IE_SERIAL: u8 = 8;

pub const IE_TIMER: u8 = 4;

pub const IE_STAT: u8 = 2;

pub const IE_VBLANK: u8 = 1;

pub const R_RAMG: u8 = 0;

pub const RAMG_SRAM_DISABLE: u8 = 0;

pub const RAMG_SRAM_ENABLE: u8 = 10;

pub const RAMG_CART_RAM_RO: u8 = 0;

pub const RAMG_CART_RAM: u8 = 10;

pub const RAMG_RTC_IN: u8 = 11;

pub const RAMG_RTC_IN_CMD: u8 = 112;

pub const RAMG_RTC_IN_ARG: u8 = 15;

pub const RAMG_RTC_OUT: u8 = 12;

pub const RAMG_RTC_OUT_CMD: u8 = 112;

pub const RAMG_RTC_OUT_RESULT: u8 = 15;

pub const RAMG_RTC_SEMAPHORE: u8 = 13;

pub const RAMG_IR: u8 = 14;

pub const R_ROMB: u16 = 8192;

pub const R_RAMB: u16 = 16384;

pub const RAMB_RTC_S: u8 = 8;

pub const RAMB_RTC_M: u8 = 9;

pub const RAMB_RTC_H: u8 = 10;

pub const RAMB_RTC_DL: u8 = 11;

pub const RAMB_RTC_DH: u8 = 12;

pub const B_RAMB_RTC_DH_CARRY: u8 = 7;

pub const B_RAMB_RTC_DH_HALT: u8 = 6;

pub const B_RAMB_RTC_DH_HIGH: u8 = 0;

pub const RAMB_RTC_DH_CARRY: u8 = 128;

pub const RAMB_RTC_DH_HALT: u8 = 64;

pub const RAMB_RTC_DH_HIGH: u8 = 1;

pub const B_RAMB_RUMBLE: u8 = 3;

pub const RAMB_RUMBLE: u8 = 8;

pub const RAMB_RUMBLE_OFF: u8 = 0;

pub const RAMB_RUMBLE_ON: u8 = 8;

pub const R_BMODE: u16 = 24576;

pub const BMODE_SIMPLE: u8 = 0;

pub const BMODE_ADVANCED: u8 = 1;

pub const R_ROM2B: u16 = 8448;

pub const R_RTCLATCH: u16 = 24576;

pub const RTCLATCH_START: u8 = 0;

pub const RTCLATCH_FINISH: u8 = 1;

pub const R_RTCREG: u16 = 40960;

pub const R_ROMB0: u16 = 8192;

pub const R_ROMB1: u16 = 12288;

pub const R_RAMBA: u16 = 1024;

pub const R_RAMBB: u16 = 2048;

pub const R_FLASH: u16 = 3072;

pub const R_FMODE: u16 = 4096;

pub const R_ROMBA: u16 = 8192;

pub const R_FLASHA: u16 = 10240;

pub const R_ROMBB: u16 = 12288;

pub const R_FLASHB: u16 = 14336;

pub const R_RAMREG: u16 = 16384;

pub const RAMREG_ENABLE: u8 = 64;

pub const R_ACCLATCH0: u16 = 40960;

pub const ACCLATCH0_START: u8 = 85;

pub const R_ACCLATCH1: u16 = 40976;

pub const ACCLATCH1_FINISH: u8 = 170;

pub const R_ACCELX0: u16 = 40992;

pub const R_ACCELX1: u16 = 41008;

pub const R_ACCELY0: u16 = 41024;

pub const R_ACCELY1: u16 = 41040;

pub const R_EEPROM: u16 = 41088;

pub const R_IRREG: u16 = 40960;

pub const IR_LED_OFF: u8 = 192;

pub const IR_LED_ON: u8 = 193;

pub const SCREEN_WIDTH_PX: u8 = 160;

pub const SCREEN_HEIGHT_PX: u8 = 144;

pub const SCREEN_WIDTH: u8 = 20;

pub const SCREEN_HEIGHT: u8 = 18;

pub const SCREEN_AREA: u16 = 360;

pub const TILEMAP_WIDTH_PX: u16 = 256;

pub const TILEMAP_HEIGHT_PX: u16 = 256;

pub const TILEMAP_WIDTH: u8 = 32;

pub const TILEMAP_HEIGHT: u8 = 32;

pub const TILEMAP_AREA: u16 = 1024;

pub const TILE_WIDTH: u8 = 8;

pub const TILE_HEIGHT: u8 = 8;

pub const TILE_SIZE: u8 = 16;

pub const COLOR_SIZE: u8 = 2;

pub const PAL_COLORS: u8 = 4;

pub const PAL_SIZE: u8 = 8;

pub const COLOR_CH_WIDTH: u8 = 5;

pub const COLOR_CH_MAX: u8 = 31;

pub const B_COLOR_RED: u8 = 0;

pub const B_COLOR_GREEN: u8 = 5;

pub const B_COLOR_BLUE: u8 = 10;

pub const COLOR_RED: u8 = 31;

pub const COLOR_GREEN_LOW: u8 = 224;

pub const COLOR_GREEN_HIGH: u8 = 3;

pub const COLOR_BLUE: u8 = 124;

pub const SHADE_WHITE: u8 = 0;

pub const SHADE_LIGHT: u8 = 1;

pub const SHADE_DARK: u8 = 2;

pub const SHADE_BLACK: u8 = 3;

pub const TILEMAP0: u16 = 38912;

pub const TILEMAP1: u16 = 39936;

pub const B_BG_PRIO: u8 = 7;

pub const B_BG_YFLIP: u8 = 6;

pub const B_BG_XFLIP: u8 = 5;

pub const B_BG_BANK1: u8 = 3;

pub const BG_PALETTE: u8 = 7;

pub const BG_PRIO: u8 = 128;

pub const BG_YFLIP: u8 = 64;

pub const BG_XFLIP: u8 = 32;

pub const BG_BANK0: u8 = 0;

pub const BG_BANK1: u8 = 8;

pub const OAM_Y_OFS: u8 = 16;

pub const OAM_X_OFS: u8 = 8;

pub const B_OAM_PRIO: u8 = 7;

pub const B_OAM_YFLIP: u8 = 6;

pub const B_OAM_XFLIP: u8 = 5;

pub const B_OAM_PAL1: u8 = 4;

pub const B_OAM_BANK1: u8 = 3;

pub const OAM_PALETTE: u8 = 7;

pub const OAM_PRIO: u8 = 128;

pub const OAM_YFLIP: u8 = 64;

pub const OAM_XFLIP: u8 = 32;

pub const OAM_PAL0: u8 = 0;

pub const OAM_PAL1: u8 = 16;

pub const OAM_BANK0: u8 = 0;

pub const OAM_BANK1: u8 = 8;

pub const OAM_COUNT: u8 = 40;

pub const OAM_SIZE: u8 = 160;

pub const AUD1RAM: u16 = 65296;

pub const AUD2RAM: u16 = 65301;

pub const AUD3RAM: u16 = 65306;

pub const AUD4RAM: u16 = 65311;

pub const AUDRAM_SIZE: u8 = 5;

pub const _AUD3WAVERAM: u16 = 65328;

pub const AUD3WAVE_SIZE: u8 = 16;

pub const INT_HANDLER_VBLANK: u8 = 64;

pub const INT_HANDLER_STAT: u8 = 72;

pub const INT_HANDLER_TIMER: u8 = 80;

pub const INT_HANDLER_SERIAL: u8 = 88;

pub const INT_HANDLER_JOYPAD: u8 = 96;

pub const BOOTUP_A_DMG: u8 = 1;

pub const BOOTUP_A_CGB: u8 = 17;

pub const BOOTUP_A_MGB: u8 = 255;

pub const BOOTUP_A_SGB: u8 = 1;

pub const BOOTUP_A_SGB2: u8 = 255;

pub const B_BOOTUP_B_AGB: u8 = 0;

pub const BOOTUP_B_CGB: u8 = 0;

pub const BOOTUP_B_AGB: u8 = 1;

pub const BOOTUP_C_DMG: u8 = 19;

pub const BOOTUP_C_SGB: u8 = 20;

pub const BOOTUP_C_CGB: u8 = 0;

pub const BOOTUP_D_MONO: u8 = 0;

pub const BOOTUP_D_COLOR: u8 = 255;

pub const BOOTUP_E_DMG0: u8 = 193;

pub const BOOTUP_E_DMG: u8 = 200;

pub const BOOTUP_E_SGB: u8 = 0;

pub const BOOTUP_E_CGB_DMGMODE: u8 = 8;

pub const BOOTUP_E_CGB: u8 = 86;

pub const R_P1: u16 = 65280;

pub const R_NR10: u16 = 65296;

pub const R_NR11: u16 = 65297;

pub const R_NR12: u16 = 65298;

pub const R_NR13: u16 = 65299;

pub const R_NR14: u16 = 65300;

pub const R_NR21: u16 = 65302;

pub const R_NR22: u16 = 65303;

pub const R_NR23: u16 = 65304;

pub const R_NR24: u16 = 65305;

pub const R_NR30: u16 = 65306;

pub const R_NR31: u16 = 65307;

pub const R_NR32: u16 = 65308;

pub const R_NR33: u16 = 65309;

pub const R_NR34: u16 = 65310;

pub const R_NR41: u16 = 65312;

pub const R_NR42: u16 = 65313;

pub const R_NR43: u16 = 65314;

pub const R_NR44: u16 = 65315;

pub const R_NR50: u16 = 65316;

pub const R_NR51: u16 = 65317;

pub const R_NR52: u16 = 65318;

pub const R_KEY0: u16 = 65356;

pub const R_KEY1: u16 = 65357;

pub const R_HDMA1: u16 = 65361;

pub const R_HDMA2: u16 = 65362;

pub const R_HDMA3: u16 = 65363;

pub const R_HDMA4: u16 = 65364;

pub const R_HDMA5: u16 = 65365;

pub const R_BCPS: u16 = 65384;

pub const R_BCPD: u16 = 65385;

pub const R_OCPS: u16 = 65386;

pub const R_OCPD: u16 = 65387;

pub const R_SVBK: u16 = 65392;

