pub(super) enum Mbc {
    Mbc0(Mbc0),
    Mbc1(Mbc1),
    Mbc2(Mbc2),
    Mbc3(Mbc3),
    Mbc5(Mbc5),
}

pub(super) struct Mbc0 {
    ram: Vec<u8>,
    ram_updated: bool,
    has_battery: bool,
}

pub(super) struct Mbc1 {
    ram: Vec<u8>,
    ram_on: bool,
    ram_updated: bool,
    banking_mode: u8,
    rom_bank: usize,
    ram_bank: usize,
    has_battery: bool,
    rom_banks: usize,
    ram_banks: usize,
}

pub(super) struct Mbc2 {
    ram: [u8; 512],
    ram_on: bool,
    ram_updated: bool,
    rom_bank: usize,
    has_battery: bool,
    rom_banks: usize,
}

pub(super) struct Mbc3 {
    ram: Vec<u8>,
    rom_bank: usize,
    ram_bank: usize,
    ram_banks: usize,
    select_rtc: bool,
    ram_on: bool,
    ram_updated: bool,
    has_battery: bool,
    rtc_ram: [u8; 5],
    rtc_latch: [u8; 5],
    rtc_zero: Option<u64>,
}

pub(super) struct Mbc5 {
    ram: Vec<u8>,
    rom_bank: usize,
    ram_bank: usize,
    ram_on: bool,
    ram_updated: bool,
    has_battery: bool,
    rom_banks: usize,
    ram_banks: usize,
}

impl Mbc {
    pub(super) fn new(rom: &[u8]) -> Self {
        match rom_header(rom, 0x147) {
            0x01..=0x03 => Self::Mbc1(Mbc1::new(rom)),
            0x05..=0x06 => Self::Mbc2(Mbc2::new(rom)),
            0x0f..=0x13 => Self::Mbc3(Mbc3::new(rom)),
            0x19..=0x1e => Self::Mbc5(Mbc5::new(rom)),
            _ => Self::Mbc0(Mbc0::new(rom)),
        }
    }

    pub(super) fn read_rom(&self, rom: &[u8], addr: u16) -> u8 {
        match self {
            Self::Mbc0(mbc) => mbc.read_rom(rom, addr),
            Self::Mbc1(mbc) => mbc.read_rom(rom, addr),
            Self::Mbc2(mbc) => mbc.read_rom(rom, addr),
            Self::Mbc3(mbc) => mbc.read_rom(rom, addr),
            Self::Mbc5(mbc) => mbc.read_rom(rom, addr),
        }
    }

    pub(super) fn read_ram(&self, addr: u16) -> u8 {
        match self {
            Self::Mbc0(mbc) => mbc.read_ram(addr),
            Self::Mbc1(mbc) => mbc.read_ram(addr),
            Self::Mbc2(mbc) => mbc.read_ram(addr),
            Self::Mbc3(mbc) => mbc.read_ram(addr),
            Self::Mbc5(mbc) => mbc.read_ram(addr),
        }
    }

    pub(super) fn write_rom(&mut self, addr: u16, value: u8) {
        match self {
            Self::Mbc0(mbc) => mbc.write_rom(addr, value),
            Self::Mbc1(mbc) => mbc.write_rom(addr, value),
            Self::Mbc2(mbc) => mbc.write_rom(addr, value),
            Self::Mbc3(mbc) => mbc.write_rom(addr, value),
            Self::Mbc5(mbc) => mbc.write_rom(addr, value),
        }
    }

    pub(super) fn write_ram(&mut self, addr: u16, value: u8) {
        match self {
            Self::Mbc0(mbc) => mbc.write_ram(addr, value),
            Self::Mbc1(mbc) => mbc.write_ram(addr, value),
            Self::Mbc2(mbc) => mbc.write_ram(addr, value),
            Self::Mbc3(mbc) => mbc.write_ram(addr, value),
            Self::Mbc5(mbc) => mbc.write_ram(addr, value),
        }
    }

    pub(super) fn current_rom_bank(&self) -> usize {
        match self {
            Self::Mbc0(_) => 1,
            Self::Mbc1(mbc) => mbc.rom_bank,
            Self::Mbc2(mbc) => mbc.rom_bank,
            Self::Mbc3(mbc) => mbc.rom_bank,
            Self::Mbc5(mbc) => mbc.rom_bank,
        }
    }

    pub(super) fn is_battery_backed(&self) -> bool {
        match self {
            Self::Mbc0(mbc) => mbc.has_battery,
            Self::Mbc1(mbc) => mbc.has_battery,
            Self::Mbc2(mbc) => mbc.has_battery,
            Self::Mbc3(mbc) => mbc.has_battery,
            Self::Mbc5(mbc) => mbc.has_battery,
        }
    }

    pub(super) fn load_ram(&mut self, data: &[u8]) -> Result<(), &'static str> {
        match self {
            Self::Mbc0(mbc) => mbc.load_ram(data),
            Self::Mbc1(mbc) => mbc.load_ram(data),
            Self::Mbc2(mbc) => mbc.load_ram(data),
            Self::Mbc3(mbc) => mbc.load_ram(data),
            Self::Mbc5(mbc) => mbc.load_ram(data),
        }
    }

    pub(super) fn dump_ram(&self) -> Vec<u8> {
        match self {
            Self::Mbc0(mbc) => mbc.dump_ram(),
            Self::Mbc1(mbc) => mbc.dump_ram(),
            Self::Mbc2(mbc) => mbc.dump_ram(),
            Self::Mbc3(mbc) => mbc.dump_ram(),
            Self::Mbc5(mbc) => mbc.dump_ram(),
        }
    }

    pub(super) fn check_and_reset_ram_updated(&mut self) -> bool {
        match self {
            Self::Mbc0(mbc) => mbc.check_and_reset_ram_updated(),
            Self::Mbc1(mbc) => mbc.check_and_reset_ram_updated(),
            Self::Mbc2(mbc) => mbc.check_and_reset_ram_updated(),
            Self::Mbc3(mbc) => mbc.check_and_reset_ram_updated(),
            Self::Mbc5(mbc) => mbc.check_and_reset_ram_updated(),
        }
    }
}

impl Mbc0 {
    pub(super) fn new(rom: &[u8]) -> Self {
        let cartridge_type = rom_header(rom, 0x147);
        let ram_banks = match cartridge_type {
            0x08 | 0x09 => ram_bank_count(rom_header(rom, 0x149)),
            _ => 0,
        };

        Self {
            ram: vec![0; ram_banks * 0x2000],
            ram_updated: false,
            has_battery: cartridge_type == 0x09,
        }
    }

    pub(super) fn read_rom(&self, rom: &[u8], addr: u16) -> u8 {
        rom.get(usize::from(addr)).copied().unwrap_or(0xff)
    }

    pub(super) fn read_ram(&self, addr: u16) -> u8 {
        self.ram
            .get(usize::from(addr & 0x1fff))
            .copied()
            .unwrap_or(0xff)
    }

    pub(super) fn write_rom(&mut self, _addr: u16, _value: u8) {}

    pub(super) fn write_ram(&mut self, addr: u16, value: u8) {
        if let Some(byte) = self.ram.get_mut(usize::from(addr & 0x1fff)) {
            *byte = value;
            self.ram_updated = true;
        }
    }

    pub(super) fn load_ram(&mut self, data: &[u8]) -> Result<(), &'static str> {
        load_exact_ram(&mut self.ram, data)
    }

    pub(super) fn dump_ram(&self) -> Vec<u8> {
        self.ram.clone()
    }

    pub(super) fn check_and_reset_ram_updated(&mut self) -> bool {
        let updated = self.ram_updated;
        self.ram_updated = false;
        updated
    }
}

impl Mbc1 {
    pub(super) fn new(rom: &[u8]) -> Self {
        let cartridge_type = rom_header(rom, 0x147);
        let ram_banks = match cartridge_type {
            0x02 | 0x03 => ram_bank_count(rom_header(rom, 0x149)),
            _ => 0,
        };

        Self {
            ram: vec![0; ram_banks * 0x2000],
            ram_on: false,
            ram_updated: false,
            banking_mode: 0,
            rom_bank: 1,
            ram_bank: 0,
            has_battery: cartridge_type == 0x03,
            rom_banks: rom_bank_count(rom),
            ram_banks,
        }
    }

    pub(super) fn read_rom(&self, rom: &[u8], addr: u16) -> u8 {
        let bank = if addr < 0x4000 {
            if self.banking_mode == 0 {
                0
            } else {
                self.rom_bank & 0xe0
            }
        } else {
            self.rom_bank
        };

        read_rom_bank(rom, bank, addr)
    }

    pub(super) fn read_ram(&self, addr: u16) -> u8 {
        if !self.ram_on || self.ram.is_empty() {
            return 0xff;
        }

        let bank = if self.banking_mode == 1 {
            self.ram_bank
        } else {
            0
        };
        read_ram_bank(&self.ram, bank, addr)
    }

    pub(super) fn write_rom(&mut self, addr: u16, value: u8) {
        match addr {
            0x0000..=0x1fff => self.ram_on = value & 0x0f == 0x0a,
            0x2000..=0x3fff => {
                let lower_bits = match usize::from(value & 0x1f) {
                    0 => 1,
                    n => n,
                };
                self.rom_bank = normalize_bank((self.rom_bank & 0x60) | lower_bits, self.rom_banks);
            }
            0x4000..=0x5fff => {
                if self.rom_banks > 0x20 {
                    let upper_bits = usize::from(value & 0x03) % (self.rom_banks >> 5).max(1);
                    self.rom_bank = normalize_bank((self.rom_bank & 0x1f) | (upper_bits << 5), self.rom_banks);
                }
                if self.ram_banks > 1 {
                    self.ram_bank = usize::from(value & 0x03) % self.ram_banks;
                }
            }
            0x6000..=0x7fff => self.banking_mode = value & 0x01,
            _ => {}
        }
    }

    pub(super) fn write_ram(&mut self, addr: u16, value: u8) {
        if !self.ram_on || self.ram.is_empty() {
            return;
        }

        let bank = if self.banking_mode == 1 {
            self.ram_bank
        } else {
            0
        };
        if write_ram_bank(&mut self.ram, bank, addr, value) {
            self.ram_updated = true;
        }
    }

    pub(super) fn load_ram(&mut self, data: &[u8]) -> Result<(), &'static str> {
        load_exact_ram(&mut self.ram, data)
    }

    pub(super) fn dump_ram(&self) -> Vec<u8> {
        self.ram.clone()
    }

    pub(super) fn check_and_reset_ram_updated(&mut self) -> bool {
        let updated = self.ram_updated;
        self.ram_updated = false;
        updated
    }
}

impl Mbc2 {
    pub(super) fn new(rom: &[u8]) -> Self {
        Self {
            ram: [0; 512],
            ram_on: false,
            ram_updated: false,
            rom_bank: 1,
            has_battery: rom_header(rom, 0x147) == 0x06,
            rom_banks: rom_bank_count(rom),
        }
    }

    pub(super) fn read_rom(&self, rom: &[u8], addr: u16) -> u8 {
        let bank = if addr < 0x4000 { 0 } else { self.rom_bank };
        read_rom_bank(rom, bank, addr)
    }

    pub(super) fn read_ram(&self, addr: u16) -> u8 {
        if !self.ram_on {
            return 0xff;
        }
        self.ram[usize::from(addr & 0x01ff)] | 0xf0
    }

    pub(super) fn write_rom(&mut self, addr: u16, value: u8) {
        if !matches!(addr, 0x0000..=0x3fff) {
            return;
        }

        if addr & 0x0100 == 0 {
            self.ram_on = value & 0x0f == 0x0a;
        } else {
            let bank = match usize::from(value & 0x0f) {
                0 => 1,
                n => n,
            };
            self.rom_bank = normalize_bank(bank, self.rom_banks);
        }
    }

    pub(super) fn write_ram(&mut self, addr: u16, value: u8) {
        if !self.ram_on {
            return;
        }

        self.ram[usize::from(addr & 0x01ff)] = value & 0x0f;
        self.ram_updated = true;
    }

    pub(super) fn load_ram(&mut self, data: &[u8]) -> Result<(), &'static str> {
        if data.len() != self.ram.len() {
            return Err("loaded RAM has incorrect length");
        }
        self.ram.copy_from_slice(data);
        Ok(())
    }

    pub(super) fn dump_ram(&self) -> Vec<u8> {
        self.ram.to_vec()
    }

    pub(super) fn check_and_reset_ram_updated(&mut self) -> bool {
        let updated = self.ram_updated;
        self.ram_updated = false;
        updated
    }
}

impl Mbc3 {
    pub(super) fn new(rom: &[u8]) -> Self {
        let cartridge_type = rom_header(rom, 0x147);
        let ram_banks = match cartridge_type {
            0x10 | 0x12 | 0x13 => ram_bank_count(rom_header(rom, 0x149)),
            _ => 0,
        };
        let has_rtc = matches!(cartridge_type, 0x0f | 0x10);

        Self {
            ram: vec![0; ram_banks * 0x2000],
            rom_bank: 1,
            ram_bank: 0,
            ram_banks,
            select_rtc: false,
            ram_on: false,
            ram_updated: false,
            has_battery: matches!(cartridge_type, 0x0f | 0x10 | 0x13),
            rtc_ram: [0; 5],
            rtc_latch: [0; 5],
            rtc_zero: has_rtc.then_some(0),
        }
    }

    pub(super) fn read_rom(&self, rom: &[u8], addr: u16) -> u8 {
        let bank = if addr < 0x4000 { 0 } else { self.rom_bank };
        read_rom_bank(rom, bank, addr)
    }

    pub(super) fn read_ram(&self, addr: u16) -> u8 {
        if !self.ram_on {
            return 0xff;
        }

        if !self.select_rtc && self.ram_bank < self.ram_banks {
            read_ram_bank(&self.ram, self.ram_bank, addr)
        } else if self.select_rtc && self.ram_bank < 5 {
            self.rtc_latch[self.ram_bank]
        } else {
            0xff
        }
    }

    pub(super) fn write_rom(&mut self, addr: u16, value: u8) {
        match addr {
            0x0000..=0x1fff => self.ram_on = value & 0x0f == 0x0a,
            0x2000..=0x3fff => {
                self.rom_bank = match usize::from(value & 0x7f) {
                    0 => 1,
                    n => n,
                };
            }
            0x4000..=0x5fff => {
                self.select_rtc = value & 0x08 == 0x08;
                self.ram_bank = usize::from(value & 0x07);
            }
            0x6000..=0x7fff => self.latch_rtc_registers(),
            _ => {}
        }
    }

    pub(super) fn write_ram(&mut self, addr: u16, value: u8) {
        if !self.ram_on {
            return;
        }

        if !self.select_rtc && self.ram_bank < self.ram_banks {
            if write_ram_bank(&mut self.ram, self.ram_bank, addr, value) {
                self.ram_updated = true;
            }
        } else if self.select_rtc && self.ram_bank < 5 {
            self.calc_rtc_registers();
            let mask = match self.ram_bank {
                0 | 1 => 0x3f,
                2 => 0x1f,
                4 => 0xc1,
                _ => 0xff,
            };
            self.rtc_ram[self.ram_bank] = value & mask;
            self.calc_rtc_zero();
            self.ram_updated = true;
        }
    }

    fn latch_rtc_registers(&mut self) {
        self.calc_rtc_registers();
        self.rtc_latch.copy_from_slice(&self.rtc_ram);
    }

    fn calc_rtc_registers(&mut self) {
        if self.rtc_ram[4] & 0x40 == 0x40 {
            return;
        }

        let Some(rtc_zero) = self.rtc_zero else {
            return;
        };
        if self.compute_diff_time() == self.rtc_zero {
            return;
        }

        let zero = std::time::UNIX_EPOCH + std::time::Duration::from_secs(rtc_zero);
        let diff = std::time::SystemTime::now()
            .duration_since(zero)
            .map(|duration| duration.as_secs())
            .unwrap_or(0);
        self.rtc_ram[0] = (diff % 60) as u8;
        self.rtc_ram[1] = ((diff / 60) % 60) as u8;
        self.rtc_ram[2] = ((diff / 3600) % 24) as u8;

        let days = diff / (3600 * 24);
        self.rtc_ram[3] = days as u8;
        self.rtc_ram[4] = (self.rtc_ram[4] & 0xfe) | (((days >> 8) & 0x01) as u8);
        if days >= 512 {
            self.rtc_ram[4] |= 0x80;
            self.calc_rtc_zero();
        }
    }

    fn compute_diff_time(&self) -> Option<u64> {
        self.rtc_zero?;

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|duration| duration.as_secs())
            .unwrap_or(0);
        let days = ((u64::from(self.rtc_ram[4]) & 0x01) << 8) | u64::from(self.rtc_ram[3]);
        Some(
            now.saturating_sub(u64::from(self.rtc_ram[0]))
                .saturating_sub(u64::from(self.rtc_ram[1]) * 60)
                .saturating_sub(u64::from(self.rtc_ram[2]) * 3600)
                .saturating_sub(days * 3600 * 24),
        )
    }

    fn calc_rtc_zero(&mut self) {
        self.rtc_zero = self.compute_diff_time();
    }

    pub(super) fn load_ram(&mut self, data: &[u8]) -> Result<(), &'static str> {
        if data.len() == self.ram.len() {
            self.ram.copy_from_slice(data);
            return Ok(());
        }
        if data.len() != self.ram.len() + 8 {
            return Err("loaded RAM has incorrect length");
        }

        let mut rtc = [0; 8];
        rtc.copy_from_slice(&data[..8]);
        if self.rtc_zero.is_some() {
            self.rtc_zero = Some(u64::from_be_bytes(rtc));
        }
        self.ram.copy_from_slice(&data[8..]);
        Ok(())
    }

    pub(super) fn dump_ram(&self) -> Vec<u8> {
        let mut data = self.rtc_zero.unwrap_or(0).to_be_bytes().to_vec();
        data.extend_from_slice(&self.ram);
        data
    }

    pub(super) fn check_and_reset_ram_updated(&mut self) -> bool {
        let updated = self.ram_updated;
        self.ram_updated = false;
        updated
    }
}

impl Mbc5 {
    pub(super) fn new(rom: &[u8]) -> Self {
        let cartridge_type = rom_header(rom, 0x147);
        let ram_banks = match cartridge_type {
            0x1a | 0x1b | 0x1d | 0x1e => ram_bank_count(rom_header(rom, 0x149)),
            _ => 0,
        };

        Self {
            ram: vec![0; ram_banks * 0x2000],
            rom_bank: 1,
            ram_bank: 0,
            ram_on: false,
            ram_updated: false,
            has_battery: matches!(cartridge_type, 0x1b | 0x1e),
            rom_banks: rom_bank_count(rom),
            ram_banks,
        }
    }

    pub(super) fn read_rom(&self, rom: &[u8], addr: u16) -> u8 {
        let bank = if addr < 0x4000 { 0 } else { self.rom_bank };
        read_rom_bank(rom, bank, addr)
    }

    pub(super) fn read_ram(&self, addr: u16) -> u8 {
        if !self.ram_on || self.ram_bank >= self.ram_banks {
            return 0xff;
        }
        read_ram_bank(&self.ram, self.ram_bank, addr)
    }

    pub(super) fn write_rom(&mut self, addr: u16, value: u8) {
        match addr {
            0x0000..=0x1fff => self.ram_on = value & 0x0f == 0x0a,
            0x2000..=0x2fff => {
                self.rom_bank = normalize_bank((self.rom_bank & 0x100) | usize::from(value), self.rom_banks);
            }
            0x3000..=0x3fff => {
                self.rom_bank = normalize_bank((self.rom_bank & 0x0ff) | (usize::from(value & 0x01) << 8), self.rom_banks);
            }
            0x4000..=0x5fff => {
                if self.ram_banks != 0 {
                    self.ram_bank = usize::from(value & 0x0f) % self.ram_banks;
                }
            }
            0x6000..=0x7fff => {}
            _ => {}
        }
    }

    pub(super) fn write_ram(&mut self, addr: u16, value: u8) {
        if !self.ram_on || self.ram_bank >= self.ram_banks {
            return;
        }

        if write_ram_bank(&mut self.ram, self.ram_bank, addr, value) {
            self.ram_updated = true;
        }
    }

    pub(super) fn load_ram(&mut self, data: &[u8]) -> Result<(), &'static str> {
        load_exact_ram(&mut self.ram, data)
    }

    pub(super) fn dump_ram(&self) -> Vec<u8> {
        self.ram.clone()
    }

    pub(super) fn check_and_reset_ram_updated(&mut self) -> bool {
        let updated = self.ram_updated;
        self.ram_updated = false;
        updated
    }
}

fn rom_header(rom: &[u8], addr: usize) -> u8 {
    rom.get(addr).copied().unwrap_or(0)
}

fn rom_bank_count(rom: &[u8]) -> usize {
    match rom_header(rom, 0x148) {
        n @ 0x00..=0x08 => 2usize << usize::from(n),
        0x52 => 72,
        0x53 => 80,
        0x54 => 96,
        _ => rom.len().div_ceil(0x4000).max(1),
    }
}

fn ram_bank_count(value: u8) -> usize {
    match value {
        0x01 | 0x02 => 1,
        0x03 => 4,
        0x04 => 16,
        0x05 => 8,
        _ => 0,
    }
}

fn normalize_bank(bank: usize, bank_count: usize) -> usize {
    if bank_count == 0 {
        bank
    } else {
        bank % bank_count
    }
}

fn read_rom_bank(rom: &[u8], bank: usize, addr: u16) -> u8 {
    let bank = normalize_bank(bank, rom_bank_count(rom));
    let offset = (bank * 0x4000) | usize::from(addr & 0x3fff);
    rom.get(offset).copied().unwrap_or(0xff)
}

fn read_ram_bank(ram: &[u8], bank: usize, addr: u16) -> u8 {
    let offset = (bank * 0x2000) | usize::from(addr & 0x1fff);
    ram.get(offset).copied().unwrap_or(0xff)
}

fn write_ram_bank(ram: &mut [u8], bank: usize, addr: u16, value: u8) -> bool {
    let offset = (bank * 0x2000) | usize::from(addr & 0x1fff);
    if let Some(byte) = ram.get_mut(offset) {
        *byte = value;
        true
    } else {
        false
    }
}

fn load_exact_ram(ram: &mut Vec<u8>, data: &[u8]) -> Result<(), &'static str> {
    if data.len() != ram.len() {
        return Err("loaded RAM has incorrect length");
    }
    ram.copy_from_slice(data);
    Ok(())
}

