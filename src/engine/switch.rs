//! Port of the switching and fainting routines (`engine/battle/core.asm`).
//! See docs/engine.md.

#![allow(unused_imports)]

use super::battle::{Engine, Flow, PARTY_LEN, PP_MASK, percent};
use super::rom;
use super::rsym as r;
use super::sym::*;
use crate::defs::*;

const MON_NAME_LENGTH: u16 = 11;
const MAX_STAT_VALUE: u16 = 999;

impl Engine {
    // --- sending out / recalling ---------------------------------------------------

    /// `PlayerSwitch` (from `TryPlayerSwitch`). Returns carry — never.
    pub fn player_switch(&mut self) -> Flow<bool> {
        self.mem.wb(W_PLAYER_IS_SWITCHING, 1);
        if self.link_mode() != 0 {
            self.link_battle_send_receive_action()?;
        }
        self.parse_enemy_action()?;
        if self.link_mode() == 0 {
            self.battle_mon_entrance()?;
            return Ok(false);
        }
        // .linked
        let action = self.mem.rb(W_BATTLE_ACTION);
        if action == BATTLEACTION_STRUGGLE || action == BATTLEACTION_SKIPTURN || action < BATTLEACTION_SWITCH1 {
            self.battle_mon_entrance()?;
            return Ok(false);
        }
        if action == BATTLEACTION_FORFEIT {
            self.unported("WildFled_EnemyFled_LinkBattleCanceled (forfeit)");
        }
        // .dont_run
        if self.mem.rb(H_SERIAL_CONNECTION_STATUS) == USING_EXTERNAL_CLOCK {
            self.enemy_mon_entrance()?;
            self.battle_mon_entrance()?;
        } else {
            self.battle_mon_entrance()?;
            self.enemy_mon_entrance()?;
        }
        Ok(false)
    }

    /// `EnemyMonEntrance`.
    pub fn enemy_mon_entrance(&mut self) -> Flow {
        self.ai_switch()?;
        self.set_enemy_turn();
        self.spikes_damage();
        Ok(())
    }

    /// `BattleMonEntrance`.
    pub fn battle_mon_entrance(&mut self) -> Flow {
        self.withdraw_mon_text();
        self.mem.res_bit(W_PLAYER_SUB_STATUS4, SUBSTATUS_RAGE);
        self.set_enemy_turn();
        if !self.pursuit_switch()? {
            self.recall_player_mon();
        }
        let cur = self.mem.rb(W_CUR_BATTLE_MON);
        self.mem.wb(W_CUR_PARTY_MON, cur);
        self.add_battle_participant();
        self.init_battle_mon();
        self.reset_player_stat_levels();
        self.send_out_mon_text();
        self.new_battle_mon_status();
        self.break_attraction();
        self.send_out_player_mon();
        self.set_player_turn();
        self.spikes_damage();
        self.mem.wb(W_MENU_CURSOR_Y, 2);
        Ok(())
    }

    /// `PassedBattleMonEntrance` (Baton Pass).
    pub fn passed_battle_mon_entrance(&mut self) {
        let cur = self.mem.rb(W_CUR_PARTY_MON);
        self.mem.wb(W_CUR_BATTLE_MON, cur);
        self.add_battle_participant();
        self.init_battle_mon();
        self.mem.wb(W_APPLY_STAT_LEVEL_MULTIPLIERS_TO_ENEMY, 0);
        self.apply_stat_level_multiplier_on_all_stats();
        self.send_out_player_mon();
        self.set_player_turn();
        self.spikes_damage();
    }

    /// `SwitchPlayerMon`. Returns nz (the enemy mon is alive).
    pub fn switch_player_mon(&mut self) -> bool {
        let old = self.mem.rb(W_CUR_BATTLE_MON);
        self.mem.wb(W_LAST_PLAYER_MON, old);
        let cur = self.mem.rb(W_CUR_PARTY_MON);
        self.mem.wb(W_CUR_BATTLE_MON, cur);
        self.add_battle_participant();
        self.init_battle_mon();
        self.reset_player_stat_levels();
        self.new_battle_mon_status();
        self.break_attraction();
        self.send_out_player_mon();
        self.mem.rw(W_ENEMY_MON_HP) != 0
    }

    /// `PlayerPartyMonEntrance`.
    pub fn player_party_mon_entrance(&mut self) {
        let old = self.mem.rb(W_CUR_BATTLE_MON);
        self.mem.wb(W_LAST_PLAYER_MON, old);
        let cur = self.mem.rb(W_CUR_PARTY_MON);
        self.mem.wb(W_CUR_BATTLE_MON, cur);
        self.add_battle_participant();
        self.init_battle_mon();
        self.reset_player_stat_levels();
        self.send_out_mon_text();
        self.new_battle_mon_status();
        self.break_attraction();
        self.send_out_player_mon();
        self.set_player_turn();
        self.spikes_damage();
    }

    /// `RecallPlayerMon`: an animation only.
    fn recall_player_mon(&mut self) {
        let turn = self.turn();
        self.mem.wb(H_BATTLE_TURN, 0);
        self.mem.wb(W_BATTLE_AFTER_ANIM, 0);
        self.mem.wb(H_BATTLE_TURN, turn);
    }

    /// `PursuitSwitch`. Returns carry (the switching mon fainted to Pursuit).
    fn pursuit_switch(&mut self) -> Flow<bool> {
        let mv = self.battle_var(BATTLE_VARS_MOVE);
        if self.get_move_effect(mv) != EFFECT_PURSUIT {
            return Ok(false);
        }
        let saved = self.mem.rb(W_CUR_BATTLE_MON);
        if self.is_player_turn() {
            self.do_player_turn()?;
        } else {
            let last = self.mem.rb(W_LAST_PLAYER_MON);
            self.mem.wb(W_CUR_BATTLE_MON, last);
            self.do_enemy_turn()?;
        }
        self.set_battle_var(BATTLE_VARS_MOVE, CANNOT_MOVE);
        self.mem.wb(W_CUR_BATTLE_MON, saved);
        if self.is_player_turn() {
            // .check_enemy_fainted
            if self.mem.rw(W_ENEMY_MON_HP) != 0 {
                return Ok(false);
            }
            self.text(r::BATTLE_TEXT_ENEMY_MON_FAINTED);
            return Ok(true);
        }
        let last = self.mem.rb(W_LAST_PLAYER_MON);
        self.update_battle_mon(last);
        if self.mem.rw(W_BATTLE_MON_HP) != 0 {
            return Ok(false);
        }
        // BUG: A Pokémon that fainted from Pursuit keeps its old status (kept).
        self.mem.wb(W_CRY_TRACKS, 0xf0);
        self.small_far_flag_action(W_BATTLE_PARTICIPANTS_NOT_FAINTED, last, FlagAction::Reset);
        self.text(r::BATTLE_TEXT_MON_FAINTED);
        Ok(true)
    }

    /// `UpdateBattleMon`: copy `wBattleMonLevel..MaxHP` into party slot `a`.
    pub fn update_battle_mon(&mut self, slot: u8) {
        let dst = W_PARTY_MON1_LEVEL + u16::from(slot) * PARTY_LEN;
        self.mem.copy(dst, W_BATTLE_MON_LEVEL, W_BATTLE_MON_MAX_HP - W_BATTLE_MON_LEVEL);
    }

    /// `ResetBattleParticipants`.
    pub fn reset_battle_participants(&mut self) {
        self.mem.wb(W_BATTLE_PARTICIPANTS_NOT_FAINTED, 0);
        self.mem.wb(W_BATTLE_PARTICIPANTS_INCLUDING_FAINTED, 0);
        self.add_battle_participant();
    }

    /// `AddBattleParticipant`.
    pub fn add_battle_participant(&mut self) {
        let c = self.mem.rb(W_CUR_BATTLE_MON);
        self.small_far_flag_action(W_BATTLE_PARTICIPANTS_NOT_FAINTED, c, FlagAction::Set);
        self.small_far_flag_action(W_BATTLE_PARTICIPANTS_INCLUDING_FAINTED, c, FlagAction::Set);
    }

    /// `SmallFarFlagAction`: bit `c` of the bit array at `hl`. Returns
    /// whether the bit is set (for `CHECK_FLAG`).
    pub fn small_far_flag_action(&mut self, hl: u16, c: u8, action: FlagAction) -> bool {
        let addr = hl + u16::from(c >> 3);
        let mask = 1u8 << (c & 7);
        let v = self.mem.rb(addr);
        match action {
            FlagAction::Reset => self.mem.wb(addr, v & !mask),
            FlagAction::Set => self.mem.wb(addr, v | mask),
            FlagAction::Check => {}
        }
        v & mask != 0
    }

    /// `InitBattleMon`: load party mon `wCurPartyMon` into `wBattleMon`.
    pub fn init_battle_mon(&mut self) {
        let hl = W_PARTY_MON1 + u16::from(self.mem.rb(W_CUR_PARTY_MON)) * PARTY_LEN;
        self.mem.copy(W_BATTLE_MON_SPECIES, hl, u16::from(MON_OT_ID));
        self.mem.copy(W_BATTLE_MON_D_VS, hl + u16::from(MON_DVS), u16::from(MON_POKERUS - MON_DVS));
        self.mem.copy(W_BATTLE_MON_LEVEL, hl + u16::from(MON_LEVEL), PARTY_LEN - u16::from(MON_LEVEL));
        let species = self.mem.rb(W_BATTLE_MON_SPECIES);
        self.mem.wb(W_TEMP_BATTLE_MON_SPECIES, species);
        self.mem.wb(W_CUR_PARTY_SPECIES, species);
        self.mem.wb(W_CUR_SPECIES, species);
        self.get_base_data();
        let t1 = self.mem.rb(W_BASE_TYPE1);
        let t2 = self.mem.rb(W_BASE_TYPE2);
        self.mem.wb(W_BATTLE_MON_TYPE1, t1);
        self.mem.wb(W_BATTLE_MON_TYPE2, t2);
        let names = W_PARTY_MON_NICKNAMES + u16::from(self.mem.rb(W_CUR_BATTLE_MON)) * MON_NAME_LENGTH;
        self.mem.copy(W_BATTLE_MON_NICKNAME, names, MON_NAME_LENGTH);
        self.mem.copy(W_PLAYER_STATS, W_BATTLE_MON_ATTACK, PARTY_LEN - u16::from(MON_ATK));
        self.apply_status_effect_on_player_stats();
        self.badge_stat_boosts();
    }

    /// `InitEnemyMon`: load OT party mon `wCurPartyMon` into `wEnemyMon`.
    pub fn init_enemy_mon(&mut self) {
        let cur = self.mem.rb(W_CUR_PARTY_MON);
        let hl = W_OT_PARTY_MON1_SPECIES + u16::from(cur) * PARTY_LEN;
        self.mem.copy(W_ENEMY_MON_SPECIES, hl, u16::from(MON_OT_ID));
        self.mem.copy(W_ENEMY_MON_D_VS, hl + u16::from(MON_DVS), u16::from(MON_POKERUS - MON_DVS));
        self.mem.copy(W_ENEMY_MON_LEVEL, hl + u16::from(MON_LEVEL), PARTY_LEN - u16::from(MON_LEVEL));
        let species = self.mem.rb(W_ENEMY_MON_SPECIES);
        self.mem.wb(W_CUR_SPECIES, species);
        self.get_base_data();
        let names = W_OT_PARTY_MON_NICKNAMES + u16::from(cur) * MON_NAME_LENGTH;
        self.mem.copy(W_ENEMY_MON_NICKNAME, names, MON_NAME_LENGTH);
        self.mem.copy(W_ENEMY_STATS, W_ENEMY_MON_ATTACK, PARTY_LEN - u16::from(MON_ATK));
        self.apply_status_effect_on_enemy_stats();
        let t1 = self.mem.rb(W_BASE_TYPE1);
        let t2 = self.mem.rb(W_BASE_TYPE2);
        self.mem.wb(W_ENEMY_MON_TYPE1, t1);
        self.mem.wb(W_ENEMY_MON_TYPE2, t2);
        for i in 0..u16::from(NUM_STATS - 1) {
            let b = self.mem.rb(W_BASE_STATS + i);
            self.mem.wb(W_ENEMY_MON_BASE_STATS + i, b);
        }
        self.mem.wb(W_CUR_OT_MON, cur);
    }

    /// `GetBaseData`: base data of `wCurSpecies` into `wCurBaseData`.
    pub fn get_base_data(&mut self) {
        let species = self.mem.rb(W_CUR_SPECIES);
        if species == EGG {
            self.unported("GetBaseData (egg)");
        }
        let src = r::BASE_DATA + usize::from(species - 1) * usize::from(BASE_DATA_SIZE);
        for i in 0..u16::from(BASE_DATA_SIZE) {
            self.mem.wb(W_CUR_BASE_DATA + i, rom::byte(src + usize::from(i)));
        }
        // .end: replace the Pokédex number with the species.
        self.mem.wb(W_BASE_DEX_NO, species);
    }

    /// `ResetPlayerStatLevels`.
    pub fn reset_player_stat_levels(&mut self) {
        self.mem.fill(W_PLAYER_STAT_LEVELS, u16::from(NUM_LEVEL_STATS), BASE_STAT_LEVEL);
    }

    /// `ResetEnemyStatLevels`.
    pub fn reset_enemy_stat_levels(&mut self) {
        self.mem.fill(W_ENEMY_STAT_LEVELS, u16::from(NUM_LEVEL_STATS), BASE_STAT_LEVEL);
    }

    /// `NewBattleMonStatus`.
    pub fn new_battle_mon_status(&mut self) {
        for addr in [W_LAST_PLAYER_COUNTER_MOVE, W_LAST_ENEMY_COUNTER_MOVE, W_LAST_PLAYER_MOVE] {
            self.mem.wb(addr, 0);
        }
        self.mem.fill(W_PLAYER_SUB_STATUS1, 5, 0);
        self.mem.fill(W_PLAYER_USED_MOVES, 4, 0);
        for addr in [
            W_PLAYER_DISABLE_COUNT, W_PLAYER_FURY_CUTTER_COUNT, W_PLAYER_PROTECT_COUNT, W_PLAYER_RAGE_COUNTER,
            W_DISABLED_MOVE, W_PLAYER_MINIMIZED, W_ENEMY_WRAP_COUNT, W_PLAYER_WRAP_COUNT, W_PLAYER_TURNS_TAKEN,
        ] {
            self.mem.wb(addr, 0);
        }
        self.mem.res_bit(W_ENEMY_SUB_STATUS5, SUBSTATUS_CANT_RUN);
    }

    /// `NewEnemyMonStatus`.
    pub fn new_enemy_mon_status(&mut self) {
        for addr in [W_LAST_PLAYER_COUNTER_MOVE, W_LAST_ENEMY_COUNTER_MOVE, W_LAST_ENEMY_MOVE] {
            self.mem.wb(addr, 0);
        }
        self.mem.fill(W_ENEMY_SUB_STATUS1, 5, 0);
        for addr in [
            W_ENEMY_DISABLE_COUNT, W_ENEMY_FURY_CUTTER_COUNT, W_ENEMY_PROTECT_COUNT, W_ENEMY_RAGE_COUNTER,
            W_ENEMY_DISABLED_MOVE, W_ENEMY_MINIMIZED, W_PLAYER_WRAP_COUNT, W_ENEMY_WRAP_COUNT, W_ENEMY_TURNS_TAKEN,
        ] {
            self.mem.wb(addr, 0);
        }
        self.mem.res_bit(W_PLAYER_SUB_STATUS5, SUBSTATUS_CANT_RUN);
    }

    /// `BreakAttraction`.
    pub fn break_attraction(&mut self) {
        self.mem.res_bit(W_PLAYER_SUB_STATUS1, SUBSTATUS_IN_LOVE);
        self.mem.res_bit(W_ENEMY_SUB_STATUS1, SUBSTATUS_IN_LOVE);
    }

    /// `SendOutPlayerMon`: the memory side of sending the mon out.
    pub fn send_out_player_mon(&mut self) {
        self.get_unown_letter(W_BATTLE_MON_D_VS);
        self.mem.wb(W_BATTLE_MENU_CURSOR_POSITION, 0);
        self.mem.wb(W_CUR_MOVE_NUM, 0);
        self.mem.wb(W_TYPE_MODIFIER, 0);
        self.mem.wb(W_PLAYER_MOVE_STRUCT_ANIMATION, 0);
        self.mem.wb(W_LAST_PLAYER_COUNTER_MOVE, 0);
        self.mem.wb(W_LAST_ENEMY_COUNTER_MOVE, 0);
        self.mem.wb(W_LAST_PLAYER_MOVE, 0);
        self.check_amulet_coin();
        self.mem.wb(W_ENEMY_WRAP_COUNT, 0);
        self.set_player_turn();
        self.mem.wb(W_BATTLE_AFTER_ANIM, 0);
        self.mem.wb(W_BATTLE_ANIM_PARAM, 0);
        if self.battle_check_player_shininess() {
            self.mem.wb(W_BATTLE_ANIM_PARAM, 1);
        }
        // Cry: CheckFaintedFrzSlp decides whether wCryTracks is set.
        let species_addr = W_PARTY_MON1_SPECIES + u16::from(self.mem.rb(W_CUR_PARTY_MON)) * PARTY_LEN;
        if !self.check_fainted_frz_slp(species_addr) {
            self.mem.wb(W_CRY_TRACKS, 0xf0);
        }
    }

    /// `CheckFaintedFrzSlp` (`bc` = a party mon): carry if fainted, frozen
    /// or asleep.
    fn check_fainted_frz_slp(&self, bc: u16) -> bool {
        if self.mem.rw(bc + u16::from(MON_HP)) == 0 {
            return true;
        }
        self.mem.rb(bc + u16::from(MON_STATUS)) & (1 << FRZ | SLP_MASK) != 0
    }

    /// `BattleCheckPlayerShininess`.
    fn battle_check_player_shininess(&self) -> bool {
        let dvs = if self.mem.bit(W_PLAYER_SUB_STATUS5, SUBSTATUS_TRANSFORMED) {
            W_PARTY_MON1_D_VS + u16::from(self.mem.rb(W_CUR_BATTLE_MON)) * PARTY_LEN
        } else {
            W_BATTLE_MON_D_VS
        };
        check_shininess(self.mem.rb(dvs), self.mem.rb(dvs + 1))
    }

    /// `GetUnownLetter` from the DVs at `hl` into `wUnownLetter`.
    pub fn get_unown_letter(&mut self, hl: u16) {
        let (atk_def, spd_spc) = (self.mem.rb(hl), self.mem.rb(hl + 1));
        let b = (atk_def & 0b0110_0000) << 1 | ((atk_def & 0b0000_0110) << 4) >> 1;
        let b = b | ((spd_spc & 0b0110_0000) >> 4) << 1;
        let a = b | (spd_spc & 0b0000_0110) >> 1;
        self.mem.wb(H_DIVIDEND + 3, a);
        self.mem.wb(H_DIVIDEND, 0);
        self.mem.wb(H_DIVIDEND + 1, 0);
        self.mem.wb(H_DIVIDEND + 2, 0);
        self.mem.wb(H_DIVISOR, 0xff / NUM_UNOWN + 1);
        self.divide(4);
        let q = self.mem.rb(H_QUOTIENT + 3).wrapping_add(1);
        self.mem.wb(W_UNOWN_LETTER, q);
    }

    /// `CheckAmuletCoin`.
    fn check_amulet_coin(&mut self) {
        let item = self.mem.rb(W_BATTLE_MON_ITEM);
        if self.get_item_held_effect(item).0 == HELD_AMULET_COIN {
            self.mem.wb(W_AMULET_COIN, 1);
        }
    }

    // --- stats --------------------------------------------------------------------

    /// `ApplyStatusEffectOnPlayerStats` (leaves `hBattleTurn` = 1).
    pub fn apply_status_effect_on_player_stats(&mut self) {
        self.mem.wb(H_BATTLE_TURN, 1);
        self.apply_prz_effect_on_speed();
        self.apply_brn_effect_on_attack();
    }

    /// `ApplyStatusEffectOnEnemyStats` (leaves `hBattleTurn` = 0).
    pub fn apply_status_effect_on_enemy_stats(&mut self) {
        self.mem.wb(H_BATTLE_TURN, 0);
        self.apply_prz_effect_on_speed();
        self.apply_brn_effect_on_attack();
    }

    /// `ApplyPrzEffectOnSpeed`: quarter the *opponent's* speed if paralyzed.
    pub fn apply_prz_effect_on_speed(&mut self) {
        let (status, speed) = if self.turn() != 0 {
            (W_BATTLE_MON_STATUS, W_BATTLE_MON_SPEED)
        } else {
            (W_ENEMY_MON_STATUS, W_ENEMY_MON_SPEED)
        };
        if self.mem.rb(status) & 1 << PAR == 0 {
            return;
        }
        let v = self.mem.rw(speed) >> 2;
        self.mem.wb(speed, (v >> 8) as u8);
        self.mem.wb(speed + 1, if v == 0 { 1 } else { v as u8 });
    }

    /// `ApplyBrnEffectOnAttack`: halve the *opponent's* attack if burned.
    pub fn apply_brn_effect_on_attack(&mut self) {
        let (status, attack) = if self.turn() != 0 {
            (W_BATTLE_MON_STATUS, W_BATTLE_MON_ATTACK)
        } else {
            (W_ENEMY_MON_STATUS, W_ENEMY_MON_ATTACK)
        };
        if self.mem.rb(status) & 1 << BRN == 0 {
            return;
        }
        let v = self.mem.rw(attack) >> 1;
        self.mem.wb(attack, (v >> 8) as u8);
        self.mem.wb(attack + 1, if v == 0 { 1 } else { v as u8 });
    }

    /// `ApplyStatLevelMultiplierOnAllStats`.
    pub fn apply_stat_level_multiplier_on_all_stats(&mut self) {
        for c in 0..NUM_BATTLE_STATS {
            self.apply_stat_level_multiplier(c);
        }
    }

    /// `ApplyStatLevelMultiplier` for stat `c`.
    pub fn apply_stat_level_multiplier(&mut self, c: u8) {
        let (hl, de, bc) = if self.mem.rb(W_APPLY_STAT_LEVEL_MULTIPLIERS_TO_ENEMY) == 0 {
            (W_BATTLE_MON_ATTACK, W_PLAYER_STATS, W_PLAYER_ATK_LEVEL)
        } else {
            (W_ENEMY_MON_ATTACK, W_ENEMY_STATS, W_ENEMY_ATK_LEVEL)
        };
        let level = self.mem.rb(bc + u16::from(c));
        let hl = hl + 2 * u16::from(c);
        let de = de + 2 * u16::from(c);
        let table = r::STAT_LEVEL_MULTIPLIERS_APPLIED + 2 * usize::from(level.wrapping_sub(1));
        self.mem.wb(H_MULTIPLICAND, 0);
        let (hi, lo) = (self.mem.rb(de), self.mem.rb(de + 1));
        self.mem.wb(H_MULTIPLICAND + 1, hi);
        self.mem.wb(H_MULTIPLICAND + 2, lo);
        self.mem.wb(H_MULTIPLIER, rom::byte(table));
        self.multiply();
        self.mem.wb(H_DIVISOR, rom::byte(table + 1));
        self.divide(4);
        let q = u16::from(self.mem.rb(H_QUOTIENT + 2)) << 8 | u16::from(self.mem.rb(H_QUOTIENT + 3));
        if q >= MAX_STAT_VALUE {
            self.mem.wb(H_QUOTIENT + 2, (MAX_STAT_VALUE >> 8) as u8);
            self.mem.wb(H_QUOTIENT + 3, MAX_STAT_VALUE as u8);
        }
        let (q2, q3) = (self.mem.rb(H_QUOTIENT + 2), self.mem.rb(H_QUOTIENT + 3));
        self.mem.wb(hl, q2);
        self.mem.wb(hl + 1, q3);
        if q2 | q3 == 0 {
            self.mem.wb(hl + 1, 1);
        }
    }

    /// `BadgeStatBoosts`, glacier-badge bug included.
    pub fn badge_stat_boosts(&mut self) {
        if self.link_mode() != 0 || self.mem.rb(W_IN_BATTLE_TOWER_BATTLE) != 0 {
            return;
        }
        let d = self.mem.rb(W_JOHTO_BADGES);
        let b = (d & 1 << PLAINBADGE) << 2;
        let c = (d & 1 << MINERALBADGE).rotate_right(2);
        let mut b = (d & (1 << ZEPHYRBADGE | 1 << HIVEBADGE | 1 << FOGBADGE | 1 << STORMBADGE | 1 << GLACIERBADGE | 1 << RISINGBADGE))
            | b
            | c;
        let mut hl = W_BATTLE_MON_ATTACK;
        let mut a = 0u8;
        for _ in 0..4 {
            a = b;
            let carry = b & 1 != 0;
            b >>= 1;
            if carry {
                a = self.boost_stat(hl);
            }
            hl += 2;
            b >>= 1;
        }
        if a & 1 != 0 {
            self.boost_stat(hl);
        }
    }

    /// `BoostStat`: raise the stat at `hl` by 1/8 (capped at 999). Returns
    /// the `a` it leaves behind (`BadgeStatBoosts` reads it).
    fn boost_stat(&mut self, hl: u16) -> u8 {
        let v = self.mem.rw(hl);
        let e = (v >> 3) as u8;
        let d = (v >> 11) as u8;
        let (lo, c1) = self.mem.rb(hl + 1).overflowing_add(e);
        self.mem.wb(hl + 1, lo);
        let hi = self.mem.rb(hl).wrapping_add(d).wrapping_add(u8::from(c1));
        self.mem.wb(hl, hi);
        let (_, b1) = lo.overflowing_sub(MAX_STAT_VALUE as u8);
        let (t, b2a) = hi.overflowing_sub((MAX_STAT_VALUE >> 8) as u8);
        let (t, b2b) = t.overflowing_sub(u8::from(b1));
        if b2a || b2b {
            return t;
        }
        self.mem.wb(hl, (MAX_STAT_VALUE >> 8) as u8);
        self.mem.wb(hl + 1, MAX_STAT_VALUE as u8);
        MAX_STAT_VALUE as u8
    }

    // --- send-out / withdraw texts (their text_asm logic) ------------------------------

    /// Enemy HP as a quarter-percentage: `hMultiplicand+1..2` × 25 ÷ (max HP / 4).
    /// (Max HP below 4 divides by zero; the original freezes.)
    fn enemy_hp_percent_of(&mut self) -> u8 {
        self.mem.wb(H_MULTIPLIER, 25);
        self.multiply();
        let max = self.mem.rw(W_ENEMY_MON_MAX_HP);
        self.mem.wb(H_DIVISOR, (max >> 2) as u8);
        self.divide(4);
        self.mem.rb(H_QUOTIENT + 3)
    }

    /// `SendOutMonText`: "Go! / Do it! / Go for it! / Your foe's weak!".
    pub fn send_out_mon_text(&mut self) {
        let text = 'text: {
            if self.link_mode() != 0 && self.mem.rb(W_BATTLE_HAS_JUST_STARTED) != 0 {
                break 'text r::GO_MON_TEXT;
            }
            // .not_linked
            if self.mem.rw(W_ENEMY_MON_HP) == 0 {
                break 'text r::GO_MON_TEXT;
            }
            self.mem.wb(H_MULTIPLICAND, 0);
            let (hi, lo) = (self.mem.rb(W_ENEMY_MON_HP), self.mem.rb(W_ENEMY_MON_HP + 1));
            self.mem.wb(W_ENEMY_HP_AT_TIME_OF_PLAYER_SWITCH, hi);
            self.mem.wb(H_MULTIPLICAND + 1, hi);
            self.mem.wb(W_ENEMY_HP_AT_TIME_OF_PLAYER_SWITCH + 1, lo);
            self.mem.wb(H_MULTIPLICAND + 2, lo);
            let pct = self.enemy_hp_percent_of();
            if pct >= 70 {
                r::GO_MON_TEXT
            } else if pct >= 40 {
                r::DO_IT_MON_TEXT
            } else if pct >= 10 {
                r::GO_FOR_IT_MON_TEXT
            } else {
                r::YOUR_FOES_WEAK_GETM_MON_TEXT
            }
        };
        self.text(text);
    }

    /// `WithdrawMonText`: "<MON>, that's enough! / come back! / OK! / good!".
    pub fn withdraw_mon_text(&mut self) {
        // HP lost since the enemy mon was out: wEnemyHPAtTimeOfPlayerSwitch - HP.
        // (hMultiplicand + 0 is not reset here — the original's leftover.)
        let then = self.mem.rw(W_ENEMY_HP_AT_TIME_OF_PLAYER_SWITCH);
        let now = self.mem.rw(W_ENEMY_MON_HP);
        let lost = then.wrapping_sub(now);
        self.mem.wb(H_MULTIPLICAND + 2, lost as u8);
        self.mem.wb(H_MULTIPLICAND + 1, (lost >> 8) as u8);
        let pct = self.enemy_hp_percent_of();
        let text = if pct == 0 {
            r::THATS_ENOUGH_COME_BACK_TEXT
        } else if pct < 30 {
            r::COME_BACK_TEXT
        } else if pct < 70 {
            r::OK_COME_BACK_TEXT
        } else {
            r::GOOD_COME_BACK_TEXT
        };
        self.text(text);
    }

    /// `SpikesDamage`: the mon whose turn it is just came in.
    pub fn spikes_damage(&mut self) {
        let (screens, types) = if self.is_player_turn() {
            (W_PLAYER_SCREENS, W_BATTLE_MON_TYPE1)
        } else {
            (W_ENEMY_SCREENS, W_ENEMY_MON_TYPE1)
        };
        if !self.mem.bit(screens, SCREENS_SPIKES) {
            return;
        }
        if self.mem.rb(types) == FLYING || self.mem.rb(types + 1) == FLYING {
            return;
        }
        self.text(r::BATTLE_TEXT_USER_HURT_BY_SPIKES);
        let bc = self.get_eighth_max_hp();
        self.subtract_hp_from_target(bc);
    }

    // --- party checks -------------------------------------------------------------

    /// `CheckPlayerPartyForFitMon`: `d` != 0 if any party mon has HP.
    pub fn check_player_party_for_fit_mon(&self) -> u8 {
        let mut a = 0u8;
        for i in 0..u16::from(self.mem.rb(W_PARTY_COUNT)) {
            let hp = W_PARTY_MON1_HP + i * PARTY_LEN;
            a |= self.mem.rb(hp) | self.mem.rb(hp + 1);
        }
        a
    }

    /// `CheckIfCurPartyMonIsFitToFight` (nz = fit).
    pub fn check_if_cur_party_mon_is_fit_to_fight(&mut self) -> bool {
        let slot = self.mem.rb(W_CUR_PARTY_MON);
        if self.mem.rw(W_PARTY_MON1_HP + u16::from(slot) * PARTY_LEN) != 0 {
            return true;
        }
        if self.mem.rb(W_BATTLE_HAS_JUST_STARTED) == 0 {
            let species = self.mem.rb(W_PARTY_SPECIES + u16::from(slot));
            self.text(if species == EGG { r::BATTLE_TEXT_AN_EGG_CANT_BATTLE } else { r::BATTLE_TEXT_THERES_NO_WILL_TO_BATTLE });
        }
        false
    }
}

impl Engine {
    // --- fainting -------------------------------------------------------------------

    /// `HandleEnemyMonFaint`.
    pub fn handle_enemy_mon_faint(&mut self) -> Flow {
        self.faint_enemy_pokemon();
        if self.mem.rw(W_BATTLE_MON_HP) == 0 {
            self.faint_your_pokemon();
        }
        self.mem.wb(W_WHICH_MON_FAINTED_FIRST, 0);
        self.update_battle_state_and_experience_after_enemy_faint();
        if self.check_player_party_for_fit_mon() == 0 {
            self.lost_battle();
            return Ok(());
        }
        if self.battle_mode() == WILD_BATTLE {
            self.mem.wb(W_BATTLE_ENDED, 1);
            return Ok(());
        }
        // .trainer
        if !self.check_enemy_trainer_defeated() {
            self.win_trainer_battle();
            return Ok(());
        }
        if self.mem.rw(W_BATTLE_MON_HP) == 0 {
            if self.ask_use_next_pokemon() {
                self.mem.wb(W_BATTLE_ENDED, 1);
                return Ok(());
            }
            // .dont_flee
            self.force_player_mon_choice()?;
            self.mem.wb(W_BATTLE_PLAYER_ACTION, BATTLEPLAYERACTION_USEITEM);
            if !self.handle_enemy_switch()? {
                self.unported("WildFled_EnemyFled_LinkBattleCanceled (forfeit)");
            }
            return self.double_switch();
        }
        // .player_mon_not_fainted
        self.mem.wb(W_BATTLE_PLAYER_ACTION, BATTLEPLAYERACTION_USEITEM);
        if !self.handle_enemy_switch()? {
            self.unported("WildFled_EnemyFled_LinkBattleCanceled (forfeit)");
        }
        self.mem.wb(W_BATTLE_PLAYER_ACTION, BATTLEPLAYERACTION_USEMOVE);
        Ok(())
    }

    /// `DoubleSwitch`.
    fn double_switch(&mut self) -> Flow {
        if self.mem.rb(H_SERIAL_CONNECTION_STATUS) == USING_EXTERNAL_CLOCK {
            let cur = self.mem.rb(W_CUR_PARTY_MON);
            self.enemy_party_mon_entrance(1)?;
            self.mem.wb(W_CUR_PARTY_MON, cur);
            self.player_party_mon_entrance();
        } else {
            self.player_party_mon_entrance();
            self.enemy_party_mon_entrance(1)?;
        }
        self.mem.wb(W_BATTLE_PLAYER_ACTION, BATTLEPLAYERACTION_USEMOVE);
        Ok(())
    }

    /// `UpdateBattleStateAndExperienceAfterEnemyFaint`.
    fn update_battle_state_and_experience_after_enemy_faint(&mut self) {
        self.update_battle_mon_in_party();
        if self.battle_mode() != WILD_BATTLE {
            let hp = W_OT_PARTY_MON1_HP + u16::from(self.mem.rb(W_CUR_OT_MON)) * PARTY_LEN;
            self.mem.ww(hp, 0);
        }
        // .wild
        self.mem.res_bit(W_PLAYER_SUB_STATUS3, SUBSTATUS_IN_LOOP);
        self.mem.ww(W_ENEMY_DAMAGE_TAKEN, 0);
        self.new_enemy_mon_status();
        self.break_attraction();
        if self.battle_mode() == WILD_BATTLE {
            self.mem.wb(W_LOW_HEALTH_ALARM, 0);
            self.mem.wb(W_BATTLE_LOW_HEALTH_ALARM, 1);
        }
        // .trainer
        if self.mem.rw(W_BATTLE_MON_HP) == 0 && self.mem.rb(W_WHICH_MON_FAINTED_FIRST) == 0 {
            self.update_fainted_player_mon();
        }
        // .player_mon_did_not_faint
        if self.check_player_party_for_fit_mon() == 0 {
            return;
        }
        let r0 = self.mem.rb(W_BATTLE_RESULT) & BATTLERESULT_BITMASK;
        self.mem.wb(W_BATTLE_RESULT, r0);
        let base_len = W_ENEMY_MON_END - W_ENEMY_MON_BASE_STATS;
        if self.is_any_mon_holding_exp_share().0 != 0 {
            for i in 0..base_len {
                let v = self.mem.rb(W_ENEMY_MON_BASE_STATS + i);
                self.mem.wb(W_ENEMY_MON_BASE_STATS + i, v >> 1);
            }
        }
        // .skip_exp
        self.mem.copy(W_BACKUP_ENEMY_MON_BASE_STATS, W_ENEMY_MON_BASE_STATS, base_len);
        self.mem.wb(W_GIVING_EXPERIENCE_TO_EXP_SHARE_HOLDERS, 0);
        self.give_experience_points();
        let (count, mask) = self.is_any_mon_holding_exp_share();
        if count == 0 {
            return;
        }
        let saved = self.mem.rb(W_BATTLE_PARTICIPANTS_NOT_FAINTED);
        self.mem.wb(W_BATTLE_PARTICIPANTS_NOT_FAINTED, mask);
        self.mem.copy(W_ENEMY_MON_BASE_STATS, W_BACKUP_ENEMY_MON_BASE_STATS, base_len);
        self.mem.wb(W_GIVING_EXPERIENCE_TO_EXP_SHARE_HOLDERS, 1);
        self.give_experience_points();
        self.mem.wb(W_BATTLE_PARTICIPANTS_NOT_FAINTED, saved);
    }

    /// `IsAnyMonHoldingExpShare`: (how many living mons hold one, their
    /// party bitmask `d`).
    fn is_any_mon_holding_exp_share(&self) -> (u8, u8) {
        let mut d = 0u8;
        let mut c = 1u8;
        for i in 0..u16::from(self.mem.rb(W_PARTY_COUNT)) {
            let mon = W_PARTY_MON1 + i * PARTY_LEN;
            if self.mem.rw(mon + u16::from(MON_HP)) != 0 && self.mem.rb(mon + u16::from(MON_ITEM)) == EXP_SHARE {
                d |= c;
            }
            c = c.wrapping_shl(1);
        }
        ((d & 0x3f).count_ones() as u8, d)
    }

    /// `FaintYourPokemon`.
    fn faint_your_pokemon(&mut self) {
        self.mem.wb(W_LOW_HEALTH_ALARM, 0);
        self.mem.wb(W_CRY_TRACKS, 0xf0);
        self.text(r::BATTLE_TEXT_MON_FAINTED);
    }

    /// `FaintEnemyPokemon`.
    fn faint_enemy_pokemon(&mut self) {
        self.text(r::BATTLE_TEXT_ENEMY_MON_FAINTED);
    }

    /// `CheckEnemyTrainerDefeated` (z = every OT mon has 0 HP; here: false).
    fn check_enemy_trainer_defeated(&self) -> bool {
        let mut a = 0u8;
        for i in 0..u16::from(self.mem.rb(W_OT_PARTY_COUNT)) {
            let hp = W_OT_PARTY_MON1_HP + i * PARTY_LEN;
            a |= self.mem.rb(hp) | self.mem.rb(hp + 1);
        }
        a != 0
    }

    /// `HandleEnemySwitch`. Returns nz (false = the link partner forfeited).
    fn handle_enemy_switch(&mut self) -> Flow<bool> {
        if self.link_mode() != 0 {
            self.link_battle_send_receive_action()?;
            if self.mem.rb(W_BATTLE_ACTION) == BATTLEACTION_FORFEIT {
                return Ok(false);
            }
        }
        // .not_linked
        if self.mem.rw(W_BATTLE_MON_HP) != 0 {
            self.enemy_party_mon_entrance(0)?;
            return Ok(true);
        }
        Ok(true) // inc a: nz
    }

    /// `EnemyPartyMonEntrance` (`a` = 0: `EnemySwitch`, else `EnemySwitch_SetMode`).
    fn enemy_party_mon_entrance(&mut self, a: u8) -> Flow {
        self.mem.wb(W_ENEMY_SWITCH_MON_INDEX, 0);
        self.new_enemy_mon_status();
        self.reset_enemy_stat_levels();
        self.break_attraction();
        if a == 0 {
            self.enemy_switch()?;
        } else {
            self.enemy_switch_set_mode();
        }
        self.reset_battle_participants();
        self.set_enemy_turn();
        self.spikes_damage();
        self.mem.wb(W_ENEMY_MOVE_STRUCT_ANIMATION, 0);
        self.mem.wb(W_BATTLE_PLAYER_ACTION, 0);
        Ok(())
    }

    /// `WinTrainerBattle`.
    fn win_trainer_battle(&mut self) {
        self.mem.wb(W_LOW_HEALTH_ALARM, 0);
        self.mem.wb(W_BATTLE_LOW_HEALTH_ALARM, 1);
        self.mem.wb(W_BATTLE_ENDED, 1);
        self.text(r::BATTLE_TEXT_ENEMY_WAS_DEFEATED);
        if self.link_mode() != 0 {
            return;
        }
        self.unported("WinTrainerBattle (money)")
    }

    /// `HandlePlayerMonFaint`.
    pub fn handle_player_mon_faint(&mut self) -> Flow {
        self.faint_your_pokemon();
        if self.mem.rw(W_ENEMY_MON_HP) == 0 {
            self.faint_enemy_pokemon();
        }
        self.mem.wb(W_WHICH_MON_FAINTED_FIRST, 1);
        self.update_fainted_player_mon();
        if self.check_player_party_for_fit_mon() == 0 {
            self.lost_battle();
            return Ok(());
        }
        let enemy_fainted = self.mem.rw(W_ENEMY_MON_HP) == 0;
        if enemy_fainted {
            self.update_battle_state_and_experience_after_enemy_faint();
            if self.battle_mode() == WILD_BATTLE {
                self.mem.wb(W_BATTLE_ENDED, 1);
                return Ok(());
            }
            if !self.check_enemy_trainer_defeated() {
                self.win_trainer_battle();
                return Ok(());
            }
        }
        // .notfainted
        if self.ask_use_next_pokemon() {
            self.mem.wb(W_BATTLE_ENDED, 1);
            return Ok(());
        }
        // .switch
        if self.force_player_mon_choice()? != 0 {
            return Ok(());
        }
        self.mem.wb(W_BATTLE_PLAYER_ACTION, BATTLEPLAYERACTION_USEITEM);
        if !self.handle_enemy_switch()? {
            self.unported("WildFled_EnemyFled_LinkBattleCanceled (forfeit)");
        }
        self.double_switch()
    }

    /// `UpdateFaintedPlayerMon`.
    fn update_fainted_player_mon(&mut self) {
        let cur = self.mem.rb(W_CUR_BATTLE_MON);
        self.small_far_flag_action(W_BATTLE_PARTICIPANTS_NOT_FAINTED, cur, FlagAction::Reset);
        self.mem.res_bit(W_ENEMY_SUB_STATUS3, SUBSTATUS_IN_LOOP);
        self.mem.wb(W_LOW_HEALTH_ALARM, 0);
        self.mem.ww(W_PLAYER_DAMAGE_TAKEN, 0);
        self.mem.wb(W_BATTLE_MON_STATUS, 0);
        self.update_battle_mon_in_party();
        let mut c = HAPPINESS_FAINTED;
        let b = self.mem.rb(W_BATTLE_MON_LEVEL).wrapping_add(30);
        if self.mem.rb(W_ENEMY_MON_LEVEL) >= b {
            c = HAPPINESS_BEATENBYSTRONGFOE;
        }
        self.mem.wb(W_CUR_PARTY_MON, cur);
        self.change_happiness(c);
        let r0 = (self.mem.rb(W_BATTLE_RESULT) & BATTLERESULT_BITMASK).wrapping_add(LOSE);
        self.mem.wb(W_BATTLE_RESULT, r0);
    }

    /// `ChangeHappiness`: happiness action `c` on `wCurPartyMon`.
    pub fn change_happiness(&mut self, c: u8) {
        let cur = self.mem.rb(W_CUR_PARTY_MON);
        if self.mem.rb(W_PARTY_SPECIES + u16::from(cur)) == EGG {
            return;
        }
        let de = W_PARTY_MON1_HAPPINESS + u16::from(cur) * PARTY_LEN;
        let happiness = self.mem.rb(de);
        let e = if happiness < HAPPINESS_THRESHOLD_1 { 0 } else if happiness < HAPPINESS_THRESHOLD_2 { 1 } else { 2 };
        let change = rom::byte(r::HAPPINESS_CHANGES + 3 * usize::from(c.wrapping_sub(1)) + e);
        let new = if change < 0x64 {
            happiness.checked_add(change).unwrap_or(0xff)
        } else {
            let (sum, carry) = happiness.overflowing_add(change);
            if carry { sum } else { 0 }
        };
        self.mem.wb(de, new);
        if self.battle_mode() == 0 {
            return;
        }
        if self.mem.rb(W_PARTY_MENU_CURSOR) != cur {
            return;
        }
        self.mem.wb(W_BATTLE_MON_HAPPINESS, new);
    }

    /// `AskUseNextPokemon` (patched: always "yes"). Returns carry (flee).
    fn ask_use_next_pokemon(&mut self) -> bool {
        self.mem.wb(W_MENU_CURSOR_Y, 1);
        false
    }

    /// `ForcePlayerMonChoice`. Returns `c` (nonzero = sent a mon out now).
    fn force_player_mon_choice(&mut self) -> Flow<u8> {
        self.force_pick_party_mon_in_battle()?;
        if self.link_mode() != 0 {
            self.mem.wb(W_BATTLE_PLAYER_ACTION, BATTLEPLAYERACTION_USEITEM);
            self.link_battle_send_receive_action()?;
        }
        // .skip_link
        self.mem.wb(W_BATTLE_PLAYER_ACTION, BATTLEPLAYERACTION_USEMOVE);
        if self.mem.rw(W_ENEMY_MON_HP) == 0 {
            return Ok(0);
        }
        // .send_out_pokemon
        let old = self.mem.rb(W_CUR_BATTLE_MON);
        self.mem.wb(W_LAST_PLAYER_MON, old);
        let cur = self.mem.rb(W_CUR_PARTY_MON);
        self.mem.wb(W_CUR_BATTLE_MON, cur);
        self.add_battle_participant();
        self.init_battle_mon();
        self.reset_player_stat_levels();
        self.send_out_mon_text();
        self.new_battle_mon_status();
        self.break_attraction();
        self.send_out_player_mon();
        self.set_player_turn();
        self.spikes_damage();
        Ok(1)
    }

    /// `ForcePickPartyMonInBattle` (patched: `bootstrap::cli::handle_forced_switch`).
    pub fn force_pick_party_mon_in_battle(&mut self) -> Flow {
        let step = self.decide(crate::bootstrap::decision::PromptSite::ForcedSwitch)?;
        let crate::bootstrap::decision::Step::ForcedSwitch { slot } = step else { unreachable!() };
        self.mem.wb(W_CUR_PARTY_MON, slot);
        Ok(())
    }

    /// `LostBattle`.
    fn lost_battle(&mut self) {
        self.mem.wb(W_BATTLE_ENDED, 1);
        if self.mem.bit(W_IN_BATTLE_TOWER_BATTLE, IN_BATTLE_TOWER_BATTLE_F) {
            self.unported("LostBattle (Battle Tower)");
        }
        if self.mem.rb(W_BATTLE_TYPE) == BATTLETYPE_CANLOSE {
            self.unported("LostBattle (BATTLETYPE_CANLOSE)");
        }
        if self.link_mode() == 0 {
            return;
        }
        // .LostLinkBattle
        self.update_enemy_mon_in_party();
        if !self.check_enemy_trainer_defeated() {
            self.text(r::TIED_AGAINST_TEXT);
            let r0 = (self.mem.rb(W_BATTLE_RESULT) & BATTLERESULT_BITMASK).wrapping_add(DRAW);
            self.mem.wb(W_BATTLE_RESULT, r0);
        } else {
            self.text(r::LOST_AGAINST_TEXT);
        }
    }

    // --- the enemy's switch-in ------------------------------------------------------

    /// `AI_Switch` (`engine/battle/ai/items.asm`).
    pub fn ai_switch(&mut self) -> Flow<bool> {
        self.mem.wb(W_ENEMY_IS_SWITCHING, 1);
        self.mem.wb(W_ENEMY_GOES_FIRST, 1);
        self.mem.res_bit(W_ENEMY_SUB_STATUS4, SUBSTATUS_RAGE);
        self.mem.wb(H_BATTLE_TURN, 0);
        let pursued = self.pursuit_switch()?;
        let de = W_OT_PARTY_MON1_STATUS + u16::from(self.mem.rb(W_CUR_OT_MON)) * PARTY_LEN;
        self.mem.copy(de, W_ENEMY_MON_STATUS, u16::from(MON_MAXHP - MON_STATUS));
        if !pursued {
            self.text(r::ENEMY_WITHDREW_TEXT);
        }
        self.mem.wb(W_BATTLE_HAS_JUST_STARTED, 1);
        self.new_enemy_mon_status();
        self.reset_enemy_stat_levels();
        self.mem.res_bit(W_PLAYER_SUB_STATUS1, SUBSTATUS_IN_LOVE);
        self.enemy_switch()?;
        self.reset_battle_participants();
        self.mem.wb(W_BATTLE_HAS_JUST_STARTED, 0);
        Ok(self.link_mode() == 0)
    }

    /// `EnemySwitch`.
    pub fn enemy_switch(&mut self) -> Flow {
        if !self.check_whether_to_ask_switch() {
            self.enemy_switch_set_mode();
            return Ok(());
        }
        self.unported("EnemySwitch (shift mode / OfferSwitch)")
    }

    /// `EnemySwitch_SetMode`.
    pub fn enemy_switch_set_mode(&mut self) {
        self.reset_enemy_battle_vars();
        let b = match self.check_whether_switchmon_is_predetermined() {
            Some(b) => b,
            None => self.find_mon_in_ot_party_to_switch_into_battle(),
        };
        self.load_enemy_mon_to_switch_to(b);
        self.mem.wb(W_ENEMY_IS_SWITCHING, 1);
        self.show_battle_text_enemy_sent_out();
        self.show_set_enemy_mon_and_send_out_animation();
    }

    /// `FindMonInOTPartyToSwitchIntoBattle`.
    fn find_mon_in_ot_party_to_switch_into_battle(&mut self) -> u8 {
        self.unported("FindMonInOTPartyToSwitchIntoBattle")
    }

    /// `CheckWhetherToAskSwitch` (carry = ask the player).
    fn check_whether_to_ask_switch(&mut self) -> bool {
        if self.mem.rb(W_BATTLE_HAS_JUST_STARTED) == 1 {
            return false;
        }
        if self.mem.rb(W_PARTY_COUNT) == 1 {
            return false;
        }
        if self.link_mode() != 0 {
            return false;
        }
        if self.mem.bit(W_OPTIONS, BATTLE_SHIFT) {
            return false;
        }
        self.unported("CheckWhetherToAskSwitch (CheckCurPartyMonFainted)")
    }

    /// `CheckWhetherSwitchmonIsPredetermined`: `Some(b)` (carry) or `None`.
    fn check_whether_switchmon_is_predetermined(&self) -> Option<u8> {
        if self.link_mode() != 0 {
            return Some(self.mem.rb(W_BATTLE_ACTION).wrapping_sub(BATTLEACTION_SWITCH1));
        }
        let index = self.mem.rb(W_ENEMY_SWITCH_MON_INDEX);
        if index != 0 {
            return Some(index - 1);
        }
        if self.mem.rb(W_BATTLE_HAS_JUST_STARTED) != 0 {
            return Some(0);
        }
        None
    }

    /// `ResetEnemyBattleVars`.
    fn reset_enemy_battle_vars(&mut self) {
        for addr in [W_LAST_PLAYER_COUNTER_MOVE, W_LAST_ENEMY_COUNTER_MOVE, W_LAST_ENEMY_MOVE, W_CUR_ENEMY_MOVE] {
            self.mem.wb(addr, 0);
        }
        self.mem.wb(W_ENEMY_ITEM_STATE, 0xff);
        self.mem.wb(W_PLAYER_WRAP_COUNT, 0);
    }

    /// `LoadEnemyMonToSwitchTo` (`b` = OT party slot).
    fn load_enemy_mon_to_switch_to(&mut self, b: u8) {
        self.mem.wb(W_CUR_PARTY_MON, b);
        let level = self.mem.rb(W_OT_PARTY_MON1_LEVEL + u16::from(b) * PARTY_LEN);
        self.mem.wb(W_CUR_PARTY_LEVEL, level);
        let species = self.mem.rb(W_OT_PARTY_COUNT + 1 + u16::from(b));
        self.mem.wb(W_TEMP_ENEMY_MON_SPECIES, species);
        self.mem.wb(W_CUR_PARTY_SPECIES, species);
        self.load_enemy_mon();
        if self.mem.rb(W_CUR_PARTY_SPECIES) == UNOWN && self.mem.rb(W_FIRST_UNOWN_SEEN) == 0 {
            self.get_unown_letter(W_ENEMY_MON_D_VS);
            let letter = self.mem.rb(W_UNOWN_LETTER);
            self.mem.wb(W_FIRST_UNOWN_SEEN, letter);
        }
        // .skip_unown
        let (hi, lo) = (self.mem.rb(W_ENEMY_MON_HP), self.mem.rb(W_ENEMY_MON_HP + 1));
        self.mem.wb(W_ENEMY_HP_AT_TIME_OF_PLAYER_SWITCH, hi);
        self.mem.wb(W_ENEMY_HP_AT_TIME_OF_PLAYER_SWITCH + 1, lo);
    }

    /// `LoadEnemyMon`.
    pub fn load_enemy_mon(&mut self) {
        self.mem.fill(W_ENEMY_MON_SPECIES, W_ENEMY_MON_END - W_ENEMY_MON, 0);
        if self.link_mode() != 0 || self.mem.bit(W_IN_BATTLE_TOWER_BATTLE, IN_BATTLE_TOWER_BATTLE_F) {
            self.init_enemy_mon();
            return;
        }
        self.unported("LoadEnemyMon (wild / trainer)")
    }

    /// `ShowBattleTextEnemySentOut`.
    fn show_battle_text_enemy_sent_out(&mut self) {
        // Battle_GetTrainerName fills wStringBuffer1 for the text (ported with
        // the text engine).
        self.text(r::BATTLE_TEXT_ENEMY_SENT_OUT);
    }

    /// `ShowSetEnemyMonAndSendOutAnimation`: the memory side.
    fn show_set_enemy_mon_and_send_out_animation(&mut self) {
        let species = self.mem.rb(W_TEMP_ENEMY_MON_SPECIES);
        self.mem.wb(W_CUR_PARTY_SPECIES, species);
        self.mem.wb(W_CUR_SPECIES, species);
        self.get_base_data();
        self.mem.wb(W_MON_TYPE, OTPARTYMON);
        self.copy_mon_to_temp_mon();
        self.mem.wb(W_BATTLE_AFTER_ANIM, 0);
        self.mem.wb(W_BATTLE_ANIM_PARAM, 0);
        self.set_enemy_turn();
        if self.battle_check_enemy_shininess() {
            self.mem.wb(W_BATTLE_ANIM_PARAM, 1);
        }
        if !self.check_fainted_frz_slp(W_TEMP_MON_SPECIES) && self.mem.bit(W_OPTIONS, BATTLE_SCENE) {
            // .cry_no_anim
            self.mem.wb(W_CRY_TRACKS, 0xf);
        }
    }

    /// `BattleCheckEnemyShininess`.
    fn battle_check_enemy_shininess(&self) -> bool {
        let dvs = if !self.mem.bit(W_ENEMY_SUB_STATUS5, SUBSTATUS_TRANSFORMED) {
            W_ENEMY_MON_D_VS
        } else if self.battle_mode() == WILD_BATTLE {
            W_ENEMY_BACKUP_D_VS
        } else {
            W_OT_PARTY_MON1_D_VS + u16::from(self.mem.rb(W_CUR_OT_MON)) * PARTY_LEN
        };
        check_shininess(self.mem.rb(dvs), self.mem.rb(dvs + 1))
    }

    /// `CopyMonToTempMon` for `wMonType` = `OTPARTYMON`.
    fn copy_mon_to_temp_mon(&mut self) {
        if self.mem.rb(W_MON_TYPE) != OTPARTYMON {
            self.unported("CopyMonToTempMon (not OTPARTYMON)");
        }
        let src = W_OT_PARTY_MON1_SPECIES + u16::from(self.mem.rb(W_CUR_PARTY_MON)) * PARTY_LEN;
        self.mem.copy(W_TEMP_MON_SPECIES, src, PARTY_LEN);
    }
}

/// `SmallFarFlagAction` modes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlagAction {
    Reset,
    Set,
    Check,
}

/// `CheckShininess`: the DVs make a shiny mon.
pub fn check_shininess(atk_def: u8, spd_spc: u8) -> bool {
    // Attack DV 2, 3, 6, 7, 10, 11, 14 or 15; Defense, Speed, Special DV 10.
    atk_def & 1 << 5 != 0 && atk_def & 0x0f == 10 && spd_spc >> 4 == 10 && spd_spc & 0x0f == 10
}
