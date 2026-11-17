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

pub use crate::battle::event::{HpBarTarget, decode, in_text_area, one_line};

/// What the original battle showed, as the harness captures it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Shown {
    /// One battle-text message, exactly as printed; text-box lines separated
    /// by `\n` (see [`one_line`] for display).
    Text(String),
    /// An HP bar moved from `from` to `to` (of `max`).
    HpBar { target: HpBarTarget, from: u16, to: u16, max: u16 },
    /// `PlayBattleAnim`: animation `wFXAnimID` (a move, or an `ANIM_*` id
    /// from $100) from `hBattleTurn`'s side, with `wBattleAnimParam`.
    Animation { anim: u16, turn: u8, param: u8 },
    /// `PlaySFX` outside an animation, text and the game's UI.
    Sound { sfx: u16 },
    /// `PlayStereoCry`: the cry of `species`.
    Cry { species: u8 },
    /// `PlayerMonFaintedAnimation` (0) / `EnemyMonFaintedAnimation` (1).
    Faint { side: u8 },
    /// `SlideBattlePicOut`: the player's (0) or the enemy's (1) picture.
    SlideOut { side: u8 },
    /// `BattleWinSlideInEnemyTrainerFrontpic`.
    TrainerSlidesIn,
    /// `AnimateExpBar`, when it animates: party mon `slot` gains `gained`.
    ExpBar { slot: u8, gained: u16 },
    /// `PrintTempMonStats`: the stats box of party mon `slot`.
    StatsBox { slot: u8 },
    /// `AnimateFrontpic`: the enemy mon's front picture animates (with its
    /// cry): a mon appearing.
    Frontpic { species: u8 },
    /// The thrown ball's animation shook `wobbles` times, then clicked
    /// (`caught`) or broke open.
    BallWobbles { wobbles: u8, caught: bool },
}

/// Capture state. Install with `cpu.events = Some(RefCell::new(EventCapture::default()))`.
#[derive(Debug, Default)]
pub struct EventCapture {
    line: String,
    lines: Vec<String>,
    events: Vec<Shown>,
    /// Also print each event to stdout as it happens (the CLI's live text
    /// view). Events are still queued for [`take_events`].
    pub echo: bool,
    /// Animations playing (their own sounds are part of them).
    anim_depth: u32,
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
    let event = Shown::HpBar { target, from: le16(0xd1ec), to: le16(0xd1ee), max: le16(0xd1ea) };
    let mut cap = cell.borrow_mut();
    cap.end_message();
    if cap.echo {
        if let Shown::HpBar { target, from, to, max } = &event {
            println!("  [{target:?} HP {from} → {to} / {max}]");
        }
    }
    cap.events.push(event);
}

/// `PlayBattleAnim` is about to play `wFXAnimID`.
pub fn battle_anim(cpu: &mut Cpu) {
    let Some(cell) = &cpu.events else { return };
    use crate::engine::sym::{H_BATTLE_TURN, W_BATTLE_ANIM_PARAM, W_FX_ANIM_ID};
    let anim = u16::from(cpu.read_byte(W_FX_ANIM_ID)) | u16::from(cpu.read_byte(W_FX_ANIM_ID + 1)) << 8;
    let event = Shown::Animation { anim, turn: cpu.read_byte(H_BATTLE_TURN), param: cpu.read_byte(W_BATTLE_ANIM_PARAM) };
    let mut cap = cell.borrow_mut();
    cap.end_message();
    cap.events.push(event);
    cap.anim_depth += 1;
}

/// `PlayBattleAnim` is done.
pub fn battle_anim_done(cpu: &mut Cpu) {
    if let Some(cell) = &cpu.events {
        let mut cap = cell.borrow_mut();
        cap.anim_depth = cap.anim_depth.saturating_sub(1);
    }
}

/// The routines whose sounds are part of something else the battle shows:
/// text-box prompts and menus (the game's UI), text commands and the texts'
/// own code (part of a message), the experience bar's.
const SOUNDS_NOT_SHOWN: [&str; 5] = ["PromptButton", "PlayClickSFX", "TextCommand_", "Text_", "AnimateExpBar"];

/// `PlaySFX` is about to play `de`.
pub fn sfx(cpu: &mut Cpu) {
    let Some(cell) = &cpu.events else { return };
    if cell.borrow().anim_depth > 0 {
        return;
    }
    let ret = cpu.read_word(cpu.sp);
    let caller = crate::engine::rsym_name(if ret < 0x4000 { usize::from(ret) } else { usize::from(cpu.current_rom_bank()) * 0x4000 + usize::from(ret - 0x4000) });
    if SOUNDS_NOT_SHOWN.iter().any(|prefix| caller.starts_with(prefix)) {
        return;
    }
    push(cpu, Shown::Sound { sfx: cpu.de() });
}

fn push(cpu: &Cpu, event: Shown) {
    if let Some(cell) = &cpu.events {
        let mut cap = cell.borrow_mut();
        cap.end_message();
        cap.events.push(event);
    }
}

/// A mon faints.
pub fn faint(cpu: &mut Cpu, side: u8) {
    push(cpu, Shown::Faint { side });
}

/// `SlideBattlePicOut` at `hl`: the player's picture (left) or the enemy's.
pub fn slide_out(cpu: &mut Cpu) {
    let column = cpu.hl().wrapping_sub(0xc4a0) % 20;
    let side = u8::from(column >= 10);
    push(cpu, Shown::SlideOut { side });
}

/// The enemy trainer slides back in.
pub fn trainer_slides_in(cpu: &mut Cpu) {
    push(cpu, Shown::TrainerSlidesIn);
}

/// `AnimateExpBar`: it animates when the mon is the active one and not at
/// the maximum level; the experience gained is in `hProduct + 2..3`.
pub fn exp_bar(cpu: &mut Cpu) {
    let slot = cpu.read_byte(crate::engine::sym::W_CUR_PARTY_MON);
    if slot != cpu.read_byte(crate::engine::sym::W_CUR_BATTLE_MON) || cpu.read_byte(crate::engine::sym::W_BATTLE_MON_LEVEL) >= crate::defs::MAX_LEVEL {
        return;
    }
    let gained = u16::from(cpu.read_byte(crate::engine::sym::H_PRODUCT + 2)) << 8 | u16::from(cpu.read_byte(crate::engine::sym::H_PRODUCT + 3));
    push(cpu, Shown::ExpBar { slot, gained });
}

/// The ball-throw animation's outcome (see the HARNESS-PATCH in
/// `poke_ball_effect_not_kurt_ball`).
pub fn ball_wobbles(cpu: &mut Cpu, wobbles: u8, caught: bool) {
    push(cpu, Shown::BallWobbles { wobbles, caught });
}

/// `AnimateFrontpic` animates the enemy's picture (`wTempEnemyMonSpecies`).
pub fn frontpic(cpu: &mut Cpu) {
    let species = cpu.read_byte(crate::engine::sym::W_TEMP_ENEMY_MON_SPECIES);
    push(cpu, Shown::Frontpic { species });
}

/// `PrintTempMonStats`: the stats box of `wCurPartyMon`.
pub fn stats_box(cpu: &mut Cpu) {
    let slot = cpu.read_byte(crate::engine::sym::W_CUR_PARTY_MON);
    push(cpu, Shown::StatsBox { slot });
}

/// `PlayStereoCry` is about to play the cry of species `a`.
pub fn cry(cpu: &mut Cpu) {
    push(cpu, Shown::Cry { species: cpu.a });
}

/// Every event since the last call (an open message is closed first).
pub fn take_events(cpu: &Cpu) -> Vec<Shown> {
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
            self.events.push(Shown::Text(message));
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
        assert_eq!(cap.events, vec![Shown::Text("Wild PIDGEY\nappeared!".into())]);
    }
}
