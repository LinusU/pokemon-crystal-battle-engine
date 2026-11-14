//! Stubs for routines the between-turn effects port (residual.rs) calls that
//! belong to other areas. Each panics with the original's name; delete a stub
//! when its real port lands.

use super::battle::Engine;

impl Engine {
    /// `ConsumeHeldItem` (`engine/battle/consume_held_item.asm`).
    pub fn consume_held_item(&mut self) { self.unported("ConsumeHeldItem") }
    /// `CalcPlayerStats` (`engine/battle/effect_commands.asm`).
    pub fn calc_player_stats(&mut self) { self.unported("CalcPlayerStats") }
    /// `CalcEnemyStats` (`engine/battle/effect_commands.asm`).
    pub fn calc_enemy_stats(&mut self) { self.unported("CalcEnemyStats") }
}
