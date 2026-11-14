//! Replay recorded fixtures on the original engine and the hand-written one in
//! lockstep: the recording's decisions drive both, and every decision point is
//! compared (site, legal actions, random stream, memory, battle text).
//!
//! cargo build --release --example engine_fixtures
//! ./target/release/examples/engine_fixtures fixtures/*.toml

use rgb2rust_output::bootstrap::player;
use rgb2rust_output::engine::{diff, oracle};
use std::path::Path;

fn main() {
    let paths: Vec<String> = std::env::args().skip(1).filter(|p| !p.ends_with(".scenario.toml")).collect();
    std::panic::set_hook(Box::new(|_| {}));
    let (mut ok, mut failed) = (0, 0);
    for path in &paths {
        let recording = match player::load(Path::new(path)) {
            Ok(r) => r,
            Err(e) => {
                println!("{path}: could not load: {e}");
                failed += 1;
                continue;
            }
        };
        let Some(mut config) = recording.scenario.clone() else {
            println!("{path}: no embedded scenario, skipped");
            continue;
        };
        config.battle.seed = Some(recording.seed);
        let script = recording.steps.iter().map(|s| s.step).collect();
        let traced = std::panic::catch_unwind(|| oracle::run_scripted(&config, recording.seed, script));
        let trace = match traced {
            Ok(Some(trace)) => trace,
            Ok(None) => {
                println!("{path}: oracle froze");
                failed += 1;
                continue;
            }
            Err(payload) => {
                let msg = payload.downcast_ref::<String>().cloned().unwrap_or_default();
                println!("{path}: oracle panicked: {msg}");
                failed += 1;
                continue;
            }
        };
        match diff::check(&config, &trace, true) {
            Ok(n) => {
                println!("{path}: ok ({n} decisions)");
                ok += 1;
            }
            Err(d) => {
                println!("{path}: {d:?}");
                failed += 1;
            }
        }
    }
    println!("{ok} ok, {failed} failed");
}
