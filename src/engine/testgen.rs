//! Random scenarios for differential testing.

use super::oracle::mix;
use crate::bootstrap::{BattleConfig, BattleHeader, BattleKind, MonSpec, names, species};

/// What kind of battles to generate.
#[derive(Debug, Clone, Copy)]
pub struct GenOptions {
    pub max_party: u8,
    pub min_level: u8,
    pub max_level: u8,
}

impl Default for GenOptions {
    fn default() -> Self {
        GenOptions { max_party: 3, min_level: 5, max_level: 60 }
    }
}

/// Held items for generated mons: every battle-relevant held effect,
/// consumable berries and the Berserk Gene included.
const HELD_ITEMS: &[&str] = &[
    "Leftovers", "Quick Claw", "King's Rock", "Focus Band", "Brightpowder",
    "Scope Lens", "Lucky Egg", "Amulet Coin", "Light Ball", "Thick Club",
    "Stick", "Metal Powder", "Charcoal", "Mystic Water", "Miracle Seed",
    "Magnet", "Twistedspoon", "Blackbelt", "Sharp Beak", "Poison Barb",
    "Soft Sand", "Hard Stone", "Silverpowder", "Spell Tag", "Nevermeltice",
    "Dragon Fang", "Blackglasses", "Pink Bow", "Metal Coat", "Polkadot Bow",
    "Berry", "Gold Berry", "PSNCureBerry", "PRZCureBerry", "Burnt Berry",
    "Ice Berry", "Mint Berry", "Bitter Berry", "MiracleBerry", "MysteryBerry",
    "Berserk Gene",
];

/// A random link battle: both parties random, every mon with 1–4 random moves
/// drawn from `moves` (move ids).
pub fn random_link_config(seed: u64, moves: &[u8], opts: GenOptions) -> BattleConfig {
    let mut distinct = moves.to_vec();
    distinct.sort_unstable();
    distinct.dedup();
    let moves = &distinct[..];
    let mut rng = seed ^ 0x6c69_6e6b;
    let party = |rng: &mut u64| -> Vec<MonSpec> {
        let count = 1 + (mix(rng) % u64::from(opts.max_party)) as usize;
        (0..count)
            .map(|_| {
                let idx = (mix(rng) % species::SPECIES_NAMES.len() as u64) as usize;
                let span = u64::from(opts.max_level - opts.min_level) + 1;
                let level = opts.min_level + (mix(rng) % span) as u8;
                let n_moves = 1 + (mix(rng) % 4) as usize;
                let mut picked: Vec<u8> = Vec::new();
                while picked.len() < n_moves.min(moves.len()) {
                    let m = moves[(mix(rng) % moves.len() as u64) as usize];
                    if !picked.contains(&m) {
                        picked.push(m);
                    }
                }
                MonSpec {
                    species: species::SPECIES_NAMES[idx].to_string(),
                    level,
                    moves: Some(picked.iter().map(|&m| names::move_name(m).to_string()).collect()),
                    item: None,
                    ot_id: None,
                }
            })
            .collect()
    };
    let mut player_party = party(&mut rng);
    let mut enemy_party = party(&mut rng);
    // Held items from their own stream, so the parties above stay as they were.
    let mut item_rng = seed ^ 0x6865_6c64;
    for mon in player_party.iter_mut().chain(enemy_party.iter_mut()) {
        if mix(&mut item_rng) % 4 == 0 {
            let pick = (mix(&mut item_rng) % HELD_ITEMS.len() as u64) as usize;
            mon.item = Some(HELD_ITEMS[pick].to_string());
        }
    }
    BattleConfig {
        battle: BattleHeader {
            kind: BattleKind::LinkBattle,
            trainer: None,
            seed: Some(mix(&mut rng)),
            johto_badges: None,
            kanto_badges: None,
            battle_scene: Some(false),
        },
        player_party,
        enemy_party,
        player_items: Vec::new(),
        enemy_items: Vec::new(),
        enemy_trainer: None,
    }
}

/// A random wild battle: a random player party with moves from `moves`, a
/// random wild mon (its level-up moveset).
pub fn random_wild_config(seed: u64, moves: &[u8], opts: GenOptions) -> BattleConfig {
    let mut config = random_link_config(seed, moves, GenOptions { max_party: opts.max_party, ..opts });
    let mut rng = seed ^ 0x7769_6c64;
    let idx = (mix(&mut rng) % species::SPECIES_NAMES.len() as u64) as usize;
    let span = u64::from(opts.max_level - opts.min_level) + 1;
    let level = opts.min_level + (mix(&mut rng) % span) as u8;
    config.battle.kind = BattleKind::WildEncounter;
    traded_and_badges(&mut config, seed);
    config.enemy_party = vec![MonSpec { species: species::SPECIES_NAMES[idx].to_string(), level, moves: None, item: None, ot_id: None }];
    config
}

/// A random trainer battle against a canonical trainer, the player party's
/// moves drawn from `moves`.
pub fn random_trainer_config(seed: u64, moves: &[u8], opts: GenOptions) -> BattleConfig {
    let mut config = crate::bootstrap::random::random_battle_config(seed);
    let link = random_link_config(seed, moves, opts);
    for (mon, donor) in config.player_party.iter_mut().zip(link.player_party.iter().cycle()) {
        mon.moves = donor.moves.clone();
        mon.item = None;
    }
    config.player_items.clear();
    config.battle.battle_scene = Some(false);
    traded_and_badges(&mut config, seed);
    config
}

/// Make about half the player's mons traded and give the player a random
/// subset of Johto badges, so obedience (`BattleCommand_CheckObedience`) and
/// the badge stat boosts get exercised. Link battles skip both.
fn traded_and_badges(config: &mut BattleConfig, seed: u64) {
    let mut rng = seed ^ 0x7472_6164;
    for mon in &mut config.player_party {
        if mix(&mut rng) % 2 == 0 {
            mon.ot_id = Some(0x5678);
        }
    }
    config.battle.johto_badges = Some(mix(&mut rng) as u8);
}
