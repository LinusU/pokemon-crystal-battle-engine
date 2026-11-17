//! Port of the status and stat-change commands of
//! `engine/battle/effect_commands.asm` (DefrostOpponent .. Curl, plus
//! `CalcPlayerStats` / `CalcEnemyStats` and `ConsumeHeldItem`). See docs/engine.md.

#![allow(unused_imports)]

use super::battle::{Engine, Flow, PARTY_LEN, PP_MASK, percent};
use super::commands::Cmd;
use super::rsym as r;
use super::sym::*;
use crate::defs::*;

impl Engine {
    // --- small helpers (animations, text, checks) --------------------------------




    /// `RefreshBattleHuds`: HUD redraw only.
    fn refresh_battle_huds(&mut self) {}











    /// The `.fail` tail shared by several commands: animate the failure, then print `hl`.
    fn fail_with_text(&mut self, hl: usize) -> Flow<Cmd> {
        self.animate_failed_move()?;
        self.text(hl);
        Ok(Cmd::Next)
    }

    /// `ConsumeHeldItem`.
    pub fn consume_held_item(&mut self) {
        let (party, de, mon) = if self.is_player_turn() {
            (W_OT_PARTY_MON1_ITEM, W_ENEMY_MON_ITEM, self.mem.rb(W_CUR_OT_MON))
        } else {
            (W_PARTY_MON1_ITEM, W_BATTLE_MON_ITEM, self.mem.rb(W_CUR_BATTLE_MON))
        };
        let (b, _) = self.get_item_held_effect(self.mem.rb(de));
        // The list ends with -1, which a held effect of $ff matches.
        if b != 0xff && !self.data.tables.consumable_effects.contains(&b) {
            return;
        }
        self.mem.wb(de, 0);
        let hl = self.get_party_location(party, mon);
        if !self.is_player_turn() || self.battle_mode().wrapping_sub(1) != 0 {
            self.mem.wb(hl, NO_ITEM);
        }
    }

    // --- DefrostOpponent, sleep, poison -----------------------------------------------

    /// `BattleCommand_DefrostOpponent`. (No move uses this effect; `b` is
    /// whatever the script reader left, taken as ATTACK.)
    pub fn battle_command_defrost_opponent(&mut self) -> Flow<Cmd> {
        self.animate_current_move()?;
        let status = self.battle_var_addr(BATTLE_VARS_STATUS_OPP);
        self.defrost(status);
        let effect = self.battle_var_addr(BATTLE_VARS_MOVE_EFFECT);
        let saved = self.mem.rb(effect);
        self.mem.wb(effect, EFFECT_ATTACK_UP);
        let res = self.stat_up(ATTACK);
        self.mem.wb(effect, saved);
        res
    }

    /// `BattleCommand_SleepTarget`.
    pub fn battle_command_sleep_target(&mut self) -> Flow<Cmd> {
        let (item, b, _) = self.get_opponent_item();
        if b == HELD_PREVENT_SLEEP {
            let it = self.mem.rb(item);
            self.mem.wb(W_NAMED_OBJECT_INDEX, it);
            self.get_item_name();
            return self.fail_with_text(r::PROTECTED_BY_TEXT);
        }
        let de = self.battle_var_addr(BATTLE_VARS_STATUS_OPP);
        if self.mem.rb(de) & SLP_MASK != 0 {
            return self.fail_with_text(r::ALREADY_ASLEEP_TEXT);
        }
        if self.mem.rb(W_ATTACK_MISSED) != 0 {
            self.print_didnt_affect2()?;
            return Ok(Cmd::Next);
        }
        if self.sleep_check_ai_random_fail() {
            return self.fail_with_text(r::DIDNT_AFFECT1_TEXT);
        }
        if self.mem.rb(de) != 0 {
            return self.fail_with_text(r::DIDNT_AFFECT1_TEXT);
        }
        if self.check_substitute_opp() {
            return self.fail_with_text(r::DIDNT_AFFECT1_TEXT);
        }
        self.animate_current_move()?;
        let b = if self.mem.rb(W_IN_BATTLE_TOWER_BATTLE) == 0 { SLP_MASK } else { 0b011 };
        let a = loop {
            let a = self.battle_random("BattleCommand_SleepTarget") & b;
            if a == 0 || a == SLP_MASK {
                continue;
            }
            break a;
        };
        self.mem.wb(de, a + 1);
        self.update_opponent_in_party();
        self.refresh_battle_huds();
        self.text(r::FELL_ASLEEP_TEXT);
        if !self.use_held_status_healing_item() {
            self.opponent_cant_move();
        }
        Ok(Cmd::Next)
    }

    /// `BattleCommand_SleepTarget.CheckAIRandomFail`: carry (true) = fail.
    fn sleep_check_ai_random_fail(&mut self) -> bool {
        if self.is_player_turn()
            || self.link_mode() != 0
            || self.mem.rb(W_IN_BATTLE_TOWER_BATTLE) != 0
            || self.mem.bit(W_PLAYER_SUB_STATUS5, SUBSTATUS_LOCK_ON)
        {
            return false;
        }
        self.battle_random("BattleCommand_SleepTarget") < percent(25) + 1
    }

    /// `BattleCommand_PoisonTarget`.
    pub fn battle_command_poison_target(&mut self) -> Flow<Cmd> {
        if self.check_substitute_opp() {
            return Ok(Cmd::Next);
        }
        if self.battle_var(BATTLE_VARS_STATUS_OPP) != 0 {
            return Ok(Cmd::Next);
        }
        if self.mem.rb(W_TYPE_MODIFIER) & EFFECTIVENESS_MASK == 0 {
            return Ok(Cmd::Next);
        }
        if self.check_if_target_is_poison_type() {
            return Ok(Cmd::Next);
        }
        let (_, b, _) = self.get_opponent_item();
        if b == HELD_PREVENT_POISON {
            return Ok(Cmd::Next);
        }
        if self.mem.rb(W_EFFECT_FAILED) != 0 {
            return Ok(Cmd::Next);
        }
        if self.safe_check_safeguard() {
            return Ok(Cmd::Next);
        }
        self.poison_opponent();
        self.play_opponent_battle_anim(ANIM_PSN);
        self.refresh_battle_huds();
        self.text(r::WAS_POISONED_TEXT);
        self.use_held_status_healing_item();
        Ok(Cmd::Next)
    }

    /// `BattleCommand_Poison`.
    pub fn battle_command_poison(&mut self) -> Flow<Cmd> {
        if self.mem.rb(W_TYPE_MODIFIER) & EFFECTIVENESS_MASK == 0 {
            return self.fail_with_text(r::DOESNT_AFFECT_TEXT);
        }
        if self.check_if_target_is_poison_type() {
            return self.fail_with_text(r::DOESNT_AFFECT_TEXT);
        }
        if self.battle_var(BATTLE_VARS_STATUS_OPP) & 1 << PSN != 0 {
            return self.fail_with_text(r::ALREADY_POISONED_TEXT);
        }
        let (item, b, _) = self.get_opponent_item();
        if b == HELD_PREVENT_POISON {
            let it = self.mem.rb(item);
            self.mem.wb(W_NAMED_OBJECT_INDEX, it);
            self.get_item_name();
            return self.fail_with_text(r::PROTECTED_BY_TEXT);
        }
        // .do_poison
        if self.battle_var(BATTLE_VARS_STATUS_OPP) != 0 {
            return self.fail_with_text(r::DIDNT_AFFECT1_TEXT);
        }
        if !self.is_player_turn()
            && self.link_mode() == 0
            && self.mem.rb(W_IN_BATTLE_TOWER_BATTLE) == 0
            && !self.mem.bit(W_PLAYER_SUB_STATUS5, SUBSTATUS_LOCK_ON)
        {
            // 25% chance AI fails
            if self.battle_random("BattleCommand_Poison") < percent(25) + 1 {
                return self.fail_with_text(r::DIDNT_AFFECT1_TEXT);
            }
        }
        // .dont_sample_failure
        if self.check_substitute_opp() || self.mem.rb(W_ATTACK_MISSED) != 0 {
            return self.fail_with_text(r::DIDNT_AFFECT1_TEXT);
        }
        // .check_toxic
        let hl = self.battle_var_addr(BATTLE_VARS_SUBSTATUS5_OPP);
        let de = if self.is_player_turn() { W_ENEMY_TOXIC_COUNT } else { W_PLAYER_TOXIC_COUNT };
        if self.battle_var(BATTLE_VARS_MOVE_EFFECT) == EFFECT_TOXIC {
            self.mem.set_bit(hl, SUBSTATUS_TOXIC);
            self.mem.wb(de, 0);
            self.poison_apply()?;
            self.text(r::BADLY_POISONED_TEXT);
        } else {
            self.poison_apply()?;
            self.text(r::WAS_POISONED_TEXT);
        }
        self.use_held_status_healing_item();
        Ok(Cmd::Next)
    }

    /// `BattleCommand_Poison.apply_poison`.
    fn poison_apply(&mut self) -> Flow {
        self.animate_current_move()?;
        self.poison_opponent();
        self.refresh_battle_huds();
        Ok(())
    }

    /// `CheckIfTargetIsPoisonType`: z (true) if the target is Poison type.
    pub fn check_if_target_is_poison_type(&self) -> bool {
        let de = if self.is_player_turn() { W_ENEMY_MON_TYPE1 } else { W_BATTLE_MON_TYPE1 };
        self.mem.rb(de) == POISON || self.mem.rb(de + 1) == POISON
    }

    /// `PoisonOpponent`.
    pub fn poison_opponent(&mut self) {
        self.set_battle_var_bit(BATTLE_VARS_STATUS_OPP, PSN);
        self.update_opponent_in_party();
    }

    // --- draining -------------------------------------------------------------------

    /// `BattleCommand_DrainTarget`.
    pub fn battle_command_drain_target(&mut self) -> Flow<Cmd> {
        self.sap_health();
        self.text(r::SUCKED_HEALTH_TEXT);
        Ok(Cmd::Next)
    }

    /// `BattleCommand_EatDream`.
    pub fn battle_command_eat_dream(&mut self) -> Flow<Cmd> {
        self.sap_health();
        self.text(r::DREAM_EATEN_TEXT);
        Ok(Cmd::Next)
    }

    /// `SapHealth`.
    pub fn sap_health(&mut self) {
        // Divide damage by 2, store it in hDividend.
        let dmg = self.mem.rw(W_CUR_DAMAGE) >> 1;
        self.mem.wb(H_DIVIDEND, (dmg >> 8) as u8);
        self.mem.wb(H_DIVIDEND + 1, if dmg == 0 { 1 } else { dmg as u8 });
        let half = self.mem.rw(H_DIVIDEND);

        let (hp, max) = if self.is_player_turn() {
            (W_BATTLE_MON_HP, W_BATTLE_MON_MAX_HP)
        } else {
            (W_ENEMY_MON_HP, W_ENEMY_MON_MAX_HP)
        };
        let cur = self.mem.rw(hp);
        let max_hp = self.mem.rw(max);
        // Little-endian buffers: current HP, max HP.
        self.mem.ww_le(W_HP_BUFFER2, cur);
        self.mem.ww_le(W_HP_BUFFER1, max_hp);

        let (sum, carry) = cur.overflowing_add(half);
        self.mem.ww(hp, sum);
        self.mem.ww_le(W_HP_BUFFER3, sum);
        if carry || max_hp < sum {
            // .max_hp
            self.mem.ww(hp, max_hp);
            self.mem.ww_le(W_HP_BUFFER3, max_hp);
        }
        // .finish
        let which = u8::from(self.is_player_turn());
        self.mem.wb(W_WHICH_HP_BAR, which);
        self.animate_hp_bar();
        self.refresh_battle_huds();
        self.update_battle_mon_in_party();
    }

    // --- burn, defrost, freeze, paralysis ---------------------------------------------

    /// `BattleCommand_BurnTarget`.
    pub fn battle_command_burn_target(&mut self) -> Flow<Cmd> {
        self.mem.wb(W_BATTLE_AFTER_ANIM, 0);
        if self.check_substitute_opp() {
            return Ok(Cmd::Next);
        }
        let status = self.battle_var_addr(BATTLE_VARS_STATUS_OPP);
        if self.mem.rb(status) != 0 {
            self.defrost(status);
            return Ok(Cmd::Next);
        }
        if self.mem.rb(W_TYPE_MODIFIER) & EFFECTIVENESS_MASK == 0 {
            return Ok(Cmd::Next);
        }
        if self.check_move_type_matches_target() {
            return Ok(Cmd::Next); // Don't burn a Fire-type
        }
        let (_, b, _) = self.get_opponent_item();
        if b == HELD_PREVENT_BURN {
            return Ok(Cmd::Next);
        }
        if self.mem.rb(W_EFFECT_FAILED) != 0 || self.safe_check_safeguard() {
            return Ok(Cmd::Next);
        }
        self.set_battle_var_bit(BATTLE_VARS_STATUS_OPP, BRN);
        self.update_opponent_in_party();
        self.apply_brn_effect_on_attack();
        self.play_opponent_battle_anim(ANIM_BRN);
        self.refresh_battle_huds();
        self.text(r::WAS_BURNED_TEXT);
        self.use_held_status_healing_item();
        Ok(Cmd::Next)
    }

    /// `Defrost`: `hl` = the opponent's status byte.
    pub fn defrost(&mut self, hl: u16) {
        if self.mem.rb(hl) & 1 << FRZ == 0 {
            return;
        }
        self.mem.wb(hl, 0);
        let (base, mon) = if self.is_player_turn() {
            (W_OT_PARTY_MON1_STATUS, self.mem.rb(W_CUR_OT_MON))
        } else {
            (W_PARTY_MON1_STATUS, self.mem.rb(W_CUR_BATTLE_MON))
        };
        let loc = self.get_party_location(base, mon);
        self.mem.wb(loc, 0);
        self.update_opponent_in_party();
        self.text(r::DEFROSTED_OPPONENT_TEXT);
    }

    /// `BattleCommand_FreezeTarget`.
    pub fn battle_command_freeze_target(&mut self) -> Flow<Cmd> {
        self.mem.wb(W_BATTLE_AFTER_ANIM, 0);
        if self.check_substitute_opp() {
            return Ok(Cmd::Next);
        }
        if self.battle_var(BATTLE_VARS_STATUS_OPP) != 0 {
            return Ok(Cmd::Next);
        }
        if self.mem.rb(W_TYPE_MODIFIER) & EFFECTIVENESS_MASK == 0 {
            return Ok(Cmd::Next);
        }
        if self.mem.rb(W_BATTLE_WEATHER) == WEATHER_SUN {
            return Ok(Cmd::Next);
        }
        if self.check_move_type_matches_target() {
            return Ok(Cmd::Next); // Don't freeze an Ice-type
        }
        let (_, b, _) = self.get_opponent_item();
        if b == HELD_PREVENT_FREEZE {
            return Ok(Cmd::Next);
        }
        if self.mem.rb(W_EFFECT_FAILED) != 0 || self.safe_check_safeguard() {
            return Ok(Cmd::Next);
        }
        self.set_battle_var_bit(BATTLE_VARS_STATUS_OPP, FRZ);
        self.update_opponent_in_party();
        self.play_opponent_battle_anim(ANIM_FRZ);
        self.refresh_battle_huds();
        self.text(r::WAS_FROZEN_TEXT);
        if self.use_held_status_healing_item() {
            return Ok(Cmd::Next);
        }
        self.opponent_cant_move();
        self.end_recharge_opp();
        let hl = if self.is_player_turn() { W_ENEMY_JUST_GOT_FROZEN } else { W_PLAYER_JUST_GOT_FROZEN };
        self.mem.wb(hl, 1);
        Ok(Cmd::Next)
    }

    /// `BattleCommand_ParalyzeTarget`.
    pub fn battle_command_paralyze_target(&mut self) -> Flow<Cmd> {
        self.mem.wb(W_BATTLE_AFTER_ANIM, 0);
        if self.check_substitute_opp() {
            return Ok(Cmd::Next);
        }
        if self.battle_var(BATTLE_VARS_STATUS_OPP) != 0 {
            return Ok(Cmd::Next);
        }
        if self.mem.rb(W_TYPE_MODIFIER) & EFFECTIVENESS_MASK == 0 {
            return Ok(Cmd::Next);
        }
        let (_, b, _) = self.get_opponent_item();
        if b == HELD_PREVENT_PARALYZE {
            return Ok(Cmd::Next);
        }
        if self.mem.rb(W_EFFECT_FAILED) != 0 || self.safe_check_safeguard() {
            return Ok(Cmd::Next);
        }
        self.set_battle_var_bit(BATTLE_VARS_STATUS_OPP, PAR);
        self.update_opponent_in_party();
        self.apply_prz_effect_on_speed();
        self.play_opponent_battle_anim(ANIM_PAR);
        self.refresh_battle_huds();
        self.print_paralyze();
        self.use_held_status_healing_item();
        Ok(Cmd::Next)
    }

    // --- stat up ------------------------------------------------------------------------

    pub fn battle_command_attack_up(&mut self) -> Flow<Cmd> { self.stat_up(ATTACK) }
    pub fn battle_command_defense_up(&mut self) -> Flow<Cmd> { self.stat_up(DEFENSE) }
    pub fn battle_command_speed_up(&mut self) -> Flow<Cmd> { self.stat_up(SPEED) }
    pub fn battle_command_special_attack_up(&mut self) -> Flow<Cmd> { self.stat_up(SP_ATTACK) }
    pub fn battle_command_special_defense_up(&mut self) -> Flow<Cmd> { self.stat_up(SP_DEFENSE) }
    pub fn battle_command_accuracy_up(&mut self) -> Flow<Cmd> { self.stat_up(ACCURACY) }
    pub fn battle_command_evasion_up(&mut self) -> Flow<Cmd> { self.stat_up(EVASION) }
    pub fn battle_command_attack_up2(&mut self) -> Flow<Cmd> { self.stat_up(0x10 | ATTACK) }
    pub fn battle_command_defense_up2(&mut self) -> Flow<Cmd> { self.stat_up(0x10 | DEFENSE) }
    pub fn battle_command_speed_up2(&mut self) -> Flow<Cmd> { self.stat_up(0x10 | SPEED) }
    pub fn battle_command_special_attack_up2(&mut self) -> Flow<Cmd> { self.stat_up(0x10 | SP_ATTACK) }
    pub fn battle_command_special_defense_up2(&mut self) -> Flow<Cmd> { self.stat_up(0x10 | SP_DEFENSE) }
    pub fn battle_command_accuracy_up2(&mut self) -> Flow<Cmd> { self.stat_up(0x10 | ACCURACY) }
    pub fn battle_command_evasion_up2(&mut self) -> Flow<Cmd> { self.stat_up(0x10 | EVASION) }

    /// `BattleCommand_StatUp` invoked bare (no script uses it; `b` taken as ATTACK).
    pub fn battle_command_stat_up(&mut self) -> Flow<Cmd> {
        self.stat_up(ATTACK)
    }

    /// `BattleCommand_StatUp` with `b` = stat (| `$10` for two stages).
    pub fn stat_up(&mut self, b: u8) -> Flow<Cmd> {
        self.raise_stat(b);
        if self.mem.rb(W_FAILED_MESSAGE) != 0 {
            return Ok(Cmd::Next);
        }
        self.minimize_drop_sub()
    }

    /// `RaiseStat` with `b` = stat (| `$10`).
    pub fn raise_stat(&mut self, b: u8) {
        self.mem.wb(W_LOWERED_STAT, b);
        let levels = if self.is_player_turn() { W_PLAYER_STAT_LEVELS } else { W_ENEMY_STAT_LEVELS };
        if self.mem.rb(W_ATTACK_MISSED) != 0 || self.mem.rb(W_EFFECT_FAILED) != 0 {
            // .stat_raise_failed
            self.mem.wb(W_FAILED_MESSAGE, 1);
            return;
        }
        let c = self.mem.rb(W_LOWERED_STAT) & 0xf;
        let hl = levels + u16::from(c);
        let mut b = self.mem.rb(hl).wrapping_add(1);
        if MAX_STAT_LEVEL < b {
            self.cant_raise_stat();
            return;
        }
        if self.mem.rb(W_LOWERED_STAT) & 0xf0 != 0 {
            b = b.wrapping_add(1);
            if MAX_STAT_LEVEL < b {
                b = MAX_STAT_LEVEL;
            }
        }
        self.mem.wb(hl, b);
        if c < ACCURACY {
            let stats = if self.is_player_turn() { W_BATTLE_MON_STATS + 1 } else { W_ENEMY_MON_STATS + 1 };
            let at = stats + 2 * u16::from(c);
            // The stat's low byte is at `at`, its high byte at `at - 1`.
            if self.mem.rb(at) == MAX_STAT_VALUE as u8 && self.mem.rb(at - 1) == (MAX_STAT_VALUE >> 8) as u8 {
                // .stats_already_max
                let v = self.mem.rb(hl).wrapping_sub(1);
                self.mem.wb(hl, v);
                self.cant_raise_stat();
                return;
            }
            if self.is_player_turn() {
                self.calc_player_stats();
            } else {
                self.calc_enemy_stats();
            }
        }
        // .done_calcing_stats
        self.mem.wb(W_FAILED_MESSAGE, 0);
    }

    /// `RaiseStat.cant_raise_stat`.
    fn cant_raise_stat(&mut self) {
        self.mem.wb(W_FAILED_MESSAGE, 2);
        self.mem.wb(W_ATTACK_MISSED, 1);
    }

    /// `MinimizeDropSub`.
    pub fn minimize_drop_sub(&mut self) -> Flow<Cmd> {
        let bc = if self.is_player_turn() { W_PLAYER_MINIMIZED } else { W_ENEMY_MINIMIZED };
        if self.battle_var(BATTLE_VARS_MOVE_ANIM) != MINIMIZE {
            return Ok(Cmd::Next);
        }
        self.mem.wb(bc, 1);
        if !self.check_battle_scene() {
            return Ok(Cmd::Next);
        }
        self.battle_command_lower_sub_no_anim()?;
        self.battle_command_move_delay()
    }

    // --- stat down ------------------------------------------------------------------------

    pub fn battle_command_attack_down(&mut self) -> Flow<Cmd> { self.stat_down(ATTACK) }
    pub fn battle_command_defense_down(&mut self) -> Flow<Cmd> { self.stat_down(DEFENSE) }
    pub fn battle_command_speed_down(&mut self) -> Flow<Cmd> { self.stat_down(SPEED) }
    pub fn battle_command_special_attack_down(&mut self) -> Flow<Cmd> { self.stat_down(SP_ATTACK) }
    pub fn battle_command_special_defense_down(&mut self) -> Flow<Cmd> { self.stat_down(SP_DEFENSE) }
    pub fn battle_command_accuracy_down(&mut self) -> Flow<Cmd> { self.stat_down(ACCURACY) }
    pub fn battle_command_evasion_down(&mut self) -> Flow<Cmd> { self.stat_down(EVASION) }
    pub fn battle_command_attack_down2(&mut self) -> Flow<Cmd> { self.stat_down(0x10 | ATTACK) }
    pub fn battle_command_defense_down2(&mut self) -> Flow<Cmd> { self.stat_down(0x10 | DEFENSE) }
    pub fn battle_command_speed_down2(&mut self) -> Flow<Cmd> { self.stat_down(0x10 | SPEED) }
    pub fn battle_command_special_attack_down2(&mut self) -> Flow<Cmd> { self.stat_down(0x10 | SP_ATTACK) }
    pub fn battle_command_special_defense_down2(&mut self) -> Flow<Cmd> { self.stat_down(0x10 | SP_DEFENSE) }
    pub fn battle_command_accuracy_down2(&mut self) -> Flow<Cmd> { self.stat_down(0x10 | ACCURACY) }
    pub fn battle_command_evasion_down2(&mut self) -> Flow<Cmd> { self.stat_down(0x10 | EVASION) }

    /// `BattleCommand_StatDown` invoked bare (no script uses it; `a` taken as ATTACK).
    pub fn battle_command_stat_down(&mut self) -> Flow<Cmd> {
        self.stat_down(ATTACK)
    }

    /// `BattleCommand_StatDown` with `a` = stat (| `$10` for two stages).
    pub fn stat_down(&mut self, a: u8) -> Flow<Cmd> {
        self.mem.wb(W_LOWERED_STAT, a);
        if self.check_mist() {
            // .Mist
            self.mem.wb(W_FAILED_MESSAGE, 2);
            self.mem.wb(W_ATTACK_MISSED, 1);
            return Ok(Cmd::Next);
        }
        let levels = if self.is_player_turn() { W_ENEMY_STAT_LEVELS } else { W_PLAYER_STAT_LEVELS };
        // .GetStatLevel
        let c = self.mem.rb(W_LOWERED_STAT) & 0xf;
        let hl = levels + u16::from(c);
        let mut b = self.mem.rb(hl).wrapping_sub(1);
        if b == 0 {
            self.stat_down_cant_lower();
            return Ok(Cmd::Next);
        }
        if self.mem.rb(W_LOWERED_STAT) & 0xf0 != 0 {
            b = b.wrapping_sub(1);
            if b == 0 {
                b = 1;
            }
        }
        // .ComputerMiss: computer opponents have a 25% chance of failing.
        let mut failed = false;
        if !self.is_player_turn()
            && self.link_mode() == 0
            && self.mem.rb(W_IN_BATTLE_TOWER_BATTLE) == 0
            && !self.mem.bit(W_PLAYER_SUB_STATUS5, SUBSTATUS_LOCK_ON)
            && self.battle_var(BATTLE_VARS_MOVE_EFFECT) != EFFECT_ACCURACY_DOWN_HIT
        {
            failed = self.battle_random("BattleCommand_StatDown") < percent(25) + 1;
        }
        // .DidntMiss
        if failed
            || self.check_substitute_opp()
            || self.mem.rb(W_ATTACK_MISSED) != 0
            || self.mem.rb(W_EFFECT_FAILED) != 0
            || self.check_hidden_opponent()
        {
            // .Failed
            self.mem.wb(W_FAILED_MESSAGE, 1);
            self.mem.wb(W_ATTACK_MISSED, 1);
            return Ok(Cmd::Next);
        }
        // Accuracy/Evasion reduction don't involve stats.
        self.mem.wb(hl, b);
        if c < ACCURACY {
            let (stats, buf) = if self.is_player_turn() {
                (W_ENEMY_MON_ATTACK + 1, W_ENEMY_STATS)
            } else {
                (W_BATTLE_MON_ATTACK + 1, W_PLAYER_STATS)
            };
            if !self.try_lower_stat(stats, buf, c) {
                // .CouldntLower
                let v = self.mem.rb(hl).wrapping_add(1);
                self.mem.wb(hl, v);
                self.stat_down_cant_lower();
                return Ok(Cmd::Next);
            }
        }
        // .Hit
        self.mem.wb(W_FAILED_MESSAGE, 0);
        Ok(Cmd::Next)
    }

    /// `BattleCommand_StatDown.CantLower`.
    fn stat_down_cant_lower(&mut self) {
        self.mem.wb(W_FAILED_MESSAGE, 3);
        self.mem.wb(W_ATTACK_MISSED, 1);
    }

    /// `CheckMist`: nz (true) if the move is a stat-lowering one and the target is protected.
    pub fn check_mist(&self) -> bool {
        let e = self.battle_var(BATTLE_VARS_MOVE_EFFECT);
        let check = (EFFECT_ATTACK_DOWN..=EFFECT_EVASION_DOWN).contains(&e)
            || (EFFECT_ATTACK_DOWN_2..=EFFECT_EVASION_DOWN_2).contains(&e)
            || (EFFECT_ATTACK_DOWN_HIT..=EFFECT_EVASION_DOWN_HIT).contains(&e);
        check && self.battle_var_bit(BATTLE_VARS_SUBSTATUS4_OPP, SUBSTATUS_MIST)
    }

    /// `TryLowerStat`: lower stat `c` from stat struct `hl`; false (z) if it is already 1.
    pub fn try_lower_stat(&mut self, hl: u16, _de: u16, c: u8) -> bool {
        let at = hl + 2 * u16::from(c);
        if self.mem.rb(at) == 1 && self.mem.rb(at - 1) == 0 {
            return false;
        }
        let player = self.is_player_turn();
        self.switch_turn();
        if player {
            self.calc_enemy_stats();
        } else {
            self.calc_player_stats();
        }
        self.switch_turn();
        true
    }

    // --- messages ---------------------------------------------------------------------------

    /// `BattleCommand_StatUpMessage`.
    pub fn battle_command_stat_up_message(&mut self) -> Flow<Cmd> {
        if self.mem.rb(W_FAILED_MESSAGE) != 0 {
            return Ok(Cmd::Next);
        }
        let b = (self.mem.rb(W_LOWERED_STAT) & 0xf) + 1;
        self.get_stat_name(b);
        self.text(r::BATTLE_COMMAND_STAT_UP_MESSAGE_STAT);
        Ok(Cmd::Next)
    }

    /// `BattleCommand_StatDownMessage`.
    pub fn battle_command_stat_down_message(&mut self) -> Flow<Cmd> {
        if self.mem.rb(W_FAILED_MESSAGE) != 0 {
            return Ok(Cmd::Next);
        }
        let b = (self.mem.rb(W_LOWERED_STAT) & 0xf) + 1;
        self.get_stat_name(b);
        self.text(r::BATTLE_COMMAND_STAT_DOWN_MESSAGE_STAT);
        Ok(Cmd::Next)
    }

    /// `BattleCommand_StatUpFailText`.
    pub fn battle_command_stat_up_fail_text(&mut self) -> Flow<Cmd> {
        let a = self.mem.rb(W_FAILED_MESSAGE);
        if a == 0 {
            return Ok(Cmd::Next);
        }
        self.battle_command_move_delay()?;
        if a == 1 {
            self.try_print_but_it_failed();
            return Ok(Cmd::Next);
        }
        let b = (self.mem.rb(W_LOWERED_STAT) & 0xf) + 1;
        self.get_stat_name(b);
        self.text(r::WONT_RISE_ANYMORE_TEXT);
        Ok(Cmd::Next)
    }

    /// `BattleCommand_StatDownFailText`.
    pub fn battle_command_stat_down_fail_text(&mut self) -> Flow<Cmd> {
        let a = self.mem.rb(W_FAILED_MESSAGE);
        if a == 0 {
            return Ok(Cmd::Next);
        }
        self.battle_command_move_delay()?;
        if a == 1 {
            self.try_print_but_it_failed();
            return Ok(Cmd::Next);
        }
        if a == 2 {
            self.text(r::PROTECTED_BY_MIST_TEXT);
            return Ok(Cmd::Next);
        }
        let b = (self.mem.rb(W_LOWERED_STAT) & 0xf) + 1;
        self.get_stat_name(b);
        self.text(r::WONT_DROP_ANYMORE_TEXT);
        Ok(Cmd::Next)
    }

    /// `GetStatName`: the `b`th name of `StatNames` into `wStringBuffer2`.
    pub fn get_stat_name(&mut self, b: u8) {
        self.names.string_buffer2 = super::names::Name::Stat(b);
    }

    // --- AllStatsUp, LowerStat, TriStatusChance, Curl ----------------------------------------

    /// `BattleCommand_AllStatsUp`.
    pub fn battle_command_all_stats_up(&mut self) -> Flow<Cmd> {
        for stat in [ATTACK, DEFENSE, SPEED, SP_ATTACK, SP_DEFENSE] {
            self.mem.wb(W_ATTACK_MISSED, 0); // ResetMiss
            self.stat_up(stat)?;
            self.battle_command_stat_up_message()?;
        }
        Ok(Cmd::Next)
    }

    /// `LowerStat` with `a` = stat (| `$10`).
    pub fn lower_stat(&mut self, a: u8) {
        self.mem.wb(W_LOWERED_STAT, a);
        let levels = if self.is_player_turn() { W_PLAYER_STAT_LEVELS } else { W_ENEMY_STAT_LEVELS };
        let c = self.mem.rb(W_LOWERED_STAT) & 0xf;
        let hl = levels + u16::from(c);
        let mut b = self.mem.rb(hl).wrapping_sub(1);
        if b == 0 {
            // .cant_lower_anymore
            self.mem.wb(W_FAILED_MESSAGE, 2);
            return;
        }
        if self.mem.rb(W_LOWERED_STAT) & 0xf0 != 0 {
            b = b.wrapping_sub(1);
            if b == 0 {
                b = 1;
            }
        }
        self.mem.wb(hl, b);
        if c < ACCURACY {
            let (stats, buf) = if self.is_player_turn() {
                (W_BATTLE_MON_STATS + 1, W_PLAYER_STATS)
            } else {
                (W_ENEMY_MON_STATS + 1, W_ENEMY_STATS)
            };
            if !self.try_lower_stat(stats, buf, c) {
                // .failed
                let v = self.mem.rb(hl).wrapping_add(1);
                self.mem.wb(hl, v);
                self.mem.wb(W_FAILED_MESSAGE, 2);
                return;
            }
        }
        // .accuracy_evasion
        if self.is_player_turn() {
            self.calc_player_stats();
        } else {
            self.calc_enemy_stats();
        }
        self.mem.wb(W_FAILED_MESSAGE, 0);
    }

    /// `BattleCommand_TriStatusChance`.
    pub fn battle_command_tri_status_chance(&mut self) -> Flow<Cmd> {
        self.battle_command_effect_chance()?;
        // 1/3 chance of each status
        let a = loop {
            let a = (self.battle_random("BattleCommand_TriStatusChance").rotate_left(4)) & 0b11;
            if a != 0 {
                break a - 1;
            }
        };
        match a {
            0 => self.battle_command_paralyze_target(),
            1 => self.battle_command_freeze_target(),
            _ => self.battle_command_burn_target(),
        }
    }

    /// `BattleCommand_Curl`.
    pub fn battle_command_curl(&mut self) -> Flow<Cmd> {
        self.set_battle_var_bit(BATTLE_VARS_SUBSTATUS2, SUBSTATUS_CURLED);
        Ok(Cmd::Next)
    }

    // --- CalcPlayerStats / CalcEnemyStats ------------------------------------------------------

    /// `CalcPlayerStats`.
    pub fn calc_player_stats(&mut self) {
        self.calc_battle_stats(W_PLAYER_ATK_LEVEL, W_PLAYER_STATS, W_BATTLE_MON_ATTACK);
        self.badge_stat_boosts();
        self.switch_turn();
        self.apply_prz_effect_on_speed();
        self.apply_brn_effect_on_attack();
        self.switch_turn();
    }

    /// `CalcEnemyStats`.
    pub fn calc_enemy_stats(&mut self) {
        self.calc_battle_stats(W_ENEMY_ATK_LEVEL, W_ENEMY_STATS, W_ENEMY_MON_ATTACK);
        self.switch_turn();
        self.apply_prz_effect_on_speed();
        self.apply_brn_effect_on_attack();
        self.switch_turn();
    }

    /// `CalcBattleStats`: stat levels at `hl`, unmodified stats at `de`, results at `bc`.
    fn calc_battle_stats(&mut self, mut hl: u16, mut de: u16, mut bc: u16) {
        for _ in 0..NUM_BATTLE_STATS {
            let level = self.mem.rb(hl);
            hl += 1;
            let (multiplier, divisor) = self.data.tables.stat_level_multipliers[usize::from(level)];
            self.mem.wb(H_MULTIPLICAND, 0);
            let hi = self.mem.rb(de);
            self.mem.wb(H_MULTIPLICAND + 1, hi);
            de += 1;
            let lo = self.mem.rb(de);
            self.mem.wb(H_MULTIPLICAND + 2, lo);
            de += 1;
            self.mem.wb(H_MULTIPLIER, multiplier);
            self.multiply();
            self.mem.wb(H_DIVISOR, divisor);
            self.divide(4);
            let q2 = self.mem.rb(H_QUOTIENT + 2);
            let q3 = self.mem.rb(H_QUOTIENT + 3);
            if q2 | q3 == 0 {
                self.mem.wb(H_QUOTIENT + 3, 1);
            } else if u16::from(q2) << 8 | u16::from(q3) >= MAX_STAT_VALUE {
                self.mem.wb(H_QUOTIENT + 3, MAX_STAT_VALUE as u8);
                self.mem.wb(H_QUOTIENT + 2, (MAX_STAT_VALUE >> 8) as u8);
            }
            let v2 = self.mem.rb(H_QUOTIENT + 2);
            self.mem.wb(bc, v2);
            bc += 1;
            let v3 = self.mem.rb(H_QUOTIENT + 3);
            self.mem.wb(bc, v3);
            bc += 1;
        }
    }
}
