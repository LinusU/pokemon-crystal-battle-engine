//! `engine/battle/effect_commands.asm`: move execution. `DoTurn` checks
//! whether the user can act, `DoMove` copies the move's effect script into
//! `wBattleScriptBuffer` and runs it command by command, exactly like the
//! original (commands may rewrite the script pointer, and `ResetTurn` runs
//! `DoMove` re-entrantly).

use super::battle::{Engine, Flow, PP_MASK, percent};
use super::rom;
use super::rsym as r;
use super::sym::*;
use crate::defs::*;

/// What a command tells the script reader.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cmd {
    /// Read the next command.
    Next,
    /// The command popped the reader's return address and ended the move
    /// (`pop bc` + `jp EndMoveEffect`): return straight out of `DoMove`.
    AbortMove,
}

/// `endturn_command`; commands `>=` it stop the reader.
const ENDTURN_COMMAND: u8 = 0xfe;
/// `endmove_command`.
pub const ENDMOVE_COMMAND: u8 = 0xff;

impl Engine {
    /// `DoPlayerTurn`.
    pub fn do_player_turn(&mut self) -> Flow {
        self.set_player_turn();
        if self.mem.rb(W_BATTLE_PLAYER_ACTION) != BATTLEPLAYERACTION_USEMOVE {
            return Ok(());
        }
        self.do_turn()
    }

    /// `DoEnemyTurn`.
    pub fn do_enemy_turn(&mut self) -> Flow {
        self.set_enemy_turn();
        if self.link_mode() != 0 {
            let action = self.mem.rb(W_BATTLE_ACTION);
            if action != BATTLEACTION_STRUGGLE && action >= BATTLEACTION_SWITCH1 {
                return Ok(());
            }
        }
        self.do_turn()
    }

    /// `DoTurn`.
    pub fn do_turn(&mut self) -> Flow {
        self.mem.wb(W_TURN_ENDED, 0);
        self.check_turn()?;
        if self.mem.rb(W_TURN_ENDED) != 0 {
            return Ok(());
        }
        self.update_move_data();
        self.do_move()
    }

    /// `DoMove`.
    pub fn do_move(&mut self) -> Flow {
        let effect = self.battle_var(BATTLE_VARS_MOVE_EFFECT);
        let bank = rom::bank_of(r::MOVE_EFFECTS);
        let mut src = rom::flat(bank, rom::ptr(r::MOVE_EFFECTS_POINTERS + 2 * usize::from(effect)));
        let mut de = W_BATTLE_SCRIPT_BUFFER;
        loop {
            let a = rom::byte(src);
            src += 1;
            self.mem.wb(de, a);
            de += 1;
            if a == ENDMOVE_COMMAND {
                break;
            }
        }
        self.mem.ww_le(W_BATTLE_SCRIPT_BUFFER_ADDRESS, W_BATTLE_SCRIPT_BUFFER);
        loop {
            // .ReadMoveEffectCommand
            let hl = self.mem.rw_le(W_BATTLE_SCRIPT_BUFFER_ADDRESS);
            let a = self.mem.rb(hl);
            self.mem.ww_le(W_BATTLE_SCRIPT_BUFFER_ADDRESS, hl.wrapping_add(1));
            if a >= ENDTURN_COMMAND {
                return Ok(());
            }
            if self.battle_command(a)? == Cmd::AbortMove {
                return Ok(());
            }
        }
    }

    /// `EndMoveEffect`: overwrite the next three script bytes with `endmove`.
    pub fn end_move_effect(&mut self) {
        let hl = self.mem.rw_le(W_BATTLE_SCRIPT_BUFFER_ADDRESS);
        for i in 0..3 {
            self.mem.wb(hl.wrapping_add(i), ENDMOVE_COMMAND);
        }
    }

    /// Run effect command `id` (`BattleCommandPointers`).
    pub fn battle_command(&mut self, id: u8) -> Flow<Cmd> {
        match id {
            0x01 => self.battle_command_check_turn(),
            0x02 => self.battle_command_check_obedience(),
            0x03 => self.battle_command_used_move_text(),
            0x04 => self.battle_command_do_turn(),
            0x05 => self.battle_command_critical(),
            0x06 => self.battle_command_damage_stats(),
            0x07 => self.battle_command_stab(),
            0x08 => self.battle_command_damage_variation(),
            0x09 => self.battle_command_check_hit(),
            0x0a => self.battle_command_lower_sub(),
            0x0b => self.battle_command_move_anim_no_sub(),
            0x0c => self.battle_command_raise_sub(),
            0x0d => self.battle_command_failure_text(),
            0x0e => self.battle_command_apply_damage(),
            0x0f => self.battle_command_critical_text(),
            0x10 => self.battle_command_super_effective_text(),
            0x11 => self.battle_command_check_faint(),
            0x12 => self.battle_command_build_opponent_rage(),
            0x13 => self.battle_command_poison_target(),
            0x14 => self.battle_command_sleep_target(),
            0x15 => self.battle_command_drain_target(),
            0x16 => self.battle_command_eat_dream(),
            0x17 => self.battle_command_burn_target(),
            0x18 => self.battle_command_freeze_target(),
            0x19 => self.battle_command_paralyze_target(),
            0x1a => self.battle_command_selfdestruct(),
            0x1b => self.battle_command_mirror_move(),
            0x1c => self.battle_command_stat_up(),
            0x1d => self.battle_command_stat_down(),
            0x1e => self.battle_command_pay_day(),
            0x1f => self.battle_command_conversion(),
            0x20 => self.battle_command_reset_stats(),
            0x21 => self.battle_command_store_energy(),
            0x22 => self.battle_command_unleash_energy(),
            0x23 => self.battle_command_force_switch(),
            0x24 => self.battle_command_end_loop(),
            0x25 => self.battle_command_flinch_target(),
            0x26 => self.battle_command_ohko(),
            0x27 => self.battle_command_recoil(),
            0x28 => self.battle_command_mist(),
            0x29 => self.battle_command_focus_energy(),
            0x2a => self.battle_command_confuse(),
            0x2b => self.battle_command_confuse_target(),
            0x2c => self.battle_command_heal(),
            0x2d => self.battle_command_transform(),
            0x2e => self.battle_command_screen(),
            0x2f => self.battle_command_poison(),
            0x30 => self.battle_command_paralyze(),
            0x31 => self.battle_command_substitute(),
            0x32 => self.battle_command_recharge_next_turn(),
            0x33 => self.battle_command_mimic(),
            0x34 => self.battle_command_metronome(),
            0x35 => self.battle_command_leech_seed(),
            0x36 => self.battle_command_splash(),
            0x37 => self.battle_command_disable(),
            0x38 => self.battle_command_clear_text(),
            0x39 => self.battle_command_charge(),
            0x3a => self.battle_command_check_charge(),
            0x3b => self.battle_command_trap_target(),
            0x3c => self.battle_command_unused3_c(),
            0x3d => self.battle_command_rampage(),
            0x3e => self.battle_command_check_rampage(),
            0x3f => self.battle_command_constant_damage(),
            0x40 => self.battle_command_counter(),
            0x41 => self.battle_command_encore(),
            0x42 => self.battle_command_pain_split(),
            0x43 => self.battle_command_snore(),
            0x44 => self.battle_command_conversion2(),
            0x45 => self.battle_command_lock_on(),
            0x46 => self.battle_command_sketch(),
            0x47 => self.battle_command_defrost_opponent(),
            0x48 => self.battle_command_sleep_talk(),
            0x49 => self.battle_command_destiny_bond(),
            0x4a => self.battle_command_spite(),
            0x4b => self.battle_command_false_swipe(),
            0x4c => self.battle_command_heal_bell(),
            0x4d => self.battle_command_held_flinch(),
            0x4e => self.battle_command_triple_kick(),
            0x4f => self.battle_command_kick_counter(),
            0x50 => self.battle_command_thief(),
            0x51 => self.battle_command_arena_trap(),
            0x52 => self.battle_command_nightmare(),
            0x53 => self.battle_command_defrost(),
            0x54 => self.battle_command_curse(),
            0x55 => self.battle_command_protect(),
            0x56 => self.battle_command_spikes(),
            0x57 => self.battle_command_foresight(),
            0x58 => self.battle_command_perish_song(),
            0x59 => self.battle_command_start_sandstorm(),
            0x5a => self.battle_command_endure(),
            0x5b => self.battle_command_check_rollout(),
            0x5c => self.battle_command_rollout_power(),
            0x5d => self.battle_command_unused5_d(),
            0x5e => self.battle_command_fury_cutter(),
            0x5f => self.battle_command_attract(),
            0x60 => self.battle_command_happiness_power(),
            0x61 => self.battle_command_present(),
            0x62 => self.battle_command_damage_calc(),
            0x63 => self.battle_command_frustration_power(),
            0x64 => self.battle_command_safeguard(),
            0x65 => self.battle_command_check_safeguard(),
            0x66 => self.battle_command_get_magnitude(),
            0x67 => self.battle_command_baton_pass(),
            0x68 => self.battle_command_pursuit(),
            0x69 => self.battle_command_clear_hazards(),
            0x6a => self.battle_command_heal_morn(),
            0x6b => self.battle_command_heal_day(),
            0x6c => self.battle_command_heal_nite(),
            0x6d => self.battle_command_hidden_power(),
            0x6e => self.battle_command_start_rain(),
            0x6f => self.battle_command_start_sun(),
            0x70 => self.battle_command_attack_up(),
            0x71 => self.battle_command_defense_up(),
            0x72 => self.battle_command_speed_up(),
            0x73 => self.battle_command_special_attack_up(),
            0x74 => self.battle_command_special_defense_up(),
            0x75 => self.battle_command_accuracy_up(),
            0x76 => self.battle_command_evasion_up(),
            0x77 => self.battle_command_attack_up2(),
            0x78 => self.battle_command_defense_up2(),
            0x79 => self.battle_command_speed_up2(),
            0x7a => self.battle_command_special_attack_up2(),
            0x7b => self.battle_command_special_defense_up2(),
            0x7c => self.battle_command_accuracy_up2(),
            0x7d => self.battle_command_evasion_up2(),
            0x7e => self.battle_command_attack_down(),
            0x7f => self.battle_command_defense_down(),
            0x80 => self.battle_command_speed_down(),
            0x81 => self.battle_command_special_attack_down(),
            0x82 => self.battle_command_special_defense_down(),
            0x83 => self.battle_command_accuracy_down(),
            0x84 => self.battle_command_evasion_down(),
            0x85 => self.battle_command_attack_down2(),
            0x86 => self.battle_command_defense_down2(),
            0x87 => self.battle_command_speed_down2(),
            0x88 => self.battle_command_special_attack_down2(),
            0x89 => self.battle_command_special_defense_down2(),
            0x8a => self.battle_command_accuracy_down2(),
            0x8b => self.battle_command_evasion_down2(),
            0x8c => self.battle_command_stat_up_message(),
            0x8d => self.battle_command_stat_down_message(),
            0x8e => self.battle_command_stat_up_fail_text(),
            0x8f => self.battle_command_stat_down_fail_text(),
            0x90 => self.battle_command_effect_chance(),
            0x91 => self.battle_command_stat_down_anim(),
            0x92 => self.battle_command_stat_up_anim(),
            0x93 => self.battle_command_switch_turn(),
            0x94 => self.battle_command_fake_out(),
            0x95 => self.battle_command_belly_drum(),
            0x96 => self.battle_command_psych_up(),
            0x97 => self.battle_command_rage(),
            0x98 => self.battle_command_double_flying_damage(),
            0x99 => self.battle_command_double_underground_damage(),
            0x9a => self.battle_command_mirror_coat(),
            0x9b => self.battle_command_check_future_sight(),
            0x9c => self.battle_command_future_sight(),
            0x9d => self.battle_command_double_minimize_damage(),
            0x9e => self.battle_command_skip_sun_charge(),
            0x9f => self.battle_command_thunder_accuracy(),
            0xa0 => self.battle_command_teleport(),
            0xa1 => self.battle_command_beat_up(),
            0xa2 => self.battle_command_rage_damage(),
            0xa3 => self.battle_command_reset_type_matchup(),
            0xa4 => self.battle_command_all_stats_up(),
            0xa5 => self.battle_command_bide_fail_text(),
            0xa6 => self.battle_command_raise_sub_no_anim(),
            0xa7 => self.battle_command_lower_sub_no_anim(),
            0xa8 => self.battle_command_beat_up_fail_text(),
            0xa9 => self.battle_command_clear_miss_damage(),
            0xaa => self.battle_command_move_delay(),
            0xab => self.battle_command_move_anim(),
            0xac => self.battle_command_tri_status_chance(),
            0xad => self.battle_command_super_effective_loop_text(),
            0xae => self.battle_command_start_loop(),
            0xaf => self.battle_command_curl(),
            _ => panic!("bad effect command {id:#04x}"),
        }
    }

    /// `UpdateMoveData`: load the user's move struct, name the move.
    pub fn update_move_data(&mut self) {
        let de = self.battle_var_addr(BATTLE_VARS_MOVE_ANIM);
        let mv = self.battle_var(BATTLE_VARS_MOVE);
        self.mem.wb(W_CUR_SPECIES, mv);
        self.mem.wb(W_NAMED_OBJECT_INDEX, mv);
        self.get_move_data(mv.wrapping_sub(1), de);
        let name = self.get_move_name();
        self.copy_name1(name);
    }

    /// `GetMoveData`: copy move struct `index` (0-based) to `de`.
    pub fn get_move_data(&mut self, index: u8, de: u16) {
        let src = r::MOVES + usize::from(index) * usize::from(MOVE_LENGTH);
        for i in 0..u16::from(MOVE_LENGTH) {
            self.mem.wb(de + i, rom::byte(src + usize::from(i)));
        }
    }

    // --- CheckTurn ---------------------------------------------------------------

    /// `CheckTurn` / `BattleCommand_CheckTurn`.
    pub fn check_turn(&mut self) -> Flow {
        if self.battle_var(BATTLE_VARS_MOVE) == CANNOT_MOVE {
            self.end_turn();
            return Ok(());
        }
        for addr in [W_ATTACK_MISSED, W_EFFECT_FAILED, W_BATTLE_ANIM_PARAM, W_ALREADY_DISOBEYED, W_ALREADY_FAILED, W_SOMEONE_IS_RAMPAGING] {
            self.mem.wb(addr, 0);
        }
        self.mem.wb(W_TYPE_MODIFIER, EFFECTIVE);
        let player = self.is_player_turn();
        let (sub1, sub3, sub4, status_addr, disable_count, disabled_move, confuse_count, cur_move) = if player {
            (W_PLAYER_SUB_STATUS1, W_PLAYER_SUB_STATUS3, W_PLAYER_SUB_STATUS4, W_BATTLE_MON_STATUS,
             W_PLAYER_DISABLE_COUNT, W_DISABLED_MOVE, W_PLAYER_CONFUSE_COUNT, W_CUR_PLAYER_MOVE)
        } else {
            (W_ENEMY_SUB_STATUS1, W_ENEMY_SUB_STATUS3, W_ENEMY_SUB_STATUS4, W_ENEMY_MON_STATUS,
             W_ENEMY_DISABLE_COUNT, W_ENEMY_DISABLED_MOVE, W_ENEMY_CONFUSE_COUNT, W_CUR_ENEMY_MOVE)
        };

        if self.mem.bit(sub4, SUBSTATUS_RECHARGE) {
            self.mem.res_bit(sub4, SUBSTATUS_RECHARGE);
            self.text(r::MUST_RECHARGE_TEXT);
            self.cant_move();
            self.end_turn();
            return Ok(());
        }

        // .no_recharge
        let sleep = self.mem.rb(status_addr) & SLP_MASK;
        if sleep != 0 {
            let left = sleep - 1;
            self.mem.wb(status_addr, left);
            // Player: `and SLP_MASK`; enemy: `and a` — the same on this value.
            if left & SLP_MASK == 0 {
                // .woke_up
                self.text(r::WOKE_UP_TEXT);
                self.cant_move();
                if player { self.update_battle_mon_in_party() } else { self.update_enemy_mon_in_party() }
                self.mem.res_bit(sub1, SUBSTATUS_NIGHTMARE);
            } else {
                // .fast_asleep
                self.text(r::FAST_ASLEEP_TEXT);
                let mv = self.mem.rb(cur_move);
                if mv != SNORE && mv != SLEEP_TALK {
                    self.cant_move();
                    self.end_turn();
                    return Ok(());
                }
            }
        }

        // .not_asleep
        if self.mem.bit(status_addr, FRZ) {
            let mv = self.mem.rb(cur_move);
            if mv != FLAME_WHEEL && mv != SACRED_FIRE {
                self.text(r::FROZEN_SOLID_TEXT);
                self.cant_move();
                self.end_turn();
                return Ok(());
            }
        }

        // .not_frozen
        if self.mem.bit(sub3, SUBSTATUS_FLINCHED) {
            self.mem.res_bit(sub3, SUBSTATUS_FLINCHED);
            self.text(r::FLINCHED_TEXT);
            self.cant_move();
            self.end_turn();
            return Ok(());
        }

        // .not_flinched
        let count = self.mem.rb(disable_count);
        if count != 0 {
            let count = count - 1;
            self.mem.wb(disable_count, count);
            if count & 0xf == 0 {
                self.mem.wb(disable_count, 0);
                self.mem.wb(disabled_move, 0);
                self.text(r::DISABLED_NO_MORE_TEXT);
            }
        }

        // .not_disabled
        if self.mem.bit(sub3, SUBSTATUS_CONFUSED) {
            let left = self.mem.rb(confuse_count).wrapping_sub(1);
            self.mem.wb(confuse_count, left);
            if left == 0 {
                self.mem.res_bit(sub3, SUBSTATUS_CONFUSED);
                self.text(r::CONFUSED_NO_MORE_TEXT);
            } else {
                // .confused
                self.text(r::IS_CONFUSED_TEXT);
                self.mem.wb(W_BATTLE_AFTER_ANIM, 0);
                if self.battle_random("CheckTurn") < percent(50) + 1 {
                    let v = self.mem.rb(sub3) & 1 << SUBSTATUS_CONFUSED;
                    self.mem.wb(sub3, v);
                    if player {
                        self.hit_confusion()?;
                    } else {
                        // The enemy's copy of HitConfusion, inline in CheckEnemyTurn.
                        self.text(r::HURT_ITSELF_TEXT);
                        self.hit_self_in_confusion();
                        self.battle_command_damage_calc()?;
                        self.battle_command_lower_sub()?;
                        self.mem.wb(W_BATTLE_AFTER_ANIM, 0);
                        self.do_enemy_damage(true);
                        self.battle_command_raise_sub()?;
                    }
                    self.cant_move();
                    self.end_turn();
                    return Ok(());
                }
            }
        }

        // .not_confused
        if self.mem.bit(sub1, SUBSTATUS_IN_LOVE) {
            self.text(r::IN_LOVE_WITH_TEXT);
            self.mem.wb(W_BATTLE_AFTER_ANIM, 0);
            if self.battle_random("CheckTurn") >= percent(50) + 1 {
                self.text(r::INFATUATION_TEXT);
                self.cant_move();
                self.end_turn();
                return Ok(());
            }
        }

        // .not_infatuated
        let disabled = self.mem.rb(disabled_move);
        if disabled != 0 && disabled == self.mem.rb(cur_move) {
            self.move_disabled();
            self.cant_move();
            self.end_turn();
            return Ok(());
        }

        // .no_disabled_move
        if !self.mem.bit(status_addr, PAR) {
            return Ok(());
        }
        if self.battle_random("CheckTurn") >= percent(25) {
            return Ok(());
        }
        self.text(r::FULLY_PARALYZED_TEXT);
        self.cant_move();
        self.end_turn();
        Ok(())
    }

    /// `CantMove`.
    pub fn cant_move(&mut self) {
        self.res_battle_var_bit(BATTLE_VARS_SUBSTATUS1, SUBSTATUS_ROLLOUT);
        let addr = self.battle_var_addr(BATTLE_VARS_SUBSTATUS3);
        let v = self.mem.rb(addr) & !(1 << SUBSTATUS_BIDE | 1 << SUBSTATUS_RAMPAGE | 1 << SUBSTATUS_CHARGED);
        self.mem.wb(addr, v);
        self.reset_fury_cutter_count();
        let anim = self.battle_var(BATTLE_VARS_MOVE_ANIM);
        if anim != FLY && anim != DIG {
            return;
        }
        // .fly_dig: hl still points at SubStatus3.
        self.mem.res_bit(addr, SUBSTATUS_UNDERGROUND);
        self.mem.res_bit(addr, SUBSTATUS_FLYING);
        self.appear_user_raise_sub();
    }

    /// `OpponentCantMove`.
    pub fn opponent_cant_move(&mut self) {
        self.switch_turn();
        self.cant_move();
        self.switch_turn();
    }

    /// `EndTurn`.
    pub fn end_turn(&mut self) {
        self.mem.wb(W_TURN_ENDED, 1);
        self.reset_damage();
    }

    /// `ResetDamage`.
    pub fn reset_damage(&mut self) {
        self.mem.wb(W_CUR_DAMAGE, 0);
        self.mem.wb(W_CUR_DAMAGE + 1, 0);
    }

    /// `MoveDisabled`.
    pub fn move_disabled(&mut self) {
        self.res_battle_var_bit(BATTLE_VARS_SUBSTATUS3, SUBSTATUS_CHARGED);
        let mv = self.battle_var(BATTLE_VARS_MOVE);
        self.mem.wb(W_NAMED_OBJECT_INDEX, mv);
        self.get_move_name();
        self.text(r::DISABLED_MOVE_TEXT);
    }

    /// `HitConfusion` (the player's).
    pub fn hit_confusion(&mut self) -> Flow {
        self.text(r::HURT_ITSELF_TEXT);
        self.mem.wb(W_CRITICAL_HIT, 0);
        self.hit_self_in_confusion();
        self.battle_command_damage_calc()?;
        self.battle_command_lower_sub()?;
        self.mem.wb(W_BATTLE_AFTER_ANIM, 0);
        self.do_player_damage(true);
        self.battle_command_raise_sub()?;
        Ok(())
    }

    // --- obedience, move text, PP ------------------------------------------------

    /// `CheckUserIsCharging` (nz = charging).
    pub fn check_user_is_charging(&self) -> bool {
        let addr = if self.is_player_turn() { W_PLAYER_CHARGING } else { W_ENEMY_CHARGING };
        self.mem.rb(addr) != 0
    }

    /// `BattleCommand_CheckObedience`.
    pub fn battle_command_check_obedience(&mut self) -> Flow<Cmd> {
        if !self.is_player_turn() || self.check_user_is_charging() {
            return Ok(Cmd::Next);
        }
        if self.mem.rb(W_ALREADY_DISOBEYED) != 0 {
            return Ok(Cmd::Next);
        }
        self.mem.wb(W_ALREADY_DISOBEYED, 0);
        if self.link_mode() != 0 || self.mem.rb(W_IN_BATTLE_TOWER_BATTLE) != 0 {
            return Ok(Cmd::Next);
        }
        let ot = W_PARTY_MON1_ID + u16::from(self.mem.rb(W_CUR_BATTLE_MON)) * super::battle::PARTY_LEN;
        if self.mem.rb(W_PLAYER_ID) == self.mem.rb(ot) && self.mem.rb(W_PLAYER_ID + 1) == self.mem.rb(ot + 1) {
            return Ok(Cmd::Next);
        }
        // .obeylevel: the maximum obedience level comes from the Johto badges.
        let badges = self.mem.rb(W_JOHTO_BADGES);
        let c: u8 = if badges & (1 << RISINGBADGE) != 0 {
            MAX_LEVEL + 1
        } else if badges & (1 << STORMBADGE) != 0 {
            70
        } else if badges & (1 << FOGBADGE) != 0 {
            50
        } else if badges & (1 << HIVEBADGE) != 0 {
            30
        } else {
            10
        };
        let d = self.mem.rb(W_BATTLE_MON_LEVEL);
        let b = c.checked_add(d).unwrap_or(0xff);
        // .checklevel
        if c >= d {
            return Ok(Cmd::Next);
        }
        // .rand1
        let a = loop {
            let a = self.battle_random("BattleCommand_CheckObedience").rotate_left(4);
            if a < b {
                break a;
            }
        };
        if a < c {
            return Ok(Cmd::Next);
        }
        if self.ignore_sleep_only() {
            return Ok(Cmd::Next);
        }
        // .rand2
        let a = loop {
            let a = self.battle_random("BattleCommand_CheckObedience");
            if a < b {
                break a;
            }
        };
        let use_instead = a < c;
        if !(use_instead && self.obedience_use_instead()?) {
            if !use_instead {
                // b = levels above the obedience level; nap chance b/256.
                let b = d - c;
                let a = self.battle_random("BattleCommand_CheckObedience").rotate_left(4);
                match a.checked_sub(b) {
                    None => {
                        // .Nap
                        let status = loop {
                            let a = self.battle_random("BattleCommand_CheckObedience");
                            let v = a.wrapping_add(a).rotate_left(4) & SLP_MASK;
                            if v != 0 {
                                break v;
                            }
                        };
                        self.mem.wb(W_BATTLE_MON_STATUS, status);
                        self.text(r::BEGAN_TO_NAP_TEXT);
                        return self.end_disobedience();
                    }
                    Some(a) if a < b => {
                        self.text(r::WONT_OBEY_TEXT);
                        self.hit_confusion()?;
                        return self.end_disobedience();
                    }
                    Some(_) => {}
                }
            }
            // .DoNothing
            self.obedience_do_nothing();
        }
        self.end_disobedience()
    }

    /// `BattleCommand_CheckObedience.DoNothing`: one of four refusal texts.
    fn obedience_do_nothing(&mut self) {
        let text = match self.battle_random("BattleCommand_CheckObedience") & 0b11 {
            0 => r::LOAFING_AROUND_TEXT,
            1 => r::WONT_OBEY_TEXT,
            2 => r::TURNED_AWAY_TEXT,
            _ => r::IGNORED_ORDERS_TEXT,
        };
        self.text(text);
    }

    /// `BattleCommand_CheckObedience.UseInstead`: use a random other move with
    /// PP. Returns false where the original jumps to `.DoNothing`.
    fn obedience_use_instead(&mut self) -> Flow<bool> {
        if self.mem.rb(W_BATTLE_MON_MOVES + 1) == 0 || self.mem.rb(W_DISABLED_MOVE) != 0 {
            return Ok(false);
        }
        // .GetTotalPP (stops at the first empty move slot after the first)
        let mut total = 0u8;
        for i in 0..NUM_MOVES as u16 {
            total = total.wrapping_add(self.mem.rb(W_BATTLE_MON_PP + i) & PP_MASK);
            if i + 1 == NUM_MOVES as u16 || self.mem.rb(W_BATTLE_MON_MOVES + i + 1) == 0 {
                break;
            }
        }
        // .CheckMovePP
        let cur = self.mem.rb(W_CUR_MOVE_NUM);
        if self.mem.rb(W_BATTLE_MON_PP + u16::from(cur)) & PP_MASK == total {
            return Ok(false);
        }
        self.mem.wb(W_ALREADY_DISOBEYED, 1);
        let rows = self.mem.rb(W2_D_MENU_NUM_ROWS);
        let saved = cur;
        // .RandomMove
        let slot = loop {
            let a = self.battle_random("BattleCommand_CheckObedience") & (NUM_MOVES - 1);
            if a >= rows || a == saved {
                continue;
            }
            self.mem.wb(W_CUR_MOVE_NUM, a);
            if self.mem.rb(W_BATTLE_MON_PP + u16::from(a)) & PP_MASK != 0 {
                break a;
            }
        };
        let mv = self.mem.rb(W_BATTLE_MON_MOVES + u16::from(slot));
        self.mem.wb(W_CUR_PLAYER_MOVE, mv);
        self.set_player_turn();
        self.update_move_data();
        self.do_move()?;
        self.mem.wb(W_CUR_MOVE_NUM, saved);
        Ok(true)
    }

    /// `BattleCommand_CheckObedience.EndDisobedience`.
    fn end_disobedience(&mut self) -> Flow<Cmd> {
        self.mem.wb(W_LAST_PLAYER_MOVE, 0);
        self.mem.wb(W_LAST_PLAYER_COUNTER_MOVE, 0);
        let v = self.mem.rb(W_PLAYER_SUB_STATUS5) & !(1 << SUBSTATUS_ENCORED);
        self.mem.wb(W_PLAYER_SUB_STATUS5, v);
        self.mem.wb(W_PLAYER_ENCORE_COUNT, 0);
        self.end_move_effect();
        Ok(Cmd::Next)
    }

    /// `IgnoreSleepOnly`: Snore / Sleep Talk while asleep are ignored outright
    /// (carry = true).
    fn ignore_sleep_only(&mut self) -> bool {
        let anim = self.battle_var(BATTLE_VARS_MOVE_ANIM);
        if anim != SNORE && anim != SLEEP_TALK {
            return false;
        }
        if self.battle_var(BATTLE_VARS_STATUS) & SLP_MASK == 0 {
            return false;
        }
        self.text(r::IGNORED_SLEEPING_TEXT);
        self.end_move_effect();
        true
    }

    /// `BattleCommand_UsedMoveText`: `DisplayUsedMoveText`, whose text runs
    /// `UsedMoveText`'s `text_asm`: record the used move for the AI and the
    /// user's last move / last counter move.
    pub fn battle_command_used_move_text(&mut self) -> Flow<Cmd> {
        if self.is_player_turn() {
            let mv = self.mem.rb(W_PLAYER_MOVE_STRUCT_ANIMATION);
            self.update_used_moves(mv);
        }
        // .start
        let last = self.battle_var_addr(BATTLE_VARS_LAST_MOVE);
        let last_counter = self.battle_var_addr(BATTLE_VARS_LAST_COUNTER_MOVE);
        let anim = self.battle_var(BATTLE_VARS_MOVE_ANIM);
        self.mem.wb(W_MOVE_GRAMMAR, anim);
        if !self.check_user_is_charging() {
            self.mem.wb(last_counter, anim);
            self.mem.wb(last, anim);
        }
        // .grammar
        self.get_move_grammar();
        self.text(r::USED_MOVE_TEXT);
        Ok(Cmd::Next)
    }

    /// `GetMoveGrammar`.
    fn get_move_grammar(&mut self) {
        let mv = self.mem.rb(W_MOVE_GRAMMAR);
        let mut b = 0u8;
        let mut p = r::MOVE_GRAMMAR;
        loop {
            let a = rom::byte(p);
            p += 1;
            if a == 0xff || a == mv {
                break;
            }
            if a == 0 {
                b += 1;
            }
        }
        self.mem.wb(W_MOVE_GRAMMAR, b);
    }

    /// `UpdateUsedMoves`.
    fn update_used_moves(&mut self, mv: u8) {
        for i in 0..NUM_MOVES as u16 {
            let a = self.mem.rb(W_PLAYER_USED_MOVES + i);
            if a == 0 {
                self.mem.wb(W_PLAYER_USED_MOVES + i, mv);
                return;
            }
            if a == mv {
                return;
            }
        }
        for i in 0..3 {
            let v = self.mem.rb(W_PLAYER_USED_MOVES + i + 1);
            self.mem.wb(W_PLAYER_USED_MOVES + i, v);
        }
        self.mem.wb(W_PLAYER_USED_MOVES + 3, mv);
    }

    /// `BattleCommand_DoTurn`: count the turn, use PP.
    pub fn battle_command_do_turn(&mut self) -> Flow<Cmd> {
        if self.check_user_is_charging() {
            return Ok(Cmd::Next);
        }
        let player = self.is_player_turn();
        let (pp, sub3, turns) = if player {
            (W_BATTLE_MON_PP, W_PLAYER_SUB_STATUS3, W_PLAYER_TURNS_TAKEN)
        } else {
            (W_ENEMY_MON_PP, W_ENEMY_SUB_STATUS3, W_ENEMY_TURNS_TAKEN)
        };
        let t = self.mem.rb(turns).wrapping_add(1);
        self.mem.wb(turns, t);
        if self.battle_var(BATTLE_VARS_MOVE) == STRUGGLE {
            return Ok(Cmd::Next);
        }
        if self.mem.rb(sub3) & (1 << SUBSTATUS_IN_LOOP | 1 << SUBSTATUS_RAMPAGE | 1 << SUBSTATUS_BIDE) != 0 {
            return Ok(Cmd::Next);
        }
        if self.do_turn_consume_pp(pp) != 0 {
            self.end_move_effect();
            return Ok(Cmd::Next);
        }
        // SubStatus5
        if self.mem.bit(sub3 + 2, SUBSTATUS_TRANSFORMED) {
            return Ok(Cmd::Next);
        }
        let hl = if player {
            W_PARTY_MON1_PP + u16::from(self.mem.rb(W_CUR_BATTLE_MON)) * super::battle::PARTY_LEN
        } else {
            if self.battle_mode() == WILD_BATTLE {
                return self.do_turn_wild();
            }
            W_OT_PARTY_MON1_PP + u16::from(self.mem.rb(W_CUR_OT_MON)) * super::battle::PARTY_LEN
        };
        if self.check_mimic_used() {
            return Ok(Cmd::Next);
        }
        self.do_turn_consume_pp(hl);
        Ok(Cmd::Next)
    }

    /// `BattleCommand_DoTurn.wild`.
    fn do_turn_wild(&mut self) -> Flow<Cmd> {
        let c = u16::from(self.mem.rb(W_CUR_ENEMY_MOVE_NUM));
        let is_mimic = self.mem.rb(W_ENEMY_MON_MOVES + c) == MIMIC;
        if !is_mimic && self.mem.rb(W_WILD_MON_MOVES + c) == MIMIC {
            return Ok(Cmd::Next);
        }
        // .mimic
        self.do_turn_consume_pp(W_WILD_MON_PP);
        Ok(Cmd::Next)
    }

    /// `BattleCommand_DoTurn.consume_pp`. Returns `b` (1 = out of PP).
    fn do_turn_consume_pp(&mut self, hl: u16) -> u8 {
        let c = if self.is_player_turn() { self.mem.rb(W_CUR_MOVE_NUM) } else { self.mem.rb(W_CUR_ENEMY_MOVE_NUM) };
        let addr = hl.wrapping_add(u16::from(c));
        let v = self.mem.rb(addr);
        if v & PP_MASK != 0 {
            self.mem.wb(addr, v.wrapping_sub(1));
            return 0;
        }
        // .out_of_pp
        let effect = self.battle_var(BATTLE_VARS_MOVE_EFFECT);
        const CONTINUOUS: [u8; 8] = [
            EFFECT_RAZOR_WIND, EFFECT_SKY_ATTACK, EFFECT_SKULL_BASH, EFFECT_SOLARBEAM,
            EFFECT_FLY, EFFECT_ROLLOUT, EFFECT_BIDE, EFFECT_RAMPAGE,
        ];
        self.text(if CONTINUOUS.contains(&effect) { r::HAS_NO_PP_LEFT_TEXT } else { r::NO_PP_LEFT_TEXT });
        1
    }

    /// `CheckMimicUsed` (carry = don't use party PP).
    fn check_mimic_used(&self) -> bool {
        let c = if self.is_player_turn() { self.mem.rb(W_CUR_MOVE_NUM) } else { self.mem.rb(W_CUR_ENEMY_MOVE_NUM) };
        if self.battle_var(BATTLE_VARS_MOVE) == MIMIC {
            return false;
        }
        let moves = self.user_party_attr(MON_MOVES);
        self.mem.rb(moves.wrapping_add(u16::from(c))) == MIMIC
    }

    /// `UserPartyAttr`: address of field `offset` of the user's party mon.
    pub fn user_party_attr(&self, offset: u8) -> u16 {
        if self.is_player_turn() { self.battle_party_attr(offset) } else { self.ot_party_attr(offset) }
    }

    /// `OpponentPartyAttr`.
    pub fn opponent_party_attr(&self, offset: u8) -> u16 {
        if self.is_player_turn() { self.ot_party_attr(offset) } else { self.battle_party_attr(offset) }
    }

    /// `BattlePartyAttr`.
    pub fn battle_party_attr(&self, offset: u8) -> u16 {
        W_PARTY_MONS + u16::from(offset) + u16::from(self.mem.rb(W_CUR_BATTLE_MON)) * super::battle::PARTY_LEN
    }

    /// `OTPartyAttr`.
    pub fn ot_party_attr(&self, offset: u8) -> u16 {
        W_OT_PARTY_MON1_SPECIES + u16::from(offset) + u16::from(self.mem.rb(W_CUR_OT_MON)) * super::battle::PARTY_LEN
    }

    // --- critical hits, STAB, type matchups, variation --------------------------------

    /// `BattleCommand_Critical`.
    pub fn battle_command_critical(&mut self) -> Flow<Cmd> {
        self.mem.wb(W_CRITICAL_HIT, 0);
        if self.battle_var(BATTLE_VARS_MOVE_POWER) == 0 {
            return Ok(Cmd::Next);
        }
        let (item, species) = if self.is_player_turn() {
            (self.mem.rb(W_BATTLE_MON_ITEM), self.mem.rb(W_BATTLE_MON_SPECIES))
        } else {
            (self.mem.rb(W_ENEMY_MON_ITEM), self.mem.rb(W_ENEMY_MON_SPECIES))
        };
        let mut c: u8 = 0;
        let tally = if species == CHANSEY && item == LUCKY_PUNCH {
            c = 2;
            true
        } else if species == FARFETCH_D && item == STICK {
            c = 2;
            true
        } else {
            false
        };
        if !tally {
            // .FocusEnergy
            if self.battle_var_bit(BATTLE_VARS_SUBSTATUS4, SUBSTATUS_FOCUS_ENERGY) {
                c += 1;
            }
            // .CheckCritical
            let anim = self.battle_var(BATTLE_VARS_MOVE_ANIM);
            if rom_in_array(r::CRITICAL_HIT_MOVES, 1, anim).is_some() {
                c += 2;
            }
            // .ScopeLens
            let (_, effect, _) = self.get_user_item();
            if effect == HELD_CRITICAL_UP {
                c += 1;
            }
        }
        // .Tally
        let chance = rom::byte(r::CRITICAL_HIT_CHANCES + usize::from(c));
        if self.battle_random("BattleCommand_Critical") < chance {
            self.mem.wb(W_CRITICAL_HIT, 1);
        }
        Ok(Cmd::Next)
    }

    /// `BattleCommand_Stab`: weather, badge boosts, STAB, type effectiveness.
    pub fn battle_command_stab(&mut self) -> Flow<Cmd> {
        self.stab_regs();
        Ok(Cmd::Next)
    }

    /// `BattleCommand_Stab`'s body. Returns the registers `b`, `c`, `d`, `e`
    /// it leaves behind (`None`: it returned early and touched none); only
    /// `BattleCommand_Present` in a colosseum battle depends on them.
    pub fn stab_regs(&mut self) -> Option<[u8; 4]> {
        if self.battle_var(BATTLE_VARS_MOVE_ANIM) == STRUGGLE {
            return None;
        }
        let (user, target) = if self.is_player_turn() {
            (W_BATTLE_MON_TYPE1, W_ENEMY_MON_TYPE1)
        } else {
            (W_ENEMY_MON_TYPE1, W_BATTLE_MON_TYPE1)
        };
        let (b, mut c) = (self.mem.rb(user), self.mem.rb(user + 1));
        let (d, e) = (self.mem.rb(target), self.mem.rb(target + 1));
        let mtype = self.battle_var(BATTLE_VARS_MOVE_TYPE);
        self.mem.wb(W_CUR_TYPE, mtype);
        self.do_weather_modifiers();
        self.do_badge_type_boosts();
        let cur = self.mem.rb(W_CUR_TYPE);
        if cur == b || cur == c {
            // .stab: damage += damage / 2
            let dmg = self.mem.rw(W_CUR_DAMAGE);
            c = (dmg >> 1) as u8;
            let dmg = dmg.wrapping_add(dmg >> 1);
            self.mem.ww(W_CUR_DAMAGE, dmg);
            self.mem.set_bit(W_TYPE_MODIFIER, STAB_DAMAGE_F);
        }
        // .SkipStab
        let mtype = self.battle_var(BATTLE_VARS_MOVE_TYPE);
        let mut hl = r::TYPE_MATCHUPS;
        loop {
            let a = rom::byte(hl);
            hl += 1;
            if a == 0xff {
                break;
            }
            if a == 0xfe {
                if self.battle_var_bit(BATTLE_VARS_SUBSTATUS1_OPP, SUBSTATUS_IDENTIFIED) {
                    break;
                }
                continue;
            }
            if a == mtype {
                let def = rom::byte(hl);
                if def == d || def == e {
                    // .GotMatchup
                    let stab = self.mem.rb(W_TYPE_MODIFIER) & STAB_DAMAGE;
                    let mut eff = rom::byte(hl + 1);
                    if eff == 0 {
                        self.mem.wb(W_ATTACK_MISSED, 1);
                        eff = 0;
                    }
                    self.mem.wb(H_MULTIPLIER, eff);
                    self.mem.wb(W_TYPE_MODIFIER, eff.wrapping_add(stab));
                    self.mem.wb(H_MULTIPLICAND, 0);
                    let dmg = self.mem.rw(W_CUR_DAMAGE);
                    self.mem.wb(H_MULTIPLICAND + 1, (dmg >> 8) as u8);
                    self.mem.wb(H_MULTIPLICAND + 2, dmg as u8);
                    self.multiply();
                    let nonzero = self.mem.rb(H_PRODUCT + 1) | self.mem.rb(H_PRODUCT + 2) | self.mem.rb(H_PRODUCT + 3);
                    if nonzero != 0 {
                        self.mem.wb(H_DIVISOR, 10);
                        self.divide(4);
                        if self.mem.rb(H_QUOTIENT + 2) | self.mem.rb(H_QUOTIENT + 3) == 0 {
                            self.mem.wb(H_MULTIPLICAND + 2, 1);
                        }
                    }
                    // .ok
                    let hi = self.mem.rb(H_MULTIPLICAND + 1);
                    let lo = self.mem.rb(H_MULTIPLICAND + 2);
                    self.mem.wb(W_CUR_DAMAGE, hi);
                    self.mem.wb(W_CUR_DAMAGE + 1, lo);
                }
            }
            // .SkipType
            hl += 2;
        }
        // .end
        self.battle_check_type_matchup();
        let m = self.mem.rb(W_TYPE_MATCHUP);
        let v = (self.mem.rb(W_TYPE_MODIFIER) & STAB_DAMAGE) | m;
        self.mem.wb(W_TYPE_MODIFIER, v);
        Some([m, c, d, e])
    }

    /// `BattleCheckTypeMatchup`.
    pub fn battle_check_type_matchup(&mut self) {
        let hl = if self.is_player_turn() { W_ENEMY_MON_TYPE1 } else { W_BATTLE_MON_TYPE1 };
        self.check_type_matchup(hl);
    }

    /// `CheckTypeMatchup`: effectiveness of the user's move type against the
    /// types at `hl`, into `wTypeMatchup`.
    pub fn check_type_matchup(&mut self, hl: u16) {
        let d = self.battle_var(BATTLE_VARS_MOVE_TYPE);
        let (b, c) = (self.mem.rb(hl), self.mem.rb(hl + 1));
        self.mem.wb(W_TYPE_MATCHUP, EFFECTIVE);
        let mut p = r::TYPE_MATCHUPS;
        loop {
            let a = rom::byte(p);
            p += 1;
            if a == 0xff {
                return;
            }
            if a == 0xfe {
                if self.battle_var_bit(BATTLE_VARS_SUBSTATUS1_OPP, SUBSTATUS_IDENTIFIED) {
                    return;
                }
                continue;
            }
            if a != d {
                p += 2;
                continue;
            }
            let def = rom::byte(p);
            p += 1;
            if def != b && def != c {
                p += 1;
                continue;
            }
            // .Yup
            self.mem.wb(H_DIVIDEND, 0);
            self.mem.wb(H_MULTIPLICAND, 0);
            self.mem.wb(H_MULTIPLICAND + 1, 0);
            self.mem.wb(H_MULTIPLICAND + 2, rom::byte(p));
            p += 1;
            let m = self.mem.rb(W_TYPE_MATCHUP);
            self.mem.wb(H_MULTIPLIER, m);
            self.multiply();
            self.mem.wb(H_DIVISOR, 10);
            self.divide(4);
            let q = self.mem.rb(H_QUOTIENT + 3);
            self.mem.wb(W_TYPE_MATCHUP, q);
        }
    }

    /// `BattleCommand_ResetTypeMatchup`.
    pub fn battle_command_reset_type_matchup(&mut self) -> Flow<Cmd> {
        self.battle_check_type_matchup();
        if self.mem.rb(W_TYPE_MATCHUP) == 0 {
            self.reset_damage();
            self.mem.wb(W_TYPE_MODIFIER, 0);
            self.mem.wb(W_ATTACK_MISSED, 1);
        } else {
            self.mem.wb(W_TYPE_MATCHUP, EFFECTIVE);
        }
        Ok(Cmd::Next)
    }

    /// `BattleCommand_DamageVariation`.
    pub fn battle_command_damage_variation(&mut self) -> Flow<Cmd> {
        let hi = self.mem.rb(W_CUR_DAMAGE);
        let lo = self.mem.rb(W_CUR_DAMAGE + 1);
        if hi == 0 && lo < 2 {
            return Ok(Cmd::Next);
        }
        self.mem.wb(H_MULTIPLICAND, 0);
        self.mem.wb(H_MULTIPLICAND + 1, hi);
        self.mem.wb(H_MULTIPLICAND + 2, lo);
        let m = loop {
            let r = self.battle_random("BattleCommand_DamageVariation").rotate_right(1);
            if r >= percent(85) + 1 {
                break r;
            }
        };
        self.mem.wb(H_MULTIPLIER, m);
        self.multiply();
        self.mem.wb(H_DIVISOR, percent(100));
        self.divide(4);
        let q2 = self.mem.rb(H_QUOTIENT + 2);
        let q3 = self.mem.rb(H_QUOTIENT + 3);
        self.mem.wb(W_CUR_DAMAGE, q2);
        self.mem.wb(W_CUR_DAMAGE + 1, q3);
        Ok(Cmd::Next)
    }
}

/// `IsInArray` over a ROM table of `stride`-byte entries ending in `-1`.
pub fn rom_in_array(mut table: usize, stride: usize, value: u8) -> Option<u8> {
    let mut b = 0u8;
    loop {
        let a = rom::byte(table);
        if a == 0xff {
            return None;
        }
        if a == value {
            return Some(b);
        }
        b = b.wrapping_add(1);
        table += stride;
    }
}
