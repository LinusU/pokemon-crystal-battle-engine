use crate::runtime::Cpu;

pub const CONST_VALUE: u8 = 0;

pub const CONST_INC: u8 = 1;

pub const LINK_NULL: u8 = 0;

pub const LINK_TIMECAPSULE: u8 = 1;

pub const LINK_TRADECENTER: u8 = 2;

pub const LINK_COLOSSEUM: u8 = 3;

pub const LINK_MOBILE: u8 = 4;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const CABLECLUBROOM_NULL: u8 = 0;

pub const CABLECLUBROOM_TRADECENTER: u8 = 1;

pub const CABLECLUBROOM_COLOSSEUM: u8 = 2;

pub const SERIAL_TIMECAPSULE: u8 = 96;

pub const SERIAL_TRADECENTER: u8 = 112;

pub const SERIAL_BATTLE: u8 = 128;

pub const ESTABLISH_CONNECTION_WITH_INTERNAL_CLOCK: u8 = 1;

pub const ESTABLISH_CONNECTION_WITH_EXTERNAL_CLOCK: u8 = 2;

pub const USING_EXTERNAL_CLOCK: u8 = 1;

pub const USING_INTERNAL_CLOCK: u8 = 2;

pub const CONNECTION_NOT_ESTABLISHED: u8 = 255;

pub const SERIAL_PATCH_LIST_LENGTH: u8 = 200;

pub const SERIAL_PATCH_DATA_SIZE: u8 = 252;

pub const SERIAL_PREAMBLE_BYTE: u8 = 253;

pub const SERIAL_NO_DATA_BYTE: u8 = 254;

pub const SERIAL_PATCH_LIST_PART_TERMINATOR: u8 = 255;

pub const SERIAL_PATCH_REPLACEMENT_BYTE: u8 = 255;

pub const SERIAL_PREAMBLE_LENGTH: u8 = 6;

pub const SERIAL_RN_PREAMBLE_LENGTH: u8 = 7;

pub const SERIAL_PATCH_PREAMBLE_LENGTH: u8 = 3;

pub const SERIAL_RNS_LENGTH: u8 = 10;

pub const SERIAL_MAIL_PREAMBLE_BYTE: u8 = 32;

pub const SERIAL_MAIL_PREAMBLE_LENGTH: u8 = 5;

pub const SERIAL_MAIL_REPLACEMENT_BYTE: u8 = 33;

pub const SERIAL_LINK_BYTE_TIMEOUT: u16 = 20480;

pub const MAX_MYSTERY_GIFT_PARTNERS: u8 = 5;

