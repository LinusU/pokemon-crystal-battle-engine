//! Generate a randomized trainer battle from a seed.
//!
//! The output is a regular [`BattleConfig`] that flows through the normal
//! `run_with` path, so `--record` captures the resolved team into the
//! recording header (with `source_scenario = "random"`) and `--play` can
//! reproduce the same matchup by re-generating from the recorded seed —
//! see [`super::random_battle_config`] callers in `src/main.rs`.
//!
//! Deterministic: the same `seed` always produces the same trainer, party,
//! and items. The seed is also stored in `BattleHeader.seed`, so the engine
//! RNG (which the battle outcome depends on) ends up keyed off the same
//! number — two `--random --seed 0xdead` invocations produce identical runs
//! end-to-end.

use super::{BattleConfig, BattleHeader, BattleKind, ItemSpec, MonSpec, species, trainers};

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

/// Build a random trainer battle keyed off `seed` and log the chosen matchup
/// to stderr so the run is debuggable even when not recording. The trainer
/// is drawn from [`trainers::all`]; player party size is the trainer's
/// canonical count ±1 (clamped to 1..=6); each player mon's level is the
/// trainer's average ±10% (clamped to 2..=100). Items are 2..=4 entries
/// drawn from [`BATTLE_ITEMS`] at 1..=3 each.
///
/// The returned `BattleConfig` carries `seed` in its header so the engine
/// RNG also seeds from the same value — making the whole run reproducible.
pub fn random_battle_config(seed: u64) -> BattleConfig {
    let mut rng = Rng::from_seed(seed);

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
        player_party.push(MonSpec {
            species: species::SPECIES_NAMES[idx].to_string(),
            level,
        });
    }

    let item_count = rng.in_range(2, 4) as usize;
    let mut player_items = Vec::with_capacity(item_count);
    for _ in 0..item_count {
        let name = BATTLE_ITEMS[rng.below(BATTLE_ITEMS.len() as u64) as usize];
        let quantity = rng.in_range(1, 3) as u8;
        player_items.push(ItemSpec { name: name.to_string(), quantity });
    }

    eprintln!(
        "random battle: seed 0x{seed:016x} — vs {} — {} mon(s), levels {}..={}",
        trainer.name, count, lvl_lo, lvl_hi,
    );
    for mon in &player_party {
        eprintln!("  party: {} L{}", mon.species, mon.level);
    }
    for item in &player_items {
        eprintln!("  bag:   {} x{}", item.name, item.quantity);
    }

    BattleConfig {
        battle: BattleHeader {
            kind: BattleKind::TrainerBattle,
            trainer: Some(trainer.name.to_string()),
            // Pin the seed into the header so `resolve_seed` returns it and
            // the engine RNG matches the matchup RNG. `run_with` honours
            // `seed_override` ahead of this for the `--play` path.
            seed: Some(seed),
        },
        player_party,
        enemy_party: Vec::new(),
        player_items,
        enemy_items: Vec::new(),
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
            assert_eq!(cfg.battle.seed, Some(seed));
            let trainer_name = cfg.battle.trainer.as_deref().expect("trainer set");
            assert!(trainers::lookup(trainer_name).is_some(), "unknown trainer {trainer_name}");
            assert!(!cfg.player_party.is_empty() && cfg.player_party.len() <= 6);
            for mon in &cfg.player_party {
                assert!(species::id_from_name(&mon.species).is_some(), "unknown species {}", mon.species);
                assert!(mon.level >= 2 && mon.level <= 100, "bad level {}", mon.level);
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

    /// Same seed -> same matchup. This is the contract `--seed` relies on.
    #[test]
    fn same_seed_same_matchup() {
        let a = random_battle_config(0xdead_beef);
        let b = random_battle_config(0xdead_beef);
        assert_eq!(a.battle.trainer, b.battle.trainer);
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
}
