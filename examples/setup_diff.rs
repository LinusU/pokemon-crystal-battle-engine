//! A battle started from a game's setup is the scenario's battle: for random
//! scenarios, `Battle::new(config)` (the scenario path) and
//! `Battle::start(setup)` with the setup `BattleSetup::from_config` makes
//! must agree on every decision (site, legal actions, state, events), the
//! battle memory at each, the outcome and the result.
//!
//! cargo build --release --example setup_diff
//! ./target/release/examples/setup_diff [link|wild|trainer] <battles> <first_seed>

use crystal_battle_harness::battle::{Battle, BattleSetup, Status};
use crystal_battle_harness::data::GameData;
use crystal_battle_harness::oracle::{diff, mix, testgen};

fn main() {
    let mut args = std::env::args().skip(1).peekable();
    let kind = match args.peek().map(String::as_str) {
        Some(k @ ("link" | "wild" | "trainer")) => {
            let k = k.to_string();
            args.next();
            k
        }
        _ => "link".to_string(),
    };
    let battles: u64 = args.next().and_then(|s| s.parse().ok()).unwrap_or(100);
    let first: u64 = args.next().and_then(|s| s.parse().ok()).unwrap_or(1);
    let moves: Vec<u8> = (1..=251).collect();
    let (mut ok, mut bad, mut skipped) = (0, 0, 0);
    for seed in first..first + battles {
        let config = match kind.as_str() {
            "wild" => testgen::random_wild_config(seed, &moves, Default::default()),
            "trainer" => testgen::random_trainer_config(seed, &moves, Default::default()),
            _ => testgen::random_link_config(seed, &moves, Default::default()),
        };
        let engine_seed = config.battle.seed.unwrap();
        let Ok(scenario) = Battle::new(config.clone(), engine_seed) else {
            skipped += 1;
            continue;
        };
        match compare(scenario, &config, engine_seed, seed) {
            Ok(n) => {
                println!("seed {seed}: ok ({n} decisions)");
                ok += 1;
            }
            Err(e) => {
                println!("seed {seed}: {e}");
                bad += 1;
            }
        }
    }
    println!("{ok} ok, {bad} differ, {skipped} skipped");
}

fn compare(mut a: Battle, config: &crystal_battle_harness::config::BattleConfig, seed: u64, policy: u64) -> Result<usize, String> {
    let (setup, rng) = BattleSetup::from_config(config, seed)?;
    let mut b = Battle::start(GameData::crystal(), &setup, rng).map_err(|e| format!("setup start: {e}"))?;
    let mut policy = policy;
    for n in 0..400 {
        let mem = diff::mem_diffs(&a.engine().engine().mem, &b.engine().engine().mem);
        if !mem.is_empty() {
            return Err(format!("memory at decision {n}: {mem:?}"));
        }
        match (a.status(), b.status()) {
            (Status::Decision(x), Status::Decision(y)) => {
                if x != y {
                    return Err(format!("decision {n} differs:\n  scenario {x:?}\n  setup    {y:?}"));
                }
                let action = x.legal[(mix(&mut policy) % x.legal.len() as u64) as usize];
                a.step(action).map_err(|e| e.to_string())?;
                b.step(action).map_err(|e| e.to_string())?;
            }
            (x, y) if x == y => {
                if a.result() != b.result() {
                    return Err(format!("result differs:\n  scenario {:?}\n  setup    {:?}", a.result(), b.result()));
                }
                return Ok(n);
            }
            (x, y) => return Err(format!("end differs after {n}:\n  scenario {x:?}\n  setup    {y:?}")),
        }
    }
    Ok(400)
}
