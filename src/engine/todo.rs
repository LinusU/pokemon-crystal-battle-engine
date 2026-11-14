//! Routines the port still needs. Each panics with the original routine's
//! name; delete an entry when its real port lands.

use super::battle::{Engine, Flow};

impl Engine {
    pub fn battle_menu(&mut self) -> Flow<bool> { self.unported("BattleMenu") }
    pub fn move_selection_screen(&mut self) -> Flow<bool> { self.unported("MoveSelectionScreen") }
    pub fn link_battle_send_receive_action(&mut self) -> Flow { self.unported("LinkBattleSendReceiveAction") }
    pub fn ai_switch(&mut self) -> Flow { self.unported("AI_Switch") }
}
