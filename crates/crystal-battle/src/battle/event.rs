//! What the battle shows: its messages, HP bars, animations, sounds and the
//! rest of what the screen does, reported on every decision.

use super::action::Side;

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
    /// A battle text.
    Message(super::message::Message),
    /// An HP bar moved from `from` to `to` (of `max`).
    HpBar { target: HpBarTarget, from: u16, to: u16, max: u16 },
    /// A battle animation (`PlayBattleAnim`): a move's own (`anim` is the
    /// move's animation id, below `$100`) or one of the game's `ANIM_*`
    /// effects (send-outs, status, stat changes, balls, weather, …), played
    /// from `user`'s side. `param` picks a variant (the hit of a multi-hit
    /// move, Present's, a ball's shakes, …). The animation's own sounds and
    /// cries are part of it.
    Animation { anim: u16, user: Side, param: u8 },
    /// A sound effect (`SFX_*`) the battle plays outside animations and text.
    Sound(u16),
    /// The cry of a species (a mon fainting, a wild mon appearing, …).
    Cry(u8),
    /// The active mon on `side` faints: its picture slides down.
    Faint(Side),
    /// The picture on `side` slides off the screen.
    SlideOut(Side),
    /// The enemy trainer slides back onto the screen (after the battle).
    TrainerSlidesIn,
    /// The active mon (party slot `slot`)'s experience bar fills by `gained`.
    ExpBar { slot: u8, gained: u16 },
    /// The stats box of party mon `slot` (after a level-up).
    StatsBox { slot: u8 },
    /// The enemy mon's front picture animates, with its cry (it appears).
    Frontpic(u8),
    /// The ball thrown (the `Animation` before) shakes `wobbles` times, then
    /// clicks shut (`caught`) or breaks open. The original's animation draws
    /// these as it plays; this is their outcome.
    BallWobbles { wobbles: u8, caught: bool },
}

/// `wTilemap` (`00:c4a0`), 20 tiles per row. The battle text box (and the
/// party-menu text box items print into) covers rows 12..=17; HUDs, names and
/// menus elsewhere on screen are not battle text.
const W_TILEMAP: u16 = 0xc4a0;
const SCREEN_WIDTH: u16 = 20;
const TEXT_AREA_START: u16 = W_TILEMAP + 12 * SCREEN_WIDTH;
const TEXT_AREA_END: u16 = W_TILEMAP + 18 * SCREEN_WIDTH;

/// Whether `addr` is in the battle text box of `wTilemap`.
pub fn in_text_area(addr: u16) -> bool {
    (TEXT_AREA_START..TEXT_AREA_END).contains(&addr)
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
}
