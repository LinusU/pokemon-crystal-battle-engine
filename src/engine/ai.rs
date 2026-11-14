//! Port of the trainer AI (`engine/battle/ai/`). See docs/engine.md.

#![allow(unused_imports)]

use super::battle::{Engine, Flow, PARTY_LEN, PP_MASK, percent};
use super::commands::Cmd;
use super::rom;
use super::rsym as r;
use super::sym::*;
use crate::defs::*;

impl Engine {
    /// `AIChooseMove`.
    pub fn ai_choose_move(&mut self) {
        if self.battle_mode() == WILD_BATTLE {
            return;
        }
        if self.link_mode() != 0 {
            return;
        }
        self.unported("AIChooseMove")
    }

    /// `AI_SwitchOrTryItem` (carry = the enemy switched or used an item).
    pub fn ai_switch_or_try_item(&mut self) -> Flow<bool> {
        if self.battle_mode() == WILD_BATTLE || self.link_mode() != 0 {
            return Ok(false);
        }
        self.unported("AI_SwitchOrTryItem")
    }
}
