//! Stubs for the move effects (attract..mist) port (effects_a.rs). Each panics with the original's
//! name; delete a stub when its port lands in effects_a.rs.

use super::battle::{Engine, Flow};
use super::commands::Cmd;

impl Engine {
    pub fn battle_command_mirror_move(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_MirrorMove") }
    pub fn battle_command_conversion(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_Conversion") }
    pub fn battle_command_store_energy(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_StoreEnergy") }
    pub fn battle_command_unleash_energy(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_UnleashEnergy") }
    pub fn battle_command_mist(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_Mist") }
    pub fn battle_command_focus_energy(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_FocusEnergy") }
    pub fn battle_command_mimic(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_Mimic") }
    pub fn battle_command_metronome(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_Metronome") }
    pub fn battle_command_leech_seed(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_LeechSeed") }
    pub fn battle_command_disable(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_Disable") }
    pub fn battle_command_counter(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_Counter") }
    pub fn battle_command_encore(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_Encore") }
    pub fn battle_command_conversion2(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_Conversion2") }
    pub fn battle_command_lock_on(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_LockOn") }
    pub fn battle_command_destiny_bond(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_DestinyBond") }
    pub fn battle_command_heal_bell(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_HealBell") }
    pub fn battle_command_curse(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_Curse") }
    pub fn battle_command_foresight(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_Foresight") }
    pub fn battle_command_endure(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_Endure") }
    pub fn battle_command_fury_cutter(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_FuryCutter") }
    pub fn battle_command_attract(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_Attract") }
    pub fn battle_command_frustration_power(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_FrustrationPower") }
    pub fn battle_command_get_magnitude(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_GetMagnitude") }
    pub fn battle_command_baton_pass(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_BatonPass") }
    pub fn battle_command_hidden_power(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_HiddenPower") }
    pub fn battle_command_belly_drum(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_BellyDrum") }
    pub fn battle_command_mirror_coat(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_MirrorCoat") }
    pub fn battle_command_check_future_sight(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_CheckFutureSight") }
    pub fn battle_command_future_sight(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_FutureSight") }
    pub fn battle_command_beat_up(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_BeatUp") }
    pub fn battle_command_beat_up_fail_text(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_BeatUpFailText") }
}
