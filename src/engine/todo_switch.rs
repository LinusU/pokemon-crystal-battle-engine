//! Stubs for routines owned by the switching / fainting port (switch.rs). Each panics with
//! the original's name; delete a stub when its real port lands in switch.rs.

use super::battle::{Engine, Flow};

impl Engine {
    pub fn spikes_damage(&mut self) { self.unported("SpikesDamage") }
    pub fn handle_player_mon_faint(&mut self) -> Flow { self.unported("HandlePlayerMonFaint") }
    pub fn handle_enemy_mon_faint(&mut self) -> Flow { self.unported("HandleEnemyMonFaint") }
}
