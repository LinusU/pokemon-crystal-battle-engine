//! Stubs for routines the trainer AI (ai*.rs) calls that belong to other
//! areas (effect commands). Each panics with the original's name; delete a
//! stub when its real port lands.

use super::battle::Engine;

impl Engine {
    /// `RaiseStat` (`engine/battle/effect_commands.asm`): raise stat `b` of
    /// the current turn's user (used by the AI's X items).
    pub fn raise_stat(&mut self, _b: u8) {
        self.unported("RaiseStat")
    }

    /// `CheckOppositeGender` (`engine/battle/move_effects/attract.asm`):
    /// carry (`true`) = the two active mons can't attract each other.
    pub fn check_opposite_gender(&mut self) -> bool {
        self.unported("CheckOppositeGender")
    }
}
