//! Watch one battle in the terminal: a random agent plays a random trainer
//! battle and every event (the original battle text, HP-bar changes) is
//! printed as it happens.
//!
//! cargo run --release --example watch -- [seed]

use crystal_battle_harness::battle::{Battle, Event, Status, one_line};
use crystal_battle_harness::bootstrap::random::random_battle_config;

fn show(events: &[Event]) {
    for event in events {
        match event {
            Event::Text(text) => println!("{}", one_line(text)),
            Event::HpBar { target, from, to, max } => println!("    [{target:?} HP {from} → {to} / {max}]"),
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
