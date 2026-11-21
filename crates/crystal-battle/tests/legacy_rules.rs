//! Crystal Legacy's battle rules (`Rules::CRYSTAL_LEGACY`). Each test plays
//! the same battle on Legacy's data twice, with Legacy's rules and with
//! Crystal's, and checks the difference the rule makes.

use crystal_battle::battle::{Action, Battle, BattleSetup, Opponent, Pokemon, Side, Status};
use crystal_battle::config::{BattleConfig, BattleHeader, BattleKind, EnemyControl, MonSpec};
use crystal_battle::data::GameData;
use crystal_battle::data::game::Rules;

const COUNTER: u8 = 68;
const SPLASH: u8 = 150;
const CRUNCH: u8 = 242;
const MIRROR_COAT: u8 = 243;
const SHADOW_BALL: u8 = 247;

const ATTACK: usize = 1;
const SP_ATTACK: usize = 4;

/// Legacy's data with `rules`.
fn legacy_with(rules: Rules) -> &'static GameData {
    Box::leak(Box::new(GameData { rules, ..*GameData::crystal_legacy() }))
}

fn mon(species: &str) -> MonSpec {
    MonSpec { species: species.into(), level: 50, moves: None, item: None, ot_id: None }
}

fn teach(data: &GameData, mon: &mut Pokemon, moves: [u8; 4]) {
    mon.moves = moves;
    mon.pp = moves.map(|m| if m == 0 { 0 } else { data.mv(m).pp });
}

/// A link battle, both sides decided here: the player's `player` against the
/// enemy's `enemy`, each knowing one move, after `edit`.
fn link(
    data: &'static GameData,
    (player, player_move): (&str, u8),
    (enemy, enemy_move): (&str, u8),
    edit: impl FnOnce(&mut Pokemon, &mut Pokemon),
) -> Battle {
    let config = BattleConfig {
        battle: BattleHeader {
            kind: BattleKind::LinkBattle,
            trainer: None,
            seed: Some(1),
            johto_badges: None,
            kanto_badges: None,
            battle_scene: None,
            enemy_control: EnemyControl::Original,
        },
        player_party: vec![mon(player)],
        enemy_party: vec![mon(enemy)],
        player_items: vec![],
        enemy_items: vec![],
        enemy_trainer: None,
    };
    let (mut setup, rng) = BattleSetup::from_config_in(data, &config, 1).expect("a valid scenario");
    let Opponent::Link { party, .. } = &mut setup.opponent else { unreachable!() };
    teach(data, &mut setup.party[0], [player_move, 0, 0, 0]);
    teach(data, &mut party[0], [enemy_move, 0, 0, 0]);
    edit(&mut setup.party[0], &mut party[0]);
    Battle::start(data, &setup, rng).expect("battle setup")
}

/// Both sides use their move once; the HP each side lost.
fn one_turn(mut battle: Battle) -> (u16, u16) {
    let start = battle.state().clone();
    for _ in 0..2 {
        let Status::Decision(decision) = battle.status() else { break };
        let action = if decision.legal.contains(&Action::Move { slot: 0 }) { Action::Move { slot: 0 } } else { decision.legal[0] };
        assert!(matches!(decision.side, Side::Player | Side::Enemy));
        battle.step(action).expect("a legal action");
    }
    let end = battle.state();
    (start.player.active.hp - end.player.active.hp, start.enemy.active.hp - end.enemy.active.hp)
}

/// The damage `user`'s move deals to a Blastoise using Splash, with the user's
/// Attack or Special Attack lowered by `weaken`.
fn hit(data: &'static GameData, user: &str, mv: u8, weaken: Option<usize>) -> u16 {
    one_turn(link(data, (user, mv), ("BLASTOISE", SPLASH), |player, _| {
        if let Some(stat) = weaken {
            player.stats[stat] = 5;
        }
    }))
    .1
}

#[test]
fn ghost_moves_are_special_under_legacy_rules() {
    let legacy = legacy_with(Rules::CRYSTAL_LEGACY);
    let full = hit(legacy, "GENGAR", SHADOW_BALL, None);
    assert!(full > 0);
    assert_eq!(hit(legacy, "GENGAR", SHADOW_BALL, Some(ATTACK)), full);
    assert!(hit(legacy, "GENGAR", SHADOW_BALL, Some(SP_ATTACK)) < full);

    let crystal = legacy_with(Rules::CRYSTAL);
    let full = hit(crystal, "GENGAR", SHADOW_BALL, None);
    assert!(hit(crystal, "GENGAR", SHADOW_BALL, Some(ATTACK)) < full);
    assert_eq!(hit(crystal, "GENGAR", SHADOW_BALL, Some(SP_ATTACK)), full);
}

#[test]
fn dark_moves_are_physical_under_legacy_rules() {
    let legacy = legacy_with(Rules::CRYSTAL_LEGACY);
    let full = hit(legacy, "TYRANITAR", CRUNCH, None);
    assert!(full > 0);
    assert!(hit(legacy, "TYRANITAR", CRUNCH, Some(ATTACK)) < full);
    assert_eq!(hit(legacy, "TYRANITAR", CRUNCH, Some(SP_ATTACK)), full);

    let crystal = legacy_with(Rules::CRYSTAL);
    let full = hit(crystal, "TYRANITAR", CRUNCH, None);
    assert_eq!(hit(crystal, "TYRANITAR", CRUNCH, Some(ATTACK)), full);
    assert!(hit(crystal, "TYRANITAR", CRUNCH, Some(SP_ATTACK)) < full);
}

/// Whether `counter` (Counter or Mirror Coat) hits back after a faster
/// Blastoise's `mv`: Lapras and Blastoise take every move here normally.
fn counters(data: &'static GameData, counter: u8, mv: u8) -> bool {
    let (_, enemy_lost) = one_turn(link(data, ("LAPRAS", counter), ("BLASTOISE", mv), |player, _| {
        player.stats[0] = 999;
        player.hp = 999;
    }));
    enemy_lost > 0
}

#[test]
fn counter_takes_dark_moves_not_ghost_moves_under_legacy_rules() {
    let legacy = legacy_with(Rules::CRYSTAL_LEGACY);
    assert!(counters(legacy, COUNTER, CRUNCH));
    assert!(!counters(legacy, COUNTER, SHADOW_BALL));
    assert!(counters(legacy, MIRROR_COAT, SHADOW_BALL));
    assert!(!counters(legacy, MIRROR_COAT, CRUNCH));

    let crystal = legacy_with(Rules::CRYSTAL);
    assert!(!counters(crystal, COUNTER, CRUNCH));
    assert!(counters(crystal, COUNTER, SHADOW_BALL));
    assert!(!counters(crystal, MIRROR_COAT, SHADOW_BALL));
    assert!(counters(crystal, MIRROR_COAT, CRUNCH));
}

#[test]
fn return_and_frustration_have_power_under_legacy_rules() {
    const RETURN: u8 = 216;
    const FRUSTRATION: u8 = 218;
    let hit = |data: &'static GameData, mv: u8, happiness: u8| {
        one_turn(link(data, ("SNORLAX", mv), ("BLASTOISE", SPLASH), |player, _| player.happiness = happiness)).1
    };
    let legacy = legacy_with(Rules::CRYSTAL_LEGACY);
    assert!(hit(legacy, RETURN, 0) > 0);
    assert!(hit(legacy, FRUSTRATION, 255) > 0);
    let crystal = legacy_with(Rules::CRYSTAL);
    assert_eq!(hit(crystal, RETURN, 0), 0);
    assert_eq!(hit(crystal, FRUSTRATION, 255), 0);
}

#[test]
fn legacy_data_has_legacy_rules() {
    assert_eq!(GameData::crystal().rules, Rules::CRYSTAL);
    assert_eq!(GameData::crystal_legacy().rules, Rules::CRYSTAL_LEGACY);
}
