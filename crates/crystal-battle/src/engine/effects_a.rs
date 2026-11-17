//! Port of the move effects attract..mist (`engine/battle/move_effects/*.asm`)
//! and the helpers they use. See docs/engine.md.
//!
//! Animations and delays are dropped; every battle-WRAM write is kept.

#![allow(unused_imports)]

use super::battle::{Engine, Flow, PARTY_LEN, PP_MASK, percent};
use super::commands::Cmd;
use super::names::Name;
use super::mem::{GLITCH_ZERO, glitch_range_kept};
use super::rsym as r;
use super::sym::*;
use crate::defs::*;

/// `wBattleMonPP - wBattleMonMoves`.
const PP_OFFSET: u16 = 6;

impl Engine {
    // --- shared helpers from effect_commands.asm ---------------------------------

    /// `ResetMiss`.
    pub fn reset_miss(&mut self) {
        self.mem.wb(W_ATTACK_MISSED, 0);
    }

    /// `GetMoveAttr` for the `Moves` table: attribute `attr` of move `mv` (1-based).
    fn move_attr(&self, mv: u8, attr: u8) -> u8 {
        self.data.mv(mv).field(attr)
    }

    /// `GetTypeName`: the name of type `wNamedObjectIndex` into `wStringBuffer1`.
    pub fn get_type_name(&mut self) {
        let kind = self.mem.rb(W_NAMED_OBJECT_INDEX);
        self.names.string_buffer1 = Name::Type(kind);
    }

    /// `ProtectChance` (`protect.asm`): carry (`true`) = failed.
    pub fn protect_chance(&mut self) -> Flow<bool> {
        let de = if self.is_player_turn() { W_PLAYER_PROTECT_COUNT } else { W_ENEMY_PROTECT_COUNT };
        let ok = 'chance: {
            if self.check_opponent_went_first() != 0 {
                break 'chance false;
            }
            if self.battle_var_bit(BATTLE_VARS_SUBSTATUS4, SUBSTATUS_SUBSTITUTE) {
                break 'chance false;
            }
            // Halve the chance of a successful Protect for each consecutive use.
            let mut b: u8 = 0xff;
            let mut c = self.mem.rb(de);
            while c != 0 {
                c -= 1;
                b >>= 1;
                if b == 0 {
                    break 'chance false;
                }
            }
            let a = loop {
                let a = self.battle_random("ProtectChance");
                if a != 0 {
                    break a;
                }
            };
            if a - 1 >= b {
                break 'chance false;
            }
            let v = self.mem.rb(de).wrapping_add(1);
            self.mem.wb(de, v);
            true
        };
        if ok {
            return Ok(false);
        }
        self.mem.wb(de, 0);
        self.animate_failed_move()?;
        self.print_but_it_failed();
        Ok(true)
    }

    /// `GetGender`: `Some(true)` male, `Some(false)` female, `None` genderless
    /// (carry). `wMonType` selects the struct, as in the original.
    pub fn get_gender(&self) -> Option<bool> {
        let mon_type = self.mem.rb(W_MON_TYPE);
        let dvs = match mon_type {
            0 => W_PARTY_MON1_D_VS + u16::from(self.mem.rb(W_CUR_PARTY_MON)) * PARTY_LEN,
            1 => W_OT_PARTY_MON1_D_VS + u16::from(self.mem.rb(W_CUR_PARTY_MON)) * PARTY_LEN,
            2 => self.unported("GetGender (sBoxMon)"),
            3 => W_TEMP_MON_D_VS,
            _ => W_ENEMY_MON_D_VS,
        };
        let b = (self.mem.rb(dvs) & 0xf0) | (self.mem.rb(dvs + 1) & 0xf0) >> 4;
        let species = self.mem.rb(W_CUR_PARTY_SPECIES);
        let ratio = self.data.species(species).gender_ratio;
        if ratio == 0xff {
            return None;
        }
        if ratio == GENDER_F0 {
            return Some(true);
        }
        if ratio == GENDER_F100 {
            return Some(false);
        }
        Some(ratio < b)
    }

    // --- attract.asm -------------------------------------------------------------

    /// `BattleCommand_Attract`.
    pub fn battle_command_attract(&mut self) -> Flow<Cmd> {
        if self.mem.rb(W_ATTACK_MISSED) != 0
            || self.check_opposite_gender()
            || self.check_hidden_opponent()
            || self.battle_var_bit(BATTLE_VARS_SUBSTATUS1_OPP, SUBSTATUS_IN_LOVE)
        {
            { self.fail_move()?; return Ok(Cmd::Next); }
        }
        self.set_battle_var_bit(BATTLE_VARS_SUBSTATUS1_OPP, SUBSTATUS_IN_LOVE);
        self.animate_current_move()?;
        self.text(r::FELL_IN_LOVE_TEXT);
        Ok(Cmd::Next)
    }

    /// `CheckOppositeGender`: carry (`true`) = same gender or genderless.
    pub fn check_opposite_gender(&mut self) -> bool {
        let species = self.mem.rb(self.battle_party_attr(MON_SPECIES));
        self.mem.wb(W_CUR_PARTY_SPECIES, species);
        let cur = self.mem.rb(W_CUR_BATTLE_MON);
        self.mem.wb(W_CUR_PARTY_MON, cur);
        self.mem.wb(W_MON_TYPE, 0);
        let Some(male) = self.get_gender() else { return true };
        let b = u8::from(male);

        let species = self.mem.rb(W_TEMP_ENEMY_MON_SPECIES);
        self.mem.wb(W_CUR_PARTY_SPECIES, species);
        let hl = if self.mem.bit(W_ENEMY_SUB_STATUS5, SUBSTATUS_TRANSFORMED) { W_ENEMY_BACKUP_D_VS } else { W_ENEMY_MON_D_VS };
        let (d0, d1) = (self.mem.rb(hl), self.mem.rb(hl + 1));
        self.mem.wb(W_TEMP_MON_D_VS, d0);
        self.mem.wb(W_TEMP_MON_D_VS + 1, d1);
        self.mem.wb(W_MON_TYPE, TEMPMON);
        let Some(male) = self.get_gender() else { return true };
        u8::from(male) ^ b == 0
    }

    // --- baton_pass.asm ----------------------------------------------------------

    /// `BattleCommand_BatonPass`.
    pub fn battle_command_baton_pass(&mut self) -> Flow<Cmd> {
        if !self.is_player_turn() {
            return self.baton_pass_enemy();
        }
        // Need something to switch to.
        if !self.check_any_other_alive_party_mons() {
            return self.failed_baton_pass();
        }
        self.update_battle_mon_in_party();
        self.animate_current_move()?;
        // ForcePickSwitchMonInBattle
        loop {
            self.force_pick_party_mon_in_battle()?;
            // SwitchMonAlreadyOut
            if self.mem.rb(W_CUR_PARTY_MON) != self.mem.rb(W_CUR_BATTLE_MON) {
                break;
            }
            self.text(r::BATTLE_TEXT_MON_IS_ALREADY_OUT);
        }
        self.baton_pass_link_player_switch()?;
        self.passed_battle_mon_entrance();
        self.reset_baton_pass_status();
        Ok(Cmd::Next)
    }

    /// `BattleCommand_BatonPass.Enemy`.
    fn baton_pass_enemy(&mut self) -> Flow<Cmd> {
        // Wildmons don't have anything to switch to.
        if self.battle_mode() == WILD_BATTLE || !self.check_any_other_alive_enemy_mons() {
            return self.failed_baton_pass();
        }
        self.update_enemy_mon_in_party();
        self.animate_current_move()?;
        self.baton_pass_link_enemy_switch()?;
        self.mem.wb(W_ENEMY_SWITCH_MON_INDEX, 0);
        self.enemy_switch_set_mode()?;
        self.reset_battle_participants();
        self.mem.wb(W_APPLY_STAT_LEVEL_MULTIPLIERS_TO_ENEMY, 1);
        self.apply_stat_level_multiplier_on_all_stats();
        self.spikes_damage();
        self.reset_baton_pass_status();
        Ok(Cmd::Next)
    }

    /// `BatonPass_LinkPlayerSwitch`.
    fn baton_pass_link_player_switch(&mut self) -> Flow {
        if self.link_mode() == 0 {
            return Ok(());
        }
        self.mem.wb(W_BATTLE_PLAYER_ACTION, BATTLEPLAYERACTION_USEITEM);
        self.link_battle_send_receive_action()?;
        self.mem.wb(W_BATTLE_PLAYER_ACTION, BATTLEPLAYERACTION_USEMOVE);
        Ok(())
    }

    /// `BatonPass_LinkEnemySwitch`.
    fn baton_pass_link_enemy_switch(&mut self) -> Flow {
        if self.link_mode() == 0 {
            return Ok(());
        }
        self.link_battle_send_receive_action()?;
        let b = self.mem.rb(W_OT_PARTY_COUNT).wrapping_add(BATTLEACTION_SWITCH1);
        let a = self.mem.rb(W_BATTLE_ACTION);
        if a < BATTLEACTION_SWITCH1 || a >= b {
            // .baton_pass
            let v = self.mem.rb(W_CUR_OT_MON).wrapping_add(BATTLEACTION_SWITCH1);
            self.mem.wb(W_BATTLE_ACTION, v);
        }
        Ok(())
    }

    /// `FailedBatonPass`.
    fn failed_baton_pass(&mut self) -> Flow<Cmd> {
        self.animate_failed_move()?;
        self.print_but_it_failed();
        Ok(Cmd::Next)
    }

    /// `ResetBatonPassStatus`: reset status changes that aren't passed.
    fn reset_baton_pass_status(&mut self) {
        // Nightmare isn't passed.
        if self.battle_var(BATTLE_VARS_STATUS) & SLP_MASK == 0 {
            self.res_battle_var_bit(BATTLE_VARS_SUBSTATUS1, SUBSTATUS_NIGHTMARE);
        }
        // Disable isn't passed.
        self.reset_actor_disable();
        // Attraction isn't passed.
        self.mem.res_bit(W_PLAYER_SUB_STATUS1, SUBSTATUS_IN_LOVE);
        self.mem.res_bit(W_ENEMY_SUB_STATUS1, SUBSTATUS_IN_LOVE);
        self.res_battle_var_bit(BATTLE_VARS_SUBSTATUS5, SUBSTATUS_TRANSFORMED);
        self.res_battle_var_bit(BATTLE_VARS_SUBSTATUS5, SUBSTATUS_ENCORED);
        // New mon hasn't used a move yet.
        self.set_battle_var(BATTLE_VARS_LAST_MOVE, 0);
        self.mem.wb(W_PLAYER_WRAP_COUNT, 0);
        self.mem.wb(W_ENEMY_WRAP_COUNT, 0);
    }

    /// `CheckAnyOtherAlivePartyMons`.
    pub fn check_any_other_alive_party_mons(&self) -> bool {
        self.check_any_other_alive_mons(W_PARTY_MON1_HP, self.mem.rb(W_PARTY_COUNT), self.mem.rb(W_CUR_BATTLE_MON))
    }

    /// `CheckAnyOtherAliveEnemyMons`.
    pub fn check_any_other_alive_enemy_mons(&self) -> bool {
        self.check_any_other_alive_mons(W_OT_PARTY_MON1_HP, self.mem.rb(W_OT_PARTY_COUNT), self.mem.rb(W_CUR_OT_MON))
    }

    /// `CheckAnyOtherAliveMons`: nz (`true`) if any of the `d` mons starting at
    /// HP address `hl`, besides `e`, has HP.
    fn check_any_other_alive_mons(&self, mut hl: u16, d: u8, e: u8) -> bool {
        let mut b = 0u8;
        for c in 0..d {
            if c != e {
                b |= self.mem.rb(hl);
                b |= self.mem.rb(hl + 1);
            }
            hl = hl.wrapping_add(PARTY_LEN);
        }
        b != 0
    }

    // --- beat_up.asm -------------------------------------------------------------

    /// `BattleCommand_BeatUp`.
    pub fn battle_command_beat_up(&mut self) -> Flow<Cmd> {
        self.reset_damage();
        if !self.is_player_turn() {
            return self.beat_up_enemy();
        }
        if self.mem.bit(W_PLAYER_SUB_STATUS3, SUBSTATUS_IN_LOOP) {
            // .next_mon
            let b = self.mem.rb(W_PLAYER_ROLLOUT_COUNT);
            let a = self.mem.rb(W_PARTY_COUNT).wrapping_sub(b);
            self.mem.wb(W_CUR_BEAT_UP_PARTY_MON, a);
        } else {
            self.mem.wb(W_PLAYER_ROLLOUT_COUNT, 0);
            self.mem.wb(W_CUR_BEAT_UP_PARTY_MON, 0);
            self.mem.wb(W_BEAT_UP_HIT_AT_LEAST_ONCE, 0);
        }
        // .got_mon
        // BUG: Beat Up can desynchronize link battles
        let cur = self.mem.rb(W_CUR_BEAT_UP_PARTY_MON);
        // A stale loop counter can run `cur` past the party (see
        // `get_beatup_mon_location`): the name is then garbage, which the
        // engine doesn't reproduce (an empty name).
        self.get_nickname(W_PARTY_MON_NICKNAMES, cur);
        let hp = self.get_beatup_mon_location(MON_HP);
        if self.mem.rb(hp) | self.mem.rb(hp + 1) == 0 {
            return self.beat_up_fail();
        }
        // `cp [hl]` compares wCurBattleMon with the HP's low byte.
        let status = if self.mem.rb(W_CUR_BATTLE_MON) == self.mem.rb(hp + 1) {
            W_BATTLE_MON_STATUS
        } else {
            self.get_beatup_mon_location(MON_STATUS)
        };
        if self.mem.rb(status) != 0 {
            return self.beat_up_fail();
        }
        self.mem.wb(W_BEAT_UP_HIT_AT_LEAST_ONCE, 1);
        self.text(r::BEAT_UP_ATTACK_TEXT);
        let species = self.mem.rb(W_ENEMY_MON_SPECIES);
        self.beat_up_damage_inputs(species, W_PLAYER_MOVE_STRUCT_POWER);
        Ok(Cmd::Next)
    }

    /// The shared tail of Beat Up: `b` = the Beat Up mon's base Attack, `c` =
    /// the target's base Defense, `d` = move power, `e` = level.
    fn beat_up_damage_inputs(&mut self, target_species: u8, power: u16) {
        self.mem.wb(W_CUR_SPECIES, target_species);
        self.get_base_data();
        let c = self.mem.rb(W_BASE_DEFENSE);
        let loc = self.get_beatup_mon_location(MON_SPECIES);
        let species = self.mem.rb(loc);
        self.mem.wb(W_CUR_SPECIES, species);
        self.get_base_data();
        let b = self.mem.rb(W_BASE_ATTACK);
        let loc = self.get_beatup_mon_location(MON_LEVEL);
        let e = self.mem.rb(loc);
        let d = self.mem.rb(power);
        self.damage_regs = [b, c, d, e];
    }

    /// `BattleCommand_BeatUp.enemy_beats_up`.
    fn beat_up_enemy(&mut self) -> Flow<Cmd> {
        if self.mem.bit(W_ENEMY_SUB_STATUS3, SUBSTATUS_IN_LOOP) {
            let b = self.mem.rb(W_ENEMY_ROLLOUT_COUNT);
            let a = self.mem.rb(W_OT_PARTY_COUNT).wrapping_sub(b);
            self.mem.wb(W_CUR_BEAT_UP_PARTY_MON, a);
        } else {
            self.mem.wb(W_ENEMY_ROLLOUT_COUNT, 0);
            self.mem.wb(W_CUR_BEAT_UP_PARTY_MON, 0);
            self.mem.wb(W_BEAT_UP_HIT_AT_LEAST_ONCE, 0);
        }
        // .enemy_got_mon
        if self.battle_mode() == WILD_BATTLE {
            // .wild
            let species = self.mem.rb(W_ENEMY_MON_SPECIES);
            self.mem.wb(W_NAMED_OBJECT_INDEX, species);
            self.get_pokemon_name();
            self.text(r::BEAT_UP_ATTACK_TEXT);
            self.enemy_attack_damage();
            return Ok(Cmd::Next);
        }
        let cur = self.mem.rb(W_CUR_BEAT_UP_PARTY_MON);
        if self.link_mode() != 0 || self.mem.rb(W_IN_BATTLE_TOWER_BATTLE) != 0 {
            // .link_or_tower
            let hl = W_OT_PARTY_MON_NICKNAMES + u16::from(NAME_LENGTH) * u16::from(cur);
            self.copy_name(W_STRING_BUFFER1, hl);
        } else {
            let species = self.mem.rb(W_OT_PARTY_SPECIES + u16::from(cur));
            self.mem.wb(W_NAMED_OBJECT_INDEX, species);
            self.get_pokemon_name();
        }
        // .got_enemy_nick
        let hp = self.get_beatup_mon_location(MON_HP);
        if self.mem.rb(hp) | self.mem.rb(hp + 1) == 0 {
            return self.beat_up_fail();
        }
        let status = if self.mem.rb(W_CUR_OT_MON) == cur { W_ENEMY_MON_STATUS } else { self.get_beatup_mon_location(MON_STATUS) };
        if self.mem.rb(status) != 0 {
            return self.beat_up_fail();
        }
        self.mem.wb(W_BEAT_UP_HIT_AT_LEAST_ONCE, 1);
        // .finish_beatup
        self.text(r::BEAT_UP_ATTACK_TEXT);
        let species = self.mem.rb(W_BATTLE_MON_SPECIES);
        self.beat_up_damage_inputs(species, W_ENEMY_MOVE_STRUCT_POWER);
        Ok(Cmd::Next)
    }

    /// `BattleCommand_BeatUp.beatup_fail`.
    fn beat_up_fail(&mut self) -> Flow<Cmd> {
        self.skip_to_battle_command(BUILDOPPONENTRAGE_COMMAND);
        Ok(Cmd::Next)
    }

    /// `BattleCommand_BeatUpFailText`.
    /// BUG: Beat Up may trigger King's Rock even if it failed.
    pub fn battle_command_beat_up_fail_text(&mut self) -> Flow<Cmd> {
        if self.mem.rb(W_BEAT_UP_HIT_AT_LEAST_ONCE) != 0 {
            return Ok(Cmd::Next);
        }
        self.print_but_it_failed();
        Ok(Cmd::Next)
    }

    /// `GetBeatupMonLocation`: address of field `offset` of the current Beat Up mon.
    ///
    /// BUG (the original's): a Beat Up that misses leaves `SUBSTATUS_IN_LOOP`
    /// set, the next one underflows the loop counter and walks the party index
    /// through all 256 values: past the party, through echo RAM, OAM and I/O,
    /// wrapping around to ROM. Where that lands on state neither engine keeps
    /// (the live stack, sound, graphics, I/O) the harness reads zeros
    /// (a HARNESS-PATCH in the transpiled routine; [`GLITCH_ZERO`]).
    fn get_beatup_mon_location(&self, offset: u8) -> u16 {
        let base = if self.is_player_turn() { W_PARTY_MON1_SPECIES } else { W_OT_PARTY_MON1_SPECIES };
        let n = self.mem.rb(W_CUR_BEAT_UP_PARTY_MON);
        let hl = self.get_party_location(base.wrapping_add(u16::from(offset)), n);
        if glitch_range_kept(hl, 2) { hl } else { GLITCH_ZERO }
    }

    // --- belly_drum.asm ----------------------------------------------------------

    /// `BattleCommand_BellyDrum`.
    /// BUG: Belly Drum sharply boosts Attack even with under 50% HP.
    pub fn battle_command_belly_drum(&mut self) -> Flow<Cmd> {
        self.battle_command_attack_up2()?;
        if self.mem.rb(W_ATTACK_MISSED) == 0 {
            let bc = self.get_half_max_hp();
            if self.check_user_has_enough_hp(bc) {
                self.animate_current_move()?;
                self.subtract_hp_from_user(bc);
                self.update_user_in_party();
                for _ in 0..MAX_STAT_LEVEL - BASE_STAT_LEVEL - 1 {
                    self.battle_command_attack_up2()?;
                }
                self.text(r::BELLY_DRUM_TEXT);
                return Ok(Cmd::Next);
            }
        }
        // .failed
        self.animate_failed_move()?;
        self.print_but_it_failed();
        Ok(Cmd::Next)
    }

    // --- bide.asm ----------------------------------------------------------------

    /// `BattleCommand_StoreEnergy`.
    pub fn battle_command_store_energy(&mut self) -> Flow<Cmd> {
        if !self.battle_var_bit(BATTLE_VARS_SUBSTATUS3, SUBSTATUS_BIDE) {
            return Ok(Cmd::Next);
        }
        let player = self.is_player_turn();
        let count = if player { W_PLAYER_ROLLOUT_COUNT } else { W_ENEMY_ROLLOUT_COUNT };
        let left = self.mem.rb(count).wrapping_sub(1);
        self.mem.wb(count, left);
        if left != 0 {
            // .still_storing
            self.text(r::STORING_ENERGY_TEXT);
            self.end_move_effect();
            return Ok(Cmd::Next);
        }
        self.res_battle_var_bit(BATTLE_VARS_SUBSTATUS3, SUBSTATUS_BIDE);
        self.text(r::UNLEASHED_ENERGY_TEXT);
        self.set_battle_var(BATTLE_VARS_MOVE_POWER, 1);
        let (taken, charging) = if player { (W_PLAYER_DAMAGE_TAKEN, W_PLAYER_CHARGING) } else { (W_ENEMY_DAMAGE_TAKEN, W_ENEMY_CHARGING) };
        // Double the damage taken, capped at $ffff.
        let low = self.mem.rb(taken + 1);
        let b = low << 1;
        self.mem.wb(W_CUR_DAMAGE + 1, b);
        let high = self.mem.rb(taken);
        let a = high << 1 | low >> 7;
        self.mem.wb(W_CUR_DAMAGE, a);
        let a = if high >> 7 != 0 {
            self.mem.wb(W_CUR_DAMAGE, 0xff);
            self.mem.wb(W_CUR_DAMAGE + 1, 0xff);
            0xff
        } else {
            a
        };
        if a | b == 0 {
            self.mem.wb(W_ATTACK_MISSED, 1);
        }
        // .built_up_something
        self.mem.wb(taken, 0);
        self.mem.wb(taken + 1, 0);
        self.mem.wb(charging, 0);
        self.set_battle_var(BATTLE_VARS_MOVE_ANIM, BIDE);
        self.skip_to_battle_command(UNLEASHENERGY_COMMAND);
        Ok(Cmd::Next)
    }

    /// `BattleCommand_UnleashEnergy`.
    pub fn battle_command_unleash_energy(&mut self) -> Flow<Cmd> {
        let (de, bc) = if self.is_player_turn() {
            (W_PLAYER_DAMAGE_TAKEN, W_PLAYER_ROLLOUT_COUNT)
        } else {
            (W_ENEMY_DAMAGE_TAKEN, W_ENEMY_ROLLOUT_COUNT)
        };
        self.set_battle_var_bit(BATTLE_VARS_SUBSTATUS3, SUBSTATUS_BIDE);
        self.mem.wb(de, 0);
        self.mem.wb(de + 1, 0);
        self.mem.wb(W_PLAYER_MOVE_STRUCT_EFFECT, 0);
        self.mem.wb(W_ENEMY_MOVE_STRUCT_EFFECT, 0);
        let a = (self.battle_random("BattleCommand_UnleashEnergy") & 1) + 2;
        self.mem.wb(bc, a);
        self.mem.wb(W_BATTLE_ANIM_PARAM, 1);
        self.animate_current_move()?;
        self.end_move_effect();
        Ok(Cmd::Next)
    }

    // --- conversion.asm, conversion2.asm -----------------------------------------

    /// `BattleCommand_Conversion`.
    pub fn battle_command_conversion(&mut self) -> Flow<Cmd> {
        let (moves, de) = if self.is_player_turn() { (W_BATTLE_MON_MOVES, W_BATTLE_MON_TYPE1) } else { (W_ENEMY_MON_MOVES, W_ENEMY_MON_TYPE1) };
        // The types of the user's moves, into wStringBuffer1.
        let mut buf = W_STRING_BUFFER1;
        for c in 0..u16::from(NUM_MOVES) {
            let a = self.mem.rb(moves + c);
            if a == 0 {
                break;
            }
            self.mem.wb(buf, self.move_attr(a, MOVE_TYPE));
            buf += 1;
        }
        for _ in 0..3 {
            self.mem.wb(buf, 0xff);
            buf += 1;
        }
        // Is there a type that isn't the user's?
        let mut hl = W_STRING_BUFFER1;
        loop {
            let t = self.mem.rb(hl);
            if t == 0xff {
                return self.conversion_fail();
            }
            if t != CURSE_TYPE && self.mem.rb(de) != t && self.mem.rb(de + 1) != t {
                break;
            }
            hl += 1;
        }
        // .loop3
        loop {
            let c = u16::from(self.battle_random("BattleCommand_Conversion") & (NUM_MOVES - 1));
            let t = self.mem.rb(W_STRING_BUFFER1 + c);
            if t == 0xff || t == CURSE_TYPE || self.mem.rb(de) == t || self.mem.rb(de + 1) == t {
                continue;
            }
            self.mem.wb(de, t);
            self.mem.wb(de + 1, t);
            self.mem.wb(W_NAMED_OBJECT_INDEX, t);
            self.get_type_name();
            self.animate_current_move()?;
            self.text(r::TRANSFORMED_TYPE_TEXT);
            return Ok(Cmd::Next);
        }
    }

    fn conversion_fail(&mut self) -> Flow<Cmd> {
        self.animate_failed_move()?;
        self.print_but_it_failed();
        Ok(Cmd::Next)
    }

    /// `BattleCommand_Conversion2`.
    pub fn battle_command_conversion2(&mut self) -> Flow<Cmd> {
        if self.mem.rb(W_ATTACK_MISSED) != 0 {
            { self.fail_move()?; return Ok(Cmd::Next); }
        }
        let hl = if self.is_player_turn() { W_BATTLE_MON_TYPE1 } else { W_ENEMY_MON_TYPE1 };
        let a = self.battle_var(BATTLE_VARS_LAST_COUNTER_MOVE_OPP);
        if a == 0 {
            { self.fail_move()?; return Ok(Cmd::Next); }
        }
        let d = self.move_attr(a, MOVE_TYPE);
        if d == CURSE_TYPE {
            { self.fail_move()?; return Ok(Cmd::Next); }
        }
        self.animate_current_move()?;
        self.switch_turn();
        loop {
            // .loop
            let a = self.battle_random("BattleCommand_Conversion2") & 0x1f;
            if a >= UNUSED_TYPES {
                if a < UNUSED_TYPES_END {
                    continue;
                }
                if a >= TYPES_END {
                    continue;
                }
            }
            // .okay
            self.mem.wb(hl, a);
            self.mem.wb(hl + 1, a);
            let addr = self.battle_var_addr(BATTLE_VARS_MOVE_TYPE);
            let saved = self.mem.rb(addr);
            self.mem.wb(addr, d);
            self.battle_check_type_matchup();
            self.mem.wb(addr, saved);
            if self.mem.rb(W_TYPE_MATCHUP) >= EFFECTIVE {
                continue;
            }
            self.switch_turn();
            let t = self.mem.rb(hl);
            self.mem.wb(W_NAMED_OBJECT_INDEX, t);
            self.get_type_name();
            self.text(r::TRANSFORMED_TYPE_TEXT);
            return Ok(Cmd::Next);
        }
    }

    // --- counter.asm, mirror_coat.asm --------------------------------------------

    /// `BattleCommand_Counter`.
    /// BUG: Counter and Mirror Coat still work if the opponent uses an item.
    pub fn battle_command_counter(&mut self) -> Flow<Cmd> {
        self.counter_or_mirror_coat(EFFECT_COUNTER, false)
    }

    /// `BattleCommand_MirrorCoat`.
    pub fn battle_command_mirror_coat(&mut self) -> Flow<Cmd> {
        self.counter_or_mirror_coat(EFFECT_MIRROR_COAT, true)
    }

    /// The shared body of Counter (physical moves) and Mirror Coat (special).
    fn counter_or_mirror_coat(&mut self, effect: u8, special: bool) -> Flow<Cmd> {
        self.mem.wb(W_ATTACK_MISSED, 1);
        let b = self.battle_var(BATTLE_VARS_LAST_COUNTER_MOVE_OPP);
        if b == 0 || self.get_move_effect(b) == effect {
            return Ok(Cmd::Next);
        }
        self.battle_command_reset_type_matchup()?;
        if self.mem.rb(W_TYPE_MATCHUP) == 0 || self.check_opponent_went_first() == 0 {
            return Ok(Cmd::Next);
        }
        let b = self.battle_var(BATTLE_VARS_LAST_COUNTER_MOVE_OPP);
        self.get_move_data(b.wrapping_sub(1), W_STRING_BUFFER1);
        if self.mem.rb(W_STRING_BUFFER1 + u16::from(MOVE_POWER)) == 0 {
            return Ok(Cmd::Next);
        }
        let mtype = self.mem.rb(W_STRING_BUFFER1 + u16::from(MOVE_TYPE));
        if special != (mtype >= SPECIAL) {
            return Ok(Cmd::Next);
        }
        let hi = self.mem.rb(W_CUR_DAMAGE);
        let lo = self.mem.rb(W_CUR_DAMAGE + 1);
        if hi | lo == 0 {
            return Ok(Cmd::Next);
        }
        let doubled = (u16::from(hi) << 8 | u16::from(lo)) << 1;
        self.mem.ww(W_CUR_DAMAGE, doubled);
        if hi >> 7 != 0 {
            // .capped
            self.mem.wb(W_CUR_DAMAGE, 0xff);
            self.mem.wb(W_CUR_DAMAGE + 1, 0xff);
        }
        self.mem.wb(W_ATTACK_MISSED, 0);
        Ok(Cmd::Next)
    }

    // --- curse.asm ---------------------------------------------------------------

    /// `BattleCommand_Curse`.
    pub fn battle_command_curse(&mut self) -> Flow<Cmd> {
        let (de, bc) = if self.is_player_turn() { (W_BATTLE_MON_TYPE1, W_PLAYER_STAT_LEVELS) } else { (W_ENEMY_MON_TYPE1, W_ENEMY_STAT_LEVELS) };
        // Curse is different for Ghost-types.
        if self.mem.rb(de) == GHOST || self.mem.rb(de + 1) == GHOST {
            return self.curse_ghost();
        }
        // If no stats can be increased, don't.
        let raise = self.mem.rb(bc) < MAX_STAT_LEVEL || self.mem.rb(bc + 1) < MAX_STAT_LEVEL;
        if !raise {
            // .cantraise
            self.get_stat_name(ABILITY + 1);
            self.animate_failed_move()?;
            self.text(r::WONT_RISE_ANYMORE_TEXT);
            return Ok(Cmd::Next);
        }
        // Raise Attack and Defense, and lower Speed.
        self.mem.wb(W_BATTLE_ANIM_PARAM, 1);
        self.animate_current_move()?;
        self.lower_stat(SPEED);
        self.battle_command_switch_turn()?;
        self.battle_command_stat_down_message()?;
        self.reset_miss();
        self.battle_command_switch_turn()?;
        self.battle_command_attack_up()?;
        self.battle_command_stat_up_message()?;
        self.reset_miss();
        self.battle_command_defense_up()?;
        self.battle_command_stat_up_message()
    }

    /// `BattleCommand_Curse.ghost`: cut HP in half and put a curse on the opponent.
    fn curse_ghost(&mut self) -> Flow<Cmd> {
        if self.check_hidden_opponent()
            || self.check_substitute_opp()
            || self.battle_var_bit(BATTLE_VARS_SUBSTATUS1_OPP, SUBSTATUS_CURSE)
        {
            self.animate_failed_move()?;
            self.print_but_it_failed();
            return Ok(Cmd::Next);
        }
        self.set_battle_var_bit(BATTLE_VARS_SUBSTATUS1_OPP, SUBSTATUS_CURSE);
        self.animate_current_move()?;
        let bc = self.get_half_max_hp();
        self.subtract_hp_from_user(bc);
        self.update_user_in_party();
        self.text(r::PUT_A_CURSE_TEXT);
        Ok(Cmd::Next)
    }

    // --- destiny_bond.asm --------------------------------------------------------

    /// `BattleCommand_DestinyBond`.
    pub fn battle_command_destiny_bond(&mut self) -> Flow<Cmd> {
        self.set_battle_var_bit(BATTLE_VARS_SUBSTATUS5, SUBSTATUS_DESTINY_BOND);
        self.animate_current_move()?;
        self.text(r::DESTINY_BOND_EFFECT_TEXT);
        Ok(Cmd::Next)
    }

    // --- disable.asm -------------------------------------------------------------

    /// `BattleCommand_Disable`.
    pub fn battle_command_disable(&mut self) -> Flow<Cmd> {
        if self.mem.rb(W_ATTACK_MISSED) != 0 {
            { self.fail_move()?; return Ok(Cmd::Next); }
        }
        let player = self.is_player_turn();
        let (de, mut hl) = if player { (W_ENEMY_DISABLE_COUNT, W_ENEMY_MON_MOVES) } else { (W_PLAYER_DISABLE_COUNT, W_BATTLE_MON_MOVES) };
        if self.mem.rb(de) != 0 {
            { self.fail_move()?; return Ok(Cmd::Next); }
        }
        let last = self.battle_var(BATTLE_VARS_LAST_COUNTER_MOVE_OPP);
        if last == 0 || last == STRUGGLE {
            { self.fail_move()?; return Ok(Cmd::Next); }
        }
        let mut c = 0xffu8;
        loop {
            c = c.wrapping_add(1);
            let a = self.mem.rb(hl);
            hl = hl.wrapping_add(1);
            if a == last {
                break;
            }
        }
        let pp = if player { W_ENEMY_MON_PP } else { W_BATTLE_MON_PP };
        if self.mem.rb(pp.wrapping_add(u16::from(c))) == 0 {
            { self.fail_move()?; return Ok(Cmd::Next); }
        }
        let a = loop {
            let a = self.battle_random("BattleCommand_Disable") & 7;
            if a != 0 {
                break a;
            }
        };
        let v = (a + 1).wrapping_add((c + 1).rotate_left(4));
        self.mem.wb(de, v);
        self.animate_current_move()?;
        let disabled = if player { W_DISABLED_MOVE + 1 } else { W_DISABLED_MOVE };
        let last = self.battle_var(BATTLE_VARS_LAST_COUNTER_MOVE_OPP);
        self.mem.wb(disabled, last);
        self.mem.wb(W_NAMED_OBJECT_INDEX, last);
        self.get_move_name();
        self.text(r::WAS_DISABLED_TEXT);
        Ok(Cmd::Next)
    }

    // --- encore.asm --------------------------------------------------------------

    /// `BattleCommand_Encore`.
    pub fn battle_command_encore(&mut self) -> Flow<Cmd> {
        let player = self.is_player_turn();
        let (mut hl, de) = if player { (W_ENEMY_MON_MOVES, W_ENEMY_ENCORE_COUNT) } else { (W_BATTLE_MON_MOVES, W_PLAYER_ENCORE_COUNT) };
        let b = self.battle_var(BATTLE_VARS_LAST_MOVE_OPP);
        if b == 0 || b == STRUGGLE || b == ENCORE || b == MIRROR_MOVE {
            { self.print_didnt_affect2()?; return Ok(Cmd::Next); }
        }
        loop {
            let a = self.mem.rb(hl);
            hl = hl.wrapping_add(1);
            if a == b {
                break;
            }
        }
        let pp = hl.wrapping_add(PP_OFFSET - 1);
        if self.mem.rb(pp) & PP_MASK == 0 || self.mem.rb(W_ATTACK_MISSED) != 0 {
            { self.print_didnt_affect2()?; return Ok(Cmd::Next); }
        }
        let substatus5 = self.battle_var_addr(BATTLE_VARS_SUBSTATUS5_OPP);
        if self.mem.bit(substatus5, SUBSTATUS_ENCORED) {
            { self.print_didnt_affect2()?; return Ok(Cmd::Next); }
        }
        self.mem.set_bit(substatus5, SUBSTATUS_ENCORED);
        let a = (self.battle_random("BattleCommand_Encore") & 3) + 3;
        self.mem.wb(de, a);
        if self.check_opponent_went_first() == 0 {
            if !player {
                // Force the player's last move.
                let b = self.mem.rb(W_LAST_PLAYER_MOVE);
                let found = (0..NUM_MOVES).find(|&c| self.mem.rb(W_BATTLE_MON_MOVES + u16::from(c)) == b);
                let Some(c) = found else {
                    self.mem.res_bit(substatus5, SUBSTATUS_ENCORED);
                    self.mem.wb(de, 0);
                    { self.print_didnt_affect2()?; return Ok(Cmd::Next); }
                };
                self.mem.wb(W_CUR_MOVE_NUM, c);
                self.mem.wb(W_CUR_PLAYER_MOVE, b);
                self.get_move_data(b.wrapping_sub(1), W_PLAYER_MOVE_STRUCT);
            } else {
                // .force_last_enemy_move
                let b = self.mem.rb(W_LAST_ENEMY_MOVE);
                let found = (0..NUM_MOVES).find(|&c| self.mem.rb(W_ENEMY_MON_MOVES + u16::from(c)) == b);
                let Some(c) = found else {
                    self.mem.res_bit(substatus5, SUBSTATUS_ENCORED);
                    self.mem.wb(de, 0);
                    { self.print_didnt_affect2()?; return Ok(Cmd::Next); }
                };
                self.mem.wb(W_CUR_ENEMY_MOVE_NUM, c);
                self.mem.wb(W_CUR_ENEMY_MOVE, b);
                self.get_move_data(b.wrapping_sub(1), W_ENEMY_MOVE_STRUCT);
            }
        }
        // .finish_move
        self.animate_current_move()?;
        self.text(r::GOT_AN_ENCORE_TEXT);
        Ok(Cmd::Next)
    }

    // --- endure.asm --------------------------------------------------------------

    /// `BattleCommand_Endure` (shares `ProtectChance` with Protect).
    pub fn battle_command_endure(&mut self) -> Flow<Cmd> {
        if self.protect_chance()? {
            return Ok(Cmd::Next);
        }
        self.set_battle_var_bit(BATTLE_VARS_SUBSTATUS1, SUBSTATUS_ENDURE);
        self.animate_current_move()?;
        self.text(r::BRACED_ITSELF_TEXT);
        Ok(Cmd::Next)
    }

    // --- focus_energy.asm, foresight.asm, lock_on.asm, mist.asm ------------------

    /// `BattleCommand_FocusEnergy`.
    pub fn battle_command_focus_energy(&mut self) -> Flow<Cmd> {
        if self.battle_var_bit(BATTLE_VARS_SUBSTATUS4, SUBSTATUS_FOCUS_ENERGY) {
            self.animate_failed_move()?;
            self.print_but_it_failed();
            return Ok(Cmd::Next);
        }
        self.set_battle_var_bit(BATTLE_VARS_SUBSTATUS4, SUBSTATUS_FOCUS_ENERGY);
        self.animate_current_move()?;
        self.text(r::GETTING_PUMPED_TEXT);
        Ok(Cmd::Next)
    }

    /// `BattleCommand_Foresight`.
    pub fn battle_command_foresight(&mut self) -> Flow<Cmd> {
        if self.mem.rb(W_ATTACK_MISSED) != 0
            || self.check_hidden_opponent()
            || self.battle_var_bit(BATTLE_VARS_SUBSTATUS1_OPP, SUBSTATUS_IDENTIFIED)
        {
            { self.fail_move()?; return Ok(Cmd::Next); }
        }
        self.set_battle_var_bit(BATTLE_VARS_SUBSTATUS1_OPP, SUBSTATUS_IDENTIFIED);
        self.animate_current_move()?;
        self.text(r::IDENTIFIED_TEXT);
        Ok(Cmd::Next)
    }

    /// `BattleCommand_LockOn`.
    pub fn battle_command_lock_on(&mut self) -> Flow<Cmd> {
        if self.check_substitute_opp() || self.mem.rb(W_ATTACK_MISSED) != 0 {
            self.animate_failed_move()?;
            self.print_didnt_affect();
            return Ok(Cmd::Next);
        }
        self.set_battle_var_bit(BATTLE_VARS_SUBSTATUS5_OPP, SUBSTATUS_LOCK_ON);
        self.animate_current_move()?;
        self.text(r::TOOK_AIM_TEXT);
        Ok(Cmd::Next)
    }

    /// `BattleCommand_Mist`.
    pub fn battle_command_mist(&mut self) -> Flow<Cmd> {
        if self.battle_var_bit(BATTLE_VARS_SUBSTATUS4, SUBSTATUS_MIST) {
            self.animate_failed_move()?;
            self.print_but_it_failed();
            return Ok(Cmd::Next);
        }
        self.set_battle_var_bit(BATTLE_VARS_SUBSTATUS4, SUBSTATUS_MIST);
        self.animate_current_move()?;
        self.text(r::MIST_TEXT);
        Ok(Cmd::Next)
    }

    // --- frustration.asm, hidden_power.asm, magnitude.asm ------------------------

    /// `BattleCommand_FrustrationPower`: move power `d` from the user's happiness.
    /// BUG: Return and Frustration deal no damage when the user's happiness is low or high, respectively.
    pub fn battle_command_frustration_power(&mut self) -> Flow<Cmd> {
        let hl = if self.is_player_turn() { W_BATTLE_MON_HAPPINESS } else { W_ENEMY_MON_HAPPINESS };
        let a = 0xffu8.wrapping_sub(self.mem.rb(hl));
        self.mem.wb(H_MULTIPLICAND + 2, a);
        self.mem.wb(H_MULTIPLICAND, 0);
        self.mem.wb(H_MULTIPLICAND + 1, 0);
        self.mem.wb(H_MULTIPLIER, 10);
        self.multiply();
        self.mem.wb(H_DIVISOR, 25);
        self.divide(4);
        self.damage_regs[2] = self.mem.rb(H_QUOTIENT + 3);
        Ok(Cmd::Next)
    }

    /// `BattleCommand_HiddenPower`.
    pub fn battle_command_hidden_power(&mut self) -> Flow<Cmd> {
        if self.mem.rb(W_ATTACK_MISSED) != 0 {
            return Ok(Cmd::Next);
        }
        self.hidden_power_damage()?;
        Ok(Cmd::Next)
    }

    /// `HiddenPowerDamage`: override Hidden Power's type and power from the user's DVs.
    pub fn hidden_power_damage(&mut self) -> Flow {
        let hl = if self.is_player_turn() { W_BATTLE_MON_D_VS } else { W_ENEMY_MON_D_VS };
        let (dv0, dv1) = (self.mem.rb(hl), self.mem.rb(hl + 1));
        // Power: the top bit of each DV.
        let a = (dv0.rotate_left(4) & 0b1000)
            | (dv0 & 0b1000) >> 1
            | (dv1.rotate_left(4) & 0b1000) >> 2
            | (dv1 & 0b1000) >> 3;
        // Multiply by 5, add Special & 3, divide by 2 and add 30 + 1.
        let a = a * 5 + (dv1 & 0b11);
        let d = (a >> 1) + 30 + 1;
        // Type: Def & 3 + (Atk & 3) << 2.
        let mut t = ((dv0 & 0b11) | ((dv0 & 0b11 << 4).rotate_left(4) << 2)) + 1;
        if t >= BIRD {
            t += 1;
            if t >= UNUSED_TYPES {
                t += UNUSED_TYPES_END - UNUSED_TYPES;
            }
        }
        self.set_battle_var(BATTLE_VARS_MOVE_TYPE, t);
        // The rest of the damage inputs, based on the new type, but keep base power.
        self.battle_command_damage_stats()?;
        self.damage_regs[2] = d;
        Ok(())
    }

    /// `BattleCommand_GetMagnitude`: move power `d` from a random magnitude.
    pub fn battle_command_get_magnitude(&mut self) -> Flow<Cmd> {
        let b = self.battle_random("BattleCommand_GetMagnitude");
        let &(_, d, magnitude) =
            self.data.tables.magnitude_power.iter().find(|&&(threshold, _, _)| threshold >= b).expect("the last threshold is $ff");
        self.mem.wb(W_TEXT_DECIMAL_BYTE, magnitude);
        self.battle_command_move_delay()?;
        self.text(r::MAGNITUDE_TEXT);
        self.damage_regs[2] = d;
        Ok(Cmd::Next)
    }

    // --- fury_cutter.asm ---------------------------------------------------------

    /// `BattleCommand_FuryCutter`.
    pub fn battle_command_fury_cutter(&mut self) -> Flow<Cmd> {
        let hl = if self.is_player_turn() { W_PLAYER_FURY_CUTTER_COUNT } else { W_ENEMY_FURY_CUTTER_COUNT };
        if self.mem.rb(W_ATTACK_MISSED) != 0 {
            self.reset_fury_cutter_count();
            return Ok(Cmd::Next);
        }
        let count = self.mem.rb(hl).wrapping_add(1);
        self.mem.wb(hl, count);
        // Damage capped at 5 turns' worth (16x).
        let mut b = if count >= 6 { 5 } else { count };
        loop {
            b = b.wrapping_sub(1);
            if b == 0 {
                return Ok(Cmd::Next);
            }
            // Double the damage.
            let lo = self.mem.rb(W_CUR_DAMAGE + 1);
            self.mem.wb(W_CUR_DAMAGE + 1, lo << 1);
            let hi = self.mem.rb(W_CUR_DAMAGE);
            self.mem.wb(W_CUR_DAMAGE, hi << 1 | lo >> 7);
            if hi >> 7 != 0 {
                // No overflow
                self.mem.wb(W_CUR_DAMAGE, 0xff);
                self.mem.wb(W_CUR_DAMAGE + 1, 0xff);
                return Ok(Cmd::Next);
            }
        }
    }

    // --- future_sight.asm --------------------------------------------------------

    /// `BattleCommand_CheckFutureSight`.
    pub fn battle_command_check_future_sight(&mut self) -> Flow<Cmd> {
        let (hl, de) = if self.is_player_turn() {
            (W_PLAYER_FUTURE_SIGHT_COUNT, W_PLAYER_FUTURE_SIGHT_DAMAGE)
        } else {
            (W_ENEMY_FUTURE_SIGHT_COUNT, W_ENEMY_FUTURE_SIGHT_DAMAGE)
        };
        if self.mem.rb(hl) != 1 {
            return Ok(Cmd::Next);
        }
        self.mem.wb(hl, 0);
        let (hi, lo) = (self.mem.rb(de), self.mem.rb(de + 1));
        self.mem.wb(W_CUR_DAMAGE, hi);
        self.mem.wb(W_CUR_DAMAGE + 1, lo);
        self.skip_to_battle_command(FUTURESIGHT_COMMAND);
        Ok(Cmd::Next)
    }

    /// `BattleCommand_FutureSight`.
    pub fn battle_command_future_sight(&mut self) -> Flow<Cmd> {
        if !self.check_user_is_charging() {
            let anim = self.battle_var(BATTLE_VARS_MOVE_ANIM);
            self.set_battle_var(BATTLE_VARS_LAST_COUNTER_MOVE, anim);
            self.set_battle_var(BATTLE_VARS_LAST_MOVE, anim);
        }
        // .AlreadyChargingFutureSight
        let player = self.is_player_turn();
        let hl = if player { W_PLAYER_FUTURE_SIGHT_COUNT } else { W_ENEMY_FUTURE_SIGHT_COUNT };
        if self.mem.rb(hl) != 0 {
            // .failed: `pop bc` + `jp EndMoveEffect` aborts the whole move.
            self.reset_damage();
            self.animate_failed_move()?;
            self.print_but_it_failed();
            self.end_move_effect();
            return Ok(Cmd::AbortMove);
        }
        self.mem.wb(hl, 4);
        self.battle_command_lower_sub()?;
        self.battle_command_move_delay()?;
        self.text(r::FORESAW_ATTACK_TEXT);
        self.battle_command_raise_sub()?;
        let de = if player { W_PLAYER_FUTURE_SIGHT_DAMAGE } else { W_ENEMY_FUTURE_SIGHT_DAMAGE };
        let (hi, lo) = (self.mem.rb(W_CUR_DAMAGE), self.mem.rb(W_CUR_DAMAGE + 1));
        self.mem.wb(de, hi);
        self.mem.wb(W_CUR_DAMAGE, 0);
        self.mem.wb(de + 1, lo);
        self.mem.wb(W_CUR_DAMAGE + 1, 0);
        self.end_move_effect();
        Ok(Cmd::Next)
    }

    // --- heal_bell.asm -----------------------------------------------------------

    /// `BattleCommand_HealBell`.
    pub fn battle_command_heal_bell(&mut self) -> Flow<Cmd> {
        self.res_battle_var_bit(BATTLE_VARS_SUBSTATUS1, SUBSTATUS_NIGHTMARE);
        let player = self.is_player_turn();
        let mut de = if player { W_PARTY_MON1_STATUS } else { W_OT_PARTY_MON1_STATUS };
        self.set_battle_var(BATTLE_VARS_STATUS, 0);
        for _ in 0..PARTY_LENGTH {
            self.mem.wb(de, 0);
            de = de.wrapping_add(PARTY_LEN);
        }
        self.animate_current_move()?;
        self.text(r::BELL_CHIMED_TEXT);
        if player { self.calc_player_stats() } else { self.calc_enemy_stats() }
        Ok(Cmd::Next)
    }

    // --- leech_seed.asm ----------------------------------------------------------

    /// `BattleCommand_LeechSeed`.
    pub fn battle_command_leech_seed(&mut self) -> Flow<Cmd> {
        if self.mem.rb(W_ATTACK_MISSED) == 0 && !self.check_substitute_opp() {
            let de = if self.is_player_turn() { W_ENEMY_MON_TYPE1 } else { W_BATTLE_MON_TYPE1 };
            if self.mem.rb(de) == GRASS || self.mem.rb(de + 1) == GRASS {
                // .grass
                self.animate_failed_move()?;
                self.print_doesnt_affect();
                return Ok(Cmd::Next);
            }
            if !self.battle_var_bit(BATTLE_VARS_SUBSTATUS4_OPP, SUBSTATUS_LEECH_SEED) {
                self.set_battle_var_bit(BATTLE_VARS_SUBSTATUS4_OPP, SUBSTATUS_LEECH_SEED);
                self.animate_current_move()?;
                self.text(r::WAS_SEEDED_TEXT);
                return Ok(Cmd::Next);
            }
        }
        // .evaded
        self.animate_failed_move()?;
        self.text(r::EVADED_TEXT);
        Ok(Cmd::Next)
    }

    // --- metronome.asm, mirror_move.asm, mimic.asm -------------------------------

    /// `BattleCommand_Metronome`.
    pub fn battle_command_metronome(&mut self) -> Flow<Cmd> {
        self.clear_last_move();
        if !self.check_user_is_charging() {
            let param = self.mem.rb(W_BATTLE_ANIM_PARAM);
            self.battle_command_lower_sub()?;
            self.mem.wb(W_BATTLE_ANIM_PARAM, param);
        }
        // .charging
        self.load_move_anim();
        loop {
            // .GetMove
            let b = self.battle_random("BattleCommand_Metronome");
            // No invalid moves.
            if b >= NUM_ATTACKS + 1 {
                continue;
            }
            // None of the moves in MetronomeExcepts.
            if self.data.tables.metronome_excepts.contains(&b) {
                continue;
            }
            // No moves the user already has.
            if self.check_user_move(b) {
                continue;
            }
            self.set_battle_var(BATTLE_VARS_MOVE, b);
            self.update_move_data();
            { self.reset_turn()?; return Ok(Cmd::Next); }
        }
    }

    /// `BattleCommand_MirrorMove`.
    pub fn battle_command_mirror_move(&mut self) -> Flow<Cmd> {
        self.clear_last_move();
        let hl = self.battle_var_addr(BATTLE_VARS_MOVE);
        let b = self.battle_var(BATTLE_VARS_LAST_COUNTER_MOVE_OPP);
        if b == 0 || self.check_user_move(b) {
            // .failed
            self.animate_failed_move()?;
            self.text(r::MIRROR_MOVE_FAILED_TEXT);
            self.end_move_effect();
            return Ok(Cmd::Next);
        }
        // .use
        self.mem.wb(hl, b);
        self.mem.wb(W_NAMED_OBJECT_INDEX, b);
        let de = self.battle_var_addr(BATTLE_VARS_MOVE_ANIM);
        self.get_move_data(b.wrapping_sub(1), de);
        let name = self.get_move_name();
        self.copy_name1(name);
        if !self.check_user_is_charging() {
            let param = self.mem.rb(W_BATTLE_ANIM_PARAM);
            self.battle_command_lower_sub()?;
            self.mem.wb(W_BATTLE_ANIM_PARAM, param);
        }
        // .done
        self.battle_command_move_delay()?;
        self.reset_turn()?;
        Ok(Cmd::Next)
    }

    /// `BattleCommand_Mimic`.
    pub fn battle_command_mimic(&mut self) -> Flow<Cmd> {
        self.clear_last_move();
        self.battle_command_move_delay()?;
        if self.mem.rb(W_ATTACK_MISSED) != 0 {
            return self.mimic_fail();
        }
        let mut hl = if self.is_player_turn() { W_BATTLE_MON_MOVES } else { W_ENEMY_MON_MOVES };
        if self.check_hidden_opponent() {
            return self.mimic_fail();
        }
        let b = self.battle_var(BATTLE_VARS_LAST_COUNTER_MOVE_OPP);
        if b == 0 || b == STRUGGLE {
            return self.mimic_fail();
        }
        for _ in 0..NUM_MOVES {
            let a = self.mem.rb(hl);
            hl += 1;
            if a == b {
                return self.mimic_fail();
            }
        }
        hl -= 1;
        loop {
            let a = self.mem.rb(hl);
            hl = hl.wrapping_sub(1);
            if a == MIMIC {
                break;
            }
        }
        hl += 1;
        let a = self.battle_var(BATTLE_VARS_LAST_COUNTER_MOVE_OPP);
        self.mem.wb(hl, a);
        self.mem.wb(W_NAMED_OBJECT_INDEX, a);
        self.mem.wb(hl + PP_OFFSET, 5);
        self.get_move_name();
        self.animate_current_move()?;
        self.text(r::MIMIC_LEARNED_MOVE_TEXT);
        Ok(Cmd::Next)
    }

    fn mimic_fail(&mut self) -> Flow<Cmd> {
        self.fail_mimic();
        Ok(Cmd::Next)
    }
}
