//! Game-data tables the engine and the scenario format use: species, move,
//! item and trainer names and ids, base PP, and the random-battle generator.

pub mod items;
pub mod move_pp;
pub mod names;
pub mod random;
pub mod species;
pub mod trainers;
pub mod trainers_data;
pub mod crystal;
pub mod game;

pub use game::GameData;
