//! Crystal Legacy's game data: a few of the changes it makes, and battles
//! played on it.

use crystal_battle::battle::{Battle, BattleSetup, Opponent, Status};
use crystal_battle::data::GameData;
use crystal_battle::data::game::Evolution;
use crystal_battle::data::random::random_battle_config;

const KADABRA: u8 = 64;
const ALAKAZAM: u8 = 65;
const ONIX: u8 = 95;
const PIKACHU: u8 = 25;
const CHARMANDER: u8 = 4;
const CUT: u8 = 15;
const BUG: u8 = 7;
const METAL_COAT: u8 = 0x8f;
const STEELIX: u8 = 208;

#[test]
fn legacy_changes_species_and_moves() {
    let crystal = GameData::crystal();
    let legacy = GameData::crystal_legacy();

    assert_eq!(crystal.species(KADABRA).evolutions, &[Evolution::Trade { item: 0xff, species: ALAKAZAM }]);
    assert_eq!(legacy.species(KADABRA).evolutions, &[Evolution::Level { level: 42, species: ALAKAZAM }]);
    assert_eq!(legacy.species(ONIX).evolutions, &[Evolution::Item { item: METAL_COAT, species: STEELIX }]);

    assert_ne!(crystal.species(PIKACHU).base_stats, legacy.species(PIKACHU).base_stats);
    assert_ne!(crystal.species(CHARMANDER).learnset, legacy.species(CHARMANDER).learnset);

    assert_eq!(crystal.mv(CUT).kind, 0); // NORMAL
    assert_eq!(legacy.mv(CUT).kind, BUG);
    assert_eq!(legacy.mv(CUT).power, 55);

    // Fossils fill item ids Crystal leaves unused, and the TMs no longer skip any.
    assert_eq!(legacy.names.items[0x94], "DOME FOSSIL");
    assert_eq!(legacy.names.items[0xc1], "TM01");
    assert_eq!(crystal.names.items[0xbf], "TM01");
}

#[test]
fn legacy_has_no_built_in_trainer_parties() {
    let legacy = GameData::crystal_legacy();
    assert!(legacy.trainer_parties.is_empty());
    assert_eq!(legacy.trainer_party(1, 1), None);
}

/// A battle against a trainer of the game's own fails: Legacy has no parties.
#[test]
fn a_built_in_trainer_is_unsupported() {
    let config = random_battle_config(1);
    let (setup, rng) = BattleSetup::from_config(&config, config.battle.seed.unwrap()).expect("a valid scenario");
    assert!(matches!(setup.opponent, Opponent::Trainer { party: None, .. }));
    assert!(Battle::start(GameData::crystal_legacy(), &setup, rng).is_err());
}

/// Random battles play to the end on Legacy's data, with mons made from it.
/// The trainer brings a party of the player's species, as Legacy has none.
#[test]
fn battles_play_on_legacy_data() {
    for seed in 0..60u64 {
        let mut config = random_battle_config(seed);
        if config.enemy_party.is_empty() {
            config.enemy_party = config.player_party.clone();
        }
        let (setup, rng): (BattleSetup, u64) =
            BattleSetup::from_config_in(GameData::crystal_legacy(), &config, config.battle.seed.unwrap())
                .expect("a valid scenario");
        let mut battle = Battle::start(GameData::crystal_legacy(), &setup, rng).expect("battle setup");
        let mut policy = seed | 1;
        for _ in 0..2000 {
            let Status::Decision(decision) = battle.status() else { break };
            policy ^= policy << 13;
            policy ^= policy >> 7;
            policy ^= policy << 17;
            let action = decision.legal[(policy % decision.legal.len() as u64) as usize];
            battle.step(action).expect("a legal action");
        }
        assert!(battle.result().is_some(), "seed {seed}: the battle ends");
    }
}
