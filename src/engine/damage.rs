//! Port of the damage pipeline of `engine/battle/effect_commands.asm`: hit
//! checks, damage stats and calculation, applying damage, and the commands
//! that surround them. See docs/engine.md.
//!
//! The original passes the damage inputs from `BattleCommand_DamageStats` to
//! `BattleCommand_DamageCalc` in registers `b`, `c`, `d`, `e` (the script
//! reader preserves them between commands); here they live in
//! [`Engine::damage_regs`].

#![allow(unused_imports)]

use super::battle::{Engine, Flow, PARTY_LEN, PP_MASK, percent};
use super::commands::Cmd;
use super::rom;
use super::rsym as r;
use super::sym::*;
use crate::defs::*;

/// `DAMAGE_CAP` (`MAX_DAMAGE - MIN_DAMAGE`).
const DAMAGE_CAP: u16 = 999 - 2;
const MIN_DAMAGE: u8 = 2;

impl Engine {
    // --- items ---------------------------------------------------------------

    /// `GetUserItem`: (address of the user's held item, its held effect `b`, its parameter `c`).
    pub fn get_user_item(&self) -> (u16, u8, u8) {
        let hl = if self.is_player_turn() { W_BATTLE_MON_ITEM } else { W_ENEMY_MON_ITEM };
        let (b, c) = self.get_item_held_effect(self.mem.rb(hl));
        (hl, b, c)
    }

    /// `GetOpponentItem`: like `get_user_item`, for the opponent.
    pub fn get_opponent_item(&self) -> (u16, u8, u8) {
        let hl = if self.is_player_turn() { W_ENEMY_MON_ITEM } else { W_BATTLE_MON_ITEM };
        let (b, c) = self.get_item_held_effect(self.mem.rb(hl));
        (hl, b, c)
    }

    /// `GetItemHeldEffect`: (effect `b`, parameter `c`) of item `b`. With no
    /// item the original returns `b = 0` and leaves `c` as it was; we use 0.
    pub fn get_item_held_effect(&self, item: u8) -> (u8, u8) {
        if item == 0 {
            return (0, 0);
        }
        let at = r::ITEM_ATTRIBUTES
            + usize::from(item - 1) * usize::from(ITEMATTR_STRUCT_LENGTH)
            + usize::from(ITEMATTR_EFFECT);
        (rom::byte(at), rom::byte(at + 1))
    }

    // --- battle scene, animations (memory effects only) ---------------------------

    /// `_CheckBattleScene`: carry (`true`) = battle animations are disabled.
    pub fn check_battle_scene(&self) -> bool {
        if self.mem.rb(W_LINK_MODE) == LINK_MOBILE {
            self.unported("CheckBattleScene (mobile)");
        }
        self.mem.rb(W_OPTIONS) & 1 << BATTLE_SCENE != 0
    }

    /// `PlayFXAnimID` (the animation itself is dropped).
    pub fn play_fx_anim_id(&mut self, e: u8, d: u8) {
        self.mem.wb(W_FX_ANIM_ID, e);
        self.mem.wb(W_FX_ANIM_ID + 1, d);
    }

    /// `LoadAnim` (the animation itself is dropped).
    pub fn load_anim(&mut self, a: u8) {
        self.mem.wb(W_FX_ANIM_ID, a);
    }

    /// `LoadMoveAnim`.
    pub fn load_move_anim(&mut self) {
        self.mem.wb(W_BATTLE_AFTER_ANIM, 0);
        self.mem.wb(W_FX_ANIM_ID + 1, 0);
        let a = self.battle_var(BATTLE_VARS_MOVE_ANIM);
        if a == 0 {
            return;
        }
        self.load_anim(a);
    }

    /// What `GetBattleMonBackpic` / `GetEnemyMonFrontpic` (and so
    /// `DropPlayerSub` / `DropEnemySub`) leave in memory: only the enemy's
    /// `wCurSpecies` (its `GetBaseData` also rewrites `wCurBaseData`, which
    /// nothing in the battle reads).
    fn redraw_user_pic_memory(&mut self, check_sub: bool) {
        if self.is_player_turn() {
            return;
        }
        if check_sub && self.mem.bit(W_ENEMY_SUB_STATUS4, SUBSTATUS_SUBSTITUTE) {
            return;
        }
        if self.mem.rb(W_ENEMY_MINIMIZED) != 0 {
            return;
        }
        let species = self.mem.rb(W_ENEMY_MON_SPECIES);
        self.mem.wb(W_CUR_SPECIES, species);
    }

    /// `AppearUserRaiseSub` (`_AppearUserRaiseSub`): graphics only, apart from
    /// what `BattleCommand_RaiseSubNoAnim` leaves.
    pub fn appear_user_raise_sub(&mut self) {
        let _ = self.battle_command_raise_sub_no_anim();
    }

    /// `AppearUserLowerSub` (`_AppearUserLowerSub`).
    pub fn appear_user_lower_sub(&mut self) {
        let _ = self.battle_command_lower_sub_no_anim();
    }

    /// `BattleCommand_RaiseSubNoAnim`.
    pub fn battle_command_raise_sub_no_anim(&mut self) -> Flow<Cmd> {
        self.redraw_user_pic_memory(true);
        Ok(Cmd::Next)
    }

    /// `BattleCommand_LowerSubNoAnim`.
    pub fn battle_command_lower_sub_no_anim(&mut self) -> Flow<Cmd> {
        self.redraw_user_pic_memory(false);
        Ok(Cmd::Next)
    }

    /// `BattleCommand_MoveDelay`: waits 40 frames.
    pub fn battle_command_move_delay(&mut self) -> Flow<Cmd> {
        Ok(Cmd::Next)
    }

    /// `BattleCommand_ClearText`: an empty textbox.
    pub fn battle_command_clear_text(&mut self) -> Flow<Cmd> {
        Ok(Cmd::Next)
    }

    /// `BattleCommand_SwitchTurn`.
    pub fn battle_command_switch_turn(&mut self) -> Flow<Cmd> {
        self.switch_turn();
        Ok(Cmd::Next)
    }

    /// `ResetFuryCutterCount`.
    pub fn reset_fury_cutter_count(&mut self) {
        let hl = if self.is_player_turn() { W_PLAYER_FURY_CUTTER_COUNT } else { W_ENEMY_FURY_CUTTER_COUNT };
        self.mem.wb(hl, 0);
    }

    // --- BattleCommand_CheckHit --------------------------------------------------

    /// `BattleCommand_CheckHit`.
    pub fn battle_command_check_hit(&mut self) -> Flow<Cmd> {
        // .DreamEater: z (eating the dream of a mon that isn't asleep) = miss.
        if self.battle_var(BATTLE_VARS_MOVE_EFFECT) == EFFECT_DREAM_EATER
            && self.battle_var(BATTLE_VARS_STATUS_OPP) & SLP_MASK == 0
        {
            return self.check_hit_miss();
        }

        // .Protect: nz (the opponent is protected) = miss.
        if self.battle_var_bit(BATTLE_VARS_SUBSTATUS1_OPP, SUBSTATUS_PROTECT) {
            self.text(r::PROTECTING_ITSELF_TEXT);
            return self.check_hit_miss();
        }

        // .DrainSub: z (an HP drain move on a substitute) = miss.
        if self.battle_var_bit(BATTLE_VARS_SUBSTATUS4_OPP, SUBSTATUS_SUBSTITUTE) {
            let effect = self.battle_var(BATTLE_VARS_MOVE_EFFECT);
            if effect == EFFECT_LEECH_HIT || effect == EFFECT_DREAM_EATER {
                return self.check_hit_miss();
            }
        }

        // .LockOn: nz = the move hits.
        let addr = self.battle_var_addr(BATTLE_VARS_SUBSTATUS5_OPP);
        let locked = self.mem.bit(addr, SUBSTATUS_LOCK_ON);
        self.mem.res_bit(addr, SUBSTATUS_LOCK_ON);
        if locked {
            let mut hits = true;
            if self.battle_var_bit(BATTLE_VARS_SUBSTATUS3_OPP, SUBSTATUS_FLYING) {
                let anim = self.battle_var(BATTLE_VARS_MOVE_ANIM);
                if anim == EARTHQUAKE || anim == FISSURE || anim == MAGNITUDE {
                    hits = false;
                }
            }
            if hits {
                return Ok(Cmd::Next);
            }
        }

        // .FlyDigMoves: nz = the move can't reach the opponent = miss.
        let s3 = self.battle_var(BATTLE_VARS_SUBSTATUS3_OPP) & (1 << SUBSTATUS_FLYING | 1 << SUBSTATUS_UNDERGROUND);
        if s3 != 0 {
            let anim = self.battle_var(BATTLE_VARS_MOVE_ANIM);
            let can_hit = if s3 & 1 << SUBSTATUS_FLYING != 0 {
                anim == GUST || anim == WHIRLWIND || anim == THUNDER || anim == TWISTER
            } else {
                anim == EARTHQUAKE || anim == FISSURE || anim == MAGNITUDE
            };
            if !can_hit {
                return self.check_hit_miss();
            }
        }

        // .ThunderRain: z = always hits in rain.
        if self.battle_var(BATTLE_VARS_MOVE_EFFECT) == EFFECT_THUNDER && self.mem.rb(W_BATTLE_WEATHER) == WEATHER_RAIN {
            return Ok(Cmd::Next);
        }

        // .XAccuracy
        if self.battle_var_bit(BATTLE_VARS_SUBSTATUS4, SUBSTATUS_X_ACCURACY) {
            return Ok(Cmd::Next);
        }

        // Perfect-accuracy moves
        if self.battle_var(BATTLE_VARS_MOVE_EFFECT) == EFFECT_ALWAYS_HIT {
            return Ok(Cmd::Next);
        }

        self.check_hit_stat_modifiers();

        let mut b = if self.is_player_turn() {
            self.mem.rb(W_PLAYER_MOVE_STRUCT_ACCURACY)
        } else {
            self.mem.rb(W_ENEMY_MOVE_STRUCT_ACCURACY)
        };

        // .BrightPowder
        let (_, effect, param) = self.get_opponent_item();
        if effect == HELD_BRIGHTPOWDER {
            let (sub, borrow) = b.overflowing_sub(param);
            b = if borrow { 0 } else { sub };
        }

        // .skip_brightpowder
        if b == 0xff {
            return Ok(Cmd::Next);
        }
        let a = self.battle_random("BattleCommand_CheckHit");
        if a >= b {
            return self.check_hit_miss();
        }
        Ok(Cmd::Next)
    }

    /// `BattleCommand_CheckHit.Miss`.
    fn check_hit_miss(&mut self) -> Flow<Cmd> {
        // Keep the damage value intact if we're using (Hi) Jump Kick.
        if self.battle_var(BATTLE_VARS_MOVE_EFFECT) != EFFECT_JUMP_KICK {
            self.reset_damage();
        }
        self.mem.wb(W_ATTACK_MISSED, 1);
        Ok(Cmd::Next)
    }

    /// `BattleCommand_CheckHit.StatModifiers`: scale the move's accuracy by
    /// the user's accuracy level and the target's evasion level.
    fn check_hit_stat_modifiers(&mut self) {
        let player = self.is_player_turn();
        let (hl, mut b, mut c) = if player {
            (W_PLAYER_MOVE_STRUCT_ACCURACY, self.mem.rb(W_PLAYER_ACC_LEVEL), self.mem.rb(W_ENEMY_EVA_LEVEL))
        } else {
            (W_ENEMY_MOVE_STRUCT_ACCURACY, self.mem.rb(W_ENEMY_ACC_LEVEL), self.mem.rb(W_PLAYER_EVA_LEVEL))
        };

        // If the target's evasion is not below the user's accuracy, check Foresight.
        if c >= b && self.battle_var_bit(BATTLE_VARS_SUBSTATUS1_OPP, SUBSTATUS_IDENTIFIED) {
            return;
        }

        // subtract evasion from 14
        c = (MAX_STAT_LEVEL + 1).wrapping_sub(c);
        self.mem.wb(H_MULTIPLICAND, 0);
        self.mem.wb(H_MULTIPLICAND + 1, 0);
        let acc = self.mem.rb(hl);
        self.mem.wb(H_MULTIPLICAND + 2, acc);
        let mut d = 2;
        loop {
            // .accuracy_loop
            let idx = usize::from(b.wrapping_sub(1).wrapping_shl(1));
            let row = r::ACCURACY_LEVEL_MULTIPLIERS + idx;
            self.mem.wb(H_MULTIPLIER, rom::byte(row));
            self.multiply();
            self.mem.wb(H_DIVISOR, rom::byte(row + 1));
            self.divide(4);
            // minimum accuracy is $0001
            if self.mem.rb(H_QUOTIENT + 3) | self.mem.rb(H_QUOTIENT + 2) == 0 {
                self.mem.wb(H_QUOTIENT + 2, 0);
                self.mem.wb(H_QUOTIENT + 3, 1);
            }
            // do the same thing to the target's evasion
            b = c;
            d -= 1;
            if d == 0 {
                break;
            }
        }
        // if the result is more than 2 bytes, max out at 100%
        let a = if self.mem.rb(H_QUOTIENT + 2) != 0 { 0xff } else { self.mem.rb(H_QUOTIENT + 3) };
        self.mem.wb(hl, a);
    }

    // --- EffectChance, substitute, animation commands -----------------------------

    /// `BattleCommand_EffectChance`.
    pub fn battle_command_effect_chance(&mut self) -> Flow<Cmd> {
        self.mem.wb(W_EFFECT_FAILED, 0);
        if !self.battle_var_bit(BATTLE_VARS_SUBSTATUS4_OPP, SUBSTATUS_SUBSTITUTE) {
            let chance = if self.is_player_turn() {
                self.mem.rb(W_PLAYER_MOVE_STRUCT_EFFECT_CHANCE)
            } else {
                self.mem.rb(W_ENEMY_MOVE_STRUCT_EFFECT_CHANCE)
            };
            // BUG: Moves with a 100% secondary effect chance will not trigger it in 1/256 uses
            if self.battle_random("BattleCommand_EffectChance") < chance {
                return Ok(Cmd::Next);
            }
        }
        // .failed
        self.mem.wb(W_EFFECT_FAILED, 1);
        Ok(Cmd::Next)
    }

    /// `BattleCommand_LowerSub`.
    pub fn battle_command_lower_sub(&mut self) -> Flow<Cmd> {
        if !self.battle_var_bit(BATTLE_VARS_SUBSTATUS4, SUBSTATUS_SUBSTITUTE) {
            return Ok(Cmd::Next);
        }
        let mut charge_turn = false;
        if !self.battle_var_bit(BATTLE_VARS_SUBSTATUS3, SUBSTATUS_CHARGED) {
            let effect = self.battle_var(BATTLE_VARS_MOVE_EFFECT);
            charge_turn = matches!(
                effect,
                EFFECT_RAZOR_WIND | EFFECT_SKY_ATTACK | EFFECT_SKULL_BASH | EFFECT_SOLARBEAM | EFFECT_FLY
            );
        }
        if !charge_turn {
            // .already_charged
            if self.lower_sub_rampage() {
                // z: charge turn
            } else if self.check_user_is_charging() {
                return Ok(Cmd::Next);
            }
        }
        // .charge_turn
        if self.check_battle_scene() {
            // .mimic_anims
            self.battle_command_lower_sub_no_anim()?;
            return self.battle_command_move_delay();
        }
        self.mem.wb(W_BATTLE_AFTER_ANIM, 0);
        self.mem.wb(W_FX_ANIM_ID + 1, 0);
        self.mem.wb(W_BATTLE_ANIM_PARAM, 1);
        self.load_anim(SUBSTITUTE);
        Ok(Cmd::Next)
    }

    /// `BattleCommand_LowerSub.Rampage` (z = true).
    fn lower_sub_rampage(&mut self) -> bool {
        let effect = self.battle_var(BATTLE_VARS_MOVE_EFFECT);
        if effect == EFFECT_ROLLOUT || effect == EFFECT_RAMPAGE {
            let old = self.mem.rb(W_SOMEONE_IS_RAMPAGING);
            self.mem.wb(W_SOMEONE_IS_RAMPAGING, 0);
            return old == 0;
        }
        false
    }

    /// `BattleCommand_RaiseSub`.
    pub fn battle_command_raise_sub(&mut self) -> Flow<Cmd> {
        if !self.battle_var_bit(BATTLE_VARS_SUBSTATUS4, SUBSTATUS_SUBSTITUTE) {
            return Ok(Cmd::Next);
        }
        if self.check_battle_scene() {
            return self.battle_command_raise_sub_no_anim();
        }
        self.mem.wb(W_BATTLE_AFTER_ANIM, 0);
        self.mem.wb(W_FX_ANIM_ID + 1, 0);
        self.mem.wb(W_BATTLE_ANIM_PARAM, 2);
        self.load_anim(SUBSTITUTE);
        Ok(Cmd::Next)
    }

    /// `BattleCommand_MoveAnim`.
    pub fn battle_command_move_anim(&mut self) -> Flow<Cmd> {
        self.battle_command_lower_sub()?;
        self.battle_command_move_anim_no_sub()?;
        self.battle_command_raise_sub()
    }

    /// `BattleCommand_MoveAnimNoSub`.
    pub fn battle_command_move_anim_no_sub(&mut self) -> Flow<Cmd> {
        if self.mem.rb(W_ATTACK_MISSED) != 0 {
            return self.battle_command_move_delay();
        }
        let player = self.is_player_turn();
        let (de, after) = if player {
            (W_PLAYER_ROLLOUT_COUNT, ANIM_ENEMY_DAMAGE - BATTLE_AFTERANIMS)
        } else {
            (W_ENEMY_ROLLOUT_COUNT, ANIM_PLAYER_DAMAGE - BATTLE_AFTERANIMS)
        };
        self.mem.wb(W_BATTLE_AFTER_ANIM, after as u8);
        let effect = self.battle_var(BATTLE_VARS_MOVE_EFFECT);
        if matches!(effect, EFFECT_MULTI_HIT | EFFECT_CONVERSION | EFFECT_DOUBLE_HIT | EFFECT_POISON_MULTI_HIT) {
            // .alternate_anim
            let param = (self.mem.rb(W_BATTLE_ANIM_PARAM) & 1) ^ 1;
            self.mem.wb(W_BATTLE_ANIM_PARAM, param);
            let first_hit = self.mem.rb(de) == 1;
            let anim = self.battle_var(BATTLE_VARS_MOVE_ANIM);
            if !first_hit {
                self.mem.wb(W_BATTLE_AFTER_ANIM, 0);
            }
            self.play_fx_anim_id(anim, 0);
            return Ok(Cmd::Next);
        }
        if effect != EFFECT_TRIPLE_KICK {
            self.mem.wb(W_BATTLE_ANIM_PARAM, 0);
        }
        // .triplekick
        let anim = self.battle_var(BATTLE_VARS_MOVE_ANIM);
        self.play_fx_anim_id(anim, 0);
        let anim = self.battle_var(BATTLE_VARS_MOVE_ANIM);
        if anim == FLY || anim == DIG {
            // .clear_sprite: AppearUserLowerSub
            self.appear_user_lower_sub();
        }
        Ok(Cmd::Next)
    }

    /// `BattleCommand_StatUpAnim`.
    pub fn battle_command_stat_up_anim(&mut self) -> Flow<Cmd> {
        if self.mem.rb(W_ATTACK_MISSED) != 0 {
            return self.battle_command_move_delay();
        }
        self.stat_up_down_anim(0)
    }

    /// `BattleCommand_StatDownAnim`.
    pub fn battle_command_stat_down_anim(&mut self) -> Flow<Cmd> {
        if self.mem.rb(W_ATTACK_MISSED) != 0 {
            return self.battle_command_move_delay();
        }
        let after = if self.is_player_turn() { ANIM_ENEMY_STAT_DOWN } else { ANIM_WOBBLE } - BATTLE_AFTERANIMS;
        self.stat_up_down_anim(after as u8)
    }

    /// `BattleCommand_StatUpDownAnim`.
    fn stat_up_down_anim(&mut self, after: u8) -> Flow<Cmd> {
        self.mem.wb(W_BATTLE_AFTER_ANIM, after);
        self.mem.wb(W_BATTLE_ANIM_PARAM, 0);
        let anim = self.battle_var(BATTLE_VARS_MOVE_ANIM);
        self.play_fx_anim_id(anim, 0);
        Ok(Cmd::Next)
    }

    // --- failure, text, loops -----------------------------------------------------

    /// `BattleCommand_FailureText`.
    pub fn battle_command_failure_text(&mut self) -> Flow<Cmd> {
        if self.mem.rb(W_ATTACK_MISSED) == 0 {
            return Ok(Cmd::Next);
        }
        self.get_failure_result_text();
        let anim = self.battle_var(BATTLE_VARS_MOVE_ANIM);
        if anim == FLY || anim == DIG {
            // .fly_dig
            self.res_battle_var_bit(BATTLE_VARS_SUBSTATUS3, SUBSTATUS_UNDERGROUND);
            self.res_battle_var_bit(BATTLE_VARS_SUBSTATUS3, SUBSTATUS_FLYING);
            self.appear_user_raise_sub();
            self.end_move_effect();
            return Ok(Cmd::Next);
        }
        // BUG: Beat Up may fail to raise Substitute
        let effect = self.battle_var(BATTLE_VARS_MOVE_EFFECT);
        if matches!(effect, EFFECT_MULTI_HIT | EFFECT_DOUBLE_HIT | EFFECT_POISON_MULTI_HIT) {
            self.battle_command_raise_sub()?;
        }
        self.end_move_effect();
        Ok(Cmd::Next)
    }

    /// `GetFailureResultText`.
    fn get_failure_result_text(&mut self) {
        let mut hl = r::DOESNT_AFFECT_TEXT;
        let mut de = r::DOESNT_AFFECT_TEXT;
        if self.mem.rb(W_TYPE_MODIFIER) & EFFECTIVENESS_MASK != 0 {
            if self.battle_var(BATTLE_VARS_MOVE_EFFECT) == EFFECT_FUTURE_SIGHT {
                hl = r::BUT_IT_FAILED_TEXT;
                de = r::IT_FAILED_TEXT;
            } else {
                hl = r::ATTACK_MISSED_TEXT;
                de = r::ATTACK_MISSED2_TEXT;
                if self.mem.rb(W_CRITICAL_HIT) == 0xff {
                    hl = r::UNAFFECTED_TEXT;
                }
            }
        }
        // .got_text: FailText_CheckOpponentProtect
        if self.battle_var_bit(BATTLE_VARS_SUBSTATUS1_OPP, SUBSTATUS_PROTECT) {
            hl = de;
        }
        self.text(hl);
        self.mem.wb(W_CRITICAL_HIT, 0);

        if self.battle_var(BATTLE_VARS_MOVE_EFFECT) != EFFECT_JUMP_KICK {
            return;
        }
        if self.mem.rb(W_TYPE_MODIFIER) & EFFECTIVENESS_MASK == 0 {
            return;
        }
        // Jump Kick crash: damage / 8, at least 1.
        let dmg = self.mem.rw(W_CUR_DAMAGE) >> 3;
        self.mem.ww(W_CUR_DAMAGE, dmg);
        if dmg == 0 {
            self.mem.wb(W_CUR_DAMAGE + 1, 1);
        }
        self.text(r::CRASHED_TEXT);
        self.mem.wb(W_BATTLE_ANIM_PARAM, 1);
        self.load_move_anim();
        if self.is_player_turn() {
            self.do_player_damage(true);
        } else {
            self.do_enemy_damage(true);
        }
    }

    /// `BattleCommand_BideFailText`.
    pub fn battle_command_bide_fail_text(&mut self) -> Flow<Cmd> {
        if self.mem.rb(W_ATTACK_MISSED) == 0 {
            return Ok(Cmd::Next);
        }
        if self.mem.rb(W_TYPE_MODIFIER) & EFFECTIVENESS_MASK == 0 {
            self.text(r::DOESNT_AFFECT_TEXT); // PrintDoesntAffect
        } else {
            self.text(r::BUT_IT_FAILED_TEXT); // PrintButItFailed
        }
        Ok(Cmd::Next)
    }

    /// `BattleCommand_CriticalText`.
    pub fn battle_command_critical_text(&mut self) -> Flow<Cmd> {
        let a = self.mem.rb(W_CRITICAL_HIT);
        if a != 0 {
            let text = match a {
                1 => r::CRITICAL_HIT_TEXT,
                2 => r::ONE_HIT_KO_TEXT,
                _ => self.unported("BattleCommand_CriticalText (wCriticalHit > 2)"),
            };
            self.text(text);
            self.mem.wb(W_CRITICAL_HIT, 0);
        }
        // .wait: 20 frames
        Ok(Cmd::Next)
    }

    /// `BattleCommand_StartLoop`.
    pub fn battle_command_start_loop(&mut self) -> Flow<Cmd> {
        let hl = if self.is_player_turn() { W_PLAYER_ROLLOUT_COUNT } else { W_ENEMY_ROLLOUT_COUNT };
        self.mem.wb(hl, 0);
        Ok(Cmd::Next)
    }

    /// `BattleCommand_SuperEffectiveLoopText`.
    pub fn battle_command_super_effective_loop_text(&mut self) -> Flow<Cmd> {
        if self.battle_var_bit(BATTLE_VARS_SUBSTATUS3, SUBSTATUS_IN_LOOP) {
            return Ok(Cmd::Next);
        }
        self.battle_command_super_effective_text()
    }

    /// `BattleCommand_SuperEffectiveText`.
    pub fn battle_command_super_effective_text(&mut self) -> Flow<Cmd> {
        let m = self.mem.rb(W_TYPE_MODIFIER) & EFFECTIVENESS_MASK;
        if m == EFFECTIVE {
            return Ok(Cmd::Next);
        }
        self.text(if m > EFFECTIVE { r::SUPER_EFFECTIVE_TEXT } else { r::NOT_VERY_EFFECTIVE_TEXT });
        Ok(Cmd::Next)
    }

    /// `BattleCommand_EndLoop`: loop back to `critical`.
    pub fn battle_command_end_loop(&mut self) -> Flow<Cmd> {
        let (de, bc) = if self.is_player_turn() {
            (W_PLAYER_ROLLOUT_COUNT, W_PLAYER_DAMAGE_TAKEN)
        } else {
            (W_ENEMY_ROLLOUT_COUNT, W_ENEMY_DAMAGE_TAKEN)
        };
        let s3 = self.battle_var_addr(BATTLE_VARS_SUBSTATUS3);
        let hits: u8;
        if self.mem.bit(s3, SUBSTATUS_IN_LOOP) {
            // .in_loop
            let n = self.mem.rb(de).wrapping_sub(1);
            self.mem.wb(de, n);
            if n != 0 {
                return self.end_loop_back_to_critical();
            }
            return self.end_loop_done(bc);
        }
        self.mem.set_bit(s3, SUBSTATUS_IN_LOOP);
        let effect = self.battle_var(BATTLE_VARS_MOVE_EFFECT);
        if effect == EFFECT_POISON_MULTI_HIT || effect == EFFECT_DOUBLE_HIT {
            // .twineedle / double hit
            hits = 1;
        } else if effect == EFFECT_BEAT_UP {
            let count = if self.is_player_turn() {
                let n = self.mem.rb(W_PARTY_COUNT);
                if n == 1 { None } else { Some(n - 1) }
            } else if self.mem.rb(W_BATTLE_MODE) == WILD_BATTLE {
                None
            } else {
                let n = self.mem.rb(W_OT_PARTY_COUNT);
                if n == 1 { None } else { Some(n - 1) }
            };
            match count {
                Some(n) => hits = n,
                None => {
                    // .only_one_beatup
                    // BUG: Beat Up works incorrectly with only one Pokemon in the party
                    self.res_battle_var_bit(BATTLE_VARS_SUBSTATUS3, SUBSTATUS_IN_LOOP);
                    self.battle_command_beat_up_fail_text()?;
                    self.end_move_effect();
                    return Ok(Cmd::Next);
                }
            }
        } else if effect == EFFECT_TRIPLE_KICK {
            let a = loop {
                let a = self.battle_random("BattleCommand_EndLoop") & 3;
                if a != 0 {
                    break a;
                }
            };
            let a = a - 1;
            if a == 0 {
                self.mem.wb(bc, 1);
                return self.end_loop_done(bc);
            }
            hits = a;
        } else {
            // .not_triple_kick
            let mut a = self.battle_random("BattleCommand_EndLoop") & 3;
            if a >= 2 {
                a = self.battle_random("BattleCommand_EndLoop") & 3;
            }
            hits = a + 1;
        }
        // .double_hit
        self.mem.wb(de, hits);
        self.mem.wb(bc, hits.wrapping_add(1));
        self.end_loop_back_to_critical()
    }

    /// `BattleCommand_EndLoop.done_loop`.
    fn end_loop_done(&mut self, bc: u16) -> Flow<Cmd> {
        self.res_battle_var_bit(BATTLE_VARS_SUBSTATUS3, SUBSTATUS_IN_LOOP);
        let text = if self.is_player_turn() { r::PLAYER_HIT_TIMES_TEXT } else { r::ENEMY_HIT_TIMES_TEXT };
        if self.battle_var(BATTLE_VARS_MOVE_EFFECT) != EFFECT_BEAT_UP {
            self.text(text);
        }
        self.mem.wb(bc, 0);
        Ok(Cmd::Next)
    }

    /// `BattleCommand_EndLoop.loop_back_to_critical`: scan the script backwards
    /// for `critical` and resume there.
    fn end_loop_back_to_critical(&mut self) -> Flow<Cmd> {
        let mut hl = self.mem.rw_le(W_BATTLE_SCRIPT_BUFFER_ADDRESS);
        loop {
            let a = self.mem.rb(hl);
            hl = hl.wrapping_sub(1);
            if a == CRITICAL_COMMAND {
                break;
            }
        }
        self.mem.ww_le(W_BATTLE_SCRIPT_BUFFER_ADDRESS, hl.wrapping_add(1));
        Ok(Cmd::Next)
    }

    // --- ApplyDamage ----------------------------------------------------------------

    /// `BattleCommand_FalseSwipe` (carry = the damage was cut down to leave 1 HP).
    pub fn false_swipe(&mut self) -> bool {
        let hl = if self.is_player_turn() { W_ENEMY_MON_HP } else { W_BATTLE_MON_HP };
        // CompareBytes(de = wCurDamage, hl = HP, 2): carry if damage < HP.
        let damage = self.mem.rw(W_CUR_DAMAGE);
        let hp = self.mem.rw(hl);
        if damage < hp {
            return false;
        }
        self.mem.wb(W_CUR_DAMAGE, self.mem.rb(hl));
        let lo = self.mem.rb(hl + 1).wrapping_sub(1);
        self.mem.wb(W_CUR_DAMAGE + 1, lo);
        if lo == 0xff {
            let hi = self.mem.rb(W_CUR_DAMAGE).wrapping_sub(1);
            self.mem.wb(W_CUR_DAMAGE, hi);
        }
        // .okay
        if self.mem.rb(W_CRITICAL_HIT) == 2 {
            self.mem.wb(W_CRITICAL_HIT, 0);
        }
        true
    }

    /// `BattleCommand_FalseSwipe` as a script command.
    pub fn battle_command_false_swipe(&mut self) -> Flow<Cmd> {
        self.false_swipe();
        Ok(Cmd::Next)
    }

    /// `BattleCommand_ApplyDamage`.
    pub fn battle_command_apply_damage(&mut self) -> Flow<Cmd> {
        let b;
        if self.battle_var_bit(BATTLE_VARS_SUBSTATUS1_OPP, SUBSTATUS_ENDURE) {
            b = if self.false_swipe() { 1 } else { 0 };
        } else {
            // .focus_band
            let (_, effect, chance) = self.get_opponent_item();
            if effect != HELD_FOCUS_BAND {
                b = 0;
            } else if self.battle_random("BattleCommand_ApplyDamage") >= chance {
                b = 0;
            } else {
                b = if self.false_swipe() { 2 } else { 0 };
            }
        }
        // .damage
        self.apply_damage_update_damage_taken();
        if self.is_player_turn() {
            self.do_enemy_damage(false);
        } else {
            self.do_player_damage(false);
        }
        match b {
            0 => {}
            1 => self.text(r::ENDURED_TEXT),
            _ => {
                let (hl, _, _) = self.get_opponent_item();
                let item = self.mem.rb(hl);
                self.mem.wb(W_NAMED_OBJECT_INDEX, item);
                self.get_item_name();
                self.text(r::HUNG_ON_TEXT);
            }
        }
        Ok(Cmd::Next)
    }

    /// `BattleCommand_ApplyDamage.update_damage_taken`.
    fn apply_damage_update_damage_taken(&mut self) {
        if self.battle_var_bit(BATTLE_VARS_SUBSTATUS4_OPP, SUBSTATUS_SUBSTITUTE) {
            return;
        }
        let de = if self.is_player_turn() { W_ENEMY_DAMAGE_TAKEN } else { W_PLAYER_DAMAGE_TAKEN };
        let total = u32::from(self.mem.rw(de)) + u32::from(self.mem.rw(W_CUR_DAMAGE));
        // The original adds low then high with carry; a carry out of the high byte caps at $ffff.
        if total > 0xffff {
            self.mem.wb(de, 0xff);
            self.mem.wb(de + 1, 0xff);
        } else {
            self.mem.ww(de, total as u16);
        }
    }

    /// `DoEnemyDamage`: `c` = ignore the substitute.
    pub fn do_enemy_damage(&mut self, c: bool) {
        self.do_damage(false, c);
    }

    /// `DoPlayerDamage`: `c` = ignore the substitute.
    pub fn do_player_damage(&mut self, c: bool) {
        self.do_damage(true, c);
    }

    /// Shared body of `DoPlayerDamage` / `DoEnemyDamage` (they differ only in
    /// the mon they hit and in which of `wHPBuffer3` they write when).
    fn do_damage(&mut self, player: bool, ignore_substitute: bool) {
        let (hp, max_hp, sub4) = if player {
            (W_BATTLE_MON_HP, W_BATTLE_MON_MAX_HP, W_PLAYER_SUB_STATUS4)
        } else {
            (W_ENEMY_MON_HP, W_ENEMY_MON_MAX_HP, W_ENEMY_SUB_STATUS4)
        };
        if self.mem.rw(W_CUR_DAMAGE) == 0 {
            // .did_no_damage: RefreshBattleHuds
            return;
        }
        if !ignore_substitute && self.mem.bit(sub4, SUBSTATUS_SUBSTITUTE) {
            self.do_substitute_damage();
            return;
        }
        // .ignore_substitute: subtract wCurDamage from HP, keeping the
        // original HP in little endian wHPBuffer2.
        let damage = self.mem.rw(W_CUR_DAMAGE);
        let old = self.mem.rw(hp);
        self.mem.wb(W_HP_BUFFER2, old as u8);
        self.mem.wb(W_HP_BUFFER2 + 1, (old >> 8) as u8);
        let (new, borrow) = old.overflowing_sub(damage);
        self.mem.ww(hp, new);
        if player {
            self.mem.wb(W_HP_BUFFER3, new as u8);
            self.mem.wb(W_HP_BUFFER3 + 1, (new >> 8) as u8);
        }
        if borrow {
            self.mem.ww(W_CUR_DAMAGE, old);
            self.mem.ww(hp, 0);
            if player {
                self.mem.wb(W_HP_BUFFER3, 0);
                self.mem.wb(W_HP_BUFFER3 + 1, 0);
            }
        }
        // .no_underflow
        let max = self.mem.rw(max_hp);
        self.mem.wb(W_HP_BUFFER1 + 1, (max >> 8) as u8);
        self.mem.wb(W_HP_BUFFER1, max as u8);
        if !player {
            let cur = self.mem.rw(hp);
            self.mem.wb(W_HP_BUFFER3 + 1, (cur >> 8) as u8);
            self.mem.wb(W_HP_BUFFER3, cur as u8);
        }
        self.mem.wb(W_WHICH_HP_BAR, u8::from(player));
        // AnimateHPBar, RefreshBattleHuds
    }

    /// `DoSubstituteDamage`.
    fn do_substitute_damage(&mut self) {
        self.text(r::SUB_TOOK_DAMAGE_TEXT);
        let de = if self.is_player_turn() { W_ENEMY_SUBSTITUTE_HP } else { W_PLAYER_SUBSTITUTE_HP };
        let mut broke = self.mem.rb(W_CUR_DAMAGE) != 0;
        if !broke {
            let (new, borrow) = self.mem.rb(de).overflowing_sub(self.mem.rb(W_CUR_DAMAGE + 1));
            self.mem.wb(de, new);
            broke = new == 0 || borrow;
        }
        if broke {
            // .broke
            self.res_battle_var_bit(BATTLE_VARS_SUBSTATUS4_OPP, SUBSTATUS_SUBSTITUTE);
            self.text(r::SUB_FADED_TEXT);
            self.switch_turn();
            let _ = self.battle_command_lower_sub_no_anim();
            if self.battle_var(BATTLE_VARS_SUBSTATUS3) & (1 << SUBSTATUS_FLYING | 1 << SUBSTATUS_UNDERGROUND) == 0 {
                self.appear_user_lower_sub();
            }
            self.switch_turn();
            let effect = self.battle_var(BATTLE_VARS_MOVE_EFFECT);
            if !matches!(
                effect,
                EFFECT_MULTI_HIT | EFFECT_DOUBLE_HIT | EFFECT_POISON_MULTI_HIT | EFFECT_TRIPLE_KICK | EFFECT_BEAT_UP
            ) {
                self.set_battle_var(BATTLE_VARS_MOVE_EFFECT, 0);
            }
            // RefreshBattleHuds
        }
        // .done
        self.reset_damage();
    }

    /// `BattleCommand_CheckFaint`.
    pub fn battle_command_check_faint(&mut self) -> Flow<Cmd> {
        let hl = if self.is_player_turn() { W_ENEMY_MON_HP } else { W_BATTLE_MON_HP };
        if self.mem.rw(hl) != 0 {
            return Ok(Cmd::Next);
        }
        if self.battle_var_bit(BATTLE_VARS_SUBSTATUS5_OPP, SUBSTATUS_DESTINY_BOND) {
            self.text(r::TOOK_DOWN_WITH_IT_TEXT);
            // The user faints too.
            let (max_hp, which) = if self.is_player_turn() { (W_BATTLE_MON_MAX_HP, 1) } else { (W_ENEMY_MON_MAX_HP, 0) };
            self.mem.wb(W_WHICH_HP_BAR, which);
            let user_hp = max_hp - 2;
            self.mem.wb(W_HP_BUFFER1, self.mem.rb(max_hp + 1));
            self.mem.wb(W_HP_BUFFER1 + 1, self.mem.rb(max_hp));
            self.mem.wb(W_HP_BUFFER2, self.mem.rb(user_hp + 1));
            self.mem.wb(user_hp + 1, 0);
            self.mem.wb(W_HP_BUFFER2 + 1, self.mem.rb(user_hp));
            self.mem.wb(user_hp, 0);
            self.mem.wb(W_HP_BUFFER3, 0);
            self.mem.wb(W_HP_BUFFER3 + 1, 0);
            // AnimateHPBar, RefreshBattleHuds
            self.battle_command_switch_turn()?;
            self.mem.wb(W_BATTLE_AFTER_ANIM, 0);
            self.mem.wb(W_FX_ANIM_ID + 1, 0);
            self.mem.wb(W_BATTLE_ANIM_PARAM, 1);
            self.load_anim(DESTINY_BOND);
            self.battle_command_switch_turn()?;
        } else {
            // .no_dbond
            let effect = self.battle_var(BATTLE_VARS_MOVE_EFFECT);
            if matches!(
                effect,
                EFFECT_MULTI_HIT | EFFECT_DOUBLE_HIT | EFFECT_POISON_MULTI_HIT | EFFECT_TRIPLE_KICK | EFFECT_BEAT_UP
            ) {
                self.battle_command_raise_sub()?;
            }
        }
        // .finish
        self.end_move_effect();
        Ok(Cmd::Next)
    }

    /// `BattleCommand_BuildOpponentRage`.
    pub fn battle_command_build_opponent_rage(&mut self) -> Flow<Cmd> {
        if self.mem.rb(W_ATTACK_MISSED) != 0 {
            return Ok(Cmd::Next);
        }
        if !self.battle_var_bit(BATTLE_VARS_SUBSTATUS4_OPP, SUBSTATUS_RAGE) {
            return Ok(Cmd::Next);
        }
        let de = if self.is_player_turn() { W_ENEMY_RAGE_COUNTER } else { W_PLAYER_RAGE_COUNTER };
        let a = self.mem.rb(de).wrapping_add(1);
        if a == 0 {
            return Ok(Cmd::Next);
        }
        self.mem.wb(de, a);
        self.switch_turn();
        self.text(r::RAGE_BUILDING_TEXT);
        self.switch_turn();
        Ok(Cmd::Next)
    }

    /// `BattleCommand_RageDamage`.
    pub fn battle_command_rage_damage(&mut self) -> Flow<Cmd> {
        let bc = self.mem.rw(W_CUR_DAMAGE);
        let mut hl = bc;
        let mut a = if self.is_player_turn() { self.mem.rb(W_PLAYER_RAGE_COUNTER) } else { self.mem.rb(W_ENEMY_RAGE_COUNTER) };
        while a != 0 {
            a -= 1;
            let (sum, carry) = hl.overflowing_add(bc);
            hl = sum;
            if carry {
                hl = 0xffff;
                break;
            }
        }
        self.mem.ww(W_CUR_DAMAGE, hl);
        Ok(Cmd::Next)
    }

    /// `BattleCommand_HeldFlinch` (King's Rock).
    pub fn battle_command_held_flinch(&mut self) -> Flow<Cmd> {
        if self.mem.rb(W_ATTACK_MISSED) != 0 {
            return Ok(Cmd::Next);
        }
        let (_, effect, chance) = self.get_user_item();
        if effect != HELD_FLINCH {
            return Ok(Cmd::Next);
        }
        if self.battle_var_bit(BATTLE_VARS_SUBSTATUS4_OPP, SUBSTATUS_SUBSTITUTE) {
            return Ok(Cmd::Next);
        }
        if self.battle_random("BattleCommand_HeldFlinch") >= chance {
            return Ok(Cmd::Next);
        }
        // EndRechargeOpp
        self.res_battle_var_bit(BATTLE_VARS_SUBSTATUS4_OPP, SUBSTATUS_RECHARGE);
        self.set_battle_var_bit(BATTLE_VARS_SUBSTATUS3_OPP, SUBSTATUS_FLINCHED);
        Ok(Cmd::Next)
    }

    // --- DamageStats ---------------------------------------------------------------

    /// `BattleCommand_DamageStats`.
    pub fn battle_command_damage_stats(&mut self) -> Flow<Cmd> {
        if self.is_player_turn() {
            self.player_attack_damage();
        } else {
            self.enemy_attack_damage();
        }
        Ok(Cmd::Next)
    }

    /// `PlayerAttackDamage`: leaves move power `d`, user level `e`, defense
    /// `c` and attack `b` in [`Engine::damage_regs`]. Returns nz (`true`)
    /// unless the move has no power.
    pub fn player_attack_damage(&mut self) -> bool {
        self.attack_damage(true)
    }

    /// `EnemyAttackDamage`; see [`Engine::player_attack_damage`].
    pub fn enemy_attack_damage(&mut self) -> bool {
        self.attack_damage(false)
    }

    /// Shared body of `PlayerAttackDamage` / `EnemyAttackDamage` (mirror images).
    fn attack_damage(&mut self, player: bool) -> bool {
        self.reset_damage();
        // (power, type, defender's defense, spec def, defender's screens, attacker's attack, spec atk,
        //  defender's unboosted defense, unboosted spec def, attacker's unboosted attack, unboosted spec atk,
        //  attacker's level)
        let (power, mtype, def, spdef, screens, atk, spatk, ub_def, ub_spdef, ub_atk, ub_spatk, level) = if player {
            (W_PLAYER_MOVE_STRUCT_POWER, W_PLAYER_MOVE_STRUCT_TYPE, W_ENEMY_MON_DEFENSE, W_ENEMY_MON_SPCL_DEF,
             W_ENEMY_SCREENS, W_BATTLE_MON_ATTACK, W_BATTLE_MON_SPCL_ATK, W_ENEMY_DEFENSE, W_ENEMY_SP_DEF,
             W_PLAYER_ATTACK, W_PLAYER_SP_ATK, W_BATTLE_MON_LEVEL)
        } else {
            (W_ENEMY_MOVE_STRUCT_POWER, W_ENEMY_MOVE_STRUCT_TYPE, W_BATTLE_MON_DEFENSE, W_BATTLE_MON_SPCL_DEF,
             W_PLAYER_SCREENS, W_ENEMY_MON_ATTACK, W_ENEMY_MON_SPCL_ATK, W_PLAYER_DEFENSE, W_PLAYER_SP_DEF,
             W_ENEMY_ATTACK, W_ENEMY_SP_ATK, W_ENEMY_MON_LEVEL)
        };

        // No damage dealt with 0 power.
        let d = self.mem.rb(power);
        self.damage_regs[2] = d;
        if d == 0 {
            return false;
        }

        let hl: u16;
        let mut bc: u16;
        if self.mem.rb(mtype) < SPECIAL {
            // physical
            bc = self.mem.rw(def);
            if self.mem.bit(screens, SCREENS_REFLECT) {
                bc <<= 1;
            }
            if self.check_damage_stats_critical(player) {
                // .thickclub
                hl = self.thick_club_boost(atk);
            } else {
                bc = self.mem.rw(ub_def);
                hl = self.thick_club_boost(ub_atk);
            }
        } else {
            // .special
            bc = self.mem.rw(spdef);
            if self.mem.bit(screens, SCREENS_LIGHT_SCREEN) {
                bc <<= 1;
            }
            if self.check_damage_stats_critical(player) {
                hl = self.light_ball_boost(spatk);
            } else {
                bc = self.mem.rw(ub_spdef);
                hl = self.light_ball_boost(ub_spatk);
            }
        }

        // .done
        let (b, c) = self.truncate_hl_bc(hl, bc);
        let e = self.mem.rb(level);
        let (b, c) = self.ditto_metal_powder(b, c);
        self.damage_regs = [b, c, d, e];
        true
    }

    /// `TruncateHL_BC`: truncate 16-bit `hl` and `bc` to 8-bit values `b` (= `l`)
    /// and `c`; returns `(b, c)`.
    pub fn truncate_hl_bc(&mut self, mut hl: u16, mut bc: u16) -> (u8, u8) {
        loop {
            if (hl >> 8) | (bc >> 8) == 0 {
                break;
            }
            bc >>= 2;
            if bc == 0 {
                bc = 1;
            }
            hl >>= 2;
            if hl == 0 {
                hl = 1;
            }
            // .finish
            // BUG: Reflect and Light Screen can make (Special) Defense wrap around above 1024
            if self.mem.rb(W_LINK_MODE) == LINK_COLOSSEUM {
                break;
            }
            if (hl >> 8) | (bc >> 8) == 0 {
                break;
            }
        }
        (hl as u8, bc as u8)
    }

    /// `CheckDamageStatsCritical`: carry (`true`) if boosted stats should be used.
    fn check_damage_stats_critical(&self, player: bool) -> bool {
        if self.mem.rb(W_CRITICAL_HIT) == 0 {
            return true;
        }
        let (mtype, sp_atk, sp_def, atk, def) = if player {
            (W_PLAYER_MOVE_STRUCT_TYPE, W_PLAYER_S_ATK_LEVEL, W_ENEMY_S_DEF_LEVEL, W_PLAYER_ATK_LEVEL, W_ENEMY_DEF_LEVEL)
        } else {
            (W_ENEMY_MOVE_STRUCT_TYPE, W_ENEMY_S_ATK_LEVEL, W_PLAYER_S_DEF_LEVEL, W_ENEMY_ATK_LEVEL, W_PLAYER_DEF_LEVEL)
        };
        let (a_level, d_level) = if self.mem.rb(mtype) < SPECIAL {
            (self.mem.rb(atk), self.mem.rb(def))
        } else {
            (self.mem.rb(sp_atk), self.mem.rb(sp_def))
        };
        d_level < a_level
    }

    /// `ThickClubBoost`: the stat at `hl`, doubled for Cubone/Marowak with a Thick Club.
    fn thick_club_boost(&self, hl: u16) -> u16 {
        self.species_item_boost(hl, CUBONE, MAROWAK, THICK_CLUB)
    }

    /// `LightBallBoost`: the stat at `hl`, doubled for Pikachu with a Light Ball.
    fn light_ball_boost(&self, hl: u16) -> u16 {
        self.species_item_boost(hl, PIKACHU, PIKACHU, LIGHT_BALL)
    }

    /// `SpeciesItemBoost`.
    fn species_item_boost(&self, stat: u16, b: u8, c: u8, d: u8) -> u16 {
        let value = self.mem.rw(stat);
        let species = if self.is_player_turn() {
            self.mem.rb(self.battle_party_attr(MON_SPECIES))
        } else {
            self.mem.rb(W_TEMP_ENEMY_MON_SPECIES)
        };
        if species != b && species != c {
            return value;
        }
        let (item, _, _) = self.get_user_item();
        if self.mem.rb(item) != d {
            return value;
        }
        // Double the stat
        // BUG: Thick Club and Light Ball can make (Special) Attack wrap around above 1024
        value << 1
    }

    /// `DittoMetalPowder`: takes and returns `(b, c)` = (attack, defense).
    fn ditto_metal_powder(&self, mut b: u8, mut c: u8) -> (u8, u8) {
        let species = if self.is_player_turn() {
            self.mem.rb(W_TEMP_ENEMY_MON_SPECIES)
        } else {
            self.mem.rb(self.battle_party_attr(MON_SPECIES))
        };
        if species != DITTO {
            return (b, c);
        }
        let (item, _, _) = self.get_opponent_item();
        if self.mem.rb(item) != METAL_POWDER {
            return (b, c);
        }
        // BUG: Metal Powder can increase damage taken with boosted (Special) Defense
        let (sum, carry) = (c >> 1).overflowing_add(c);
        c = sum;
        if !carry {
            return (b, c);
        }
        b >>= 1;
        if b == 0 {
            b = 1;
        }
        c = 0x80 | c >> 1;
        (b, c)
    }

    /// `BattleCommand_ClearMissDamage`.
    pub fn battle_command_clear_miss_damage(&mut self) -> Flow<Cmd> {
        if self.mem.rb(W_ATTACK_MISSED) != 0 {
            self.reset_damage();
        }
        Ok(Cmd::Next)
    }

    /// `HitSelfInConfusion`: sets up confusion damage for `BattleCommand_DamageCalc`
    /// (power 40, the user's level, its attack and defense).
    pub fn hit_self_in_confusion(&mut self) {
        self.reset_damage();
        let (def, screens, level) = if self.is_player_turn() {
            (W_BATTLE_MON_DEFENSE, W_PLAYER_SCREENS, self.mem.rb(W_BATTLE_MON_LEVEL))
        } else {
            (W_ENEMY_MON_DEFENSE, W_ENEMY_SCREENS, self.mem.rb(W_ENEMY_MON_LEVEL))
        };
        let mut bc = self.mem.rw(def);
        if self.mem.bit(screens, SCREENS_REFLECT) {
            bc <<= 1;
        }
        // .mimic_screen: hl = the attack stat, three bytes below the defense's low byte.
        let hl = self.mem.rw(def - 2);
        let (b, c) = self.truncate_hl_bc(hl, bc);
        self.damage_regs = [b, c, 40, level];
    }

    // --- DamageCalc ----------------------------------------------------------------

    /// `BattleCommand_DamageCalc`: a damage value for move power `d`, level `e`,
    /// defense `c` and attack `b` (from [`Engine::damage_regs`]).
    /// BUG: Confusion damage is affected by type-boosting items and Explosion/Self-Destruct doubling.
    pub fn battle_command_damage_calc(&mut self) -> Flow<Cmd> {
        let [b, mut c, d, e] = self.damage_regs;
        let effect = self.battle_var(BATTLE_VARS_MOVE_EFFECT);

        // Selfdestruct and Explosion halve defense.
        if effect == EFFECT_SELFDESTRUCT {
            c >>= 1;
            if c == 0 {
                c += 1;
            }
        }

        // Variable-hit moves and Conversion can have a power of 0.
        if effect != EFFECT_MULTI_HIT && effect != EFFECT_CONVERSION {
            // No damage if move power is 0.
            if d == 0 {
                return Ok(Cmd::Next);
            }
        }

        // Minimum defense value is 1.
        if c == 0 {
            c = 1;
        }

        self.mem.wb(H_DIVIDEND, 0);
        self.mem.wb(H_DIVIDEND + 1, 0);
        self.mem.wb(H_DIVIDEND + 2, 0);

        // Level * 2
        let (lvl2, overflow) = e.overflowing_add(e);
        if overflow {
            self.mem.wb(H_DIVIDEND + 2, 1);
        }
        self.mem.wb(H_DIVIDEND + 3, lvl2);

        // / 5
        self.mem.wb(H_DIVISOR, 5);
        self.divide(4);

        // + 2
        let q = self.mem.rb(H_QUOTIENT + 3).wrapping_add(2);
        self.mem.wb(H_QUOTIENT + 3, q);

        // * bp
        self.mem.wb(H_MULTIPLIER, d);
        self.multiply();

        // * Attack
        self.mem.wb(H_MULTIPLIER, b);
        self.multiply();

        // / Defense
        self.mem.wb(H_DIVISOR, c);
        self.divide(4);

        // / 50
        self.mem.wb(H_DIVISOR, 50);
        self.divide(4);

        // Item boosts
        let (_, item_effect, item_param) = self.get_user_item();
        if item_effect != 0 {
            let mut hl = r::TYPE_BOOST_ITEMS;
            loop {
                let a = rom::byte(hl);
                hl += 1;
                if a == 0xff {
                    break;
                }
                let boost_type = rom::byte(hl);
                hl += 1;
                if a != item_effect {
                    continue;
                }
                // Type
                if self.battle_var(BATTLE_VARS_MOVE_TYPE) == boost_type {
                    // * 100 + item effect amount
                    self.mem.wb(H_MULTIPLIER, item_param.wrapping_add(100));
                    self.multiply();
                    // / 100
                    self.mem.wb(H_DIVISOR, 100);
                    self.divide(4);
                }
                break;
            }
        }

        // .DoneItem: Critical hits
        if self.mem.rb(W_CRITICAL_HIT) != 0 {
            // x2
            let q3 = self.mem.rb(H_QUOTIENT + 3);
            self.mem.wb(H_QUOTIENT + 3, q3.wrapping_shl(1));
            let q2 = self.mem.rb(H_QUOTIENT + 2);
            let carry_in = q3 & 0x80 != 0;
            self.mem.wb(H_QUOTIENT + 2, q2.wrapping_shl(1) | u8::from(carry_in));
            // Cap at $ffff.
            if q2 & 0x80 != 0 {
                self.mem.wb(H_QUOTIENT + 2, 0xff);
                self.mem.wb(H_QUOTIENT + 3, 0xff);
            }
        }

        // Update wCurDamage. Max 999 (capped at 997, then add 2).
        let cap = self.damage_calc_update();
        let hl = W_CUR_DAMAGE;
        if cap {
            self.mem.ww(hl, DAMAGE_CAP);
        }
        // Add back MIN_DAMAGE (capping at 999).
        let (lo, carry) = self.mem.rb(hl + 1).overflowing_add(MIN_DAMAGE);
        self.mem.wb(hl + 1, lo);
        if carry {
            let hi = self.mem.rb(hl).wrapping_add(1);
            self.mem.wb(hl, hi);
        }
        Ok(Cmd::Next)
    }

    /// The "update wCurDamage" part of `BattleCommand_DamageCalc`, up to
    /// `.Cap` / `.dont_cap_3`: adds the quotient to `wCurDamage` and returns
    /// whether the result must be capped.
    fn damage_calc_update(&mut self) -> bool {
        let hl = W_CUR_DAMAGE;
        let b = self.mem.rb(hl);
        let (q3, carry) = self.mem.rb(H_QUOTIENT + 3).overflowing_add(b);
        self.mem.wb(H_QUOTIENT + 3, q3);
        if carry {
            let q2 = self.mem.rb(H_QUOTIENT + 2).wrapping_add(1);
            self.mem.wb(H_QUOTIENT + 2, q2);
            if q2 == 0 {
                return true;
            }
        }
        // .dont_cap_1
        if self.mem.rb(H_QUOTIENT + 1) != 0 {
            return true;
        }
        let q2 = self.mem.rb(H_QUOTIENT + 2);
        const HI: u8 = ((DAMAGE_CAP + 1) >> 8) as u8;
        const LO: u8 = (DAMAGE_CAP + 1) as u8;
        if q2 >= HI {
            if q2 >= HI + 1 {
                return true;
            }
            if self.mem.rb(H_QUOTIENT + 3) >= LO {
                return true;
            }
        }
        // .dont_cap_2
        let q3 = self.mem.rb(H_QUOTIENT + 3);
        let (lo, c1) = self.mem.rb(hl + 1).overflowing_add(q3);
        self.mem.wb(hl + 1, lo);
        let (hi, c2a) = self.mem.rb(hl).overflowing_add(self.mem.rb(H_QUOTIENT + 2));
        let (hi, c2b) = hi.overflowing_add(u8::from(c1));
        self.mem.wb(hl, hi);
        if c2a || c2b {
            return true;
        }
        if hi >= HI {
            if hi >= HI + 1 {
                return true;
            }
            if self.mem.rb(hl + 1) >= LO {
                return true;
            }
        }
        false
    }

    // --- ConstantDamage ---------------------------------------------------------------

    /// `BattleCommand_ConstantDamage`.
    pub fn battle_command_constant_damage(&mut self) -> Flow<Cmd> {
        let level_addr = if self.is_player_turn() { W_BATTLE_MON_LEVEL } else { W_ENEMY_MON_LEVEL };
        let effect = self.battle_var(BATTLE_VARS_MOVE_EFFECT);
        let mut b = self.mem.rb(level_addr);
        let a: u8;
        if effect == EFFECT_LEVEL_DAMAGE {
            a = 0;
        } else if effect == EFFECT_PSYWAVE {
            b = (b >> 1).wrapping_add(b);
            let r = loop {
                let r = self.battle_random("BattleCommand_ConstantDamage");
                if r != 0 && r < b {
                    break r;
                }
            };
            b = r;
            a = 0;
        } else if effect == EFFECT_SUPER_FANG {
            let hl = if self.is_player_turn() { W_ENEMY_MON_HP } else { W_BATTLE_MON_HP };
            let hp = self.mem.rw(hl) >> 1;
            let (hi, lo) = ((hp >> 8) as u8, hp as u8);
            if hi != 0 || lo != 0 {
                a = hi;
                b = lo;
            } else {
                a = 0;
                b = 1;
            }
        } else if effect == EFFECT_REVERSAL {
            return self.constant_damage_reversal();
        } else {
            b = self.battle_var(BATTLE_VARS_MOVE_POWER);
            a = 0;
        }
        // .got_power
        self.mem.wb(W_CUR_DAMAGE, a);
        self.mem.wb(W_CUR_DAMAGE + 1, b);
        Ok(Cmd::Next)
    }

    /// `BattleCommand_ConstantDamage.reversal` (Flail / Reversal).
    fn constant_damage_reversal(&mut self) -> Flow<Cmd> {
        let hl = if self.is_player_turn() { W_BATTLE_MON_HP } else { W_ENEMY_MON_HP };
        self.mem.wb(H_DIVIDEND, 0);
        self.mem.wb(H_MULTIPLICAND, 0);
        let hp_hi = self.mem.rb(hl);
        let hp_lo = self.mem.rb(hl + 1);
        self.mem.wb(H_MULTIPLICAND + 1, hp_hi);
        self.mem.wb(H_MULTIPLICAND + 2, hp_lo);
        self.mem.wb(H_MULTIPLIER, 48);
        self.multiply();
        let max_hi = self.mem.rb(hl + 2);
        let max_lo = self.mem.rb(hl + 3);
        self.mem.wb(H_DIVISOR, max_lo);
        if max_hi != 0 {
            // ldh a, [hProduct + 4] (= hDivisor, just written)
            let max = (u16::from(max_hi) << 8 | u16::from(self.mem.rb(H_PRODUCT + 4))) >> 2;
            self.mem.wb(H_DIVISOR, max as u8);
            let prod = (u16::from(self.mem.rb(H_PRODUCT + 2)) << 8 | u16::from(self.mem.rb(H_PRODUCT + 3))) >> 2;
            self.mem.wb(H_DIVIDEND + 3, prod as u8);
            self.mem.wb(H_DIVIDEND + 2, (prod >> 8) as u8);
        }
        // .skip_to_divide
        self.divide(4);
        let b = self.mem.rb(H_QUOTIENT + 3);
        let mut p = r::FLAIL_REVERSAL_POWER;
        loop {
            let a = rom::byte(p);
            p += 1;
            if a >= b {
                break;
            }
            p += 1;
        }
        // .break_loop
        let power = rom::byte(p);
        let hl = if self.is_player_turn() { W_PLAYER_MOVE_STRUCT_POWER } else { W_ENEMY_MOVE_STRUCT_POWER };
        self.mem.wb(hl, power);
        if self.is_player_turn() {
            self.player_attack_damage();
        } else {
            self.enemy_attack_damage();
        }
        self.battle_command_damage_calc()?;
        self.mem.wb(hl, 1);
        Ok(Cmd::Next)
    }

    // --- Weather and badge boosts (engine/battle/misc.asm) -------------------------------

    /// `DoWeatherModifiers`.
    pub fn do_weather_modifiers(&mut self) {
        let b = self.mem.rb(W_BATTLE_WEATHER);
        let c = self.mem.rb(W_CUR_TYPE);
        let mut found = None;
        let mut de = r::WEATHER_TYPE_MODIFIERS;
        loop {
            let a = rom::byte(de);
            de += 1;
            if a == 0xff {
                break;
            }
            if a == b && rom::byte(de) == c {
                found = Some(de);
                break;
            }
            de += 2;
        }
        if found.is_none() {
            let c = self.battle_var(BATTLE_VARS_MOVE_EFFECT);
            let mut de = r::WEATHER_MOVE_MODIFIERS;
            loop {
                let a = rom::byte(de);
                de += 1;
                if a == 0xff {
                    break;
                }
                if a == b && rom::byte(de) == c {
                    found = Some(de);
                    break;
                }
                de += 2;
            }
        }
        let Some(de) = found else { return };
        // .ApplyModifier
        self.mem.wb(H_MULTIPLICAND, 0);
        let hi = self.mem.rb(W_CUR_DAMAGE);
        let lo = self.mem.rb(W_CUR_DAMAGE + 1);
        self.mem.wb(H_MULTIPLICAND + 1, hi);
        self.mem.wb(H_MULTIPLICAND + 2, lo);
        self.mem.wb(H_MULTIPLIER, rom::byte(de + 1));
        self.multiply();
        self.mem.wb(H_DIVISOR, 10);
        self.divide(4);
        let bc = if self.mem.rb(H_QUOTIENT + 1) != 0 {
            0xffff
        } else {
            let bc = u16::from(self.mem.rb(H_QUOTIENT + 2)) << 8 | u16::from(self.mem.rb(H_QUOTIENT + 3));
            if bc == 0 { 1 } else { bc }
        };
        self.mem.ww(W_CUR_DAMAGE, bc);
    }

    /// `DoBadgeTypeBoosts`: the player's badges boost the damage of their types by 1/8.
    pub fn do_badge_type_boosts(&mut self) {
        if self.mem.rb(W_LINK_MODE) != 0 {
            return;
        }
        if self.mem.rb(W_IN_BATTLE_TOWER_BATTLE) != 0 {
            return;
        }
        if !self.is_player_turn() {
            return;
        }
        let mut bc = u16::from(self.mem.rb(W_KANTO_BADGES)) << 8 | u16::from(self.mem.rb(W_JOHTO_BADGES));
        let mut hl = r::BADGE_TYPE_BOOSTS;
        loop {
            let t = rom::byte(hl);
            if t == 0xff {
                return;
            }
            let carry = bc & 1 != 0;
            bc >>= 1;
            if carry && self.mem.rb(W_CUR_TYPE) == t {
                break;
            }
            // .NextBadge
            hl += 1;
        }
        // .ApplyBoost
        let damage = self.mem.rw(W_CUR_DAMAGE);
        let mut de = damage >> 3;
        if de == 0 {
            de = 1;
        }
        let (sum, carry) = damage.overflowing_add(de);
        self.mem.ww(W_CUR_DAMAGE, if carry { 0xffff } else { sum });
    }
}
