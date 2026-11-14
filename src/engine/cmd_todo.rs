//! Effect commands not ported yet. Each panics with the original's name;
//! delete a stub when its port lands.

use super::battle::{Engine, Flow};
use super::commands::Cmd;

impl Engine {/// `BattleCommand_CheckTurn` as a script command (it is `CheckTurn`).
    pub fn battle_command_check_turn(&mut self) -> Flow<Cmd> {
        self.check_turn()?;
        Ok(Cmd::Next)
    }


}
