//! What the network sees: a battle state from one side's point of view, and
//! each legal action, as sparse feature vectors.
//!
//! Everything is read from the [`BattleState`] the library hands out (the
//! original's WRAM at a decision point) plus the game data in the ROM image
//! (move, base-stat and type-chart tables), so the encoding is the same for
//! [`crate::battle::Battle`], [`crate::battle::EmulatedBattle`] and the
//! training loop. The view is always "me" (the side deciding) against "foe":
//! the same network plays the player and the enemy trainer.
//!
//! The vectors are sparse (ids are one-hot over 256 values: species, moves,
//! move effects, items), which keeps the first layer cheap: a typical state
//! has a few hundred non-zero entries out of [`OBS_DIM`].

use crate::battle::state::{BattleMon, PartyMon, SideState};
use crate::battle::{Action, BattleState, PromptSite, Side};
use crate::defs;
use crate::engine::{rom, rsym as r};
use std::sync::OnceLock;

/// Bump when the encoding changes: saved models record it and refuse to load
/// against a different encoding.
pub const FEATURES_VERSION: u32 = 1;

/// A sparse vector: parallel index / value lists.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Sparse {
    pub idx: Vec<u32>,
    pub val: Vec<f32>,
}

/// Builds a [`Sparse`] in fixed-width fields. Every encoder advances the
/// cursor by the same amount whatever the values, so a feature always lands
/// at the same index.
struct Enc {
    out: Sparse,
    at: u32,
}

impl Enc {
    fn new() -> Enc {
        Enc { out: Sparse { idx: Vec::with_capacity(512), val: Vec::with_capacity(512) }, at: 0 }
    }

    /// One scalar.
    fn f(&mut self, v: f32) {
        if v != 0.0 {
            self.out.idx.push(self.at);
            self.out.val.push(v);
        }
        self.at += 1;
    }

    fn b(&mut self, v: bool) {
        self.f(if v { 1.0 } else { 0.0 });
    }

    /// A one-hot field of `width` (nothing set when `i` is `None` or out of range).
    fn one_hot(&mut self, width: u32, i: Option<usize>) {
        if let Some(i) = i.filter(|&i| (i as u32) < width) {
            self.out.idx.push(self.at + i as u32);
            self.out.val.push(1.0);
        }
        self.at += width;
    }

    /// A multi-hot field of `width`.
    fn multi_hot(&mut self, width: u32, set: impl IntoIterator<Item = usize>) {
        let start = self.out.idx.len();
        for i in set {
            if (i as u32) < width && !self.out.idx[start..].contains(&(self.at + i as u32)) {
                self.out.idx.push(self.at + i as u32);
                self.out.val.push(1.0);
            }
        }
        self.at += width;
    }

    /// Skip `width` all-zero features.
    fn skip(&mut self, width: u32) {
        self.at += width;
    }

    fn finish(self, width: u32) -> Sparse {
        assert_eq!(self.at, width, "feature layout drifted");
        self.out
    }
}

// --- game data -----------------------------------------------------------------

/// Number of type slots: the 18 real types plus `CURSE_TYPE` (`???`).
const TYPES: u32 = 19;

/// Dense index of a type byte (`NORMAL`..`STEEL` are 0..=9, `CURSE_TYPE`..`DARK`
/// are 19..=27).
fn type_index(t: u8) -> Option<usize> {
    match t {
        0..=9 => Some(t as usize),
        19..=27 => Some(t as usize - 9),
        _ => None,
    }
}

/// One entry of the `Moves` table.
#[derive(Debug, Clone, Copy, Default)]
pub struct MoveData {
    pub effect: u8,
    pub power: u8,
    pub kind: u8,
    pub accuracy: u8,
    pub pp: u8,
    pub chance: u8,
}

pub fn move_data(id: u8) -> MoveData {
    if id == 0 || id > defs::NUM_ATTACKS {
        return MoveData::default();
    }
    let at = r::MOVES + usize::from(id - 1) * usize::from(defs::MOVE_LENGTH);
    MoveData {
        effect: rom::byte(at + usize::from(defs::MOVE_EFFECT)),
        power: rom::byte(at + usize::from(defs::MOVE_POWER)),
        kind: rom::byte(at + usize::from(defs::MOVE_TYPE)),
        accuracy: rom::byte(at + usize::from(defs::MOVE_ACC)),
        pp: rom::byte(at + usize::from(defs::MOVE_PP)),
        chance: rom::byte(at + usize::from(defs::MOVE_CHANCE)),
    }
}

/// A species' two types (`BaseData`).
pub fn species_types(species: u8) -> [u8; 2] {
    if species == 0 || species > defs::NUM_POKEMON {
        return [0xff, 0xff];
    }
    let at = r::BASE_DATA + usize::from(species - 1) * usize::from(defs::BASE_DATA_SIZE) + usize::from(defs::BASE_TYPES);
    [rom::byte(at), rom::byte(at + 1)]
}

/// `TypeMatchups` as a table: effectiveness ×10 (0, 5, 10, 20) of an
/// attacking type against one defending type.
fn type_chart() -> &'static [[u8; TYPES as usize]; TYPES as usize] {
    static CHART: OnceLock<[[u8; TYPES as usize]; TYPES as usize]> = OnceLock::new();
    CHART.get_or_init(|| {
        let mut chart = [[defs::EFFECTIVE; TYPES as usize]; TYPES as usize];
        let mut at = r::TYPE_MATCHUPS;
        loop {
            let atk = rom::byte(at);
            if atk == 0xff {
                break;
            }
            if atk == 0xfe {
                // The Foresight marker: the entries after it still apply.
                at += 1;
                continue;
            }
            let (def, eff) = (rom::byte(at + 1), rom::byte(at + 2));
            if let (Some(a), Some(d)) = (type_index(atk), type_index(def)) {
                chart[a][d] = eff;
            }
            at += 3;
        }
        chart
    })
}

/// Effectiveness multiplier of a move type against a pair of defending types.
pub fn effectiveness(move_type: u8, def: [u8; 2]) -> f32 {
    let Some(a) = type_index(move_type) else { return 1.0 };
    let chart = type_chart();
    let mut m = 1.0;
    for (i, &t) in def.iter().enumerate() {
        if i == 1 && t == def[0] {
            break;
        }
        if let Some(d) = type_index(t) {
            m *= f32::from(chart[a][d]) / 10.0;
        }
    }
    m
}

/// A rough damage estimate (the core of `BattleCommand_DamageCalc`, average
/// roll, no crits or items) of `mv` from an attacker with `atk_stats` /
/// `atk_types` at `level` on a defender, as a fraction of `def_hp`.
fn damage_fraction(mv: u8, level: u8, atk_stats: &[u16; 5], atk_types: [u8; 2], def_stats: &[u16; 5], def_types: [u8; 2], def_hp: u16) -> f32 {
    let data = move_data(mv);
    if data.power <= 1 || def_hp == 0 {
        return 0.0;
    }
    let special = data.kind >= defs::SPECIAL;
    let (a, d) = if special { (atk_stats[3], def_stats[4]) } else { (atk_stats[0], def_stats[1]) };
    let base = (2.0 * f32::from(level) / 5.0 + 2.0) * f32::from(data.power) * f32::from(a) / f32::from(d.max(1)) / 50.0 + 2.0;
    let stab = if atk_types.contains(&data.kind) { 1.5 } else { 1.0 };
    let dmg = base * stab * effectiveness(data.kind, def_types) * 0.925;
    (dmg / f32::from(def_hp)).min(3.0)
}

// --- the observation ---------------------------------------------------------

/// Per move slot: flags, ids, data, and the matchup against the other active mon.
const MOVE_W: u32 = 4 + 256 + 256 + TYPES + 6 + 4;
/// The active mon.
const ACTIVE_W: u32 = 256 + 3 + 5 + TYPES + 6 + 256 + 7 + 40 + 14 + 7 + 1 + 4 * MOVE_W;
/// One party slot.
const PARTY_W: u32 = 3 + 2 + 5 + TYPES + 5 + TYPES + 2;
/// One side.
const SIDE_W: u32 = ACTIVE_W + 6 * PARTY_W + 2;
/// Global features: the prompt site, battle kind, weather.
const GLOBAL_W: u32 = 8 + 2 + 4 + 1;

/// Width of an observation vector.
pub const OBS_DIM: u32 = GLOBAL_W + 2 * SIDE_W;

fn site_index(site: PromptSite) -> usize {
    match site {
        PromptSite::BattleMenu => 0,
        PromptSite::MoveSelection => 1,
        PromptSite::ForcedSwitch => 2,
        PromptSite::OfferSwitch => 3,
        PromptSite::EnemyAction => 4,
        PromptSite::EnemyTrainerAction => 5,
        PromptSite::EnemySwitchIn => 6,
    }
}

/// `(me, foe)` for the side deciding.
pub fn sides(state: &BattleState, side: Side) -> (&SideState, &SideState) {
    match side {
        Side::Player => (&state.player, &state.enemy),
        Side::Enemy => (&state.enemy, &state.player),
    }
}

fn status_bits(e: &mut Enc, status: u8) {
    let sleep = status & defs::SLP_MASK;
    e.b(sleep != 0);
    e.b(status & 1 << defs::PSN != 0);
    e.b(status & 1 << defs::BRN != 0);
    e.b(status & 1 << defs::FRZ != 0);
    e.b(status & 1 << defs::PAR != 0);
}

fn hp_frac(hp: u16, max: u16) -> f32 {
    if max == 0 { 0.0 } else { f32::from(hp) / f32::from(max) }
}

/// The active mon's types: the in-battle types (Conversion changes them).
fn active_types(mon: &BattleMon) -> [u8; 2] {
    mon.types
}

/// One move slot of an active mon (`user`), against the other active mon.
fn encode_move(e: &mut Enc, user: &SideState, foe: &SideState, slot: usize) {
    let mon = &user.active;
    let mv = mon.moves[slot];
    if mv == 0 {
        e.skip(MOVE_W);
        return;
    }
    let data = move_data(mv);
    let pp = mon.pp[slot] & defs::PP_MASK;
    let disabled = (user.disable_count >> 4) as usize == slot + 1;
    e.b(true);
    e.f(f32::from(pp) / 64.0);
    e.b(pp == 0);
    e.b(disabled);
    e.one_hot(256, Some(mv as usize));
    e.one_hot(256, Some(data.effect as usize));
    e.one_hot(TYPES, type_index(data.kind));
    e.f(f32::from(data.power) / 200.0);
    e.f(f32::from(data.accuracy) / 255.0);
    e.f(f32::from(data.chance) / 255.0);
    e.b(data.kind >= defs::SPECIAL);
    e.f(f32::from(data.pp) / 40.0);
    e.b(data.power > 0);
    // Matchup against the foe's active mon.
    let eff = effectiveness(data.kind, active_types(&foe.active));
    e.f(if data.power > 0 { eff.max(0.125).log2() / 2.0 } else { 0.0 });
    e.b(data.power > 0 && eff == 0.0);
    e.b(active_types(mon).contains(&data.kind));
    e.f(damage_fraction(mv, mon.level, &mon.stats, active_types(mon), &foe.active.stats, active_types(&foe.active), foe.active.hp));
}

fn encode_active(e: &mut Enc, me: &SideState, foe: &SideState) {
    let mon = &me.active;
    e.one_hot(256, Some(mon.species as usize));
    e.f(hp_frac(mon.hp, mon.max_hp));
    e.f(f32::from(mon.hp) / 400.0);
    e.f(f32::from(mon.level) / 100.0);
    for s in mon.stats {
        e.f(f32::from(s) / 400.0);
    }
    e.multi_hot(TYPES, active_types(mon).iter().filter_map(|&t| type_index(t)));
    status_bits(e, mon.status);
    e.f(f32::from(mon.status & defs::SLP_MASK) / 7.0);
    e.one_hot(256, Some(mon.item as usize).filter(|&i| i != 0));
    for l in me.stat_levels {
        e.f((f32::from(l) - 7.0) / 6.0);
    }
    for byte in me.substatus {
        for bit in 0..8 {
            e.b(byte & 1 << bit != 0);
        }
    }
    e.f(f32::from(me.rollout_count) / 5.0);
    e.f(f32::from(me.confuse_count) / 5.0);
    e.f(f32::from(me.toxic_count) / 15.0);
    e.f(f32::from(me.disable_count & 0xf) / 8.0);
    e.f(f32::from(me.encore_count) / 8.0);
    e.f(f32::from(me.perish_count) / 4.0);
    e.f(f32::from(me.fury_cutter_count) / 5.0);
    e.f(f32::from(me.protect_count) / 4.0);
    e.f(if mon.max_hp == 0 { 0.0 } else { f32::from(me.substitute_hp) / f32::from(mon.max_hp) });
    e.f(f32::from(me.wrap_count) / 6.0);
    e.b(me.charging != 0);
    e.f(f32::from(me.rage_counter) / 8.0);
    e.f(f32::from(me.future_sight_count) / 3.0);
    e.f(f32::from(me.damage_taken.min(999)) / 400.0);
    e.b(me.screens & 1 != 0);
    e.b(me.screens & 1 << 2 != 0);
    e.b(me.screens & 1 << 3 != 0);
    e.b(me.screens & 1 << 4 != 0);
    e.f(f32::from(me.safeguard_count) / 5.0);
    e.f(f32::from(me.light_screen_count) / 5.0);
    e.f(f32::from(me.reflect_count) / 5.0);
    e.f(f32::from(me.turns_taken.min(20)) / 20.0);
    for slot in 0..4 {
        encode_move(e, me, foe, slot);
    }
}

/// One party mon (`slot` of `side`), with how it matches up against `foe`'s
/// active mon.
fn encode_party_mon(e: &mut Enc, side: &SideState, slot: usize, foe: &SideState) {
    let Some(mon) = side.party.get(slot) else {
        e.skip(PARTY_W);
        return;
    };
    e.b(true);
    e.b(mon.hp > 0);
    e.b(side.active_slot as usize == slot);
    e.f(hp_frac(mon.hp, mon.max_hp));
    e.f(f32::from(mon.level) / 100.0);
    status_bits(e, mon.status);
    let types = species_types(mon.species);
    e.multi_hot(TYPES, types.iter().filter_map(|&t| type_index(t)));
    for s in mon.stats {
        e.f(f32::from(s) / 400.0);
    }
    e.multi_hot(TYPES, mon.moves.iter().filter(|&&m| m != 0).filter_map(|&m| type_index(move_data(m).kind)));
    let (offence, defence) = party_matchup(mon, foe);
    e.f(offence);
    e.f(defence);
}

/// The best damage fraction `mon` deals to `foe`'s active mon, and the best
/// the foe's active mon deals to it.
fn party_matchup(mon: &PartyMon, foe: &SideState) -> (f32, f32) {
    let types = species_types(mon.species);
    let f = &foe.active;
    let offence = mon
        .moves
        .iter()
        .map(|&m| damage_fraction(m, mon.level, &mon.stats, types, &f.stats, active_types(f), f.hp))
        .fold(0.0, f32::max);
    let defence = f
        .moves
        .iter()
        .map(|&m| damage_fraction(m, f.level, &f.stats, active_types(f), &mon.stats, types, mon.hp))
        .fold(0.0, f32::max);
    (offence.min(3.0) / 3.0, defence.min(3.0) / 3.0)
}

fn encode_side(e: &mut Enc, me: &SideState, foe: &SideState) {
    encode_active(e, me, foe);
    for slot in 0..6 {
        encode_party_mon(e, me, slot, foe);
    }
    let alive = me.party.iter().filter(|m| m.hp > 0).count();
    e.f(alive as f32 / 6.0);
    e.f(me.party.iter().map(|m| hp_frac(m.hp, m.max_hp)).sum::<f32>() / 6.0);
}

/// The observation for `side` deciding at `site`.
pub fn observe(state: &BattleState, site: PromptSite, side: Side) -> Sparse {
    let (me, foe) = sides(state, side);
    let mut e = Enc::new();
    e.one_hot(8, Some(site_index(site)));
    e.b(state.battle_mode == defs::TRAINER_BATTLE);
    e.b(state.link_mode != 0);
    e.one_hot(4, Some(state.weather as usize));
    e.f(f32::from(state.weather_count) / 5.0);
    encode_side(&mut e, me, foe);
    encode_side(&mut e, foe, me);
    e.finish(OBS_DIM)
}

// --- the actions -----------------------------------------------------------------

/// Width of an action vector.
pub const ACT_DIM: u32 = 6 + 4 + MOVE_W + 6 + PARTY_W + 256 + 6 + PARTY_W + 4;

/// The features of one legal action for `side`.
pub fn action(state: &BattleState, side: Side, action: Action) -> Sparse {
    let (me, foe) = sides(state, side);
    let mut e = Enc::new();
    let kind = match action {
        Action::Move { .. } => 0,
        Action::Fight => 1,
        Action::Switch { .. } | Action::ForcedSwitch { .. } | Action::OfferSwitch { keep: false, .. } => 2,
        Action::Item { .. } => 3,
        Action::Run => 4,
        Action::OfferSwitch { keep: true, .. } => 5,
    };
    e.one_hot(6, Some(kind));
    // A move.
    match action {
        Action::Move { slot } => {
            e.one_hot(4, Some(slot as usize));
            encode_move(&mut e, me, foe, slot as usize);
        }
        _ => e.skip(4 + MOVE_W),
    }
    // A switch target.
    let target = match action {
        Action::Switch { slot } | Action::ForcedSwitch { slot } => Some(slot),
        Action::OfferSwitch { keep: false, slot } => slot,
        _ => None,
    };
    match target {
        Some(slot) => {
            e.one_hot(6, Some(slot as usize));
            encode_party_mon(&mut e, me, slot as usize, foe);
        }
        None => e.skip(6 + PARTY_W),
    }
    // An item, its target mon and move.
    match action {
        Action::Item { id, target, move_slot } => {
            e.one_hot(256, Some(id as usize));
            // Trainer items and self-targeting bag items act on the active mon.
            let slot = target.unwrap_or(me.active_slot);
            e.one_hot(6, Some(slot as usize));
            encode_party_mon(&mut e, me, slot as usize, foe);
            e.one_hot(4, move_slot.map(usize::from));
        }
        _ => e.skip(256 + 6 + PARTY_W + 4),
    }
    e.finish(ACT_DIM)
}

/// A side's "how well am I doing" in `[-1, 1]`: its party's mean HP fraction
/// minus the foe's. The reward shaping potential.
pub fn potential(state: &BattleState, side: Side) -> f32 {
    let (me, foe) = sides(state, side);
    let health = |s: &SideState| -> f32 {
        if s.party.is_empty() {
            return 0.0;
        }
        s.party.iter().map(|m| hp_frac(m.hp, m.max_hp)).sum::<f32>() / s.party.len() as f32
    };
    health(me) - health(foe)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn type_chart_matches_the_game() {
        // Water on Fire, Electric on Ground, Normal on Ghost, Grass on Water/Ground.
        assert_eq!(effectiveness(defs::WATER, [defs::FIRE, defs::FIRE]), 2.0);
        assert_eq!(effectiveness(defs::ELECTRIC, [defs::GROUND, defs::GROUND]), 0.0);
        assert_eq!(effectiveness(defs::NORMAL, [defs::GHOST, defs::POISON]), 0.0);
        assert_eq!(effectiveness(defs::GRASS, [defs::WATER, defs::GROUND]), 4.0);
        assert_eq!(effectiveness(defs::FIRE, [defs::WATER, defs::DRAGON]), 0.25);
    }

    #[test]
    fn move_data_reads_the_rom() {
        let tackle = move_data(defs::TACKLE);
        assert_eq!(tackle.power, 35);
        assert_eq!(tackle.kind, defs::NORMAL);
        let thunderbolt = move_data(defs::THUNDERBOLT);
        assert_eq!(thunderbolt.power, 95);
        assert_eq!(thunderbolt.kind, defs::ELECTRIC);
    }
}
