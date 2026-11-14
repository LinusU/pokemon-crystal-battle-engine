//! Stubs for routines owned by the switching / fainting port (switch.rs). Each panics with
//! the original's name; delete a stub when its real port lands in switch.rs.

use super::battle::{Engine, Flow};

impl Engine {
    /// `GetEighthMaxHP` (owned by residual.rs): returns `bc`.
    pub fn get_eighth_max_hp(&mut self) -> u16 { self.unported("GetEighthMaxHP") }
    /// `SubtractHPFromTarget` (owned by residual.rs), `bc` = amount.
    pub fn subtract_hp_from_target(&mut self, _bc: u16) { self.unported("SubtractHPFromTarget") }
    pub fn give_experience_points(&mut self) { if self.link_mode() != 0 { return; } self.unported("GiveExperiencePoints") }
}
