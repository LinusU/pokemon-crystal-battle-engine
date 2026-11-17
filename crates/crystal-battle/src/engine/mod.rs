//! The hand-written battle engine.
//!
//! A port of pret/pokecrystal's battle engine to plain Rust: no CPU, no
//! registers, no frames — the original's control flow written as ordinary
//! functions, operating on the original's memory layout ([`mem::Mem`]) and
//! data ([`rom`]). It is verified against the original (the transpiled engine
//! running on the emulated CPU, the harness's `oracle` module) by differential testing.
//!
//! Status: under construction. See docs/goal.md.

pub mod ai;
pub mod ai_items;
pub mod ai_smart;
pub mod ai_switch;
pub mod api;
pub mod battle;
pub mod status;
pub mod misc;
pub mod effects_a;
pub mod effects_b;
pub mod commands;
pub mod core;
pub mod harness;
pub mod items;
pub mod load;
pub mod math;
pub mod mem;
pub mod message;
pub mod names;
pub mod rom;
pub mod setup;
pub mod start;
pub mod rsym;
pub mod sym;
pub mod text;
pub mod damage;
pub mod residual;
pub mod switch;

/// The ROM label covering flat offset `addr`, e.g. `UsedMoveText+3`.
pub fn rsym_name(addr: usize) -> String {
    let t = rsym::ROM_SYMBOLS;
    let i = t.partition_point(|&(a, _)| a <= addr);
    if i == 0 {
        return format!("{addr:#08x}");
    }
    let (base, name) = t[i - 1];
    if base == addr { name.to_string() } else { format!("{name}+{}", addr - base) }
}
