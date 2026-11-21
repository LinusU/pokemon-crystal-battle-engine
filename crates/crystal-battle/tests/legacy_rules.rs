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
    player: (&str, u8),
    enemy: (&str, u8),
    edit: impl FnOnce(&mut Pokemon, &mut Pokemon),
) -> Battle {
    link_seeded(data, 1, player, enemy, edit)
}

/// [`link`] with battle seed `seed`.
fn link_seeded(
    data: &'static GameData,
    seed: u64,
    (player, player_move): (&str, u8),
    (enemy, enemy_move): (&str, u8),
    edit: impl FnOnce(&mut Pokemon, &mut Pokemon),
) -> Battle {
    let config = BattleConfig {
        battle: BattleHeader {
            kind: BattleKind::LinkBattle,
            trainer: None,
            seed: Some(seed),
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
    let (mut setup, rng) = BattleSetup::from_config_in(data, &config, seed).expect("a valid scenario");
    let Opponent::Link { party, .. } = &mut setup.opponent else { unreachable!() };
    teach(data, &mut setup.party[0], [player_move, 0, 0, 0]);
    teach(data, &mut party[0], [enemy_move, 0, 0, 0]);
    edit(&mut setup.party[0], &mut party[0]);
    Battle::start(data, &setup, rng).expect("battle setup")
}

/// Both sides use their move once; the HP each side lost.
fn one_turn(mut battle: Battle) -> (u16, u16) {
    one_turn_on(&mut battle)
}

/// [`one_turn`], keeping the battle.
fn one_turn_on(battle: &mut Battle) -> (u16, u16) {
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

/// Whether a wild level-2 Abra (it knows only Teleport) gets away from a
/// level-100 Snorlax using Splash, with battle seed `seed`.
fn abra_teleports(data: &'static GameData, seed: u64) -> bool {
    let config = BattleConfig {
        battle: BattleHeader {
            kind: BattleKind::WildEncounter,
            trainer: None,
            seed: Some(seed),
            johto_badges: None,
            kanto_badges: None,
            battle_scene: None,
            enemy_control: EnemyControl::Original,
        },
        player_party: vec![MonSpec { level: 100, ..mon("SNORLAX") }],
        enemy_party: vec![MonSpec { level: 2, ..mon("ABRA") }],
        player_items: vec![],
        enemy_items: vec![],
        enemy_trainer: None,
    };
    let (mut setup, rng) = BattleSetup::from_config_in(data, &config, seed).expect("a valid scenario");
    teach(data, &mut setup.party[0], [SPLASH, 0, 0, 0]);
    let mut battle = Battle::start(data, &setup, rng).expect("battle setup");
    battle.step(Action::Move { slot: 0 }).expect("a legal action");
    battle.outcome().is_some()
}

#[test]
fn wild_teleport_can_fail_under_legacy_rules() {
    let crystal = legacy_with(Rules::CRYSTAL);
    assert!((1..40).all(|seed| abra_teleports(crystal, seed)));
    let legacy = legacy_with(Rules::CRYSTAL_LEGACY);
    let escapes = (1..40).filter(|&seed| abra_teleports(legacy, seed)).count();
    assert!(escapes > 0 && escapes < 39, "{escapes} of 39 escaped");
}

#[test]
fn belly_drum_needs_the_hp_first_under_legacy_rules() {
    const BELLY_DRUM: u8 = 187;
    let attack_level = |data: &'static GameData| {
        let mut battle = link(data, ("SNORLAX", BELLY_DRUM), ("BLASTOISE", SPLASH), |player, _| player.hp = 1);
        one_turn_on(&mut battle);
        battle.state().player.stat_levels[0]
    };
    assert_eq!(attack_level(legacy_with(Rules::CRYSTAL_LEGACY)), 7);
    assert_eq!(attack_level(legacy_with(Rules::CRYSTAL)), 9);
}

#[test]
fn light_ball_caps_the_stat_under_legacy_rules() {
    use crystal_battle::defs::{LIGHT_BALL, THUNDERBOLT};
    let hit = |data: &'static GameData, sp_attack: u16| {
        one_turn(link(data, ("PIKACHU", THUNDERBOLT), ("BLASTOISE", SPLASH), |player, enemy| {
            player.item = LIGHT_BALL;
            player.stats[SP_ATTACK] = sp_attack;
            (enemy.stats[0], enemy.hp, enemy.stats[5]) = (999, 999, 999);
        }))
        .1
    };
    let legacy = legacy_with(Rules::CRYSTAL_LEGACY);
    assert_eq!(hit(legacy, 600), hit(legacy, 999));
    let crystal = legacy_with(Rules::CRYSTAL);
    assert_ne!(hit(crystal, 600), hit(crystal, 999));
}

#[test]
fn psywave_deals_at_least_the_level_under_legacy_rules() {
    const PSYWAVE: u8 = 149;
    let lowest = |data: &'static GameData| {
        // (A miss deals nothing.)
        (1..60)
            .map(|seed| one_turn(link_seeded(data, seed, ("ALAKAZAM", PSYWAVE), ("BLASTOISE", SPLASH), |_, _| {})).1)
            .filter(|&damage| damage > 0)
            .min()
            .unwrap()
    };
    assert!(lowest(legacy_with(Rules::CRYSTAL_LEGACY)) >= 50);
    assert!(lowest(legacy_with(Rules::CRYSTAL)) < 50);
}

#[test]
fn legacy_data_has_legacy_rules() {
    assert_eq!(GameData::crystal().rules, Rules::CRYSTAL);
    assert_eq!(GameData::crystal_legacy().rules, Rules::CRYSTAL_LEGACY);
}
