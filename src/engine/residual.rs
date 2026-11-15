//! Port of the between-turn effects, the HP routines and the held-item
//! healing routines (`engine/battle/core.asm`, `engine/pokemon/health.asm`
//! `AnimateHPBar`, `engine/battle/anim_hp_bar.asm`). See docs/engine.md.
//!
//! Graphics, sound and frame delays are dropped. The animations' writes to
//! battle WRAM are kept: `wFXAnimID`, `wBattleAfterAnim`, the HP buffers and
//! the `wCurHPAnim*` variables the HP bar animation works in.
//!
//! Dropped on purpose: `UpdateBattleHuds` / `RefreshBattleHuds` (HUD redraw,
//! which also leaves `wTempMon*`, the HP palettes and `wLowHealthAlarm`
//! behind) and `PrintNum` inside `HPBarAnim_UpdateHPRemaining` (it only
//! touches `hPrintNumBuffer` and the tilemap).

#![allow(unused_imports)]

use super::battle::{Engine, Flow, PARTY_LEN, PP_MASK, percent};
use super::rom;
use super::rsym as r;
use super::sym::*;
use crate::defs::*;

/// `HeldStatusHealingEffects`: (held effect, status mask).
const HELD_STATUS_HEALING_EFFECTS: [(u8, u8); 6] = [
    (HELD_HEAL_POISON, 1 << PSN),
    (HELD_HEAL_FREEZE, 1 << FRZ),
    (HELD_HEAL_BURN, 1 << BRN),
    (HELD_HEAL_SLEEP, SLP_MASK),
    (HELD_HEAL_PARALYZE, 1 << PAR),
    (HELD_HEAL_STATUS, ALL_STATUS),
];

/// `"Your@"` in the game's charmap.
const YOUR: [u8; 5] = [0x98, 0xae, 0xb4, 0xb1, 0x50];
/// `"Enemy@"` in the game's charmap.
const ENEMY: [u8; 6] = [0x84, 0xad, 0xa4, 0xac, 0xb8, 0x50];

impl Engine {
    // --- small helpers -------------------------------------------------------

    /// `hSerialConnectionStatus == USING_EXTERNAL_CLOCK`.
    fn using_external_clock(&self) -> bool {
        self.mem.rb(H_SERIAL_CONNECTION_STATUS) == USING_EXTERNAL_CLOCK
    }

    /// The shared opening of `HandlePerishSong`, `HandleWrap`, `HandleLeftovers`,
    /// `HandleMysteryberry` and `HandleFutureSight`: run `.do_it` for the
    /// player then the enemy (or the other way round on the external clock).
    fn do_for_both_sides(&mut self, do_it: fn(&mut Engine)) {
        if self.using_external_clock() {
            self.set_enemy_turn();
            do_it(self);
            self.set_player_turn();
            do_it(self);
        } else {
            self.set_player_turn();
            do_it(self);
            self.set_enemy_turn();
            do_it(self);
        }
    }

    /// `GetPartyLocation`: address of party struct `n` starting at `base`.
    pub fn get_party_location(&self, base: u16, n: u8) -> u16 {
        base.wrapping_add(u16::from(n).wrapping_mul(PARTY_LEN))
    }

    /// `UpdateBattleHuds`: HUD redraw only (see the module docs), so nothing.
    pub fn update_battle_huds(&mut self) {}

    /// `Call_PlayBattleAnim`: the animation itself is dropped, `wFXAnimID` is kept.
    fn call_play_battle_anim(&mut self, anim: u16) {
        self.mem.wb(W_FX_ANIM_ID, anim as u8);
        self.mem.wb(W_FX_ANIM_ID + 1, (anim >> 8) as u8);
    }

    /// `Call_PlayBattleAnim_OnlyIfVisible`.
    pub fn call_play_battle_anim_only_if_visible(&mut self, anim: u16) {
        let sub3 = self.battle_var(BATTLE_VARS_SUBSTATUS3);
        if sub3 & (1 << SUBSTATUS_FLYING | 1 << SUBSTATUS_UNDERGROUND) != 0 {
            return;
        }
        self.call_play_battle_anim(anim);
    }

    /// `GetItemHeldEffect` for item `item`: (effect `b`, parameter `c`).
    /// (For item 0 the original leaves `c` alone; 0 here.)
    fn item_effect_and_param(&self, item: u8) -> (u8, u8) {
        if item == 0 {
            return (0, 0);
        }
        let entry = r::ITEM_ATTRIBUTES + usize::from(item - 1) * usize::from(ITEMATTR_STRUCT_LENGTH);
        (rom::byte(entry + usize::from(ITEMATTR_EFFECT)), rom::byte(entry + usize::from(ITEMATTR_PARAM)))
    }

    // --- ResidualDamage ------------------------------------------------------

    /// `ResidualDamage`: poison/burn, Leech Seed, Nightmare and Curse damage to
    /// the user. Returns z: the user fainted before or as a result.
    pub fn residual_damage(&mut self) -> bool {
        if self.has_user_fainted() {
            return true;
        }
        let status = self.battle_var(BATTLE_VARS_STATUS) & (1 << PSN | 1 << BRN);
        if status != 0 {
            let text = if status & 1 << BRN == 0 { r::HURT_BY_POISON_TEXT } else { r::HURT_BY_BURN_TEXT };
            self.text(text);
            self.mem.wb(W_BATTLE_AFTER_ANIM, 0);
            let anim = if status & 1 << BRN == 0 { ANIM_PSN } else { ANIM_BRN };
            self.call_play_battle_anim_only_if_visible(anim);
            let mut bc = self.get_eighth_max_hp();
            let toxic = if self.is_player_turn() { W_PLAYER_TOXIC_COUNT } else { W_ENEMY_TOXIC_COUNT };
            if self.battle_var_bit(BATTLE_VARS_SUBSTATUS5, SUBSTATUS_TOXIC) {
                bc = self.get_sixteenth_max_hp();
                let n = self.mem.rb(toxic).wrapping_add(1);
                self.mem.wb(toxic, n);
                // .add: do-while, so a count that wrapped to 0 adds 256 times.
                let mut hl = 0u16;
                let mut a = n;
                loop {
                    hl = hl.wrapping_add(bc);
                    a = a.wrapping_sub(1);
                    if a == 0 {
                        break;
                    }
                }
                bc = hl;
            }
            self.subtract_hp_from_user(bc);
        }
        // .did_psn_brn
        if !self.has_user_fainted() {
            if self.battle_var_bit(BATTLE_VARS_SUBSTATUS4, SUBSTATUS_LEECH_SEED) {
                self.switch_turn();
                self.mem.wb(W_BATTLE_AFTER_ANIM, 0);
                let sub3 = self.battle_var(BATTLE_VARS_SUBSTATUS3_OPP);
                if sub3 & (1 << SUBSTATUS_FLYING | 1 << SUBSTATUS_UNDERGROUND) == 0 {
                    self.call_play_battle_anim_only_if_visible(ANIM_SAP);
                }
                self.switch_turn();
                let bc = self.get_eighth_max_hp();
                // RestoreHP heals by the bc SubtractHP left (the mon's old HP if it hit 0).
                let bc = self.subtract_hp_from_user(bc);
                self.mem.wb(H_BG_MAP_MODE, 1);
                self.restore_hp(bc);
                self.text(r::LEECH_SEED_SAPS_TEXT);
            }
            // .not_seeded
            if !self.has_user_fainted() {
                if self.battle_var_bit(BATTLE_VARS_SUBSTATUS1, SUBSTATUS_NIGHTMARE) {
                    self.mem.wb(W_BATTLE_AFTER_ANIM, 0);
                    self.call_play_battle_anim_only_if_visible(ANIM_IN_NIGHTMARE);
                    let bc = self.get_quarter_max_hp();
                    self.subtract_hp_from_user(bc);
                    self.text(r::HAS_A_NIGHTMARE_TEXT);
                }
                // .not_nightmare
                if !self.has_user_fainted() {
                    if self.battle_var_bit(BATTLE_VARS_SUBSTATUS1, SUBSTATUS_CURSE) {
                        self.mem.wb(W_BATTLE_AFTER_ANIM, 0);
                        self.call_play_battle_anim_only_if_visible(ANIM_IN_NIGHTMARE);
                        let bc = self.get_quarter_max_hp();
                        self.subtract_hp_from_user(bc);
                        self.text(r::HURT_BY_CURSE_TEXT);
                    }
                    // .not_cursed
                    let hp = if self.is_player_turn() { W_BATTLE_MON_HP } else { W_ENEMY_MON_HP };
                    if self.mem.rw(hp) != 0 {
                        return false;
                    }
                }
            }
        }
        // .fainted: RefreshBattleHuds, DelayFrames
        true
    }

    // --- HandlePerishSong ----------------------------------------------------

    /// `HandlePerishSong`.
    pub fn handle_perish_song(&mut self) {
        self.do_for_both_sides(Engine::handle_perish_song_do_it);
    }

    /// `HandlePerishSong.do_it`.
    fn handle_perish_song_do_it(&mut self) {
        let count = if self.is_player_turn() { W_PLAYER_PERISH_COUNT } else { W_ENEMY_PERISH_COUNT };
        if !self.battle_var_bit(BATTLE_VARS_SUBSTATUS1, SUBSTATUS_PERISH) {
            return;
        }
        let n = self.mem.rb(count).wrapping_sub(1);
        self.mem.wb(count, n);
        self.mem.wb(W_TEXT_DECIMAL_BYTE, n);
        self.text(r::PERISH_COUNT_TEXT);
        if n != 0 {
            return;
        }
        self.res_battle_var_bit(BATTLE_VARS_SUBSTATUS1, SUBSTATUS_PERISH);
        if self.is_player_turn() {
            self.mem.ww(W_BATTLE_MON_HP, 0);
            let cur = self.mem.rb(W_CUR_BATTLE_MON);
            let hl = self.get_party_location(W_PARTY_MON1_HP, cur);
            self.mem.ww(hl, 0);
        } else {
            // .kill_enemy
            self.mem.ww(W_ENEMY_MON_HP, 0);
            if self.battle_mode() == WILD_BATTLE {
                return;
            }
            let cur = self.mem.rb(W_CUR_OT_MON);
            let hl = self.get_party_location(W_OT_PARTY_MON1_HP, cur);
            self.mem.ww(hl, 0);
        }
    }

    // --- HandleWrap ----------------------------------------------------------

    /// `HandleWrap`.
    pub fn handle_wrap(&mut self) {
        self.do_for_both_sides(Engine::handle_wrap_do_it);
    }

    /// `HandleWrap.do_it`.
    fn handle_wrap_do_it(&mut self) {
        let (count, move_addr) = if self.is_player_turn() {
            (W_PLAYER_WRAP_COUNT, W_PLAYER_TRAPPING_MOVE)
        } else {
            (W_ENEMY_WRAP_COUNT, W_ENEMY_TRAPPING_MOVE)
        };
        if self.mem.rb(count) == 0 {
            return;
        }
        if self.battle_var_bit(BATTLE_VARS_SUBSTATUS4, SUBSTATUS_SUBSTITUTE) {
            return;
        }
        let mv = self.mem.rb(move_addr);
        self.mem.wb(W_NAMED_OBJECT_INDEX, mv);
        self.mem.wb(W_FX_ANIM_ID, mv);
        self.get_move_name();
        let n = self.mem.rb(count).wrapping_sub(1);
        self.mem.wb(count, n);
        if n == 0 {
            // .release_from_bounds
            self.text(r::BATTLE_TEXT_USER_WAS_RELEASED_FROM_STRING_BUFFER1);
            return;
        }
        let sub3 = self.battle_var(BATTLE_VARS_SUBSTATUS3);
        if sub3 & (1 << SUBSTATUS_FLYING | 1 << SUBSTATUS_UNDERGROUND) == 0 {
            self.switch_turn();
            self.mem.wb(W_BATTLE_AFTER_ANIM, 0);
            self.mem.wb(W_FX_ANIM_ID + 1, 0);
            // predef PlayBattleAnim
            self.switch_turn();
        }
        // .skip_anim
        let bc = self.get_sixteenth_max_hp();
        self.subtract_hp_from_user(bc);
        self.text(r::BATTLE_TEXT_USERS_HURT_BY_STRING_BUFFER1);
    }

    // --- HandleLeftovers -----------------------------------------------------

    /// `HandleLeftovers`.
    pub fn handle_leftovers(&mut self) {
        self.do_for_both_sides(Engine::handle_leftovers_do_it);
    }

    /// `HandleLeftovers.do_it`. Leaves the turn switched when it restores HP.
    fn handle_leftovers_do_it(&mut self) {
        let (item_addr, effect, _) = self.get_user_item();
        let item = self.mem.rb(item_addr);
        self.mem.wb(W_NAMED_OBJECT_INDEX, item);
        self.get_item_name();
        if effect != HELD_LEFTOVERS {
            return;
        }
        let hp = if self.is_player_turn() { W_BATTLE_MON_HP } else { W_ENEMY_MON_HP };
        // Don't restore if we're already at max HP
        if self.mem.rw(hp) == self.mem.rw(hp + 2) {
            return;
        }
        let bc = self.get_sixteenth_max_hp();
        self.switch_turn();
        self.restore_hp(bc);
        self.text(r::BATTLE_TEXT_TARGET_RECOVERED_WITH_ITEM);
    }

    // --- HandleMysteryberry --------------------------------------------------

    /// `HandleMysteryberry`.
    pub fn handle_mysteryberry(&mut self) {
        self.do_for_both_sides(Engine::handle_mysteryberry_do_it);
    }

    /// `HandleMysteryberry.do_it`.
    fn handle_mysteryberry_do_it(&mut self) {
        let (_, effect, _) = self.get_user_item();
        if effect != HELD_RESTORE_PP {
            return;
        }
        let cur = self.mem.rb(W_CUR_BATTLE_MON);
        let mut de = self.get_party_location(W_PARTY_MON1_PP, cur);
        let mut hl = self.get_party_location(W_PARTY_MON1_MOVES, cur);
        if !self.is_player_turn() {
            de = W_WILD_MON_PP;
            hl = W_WILD_MON_MOVES;
            if self.battle_mode() != WILD_BATTLE {
                let cur = self.mem.rb(W_CUR_OT_MON);
                de = self.get_party_location(W_OT_PARTY_MON1_PP, cur);
                hl = self.get_party_location(W_OT_PARTY_MON1_MOVES, cur);
            }
        }
        // .wild
        let mut c = 0u8;
        loop {
            if self.mem.rb(hl) == 0 {
                return; // .quit
            }
            if self.mem.rb(de) & PP_MASK == 0 {
                break; // .restore
            }
            hl += 1;
            de += 1;
            c += 1;
            if c == NUM_MOVES {
                return; // .quit
            }
        }
        // .restore
        let mv = self.mem.rb(hl);
        // lousy hack
        let b = if mv == SKETCH { 1 } else { 5 };
        let pp = self.mem.rb(de).wrapping_add(b);
        self.mem.wb(de, pp);
        self.mem.wb(W_TEMP_BYTE_VALUE, mv);
        let (moves, pps) = if self.is_player_turn() {
            (W_BATTLE_MON_MOVES, W_BATTLE_MON_PP)
        } else {
            (W_ENEMY_MON_MOVES, W_ENEMY_MON_PP)
        };
        let de = pps + u16::from(c);
        let hl = moves + u16::from(c);
        if self.mem.rb(W_TEMP_BYTE_VALUE) == self.mem.rb(hl) {
            let sub5 = if self.is_player_turn() {
                self.mem.rb(W_PLAYER_SUB_STATUS5)
            } else {
                self.mem.rb(W_ENEMY_SUB_STATUS5)
            };
            if sub5 & 1 << SUBSTATUS_TRANSFORMED == 0 {
                let pp = self.mem.rb(de).wrapping_add(b);
                self.mem.wb(de, pp);
            }
        }
        // .skip_checks
        let (item_addr, _, _) = self.get_user_item();
        let item = self.mem.rb(item_addr);
        self.mem.wb(W_NAMED_OBJECT_INDEX, item);
        self.mem.wb(item_addr, 0);
        let (mut hl, _) = self.get_partymon_item();
        let mut consume = true;
        if !self.is_player_turn() {
            if self.battle_mode() == WILD_BATTLE {
                consume = false; // .skip_consumption
            } else {
                (hl, _) = self.get_ot_partymon_item();
            }
        }
        if consume {
            self.mem.wb(hl, 0);
        }
        // .skip_consumption
        self.get_item_name();
        self.switch_turn();
        self.item_recovery_anim();
        self.switch_turn();
        self.text(r::BATTLE_TEXT_USER_RECOVERED_PP_USING);
    }

    // --- HandleFutureSight ---------------------------------------------------

    /// `HandleFutureSight`.
    pub fn handle_future_sight(&mut self) {
        self.do_for_both_sides(Engine::handle_future_sight_do_it);
    }

    /// `HandleFutureSight.do_it`.
    fn handle_future_sight_do_it(&mut self) {
        let count = if self.is_player_turn() { W_PLAYER_FUTURE_SIGHT_COUNT } else { W_ENEMY_FUTURE_SIGHT_COUNT };
        let n = self.mem.rb(count);
        if n == 0 {
            return;
        }
        let n = n - 1;
        self.mem.wb(count, n);
        if n != 1 {
            return;
        }
        self.text(r::BATTLE_TEXT_TARGET_WAS_HIT_BY_FUTURE_SIGHT);
        let addr = self.battle_var_addr(BATTLE_VARS_MOVE);
        let old_move = self.mem.rb(addr);
        self.mem.wb(addr, FUTURE_SIGHT);
        self.update_move_data();
        self.mem.wb(W_ATTACK_MISSED, 0);
        self.mem.wb(W_ALREADY_DISOBEYED, 0);
        self.mem.wb(W_TYPE_MODIFIER, EFFECTIVE);
        self.do_move().expect("DoMove inside HandleFutureSight never needs a decision");
        self.mem.wb(W_CUR_DAMAGE, 0);
        self.mem.wb(W_CUR_DAMAGE + 1, 0);
        let addr = self.battle_var_addr(BATTLE_VARS_MOVE);
        self.mem.wb(addr, old_move);
        self.update_battle_mon_in_party();
        self.update_enemy_mon_in_party();
    }

    // --- HandleDefrost -------------------------------------------------------

    /// `HandleDefrost`.
    pub fn handle_defrost(&mut self) {
        if self.using_external_clock() {
            self.handle_defrost_enemy();
            self.handle_defrost_player();
        } else {
            self.handle_defrost_player();
            self.handle_defrost_enemy();
        }
    }

    /// `HandleDefrost.do_player_turn`.
    fn handle_defrost_player(&mut self) {
        if !self.mem.bit(W_BATTLE_MON_STATUS, FRZ) {
            return;
        }
        if self.mem.rb(W_PLAYER_JUST_GOT_FROZEN) != 0 {
            return;
        }
        if self.battle_random("HandleDefrost") >= percent(10) {
            return;
        }
        self.mem.wb(W_BATTLE_MON_STATUS, 0);
        let cur = self.mem.rb(W_CUR_BATTLE_MON);
        let hl = self.get_party_location(W_PARTY_MON1_STATUS, cur);
        self.mem.wb(hl, 0);
        self.update_battle_huds();
        self.set_enemy_turn();
        self.text(r::DEFROSTED_OPPONENT_TEXT);
    }

    /// `HandleDefrost.do_enemy_turn`.
    fn handle_defrost_enemy(&mut self) {
        if !self.mem.bit(W_ENEMY_MON_STATUS, FRZ) {
            return;
        }
        if self.mem.rb(W_ENEMY_JUST_GOT_FROZEN) != 0 {
            return;
        }
        if self.battle_random("HandleDefrost") >= percent(10) {
            return;
        }
        self.mem.wb(W_ENEMY_MON_STATUS, 0);
        if self.battle_mode() != WILD_BATTLE {
            let cur = self.mem.rb(W_CUR_OT_MON);
            let hl = self.get_party_location(W_OT_PARTY_MON1_STATUS, cur);
            self.mem.wb(hl, 0);
        }
        // .wild
        self.update_battle_huds();
        self.set_player_turn();
        self.text(r::DEFROSTED_OPPONENT_TEXT);
    }

    // --- HandleSafeguard -----------------------------------------------------

    /// `HandleSafeguard`.
    pub fn handle_safeguard(&mut self) {
        if self.using_external_clock() {
            self.handle_safeguard_enemy();
            self.handle_safeguard_player();
        } else {
            self.handle_safeguard_player();
            self.handle_safeguard_enemy();
        }
    }

    /// `HandleSafeguard.CheckPlayer`.
    fn handle_safeguard_player(&mut self) {
        let screens = self.mem.rb(W_PLAYER_SCREENS);
        if screens & 1 << SCREENS_SAFEGUARD == 0 {
            return;
        }
        let n = self.mem.rb(W_PLAYER_SAFEGUARD_COUNT).wrapping_sub(1);
        self.mem.wb(W_PLAYER_SAFEGUARD_COUNT, n);
        if n != 0 {
            return;
        }
        self.mem.wb(W_PLAYER_SCREENS, screens & !(1 << SCREENS_SAFEGUARD));
        // .print
        self.mem.wb(H_BATTLE_TURN, 0);
        self.text(r::BATTLE_TEXT_SAFEGUARD_FADED);
    }

    /// `HandleSafeguard.CheckEnemy`.
    fn handle_safeguard_enemy(&mut self) {
        let screens = self.mem.rb(W_ENEMY_SCREENS);
        if screens & 1 << SCREENS_SAFEGUARD == 0 {
            return;
        }
        let n = self.mem.rb(W_ENEMY_SAFEGUARD_COUNT).wrapping_sub(1);
        self.mem.wb(W_ENEMY_SAFEGUARD_COUNT, n);
        if n != 0 {
            return;
        }
        self.mem.wb(W_ENEMY_SCREENS, screens & !(1 << SCREENS_SAFEGUARD));
        // .print
        self.mem.wb(H_BATTLE_TURN, 1);
        self.text(r::BATTLE_TEXT_SAFEGUARD_FADED);
    }

    // --- HandleScreens -------------------------------------------------------

    /// `HandleScreens`.
    pub fn handle_screens(&mut self) {
        if self.using_external_clock() {
            self.handle_screens_enemy();
            self.handle_screens_player();
        } else {
            self.handle_screens_player();
            self.handle_screens_enemy();
        }
    }

    /// `HandleScreens.CheckPlayer`.
    fn handle_screens_player(&mut self) {
        self.set_player_turn();
        for (i, &b) in YOUR.iter().enumerate() {
            self.mem.wb(W_STRING_BUFFER1 + i as u16, b);
        }
        self.tick_screens(W_PLAYER_SCREENS, W_PLAYER_LIGHT_SCREEN_COUNT);
    }

    /// `HandleScreens.CheckEnemy`.
    fn handle_screens_enemy(&mut self) {
        self.set_enemy_turn();
        for (i, &b) in ENEMY.iter().enumerate() {
            self.mem.wb(W_STRING_BUFFER1 + i as u16, b);
        }
        self.tick_screens(W_ENEMY_SCREENS, W_ENEMY_LIGHT_SCREEN_COUNT);
    }

    /// `HandleScreens.TickScreens` (with `.LightScreenTick` / `.ReflectTick`).
    fn tick_screens(&mut self, screens: u16, light_screen_count: u16) {
        if self.mem.bit(screens, SCREENS_LIGHT_SCREEN) {
            let n = self.mem.rb(light_screen_count).wrapping_sub(1);
            self.mem.wb(light_screen_count, n);
            if n == 0 {
                self.mem.res_bit(screens, SCREENS_LIGHT_SCREEN);
                self.text(r::BATTLE_TEXT_MONS_LIGHT_SCREEN_FELL);
            }
        }
        if self.mem.bit(screens, SCREENS_REFLECT) {
            let reflect_count = light_screen_count + 1;
            let n = self.mem.rb(reflect_count).wrapping_sub(1);
            self.mem.wb(reflect_count, n);
            if n == 0 {
                self.mem.res_bit(screens, SCREENS_REFLECT);
                self.text(r::BATTLE_TEXT_MONS_REFLECT_FADED);
            }
        }
    }

    // --- HandleWeather -------------------------------------------------------

    /// `HandleWeather`.
    pub fn handle_weather(&mut self) {
        let weather = self.mem.rb(W_BATTLE_WEATHER);
        if weather == WEATHER_NONE {
            return;
        }
        let n = self.mem.rb(W_WEATHER_COUNT).wrapping_sub(1);
        self.mem.wb(W_WEATHER_COUNT, n);
        if n == 0 {
            // .ended
            let text = match weather {
                WEATHER_RAIN => Some(r::BATTLE_TEXT_THE_RAIN_STOPPED),
                WEATHER_SUN => Some(r::BATTLE_TEXT_THE_SUNLIGHT_FADED),
                WEATHER_SANDSTORM => Some(r::BATTLE_TEXT_THE_SANDSTORM_SUBSIDED),
                _ => None,
            };
            if let Some(text) = text {
                self.text(text);
            }
            self.mem.wb(W_BATTLE_WEATHER, WEATHER_NONE);
            return;
        }
        let text = match weather {
            WEATHER_RAIN => Some(r::BATTLE_TEXT_RAIN_CONTINUES_TO_FALL),
            WEATHER_SUN => Some(r::BATTLE_TEXT_THE_SUNLIGHT_IS_STRONG),
            WEATHER_SANDSTORM => Some(r::BATTLE_TEXT_THE_SANDSTORM_RAGES),
            _ => None,
        };
        if let Some(text) = text {
            self.text(text);
        }
        if self.mem.rb(W_BATTLE_WEATHER) != WEATHER_SANDSTORM {
            return;
        }
        if self.using_external_clock() {
            self.set_enemy_turn();
            self.sandstorm_damage();
            self.set_player_turn();
            self.sandstorm_damage();
        } else {
            self.set_player_turn();
            self.sandstorm_damage();
            self.set_enemy_turn();
            self.sandstorm_damage();
        }
    }

    /// `HandleWeather.SandstormDamage`.
    fn sandstorm_damage(&mut self) {
        if self.battle_var_bit(BATTLE_VARS_SUBSTATUS3, SUBSTATUS_UNDERGROUND) {
            return;
        }
        let types = if self.is_player_turn() { W_BATTLE_MON_TYPE1 } else { W_ENEMY_MON_TYPE1 };
        for t in [self.mem.rb(types), self.mem.rb(types + 1)] {
            if t == ROCK || t == GROUND || t == STEEL {
                return;
            }
        }
        self.switch_turn();
        self.mem.wb(W_BATTLE_AFTER_ANIM, 0);
        self.call_play_battle_anim(ANIM_IN_SANDSTORM);
        self.switch_turn();
        let bc = self.get_eighth_max_hp();
        self.subtract_hp_from_user(bc);
        self.text(r::SANDSTORM_HITS_TEXT);
    }

    // --- HP routines ---------------------------------------------------------

    /// `SubtractHPFromTarget`. Returns `bc` as `SubtractHP` leaves it.
    pub fn subtract_hp_from_target(&mut self, bc: u16) -> u16 {
        let bc = self.subtract_hp(bc);
        self.update_hp_bar();
        bc
    }

    /// `SubtractHPFromUser`. Returns `bc` as `SubtractHP` leaves it.
    pub fn subtract_hp_from_user(&mut self, bc: u16) -> u16 {
        let bc = self.subtract_hp(bc);
        self.update_hp_bar_battle_huds();
        bc
    }

    /// `SubtractHP`: subtract `bc` from the user's HP, flooring at 0.
    /// Returns `bc`: unchanged, or the HP the mon had when the subtraction
    /// went below zero (the damage actually done).
    pub fn subtract_hp(&mut self, bc: u16) -> u16 {
        let hp = if self.is_player_turn() { W_BATTLE_MON_HP } else { W_ENEMY_MON_HP };
        let old = self.mem.rw(hp);
        let (new, borrow) = old.overflowing_sub(bc);
        self.mem.ww_le(W_HP_BUFFER2, old);
        self.mem.ww_le(W_HP_BUFFER3, new);
        self.mem.ww(hp, new);
        if !borrow {
            return bc;
        }
        self.mem.ww(hp, 0);
        self.mem.ww_le(W_HP_BUFFER3, 0);
        old
    }

    /// `GetSixteenthMaxHP`: `bc`, at least 1 (in `c`).
    pub fn get_sixteenth_max_hp(&mut self) -> u16 {
        let bc = self.get_quarter_max_hp();
        let (b, mut c) = ((bc >> 8) as u8, bc as u8);
        c >>= 2;
        if c == 0 {
            c = 1;
        }
        u16::from(b) << 8 | u16::from(c)
    }

    /// `GetEighthMaxHP`: `bc`, at least 1 (in `c`).
    pub fn get_eighth_max_hp(&mut self) -> u16 {
        let bc = self.get_quarter_max_hp();
        let (b, mut c) = ((bc >> 8) as u8, bc as u8);
        c >>= 1;
        if c == 0 {
            c = 1;
        }
        u16::from(b) << 8 | u16::from(c)
    }

    /// `GetQuarterMaxHP`: `bc`, at least 1 (in `c`).
    pub fn get_quarter_max_hp(&mut self) -> u16 {
        let bc = self.get_max_hp() >> 2;
        let (b, mut c) = ((bc >> 8) as u8, bc as u8);
        if c == 0 {
            c = 1;
        }
        u16::from(b) << 8 | u16::from(c)
    }

    /// `GetHalfMaxHP`: `bc`, at least 1.
    pub fn get_half_max_hp(&mut self) -> u16 {
        let bc = self.get_max_hp() >> 1;
        if bc == 0 { 1 } else { bc }
    }

    /// `GetMaxHP`: the user's max HP in `bc`, and in `wHPBuffer1` (little endian).
    pub fn get_max_hp(&mut self) -> u16 {
        let max = if self.is_player_turn() { W_BATTLE_MON_MAX_HP } else { W_ENEMY_MON_MAX_HP };
        let bc = self.mem.rw(max);
        self.mem.ww_le(W_HP_BUFFER1, bc);
        bc
    }

    /// `CheckUserHasEnoughHP`: returns carry, `bc < the user's current HP`.
    pub fn check_user_has_enough_hp(&self, bc: u16) -> bool {
        let hp = if self.is_player_turn() { W_BATTLE_MON_HP } else { W_ENEMY_MON_HP };
        bc < self.mem.rw(hp)
    }

    /// `RestoreHP`: heal the mon of the side that is *not* taking its turn by
    /// `bc`, capped at its max HP. Returns `bc` as the original leaves it
    /// (the healed mon's max HP).
    pub fn restore_hp(&mut self, bc: u16) -> u16 {
        let max_addr = if self.is_player_turn() { W_ENEMY_MON_MAX_HP } else { W_BATTLE_MON_MAX_HP };
        let hp_addr = max_addr - 2;
        let max = self.mem.rw(max_addr);
        self.mem.ww_le(W_HP_BUFFER1, max);
        let old = self.mem.rw(hp_addr);
        self.mem.ww_le(W_HP_BUFFER2, old);
        let new = old.wrapping_add(bc);
        self.mem.ww(hp_addr, new);
        self.mem.ww_le(W_HP_BUFFER3, new);
        // sub / sbc: carry (HP below max) skips the cap.
        if new >= max {
            self.mem.ww(hp_addr, max);
            self.mem.ww_le(W_HP_BUFFER3, max);
        }
        // .overflow
        self.switch_turn();
        self.update_hp_bar_battle_huds();
        self.switch_turn();
        max
    }

    /// `UpdateHPBarBattleHuds`.
    pub fn update_hp_bar_battle_huds(&mut self) {
        self.update_hp_bar();
        self.update_battle_huds();
    }

    /// `UpdateHPBar`: `wWhichHPBar` is 1 for the player's turn, 0 for the
    /// enemy's; then the (memory side of the) HP bar animation.
    pub fn update_hp_bar(&mut self) {
        let which = u8::from(self.is_player_turn());
        self.mem.wb(W_WHICH_HP_BAR, which);
        self.animate_hp_bar();
    }

    // --- AnimateHPBar (memory effects only) ------------------------------------

    /// `GetHPPal`: palette for an HP bar `e` pixels long.
    fn get_hp_pal(e: u8) -> u8 {
        if e >= (HP_BAR_LENGTH_PX * 50 / 100) as u8 {
            HP_GREEN
        } else if e >= (HP_BAR_LENGTH_PX * 21 / 100) as u8 {
            HP_YELLOW
        } else {
            HP_RED
        }
    }

    /// `ComputeHPBarPixels`: `e = bc * HP_BAR_LENGTH_PX / de` (at least 1 unless
    /// `bc` is 0), through the real Multiply/Divide buffers.
    fn compute_hp_bar_pixels(&mut self, bc: u16, de: u16) -> u8 {
        if bc == 0 {
            return 0;
        }
        let (d, mut e) = ((de >> 8) as u8, de as u8);
        self.mem.wb(H_MULTIPLICAND, 0);
        self.mem.wb(H_MULTIPLICAND + 1, (bc >> 8) as u8);
        self.mem.wb(H_MULTIPLICAND + 2, bc as u8);
        self.mem.wb(H_MULTIPLIER, HP_BAR_LENGTH_PX as u8);
        self.multiply();
        // We need de to be under 256 because hDivisor is only 1 byte.
        if d != 0 {
            e = (de >> 2) as u8;
            let p = u16::from(self.mem.rb(H_PRODUCT + 2)) << 8 | u16::from(self.mem.rb(H_PRODUCT + 3));
            let p = p >> 2;
            self.mem.wb(H_DIVIDEND + 3, p as u8);
            self.mem.wb(H_DIVIDEND + 2, (p >> 8) as u8);
        }
        self.mem.wb(H_DIVISOR, e);
        self.divide(4);
        let e = self.mem.rb(H_QUOTIENT + 3);
        if e != 0 { e } else { 1 }
    }

    /// `AnimateHPBar` / `_AnimateHPBar`: the HP bar animation's effect on
    /// memory (`wCurHPAnim*`, `wCurHPBarPixels`, ...); nothing is drawn.
    pub fn animate_hp_bar(&mut self) {
        if let Some(events) = self.events.as_mut() {
            use crate::bootstrap::events::{Event, HpBarTarget};
            let target = match self.mem.rb(W_WHICH_HP_BAR) {
                0 => HpBarTarget::Enemy,
                1 => HpBarTarget::Player,
                _ => HpBarTarget::Party(self.mem.rb(W_CUR_PARTY_MON)),
            };
            events.push(Event::HpBar {
                target,
                from: self.mem.rw_le(W_CUR_HP_ANIM_OLD_HP),
                to: self.mem.rw_le(W_CUR_HP_ANIM_NEW_HP),
                max: self.mem.rw_le(W_CUR_HP_ANIM_MAX_HP),
            });
        }
        let more_than_48 = self.mem.rb(W_CUR_HP_ANIM_MAX_HP + 1) != 0
            || i32::from(self.mem.rb(W_CUR_HP_ANIM_MAX_HP)) >= HP_BAR_LENGTH_PX;
        let step = self.hp_bar_compute_pixels();
        if !more_than_48 {
            // .ShortAnimLoop
            loop {
                // ShortAnim_UpdateVariables
                let done = if self.mem.rb(W_NEW_HP_BAR_PIXELS) == self.mem.rb(W_CUR_HP_BAR_PIXELS) {
                    true
                } else {
                    let cur = self.mem.rb(W_CUR_HP_BAR_PIXELS).wrapping_add(step as u8);
                    self.mem.wb(W_CUR_HP_BAR_PIXELS, cur);
                    self.short_hp_bar_calc_pixel_frame();
                    false
                };
                // ShortHPBarAnim_UpdateTiles
                self.hp_bar_anim_update_hp_remaining();
                let e = self.mem.rb(W_CUR_HP_BAR_PIXELS);
                self.hp_bar_anim_palette_update(e);
                if done {
                    return;
                }
            }
        }
        // .LongAnimLoop
        loop {
            // LongAnim_UpdateVariables
            let finished = loop {
                let old = self.mem.rw_le(W_CUR_HP_ANIM_OLD_HP);
                let new = self.mem.rw_le(W_CUR_HP_ANIM_NEW_HP);
                if old == new {
                    break true;
                }
                let next = old.wrapping_add(step as u16);
                self.mem.ww_le(W_CUR_HP_ANIM_OLD_HP, next);
                let max = self.mem.rw_le(W_CUR_HP_ANIM_MAX_HP);
                // BUG: HP bar animation is slow for high HP. `pop de` before
                // `ld a, e` drops the pixels just computed: the comparison
                // (and wCurHPBarPixels) gets the low byte of the HP before
                // this step, so nearly every step redraws.
                self.compute_hp_bar_pixels(next, max);
                let e = old as u8;
                if e == self.mem.rb(W_CUR_HP_BAR_PIXELS) {
                    continue;
                }
                self.mem.wb(W_CUR_HP_BAR_PIXELS, e);
                break false;
            };
            if finished {
                return;
            }
            // LongHPBarAnim_UpdateTiles
            self.hp_bar_anim_update_hp_remaining();
            let old = self.mem.rw_le(W_CUR_HP_ANIM_OLD_HP);
            let max = self.mem.rw_le(W_CUR_HP_ANIM_MAX_HP);
            let e = self.compute_hp_bar_pixels(old, max);
            self.hp_bar_anim_palette_update(e);
        }
    }

    /// `_AnimateHPBar.ComputePixels`. Returns `bc`, the animation step (1 or -1).
    fn hp_bar_compute_pixels(&mut self) -> i16 {
        let max = self.mem.rw_le(W_CUR_HP_ANIM_MAX_HP);
        let old = self.mem.rw_le(W_CUR_HP_ANIM_OLD_HP);
        let e = self.compute_hp_bar_pixels(old, max);
        self.mem.wb(W_CUR_HP_BAR_PIXELS, e);

        let new = self.mem.rw_le(W_CUR_HP_ANIM_NEW_HP);
        let e = self.compute_hp_bar_pixels(new, max);
        self.mem.wb(W_NEW_HP_BAR_PIXELS, e);

        let diff = new.wrapping_sub(old);
        let (mut d, mut e) = ((diff >> 8) as u8, diff as u8);
        let step;
        if new >= old {
            self.mem.wb(W_CUR_HP_ANIM_LOW_HP, old as u8);
            self.mem.wb(W_CUR_HP_ANIM_HIGH_HP, new as u8);
            step = 1;
        } else {
            self.mem.wb(W_CUR_HP_ANIM_HIGH_HP, old as u8);
            self.mem.wb(W_CUR_HP_ANIM_LOW_HP, new as u8);
            // Negate, without carrying into the high byte.
            e = (e ^ 0xff).wrapping_add(1);
            d ^= 0xff;
            step = -1;
        }
        self.mem.wb(W_CUR_HP_ANIM_DELTA_HP, d);
        self.mem.wb(W_CUR_HP_ANIM_DELTA_HP + 1, e);
        step
    }

    /// `ShortHPBar_CalcPixelFrame`.
    fn short_hp_bar_calc_pixel_frame(&mut self) {
        let c = self.mem.rb(W_CUR_HP_ANIM_MAX_HP);
        let pixels = self.mem.rb(W_CUR_HP_BAR_PIXELS);
        if i32::from(pixels) >= HP_BAR_LENGTH_PX {
            // .return_max
            self.mem.wb(W_CUR_HP_ANIM_OLD_HP, c);
            return;
        }
        if pixels == 0 {
            // .return_zero
            self.mem.wb(W_CUR_HP_ANIM_OLD_HP, 0);
            return;
        }
        // AddNTimes
        let mut hl = u16::from(c).wrapping_mul(u16::from(pixels));
        let mut b = 0u8;
        loop {
            let (nhl, borrow) = hl.overflowing_sub(HP_BAR_LENGTH_PX as u16);
            hl = nhl;
            if borrow {
                break;
            }
            b += 1;
        }
        // .done
        hl = hl.wrapping_add(0x80);
        if hl >= HP_BAR_LENGTH_PX as u16 {
            b += 1;
        }
        // .no_carry: clamp to [wCurHPAnimLowHP, wCurHPAnimHighHP] (low bytes);
        // .finish stores whichever is in `a`.
        let (low, high) = (self.mem.rb(W_CUR_HP_ANIM_LOW_HP), self.mem.rb(W_CUR_HP_ANIM_HIGH_HP));
        let a = if low >= b {
            low
        } else if high < b {
            high
        } else {
            b
        };
        self.mem.wb(W_CUR_HP_ANIM_OLD_HP, a);
    }

    /// `HPBarAnim_UpdateHPRemaining` (the part that is not drawing).
    fn hp_bar_anim_update_hp_remaining(&mut self) {
        if self.mem.rb(W_WHICH_HP_BAR) == 0 {
            return;
        }
        let old = self.mem.rb(W_CUR_HP_ANIM_OLD_HP);
        let old_hi = self.mem.rb(W_CUR_HP_ANIM_OLD_HP + 1);
        self.mem.wb(W_STRING_BUFFER2 + 1, old);
        self.mem.wb(W_STRING_BUFFER2, old_hi);
    }

    /// `HPBarAnim_PaletteUpdate` for a bar `e` pixels long (Game Boy Color only).
    fn hp_bar_anim_palette_update(&mut self, e: u8) {
        if self.mem.rb(H_CGB) == 0 {
            return;
        }
        self.mem.wb(W_CUR_HP_ANIM_PAL, Self::get_hp_pal(e));
    }

    // --- held-item healing -------------------------------------------------------

    /// `HandleHealingItems`.
    pub fn handle_healing_items(&mut self) {
        if self.using_external_clock() {
            self.set_enemy_turn();
            self.handle_healing_items_for_turn();
            self.set_player_turn();
            self.handle_healing_items_for_turn();
        } else {
            self.set_player_turn();
            self.handle_healing_items_for_turn();
            self.set_enemy_turn();
            self.handle_healing_items_for_turn();
        }
    }

    fn handle_healing_items_for_turn(&mut self) {
        self.handle_hp_healing_item();
        self.use_held_status_healing_item();
        self.use_confusion_healing_item();
    }

    /// `HandleHPHealingItem`: the opponent's HP-restoring berry.
    pub fn handle_hp_healing_item(&mut self) {
        let (_, effect, param) = self.get_opponent_item();
        if effect != HELD_BERRY {
            return;
        }
        let (hp, max) = if self.is_player_turn() {
            (W_ENEMY_MON_HP, W_ENEMY_MON_MAX_HP)
        } else {
            (W_BATTLE_MON_HP, W_BATTLE_MON_MAX_HP)
        };
        // If, and only if, Pokemon's HP is less than half max, use the item.
        let current = self.mem.rw(hp);
        self.mem.ww_le(W_HP_BUFFER2, current);
        let doubled = current.wrapping_mul(2);
        if doubled >= self.mem.rw(max) {
            return;
        }
        // .less
        self.item_recovery_anim();
        let max_hp = self.mem.rw(max);
        self.mem.ww_le(W_HP_BUFFER1, max_hp);
        let new = current.wrapping_add(u16::from(param));
        self.mem.ww_le(W_HP_BUFFER3, new);
        if max_hp < new {
            self.mem.ww_le(W_HP_BUFFER3, max_hp);
        }
        // .okay
        let new = self.mem.rw_le(W_HP_BUFFER3);
        self.mem.ww(hp, new);
        let turn = self.turn();
        self.mem.wb(W_WHICH_HP_BAR, turn);
        self.animate_hp_bar();
        self.use_opponent_item();
    }

    /// `UseOpponentItem`.
    pub fn use_opponent_item(&mut self) {
        // RefreshBattleHuds: HUD redraw
        let (item_addr, _, _) = self.get_opponent_item();
        let item = self.mem.rb(item_addr);
        self.mem.wb(W_NAMED_OBJECT_INDEX, item);
        self.get_item_name();
        self.consume_held_item();
        self.text(r::RECOVERED_USING_TEXT);
    }

    /// `ItemRecoveryAnim`: the animation is dropped, its `wFXAnimID` and
    /// `wBattleAfterAnim` writes are kept.
    pub fn item_recovery_anim(&mut self) {
        // EmptyBattleTextbox
        self.mem.wb(W_FX_ANIM_ID, RECOVER);
        self.switch_turn();
        self.mem.wb(W_BATTLE_AFTER_ANIM, 0);
        self.mem.wb(W_FX_ANIM_ID + 1, 0);
        // predef PlayBattleAnim
        self.switch_turn();
    }

    /// `UseHeldStatusHealingItem`. Returns nz: the item was used.
    pub fn use_held_status_healing_item(&mut self) -> bool {
        let (_, effect, _) = self.get_opponent_item();
        let Some(&(_, mask)) = HELD_STATUS_HEALING_EFFECTS.iter().find(|&&(e, _)| e == effect) else {
            return false;
        };
        let status = self.battle_var_addr(BATTLE_VARS_STATUS_OPP);
        if self.mem.rb(status) & mask == 0 {
            return false;
        }
        self.mem.wb(status, 0);
        self.update_opponent_in_party();
        self.res_battle_var_bit(BATTLE_VARS_SUBSTATUS5_OPP, SUBSTATUS_TOXIC);
        self.res_battle_var_bit(BATTLE_VARS_SUBSTATUS1_OPP, SUBSTATUS_NIGHTMARE);
        if mask == ALL_STATUS {
            self.res_battle_var_bit(BATTLE_VARS_SUBSTATUS3_OPP, SUBSTATUS_CONFUSED);
        }
        // .skip_confuse
        let player_turn = self.is_player_turn();
        self.switch_turn();
        if player_turn {
            self.calc_enemy_stats();
        } else {
            self.calc_player_stats();
        }
        self.switch_turn();
        self.item_recovery_anim();
        self.use_opponent_item();
        true
    }

    /// `UseConfusionHealingItem`.
    pub fn use_confusion_healing_item(&mut self) {
        if !self.battle_var_bit(BATTLE_VARS_SUBSTATUS3_OPP, SUBSTATUS_CONFUSED) {
            return;
        }
        let (item_addr, effect, _) = self.get_opponent_item();
        if effect != HELD_HEAL_CONFUSION && effect != HELD_HEAL_STATUS {
            return;
        }
        // .heal_status
        let item = self.mem.rb(item_addr);
        self.mem.wb(W_NAMED_OBJECT_INDEX, item);
        self.res_battle_var_bit(BATTLE_VARS_SUBSTATUS3_OPP, SUBSTATUS_CONFUSED);
        self.get_item_name();
        self.item_recovery_anim();
        self.text(r::BATTLE_TEXT_ITEM_HEALED_CONFUSION);
        if self.is_player_turn() {
            let (hl, bc) = self.get_ot_partymon_item();
            self.mem.wb(bc, 0);
            if self.battle_mode() == WILD_BATTLE {
                return;
            }
            self.mem.wb(hl, 0);
        } else {
            // .do_partymon
            let (hl, bc) = self.get_partymon_item();
            self.mem.wb(bc, 0);
            self.mem.wb(hl, 0);
        }
    }

    /// `HandleStatBoostingHeldItems` (the effects handled here are not used in-game).
    pub fn handle_stat_boosting_held_items(&mut self) {
        if self.using_external_clock() {
            self.stat_boosting_held_item_enemy();
            self.stat_boosting_held_item_player();
        } else {
            self.stat_boosting_held_item_player();
            self.stat_boosting_held_item_enemy();
        }
    }

    /// `HandleStatBoostingHeldItems.DoPlayer`.
    fn stat_boosting_held_item_player(&mut self) {
        let (hl, bc) = self.get_partymon_item();
        self.stat_boosting_held_item(hl, bc, 0);
    }

    /// `HandleStatBoostingHeldItems.DoEnemy`.
    fn stat_boosting_held_item_enemy(&mut self) {
        let (hl, bc) = self.get_ot_partymon_item();
        self.stat_boosting_held_item(hl, bc, 1);
    }

    /// `HandleStatBoostingHeldItems.HandleItem`: `de` = party item, `bc` = battle mon item.
    fn stat_boosting_held_item(&mut self, de: u16, bc: u16, turn: u8) {
        self.mem.wb(H_BATTLE_TURN, turn);
        let item = self.mem.rb(bc);
        let (effect, _) = self.item_effect_and_param(item);
        // HeldStatUpItems
        let stat_up: fn(&mut Engine) -> Flow<super::commands::Cmd> = match effect {
            HELD_ATTACK_UP => Engine::battle_command_attack_up,
            HELD_DEFENSE_UP => Engine::battle_command_defense_up,
            HELD_SPEED_UP => Engine::battle_command_speed_up,
            HELD_SP_ATTACK_UP => Engine::battle_command_special_attack_up,
            HELD_SP_DEFENSE_UP => Engine::battle_command_special_defense_up,
            HELD_ACCURACY_UP => Engine::battle_command_accuracy_up,
            HELD_EVASION_UP => Engine::battle_command_evasion_up,
            _ => return, // .finish
        };
        let item = self.mem.rb(bc);
        self.mem.wb(W_NAMED_OBJECT_INDEX, item);
        stat_up(self).expect("a stat-up command never needs a decision");
        if self.mem.rb(W_FAILED_MESSAGE) != 0 {
            return;
        }
        self.mem.wb(bc, 0);
        self.mem.wb(de, 0);
        self.get_item_name();
        self.text(r::BATTLE_TEXT_USERS_STRING_BUFFER1_ACTIVATED);
        self.battle_command_stat_up_message().expect("BattleCommand_StatUpMessage never needs a decision");
    }

    /// `GetPartymonItem`: (`hl` = the party mon's item, `bc` = `wBattleMonItem`).
    pub fn get_partymon_item(&self) -> (u16, u16) {
        let cur = self.mem.rb(W_CUR_BATTLE_MON);
        (self.get_party_location(W_PARTY_MON1_ITEM, cur), W_BATTLE_MON_ITEM)
    }

    /// `GetOTPartymonItem`: (`hl` = the OT party mon's item, `bc` = `wEnemyMonItem`).
    pub fn get_ot_partymon_item(&self) -> (u16, u16) {
        let cur = self.mem.rb(W_CUR_OT_MON);
        (self.get_party_location(W_OT_PARTY_MON1_ITEM, cur), W_ENEMY_MON_ITEM)
    }
}
