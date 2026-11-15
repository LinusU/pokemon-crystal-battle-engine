//! Capture what the battle shows: the exact battle text and HP-bar changes.
//!
//! Hooks in the transpiled engine (all marked `HARNESS-PATCH:`) report every
//! character the text engine (`home/text.asm`) places into the text-box area
//! of the screen, every number `text_decimal` prints there, every line break /
//! message end, and every HP-bar animation (`_AnimateHPBar`). The text comes
//! out exactly as a player would read it, one string per message, lines
//! separated by `\n`.
//!
//! Capture is off unless `cpu.events` is `Some`, so the hooks cost one branch
//! when nobody is listening. Events only *observe* — nothing here writes to
//! the engine's memory, so capture can never change a battle.

use crate::runtime::Cpu;

/// Which HP bar an [`Event::HpBar`] animates.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HpBarTarget {
    /// The enemy's battle HUD.
    Enemy,
    /// The player's battle HUD.
    Player,
    /// A party-menu bar (an item used on party slot `n`).
    Party(u8),
}

/// Something the battle showed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    /// One battle-text message, exactly as printed; text-box lines separated
    /// by `\n` (see [`one_line`] for display).
    Text(String),
    /// An HP bar moved from `from` to `to` (of `max`).
    HpBar { target: HpBarTarget, from: u16, to: u16, max: u16 },
}

/// `wTilemap` (`00:c4a0`), 20 tiles per row. The battle text box (and the
/// party-menu text box items print into) covers rows 12..=17; HUDs, names and
/// menus elsewhere on screen are not battle text.
const W_TILEMAP: u16 = 0xc4a0;
const SCREEN_WIDTH: u16 = 20;
const TEXT_AREA_START: u16 = W_TILEMAP + 12 * SCREEN_WIDTH;
const TEXT_AREA_END: u16 = W_TILEMAP + 18 * SCREEN_WIDTH;

/// Capture state. Install with `cpu.events = Some(RefCell::new(EventCapture::default()))`.
#[derive(Debug, Default)]
pub struct EventCapture {
    line: String,
    lines: Vec<String>,
    events: Vec<Event>,
    /// Also print each event to stdout as it happens (the CLI's live text
    /// view). Events are still queued for [`take_events`].
    pub echo: bool,
}

pub(crate) fn in_text_area(addr: u16) -> bool {
    (TEXT_AREA_START..TEXT_AREA_END).contains(&addr)
}

/// `PlacePartyMenuText`'s prompts ("Use on which PKMN?", …) are party-menu UI
/// drawn with a bare `PlaceString`, not battle text: `PartyMenuStrings`
/// through `YouHaveNoPKMNString` in bank 0x14 (`14:44d2`..`14:4570`). So is
/// `PlacePartyNicknames`' "CANCEL" (`14:40c8`), which a full party draws on
/// the text box's top row.
fn is_party_menu_prompt(cpu: &Cpu) -> bool {
    cpu.current_rom_bank() == 0x14
        && ((0x44d2..0x4570).contains(&cpu.de()) || (0x40c8..0x40cf).contains(&cpu.de()))
}

/// `.place` in `CheckDict`: character `cpu.a` is about to be written at `hl`.
pub fn place(cpu: &mut Cpu) {
    let (ch, addr) = (cpu.a, cpu.hl());
    let party_prompt = cpu.events.is_some() && is_party_menu_prompt(cpu);
    if let Some(cell) = &cpu.events {
        let mut cap = cell.borrow_mut();
        if in_text_area(addr) && !party_prompt {
            cap.line.push_str(decode(ch));
        }
    }
}

/// `text_decimal` printed its digits into `start..end`.
pub fn placed_range(cpu: &mut Cpu, start: u16, end: u16) {
    let Some(cell) = &cpu.events else { return };
    if !in_text_area(start) {
        return;
    }
    let text: String = (start..end).map(|a| decode(cpu.read_byte(a))).collect();
    cell.borrow_mut().line.push_str(&text);
}

/// A line break (`<LINE>`, `<NEXT>`, `<LF>`, a scroll).
pub fn newline(cpu: &mut Cpu) {
    if let Some(cell) = &cpu.events {
        cell.borrow_mut().end_line();
    }
}

/// The end of a message (`<PARA>`, `<PROMPT>`, `<DONE>`, `text_promptbutton`).
pub fn end_message(cpu: &mut Cpu) {
    if let Some(cell) = &cpu.events {
        cell.borrow_mut().end_message();
    }
}

/// `_AnimateHPBar` is about to animate `wWhichHPBar` (0 enemy, 1 player,
/// 2 party menu) from `wCurHPAnimOldHP` to `wCurHPAnimNewHP` of
/// `wCurHPAnimMaxHP` (little-endian words at `d1ec` / `d1ee` / `d1ea`).
pub fn hp_bar(cpu: &mut Cpu) {
    let Some(cell) = &cpu.events else { return };
    let le16 = |addr: u16| u16::from(cpu.read_byte(addr)) | u16::from(cpu.read_byte(addr + 1)) << 8;
    let target = match cpu.read_byte(0xd10a) {
        0 => HpBarTarget::Enemy,
        1 => HpBarTarget::Player,
        _ => HpBarTarget::Party(cpu.read_byte(0xd109)),
    };
    let event = Event::HpBar { target, from: le16(0xd1ec), to: le16(0xd1ee), max: le16(0xd1ea) };
    let mut cap = cell.borrow_mut();
    cap.end_message();
    if cap.echo {
        if let Event::HpBar { target, from, to, max } = &event {
            println!("  [{target:?} HP {from} → {to} / {max}]");
        }
    }
    cap.events.push(event);
}

/// Every event since the last call (an open message is closed first).
pub fn take_events(cpu: &Cpu) -> Vec<Event> {
    match &cpu.events {
        Some(cell) => {
            let mut cap = cell.borrow_mut();
            cap.end_message();
            std::mem::take(&mut cap.events)
        }
        None => Vec::new(),
    }
}

impl EventCapture {
    /// A capture that also prints each event to stdout as it happens.
    pub fn echoing() -> Self {
        EventCapture { echo: true, ..Default::default() }
    }

    fn end_line(&mut self) {
        let line = std::mem::take(&mut self.line);
        let line = line.trim_end();
        if !line.is_empty() {
            self.lines.push(line.to_string());
        }
    }

    fn end_message(&mut self) {
        self.end_line();
        if !self.lines.is_empty() {
            let message = std::mem::take(&mut self.lines).join("\n");
            if self.echo {
                println!("» {}", one_line(&message));
            }
            self.events.push(Event::Text(message));
        }
    }
}

/// A message on one line: text-box lines joined by a space, except after a
/// line that ends in a hyphen ("super-" / "effective!").
pub fn one_line(message: &str) -> String {
    let mut out = String::new();
    for line in message.split('\n') {
        if !out.is_empty() && !out.ends_with('-') {
            out.push(' ');
        }
        out.push_str(line);
    }
    out
}

/// One tile of the game's charmap (`constants/charmap.asm`) as text.
pub fn decode(ch: u8) -> &'static str {
    const UPPER: [&str; 26] = [
        "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T",
        "U", "V", "W", "X", "Y", "Z",
    ];
    const LOWER: [&str; 26] = [
        "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t",
        "u", "v", "w", "x", "y", "z",
    ];
    const DIGITS: [&str; 10] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
    match ch {
        0x80..=0x99 => UPPER[usize::from(ch - 0x80)],
        0xa0..=0xb9 => LOWER[usize::from(ch - 0xa0)],
        0xf6..=0xff => DIGITS[usize::from(ch - 0xf6)],
        0x7f => " ",
        0x70 => "PO",
        0x71 => "KE",
        0x72 => "“",
        0x73 => "”",
        0x74 => "·",
        0x75 => "…",
        0x9a => "(",
        0x9b => ")",
        0x9c => ":",
        0x9d => ";",
        0x9e => "[",
        0x9f => "]",
        0xc0 => "Ä",
        0xc1 => "Ö",
        0xc2 => "Ü",
        0xc3 => "ä",
        0xc4 => "ö",
        0xc5 => "ü",
        0xd0 => "'d",
        0xd1 => "'l",
        0xd2 => "'m",
        0xd3 => "'r",
        0xd4 => "'s",
        0xd5 => "'t",
        0xd6 => "'v",
        0xdf => "←",
        0xe0 => "'",
        0xe1 => "PK",
        0xe2 => "MN",
        0xe3 => "-",
        0xe6 => "?",
        0xe7 => "!",
        0xe8 => ".",
        0xe9 => "&",
        0xea => "é",
        0xeb => "→",
        0xec => "▷",
        0xed => "▶",
        0xee => "▼",
        0xef => "♂",
        0xf0 => "¥",
        0xf1 => "×",
        0xf2 => ".",
        0xf3 => "/",
        0xf4 => ",",
        0xf5 => "♀",
        _ => "",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_the_english_charmap() {
        let text: String = [0x87, 0xa4, 0xab, 0xab, 0xae, 0xe7, 0x7f, 0xf7, 0xf8, 0x75].iter().map(|&c| decode(c)).collect();
        assert_eq!(text, "Hello! 12…");
        assert_eq!(decode(0xe1).to_string() + decode(0xe2), "PKMN");
    }

    #[test]
    fn one_line_keeps_hyphenated_words_together() {
        assert_eq!(one_line("It's super-\neffective!"), "It's super-effective!");
        assert_eq!(one_line("Enemy GROWLITHE\nused EMBER!"), "Enemy GROWLITHE used EMBER!");
    }

    #[test]
    fn messages_split_on_boundaries() {
        let mut cap = EventCapture::default();
        cap.line.push_str("Wild PIDGEY");
        cap.end_line();
        cap.line.push_str("appeared!   ");
        cap.end_message();
        cap.end_message();
        assert_eq!(cap.events, vec![Event::Text("Wild PIDGEY\nappeared!".into())]);
    }
}
