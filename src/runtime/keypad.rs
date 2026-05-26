use std::sync::mpsc::Receiver;

pub struct Keypad {
    row0: u8,
    row1: u8,
    data: u8,
    pub interrupt: u8,
    events: Option<Receiver<KeypadEvent>>,
}

#[derive(Copy, Clone, Debug)]
pub enum KeypadKey {
    Right,
    Left,
    Up,
    Down,
    A,
    B,
    Select,
    Start,
}

#[derive(Copy, Clone, Debug)]
pub enum KeypadEvent {
    Down(KeypadKey),
    Up(KeypadKey),
}

impl Default for Keypad {
    fn default() -> Self {
        Self::new(None)
    }
}

impl Keypad {
    pub fn new(events: Option<Receiver<KeypadEvent>>) -> Keypad {
        Keypad {
            row0: 0x0F,
            row1: 0x0F,
            data: 0xFF,
            interrupt: 0,
            events,
        }
    }

    /// Read the joypad register (FF00). Returns the currently latched value;
    /// callers must invoke [`Keypad::do_cycle`] periodically to drain pending
    /// input events into the latched state.
    pub fn rb(&self) -> u8 {
        self.data
    }

    /// Write to the joypad register (FF00). Only the row-select bits (4 and 5)
    /// are writable.
    pub fn wb(&mut self, value: u8) {
        self.data = (self.data & 0xCF) | (value & 0x30);
        self.refresh();
    }

    /// Drain any pending keypad events from the receiver and update the
    /// latched joypad state. Should be called from the CPU's per-cycle hook.
    pub fn do_cycle(&mut self) {
        // Pull pending events into a local buffer first so that we don't hold
        // an immutable borrow of `self.events` while we mutate the row state.
        let mut pending: [KeypadEvent; 16] = [KeypadEvent::Up(KeypadKey::A); 16];
        let mut count = 0usize;
        if let Some(events) = &self.events {
            while count < pending.len() {
                match events.try_recv() {
                    Ok(event) => {
                        pending[count] = event;
                        count += 1;
                    }
                    Err(_) => break,
                }
            }
        }
        for event in &pending[..count] {
            match *event {
                KeypadEvent::Down(key) => self.keydown(key),
                KeypadEvent::Up(key) => self.keyup(key),
            }
        }
        self.refresh();
    }

    fn refresh(&mut self) {
        let old_values = self.data & 0xF;
        let mut new_values = 0xF;

        if self.data & 0x10 == 0x00 {
            new_values &= self.row0;
        }
        if self.data & 0x20 == 0x00 {
            new_values &= self.row1;
        }

        // Any high-to-low transition on the selected lines triggers the
        // joypad interrupt (bit 4 of IF).
        if old_values & !new_values != 0 {
            self.interrupt |= 0x10;
        }

        self.data = (self.data & 0xF0) | new_values;
    }

    fn keydown(&mut self, key: KeypadKey) {
        match key {
            KeypadKey::Right => self.row0 &= !(1 << 0),
            KeypadKey::Left => self.row0 &= !(1 << 1),
            KeypadKey::Up => self.row0 &= !(1 << 2),
            KeypadKey::Down => self.row0 &= !(1 << 3),
            KeypadKey::A => self.row1 &= !(1 << 0),
            KeypadKey::B => self.row1 &= !(1 << 1),
            KeypadKey::Select => self.row1 &= !(1 << 2),
            KeypadKey::Start => self.row1 &= !(1 << 3),
        }
    }

    fn keyup(&mut self, key: KeypadKey) {
        match key {
            KeypadKey::Right => self.row0 |= 1 << 0,
            KeypadKey::Left => self.row0 |= 1 << 1,
            KeypadKey::Up => self.row0 |= 1 << 2,
            KeypadKey::Down => self.row0 |= 1 << 3,
            KeypadKey::A => self.row1 |= 1 << 0,
            KeypadKey::B => self.row1 |= 1 << 1,
            KeypadKey::Select => self.row1 |= 1 << 2,
            KeypadKey::Start => self.row1 |= 1 << 3,
        }
    }
}
