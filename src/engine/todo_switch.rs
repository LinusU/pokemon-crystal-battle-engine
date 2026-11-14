//! Stubs for routines owned by the switching / fainting port (switch.rs). Each panics with
//! the original's name; delete a stub when its real port lands in switch.rs.

use super::battle::{Engine, Flow};

impl Engine {
    pub fn give_experience_points(&mut self) { if self.link_mode() != 0 { return; } self.unported("GiveExperiencePoints") }
}
