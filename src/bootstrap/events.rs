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

pub use crate::battle::event::{Event, HpBarTarget, decode, in_text_area, one_line};

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

#[cfg(test)]
mod tests {
    use super::*;

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
