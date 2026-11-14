//! Routines the port still needs. Each panics with the original routine's
//! name; delete an entry when its real port lands.

use super::battle::{Engine, Flow};

impl Engine {
    pub fn battle_menu(&mut self) -> Flow<bool> { self.unported("BattleMenu") }
    pub fn move_selection_screen(&mut self) -> Flow<bool> { self.unported("MoveSelectionScreen") }
    pub fn link_battle_send_receive_action(&mut self) -> Flow { self.unported("LinkBattleSendReceiveAction") }
    pub fn update_move_data(&mut self) { self.unported("UpdateMoveData") }
    pub fn do_player_turn(&mut self) -> Flow { self.unported("DoPlayerTurn") }
    pub fn do_enemy_turn(&mut self) -> Flow { self.unported("DoEnemyTurn") }
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
    pub fn get_user_item(&self) -> (u16, u8) { self.unported("GetUserItem") }
    pub fn get_user_item_full(&self) -> (u16, u8, u8) { self.unported("GetUserItem") }
    pub fn get_opponent_item_full(&self) -> (u16, u8, u8) { self.unported("GetOpponentItem") }
    pub fn spikes_damage(&mut self) { self.unported("SpikesDamage") }
    pub fn ai_switch(&mut self) -> Flow { self.unported("AI_Switch") }
    pub fn residual_damage(&mut self) -> bool { self.unported("ResidualDamage") }
    pub fn handle_player_mon_faint(&mut self) -> Flow { self.unported("HandlePlayerMonFaint") }
    pub fn handle_enemy_mon_faint(&mut self) -> Flow { self.unported("HandleEnemyMonFaint") }
}
