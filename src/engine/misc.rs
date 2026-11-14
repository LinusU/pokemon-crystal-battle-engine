//! Port of the misc effect commands: `engine/battle/effect_commands.asm` from
//! `BattleCommand_CheckRampage` to the end of the file (rampage, forced
//! switching, flinching, charging, trapping, recoil, confusion, paralysis,
//! screens, healing, and the small helpers they share). See docs/engine.md.

use super::battle::{Engine, Flow, PARTY_LEN, percent};
use super::commands::Cmd;
use super::rom;
use super::rsym as r;
use super::sym::*;
use crate::defs::*;

/// `endturn_command`.
const ENDTURN_CMD: u8 = 0xfe;

impl Engine {
    // --- script helpers -----------------------------------------------------------

    /// `SkipToBattleCommand`: skip over commands until reaching command `b`.
    pub fn skip_to_battle_command(&mut self, b: u8) {
        let mut hl = self.mem.rw_le(W_BATTLE_SCRIPT_BUFFER_ADDRESS);
        loop {
            let a = self.mem.rb(hl);
            hl = hl.wrapping_add(1);
            if a == b {
                break;
            }
        }
        self.mem.ww_le(W_BATTLE_SCRIPT_BUFFER_ADDRESS, hl);
    }

    // --- animation helpers (memory effects only) --------------------------------------

    /// `AnimateCurrentMove`.
    pub fn animate_current_move(&mut self) -> Flow {
        let param = self.mem.rb(W_BATTLE_ANIM_PARAM);
        self.battle_command_lower_sub()?;
        self.mem.wb(W_BATTLE_ANIM_PARAM, param);
        self.load_move_anim();
        self.battle_command_raise_sub()?;
        Ok(())
    }

    /// `AnimateFailedMove`.
    pub fn animate_failed_move(&mut self) -> Flow {
        self.battle_command_lower_sub()?;
        self.battle_command_move_delay()?;
        self.battle_command_raise_sub()?;
        Ok(())
    }

    /// `PlayOpponentBattleAnim` (`de` = animation id).
    fn play_opponent_battle_anim(&mut self, de: u16) {
        self.mem.wb(W_FX_ANIM_ID, de as u8);
        self.mem.wb(W_FX_ANIM_ID + 1, (de >> 8) as u8);
        self.mem.wb(W_BATTLE_AFTER_ANIM, 0);
    }

    // --- failure texts ----------------------------------------------------------------

    /// `FailText_CheckOpponentProtect`: `hl`, or `de` if the opponent protected.
    fn fail_text_check_opponent_protect(&mut self, hl: usize, de: usize) {
        let text = if self.battle_var_bit(BATTLE_VARS_SUBSTATUS1_OPP, SUBSTATUS_PROTECT) { de } else { hl };
        self.text(text);
    }

    /// `PrintDoesntAffect`.
    pub fn print_doesnt_affect(&mut self) {
        self.text(r::DOESNT_AFFECT_TEXT);
    }

    /// `PrintNothingHappened`.
    pub fn print_nothing_happened(&mut self) {
        self.text(r::NOTHING_HAPPENED_TEXT);
    }

    /// `TryPrintButItFailed`.
    pub fn try_print_but_it_failed(&mut self) {
        if self.mem.rb(W_ALREADY_FAILED) != 0 {
            return;
        }
        self.print_but_it_failed();
    }

    /// `PrintButItFailed`.
    pub fn print_but_it_failed(&mut self) {
        self.text(r::BUT_IT_FAILED_TEXT);
    }

    /// `FailMove`.
    pub fn fail_move(&mut self) -> Flow {
        self.animate_failed_move()?;
        self.fail_mimic();
        Ok(())
    }

    /// `FailMimic`.
    pub fn fail_mimic(&mut self) {
        self.fail_text_check_opponent_protect(r::BUT_IT_FAILED_TEXT, r::IT_FAILED_TEXT);
    }

    /// `PrintDidntAffect`.
    pub fn print_didnt_affect(&mut self) {
        self.text(r::DIDNT_AFFECT1_TEXT);
    }

    /// `PrintDidntAffect2`.
    pub fn print_didnt_affect2(&mut self) -> Flow {
        self.animate_failed_move()?;
        self.fail_text_check_opponent_protect(r::DIDNT_AFFECT1_TEXT, r::DIDNT_AFFECT2_TEXT);
        Ok(())
    }

    /// `PrintParalyze`.
    pub fn print_paralyze(&mut self) {
        self.text(r::PARALYZED_TEXT);
    }

    /// `BattleEffect_ButItFailed`.
    pub fn battle_effect_but_it_failed(&mut self) -> Flow {
        self.animate_failed_move()?;
        self.print_but_it_failed();
        Ok(())
    }

    // --- small checks -------------------------------------------------------------------

    /// `CheckSubstituteOpp`: nz (`true`) = the opponent has a substitute.
    pub fn check_substitute_opp(&self) -> bool {
        self.battle_var_bit(BATTLE_VARS_SUBSTATUS4_OPP, SUBSTATUS_SUBSTITUTE)
    }

    /// `CheckHiddenOpponent`: nz (`true`) = the opponent is flying or underground.
    pub fn check_hidden_opponent(&self) -> bool {
        // BUG: Lock-On and Mind Reader don't always bypass Fly and Dig (see docs/bugs_and_glitches.md)
        self.battle_var(BATTLE_VARS_SUBSTATUS3_OPP) & (1 << SUBSTATUS_FLYING | 1 << SUBSTATUS_UNDERGROUND) != 0
    }

    /// `CheckOpponentWentFirst`: 0 (z) if the user went first, 1 (nz) if the opponent did.
    pub fn check_opponent_went_first(&self) -> u8 {
        self.mem.rb(W_ENEMY_GOES_FIRST) ^ self.turn()
    }

    /// `CheckMoveTypeMatchesTarget`: z (`true`) if the move's type matches the
    /// opponent's type (never for Normal).
    pub fn check_move_type_matches_target(&self) -> bool {
        let hl = if self.is_player_turn() { W_ENEMY_MON_TYPE1 } else { W_BATTLE_MON_TYPE1 };
        let ty = self.battle_var(BATTLE_VARS_MOVE_TYPE);
        if ty == NORMAL {
            return false;
        }
        ty == self.mem.rb(hl) || ty == self.mem.rb(hl + 1)
    }

    /// `CheckUserMove`: z (`true`) if the user has move `a`.
    pub fn check_user_move(&self, a: u8) -> bool {
        let de = if self.is_player_turn() { W_BATTLE_MON_MOVES } else { W_ENEMY_MON_MOVES };
        (0..NUM_MOVES as u16).any(|i| self.mem.rb(de + i) == a)
    }

    /// `SafeCheckSafeguard`: nz (`true`) = the *opponent of the turn's
    /// opponent* — i.e. the user's own side — has Safeguard up.
    pub fn safe_check_safeguard(&self) -> bool {
        let hl = if self.is_player_turn() { W_ENEMY_SCREENS } else { W_PLAYER_SCREENS };
        self.mem.bit(hl, SCREENS_SAFEGUARD)
    }

    /// `EndRechargeOpp`.
    pub fn end_recharge_opp(&mut self) {
        self.res_battle_var_bit(BATTLE_VARS_SUBSTATUS4_OPP, SUBSTATUS_RECHARGE);
    }

    /// `ClearLastMove`.
    pub fn clear_last_move(&mut self) {
        self.set_battle_var(BATTLE_VARS_LAST_COUNTER_MOVE, 0);
        self.set_battle_var(BATTLE_VARS_LAST_MOVE, 0);
    }

    /// `ResetActorDisable`.
    pub fn reset_actor_disable(&mut self) {
        if self.is_player_turn() {
            self.mem.wb(W_PLAYER_DISABLE_COUNT, 0);
            self.mem.wb(W_DISABLED_MOVE, 0);
        } else {
            self.mem.wb(W_ENEMY_DISABLE_COUNT, 0);
            self.mem.wb(W_ENEMY_DISABLED_MOVE, 0);
        }
    }

    /// `ResetTurn`.
    pub fn reset_turn(&mut self) -> Flow {
        let hl = if self.is_player_turn() { W_PLAYER_CHARGING } else { W_ENEMY_CHARGING };
        self.mem.wb(hl, 1);
        self.mem.wb(W_ALREADY_DISOBEYED, 0);
        self.do_move()?;
        self.end_move_effect();
        Ok(())
    }

    /// `SetBattleDraw`.
    pub fn set_battle_draw(&mut self) {
        let result = self.mem.rb(W_BATTLE_RESULT) & BATTLERESULT_BITMASK | DRAW;
        self.mem.wb(W_BATTLE_RESULT, result);
    }

    // --- rampage ------------------------------------------------------------------------

    /// `BattleCommand_CheckRampage`.
    pub fn battle_command_check_rampage(&mut self) -> Flow<Cmd> {
        let de = if self.is_player_turn() { W_PLAYER_ROLLOUT_COUNT } else { W_ENEMY_ROLLOUT_COUNT };
        if !self.battle_var_bit(BATTLE_VARS_SUBSTATUS3, SUBSTATUS_RAMPAGE) {
            return Ok(Cmd::Next);
        }
        let count = self.mem.rb(de).wrapping_sub(1);
        self.mem.wb(de, count);
        if count == 0 {
            self.res_battle_var_bit(BATTLE_VARS_SUBSTATUS3, SUBSTATUS_RAMPAGE);
            self.switch_turn();
            let safeguarded = self.safe_check_safeguard();
            self.switch_turn();
            if !safeguarded {
                self.set_battle_var_bit(BATTLE_VARS_SUBSTATUS3, SUBSTATUS_CONFUSED);
                // Confused for 2 or 3 turns.
                let a = (self.battle_random("BattleCommand_CheckRampage") & 1) + 2;
                self.mem.wb(de + 1, a); // ConfuseCount
            }
        }
        // .continue_rampage
        self.skip_to_battle_command(RAMPAGE_COMMAND);
        Ok(Cmd::Next)
    }

    /// `BattleCommand_Rampage`.
    pub fn battle_command_rampage(&mut self) -> Flow<Cmd> {
        // No rampage during Sleep Talk.
        if self.battle_var(BATTLE_VARS_STATUS) & SLP_MASK != 0 {
            return Ok(Cmd::Next);
        }
        let de = if self.is_player_turn() { W_PLAYER_ROLLOUT_COUNT } else { W_ENEMY_ROLLOUT_COUNT };
        self.set_battle_var_bit(BATTLE_VARS_SUBSTATUS3, SUBSTATUS_RAMPAGE);
        // Rampage for 1 or 2 more turns.
        let a = (self.battle_random("BattleCommand_Rampage") & 1) + 1;
        self.mem.wb(de, a);
        self.mem.wb(W_SOMEONE_IS_RAMPAGING, 1);
        Ok(Cmd::Next)
    }

    // --- forced switching ------------------------------------------------------------------

    /// `FindAliveEnemyMons` (`engine/battle/ai/switch.asm`): carry (`true`) =
    /// there is no other living mon in the enemy's party.
    pub fn find_alive_enemy_mons(&mut self) -> bool {
        let count = self.mem.rb(W_OT_PARTY_COUNT);
        if count < 2 {
            return true;
        }
        let cur = self.mem.rb(W_CUR_OT_MON);
        let mut b: u8 = 1 << (PARTY_LENGTH - 1);
        let mut c: u8 = 0;
        let mut hl = W_OT_PARTY_MON1_HP;
        for e in 0..count {
            if cur != e && self.mem.rw(hl) != 0 {
                c |= b;
            }
            b >>= 1;
            hl += PARTY_LEN;
        }
        c == 0
    }

    /// `CheckPlayerHasMonToSwitchTo`: carry (`true`) = none.
    fn check_player_has_mon_to_switch_to(&self) -> bool {
        let cur = self.mem.rb(W_CUR_BATTLE_MON);
        for e in 0..self.mem.rb(W_PARTY_COUNT) {
            if cur != e && self.mem.rw(self.get_party_location(W_PARTY_MON1_HP, e)) != 0 {
                return false;
            }
        }
        true
    }

    /// `ForceEnemySwitch` (`engine/battle/core.asm`).
    fn force_enemy_switch(&mut self) {
        self.reset_enemy_battle_vars();
        let b = self.mem.rb(W_ENEMY_SWITCH_MON_INDEX).wrapping_sub(1);
        self.load_enemy_mon_to_switch_to(b);
        // ClearEnemyMonBox: graphics only.
        self.new_enemy_mon_status();
        self.reset_enemy_stat_levels();
        self.show_set_enemy_mon_and_send_out_animation();
        self.break_attraction();
        self.reset_battle_participants();
    }

    /// `BattleCommand_ForceSwitch` (Roar, Whirlwind).
    pub fn battle_command_force_switch(&mut self) -> Flow<Cmd> {
        let battle_type = self.mem.rb(W_BATTLE_TYPE);
        if matches!(battle_type, BATTLETYPE_FORCESHINY | BATTLETYPE_TRAP | BATTLETYPE_CELEBI | BATTLETYPE_SUICUNE) {
            return self.force_switch_fail();
        }
        if !self.is_player_turn() {
            return self.force_player_switch();
        }
        if self.mem.rb(W_ATTACK_MISSED) != 0 {
            return self.force_switch_fail();
        }
        if self.battle_mode() != WILD_BATTLE {
            // .trainer
            if self.find_alive_enemy_mons() {
                return self.force_switch_fail();
            }
            if self.mem.rb(W_ENEMY_GOES_FIRST) == 0 {
                return self.force_switch_fail();
            }
            self.update_enemy_mon_in_party();
            self.mem.wb(W_BATTLE_ANIM_PARAM, 1);
            self.animate_current_move()?;
            let b = self.mem.rb(W_OT_PARTY_COUNT);
            let c = self.mem.rb(W_CUR_OT_MON);
            // Select a random enemy mon to switch to.
            let index = loop {
                let a = self.battle_random("BattleCommand_ForceSwitch") & 7;
                if a >= b || a == c {
                    continue;
                }
                if self.mem.rw(self.get_party_location(W_OT_PARTY_MON1_HP, a)) == 0 {
                    continue;
                }
                break a;
            };
            self.mem.wb(W_ENEMY_SWITCH_MON_INDEX, index + 1);
            self.force_enemy_switch();
            self.text(r::DRAGGED_OUT_TEXT);
            self.spikes_damage();
            return Ok(Cmd::Next);
        }
        // Wild battle.
        let mut b = self.mem.rb(W_CUR_PARTY_LEVEL);
        let a = self.mem.rb(W_BATTLE_MON_LEVEL);
        if a < b {
            let c = a.wrapping_add(b).wrapping_add(1);
            let roll = loop {
                let roll = self.battle_random("BattleCommand_ForceSwitch");
                if roll < c {
                    break roll;
                }
            };
            b >>= 2;
            if roll < b {
                return self.force_switch_fail();
            }
        }
        // .wild_force_flee
        self.update_battle_mon_in_party();
        self.mem.wb(W_BATTLE_AFTER_ANIM, 0);
        self.mem.wb(W_FORCED_SWITCH, 1);
        self.set_battle_draw();
        let anim = self.mem.rb(W_PLAYER_MOVE_STRUCT_ANIMATION);
        self.force_switch_succeed(anim)
    }

    /// `BattleCommand_ForceSwitch.force_player_switch`.
    fn force_player_switch(&mut self) -> Flow<Cmd> {
        if self.mem.rb(W_ATTACK_MISSED) != 0 {
            return self.force_switch_fail();
        }
        if self.battle_mode() != WILD_BATTLE {
            // .vs_trainer
            if self.check_player_has_mon_to_switch_to() {
                return self.force_switch_fail();
            }
            if self.mem.rb(W_ENEMY_GOES_FIRST) == 1 {
                return self.force_switch_fail();
            }
            self.update_battle_mon_in_party();
            self.mem.wb(W_BATTLE_ANIM_PARAM, 1);
            self.animate_current_move()?;
            let b = self.mem.rb(W_PARTY_COUNT);
            let c = self.mem.rb(W_CUR_BATTLE_MON);
            let index = loop {
                let a = self.battle_random("BattleCommand_ForceSwitch") & 7;
                if a >= b || a == c {
                    continue;
                }
                if self.mem.rw(self.get_party_location(W_PARTY_MON1_HP, a)) == 0 {
                    continue;
                }
                break a;
            };
            self.mem.wb(W_CUR_PARTY_MON, index);
            self.switch_player_mon();
            self.text(r::DRAGGED_OUT_TEXT);
            self.spikes_damage();
            return Ok(Cmd::Next);
        }
        let mut b = self.mem.rb(W_BATTLE_MON_LEVEL);
        let a = self.mem.rb(W_CUR_PARTY_LEVEL);
        if a < b {
            let c = a.wrapping_add(b).wrapping_add(1);
            let roll = loop {
                let roll = self.battle_random("BattleCommand_ForceSwitch");
                if roll < c {
                    break roll;
                }
            };
            b >>= 2;
            if roll < b {
                return self.force_switch_fail();
            }
        }
        // .wild_succeed_playeristarget
        self.update_battle_mon_in_party();
        self.mem.wb(W_BATTLE_AFTER_ANIM, 0);
        self.mem.wb(W_FORCED_SWITCH, 1);
        self.set_battle_draw();
        let anim = self.mem.rb(W_ENEMY_MOVE_STRUCT_ANIMATION);
        self.force_switch_succeed(anim)
    }

    /// `BattleCommand_ForceSwitch.fail`.
    fn force_switch_fail(&mut self) -> Flow<Cmd> {
        self.battle_command_lower_sub()?;
        self.battle_command_move_delay()?;
        self.battle_command_raise_sub()?;
        self.print_but_it_failed();
        Ok(Cmd::Next)
    }

    /// `BattleCommand_ForceSwitch.succeed` (`a` = the move's animation).
    fn force_switch_succeed(&mut self, a: u8) -> Flow<Cmd> {
        self.set_battle_draw();
        self.mem.wb(W_BATTLE_ANIM_PARAM, 1);
        self.animate_current_move()?;
        self.text(if a == ROAR { r::FLED_IN_FEAR_TEXT } else { r::BLOWN_AWAY_TEXT });
        Ok(Cmd::Next)
    }

    // --- flinching ------------------------------------------------------------------------

    /// `BattleCommand_FakeOut`.
    pub fn battle_command_fake_out(&mut self) -> Flow<Cmd> {
        if self.mem.rb(W_ATTACK_MISSED) != 0 {
            return Ok(Cmd::Next);
        }
        let fail = self.check_substitute_opp()
            || self.battle_var(BATTLE_VARS_STATUS_OPP) & (1 << FRZ | SLP_MASK) != 0
            || self.check_opponent_went_first() != 0;
        if fail {
            self.mem.wb(W_ATTACK_MISSED, 1);
            return Ok(Cmd::Next);
        }
        self.flinch_target();
        Ok(Cmd::Next)
    }

    /// `BattleCommand_FlinchTarget`.
    pub fn battle_command_flinch_target(&mut self) -> Flow<Cmd> {
        if self.check_substitute_opp() {
            return Ok(Cmd::Next);
        }
        if self.battle_var(BATTLE_VARS_STATUS_OPP) & (1 << FRZ | SLP_MASK) != 0 {
            return Ok(Cmd::Next);
        }
        if self.check_opponent_went_first() != 0 {
            return Ok(Cmd::Next);
        }
        if self.mem.rb(W_EFFECT_FAILED) != 0 {
            return Ok(Cmd::Next);
        }
        self.flinch_target();
        Ok(Cmd::Next)
    }

    /// `FlinchTarget`.
    fn flinch_target(&mut self) {
        self.set_battle_var_bit(BATTLE_VARS_SUBSTATUS3_OPP, SUBSTATUS_FLINCHED);
        self.end_recharge_opp();
    }

    // --- OHKO ------------------------------------------------------------------------------

    /// `BattleCommand_OHKO`.
    pub fn battle_command_ohko(&mut self) -> Flow<Cmd> {
        self.reset_damage();
        if self.mem.rb(W_TYPE_MODIFIER) & EFFECTIVENESS_MASK == 0 {
            return Ok(self.ohko_no_effect());
        }
        let (target_level, user_level, bc) = if self.is_player_turn() {
            (W_ENEMY_MON_LEVEL, W_BATTLE_MON_LEVEL, W_PLAYER_MOVE_STRUCT + u16::from(MOVE_ACC))
        } else {
            (W_BATTLE_MON_LEVEL, W_ENEMY_MON_LEVEL, W_ENEMY_MOVE_STRUCT + u16::from(MOVE_ACC))
        };
        let Some(diff) = self.mem.rb(user_level).checked_sub(self.mem.rb(target_level)) else {
            return Ok(self.ohko_no_effect());
        };
        let e = diff.wrapping_add(diff);
        let acc = self.mem.rb(bc).checked_add(e).unwrap_or(0xff);
        self.mem.wb(bc, acc);
        self.battle_command_check_hit()?;
        self.mem.wb(W_CUR_DAMAGE, 0xff);
        self.mem.wb(W_CUR_DAMAGE + 1, 0xff);
        self.mem.wb(W_CRITICAL_HIT, 2);
        Ok(Cmd::Next)
    }

    /// `BattleCommand_OHKO.no_effect`.
    fn ohko_no_effect(&mut self) -> Cmd {
        self.mem.wb(W_CRITICAL_HIT, 0xff);
        self.mem.wb(W_ATTACK_MISSED, 1);
        Cmd::Next
    }

    // --- charging ----------------------------------------------------------------------------

    /// `BattleCommand_CheckCharge`.
    pub fn battle_command_check_charge(&mut self) -> Flow<Cmd> {
        if !self.battle_var_bit(BATTLE_VARS_SUBSTATUS3, SUBSTATUS_CHARGED) {
            return Ok(Cmd::Next);
        }
        self.res_battle_var_bit(BATTLE_VARS_SUBSTATUS3, SUBSTATUS_CHARGED);
        self.res_battle_var_bit(BATTLE_VARS_SUBSTATUS3, SUBSTATUS_UNDERGROUND);
        self.res_battle_var_bit(BATTLE_VARS_SUBSTATUS3, SUBSTATUS_FLYING);
        self.skip_to_battle_command(CHARGE_COMMAND);
        Ok(Cmd::Next)
    }

    /// `BattleCommand_Charge`.
    pub fn battle_command_charge(&mut self) -> Flow<Cmd> {
        self.battle_command_clear_text()?;
        if self.battle_var(BATTLE_VARS_STATUS) & SLP_MASK != 0 {
            self.battle_command_move_delay()?;
            self.battle_command_raise_sub()?;
            self.print_but_it_failed();
            self.end_move_effect();
            return Ok(Cmd::Next);
        }
        // .awake
        self.set_battle_var_bit(BATTLE_VARS_SUBSTATUS3, SUBSTATUS_CHARGED);
        if self.mem.rb(W_ALREADY_DISOBEYED) != 0 {
            self.text(r::IGNORED_ORDERS2_TEXT);
        }
        self.battle_command_lower_sub()?;
        self.mem.wb(W_BATTLE_AFTER_ANIM, 0);
        self.mem.wb(W_BATTLE_ANIM_PARAM, 1);
        self.load_move_anim();
        let anim = self.battle_var(BATTLE_VARS_MOVE_ANIM);
        if anim == FLY || anim == DIG {
            // DisappearUser: graphics only.
        } else {
            self.battle_command_raise_sub()?;
        }
        let b = self.battle_var(BATTLE_VARS_MOVE_ANIM);
        if b == FLY {
            self.set_battle_var_bit(BATTLE_VARS_SUBSTATUS3, SUBSTATUS_FLYING);
        } else if b == DIG {
            self.set_battle_var_bit(BATTLE_VARS_SUBSTATUS3, SUBSTATUS_UNDERGROUND);
        }
        if !self.check_user_is_charging() {
            self.set_battle_var(BATTLE_VARS_LAST_COUNTER_MOVE, b);
            self.set_battle_var(BATTLE_VARS_LAST_MOVE, b);
        }
        // .mimic
        self.reset_damage();

        // .UsedText: "<USER>" then the move's own message.
        self.text(r::TEXT_BATTLE_USER);
        let anim = self.battle_var(BATTLE_VARS_MOVE_ANIM);
        let used = match anim {
            RAZOR_WIND => Some(r::_BATTLE_MADE_WHIRLWIND_TEXT),
            SOLARBEAM => Some(r::_BATTLE_TOOK_SUNLIGHT_TEXT),
            SKULL_BASH => Some(r::_BATTLE_LOWERED_HEAD_TEXT),
            SKY_ATTACK => Some(r::_BATTLE_GLOWING_TEXT),
            FLY => Some(r::_BATTLE_FLEW_TEXT),
            DIG => Some(r::_BATTLE_DUG_TEXT),
            _ => None,
        };
        if let Some(text) = used {
            self.text(text);
        }

        if self.battle_var(BATTLE_VARS_MOVE_EFFECT) == EFFECT_SKULL_BASH {
            self.skip_to_battle_command(ENDTURN_CMD);
        } else {
            self.end_move_effect();
        }
        Ok(Cmd::Next)
    }

    /// `BattleCommand_Unused3C`.
    pub fn battle_command_unused3_c(&mut self) -> Flow<Cmd> {
        Ok(Cmd::Next)
    }

    // --- trapping --------------------------------------------------------------------------------

    /// `BattleCommand_TrapTarget`.
    pub fn battle_command_trap_target(&mut self) -> Flow<Cmd> {
        if self.mem.rb(W_ATTACK_MISSED) != 0 {
            return Ok(Cmd::Next);
        }
        let (hl, de) = if self.is_player_turn() {
            (W_ENEMY_WRAP_COUNT, W_ENEMY_TRAPPING_MOVE)
        } else {
            (W_PLAYER_WRAP_COUNT, W_PLAYER_TRAPPING_MOVE)
        };
        if self.mem.rb(hl) != 0 {
            return Ok(Cmd::Next);
        }
        if self.check_substitute_opp() {
            return Ok(Cmd::Next);
        }
        // Trapped for 2-5 turns.
        let turns = (self.battle_random("BattleCommand_TrapTarget") & 3) + 3;
        self.mem.wb(hl, turns);
        let anim = self.battle_var(BATTLE_VARS_MOVE_ANIM);
        self.mem.wb(de, anim);
        let text = match anim {
            BIND => r::USED_BIND_TEXT,
            WRAP => r::WRAPPED_BY_TEXT,
            FIRE_SPIN => r::FIRE_SPIN_TRAP_TEXT,
            CLAMP => r::CLAMPED_BY_TEXT,
            WHIRLPOOL => r::WHIRLPOOL_TRAP_TEXT,
            _ => unreachable!("BattleCommand_TrapTarget: no trap text for move {anim}"),
        };
        self.text(text);
        Ok(Cmd::Next)
    }

    // --- recoil ----------------------------------------------------------------------------------

    /// `BattleCommand_Recoil`.
    pub fn battle_command_recoil(&mut self) -> Flow<Cmd> {
        let hl = if self.is_player_turn() { W_BATTLE_MON_MAX_HP } else { W_ENEMY_MON_MAX_HP };
        // Get 1/4 damage or 1 HP, whichever is higher.
        let mut bc = self.mem.rw(W_CUR_DAMAGE) >> 2;
        if bc == 0 {
            bc = 1;
        }
        let (b, c) = ((bc >> 8) as u8, bc as u8);
        let max_hi = self.mem.rb(hl);
        let max_lo = self.mem.rb(hl + 1);
        self.mem.wb(W_HP_BUFFER1 + 1, max_hi);
        self.mem.wb(W_HP_BUFFER1, max_lo);
        let hp_hi = hl - 2;
        let hp_lo = hl - 1;
        let lo = self.mem.rb(hp_lo);
        self.mem.wb(W_HP_BUFFER2, lo);
        let (new_lo, borrow_lo) = lo.overflowing_sub(c);
        self.mem.wb(hp_lo, new_lo);
        self.mem.wb(W_HP_BUFFER3, new_lo);
        let hi = self.mem.rb(hp_hi);
        self.mem.wb(W_HP_BUFFER2 + 1, hi);
        let (t, b1) = hi.overflowing_sub(b);
        let (new_hi, b2) = t.overflowing_sub(u8::from(borrow_lo));
        self.mem.wb(hp_hi, new_hi);
        self.mem.wb(W_HP_BUFFER3 + 1, new_hi);
        if b1 || b2 {
            // .ko
            self.mem.wb(hp_hi, 0);
            self.mem.wb(hp_lo, 0);
            self.mem.wb(W_HP_BUFFER3, 0);
            self.mem.wb(W_HP_BUFFER3 + 1, 0);
        }
        // .dont_ko
        let which = u8::from(self.is_player_turn());
        self.mem.wb(W_WHICH_HP_BAR, which);
        self.animate_hp_bar();
        // RefreshBattleHuds: graphics only.
        self.text(r::RECOIL_TEXT);
        Ok(Cmd::Next)
    }

    // --- confusion -----------------------------------------------------------------------------------

    /// `BattleCommand_ConfuseTarget`.
    pub fn battle_command_confuse_target(&mut self) -> Flow<Cmd> {
        let (_, b, _) = self.get_opponent_item();
        if b == HELD_PREVENT_CONFUSE {
            return Ok(Cmd::Next);
        }
        if self.mem.rb(W_EFFECT_FAILED) != 0 {
            return Ok(Cmd::Next);
        }
        if self.safe_check_safeguard() {
            return Ok(Cmd::Next);
        }
        if self.check_substitute_opp() {
            return Ok(Cmd::Next);
        }
        if self.battle_var_bit(BATTLE_VARS_SUBSTATUS3_OPP, SUBSTATUS_CONFUSED) {
            return Ok(Cmd::Next);
        }
        self.battle_command_finish_confusing_target()
    }

    /// `BattleCommand_Confuse`.
    pub fn battle_command_confuse(&mut self) -> Flow<Cmd> {
        let (item_addr, b, _) = self.get_opponent_item();
        if b == HELD_PREVENT_CONFUSE {
            let item = self.mem.rb(item_addr);
            self.mem.wb(W_NAMED_OBJECT_INDEX, item);
            self.get_item_name();
            self.animate_failed_move()?;
            self.text(r::PROTECTED_BY_TEXT);
            return Ok(Cmd::Next);
        }
        if self.battle_var_bit(BATTLE_VARS_SUBSTATUS3_OPP, SUBSTATUS_CONFUSED) {
            self.animate_failed_move()?;
            self.text(r::ALREADY_CONFUSED_TEXT);
            return Ok(Cmd::Next);
        }
        if self.check_substitute_opp() || self.mem.rb(W_ATTACK_MISSED) != 0 {
            return self.battle_command_confuse_check_snore_swagger_confuse_hit();
        }
        self.battle_command_finish_confusing_target()
    }

    /// `BattleCommand_FinishConfusingTarget`.
    pub fn battle_command_finish_confusing_target(&mut self) -> Flow<Cmd> {
        let bc = if self.is_player_turn() { W_ENEMY_CONFUSE_COUNT } else { W_PLAYER_CONFUSE_COUNT };
        self.set_battle_var_bit(BATTLE_VARS_SUBSTATUS3_OPP, SUBSTATUS_CONFUSED);
        // Confused for 2-5 turns.
        let turns = (self.battle_random("BattleCommand_FinishConfusingTarget") & 3) + 2;
        self.mem.wb(bc, turns);

        let effect = self.battle_var(BATTLE_VARS_MOVE_EFFECT);
        if !matches!(effect, EFFECT_CONFUSE_HIT | EFFECT_SNORE | EFFECT_SWAGGER) {
            self.animate_current_move()?;
        }
        self.play_opponent_battle_anim(ANIM_CONFUSED);
        self.text(r::BECAME_CONFUSED_TEXT);

        let (_, b, _) = self.get_opponent_item();
        if b == HELD_HEAL_STATUS || b == HELD_HEAL_CONFUSION {
            self.use_confusion_healing_item();
        }
        Ok(Cmd::Next)
    }

    /// `BattleCommand_Confuse_CheckSnore_Swagger_ConfuseHit`.
    fn battle_command_confuse_check_snore_swagger_confuse_hit(&mut self) -> Flow<Cmd> {
        let effect = self.battle_var(BATTLE_VARS_MOVE_EFFECT);
        if matches!(effect, EFFECT_CONFUSE_HIT | EFFECT_SNORE | EFFECT_SWAGGER) {
            return Ok(Cmd::Next);
        }
        self.print_didnt_affect2()?;
        Ok(Cmd::Next)
    }

    // --- paralysis -----------------------------------------------------------------------------------

    /// `BattleCommand_Paralyze`.
    pub fn battle_command_paralyze(&mut self) -> Flow<Cmd> {
        if self.battle_var_bit(BATTLE_VARS_STATUS_OPP, PAR) {
            self.animate_failed_move()?;
            self.text(r::ALREADY_PARALYZED_TEXT);
            return Ok(Cmd::Next);
        }
        if self.mem.rb(W_TYPE_MODIFIER) & EFFECTIVENESS_MASK == 0 {
            self.animate_failed_move()?;
            self.print_doesnt_affect();
            return Ok(Cmd::Next);
        }
        let (item_addr, b, _) = self.get_opponent_item();
        if b == HELD_PREVENT_PARALYZE {
            let item = self.mem.rb(item_addr);
            self.mem.wb(W_NAMED_OBJECT_INDEX, item);
            self.get_item_name();
            self.animate_failed_move()?;
            self.text(r::PROTECTED_BY_TEXT);
            return Ok(Cmd::Next);
        }
        // .no_item_protection
        let sample_failure = !self.is_player_turn()
            && self.mem.rb(W_LINK_MODE) == 0
            && self.mem.rb(W_IN_BATTLE_TOWER_BATTLE) == 0
            && !self.mem.bit(W_PLAYER_SUB_STATUS5, SUBSTATUS_LOCK_ON);
        let mut failed = false;
        if sample_failure {
            // 25% chance AI fails.
            failed = self.battle_random("BattleCommand_Paralyze") < percent(25) + 1;
        }
        // .dont_sample_failure
        if failed
            || self.battle_var(BATTLE_VARS_STATUS_OPP) != 0
            || self.mem.rb(W_ATTACK_MISSED) != 0
            || self.check_substitute_opp()
        {
            self.print_didnt_affect2()?;
            return Ok(Cmd::Next);
        }
        self.animate_current_move()?;
        self.set_battle_var_bit(BATTLE_VARS_STATUS_OPP, PAR);
        self.update_opponent_in_party();
        self.apply_prz_effect_on_speed();
        self.update_battle_huds();
        self.print_paralyze();
        self.use_held_status_healing_item();
        Ok(Cmd::Next)
    }

    /// `BattleCommand_RechargeNextTurn`.
    pub fn battle_command_recharge_next_turn(&mut self) -> Flow<Cmd> {
        self.set_battle_var_bit(BATTLE_VARS_SUBSTATUS4, SUBSTATUS_RECHARGE);
        Ok(Cmd::Next)
    }

    // --- damage doubling ---------------------------------------------------------------------------------

    /// `BattleCommand_DoubleFlyingDamage`.
    pub fn battle_command_double_flying_damage(&mut self) -> Flow<Cmd> {
        if self.battle_var_bit(BATTLE_VARS_SUBSTATUS3_OPP, SUBSTATUS_FLYING) {
            self.double_damage();
        }
        Ok(Cmd::Next)
    }

    /// `BattleCommand_DoubleUndergroundDamage`.
    pub fn battle_command_double_underground_damage(&mut self) -> Flow<Cmd> {
        if self.battle_var_bit(BATTLE_VARS_SUBSTATUS3_OPP, SUBSTATUS_UNDERGROUND) {
            self.double_damage();
        }
        Ok(Cmd::Next)
    }

    /// `DoubleDamage`: `wCurDamage` * 2, capped at $ffff.
    fn double_damage(&mut self) {
        let damage = self.mem.rw(W_CUR_DAMAGE);
        let doubled = if damage & 0x8000 != 0 { 0xffff } else { damage << 1 };
        self.mem.ww(W_CUR_DAMAGE, doubled);
    }

    /// `BattleCommand_DoubleMinimizeDamage`.
    pub fn battle_command_double_minimize_damage(&mut self) -> Flow<Cmd> {
        let hl = if self.is_player_turn() { W_ENEMY_MINIMIZED } else { W_PLAYER_MINIMIZED };
        if self.mem.rb(hl) != 0 {
            self.double_damage();
        }
        Ok(Cmd::Next)
    }

    /// `BattleCommand_SkipSunCharge`.
    pub fn battle_command_skip_sun_charge(&mut self) -> Flow<Cmd> {
        if self.mem.rb(W_BATTLE_WEATHER) != WEATHER_SUN {
            return Ok(Cmd::Next);
        }
        self.skip_to_battle_command(CHARGE_COMMAND);
        Ok(Cmd::Next)
    }

    // --- stats, healing, screens --------------------------------------------------------------------------

    /// `BattleCommand_ResetStats` (Haze).
    pub fn battle_command_reset_stats(&mut self) -> Flow<Cmd> {
        self.mem.fill(W_PLAYER_STAT_LEVELS, u16::from(NUM_LEVEL_STATS), BASE_STAT_LEVEL);
        self.mem.fill(W_ENEMY_STAT_LEVELS, u16::from(NUM_LEVEL_STATS), BASE_STAT_LEVEL);
        let turn = self.turn();
        self.set_player_turn();
        self.calc_player_stats();
        self.set_enemy_turn();
        self.calc_enemy_stats();
        self.mem.wb(H_BATTLE_TURN, turn);
        self.animate_current_move()?;
        self.text(r::ELIMINATED_STATS_TEXT);
        Ok(Cmd::Next)
    }

    /// `BattleCommand_Heal` (Recover, Softboiled, Rest, ...).
    pub fn battle_command_heal(&mut self) -> Flow<Cmd> {
        let (de, hl) = if self.is_player_turn() {
            (W_BATTLE_MON_HP, W_BATTLE_MON_MAX_HP)
        } else {
            (W_ENEMY_MON_HP, W_ENEMY_MON_MAX_HP)
        };
        let b = self.battle_var(BATTLE_VARS_MOVE_ANIM);
        if self.mem.rw(de) == self.mem.rw(hl) {
            // .hp_full
            self.animate_failed_move()?;
            self.text(r::HP_IS_FULL_TEXT);
            return Ok(Cmd::Next);
        }
        let bc = if b == REST {
            self.battle_command_move_delay()?;
            self.res_battle_var_bit(BATTLE_VARS_SUBSTATUS5, SUBSTATUS_TOXIC);
            let status = self.battle_var(BATTLE_VARS_STATUS);
            self.set_battle_var(BATTLE_VARS_STATUS, REST_SLEEP_TURNS + 1);
            self.text(if status == 0 { r::WENT_TO_SLEEP_TEXT } else { r::RESTED_TEXT });
            if self.is_player_turn() {
                self.calc_player_stats();
            } else {
                self.calc_enemy_stats();
            }
            self.get_max_hp()
        } else {
            self.get_half_max_hp()
        };
        // .finish
        self.animate_current_move()?;
        self.switch_turn();
        self.restore_hp(bc);
        self.switch_turn();
        self.update_user_in_party();
        // RefreshBattleHuds: graphics only.
        self.text(r::REGAINED_HEALTH_TEXT);
        Ok(Cmd::Next)
    }

    /// `BattleCommand_Screen` (Light Screen, Reflect).
    pub fn battle_command_screen(&mut self) -> Flow<Cmd> {
        let (hl, bc) = if self.is_player_turn() {
            (W_PLAYER_SCREENS, W_PLAYER_LIGHT_SCREEN_COUNT)
        } else {
            (W_ENEMY_SCREENS, W_ENEMY_LIGHT_SCREEN_COUNT)
        };
        let text;
        if self.battle_var(BATTLE_VARS_MOVE_EFFECT) == EFFECT_LIGHT_SCREEN {
            if self.mem.bit(hl, SCREENS_LIGHT_SCREEN) {
                return self.screen_failed();
            }
            self.mem.set_bit(hl, SCREENS_LIGHT_SCREEN);
            self.mem.wb(bc, 5);
            text = r::LIGHT_SCREEN_EFFECT_TEXT;
        } else {
            // .Reflect
            if self.mem.bit(hl, SCREENS_REFLECT) {
                return self.screen_failed();
            }
            self.mem.set_bit(hl, SCREENS_REFLECT);
            // LightScreenCount -> ReflectCount
            self.mem.wb(bc + 1, 5);
            text = r::REFLECT_EFFECT_TEXT;
        }
        self.animate_current_move()?;
        self.text(text);
        Ok(Cmd::Next)
    }

    /// `BattleCommand_Screen.failed`.
    fn screen_failed(&mut self) -> Flow<Cmd> {
        self.animate_failed_move()?;
        self.print_but_it_failed();
        Ok(Cmd::Next)
    }

    /// `BattleCommand_ArenaTrap` (Mean Look, Spider Web).
    pub fn battle_command_arena_trap(&mut self) -> Flow<Cmd> {
        // Doesn't work on an absent opponent, or one that is already trapped.
        if self.check_hidden_opponent() || self.battle_var_bit(BATTLE_VARS_SUBSTATUS5, SUBSTATUS_CANT_RUN) {
            self.animate_failed_move()?;
            self.print_but_it_failed();
            return Ok(Cmd::Next);
        }
        self.set_battle_var_bit(BATTLE_VARS_SUBSTATUS5, SUBSTATUS_CANT_RUN);
        self.animate_current_move()?;
        self.text(r::CANT_ESCAPE_NOW_TEXT);
        Ok(Cmd::Next)
    }

    /// `BattleCommand_Defrost`: thaw the user.
    pub fn battle_command_defrost(&mut self) -> Flow<Cmd> {
        if !self.battle_var_bit(BATTLE_VARS_STATUS, FRZ) {
            return Ok(Cmd::Next);
        }
        self.res_battle_var_bit(BATTLE_VARS_STATUS, FRZ);
        // Don't update the enemy's party struct in a wild battle.
        if self.is_player_turn() || self.battle_mode() != WILD_BATTLE {
            let hl = self.user_party_attr(MON_STATUS);
            self.mem.res_bit(hl, FRZ);
        }
        // RefreshBattleHuds: graphics only.
        self.text(r::WAS_DEFROSTED_TEXT);
        Ok(Cmd::Next)
    }

    /// `BattleCommand_Unused5D`.
    pub fn battle_command_unused5_d(&mut self) -> Flow<Cmd> {
        Ok(Cmd::Next)
    }

    /// `BattleCommand_CheckSafeguard`.
    pub fn battle_command_check_safeguard(&mut self) -> Flow<Cmd> {
        let hl = if self.is_player_turn() { W_ENEMY_SCREENS } else { W_PLAYER_SCREENS };
        if !self.mem.bit(hl, SCREENS_SAFEGUARD) {
            return Ok(Cmd::Next);
        }
        self.mem.wb(W_ATTACK_MISSED, 1);
        self.battle_command_move_delay()?;
        self.text(r::SAFEGUARD_PROTECT_TEXT);
        self.end_move_effect();
        Ok(Cmd::Next)
    }

    // --- time-based healing -------------------------------------------------------------------------------

    /// `BattleCommand_HealMorn`.
    pub fn battle_command_heal_morn(&mut self) -> Flow<Cmd> {
        self.time_based_heal(MORN_F)
    }

    /// `BattleCommand_HealDay`.
    pub fn battle_command_heal_day(&mut self) -> Flow<Cmd> {
        self.time_based_heal(DAY_F)
    }

    /// `BattleCommand_HealNite`.
    pub fn battle_command_heal_nite(&mut self) -> Flow<Cmd> {
        self.time_based_heal(NITE_F)
    }

    /// `BattleCommand_TimeBasedHealContinue`: time- and weather-sensitive heal.
    fn time_based_heal(&mut self, b: u8) -> Flow<Cmd> {
        let (hl, de) = if self.is_player_turn() {
            (W_BATTLE_MON_MAX_HP, W_BATTLE_MON_HP)
        } else {
            (W_ENEMY_MON_MAX_HP, W_ENEMY_MON_HP)
        };
        // Index for .Multipliers; the default restores half max HP.
        let mut c: u8 = 2;
        // Don't bother healing if HP is already full.
        if self.mem.rw(de) == self.mem.rw(hl) {
            self.animate_failed_move()?;
            self.text(r::HP_IS_FULL_TEXT);
            return Ok(Cmd::Next);
        }
        // Don't factor in time of day in link battles.
        if self.mem.rb(W_LINK_MODE) == 0 && self.mem.rb(W_TIME_OF_DAY) != b {
            c -= 1; // double
        }
        // x2 in sun, /2 in rain/sandstorm.
        let weather = self.mem.rb(W_BATTLE_WEATHER);
        if weather != 0 {
            if weather == WEATHER_SUN {
                c += 1;
            } else {
                c -= 1;
            }
        }
        let bc = match c {
            0 => self.get_eighth_max_hp(),
            1 => self.get_quarter_max_hp(),
            2 => self.get_half_max_hp(),
            _ => self.get_max_hp(),
        };
        self.animate_current_move()?;
        self.switch_turn();
        self.restore_hp(bc);
        self.switch_turn();
        self.update_user_in_party();
        self.text(r::REGAINED_HEALTH_TEXT);
        Ok(Cmd::Next)
    }
}
