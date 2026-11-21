//! Crystal Legacy's game data: a few of the changes it makes, and battles
//! played on it.

use crystal_battle::battle::{Battle, BattleSetup, Opponent, Status};
use crystal_battle::data::GameData;
use crystal_battle::data::game::{Evolution, Rules, TrainerMon, TrainerParty, TrainerStage};
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

/// The battle state once the first Pokémon is out, against trainer `class`
/// and `id`, with `badges` Johto badges.
fn against(data: &'static GameData, (class, id): (u8, u8), badges: u8) -> Result<crystal_battle::battle::BattleState, String> {
    let config = random_battle_config(1);
    let (mut setup, rng) = BattleSetup::from_config_in(data, &config, 1).expect("a valid scenario");
    setup.player.johto_badges = badges;
    let Opponent::Trainer { class: c, id: i, party, .. } = &mut setup.opponent else { panic!("a trainer battle") };
    (*c, *i, *party) = (class, id, None);
    Battle::start(data, &setup, rng).map(|battle| battle.state().clone()).map_err(|e| e.to_string())
}

/// Legacy's data with one party for trainer class 1: `party`.
fn with_party(rules: Rules, party: TrainerParty) -> &'static GameData {
    let mut firsts = [u16::MAX; 256];
    firsts[1] = 0;
    Box::leak(Box::new(GameData {
        rules,
        trainer_parties: Box::leak(Box::new([party])),
        trainer_class_first_party: Box::leak(Box::new(firsts)),
        ..*GameData::crystal_legacy()
    }))
}

const fn mon(level: u8, species: u8, nickname: Option<&'static str>) -> TrainerMon {
    TrainerMon { level, species, item: None, moves: None, nickname, dvs: None, stat_exp: None, happiness: None }
}

#[test]
fn a_legacy_trainer_mon_goes_by_its_nickname() {
    use crystal_battle::battle::{Event, Message};
    const NICKNAME: u8 = 1 << 2;
    let sent_out = |rules: Rules| {
        let party = TrainerParty { name: "JOEY", kind: NICKNAME, mons: &[const { mon(10, PIKACHU, Some("BUDDY")) }], stages: &[] };
        let data = with_party(rules, party);
        let config = random_battle_config(1);
        let (mut setup, rng) = BattleSetup::from_config_in(data, &config, 1).unwrap();
        let Opponent::Trainer { class, id, party, .. } = &mut setup.opponent else { panic!("a trainer battle") };
        (*class, *id, *party) = (1, 1, None);
        let battle = Battle::start(data, &setup, rng).unwrap();
        battle.events().iter().find_map(|e| if let Event::Message(Message::EnemySentOut { mon, .. }) = e { Some(mon.clone()) } else { None }).unwrap()
    };
    assert_eq!(sent_out(Rules::CRYSTAL_LEGACY), "BUDDY");
    assert_eq!(sent_out(Rules { trainer_nicknames: false, ..Rules::CRYSTAL_LEGACY }), "PIKACHU");
}

#[test]
fn a_staged_legacy_party_follows_the_badges() {
    const VARIABLE: u8 = 1 << 5;
    const RATTATA: u8 = 19;
    const PIDGEY: u8 = 16;
    let stages = &[
        TrainerStage { kind: 0, mons: &[const { mon(5, RATTATA, None) }] },
        TrainerStage { kind: 0, mons: &[const { mon(12, PIDGEY, None) }] },
    ];
    let data = with_party(Rules::CRYSTAL_LEGACY, TrainerParty { name: "JOEY", kind: VARIABLE, mons: &[], stages });
    let first = |badges| against(data, (1, 1), badges).map(|state| (state.enemy.party[0].species, state.enemy.party[0].level));
    assert_eq!(first(0), Ok((RATTATA, 5)));
    assert_eq!(first(1 << 3), Ok((PIDGEY, 12)));
    assert!(first(0b11).is_err(), "two badges, two stages");
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
