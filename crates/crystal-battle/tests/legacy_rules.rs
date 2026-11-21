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

/// Crystal ends a lone Beat Up at `endloop`, before King's Rock; Crystal
/// Legacy plays the rest of the move.
#[test]
fn a_lone_beat_up_reaches_kings_rock_under_legacy_rules() {
    use crystal_battle::battle::{Event, Message};
    use crystal_battle::defs::{BEAT_UP, KINGS_ROCK};
    let flinches = |data: &'static GameData| {
        (1..80)
            .filter(|&seed| {
                let mut battle =
                    link_seeded(data, seed, ("SNEASEL", BEAT_UP), ("SLOWBRO", SPLASH), |player, _| player.item = KINGS_ROCK);
                let Status::Decision(_) = battle.status() else { unreachable!() };
                battle.step(Action::Move { slot: 0 }).unwrap();
                battle.step(Action::Move { slot: 0 }).unwrap();
                battle.events().iter().any(|e| matches!(e, Event::Message(Message::Flinched { .. })))
            })
            .count()
    };
    assert!(flinches(legacy_with(Rules::CRYSTAL_LEGACY)) > 0);
    assert_eq!(flinches(legacy_with(Rules::CRYSTAL)), 0);
}

/// The HP a confused side loses hurting itself on turn 1 (after a faster
/// Gengar's Confuse Ray), for the seeds where it does.
fn self_hits(data: &'static GameData, enemy_confused: bool, bow: bool) -> Vec<(u64, u16)> {
    use crystal_battle::battle::{Event, Message};
    use crystal_battle::defs::{CONFUSE_RAY, PINK_BOW, TACKLE};
    (1..60)
        .filter_map(|seed| {
            let (player, enemy) = if enemy_confused { (("GENGAR", CONFUSE_RAY), ("SNORLAX", TACKLE)) } else { (("SNORLAX", TACKLE), ("GENGAR", CONFUSE_RAY)) };
            let mut battle = link_seeded(data, seed, player, enemy, |player, enemy| {
                let snorlax = if enemy_confused { enemy } else { player };
                if bow {
                    snorlax.item = PINK_BOW;
                }
            });
            let (player_lost, enemy_lost) = one_turn_on(&mut battle);
            let hurt = battle.events().iter().any(|e| matches!(e, Event::Message(Message::HurtItself)));
            hurt.then_some((seed, if enemy_confused { enemy_lost } else { player_lost }))
        })
        .collect()
}

#[test]
fn confusion_ignores_type_boosting_items_under_legacy_rules() {
    let legacy = legacy_with(Rules::CRYSTAL_LEGACY);
    let hits = self_hits(legacy, false, false);
    assert!(!hits.is_empty());
    assert_eq!(self_hits(legacy, false, true), hits);
    let crystal = legacy_with(Rules::CRYSTAL);
    assert_ne!(self_hits(crystal, false, true), self_hits(crystal, false, false));
}

/// Crystal Legacy's enemy runs the damage update twice: 2 more damage
/// (under 256) than Crystal's.
#[test]
fn an_enemy_hurts_itself_twice_over_under_legacy_rules() {
    let crystal = self_hits(legacy_with(Rules::CRYSTAL), true, false);
    let legacy = self_hits(legacy_with(Rules::CRYSTAL_LEGACY), true, false);
    assert!(!crystal.is_empty());
    assert_eq!(legacy, crystal.iter().map(|&(seed, hp)| (seed, hp + 2)).collect::<Vec<_>>());
}

/// The messages of the player's `mv` on a Blastoise whose status is `status`.
fn messages_against(data: &'static GameData, user: &str, mv: u8, status: u8) -> Vec<crystal_battle::battle::Message> {
    use crystal_battle::battle::Event;
    let mut battle = link(data, (user, mv), ("BLASTOISE", SPLASH), |_, enemy| enemy.status = status);
    let mut messages = vec![];
    for _ in 0..2 {
        let Status::Decision(_) = battle.status() else { break };
        battle.step(Action::Move { slot: 0 }).unwrap();
        messages.extend(battle.events().iter().filter_map(|e| if let Event::Message(m) = e { Some(m.clone()) } else { None }));
    }
    messages
}

#[test]
fn status_moves_name_the_status_already_there_under_legacy_rules() {
    use crystal_battle::battle::Message as M;
    use crystal_battle::defs::{BRN, FRZ, HYPNOSIS, PAR, POISONPOWDER, THUNDER_WAVE};
    let legacy = legacy_with(Rules::CRYSTAL_LEGACY);
    let crystal = legacy_with(Rules::CRYSTAL);
    let cases: [(&str, u8, u8, fn(&M) -> bool); 3] = [
        ("PIKACHU", THUNDER_WAVE, 1 << BRN, |m| matches!(m, M::AlreadyBurned { .. })),
        ("GENGAR", HYPNOSIS, 1 << FRZ, |m| matches!(m, M::AlreadyFrozen { .. })),
        ("VILEPLUME", POISONPOWDER, 1 << PAR, |m| matches!(m, M::AlreadyParalyzed { .. })),
    ];
    for (user, mv, status, already) in cases {
        let messages = messages_against(legacy, user, mv, status);
        assert!(messages.iter().any(already), "{user}: {messages:?}");
        let messages = messages_against(crystal, user, mv, status);
        assert!(!messages.iter().any(already), "{user}: {messages:?}");
        assert!(messages.iter().any(|m| matches!(m, M::DidntAffect { .. })), "{user}: {messages:?}");
    }
}

#[test]
fn the_zephyr_badge_commands_obedience_under_legacy_rules() {
    use crystal_battle::battle::{Event, Message as M};
    use crystal_battle::defs::{TACKLE, ZEPHYRBADGE};
    let disobeys = |data: &'static GameData| {
        (1..40)
            .filter(|&seed| {
                let config = BattleConfig {
                    battle: BattleHeader {
                        kind: BattleKind::WildEncounter,
                        trainer: None,
                        seed: Some(seed),
                        johto_badges: Some(1 << ZEPHYRBADGE),
                        kanto_badges: None,
                        battle_scene: None,
                        enemy_control: EnemyControl::Original,
                    },
                    player_party: vec![MonSpec { level: 15, ot_id: Some(1), ..mon("RATTATA") }],
                    enemy_party: vec![MonSpec { level: 10, ..mon("MAGIKARP") }],
                    player_items: vec![],
                    enemy_items: vec![],
                    enemy_trainer: None,
                };
                let (mut setup, rng) = BattleSetup::from_config_in(data, &config, seed).unwrap();
                teach(data, &mut setup.party[0], [TACKLE, 0, 0, 0]);
                let mut battle = Battle::start(data, &setup, rng).unwrap();
                battle.step(Action::Move { slot: 0 }).unwrap();
                battle.events().iter().any(|e| {
                    matches!(e, Event::Message(M::WontObey { .. } | M::IgnoredOrders { .. } | M::LoafingAround { .. } | M::TurnedAway { .. } | M::BeganToNap { .. }))
                })
            })
            .count()
    };
    assert_eq!(disobeys(legacy_with(Rules::CRYSTAL_LEGACY)), 0);
    assert!(disobeys(legacy_with(Rules::CRYSTAL)) > 0);
}

/// A poisoned Pokémon with 1 HP switches out into a Pursuit: its status
/// afterwards.
fn pursued_status(data: &'static GameData) -> u8 {
    use crystal_battle::defs::{PSN, PURSUIT};
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
        player_party: vec![mon("SNORLAX"), mon("LAPRAS")],
        enemy_party: vec![mon("TYRANITAR")],
        player_items: vec![],
        enemy_items: vec![],
        enemy_trainer: None,
    };
    let (mut setup, rng) = BattleSetup::from_config_in(data, &config, 1).expect("a valid scenario");
    let Opponent::Link { party, .. } = &mut setup.opponent else { unreachable!() };
    teach(data, &mut party[0], [PURSUIT, 0, 0, 0]);
    (setup.party[0].hp, setup.party[0].status) = (1, 1 << PSN);
    let mut battle = Battle::start(data, &setup, rng).expect("battle setup");
    battle.step(Action::Switch { slot: 1 }).expect("a legal switch");
    battle.step(Action::Move { slot: 0 }).expect("a legal move");
    let fainted = &battle.state().player.party[0];
    assert_eq!(fainted.hp, 0);
    fainted.status
}

#[test]
fn pursuit_cures_the_fainted_pokemon_under_legacy_rules() {
    assert_eq!(pursued_status(legacy_with(Rules::CRYSTAL_LEGACY)), 0);
    assert_ne!(pursued_status(legacy_with(Rules::CRYSTAL)), 0);
}

/// A trainer battle (a random scenario's trainer) against a Snorlax of the
/// party's own, after `edit`: the battle state once it is sent out.
fn trainer_snorlax(data: &'static GameData, edit: impl FnOnce(&mut Pokemon)) -> crystal_battle::battle::BattleState {
    use crystal_battle::data::random::random_battle_config;
    let mut config = random_battle_config(1);
    config.enemy_party = vec![mon("SNORLAX")];
    config.player_party = vec![mon("BLASTOISE")];
    let (mut setup, rng) = BattleSetup::from_config_in(data, &config, 1).expect("a valid scenario");
    let snorlax_config = BattleConfig { player_party: vec![mon("SNORLAX")], ..config.clone() };
    let mut snorlax = BattleSetup::from_config_in(data, &snorlax_config, 1).expect("a valid scenario").0.party[0].clone();
    edit(&mut snorlax);
    let Opponent::Trainer { party, .. } = &mut setup.opponent else { panic!("a trainer battle") };
    *party = Some(vec![snorlax]);
    Battle::start(data, &setup, rng).expect("battle setup").state().clone()
}

#[test]
fn trainer_pokemon_bring_their_dvs_and_happiness_under_legacy_rules() {
    let edit = |snorlax: &mut Pokemon| {
        snorlax.dvs = [0x12, 0x34];
        snorlax.happiness = 200;
    };
    let legacy = trainer_snorlax(legacy_with(Rules::CRYSTAL_LEGACY), edit);
    assert_eq!((legacy.enemy.active.dvs, legacy.enemy.active.happiness), (0x1234, 200));
    let crystal = trainer_snorlax(legacy_with(Rules::CRYSTAL), edit);
    assert_ne!(crystal.enemy.active.dvs, 0x1234);
    assert_ne!(crystal.enemy.active.happiness, 200);
}

#[test]
fn a_paralyzed_enemy_is_slowed_when_sent_out_under_legacy_rules() {
    use crystal_battle::defs::PAR;
    const SPEED: usize = 2;
    let speed = |data: &'static GameData, status: u8| trainer_snorlax(data, |snorlax| snorlax.status = status).enemy.active.stats[SPEED];
    let legacy = legacy_with(Rules::CRYSTAL_LEGACY);
    assert_eq!(speed(legacy, 1 << PAR), (speed(legacy, 0) / 4).max(1));
    let crystal = legacy_with(Rules::CRYSTAL);
    assert_eq!(speed(crystal, 1 << PAR), speed(crystal, 0));
}

#[test]
fn legacy_data_has_legacy_rules() {
    assert_eq!(GameData::crystal().rules, Rules::CRYSTAL);
    assert_eq!(GameData::crystal_legacy().rules, Rules::CRYSTAL_LEGACY);
}
