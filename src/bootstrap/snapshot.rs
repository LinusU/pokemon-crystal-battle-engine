//! Read a comprehensive slice of battle state out of WRAM so the recorder can
//! capture per-step expectations and playback can assert them. Pure observer:
//! it never mutates the CPU.
//!
//! Addresses come from `scratch/pokecrystal/pokecrystal.sym`; the struct
//! layouts are the `battle_struct` / `party_struct` macros in
//! `scratch/pokecrystal/macros/ram.asm` (mirrored in docs/memory_layout.md).
//!
//! Both halves of the record/playback contract live here:
//!
//! - [`BattleSnapshot::read`] observes the live battle into an owned snapshot.
//! - [`diff`] compares a tolerant [`Expect`] (parsed from the recording) against
//!   an observed snapshot, returning one [`Mismatch`] per disagreeing field.
//!
//! The recorder ([`super::recorder`]) serialises a snapshot into each
//! `[[step]]`'s `expect.*` dotted keys; the player ([`super::player`])
//! deserialises those keys into an [`Expect`] and diffs it on replay.

use serde::Deserialize;

use super::{items, species};
use crate::runtime::Cpu;

// ---------------------------------------------------------------------------
// WRAM addresses. Source of truth: scratch/pokecrystal/pokecrystal.sym.

// Active player mon — `wBattleMon`, base 0xc62c. The engine keeps this copy of
// the active party slot in sync for the duration of the battle.
const W_BATTLE_MON_SPECIES: u16 = 0xc62c;
const W_BATTLE_MON_ITEM: u16 = 0xc62d; // held item id (`battle_struct` +1)
const W_BATTLE_MON_PP: u16 = 0xc634; // 4 bytes, one per move slot
const W_BATTLE_MON_LEVEL: u16 = 0xc639;
const W_BATTLE_MON_STATUS: u16 = 0xc63a;
const W_BATTLE_MON_HP: u16 = 0xc63c; // big-endian u16
const W_BATTLE_MON_MAX_HP: u16 = 0xc63e; // big-endian u16

// Active enemy mon — `wEnemyMon`, base 0xd206. Same layout as `wBattleMon`.
const W_ENEMY_MON_SPECIES: u16 = 0xd206;
const W_ENEMY_MON_ITEM: u16 = 0xd207; // held item id (`battle_struct` +1)
const W_ENEMY_MON_PP: u16 = 0xd20e; // 4 bytes
const W_ENEMY_MON_LEVEL: u16 = 0xd213;
const W_ENEMY_MON_STATUS: u16 = 0xd214;
const W_ENEMY_MON_HP: u16 = 0xd216;
const W_ENEMY_MON_MAX_HP: u16 = 0xd218;

// Which party slot each side currently has out.
const W_CUR_BATTLE_MON: u16 = 0xd0d4; // player active slot
const W_CUR_OT_MON: u16 = 0xc663; // enemy active slot

// Party lists. Each slot is a 48-byte `party_struct`.
const W_PARTY_COUNT: u16 = 0xdcd7;
const W_PARTY_MON1: u16 = 0xdcdf;
const W_OT_PARTY_COUNT: u16 = 0xd280;
const W_OT_PARTY_MON1: u16 = 0xd288;
const PARTY_STRUCT_LEN: u16 = 48;

// `party_struct` field offsets (see docs/memory_layout.md).
const PARTY_OFFSET_SPECIES: u16 = 0;
const PARTY_OFFSET_ITEM: u16 = 1; // held item id
const PARTY_OFFSET_LEVEL: u16 = 31;
const PARTY_OFFSET_STATUS: u16 = 32;
const PARTY_OFFSET_HP: u16 = 34; // big-endian u16
const PARTY_OFFSET_MAX_HP: u16 = 36; // big-endian u16

// `wBattleResult` (0xd0ee): low nibble is the WIN/LOSE/DRAW outcome; the high
// bits carry CAUGHT_CELEBI / BOX_FULL flags we don't assert on.
const W_BATTLE_RESULT: u16 = 0xd0ee;
const BATTLE_RESULT_OUTCOME_MASK: u8 = 0x0f;

// Status-byte bit layout (constants/pokemon_data_constants.asm). Sleep is a
// turn counter in the low three bits; the rest are single flags.
const STATUS_SLP_MASK: u8 = 0b0000_0111;
const STATUS_PSN: u8 = 1 << 3;
const STATUS_BRN: u8 = 1 << 4;
const STATUS_FRZ: u8 = 1 << 5;
const STATUS_PAR: u8 = 1 << 6;
const STATUS_TOX: u8 = 1 << 7;

// Current PP lives in the low 6 bits; the top two are applied PP Ups.
const PP_MASK: u8 = 0b0011_1111;

const MAX_PARTY: u8 = 6;

// ---------------------------------------------------------------------------
// Status / outcome enums

/// A mon's major status condition, collapsed from the engine's status byte
/// into a single tag. The engine never sets two non-sleep flags at once, so the
/// first matching bit is unambiguous.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    None,
    Sleep,
    Poison,
    Burn,
    Freeze,
    Paralysis,
    Toxic,
}

impl Status {
    /// The lowercase tag written into / parsed from the recording TOML.
    pub fn as_str(self) -> &'static str {
        match self {
            Status::None => "none",
            Status::Sleep => "sleep",
            Status::Poison => "poison",
            Status::Burn => "burn",
            Status::Freeze => "freeze",
            Status::Paralysis => "paralysis",
            Status::Toxic => "toxic",
        }
    }

    fn decode(byte: u8) -> Status {
        if byte & STATUS_SLP_MASK != 0 {
            Status::Sleep
        } else if byte & STATUS_TOX != 0 {
            Status::Toxic
        } else if byte & STATUS_PAR != 0 {
            Status::Paralysis
        } else if byte & STATUS_FRZ != 0 {
            Status::Freeze
        } else if byte & STATUS_BRN != 0 {
            Status::Burn
        } else if byte & STATUS_PSN != 0 {
            Status::Poison
        } else {
            Status::None
        }
    }
}

/// The battle's final result, decoded from `wBattleResult`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Outcome {
    Win,
    Loss,
    Draw,
    /// The wild mon was caught with a Poké Ball. The engine reports a capture
    /// as a plain WIN in `wBattleResult` (the catch path masks the result to
    /// WIN — see `engine/battle/core.asm` `.run`), so this variant is driven by
    /// the harness side-channel `cpu.caught_wild_mon`, not by WRAM.
    Caught,
}

impl Outcome {
    /// The lowercase tag written into / parsed from the recording TOML.
    pub fn as_str(self) -> &'static str {
        match self {
            Outcome::Win => "win",
            Outcome::Loss => "loss",
            Outcome::Draw => "draw",
            Outcome::Caught => "caught",
        }
    }
}

/// Read the engine's battle outcome flag. Only meaningful once the battle has
/// ended (i.e. after `start_battle` returns).
///
/// A capture is reported by the engine as a WIN (the catch path masks
/// `wBattleResult` to WIN), so it is indistinguishable from a KO win in WRAM.
/// The CLI capture path sets `cpu.caught_wild_mon` when a thrown ball succeeds;
/// we check that side-channel first so a capture surfaces as `Outcome::Caught`.
pub fn read_outcome(cpu: &Cpu) -> Outcome {
    if cpu.caught_wild_mon {
        return Outcome::Caught;
    }
    match cpu.read_byte(W_BATTLE_RESULT) & BATTLE_RESULT_OUTCOME_MASK {
        0 => Outcome::Win,
        1 => Outcome::Loss,
        _ => Outcome::Draw,
    }
}

// ---------------------------------------------------------------------------
// Observed snapshot

/// A whole-battle snapshot taken at one decision point (or at battle end).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BattleSnapshot {
    pub player: SideSnapshot,
    pub enemy: SideSnapshot,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SideSnapshot {
    /// Party slot index of the mon currently out.
    pub active_slot: u8,
    /// The active mon (mirror of the active party slot, plus working PP).
    pub active: ActiveSnapshot,
    /// Every party slot, in order.
    pub party: Vec<PartySnapshot>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActiveSnapshot {
    pub species: String,
    pub level: u8,
    pub hp: u16,
    pub max_hp: u16,
    pub status: Status,
    /// Current PP per move slot (PP Ups masked off).
    pub pp: [u8; 4],
    /// Held item display name (`"none"` when unequipped). See [`item_name`].
    pub item: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PartySnapshot {
    pub species: String,
    pub level: u8,
    pub hp: u16,
    pub max_hp: u16,
    pub status: Status,
    /// Held item display name (`"none"` when unequipped). See [`item_name`].
    pub item: String,
}

impl BattleSnapshot {
    /// Read the current battle state from WRAM. Call this when the engine is
    /// between turns (at a CLI prompt site or at battle end) — values read
    /// mid-animation are not meaningful.
    pub fn read(cpu: &Cpu) -> Self {
        BattleSnapshot {
            player: SideSnapshot {
                active_slot: cpu.read_byte(W_CUR_BATTLE_MON),
                active: ActiveSnapshot {
                    species: species_name(cpu.read_byte(W_BATTLE_MON_SPECIES)),
                    level: cpu.read_byte(W_BATTLE_MON_LEVEL),
                    hp: read_be_word(cpu, W_BATTLE_MON_HP),
                    max_hp: read_be_word(cpu, W_BATTLE_MON_MAX_HP),
                    status: Status::decode(cpu.read_byte(W_BATTLE_MON_STATUS)),
                    pp: read_pp(cpu, W_BATTLE_MON_PP),
                    item: item_name(cpu.read_byte(W_BATTLE_MON_ITEM)),
                },
                party: read_party(cpu, W_PARTY_COUNT, W_PARTY_MON1),
            },
            enemy: SideSnapshot {
                active_slot: cpu.read_byte(W_CUR_OT_MON),
                active: ActiveSnapshot {
                    species: species_name(cpu.read_byte(W_ENEMY_MON_SPECIES)),
                    level: cpu.read_byte(W_ENEMY_MON_LEVEL),
                    hp: read_be_word(cpu, W_ENEMY_MON_HP),
                    max_hp: read_be_word(cpu, W_ENEMY_MON_MAX_HP),
                    status: Status::decode(cpu.read_byte(W_ENEMY_MON_STATUS)),
                    pp: read_pp(cpu, W_ENEMY_MON_PP),
                    item: item_name(cpu.read_byte(W_ENEMY_MON_ITEM)),
                },
                party: read_party(cpu, W_OT_PARTY_COUNT, W_OT_PARTY_MON1),
            },
        }
    }
}

fn read_party(cpu: &Cpu, count_addr: u16, base: u16) -> Vec<PartySnapshot> {
    let n = cpu.read_byte(count_addr).min(MAX_PARTY);
    (0..n)
        .map(|i| {
            let addr = base + u16::from(i) * PARTY_STRUCT_LEN;
            PartySnapshot {
                species: species_name(cpu.read_byte(addr + PARTY_OFFSET_SPECIES)),
                level: cpu.read_byte(addr + PARTY_OFFSET_LEVEL),
                hp: read_be_word(cpu, addr + PARTY_OFFSET_HP),
                max_hp: read_be_word(cpu, addr + PARTY_OFFSET_MAX_HP),
                status: Status::decode(cpu.read_byte(addr + PARTY_OFFSET_STATUS)),
                item: item_name(cpu.read_byte(addr + PARTY_OFFSET_ITEM)),
            }
        })
        .collect()
}

fn read_pp(cpu: &Cpu, base: u16) -> [u8; 4] {
    [
        cpu.read_byte(base) & PP_MASK,
        cpu.read_byte(base + 1) & PP_MASK,
        cpu.read_byte(base + 2) & PP_MASK,
        cpu.read_byte(base + 3) & PP_MASK,
    ]
}

fn read_be_word(cpu: &Cpu, addr: u16) -> u16 {
    u16::from_be_bytes([cpu.read_byte(addr), cpu.read_byte(addr + 1)])
}

/// Map a species id to its display name, mirroring `bootstrap::cli`. Falls back
/// to `#id` for the 0 / out-of-range slots that appear in empty party entries.
fn species_name(id: u8) -> String {
    if id == 0 || id as usize > species::SPECIES_NAMES.len() {
        format!("#{id}")
    } else {
        species::SPECIES_NAMES[id as usize - 1].to_string()
    }
}

/// Map a held-item id to its display name. `0` (no item) renders as `"none"`.
/// Ids the harness doesn't have a name for (TMs, mail, unused slots a ROM
/// trainer might hold) fall back to `#id`. Both the recorder and the playback
/// reader go through here, so the strings match on either side of a round-trip.
fn item_name(id: u8) -> String {
    if id == 0 {
        "none".to_string()
    } else {
        items::name_from_id(id).map_or_else(|| format!("#{id}"), |n| n.to_string())
    }
}

// ---------------------------------------------------------------------------
// Expectations: parsed from the recording, diffed on playback.
//
// Every field is optional. The recorder fills them all; a hand-edited recording
// may set only the fields it cares about, and the diff only checks what's set.

/// A tolerant block of expected post-step state, deserialised from the
/// `expect.*` dotted keys in a `[[step]]`.
#[derive(Debug, Clone, Default, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct Expect {
    pub player: Option<SideExpect>,
    pub enemy: Option<SideExpect>,
    pub battle: Option<BattleExpect>,
}

impl Expect {
    pub fn is_empty(&self) -> bool {
        self.player.is_none() && self.enemy.is_none() && self.battle.is_none()
    }
}

#[derive(Debug, Clone, Default, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct SideExpect {
    pub active_slot: Option<u8>,
    pub active: Option<ActiveExpect>,
    pub party: Option<Vec<PartyExpect>>,
}

#[derive(Debug, Clone, Default, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct ActiveExpect {
    pub species: Option<String>,
    pub level: Option<u8>,
    pub hp: Option<u16>,
    pub max_hp: Option<u16>,
    pub status: Option<Status>,
    pub pp: Option<[u8; 4]>,
    pub item: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct PartyExpect {
    pub species: Option<String>,
    pub level: Option<u8>,
    pub hp: Option<u16>,
    pub max_hp: Option<u16>,
    pub status: Option<Status>,
    pub item: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct BattleExpect {
    pub outcome: Option<Outcome>,
}

// ---------------------------------------------------------------------------
// Diff

/// One field that disagreed between the recorded expectation and the observed
/// snapshot.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mismatch {
    pub path: String,
    pub expected: String,
    pub actual: String,
}

/// Compare a tolerant `expect` block against an observed snapshot, emitting one
/// [`Mismatch`] per disagreeing field. `outcome` is checked only when `Some`
/// (the final step, after the battle has ended).
pub fn diff(expect: &Expect, snap: &BattleSnapshot, outcome: Option<Outcome>) -> Vec<Mismatch> {
    let mut out = Vec::new();
    if let Some(side) = &expect.player {
        diff_side("player", side, &snap.player, &mut out);
    }
    if let Some(side) = &expect.enemy {
        diff_side("enemy", side, &snap.enemy, &mut out);
    }
    if let Some(battle) = &expect.battle {
        if let Some(expected) = battle.outcome {
            if Some(expected) != outcome {
                out.push(Mismatch {
                    path: "battle.outcome".into(),
                    expected: expected.as_str().into(),
                    actual: outcome.map_or_else(|| "<in progress>".into(), |o| o.as_str().to_string()),
                });
            }
        }
    }
    out
}

fn diff_side(side: &str, expect: &SideExpect, snap: &SideSnapshot, out: &mut Vec<Mismatch>) {
    if let Some(expected) = expect.active_slot {
        if expected != snap.active_slot {
            out.push(scalar(format!("{side}.active_slot"), expected, snap.active_slot));
        }
    }
    if let Some(active) = &expect.active {
        diff_active(side, active, &snap.active, out);
    }
    if let Some(party) = &expect.party {
        for (i, slot) in party.iter().enumerate() {
            match snap.party.get(i) {
                Some(actual) => diff_party_slot(side, i, slot, actual, out),
                None => out.push(Mismatch {
                    path: format!("{side}.party[{i}]"),
                    expected: "present".into(),
                    actual: "missing".into(),
                }),
            }
        }
    }
}

fn diff_active(side: &str, e: &ActiveExpect, a: &ActiveSnapshot, out: &mut Vec<Mismatch>) {
    if let Some(v) = &e.species {
        if v != &a.species {
            out.push(text(format!("{side}.active.species"), v.clone(), a.species.clone()));
        }
    }
    if let Some(v) = e.level {
        if v != a.level {
            out.push(scalar(format!("{side}.active.level"), v, a.level));
        }
    }
    if let Some(v) = e.hp {
        if v != a.hp {
            out.push(scalar(format!("{side}.active.hp"), v, a.hp));
        }
    }
    if let Some(v) = e.max_hp {
        if v != a.max_hp {
            out.push(scalar(format!("{side}.active.max_hp"), v, a.max_hp));
        }
    }
    if let Some(v) = e.status {
        if v != a.status {
            out.push(text(format!("{side}.active.status"), v.as_str().into(), a.status.as_str().into()));
        }
    }
    if let Some(v) = &e.pp {
        if v != &a.pp {
            out.push(text(format!("{side}.active.pp"), format!("{v:?}"), format!("{:?}", a.pp)));
        }
    }
    if let Some(v) = &e.item {
        if v != &a.item {
            out.push(text(format!("{side}.active.item"), v.clone(), a.item.clone()));
        }
    }
}

fn diff_party_slot(side: &str, i: usize, e: &PartyExpect, a: &PartySnapshot, out: &mut Vec<Mismatch>) {
    let path = |f: &str| format!("{side}.party[{i}].{f}");
    if let Some(v) = &e.species {
        if v != &a.species {
            out.push(text(path("species"), v.clone(), a.species.clone()));
        }
    }
    if let Some(v) = e.level {
        if v != a.level {
            out.push(scalar(path("level"), v, a.level));
        }
    }
    if let Some(v) = e.hp {
        if v != a.hp {
            out.push(scalar(path("hp"), v, a.hp));
        }
    }
    if let Some(v) = e.max_hp {
        if v != a.max_hp {
            out.push(scalar(path("max_hp"), v, a.max_hp));
        }
    }
    if let Some(v) = e.status {
        if v != a.status {
            out.push(text(path("status"), v.as_str().into(), a.status.as_str().into()));
        }
    }
    if let Some(v) = &e.item {
        if v != &a.item {
            out.push(text(path("item"), v.clone(), a.item.clone()));
        }
    }
}

fn scalar<T: std::fmt::Display>(path: String, expected: T, actual: T) -> Mismatch {
    Mismatch { path, expected: expected.to_string(), actual: actual.to_string() }
}

fn text(path: String, expected: String, actual: String) -> Mismatch {
    Mismatch { path, expected, actual }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn active(species: &str, hp: u16) -> ActiveSnapshot {
        ActiveSnapshot {
            species: species.to_string(),
            level: 10,
            hp,
            max_hp: 40,
            status: Status::None,
            pp: [10, 0, 0, 0],
            item: "none".into(),
        }
    }

    fn snap() -> BattleSnapshot {
        BattleSnapshot {
            player: SideSnapshot {
                active_slot: 0,
                active: active("QUILAVA", 30),
                party: vec![PartySnapshot {
                    species: "QUILAVA".into(),
                    level: 10,
                    hp: 30,
                    max_hp: 40,
                    status: Status::None,
                    item: "none".into(),
                }],
            },
            enemy: SideSnapshot {
                active_slot: 0,
                active: active("PIDGEY", 0),
                party: vec![PartySnapshot {
                    species: "PIDGEY".into(),
                    level: 9,
                    hp: 0,
                    max_hp: 28,
                    status: Status::None,
                    item: "none".into(),
                }],
            },
        }
    }

    #[test]
    fn item_id_decodes_to_a_name_or_none() {
        assert_eq!(item_name(0), "none");
        assert_eq!(item_name(0x92), "Leftovers");
        assert_eq!(item_name(0x52), "King's Rock");
        // An id with no harness name (e.g. a TM) falls back to `#id`.
        assert_eq!(item_name(0xbf), "#191");
    }

    #[test]
    fn held_item_mismatch_is_reported() {
        // A held-item expectation that disagrees with the observed item surfaces
        // on both the active mon and a benched party slot.
        let mut s = snap();
        s.player.active.item = "Leftovers".into();
        s.player.party[0].item = "Leftovers".into();
        let e = Expect {
            player: Some(SideExpect {
                active: Some(ActiveExpect { item: Some("King's Rock".into()), ..Default::default() }),
                party: Some(vec![PartyExpect { item: Some("none".into()), ..Default::default() }]),
                ..Default::default()
            }),
            ..Default::default()
        };
        let ms = diff(&e, &s, None);
        let paths: Vec<&str> = ms.iter().map(|m| m.path.as_str()).collect();
        assert!(paths.contains(&"player.active.item"), "{paths:?}");
        assert!(paths.contains(&"player.party[0].item"), "{paths:?}");
        // And a matching expectation produces no mismatch.
        let ok = Expect {
            player: Some(SideExpect {
                active: Some(ActiveExpect { item: Some("Leftovers".into()), ..Default::default() }),
                ..Default::default()
            }),
            ..Default::default()
        };
        assert!(diff(&ok, &s, None).is_empty());
    }

    #[test]
    fn status_byte_decodes_to_the_dominant_condition() {
        assert_eq!(Status::decode(0x00), Status::None);
        assert_eq!(Status::decode(0x03), Status::Sleep); // sleep counter
        assert_eq!(Status::decode(STATUS_PSN), Status::Poison);
        assert_eq!(Status::decode(STATUS_BRN), Status::Burn);
        assert_eq!(Status::decode(STATUS_FRZ), Status::Freeze);
        assert_eq!(Status::decode(STATUS_PAR), Status::Paralysis);
        assert_eq!(Status::decode(STATUS_TOX), Status::Toxic);
    }

    #[test]
    fn empty_expect_matches_anything() {
        let e = Expect::default();
        assert!(e.is_empty());
        assert!(diff(&e, &snap(), None).is_empty());
    }

    #[test]
    fn matching_fields_produce_no_mismatch() {
        let e = Expect {
            player: Some(SideExpect {
                active_slot: Some(0),
                active: Some(ActiveExpect { hp: Some(30), pp: Some([10, 0, 0, 0]), ..Default::default() }),
                party: None,
            }),
            enemy: Some(SideExpect {
                active: Some(ActiveExpect { hp: Some(0), species: Some("PIDGEY".into()), ..Default::default() }),
                ..Default::default()
            }),
            battle: Some(BattleExpect { outcome: Some(Outcome::Win) }),
        };
        assert!(diff(&e, &snap(), Some(Outcome::Win)).is_empty());
    }

    #[test]
    fn disagreeing_fields_are_reported_with_paths() {
        let e = Expect {
            player: Some(SideExpect {
                active: Some(ActiveExpect { hp: Some(31), ..Default::default() }),
                party: Some(vec![PartyExpect { hp: Some(40), ..Default::default() }]),
                ..Default::default()
            }),
            enemy: None,
            battle: Some(BattleExpect { outcome: Some(Outcome::Loss) }),
        };
        let ms = diff(&e, &snap(), Some(Outcome::Win));
        let paths: Vec<&str> = ms.iter().map(|m| m.path.as_str()).collect();
        assert!(paths.contains(&"player.active.hp"), "{paths:?}");
        assert!(paths.contains(&"player.party[0].hp"), "{paths:?}");
        assert!(paths.contains(&"battle.outcome"), "{paths:?}");
    }

    #[test]
    fn non_final_steps_carry_no_outcome_so_mid_battle_diffs_are_quiet() {
        // The real flow: non-final steps never have a `battle` block (the
        // recorder only writes the outcome in `finalize`), and those are the
        // only expects diffed mid-battle (`outcome = None`). With no battle
        // block, nothing is flagged.
        let e = Expect {
            player: Some(SideExpect {
                active: Some(ActiveExpect { hp: Some(30), ..Default::default() }),
                ..Default::default()
            }),
            ..Default::default()
        };
        assert!(diff(&e, &snap(), None).is_empty());
    }

    #[test]
    fn outcome_expected_but_battle_still_in_progress_is_a_mismatch() {
        // Defensive: if a (hand-written) recording expects an outcome but the
        // battle hasn't ended, surface it rather than passing silently.
        let e = Expect {
            battle: Some(BattleExpect { outcome: Some(Outcome::Win) }),
            ..Default::default()
        };
        let ms = diff(&e, &snap(), None);
        assert_eq!(ms.len(), 1);
        assert_eq!(ms[0].path, "battle.outcome");
        assert_eq!(ms[0].actual, "<in progress>");
    }
}
