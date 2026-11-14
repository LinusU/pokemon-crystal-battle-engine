//! Stubs for the misc effect commands port (misc.rs). Each panics with the original's
//! name; delete a stub when its port lands in misc.rs.

use super::battle::{Engine, Flow};
use super::commands::Cmd;

impl Engine {
    pub fn battle_command_reset_stats(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_ResetStats") }
    pub fn battle_command_force_switch(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_ForceSwitch") }
    pub fn battle_command_flinch_target(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_FlinchTarget") }
    pub fn battle_command_ohko(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_OHKO") }
    pub fn battle_command_recoil(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_Recoil") }
    pub fn battle_command_confuse(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_Confuse") }
    pub fn battle_command_confuse_target(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_ConfuseTarget") }
    pub fn battle_command_heal(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_Heal") }
    pub fn battle_command_screen(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_Screen") }
    pub fn battle_command_paralyze(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_Paralyze") }
    pub fn battle_command_recharge_next_turn(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_RechargeNextTurn") }
    pub fn battle_command_charge(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_Charge") }
    pub fn battle_command_check_charge(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_CheckCharge") }
    pub fn battle_command_trap_target(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_TrapTarget") }
    pub fn battle_command_unused3_c(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_Unused3C") }
    pub fn battle_command_rampage(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_Rampage") }
    pub fn battle_command_check_rampage(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_CheckRampage") }
    pub fn battle_command_arena_trap(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_ArenaTrap") }
    pub fn battle_command_defrost(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_Defrost") }
    pub fn battle_command_unused5_d(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_Unused5D") }
    pub fn battle_command_check_safeguard(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_CheckSafeguard") }
    pub fn battle_command_heal_morn(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_HealMorn") }
    pub fn battle_command_heal_day(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_HealDay") }
    pub fn battle_command_heal_nite(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_HealNite") }
    pub fn battle_command_fake_out(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_FakeOut") }
    pub fn battle_command_double_flying_damage(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_DoubleFlyingDamage") }
    pub fn battle_command_double_underground_damage(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_DoubleUndergroundDamage") }
    pub fn battle_command_double_minimize_damage(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_DoubleMinimizeDamage") }
    pub fn battle_command_skip_sun_charge(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_SkipSunCharge") }
}
