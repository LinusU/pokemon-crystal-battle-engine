//! The emulated harness's replacements for the original's menus.
//!
//! The oracle runs the ROM code with `HARNESS-PATCH`es where the original
//! would wait for buttons (see docs/transpiled_code.md): `BattleMenu`,
//! `MoveSelectionScreen` and `LinkBattleSendReceiveAction` take decisions
//! instead. These are ports of those patched routines (`bootstrap::cli`
//! `handle_battle_menu`, the `move_selection_screen` patch and
//! `handle_enemy_action`), so the engine matches the oracle byte for byte.

use super::battle::{Engine, Flow};
use super::rsym as r;
use super::sym::*;
use crate::battle::action::{PromptSite, Step};
use crate::defs::*;

/// `wBattleMenuCursorPosition`.
const W_BATTLE_MENU_CURSOR_POSITION: u16 = 0xd0d2;

impl Engine {
    /// `BattleMenu` (patched: `bootstrap::cli::handle_battle_menu`). Returns
    /// carry (leave the turn loop).
    pub fn battle_menu(&mut self) -> Flow<bool> {
        self.flags.pending_move = None;
        self.flags.pending_struggle = false;
        match self.decide(PromptSite::BattleMenu)? {
            Step::Move { slot } => {
                self.flags.pending_move = Some(slot);
                self.mem.wb(W_BATTLE_PLAYER_ACTION, BATTLEPLAYERACTION_USEMOVE);
                self.mem.wb(W_BATTLE_MENU_CURSOR_POSITION, 1);
                Ok(self.battle_menu_fight())
            }
            Step::Fight => {
                self.flags.pending_struggle = !crate::battle::legal::fight_skips_move_menu(&self.mem);
                self.mem.wb(W_BATTLE_PLAYER_ACTION, BATTLEPLAYERACTION_USEMOVE);
                self.mem.wb(W_BATTLE_MENU_CURSOR_POSITION, 1);
                Ok(self.battle_menu_fight())
            }
            Step::Switch { slot } => {
                // The writes of TryPlayerSwitch, then PlayerSwitch.
                let old = self.mem.rb(W_CUR_BATTLE_MON);
                self.mem.wb(W_LAST_PLAYER_MON, old);
                self.mem.wb(W_BATTLE_PLAYER_ACTION, BATTLEPLAYERACTION_SWITCH);
                self.mem.wb(W_CUR_PARTY_MON, slot);
                self.mem.wb(W_CUR_BATTLE_MON, slot);
                self.mem.wb(W_BATTLE_MENU_CURSOR_POSITION, 2);
                self.player_switch()
            }
            Step::Item { id, target, move_slot } => self.battle_menu_pack(id, target, move_slot),
            Step::Run => {
                self.mem.wb(W_BATTLE_MENU_CURSOR_POSITION, 4);
                self.battle_menu_run()
            }
            step => unreachable!("{step:?} at the battle menu"),
        }
    }

    /// `BattleMenu_Fight`: `xor a; ld [wNumFleeAttempts], a; ... and a; ret`.
    fn battle_menu_fight(&mut self) -> bool {
        self.mem.wb(W_NUM_FLEE_ATTEMPTS, 0);
        false
    }

    /// `BattleMenu_Run`. Returns carry (the battle is over).
    fn battle_menu_run(&mut self) -> Flow<bool> {
        self.mem.wb(W_MENU_CURSOR_Y, 3);
        let escaped = self.try_to_run_away_from_battle(W_BATTLE_MON_SPEED, W_ENEMY_MON_SPEED)?;
        self.mem.wb(W_FAILED_TO_FLEE, 0);
        if escaped {
            return Ok(true);
        }
        if self.mem.rb(W_BATTLE_PLAYER_ACTION) != BATTLEPLAYERACTION_USEMOVE {
            return Ok(false);
        }
        // jp BattleMenu
        self.battle_menu()
    }

    /// `TryToRunAwayFromBattle` (`hl` = the player's speed, `de` = the
    /// enemy's). Returns carry (got away).
    pub fn try_to_run_away_from_battle(&mut self, hl: u16, de: u16) -> Flow<bool> {
        let battle_type = self.mem.rb(W_BATTLE_TYPE);
        let escape = 'outcome: {
            if battle_type == BATTLETYPE_DEBUG || battle_type == BATTLETYPE_CONTEST {
                break 'outcome Escape::Can;
            }
            if [BATTLETYPE_TRAP, BATTLETYPE_CELEBI, BATTLETYPE_FORCESHINY, BATTLETYPE_SUICUNE].contains(&battle_type) {
                break 'outcome Escape::Cant(r::BATTLE_TEXT_CANT_ESCAPE);
            }
            if self.link_mode() != 0 {
                break 'outcome Escape::Can;
            }
            if self.battle_mode() != WILD_BATTLE {
                break 'outcome Escape::Cant(r::BATTLE_TEXT_THERES_NO_ESCAPE_FROM_TRAINER_BATTLE);
            }
            if self.mem.bit(W_ENEMY_SUB_STATUS5, SUBSTATUS_CANT_RUN) || self.mem.rb(W_PLAYER_WRAP_COUNT) != 0 {
                break 'outcome Escape::Cant(r::BATTLE_TEXT_CANT_ESCAPE);
            }
            let item = self.mem.rb(W_BATTLE_MON_ITEM);
            self.mem.wb(W_NAMED_OBJECT_INDEX, item);
            if self.get_item_held_effect(item).0 == HELD_ESCAPE {
                self.set_player_turn();
                self.get_item_name();
                self.text(r::BATTLE_TEXT_USER_FLED_USING_A_STRING_BUFFER1);
                break 'outcome Escape::Can;
            }
            // .no_flee_item
            let attempts = self.mem.rb(W_NUM_FLEE_ATTEMPTS).wrapping_add(1);
            self.mem.wb(W_NUM_FLEE_ATTEMPTS, attempts);
            let (p0, p1) = (self.mem.rb(hl), self.mem.rb(hl + 1));
            self.mem.wb(H_MULTIPLICAND + 1, p0);
            self.mem.wb(H_MULTIPLICAND + 2, p1);
            let (e0, e1) = (self.mem.rb(de), self.mem.rb(de + 1));
            self.mem.wb(H_ENEMY_MON_SPEED, e0);
            self.mem.wb(H_ENEMY_MON_SPEED + 1, e1);
            // CompareBytes(de = hMultiplicand + 1, hl = hEnemyMonSpeed): nc → escape
            if (p0, p1) >= (e0, e1) {
                break 'outcome Escape::Can;
            }
            self.mem.wb(H_MULTIPLICAND, 0);
            self.mem.wb(H_MULTIPLIER, 32);
            self.multiply();
            let (q2, q3) = (self.mem.rb(H_PRODUCT + 2), self.mem.rb(H_PRODUCT + 3));
            self.mem.wb(H_DIVIDEND, q2);
            self.mem.wb(H_DIVIDEND + 1, q3);
            let speed = u16::from(e0) << 8 | u16::from(e1);
            let divisor = (speed >> 2) as u8;
            if divisor == 0 {
                break 'outcome Escape::Can;
            }
            self.mem.wb(H_DIVISOR, divisor);
            self.divide(2);
            if self.mem.rb(H_QUOTIENT + 2) != 0 {
                break 'outcome Escape::Can;
            }
            let mut c = self.mem.rb(W_NUM_FLEE_ATTEMPTS);
            loop {
                c = c.wrapping_sub(1);
                if c == 0 {
                    break;
                }
                let (sum, carry) = self.mem.rb(H_QUOTIENT + 3).overflowing_add(30);
                self.mem.wb(H_QUOTIENT + 3, sum);
                if carry {
                    break 'outcome Escape::Can;
                }
            }
            // .cant_escape_2
            let b = self.battle_random("TryToRunAwayFromBattle");
            if self.mem.rb(H_QUOTIENT + 3) >= b {
                break 'outcome Escape::Can;
            }
            self.mem.wb(W_BATTLE_PLAYER_ACTION, BATTLEPLAYERACTION_USEITEM);
            Escape::Cant(r::BATTLE_TEXT_CANT_ESCAPE2)
        };
        match escape {
            Escape::Cant(text) => {
                // .print_inescapable_text
                self.text(text);
                self.mem.wb(W_FAILED_TO_FLEE, 1);
                Ok(false)
            }
            Escape::Can => {
                let outcome = if self.link_mode() == 0 {
                    DRAW
                } else {
                    self.mem.wb(W_BATTLE_PLAYER_ACTION, BATTLEPLAYERACTION_USEMOVE);
                    self.mem.wb(W_CUR_MOVE_NUM, BATTLEACTION_FORFEIT);
                    self.mem.wb(W_CUR_PLAYER_MOVE, 0);
                    self.link_battle_send_receive_action()?;
                    if self.mem.rb(W_BATTLE_ACTION) == BATTLEACTION_FORFEIT { DRAW } else { DRAW - 1 }
                };
                // .fled
                let r0 = (self.mem.rb(W_BATTLE_RESULT) & BATTLERESULT_BITMASK).wrapping_add(outcome);
                self.mem.wb(W_BATTLE_RESULT, r0);
                self.mem.wb(W_LOW_HEALTH_ALARM, 0); // StopDangerSound
                self.show(crate::battle::Event::Sound(SFX_RUN.into()));
                self.text(r::BATTLE_TEXT_GOT_AWAY_SAFELY);
                Ok(true)
            }
        }
    }

    /// `MoveSelectionScreen` (patched: see the `move_selection_screen`
    /// HARNESS-PATCH). Returns nz (go back to the battle menu) — never.
    pub fn move_selection_screen(&mut self) -> Flow<bool> {
        // The Ether menu (`wMoveSelectionMenuType == 2`) is opened by the item
        // effect itself (items.rs `ether_move_menu`), never from here.
        assert!(self.mem.rb(W_MOVE_SELECTION_MENU_TYPE) != 2, "Ether move menu opened outside an item");
        if std::mem::take(&mut self.flags.pending_struggle) {
            // call CheckPlayerHasUsableMoves / ret z
            let usable = self.check_player_has_usable_moves();
            assert!(!usable, "Struggle offered with a usable move");
            return Ok(false);
        }
        // The skipped menu's ListMoves leaves wNumMoves = index of the last
        // move, and the 2D menu w2DMenuNumRows = wNumMoves + 1 (as the
        // HARNESS-PATCH in move_selection_screen does).
        let count = (0..4u16).take_while(|&i| self.mem.rb(W_BATTLE_MON_MOVES + i) != 0).count() as u8;
        if count > 0 {
            self.mem.wb(W_NUM_MOVES, count - 1);
        }
        let rows = self.mem.rb(W_NUM_MOVES).wrapping_add(1);
        self.mem.wb(W2_D_MENU_NUM_ROWS, rows);
        let slot = match self.flags.pending_move.take() {
            Some(s) => s,
            None => {
                let Step::Move { slot } = self.decide(PromptSite::MoveSelection)? else { unreachable!() };
                slot
            }
        };
        let mv = self.mem.rb(W_BATTLE_MON_MOVES + u16::from(slot));
        self.mem.wb(W_MENU_CURSOR_Y, slot);
        self.mem.wb(W_CUR_MOVE_NUM, slot);
        self.mem.wb(W_CUR_PLAYER_MOVE, mv);
        Ok(false)
    }

    /// `CheckPlayerHasUsableMoves`: sets `wCurPlayerMove = STRUGGLE`; when no
    /// move is usable, prints the text and returns z (here: false).
    pub fn check_player_has_usable_moves(&mut self) -> bool {
        self.mem.wb(W_CUR_PLAYER_MOVE, STRUGGLE);
        let usable = !crate::battle::legal::player_must_struggle(&self.mem);
        if !usable {
            self.text(r::BATTLE_TEXT_MON_HAS_NO_MOVES_LEFT);
        }
        usable
    }

    /// `LinkBattleSendReceiveAction` (patched: `bootstrap::cli::handle_enemy_action`).
    pub fn link_battle_send_receive_action(&mut self) -> Flow {
        let enemy_fainted = self.mem.rw(W_ENEMY_MON_HP) == 0;
        if self.mem.rb(W_BATTLE_PLAYER_ACTION) == BATTLEPLAYERACTION_USEITEM && !enemy_fainted {
            self.mem.wb(W_BATTLE_ACTION, BATTLEACTION_SKIPTURN);
            return Ok(());
        }
        if crate::battle::legal::legal_steps(&super::mem::WithData { mem: &self.mem, data: self.data }, PromptSite::EnemyAction).is_empty() {
            self.mem.wb(W_BATTLE_ACTION, BATTLEACTION_STRUGGLE);
            return Ok(());
        }
        let byte = match self.decide(PromptSite::EnemyAction)? {
            Step::Move { slot } => BATTLEACTION_MOVE1 + slot,
            Step::Switch { slot } => BATTLEACTION_SWITCH1 + slot,
            step => unreachable!("{step:?} as the enemy's action"),
        };
        self.mem.wb(W_BATTLE_ACTION, byte);
        Ok(())
    }
}

enum Escape {
    Can,
    Cant(usize),
}
