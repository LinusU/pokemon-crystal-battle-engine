//! `engine/battle/core.asm`: the turn loop, action parsing, move order,
//! residual effects, fainting and switching.
//!
//! Each function is a port of the routine named in its doc comment. Control
//! flow follows the original; graphics, sound and frame delays are dropped
//! (they never affect the battle — see docs/goal.md), everything that touches
//! battle memory is kept.

use super::battle::{Engine, Flow, PARTY_LEN, PP_MASK, percent};
use super::sym::*;
use crate::bootstrap::decision::{PromptSite, Step};
use crate::defs::*;

/// Where [`Engine::battle_turn`] starts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TurnEntry {
    /// The top of `BattleTurn.loop`.
    TurnStart,
    /// `BattleTurn` just before `BattleMenu` — after `AIChooseMove` and
    /// `CheckPlayerLockedIn` (where the emulated engine is at its first
    /// decision point).
    BattleMenu,
}

/// How a turn ended.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TurnEnd {
    /// The battle goes on; next turn starts at the top of the loop.
    Continue,
    /// `BattleTurn` returned (`.quit`): the battle is over.
    Quit,
}

impl Engine {
    /// Panic for a routine that has not been ported yet.
    pub fn unported(&self, routine: &str) -> ! {
        panic!("engine: unported routine {routine}")
    }

    /// One iteration of `BattleTurn.loop`, from `entry`.
    pub fn battle_turn(&mut self, entry: TurnEntry) -> Flow<TurnEnd> {
        if entry == TurnEntry::TurnStart {
            // Stubbed_Increments5_a89a: ret
            if self.check_contest_battle_over() {
                return Ok(TurnEnd::Quit);
            }
            for addr in [
                W_PLAYER_IS_SWITCHING,
                W_ENEMY_IS_SWITCHING,
                W_BATTLE_HAS_JUST_STARTED,
                W_PLAYER_JUST_GOT_FROZEN,
                W_ENEMY_JUST_GOT_FROZEN,
                W_CUR_DAMAGE,
                W_CUR_DAMAGE + 1,
            ] {
                self.mem.wb(addr, 0);
            }
            self.handle_berserk_gene();
            self.update_battle_mon_in_party();
            self.ai_choose_move();
            // IsMobileBattle: never.
            if self.check_player_locked_in() {
                return self.battle_turn_skip_iteration();
            }
        }
        loop {
            // .loop1
            if self.battle_menu()? {
                return Ok(TurnEnd::Quit);
            }
            if self.mem.rb(W_BATTLE_ENDED) != 0 || self.mem.rb(W_FORCED_SWITCH) != 0 {
                return Ok(TurnEnd::Quit);
            }
            // .skip_iteration
            if self.parse_player_action()? {
                continue;
            }
            return self.battle_turn_after_parse();
        }
    }

    /// `BattleTurn.skip_iteration` entered from `CheckPlayerLockedIn`.
    fn battle_turn_skip_iteration(&mut self) -> Flow<TurnEnd> {
        loop {
            if !self.parse_player_action()? {
                return self.battle_turn_after_parse();
            }
            // jr nz, .loop1
            if self.battle_menu()? {
                return Ok(TurnEnd::Quit);
            }
            if self.mem.rb(W_BATTLE_ENDED) != 0 || self.mem.rb(W_FORCED_SWITCH) != 0 {
                return Ok(TurnEnd::Quit);
            }
        }
    }

    /// `BattleTurn` from `call EnemyTriesToFlee` to the end of the loop body.
    fn battle_turn_after_parse(&mut self) -> Flow<TurnEnd> {
        if self.enemy_tries_to_flee() {
            return Ok(TurnEnd::Quit);
        }
        if self.determine_move_order()? {
            self.battle_player_first()?;
        } else {
            self.battle_enemy_first()?;
        }
        // CheckMobileBattleError: never.
        if self.mem.rb(W_FORCED_SWITCH) != 0 || self.mem.rb(W_BATTLE_ENDED) != 0 {
            return Ok(TurnEnd::Quit);
        }
        self.handle_between_turn_effects()?;
        if self.mem.rb(W_BATTLE_ENDED) != 0 {
            return Ok(TurnEnd::Quit);
        }
        Ok(TurnEnd::Continue)
    }

    /// `CheckContestBattleOver`. Returns carry (contest over).
    fn check_contest_battle_over(&mut self) -> bool {
        if self.mem.rb(W_BATTLE_TYPE) != BATTLETYPE_CONTEST {
            return false;
        }
        if self.mem.rb(W_PARK_BALLS_REMAINING) != 0 {
            return false;
        }
        let r = (self.mem.rb(W_BATTLE_RESULT) & BATTLERESULT_BITMASK).wrapping_add(DRAW);
        self.mem.wb(W_BATTLE_RESULT, r);
        true
    }

    /// `HandleBetweenTurnEffects`.
    pub fn handle_between_turn_effects(&mut self) -> Flow {
        // hSerialConnectionStatus is never USING_EXTERNAL_CLOCK in the harness.
        if self.check_faint_player_then_enemy()? {
            return Ok(());
        }
        self.handle_future_sight();
        if self.check_faint_player_then_enemy()? {
            return Ok(());
        }
        self.handle_weather();
        if self.check_faint_player_then_enemy()? {
            return Ok(());
        }
        self.handle_wrap();
        if self.check_faint_player_then_enemy()? {
            return Ok(());
        }
        self.handle_perish_song();
        if self.check_faint_player_then_enemy()? {
            return Ok(());
        }
        // .NoMoreFaintingConditions
        self.handle_leftovers();
        self.handle_mysteryberry();
        self.handle_defrost();
        self.handle_safeguard();
        self.handle_screens();
        self.handle_stat_boosting_held_items();
        self.handle_healing_items();
        self.update_battle_mon_in_party();
        self.handle_encore();
        Ok(())
    }

    /// `CheckFaint_PlayerThenEnemy`. Returns carry (the battle is over).
    fn check_faint_player_then_enemy(&mut self) -> Flow<bool> {
        if self.has_player_fainted() {
            self.handle_player_mon_faint()?;
            if self.mem.rb(W_BATTLE_ENDED) != 0 {
                return Ok(true);
            }
        }
        if self.has_enemy_fainted() {
            self.handle_enemy_mon_faint()?;
            if self.mem.rb(W_BATTLE_ENDED) != 0 {
                return Ok(true);
            }
        }
        Ok(false)
    }

    /// `HandleBerserkGene`.
    fn handle_berserk_gene(&mut self) {
        for side in [0u8, 1] {
            // .player / .enemy
            let (party_item, cur) = if side == 0 {
                self.set_player_turn();
                (W_PARTY_MON1_ITEM, self.mem.rb(W_CUR_BATTLE_MON))
            } else {
                self.set_enemy_turn();
                (W_OT_PARTY_MON1_ITEM, self.mem.rb(W_CUR_OT_MON))
            };
            let (item_addr, _) = self.get_user_item();
            let item = self.mem.rb(item_addr);
            self.mem.wb(W_NAMED_OBJECT_INDEX, item);
            if item != BERSERK_GENE {
                continue;
            }
            self.unported("HandleBerserkGene (holding Berserk Gene)");
            #[allow(unreachable_code)]
            {
                let _ = (party_item, cur);
            }
        }
    }

    /// `EnemyTriesToFlee`. Returns carry (the link partner forfeited).
    fn enemy_tries_to_flee(&mut self) -> bool {
        if self.link_mode() != 0 && self.mem.rb(W_BATTLE_ACTION) == BATTLEACTION_FORFEIT {
            self.unported("WildFled_EnemyFled_LinkBattleCanceled (forfeit)");
        }
        false
    }

    /// `DetermineMoveOrder`. Returns carry (the player goes first).
    fn determine_move_order(&mut self) -> Flow<bool> {
        let use_move = 'use_move: {
            if self.link_mode() == 0 {
                break 'use_move true;
            }
            let action = self.mem.rb(W_BATTLE_ACTION);
            if action == BATTLEACTION_STRUGGLE || action == BATTLEACTION_SKIPTURN {
                break 'use_move true;
            }
            if action < BATTLEACTION_SWITCH1 {
                break 'use_move true;
            }
            false
        };
        if !use_move {
            if self.mem.rb(W_BATTLE_PLAYER_ACTION) != BATTLEPLAYERACTION_SWITCH {
                // .switch: the enemy switches.
                self.ai_switch()?;
                self.set_enemy_turn();
                self.spikes_damage();
                return Ok(false);
            }
            // Both switch: hSerialConnectionStatus is USING_EXTERNAL_CLOCK-free.
            return Ok(self.battle_random("DetermineMoveOrder") < percent(50) + 1);
        }
        // .use_move
        if self.mem.rb(W_BATTLE_PLAYER_ACTION) != BATTLEPLAYERACTION_USEMOVE {
            return Ok(true);
        }
        match self.compare_move_priority() {
            std::cmp::Ordering::Less => return Ok(true), // carry: player first
            std::cmp::Ordering::Greater => return Ok(false),
            std::cmp::Ordering::Equal => {}
        }
        // .equal_priority
        self.set_player_turn();
        let (_, player_effect, player_param) = self.get_user_item_full();
        let (_, enemy_effect, enemy_param) = self.get_opponent_item_full();
        let (d, e) = (player_effect, player_param);
        let (b, c) = (enemy_effect, enemy_param);
        if d == HELD_QUICK_CLAW {
            if b != HELD_QUICK_CLAW {
                if self.battle_random("DetermineMoveOrder") < e {
                    return Ok(true);
                }
            } else {
                // .both_have_quick_claw (not player 2)
                if self.battle_random("DetermineMoveOrder") < c {
                    return Ok(false);
                }
                if self.battle_random("DetermineMoveOrder") < e {
                    return Ok(true);
                }
            }
        } else if b == HELD_QUICK_CLAW {
            // .player_no_quick_claw
            if self.battle_random("DetermineMoveOrder") < c {
                return Ok(false);
            }
        }
        // .speed_check
        let player = self.mem.rw(W_BATTLE_MON_SPEED);
        let enemy = self.mem.rw(W_ENEMY_MON_SPEED);
        if player != enemy {
            return Ok(player > enemy);
        }
        // .speed_tie
        Ok(self.battle_random("DetermineMoveOrder") < percent(50) + 1)
    }

    /// `CheckPlayerLockedIn`. Returns carry (locked in).
    pub fn check_player_locked_in(&mut self) -> bool {
        if self.mem.rb(W_PLAYER_SUB_STATUS4) & 1 << SUBSTATUS_RECHARGE != 0 {
            return true;
        }
        self.mem.res_bit(W_ENEMY_SUB_STATUS3, SUBSTATUS_FLINCHED);
        self.mem.res_bit(W_PLAYER_SUB_STATUS3, SUBSTATUS_FLINCHED);
        if self.mem.rb(W_PLAYER_SUB_STATUS3) & (1 << SUBSTATUS_CHARGED | 1 << SUBSTATUS_RAMPAGE) != 0 {
            return true;
        }
        self.mem.bit(W_PLAYER_SUB_STATUS1, SUBSTATUS_ROLLOUT)
    }

    /// `ParsePlayerAction`. Returns nz (go back to the battle menu).
    fn parse_player_action(&mut self) -> Flow<bool> {
        let encored = 'locked: {
            if self.check_player_locked_in() {
                return self.parse_player_action_locked_in();
            }
            if self.mem.bit(W_PLAYER_SUB_STATUS5, SUBSTATUS_ENCORED) {
                let last = self.mem.rb(W_LAST_PLAYER_MOVE);
                self.mem.wb(W_CUR_PLAYER_MOVE, last);
                break 'locked true;
            }
            false
        };
        if !encored {
            let action = self.mem.rb(W_BATTLE_PLAYER_ACTION);
            if action == BATTLEPLAYERACTION_SWITCH {
                // .reset_rage
                self.reset_player_rage_vars();
                return Ok(false);
            }
            if action != 0 {
                // .reset_bide
                self.mem.res_bit(W_PLAYER_SUB_STATUS3, SUBSTATUS_BIDE);
                return self.parse_player_action_locked_in();
            }
            if self.mem.rb(W_PLAYER_SUB_STATUS3) & 1 << SUBSTATUS_BIDE != 0 {
                return self.parse_player_action_locked_in();
            }
            self.mem.wb(W_MOVE_SELECTION_MENU_TYPE, 0);
            self.mem.wb(W_FX_ANIM_ID, POUND);
            if self.move_selection_screen()? {
                return Ok(true);
            }
        }
        // .encored
        self.set_player_turn();
        self.update_move_data();
        self.mem.wb(W_PLAYER_CHARGING, 0);
        let effect = self.mem.rb(W_PLAYER_MOVE_STRUCT_EFFECT);
        if effect != EFFECT_FURY_CUTTER {
            self.mem.wb(W_PLAYER_FURY_CUTTER_COUNT, 0);
        }
        if effect != EFFECT_RAGE {
            self.mem.res_bit(W_PLAYER_SUB_STATUS4, SUBSTATUS_RAGE);
            self.mem.wb(W_PLAYER_RAGE_COUNTER, 0);
        }
        if effect != EFFECT_PROTECT && effect != EFFECT_ENDURE {
            self.mem.wb(W_PLAYER_PROTECT_COUNT, 0);
        }
        // .continue_protect
        self.parse_enemy_action()?;
        Ok(false)
    }

    /// `ParsePlayerAction.locked_in`.
    fn parse_player_action_locked_in(&mut self) -> Flow<bool> {
        self.mem.wb(W_PLAYER_FURY_CUTTER_COUNT, 0);
        self.mem.wb(W_PLAYER_PROTECT_COUNT, 0);
        self.mem.wb(W_PLAYER_RAGE_COUNTER, 0);
        self.mem.res_bit(W_PLAYER_SUB_STATUS4, SUBSTATUS_RAGE);
        // .continue_protect
        self.parse_enemy_action()?;
        Ok(false)
    }

    /// `ParsePlayerAction.reset_rage`.
    fn reset_player_rage_vars(&mut self) {
        self.mem.wb(W_PLAYER_FURY_CUTTER_COUNT, 0);
        self.mem.wb(W_PLAYER_PROTECT_COUNT, 0);
        self.mem.wb(W_PLAYER_RAGE_COUNTER, 0);
        self.mem.res_bit(W_PLAYER_SUB_STATUS4, SUBSTATUS_RAGE);
    }

    /// `HandleEncore`.
    fn handle_encore(&mut self) {
        // .do_player
        if self.mem.bit(W_PLAYER_SUB_STATUS5, SUBSTATUS_ENCORED) {
            let n = self.mem.rb(W_PLAYER_ENCORE_COUNT).wrapping_sub(1);
            self.mem.wb(W_PLAYER_ENCORE_COUNT, n);
            let pp = self.mem.rb(W_BATTLE_MON_PP + u16::from(self.mem.rb(W_CUR_MOVE_NUM))) & PP_MASK;
            if n == 0 || pp == 0 {
                self.mem.res_bit(W_PLAYER_SUB_STATUS5, SUBSTATUS_ENCORED);
                self.set_enemy_turn();
                self.text(BATTLE_TEXT_TARGETS_ENCORE_ENDED);
            }
        }
        // .do_enemy
        if self.mem.bit(W_ENEMY_SUB_STATUS5, SUBSTATUS_ENCORED) {
            let n = self.mem.rb(W_ENEMY_ENCORE_COUNT).wrapping_sub(1);
            self.mem.wb(W_ENEMY_ENCORE_COUNT, n);
            let pp = self.mem.rb(W_ENEMY_MON_PP + u16::from(self.mem.rb(W_CUR_ENEMY_MOVE_NUM))) & PP_MASK;
            if n == 0 || pp == 0 {
                self.mem.res_bit(W_ENEMY_SUB_STATUS5, SUBSTATUS_ENCORED);
                self.set_player_turn();
                self.text(BATTLE_TEXT_TARGETS_ENCORE_ENDED);
            }
        }
    }

    /// `CompareMovePriority`: player's priority vs the enemy's (`Less` =
    /// player goes first, i.e. carry).
    fn compare_move_priority(&self) -> std::cmp::Ordering {
        let player = self.get_move_priority(self.mem.rb(W_CUR_PLAYER_MOVE));
        let enemy = self.get_move_priority(self.mem.rb(W_CUR_ENEMY_MOVE));
        // `cp b` with a = enemy, b = player: carry when enemy < player.
        player.cmp(&enemy).reverse()
    }

    /// `GetMovePriority`.
    fn get_move_priority(&self, mv: u8) -> u8 {
        if mv == VITAL_THROW {
            return 0;
        }
        let effect = self.get_move_effect(mv);
        let mut p = MOVE_EFFECT_PRIORITIES;
        loop {
            let a = super::rom::byte(p);
            if a == effect {
                return super::rom::byte(p + 1);
            }
            p += 2;
            if a == 0xff {
                return BASE_PRIORITY;
            }
        }
    }

    /// `GetMoveEffect`.
    pub fn get_move_effect(&self, mv: u8) -> u8 {
        super::rom::byte(MOVES + (usize::from(mv) - 1) * usize::from(MOVE_LENGTH) + usize::from(MOVE_EFFECT))
    }

    /// `Battle_EnemyFirst`.
    fn battle_enemy_first(&mut self) -> Flow {
        if self.try_enemy_flee() {
            self.unported("WildFled_EnemyFled_LinkBattleCanceled (enemy fled)");
        }
        self.set_enemy_turn();
        self.mem.wb(W_ENEMY_GOES_FIRST, 1);
        let switched_or_item = self.ai_switch_or_try_item()?;
        if !switched_or_item {
            self.enemy_turn_end_opponent_protect_endure_destiny_bond()?;
            if self.mem.rb(W_FORCED_SWITCH) != 0 {
                return Ok(());
            }
            if self.has_player_fainted() {
                return self.handle_player_mon_faint();
            }
            if self.has_enemy_fainted() {
                return self.handle_enemy_mon_faint();
            }
        }
        // .switch_item
        self.set_enemy_turn();
        if self.residual_damage() {
            return self.handle_enemy_mon_faint();
        }
        self.player_turn_end_opponent_protect_endure_destiny_bond()?;
        if self.mem.rb(W_FORCED_SWITCH) != 0 {
            return Ok(());
        }
        if self.has_enemy_fainted() {
            return self.handle_enemy_mon_faint();
        }
        if self.has_player_fainted() {
            return self.handle_player_mon_faint();
        }
        self.set_player_turn();
        if self.residual_damage() {
            return self.handle_player_mon_faint();
        }
        self.mem.wb(W_BATTLE_PLAYER_ACTION, BATTLEPLAYERACTION_USEMOVE);
        Ok(())
    }

    /// `Battle_PlayerFirst`.
    fn battle_player_first(&mut self) -> Flow {
        self.mem.wb(W_ENEMY_GOES_FIRST, 0);
        self.set_enemy_turn();
        let switched_or_item = self.ai_switch_or_try_item()?;
        self.player_turn_end_opponent_protect_endure_destiny_bond()?;
        if self.mem.rb(W_FORCED_SWITCH) != 0 {
            return Ok(());
        }
        if self.has_enemy_fainted() {
            return self.handle_enemy_mon_faint();
        }
        if self.has_player_fainted() {
            return self.handle_player_mon_faint();
        }
        self.set_player_turn();
        if self.residual_damage() {
            return self.handle_player_mon_faint();
        }
        if !switched_or_item {
            if self.try_enemy_flee() {
                self.unported("WildFled_EnemyFled_LinkBattleCanceled (enemy fled)");
            }
            self.enemy_turn_end_opponent_protect_endure_destiny_bond()?;
            if self.mem.rb(W_FORCED_SWITCH) != 0 {
                return Ok(());
            }
            if self.has_player_fainted() {
                return self.handle_player_mon_faint();
            }
            if self.has_enemy_fainted() {
                return self.handle_enemy_mon_faint();
            }
        }
        // .switched_or_used_item
        self.set_enemy_turn();
        if self.residual_damage() {
            return self.handle_enemy_mon_faint();
        }
        self.mem.wb(W_BATTLE_PLAYER_ACTION, BATTLEPLAYERACTION_USEMOVE);
        Ok(())
    }

    /// `PlayerTurn_EndOpponentProtectEndureDestinyBond`.
    fn player_turn_end_opponent_protect_endure_destiny_bond(&mut self) -> Flow {
        self.set_player_turn();
        self.end_user_destiny_bond();
        self.do_player_turn()?;
        self.end_opponent_protect_endure_destiny_bond();
        Ok(())
    }

    /// `EnemyTurn_EndOpponentProtectEndureDestinyBond`.
    fn enemy_turn_end_opponent_protect_endure_destiny_bond(&mut self) -> Flow {
        self.set_enemy_turn();
        self.end_user_destiny_bond();
        self.do_enemy_turn()?;
        self.end_opponent_protect_endure_destiny_bond();
        Ok(())
    }

    /// `EndOpponentProtectEndureDestinyBond`.
    fn end_opponent_protect_endure_destiny_bond(&mut self) {
        self.res_battle_var_bit(BATTLE_VARS_SUBSTATUS1_OPP, SUBSTATUS_PROTECT);
        self.res_battle_var_bit(BATTLE_VARS_SUBSTATUS1_OPP, SUBSTATUS_ENDURE);
        self.res_battle_var_bit(BATTLE_VARS_SUBSTATUS5_OPP, SUBSTATUS_DESTINY_BOND);
    }

    /// `EndUserDestinyBond`.
    fn end_user_destiny_bond(&mut self) {
        self.res_battle_var_bit(BATTLE_VARS_SUBSTATUS5, SUBSTATUS_DESTINY_BOND);
    }

    /// `HasUserFainted` (z = fainted).
    pub fn has_user_fainted(&self) -> bool {
        if self.is_player_turn() { self.has_player_fainted() } else { self.has_enemy_fainted() }
    }

    /// `HasEnemyFainted`.
    pub fn has_enemy_fainted(&self) -> bool {
        self.mem.rw(W_ENEMY_MON_HP) == 0
    }

    /// `HasPlayerFainted`.
    pub fn has_player_fainted(&self) -> bool {
        self.mem.rw(W_BATTLE_MON_HP) == 0
    }

    /// `ParseEnemyAction`.
    pub fn parse_enemy_action(&mut self) -> Flow {
        if self.mem.rb(W_ENEMY_IS_SWITCHING) != 0 {
            return Ok(());
        }
        let mv = 'finish: {
            if self.link_mode() != 0 {
                if self.mem.rb(W_BATTLE_PLAYER_ACTION) == BATTLEPLAYERACTION_USEMOVE {
                    self.link_battle_send_receive_action()?;
                }
                let action = self.mem.rb(W_BATTLE_ACTION);
                if action == BATTLEACTION_STRUGGLE {
                    break 'finish Some(STRUGGLE);
                }
                if action == BATTLEACTION_SKIPTURN {
                    break 'finish Some(0xff);
                }
                if action >= BATTLEACTION_SWITCH1 {
                    self.reset_vars_for_substatus_rage();
                    return Ok(());
                }
                self.mem.wb(W_CUR_ENEMY_MOVE_NUM, action);
                if self.mem.bit(W_ENEMY_SUB_STATUS1, SUBSTATUS_ROLLOUT) {
                    break 'finish None;
                }
                if self.mem.rb(W_ENEMY_SUB_STATUS3)
                    & (1 << SUBSTATUS_CHARGED | 1 << SUBSTATUS_RAMPAGE | 1 << SUBSTATUS_BIDE)
                    != 0
                {
                    break 'finish None;
                }
                if self.mem.bit(W_ENEMY_SUB_STATUS5, SUBSTATUS_ENCORED) {
                    break 'finish Some(self.mem.rb(W_LAST_ENEMY_MOVE));
                }
                break 'finish Some(self.mem.rb(W_ENEMY_MON_MOVES + u16::from(action)));
            }
            // .not_linked
            if self.mem.bit(W_ENEMY_SUB_STATUS5, SUBSTATUS_ENCORED) {
                break 'finish Some(self.mem.rb(W_LAST_ENEMY_MOVE));
            }
            if self.check_enemy_locked_in() {
                self.reset_vars_for_substatus_rage();
                return Ok(());
            }
            // .continue
            let mut usable = false;
            for i in 0..NUM_MOVES as u16 {
                let mv = self.mem.rb(W_ENEMY_MON_MOVES + i);
                if mv == 0 {
                    break 'finish Some(STRUGGLE);
                }
                if self.mem.rb(W_ENEMY_DISABLED_MOVE) == mv {
                    continue;
                }
                if self.mem.rb(W_ENEMY_MON_PP + i) & PP_MASK != 0 {
                    usable = true;
                    break;
                }
            }
            if !usable {
                break 'finish Some(STRUGGLE);
            }
            // .enough_pp
            if self.battle_mode() != WILD_BATTLE {
                break 'finish None;
            }
            loop {
                // .loop2
                let c = self.battle_random("ParseEnemyAction") & (NUM_MOVES - 1);
                let disabled = (self.mem.rb(W_ENEMY_DISABLE_COUNT) >> 4).wrapping_sub(1);
                if disabled == c {
                    continue;
                }
                let mv = self.mem.rb(W_ENEMY_MON_MOVES + u16::from(c));
                if mv == 0 {
                    continue;
                }
                if self.mem.rb(W_ENEMY_MON_PP + u16::from(c)) & PP_MASK == 0 {
                    continue;
                }
                self.mem.wb(W_CUR_ENEMY_MOVE_NUM, c);
                break 'finish Some(mv);
            }
        };
        if let Some(mv) = mv {
            // .finish
            self.mem.wb(W_CUR_ENEMY_MOVE, mv);
        }
        // .skip_load
        self.set_enemy_turn();
        self.update_move_data();
        if !self.check_enemy_locked_in() {
            self.mem.wb(W_ENEMY_CHARGING, 0);
        }
        let effect = self.mem.rb(W_ENEMY_MOVE_STRUCT_EFFECT);
        if effect != EFFECT_FURY_CUTTER {
            self.mem.wb(W_ENEMY_FURY_CUTTER_COUNT, 0);
        }
        if effect != EFFECT_RAGE {
            self.mem.res_bit(W_ENEMY_SUB_STATUS4, SUBSTATUS_RAGE);
            self.mem.wb(W_ENEMY_RAGE_COUNTER, 0);
        }
        if effect == EFFECT_PROTECT || effect == EFFECT_ENDURE {
            return Ok(());
        }
        self.mem.wb(W_ENEMY_PROTECT_COUNT, 0);
        Ok(())
    }

    /// `ResetVarsForSubstatusRage`.
    fn reset_vars_for_substatus_rage(&mut self) {
        self.mem.wb(W_ENEMY_FURY_CUTTER_COUNT, 0);
        self.mem.wb(W_ENEMY_PROTECT_COUNT, 0);
        self.mem.wb(W_ENEMY_RAGE_COUNTER, 0);
        self.mem.res_bit(W_ENEMY_SUB_STATUS4, SUBSTATUS_RAGE);
    }

    /// `CheckEnemyLockedIn` (nz = locked in).
    pub fn check_enemy_locked_in(&self) -> bool {
        if self.mem.rb(W_ENEMY_SUB_STATUS4) & 1 << SUBSTATUS_RECHARGE != 0 {
            return true;
        }
        if self.mem.rb(W_ENEMY_SUB_STATUS3)
            & (1 << SUBSTATUS_CHARGED | 1 << SUBSTATUS_RAMPAGE | 1 << SUBSTATUS_BIDE)
            != 0
        {
            return true;
        }
        self.mem.bit(W_ENEMY_SUB_STATUS1, SUBSTATUS_ROLLOUT)
    }

    /// `TryEnemyFlee` (carry = the wild mon flees).
    fn try_enemy_flee(&mut self) -> bool {
        if self.battle_mode() != WILD_BATTLE {
            return false;
        }
        self.unported("TryEnemyFlee (wild battle)")
    }

    /// `UpdateBattleMonInParty`: copy `wBattleMonLevel..wBattleMonMaxHP`
    /// into the active party mon.
    pub fn update_battle_mon_in_party(&mut self) {
        let slot = self.mem.rb(W_CUR_BATTLE_MON);
        let dst = W_PARTY_MON1_LEVEL + u16::from(slot) * PARTY_LEN;
        self.mem.copy(dst, W_BATTLE_MON_LEVEL, W_BATTLE_MON_MAX_HP - W_BATTLE_MON_LEVEL);
    }

    /// `UpdateEnemyMonInParty`.
    pub fn update_enemy_mon_in_party(&mut self) {
        if self.battle_mode() == WILD_BATTLE {
            return;
        }
        let slot = self.mem.rb(W_CUR_OT_MON);
        let dst = W_OT_PARTY_MON1_LEVEL + u16::from(slot) * PARTY_LEN;
        self.mem.copy(dst, W_ENEMY_MON_LEVEL, W_ENEMY_MON_MAX_HP - W_ENEMY_MON_LEVEL);
    }

    /// `UpdateUserInParty`.
    pub fn update_user_in_party(&mut self) {
        if self.is_player_turn() { self.update_battle_mon_in_party() } else { self.update_enemy_mon_in_party() }
    }

    /// `UpdateOpponentInParty`.
    pub fn update_opponent_in_party(&mut self) {
        if self.is_player_turn() { self.update_enemy_mon_in_party() } else { self.update_battle_mon_in_party() }
    }

    /// `AIChooseMove`.
    fn ai_choose_move(&mut self) {
        if self.battle_mode() == WILD_BATTLE {
            return;
        }
        if self.link_mode() != 0 {
            return;
        }
        self.unported("AIChooseMove")
    }

    /// `AI_SwitchOrTryItem` (carry = the enemy switched or used an item).
    fn ai_switch_or_try_item(&mut self) -> Flow<bool> {
        if self.link_mode() != 0 {
            return Ok(false);
        }
        self.unported("AI_SwitchOrTryItem")
    }
}

#[allow(dead_code)]
fn _unused(_: Step, _: PromptSite) {}
