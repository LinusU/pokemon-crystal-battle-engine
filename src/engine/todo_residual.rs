//! Stubs for routines owned by the between-turn effects port (residual.rs). Each panics with
//! the original's name; delete a stub when its real port lands in residual.rs.

use super::battle::{Engine, Flow};

impl Engine {
    pub fn handle_future_sight(&mut self) { self.unported("HandleFutureSight") }
    pub fn handle_weather(&mut self) { self.unported("HandleWeather") }
    pub fn handle_wrap(&mut self) { self.unported("HandleWrap") }
    pub fn handle_perish_song(&mut self) { self.unported("HandlePerishSong") }
    pub fn handle_leftovers(&mut self) { self.unported("HandleLeftovers") }
    pub fn handle_mysteryberry(&mut self) { self.unported("HandleMysteryberry") }
    pub fn handle_defrost(&mut self) { self.unported("HandleDefrost") }
    pub fn handle_safeguard(&mut self) { self.unported("HandleSafeguard") }
    pub fn handle_screens(&mut self) { self.unported("HandleScreens") }
    pub fn handle_stat_boosting_held_items(&mut self) { self.unported("HandleStatBoostingHeldItems") }
    pub fn handle_healing_items(&mut self) { self.unported("HandleHealingItems") }
    pub fn residual_damage(&mut self) -> bool { self.unported("ResidualDamage") }
}
