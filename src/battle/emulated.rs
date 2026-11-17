//! The library API on the original engine: the transpiled game code running
//! on the emulated CPU, on its own thread, blocking at decision points. The
//! reference the hand-written engine ([`super::Battle`]) is tested against,
//! and what the `gui` window plays.

use std::sync::mpsc::{Receiver, SyncSender, sync_channel};
use std::thread::JoinHandle;

use super::{Action, BattleConfig, BattleState, Error, Outcome, PromptSite, Side, panic_message};
use crate::bootstrap::events::Shown;

/// A decision on the original engine: the library's
/// [`Decision`](super::Decision), with what the original showed (its
/// captured text and HP bars) where the library has its events.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmulatedDecision {
    pub site: PromptSite,
    pub side: Side,
    pub legal: Vec<Action>,
    pub state: BattleState,
    pub shown: Vec<Shown>,
}

/// Where an [`EmulatedBattle`] is.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EmulatedStatus<'a> {
    Decision(&'a EmulatedDecision),
    Ended { outcome: Outcome, state: &'a BattleState, shown: &'a [Shown] },
}
use crate::bootstrap::decision::Decider;
use crate::runtime::Cpu;

enum EngineMsg {
    Decision(EmulatedDecision),
    Ended { outcome: Outcome, state: BattleState, shown: Vec<Shown> },
    Failed(Error),
}

enum Current {
    Decision(EmulatedDecision),
    Ended { outcome: Outcome, state: BattleState, shown: Vec<Shown> },
}

/// One battle in progress on the original engine.
pub struct EmulatedBattle {
    current: Current,
    to_engine: Option<SyncSender<Action>>,
    from_engine: Receiver<EngineMsg>,
    thread: Option<JoinHandle<()>>,
}

/// Unwind payload used to stop the engine thread when the [`EmulatedBattle`] is
/// dropped mid-battle.
struct Abandoned;

/// The [`Decider`] the library installs: hands each decision to the caller.
struct ChannelDecider {
    to_api: SyncSender<EngineMsg>,
    from_api: Receiver<Action>,
    /// Set once the end of the battle was reported.
    ended: std::sync::Arc<std::sync::atomic::AtomicBool>,
}

impl Decider for ChannelDecider {
    fn decide(&mut self, cpu: &Cpu, site: PromptSite, legal: &[Action]) -> Action {
        let side = site.side();
        let decision = EmulatedDecision {
            site,
            side,
            legal: legal.to_vec(),
            state: BattleState::read(cpu),
            shown: crate::bootstrap::events::take_events(cpu),
        };
        if self.to_api.send(EngineMsg::Decision(decision)).is_err() {
            std::panic::resume_unwind(Box::new(Abandoned));
        }
        match self.from_api.recv() {
            Ok(action) => action,
            Err(_) => std::panic::resume_unwind(Box::new(Abandoned)),
        }
    }

    fn battle_ended(&mut self, cpu: &Cpu) {
        // The end of the battle is here, before the game's post-battle
        // processing (`ExitBattle`).
        self.ended.store(true, std::sync::atomic::Ordering::SeqCst);
        let outcome = crate::bootstrap::snapshot::read_outcome(cpu);
        let state = BattleState::read(cpu);
        let shown = crate::bootstrap::events::take_events(cpu);
        let _ = self.to_api.send(EngineMsg::Ended { outcome, state, shown });
    }
}

/// Stack for the engine thread. The transpiled code models every `call` as a
/// Rust call, so deep engine paths use a lot of native stack.
const ENGINE_STACK: usize = 64 * 1024 * 1024;

impl EmulatedBattle {
    /// Start a battle. `seed` seeds the engine's random-byte stream; the same
    /// config, seed and actions always replay the same battle.
    pub fn new(config: BattleConfig, seed: u64) -> Result<EmulatedBattle, Error> {
        let (to_api, from_engine) = sync_channel::<EngineMsg>(1);
        let (to_engine, from_api) = sync_channel::<Action>(1);
        let thread = std::thread::Builder::new()
            .name("battle-engine".into())
            .stack_size(ENGINE_STACK)
            .spawn(move || run_engine(config, seed, to_api, from_api))
            .map_err(|e| Error::Engine(format!("could not spawn the engine thread: {e}")))?;
        let mut battle = EmulatedBattle {
            current: Current::Ended { outcome: Outcome::Draw, state: placeholder_state(), shown: Vec::new() },
            to_engine: Some(to_engine),
            from_engine,
            thread: Some(thread),
        };
        battle.current = battle.receive()?;
        Ok(battle)
    }

    /// What the battle showed during the last [`EmulatedBattle::step`] (or, before the
    /// first step, since the battle started).
    pub fn shown(&self) -> &[Shown] {
        match &self.current {
            Current::Decision(d) => &d.shown,
            Current::Ended { shown, .. } => shown,
        }
    }

    /// The pending decision, or the result.
    pub fn status(&self) -> EmulatedStatus<'_> {
        match &self.current {
            Current::Decision(d) => EmulatedStatus::Decision(d),
            Current::Ended { outcome, state, shown } => EmulatedStatus::Ended { outcome: *outcome, state, shown },
        }
    }

    /// The pending decision, if the battle is still running.
    pub fn decision(&self) -> Option<&EmulatedDecision> {
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
    pub fn step(&mut self, action: Action) -> Result<EmulatedStatus<'_>, Error> {
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
            Ok(EngineMsg::Ended { outcome, state, shown }) => {
                self.to_engine = None;
                Ok(Current::Ended { outcome, state, shown })
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

impl Drop for EmulatedBattle {
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
        let ended = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        cpu.decider = Some(std::cell::RefCell::new(Box::new(ChannelDecider {
            to_api: to_api.clone(),
            from_api,
            ended: ended.clone(),
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
        if !ended.load(std::sync::atomic::Ordering::SeqCst) {
            let outcome = crate::bootstrap::snapshot::read_outcome(&cpu);
            let state = BattleState::read(&cpu);
            let shown = crate::bootstrap::events::take_events(&cpu);
            let _ = to_api.send(EngineMsg::Ended { outcome, state, shown });
        }
    }));
    if let Err(payload) = result {
        if payload.downcast_ref::<Abandoned>().is_none() {
            let _ = failures.send(EngineMsg::Failed(Error::Engine(panic_message(&payload))));
        }
    }
}

fn placeholder_state() -> BattleState {
    BattleState::read(&Cpu::new_cgb(None, None))
}
