//! The text engine on garbage, both engines side by side: Beat Up's
//! "<name>'s attack!" printed with random raw names in `wStringBuffer1`
//! (control characters, kana, `<MOBILE>` scripts), from the state at a
//! battle's first decision. Text and memory must match.
//!
//! cargo build --release --example text_diff
//! ./target/release/examples/text_diff [names] [first_seed]
//!
//! "The original can't print": it panicked (a `<MOBILE>` jump into garbage),
//! left WRAM bank 4 mapped, or ran out of cycles (`TEXT_CYCLES`, default
//! 2^26: garbage over `wTextDelayFrames` can make it print very slowly).
//! `ONLY=<seed>` checks one name on the link bench.

use rgb2rust_output::engine::diff::{GarbageText, check_garbage_name};
use rgb2rust_output::engine::oracle::{TextBench, mix};
use rgb2rust_output::engine::testgen;

fn main() {
    // The original recurses as deep as garbage takes it; the cycle watchdog
    // stops it, given room.
    std::thread::Builder::new().stack_size(1 << 30).spawn(run).unwrap().join().unwrap();
}

fn run() {
    let mut args = std::env::args().skip(1);
    let names: u64 = args.next().and_then(|s| s.parse().ok()).unwrap_or(1000);
    let first: u64 = args.next().and_then(|s| s.parse().ok()).unwrap_or(1);
    std::panic::set_hook(Box::new(|_| {}));
    if let Ok(only) = std::env::var("ONLY") {
        let seed: u64 = only.parse().unwrap();
        let moves: Vec<u8> = (1..=251).collect();
        let config = testgen::random_link_config(1, &moves, Default::default());
        let mut bench = TextBench::new(&config, config.battle.seed.unwrap()).expect("first decision");
        println!("{:?}", check_garbage_name(&mut bench, &random_name(seed)));
        return;
    }
    let moves: Vec<u8> = (1..=251).collect();
    let configs = [
        ("link", testgen::random_link_config(1, &moves, Default::default())),
        ("trainer", testgen::random_trainer_config(2, &moves, Default::default())),
    ];
    for (kind, config) in &configs {
        let mut bench = TextBench::new(config, config.battle.seed.unwrap()).expect("first decision");
        let (mut exact, mut oracle_failed, mut bad) = (0, 0, 0);
        for seed in first..first + names {
            let name = random_name(seed);
            match check_garbage_name(&mut bench, &name) {
                GarbageText::Exact => exact += 1,
                GarbageText::OracleFailed { oracle, engine } => {
                    oracle_failed += 1;
                    if !engine.starts_with("panicked") {
                        println!("{kind} {seed} {name:02x?}: oracle {oracle}, engine {engine}");
                    }
                }
                other => {
                    bad += 1;
                    println!("{kind} {seed} {name:02x?}: {other:?}");
                }
            }
        }
        println!("{kind}: {exact} exact, {oracle_failed} the original can't print, {bad} differ");
    }
}

/// A random name: half are `<MOBILE>` scripts (each command with random
/// parameters, small values likelier), half random bytes.
fn random_name(seed: u64) -> Vec<u8> {
    let mut s = seed.wrapping_mul(0x9e37_79b9_7f4a_7c15) | 1;
    let mut byte = || mix(&mut s) as u8;
    let mut name = Vec::new();
    let small = |b: u8, r: u8| if r & 1 == 0 { b % 24 } else { b };
    if byte() & 1 == 0 {
        for _ in 0..byte() % 4 {
            name.push(0x80 + byte() % 26);
        }
        name.push(0x15);
        for _ in 0..1 + byte() % 3 {
            let command = [1, 2, 3, 4, 5, 6, 7, 8, 13, 14, 15, 0x50, 0x10, 0x80][usize::from(byte() % 14)];
            name.push(command);
            for _ in 0..byte() % 9 {
                let (b, r) = (byte(), byte());
                name.push(small(b, r));
            }
        }
        for _ in 0..byte() % 4 {
            name.push(0x80 + byte() % 26);
        }
    } else {
        for _ in 0..4 + byte() % 20 {
            name.push(byte());
        }
    }
    name.push(0x50);
    name
}
