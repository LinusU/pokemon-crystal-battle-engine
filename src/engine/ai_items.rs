//! `engine/battle/ai/items.asm`: deciding between switching, using an item
//! and attacking.
//!
//! The item routines return carry = "don't use" (`.DontUse` is `scf`,
//! `.Use` is `and a`) as a `bool`, `true` = don't use. Sound, HUD redraws
//! and HP bar drawing are dropped (see docs/engine.md); their memory effects
//! are kept.

#![allow(unused_imports)]

use super::battle::{Engine, Flow, PARTY_LEN, percent};
use super::rom;
use super::rsym as r;
use super::sym::*;
use crate::defs::*;

impl Engine {
    /// `AI_SwitchOrTryItem` (carry = the enemy switched or used an item).
    pub fn ai_switch_or_try_item(&mut self) -> Flow<bool> {
        if self.battle_mode() == WILD_BATTLE {
            return Ok(false);
        }
        if self.link_mode() != 0 {
            return Ok(false);
        }
        if self.check_enemy_locked_in() {
            return Ok(false);
        }

        if self.mem.bit(W_PLAYER_SUB_STATUS5, SUBSTATUS_CANT_RUN) || self.mem.rb(W_ENEMY_WRAP_COUNT) != 0 {
            return Ok(self.ai_dont_switch());
        }

        // Always load the first trainer class in wTrainerClass for Battle Tower trainers.
        let mut hl = r::TRAINER_CLASS_ATTRIBUTES + usize::from(TRNATTR_AI_ITEM_SWITCH);
        if self.mem.rb(W_IN_BATTLE_TOWER_BATTLE) == 0 {
            let class = self.mem.rb(W_TRAINER_CLASS).wrapping_sub(1);
            hl += usize::from(class) * usize::from(NUM_TRAINER_ATTRIBUTES);
        }
        let flags = self.rom_byte(hl);
        // (SwitchOften, SwitchRarely, SwitchSometimes) thresholds for the $10, $20 and $30 cases:
        // switch if `Random < threshold` for $10 and $20; for $30, don't switch if
        // `Random < threshold`.
        let thresholds = if flags & 1 << SWITCH_OFTEN_F != 0 {
            ("SwitchOften", percent(50) + 1, percent(79) - 1, percent(4))
        } else if flags & 1 << SWITCH_RARELY_F != 0 {
            ("SwitchRarely", percent(8), percent(12), percent(79) - 1)
        } else if flags & 1 << SWITCH_SOMETIMES_F != 0 {
            ("SwitchSometimes", percent(20) - 1, percent(50) + 1, percent(20) - 1)
        } else {
            return Ok(self.ai_dont_switch());
        };
        self.ai_switch_variant(thresholds)
    }

    /// `DontSwitch`: `call AI_TryItem; ret`.
    fn ai_dont_switch(&mut self) -> bool {
        self.ai_try_item()
    }

    /// `SwitchOften` / `SwitchRarely` / `SwitchSometimes`.
    fn ai_switch_variant(&mut self, (name, t10, t20, t30): (&'static str, u8, u8, u8)) -> Flow<bool> {
        self.check_able_to_switch();
        let param = self.mem.rb(W_ENEMY_SWITCH_MON_PARAM);
        let hi = param & 0xf0;
        if hi == 0 {
            return Ok(self.ai_dont_switch());
        }
        let switch = if hi == 0x10 {
            self.random(name) < t10
        } else if hi == 0x20 {
            self.random(name) < t20
        } else {
            // $30
            self.random(name) >= t30
        };
        if !switch {
            return Ok(self.ai_dont_switch());
        }
        // .switch: in register 'a' is the number (1-6) of the mon to switch to
        let a = (self.mem.rb(W_ENEMY_SWITCH_MON_PARAM) & 0xf).wrapping_add(1);
        self.mem.wb(W_ENEMY_SWITCH_MON_INDEX, a);
        self.ai_try_switch()
    }

    /// `AI_TrySwitch`: determine whether the AI can switch based on how many
    /// Pokemon are still alive. If it can switch, it will.
    fn ai_try_switch(&mut self) -> Flow<bool> {
        let mut hl = W_OT_PARTY_MON1_HP;
        let mut d = 0u8;
        for _ in 0..self.mem.rb(W_OT_PARTY_COUNT) {
            if (self.mem.rb(hl) | self.mem.rb(hl + 1)) != 0 {
                d += 1;
            }
            hl += PARTY_LEN;
        }
        if d >= 2 {
            return self.ai_switch();
        }
        Ok(false)
    }

    /// `AI_TryItem` (carry = an item was used).
    pub fn ai_try_item(&mut self) -> bool {
        // Items are not allowed in the Battle Tower.
        if self.mem.rb(W_IN_BATTLE_TOWER_BATTLE) != 0 {
            return false;
        }
        if (self.mem.rb(W_ENEMY_TRAINER_ITEM1) | self.mem.rb(W_ENEMY_TRAINER_ITEM2)) == 0 {
            return false;
        }
        if !self.ai_is_highest_level() {
            return false;
        }

        let class = self.mem.rb(W_TRAINER_CLASS).wrapping_sub(1);
        let bc = r::TRAINER_CLASS_ATTRIBUTES
            + usize::from(TRNATTR_AI_ITEM_SWITCH)
            + usize::from(class) * usize::from(NUM_TRAINER_ATTRIBUTES);
        let flags = self.rom_byte(bc);
        let mut hl = r::AI_ITEMS;
        // BUG: AI might use its base reward value as an item
        let mut de = W_ENEMY_TRAINER_ITEM1;
        loop {
            let item = rom::byte(hl);
            if item == 0xff {
                return false;
            }
            if self.mem.rb(de) != item {
                de += 1;
                if self.mem.rb(de) != item {
                    de -= 1;
                    hl += 3;
                    continue;
                }
            }
            // .has_item
            let dont_use = self.ai_item_handler(item, flags);
            if dont_use {
                hl += 3;
                continue;
            }

            // used item
            self.mem.wb(de, 0);
            self.mem.wb(W_ENEMY_GOES_FIRST, 1);
            self.mem.res_bit(W_ENEMY_SUB_STATUS3, SUBSTATUS_BIDE);
            self.mem.wb(W_ENEMY_FURY_CUTTER_COUNT, 0);
            self.mem.wb(W_ENEMY_PROTECT_COUNT, 0);
            self.mem.wb(W_ENEMY_RAGE_COUNTER, 0);
            self.mem.res_bit(W_ENEMY_SUB_STATUS4, SUBSTATUS_RAGE);
            self.mem.wb(W_LAST_ENEMY_COUNTER_MOVE, 0);
            return true;
        }
    }

    /// `AI_TryItem.IsHighestLevel`: carry if the current mon's level is the
    /// highest in the party.
    fn ai_is_highest_level(&self) -> bool {
        let mut e = 0u8;
        let mut hl = W_OT_PARTY_MON1_LEVEL;
        for _ in 0..self.mem.rb(W_OT_PARTY_COUNT) {
            let a = self.mem.rb(hl);
            if a >= e {
                e = a;
            }
            hl += PARTY_LEN;
        }
        let cur = W_OT_PARTY_MON1_LEVEL + u16::from(self.mem.rb(W_CUR_OT_MON)) * PARTY_LEN;
        self.mem.rb(cur) >= e
    }

    /// The handlers of the `AI_Items` table (`.FullRestore`, `.MaxPotion`, ...).
    /// `flags` is the byte at `bc`. Returns carry (`true` = don't use).
    fn ai_item_handler(&mut self, item: u8, flags: u8) -> bool {
        match item {
            FULL_RESTORE => {
                if self.ai_heal_item(flags) {
                    // .HealItem said don't use (nc = use)
                    if flags & 1 << CONTEXT_USE_F == 0 {
                        return true;
                    }
                    if self.ai_item_status(flags) {
                        return true;
                    }
                }
                // .UseFullRestore
                self.enemy_used_full_restore();
                false
            }
            MAX_POTION => {
                if self.ai_heal_item(flags) {
                    return true;
                }
                self.enemy_used_max_potion();
                false
            }
            HYPER_POTION => {
                if self.ai_heal_item(flags) {
                    return true;
                }
                self.enemy_used_potion(HYPER_POTION, 200);
                false
            }
            SUPER_POTION => {
                if self.ai_heal_item(flags) {
                    return true;
                }
                self.enemy_used_potion(SUPER_POTION, 50);
                false
            }
            POTION => {
                if self.ai_heal_item(flags) {
                    return true;
                }
                self.enemy_used_potion(POTION, 20);
                false
            }
            FULL_HEAL => {
                if self.ai_item_status(flags) {
                    return true;
                }
                self.enemy_used_full_heal();
                false
            }
            X_ACCURACY | GUARD_SPEC | DIRE_HIT | X_ATTACK | X_DEFEND | X_SPEED | X_SPECIAL => {
                if self.ai_x_item(flags) {
                    return true;
                }
                match item {
                    X_ACCURACY => self.enemy_used_x_accuracy(),
                    GUARD_SPEC => self.enemy_used_guard_spec(),
                    DIRE_HIT => self.enemy_used_dire_hit(),
                    X_ATTACK => self.enemy_used_x_item(ATTACK, X_ATTACK),
                    X_DEFEND => self.enemy_used_x_item(DEFENSE, X_DEFEND),
                    X_SPEED => self.enemy_used_x_item(SPEED, X_SPEED),
                    _ => self.enemy_used_x_item(SP_ATTACK, X_SPECIAL),
                }
                false
            }
            _ => unreachable!("not an AI item"),
        }
    }

    /// `AI_Items.Status`: carry (`true`) = don't use.
    fn ai_item_status(&mut self, flags: u8) -> bool {
        if self.mem.rb(W_ENEMY_MON_STATUS) == 0 {
            return true;
        }
        if flags & 1 << CONTEXT_USE_F == 0 {
            if flags & 1 << ALWAYS_USE_F != 0 {
                return false;
            }
            return self.random("AI_Items.Status") >= percent(20) - 1;
        }
        // .StatusCheckContext
        if self.mem.bit(W_ENEMY_SUB_STATUS5, SUBSTATUS_TOXIC)
            && self.mem.rb(W_ENEMY_TOXIC_COUNT) >= 4
            && self.random("AI_Items.Status") < percent(50) + 1
        {
            return false;
        }
        // .FailToxicCheck
        self.mem.rb(W_ENEMY_MON_STATUS) & (1 << FRZ | SLP_MASK) == 0
    }

    /// `AI_Items.HealItem`: carry (`true`) = don't use.
    fn ai_heal_item(&mut self, flags: u8) -> bool {
        if flags & 1 << CONTEXT_USE_F != 0 {
            // .CheckHalfOrQuarterHP
            if self.ai_check_enemy_half_hp() {
                return true;
            }
            if !self.ai_check_enemy_quarter_hp() {
                return false;
            }
            return self.random("AI_Items.HealItem") >= percent(20) - 1;
        }
        if self.ai_check_enemy_half_hp() {
            return true;
        }
        if flags & 1 << UNKNOWN_USE_F != 0 {
            // .CheckQuarterHP
            if self.ai_check_enemy_quarter_hp() {
                return true;
            }
            return self.random("AI_Items.HealItem") < percent(20) - 1;
        }
        if !self.ai_check_enemy_quarter_hp() {
            return false;
        }
        self.random("AI_Items.HealItem") >= percent(50) + 1
    }

    /// `AI_Items.XItem`: carry (`true`) = don't use.
    fn ai_x_item(&mut self, flags: u8) -> bool {
        if self.mem.rb(W_ENEMY_TURNS_TAKEN) != 0 {
            // .notfirstturnout
            if flags & 1 << ALWAYS_USE_F == 0 {
                return true;
            }
            return self.random("AI_Items.XItem") >= percent(20) - 1;
        }
        if flags & 1 << ALWAYS_USE_F != 0 {
            return false;
        }
        if self.random("AI_Items.XItem") < percent(50) + 1 {
            return true;
        }
        if flags & 1 << CONTEXT_USE_F != 0 {
            return false;
        }
        self.random("AI_Items.XItem") < percent(50) + 1
    }

    // --- using the items ---------------------------------------------------------------

    /// `AIUpdateHUD`: the HUD redraw is dropped; returns carry (`true`).
    fn ai_update_hud(&mut self) -> bool {
        self.update_enemy_mon_in_party();
        let v = self.mem.rb(W_ENEMY_ITEM_STATE).wrapping_sub(1);
        self.mem.wb(W_ENEMY_ITEM_STATE, v);
        true
    }

    /// `PrintText_UsedItemOn_AND_AIUpdateHUD`.
    fn ai_print_used_item_on_and_update_hud(&mut self, item: u8) {
        self.mem.wb(W_CUR_ENEMY_ITEM, item);
        self.ai_print_text_used_item_on();
        self.ai_update_hud();
    }

    /// `PrintText_UsedItemOn`.
    fn ai_print_text_used_item_on(&mut self) {
        let item = self.mem.rb(W_CUR_ENEMY_ITEM);
        self.mem.wb(W_NAMED_OBJECT_INDEX, item);
        self.get_item_name();
        self.mem.copy(W_MON_OR_ITEM_NAME_BUFFER, W_STRING_BUFFER1, u16::from(ITEM_NAME_LENGTH));
        self.text(r::ENEMY_USED_ON_TEXT);
    }

    /// `EnemyUsedFullHeal`.
    fn enemy_used_full_heal(&mut self) {
        self.ai_heal_status();
        self.ai_print_used_item_on_and_update_hud(FULL_HEAL);
    }

    /// `AI_HealStatus`.
    // BUG: AI use of Full Heal or Full Restore does not cure Nightmare status
    // BUG: AI use of Full Heal or Full Restore does not cure Attack or Speed drops from burn or paralysis
    fn ai_heal_status(&mut self) {
        let hl = W_OT_PARTY_MON1_STATUS + u16::from(self.mem.rb(W_CUR_OT_MON)) * PARTY_LEN;
        self.mem.wb(hl, 0);
        self.mem.wb(W_ENEMY_MON_STATUS, 0);
        self.mem.res_bit(W_ENEMY_SUB_STATUS5, SUBSTATUS_TOXIC);
    }

    /// `EnemyUsedMaxPotion`.
    fn enemy_used_max_potion(&mut self) {
        self.mem.wb(W_CUR_ENEMY_ITEM, MAX_POTION);
        self.full_restore_continue();
    }

    /// `EnemyUsedFullRestore`.
    // BUG: AI use of Full Heal does not cure confusion status
    fn enemy_used_full_restore(&mut self) {
        self.ai_heal_status();
        self.mem.wb(W_CUR_ENEMY_ITEM, FULL_RESTORE);
        self.mem.res_bit(W_ENEMY_SUB_STATUS3, SUBSTATUS_CONFUSED);
        self.mem.wb(W_ENEMY_CONFUSE_COUNT, 0);
        self.full_restore_continue();
    }

    /// `FullRestoreContinue`.
    fn full_restore_continue(&mut self) {
        let hp_lo = self.mem.rb(W_ENEMY_MON_HP + 1);
        let hp_hi = self.mem.rb(W_ENEMY_MON_HP);
        self.mem.wb(W_CUR_HP_ANIM_OLD_HP, hp_lo);
        self.mem.wb(W_CUR_HP_ANIM_OLD_HP + 1, hp_hi);
        let max_lo = self.mem.rb(W_ENEMY_MON_MAX_HP + 1);
        let max_hi = self.mem.rb(W_ENEMY_MON_MAX_HP);
        self.mem.wb(W_CUR_HP_ANIM_NEW_HP, max_lo);
        self.mem.wb(W_CUR_HP_ANIM_MAX_HP, max_lo);
        self.mem.wb(W_ENEMY_MON_HP + 1, max_lo);
        self.mem.wb(W_CUR_HP_ANIM_NEW_HP + 1, max_hi);
        self.mem.wb(W_CUR_HP_ANIM_MAX_HP + 1, max_hi);
        self.mem.wb(W_ENEMY_MON_HP, max_hi);
        self.enemy_potion_finish();
    }

    /// `EnemyUsedPotion` / `EnemyUsedSuperPotion` / `EnemyUsedHyperPotion` →
    /// `EnemyPotionContinue`.
    fn enemy_used_potion(&mut self, item: u8, b: u8) {
        self.mem.wb(W_CUR_ENEMY_ITEM, item);
        let old_lo = self.mem.rb(W_ENEMY_MON_HP + 1);
        self.mem.wb(W_CUR_HP_ANIM_OLD_HP, old_lo);
        let (new_lo, carry) = old_lo.overflowing_add(b);
        self.mem.wb(W_ENEMY_MON_HP + 1, new_lo);
        self.mem.wb(W_CUR_HP_ANIM_NEW_HP, new_lo);
        let mut hi = self.mem.rb(W_ENEMY_MON_HP);
        self.mem.wb(W_CUR_HP_ANIM_OLD_HP + 1, hi);
        self.mem.wb(W_CUR_HP_ANIM_NEW_HP + 1, hi);
        if carry {
            hi = hi.wrapping_add(1);
            self.mem.wb(W_ENEMY_MON_HP, hi);
            self.mem.wb(W_CUR_HP_ANIM_NEW_HP + 1, hi);
        }
        // .ok
        let max_lo = self.mem.rb(W_ENEMY_MON_MAX_HP + 1);
        let max_hi = self.mem.rb(W_ENEMY_MON_MAX_HP);
        self.mem.wb(W_CUR_HP_ANIM_MAX_HP, max_lo);
        self.mem.wb(W_CUR_HP_ANIM_MAX_HP + 1, max_hi);
        let hp = self.mem.rw(W_ENEMY_MON_HP);
        if self.mem.rw(W_ENEMY_MON_MAX_HP) < hp {
            // The new HP is above the maximum: use the maximum.
            self.mem.wb(W_ENEMY_MON_HP + 1, max_lo);
            self.mem.wb(W_CUR_HP_ANIM_NEW_HP, max_lo);
            self.mem.wb(W_ENEMY_MON_HP, max_hi);
            self.mem.wb(W_CUR_HP_ANIM_NEW_HP + 1, max_hi);
        }
        self.enemy_potion_finish();
    }

    /// `EnemyPotionFinish`.
    fn enemy_potion_finish(&mut self) {
        self.ai_print_text_used_item_on();
        self.mem.wb(W_WHICH_HP_BAR, 0);
        self.animate_hp_bar();
        self.ai_update_hud();
    }

    /// `EnemyUsedXAccuracy`.
    fn enemy_used_x_accuracy(&mut self) {
        self.mem.set_bit(W_ENEMY_SUB_STATUS4, SUBSTATUS_X_ACCURACY);
        self.ai_print_used_item_on_and_update_hud(X_ACCURACY);
    }

    /// `EnemyUsedGuardSpec`.
    fn enemy_used_guard_spec(&mut self) {
        self.mem.set_bit(W_ENEMY_SUB_STATUS4, SUBSTATUS_MIST);
        self.ai_print_used_item_on_and_update_hud(GUARD_SPEC);
    }

    /// `EnemyUsedDireHit`.
    fn enemy_used_dire_hit(&mut self) {
        self.mem.set_bit(W_ENEMY_SUB_STATUS4, SUBSTATUS_FOCUS_ENERGY);
        self.ai_print_used_item_on_and_update_hud(DIRE_HIT);
    }

    /// `EnemyUsedXItem` (`EnemyUsedXAttack`, `EnemyUsedXDefend`,
    /// `EnemyUsedXSpeed`, `EnemyUsedXSpecial`): `b` = stat, `item` = item.
    fn enemy_used_x_item(&mut self, b: u8, item: u8) {
        self.mem.wb(W_CUR_ENEMY_ITEM, item);
        self.ai_print_text_used_item_on();
        self.raise_stat(b);
        self.ai_update_hud();
    }
}
