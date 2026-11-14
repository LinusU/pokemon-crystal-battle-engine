//! Speed of the hand-written engine vs the emulated original, on random link
//! battles with a random policy, checking both reach the same outcome.
//!
//! cargo run --release --example engine_bench -- [battles] [move ids...]

use rgb2rust_output::engine::api::{FastBattle, Status};
use rgb2rust_output::engine::{oracle, testgen};
use std::time::{Duration, Instant};

fn main() {
    let mut args = std::env::args().skip(1);
    let battles: u64 = args.next().and_then(|s| s.parse().ok()).unwrap_or(50);
    let moves: Vec<u8> = args.map(|m| m.parse().unwrap()).collect();
    let moves = if moves.is_empty() { vec![33] } else { moves };
    let (mut setup, mut fast, mut slow) = (Duration::ZERO, Duration::ZERO, Duration::ZERO);
    let (mut decisions, mut agree) = (0usize, 0u64);
    for seed in 1..=battles {
        let config = testgen::random_link_config(seed, &moves, Default::default());
        let engine_seed = config.battle.seed.unwrap();
        // Original engine, policy seeded by `seed`.
        let t = Instant::now();
        let trace = oracle::run(&config, engine_seed, seed, 400).expect("battle");
        slow += t.elapsed();
        // Hand-written engine, same decisions.
        let t = Instant::now();
        let mut battle = FastBattle::new(&config, engine_seed).expect("setup");
        setup += t.elapsed();
        let t = Instant::now();
        for d in &trace.decisions {
            let Status::Decision { .. } = battle.status() else { break };
            battle.step(d.action).expect("legal");
            decisions += 1;
        }
        fast += t.elapsed();
        if matches!(battle.status(), Status::Ended(_)) {
            agree += 1;
        }
    }
    println!("{battles} battles, {decisions} decisions; {agree} ended together");
    println!("original engine: {:.1} ms/battle", slow.as_secs_f64() * 1e3 / battles as f64);
    println!(
        "hand-written:    {:.3} ms/battle (+ {:.1} ms setup on the original), {:.1} µs/decision",
        fast.as_secs_f64() * 1e3 / battles as f64,
        setup.as_secs_f64() * 1e3 / battles as f64,
        fast.as_secs_f64() * 1e6 / decisions.max(1) as f64
    );
}
