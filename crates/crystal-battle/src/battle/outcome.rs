//! How a battle ended.

use serde::Deserialize;

/// The battle's final result, decoded from `wBattleResult`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Outcome {
    Win,
    Loss,
    Draw,
    /// The wild mon was caught with a Poké Ball. The engine reports a capture
    /// as a plain WIN in `wBattleResult` (the catch path masks the result to
    /// WIN — see `engine/battle/core.asm` `.run`), so this variant is driven by
    /// the harness side-channel `cpu.caught_wild_mon`, not by WRAM.
    Caught,
}

impl Outcome {
    /// The lowercase tag written into / parsed from the recording TOML.
    pub fn as_str(self) -> &'static str {
        match self {
            Outcome::Win => "win",
            Outcome::Loss => "loss",
            Outcome::Draw => "draw",
            Outcome::Caught => "caught",
        }
    }
}

