//! Pokémon Crystal's battle engine as a library.
//!
//! [`battle::Battle`] drives one battle decision by decision on a hand-written
//! port of the original engine: plain Rust, no emulator, `Clone` to search
//! ahead, and exactly the original's states, random draws and text. A battle
//! starts from a [`config::BattleConfig`]; [`data`] holds the species, move,
//! item and trainer tables that config refers to by name.
//!
//! The repository root holds the harness this is verified against: the
//! transpiled original running on an emulated Game Boy (`crystal-battle-harness`).
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unreachable_code)]
#![allow(unused_comparisons)]
#![allow(unused_parens)]

#[cfg(feature = "ai")]
pub mod ai;
pub mod battle;
pub mod config;
pub mod data;
pub mod defs;
pub mod engine;
pub mod rom;
