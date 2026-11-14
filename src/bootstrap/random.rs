//! Generate a randomized trainer battle from a seed.
//!
//! The output is a regular [`BattleConfig`] that flows through the normal
//! `run_with` path. When recorded, `--record` writes the *resolved* matchup
//! into the recording as an embedded `[scenario.*]` block, and `--play`
//! replays that concrete matchup directly — it never re-runs this generator.
//! That decoupling is deliberate: the matchup generator can change freely
//! (e.g. to improve trainer spread) without invalidating any recorded fixture.
//!
//! ## Two independent seeds
//!
//! A single user-facing base seed (`--seed`, else system entropy) is split
//! into two *independent* seeds via domain-separated [`splitmix64`]:
//!
//! - the **generation seed** drives trainer / party / item selection here, and
//! - the **engine seed** is stored in `BattleHeader.seed` and drives the
//!   battle RNG (damage, crits, AI) downstream.
//!
//! They are never the same number, so changing how a matchup is generated has
//! no effect on the engine's RNG stream, and vice versa — the battle engine
//! stays stable and predictable. Reproducibility is preserved: the same base
//! seed always yields the same matchup *and* the same engine seed, so two
//! `--random --seed 0xdead` invocations still produce identical runs.
//!
//! `splitmix64` also fixes a real bias: feeding small or sequential base seeds
//! straight into the xorshift generator made its first output cluster, so a
//! seed-swept corpus (`--seed 0,1,2,…`) drew the same handful of trainers.
//! Mixing the base before use spreads the selection across all trainers.

use super::{BattleConfig, BattleHeader, BattleKind, ItemSpec, MonSpec, species, trainers};

/// Domain salts that split one base seed into two independent streams. Any two
/// distinct constants work; these are arbitrary 64-bit odd values.
const GEN_DOMAIN: u64 = 0x9e37_79b9_7f4a_7c15;
const ENGINE_DOMAIN: u64 = 0xd1b5_4a32_d192_ed03;

/// SplitMix64 finalizer — the standard high-quality mixer used to derive
/// well-distributed seeds from a counter or structured input. Used here to
/// turn a base seed (possibly small or sequential) into the generation and
/// engine seeds without correlation between them.
fn splitmix64(seed: u64) -> u64 {
    let mut z = seed.wrapping_add(0x9e37_79b9_7f4a_7c15);
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}

/// Items the player can actually press "use" on in a trainer battle. Held
/// items, balls, evolution stones, valuables, berries, permanent stat
/// boosters, and escape items are excluded — using them in-battle either
/// does nothing useful or is blocked outright.
const BATTLE_ITEMS: &[&str] = &[
    "Potion", "Super Potion", "Hyper Potion", "Max Potion", "Full Restore",
    "Fresh Water", "Soda Pop", "Lemonade", "Moomoo Milk",
    "Antidote", "Burn Heal", "Ice Heal", "Awakening", "Parlyz Heal", "Full Heal",
    "Revive", "Max Revive",
    "Ether", "Max Ether", "Elixer", "Max Elixer",
    "X Attack", "X Defend", "X Speed", "X Special", "X Accuracy",
    "Dire Hit", "Guard Spec",
];

/// Held items a generated mon may walk into battle carrying. Curated to items
/// with an in-battle effect the engine acts on — Leftovers' end-of-turn heal,
/// Quick Claw's priority roll, the flinch / crit boosters, evasion / accuracy
/// tweaks, and the type-boosting held items. Consumable berries are left out so
/// a recorded matchup's held item stays stable across the whole fight. Every
/// name must resolve via [`items::lookup`] — asserted by `held_items_all_resolve`.
const HELD_ITEMS: &[&str] = &[
    "Leftovers", "Quick Claw", "King's Rock", "Focus Band", "Brightpowder",
    "Scope Lens", "Lucky Egg",
    "Charcoal", "Mystic Water", "Miracle Seed", "Magnet", "Twistedspoon",
    "Blackbelt", "Sharp Beak", "Poison Barb", "Soft Sand", "Hard Stone",
    "Silverpowder", "Spell Tag", "Nevermeltice", "Dragon Fang", "Blackglasses",
    "Pink Bow", "Metal Coat",
];

/// Per-mon chance (in percent) that a `--random` party member is generated
/// holding an item from [`HELD_ITEMS`]. Small on purpose — held items should be
/// an occasional spice, not the norm.
const HELD_ITEM_CHANCE_PERCENT: u64 = 15;

/// Build a random trainer battle from a `base_seed` and log the chosen matchup
/// to stderr so the run is debuggable even when not recording. The trainer
/// is drawn from [`trainers::all`]; player party size is the trainer's
/// canonical count ±1 (clamped to 1..=6); each player mon's level is the
/// trainer's average ±10% (clamped to 2..=100). Items are 2..=4 entries
/// drawn from [`BATTLE_ITEMS`] at 1..=3 each.
///
/// `base_seed` (the user's `--seed`, or system entropy) is split into two
/// independent seeds: the generation seed drives the selection below, and the
/// engine seed is stored in `BattleHeader.seed`. The two are decoupled — see
/// the module docs — so the same `base_seed` reproduces the whole run while
/// the matchup generator and the battle RNG never share state.
pub fn random_battle_config(base_seed: u64) -> BattleConfig {
    generate(base_seed, false)
}

/// [`random_battle_config`], also printing the matchup to stderr (the CLI's
/// `--random`).
pub fn random_battle_config_verbose(base_seed: u64) -> BattleConfig {
    generate(base_seed, true)
}

fn generate(base_seed: u64, verbose: bool) -> BattleConfig {
    let gen_seed = splitmix64(base_seed ^ GEN_DOMAIN);
    let engine_seed = splitmix64(base_seed ^ ENGINE_DOMAIN);
    let mut rng = Rng::from_seed(gen_seed);

    let trainers_list = trainers::all();
    let trainer = &trainers_list[rng.below(trainers_list.len() as u64) as usize];

    let avg = trainer.avg_level as i32;
    // ceil(10% of avg), clamped to at least 1 so even L8 Falkner gets some
    // spread. Integer math: (avg * 10 + 99) / 100 == ceil(avg / 10).
    let delta = ((avg * 10 + 99) / 100).max(1);
    let lvl_lo = (avg - delta).max(2) as u8;
    let lvl_hi = (avg + delta).min(100) as u8;

    let count_lo = trainer.party_count.saturating_sub(1).max(1);
    let count_hi = (trainer.party_count + 1).min(6);
    let count = rng.in_range(count_lo as u64, count_hi as u64) as u8;

    let mut player_party = Vec::with_capacity(count as usize);
    for _ in 0..count {
        let idx = rng.below(species::SPECIES_NAMES.len() as u64) as usize;
        let level = rng.in_range(lvl_lo as u64, lvl_hi as u64) as u8;
        // A small chance to walk in holding a battle-relevant item. Roll first,
        // then pick, so the held-item draw stays a fixed two-step slice of the
        // RNG stream regardless of whether the mon ends up holding anything.
        let held = rng.below(100) < HELD_ITEM_CHANCE_PERCENT;
        let pick = rng.below(HELD_ITEMS.len() as u64) as usize;
        player_party.push(MonSpec {
            species: species::SPECIES_NAMES[idx].to_string(),
            level,
            // The generator leans on the engine's canonical level-up moveset.
            moves: None,
            item: held.then(|| HELD_ITEMS[pick].to_string()),
            ot_id: None,
        });
    }

    let item_count = rng.in_range(2, 4) as usize;
    let mut player_items = Vec::with_capacity(item_count);
    for _ in 0..item_count {
        let name = BATTLE_ITEMS[rng.below(BATTLE_ITEMS.len() as u64) as usize];
        let quantity = rng.in_range(1, 3) as u8;
        player_items.push(ItemSpec { name: name.to_string(), quantity });
    }

    if verbose {
        eprintln!(
            "random battle: seed 0x{base_seed:016x} — vs {} — {} mon(s), levels {}..={}",
            trainer.name, count, lvl_lo, lvl_hi,
        );
        for mon in &player_party {
            match &mon.item {
                Some(item) => eprintln!("  party: {} L{} @ {}", mon.species, mon.level, item),
                None => eprintln!("  party: {} L{}", mon.species, mon.level),
            }
        }
        for item in &player_items {
            eprintln!("  bag:   {} x{}", item.name, item.quantity);
        }
    }

    BattleConfig {
        battle: BattleHeader {
            kind: BattleKind::TrainerBattle,
            trainer: Some(trainer.name.to_string()),
            // The *engine* seed (independent of the generation seed used
            // above). `resolve_seed` returns it so the battle RNG keys off it;
            // `run_with` honours `seed_override` ahead of this for `--play`.
            seed: Some(engine_seed),
            johto_badges: None,
            kanto_badges: None,
            battle_scene: None,
        },
        player_party,
        enemy_party: Vec::new(),
        player_items,
        enemy_items: Vec::new(),
        enemy_trainer: None,
    }
}

/// xorshift64 PRNG seeded from a `u64`. Same shape as the entropy seed
/// generator in `bootstrap/mod.rs`; kept private here because the random
/// battle generator wants its own independent stream from the battle RNG.
struct Rng {
    state: u64,
}

impl Rng {
    fn from_seed(seed: u64) -> Self {
        // xorshift jams on 0, so guarantee a non-zero state for seed = 0.
        let state = if seed == 0 { 0x9e37_79b9_7f4a_7c15 } else { seed };
        Rng { state }
    }

    fn next_u64(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        x
    }

    fn below(&mut self, exclusive_upper: u64) -> u64 {
        self.next_u64() % exclusive_upper
    }

    fn in_range(&mut self, lo: u64, hi: u64) -> u64 {
        lo + self.below(hi - lo + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bootstrap::items;

    /// Every name the generator emits must round-trip through the harness's
    /// lookup tables, every level must fit the engine's 1..=100 range, and
    /// the BATTLE_ITEMS list must be self-consistent with `items::lookup`.
    /// Run a few iterations to cover the RNG's spread.
    #[test]
    fn random_battle_is_well_formed() {
        for seed in [1u64, 0xdead, 0xc0ffee, 0xffff_ffff_ffff_ffff, 42, 0x9e37_79b9_7f4a_7c15] {
            let cfg = random_battle_config(seed);
            assert!(matches!(cfg.battle.kind, BattleKind::TrainerBattle));
            // The header carries the *engine* seed, not the base seed.
            assert_eq!(cfg.battle.seed, Some(splitmix64(seed ^ ENGINE_DOMAIN)));
            let trainer_name = cfg.battle.trainer.as_deref().expect("trainer set");
            assert!(trainers::lookup(trainer_name).is_some(), "unknown trainer {trainer_name}");
            assert!(!cfg.player_party.is_empty() && cfg.player_party.len() <= 6);
            for mon in &cfg.player_party {
                assert!(species::id_from_name(&mon.species).is_some(), "unknown species {}", mon.species);
                assert!(mon.level >= 2 && mon.level <= 100, "bad level {}", mon.level);
                // A generated held item (when present) must resolve, or seeding
                // would error out only after the user picked that seed.
                if let Some(item) = &mon.item {
                    assert!(items::lookup(item).is_some(), "unknown held item {item}");
                }
            }
            for item in &cfg.player_items {
                assert!(items::lookup(&item.name).is_some(), "unknown item {}", item.name);
                assert!(item.quantity >= 1 && item.quantity <= 3);
            }
        }
    }

    /// Every entry in BATTLE_ITEMS must resolve to a known harness item — a
    /// typo here would surface as a runtime error inside `seed_player_items`
    /// only after the user picked that item randomly.
    #[test]
    fn battle_items_all_resolve() {
        for name in BATTLE_ITEMS {
            assert!(items::lookup(name).is_some(), "BATTLE_ITEMS contains unknown name {name:?}");
        }
    }

    /// Same guard for the held-item pool: a typo would only blow up inside
    /// `seed_party` after a `--random` run happened to roll that entry.
    #[test]
    fn held_items_all_resolve() {
        for name in HELD_ITEMS {
            assert!(items::lookup(name).is_some(), "HELD_ITEMS contains unknown name {name:?}");
        }
    }

    /// Held items are an *occasional* spice: a sweep of seeds must produce some
    /// mons holding an item and some holding none. This catches both a stuck-on
    /// (always equips) and stuck-off (never equips) regression in the roll.
    #[test]
    fn held_items_are_sometimes_but_not_always_assigned() {
        let mut with = 0usize;
        let mut without = 0usize;
        for seed in 0..200u64 {
            for mon in random_battle_config(seed).player_party {
                match mon.item {
                    Some(_) => with += 1,
                    None => without += 1,
                }
            }
        }
        assert!(with > 0, "no generated mon ever held an item");
        assert!(without > 0, "every generated mon held an item");
    }

    /// The engine seed in the header must be decoupled from the base seed: a
    /// different number, derived independently of the generation seed. This is
    /// what keeps the battle RNG from sharing state with the matchup generator.
    #[test]
    fn engine_seed_is_decoupled_from_base() {
        for base in [0u64, 1, 2, 0xdead, 0xc0ffee] {
            let cfg = random_battle_config(base);
            let engine = cfg.battle.seed.expect("engine seed set");
            assert_ne!(engine, base, "engine seed must not equal the base seed");
            assert_ne!(
                engine,
                splitmix64(base ^ GEN_DOMAIN),
                "engine seed must differ from the generation seed",
            );
            // Deterministic: same base -> same engine seed.
            assert_eq!(engine, splitmix64(base ^ ENGINE_DOMAIN));
        }
    }

    /// Same seed -> same matchup. This is the contract `--seed` relies on.
    #[test]
    fn same_seed_same_matchup() {
        let a = random_battle_config(0xdead_beef);
        let b = random_battle_config(0xdead_beef);
        assert_eq!(a.battle.trainer, b.battle.trainer);
        assert_eq!(a.battle.seed, b.battle.seed);
        assert_eq!(a.player_party.len(), b.player_party.len());
        for (ma, mb) in a.player_party.iter().zip(b.player_party.iter()) {
            assert_eq!(ma.species, mb.species);
            assert_eq!(ma.level, mb.level);
        }
        assert_eq!(a.player_items.len(), b.player_items.len());
        for (ia, ib) in a.player_items.iter().zip(b.player_items.iter()) {
            assert_eq!(ia.name, ib.name);
            assert_eq!(ia.quantity, ib.quantity);
        }
    }

    /// Different seeds -> different matchups (probabilistically; check that
    /// at least one of several pairs differs so we don't silently degenerate
    /// to a constant).
    #[test]
    fn different_seeds_different_matchups() {
        let a = random_battle_config(1);
        let b = random_battle_config(2);
        let c = random_battle_config(3);
        let diff_ab = a.battle.trainer != b.battle.trainer
            || a.player_party.len() != b.player_party.len()
            || a.player_party.first().map(|m| &m.species)
                != b.player_party.first().map(|m| &m.species);
        let diff_ac = a.battle.trainer != c.battle.trainer
            || a.player_party.len() != c.player_party.len()
            || a.player_party.first().map(|m| &m.species)
                != c.player_party.first().map(|m| &m.species);
        assert!(diff_ab || diff_ac, "generator is not spreading across seeds");
    }

    /// Regression for the trainer-clustering bug: a seed sweep (`--seed 0,1,2,…`,
    /// the natural way to build a corpus) must draw from across the trainer
    /// roster, not collapse onto a handful. Before the `splitmix64` seeding,
    /// seeds 0..100 hit only ~5 trainers with one taking the majority.
    ///
    /// The roster is now the whole game (hundreds of trainers), so a 100-seed
    /// sweep can't cover most of it — instead we assert the draws are nearly
    /// *distinct*. With near-uniform selection, 100 seeds yield ~90 unique
    /// trainers; clustering would collapse that to a handful, so a floor of 70
    /// is robust but still catches a regression.
    #[test]
    fn sequential_seeds_spread_across_trainers() {
        use std::collections::HashSet;
        const SEEDS: u64 = 100;
        let seen: HashSet<String> = (0..SEEDS)
            .map(|s| random_battle_config(s).battle.trainer.expect("trainer set"))
            .collect();
        assert!(
            seen.len() >= 70,
            "only {} distinct trainers across seeds 0..{SEEDS} — selection is clustering",
            seen.len(),
        );
    }
}
