//! Watch one battle in the terminal: a random agent plays a random trainer
//! battle and every event (its messages, HP-bar changes) is printed as it
//! happens. The library doesn't render text: messages print as data.
//!
//! cargo run --release -p crystal-battle --example watch -- [seed]

use crystal_battle::battle::{Battle, Event, Status};
use crystal_battle::data::random::random_battle_config;

fn show(events: &[Event]) {
    for event in events {
        match event {
            Event::Message(message) => println!("{message:?}"),
            Event::HpBar { target, from, to, max } => println!("    [{target:?} HP {from} → {to} / {max}]"),
            other => println!("    [{other:?}]"),
        }
    }
}

fn main() {
    let seed: u64 = std::env::args().nth(1).and_then(|s| s.parse().ok()).unwrap_or(1);
    let config = random_battle_config(seed);
    let engine_seed = config.battle.seed.expect("random configs carry their engine seed");
    let mut battle = Battle::new(config, engine_seed).expect("battle setup");
    let mut rng = seed | 1;
    loop {
        show(battle.events());
        let Status::Decision(decision) = battle.status() else { break };
        rng ^= rng << 13;
        rng ^= rng >> 7;
        rng ^= rng << 17;
        let action = decision.legal[(rng % decision.legal.len() as u64) as usize];
        println!("  > {action:?}");
        battle.step(action).expect("legal action");
    }
    println!("=> {:?}", battle.outcome().unwrap());
}
