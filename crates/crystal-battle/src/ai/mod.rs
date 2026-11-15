//! A trained battle AI (the `ai` cargo feature). See docs/ai.md.
//!
//! ```no_run
//! use crystal_battle::ai::Agent;
//! use crystal_battle::battle::{Battle, Status};
//! use crystal_battle::config::EnemyControl;
//! use crystal_battle::data::random::random_battle_config;
//!
//! // A random trainer battle where the AI plays the trainer.
//! let mut config = random_battle_config(7);
//! config.battle.enemy_control = EnemyControl::External;
//! let seed = config.battle.seed.unwrap();
//! let mut battle = Battle::new(config, seed).unwrap();
//! let mut ai = Agent::builtin();
//! while let Status::Decision(decision) = battle.status() {
//!     let action = ai.choose(decision); // the AI plays both sides here
//!     battle.step(action).unwrap();
//! }
//! ```
//!
//! - [`features`]: the battle state and each legal action as sparse vectors.
//! - [`net`]: the policy/value network (MLP, hand-written backprop, Adam).
//! - [`Agent`]: a network as a player. The harness's
//!   `bootstrap::decision::AiDecider` hooks it into the CLI / window (the
//!   emulated original) as the enemy trainer.
//! - [`train`]: PPO self-play training and evaluation.

pub mod agent;
pub mod features;
pub mod net;
pub mod train;

pub use agent::{Agent, Play};
