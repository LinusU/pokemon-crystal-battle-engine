//! Stubs for the status / stat-change commands port (status.rs). Each panics with the original's
//! name; delete a stub when its port lands in status.rs.

use super::battle::{Engine, Flow};
use super::commands::Cmd;

impl Engine {
    pub fn battle_command_poison_target(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_PoisonTarget") }
    pub fn battle_command_sleep_target(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_SleepTarget") }
    pub fn battle_command_drain_target(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_DrainTarget") }
    pub fn battle_command_eat_dream(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_EatDream") }
    pub fn battle_command_burn_target(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_BurnTarget") }
    pub fn battle_command_freeze_target(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_FreezeTarget") }
    pub fn battle_command_paralyze_target(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_ParalyzeTarget") }
    pub fn battle_command_stat_up(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_StatUp") }
    pub fn battle_command_stat_down(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_StatDown") }
    pub fn battle_command_poison(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_Poison") }
    pub fn battle_command_defrost_opponent(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_DefrostOpponent") }
    pub fn battle_command_attack_up(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_AttackUp") }
    pub fn battle_command_defense_up(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_DefenseUp") }
    pub fn battle_command_speed_up(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_SpeedUp") }
    pub fn battle_command_special_attack_up(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_SpecialAttackUp") }
    pub fn battle_command_special_defense_up(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_SpecialDefenseUp") }
    pub fn battle_command_accuracy_up(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_AccuracyUp") }
    pub fn battle_command_evasion_up(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_EvasionUp") }
    pub fn battle_command_attack_up2(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_AttackUp2") }
    pub fn battle_command_defense_up2(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_DefenseUp2") }
    pub fn battle_command_speed_up2(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_SpeedUp2") }
    pub fn battle_command_special_attack_up2(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_SpecialAttackUp2") }
    pub fn battle_command_special_defense_up2(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_SpecialDefenseUp2") }
    pub fn battle_command_accuracy_up2(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_AccuracyUp2") }
    pub fn battle_command_evasion_up2(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_EvasionUp2") }
    pub fn battle_command_attack_down(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_AttackDown") }
    pub fn battle_command_defense_down(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_DefenseDown") }
    pub fn battle_command_speed_down(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_SpeedDown") }
    pub fn battle_command_special_attack_down(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_SpecialAttackDown") }
    pub fn battle_command_special_defense_down(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_SpecialDefenseDown") }
    pub fn battle_command_accuracy_down(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_AccuracyDown") }
    pub fn battle_command_evasion_down(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_EvasionDown") }
    pub fn battle_command_attack_down2(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_AttackDown2") }
    pub fn battle_command_defense_down2(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_DefenseDown2") }
    pub fn battle_command_speed_down2(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_SpeedDown2") }
    pub fn battle_command_special_attack_down2(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_SpecialAttackDown2") }
    pub fn battle_command_special_defense_down2(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_SpecialDefenseDown2") }
    pub fn battle_command_accuracy_down2(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_AccuracyDown2") }
    pub fn battle_command_evasion_down2(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_EvasionDown2") }
    pub fn battle_command_stat_up_message(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_StatUpMessage") }
    pub fn battle_command_stat_down_message(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_StatDownMessage") }
    pub fn battle_command_stat_up_fail_text(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_StatUpFailText") }
    pub fn battle_command_stat_down_fail_text(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_StatDownFailText") }
    pub fn battle_command_all_stats_up(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_AllStatsUp") }
    pub fn battle_command_tri_status_chance(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_TriStatusChance") }
    pub fn battle_command_curl(&mut self) -> Flow<Cmd> { self.unported("BattleCommand_Curl") }
}
