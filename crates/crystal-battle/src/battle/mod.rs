//! The library API: drive one battle decision by decision.
//!
//! ```no_run
//! use crystal_battle::battle::{Battle, Status};
//! use crystal_battle::data::random::random_battle_config;
//!
//! let config = random_battle_config(7);
//! let mut battle = Battle::new(config, 0x1234).unwrap();
//! while let Status::Decision(decision) = battle.status() {
//!     let action = decision.legal[0]; // a (bad) policy
//!     battle.step(action).unwrap();
//! }
//! println!("{:?}", battle.status());
//! ```
//!
//! [`Battle`] runs on the hand-written engine (`crate::engine`): plain Rust,
//! no emulator, microseconds per decision, and `Clone` — copy a battle to
//! search ahead. Every place the original game waits for a choice — the action
//! menu, a forced switch after a faint, the "switch before the trainer's next
//! mon?" offer, and the enemy's action in a link battle — is a [`Decision`]
//! carrying the legal actions and the [`BattleState`]. Given the same scenario,
//! seed and actions, a battle plays out exactly as the original game's code
//! does: same states, same random draws, same text (see docs/goal.md for how
//! that is verified).
//!
//! The harness crate (`crystal-battle-harness`, the repository root) has an
//! `EmulatedBattle` with the same API on the original code itself (the
//! transpiled engine on the emulated CPU). It is the reference the engine is
//! tested against, and what the `gui` window plays.

pub mod action;
pub mod event;
pub mod legal;
pub mod message;
pub mod outcome;
pub mod result;
pub mod setup;
pub mod state;

pub use crate::config::BattleConfig;
pub use action::{PromptSite, Side, Step as Action};
pub use event::{Event, HpBarTarget, one_line};
pub use message::Message;
pub use outcome::Outcome;
pub use result::BattleResult;
pub use setup::{Bag, BattleSetup, Opponent, Options, Player, Pokemon, World};
pub use state::BattleState;

use crate::engine::api::{FastBattle, Status as EngineStatus};

/// A choice the engine is waiting for.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Decision {
    /// Which menu the original game would be showing.
    pub site: PromptSite,
    /// Who decides: the player, or (in a link battle) the enemy player.
    pub side: Side,
    /// Every choice the original menus accept here, in a stable order. Never
    /// empty.
    pub legal: Vec<Action>,
    /// The battle state at this decision point.
    pub state: BattleState,
    /// What the battle showed since the previous decision (or since the start):
    /// the exact battle text and HP-bar changes, in order.
    pub events: Vec<Event>,
}

/// Where the battle is.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Status<'a> {
    /// Waiting for [`Battle::step`] (or the harness's `EmulatedBattle::step`).
    Decision(&'a Decision),
    /// The battle is over.
    Ended { outcome: Outcome, state: &'a BattleState, events: &'a [Event] },
}

/// Why a call failed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// The scenario could not be set up (unknown species, bad override, …).
    Setup(String),
    /// The action is not in the current decision's legal list.
    IllegalAction(Action),
    /// The battle already ended; there is nothing to decide.
    Ended,
    /// The engine panicked. This is a bug in the engine or harness.
    Engine(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Setup(msg) => write!(f, "battle setup failed: {msg}"),
            Error::IllegalAction(a) => write!(f, "illegal action {a:?}"),
            Error::Ended => write!(f, "the battle has ended"),
            Error::Engine(msg) => write!(f, "engine failure: {msg}"),
        }
    }
}

impl std::error::Error for Error {}


#[derive(Clone)]
enum Current {
    Decision(Decision),
    Ended { outcome: Outcome, state: BattleState, events: Vec<Event> },
}

/// One battle in progress, on the hand-written engine.
#[derive(Clone)]
pub struct Battle {
    engine: FastBattle,
    current: Current,
}

impl Battle {
    /// Start a battle. `seed` seeds the random-byte stream; the same config,
    /// seed and actions always replay the same battle.
    pub fn new(config: BattleConfig, seed: u64) -> Result<Battle, Error> {
        let engine = catch(|| FastBattle::with_events(&config, seed, true))?.map_err(Error::Setup)?;
        let current = current(&engine);
        Ok(Battle { engine, current })
    }

    /// Start a battle from what a game holds ([`BattleSetup`]), with the
    /// game's data `data` ([`GameData::crystal`](crate::data::GameData::crystal)
    /// for Pokémon Crystal) and the random stream seeded with `seed`.
    pub fn start(data: &'static crate::data::GameData, setup: &BattleSetup, seed: u64) -> Result<Battle, Error> {
        let engine = catch(|| FastBattle::from_setup(data, setup, seed, true))?.map_err(Error::Setup)?;
        let current = current(&engine);
        Ok(Battle { engine, current })
    }

    /// What the battle left behind for the game (the party, money, bag, …),
    /// once it has ended.
    pub fn result(&self) -> Option<BattleResult> {
        self.engine.result()
    }

    /// What the battle showed during the last [`Battle::step`] (or, before the
    /// first step, since the battle started).
    pub fn events(&self) -> &[Event] {
        match &self.current {
            Current::Decision(d) => &d.events,
            Current::Ended { events, .. } => events,
        }
    }

    /// The pending decision, or the result.
    pub fn status(&self) -> Status<'_> {
        match &self.current {
            Current::Decision(d) => Status::Decision(d),
            Current::Ended { outcome, state, events } => Status::Ended { outcome: *outcome, state, events },
        }
    }

    /// The pending decision, if the battle is still running.
    pub fn decision(&self) -> Option<&Decision> {
        match &self.current {
            Current::Decision(d) => Some(d),
            Current::Ended { .. } => None,
        }
    }

    /// The current battle state (at the pending decision, or at the end).
    pub fn state(&self) -> &BattleState {
        match &self.current {
            Current::Decision(d) => &d.state,
            Current::Ended { state, .. } => state,
        }
    }

    /// The outcome, once the battle has ended.
    pub fn outcome(&self) -> Option<Outcome> {
        match &self.current {
            Current::Decision(_) => None,
            Current::Ended { outcome, .. } => Some(*outcome),
        }
    }

    /// Answer the pending decision and run to the next one (or to the end of
    /// the battle).
    pub fn step(&mut self, action: Action) -> Result<Status<'_>, Error> {
        let Current::Decision(decision) = &self.current else {
            return Err(Error::Ended);
        };
        if !decision.legal.contains(&action) {
            return Err(Error::IllegalAction(action));
        }
        let mut engine = self.engine.clone();
        catch(move || {
            engine.step(action).expect("a legal action");
            engine
        })
        .map(|engine| {
            self.current = current(&engine);
            self.engine = engine;
        })?;
        Ok(self.status())
    }

    /// The underlying engine battle (raw WRAM state, the printed text as ROM
    /// labels).
    pub fn engine(&self) -> &FastBattle {
        &self.engine
    }
}

/// The API view of `engine` now.
fn current(engine: &FastBattle) -> Current {
    let state = engine.state();
    let events = engine.events().to_vec();
    match engine.status() {
        EngineStatus::Decision { site, legal } => {
            let side = site.side();
            Current::Decision(Decision { site: *site, side, legal: legal.clone(), state, events })
        }
        EngineStatus::Ended(outcome) => Current::Ended { outcome: *outcome, state, events },
    }
}

/// Run `f`, turning a panic (a bug in the engine) into [`Error::Engine`].
fn catch<T>(f: impl FnOnce() -> T) -> Result<T, Error> {
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(f)).map_err(|payload| Error::Engine(panic_message(&payload)))
}

/// The message of a caught panic.
#[doc(hidden)]
pub fn panic_message(payload: &Box<dyn std::any::Any + Send>) -> String {
    if let Some(s) = payload.downcast_ref::<&str>() {
        (*s).to_string()
    } else if let Some(s) = payload.downcast_ref::<String>() {
        s.clone()
    } else {
        "engine panicked".into()
    }
}
