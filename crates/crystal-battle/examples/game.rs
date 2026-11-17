//! A battle the way a game drives it: hand the library a setup (here made
//! from a random scenario, then edited the way a save would differ: a
//! nickname, money, the Pokédex), pump its events, make the decisions, and
//! take the result back.
//!
//! cargo run --release -p crystal-battle --example game -- [seed]

use crystal_battle::battle::{Battle, BattleSetup, Event, Status};
use crystal_battle::data::GameData;
use crystal_battle::data::random::random_battle_config;

fn main() {
    let seed: u64 = std::env::args().nth(1).and_then(|s| s.parse().ok()).unwrap_or(7);
    let config = random_battle_config(seed);
    let (mut setup, rng): (BattleSetup, u64) =
        BattleSetup::from_config(&config, config.battle.seed.unwrap()).expect("a valid scenario");
    setup.player.name = "GOLD".into();
    setup.player.money = 3000;
    setup.party[0].nickname = "BUDDY".into();
    setup.world.caught = setup.party.iter().map(|mon| mon.species).collect();

    let data = GameData::crystal();
    let mut battle = Battle::start(data, &setup, rng).expect("battle setup");
    let mut policy = seed | 1;
    loop {
        for event in battle.events() {
            match event {
                Event::Message(message) => println!("{message:?}"),
                Event::Animation { anim, user, param } => println!("    (animation {anim} from {user:?}, {param})"),
                other => println!("    ({other:?})"),
            }
        }
        let Status::Decision(decision) = battle.status() else { break };
        policy ^= policy << 13;
        policy ^= policy >> 7;
        policy ^= policy << 17;
        let action = decision.legal[(policy % decision.legal.len() as u64) as usize];
        println!("> {:?} chooses {action:?}", decision.side);
        battle.step(action).expect("a legal action");
    }

    let result = battle.result().expect("the battle has ended");
    println!("\n{:?}: money {} → {}", result.outcome, setup.player.money, result.money);
    for mon in &result.party {
        let species = data.names.species[usize::from(mon.species)];
        println!("  {} ({species}) L{} {}/{} HP, {} exp", mon.nickname, mon.level, mon.hp, mon.stats[0], mon.exp);
    }
    if !result.may_evolve.is_empty() {
        println!("  may evolve: {:?}", result.may_evolve);
    }
}
