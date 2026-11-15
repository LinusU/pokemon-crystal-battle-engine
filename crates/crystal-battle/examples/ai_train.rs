//! Train the battle AI with PPO self-play (see docs/ai.md).
//!
//! cargo run --release -p crystal-battle --features ai --example ai_train -- <out.bin> [iterations] [--resume <model.bin>] [--seed <n>]
//!
//! Saves the network to `<out.bin>` after every iteration (and a copy every
//! 50 as `<out.bin>.<iteration>`), and every 25 iterations evaluates it as
//! the trainer against the original trainer AI on 2,000 held-out battles.

use crystal_battle::ai::net::Net;
use crystal_battle::ai::train::{Policy, TrainConfig, Trainer, evaluate};
use std::sync::Arc;

fn main() {
    let mut args = std::env::args().skip(1);
    let out = args.next().expect("usage: ai_train <out.bin> [iterations] [--resume <model.bin>] [--seed <n>]");
    let mut iterations = 1000usize;
    let mut resume = None;
    let mut seed = 1u64;
    while let Some(a) = args.next() {
        match a.as_str() {
            "--resume" => resume = args.next(),
            "--seed" => seed = args.next().and_then(|s| s.parse().ok()).expect("--seed <n>"),
            n => iterations = n.parse().expect("iterations"),
        }
    }
    let net = match resume {
        Some(path) => Net::from_bytes(&std::fs::read(&path).expect("read model")).expect("parse model"),
        None => Net::default_shape(seed),
    };
    let cfg = TrainConfig::default();
    eprintln!("{} parameters, {} threads, {} samples/iteration", net.len(), cfg.threads, cfg.samples_per_iter);
    let threads = cfg.threads;
    let mut trainer = Trainer::new(net, cfg);
    let started = std::time::Instant::now();
    for it in 1..=iterations {
        let stats = trainer.iterate(seed.wrapping_mul(1_000_003));
        let r = &stats.rollout;
        let l = stats.losses;
        println!(
            "it {it:4} {:6.0}s | {:5} battles {:6} samples | self {} | vs original {} | vs pool {} | pi {:+.3} v {:.3} H {:.3} clip {:.2} kl {:.4} | {:.1}s+{:.1}s",
            started.elapsed().as_secs_f32(),
            r.battles,
            r.decisions,
            r.vs_self,
            r.vs_original,
            r.vs_pool,
            l.policy,
            l.value,
            l.entropy,
            l.clip_fraction,
            l.approx_kl,
            stats.rollout_secs,
            stats.update_secs,
        );
        let bytes = trainer.net.to_bytes();
        let tmp = format!("{out}.tmp");
        std::fs::write(&tmp, &bytes).expect("write model");
        std::fs::rename(&tmp, &out).expect("rename model");
        if it % 50 == 0 {
            std::fs::write(format!("{out}.{it}"), &bytes).expect("write checkpoint");
        }
        if it % 25 == 0 {
            let ai = Policy::Net { net: Arc::new(trainer.net.clone()), greedy: true };
            // Held-out seeds (training uses seed * 2^32 + game).
            let vs_greedy = evaluate(&Policy::Greedy, &ai, 2000, 7_000_000, threads);
            let original_vs_greedy = evaluate(&Policy::Greedy, &Policy::Original, 2000, 7_000_000, threads);
            let as_player = evaluate(&ai, &Policy::Original, 2000, 7_000_000, threads);
            println!(
                "   eval: trainer score vs greedy player — AI {vs_greedy}, original AI {original_vs_greedy} | AI as player vs original AI: original scores {as_player}"
            );
        }
    }
}
