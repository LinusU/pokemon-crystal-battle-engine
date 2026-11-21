//! `engine/battle/ai/switch.asm`: how the enemy scores a potential switch.
//!
//! The helpers that pass their result in registers return them: `c` is the
//! bit mask of party mons (bit 5 = first mon ... bit 0 = sixth), `e` the
//! best move class found by `FindEnemyMonsWithASuperEffectiveMove`.

#![allow(unused_imports)]

use super::battle::{Engine, PARTY_LEN};
use super::rsym as r;
use super::sym::*;
use crate::defs::*;

/// `1 << (PARTY_LENGTH - 1)`.
const FIRST_MON_BIT: u8 = 1 << (PARTY_LENGTH - 1);

/// `sla`: returns (result, carry).
fn sla(v: u8) -> (u8, bool) {
    (v << 1, v & 0x80 != 0)
}

impl Engine {
    /// `GetMoveAttr` + `GetMoveByte`: attribute at offset `off` of move `mv`
    /// (the original is handed `mv - 1`, wrapping for 0).
    fn ai_move_attr(&self, mv: u8, off: u8) -> u8 {
        self.data.mv(mv).field(off)
    }

    /// `CheckPlayerMoveTypeMatchups`: check how well the moves you've already
    /// used fare against the enemy's Pokemon. Used to score a potential switch.
    pub fn check_player_move_type_matchups(&mut self) {
        self.mem.wb(W_ENEMY_AI_SWITCH_SCORE, BASE_AI_SWITCH_SCORE);
        let mut hl = W_PLAYER_USED_MOVES;
        if self.mem.rb(hl) == 0 {
            // .unknown_moves
            let b = self.mem.rb(W_BATTLE_MON_TYPE1);
            self.ai_check_type_matchup(b, W_ENEMY_MON_TYPE1);
            if self.mem.rb(W_TYPE_MATCHUP) >= EFFECTIVE + 1 {
                self.ai_switch_score(-1);
            }
            // .ok
            let type2 = self.mem.rb(W_BATTLE_MON_TYPE2);
            if type2 != b {
                self.ai_check_type_matchup(type2, W_ENEMY_MON_TYPE1);
                if self.mem.rb(W_TYPE_MATCHUP) >= EFFECTIVE + 1 {
                    self.ai_switch_score(-1);
                }
            }
        } else {
            let mut d = NUM_MOVES;
            let mut e = 0u8;
            // 0 = fell out of the loop (.exit), 1 = found a super-effective move (.done)
            let mut super_effective = false;
            loop {
                let a = self.mem.rb(hl);
                hl += 1;
                if a == 0 {
                    break;
                }
                if self.ai_move_attr(a, MOVE_POWER) != 0 {
                    self.ai_check_type_matchup(self.ai_move_attr(a, MOVE_TYPE), W_ENEMY_MON_TYPE1);
                    let m = self.mem.rb(W_TYPE_MATCHUP);
                    if m >= EFFECTIVE + 1 {
                        // .super_effective
                        self.ai_switch_score(-1);
                        super_effective = true;
                        break;
                    }
                    if m == 0 {
                        // immune: nothing
                    } else if m >= EFFECTIVE {
                        // .neutral
                        e = 2;
                    } else if e < 1 {
                        // not very effective
                        e = 1;
                    }
                }
                // .next
                d -= 1;
                if d == 0 {
                    break;
                }
            }
            if !super_effective && e != 2 {
                // .exit
                self.ai_switch_score(1);
                if e == 0 {
                    self.ai_switch_score(1);
                }
            }
        }
        // .done
        self.check_enemy_move_matchups();
    }

    /// `CheckPlayerMoveTypeMatchups.DecreaseScore` / `.IncreaseScore`.
    fn ai_switch_score(&mut self, delta: i8) {
        let v = self.mem.rb(W_ENEMY_AI_SWITCH_SCORE).wrapping_add(delta as u8);
        self.mem.wb(W_ENEMY_AI_SWITCH_SCORE, v);
    }

    /// `CheckPlayerMoveTypeMatchups.CheckEnemyMoveMatchups`.
    fn check_enemy_move_matchups(&mut self) {
        let mut de = W_ENEMY_MON_MOVES;
        let mut b = NUM_MOVES + 1;
        let mut c = 0u8;
        let saved = self.mem.rb(W_TYPE_MATCHUP);
        loop {
            b -= 1;
            if b == 0 {
                break;
            }
            let a = self.mem.rb(de);
            if a == 0 {
                break;
            }
            de += 1;
            if self.ai_move_attr(a, MOVE_POWER) == 0 {
                continue;
            }
            self.ai_check_type_matchup(self.ai_move_attr(a, MOVE_TYPE), W_BATTLE_MON_TYPE1);
            let m = self.mem.rb(W_TYPE_MATCHUP);
            // immune
            if m == 0 {
                continue;
            }
            // not very effective
            c = c.wrapping_add(1);
            if m < EFFECTIVE {
                continue;
            }
            // neutral
            c = c.wrapping_add(5);
            if m == EFFECTIVE {
                continue;
            }
            // super effective
            c = 100;
        }
        // .exit2
        self.mem.wb(W_TYPE_MATCHUP, saved);
        if c == 0 {
            // .doubledown
            self.ai_switch_score(-1);
            self.ai_switch_score(-1);
        } else if c < 5 {
            self.ai_switch_score(-1);
        } else if c >= 100 {
            self.ai_switch_score(1);
        }
    }

    /// `CheckAbleToSwitch`.
    pub fn check_able_to_switch(&mut self) {
        self.mem.wb(W_ENEMY_SWITCH_MON_PARAM, 0);
        let (carry, _) = self.find_alive_enemy_mons();
        if carry {
            return;
        }

        if self.mem.bit(W_ENEMY_SUB_STATUS1, SUBSTATUS_PERISH) && self.mem.rb(W_ENEMY_PERISH_COUNT) == 1 {
            // Perish count is 1.
            let (_, c) = self.find_alive_enemy_mons();
            let c = self.find_enemy_mons_with_at_least_quarter_max_hp(c);
            let c = self.find_enemy_mons_that_resist_player(c);
            let e = self.find_alive_enemy_mons_with_a_super_effective_move(c);

            if e == 2 {
                let a = self.mem.rb(W_ENEMY_AI_SWITCH_SCORE).wrapping_add(0x30); // maximum chance
                self.mem.wb(W_ENEMY_SWITCH_MON_PARAM, a);
                return;
            }
            // .not_2
            let (_, c) = self.find_alive_enemy_mons();
            let (mut c, _) = sla(c);
            c = sla(c).0;
            let mut b = 0xffu8;
            loop {
                b = b.wrapping_add(1);
                let (nc, carry) = sla(c);
                c = nc;
                if carry {
                    break;
                }
            }
            self.mem.wb(W_ENEMY_SWITCH_MON_PARAM, b.wrapping_add(0x30)); // maximum chance
            return;
        }

        // .no_perish
        self.check_player_move_type_matchups();
        if self.mem.rb(W_ENEMY_AI_SWITCH_SCORE) >= 11 {
            return;
        }

        if self.mem.rb(W_LAST_PLAYER_COUNTER_MOVE) != 0 {
            self.find_enemy_mons_immune_to_last_counter_move();
            let score = self.mem.rb(W_ENEMY_AI_SWITCH_SCORE);
            if score != 0 {
                let c = score;
                let e = self.find_enemy_mons_with_a_super_effective_move(c);
                let score = self.mem.rb(W_ENEMY_AI_SWITCH_SCORE);
                if score == 0xff {
                    return;
                }
                let b = score;
                if e == 2 {
                    // .not_2_again
                    let mut c = 0x10u8;
                    self.check_player_move_type_matchups();
                    if self.mem.rb(W_ENEMY_AI_SWITCH_SCORE) < BASE_AI_SWITCH_SCORE {
                        c = 0x20;
                    }
                    // .okay
                    self.mem.wb(W_ENEMY_SWITCH_MON_PARAM, b.wrapping_add(c));
                    return;
                }
                self.check_player_move_type_matchups();
                if self.mem.rb(W_ENEMY_AI_SWITCH_SCORE) >= BASE_AI_SWITCH_SCORE {
                    return;
                }
                self.mem.wb(W_ENEMY_SWITCH_MON_PARAM, b.wrapping_add(0x10));
                return;
            }
        }

        // .no_last_counter_move
        self.check_player_move_type_matchups();
        if self.mem.rb(W_ENEMY_AI_SWITCH_SCORE) >= BASE_AI_SWITCH_SCORE {
            return;
        }
        let (_, c) = self.find_alive_enemy_mons();
        let c = self.find_enemy_mons_with_at_least_quarter_max_hp(c);
        let c = self.find_enemy_mons_that_resist_player(c);
        let e = self.find_alive_enemy_mons_with_a_super_effective_move(c);
        if e != 2 {
            return;
        }
        let a = self.mem.rb(W_ENEMY_AI_SWITCH_SCORE).wrapping_add(0x10);
        self.mem.wb(W_ENEMY_SWITCH_MON_PARAM, a);
    }

    /// `FindAliveEnemyMons`: returns (carry = only one mon, so no switch is
    /// possible; `c` = bit mask of the other conscious mons).
    pub fn find_alive_enemy_mons(&mut self) -> (bool, u8) {
        let count = self.mem.rb(W_OT_PARTY_COUNT);
        if count < 2 {
            return (true, 0);
        }
        let mut b = FIRST_MON_BIT;
        let mut c = 0u8;
        let mut hl = W_OT_PARTY_MON1_HP;
        for e in 0..count {
            if self.mem.rb(W_CUR_OT_MON) != e && (self.mem.rb(hl) | self.mem.rb(hl + 1)) != 0 {
                c |= b;
            }
            b >>= 1;
            hl += PARTY_LEN;
        }
        (c == 0, c)
    }

    /// `FindEnemyMonsImmuneToLastCounterMove`: sets `wEnemyAISwitchScore` to
    /// the mask of mons that are immune to the player's last move.
    fn find_enemy_mons_immune_to_last_counter_move(&mut self) {
        let mut hl = W_OT_PARTY_MON1;
        let mut b = self.mem.rb(W_OT_PARTY_COUNT);
        let mut c = FIRST_MON_BIT;
        let mut d = 0u8;
        self.mem.wb(W_ENEMY_AI_SWITCH_SCORE, 0);
        loop {
            if self.mem.rb(W_CUR_OT_MON) != d {
                // If the Pokemon has at least 1 HP...
                let hp = hl + u16::from(MON_HP);
                if (self.mem.rb(hp) | self.mem.rb(hp + 1)) != 0 {
                    let species = self.mem.rb(hl);
                    self.mem.wb(W_CUR_SPECIES, species);
                    self.get_base_data();
                    // the player's last move is damaging...
                    let last = self.mem.rb(W_LAST_PLAYER_COUNTER_MOVE);
                    if self.ai_move_attr(last, MOVE_POWER) != 0 {
                        // and the Pokemon is immune to it...
                        self.ai_check_type_matchup(self.ai_move_attr(last, MOVE_TYPE), W_BASE_TYPE);
                        if self.mem.rb(W_TYPE_MATCHUP) == 0 {
                            // ... encourage that Pokemon.
                            let s = self.mem.rb(W_ENEMY_AI_SWITCH_SCORE) | c;
                            self.mem.wb(W_ENEMY_AI_SWITCH_SCORE, s);
                        }
                    }
                }
            }
            // .next
            b -= 1;
            if b == 0 {
                return;
            }
            hl += PARTY_LEN;
            d += 1;
            c >>= 1;
        }
    }

    /// `FindAliveEnemyMonsWithASuperEffectiveMove`: restrict `c` to the
    /// conscious mons (this includes the active one), then see which have a
    /// super-effective move. Returns `e`.
    fn find_alive_enemy_mons_with_a_super_effective_move(&mut self, c_in: u8) -> u8 {
        let mut e = self.mem.rb(W_OT_PARTY_COUNT);
        let mut hl = W_OT_PARTY_MON1_HP;
        let mut b = FIRST_MON_BIT;
        let mut c = 0u8;
        loop {
            if (self.mem.rb(hl) | self.mem.rb(hl + 1)) != 0 {
                c |= b;
            }
            b >>= 1;
            hl += PARTY_LEN;
            e -= 1;
            if e == 0 {
                break;
            }
        }
        self.find_enemy_mons_with_a_super_effective_move(c & c_in)
    }

    /// `FindEnemyMonsWithASuperEffectiveMove`: `c` = mask of mons to look at.
    /// Sets `wEnemyAISwitchScore` to the index of a mon with a super-effective
    /// move (or `$ff`). Returns `e` (2 = a super-effective move was found).
    fn find_enemy_mons_with_a_super_effective_move(&mut self, c: u8) -> u8 {
        self.mem.wb(W_ENEMY_AI_SWITCH_SCORE, 0xff);
        let mut hl = W_OT_PARTY_MON1_MOVES;
        let mut b = FIRST_MON_BIT;
        let mut d = 0u8;
        let mut e = 0u8;
        loop {
            if b & c != 0 {
                // for move on mon:
                let mut mp = hl;
                for _ in 0..NUM_MOVES {
                    let mv = self.mem.rb(mp);
                    mp += 1;
                    // if move is None: break
                    if mv == 0 {
                        break;
                    }
                    // if move has no power: continue
                    if self.ai_move_attr(mv, MOVE_POWER) == 0 {
                        continue;
                    }
                    // check type matchups
                    self.ai_check_type_matchup(self.ai_move_attr(mv, MOVE_TYPE), W_BATTLE_MON_TYPE1);
                    let m = self.mem.rb(W_TYPE_MATCHUP);
                    // if immune or not very effective: continue
                    if m < EFFECTIVE {
                        continue;
                    }
                    // if neutral: load 1 and continue
                    e = 1;
                    if m < EFFECTIVE + 1 {
                        continue;
                    }
                    // if super-effective: load 2 and break
                    e = 2;
                    break;
                }
                if e == 2 {
                    // at least one move is super-effective
                    return self.ai_super_effective_index(b, e);
                }
                if e == 1 {
                    // encourage this pokemon
                    d |= b;
                }
                // (otherwise no move does more than half damage)
            }
            // .next
            hl += PARTY_LEN;
            let carry = b & 1 != 0;
            b >>= 1;
            if carry {
                break;
            }
        }
        // if no pokemon has a super-effective move: return
        if d == 0 {
            return e;
        }
        self.ai_super_effective_index(d, e)
    }

    /// `.done2`: convert the bit flag `b` to an int in `wEnemyAISwitchScore`.
    fn ai_super_effective_index(&mut self, b: u8, e: u8) -> u8 {
        let (b, _) = sla(b);
        let (mut b, _) = sla(b);
        let mut c = 0xffu8;
        loop {
            c = c.wrapping_add(1);
            let (nb, carry) = sla(b);
            b = nb;
            if carry {
                break;
            }
        }
        self.mem.wb(W_ENEMY_AI_SWITCH_SCORE, c);
        e
    }

    /// `FindEnemyMonsThatResistPlayer`: `c` &= mask of mons that resist the
    /// player's last move (or the player's types if it has none).
    fn find_enemy_mons_that_resist_player(&mut self, c_in: u8) -> u8 {
        let mut hl = W_OT_PARTY_SPECIES;
        let mut b = FIRST_MON_BIT;
        let mut c = 0u8;
        loop {
            let species = self.mem.rb(hl);
            hl += 1;
            if species == 0xff {
                break;
            }
            self.mem.wb(W_CUR_SPECIES, species);
            self.get_base_data();
            let last = self.mem.rb(W_LAST_PLAYER_COUNTER_MOVE);
            let mut choose = true;
            // The type `.check_type` checks: the player's second type after
            // `.skip_move` (which checks the first), or the last move's.
            let check_type;
            if last == 0 || self.ai_move_attr(last, MOVE_POWER) == 0 {
                // .skip_move: the type matchup is checked twice; with the original's
                // CheckTypeMatchup ignoring its type argument both give the same result.
                self.ai_check_type_matchup(self.mem.rb(W_BATTLE_MON_TYPE1), W_BASE_TYPE);
                if self.mem.rb(W_TYPE_MATCHUP) >= EFFECTIVE + 1 {
                    choose = false;
                }
                check_type = self.mem.rb(W_BATTLE_MON_TYPE2);
            } else {
                check_type = self.ai_move_attr(last, MOVE_TYPE);
            }
            if choose {
                // .check_type
                self.ai_check_type_matchup(check_type, W_BASE_TYPE);
                if self.mem.rb(W_TYPE_MATCHUP) >= EFFECTIVE + 1 {
                    choose = false;
                }
            }
            if choose {
                c |= b;
            }
            // .dont_choose_mon
            b >>= 1;
        }
        c & c_in
    }

    /// `FindEnemyMonsWithAtLeastQuarterMaxHP`: `c` &= mask of mons that pass
    /// the (quirky) HP check.
    fn find_enemy_mons_with_at_least_quarter_max_hp(&mut self, c_in: u8) -> u8 {
        let mut de = W_OT_PARTY_SPECIES;
        let mut b = FIRST_MON_BIT;
        let mut c = 0u8;
        let mut hl = W_OT_PARTY_MON1_HP;
        loop {
            let species = self.mem.rb(de);
            de += 1;
            if species == 0xff {
                break;
            }
            let mut hb = self.mem.rb(hl);
            let mut hc = self.mem.rb(hl + 1);
            let max = self.mem.rw(hl + 2);
            // hl = MaxHP + 1; bc = [CurHP] * 4 (srl c / rl b, twice)
            for _ in 0..2 {
                let carry = hc & 1;
                hc >>= 1;
                hb = hb << 1 | carry;
            }
            // if bc >= [hl], encourage
            let bc = u16::from(hb) << 8 | u16::from(hc);
            if max < bc {
                c |= b;
            }
            // .next
            b >>= 1;
            hl += PARTY_LEN;
        }
        c & c_in
    }
}
