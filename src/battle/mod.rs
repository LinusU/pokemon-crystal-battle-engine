//! The library API: drive one battle decision by decision.
//!
//! ```no_run
//! use rgb2rust_output::battle::{Battle, Status};
//! use rgb2rust_output::bootstrap::random::random_battle_config;
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
//! The battle runs the original game's code (the transpiled engine) on its own
//! thread. Every place the original waits for a choice — the action menu, a
//! forced switch after a faint, the "switch before the trainer's next mon?"
//! offer, and the enemy's action in a link battle — becomes a [`Decision`]
//! carrying the legal actions and the [`BattleState`]. The engine thread blocks
//! until [`Battle::step`] supplies the choice, then runs until the next
//! decision or the end of the battle. Nothing about the engine's control flow
//! changes, so the battle plays out exactly as the original would given the
//! same choices and random bytes.

pub mod state;

use std::sync::mpsc::{Receiver, SyncSender, sync_channel};
use std::thread::JoinHandle;

pub use crate::bootstrap::BattleConfig;
pub use crate::bootstrap::events::{Event, HpBarTarget, one_line};
pub use crate::bootstrap::decision::{PromptSite, Step as Action};
pub use crate::bootstrap::recorder::Side;
pub use crate::bootstrap::snapshot::Outcome;
pub use state::BattleState;

use crate::bootstrap::decision::Decider;
use crate::runtime::Cpu;

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
    /// Waiting for [`Battle::step`].
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

enum EngineMsg {
    Decision(Decision),
    Ended { outcome: Outcome, state: BattleState, events: Vec<Event> },
    Failed(Error),
}

enum Current {
    Decision(Decision),
    Ended { outcome: Outcome, state: BattleState, events: Vec<Event> },
}

/// One battle in progress.
pub struct Battle {
    current: Current,
    to_engine: Option<SyncSender<Action>>,
    from_engine: Receiver<EngineMsg>,
    thread: Option<JoinHandle<()>>,
}

/// Unwind payload used to stop the engine thread when the [`Battle`] is
/// dropped mid-battle.
struct Abandoned;

/// The [`Decider`] the library installs: hands each decision to the caller.
struct ChannelDecider {
    to_api: SyncSender<EngineMsg>,
    from_api: Receiver<Action>,
}

impl Decider for ChannelDecider {
    fn decide(&mut self, cpu: &Cpu, site: PromptSite, legal: &[Action]) -> Action {
        let side = if site == PromptSite::EnemyAction { Side::Enemy } else { Side::Player };
        let decision = Decision {
            site,
            side,
            legal: legal.to_vec(),
            state: BattleState::read(cpu),
            events: crate::bootstrap::events::take_events(cpu),
        };
        if self.to_api.send(EngineMsg::Decision(decision)).is_err() {
            std::panic::resume_unwind(Box::new(Abandoned));
        }
        match self.from_api.recv() {
            Ok(action) => action,
            Err(_) => std::panic::resume_unwind(Box::new(Abandoned)),
        }
    }
}

/// Stack for the engine thread. The transpiled code models every `call` as a
/// Rust call, so deep engine paths use a lot of native stack.
const ENGINE_STACK: usize = 64 * 1024 * 1024;

impl Battle {
    /// Start a battle. `seed` seeds the engine's random-byte stream; the same
    /// config, seed and actions always replay the same battle.
    pub fn new(config: BattleConfig, seed: u64) -> Result<Battle, Error> {
        let (to_api, from_engine) = sync_channel::<EngineMsg>(1);
        let (to_engine, from_api) = sync_channel::<Action>(1);
        let thread = std::thread::Builder::new()
            .name("battle-engine".into())
            .stack_size(ENGINE_STACK)
            .spawn(move || run_engine(config, seed, to_api, from_api))
            .map_err(|e| Error::Engine(format!("could not spawn the engine thread: {e}")))?;
        let mut battle = Battle {
            current: Current::Ended { outcome: Outcome::Draw, state: placeholder_state(), events: Vec::new() },
            to_engine: Some(to_engine),
            from_engine,
            thread: Some(thread),
        };
        battle.current = battle.receive()?;
        Ok(battle)
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

    /// Answer the pending decision and run the engine to the next one (or to
    /// the end of the battle).
    pub fn step(&mut self, action: Action) -> Result<Status<'_>, Error> {
        let Current::Decision(decision) = &self.current else {
            return Err(Error::Ended);
        };
        if !decision.legal.contains(&action) {
            return Err(Error::IllegalAction(action));
        }
        let sender = self.to_engine.as_ref().ok_or(Error::Ended)?;
        if sender.send(action).is_err() {
            return Err(self.engine_failure());
        }
        self.current = self.receive()?;
        Ok(self.status())
    }

    fn receive(&mut self) -> Result<Current, Error> {
        match self.from_engine.recv() {
            Ok(EngineMsg::Decision(d)) => Ok(Current::Decision(d)),
            Ok(EngineMsg::Ended { outcome, state, events }) => {
                self.to_engine = None;
                Ok(Current::Ended { outcome, state, events })
            }
            Ok(EngineMsg::Failed(e)) => Err(e),
            Err(_) => Err(self.engine_failure()),
        }
    }

    fn engine_failure(&mut self) -> Error {
        match self.thread.take().map(JoinHandle::join) {
            Some(Err(payload)) => Error::Engine(panic_message(&payload)),
            _ => Error::Engine("engine thread stopped".into()),
        }
    }
}

impl Drop for Battle {
    fn drop(&mut self) {
        // Closing the action channel makes a waiting engine unwind out.
        self.to_engine = None;
        if let Some(thread) = self.thread.take() {
            while self.from_engine.try_recv().is_ok() {}
            let _ = thread.join();
        }
    }
}

fn run_engine(config: BattleConfig, seed: u64, to_api: SyncSender<EngineMsg>, from_api: Receiver<Action>) {
    let failures = to_api.clone();
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
        let mut cpu = Cpu::new_cgb(None, None);
        cpu.interrupt_vectors = crate::interrupt_vectors();
        cpu.events = Some(std::cell::RefCell::new(Default::default()));
        cpu.decider = Some(std::cell::RefCell::new(Box::new(ChannelDecider {
            to_api: to_api.clone(),
            from_api,
        })));
        let options = crate::bootstrap::RunOptions {
            seed_override: Some(seed),
            quiet: true,
            ..Default::default()
        };
        if let Err(msg) = crate::bootstrap::run_with(&mut cpu, &config, options) {
            let _ = to_api.send(EngineMsg::Failed(Error::Setup(msg)));
            return;
        }
        let outcome = crate::bootstrap::snapshot::read_outcome(&cpu);
        let state = BattleState::read(&cpu);
        let events = crate::bootstrap::events::take_events(&cpu);
        let _ = to_api.send(EngineMsg::Ended { outcome, state, events });
    }));
    if let Err(payload) = result {
        if payload.downcast_ref::<Abandoned>().is_none() {
            let _ = failures.send(EngineMsg::Failed(Error::Engine(panic_message(&payload))));
        }
    }
}

fn panic_message(payload: &Box<dyn std::any::Any + Send>) -> String {
    if let Some(s) = payload.downcast_ref::<&str>() {
        (*s).to_string()
    } else if let Some(s) = payload.downcast_ref::<String>() {
        s.clone()
    } else {
        "engine panicked".into()
    }
}

fn placeholder_state() -> BattleState {
    BattleState::read(&Cpu::new_cgb(None, None))
}
