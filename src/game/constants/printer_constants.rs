use crate::runtime::Cpu;

pub const CONST_VALUE: u8 = 1;

pub const CONST_INC: u8 = 1;

pub const PRINTER_STATUS_CHECKING: u8 = 1;

pub const PRINTER_STATUS_TRANSMITTING: u8 = 2;

pub const PRINTER_STATUS_PRINTING: u8 = 3;

pub const PRINTER_ERROR_1: u8 = 4;

pub const PRINTER_ERROR_2: u8 = 5;

pub const PRINTER_ERROR_3: u8 = 6;

pub const PRINTER_ERROR_4: u8 = 7;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const PRINTER_STATUS_ERROR_3: u8 = 32;

pub const PRINTER_STATUS_ERROR_3_F: u8 = 5;

pub const PRINTER_STATUS_ERROR_4: u8 = 64;

pub const PRINTER_STATUS_ERROR_4_F: u8 = 6;

pub const PRINTER_STATUS_ERROR_1: u8 = 128;

pub const PRINTER_STATUS_ERROR_1_F: u8 = 7;

// duplicate assembler constant CONST_VALUE

// duplicate assembler constant CONST_INC

pub const PRINTER_CONNECTION_OPEN: u8 = 0;

pub const PRINTER_CONNECTION_SUCCESS: u8 = 1;

