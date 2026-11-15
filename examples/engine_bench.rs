//! Battles per second: the hand-written engine vs the emulated original, on
//! random battles with a random policy (every battle kind, all moves).
//!
//! cargo build --release --example engine_bench
//! ./target/release/examples/engine_bench [battles per kind]
//!
//! Measured per kind, single-threaded, setup included:
//! - `FastBattle`: the engine with state only (what a search or a training
//!   loop that doesn't need text would use);
//! - `Battle`: the library API (text and HP-bar events, typed state);
//! - `EmulatedBattle`: the original code on the emulated CPU.

use rgb2rust_output::battle::{Battle, EmulatedBattle, Status};
use rgb2rust_output::bootstrap::BattleConfig;
use rgb2rust_output::engine::api::{FastBattle, Status as FastStatus};
use rgb2rust_output::oracle::{mix, testgen};
use std::time::Instant;

fn main() {
    let battles: u64 = std::env::args().nth(1).and_then(|s| s.parse().ok()).unwrap_or(100);
    let moves: Vec<u8> = (1..=251).collect();
    println!("{:<8} {:>14} {:>14} {:>16} {:>9}", "kind", "FastBattle/s", "Battle/s", "EmulatedBattle/s", "speedup");
    for kind in ["link", "wild", "trainer"] {
        let configs: Vec<BattleConfig> = (1..=battles)
            .map(|seed| match kind {
                "wild" => testgen::random_wild_config(seed, &moves, Default::default()),
                "trainer" => testgen::random_trainer_config(seed, &moves, Default::default()),
                _ => testgen::random_link_config(seed, &moves, Default::default()),
            })
            .collect();
        let fast = rate(&configs, |c, policy| {
            let mut b = FastBattle::new(c, c.battle.seed.unwrap()).expect("setup");
            let mut policy = policy;
            for _ in 0..400 {
                let FastStatus::Decision { legal, .. } = b.status() else { break };
                let action = legal[(mix(&mut policy) % legal.len() as u64) as usize];
                b.step(action).expect("legal");
            }
        });
        let api = rate(&configs, |c, policy| {
            let mut b = Battle::new(c.clone(), c.battle.seed.unwrap()).expect("setup");
            let mut policy = policy;
            for _ in 0..400 {
                let Status::Decision(d) = b.status() else { break };
                let action = d.legal[(mix(&mut policy) % d.legal.len() as u64) as usize];
                b.step(action).expect("step");
            }
        });
        let slow = rate(&configs, |c, policy| {
            let mut b = EmulatedBattle::new(c.clone(), c.battle.seed.unwrap()).expect("setup");
            let mut policy = policy;
            for _ in 0..400 {
                let Status::Decision(d) = b.status() else { break };
                let action = d.legal[(mix(&mut policy) % d.legal.len() as u64) as usize];
                b.step(action).expect("step");
            }
        });
        println!("{kind:<8} {fast:>14.0} {api:>14.0} {slow:>16.1} {:>8.0}x", fast / slow);
    }
}

/// Battles per second running `play` over `configs` (policy seeded by index).
fn rate(configs: &[BattleConfig], play: impl Fn(&BattleConfig, u64)) -> f64 {
    let t = Instant::now();
    for (i, c) in configs.iter().enumerate() {
        play(c, i as u64 + 1);
    }
    configs.len() as f64 / t.elapsed().as_secs_f64()
}
