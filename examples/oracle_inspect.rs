//! Print which WRAM symbols the original engine changes between consecutive
//! decision points of a random link battle — a map of what a turn touches.
//!
//! cargo run --release --example oracle_inspect -- [seed] [moves...]

use crystal_battle_harness::engine::sym;
use crystal_battle_harness::oracle::{self, testgen};
use std::collections::BTreeMap;

fn main() {
    let mut args = std::env::args().skip(1);
    let seed: u64 = args.next().and_then(|s| s.parse().ok()).unwrap_or(1);
    let moves: Vec<u8> = args.map(|m| m.parse().unwrap()).collect();
    let moves = if moves.is_empty() { vec![33] } else { moves }; // TACKLE
    let config = testgen::random_link_config(seed, &moves, Default::default());
    let trace = oracle::run(&config, config.battle.seed.unwrap(), seed, 400).expect("finished");
    println!("{} decisions", trace.decisions.len());
    let mut changed: BTreeMap<String, usize> = BTreeMap::new();
    let mems: Vec<_> = trace.decisions.iter().map(|d| &d.mem).chain([&trace.end_mem]).collect();
    for pair in mems.windows(2) {
        let (a, b) = (pair[0].wram(), pair[1].wram());
        let mut seen = std::collections::BTreeSet::new();
        for i in 0..a.len() {
            if a[i] != b[i] {
                let name = sym::ram_name(0xc000 + i as u16);
                let base = name.split('+').next().unwrap().to_string();
                seen.insert(base);
            }
        }
        for s in seen {
            *changed.entry(s).or_default() += 1;
        }
    }
    for (name, n) in changed {
        println!("{n:4} {name}");
    }
}
