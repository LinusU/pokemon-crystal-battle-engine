//! Battle text: a port of the original text engine's command interpreter
//! (`home/text.asm`) that renders a text label to the exact strings the
//! emulated harness captures (see `bootstrap::events`): one string per
//! message, text-box lines separated by `\n`.
//!
//! Rendering only reads memory: the logic of `text_asm` blocks that change the
//! battle is ported where the battle code prints them; here each block only
//! chooses which text to print next.

use super::battle::Engine;
use super::rom;
use super::rsym as r;
use super::sym::*;
use crate::bootstrap::events::decode;

/// Text being assembled into messages.
#[derive(Default)]
struct Out {
    line: String,
    lines: Vec<String>,
    messages: Vec<String>,
}

impl Out {
    fn push(&mut self, s: &str) {
        self.line.push_str(s);
    }

    fn newline(&mut self) {
        let line = std::mem::take(&mut self.line);
        let line = line.trim_end();
        if !line.is_empty() {
            self.lines.push(line.to_string());
        }
    }

    fn end_message(&mut self) {
        self.newline();
        if !self.lines.is_empty() {
            self.messages.push(std::mem::take(&mut self.lines).join("\n"));
        }
    }
}

/// A place text can live: ROM (flat offset) or RAM.
#[derive(Clone, Copy)]
enum Ptr {
    Rom(usize),
    Ram(u16),
}

impl Engine {
    fn text_byte(&self, p: Ptr) -> u8 {
        match p {
            Ptr::Rom(o) => rom::byte(o),
            Ptr::Ram(a) => self.mem.rb(a),
        }
    }

    /// Render the text at ROM `label` to its messages.
    pub fn render_text(&self, label: usize, choices: &[usize]) -> Vec<String> {
        let mut out = Out::default();
        let mut choices = choices.iter();
        self.run_text_commands(Ptr::Rom(label), &mut out, 0, &mut choices);
        out.end_message();
        out.messages
    }

    /// `DoTextUntilTerminator`.
    fn run_text_commands(&self, start: Ptr, out: &mut Out, depth: u8, choices: &mut std::slice::Iter<'_, usize>) {
        assert!(depth < 16, "text nesting too deep");
        let mut p = start;
        let next = |p: Ptr, n: usize| match p {
            Ptr::Rom(o) => Ptr::Rom(o + n),
            Ptr::Ram(a) => Ptr::Ram(a.wrapping_add(n as u16)),
        };
        let bank = |p: Ptr| match p {
            Ptr::Rom(o) => rom::bank_of(o),
            Ptr::Ram(_) => 0,
        };
        let word = |e: &Self, p: Ptr| u16::from(e.text_byte(p)) | u16::from(e.text_byte(next(p, 1))) << 8;
        loop {
            let cmd = self.text_byte(p);
            p = next(p, 1);
            match cmd {
                0x50 => return, // TX_END
                0x00 => match self.place_string(p, out) {
                    Some(n) => p = n,
                    None => return,
                },
                0x01 => {
                    // TX_RAM
                    let addr = word(self, p);
                    p = next(p, 2);
                    if self.place_string(Ptr::Ram(addr), out).is_none() {
                        return;
                    }
                }
                0x02 => {
                    // TX_BCD
                    self.unported("text_bcd");
                }
                0x03 | 0x04 => {
                    // TX_MOVE / TX_BOX: cursor moves and boxes; no characters.
                    p = next(p, if cmd == 0x03 { 2 } else { 4 });
                }
                0x05 | 0x07 => out.newline(), // TX_LOW, TX_SCROLL
                0x06 | 0x0d => out.end_message(), // TX_PROMPT_BUTTON, TX_WAIT_BUTTON
                0x08 => {
                    // TX_START_ASM: the code picks the next text.
                    let here = match p {
                        Ptr::Rom(o) => o,
                        Ptr::Ram(_) => panic!("text_asm in RAM"),
                    };
                    match self.text_asm(here, choices) {
                        Some(label) => return self.run_text_commands(Ptr::Rom(label), out, depth + 1, choices),
                        None => return,
                    }
                }
                0x09 => {
                    // TX_DECIMAL: address, then (bytes << 4) | digits.
                    let addr = word(self, p);
                    let param = self.text_byte(next(p, 2));
                    p = next(p, 3);
                    let bytes = param >> 4;
                    let digits = usize::from(param & 0xf);
                    let mut value: u32 = 0;
                    for i in 0..u16::from(bytes) {
                        value = value << 8 | u32::from(self.mem.rb(addr + i));
                    }
                    for c in print_num_left_aligned(value, digits as u8) {
                        out.push(decode(c));
                    }
                }
                0x0a | 0x0b | 0x0e..=0x13 => {} // pause and sounds
                0x0c => {
                    // TX_DOTS: n ellipses.
                    let n = self.text_byte(p);
                    p = next(p, 1);
                    for _ in 0..n {
                        out.push("…");
                    }
                }
                0x14 => {
                    // TX_STRINGBUFFER
                    self.unported("text_buffer");
                }
                0x15 => self.unported("text_today"),
                0x16 => {
                    // TX_FAR: address, bank.
                    let addr = word(self, p);
                    let b = self.text_byte(next(p, 2));
                    p = next(p, 3);
                    self.run_text_commands(Ptr::Rom(rom::flat(b, addr)), out, depth + 1, choices);
                }
                _ => panic!("unknown text command {cmd:#04x} at bank {:02x}", bank(p)),
            }
        }
    }

    /// `PlaceString` / `PlaceNextChar`: characters up to `@`. Returns the
    /// pointer past the terminator, or `None` when `<DONE>` / `<PROMPT>`
    /// ended the whole text.
    fn place_string(&self, mut p: Ptr, out: &mut Out) -> Option<Ptr> {
        let step = |p: Ptr| match p {
            Ptr::Rom(o) => Ptr::Rom(o + 1),
            Ptr::Ram(a) => Ptr::Ram(a.wrapping_add(1)),
        };
        loop {
            let c = self.text_byte(p);
            p = step(p);
            match c {
                0x50 => return Some(p), // '@'
                0x4f | 0x4e | 0x22 | 0x4b | 0x4c | 0x55 => out.newline(), // LINE NEXT LF _CONT SCROLL CONT
                0x51 => out.end_message(),                                  // PARA
                0x57 | 0x58 => {
                    // DONE / PROMPT end the whole text.
                    out.end_message();
                    return None;
                }
                0x52 => {
                    self.place_string(Ptr::Ram(W_PLAYER_NAME), out);
                }
                0x14 => {
                    // <PLAY_G>: the player's name (no suffix in English).
                    self.place_string(Ptr::Ram(W_PLAYER_NAME), out);
                }
                0x53 => {
                    self.place_string(Ptr::Ram(W_RIVAL_NAME), out);
                }
                0x54 => out.push("POKé"),
                0x56 => out.push("……"),
                0x5b => out.push("PC"),
                0x5c => out.push("TM"),
                0x5d => out.push("TRAINER"),
                0x5e => out.push("ROCKET"),
                0x4a => out.push("PKMN"),
                0x24 => out.push("POKE"),
                0x1f => out.push(" "),
                0x25 => {} // <WBR>
                0x59 | 0x5a => {
                    // <TARGET> / <USER>
                    let turn = self.turn() ^ u8::from(c == 0x59);
                    if turn == 0 {
                        self.place_string(Ptr::Ram(W_BATTLE_MON_NICKNAME), out);
                    } else {
                        self.place_string(Ptr::Rom(r::ENEMY_TEXT), out);
                        self.place_string(Ptr::Ram(W_ENEMY_MON_NICKNAME), out);
                    }
                }
                0x3f => {
                    // <ENEMY>
                    if self.link_mode() != 0 {
                        self.place_string(Ptr::Ram(W_OT_CLASS_NAME), out);
                    } else {
                        let class = self.mem.rb(W_TRAINER_CLASS);
                        if class == crate::defs::RIVAL1 || class == crate::defs::RIVAL2 {
                            self.place_string(Ptr::Ram(W_RIVAL_NAME), out);
                        } else {
                            self.place_string(Ptr::Ram(W_OT_CLASS_NAME), out);
                            out.push(" ");
                            self.place_string(Ptr::Ram(W_STRING_BUFFER1), out);
                        }
                    }
                }
                _ => out.push(decode(c)),
            }
        }
    }

    /// The `text_asm` block at ROM `addr` (just after its `text_asm` byte):
    /// returns the text it hands back in `hl`, or `None` to stop. Blocks
    /// whose choice the battle code already computed take it from `choices`.
    fn text_asm(&self, addr: usize, choices: &mut std::slice::Iter<'_, usize>) -> Option<usize> {
        let name = super::rsym_name(addr);
        let base = name.split('+').next().unwrap_or("");
        let grammar = self.mem.rb(W_MOVE_GRAMMAR);
        let disobeyed = self.mem.rb(W_ALREADY_DISOBEYED) != 0;
        Some(match base {
            "UsedMoveText" => {
                if disobeyed || grammar < 3 { r::USED_MOVE2_TEXT } else { r::USED_MOVE1_TEXT }
            }
            // UsedMove2Text's text_asm falls straight into this label.
            "UsedMove1Text" | "UsedMove2Text" | "UsedMoveText_CheckObedience" => {
                if disobeyed { r::USED_MOVE_TEXT_CHECK_OBEDIENCE_USED_INSTEAD_TEXT } else { r::MOVE_NAME_TEXT }
            }
            "UsedMoveText_CheckObedience.UsedInsteadText" | "UsedMoveText_CheckObedience.GetMoveNameText" => r::MOVE_NAME_TEXT,
            "MoveNameText" => [
                r::END_USED_MOVE1_TEXT,
                r::END_USED_MOVE2_TEXT,
                r::END_USED_MOVE3_TEXT,
                r::END_USED_MOVE4_TEXT,
                r::END_USED_MOVE5_TEXT,
            ][usize::from(grammar)],
            "GoMonText" | "DoItMonText" | "GoForItMonText" | "YourFoesWeakGetmMonText" | "PrepareBattleMonNicknameText" => r::BATTLE_MON_NICKNAME_TEXT,
            "WithdrawMonText.WithdrawMonText" => *choices.next().expect("WithdrawMonText needs its choice"),
            "Text_MonGainedExpPoint" => {
                if self.mem.rb(W_STRING_BUFFER2 + 2) == 0 { r::EXP_POINTS_TEXT } else { r::BOOSTED_EXP_POINTS_TEXT }
            }
            "BattleCommand_StatUpMessage" | "BattleCommand_StatUpMessage.stat" => {
                if self.mem.rb(W_LOWERED_STAT) & 0xf0 == 0 {
                    r::BATTLE_COMMAND_STAT_UP_MESSAGE_BATTLE_STAT_WENT_UP_TEXT
                } else {
                    r::BATTLE_COMMAND_STAT_UP_MESSAGE_BATTLE_STAT_WENT_WAY_UP_TEXT
                }
            }
            "BattleCommand_StatDownMessage" | "BattleCommand_StatDownMessage.stat" => {
                if self.mem.rb(W_LOWERED_STAT) & 0xf0 == 0 {
                    r::BATTLE_COMMAND_STAT_DOWN_MESSAGE_BATTLE_STAT_FELL_TEXT
                } else {
                    r::BATTLE_COMMAND_STAT_DOWN_MESSAGE_BATTLE_STAT_SHARPLY_FELL_TEXT
                }
            }
            "BattleCommand_Charge.UsedText" => {
                use crate::defs::*;
                match self.battle_var(BATTLE_VARS_MOVE_ANIM) {
                    RAZOR_WIND => r::BATTLE_COMMAND_CHARGE_BATTLE_MADE_WHIRLWIND_TEXT,
                    SOLARBEAM => r::BATTLE_COMMAND_CHARGE_BATTLE_TOOK_SUNLIGHT_TEXT,
                    SKULL_BASH => r::BATTLE_COMMAND_CHARGE_BATTLE_LOWERED_HEAD_TEXT,
                    SKY_ATTACK => r::BATTLE_COMMAND_CHARGE_BATTLE_GLOWING_TEXT,
                    FLY => r::BATTLE_COMMAND_CHARGE_BATTLE_FLEW_TEXT,
                    // DIG — and, as in the original, `hl` stays DugText for anything else.
                    _ => r::BATTLE_COMMAND_CHARGE_BATTLE_DUG_TEXT,
                }
            }
            "Text_GotchaMonWasCaught" => r::WAIT_BUTTON_TEXT,
            _ => panic!("unported text_asm at {addr:#08x} ({name})"),
        })
    }
}

/// `_PrintNum` as `TextCommand_DECIMAL` calls it (left-aligned, no leading
/// zeros, no money sign): the characters it writes for `value`. Faithful to
/// the original's quirks: a digit count outside 2..=6 prints seven places,
/// a place's digit can exceed 9 when `value` doesn't fit the places (it is
/// then the character after `'9'`), and the final two places only look at
/// the low byte of what is left.
fn print_num_left_aligned(value: u32, digits: u8) -> Vec<u8> {
    const ZERO: u8 = 0xf6; // '0'
    let places: &[u32] = match digits {
        2 => &[],
        3 => &[100],
        4 => &[1000, 100],
        5 => &[10000, 1000, 100],
        6 => &[100_000, 10000, 1000, 100],
        _ => &[1_000_000, 100_000, 10000, 1000, 100],
    };
    let mut v = value & 0xff_ffff;
    let mut out = Vec::new();
    let mut started = false;
    for &place in places {
        let c = (v / place) as u8;
        v %= place;
        if started || c != 0 {
            started = true;
            out.push(ZERO.wrapping_add(c));
        }
    }
    // .two: tens and units from the low byte.
    let low = (v & 0xff) as u8;
    let (tens, units) = (low / 10, low % 10);
    if started || tens != 0 {
        out.push(ZERO.wrapping_add(tens));
    }
    out.push(ZERO + units);
    out
}
