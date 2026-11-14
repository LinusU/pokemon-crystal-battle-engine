//! Stubs for routines owned by the damage pipeline port (damage.rs). Each panics with
//! the original's name; delete a stub when its real port lands in damage.rs.

use super::battle::{Engine, Flow};

impl Engine {
    /// `GetUserItem`: (address of the user's held item, its held effect `b`, its parameter `c`).
    pub fn get_user_item(&self) -> (u16, u8, u8) { self.unported("GetUserItem") }
    /// `GetOpponentItem`: like `get_user_item`, for the opponent.
    pub fn get_opponent_item(&self) -> (u16, u8, u8) { self.unported("GetOpponentItem") }
    pub fn hit_self_in_confusion(&mut self) { self.unported("HitSelfInConfusion") }
    pub fn do_player_damage(&mut self, _c: bool) { self.unported("DoPlayerDamage") }
    pub fn do_enemy_damage(&mut self, _c: bool) { self.unported("DoEnemyDamage") }
    pub fn appear_user_raise_sub(&mut self) { self.unported("AppearUserRaiseSub") }
    pub fn reset_fury_cutter_count(&mut self) { self.unported("ResetFuryCutterCount") }
    pub fn do_weather_modifiers(&mut self) { self.unported("DoWeatherModifiers") }
    pub fn do_badge_type_boosts(&mut self) { self.unported("DoBadgeTypeBoosts") }
}
