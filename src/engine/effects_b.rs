//! Port of the move effects nightmare..transform
//! (`engine/battle/move_effects/*.asm`) and the small shared helpers of
//! `effect_commands.asm` they use (animation, failure text, `ResetTurn`).
//! See docs/engine.md.

#![allow(unused_imports)]

use super::battle::{Engine, Flow, PARTY_LEN, PP_MASK, percent};
use super::commands::{Cmd, rom_in_array};
use super::rom;
use super::rsym as r;
use super::sym::*;
use crate::defs::*;

/// `MAX_ROLLOUT_COUNT`.
const MAX_ROLLOUT_COUNT: u8 = 5;

impl Engine {
    // --- shared helpers from effect_commands.asm -------------------------------------

    /// `AnimateCurrentMoveEitherSide`.
    pub fn animate_current_move_either_side(&mut self) {
        let param = self.mem.rb(W_BATTLE_ANIM_PARAM);
        let _ = self.battle_command_lower_sub();
        self.mem.wb(W_BATTLE_ANIM_PARAM, param);
        self.play_damage_anim();
        let _ = self.battle_command_raise_sub();
    }

    /// `PlayDamageAnim`.
    fn play_damage_anim(&mut self) {
        self.mem.wb(W_FX_ANIM_ID + 1, 0);
        let anim = self.battle_var(BATTLE_VARS_MOVE_ANIM);
        if anim == 0 {
            return;
        }
        self.mem.wb(W_FX_ANIM_ID, anim);
        let after = if self.is_player_turn() { ANIM_ENEMY_DAMAGE } else { ANIM_PLAYER_DAMAGE } - BATTLE_AFTERANIMS;
        self.mem.wb(W_BATTLE_AFTER_ANIM, after as u8);
    }

    /// `AICheckPlayerMaxHP` / `AICheckEnemyMaxHP`: carry (`true`) if the HP at `hp` equals max HP at `max`.
    fn ai_check_max_hp(&self, hp: u16, max: u16) -> bool {
        self.mem.rb(hp) == self.mem.rb(max) && self.mem.rb(hp + 1) == self.mem.rb(max + 1)
    }

    // --- nightmare.asm -----------------------------------------------------------------

    /// `BattleCommand_Nightmare`.
    pub fn battle_command_nightmare(&mut self) -> Flow<Cmd> {
        if self.check_hidden_opponent()
            || self.check_substitute_opp()
            || self.battle_var(BATTLE_VARS_STATUS_OPP) & SLP_MASK == 0
            || self.battle_var_bit(BATTLE_VARS_SUBSTATUS1_OPP, SUBSTATUS_NIGHTMARE)
        {
            self.animate_failed_move();
            self.print_but_it_failed();
            return Ok(Cmd::Next);
        }
        self.set_battle_var_bit(BATTLE_VARS_SUBSTATUS1_OPP, SUBSTATUS_NIGHTMARE);
        self.animate_current_move();
        self.text(r::STARTED_NIGHTMARE_TEXT);
        Ok(Cmd::Next)
    }

    // --- pain_split.asm ----------------------------------------------------------------

    /// `BattleCommand_PainSplit`.
    pub fn battle_command_pain_split(&mut self) -> Flow<Cmd> {
        if self.mem.rb(W_ATTACK_MISSED) != 0 || self.check_substitute_opp() {
            self.print_didnt_affect2();
            return Ok(Cmd::Next);
        }
        self.animate_current_move();
        self.pain_split_player_share_hp();
        self.mem.wb(W_WHICH_HP_BAR, 1);
        self.animate_hp_bar();
        let hp = W_ENEMY_MON_HP;
        self.mem.wb(W_HP_BUFFER2 + 1, self.mem.rb(hp));
        self.mem.wb(W_HP_BUFFER2, self.mem.rb(hp + 1));
        self.mem.wb(W_HP_BUFFER1 + 1, self.mem.rb(hp + 2));
        self.mem.wb(W_HP_BUFFER1, self.mem.rb(hp + 3));
        self.pain_split_enemy_share_hp(W_ENEMY_MON_MAX_HP + 1);
        self.mem.wb(W_WHICH_HP_BAR, 0);
        self.reset_damage();
        self.animate_hp_bar();
        self.update_battle_huds();
        self.text(r::SHARED_PAIN_TEXT);
        Ok(Cmd::Next)
    }

    /// `BattleCommand_PainSplit.PlayerShareHP`, falling through to `.EnemyShareHP`.
    fn pain_split_player_share_hp(&mut self) {
        let max = W_BATTLE_MON_MAX_HP;
        let hp = W_BATTLE_MON_HP;
        self.mem.wb(W_HP_BUFFER1, self.mem.rb(max + 1));
        self.mem.wb(W_HP_BUFFER1 + 1, self.mem.rb(max));
        let b = self.mem.rb(hp + 1);
        self.mem.wb(W_HP_BUFFER2, b);
        self.mem.wb(W_HP_BUFFER2 + 1, self.mem.rb(hp));
        let enemy_hp = W_ENEMY_MON_HP;
        let sum_lo = u16::from(self.mem.rb(enemy_hp + 1)) + u16::from(b);
        self.mem.wb(W_CUR_DAMAGE + 1, sum_lo as u8);
        let hi = self.mem.rb(hp);
        let sum_hi = u16::from(self.mem.rb(enemy_hp)) + u16::from(hi) + (sum_lo >> 8);
        // srl a: bit 0 of the (8-bit) sum goes into the carry for the `rr` below.
        let carry = sum_hi & 1 != 0;
        let avg_hi = ((sum_hi & 0xff) >> 1) as u8;
        self.mem.wb(W_CUR_DAMAGE, avg_hi);
        let lo = self.mem.rb(W_CUR_DAMAGE + 1);
        self.mem.wb(W_CUR_DAMAGE + 1, lo >> 1 | u8::from(carry) << 7);
        self.pain_split_enemy_share_hp(max + 1);
    }

    /// `BattleCommand_PainSplit.EnemyShareHP`: `hl` is the max HP low byte of the mon to set.
    fn pain_split_enemy_share_hp(&mut self, hl: u16) {
        let c = self.mem.rb(hl);
        let b = self.mem.rb(hl - 1);
        let dmg = self.mem.rw(W_CUR_DAMAGE);
        let max = u16::from(b) << 8 | u16::from(c);
        let (b, c) = if dmg >= max { (b, c) } else { ((dmg >> 8) as u8, dmg as u8) };
        let hp = hl - 3;
        self.mem.wb(hp + 1, c);
        self.mem.wb(W_HP_BUFFER3, c);
        self.mem.wb(hp, b);
        self.mem.wb(W_HP_BUFFER3 + 1, b);
    }

    // --- pay_day.asm -------------------------------------------------------------------

    /// `BattleCommand_PayDay`.
    pub fn battle_command_pay_day(&mut self) -> Flow<Cmd> {
        self.mem.wb(W_STRING_BUFFER1, 0);
        let level = self.mem.rb(if self.is_player_turn() { W_BATTLE_MON_LEVEL } else { W_ENEMY_MON_LEVEL });
        let a = level.wrapping_add(level);
        let (lo, carry) = a.overflowing_add(self.mem.rb(W_PAY_DAY_MONEY + 2));
        self.mem.wb(W_PAY_DAY_MONEY + 2, lo);
        if carry {
            let mid = self.mem.rb(W_PAY_DAY_MONEY + 1).wrapping_add(1);
            self.mem.wb(W_PAY_DAY_MONEY + 1, mid);
            if mid == 0 {
                let hi = self.mem.rb(W_PAY_DAY_MONEY).wrapping_add(1);
                self.mem.wb(W_PAY_DAY_MONEY, hi);
            }
        }
        self.text(r::COINS_SCATTERED_TEXT);
        Ok(Cmd::Next)
    }

    // --- perish_song.asm ---------------------------------------------------------------

    /// `BattleCommand_PerishSong`.
    pub fn battle_command_perish_song(&mut self) -> Flow<Cmd> {
        if self.mem.bit(W_PLAYER_SUB_STATUS1, SUBSTATUS_PERISH) && self.mem.bit(W_ENEMY_SUB_STATUS1, SUBSTATUS_PERISH) {
            self.animate_failed_move();
            self.print_but_it_failed();
            return Ok(Cmd::Next);
        }
        if !self.mem.bit(W_PLAYER_SUB_STATUS1, SUBSTATUS_PERISH) {
            self.mem.set_bit(W_PLAYER_SUB_STATUS1, SUBSTATUS_PERISH);
            self.mem.wb(W_PLAYER_PERISH_COUNT, 4);
        }
        if !self.mem.bit(W_ENEMY_SUB_STATUS1, SUBSTATUS_PERISH) {
            self.mem.set_bit(W_ENEMY_SUB_STATUS1, SUBSTATUS_PERISH);
            self.mem.wb(W_ENEMY_PERISH_COUNT, 4);
        }
        self.animate_current_move();
        self.text(r::START_PERISH_TEXT);
        Ok(Cmd::Next)
    }

    // --- present.asm -------------------------------------------------------------------

    /// `BattleCommand_Present`. `damage_regs` is the original's `bc`/`de` (see
    /// `damage.rs`); in a colosseum battle the original does not preserve
    /// them across `BattleCommand_Stab` (a documented bug), so neither do we.
    pub fn battle_command_present(&mut self) -> Flow<Cmd> {
        let colosseum = self.mem.rb(W_LINK_MODE) == LINK_COLOSSEUM;
        if let Some(regs) = self.stab_regs() {
            if colosseum {
                self.damage_regs = regs;
            }
        }
        if self.mem.rb(W_TYPE_MATCHUP) == 0 || self.mem.rb(W_ATTACK_MISSED) != 0 {
            self.animate_failed_move();
            return Ok(Cmd::Next);
        }
        let b = self.battle_random("BattleCommand_Present");
        let mut hl = r::PRESENT_POWER;
        let mut c = 0u8;
        loop {
            let a = rom::byte(hl);
            hl += 1;
            if a == 0xff {
                break;
            }
            if a >= b {
                // .got_power
                self.mem.wb(W_BATTLE_ANIM_PARAM, c);
                self.animate_current_move_either_side();
                self.damage_regs[2] = rom::byte(hl);
                return Ok(Cmd::Next);
            }
            c += 1;
            hl += 1;
        }
        // .heal_effect
        self.mem.wb(W_BATTLE_ANIM_PARAM, 3);
        self.animate_current_move();
        self.switch_turn();
        let full = if self.is_player_turn() {
            self.ai_check_max_hp(W_BATTLE_MON_HP, W_BATTLE_MON_MAX_HP)
        } else {
            self.ai_check_max_hp(W_ENEMY_MON_HP, W_ENEMY_MON_MAX_HP)
        };
        if !full {
            let bc = self.get_quarter_max_hp();
            self.switch_turn();
            self.restore_hp(bc);
            self.switch_turn();
            self.text(r::REGAINED_HEALTH_TEXT);
            self.switch_turn();
            self.update_opponent_in_party();
        } else {
            // .already_fully_healed
            self.switch_turn();
            if self.check_battle_scene() {
                self.animate_failed_move();
                self.text(r::PRESENT_FAILED_TEXT);
            }
        }
        self.end_move_effect();
        Ok(Cmd::Next)
    }

    // --- protect.asm -------------------------------------------------------------------

    /// `BattleCommand_Protect`.
    pub fn battle_command_protect(&mut self) -> Flow<Cmd> {
        if self.protect_chance() {
            return Ok(Cmd::Next);
        }
        self.set_battle_var_bit(BATTLE_VARS_SUBSTATUS1, SUBSTATUS_PROTECT);
        self.animate_current_move();
        self.text(r::PROTECTED_ITSELF_TEXT);
        Ok(Cmd::Next)
    }

    /// `ProtectChance`: returns carry (`true`) if the move failed.
    fn protect_chance(&mut self) -> bool {
        let de = if self.is_player_turn() { W_PLAYER_PROTECT_COUNT } else { W_ENEMY_PROTECT_COUNT };
        let failed = 'check: {
            if self.check_opponent_went_first() != 0 {
                break 'check true;
            }
            if self.battle_var_bit(BATTLE_VARS_SUBSTATUS4, SUBSTATUS_SUBSTITUTE) {
                break 'check true;
            }
            // Halve the chance of success for each consecutive use.
            let mut b = 0xffu8;
            let mut c = self.mem.rb(de);
            while c != 0 {
                c -= 1;
                b >>= 1;
                if b == 0 {
                    break 'check true;
                }
            }
            let mut a;
            loop {
                a = self.battle_random("ProtectChance");
                if a != 0 {
                    break;
                }
            }
            a -= 1;
            if a >= b {
                break 'check true;
            }
            let count = self.mem.rb(de).wrapping_add(1);
            self.mem.wb(de, count);
            false
        };
        if failed {
            self.mem.wb(de, 0);
            self.animate_failed_move();
            self.print_but_it_failed();
        }
        failed
    }

    // --- psych_up.asm ------------------------------------------------------------------

    /// `BattleCommand_PsychUp`.
    pub fn battle_command_psych_up(&mut self) -> Flow<Cmd> {
        let (src, dst) = if self.is_player_turn() {
            (W_ENEMY_STAT_LEVELS, W_PLAYER_STAT_LEVELS)
        } else {
            (W_PLAYER_STAT_LEVELS, W_ENEMY_STAT_LEVELS)
        };
        let n = u16::from(NUM_LEVEL_STATS);
        if (0..n).all(|i| self.mem.rb(src + i) == BASE_STAT_LEVEL) {
            self.animate_failed_move();
            self.print_but_it_failed();
            return Ok(Cmd::Next);
        }
        self.mem.copy(dst, src, n);
        if self.is_player_turn() {
            self.calc_player_stats();
        } else {
            self.calc_enemy_stats();
        }
        self.animate_current_move();
        self.text(r::COPIED_STATS_TEXT);
        Ok(Cmd::Next)
    }

    // --- pursuit.asm -------------------------------------------------------------------

    /// `BattleCommand_Pursuit`: double damage if the opponent is switching.
    pub fn battle_command_pursuit(&mut self) -> Flow<Cmd> {
        let hl = if self.is_player_turn() { W_ENEMY_IS_SWITCHING } else { W_PLAYER_IS_SWITCHING };
        if self.mem.rb(hl) == 0 {
            return Ok(Cmd::Next);
        }
        self.double_cur_damage();
        Ok(Cmd::Next)
    }

    /// `sla [wCurDamage + 1]` / `rl [wCurDamage]`, setting `$ffff` on overflow.
    /// Returns whether it overflowed.
    fn double_cur_damage(&mut self) -> bool {
        let d = self.mem.rw(W_CUR_DAMAGE);
        if d & 0x8000 != 0 {
            self.mem.wb(W_CUR_DAMAGE, 0xff);
            self.mem.wb(W_CUR_DAMAGE + 1, 0xff);
            return true;
        }
        self.mem.ww(W_CUR_DAMAGE, d << 1);
        false
    }

    // --- rage.asm ----------------------------------------------------------------------

    /// `BattleCommand_Rage`.
    pub fn battle_command_rage(&mut self) -> Flow<Cmd> {
        self.set_battle_var_bit(BATTLE_VARS_SUBSTATUS4, SUBSTATUS_RAGE);
        Ok(Cmd::Next)
    }

    // --- rain_dance.asm, sunny_day.asm, sandstorm.asm ----------------------------------

    /// `BattleCommand_StartRain`.
    pub fn battle_command_start_rain(&mut self) -> Flow<Cmd> {
        self.mem.wb(W_BATTLE_WEATHER, WEATHER_RAIN);
        self.mem.wb(W_WEATHER_COUNT, 5);
        self.animate_current_move();
        self.text(r::DOWNPOUR_TEXT);
        Ok(Cmd::Next)
    }

    /// `BattleCommand_StartSun`.
    pub fn battle_command_start_sun(&mut self) -> Flow<Cmd> {
        self.mem.wb(W_BATTLE_WEATHER, WEATHER_SUN);
        self.mem.wb(W_WEATHER_COUNT, 5);
        self.animate_current_move();
        self.text(r::SUN_GOT_BRIGHT_TEXT);
        Ok(Cmd::Next)
    }

    /// `BattleCommand_StartSandstorm`.
    pub fn battle_command_start_sandstorm(&mut self) -> Flow<Cmd> {
        if self.mem.rb(W_BATTLE_WEATHER) == WEATHER_SANDSTORM {
            self.animate_failed_move();
            self.print_but_it_failed();
            return Ok(Cmd::Next);
        }
        self.mem.wb(W_BATTLE_WEATHER, WEATHER_SANDSTORM);
        self.mem.wb(W_WEATHER_COUNT, 5);
        self.animate_current_move();
        self.text(r::SANDSTORM_BREWED_TEXT);
        Ok(Cmd::Next)
    }

    // --- rapid_spin.asm ----------------------------------------------------------------

    /// `BattleCommand_ClearHazards`.
    pub fn battle_command_clear_hazards(&mut self) -> Flow<Cmd> {
        if self.battle_var_bit(BATTLE_VARS_SUBSTATUS4, SUBSTATUS_LEECH_SEED) {
            self.res_battle_var_bit(BATTLE_VARS_SUBSTATUS4, SUBSTATUS_LEECH_SEED);
            self.text(r::SHED_LEECH_SEED_TEXT);
        }
        let (screens, wrap) = if self.is_player_turn() {
            (W_PLAYER_SCREENS, W_PLAYER_WRAP_COUNT)
        } else {
            (W_ENEMY_SCREENS, W_ENEMY_WRAP_COUNT)
        };
        if self.mem.bit(screens, SCREENS_SPIKES) {
            self.mem.res_bit(screens, SCREENS_SPIKES);
            self.text(r::BLEW_SPIKES_TEXT);
        }
        if self.mem.rb(wrap) == 0 {
            return Ok(Cmd::Next);
        }
        self.mem.wb(wrap, 0);
        self.text(r::RELEASED_BY_TEXT);
        Ok(Cmd::Next)
    }

    // --- return.asm --------------------------------------------------------------------

    /// `BattleCommand_HappinessPower`: the move power `d` into `damage_regs`.
    pub fn battle_command_happiness_power(&mut self) -> Flow<Cmd> {
        let hl = if self.is_player_turn() { W_BATTLE_MON_HAPPINESS } else { W_ENEMY_MON_HAPPINESS };
        self.mem.wb(H_MULTIPLICAND, 0);
        self.mem.wb(H_MULTIPLICAND + 1, 0);
        self.mem.wb(H_MULTIPLICAND + 2, self.mem.rb(hl));
        self.mem.wb(H_MULTIPLIER, 10);
        self.multiply();
        self.mem.wb(H_DIVISOR, 25);
        self.divide(4);
        self.damage_regs[2] = self.mem.rb(H_QUOTIENT + 3);
        Ok(Cmd::Next)
    }

    // --- rollout.asm -------------------------------------------------------------------

    /// `BattleCommand_CheckRollout`.
    pub fn battle_command_check_rollout(&mut self) -> Flow<Cmd> {
        let de = if self.is_player_turn() { W_PLAYER_ROLLOUT_COUNT } else { W_ENEMY_ROLLOUT_COUNT };
        if self.battle_var_bit(BATTLE_VARS_SUBSTATUS1, SUBSTATUS_ROLLOUT) {
            self.skip_to_battle_command(DOTURN_COMMAND);
        } else {
            self.mem.wb(de, 0);
        }
        Ok(Cmd::Next)
    }

    /// `BattleCommand_RolloutPower`.
    pub fn battle_command_rollout_power(&mut self) -> Flow<Cmd> {
        if self.battle_var(BATTLE_VARS_STATUS) & SLP_MASK != 0 {
            return Ok(Cmd::Next);
        }
        let hl = if self.is_player_turn() { W_PLAYER_ROLLOUT_COUNT } else { W_ENEMY_ROLLOUT_COUNT };
        if self.mem.rb(hl) == 0 {
            self.mem.wb(W_SOMEONE_IS_RAMPAGING, 1);
        }
        if self.mem.rb(W_ATTACK_MISSED) != 0 {
            self.res_battle_var_bit(BATTLE_VARS_SUBSTATUS1, SUBSTATUS_ROLLOUT);
            return Ok(Cmd::Next);
        }
        let count = self.mem.rb(hl).wrapping_add(1);
        self.mem.wb(hl, count);
        let mut b = count;
        if count >= MAX_ROLLOUT_COUNT {
            self.res_battle_var_bit(BATTLE_VARS_SUBSTATUS1, SUBSTATUS_ROLLOUT);
        } else {
            self.set_battle_var_bit(BATTLE_VARS_SUBSTATUS1, SUBSTATUS_ROLLOUT);
        }
        if self.battle_var_bit(BATTLE_VARS_SUBSTATUS2, SUBSTATUS_CURLED) {
            b = b.wrapping_add(1);
        }
        loop {
            b = b.wrapping_sub(1);
            if b == 0 {
                break;
            }
            if self.double_cur_damage() {
                break;
            }
        }
        Ok(Cmd::Next)
    }

    // --- safeguard.asm -----------------------------------------------------------------

    /// `BattleCommand_Safeguard`.
    pub fn battle_command_safeguard(&mut self) -> Flow<Cmd> {
        let (hl, de) = if self.is_player_turn() {
            (W_PLAYER_SCREENS, W_PLAYER_SAFEGUARD_COUNT)
        } else {
            (W_ENEMY_SCREENS, W_ENEMY_SAFEGUARD_COUNT)
        };
        if self.mem.bit(hl, SCREENS_SAFEGUARD) {
            self.animate_failed_move();
            self.print_but_it_failed();
            return Ok(Cmd::Next);
        }
        self.mem.set_bit(hl, SCREENS_SAFEGUARD);
        self.mem.wb(de, 5);
        self.animate_current_move();
        self.text(r::COVERED_BY_VEIL_TEXT);
        Ok(Cmd::Next)
    }

    // --- selfdestruct.asm --------------------------------------------------------------

    /// `BattleCommand_Selfdestruct`.
    pub fn battle_command_selfdestruct(&mut self) -> Flow<Cmd> {
        self.mem.wb(W_BATTLE_AFTER_ANIM, (ANIM_PLAYER_DAMAGE - BATTLE_AFTERANIMS) as u8);
        let status = self.battle_var_addr(BATTLE_VARS_STATUS);
        self.mem.wb(status, 0);
        self.mem.wb(status + 2, 0);
        self.mem.wb(status + 3, 0);
        self.mem.wb(W_BATTLE_ANIM_PARAM, 1);
        let _ = self.battle_command_lower_sub();
        self.load_move_anim();
        self.res_battle_var_bit(BATTLE_VARS_SUBSTATUS4, SUBSTATUS_LEECH_SEED);
        self.res_battle_var_bit(BATTLE_VARS_SUBSTATUS5_OPP, SUBSTATUS_DESTINY_BOND);
        Ok(Cmd::Next)
    }

    // --- sketch.asm --------------------------------------------------------------------

    /// `BattleCommand_Sketch`.
    pub fn battle_command_sketch(&mut self) -> Flow<Cmd> {
        self.clear_last_move();
        if self.mem.rb(W_LINK_MODE) != 0 {
            self.animate_failed_move();
            self.print_nothing_happened();
            return Ok(Cmd::Next);
        }
        if self.sketch_try().is_none() {
            // .fail
            self.animate_failed_move();
            self.print_didnt_affect();
        }
        Ok(Cmd::Next)
    }

    /// The body of `BattleCommand_Sketch` after the link check; `None` = `.fail`.
    fn sketch_try(&mut self) -> Option<()> {
        if self.check_substitute_opp() {
            return None;
        }
        // BUG: A Transformed Pokémon can use Sketch and learn otherwise unobtainable moves
        if self.battle_var_bit(BATTLE_VARS_SUBSTATUS5_OPP, SUBSTATUS_TRANSFORMED) {
            return None;
        }
        let de = self.user_party_attr(MON_MOVES);
        let mut hl = if self.is_player_turn() { W_BATTLE_MON_MOVES } else { W_ENEMY_MON_MOVES };
        let b = self.battle_var(BATTLE_VARS_LAST_COUNTER_MOVE_OPP);
        self.mem.wb(W_NAMED_OBJECT_INDEX, b);
        if b == 0 || b == STRUGGLE {
            return None;
        }
        for _ in 0..NUM_MOVES {
            let a = self.mem.rb(hl);
            hl = hl.wrapping_add(1);
            if a == b {
                return None;
            }
        }
        hl = hl.wrapping_sub(1);
        let mut c = NUM_MOVES;
        loop {
            c = c.wrapping_sub(1);
            let a = self.mem.rb(hl);
            hl = hl.wrapping_sub(1);
            if a == SKETCH {
                break;
            }
        }
        hl = hl.wrapping_add(1);
        self.mem.wb(hl, b);
        let pp = rom::byte(r::MOVES + usize::from(b - 1) * usize::from(MOVE_LENGTH) + usize::from(MOVE_PP));
        let hl = hl.wrapping_add(W_BATTLE_MON_PP - W_BATTLE_MON_MOVES);
        self.mem.wb(hl, pp);
        if !self.is_player_turn() && self.mem.rb(W_BATTLE_MODE) == 1 {
            // wildmon
            let a = self.mem.rb(hl);
            self.mem.wb(W_WILD_MON_PP + u16::from(c), a);
            self.mem.wb(W_WILD_MON_MOVES + u16::from(c), b);
        } else {
            // .user_trainer
            let a = self.mem.rb(hl);
            let slot = de.wrapping_add(u16::from(c));
            self.mem.wb(slot, b);
            self.mem.wb(slot.wrapping_add(u16::from(MON_PP - MON_MOVES)), a);
        }
        self.get_move_name();
        self.animate_current_move();
        self.text(r::SKETCHED_TEXT);
        Some(())
    }

    // --- sleep_talk.asm ----------------------------------------------------------------

    /// `BattleCommand_SleepTalk`.
    pub fn battle_command_sleep_talk(&mut self) -> Flow<Cmd> {
        self.clear_last_move();
        if self.sleep_talk_try()? {
            return Ok(Cmd::Next);
        }
        // .fail
        self.animate_failed_move();
        self.try_print_but_it_failed();
        Ok(Cmd::Next)
    }

    /// Everything but `.fail`; `false` = `.fail`.
    fn sleep_talk_try(&mut self) -> Flow<bool> {
        if self.mem.rb(W_ATTACK_MISSED) != 0 {
            return Ok(false);
        }
        let (moves, d) = if self.is_player_turn() {
            (W_BATTLE_MON_MOVES, self.mem.rb(W_DISABLED_MOVE))
        } else {
            (W_ENEMY_MON_MOVES, self.mem.rb(W_ENEMY_DISABLED_MOVE))
        };
        if self.battle_var(BATTLE_VARS_STATUS) & SLP_MASK == 0 {
            return Ok(false);
        }
        if self.mem.rb(moves + 1) == 0 {
            return Ok(false);
        }
        if self.sleep_talk_check_has_usable_move(moves) {
            return Ok(false);
        }
        let e = loop {
            let c = u16::from(self.battle_random("BattleCommand_SleepTalk") & 3);
            let e = self.mem.rb(moves + c);
            if e == 0 {
                continue;
            }
            if e == self.battle_var(BATTLE_VARS_MOVE_ANIM) {
                continue;
            }
            if e == d {
                continue;
            }
            if self.sleep_talk_check_two_turn_move(e) {
                continue;
            }
            break e;
        };
        self.set_battle_var(BATTLE_VARS_MOVE, e);
        if !self.check_user_is_charging() {
            let param = self.mem.rb(W_BATTLE_ANIM_PARAM);
            let _ = self.battle_command_lower_sub();
            self.mem.wb(W_BATTLE_ANIM_PARAM, param);
        }
        self.load_move_anim();
        self.update_move_data();
        self.reset_turn()?;
        Ok(true)
    }

    /// `.check_has_usable_move`: carry (`true`) if there is none.
    fn sleep_talk_check_has_usable_move(&self, moves: u16) -> bool {
        let b = if self.is_player_turn() { self.mem.rb(W_DISABLED_MOVE) } else { self.mem.rb(W_ENEMY_DISABLED_MOVE) };
        let c = self.battle_var(BATTLE_VARS_MOVE);
        for i in 0..u16::from(NUM_MOVES) {
            let a = self.mem.rb(moves + i);
            if a == 0 {
                return true;
            }
            if a == c || a == b {
                continue;
            }
            if !self.sleep_talk_check_two_turn_move(a) {
                return false;
            }
        }
        true
    }

    /// `.check_two_turn_move`: z (`true`) for charging moves and Bide.
    fn sleep_talk_check_two_turn_move(&self, mv: u8) -> bool {
        let effect = self.get_move_effect(mv);
        matches!(effect, EFFECT_SKULL_BASH | EFFECT_RAZOR_WIND | EFFECT_SKY_ATTACK | EFFECT_SOLARBEAM | EFFECT_FLY | EFFECT_BIDE)
    }

    // --- snore.asm ---------------------------------------------------------------------

    /// `BattleCommand_Snore`.
    pub fn battle_command_snore(&mut self) -> Flow<Cmd> {
        if self.battle_var(BATTLE_VARS_STATUS) & SLP_MASK != 0 {
            return Ok(Cmd::Next);
        }
        self.reset_damage();
        self.mem.wb(W_ATTACK_MISSED, 1);
        self.fail_move();
        self.end_move_effect();
        Ok(Cmd::Next)
    }

    // --- spikes.asm --------------------------------------------------------------------

    /// `BattleCommand_Spikes`.
    pub fn battle_command_spikes(&mut self) -> Flow<Cmd> {
        let hl = if self.is_player_turn() { W_ENEMY_SCREENS } else { W_PLAYER_SCREENS };
        if self.mem.bit(hl, SCREENS_SPIKES) {
            self.fail_move();
            return Ok(Cmd::Next);
        }
        self.mem.set_bit(hl, SCREENS_SPIKES);
        self.animate_current_move();
        self.text(r::SPIKES_TEXT);
        Ok(Cmd::Next)
    }

    // --- spite.asm ---------------------------------------------------------------------

    /// `BattleCommand_Spite`.
    pub fn battle_command_spite(&mut self) -> Flow<Cmd> {
        if !self.spite_try() {
            self.print_didnt_affect2();
        }
        Ok(Cmd::Next)
    }

    /// Everything but `.failed`; `false` = `.failed`.
    fn spite_try(&mut self) -> bool {
        if self.mem.rb(W_ATTACK_MISSED) != 0 {
            return false;
        }
        let mut hl = if self.is_player_turn() { W_ENEMY_MON_MOVES } else { W_BATTLE_MON_MOVES };
        let b = self.battle_var(BATTLE_VARS_LAST_COUNTER_MOVE_OPP);
        if b == 0 || b == STRUGGLE {
            return false;
        }
        let mut c = 0xffu8;
        loop {
            c = c.wrapping_add(1);
            let a = self.mem.rb(hl);
            hl = hl.wrapping_add(1);
            if a == b {
                self.mem.wb(W_NAMED_OBJECT_INDEX, a);
                break;
            }
        }
        hl = hl.wrapping_sub(1).wrapping_add(W_BATTLE_MON_PP - W_BATTLE_MON_MOVES);
        if self.mem.rb(hl) & PP_MASK == 0 {
            return false;
        }
        self.get_move_name();
        // lose 2-5 PP
        let mut b = (self.battle_random("BattleCommand_Spite") & 3) + 2;
        let pp = self.mem.rb(hl) & PP_MASK;
        if pp < b {
            b = pp;
        }
        let new = self.mem.rb(hl).wrapping_sub(b);
        self.mem.wb(hl, new);
        let d = b;
        let mut hl = self.opponent_party_attr(MON_PP).wrapping_add(u16::from(c));
        if !self.battle_var_bit(BATTLE_VARS_SUBSTATUS5_OPP, SUBSTATUS_TRANSFORMED) {
            if self.is_player_turn() && self.mem.rb(W_BATTLE_MODE) == 1 {
                hl = W_WILD_MON_PP + u16::from(c);
            }
            self.mem.wb(hl, new);
        }
        self.animate_current_move();
        self.mem.wb(W_TEXT_DECIMAL_BYTE, d);
        self.text(r::SPITE_EFFECT_TEXT);
        true
    }

    // --- splash.asm --------------------------------------------------------------------

    /// `BattleCommand_Splash`.
    pub fn battle_command_splash(&mut self) -> Flow<Cmd> {
        self.animate_current_move();
        self.print_nothing_happened();
        Ok(Cmd::Next)
    }

    // --- substitute.asm ----------------------------------------------------------------

    /// `BattleCommand_Substitute`.
    pub fn battle_command_substitute(&mut self) -> Flow<Cmd> {
        let (hl, de) = if self.is_player_turn() {
            (W_BATTLE_MON_MAX_HP, W_PLAYER_SUBSTITUTE_HP)
        } else {
            (W_ENEMY_MON_MAX_HP, W_ENEMY_SUBSTITUTE_HP)
        };
        if self.battle_var_bit(BATTLE_VARS_SUBSTATUS4, SUBSTATUS_SUBSTITUTE) {
            // .already_has_sub
            if self.check_user_is_charging() {
                let _ = self.battle_command_raise_sub();
            }
            self.text(r::HAS_SUBSTITUTE_TEXT);
            return Ok(Cmd::Next);
        }
        let quarter = self.mem.rw(hl) >> 2;
        let b = quarter as u8;
        self.mem.wb(de, b);
        let hp = hl - 2;
        let (lo, borrow_lo) = self.mem.rb(hp + 1).overflowing_sub(b);
        let (hi, borrow_hi) = self.mem.rb(hp).overflowing_sub(u8::from(borrow_lo));
        if borrow_hi || hi | lo == 0 {
            // .too_weak_to_sub
            if self.check_user_is_charging() {
                let _ = self.battle_command_raise_sub();
            }
            self.text(r::TOO_WEAK_SUB_TEXT);
            return Ok(Cmd::Next);
        }
        self.mem.wb(hp, hi);
        self.mem.wb(hp + 1, lo);
        self.set_battle_var_bit(BATTLE_VARS_SUBSTATUS4, SUBSTATUS_SUBSTITUTE);
        let (wrap, trap) = if self.is_player_turn() {
            (W_PLAYER_WRAP_COUNT, W_PLAYER_TRAPPING_MOVE)
        } else {
            (W_ENEMY_WRAP_COUNT, W_ENEMY_TRAPPING_MOVE)
        };
        self.mem.wb(wrap, 0);
        self.mem.wb(trap, 0);
        if self.check_battle_scene() {
            let _ = self.battle_command_raise_sub_no_anim();
        } else {
            self.mem.wb(W_BATTLE_AFTER_ANIM, 0);
            self.mem.wb(W_FX_ANIM_ID + 1, 0);
            self.mem.wb(W_BATTLE_ANIM_PARAM, 0);
            self.load_anim(SUBSTITUTE);
        }
        self.text(r::MADE_SUBSTITUTE_TEXT);
        Ok(Cmd::Next)
    }

    // --- teleport.asm ------------------------------------------------------------------

    /// `BattleCommand_Teleport`.
    pub fn battle_command_teleport(&mut self) -> Flow<Cmd> {
        let battle_type = self.mem.rb(W_BATTLE_TYPE);
        let escapes = 'check: {
            if matches!(battle_type, BATTLETYPE_FORCESHINY | BATTLETYPE_TRAP | BATTLETYPE_CELEBI | BATTLETYPE_SUICUNE) {
                break 'check false;
            }
            if self.battle_var_bit(BATTLE_VARS_SUBSTATUS5_OPP, SUBSTATUS_CANT_RUN) {
                break 'check false;
            }
            if self.mem.rb(W_BATTLE_MODE) != 1 {
                break 'check false;
            }
            if self.is_player_turn() {
                let mut b = self.mem.rb(W_CUR_PARTY_LEVEL);
                let a = self.mem.rb(W_BATTLE_MON_LEVEL);
                if a >= b {
                    break 'check true;
                }
                let c = a.wrapping_add(b).wrapping_add(1);
                let a = loop {
                    let a = self.battle_random("BattleCommand_Teleport");
                    if a < c {
                        break a;
                    }
                };
                b >>= 2;
                a >= b
            } else {
                let b = self.mem.rb(W_BATTLE_MON_LEVEL);
                let a = self.mem.rb(W_CUR_PARTY_LEVEL);
                if a >= b {
                    break 'check true;
                }
                let c = a.wrapping_add(b).wrapping_add(1);
                loop {
                    // BUG: Wild Pokémon can always Teleport regardless of level difference
                    let a = self.battle_random("BattleCommand_Teleport");
                    if a < c {
                        break;
                    }
                }
                true
            }
        };
        if !escapes {
            self.animate_failed_move();
            self.print_but_it_failed();
            return Ok(Cmd::Next);
        }
        // .run_away
        self.update_battle_mon_in_party();
        self.mem.wb(W_BATTLE_AFTER_ANIM, 0);
        self.mem.wb(W_FORCED_SWITCH, 1);
        self.mem.wb(W_BATTLE_ANIM_PARAM, 1);
        self.set_battle_draw();
        let _ = self.battle_command_lower_sub();
        self.load_move_anim();
        self.set_battle_draw();
        self.text(r::FLED_FROM_BATTLE_TEXT);
        Ok(Cmd::Next)
    }

    // --- thief.asm ---------------------------------------------------------------------

    /// `BattleCommand_Thief`.
    pub fn battle_command_thief(&mut self) -> Flow<Cmd> {
        let player = self.is_player_turn();
        // (party mon item slot, battle mon item slot) of the thief and of the victim.
        let (me_party, me_mon) = if player {
            (self.battle_party_attr(MON_ITEM), W_BATTLE_MON_ITEM)
        } else {
            (self.ot_party_attr(MON_ITEM), W_ENEMY_MON_ITEM)
        };
        let (foe_party, foe_mon) = if player {
            (self.ot_party_attr(MON_ITEM), W_ENEMY_MON_ITEM)
        } else {
            (self.battle_party_attr(MON_ITEM), W_BATTLE_MON_ITEM)
        };
        // The thief can't already hold an item; the victim must have one.
        if self.mem.rb(me_mon) != 0 {
            return Ok(Cmd::Next);
        }
        let item = self.mem.rb(foe_mon);
        if item == 0 {
            return Ok(Cmd::Next);
        }
        // Can't steal mail.
        self.mem.wb(W_NAMED_OBJECT_INDEX, item);
        if rom_in_array(r::MAIL_ITEMS, 1, item).is_some() {
            return Ok(Cmd::Next);
        }
        if self.mem.rb(W_EFFECT_FAILED) != 0 {
            return Ok(Cmd::Next);
        }
        if player && self.mem.rb(W_LINK_MODE) != 0 && self.mem.rb(W_BATTLE_MODE) == 1 {
            return Ok(Cmd::Next);
        }
        self.mem.wb(foe_mon, 0);
        self.mem.wb(foe_party, 0);
        self.mem.wb(me_mon, item);
        self.mem.wb(me_party, item);
        self.get_item_name();
        self.text(r::STOLE_TEXT);
        Ok(Cmd::Next)
    }

    // --- thunder.asm -------------------------------------------------------------------

    /// `BattleCommand_ThunderAccuracy`.
    pub fn battle_command_thunder_accuracy(&mut self) -> Flow<Cmd> {
        let hl = self.battle_var_addr(BATTLE_VARS_MOVE_TYPE) + 1;
        match self.mem.rb(W_BATTLE_WEATHER) {
            WEATHER_RAIN => self.mem.wb(hl, percent(100)),
            WEATHER_SUN => self.mem.wb(hl, percent(50) + 1),
            _ => {}
        }
        Ok(Cmd::Next)
    }

    // --- transform.asm -----------------------------------------------------------------

    /// `BattleCommand_Transform`.
    pub fn battle_command_transform(&mut self) -> Flow<Cmd> {
        self.clear_last_move();
        if self.battle_var_bit(BATTLE_VARS_SUBSTATUS5_OPP, SUBSTATUS_TRANSFORMED) || self.check_hidden_opponent() {
            self.battle_effect_but_it_failed();
            return Ok(Cmd::Next);
        }
        self.mem.wb(W_BATTLE_AFTER_ANIM, 0);
        self.mem.wb(W_FX_ANIM_ID + 1, 0);
        self.mem.wb(W_BATTLE_ANIM_PARAM, 1);
        let had_sub = self.battle_var_bit(BATTLE_VARS_SUBSTATUS4, SUBSTATUS_SUBSTITUTE);
        if had_sub && !self.check_user_is_charging() {
            self.load_anim(SUBSTITUTE);
        }
        self.set_battle_var_bit(BATTLE_VARS_SUBSTATUS5, SUBSTATUS_TRANSFORMED);
        self.reset_actor_disable();
        let player = self.is_player_turn();
        let (src, dst) = if player {
            self.mem.wb(W_CUR_MOVE_NUM, 0);
            (W_ENEMY_MON, W_BATTLE_MON)
        } else {
            (W_BATTLE_MON, W_ENEMY_MON)
        };
        let off = |sym: u16| sym - W_BATTLE_MON;
        // species
        let species = self.mem.rb(src);
        self.mem.wb(dst, species);
        // moves
        self.mem.copy(dst + off(W_BATTLE_MON_MOVES), src + off(W_BATTLE_MON_MOVES), u16::from(NUM_MOVES));
        // DVs
        let dvs = off(W_BATTLE_MON_D_VS);
        if !player {
            self.mem.wb(W_ENEMY_BACKUP_D_VS, self.mem.rb(dst + dvs));
            self.mem.wb(W_ENEMY_BACKUP_D_VS + 1, self.mem.rb(dst + dvs + 1));
        }
        self.mem.copy(dst + dvs, src + dvs, 2);
        // stats and types
        let stats = off(W_BATTLE_MON_STATS);
        self.mem.copy(dst + stats, src + stats, W_BATTLE_MON_STRUCT_END - W_BATTLE_MON_STATS);
        // init the power points
        let moves = off(W_BATTLE_MON_MOVES);
        let pp = off(W_BATTLE_MON_PP);
        for i in 0..u16::from(NUM_MOVES) {
            let m = self.mem.rb(src + moves + i);
            let v = if m == 0 {
                0
            } else if m == SKETCH {
                1
            } else {
                5
            };
            self.mem.wb(dst + pp + i, v);
        }
        self.mem.wb(W_NAMED_OBJECT_INDEX, species);
        self.get_pokemon_name();
        // BattleSideCopy: the user's copy of the opponent's stats and stat levels.
        let (user_stats, foe_stats) = if player { (W_PLAYER_STATS, W_ENEMY_STATS) } else { (W_ENEMY_STATS, W_PLAYER_STATS) };
        self.mem.copy(user_stats, foe_stats, 2 * 5);
        let (user_levels, foe_levels) =
            if player { (W_PLAYER_STAT_LEVELS, W_ENEMY_STAT_LEVELS) } else { (W_ENEMY_STAT_LEVELS, W_PLAYER_STAT_LEVELS) };
        self.mem.copy(user_levels, foe_levels, 8);
        let minimized = self.mem.rb(if player { W_PLAYER_MINIMIZED } else { W_ENEMY_MINIMIZED });
        if !self.check_battle_scene() && minimized == 0 {
            self.load_move_anim();
        } else {
            // .mimic_anims
            let _ = self.battle_command_move_delay();
            let _ = self.battle_command_raise_sub_no_anim();
        }
        self.mem.wb(W_BATTLE_AFTER_ANIM, 0);
        self.mem.wb(W_FX_ANIM_ID + 1, 0);
        self.mem.wb(W_BATTLE_ANIM_PARAM, 2);
        if had_sub {
            self.load_anim(SUBSTITUTE);
        }
        self.text(r::TRANSFORMED_TEXT);
        Ok(Cmd::Next)
    }

    // --- triple_kick.asm ---------------------------------------------------------------

    /// `BattleCommand_TripleKick`.
    pub fn battle_command_triple_kick(&mut self) -> Flow<Cmd> {
        let mut b = self.mem.rb(W_BATTLE_ANIM_PARAM).wrapping_add(1);
        let de = self.mem.rw(W_CUR_DAMAGE);
        loop {
            b = b.wrapping_sub(1);
            if b == 0 {
                return Ok(Cmd::Next);
            }
            let (sum, carry) = self.mem.rw(W_CUR_DAMAGE).overflowing_add(de);
            self.mem.ww(W_CUR_DAMAGE, sum);
            if carry {
                self.mem.wb(W_CUR_DAMAGE, 0xff);
                self.mem.wb(W_CUR_DAMAGE + 1, 0xff);
                return Ok(Cmd::Next);
            }
        }
    }

    /// `BattleCommand_KickCounter`.
    pub fn battle_command_kick_counter(&mut self) -> Flow<Cmd> {
        let v = self.mem.rb(W_BATTLE_ANIM_PARAM).wrapping_add(1);
        self.mem.wb(W_BATTLE_ANIM_PARAM, v);
        Ok(Cmd::Next)
    }
}
