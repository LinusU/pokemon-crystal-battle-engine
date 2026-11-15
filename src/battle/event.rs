//! What the battle shows: battle text (exactly as printed) and HP-bar
//! changes, reported on every decision.

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
