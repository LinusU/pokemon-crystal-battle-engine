//! Port of experience, enemy-party setup and trainer-battle switching
//! (`engine/battle/core.asm`, `engine/pokemon/`). See docs/engine.md.
//!
//! Dropped on purpose (graphics, sound, frame timing): the exp bar animation
//! (`CalcExpBar` / `PlaceExpBar`, which only leave junk in HRAM that the next
//! `Multiply` / `Divide` overwrites), the level-up stat screen, `UpdatePlayerHUD`
//! / `PrintPlayerHUD` (HUD redraws that also leave `wTempMon*` behind), `ExitMenu` /
//! `ClearBox` / `FinishBattleAnim`. Menu RAM (`wMenuCursorY`, `w2DMenu*`, ...)
//! is not written by the learn-move flow.
//!
//! The harness replaces the learn-move UI by nothing: its joypad patch presses
//! A every frame, so `YesNoBox` answers "yes", and the "which move to forget"
//! menu picks the first move. The port does the same (see `forget_move`).

#![allow(unused_imports)]

use super::battle::{Engine, Flow, PARTY_LEN, PP_MASK, percent};
use super::rom;
use super::rsym as r;
use super::switch::FlagAction;
use super::sym::*;
use crate::battle::action::{PromptSite, Step};
use crate::defs::*;

const NAME_LENGTH: u16 = 11;
const MON_NAME_LENGTH_U16: u16 = 11;
/// `CorrectNickErrors.textcommands`.
const NICK_TEXT_COMMANDS: usize = rom::flat(0x01, 0x66cf);
/// `MapGroupPointers`.
const MAP_GROUP_POINTERS: u16 = 0x4000;
const MAP_GROUP_POINTERS_BANK: u8 = 0x25;

impl Engine {
    // --- small helpers -----------------------------------------------------------

    /// `GetPartyParamLocation`: parameter `a` of `wCurPartyMon`.
    pub fn get_party_param_location(&self, a: u8) -> u16 {
        let cur = self.mem.rb(W_CUR_PARTY_MON);
        W_PARTY_MONS.wrapping_add(u16::from(a)).wrapping_add(u16::from(cur).wrapping_mul(PARTY_LEN))
    }

    /// `GetNickname`: nickname `a` of the list at `hl` into `wStringBuffer1`
    /// (through `CorrectNickErrors`).
    pub fn get_nickname(&mut self, hl: u16, a: u8) {
        let src = hl.wrapping_add(u16::from(a).wrapping_mul(NAME_LENGTH));
        self.mem.copy(W_STRING_BUFFER1, src, MON_NAME_LENGTH_U16);
        self.correct_nick_errors(W_STRING_BUFFER1);
    }

    /// `CorrectNickErrors`.
    fn correct_nick_errors(&mut self, de0: u16) {
        let mut de = de0;
        let mut b = MON_NAME_LENGTH_U16 as u8;
        loop {
            // .checkchar
            if self.mem.rb(de) == 0x50 {
                return;
            }
            let mut hl = NICK_TEXT_COMMANDS;
            loop {
                if rom::byte(hl) == 0xff {
                    break;
                }
                let c = self.mem.rb(de);
                if c >= rom::byte(hl) && c < rom::byte(hl + 1) {
                    self.mem.wb(de, 0xe6);
                }
                hl += 2;
            }
            // .done
            de += 1;
            b -= 1;
            if b == 0 {
                break;
            }
        }
        self.mem.wb(de0, 0xe6); // '?'
        self.mem.wb(de0 + 1, 0x50);
    }

    /// `GetSquareRoot`: the index of the first square not below `de`, in `b`.
    fn get_square_root(&self, de: u16) -> u8 {
        let mut b = 0u8;
        loop {
            b = b.wrapping_add(1);
            if b == 255 {
                return b;
            }
            // `GetSquareRoot.Squares` holds b² for b = 1..=254.
            let sq = u16::from(b) * u16::from(b);
            if sq >= de {
                return b;
            }
        }
    }

    // --- GetBaseData / names ---------------------------------------------------------

    /// `GetTMHMName`: "TM##" / "HM##" for item `wNamedObjectIndex` into `wStringBuffer1`.
    pub fn get_tmhm_name(&mut self) {
        let item = self.mem.rb(W_NAMED_OBJECT_INDEX);
        let is_hm = item >= HM01;
        // "TM" / "HM" in the game's charmap.
        let prefix: [u8; 2] = if is_hm { [0x87, 0x8c] } else { [0x93, 0x8c] };
        let mut de = W_STRING_BUFFER1;
        for c in prefix {
            self.mem.wb(de, c);
            de += 1;
        }
        // GetTMHMNumber: skip the dummy items, then number from 1.
        let mut a = item;
        if a >= ITEM_C3 {
            if a >= ITEM_DC {
                a = a.wrapping_sub(1);
            }
            a = a.wrapping_sub(1);
        }
        let mut a = a.wrapping_sub(TM01).wrapping_add(1);
        // HM numbers start from 51, not 1
        if is_hm {
            a = a.wrapping_sub(NUM_TMS);
        }
        // Divide and mod by 10 to get the top and bottom digits respectively
        let mut b = 0xf6u8; // '0'
        loop {
            let (r, borrow) = a.overflowing_sub(10);
            if borrow {
                break;
            }
            a = r;
            b += 1;
        }
        self.mem.wb(de, b);
        de += 1;
        self.mem.wb(de, a.wrapping_add(0xf6));
        de += 1;
        self.mem.wb(de, 0x50);
    }
}

// --- stats and experience tables -----------------------------------------------------

impl Engine {
    /// `CalcMonStats`: all six stats of the mon whose stat experience is at `hl`
    /// (`MON_STAT_EXP - 1` relative to the struct) into `de`; `with_exp` is `b`.
    /// Uses `wCurPartyLevel` and `wBaseStats`.
    pub fn calc_mon_stats(&mut self, hl: u16, mut de: u16, with_exp: bool) {
        let mut c = STAT_HP - 1;
        loop {
            c += 1;
            self.calc_mon_stat_c(hl, c, with_exp);
            let hi = self.mem.rb(H_MULTIPLICAND + 1);
            self.mem.wb(de, hi);
            de += 1;
            let lo = self.mem.rb(H_MULTIPLICAND + 2);
            self.mem.wb(de, lo);
            de += 1;
            if c == STAT_SDEF {
                return;
            }
        }
    }

    /// `CalcMonStatC`: stat `c` (1 = HP .. 6 = SpDef), result in `hMultiplicand + 1..2`.
    pub(super) fn calc_mon_stat_c(&mut self, hl: u16, c: u8, with_exp: bool) {
        let base = self.mem.rb(W_BASE_STATS + u16::from(c) - 1);
        let mut exp_hl = hl;
        if c == STAT_SDEF {
            exp_hl = exp_hl.wrapping_sub(2);
        }
        let mut b = 0u8;
        if with_exp {
            let low = exp_hl.wrapping_add(2 * u16::from(c));
            b = self.get_square_root(self.mem.rw(low.wrapping_sub(1)));
        }
        let dvs = hl.wrapping_add(u16::from(MON_DVS - MON_HP_EXP + 1));
        let dv = match c {
            STAT_ATK => self.mem.rb(dvs) >> 4,
            STAT_DEF => self.mem.rb(dvs) & 0xf,
            STAT_SPD => self.mem.rb(dvs + 1) >> 4,
            STAT_SATK | STAT_SDEF => self.mem.rb(dvs + 1) & 0xf,
            _ => {
                let (a, s) = (self.mem.rb(dvs), self.mem.rb(dvs + 1));
                ((a >> 4) & 1) << 3 | (a & 1) << 2 | ((s >> 4) & 1) << 1 | (s & 1)
            }
        };
        let sum = u16::from(base) + u16::from(dv);
        let (mut e, mut d) = (sum as u8, (sum >> 8) as u8);
        // sla e; rl d
        d = d << 1 | e >> 7;
        e <<= 1;
        b >>= 2;
        let (a, carry) = b.overflowing_add(e);
        if carry {
            d = d.wrapping_add(1);
        }
        self.mem.wb(H_MULTIPLICAND + 2, a);
        self.mem.wb(H_MULTIPLICAND + 1, d);
        self.mem.wb(H_MULTIPLICAND, 0);
        let level = self.mem.rb(W_CUR_PARTY_LEVEL);
        self.mem.wb(H_MULTIPLIER, level);
        self.multiply();
        let v = self.mem.rb(H_PRODUCT + 1);
        self.mem.wb(H_DIVIDEND, v);
        let v = self.mem.rb(H_PRODUCT + 2);
        self.mem.wb(H_DIVIDEND + 1, v);
        let v = self.mem.rb(H_PRODUCT + 3);
        self.mem.wb(H_DIVIDEND + 2, v);
        self.mem.wb(H_DIVISOR, 100);
        self.divide(3);
        let mut a = STAT_MIN_NORMAL;
        if c == STAT_HP {
            let (s, carry) = self.mem.rb(H_QUOTIENT + 3).overflowing_add(level);
            self.mem.wb(H_MULTIPLICAND + 2, s);
            if carry {
                let v = self.mem.rb(H_QUOTIENT + 2).wrapping_add(1);
                self.mem.wb(H_MULTIPLICAND + 1, v);
            }
            a = STAT_MIN_HP;
        }
        let (s, carry) = self.mem.rb(H_QUOTIENT + 3).overflowing_add(a);
        self.mem.wb(H_MULTIPLICAND + 2, s);
        if carry {
            let v = self.mem.rb(H_QUOTIENT + 2).wrapping_add(1);
            self.mem.wb(H_MULTIPLICAND + 1, v);
        }
        let q2 = self.mem.rb(H_QUOTIENT + 2);
        let max = if q2 >= (MAX_STAT_VALUE_PLUS_1 >> 8) as u8 + 1 {
            true
        } else if q2 < (MAX_STAT_VALUE_PLUS_1 >> 8) as u8 {
            false
        } else {
            self.mem.rb(H_QUOTIENT + 3) >= MAX_STAT_VALUE_PLUS_1 as u8
        };
        if max {
            self.mem.wb(H_MULTIPLICAND + 1, (MAX_STAT_VALUE >> 8) as u8);
            self.mem.wb(H_MULTIPLICAND + 2, MAX_STAT_VALUE as u8);
        }
    }

    /// `CalcExpAtLevel` for level `d`: `(a/b)*n^3 + c*n^2 + d*n - e` into
    /// `hMultiplicand..+2` (the three bytes of `hProduct + 1..3`).
    pub fn calc_exp_at_level(&mut self, d: u8) {
        let growth = self.mem.rb(W_BASE_GROWTH_RATE);
        let rate = *self.data.growth_rate(growth);
        // Cube the level
        self.level_squared(d);
        self.mem.wb(H_MULTIPLIER, d);
        self.multiply();
        // Multiply by a
        self.mem.wb(H_MULTIPLIER, (rate.cubic & 0xf0) >> 4);
        self.multiply();
        // Divide by b
        self.mem.wb(H_DIVISOR, rate.cubic & 0x0f);
        self.divide(4);
        // Push the cubic term to the stack
        let cubic = [self.mem.rb(H_QUOTIENT + 1), self.mem.rb(H_QUOTIENT + 2), self.mem.rb(H_QUOTIENT + 3)];
        // Square the level and multiply by the lower 7 bits of c
        self.level_squared(d);
        self.mem.wb(H_MULTIPLIER, rate.quadratic & 0x7f);
        self.multiply();
        // Push the absolute value of the quadratic term to the stack
        let quad = [self.mem.rb(H_PRODUCT + 1), self.mem.rb(H_PRODUCT + 2), self.mem.rb(H_PRODUCT + 3)];
        let c_byte = rate.quadratic;
        // Multiply the level by d
        self.mem.wb(H_MULTIPLICAND, 0);
        self.mem.wb(H_MULTIPLICAND + 1, 0);
        self.mem.wb(H_MULTIPLICAND + 2, d);
        self.mem.wb(H_MULTIPLIER, rate.linear);
        self.multiply();
        // Subtract e
        let b = rate.constant;
        let (v, borrow) = self.mem.rb(H_PRODUCT + 3).overflowing_sub(b);
        self.mem.wb(H_MULTIPLICAND + 2, v);
        let (v, borrow) = sbc(self.mem.rb(H_PRODUCT + 2), 0, borrow);
        self.mem.wb(H_MULTIPLICAND + 1, v);
        let (v, _) = sbc(self.mem.rb(H_PRODUCT + 1), 0, borrow);
        self.mem.wb(H_MULTIPLICAND, v);
        // If bit 7 of c is set, c is negative; otherwise, it's positive
        if c_byte & 0x80 == 0 {
            // Add c*n**2 to (d*n - e)
            self.add_term(quad, false);
        } else {
            // Subtract c*n**2 from (d*n - e)
            self.add_term(quad, true);
        }
        // Add (a/b)*n**3 to (d*n - e +/- c*n**2)
        self.add_term(cubic, false);
    }

    /// `hMultiplicand..+2 ±= term` (`term` is big endian), as `CalcExpAtLevel` does
    /// it through `hProduct + 1..3`.
    fn add_term(&mut self, term: [u8; 3], subtract: bool) {
        let mut carry = false;
        for (i, t) in [(3u16, term[2]), (2, term[1]), (1, term[0])] {
            let cur = self.mem.rb(H_PRODUCT + i);
            let (v, c) = if subtract { sbc(cur, t, carry) } else { adc(cur, t, carry) };
            self.mem.wb(H_MULTIPLICAND + i - 1, v);
            carry = c;
        }
    }

    /// `CalcExpAtLevel.LevelSquared`.
    fn level_squared(&mut self, d: u8) {
        self.mem.wb(H_MULTIPLICAND, 0);
        self.mem.wb(H_MULTIPLICAND + 1, 0);
        self.mem.wb(H_MULTIPLICAND + 2, d);
        self.mem.wb(H_MULTIPLIER, d);
        self.multiply();
    }

    /// `CalcLevel`: the level of `wTempMon`'s experience, in `d`.
    pub fn calc_level(&mut self) -> u8 {
        let species = self.mem.rb(W_TEMP_MON_SPECIES);
        self.mem.wb(W_CUR_SPECIES, species);
        self.get_base_data();
        let mut d = 1u8;
        loop {
            d = d.wrapping_add(1);
            if d == (u16::from(MAX_LEVEL) + 1) as u8 {
                break;
            }
            self.calc_exp_at_level(d);
            let exp = W_TEMP_MON_EXP;
            let (_, borrow) = self.mem.rb(exp + 2).overflowing_sub(self.mem.rb(H_PRODUCT + 3));
            let (_, borrow) = sbc(self.mem.rb(exp + 1), self.mem.rb(H_PRODUCT + 2), borrow);
            let (_, borrow) = sbc(self.mem.rb(exp), self.mem.rb(H_PRODUCT + 1), borrow);
            if borrow {
                break;
            }
        }
        d.wrapping_sub(1)
    }

    /// `CopyMonToTempMon` (`wMonType` = `PARTYMON` or `OTPARTYMON`, `wCurPartyMon`).
    pub fn copy_mon_to_temp_mon(&mut self) {
        let cur = self.mem.rb(W_CUR_PARTY_MON);
        // GetMonSpecies
        let ty = self.mem.rb(W_MON_TYPE);
        let species_list = match ty {
            PARTYMON => W_PARTY_SPECIES,
            OTPARTYMON => W_OT_PARTY_SPECIES,
            _ => self.unported("CopyMonToTempMon (BOXMON / TEMPMON: needs SRAM / breed mon)"),
        };
        let species = self.mem.rb(species_list + u16::from(cur));
        self.mem.wb(W_CUR_PARTY_SPECIES, species);
        self.mem.wb(W_CUR_SPECIES, species);
        self.get_base_data();
        let base = if ty == PARTYMON { W_PARTY_MON1 } else { W_OT_PARTY_MON1_SPECIES };
        let src = base.wrapping_add(u16::from(cur).wrapping_mul(PARTY_LEN));
        self.mem.copy(W_TEMP_MON_SPECIES, src, PARTY_LEN);
    }

    /// `LevelUpHappinessMod`.
    fn level_up_happiness_mod(&mut self) {
        let cur = self.mem.rb(W_CUR_PARTY_MON);
        let hl = W_PARTY_MON1_CAUGHT_LOCATION + u16::from(cur) * PARTY_LEN;
        let d = self.mem.rb(hl) & CAUGHT_LOCATION_MASK;
        let (b, c) = (self.mem.rb(W_MAP_GROUP), self.mem.rb(W_MAP_NUMBER));
        let loc = self.get_world_map_location(b, c);
        let c = if loc != d { HAPPINESS_GAINLEVEL } else { HAPPINESS_GAINLEVELATHOME };
        self.change_happiness(c);
    }

    /// `GetWorldMapLocation`: the Pokégear location of map `b`:`c`.
    pub fn get_world_map_location(&self, b: u8, c: u8) -> u8 {
        // GetAnyMapPointer
        let ptr_addr = MAP_GROUP_POINTERS.wrapping_add(2 * u16::from(b.wrapping_sub(1)));
        let map_ptr = u16::from(self.map_byte(ptr_addr)) | u16::from(self.map_byte(ptr_addr.wrapping_add(1))) << 8;
        let hl = map_ptr.wrapping_add(u16::from(c.wrapping_sub(1)).wrapping_mul(u16::from(MAP_LENGTH)));
        self.map_byte(hl.wrapping_add(u16::from(MAP_LOCATION)))
    }

    /// A byte at `MapGroupPointers`' bank (ROM) or, outside it, plain memory.
    fn map_byte(&self, addr: u16) -> u8 {
        if addr >= 0xc000 {
            self.mem.rb(addr)
        } else if addr >= 0x4000 && addr < 0x8000 {
            rom::byte(rom::flat(MAP_GROUP_POINTERS_BANK, addr))
        } else if addr < 0x4000 {
            rom::byte(usize::from(addr))
        } else {
            0
        }
    }
}

/// `MAX_STAT_VALUE + 1`.
const MAX_STAT_VALUE_PLUS_1: u16 = 1000;
const MAX_STAT_VALUE: u16 = 999;

/// `a + b + carry`, with carry out.
fn adc(a: u8, b: u8, carry: bool) -> (u8, bool) {
    let s = u16::from(a) + u16::from(b) + u16::from(carry);
    (s as u8, s > 0xff)
}

/// `a - b - carry`, with borrow out.
fn sbc(a: u8, b: u8, carry: bool) -> (u8, bool) {
    let s = i16::from(a) - i16::from(b) - i16::from(carry);
    (s as u8, s < 0)
}

// --- GiveExperiencePoints ----------------------------------------------------------------

impl Engine {
    /// `GiveExperiencePoints`: split the fainted enemy's experience among the
    /// participants (`wBattleParticipantsNotFainted`), level them up, and teach moves.
    pub fn give_experience_points(&mut self) {
        // Don't give experience if linked or in the Battle Tower.
        if self.link_mode() != 0 {
            return;
        }
        if self.mem.bit(W_IN_BATTLE_TOWER_BATTLE, IN_BATTLE_TOWER_BATTLE_F) {
            return;
        }
        self.evenly_divide_exp_among_participants();
        self.mem.wb(W_CUR_PARTY_MON, 0);
        let mut bc = W_PARTY_MON1_SPECIES;
        loop {
            self.give_experience_to_mon(bc);
            // .next_mon
            let count = self.mem.rb(W_PARTY_COUNT);
            let next = self.mem.rb(W_CUR_PARTY_MON).wrapping_add(1);
            if next == count {
                break;
            }
            self.mem.wb(W_CUR_PARTY_MON, next);
            bc = self.get_party_param_location(MON_SPECIES);
        }
        // .done
        self.reset_battle_participants();
    }

    /// The loop body of `GiveExperiencePoints` for the party mon at `bc`.
    fn give_experience_to_mon(&mut self, bc: u16) {
        let cur = self.mem.rb(W_CUR_PARTY_MON);
        if self.mem.rw(bc + u16::from(MON_HP)) == 0 {
            return; // fainted
        }
        if !self.small_far_flag_action(W_BATTLE_PARTICIPANTS_NOT_FAINTED, cur, FlagAction::Check) {
            return;
        }
        // give stat exp
        let mut de = bc + u16::from(MON_STAT_EXP) + 1;
        for i in 0..u16::from(NUM_EXP_STATS) {
            let hl = W_ENEMY_MON_BASE_STATS + i;
            let mut maxed = false;
            let (sum, carry) = self.mem.rb(de).overflowing_add(self.mem.rb(hl));
            self.mem.wb(de, sum);
            if carry {
                de -= 1;
                let a = self.mem.rb(de).wrapping_add(1);
                if a == 0 {
                    maxed = true;
                } else {
                    self.mem.wb(de, a);
                    de += 1;
                }
            }
            if !maxed {
                // .no_carry_stat_exp: Pokérus doubles it
                let pokerus = self.get_party_param_location(MON_POKERUS);
                if self.mem.rb(pokerus) != 0 {
                    let (sum, carry) = self.mem.rb(de).overflowing_add(self.mem.rb(hl));
                    self.mem.wb(de, sum);
                    if carry {
                        de -= 1;
                        let a = self.mem.rb(de).wrapping_add(1);
                        if a == 0 {
                            maxed = true;
                        } else {
                            self.mem.wb(de, a);
                            de += 1;
                        }
                    }
                }
            }
            if maxed {
                self.mem.wb(de, 0xff);
                de += 1;
                self.mem.wb(de, 0xff);
            }
            // .stat_exp_awarded
            de += 2;
        }
        self.mem.wb(H_MULTIPLICAND, 0);
        self.mem.wb(H_MULTIPLICAND + 1, 0);
        let base_exp = self.mem.rb(W_ENEMY_MON_BASE_EXP);
        self.mem.wb(H_MULTIPLICAND + 2, base_exp);
        let level = self.mem.rb(W_ENEMY_MON_LEVEL);
        self.mem.wb(H_MULTIPLIER, level);
        self.multiply();
        self.mem.wb(H_DIVISOR, 7);
        self.divide(4);
        // Boost Experience for traded Pokemon
        let ot_id = bc + u16::from(MON_OT_ID);
        let traded = self.mem.rb(W_PLAYER_ID) != self.mem.rb(ot_id)
            || self.mem.rb(W_PLAYER_ID + 1) != self.mem.rb(ot_id + 1);
        if traded {
            self.boost_exp();
        }
        // Boost experience for a Trainer Battle
        self.mem.wb(W_STRING_BUFFER2 + 2, u8::from(traded));
        if self.battle_mode() != WILD_BATTLE {
            self.boost_exp();
        }
        // Boost experience for Lucky Egg
        let item = self.get_party_param_location(MON_ITEM);
        if self.mem.rb(item) == LUCKY_EGG {
            self.boost_exp();
        }
        let q3 = self.mem.rb(H_QUOTIENT + 3);
        self.mem.wb(W_STRING_BUFFER2 + 1, q3);
        let q2 = self.mem.rb(H_QUOTIENT + 2);
        self.mem.wb(W_STRING_BUFFER2, q2);
        self.get_nickname(W_PARTY_MON_NICKNAMES, cur);
        self.text(r::TEXT_MON_GAINED_EXP_POINT);
        let v = self.mem.rb(W_STRING_BUFFER2 + 1);
        self.mem.wb(H_QUOTIENT + 3, v);
        let v = self.mem.rb(W_STRING_BUFFER2);
        self.mem.wb(H_QUOTIENT + 2, v);
        self.animate_exp_bar();
        // MON_EXP += hQuotient + 2..3
        let exp = bc + u16::from(MON_EXP);
        let (v, carry) = self.mem.rb(exp + 2).overflowing_add(self.mem.rb(H_QUOTIENT + 3));
        self.mem.wb(exp + 2, v);
        let (v, carry) = adc(self.mem.rb(exp + 1), self.mem.rb(H_QUOTIENT + 2), carry);
        self.mem.wb(exp + 1, v);
        if carry {
            let v = self.mem.rb(exp).wrapping_add(1);
            self.mem.wb(exp, v);
            if v == 0 {
                self.mem.wb(exp, 0xff);
                self.mem.wb(exp + 1, 0xff);
                self.mem.wb(exp + 2, 0xff);
            }
        }
        // .no_exp_overflow
        let species = self.mem.rb(W_PARTY_SPECIES + u16::from(cur));
        self.mem.wb(W_CUR_SPECIES, species);
        self.get_base_data();
        self.calc_exp_at_level(MAX_LEVEL);
        // If the exp reached the maximum, clamp it.
        let (b, c, d) = (self.mem.rb(H_QUOTIENT + 1), self.mem.rb(H_QUOTIENT + 2), self.mem.rb(H_QUOTIENT + 3));
        let (_, borrow) = self.mem.rb(exp + 2).overflowing_sub(d);
        let (_, borrow) = sbc(self.mem.rb(exp + 1), c, borrow);
        let (_, borrow) = sbc(self.mem.rb(exp), b, borrow);
        if !borrow {
            self.mem.wb(exp, b);
            self.mem.wb(exp + 1, c);
            self.mem.wb(exp + 2, d);
        }
        // .not_max_exp: Check if the mon leveled up
        self.mem.wb(W_MON_TYPE, PARTYMON);
        self.copy_mon_to_temp_mon();
        let d = self.calc_level();
        let old_level = self.mem.rb(bc + u16::from(MON_LEVEL));
        if old_level >= MAX_LEVEL || old_level == d {
            return;
        }
        // <NICKNAME> grew to level ##!
        self.mem.wb(W_TEMP_LEVEL, old_level);
        let saved_party_level = self.mem.rb(W_CUR_PARTY_LEVEL);
        self.mem.wb(W_CUR_PARTY_LEVEL, d);
        self.mem.wb(bc + u16::from(MON_LEVEL), d);
        let species = self.mem.rb(bc + u16::from(MON_SPECIES));
        self.mem.wb(W_CUR_SPECIES, species);
        self.mem.wb(W_TEMP_SPECIES, species);
        self.get_base_data();
        let old_max_hp = self.mem.rw(bc + u16::from(MON_MAXHP));
        self.calc_mon_stats(bc + u16::from(MON_STAT_EXP) - 1, bc + u16::from(MON_MAXHP), true);
        let diff = self.mem.rw(bc + u16::from(MON_MAXHP)).wrapping_sub(old_max_hp);
        let hp = self.mem.rw(bc + u16::from(MON_HP)).wrapping_add(diff);
        self.mem.ww(bc + u16::from(MON_HP), hp);
        if self.mem.rb(W_CUR_BATTLE_MON) == cur {
            // The active mon: refresh its battle struct.
            let hl = bc + u16::from(MON_HP);
            let (hi, lo) = (self.mem.rb(hl), self.mem.rb(hl + 1));
            self.mem.wb(W_BATTLE_MON_HP, hi);
            self.mem.wb(W_BATTLE_MON_HP + 1, lo);
            self.mem.copy(W_BATTLE_MON_MAX_HP, bc + u16::from(MON_MAXHP), PARTY_LEN - u16::from(MON_MAXHP));
            let level = self.mem.rb(bc + u16::from(MON_LEVEL));
            self.mem.wb(W_BATTLE_MON_LEVEL, level);
            if !self.mem.bit(W_PLAYER_SUB_STATUS5, SUBSTATUS_TRANSFORMED) {
                self.mem.copy(W_PLAYER_STATS, bc + u16::from(MON_ATK), PARTY_LEN - u16::from(MON_ATK));
            }
            // .transformed
            self.mem.wb(W_APPLY_STAT_LEVEL_MULTIPLIERS_TO_ENEMY, 0);
            self.apply_stat_level_multiplier_on_all_stats();
            self.apply_status_effect_on_player_stats();
            self.badge_stat_boosts();
            // UpdatePlayerHUD, EmptyBattleTextbox, LoadTilemapToTempTilemap: graphics
        }
        // .skip_active_mon_update
        self.level_up_happiness_mod();
        if self.mem.rb(W_CUR_BATTLE_MON) != cur {
            self.text(r::BATTLE_TEXT_STRING_BUFFER1_GREW_TO_LEVEL);
        }
        // .skip_exp_bar_animation
        self.mem.wb(W_MON_TYPE, PARTYMON);
        self.copy_mon_to_temp_mon();
        // PrintTempMonStats, WaitPressAorB_BlinkCursor: the stat screen
        self.mem.wb(W_MON_TYPE, PARTYMON);
        let species = self.mem.rb(W_CUR_SPECIES);
        self.mem.wb(W_TEMP_SPECIES, species);
        let new_level = self.mem.rb(W_CUR_PARTY_LEVEL);
        let mut b = self.mem.rb(W_TEMP_LEVEL);
        loop {
            b = b.wrapping_add(1);
            self.mem.wb(W_CUR_PARTY_LEVEL, b);
            self.learn_level_moves();
            if b == new_level {
                break;
            }
        }
        self.mem.wb(W_CUR_PARTY_LEVEL, new_level);
        self.small_far_flag_action(W_EVOLVABLE_FLAGS, cur, FlagAction::Set);
        self.mem.wb(W_CUR_PARTY_LEVEL, saved_party_level);
    }

    /// `GiveExperiencePoints.EvenlyDivideExpAmongParticipants`.
    fn evenly_divide_exp_among_participants(&mut self) {
        let mut b = self.mem.rb(W_BATTLE_PARTICIPANTS_NOT_FAINTED);
        let mut count = 0u8;
        for _ in 0..PARTY_LENGTH {
            count += b & 1;
            b >>= 1;
        }
        if count < 2 {
            return;
        }
        self.mem.wb(W_TEMP_BYTE_VALUE, count);
        for i in 0..(W_ENEMY_MON_END - W_ENEMY_MON_BASE_STATS) {
            self.mem.wb(H_DIVIDEND, 0);
            let v = self.mem.rb(W_ENEMY_MON_BASE_STATS + i);
            self.mem.wb(H_DIVIDEND + 1, v);
            let v = self.mem.rb(W_TEMP_BYTE_VALUE);
            self.mem.wb(H_DIVISOR, v);
            self.divide(2);
            let q = self.mem.rb(H_QUOTIENT + 3);
            self.mem.wb(W_ENEMY_MON_BASE_STATS + i, q);
        }
    }

    /// `BoostExp`: multiply `hProduct + 2..3` by 1.5.
    fn boost_exp(&mut self) {
        let b = self.mem.rb(H_PRODUCT + 2);
        let c = self.mem.rb(H_PRODUCT + 3);
        let half_c = c >> 1 | (b & 1) << 7;
        let half_b = b >> 1;
        let (a, carry) = c.overflowing_add(half_c);
        self.mem.wb(H_PRODUCT + 3, a);
        let (a, _) = adc(b, half_b, carry);
        self.mem.wb(H_PRODUCT + 2, a);
    }

    /// `AnimateExpBar`: the memory side. The active mon's `wTempMon` gets the new
    /// experience and `wBattleMonLevel` follows the levels the bar passes.
    fn animate_exp_bar(&mut self) {
        if self.mem.rb(W_CUR_BATTLE_MON) != self.mem.rb(W_CUR_PARTY_MON) {
            return;
        }
        if self.mem.rb(W_BATTLE_MON_LEVEL) >= MAX_LEVEL {
            return;
        }
        let p3 = self.mem.rb(H_PRODUCT + 3);
        self.mem.wb(W_EXPERIENCE_GAINED + 2, p3);
        let p2 = self.mem.rb(H_PRODUCT + 2);
        self.mem.wb(W_EXPERIENCE_GAINED + 1, p2);
        self.mem.wb(W_EXPERIENCE_GAINED, 0);
        self.mem.wb(W_MON_TYPE, PARTYMON);
        self.copy_mon_to_temp_mon();
        let mut e = self.mem.rb(W_TEMP_MON_LEVEL);
        // CalcExpBar (HRAM scratch only)
        let exp = W_TEMP_MON_EXP;
        let (v, carry) = self.mem.rb(W_EXPERIENCE_GAINED + 2).overflowing_add(self.mem.rb(exp + 2));
        self.mem.wb(exp + 2, v);
        let (v, carry) = adc(self.mem.rb(W_EXPERIENCE_GAINED + 1), self.mem.rb(exp + 1), carry);
        self.mem.wb(exp + 1, v);
        if carry {
            let v = self.mem.rb(exp).wrapping_add(1);
            self.mem.wb(exp, v);
            if v == 0 {
                self.mem.wb(exp, 0xff);
                self.mem.wb(exp + 1, 0xff);
                self.mem.wb(exp + 2, 0xff);
            }
        }
        // .NoOverflow
        self.calc_exp_at_level(MAX_LEVEL);
        let (b, c, d) = (self.mem.rb(H_PRODUCT + 1), self.mem.rb(H_PRODUCT + 2), self.mem.rb(H_PRODUCT + 3));
        let (_, borrow) = self.mem.rb(exp + 2).overflowing_sub(d);
        let (_, borrow) = sbc(self.mem.rb(exp + 1), c, borrow);
        let (_, borrow) = sbc(self.mem.rb(exp), b, borrow);
        if !borrow {
            self.mem.wb(exp, b);
            self.mem.wb(exp + 1, c);
            self.mem.wb(exp + 2, d);
        }
        let mut d = self.calc_level();
        if d < e {
            d = e;
        }
        // .LoopLevels
        loop {
            if e >= MAX_LEVEL || e == d {
                break;
            }
            let level = e + 1;
            self.mem.wb(W_TEMP_MON_LEVEL, level);
            self.mem.wb(W_CUR_PARTY_LEVEL, level);
            self.mem.wb(W_BATTLE_MON_LEVEL, level);
            // PrintPlayerHUD, the level-up jingle: graphics and sound
            self.mem.copy(W_STRING_BUFFER1, W_BATTLE_MON_NICKNAME, MON_NAME_LENGTH_U16);
            self.text(r::BATTLE_TEXT_STRING_BUFFER1_GREW_TO_LEVEL);
            e += 1;
        }
        // .FinishExpBar: restores hProduct + 2..3 around the bar's HRAM scratch work.
        self.mem.wb(H_PRODUCT + 2, p2);
        self.mem.wb(H_PRODUCT + 3, p3);
    }
}

// --- LearnLevelMoves / LearnMove -----------------------------------------------------------

impl Engine {
    /// `LearnLevelMoves`: teach `wTempSpecies`' moves for `wCurPartyLevel` to
    /// `wCurPartyMon`.
    fn learn_level_moves(&mut self) {
        let species = self.mem.rb(W_TEMP_SPECIES);
        self.mem.wb(W_CUR_PARTY_SPECIES, species);
        for &(b, a) in self.data.species(species).learnset {
            // .find_move
            if self.mem.rb(W_CUR_PARTY_LEVEL) != b {
                continue;
            }
            let d = a;
            let moves = W_PARTY_MON1_MOVES + u16::from(self.mem.rb(W_CUR_PARTY_MON)) * PARTY_LEN;
            let has_move = (0..u16::from(NUM_MOVES)).any(|i| self.mem.rb(moves + i) == d);
            if has_move {
                continue;
            }
            // .learn
            self.mem.wb(W_PUTATIVE_TMHM_MOVE, d);
            self.mem.wb(W_NAMED_OBJECT_INDEX, d);
            self.get_move_name();
            self.copy_name1(W_STRING_BUFFER1);
            self.learn_move();
        }
        // .done
        let species = self.mem.rb(W_CUR_PARTY_SPECIES);
        self.mem.wb(W_TEMP_SPECIES, species);
    }

    /// `LearnMove`. Returns `b`: 1 if the move was learned. The harness answers every
    /// prompt with A: "yes, forget a move" and the first move in the list.
    fn learn_move(&mut self) -> u8 {
        let cur = self.mem.rb(W_CUR_PARTY_MON);
        self.get_nickname(W_PARTY_MON_NICKNAMES, cur);
        self.mem.copy(W_MON_OR_ITEM_NAME_BUFFER, W_STRING_BUFFER1, MON_NAME_LENGTH_U16);
        let de = W_PARTY_MON1_MOVES + u16::from(cur) * PARTY_LEN;
        let mut hl = de;
        // Get the first empty move slot.
        let mut found = false;
        for _ in 0..NUM_MOVES {
            if self.mem.rb(hl) == 0 {
                found = true;
                break;
            }
            hl += 1;
        }
        if !found {
            // The party mon has all four slots occupied: forget one.
            self.text(r::ASK_FORGET_MOVE_TEXT);
            // YesNoBox answers yes; MoveAskForgetText; the menu picks the first move.
            self.text(r::MOVE_ASK_FORGET_TEXT);
            let slot = de;
            let forgotten = self.mem.rb(slot);
            if self.data.tables.hm_moves.contains(&forgotten) {
                // MoveCantForgetHMText, then the menu opens again and A picks the same
                // move: the original never gets out of this loop.
                panic!("engine: the oracle hangs forgetting an HM move ({forgotten})");
            }
            hl = slot;
            self.mem.wb(W_NAMED_OBJECT_INDEX, forgotten);
            if self.battle_mode() != 0 && self.mem.rb(W_DISABLED_MOVE) == forgotten {
                self.mem.wb(W_DISABLED_MOVE, 0);
                self.mem.wb(W_PLAYER_DISABLE_COUNT, 0);
            }
            self.get_move_name();
            self.text(r::TEXT_1_2_AND_POOF);
        }
        // .learn
        let mv = self.mem.rb(W_PUTATIVE_TMHM_MOVE);
        self.mem.wb(hl, mv);
        let pp_slot = hl + u16::from(MON_PP - MON_MOVES);
        let pp = self.data.mv(mv).pp;
        self.mem.wb(pp_slot, pp);
        if self.battle_mode() != 0
            && self.mem.rb(W_CUR_BATTLE_MON) == cur
            && !self.mem.bit(W_PLAYER_SUB_STATUS5, SUBSTATUS_TRANSFORMED)
        {
            self.mem.copy(W_BATTLE_MON_MOVES, de, u16::from(NUM_MOVES));
            self.mem.copy(W_BATTLE_MON_PP, de + u16::from(MON_PP - MON_MOVES), u16::from(NUM_MOVES));
        }
        // .learned
        self.text(r::LEARNED_MOVE_TEXT);
        1
    }
}

// --- the enemy's switch-in (trainer battles) -----------------------------------------------

impl Engine {
    /// `EnemySwitch`.
    pub fn enemy_switch(&mut self) -> Flow {
        if !self.check_whether_to_ask_switch() {
            self.enemy_switch_set_mode()?;
            return Ok(());
        }
        // Shift Mode
        self.reset_enemy_battle_vars();
        let b = match self.check_whether_switchmon_is_predetermined() {
            Some(b) => b,
            None => self.find_mon_in_ot_party_to_switch_into_battle()?,
        };
        // 'b' contains the PartyNr of the mon the AI will switch to
        self.load_enemy_mon_to_switch_to(b);
        let keep = self.offer_switch()?;
        // ClearEnemyMonBox: graphics only
        self.show_battle_text_enemy_sent_out();
        self.show_set_enemy_mon_and_send_out_animation();
        if keep {
            return Ok(());
        }
        // If we're here, then we're switching too
        self.mem.wb(W_BATTLE_PARTICIPANTS_NOT_FAINTED, 0);
        self.mem.wb(W_BATTLE_PARTICIPANTS_INCLUDING_FAINTED, 0);
        self.mem.wb(W_BATTLE_PLAYER_ACTION, 0);
        self.mem.wb(W_ENEMY_IS_SWITCHING, 1);
        self.player_switch()?;
        Ok(())
    }

    /// `OfferSwitch` as the harness patches it: one `OfferSwitch` decision. Returns
    /// carry (keep the current mon). A swap sets `wLastPlayerMon` / `wCurBattleMon` and
    /// zeroes both current moves.
    fn offer_switch(&mut self) -> Flow<bool> {
        let step = self.decide(PromptSite::OfferSwitch)?;
        let Step::OfferSwitch { keep, slot } = step else { unreachable!("offer switch only accepts offer_switch") };
        if keep {
            return Ok(true);
        }
        let old = self.mem.rb(W_CUR_BATTLE_MON);
        self.mem.wb(W_LAST_PLAYER_MON, old);
        self.mem.wb(W_CUR_BATTLE_MON, slot.expect("offer_switch without keep needs a slot"));
        self.mem.wb(W_CUR_PLAYER_MOVE, 0);
        self.mem.wb(W_CUR_ENEMY_MOVE, 0);
        Ok(false)
    }

    /// `CheckWhetherToAskSwitch` (carry = ask the player).
    pub(super) fn check_whether_to_ask_switch(&mut self) -> bool {
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
        let cur = self.mem.rb(W_CUR_PARTY_MON);
        let battle_mon = self.mem.rb(W_CUR_BATTLE_MON);
        self.mem.wb(W_CUR_PARTY_MON, battle_mon);
        let all_fainted = self.check_cur_party_mon_fainted();
        self.mem.wb(W_CUR_PARTY_MON, cur);
        !all_fainted
    }

    /// `CheckCurPartyMonFainted`: carry if every party mon other than `wCurPartyMon`
    /// has fainted.
    fn check_cur_party_mon_fainted(&self) -> bool {
        let cur = self.mem.rb(W_CUR_PARTY_MON);
        let mut b = 0u8;
        let mut hl = W_PARTY_MON1_HP;
        loop {
            if cur != b && self.mem.rw(hl) != 0 {
                return false;
            }
            b += 1;
            if self.mem.rb(W_PARTY_COUNT) == b {
                return true;
            }
            hl += PARTY_LEN;
        }
    }

    /// `FindMonInOTPartyToSwitchIntoBattle`: the OT party slot the AI switches to.
    /// An external trainer (`enemy_control = "external"`) picks with an
    /// `EnemySwitchIn` decision instead.
    pub(super) fn find_mon_in_ot_party_to_switch_into_battle(&mut self) -> Flow<u8> {
        if self.flags.external_enemy {
            let step = self.decide(PromptSite::EnemySwitchIn)?;
            let Step::Switch { slot } = step else { unreachable!("enemy switch-in only accepts switch") };
            return Ok(slot);
        }
        Ok(self.find_mon_in_ot_party_to_switch_into_battle_ai())
    }

    /// `FindMonInOTPartyToSwitchIntoBattle` itself.
    fn find_mon_in_ot_party_to_switch_into_battle_ai(&mut self) -> u8 {
        let mut b = 0xffu8;
        self.mem.wb(W_ENEMY_EFFECTIVENESS_VS_PLAYER_MONS, 1);
        self.mem.wb(W_PLAYER_EFFECTIVENESS_VS_ENEMY_MONS, 1);
        loop {
            let v = self.mem.rb(W_ENEMY_EFFECTIVENESS_VS_PLAYER_MONS) << 1;
            self.mem.wb(W_ENEMY_EFFECTIVENESS_VS_PLAYER_MONS, v);
            let v = self.mem.rb(W_PLAYER_EFFECTIVENESS_VS_ENEMY_MONS) << 1;
            self.mem.wb(W_PLAYER_EFFECTIVENESS_VS_ENEMY_MONS, v);
            b = b.wrapping_add(1);
            if self.mem.rb(W_OT_PARTY_COUNT) == b {
                return self.score_mon_type_matchups();
            }
            let fit = self.mem.rb(W_CUR_OT_MON) != b
                && self.mem.rw(self.get_party_location(W_OT_PARTY_MON1_HP, b)) != 0;
            if fit {
                self.look_up_the_effectiveness_of_every_move(b);
                self.is_the_player_mon_types_effective_against_ot_mon(b);
            } else {
                // .discourage
                self.mem.set_bit(W_PLAYER_EFFECTIVENESS_VS_ENEMY_MONS, 0);
            }
        }
    }

    /// `LookUpTheEffectivenessOfEveryMove`: is any of OT mon `b`'s moves super effective
    /// against the player's mon?
    fn look_up_the_effectiveness_of_every_move(&mut self, b: u8) {
        let mut hl = self.get_party_location(W_OT_PARTY_MON1_MOVES, b);
        for _ in 0..NUM_MOVES {
            let a = self.mem.rb(hl);
            hl += 1;
            if a == 0 {
                return;
            }
            for (i, b) in (0..).zip(self.data.mv(a).bytes()) {
                self.mem.wb(W_ENEMY_MOVE_STRUCT + i, b);
            }
            self.set_enemy_turn();
            self.battle_check_type_matchup();
            if self.mem.rb(W_TYPE_MATCHUP) > EFFECTIVE {
                self.mem.set_bit(W_ENEMY_EFFECTIVENESS_VS_PLAYER_MONS, 0);
                return;
            }
        }
    }

    /// `IsThePlayerMonTypesEffectiveAgainstOTMon`: the player's mon's types against
    /// OT mon `b` (whose base types are written over `wEnemyMonType`, as in the original).
    fn is_the_player_mon_types_effective_against_ot_mon(&mut self, b: u8) {
        let species = self.mem.rb(W_OT_PARTY_COUNT + 1 + u16::from(b));
        for (i, t) in (0..).zip(self.data.species(species).types) {
            self.mem.wb(W_ENEMY_MON_TYPE1 + i, t);
        }
        let t1 = self.mem.rb(W_BATTLE_MON_TYPE1);
        self.mem.wb(W_PLAYER_MOVE_STRUCT + u16::from(MOVE_TYPE), t1);
        self.set_player_turn();
        self.battle_check_type_matchup();
        let mut super_effective = self.mem.rb(W_TYPE_MATCHUP) > EFFECTIVE;
        if !super_effective {
            let t2 = self.mem.rb(W_BATTLE_MON_TYPE2);
            self.mem.wb(W_PLAYER_MOVE_STRUCT + u16::from(MOVE_TYPE), t2);
            self.battle_check_type_matchup();
            super_effective = self.mem.rb(W_TYPE_MATCHUP) > EFFECTIVE;
        }
        if !super_effective {
            return;
        }
        if self.mem.bit(W_ENEMY_EFFECTIVENESS_VS_PLAYER_MONS, 0) {
            self.mem.res_bit(W_ENEMY_EFFECTIVENESS_VS_PLAYER_MONS, 0);
        } else {
            self.mem.set_bit(W_PLAYER_EFFECTIVENESS_VS_ENEMY_MONS, 0);
        }
    }

    /// `ScoreMonTypeMatchups`: pick the slot from the two effectiveness bit arrays.
    fn score_mon_type_matchups(&mut self) -> u8 {
        // .loop1
        loop {
            let v = self.mem.rb(W_ENEMY_EFFECTIVENESS_VS_PLAYER_MONS) << 1;
            self.mem.wb(W_ENEMY_EFFECTIVENESS_VS_PLAYER_MONS, v);
            let p = self.mem.rb(W_PLAYER_EFFECTIVENESS_VS_ENEMY_MONS);
            self.mem.wb(W_PLAYER_EFFECTIVENESS_VS_ENEMY_MONS, p << 1);
            if p & 0x80 != 0 {
                break;
            }
        }
        let mut b = self.mem.rb(W_OT_PARTY_COUNT);
        let mut c = self.mem.rb(W_PLAYER_EFFECTIVENESS_VS_ENEMY_MONS);
        // .loop2
        let all_discouraged = loop {
            let carry = c & 0x80 != 0;
            c <<= 1;
            if !carry {
                break false;
            }
            b = b.wrapping_sub(1);
            if b == 0 {
                break true;
            }
        };
        if !all_discouraged {
            // .okay
            let a = self.mem.rb(W_ENEMY_EFFECTIVENESS_VS_PLAYER_MONS);
            if a != 0 {
                let mut b = 0xffu8;
                let mut c = a;
                loop {
                    b = b.wrapping_add(1);
                    let carry = c & 0x80 != 0;
                    c <<= 1;
                    if carry {
                        return b;
                    }
                }
            }
            // .okay2
            let mut b = 0xffu8;
            let mut c = self.mem.rb(W_PLAYER_EFFECTIVENESS_VS_ENEMY_MONS);
            loop {
                b = b.wrapping_add(1);
                let carry = c & 0x80 != 0;
                c <<= 1;
                if !carry {
                    return b;
                }
            }
        }
        // .loop5
        loop {
            let b = self.mem.rb(W_OT_PARTY_COUNT);
            let a = self.battle_random("ScoreMonTypeMatchups") & 7;
            if a >= b {
                continue;
            }
            if self.mem.rb(W_CUR_OT_MON) == a {
                continue;
            }
            if self.mem.rw(self.get_party_location(W_OT_PARTY_MON1_HP, a)) == 0 {
                continue;
            }
            return a;
        }
    }
}

// --- LoadEnemyMon -----------------------------------------------------------------------------

impl Engine {
    /// `LoadEnemyMon`: build `wEnemyMon` for `wTempEnemyMonSpecies`.
    pub fn load_enemy_mon(&mut self) {
        // Clear the whole enemy mon struct (wEnemyMon)
        self.mem.fill(W_ENEMY_MON_SPECIES, W_ENEMY_MON_END - W_ENEMY_MON, 0);
        // We don't need to be here if we're in a link battle, or a Battle Tower battle.
        if self.link_mode() != 0 || self.mem.bit(W_IN_BATTLE_TOWER_BATTLE, IN_BATTLE_TOWER_BATTLE_F) {
            self.init_enemy_mon();
            return;
        }
        let species = self.mem.rb(W_TEMP_ENEMY_MON_SPECIES);
        self.mem.wb(W_ENEMY_MON_SPECIES, species);
        self.mem.wb(W_CUR_SPECIES, species);
        self.mem.wb(W_CUR_PARTY_SPECIES, species);
        self.get_base_data();
        let mode = self.battle_mode();
        // Let's get the item
        let item = if mode == WILD_BATTLE {
            // In a wild battle, we pull from the item slots in BaseData.
            // Force Item1 (Ho-Oh, Lugia and Snorlax encounters)
            let item1 = self.mem.rb(W_BASE_ITEM1);
            if self.mem.rb(W_BATTLE_TYPE) == BATTLETYPE_FORCEITEM {
                item1
            } else if self.battle_random("LoadEnemyMon") < percent(75) + 1 {
                // 25% chance of getting an item
                NO_ITEM
            } else if self.battle_random("LoadEnemyMon") >= percent(8) {
                // From there, an 8% chance for Item2
                item1
            } else {
                self.mem.rb(W_BASE_ITEM2)
            }
        } else {
            // If we're in a trainer battle, the item is in the party struct
            let cur = self.mem.rb(W_CUR_PARTY_MON);
            self.mem.rb(self.get_party_location(W_OT_PARTY_MON1_ITEM, cur))
        };
        self.mem.wb(W_ENEMY_MON_ITEM, item);
        // Initialize DVs
        if mode != 0 && self.mem.bit(W_ENEMY_SUB_STATUS5, SUBSTATUS_TRANSFORMED) {
            // Transformed: the DVs it had before.
            let (hi, lo) = (self.mem.rb(W_ENEMY_BACKUP_D_VS), self.mem.rb(W_ENEMY_BACKUP_D_VS + 1));
            self.mem.wb(W_ENEMY_MON_D_VS, hi);
            self.mem.wb(W_ENEMY_MON_D_VS + 1, lo);
        } else {
            self.init_enemy_mon_dvs(mode);
        }
        // .Happiness
        self.mem.wb(W_ENEMY_MON_HAPPINESS, BASE_HAPPINESS);
        let level = self.mem.rb(W_CUR_PARTY_LEVEL);
        self.mem.wb(W_ENEMY_MON_LEVEL, level);
        self.calc_mon_stats(
            W_ENEMY_MON_D_VS - u16::from(MON_DVS - MON_STAT_EXP + 1),
            W_ENEMY_MON_MAX_HP,
            false,
        );
        if mode == TRAINER_BATTLE {
            // .OpponentParty: the rest comes from the party struct.
            let cur = self.mem.rb(W_CUR_PARTY_MON);
            let hp = self.get_party_location(W_OT_PARTY_MON1_HP, cur);
            let (hi, lo) = (self.mem.rb(hp), self.mem.rb(hp + 1));
            self.mem.wb(W_ENEMY_MON_HP + 1, lo);
            self.mem.wb(W_ENEMY_MON_HP, hi);
            // Make sure everything knows which monster the opponent is using
            self.mem.wb(W_CUR_OT_MON, cur);
            let status = self.mem.rb(hp - 2);
            self.mem.wb(W_ENEMY_MON_STATUS, status);
        } else if mode == 0 || !self.mem.bit(W_ENEMY_SUB_STATUS5, SUBSTATUS_TRANSFORMED) {
            // .TreeMon: if we're headbutting trees, some monsters enter battle asleep
            let status = if self.check_sleeping_tree_mon() { TREEMON_SLEEP_TURNS } else { 0 };
            self.mem.wb(W_ENEMY_MON_STATUS, status);
            self.mem.wb(W_ENEMY_MON_STATUS + 1, 0);
            // Full HP..
            let (hi, lo) = (self.mem.rb(W_ENEMY_MON_MAX_HP), self.mem.rb(W_ENEMY_MON_MAX_HP + 1));
            self.mem.wb(W_ENEMY_MON_HP, hi);
            self.mem.wb(W_ENEMY_MON_HP + 1, lo);
            // ..unless it's a RoamMon
            if self.mem.rb(W_BATTLE_TYPE) == BATTLETYPE_ROAMING {
                let hl = self.roam_mon_field(W_ROAM_MON1_HP);
                let a = self.mem.rb(hl);
                if a == 0 {
                    // .InitRoamHP: HP only uses the lo byte in the RoamMon struct
                    let hp = self.mem.rb(W_ENEMY_MON_HP + 1);
                    self.mem.wb(hl, hp);
                } else {
                    self.mem.wb(W_ENEMY_MON_HP + 1, a);
                }
            }
        }
        // .Moves
        let (t1, t2) = (self.mem.rb(W_BASE_TYPE1), self.mem.rb(W_BASE_TYPE2));
        self.mem.wb(W_ENEMY_MON_TYPE1, t1);
        self.mem.wb(W_ENEMY_MON_TYPE1 + 1, t2);
        // Get moves
        if mode == TRAINER_BATTLE {
            // Then copy moves from the party struct
            let cur = self.mem.rb(W_CUR_PARTY_MON);
            let src = self.get_party_location(W_OT_PARTY_MON1_MOVES, cur);
            self.mem.copy(W_ENEMY_MON_MOVES, src, u16::from(NUM_MOVES));
        } else {
            // .WildMoves: clear wEnemyMonMoves, then fill moves based on level
            self.mem.fill(W_ENEMY_MON_MOVES, u16::from(NUM_MOVES), 0);
            self.mem.wb(W_SKIP_MOVES_BEFORE_LEVEL_UP, 0);
            self.fill_moves(W_ENEMY_MON_MOVES);
        }
        // .PP
        if mode == TRAINER_BATTLE {
            let cur = self.mem.rb(W_CUR_PARTY_MON);
            let src = self.get_party_location(W_OT_PARTY_MON1_PP, cur);
            self.mem.copy(W_ENEMY_MON_PP, src, u16::from(NUM_MOVES));
        } else {
            self.fill_pp(W_ENEMY_MON_MOVES, W_ENEMY_MON_PP);
        }
        // .Finish: the first five base stats (the enemy mon's base Sp. Atk is also used
        // to calculate Sp. Def stat experience), the catch rate and the base exp
        self.mem.copy(W_ENEMY_MON_BASE_STATS, W_BASE_STATS, u16::from(NUM_STATS - 1));
        let base = W_ENEMY_MON_BASE_STATS + u16::from(NUM_STATS - 1);
        let v = self.mem.rb(W_BASE_CATCH_RATE);
        self.mem.wb(base, v);
        let v = self.mem.rb(W_BASE_EXP);
        self.mem.wb(base + 1, v);
        let species = self.mem.rb(W_TEMP_ENEMY_MON_SPECIES);
        self.mem.wb(W_NAMED_OBJECT_INDEX, species);
        self.get_pokemon_name();
        // Did we catch it?
        if mode == 0 {
            return;
        }
        // Update enemy nickname
        self.mem.copy(W_ENEMY_MON_NICKNAME, W_STRING_BUFFER1, MON_NAME_LENGTH_U16);
        // Saw this mon
        let species = self.mem.rb(W_TEMP_ENEMY_MON_SPECIES);
        self.small_far_flag_action(W_POKEDEX_SEEN, species.wrapping_sub(1), FlagAction::Set);
        self.mem.copy(W_ENEMY_STATS, W_ENEMY_MON_STATS, u16::from(NUM_BATTLE_STATS) * 2);
        // BUG: PRZ and BRN stat reductions don't apply to switched Pokémon (kept)
    }

    /// `LoadEnemyMon`'s `.InitDVs` .. `.Magikarp`: the trainer class DVs, or a wild
    /// mon's (roaming, forced shiny or random, with the Unown and Magikarp filters).
    fn init_enemy_mon_dvs(&mut self, mode: u8) {
        let (mut b, mut c) = self.get_trainer_dvs();
        if mode == WILD_BATTLE {
            let battle_type = self.mem.rb(W_BATTLE_TYPE);
            if battle_type == BATTLETYPE_ROAMING {
                // Roaming monsters (Entei, Raikou) have their own structs.
                let initialized = self.mem.rb(self.roam_mon_field(W_ROAM_MON1_HP)) != 0;
                let dvs = self.roam_mon_field(W_ROAM_MON1_D_VS);
                c = self.mem.rb(dvs + 1);
                b = self.mem.rb(dvs);
                if !initialized {
                    let r1 = self.battle_random("LoadEnemyMon");
                    self.mem.wb(dvs + 1, r1);
                    c = r1;
                    let r2 = self.battle_random("LoadEnemyMon");
                    self.mem.wb(dvs, r2);
                    b = r2;
                }
            } else if battle_type == BATTLETYPE_FORCESHINY {
                b = ATKDEFDV_SHINY;
                c = SPDSPCDV_SHINY;
            } else {
                // .GenerateDVs
                b = self.battle_random("LoadEnemyMon");
                c = self.battle_random("LoadEnemyMon");
            }
        }
        loop {
            // .UpdateDVs
            self.mem.wb(W_ENEMY_MON_D_VS, b);
            self.mem.wb(W_ENEMY_MON_D_VS + 1, c);
            if mode != WILD_BATTLE {
                return;
            }
            let regenerate = self.wild_dvs_rejected();
            if !regenerate {
                return;
            }
            // .GenerateDVs: try again
            b = self.battle_random("LoadEnemyMon");
            c = self.battle_random("LoadEnemyMon");
        }
    }

    /// `LoadEnemyMon`'s species-specific filters on a wild mon's DVs (`.Unown`,
    /// `.Magikarp`, `.CheckMagikarpArea`): true to roll new DVs.
    fn wild_dvs_rejected(&mut self) -> bool {
        let species = self.mem.rb(W_TEMP_ENEMY_MON_SPECIES);
        if species == UNOWN {
            // Can't use any letters that haven't been unlocked
            self.get_unown_letter(W_ENEMY_MON_D_VS);
            if self.check_unown_letter() {
                return true;
            }
        }
        if species != MAGIKARP {
            return false;
        }
        // BUG: Magikarp length limits have a unit conversion error (kept)
        self.calc_magikarp_length(W_ENEMY_MON_D_VS, W_PLAYER_ID);
        // No reason to keep going if length > 1536 mm (i.e. if HIGH(length) > 6 feet)
        if self.mem.rb(W_MAGIKARP_LENGTH) == 6 {
            // 5% chance of skipping both size checks
            if self.random("LoadEnemyMon") >= percent(5) {
                // Try again if length >= 1616 mm (i.e. if LOW(length) >= 4 inches)
                if self.mem.rb(W_MAGIKARP_LENGTH + 1) >= 1616u16 as u8 {
                    return true;
                }
                // 20% chance of skipping this check
                if self.random("LoadEnemyMon") >= percent(20) - 1 {
                    // Try again if length >= 1600 mm (i.e. if LOW(length) >= 3 inches)
                    if self.mem.rb(W_MAGIKARP_LENGTH + 1) >= 1600u16 as u8 {
                        return true;
                    }
                }
            }
        }
        // .CheckMagikarpArea
        // BUG: Magikarp in Lake of Rage are shorter, not longer (kept)
        if self.mem.rb(W_MAP_GROUP) == GROUP_LAKE_OF_RAGE {
            return false;
        }
        if self.mem.rb(W_MAP_NUMBER) == MAP_LAKE_OF_RAGE {
            return false;
        }
        // 40% chance of not flooring
        if self.random("LoadEnemyMon") < percent(39) + 1 {
            return false;
        }
        // Try again if length < 1024 mm (i.e. if HIGH(length) < 3 feet)
        self.mem.rb(W_MAGIKARP_LENGTH) < (1024u16 >> 8) as u8
    }

    /// `GetTrainerDVs`: the DVs of `wOtherTrainerClass` in `bc`.
    pub(super) fn get_trainer_dvs(&self) -> (u8, u8) {
        let [b, c] = self.data.trainer_class(self.mem.rb(W_OTHER_TRAINER_CLASS)).dvs;
        (b, c)
    }

    /// `GetRoamMonHP` / `GetRoamMonDVs`: the field at `first_mon_field` (a field of
    /// `wRoamMon1`) of the roaming mon matching `wTempEnemyMonSpecies`.
    fn roam_mon_field(&self, first_mon_field: u16) -> u16 {
        let species = self.mem.rb(W_TEMP_ENEMY_MON_SPECIES);
        let size = W_ROAM_MON2_HP - W_ROAM_MON1_HP;
        if self.mem.rb(W_ROAM_MON1_SPECIES) == species {
            first_mon_field
        } else if self.mem.rb(W_ROAM_MON1_SPECIES + size) == species {
            first_mon_field + size
        } else {
            first_mon_field + 2 * size
        }
    }

    /// `CheckSleepingTreeMon`: carry if the species sleeps in trees at this time of day.
    pub(super) fn check_sleeping_tree_mon(&self) -> bool {
        if self.mem.rb(W_BATTLE_TYPE) != BATTLETYPE_TREE {
            return false;
        }
        let tod = self.mem.rb(W_TIME_OF_DAY);
        let tables = self.data.tables;
        let list = if tod < DAY_F {
            tables.asleep_tree_mons_morn
        } else if tod == DAY_F {
            tables.asleep_tree_mons_day
        } else {
            tables.asleep_tree_mons_nite
        };
        list.contains(&self.mem.rb(W_TEMP_ENEMY_MON_SPECIES))
    }

    /// `CheckUnownLetter`: carry if `wUnownLetter` hasn't been unlocked yet.
    fn check_unown_letter(&self) -> bool {
        !self.unown_letter_unlocked(self.mem.rb(W_UNOWN_LETTER))
    }

    /// Whether a wild Unown's DV re-roll in `LoadEnemyMon` can ever end: some
    /// letter a DV roll gives (`GetUnownLetter`: 1..=`NUM_UNOWN`) is in an
    /// unlocked set. If none is, the original re-rolls forever.
    pub(super) fn any_unown_letter_unlocked(&self) -> bool {
        (1..=NUM_UNOWN).any(|letter| self.unown_letter_unlocked(letter))
    }

    /// `CheckUnownLetter`'s test: `letter` is in one of the sets
    /// `wUnlockedUnowns` has unlocked.
    fn unown_letter_unlocked(&self, letter: u8) -> bool {
        let mut c = self.mem.rb(W_UNLOCKED_UNOWNS);
        for set in self.data.tables.unlocked_unown_letter_sets.iter().take(usize::from(NUM_UNLOCKED_UNOWN_SETS)) {
            let unlocked = c & 1 != 0;
            c >>= 1;
            if unlocked && set.contains(&letter) {
                return true;
            }
        }
        // Hasn't been unlocked, or the letter is invalid
        false
    }

    /// `CalcMagikarpLength`: Magikarp's length (feet, inches) into `wMagikarpLength`
    /// from its DVs at `de` and the trainer ID at `bc`.
    fn calc_magikarp_length(&mut self, de: u16, bc: u16) {
        // bc = rrc(dv[0]) ++ rrc(dv[1]) ^ rrc(id)
        let b = self.mem.rb(bc).rotate_right(1);
        let c = self.mem.rb(bc + 1).rotate_right(1);
        let b = self.mem.rb(de).rotate_right(2) ^ b;
        let c = self.mem.rb(de + 1).rotate_right(2) ^ c;
        let mut bc16 = u16::from(b) << 8 | u16::from(c);
        let mm: u16;
        if b == 0 && c < 10 {
            mm = bc16.wrapping_add(190);
        } else {
            let mut counter = 2u8;
            self.mem.wb(W_TEMP_BYTE_VALUE, counter);
            let mut found = None;
            let mut last_de = 0u16;
            for &(de16, divisor) in self.data.tables.magikarp_lengths.iter().take(14) {
                last_de = de16;
                // BUG: Magikarp lengths can be miscalculated: only b is compared (kept)
                if (bc16 >> 8) as u8 >= (de16 >> 8) as u8 {
                    // .next
                    counter += 1;
                    self.mem.wb(W_TEMP_BYTE_VALUE, counter);
                    continue;
                }
                // c = (bc - de) / [hl]
                bc16 = bc16.wrapping_sub(de16);
                self.mem.wb(H_DIVIDEND, (bc16 >> 8) as u8);
                self.mem.wb(H_DIVIDEND + 1, bc16 as u8);
                self.mem.wb(H_DIVISOR, divisor);
                self.divide(2);
                let c = self.mem.rb(H_QUOTIENT + 3);
                // de = c + 100 × (2 + i)
                self.mem.wb(H_MULTIPLICAND, 0);
                self.mem.wb(H_MULTIPLICAND + 1, 0);
                self.mem.wb(H_MULTIPLICAND + 2, 100);
                self.mem.wb(H_MULTIPLIER, counter);
                self.multiply();
                let (e, carry) = self.mem.rb(H_PRODUCT + 3).overflowing_add(c);
                let (d, _) = adc(self.mem.rb(H_PRODUCT + 2), 0, carry);
                found = Some(u16::from(d) << 8 | u16::from(e));
                break;
            }
            mm = match found {
                Some(v) => v,
                None => bc16.wrapping_sub(last_de).wrapping_add(1600),
            };
        }
        // .done: convert from mm to feet and inches
        let hl = mm.wrapping_mul(10);
        let mut a = 0xffu8;
        let mut hl = hl;
        loop {
            a = a.wrapping_add(1);
            let (v, carry) = hl.overflowing_add(0u16.wrapping_sub(254));
            hl = v;
            if !carry {
                break;
            }
        }
        let mut d = 0u8;
        while a >= 12 {
            a -= 12;
            d += 1;
        }
        self.mem.wb(W_MAGIKARP_LENGTH, d);
        self.mem.wb(W_MAGIKARP_LENGTH + 1, a);
    }

    /// `FillMoves`: fill in the moves at `de` for `wCurPartySpecies` at `wCurPartyLevel`.
    pub fn fill_moves(&mut self, de0: u16) {
        let species = self.mem.rb(W_CUR_PARTY_SPECIES);
        'moves: for &(b, mv) in self.data.species(species).learnset {
            // .GetLevel
            if self.mem.rb(W_CUR_PARTY_LEVEL) < b {
                return;
            }
            if self.mem.rb(W_SKIP_MOVES_BEFORE_LEVEL_UP) != 0 && self.mem.rb(W_PREV_PARTY_LEVEL) >= b {
                // .GetMove
                continue;
            }
            // .CheckMove
            for i in 0..u16::from(NUM_MOVES) {
                if self.mem.rb(de0 + i) == mv {
                    // .NextMove
                    continue 'moves;
                }
            }
            let mut de = de0;
            let mut empty = false;
            for _ in 0..NUM_MOVES {
                if self.mem.rb(de) == 0 {
                    empty = true;
                    break;
                }
                de += 1;
            }
            if !empty {
                // All four slots are occupied: shift the moves down.
                self.shift_moves(de0);
                de = de0 + u16::from(NUM_MOVES) - 1;
                if self.mem.rb(W_EVOLUTION_OLD_SPECIES) != 0 {
                    self.shift_moves(de0 + u16::from(MON_PP - MON_MOVES));
                }
            }
            // .LearnMove
            self.mem.wb(de, mv);
            if self.mem.rb(W_EVOLUTION_OLD_SPECIES) != 0 {
                let pp = self.data.mv(mv).pp;
                self.mem.wb(de + u16::from(MON_PP - MON_MOVES), pp);
            }
            // .NextMove
        }
    }

    /// `ShiftMoves`: the three moves after `hl` move down one slot.
    fn shift_moves(&mut self, hl: u16) {
        for i in 0..u16::from(NUM_MOVES - 1) {
            let v = self.mem.rb(hl + i + 1);
            self.mem.wb(hl + i, v);
        }
    }

    /// `FillPP`: the PP of the moves at `hl` into `de`.
    pub fn fill_pp(&mut self, hl: u16, de: u16) {
        for i in 0..u16::from(NUM_MOVES) {
            let mv = self.mem.rb(hl + i);
            let mut a = mv;
            if mv != 0 {
                for (j, b) in (0..).zip(self.data.mv(mv).bytes()) {
                    self.mem.wb(W_STRING_BUFFER1 + j, b);
                }
                a = self.mem.rb(W_STRING_BUFFER1 + u16::from(MOVE_PP));
            }
            self.mem.wb(de + i, a);
        }
    }
}

// --- winning and losing ------------------------------------------------------------------

impl Engine {
    /// `WinTrainerBattle`.
    pub(super) fn win_trainer_battle(&mut self) {
        self.mem.wb(W_LOW_HEALTH_ALARM, 0);
        self.mem.wb(W_BATTLE_LOW_HEALTH_ALARM, 1);
        self.mem.wb(W_BATTLE_ENDED, 1);
        self.text(r::BATTLE_TEXT_ENEMY_WAS_DEFEATED);
        if self.link_mode() != 0 {
            return;
        }
        if self.mem.bit(W_IN_BATTLE_TOWER_BATTLE, IN_BATTLE_TOWER_BATTLE_F) {
            return;
        }
        if self.mem.rb(W_BATTLE_TYPE) == BATTLETYPE_CANLOSE {
            self.unported("WinTrainerBattle (BATTLETYPE_CANLOSE: HealParty)");
        }
        // PrintWinLossText: the harness prints nothing without a win text pointer.
        // .give_money
        if self.mem.rb(W_AMULET_COIN) != 0 {
            self.double_reward();
        }
        let mom_below_max = self.check_maxed_out_mom_money();
        // BUG (kept): `a` is FALSE only when Mom's money is maxed out; otherwise it is
        // left holding the masked `wMomSavingMoney` (TRUE = +1 only for "some + half").
        let mut b = 0u8;
        if mom_below_max {
            let saving = self.mem.rb(W_MOM_SAVING_MONEY) & MOM_SAVING_MONEY_MASK;
            b = saving;
            if saving == (1 << MOM_SAVING_SOME_MONEY_F) | (1 << MOM_SAVING_HALF_MONEY_F) {
                b += 1;
            }
        }
        let mut c = 4u8;
        while b != 0 {
            self.add_battle_money_to_account(W_BATTLE_REWARD + 2, W_MOMS_MONEY + 2);
            c = c.wrapping_sub(1);
            b -= 1;
        }
        while c != 0 {
            self.add_battle_money_to_account(W_BATTLE_REWARD + 2, W_MONEY + 2);
            c = c.wrapping_sub(1);
        }
        self.double_reward();
        self.double_reward();
        let saving = self.mem.rb(W_MOM_SAVING_MONEY) & MOM_SAVING_MONEY_MASK;
        if !mom_below_max || saving == 0 {
            self.text(r::GOT_MONEY_FOR_WINNING_TEXT);
            return;
        }
        let text = match saving {
            1 => r::SENT_SOME_TO_MOM_TEXT,
            2 => r::SENT_HALF_TO_MOM_TEXT,
            3 => r::SENT_ALL_TO_MOM_TEXT,
            _ => self.unported("WinTrainerBattle (.SentToMomTexts past its end)"),
        };
        self.text(text);
    }

    /// `WinTrainerBattle.DoubleReward`: `wBattleReward` *= 2, capped at 0xffffff.
    fn double_reward(&mut self) {
        let hl = W_BATTLE_REWARD + 2;
        let v = u32::from(self.mem.rb(W_BATTLE_REWARD)) << 16
            | u32::from(self.mem.rb(W_BATTLE_REWARD + 1)) << 8
            | u32::from(self.mem.rb(hl));
        let doubled = v << 1;
        self.mem.wb(hl, doubled as u8);
        self.mem.wb(hl - 1, (doubled >> 8) as u8);
        self.mem.wb(hl - 2, (doubled >> 16) as u8);
        if doubled >> 24 != 0 {
            self.mem.wb(W_BATTLE_REWARD, 0xff);
            self.mem.wb(W_BATTLE_REWARD + 1, 0xff);
            self.mem.wb(W_BATTLE_REWARD + 2, 0xff);
        }
    }

    /// `WinTrainerBattle.CheckMaxedOutMomMoney`: carry if Mom's money is below the maximum.
    fn check_maxed_out_mom_money(&self) -> bool {
        self.money_at(W_MOMS_MONEY) < MAX_MONEY
    }

    fn money_at(&self, addr: u16) -> u32 {
        u32::from(self.mem.rb(addr)) << 16 | u32::from(self.mem.rb(addr + 1)) << 8 | u32::from(self.mem.rb(addr + 2))
    }

    /// `AddBattleMoneyToAccount`: add the 3-byte amount ending at `hl` to the account
    /// ending at `de`, capped at `MAX_MONEY`.
    fn add_battle_money_to_account(&mut self, hl: u16, de: u16) {
        let mut carry = false;
        for i in 0..3u16 {
            let (v, c) = adc(self.mem.rb(de - i), self.mem.rb(hl - i), carry);
            self.mem.wb(de - i, v);
            carry = c;
        }
        let account = de - 2;
        if self.money_at(account) >= MAX_MONEY {
            self.mem.wb(account, (MAX_MONEY >> 16) as u8);
            self.mem.wb(account + 1, (MAX_MONEY >> 8) as u8);
            self.mem.wb(account + 2, MAX_MONEY as u8);
        }
    }

    /// `LostBattle`.
    pub(super) fn lost_battle(&mut self) {
        self.mem.wb(W_BATTLE_ENDED, 1);
        if self.mem.bit(W_IN_BATTLE_TOWER_BATTLE, IN_BATTLE_TOWER_BATTLE_F) {
            // .battle_tower: the screen and the Battle Tower text only
            return;
        }
        if self.mem.rb(W_BATTLE_TYPE) == BATTLETYPE_CANLOSE {
            // The enemy slides in, PrintWinLossText (nothing without a pointer).
            return;
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
}
