//! Differential test: random link battles on the original engine and the
//! hand-written engine, compared at every decision.
//!
//! cargo run --release --example engine_diff -- [link|wild|trainer] [battles] [first_seed] [move ids...]

use rgb2rust_output::engine::{diff, oracle, testgen};

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
    let moves = if moves.is_empty() { vec![33] } else { moves };
    let (mut ok, mut bad) = (0, 0);
    std::panic::set_hook(Box::new(|_| {}));
    for seed in first..first + battles {
        let config = match kind.as_str() {
            "wild" => testgen::random_wild_config(seed, &moves, Default::default()),
            "trainer" => testgen::random_trainer_config(seed, &moves, Default::default()),
            _ => testgen::random_link_config(seed, &moves, Default::default()),
        };
        let Some(trace) = oracle::run(&config, config.battle.seed.unwrap(), seed, 400) else {
            println!("seed {seed}: oracle battle too long, skipped");
            continue;
        };
        match diff::check(&trace, kind == "link", std::env::var_os("NO_TEXT").is_none()) {
            Ok(n) => {
                ok += 1;
                println!("seed {seed}: ok ({n} decisions)");
            }
            Err(d) => {
                bad += 1;
                let sites: Vec<_> = trace.decisions.iter().take(6).map(|d| format!("{:?}/{:?}", d.site, d.action)).collect();
                println!("seed {seed}: {d:?}\n    oracle decisions: {}", sites.join(", "));
            }
        }
    }
    println!("{ok} ok, {bad} diverged");
}
