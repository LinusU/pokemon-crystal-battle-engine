//! Differential test: random link battles on the original engine and the
//! hand-written engine, compared at every decision.
//!
//! cargo run --release --example engine_diff -- [link|wild|trainer|external] [battles] [first_seed] [move ids...]
//!
//! `DIFF_PANICS=1` keeps the panic hook, to see where the oracle panics.
//!
//! `external` is a trainer battle with the trainer controlled from outside
//! (`enemy_control = "external"`): the random policy plays both sides.

use crystal_battle_harness::oracle::{self, diff, testgen};

fn main() {
    let mut args = std::env::args().skip(1).peekable();
    let kind = match args.peek().map(String::as_str) {
        Some(k @ ("link" | "wild" | "trainer" | "external")) => {
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
    let opts = testgen::GenOptions { items: std::env::var_os("NO_ITEMS").is_none(), ..Default::default() };
    let (mut ok, mut bad, mut unported) = (0, 0, 0);
    let (mut start_ok, mut start_before_ai, mut start_bad, mut unsupported) = (0, 0, 0, 0);
    if std::env::var_os("DIFF_PANICS").is_none() {
        std::panic::set_hook(Box::new(|_| {}));
    }
    for seed in first..first + battles {
        let config = match kind.as_str() {
            "wild" => testgen::random_wild_config(seed, &moves, opts),
            "trainer" => testgen::random_trainer_config(seed, &moves, opts),
            "external" => testgen::random_external_trainer_config(seed, &moves, opts),
            _ => testgen::random_link_config(seed, &moves, opts),
        };
        // A few trainer scenarios make the oracle itself panic (a mobile script
        // the harness can't run): skip them like the frozen ones.
        let traced = std::panic::catch_unwind(|| oracle::run(&config, config.battle.seed.unwrap(), seed, 400));
        let trace = match traced {
            Ok(Some(trace)) => trace,
            Ok(None) => {
                println!("seed {seed}: oracle battle too long or frozen, skipped");
                continue;
            }
            Err(payload) => {
                let msg = payload
                    .downcast_ref::<String>()
                    .cloned()
                    .or_else(|| payload.downcast_ref::<&str>().map(|s| s.to_string()))
                    .unwrap_or_default();
                println!("seed {seed}: oracle panicked ({msg}), skipped");
                continue;
            }
        };
        let check_text = std::env::var_os("NO_TEXT").is_none();
        // First the engine's own start state against the oracle's first decision.
        match diff::check_start(&config, &trace, check_text) {
            Ok(diff::StartCheck::Exact) => start_ok += 1,
            Ok(diff::StartCheck::BeforeFirstTurn) => start_before_ai += 1,
            Err(diff::Divergence::Unsupported { reason }) => {
                unsupported += 1;
                println!("seed {seed}: start not supported by the engine ({reason}), skipped");
                continue;
            }
            Err(d) => {
                start_bad += 1;
                println!("seed {seed}: start state differs: {d:?}");
                continue;
            }
        }
        match diff::check(&config, &trace, check_text) {
            Ok(n) => {
                ok += 1;
                println!("seed {seed}: ok ({n} decisions)");
            }
            // An unported routine: everything before it matched.
            Err(diff::Divergence::Panic { decision, message }) if message.contains("unported routine") => {
                unported += 1;
                println!("seed {seed}: ok for {decision} decisions, then {message}");
            }
            Err(d) => {
                bad += 1;
                let sites: Vec<_> = trace.decisions.iter().take(6).map(|d| format!("{:?}/{:?}", d.site, d.action)).collect();
                println!("seed {seed}: {d:?}\n    oracle decisions: {}", sites.join(", "));
            }
        }
    }
    println!(
        "start state: {start_ok} exact, {start_before_ai} exact before the first turn (AI unported), {start_bad} differ, {unsupported} unsupported"
    );
    println!("{ok} ok, {unported} ok until an unported routine, {bad} diverged");
}
