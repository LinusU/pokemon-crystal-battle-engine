//! A random agent: plays randomly generated trainer battles by picking a
//! uniformly random legal action at every decision.
//!
//! cargo run --release --example random_agent -- [battles] [first_seed]

use rgb2rust_output::battle::{Battle, Status};
use rgb2rust_output::bootstrap::names;
use rgb2rust_output::bootstrap::random::random_battle_config;

/// splitmix64 — the agent's own randomness, independent of the engine's.
fn next(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}

fn main() {
    let mut args = std::env::args().skip(1);
    let battles: u64 = args.next().and_then(|s| s.parse().ok()).unwrap_or(10);
    let first: u64 = args.next().and_then(|s| s.parse().ok()).unwrap_or(1);
    let started = std::time::Instant::now();
    let mut decisions = 0usize;
    for seed in first..first + battles {
        let config = random_battle_config(seed);
        let engine_seed = config.battle.seed.expect("random configs carry their engine seed");
        let mut battle = Battle::new(config, engine_seed).expect("battle setup");
        let mut rng = seed;
        while let Status::Decision(decision) = battle.status() {
            let pick = (next(&mut rng) % decision.legal.len() as u64) as usize;
            let action = decision.legal[pick];
            decisions += 1;
            battle.step(action).expect("legal action");
        }
        let state = battle.state();
        println!(
            "seed {seed}: {:?} — {} vs {}",
            battle.outcome().unwrap(),
            names::species_name(state.player.active.species),
            names::species_name(state.enemy.active.species),
        );
    }
    let secs = started.elapsed().as_secs_f64();
    println!(
        "{battles} battles, {decisions} decisions in {secs:.1}s ({:.2} battles/s)",
        battles as f64 / secs
    );
}
