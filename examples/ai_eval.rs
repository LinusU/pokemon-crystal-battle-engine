//! Evaluate a battle AI model against the original trainer AI (see docs/ai.md).
//!
//! cargo run --release --features ai --example ai_eval -- [model.bin] [games] [first_seed]
//!
//! Without a model path, evaluates the built-in model. Scores are
//! win = 1, draw = ½, over the same held-out scenarios for every row.

use rgb2rust_output::ai::Agent;
use rgb2rust_output::ai::train::{Policy, Record, evaluate};
use std::sync::Arc;

fn main() {
    let mut args = std::env::args().skip(1).peekable();
    let agent = match args.peek() {
        Some(a) if a.parse::<usize>().is_err() => Agent::load(args.next().unwrap()).expect("load model"),
        _ => Agent::builtin(),
    };
    let games: usize = args.next().and_then(|s| s.parse().ok()).unwrap_or(5000);
    let first: u64 = args.next().and_then(|s| s.parse().ok()).unwrap_or(9_000_000);
    let threads = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(4);
    let ai = Policy::Net { net: Arc::new(agent.net().clone()), greedy: true };
    let row = |label: &str, r: Record| println!("  {label:<44} {r}");

    println!("Trainer's score against a fixed player policy ({games} battles each):");
    for (name, player) in [("greedy", Policy::Greedy), ("random", Policy::Random)] {
        row(&format!("original trainer AI vs {name} player"), evaluate(&player, &Policy::Original, games, first, threads));
        row(&format!("trained AI as trainer vs {name} player"), evaluate(&player, &ai, games, first, threads));
    }
    println!("Head-on, the trained AI as the player:");
    let r = evaluate(&ai, &Policy::Original, games, first, threads);
    let flipped = Record { wins: r.losses, losses: r.wins, draws: r.draws };
    row("trained AI (player) vs original trainer AI", flipped);
    println!("Baselines as the player against the original trainer AI:");
    for (name, player) in [("greedy", Policy::Greedy), ("random", Policy::Random)] {
        let r = evaluate(&player, &Policy::Original, games, first, threads);
        row(&format!("{name} (player) vs original trainer AI"), Record { wins: r.losses, losses: r.wins, draws: r.draws });
    }
}
