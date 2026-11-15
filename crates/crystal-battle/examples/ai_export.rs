//! Write a training checkpoint as a compact (bf16) model, e.g. the shipped
//! `crates/crystal-battle/models/crystal-ai.bin` (see docs/ai.md).
//!
//! cargo run --release -p crystal-battle --features ai --example ai_export -- <checkpoint.bin> <out.bin>

use crystal_battle::ai::net::Net;

fn main() {
    let mut args = std::env::args().skip(1);
    let (Some(input), Some(output)) = (args.next(), args.next()) else {
        eprintln!("usage: ai_export <checkpoint.bin> <out.bin>");
        std::process::exit(1);
    };
    let net = Net::from_bytes(&std::fs::read(&input).expect("read checkpoint")).expect("parse checkpoint");
    std::fs::write(&output, net.to_bytes_bf16()).expect("write model");
    eprintln!("{} parameters → {output}", net.len());
}
