//! Port of the trainer AI (`engine/battle/ai/move.asm`, `redundant.asm` and
//! the scoring layers of `scoring.asm`). See docs/engine.md.
//!
//! The scoring routines work on `hl` = the address of one move's score in
//! `wEnemyAIMoveScores`; here that is a `u16` address. Carry/zero flags the
//! original returns become `bool`s, documented per routine. The AI never
//! suspends for a decision, so none of these routines return `Flow`.
//!
//! The `AI_Smart_*` handlers live in `ai_smart.rs`, the switching logic of
//! `switch.asm` in `ai_switch.rs`, and `items.asm` in `ai_items.rs`.

#![allow(unused_imports)]

use super::battle::{Engine, Flow, PARTY_LEN, PP_MASK, percent};
use super::commands::{Cmd, rom_in_array};
use super::rom;
use super::rsym as r;
use super::sym::*;
use crate::defs::*;

/// `wEnemyAIMoveScores`.
pub(super) const SCORES: u16 = W_ENEMY_AI_MOVE_SCORES;

impl Engine {
    // --- AIChooseMove --------------------------------------------------------------

    /// `AIChooseMove`.
    pub fn ai_choose_move(&mut self) {
        // Wildmons attack at random.
        if self.battle_mode() == WILD_BATTLE {
            return;
        }
        if self.link_mode() != 0 {
            return;
        }
        // No use picking a move if there's no choice.
        if self.check_enemy_locked_in() {
            return;
        }

        // The default score is 20. Unusable moves are given a score of 80.
        for i in 0..4 {
            self.mem.wb(SCORES + i, 20);
        }

        // Don't pick disabled moves.
        let disabled = self.mem.rb(W_ENEMY_DISABLED_MOVE);
        if disabled != 0 {
            let mut c = 0u8;
            let mut hl = W_ENEMY_MON_MOVES;
            while self.mem.rb(hl) != disabled {
                c = c.wrapping_add(1);
                hl += 1;
            }
            self.mem.wb(SCORES + u16::from(c), 80);
        }

        // Don't pick moves with 0 PP.
        let mut hl = SCORES - 1;
        let mut de = W_ENEMY_MON_PP;
        let mut b = 0u8;
        loop {
            b += 1;
            if b == NUM_MOVES + 1 {
                break;
            }
            hl += 1;
            let a = self.mem.rb(de);
            de += 1;
            if a & PP_MASK == 0 {
                self.mem.wb(hl, 80);
            }
        }

        // Apply AI scoring layers depending on the trainer class.
        // In the Battle Tower, always use the attributes of the first class.
        let mut attrs = r::TRAINER_CLASS_ATTRIBUTES + usize::from(TRNATTR_AI_MOVE_WEIGHTS);
        if self.mem.rb(W_IN_BATTLE_TOWER_BATTLE) & 1 << IN_BATTLE_TOWER_BATTLE_F == 0 {
            let class = self.mem.rb(W_TRAINER_CLASS).wrapping_sub(1);
            attrs += usize::from(class) * usize::from(NUM_TRAINER_ATTRIBUTES);
        }
        for layer in 0u8..16 {
            // SmallFarFlagAction (CHECK_FLAG) on the ROM flags.
            let set = rom::byte(attrs + usize::from(layer >> 3)) & 1 << (layer & 7) != 0;
            if set {
                self.ai_run_layer(layer);
            }
        }

        // Decrement the scores of all moves one by one until one reaches 0.
        let (hl, c) = 'decrement: loop {
            let mut hl = SCORES;
            let mut de = W_ENEMY_MON_MOVES;
            let mut c = NUM_MOVES;
            loop {
                // If the enemy has no moves, this will infinite.
                let a = self.mem.rb(de);
                de += 1;
                if a == 0 {
                    continue 'decrement;
                }
                // We are done whenever a score reaches 0.
                let v = self.mem.rb(hl).wrapping_sub(1);
                self.mem.wb(hl, v);
                if v == 0 {
                    break 'decrement (hl, c);
                }
                // If we just decremented the fourth move's score, go back to the first move.
                hl += 1;
                c -= 1;
                if c == 0 {
                    continue 'decrement;
                }
            }
        };

        // In order to avoid bias towards the moves located first in memory, increment the scores
        // that were decremented one more time than the rest (in case there was a tie).
        // This means that the minimum score will be 1.
        let mut hl = hl;
        let mut a = c;
        loop {
            let v = self.mem.rb(hl).wrapping_add(1);
            self.mem.wb(hl, v);
            hl = hl.wrapping_sub(1);
            a += 1;
            if a == NUM_MOVES + 1 {
                break;
            }
        }

        let mut hl = SCORES;
        let mut de = W_ENEMY_MON_MOVES;
        for _ in 0..NUM_MOVES {
            // Give a score of 0 to a blank move.
            let mv = self.mem.rb(de);
            if mv == 0 {
                self.mem.wb(hl, 0);
            }
            // Disregard the move if its score is not 1.
            if self.mem.rb(hl).wrapping_sub(1) == 0 {
                self.mem.wb(hl, mv);
            } else {
                self.mem.wb(hl, 0);
            }
            hl += 1;
            de += 1;
        }

        // Randomly choose one of the moves with a score of 1.
        loop {
            let c = self.random("AIChooseMove") & (NUM_MOVES - 1);
            let a = self.mem.rb(SCORES + u16::from(c));
            if a == 0 {
                continue;
            }
            self.mem.wb(W_CUR_ENEMY_MOVE, a);
            self.mem.wb(W_CUR_ENEMY_MOVE_NUM, c);
            return;
        }
    }

    /// `AIScoringPointers`: run scoring layer number `layer`.
    fn ai_run_layer(&mut self, layer: u8) {
        match layer {
            0 => self.ai_basic(),
            1 => self.ai_setup(),
            2 => self.ai_types(),
            3 => self.ai_offensive(),
            4 => self.ai_smart(),
            5 => self.ai_opportunist(),
            6 => self.ai_aggressive(),
            7 => self.ai_cautious(),
            8 => self.ai_status(),
            9 => self.ai_risky(),
            _ => {} // AI_None
        }
    }

    // --- AI_Redundant ----------------------------------------------------------------

    /// `AI_Redundant`: whether move effect `c` will fail because it's already
    /// been used. Returns nz (`true`) if the move is a bad choice.
    pub fn ai_redundant(&mut self, c: u8) -> bool {
        let enemy_screens = self.mem.rb(W_ENEMY_SCREENS);
        match c {
            EFFECT_DREAM_EATER => self.mem.rb(W_BATTLE_MON_STATUS) & SLP_MASK == 0,
            EFFECT_HEAL | EFFECT_MORNING_SUN | EFFECT_SYNTHESIS | EFFECT_MOONLIGHT => {
                // farcall AICheckEnemyMaxHP; not redundant unless at full HP.
                self.ai_check_enemy_max_hp()
            }
            EFFECT_LIGHT_SCREEN => enemy_screens & 1 << SCREENS_LIGHT_SCREEN != 0,
            EFFECT_MIST => self.mem.bit(W_ENEMY_SUB_STATUS4, SUBSTATUS_MIST),
            EFFECT_FOCUS_ENERGY => self.mem.bit(W_ENEMY_SUB_STATUS4, SUBSTATUS_FOCUS_ENERGY),
            EFFECT_CONFUSE => {
                if self.mem.bit(W_PLAYER_SUB_STATUS3, SUBSTATUS_CONFUSED) {
                    return true;
                }
                self.mem.bit(W_PLAYER_SCREENS, SCREENS_SAFEGUARD)
            }
            EFFECT_TRANSFORM => self.mem.bit(W_ENEMY_SUB_STATUS5, SUBSTATUS_TRANSFORMED),
            EFFECT_REFLECT => enemy_screens & 1 << SCREENS_REFLECT != 0,
            EFFECT_SUBSTITUTE => self.mem.bit(W_ENEMY_SUB_STATUS4, SUBSTATUS_SUBSTITUTE),
            EFFECT_LEECH_SEED => self.mem.bit(W_PLAYER_SUB_STATUS4, SUBSTATUS_LEECH_SEED),
            EFFECT_DISABLE => self.mem.rb(W_PLAYER_DISABLE_COUNT) != 0,
            EFFECT_ENCORE => self.mem.bit(W_PLAYER_SUB_STATUS5, SUBSTATUS_ENCORED),
            EFFECT_SNORE | EFFECT_SLEEP_TALK => self.mem.rb(W_ENEMY_MON_STATUS) & SLP_MASK == 0,
            EFFECT_MEAN_LOOK => self.mem.bit(W_ENEMY_SUB_STATUS5, SUBSTATUS_CANT_RUN),
            EFFECT_NIGHTMARE => {
                // BUG: AI does not discourage Nightmare if the player has any status condition
                if self.mem.rb(W_BATTLE_MON_STATUS) == 0 {
                    return true;
                }
                self.mem.bit(W_PLAYER_SUB_STATUS1, SUBSTATUS_NIGHTMARE)
            }
            EFFECT_SPIKES => self.mem.bit(W_PLAYER_SCREENS, SCREENS_SPIKES),
            EFFECT_FORESIGHT => self.mem.bit(W_PLAYER_SUB_STATUS1, SUBSTATUS_IDENTIFIED),
            EFFECT_PERISH_SONG => self.mem.bit(W_PLAYER_SUB_STATUS1, SUBSTATUS_PERISH),
            EFFECT_SANDSTORM => self.mem.rb(W_BATTLE_WEATHER) == WEATHER_SANDSTORM,
            EFFECT_ATTRACT => {
                if self.check_opposite_gender() {
                    return true;
                }
                self.mem.bit(W_PLAYER_SUB_STATUS1, SUBSTATUS_IN_LOVE)
            }
            EFFECT_SAFEGUARD => enemy_screens & 1 << SCREENS_SAFEGUARD != 0,
            EFFECT_RAIN_DANCE => self.mem.rb(W_BATTLE_WEATHER) == WEATHER_RAIN,
            EFFECT_SUNNY_DAY => self.mem.rb(W_BATTLE_WEATHER) == WEATHER_SUN,
            EFFECT_TELEPORT => true,
            EFFECT_SWAGGER => self.mem.bit(W_PLAYER_SUB_STATUS3, SUBSTATUS_CONFUSED),
            EFFECT_FUTURE_SIGHT => {
                // BUG: AI does not discourage Future Sight when it's already been used
                enemy_screens & 1 << SCREENS_UNUSED != 0
            }
            _ => false,
        }
    }

    // --- layers ------------------------------------------------------------------------

    /// Address of the score of move slot `i`.
    fn ai_score_addr(i: u8) -> u16 {
        SCORES + u16::from(i)
    }

    /// `AI_Basic`: don't do anything redundant.
    pub fn ai_basic(&mut self) {
        for i in 0..NUM_MOVES {
            let hl = Self::ai_score_addr(i);
            let mv = self.mem.rb(W_ENEMY_MON_MOVES + u16::from(i));
            if mv == 0 {
                return;
            }
            self.ai_get_enemy_move(mv);
            let c = self.mem.rb(W_ENEMY_MOVE_STRUCT + u16::from(MOVE_EFFECT));

            // Dismiss moves with special effects if they are useless or not a good choice
            // right now. For example, healing moves, weather moves, Dream Eater...
            if self.ai_redundant(c) {
                self.ai_discourage_move(hl);
                continue;
            }

            // Dismiss status-only moves if the player can't be statused.
            if rom_in_array(r::STATUS_ONLY_EFFECTS, 1, c).is_none() {
                continue;
            }
            if self.mem.rb(W_BATTLE_MON_STATUS) != 0 {
                self.ai_discourage_move(hl);
                continue;
            }
            // Dismiss status moves if the player is Safeguarded.
            if !self.mem.bit(W_PLAYER_SCREENS, SCREENS_SAFEGUARD) {
                continue;
            }
            self.ai_discourage_move(hl);
        }
    }

    /// `AI_Setup`: use stat-modifying moves on turn 1.
    pub fn ai_setup(&mut self) {
        for i in 0..NUM_MOVES {
            let hl = Self::ai_score_addr(i);
            let mv = self.mem.rb(W_ENEMY_MON_MOVES + u16::from(i));
            if mv == 0 {
                return;
            }
            self.ai_get_enemy_move(mv);
            let a = self.mem.rb(W_ENEMY_MOVE_STRUCT + u16::from(MOVE_EFFECT));

            // Some(true) = stat up, Some(false) = stat down.
            let kind = if a < EFFECT_ATTACK_UP {
                None
            } else if a < EFFECT_EVASION_UP + 1 {
                Some(true)
            } else if a == EFFECT_EVASION_UP + 1 {
                None
            } else if a < EFFECT_EVASION_DOWN + 1 {
                Some(false)
            } else if a < EFFECT_ATTACK_UP_2 {
                None
            } else if a < EFFECT_EVASION_UP_2 + 1 {
                Some(true)
            } else if a == EFFECT_EVASION_UP_2 + 1 {
                None
            } else if a < EFFECT_EVASION_DOWN_2 + 1 {
                Some(false)
            } else {
                None
            };
            let Some(up) = kind else { continue };

            let discourage = if up {
                self.mem.rb(W_ENEMY_TURNS_TAKEN) != 0
            } else {
                self.mem.rb(W_PLAYER_TURNS_TAKEN) != 0
            };
            if !discourage {
                // .encourage
                if self.ai_50_50() {
                    continue;
                }
                self.ai_dec(hl, 2);
            } else {
                // .discourage
                if self.random("AI_Setup") < percent(12) {
                    continue;
                }
                self.ai_inc(hl, 2);
            }
        }
    }

    /// `AI_Types`: dismiss any move that the player is immune to; encourage
    /// super-effective moves; discourage not very effective moves unless all
    /// damaging moves are of the same type.
    pub fn ai_types(&mut self) {
        for i in 0..NUM_MOVES {
            let hl = Self::ai_score_addr(i);
            let mv = self.mem.rb(W_ENEMY_MON_MOVES + u16::from(i));
            if mv == 0 {
                return;
            }
            self.ai_get_enemy_move(mv);

            self.mem.wb(H_BATTLE_TURN, 1);
            self.battle_check_type_matchup();

            let a = self.mem.rb(W_TYPE_MATCHUP);
            if a == 0 {
                // .immune
                self.ai_discourage_move(hl);
                continue;
            }
            if a == EFFECTIVE {
                continue;
            }
            if a > EFFECTIVE {
                // effective
                if self.mem.rb(W_ENEMY_MOVE_STRUCT + u16::from(MOVE_POWER)) == 0 {
                    continue;
                }
                self.ai_dec(hl, 1);
                continue;
            }
            // .noteffective: discourage this move if there are any moves that do damage
            // of a different type.
            let d = self.mem.rb(W_ENEMY_MOVE_STRUCT + u16::from(MOVE_TYPE));
            let mut c = 0u8;
            let mut b = NUM_MOVES + 1;
            let mut mp = W_ENEMY_MON_MOVES;
            loop {
                b -= 1;
                if b == 0 {
                    break;
                }
                let m = self.mem.rb(mp);
                mp += 1;
                if m == 0 {
                    break;
                }
                self.ai_get_enemy_move(m);
                if self.mem.rb(W_ENEMY_MOVE_STRUCT + u16::from(MOVE_TYPE)) == d {
                    continue;
                }
                let power = self.mem.rb(W_ENEMY_MOVE_STRUCT + u16::from(MOVE_POWER));
                if power != 0 {
                    c = power;
                    break;
                }
            }
            if c == 0 {
                continue;
            }
            self.ai_inc(hl, 1);
        }
    }

    /// `AI_Offensive`: greatly discourage non-damaging moves.
    pub fn ai_offensive(&mut self) {
        for i in 0..NUM_MOVES {
            let hl = Self::ai_score_addr(i);
            let mv = self.mem.rb(W_ENEMY_MON_MOVES + u16::from(i));
            if mv == 0 {
                return;
            }
            self.ai_get_enemy_move(mv);
            if self.mem.rb(W_ENEMY_MOVE_STRUCT + u16::from(MOVE_POWER)) != 0 {
                continue;
            }
            self.ai_inc(hl, 2);
        }
    }

    /// `AI_Opportunist`: discourage stall moves when the enemy's HP is low.
    pub fn ai_opportunist(&mut self) {
        // Do nothing if enemy's HP is above 50%.
        if self.ai_check_enemy_half_hp() {
            return;
        }
        // Discourage stall moves if enemy's HP is below 25%.
        if self.ai_check_enemy_quarter_hp() {
            // 50% chance to discourage stall moves if enemy's HP is between 25% and 50%.
            if self.ai_50_50() {
                return;
            }
        }
        for i in 0..NUM_MOVES {
            let hl = Self::ai_score_addr(i);
            let mv = self.mem.rb(W_ENEMY_MON_MOVES + u16::from(i));
            if mv == 0 {
                return;
            }
            if rom_in_array(r::STALL_MOVES, 1, mv).is_some() {
                self.ai_inc(hl, 1);
            }
        }
    }

    /// `AI_Aggressive`: use whatever does the most damage.
    pub fn ai_aggressive(&mut self) {
        // Figure out which attack does the most damage and put it in c.
        let mut hl = W_ENEMY_MON_MOVES;
        let mut b = 0u8;
        let mut c = 0u8;
        let mut de = 0u16;
        loop {
            b += 1;
            if b == NUM_MOVES + 1 {
                break;
            }
            let mv = self.mem.rb(hl);
            hl += 1;
            if mv == 0 {
                break;
            }
            self.ai_get_enemy_move(mv);
            if self.mem.rb(W_ENEMY_MOVE_STRUCT + u16::from(MOVE_POWER)) == 0 {
                continue;
            }
            self.ai_damage_calc();

            // Update current move if damage is highest so far.
            let dmg = self.mem.rw(W_CUR_DAMAGE);
            if dmg < de {
                continue;
            }
            de = dmg;
            c = b;
        }

        // Nothing we can do if no attacks did damage.
        if c == 0 {
            return;
        }

        // Discourage moves that do less damage unless they're reckless too.
        let mut hl = SCORES - 1;
        let mut de = W_ENEMY_MON_MOVES;
        let mut b = 0u8;
        loop {
            b += 1;
            if b == NUM_MOVES + 1 {
                return;
            }
            let mv = self.mem.rb(de);
            de += 1;
            hl += 1;
            // Ignore this move if it is the highest damaging one.
            if b == c {
                continue;
            }
            self.ai_get_enemy_move(mv);
            // Ignore this move if its power is 0 or 1.
            // Moves such as Seismic Toss, Hidden Power, Counter and Fissure have a base power of 1.
            if self.mem.rb(W_ENEMY_MOVE_STRUCT + u16::from(MOVE_POWER)) < 2 {
                continue;
            }
            // Ignore this move if it is reckless.
            let effect = self.mem.rb(W_ENEMY_MOVE_STRUCT + u16::from(MOVE_EFFECT));
            if rom_in_array(r::RECKLESS_MOVES, 1, effect).is_some() {
                continue;
            }
            // If we made it this far, discourage this move.
            self.ai_inc(hl, 1);
        }
    }

    /// `AIDamageCalc`.
    pub fn ai_damage_calc(&mut self) {
        self.mem.wb(H_BATTLE_TURN, 1);
        let effect = self.mem.rb(W_ENEMY_MOVE_STRUCT + u16::from(MOVE_EFFECT));
        if rom_in_array(r::CONSTANT_DAMAGE_EFFECTS, 1, effect).is_some() {
            let _ = self.battle_command_constant_damage();
            return;
        }
        self.enemy_attack_damage();
        let _ = self.battle_command_damage_calc();
        let _ = self.battle_command_stab();
    }

    /// `AI_Cautious`: 90% chance to discourage moves with residual effects
    /// after the first turn.
    pub fn ai_cautious(&mut self) {
        if self.mem.rb(W_ENEMY_TURNS_TAKEN) == 0 {
            return;
        }
        for i in 0..NUM_MOVES {
            let hl = Self::ai_score_addr(i);
            let mv = self.mem.rb(W_ENEMY_MON_MOVES + u16::from(i));
            if mv == 0 {
                return;
            }
            if rom_in_array(r::RESIDUAL_MOVES, 1, mv).is_none() {
                continue;
            }
            // BUG: "Cautious" AI may fail to discourage residual moves
            if self.random("AI_Cautious") >= percent(90) + 1 {
                return;
            }
            self.ai_inc(hl, 1);
        }
    }

    /// `AI_Status`: dismiss status moves that don't affect the player.
    pub fn ai_status(&mut self) {
        for i in 0..NUM_MOVES {
            let hl = Self::ai_score_addr(i);
            let mv = self.mem.rb(W_ENEMY_MON_MOVES + u16::from(i));
            if mv == 0 {
                return;
            }
            self.ai_get_enemy_move(mv);
            let effect = self.mem.rb(W_ENEMY_MOVE_STRUCT + u16::from(MOVE_EFFECT));

            // true = poison immunity check first, false = straight to the type check.
            let poison_check = match effect {
                EFFECT_TOXIC | EFFECT_POISON => true,
                EFFECT_SLEEP | EFFECT_PARALYZE => false,
                _ => {
                    if self.mem.rb(W_ENEMY_MOVE_STRUCT + u16::from(MOVE_POWER)) == 0 {
                        continue;
                    }
                    false
                }
            };
            if poison_check
                && (self.mem.rb(W_BATTLE_MON_TYPE1) == POISON || self.mem.rb(W_BATTLE_MON_TYPE2) == POISON)
            {
                // .immune
                self.ai_discourage_move(hl);
                continue;
            }
            // .typeimmunity
            self.mem.wb(H_BATTLE_TURN, 1);
            self.battle_check_type_matchup();
            if self.mem.rb(W_TYPE_MATCHUP) != 0 {
                continue;
            }
            self.ai_discourage_move(hl);
        }
    }

    /// `AI_Risky`: use any move that will KO the target. Risky moves will
    /// often be an exception.
    pub fn ai_risky(&mut self) {
        for i in 0..NUM_MOVES {
            let hl = Self::ai_score_addr(i);
            let mv = self.mem.rb(W_ENEMY_MON_MOVES + u16::from(i));
            if mv == 0 {
                return;
            }
            self.ai_get_enemy_move(mv);
            if self.mem.rb(W_ENEMY_MOVE_STRUCT + u16::from(MOVE_POWER)) == 0 {
                continue;
            }
            // Don't use risky moves at max HP.
            let effect = self.mem.rb(W_ENEMY_MOVE_STRUCT + u16::from(MOVE_EFFECT));
            if rom_in_array(r::RISKY_EFFECTS, 1, effect).is_some() {
                if self.ai_check_enemy_max_hp() {
                    continue;
                }
                // Else, 80% chance to exclude them.
                if self.random("AI_Risky") < percent(79) - 1 {
                    continue;
                }
            }
            // .checkko
            self.ai_damage_calc();
            let dmg = self.mem.rw(W_CUR_DAMAGE);
            if self.mem.rw(W_BATTLE_MON_HP) >= dmg {
                continue;
            }
            self.ai_dec(hl, 5);
        }
    }

    // --- helpers -------------------------------------------------------------------------

    /// `inc [hl]` `n` times.
    pub(super) fn ai_inc(&mut self, hl: u16, n: u8) {
        let v = self.mem.rb(hl).wrapping_add(n);
        self.mem.wb(hl, v);
    }

    /// `dec [hl]` `n` times.
    pub(super) fn ai_dec(&mut self, hl: u16, n: u8) {
        let v = self.mem.rb(hl).wrapping_sub(n);
        self.mem.wb(hl, v);
    }

    /// `AIDiscourageMove`.
    pub fn ai_discourage_move(&mut self, hl: u16) {
        let v = self.mem.rb(hl).wrapping_add(10);
        self.mem.wb(hl, v);
    }

    /// `AIGetEnemyMove`: load the attributes of move `a` into `wEnemyMoveStruct`.
    pub fn ai_get_enemy_move(&mut self, a: u8) {
        let src = r::MOVES + usize::from(a.wrapping_sub(1)) * usize::from(MOVE_LENGTH);
        for i in 0..usize::from(MOVE_LENGTH) {
            self.mem.wb(W_ENEMY_MOVE_STRUCT + i as u16, rom::byte(src + i));
        }
    }

    /// `AI_80_20`: carry (`true`) 20% of the time.
    pub fn ai_80_20(&mut self) -> bool {
        self.random("AI_80_20") < percent(20) - 1
    }

    /// `AI_50_50`: carry (`true`) 50% of the time.
    pub fn ai_50_50(&mut self) -> bool {
        self.random("AI_50_50") < percent(50) + 1
    }

    /// `AICompareSpeed`: carry (`true`) if the enemy is faster than the player.
    pub fn ai_compare_speed(&self) -> bool {
        self.mem.rw(W_BATTLE_MON_SPEED) < self.mem.rw(W_ENEMY_MON_SPEED)
    }

    /// `AICheckMaxHP`: carry (`true`) if the HP at `hp` matches the max HP
    /// right after it.
    fn ai_check_max_hp(&self, hp: u16) -> bool {
        self.mem.rw(hp) == self.mem.rw(hp + 2)
    }

    /// `AICheckPlayerMaxHP`.
    pub fn ai_check_player_max_hp(&self) -> bool {
        self.ai_check_max_hp(W_BATTLE_MON_HP)
    }

    /// `AICheckEnemyMaxHP`.
    pub fn ai_check_enemy_max_hp(&self) -> bool {
        self.ai_check_max_hp(W_ENEMY_MON_HP)
    }

    /// Carry (`true`) if `max HP < HP * 2^shift` (16-bit arithmetic, like the
    /// original's `sla c / rl b`).
    fn ai_check_hp_fraction(&self, hp: u16, shift: u32) -> bool {
        let scaled = self.mem.rw(hp) << shift;
        self.mem.rw(hp + 2) < scaled
    }

    /// `AICheckPlayerHalfHP`: carry (`true`) if the player's HP is above 50%.
    pub fn ai_check_player_half_hp(&self) -> bool {
        self.ai_check_hp_fraction(W_BATTLE_MON_HP, 1)
    }

    /// `AICheckEnemyHalfHP`: carry (`true`) if the enemy's HP is above 50%.
    pub fn ai_check_enemy_half_hp(&self) -> bool {
        self.ai_check_hp_fraction(W_ENEMY_MON_HP, 1)
    }

    /// `AICheckEnemyQuarterHP`: carry (`true`) if the enemy's HP is above 25%.
    pub fn ai_check_enemy_quarter_hp(&self) -> bool {
        self.ai_check_hp_fraction(W_ENEMY_MON_HP, 2)
    }

    /// `AICheckPlayerQuarterHP`: carry (`true`) if the player's HP is above 25%.
    pub fn ai_check_player_quarter_hp(&self) -> bool {
        self.ai_check_hp_fraction(W_BATTLE_MON_HP, 2)
    }

    /// `AIHasMoveEffect`: carry (`true`) if the enemy has a move with effect `b`.
    pub fn ai_has_move_effect(&mut self, b: u8) -> bool {
        let mut hl = W_ENEMY_MON_MOVES;
        for _ in 0..NUM_MOVES {
            let mv = self.mem.rb(hl);
            hl += 1;
            if mv == 0 {
                return false;
            }
            self.ai_get_enemy_move(mv);
            if self.mem.rb(W_ENEMY_MOVE_STRUCT + u16::from(MOVE_EFFECT)) == b {
                return true;
            }
        }
        false
    }

    /// `AIHasMoveInArray`: carry (`true`) if the enemy has a move in the
    /// `-1`-terminated ROM array at `table`.
    pub fn ai_has_move_in_array(&self, table: usize) -> bool {
        let mut hl = table;
        loop {
            let b = rom::byte(hl);
            hl += 1;
            if b == 0xff {
                return false;
            }
            for i in 0..u16::from(NUM_MOVES) {
                if self.mem.rb(W_ENEMY_MON_MOVES + i) == b {
                    return true;
                }
            }
        }
    }

    /// `AICheckLastPlayerMon`: nz (`true`) if the player has another
    /// conscious Pokemon besides the active one; z (`false`) if the active
    /// mon is the last one.
    pub fn ai_check_last_player_mon(&self) -> bool {
        let mut hl = W_PARTY_MON1_HP;
        for c in 0..self.mem.rb(W_PARTY_COUNT) {
            if self.mem.rb(W_CUR_BATTLE_MON) != c && (self.mem.rb(hl) | self.mem.rb(hl + 1)) != 0 {
                return true;
            }
            hl += PARTY_LEN;
        }
        false
    }
}
