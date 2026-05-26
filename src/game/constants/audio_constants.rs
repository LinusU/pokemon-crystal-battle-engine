use crate::runtime::Cpu;

pub const CONST_VALUE: u8 = 1;

pub const CONST_INC: u8 = 1;

pub const C_: u8 = 1;

pub const C: u8 = 2;

pub const D_: u8 = 3;

pub const D: u8 = 4;

pub const E_: u8 = 5;

pub const F_: u8 = 6;

pub const F: u8 = 7;

pub const G_: u8 = 8;

pub const G: u8 = 9;

pub const A_: u8 = 10;

pub const A: u8 = 11;

pub const B_: u8 = 12;

pub const NUM_NOTES: u8 = 12;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const CHAN1: u8 = 0;

pub const CHAN2: u8 = 1;

pub const CHAN3: u8 = 2;

pub const CHAN4: u8 = 3;

pub const NUM_MUSIC_CHANS: u8 = 4;

pub const CHAN5: u8 = 4;

pub const CHAN6: u8 = 5;

pub const CHAN7: u8 = 6;

pub const CHAN8: u8 = 7;

pub const NUM_NOISE_CHANS: u8 = 4;

pub const NUM_CHANNELS: u8 = 8;

pub const CHANNEL_STRUCT_LENGTH: u8 = 50;

pub const NOISE_CHAN_F: u8 = 2;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const SOUND_CHANNEL_ON: u8 = 0;

pub const SOUND_SUBROUTINE: u8 = 1;

pub const SOUND_LOOPING: u8 = 2;

pub const SOUND_SFX: u8 = 3;

pub const SOUND_NOISE: u8 = 4;

pub const SOUND_CRY: u8 = 5;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const SOUND_VIBRATO: u8 = 0;

pub const SOUND_PITCH_SLIDE: u8 = 1;

pub const SOUND_DUTY_LOOP: u8 = 2;

pub const SOUND_UNKN_0B: u8 = 3;

pub const SOUND_PITCH_OFFSET: u8 = 4;

pub const SOUND_UNKN_0D: u8 = 5;

pub const SOUND_UNKN_0E: u8 = 6;

pub const SOUND_UNKN_0F: u8 = 7;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const SOUND_VIBRATO_DIR: u8 = 0;

pub const SOUND_PITCH_SLIDE_DIR: u8 = 1;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const NOTE_DUTY_OVERRIDE: u8 = 0;

pub const NOTE_FREQ_OVERRIDE: u8 = 1;

pub const NOTE_UNUSED: u8 = 2;

pub const NOTE_PITCH_SWEEP: u8 = 3;

pub const NOTE_NOISE_SAMPLING: u8 = 4;

pub const NOTE_REST: u8 = 5;

pub const NOTE_VIBRATO_OVERRIDE: u8 = 6;

pub const MAX_VOLUME: u8 = 119;

pub const SOUND_INPUT_CH1_F: u8 = 0;

pub const SOUND_INPUT_CH2_F: u8 = 1;

pub const SOUND_INPUT_CH3_F: u8 = 2;

pub const SOUND_INPUT_CH4_F: u8 = 3;

pub const SOUND_INPUT_GLOBAL_F: u8 = 7;

pub const DANGER_PITCH_F: u8 = 4;

pub const DANGER_ON_F: u8 = 7;

pub const MUSIC_FADE_IN_F: u8 = 7;

