//! The hand-written battle engine.
//!
//! A port of pret/pokecrystal's battle engine to plain Rust: no CPU, no
//! registers, no frames — the original's control flow written as ordinary
//! functions, operating on the original's memory layout ([`mem::Mem`]) and
//! data ([`rom`]). It is verified against the original (the transpiled engine
//! running on the emulated CPU, see [`oracle`]) by differential testing.
//!
//! Status: under construction. See docs/goal.md.

pub mod battle;
mod cmd_todo;
pub mod commands;
pub mod core;
pub mod diff;
pub mod math;
pub mod mem;
pub mod names;
pub mod oracle;
pub mod rom;
pub mod rsym;
pub mod sym;
pub mod testgen;
mod todo;
mod todo_damage;
mod todo_residual;
mod todo_switch;
pub mod damage;
pub mod residual;
pub mod switch;
