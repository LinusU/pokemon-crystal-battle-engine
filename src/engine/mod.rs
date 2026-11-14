//! The hand-written battle engine.
//!
//! A port of pret/pokecrystal's battle engine to plain Rust: no CPU, no
//! registers, no frames — the original's control flow written as ordinary
//! functions, operating on the original's memory layout ([`mem::Mem`]) and
//! data ([`rom`]). It is verified against the original (the transpiled engine
//! running on the emulated CPU, see [`oracle`]) by differential testing.
//!
//! Status: under construction. See docs/goal.md.

pub mod ai;
pub mod api;
pub mod battle;
mod cmd_todo;
mod cmd_todo_status;
pub mod status;
mod cmd_todo_misc;
pub mod misc;
mod cmd_todo_effects_a;
pub mod effects_a;
mod cmd_todo_effects_b;
pub mod effects_b;
pub mod commands;
pub mod core;
pub mod diff;
pub mod harness;
pub mod math;
pub mod mem;
pub mod names;
pub mod oracle;
pub mod rom;
pub mod setup;
pub mod rsym;
pub mod sym;
pub mod testgen;
pub mod text;
mod todo;
mod todo_damage;
mod todo_residual;
mod todo_switch;
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
