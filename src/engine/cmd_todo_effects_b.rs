//! Stubs for the move effects (nightmare..transform) port (effects_b.rs). Each panics with the original's
//! name; delete a stub when its port lands in effects_b.rs.

use super::battle::{Engine, Flow};
use super::commands::Cmd;

impl Engine {
    pub fn battle_command_selfdestruct(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_Selfdestruct") }
    pub fn battle_command_pay_day(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_PayDay") }
    pub fn battle_command_transform(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_Transform") }
    pub fn battle_command_substitute(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_Substitute") }
    pub fn battle_command_splash(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_Splash") }
    pub fn battle_command_pain_split(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_PainSplit") }
    pub fn battle_command_snore(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_Snore") }
    pub fn battle_command_sketch(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_Sketch") }
    pub fn battle_command_sleep_talk(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_SleepTalk") }
    pub fn battle_command_spite(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_Spite") }
    pub fn battle_command_triple_kick(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_TripleKick") }
    pub fn battle_command_kick_counter(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_KickCounter") }
    pub fn battle_command_thief(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_Thief") }
    pub fn battle_command_nightmare(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_Nightmare") }
    pub fn battle_command_protect(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_Protect") }
    pub fn battle_command_spikes(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_Spikes") }
    pub fn battle_command_perish_song(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_PerishSong") }
    pub fn battle_command_start_sandstorm(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_StartSandstorm") }
    pub fn battle_command_check_rollout(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_CheckRollout") }
    pub fn battle_command_rollout_power(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_RolloutPower") }
    pub fn battle_command_happiness_power(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_HappinessPower") }
    pub fn battle_command_present(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_Present") }
    pub fn battle_command_safeguard(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_Safeguard") }
    pub fn battle_command_pursuit(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_Pursuit") }
    pub fn battle_command_clear_hazards(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_ClearHazards") }
    pub fn battle_command_start_rain(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_StartRain") }
    pub fn battle_command_start_sun(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_StartSun") }
    pub fn battle_command_psych_up(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_PsychUp") }
    pub fn battle_command_rage(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_Rage") }
    pub fn battle_command_thunder_accuracy(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_ThunderAccuracy") }
    pub fn battle_command_teleport(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_Teleport") }
}
