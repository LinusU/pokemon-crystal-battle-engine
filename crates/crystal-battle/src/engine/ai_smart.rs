//! `AI_Smart` and its per-effect handlers (`engine/battle/ai/scoring.asm`).
//!
//! Every handler takes `hl`, the address of the current move's score in
//! `wEnemyAIMoveScores`, and adjusts it. Carry flags returned by the helpers
//! in `ai.rs` are `bool`s (`true` = carry).

#![allow(unused_imports)]

use super::ai::SCORES;
use super::battle::{Engine, PARTY_LEN, percent};
use super::rsym as r;
use super::sym::*;
use crate::defs::*;

impl Engine {
    /// `AI_Smart`: context-specific scoring.
    pub fn ai_smart(&mut self) {
        for i in 0..NUM_MOVES {
            let hl = SCORES + u16::from(i);
            let mv = self.mem.rb(W_ENEMY_MON_MOVES + u16::from(i));
            if mv == 0 {
                return;
            }
            self.ai_get_enemy_move(mv);
            let effect = self.mem.rb(W_ENEMY_MOVE_STRUCT + u16::from(MOVE_EFFECT));
            self.ai_smart_dispatch(effect, hl);
        }
    }

    /// `AI_Smart_EffectHandlers`.
    fn ai_smart_dispatch(&mut self, effect: u8, hl: u16) {
        match effect {
            EFFECT_SLEEP => self.ai_smart_sleep(hl),
            EFFECT_LEECH_HIT => self.ai_smart_leech_hit(hl),
            EFFECT_SELFDESTRUCT => self.ai_smart_selfdestruct(hl),
            EFFECT_DREAM_EATER => self.ai_smart_dream_eater(hl),
            EFFECT_MIRROR_MOVE => self.ai_smart_mirror_move(hl),
            EFFECT_EVASION_UP => self.ai_smart_evasion_up(hl),
            EFFECT_ALWAYS_HIT => self.ai_smart_always_hit(hl),
            EFFECT_ACCURACY_DOWN => self.ai_smart_accuracy_down(hl),
            EFFECT_RESET_STATS => self.ai_smart_reset_stats(hl),
            EFFECT_BIDE => self.ai_smart_bide(hl),
            EFFECT_FORCE_SWITCH => self.ai_smart_force_switch(hl),
            EFFECT_HEAL | EFFECT_MORNING_SUN | EFFECT_SYNTHESIS | EFFECT_MOONLIGHT => self.ai_smart_heal(hl),
            EFFECT_TOXIC | EFFECT_LEECH_SEED => self.ai_smart_toxic(hl),
            EFFECT_LIGHT_SCREEN | EFFECT_REFLECT => self.ai_smart_light_screen(hl),
            EFFECT_OHKO => self.ai_smart_ohko(hl),
            EFFECT_RAZOR_WIND | EFFECT_UNUSED_2B => self.ai_smart_razor_wind(hl),
            EFFECT_SUPER_FANG => self.ai_smart_super_fang(hl),
            EFFECT_TRAP_TARGET => self.ai_smart_trap_target(hl),
            EFFECT_CONFUSE => self.ai_smart_confuse(hl),
            EFFECT_SP_DEF_UP_2 => self.ai_smart_sp_defense_up2(hl),
            EFFECT_PARALYZE => self.ai_smart_paralyze(hl),
            EFFECT_SPEED_DOWN_HIT => self.ai_smart_speed_down_hit(hl),
            EFFECT_SUBSTITUTE => self.ai_smart_substitute(hl),
            EFFECT_HYPER_BEAM => self.ai_smart_hyper_beam(hl),
            EFFECT_RAGE => self.ai_smart_rage(hl),
            EFFECT_MIMIC => self.ai_smart_mimic(hl),
            EFFECT_DISABLE => self.ai_smart_disable(hl),
            EFFECT_COUNTER => self.ai_smart_counter(hl),
            EFFECT_ENCORE => self.ai_smart_encore(hl),
            EFFECT_PAIN_SPLIT => self.ai_smart_pain_split(hl),
            EFFECT_SNORE | EFFECT_SLEEP_TALK => self.ai_smart_snore(hl),
            EFFECT_CONVERSION2 => self.ai_smart_conversion2(hl),
            EFFECT_LOCK_ON => self.ai_smart_lock_on(hl),
            EFFECT_DEFROST_OPPONENT => self.ai_smart_defrost_opponent(hl),
            EFFECT_DESTINY_BOND | EFFECT_REVERSAL | EFFECT_SKULL_BASH => self.ai_smart_destiny_bond(hl),
            EFFECT_SPITE => self.ai_smart_spite(hl),
            EFFECT_HEAL_BELL => self.ai_smart_heal_bell(hl),
            EFFECT_PRIORITY_HIT => self.ai_smart_priority_hit(hl),
            EFFECT_THIEF => self.ai_smart_thief(hl),
            EFFECT_MEAN_LOOK => self.ai_smart_mean_look(hl),
            EFFECT_NIGHTMARE => self.ai_smart_nightmare(hl),
            EFFECT_FLAME_WHEEL => self.ai_smart_flame_wheel(hl),
            EFFECT_CURSE => self.ai_smart_curse(hl),
            EFFECT_PROTECT => self.ai_smart_protect(hl),
            EFFECT_FORESIGHT => self.ai_smart_foresight(hl),
            EFFECT_PERISH_SONG => self.ai_smart_perish_song(hl),
            EFFECT_SANDSTORM => self.ai_smart_sandstorm(hl),
            EFFECT_ENDURE => self.ai_smart_endure(hl),
            EFFECT_ROLLOUT => self.ai_smart_rollout(hl),
            EFFECT_SWAGGER | EFFECT_ATTRACT => self.ai_smart_swagger(hl),
            EFFECT_FURY_CUTTER => self.ai_smart_fury_cutter(hl),
            EFFECT_SAFEGUARD => self.ai_smart_safeguard(hl),
            EFFECT_MAGNITUDE | EFFECT_EARTHQUAKE => self.ai_smart_magnitude(hl),
            EFFECT_BATON_PASS => self.ai_smart_baton_pass(hl),
            EFFECT_PURSUIT => self.ai_smart_pursuit(hl),
            EFFECT_RAPID_SPIN => self.ai_smart_rapid_spin(hl),
            EFFECT_HIDDEN_POWER => self.ai_smart_hidden_power(hl),
            EFFECT_RAIN_DANCE => self.ai_smart_rain_dance(hl),
            EFFECT_SUNNY_DAY => self.ai_smart_sunny_day(hl),
            EFFECT_BELLY_DRUM => self.ai_smart_belly_drum(hl),
            EFFECT_PSYCH_UP => self.ai_smart_psych_up(hl),
            EFFECT_MIRROR_COAT => self.ai_smart_mirror_coat(hl),
            EFFECT_TWISTER | EFFECT_GUST => self.ai_smart_twister(hl),
            EFFECT_FUTURE_SIGHT => self.ai_smart_future_sight(hl),
            EFFECT_STOMP => self.ai_smart_stomp(hl),
            EFFECT_SOLARBEAM => self.ai_smart_solarbeam(hl),
            EFFECT_THUNDER => self.ai_smart_thunder(hl),
            EFFECT_FLY => self.ai_smart_fly(hl),
            _ => {}
        }
    }

    /// `BattleCheckTypeMatchup` with `hBattleTurn` set to the enemy's.
    fn ai_enemy_type_matchup(&mut self) {
        self.mem.wb(H_BATTLE_TURN, 1);
        self.battle_check_type_matchup();
    }

    /// `AI_Smart_Sleep`: greatly encourage sleep inducing moves if the enemy
    /// has either Dream Eater or Nightmare; 50% chance to greatly encourage
    /// sleep inducing moves otherwise.
    fn ai_smart_sleep(&mut self, hl: u16) {
        if !self.ai_has_move_effect(EFFECT_DREAM_EATER) && !self.ai_has_move_effect(EFFECT_NIGHTMARE) {
            return;
        }
        // .encourage
        if self.ai_50_50() {
            return;
        }
        self.ai_dec(hl, 2);
    }

    /// `AI_Smart_LeechHit`.
    fn ai_smart_leech_hit(&mut self, hl: u16) {
        self.ai_enemy_type_matchup();

        // 60% chance to discourage this move if not very effective.
        let m = self.mem.rb(W_TYPE_MATCHUP);
        if m < EFFECTIVE {
            if self.random("AI_Smart_LeechHit") < percent(39) + 1 {
                return;
            }
            self.ai_inc(hl, 1);
            return;
        }
        // Do nothing if effectiveness is neutral.
        if m == EFFECTIVE {
            return;
        }
        // Do nothing if enemy's HP is full.
        if self.ai_check_enemy_max_hp() {
            return;
        }
        // 80% chance to encourage this move otherwise.
        if self.ai_80_20() {
            return;
        }
        self.ai_dec(hl, 1);
    }

    /// `AI_Smart_LockOn`.
    fn ai_smart_lock_on(&mut self, hl: u16) {
        if self.mem.bit(W_PLAYER_SUB_STATUS5, SUBSTATUS_LOCK_ON) {
            // .player_locked_on
            for i in 0..NUM_MOVES {
                let mv = self.mem.rb(W_ENEMY_MON_MOVES + u16::from(i));
                if mv == 0 {
                    break;
                }
                self.ai_get_enemy_move(mv);
                if self.mem.rb(W_ENEMY_MOVE_STRUCT + u16::from(MOVE_ACC)) >= percent(71) - 1 {
                    continue;
                }
                self.ai_dec(SCORES + u16::from(i), 2);
            }
            // .dismiss
            self.ai_discourage_move(hl);
            return;
        }

        enum Outcome {
            Discourage,
            MaybeEncourage,
            DoNothing,
        }
        let outcome = 'out: {
            if !self.ai_check_enemy_quarter_hp() {
                break 'out Outcome::Discourage;
            }
            if !self.ai_check_enemy_half_hp() && !self.ai_compare_speed() {
                break 'out Outcome::Discourage;
            }
            // .skip_speed_check
            let eva = self.mem.rb(W_PLAYER_EVA_LEVEL);
            if eva >= BASE_STAT_LEVEL + 3 {
                break 'out Outcome::MaybeEncourage;
            }
            if eva >= BASE_STAT_LEVEL + 1 {
                break 'out Outcome::DoNothing;
            }
            let acc = self.mem.rb(W_ENEMY_ACC_LEVEL);
            if acc < BASE_STAT_LEVEL - 2 {
                break 'out Outcome::MaybeEncourage;
            }
            if acc < BASE_STAT_LEVEL {
                break 'out Outcome::DoNothing;
            }
            for i in 0..NUM_MOVES {
                let mv = self.mem.rb(W_ENEMY_MON_MOVES + u16::from(i));
                if mv == 0 {
                    break 'out Outcome::Discourage;
                }
                self.ai_get_enemy_move(mv);
                if self.mem.rb(W_ENEMY_MOVE_STRUCT + u16::from(MOVE_ACC)) >= percent(71) - 1 {
                    continue;
                }
                self.ai_enemy_type_matchup();
                if self.mem.rb(W_TYPE_MATCHUP) < EFFECTIVE {
                    continue;
                }
                break 'out Outcome::DoNothing;
            }
            Outcome::Discourage
        };
        match outcome {
            Outcome::DoNothing => {}
            Outcome::Discourage => self.ai_inc(hl, 1),
            Outcome::MaybeEncourage => {
                if self.ai_50_50() {
                    return;
                }
                self.ai_dec(hl, 2);
            }
        }
    }

    /// `AI_Smart_Selfdestruct`: Selfdestruct, Explosion.
    fn ai_smart_selfdestruct(&mut self, hl: u16) {
        // Unless this is the enemy's last Pokemon...
        let (last_mon, _) = self.find_alive_enemy_mons();
        let discourage = if last_mon {
            // ...greatly discourage this move unless this is the player's last Pokemon too.
            self.ai_check_last_player_mon()
        } else {
            false
        };
        if !discourage {
            // .notlastmon: greatly discourage this move if enemy's HP is above 50%.
            if !self.ai_check_enemy_half_hp() {
                // Do nothing if enemy's HP is below 25%.
                if !self.ai_check_enemy_quarter_hp() {
                    return;
                }
                // If enemy's HP is between 25% and 50%, over 90% chance to greatly discourage this move.
                if self.random("AI_Smart_Selfdestruct") < percent(8) {
                    return;
                }
            }
        }
        self.ai_inc(hl, 3);
    }

    /// `AI_Smart_DreamEater`: 90% chance to greatly encourage this move.
    fn ai_smart_dream_eater(&mut self, hl: u16) {
        if self.random("AI_Smart_DreamEater") < percent(10) {
            return;
        }
        self.ai_dec(hl, 3);
    }

    /// `AI_Smart_EvasionUp`.
    fn ai_smart_evasion_up(&mut self, hl: u16) {
        // Dismiss this move if enemy's evasion can't raise anymore.
        if self.mem.rb(W_ENEMY_EVA_LEVEL) >= MAX_STAT_LEVEL {
            return self.ai_discourage_move(hl);
        }
        self.ai_hp_context_encourage(hl, true);
    }

    /// The body shared (up to the HP checks being on the enemy's or the
    /// player's side) by `AI_Smart_EvasionUp` and `AI_Smart_AccuracyDown`.
    fn ai_hp_context_encourage(&mut self, hl: u16, enemy: bool) {
        let max_hp = |e: &Engine| if enemy { e.ai_check_enemy_max_hp() } else { e.ai_check_player_max_hp() };
        let quarter = |e: &Engine| if enemy { e.ai_check_enemy_quarter_hp() } else { e.ai_check_player_quarter_hp() };
        let half = |e: &Engine| if enemy { e.ai_check_enemy_half_hp() } else { e.ai_check_player_half_hp() };

        // 0 = greatly encourage, 1 = not encouraged, 2 = hp_mismatch_2 (discourage twice first).
        enum Step {
            GreatlyEncourage,
            NotEncouraged,
            Mismatch2,
        }
        let step = 'out: {
            // If the HP is full (and, for Accuracy Down, the enemy's HP is above 50%)...
            let full = if enemy { max_hp(self) } else { max_hp(self) && half_of_enemy(self) };
            if full {
                // ...greatly encourage this move if player is badly poisoned.
                if self.mem.bit(W_PLAYER_SUB_STATUS5, SUBSTATUS_TOXIC) {
                    break 'out Step::GreatlyEncourage;
                }
                // ...70% chance to greatly encourage this move if player is not badly poisoned.
                let name = if enemy { "AI_Smart_EvasionUp" } else { "AI_Smart_AccuracyDown" };
                if self.random(name) >= percent(70) {
                    break 'out Step::NotEncouraged;
                }
                break 'out Step::GreatlyEncourage;
            }
            // .hp_mismatch_1: greatly discourage this move if HP is below 25%.
            if !quarter(self) {
                break 'out Step::Mismatch2;
            }
            // If HP is above 25% but not full, 4% chance to greatly encourage this move.
            let name = if enemy { "AI_Smart_EvasionUp" } else { "AI_Smart_AccuracyDown" };
            if self.random(name) < percent(4) {
                break 'out Step::GreatlyEncourage;
            }
            // If HP is between 25% and 50%,...
            if !half(self) {
                // .hp_mismatch_3: ...50% chance to greatly discourage this move.
                if self.ai_50_50() {
                    break 'out Step::NotEncouraged;
                }
                break 'out Step::Mismatch2;
            }
            // If HP is above 50% but not full, 20% chance to greatly encourage this move.
            if self.ai_80_20() {
                break 'out Step::GreatlyEncourage;
            }
            Step::NotEncouraged
        };

        fn half_of_enemy(e: &Engine) -> bool {
            // AccuracyDown: `AICheckEnemyHalfHP; jr nc, .hp_mismatch_1`.
            e.ai_check_enemy_half_hp()
        }

        match step {
            Step::GreatlyEncourage => {
                self.ai_dec(hl, 2);
                return;
            }
            Step::Mismatch2 => self.ai_inc(hl, 2),
            Step::NotEncouraged => {}
        }

        // .not_encouraged
        if self.mem.bit(W_PLAYER_SUB_STATUS5, SUBSTATUS_TOXIC) {
            // .maybe_greatly_encourage: 70% chance to greatly encourage this move.
            let name = if enemy { "AI_Smart_EvasionUp" } else { "AI_Smart_AccuracyDown" };
            if self.random(name) < percent(31) + 1 {
                return;
            }
            self.ai_dec(hl, 2);
            return;
        }
        if self.mem.bit(W_PLAYER_SUB_STATUS4, SUBSTATUS_LEECH_SEED) {
            // Player is seeded: 50% chance to encourage this move.
            if self.ai_50_50() {
                return;
            }
            self.ai_dec(hl, 1);
            return;
        }
        // Discourage this move if enemy's evasion level is higher than player's accuracy level.
        if self.mem.rb(W_PLAYER_ACC_LEVEL) < self.mem.rb(W_ENEMY_EVA_LEVEL) {
            self.ai_inc(hl, 1);
            return;
        }
        // Greatly encourage this move if the player is in the middle of Fury Cutter or Rollout.
        if self.mem.rb(W_PLAYER_FURY_CUTTER_COUNT) != 0 || self.mem.bit(W_PLAYER_SUB_STATUS1, SUBSTATUS_ROLLOUT) {
            self.ai_dec(hl, 2);
            return;
        }
        self.ai_inc(hl, 1);
    }

    /// `AI_Smart_AlwaysHit`.
    fn ai_smart_always_hit(&mut self, hl: u16) {
        // 80% chance to greatly encourage this move if either enemy's accuracy level has been
        // lowered three or more stages...
        if self.mem.rb(W_ENEMY_ACC_LEVEL) >= BASE_STAT_LEVEL - 2 {
            // ...or player's evasion level has been raised three or more stages.
            if self.mem.rb(W_PLAYER_EVA_LEVEL) < BASE_STAT_LEVEL + 3 {
                return;
            }
        }
        if self.ai_80_20() {
            return;
        }
        self.ai_dec(hl, 2);
    }

    /// `AI_Smart_MirrorMove`.
    fn ai_smart_mirror_move(&mut self, hl: u16) {
        // If the player did not use any move last turn...
        let last = self.mem.rb(W_LAST_PLAYER_COUNTER_MOVE);
        if last == 0 {
            // ...do nothing if enemy is slower than player
            if !self.ai_compare_speed() {
                return;
            }
            // ...or dismiss this move if enemy is faster than player.
            return self.ai_discourage_move(hl);
        }
        // If the player did use a move last turn, do nothing if he didn't use a useful move.
        if !self.data.tables.useful_moves.contains(&last) {
            return;
        }
        // If he did, 50% chance to encourage this move...
        if self.ai_50_50() {
            return;
        }
        self.ai_dec(hl, 1);
        // ...and 90% chance to encourage this move again if the enemy is faster.
        if !self.ai_compare_speed() {
            return;
        }
        if self.random("AI_Smart_MirrorMove") < percent(10) {
            return;
        }
        self.ai_dec(hl, 1);
    }

    /// `AI_Smart_AccuracyDown`.
    fn ai_smart_accuracy_down(&mut self, hl: u16) {
        self.ai_hp_context_encourage(hl, false);
    }

    /// `AI_Smart_ResetStats`.
    fn ai_smart_reset_stats(&mut self, hl: u16) {
        // 85% chance to encourage this move if any of enemy's stat levels is lower than -2.
        let mut encourage = false;
        for i in 0..u16::from(NUM_LEVEL_STATS - 1) {
            if self.mem.rb(W_ENEMY_ATK_LEVEL + i) < BASE_STAT_LEVEL - 2 {
                encourage = true;
                break;
            }
        }
        // 85% chance to encourage this move if any of player's stat levels is higher than +2.
        if !encourage {
            for i in 0..u16::from(NUM_LEVEL_STATS - 1) {
                if self.mem.rb(W_PLAYER_ATK_LEVEL + i) >= BASE_STAT_LEVEL + 3 {
                    encourage = true;
                    break;
                }
            }
        }
        if encourage {
            if self.random("AI_Smart_ResetStats") < percent(16) {
                return;
            }
            self.ai_dec(hl, 1);
        } else {
            // Discourage this move if neither.
            self.ai_inc(hl, 1);
        }
    }

    /// `AI_Smart_Bide`: 90% chance to discourage this move unless enemy's HP is full.
    fn ai_smart_bide(&mut self, hl: u16) {
        if self.ai_check_enemy_max_hp() {
            return;
        }
        if self.random("AI_Smart_Bide") < percent(10) {
            return;
        }
        self.ai_inc(hl, 1);
    }

    /// `AI_Smart_ForceSwitch`: Whirlwind, Roar.
    fn ai_smart_force_switch(&mut self, hl: u16) {
        self.check_player_move_type_matchups();
        if self.mem.rb(W_ENEMY_AI_SWITCH_SCORE) < BASE_AI_SWITCH_SCORE {
            return;
        }
        self.ai_inc(hl, 1);
    }

    /// `AI_Smart_Heal` (also Morning Sun, Synthesis, Moonlight).
    fn ai_smart_heal(&mut self, hl: u16) {
        // 90% chance to greatly encourage this move if enemy's HP is below 25%.
        if self.ai_check_enemy_quarter_hp() {
            // Discourage this move if enemy's HP is higher than 50%.
            if !self.ai_check_enemy_half_hp() {
                return;
            }
            self.ai_inc(hl, 1);
            return;
        }
        if self.random("AI_Smart_Heal") < percent(10) {
            return;
        }
        self.ai_dec(hl, 2);
    }

    /// `AI_Smart_Toxic` (also Leech Seed): discourage this move if player's HP is below 50%.
    fn ai_smart_toxic(&mut self, hl: u16) {
        if self.ai_check_player_half_hp() {
            return;
        }
        self.ai_inc(hl, 1);
    }

    /// `AI_Smart_LightScreen` (also Reflect): over 90% chance to discourage
    /// this move unless enemy's HP is full.
    fn ai_smart_light_screen(&mut self, hl: u16) {
        if self.ai_check_enemy_max_hp() {
            return;
        }
        if self.random("AI_Smart_LightScreen") < percent(8) {
            return;
        }
        self.ai_inc(hl, 1);
    }

    /// `AI_Smart_Ohko`: dismiss this move if player's level is higher than
    /// enemy's level; else, discourage this move is player's HP is below 50%.
    fn ai_smart_ohko(&mut self, hl: u16) {
        if self.mem.rb(W_ENEMY_MON_LEVEL) < self.mem.rb(W_BATTLE_MON_LEVEL) {
            return self.ai_discourage_move(hl);
        }
        if self.ai_check_player_half_hp() {
            return;
        }
        self.ai_inc(hl, 1);
    }

    /// `AI_Smart_TrapTarget`: Bind, Wrap, Fire Spin, Clamp.
    fn ai_smart_trap_target(&mut self, hl: u16) {
        // 50% chance to discourage this move if the player is already trapped.
        let encourage = if self.mem.rb(W_PLAYER_WRAP_COUNT) != 0 {
            false
        } else if self.mem.bit(W_PLAYER_SUB_STATUS5, SUBSTATUS_TOXIC) {
            // 50% chance to greatly encourage this move if player is either badly poisoned,
            // in love, identified, stuck in Rollout, or has a Nightmare.
            true
        } else if self.mem.rb(W_PLAYER_SUB_STATUS1)
            & (1 << SUBSTATUS_IN_LOVE | 1 << SUBSTATUS_ROLLOUT | 1 << SUBSTATUS_IDENTIFIED | 1 << SUBSTATUS_NIGHTMARE)
            != 0
        {
            true
        } else {
            // Else, 50% chance to greatly encourage this move if it's the player's Pokemon first turn.
            self.mem.rb(W_PLAYER_TURNS_TAKEN) == 0
        };
        if !encourage {
            // .discourage
            if self.ai_50_50() {
                return;
            }
            self.ai_inc(hl, 1);
            return;
        }
        // .encourage
        if !self.ai_check_enemy_quarter_hp() {
            return;
        }
        if self.ai_50_50() {
            return;
        }
        self.ai_dec(hl, 2);
    }

    /// `AI_Smart_RazorWind` (also `AI_Smart_Unused2B`).
    fn ai_smart_razor_wind(&mut self, hl: u16) {
        if self.mem.bit(W_ENEMY_SUB_STATUS1, SUBSTATUS_PERISH) && self.mem.rb(W_ENEMY_PERISH_COUNT) < 3 {
            // .discourage
            return self.ai_inc(hl, 1);
        }
        // .no_perish_count
        for i in 0..u16::from(NUM_MOVES) {
            let mv = self.mem.rb(W_PLAYER_USED_MOVES + i);
            if mv == 0 {
                break;
            }
            self.ai_get_enemy_move(mv);
            if self.mem.rb(W_ENEMY_MOVE_STRUCT + u16::from(MOVE_EFFECT)) == EFFECT_PROTECT {
                // .dismiss
                self.ai_inc(hl, 6);
                return;
            }
        }
        // .movesdone
        if !self.mem.bit(W_ENEMY_SUB_STATUS3, SUBSTATUS_CONFUSED) && self.ai_check_enemy_half_hp() {
            return;
        }
        // .maybe_discourage
        if self.random("AI_Smart_RazorWind") < percent(79) - 1 {
            return;
        }
        self.ai_inc(hl, 1);
    }

    /// `AI_Smart_Confuse`: 90% chance to discourage this move if player's HP
    /// is between 25% and 50%.
    fn ai_smart_confuse(&mut self, hl: u16) {
        if self.ai_check_player_half_hp() {
            return;
        }
        if self.random("AI_Smart_Confuse") >= percent(10) {
            self.ai_inc(hl, 1);
        }
        // .discourage: discourage again if player's HP is below 25%.
        if self.ai_check_player_quarter_hp() {
            return;
        }
        self.ai_inc(hl, 1);
    }

    /// `AI_Smart_SpDefenseUp2`.
    fn ai_smart_sp_defense_up2(&mut self, hl: u16) {
        // Discourage this move if enemy's HP is lower than 50%.
        if !self.ai_check_enemy_half_hp() {
            return self.ai_inc(hl, 1);
        }
        // Discourage this move if enemy's special defense level is higher than +3.
        let a = self.mem.rb(W_ENEMY_S_DEF_LEVEL);
        if a >= BASE_STAT_LEVEL + 4 {
            return self.ai_inc(hl, 1);
        }
        // 80% chance to greatly encourage this move if enemy's Special Defense level is lower
        // than +2, and the player is of a special type.
        if a >= BASE_STAT_LEVEL + 2 {
            return;
        }
        let t1 = self.mem.rb(W_BATTLE_MON_TYPE1);
        if self.data.rules.ghost_special_dark_physical && t1 == DARK {
            return;
        }
        let special_type1 = t1 >= SPECIAL || (self.data.rules.ghost_special_dark_physical && t1 == GHOST);
        if !special_type1 && self.mem.rb(W_BATTLE_MON_TYPE2) < SPECIAL {
            return;
        }
        // .encourage
        if self.ai_80_20() {
            return;
        }
        self.ai_dec(hl, 2);
    }

    /// `AI_Smart_Fly`: Fly, Dig.
    fn ai_smart_fly(&mut self, hl: u16) {
        // Greatly encourage this move if the player is flying or underground, and slower than the enemy.
        if self.mem.rb(W_PLAYER_SUB_STATUS3) & (1 << SUBSTATUS_FLYING | 1 << SUBSTATUS_UNDERGROUND) == 0 {
            return;
        }
        if !self.ai_compare_speed() {
            return;
        }
        self.ai_dec(hl, 3);
    }

    /// `AI_Smart_SuperFang`: discourage this move if player's HP is below 25%.
    fn ai_smart_super_fang(&mut self, hl: u16) {
        if self.ai_check_player_quarter_hp() {
            return;
        }
        self.ai_inc(hl, 1);
    }

    /// `AI_Smart_Paralyze`.
    fn ai_smart_paralyze(&mut self, hl: u16) {
        // 50% chance to discourage this move if player's HP is below 25%.
        if !self.ai_check_player_quarter_hp() {
            if self.ai_50_50() {
                return;
            }
            return self.ai_inc(hl, 1);
        }
        // 80% chance to greatly encourage this move if enemy is slower than player and its HP
        // is above 25%.
        if self.ai_compare_speed() {
            return;
        }
        if !self.ai_check_enemy_quarter_hp() {
            return;
        }
        if self.ai_80_20() {
            return;
        }
        self.ai_dec(hl, 2);
    }

    /// `AI_Smart_SpeedDownHit`: Icy Wind.
    fn ai_smart_speed_down_hit(&mut self, hl: u16) {
        if self.mem.rb(W_ENEMY_MOVE_STRUCT + u16::from(MOVE_ANIM)) != ICY_WIND {
            return;
        }
        if !self.ai_check_enemy_quarter_hp() {
            return;
        }
        if self.mem.rb(W_PLAYER_TURNS_TAKEN) != 0 {
            return;
        }
        if self.ai_compare_speed() {
            return;
        }
        if self.random("AI_Smart_SpeedDownHit") < percent(12) {
            return;
        }
        self.ai_dec(hl, 2);
    }

    /// `AI_Smart_Substitute`: dismiss this move if enemy's HP is below 50%.
    fn ai_smart_substitute(&mut self, hl: u16) {
        if self.ai_check_enemy_half_hp() {
            return;
        }
        self.ai_discourage_move(hl);
    }

    /// `AI_Smart_HyperBeam`.
    fn ai_smart_hyper_beam(&mut self, hl: u16) {
        if self.ai_check_enemy_half_hp() {
            // .discourage: if enemy's HP is above 50%, discourage this move at random.
            if self.random("AI_Smart_HyperBeam") < percent(16) {
                return;
            }
            self.ai_inc(hl, 1);
            if self.ai_50_50() {
                return;
            }
            self.ai_inc(hl, 1);
            return;
        }
        // 50% chance to encourage this move if enemy's HP is below 25%.
        if self.ai_check_enemy_quarter_hp() {
            return;
        }
        if self.ai_50_50() {
            return;
        }
        self.ai_dec(hl, 1);
    }

    /// `AI_Smart_Rage`.
    fn ai_smart_rage(&mut self, hl: u16) {
        if self.mem.bit(W_ENEMY_SUB_STATUS4, SUBSTATUS_RAGE) {
            // If enemy's Rage is building, 50% chance to encourage this move.
            if !self.ai_50_50() {
                self.ai_dec(hl, 1);
            }
            // .encourage: encourage this move based on Rage's counter.
            if self.mem.rb(W_ENEMY_RAGE_COUNTER) < 2 {
                return;
            }
            self.ai_dec(hl, 1);
            if self.mem.rb(W_ENEMY_RAGE_COUNTER) < 3 {
                return;
            }
            self.ai_dec(hl, 1);
            return;
        }
        // .notbuilding: discourage this move if enemy's HP is below 50%.
        if !self.ai_check_enemy_half_hp() {
            return self.ai_inc(hl, 1);
        }
        // 20% chance to encourage this move otherwise.
        if !self.ai_80_20() {
            return;
        }
        self.ai_dec(hl, 1);
    }

    /// `AI_Smart_Mimic`.
    fn ai_smart_mimic(&mut self, hl: u16) {
        let last = self.mem.rb(W_LAST_PLAYER_COUNTER_MOVE);
        if last == 0 {
            // .dismiss: dismiss this move if the enemy is faster than the player.
            if self.ai_compare_speed() {
                return self.ai_discourage_move(hl);
            }
            return self.ai_inc(hl, 1);
        }
        if !self.ai_check_enemy_half_hp() {
            return self.ai_inc(hl, 1);
        }
        self.ai_get_enemy_move(last);
        self.ai_enemy_type_matchup();
        let m = self.mem.rb(W_TYPE_MATCHUP);
        if m < EFFECTIVE {
            return self.ai_inc(hl, 1);
        }
        if m != EFFECTIVE {
            if !self.ai_50_50() {
                self.ai_dec(hl, 1);
            }
        }
        // .encourage
        let last = self.mem.rb(W_LAST_PLAYER_COUNTER_MOVE);
        if !self.data.tables.useful_moves.contains(&last) {
            return;
        }
        if self.ai_50_50() {
            return;
        }
        self.ai_dec(hl, 1);
    }

    /// Shared scan of `wPlayerUsedMoves` of `AI_Smart_Counter` (`special` =
    /// false) and `AI_Smart_MirrorCoat` (`special` = true).
    fn ai_smart_counter_like(&mut self, hl: u16, special: bool, name: &'static str) {
        // `last`: the check of the player's last move. In Crystal Legacy's
        // Mirror Coat it treats Ghost as physical and Dark as special (it
        // copies Counter's), unlike its scan of the player's moves.
        let is_wanted = |e: &Engine, last: bool| -> bool {
            let power = e.mem.rb(W_ENEMY_MOVE_STRUCT + u16::from(MOVE_POWER));
            if power == 0 {
                return false;
            }
            let t = e.mem.rb(W_ENEMY_MOVE_STRUCT + u16::from(MOVE_TYPE));
            if e.data.rules.ghost_special_dark_physical && (t == GHOST || t == DARK) {
                return if special && !last { t == GHOST } else { t == DARK };
            }
            if special { t >= SPECIAL } else { t < SPECIAL }
        };
        let mut b = 0u8;
        for i in 0..u16::from(NUM_MOVES) {
            let mv = self.mem.rb(W_PLAYER_USED_MOVES + i);
            if mv == 0 {
                continue;
            }
            self.ai_get_enemy_move(mv);
            if is_wanted(self, false) {
                b += 1;
            }
        }
        if b == 0 {
            // .discourage
            return self.ai_inc(hl, 1);
        }
        if b < 3 {
            let last = self.mem.rb(W_LAST_PLAYER_COUNTER_MOVE);
            if last == 0 {
                return;
            }
            self.ai_get_enemy_move(last);
            if !is_wanted(self, true) {
                return;
            }
        }
        // .encourage
        if self.random(name) < percent(39) + 1 {
            return;
        }
        self.ai_dec(hl, 1);
    }

    /// `AI_Smart_Counter`.
    fn ai_smart_counter(&mut self, hl: u16) {
        self.ai_smart_counter_like(hl, false, "AI_Smart_Counter");
    }

    /// `AI_Smart_MirrorCoat`.
    fn ai_smart_mirror_coat(&mut self, hl: u16) {
        self.ai_smart_counter_like(hl, true, "AI_Smart_MirrorCoat");
    }

    /// `AI_Smart_Disable`.
    fn ai_smart_disable(&mut self, hl: u16) {
        let discourage = if !self.ai_compare_speed() {
            true
        } else if self.data.tables.useful_moves.contains(&self.mem.rb(W_LAST_PLAYER_COUNTER_MOVE)) {
            if self.random("AI_Smart_Disable") < percent(39) + 1 {
                return;
            }
            return self.ai_dec(hl, 1);
        } else {
            // .notencourage
            if self.mem.rb(W_ENEMY_MOVE_STRUCT + u16::from(MOVE_POWER)) != 0 {
                return;
            }
            true
        };
        if discourage {
            // .discourage
            if self.random("AI_Smart_Disable") < percent(8) {
                return;
            }
            self.ai_inc(hl, 1);
        }
    }

    /// `AI_Smart_Encore`.
    fn ai_smart_encore(&mut self, hl: u16) {
        if !self.ai_compare_speed() {
            return self.ai_inc(hl, 3);
        }
        let last = self.mem.rb(W_LAST_PLAYER_MOVE);
        if last == 0 {
            return self.ai_discourage_move(hl);
        }
        self.ai_get_enemy_move(last);

        let mut weak = true;
        if self.mem.rb(W_ENEMY_MOVE_STRUCT + u16::from(MOVE_POWER)) != 0 {
            self.check_type_matchup(W_ENEMY_MON_TYPE1);
            let m = self.mem.rb(W_TYPE_MATCHUP);
            if m < EFFECTIVE {
                if m != 0 {
                    return;
                }
                weak = false;
            }
        }
        if weak {
            // .weakmove
            let last = self.mem.rb(W_LAST_PLAYER_COUNTER_MOVE);
            if !self.data.tables.encore_moves.contains(&last) {
                return self.ai_inc(hl, 3);
            }
        }
        // .encourage
        if self.random("AI_Smart_Encore") < percent(28) - 1 {
            return;
        }
        self.ai_dec(hl, 2);
    }

    /// `AI_Smart_PainSplit`: discourage this move if enemy's current HP * 2 > player's current HP.
    fn ai_smart_pain_split(&mut self, hl: u16) {
        let enemy_x2 = self.mem.rw(W_ENEMY_MON_HP) << 1;
        // `ret nc`: player's HP >= enemy's HP * 2.
        if self.mem.rw(W_BATTLE_MON_HP) >= enemy_x2 {
            return;
        }
        self.ai_inc(hl, 1);
    }

    /// `AI_Smart_Snore` (also Sleep Talk): greatly encourage this move if
    /// enemy is fast asleep; greatly discourage this move otherwise.
    fn ai_smart_snore(&mut self, hl: u16) {
        if self.mem.rb(W_ENEMY_MON_STATUS) & SLP_MASK == 1 {
            self.ai_inc(hl, 3);
        } else {
            self.ai_dec(hl, 3);
        }
    }

    /// `AI_Smart_DefrostOpponent`: no move has this effect, so the layer is unused.
    fn ai_smart_defrost_opponent(&mut self, hl: u16) {
        if self.mem.rb(W_ENEMY_MON_STATUS) & 1 << FRZ == 0 {
            return;
        }
        self.ai_dec(hl, 3);
    }

    /// `AI_Smart_Spite`.
    fn ai_smart_spite(&mut self, hl: u16) {
        let last = self.mem.rb(W_LAST_PLAYER_COUNTER_MOVE);
        if last == 0 {
            if self.ai_compare_speed() {
                return self.ai_discourage_move(hl);
            }
            if self.ai_50_50() {
                return;
            }
            return self.ai_inc(hl, 1);
        }
        // .usedmove
        let mut found = None;
        for i in 0..u16::from(NUM_MOVES) {
            if self.mem.rb(W_BATTLE_MON_MOVES + i) == last {
                found = Some(self.mem.rb(W_BATTLE_MON_PP + i));
                break;
            }
        }
        let Some(pp) = found else { return };
        if pp < 6 {
            // .encourage
            if self.random("AI_Smart_Spite") < percent(39) + 1 {
                return;
            }
            return self.ai_dec(hl, 2);
        }
        if pp < 15 {
            if self.random("AI_Smart_Spite") >= percent(39) + 1 {
                return;
            }
        }
        // .discourage
        self.ai_inc(hl, 1);
    }

    /// `AI_Smart_DestinyBond` (also Reversal, Skull Bash): discourage this
    /// move if enemy's HP is above 25%.
    fn ai_smart_destiny_bond(&mut self, hl: u16) {
        if !self.ai_check_enemy_quarter_hp() {
            return;
        }
        self.ai_inc(hl, 1);
    }

    /// `AI_Smart_HealBell`.
    fn ai_smart_heal_bell(&mut self, hl: u16) {
        let mut c = 0u8;
        let mut mon = W_OT_PARTY_MON1_HP;
        for _ in 0..self.mem.rb(W_OT_PARTY_COUNT) {
            if (self.mem.rb(mon) | self.mem.rb(mon + 1)) != 0 {
                // status
                c |= self.mem.rb(mon - 2);
            }
            mon += PARTY_LEN;
        }
        if c == 0 {
            // .no_status
            if self.mem.rb(W_ENEMY_MON_STATUS) != 0 {
                return;
            }
            return self.ai_discourage_move(hl);
        }
        let status = self.mem.rb(W_ENEMY_MON_STATUS);
        if status != 0 {
            self.ai_dec(hl, 1);
        }
        // .ok
        if status & (1 << FRZ | SLP_MASK) == 0 {
            return;
        }
        if self.ai_50_50() {
            return;
        }
        self.ai_dec(hl, 2);
    }

    /// `AI_Smart_PriorityHit`.
    fn ai_smart_priority_hit(&mut self, hl: u16) {
        if self.ai_compare_speed() {
            return;
        }
        // Dismiss this move if the player is flying or underground.
        if self.mem.rb(W_PLAYER_SUB_STATUS3) & (1 << SUBSTATUS_FLYING | 1 << SUBSTATUS_UNDERGROUND) != 0 {
            return self.ai_discourage_move(hl);
        }
        // Greatly encourage this move if it will KO the player.
        self.mem.wb(H_BATTLE_TURN, 1);
        self.enemy_attack_damage();
        let _ = self.battle_command_damage_calc();
        let _ = self.battle_command_stab();
        if self.mem.rw(W_BATTLE_MON_HP) >= self.mem.rw(W_CUR_DAMAGE) {
            return;
        }
        self.ai_dec(hl, 3);
    }

    /// `AI_Smart_Thief`: don't use Thief unless it's the only move available.
    fn ai_smart_thief(&mut self, hl: u16) {
        self.ai_inc(hl, 0x1e);
    }

    /// `AI_Smart_Conversion2`.
    fn ai_smart_conversion2(&mut self, hl: u16) {
        // BUG: "Smart" AI discourages Conversion2 after the first turn
        let discourage = if self.mem.rb(W_LAST_PLAYER_MOVE) != 0 {
            true
        } else {
            // The original indexes `Moves` with `a - 1` = $ff here: move 0.
            let ty = self.data.mv(0).kind;
            self.mem.wb(W_PLAYER_MOVE_STRUCT + u16::from(MOVE_TYPE), ty);
            self.mem.wb(H_BATTLE_TURN, 0);
            self.battle_check_type_matchup();
            let m = self.mem.rb(W_TYPE_MATCHUP);
            if m == EFFECTIVE {
                return;
            }
            if m > EFFECTIVE {
                if self.ai_50_50() {
                    return;
                }
                return self.ai_dec(hl, 1);
            }
            true
        };
        if discourage {
            if self.random("AI_Smart_Conversion2") < percent(10) {
                return;
            }
            self.ai_inc(hl, 1);
        }
    }

    /// `AI_Smart_MeanLook`.
    fn ai_smart_mean_look(&mut self, hl: u16) {
        let discourage = 'out: {
            if !self.ai_check_enemy_half_hp() {
                break 'out true;
            }
            if !self.ai_check_last_player_mon() {
                return self.ai_discourage_move(hl);
            }
            // 80% chance to greatly encourage this move if the enemy is badly poisoned.
            // BUG: "Smart" AI encourages Mean Look if its own Pokémon is badly poisoned
            if self.mem.bit(W_ENEMY_SUB_STATUS5, SUBSTATUS_TOXIC) {
                break 'out false;
            }
            // 80% chance to greatly encourage this move if the player is either in love,
            // identified, stuck in Rollout, or has a Nightmare.
            if self.mem.rb(W_PLAYER_SUB_STATUS1)
                & (1 << SUBSTATUS_IN_LOVE | 1 << SUBSTATUS_ROLLOUT | 1 << SUBSTATUS_IDENTIFIED | 1 << SUBSTATUS_NIGHTMARE)
                != 0
            {
                break 'out false;
            }
            // Otherwise, discourage this move unless the player only has not very effective
            // moves against the enemy.
            self.check_player_move_type_matchups();
            if self.mem.rb(W_ENEMY_AI_SWITCH_SCORE) >= BASE_AI_SWITCH_SCORE + 1 {
                return;
            }
            true
        };
        if discourage {
            return self.ai_inc(hl, 1);
        }
        // .encourage
        if self.ai_80_20() {
            return;
        }
        self.ai_dec(hl, 3);
    }

    /// `AI_Smart_Nightmare`: 50% chance to encourage this move.
    fn ai_smart_nightmare(&mut self, hl: u16) {
        if self.ai_50_50() {
            return;
        }
        self.ai_dec(hl, 1);
    }

    /// `AI_Smart_FlameWheel`: use this move if the enemy is frozen.
    fn ai_smart_flame_wheel(&mut self, hl: u16) {
        if self.mem.rb(W_ENEMY_MON_STATUS) & 1 << FRZ == 0 {
            return;
        }
        self.ai_dec(hl, 5);
    }

    /// `AI_Smart_Curse`.
    fn ai_smart_curse(&mut self, hl: u16) {
        if self.mem.rb(W_ENEMY_MON_TYPE1) == GHOST || self.mem.rb(W_ENEMY_MON_TYPE2) == GHOST {
            return self.ai_smart_ghost_curse(hl);
        }
        if !self.ai_check_enemy_half_hp() {
            return self.ai_inc(hl, 1);
        }
        let a = self.mem.rb(W_ENEMY_ATK_LEVEL);
        if a >= BASE_STAT_LEVEL + 4 {
            return self.ai_inc(hl, 1);
        }
        if a >= BASE_STAT_LEVEL + 2 {
            return;
        }
        let t1 = self.mem.rb(W_BATTLE_MON_TYPE1);
        if t1 == GHOST {
            // .greatly_discourage
            return self.ai_inc(hl, 2);
        }
        let dark_physical = self.data.rules.ghost_special_dark_physical;
        if t1 >= SPECIAL && !(dark_physical && t1 == DARK) {
            return;
        }
        let t2 = self.mem.rb(W_BATTLE_MON_TYPE2);
        if t2 >= SPECIAL && !(dark_physical && t2 == DARK) {
            return;
        }
        if self.ai_80_20() {
            return;
        }
        self.ai_dec(hl, 2);
    }

    /// `AI_Smart_Curse.ghost_curse`.
    fn ai_smart_ghost_curse(&mut self, hl: u16) {
        if self.mem.bit(W_PLAYER_SUB_STATUS1, SUBSTATUS_CURSE) {
            return self.ai_discourage_move(hl);
        }
        let (last_mon, _) = self.find_alive_enemy_mons();
        // 0 = .highly_discourage, 1 = .ghost_continue, 2 = .maybe_greatly_encourage
        let step = if last_mon {
            if self.ai_check_last_player_mon() { 0 } else { 1 }
        } else if !self.ai_check_last_player_mon() {
            2
        } else {
            1
        };
        if step == 1 {
            // .ghost_continue
            if !self.ai_check_enemy_quarter_hp() {
                return self.ai_inc(hl, 4);
            }
            if !self.ai_check_enemy_half_hp() {
                return self.ai_inc(hl, 2);
            }
            if !self.ai_check_enemy_max_hp() {
                return;
            }
            if self.mem.rb(W_PLAYER_TURNS_TAKEN) != 0 {
                return;
            }
        } else if step == 0 {
            return self.ai_inc(hl, 4);
        }
        // .maybe_greatly_encourage
        if self.ai_50_50() {
            return;
        }
        self.ai_dec(hl, 2);
    }

    /// `AI_Smart_Protect`.
    fn ai_smart_protect(&mut self, hl: u16) {
        // 0 = greatly discourage, 1 = discourage, 2 = encourage
        let step = 'out: {
            // Greatly discourage this move if the enemy already used Protect.
            if self.mem.rb(W_ENEMY_PROTECT_COUNT) != 0 {
                break 'out 0;
            }
            // Discourage this move if the player is locked on.
            if self.mem.bit(W_PLAYER_SUB_STATUS5, SUBSTATUS_LOCK_ON) {
                break 'out 1;
            }
            // Encourage this move if the player's Fury Cutter is boosted enough.
            if self.mem.rb(W_PLAYER_FURY_CUTTER_COUNT) >= 3 {
                break 'out 2;
            }
            // Encourage this move if the player has charged a two-turn move.
            if self.mem.bit(W_PLAYER_SUB_STATUS3, SUBSTATUS_CHARGED) {
                break 'out 2;
            }
            // Encourage this move if the player is affected by Toxic, Leech Seed, or Curse.
            if self.mem.bit(W_PLAYER_SUB_STATUS5, SUBSTATUS_TOXIC)
                || self.mem.bit(W_PLAYER_SUB_STATUS4, SUBSTATUS_LEECH_SEED)
                || self.mem.bit(W_PLAYER_SUB_STATUS1, SUBSTATUS_CURSE)
            {
                break 'out 2;
            }
            // Discourage this move if the player's Rollout count is not boosted enough.
            if !self.mem.bit(W_PLAYER_SUB_STATUS1, SUBSTATUS_ROLLOUT) {
                break 'out 1;
            }
            if self.mem.rb(W_PLAYER_ROLLOUT_COUNT) < 3 {
                break 'out 1;
            }
            2
        };
        if step == 2 {
            // 80% chance to encourage this move otherwise.
            if self.ai_80_20() {
                return;
            }
            return self.ai_dec(hl, 1);
        }
        if step == 0 {
            // .greatly_discourage
            self.ai_inc(hl, 1);
        }
        // .discourage
        if self.random("AI_Smart_Protect") < percent(8) {
            return;
        }
        self.ai_inc(hl, 2);
    }

    /// `AI_Smart_Foresight`.
    fn ai_smart_foresight(&mut self, hl: u16) {
        // 60% chance to encourage this move if the enemy's accuracy is sharply lowered, the
        // player's evasion is sharply raised, or the player is a Ghost type.
        let encourage = self.mem.rb(W_ENEMY_ACC_LEVEL) < BASE_STAT_LEVEL - 2
            || self.mem.rb(W_PLAYER_EVA_LEVEL) >= BASE_STAT_LEVEL + 3
            || self.mem.rb(W_BATTLE_MON_TYPE1) == GHOST
            || self.mem.rb(W_BATTLE_MON_TYPE2) == GHOST;
        if !encourage {
            // 92% chance to discourage this move otherwise.
            if self.random("AI_Smart_Foresight") < percent(8) {
                return;
            }
            return self.ai_inc(hl, 1);
        }
        if self.random("AI_Smart_Foresight") < percent(39) + 1 {
            return;
        }
        self.ai_dec(hl, 2);
    }

    /// `AI_Smart_PerishSong`.
    fn ai_smart_perish_song(&mut self, hl: u16) {
        let (last_mon, _) = self.find_alive_enemy_mons();
        if last_mon {
            // .no
            self.ai_inc(hl, 5);
            return;
        }
        if self.mem.bit(W_PLAYER_SUB_STATUS5, SUBSTATUS_CANT_RUN) {
            // .yes
            if self.ai_50_50() {
                return;
            }
            return self.ai_dec(hl, 1);
        }
        self.check_player_move_type_matchups();
        if self.mem.rb(W_ENEMY_AI_SWITCH_SCORE) < BASE_AI_SWITCH_SCORE {
            return;
        }
        if self.ai_50_50() {
            return;
        }
        self.ai_inc(hl, 1);
    }

    /// `AI_Smart_Sandstorm`.
    fn ai_smart_sandstorm(&mut self, hl: u16) {
        let immune = |t: u8| t == ROCK || t == GROUND || t == STEEL;
        // Greatly discourage this move if the player is immune to Sandstorm damage.
        if immune(self.mem.rb(W_BATTLE_MON_TYPE1)) || immune(self.mem.rb(W_BATTLE_MON_TYPE2)) {
            return self.ai_inc(hl, 2);
        }
        // Discourage this move if player's HP is below 50%.
        if !self.ai_check_player_half_hp() {
            return self.ai_inc(hl, 1);
        }
        // 50% chance to encourage this move otherwise.
        if self.ai_50_50() {
            return;
        }
        self.ai_dec(hl, 1);
    }

    /// `AI_Smart_Endure`.
    fn ai_smart_endure(&mut self, hl: u16) {
        // Greatly discourage this move if the enemy already used Protect.
        if self.mem.rb(W_ENEMY_PROTECT_COUNT) != 0 {
            return self.ai_inc(hl, 2);
        }
        // Greatly discourage this move if the enemy's HP is full.
        if self.ai_check_enemy_max_hp() {
            return self.ai_inc(hl, 2);
        }
        // Discourage this move if the enemy's HP is at least 25%.
        if self.ai_check_enemy_quarter_hp() {
            return self.ai_inc(hl, 1);
        }
        // If the enemy has Reversal...
        if self.ai_has_move_effect(EFFECT_REVERSAL) {
            // ...80% chance to greatly encourage this move.
            if self.ai_80_20() {
                return;
            }
            return self.ai_dec(hl, 3);
        }
        // If the enemy is not locked on, do nothing.
        if !self.mem.bit(W_ENEMY_SUB_STATUS5, SUBSTATUS_LOCK_ON) {
            return;
        }
        // 50% chance to greatly encourage this move.
        if self.ai_50_50() {
            return;
        }
        self.ai_dec(hl, 2);
    }

    /// `AI_Smart_FuryCutter`: encourage this move based on Fury Cutter's
    /// count, then fall through to `AI_Smart_Rollout`.
    fn ai_smart_fury_cutter(&mut self, hl: u16) {
        let a = self.mem.rb(W_ENEMY_FURY_CUTTER_COUNT);
        if a != 0 {
            self.ai_dec(hl, 1);
            if a >= 2 {
                self.ai_dec(hl, 2);
                if a >= 3 {
                    self.ai_dec(hl, 3);
                }
            }
        }
        self.ai_smart_rollout(hl);
    }

    /// `AI_Smart_Rollout`: Rollout, Fury Cutter.
    fn ai_smart_rollout(&mut self, hl: u16) {
        // 80% chance to discourage this move if the enemy is in love, confused, or paralyzed,
        // or if its HP is below 25%, or if accuracy or evasion modifiers favour the player.
        let maybe_discourage = self.mem.bit(W_ENEMY_SUB_STATUS1, SUBSTATUS_IN_LOVE)
            || self.mem.bit(W_ENEMY_SUB_STATUS3, SUBSTATUS_CONFUSED)
            || self.mem.bit(W_ENEMY_MON_STATUS, PAR)
            || !self.ai_check_enemy_quarter_hp()
            || self.mem.rb(W_ENEMY_ACC_LEVEL) < BASE_STAT_LEVEL
            || self.mem.rb(W_PLAYER_EVA_LEVEL) >= BASE_STAT_LEVEL + 1;
        if !maybe_discourage {
            // 80% chance to greatly encourage this move otherwise.
            if self.random("AI_Smart_Rollout") >= percent(79) - 1 {
                return;
            }
            return self.ai_dec(hl, 2);
        }
        // .maybe_discourage
        if self.ai_80_20() {
            return;
        }
        self.ai_inc(hl, 1);
    }

    /// `AI_Smart_Swagger` (also Attract): 80% chance to encourage this move
    /// during the first turn of player's Pokemon; 80% chance to discourage
    /// this move otherwise.
    fn ai_smart_swagger(&mut self, hl: u16) {
        if self.mem.rb(W_PLAYER_TURNS_TAKEN) == 0 {
            // .first_turn
            if self.random("AI_Smart_Swagger") >= percent(79) - 1 {
                return;
            }
            return self.ai_dec(hl, 1);
        }
        if self.ai_80_20() {
            return;
        }
        self.ai_inc(hl, 1);
    }

    /// `AI_Smart_Safeguard`: 80% chance to discourage this move if player's HP is below 50%.
    fn ai_smart_safeguard(&mut self, hl: u16) {
        if self.ai_check_player_half_hp() {
            return;
        }
        if self.ai_80_20() {
            return;
        }
        self.ai_inc(hl, 1);
    }

    /// `AI_Smart_Magnitude` (also Earthquake): greatly encourage this move if
    /// the player is underground and the enemy is faster.
    fn ai_smart_magnitude(&mut self, hl: u16) {
        if self.mem.rb(W_LAST_PLAYER_COUNTER_MOVE) != DIG {
            return;
        }
        if self.mem.bit(W_PLAYER_SUB_STATUS3, SUBSTATUS_UNDERGROUND) {
            if !self.ai_compare_speed() {
                return;
            }
            return self.ai_dec(hl, 2);
        }
        // .could_dig: try to predict if the player will use Dig this turn.
        // 50% chance to encourage this move if the enemy is slower than the player.
        if self.ai_compare_speed() {
            return;
        }
        if self.ai_50_50() {
            return;
        }
        self.ai_dec(hl, 1);
    }

    /// `AI_Smart_BatonPass`: discourage this move if the player hasn't shown
    /// super-effective moves against the enemy.
    fn ai_smart_baton_pass(&mut self, hl: u16) {
        self.check_player_move_type_matchups();
        if self.mem.rb(W_ENEMY_AI_SWITCH_SCORE) < BASE_AI_SWITCH_SCORE {
            return;
        }
        self.ai_inc(hl, 1);
    }

    /// `AI_Smart_Pursuit`.
    fn ai_smart_pursuit(&mut self, hl: u16) {
        // 50% chance to greatly encourage this move if player's HP is below 25%.
        if !self.ai_check_player_quarter_hp() {
            if self.ai_50_50() {
                return;
            }
            return self.ai_dec(hl, 2);
        }
        // 80% chance to discourage this move otherwise.
        if self.ai_80_20() {
            return;
        }
        self.ai_inc(hl, 1);
    }

    /// `AI_Smart_RapidSpin`: 80% chance to greatly encourage this move if the
    /// enemy is trapped, seeded, or scattered with spikes.
    fn ai_smart_rapid_spin(&mut self, hl: u16) {
        let encourage = self.mem.rb(W_ENEMY_WRAP_COUNT) != 0
            || self.mem.bit(W_ENEMY_SUB_STATUS4, SUBSTATUS_LEECH_SEED)
            || self.mem.bit(W_ENEMY_SCREENS, SCREENS_SPIKES);
        if !encourage {
            return;
        }
        if self.ai_80_20() {
            return;
        }
        self.ai_dec(hl, 2);
    }

    /// `AI_Smart_HiddenPower`.
    fn ai_smart_hidden_power(&mut self, hl: u16) {
        self.mem.wb(H_BATTLE_TURN, 1);
        // Calculate Hidden Power's type and base power based on enemy's DVs.
        let _ = self.hidden_power_damage();
        let d = self.damage_regs[2];
        self.battle_check_type_matchup();

        // Discourage Hidden Power if not very effective.
        let m = self.mem.rb(W_TYPE_MATCHUP);
        // Discourage Hidden Power if its base power is lower than 50.
        if m < EFFECTIVE || d < 50 {
            return self.ai_inc(hl, 1);
        }
        // Encourage Hidden Power if super-effective, or if its base power is 70.
        if m < EFFECTIVE + 1 && d < 70 {
            return;
        }
        self.ai_dec(hl, 1);
    }

    /// `AI_Smart_RainDance`.
    fn ai_smart_rain_dance(&mut self, hl: u16) {
        // Greatly discourage this move if it would favour the player type-wise.
        // Particularly, if the player is a Water-type.
        for t in [self.mem.rb(W_BATTLE_MON_TYPE1), self.mem.rb(W_BATTLE_MON_TYPE2)] {
            if t == WATER {
                return self.ai_bad_weather_type(hl);
            }
            if t == FIRE {
                return self.ai_good_weather_type(hl);
            }
        }
        self.ai_smart_weather_move(hl, self.data.tables.rain_dance_moves);
    }

    /// `AI_Smart_SunnyDay`.
    fn ai_smart_sunny_day(&mut self, hl: u16) {
        // Greatly discourage this move if it would favour the player type-wise.
        // Particularly, if the player is a Fire-type.
        for t in [self.mem.rb(W_BATTLE_MON_TYPE1), self.mem.rb(W_BATTLE_MON_TYPE2)] {
            if t == FIRE {
                return self.ai_bad_weather_type(hl);
            }
            if t == WATER {
                return self.ai_good_weather_type(hl);
            }
        }
        self.ai_smart_weather_move(hl, self.data.tables.sunny_day_moves);
    }

    /// `AI_Smart_WeatherMove`: Rain Dance, Sunny Day.
    fn ai_smart_weather_move(&mut self, hl: u16, table: &[u8]) {
        // Greatly discourage this move if the enemy doesn't have one of the useful moves.
        if !self.ai_has_move_in_array(table) {
            return self.ai_bad_weather_type(hl);
        }
        // Greatly discourage this move if player's HP is below 50%.
        if !self.ai_check_player_half_hp() {
            return self.ai_bad_weather_type(hl);
        }
        // 50% chance to encourage this move otherwise.
        if self.ai_50_50() {
            return;
        }
        self.ai_dec(hl, 1);
    }

    /// `AIBadWeatherType`.
    fn ai_bad_weather_type(&mut self, hl: u16) {
        self.ai_inc(hl, 3);
    }

    /// `AIGoodWeatherType`.
    fn ai_good_weather_type(&mut self, hl: u16) {
        // Greatly encourage this move if it would disfavour the player type-wise and player's HP
        // is above 50%...
        if !self.ai_check_player_half_hp() {
            return;
        }
        // ...as long as it's the first turn of the player's or the enemy's Pokemon.
        if self.mem.rb(W_PLAYER_TURNS_TAKEN) != 0 && self.mem.rb(W_ENEMY_TURNS_TAKEN) != 0 {
            return;
        }
        self.ai_dec(hl, 2);
    }

    /// `AI_Smart_BellyDrum`.
    fn ai_smart_belly_drum(&mut self, hl: u16) {
        // Dismiss this move if enemy's attack is higher than +2 or if enemy's HP is below 50%.
        // Else, discourage this move if enemy's HP is not full.
        let dismiss = if self.mem.rb(W_ENEMY_ATK_LEVEL) >= BASE_STAT_LEVEL + 3 {
            true
        } else {
            if self.ai_check_enemy_max_hp() {
                return;
            }
            self.ai_inc(hl, 1);
            !self.ai_check_enemy_half_hp()
        };
        if dismiss {
            // .discourage
            self.ai_inc(hl, 5);
        }
    }

    /// `AI_Smart_PsychUp`.
    fn ai_smart_psych_up(&mut self, hl: u16) {
        // Sum all enemy's and player's stat level modifiers (plus 100 to prevent underflow).
        let mut c = 100u8;
        for i in 0..u16::from(NUM_LEVEL_STATS) {
            c = self.mem.rb(W_ENEMY_ATK_LEVEL + i).wrapping_sub(BASE_STAT_LEVEL).wrapping_add(c);
        }
        let mut d = 100u8;
        for i in 0..u16::from(NUM_LEVEL_STATS) {
            d = self.mem.rb(W_PLAYER_ATK_LEVEL + i).wrapping_sub(BASE_STAT_LEVEL).wrapping_add(d);
        }
        // Greatly discourage this move if enemy's stat levels are higher than player's (if c>=d).
        if c >= d {
            return self.ai_inc(hl, 2);
        }
        // Else, 80% chance to encourage this move unless player's accuracy level is lower than
        // -1...
        if self.mem.rb(W_PLAYER_ACC_LEVEL) < BASE_STAT_LEVEL - 1 {
            return;
        }
        // ...or enemy's evasion level is higher than +0.
        if self.mem.rb(W_ENEMY_EVA_LEVEL) >= BASE_STAT_LEVEL + 1 {
            return;
        }
        if self.ai_80_20() {
            return;
        }
        self.ai_dec(hl, 1);
    }

    /// `AI_Smart_Twister` (also Gust): greatly encourage this move if the
    /// player is flying and the enemy is faster.
    fn ai_smart_twister(&mut self, hl: u16) {
        if self.mem.rb(W_LAST_PLAYER_COUNTER_MOVE) != FLY {
            return;
        }
        if self.mem.bit(W_PLAYER_SUB_STATUS3, SUBSTATUS_FLYING) {
            if !self.ai_compare_speed() {
                return;
            }
            return self.ai_dec(hl, 2);
        }
        // .couldFly: try to predict if the player will use Fly this turn.
        // 50% chance to encourage this move if the enemy is slower than the player.
        if self.ai_compare_speed() {
            return;
        }
        if self.ai_50_50() {
            return;
        }
        self.ai_dec(hl, 1);
    }

    /// `AI_Smart_FutureSight`: greatly encourage this move if the player is
    /// flying or underground, and slower than the enemy.
    fn ai_smart_future_sight(&mut self, hl: u16) {
        if !self.ai_compare_speed() {
            return;
        }
        if self.mem.rb(W_PLAYER_SUB_STATUS3) & (1 << SUBSTATUS_FLYING | 1 << SUBSTATUS_UNDERGROUND) == 0 {
            return;
        }
        self.ai_dec(hl, 2);
    }

    /// `AI_Smart_Stomp`: 80% chance to encourage this move if the player has used Minimize.
    fn ai_smart_stomp(&mut self, hl: u16) {
        if self.mem.rb(W_PLAYER_MINIMIZED) == 0 {
            return;
        }
        if self.ai_80_20() {
            return;
        }
        self.ai_dec(hl, 1);
    }

    /// `AI_Smart_Solarbeam`.
    fn ai_smart_solarbeam(&mut self, hl: u16) {
        let weather = self.mem.rb(W_BATTLE_WEATHER);
        if weather == WEATHER_SUN {
            // .encourage: 80% chance to encourage this move when it's sunny.
            if self.ai_80_20() {
                return;
            }
            return self.ai_dec(hl, 2);
        }
        if weather != WEATHER_RAIN {
            return;
        }
        // 90% chance to discourage this move when it's raining.
        if self.random("AI_Smart_Solarbeam") < percent(10) {
            return;
        }
        self.ai_inc(hl, 2);
    }

    /// `AI_Smart_Thunder`: 90% chance to discourage this move when it's sunny.
    fn ai_smart_thunder(&mut self, hl: u16) {
        if self.mem.rb(W_BATTLE_WEATHER) != WEATHER_SUN {
            return;
        }
        if self.random("AI_Smart_Thunder") < percent(10) {
            return;
        }
        self.ai_inc(hl, 1);
    }

}
