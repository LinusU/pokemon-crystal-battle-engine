//! The library API on both engines, side by side: `Battle` (the hand-written
//! engine) and `EmulatedBattle` (the original code on the emulated CPU) play
//! the same random battles with the same random policy; every `Decision`
//! (site, side, legal actions, typed state, events) and the outcome must be
//! equal.
//!
//! cargo build --release --example api_diff
//! ./target/release/examples/api_diff [link|wild|trainer] <battles> <first_seed> [moves...]

use crystal_battle_harness::battle::{Battle, EmulatedBattle, Status};
use crystal_battle_harness::oracle::{mix, testgen};

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
    let battles: u64 = args.next().and_then(|s| s.parse().ok()).unwrap_or(20);
    let first: u64 = args.next().and_then(|s| s.parse().ok()).unwrap_or(1);
    let moves: Vec<u8> = args.map(|m| m.parse().unwrap()).collect();
    let moves = if moves.is_empty() { (1..=251).collect() } else { moves };
    let (mut ok, mut bad) = (0, 0);
    for seed in first..first + battles {
        let config = match kind.as_str() {
            "wild" => testgen::random_wild_config(seed, &moves, Default::default()),
            "trainer" => testgen::random_trainer_config(seed, &moves, Default::default()),
            _ => testgen::random_link_config(seed, &moves, Default::default()),
        };
        let engine_seed = config.battle.seed.unwrap();
        match compare(&config, engine_seed, seed) {
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
    println!("{ok} ok, {bad} differ");
}

fn compare(config: &crystal_battle_harness::battle::BattleConfig, seed: u64, policy: u64) -> Result<usize, String> {
    let mut fast = Battle::new(config.clone(), seed).map_err(|e| format!("engine setup: {e}"))?;
    let mut slow = EmulatedBattle::new(config.clone(), seed).map_err(|e| format!("oracle setup: {e}"))?;
    let mut policy = policy;
    for n in 0..400 {
        match (fast.status(), slow.status()) {
            (Status::Decision(a), Status::Decision(b)) => {
                if a != b {
                    return Err(format!("decision {n} differs: {}", field_diff(&format!("{a:?}"), &format!("{b:?}"))));
                }
                let action = a.legal[(mix(&mut policy) % a.legal.len() as u64) as usize];
                fast.step(action).map_err(|e| format!("engine step {n}: {e}"))?;
                slow.step(action).map_err(|e| format!("oracle step {n}: {e}"))?;
            }
            (a, b) if a == b => return Ok(n),
            (a, b) => return Err(format!("end differs after {n}: {}", field_diff(&format!("{a:?}"), &format!("{b:?}")))),
        }
    }
    Ok(400)
}

/// The first place two Debug renderings differ, with some context.
fn field_diff(a: &str, b: &str) -> String {
    let i = a.bytes().zip(b.bytes()).take_while(|(x, y)| x == y).count();
    let from = a[..i].rfind(|c| c == ',' || c == '{' || c == '[').map_or(0, |p| p.saturating_sub(60));
    let end = |s: &str| (i + 120).min(s.len());
    format!("…{} | engine: {} | oracle: {}", &a[from..i], &a[i..end(a)], &b[i..end(b)])
}
