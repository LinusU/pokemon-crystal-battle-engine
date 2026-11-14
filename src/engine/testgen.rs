//! Random scenarios for differential testing.

use super::oracle::mix;
use crate::bootstrap::{BattleConfig, BattleHeader, BattleKind, ItemSpec, MonSpec, names, species};

/// What kind of battles to generate.
#[derive(Debug, Clone, Copy)]
pub struct GenOptions {
    pub max_party: u8,
    pub min_level: u8,
    pub max_level: u8,
    /// Give the player a bag (wild and trainer battles; link battles allow
    /// no items).
    pub items: bool,
}

impl Default for GenOptions {
    fn default() -> Self {
        GenOptions { max_party: 3, min_level: 5, max_level: 60, items: true }
    }
}

/// Bag items a generated player may carry: every item the legal-step code
/// offers (heals, status heals, revives, PP items, X items, Dire Hit,
/// Guard Spec, X Accuracy, berries).
const BAG_ITEMS: &[&str] = &[
    "Potion", "Super Potion", "Hyper Potion", "Max Potion", "Full Restore",
    "Fresh Water", "Soda Pop", "Lemonade", "Moomoo Milk", "Berry Juice", "Ragecandybar",
    "Energypowder", "Energy Root", "Heal Powder", "Revival Herb",
    "Antidote", "Burn Heal", "Ice Heal", "Awakening", "Parlyz Heal", "Full Heal",
    "Revive", "Max Revive",
    "Ether", "Max Ether", "Elixer", "Max Elixer", "Mysteryberry",
    "X Attack", "X Defend", "X Speed", "X Special", "X Accuracy", "Dire Hit", "Guard Spec",
    "Berry", "Gold Berry", "Psncureberry", "Przcureberry", "Burnt Berry", "Ice Berry", "Bitter Berry",
    "Mint Berry", "Miracleberry",
];

/// Poké Balls a generated player may carry.
const BAG_BALLS: &[&str] = &[
    "Poke Ball", "Great Ball", "Ultra Ball", "Heavy Ball", "Level Ball", "Lure Ball", "Fast Ball",
    "Moon Ball", "Love Ball",
];

/// A random bag: 3–8 distinct items (1–3 each), 0–2 kinds of ball, and a Poké
/// Doll now and then.
pub fn random_bag(rng: &mut u64) -> Vec<ItemSpec> {
    let mut bag: Vec<ItemSpec> = Vec::new();
    let mut add = |bag: &mut Vec<ItemSpec>, name: &str, quantity: u8| {
        if !bag.iter().any(|i| i.name == name) {
            bag.push(ItemSpec { name: name.to_string(), quantity });
        }
    };
    for _ in 0..3 + mix(rng) % 6 {
        let name = BAG_ITEMS[(mix(rng) % BAG_ITEMS.len() as u64) as usize];
        add(&mut bag, name, 1 + (mix(rng) % 3) as u8);
    }
    for _ in 0..mix(rng) % 3 {
        let name = BAG_BALLS[(mix(rng) % BAG_BALLS.len() as u64) as usize];
        add(&mut bag, name, 1 + (mix(rng) % 3) as u8);
    }
    if mix(rng) % 4 == 0 {
        add(&mut bag, "Poke Doll", 1 + (mix(rng) % 2) as u8);
    }
    bag
}

/// A random link battle: both parties random, every mon with 1–4 random moves
/// drawn from `moves` (move ids).
pub fn random_link_config(seed: u64, moves: &[u8], opts: GenOptions) -> BattleConfig {
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
                }
            })
            .collect()
    };
    let player_party = party(&mut rng);
    let enemy_party = party(&mut rng);
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
    if opts.items {
        config.player_items = random_bag(&mut rng);
    }
    config.enemy_party = vec![MonSpec { species: species::SPECIES_NAMES[idx].to_string(), level, moves: None, item: None }];
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
    if opts.items {
        config.player_items = random_bag(&mut (seed ^ 0x6261_6773));
    }
    config.battle.battle_scene = Some(false);
    config
}
