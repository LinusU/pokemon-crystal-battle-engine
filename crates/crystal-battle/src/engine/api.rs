//! The engine's public face: a battle you drive decision by decision, that
//! you can clone to search ahead.
//!
//! Each step replays the current turn from a snapshot taken at its start with
//! the decisions made so far plus the new one (see docs/engine.md): the
//! port stays a straight copy of the original's sequential code, and cloning
//! is copying the snapshot.

use super::battle::{Engine, Suspend, TextRef};
use super::core::{TurnEnd, TurnEntry};
use crate::battle::BattleState;
use crate::config::BattleConfig;
use crate::battle::action::{PromptSite, Step};
use crate::battle::event::Event;
use crate::battle::outcome::Outcome;

/// Where a [`FastBattle`] is.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Status {
    /// Waiting for a decision at `site`; `legal` lists every allowed action.
    Decision { site: PromptSite, legal: Vec<Step> },
    /// The battle is over.
    Ended(Outcome),
}

/// A battle on the hand-written engine.
#[derive(Debug, Clone)]
pub struct FastBattle {
    /// The engine at the start of the current turn.
    base: Engine,
    entry: TurnEntry,
    /// Decisions taken so far in the current turn.
    turn_decisions: Vec<Step>,
    /// The engine at the pending decision (or at the end).
    current: Engine,
    status: Status,
    /// Text printed during the last step.
    texts: Vec<TextRef>,
    /// How many of the current turn's texts were already reported.
    turn_texts_seen: usize,
    /// Events (text messages, HP bars) shown during the last step, when on.
    events: Vec<Event>,
    /// How many of the current turn's events were already reported.
    turn_events_seen: usize,
}

impl FastBattle {
    /// Start a battle on the hand-written engine (`start::start`: the
    /// scenario's WRAM, the game's own battle start). A scenario the engine
    /// doesn't start (see `start::StartError::Unsupported`) is an error.
    pub fn new(config: &BattleConfig, seed: u64) -> Result<FastBattle, String> {
        Self::with_events(config, seed, false)
    }

    /// [`FastBattle::new`], and with `events` also produce the battle's
    /// [`Event`]s (exact text messages and HP-bar changes, see
    /// [`FastBattle::events`]). Rendering text costs time; leave it off when
    /// only the state matters.
    pub fn with_events(config: &BattleConfig, seed: u64, events: bool) -> Result<FastBattle, String> {
        let mut engine = match super::start::start(config, seed, false) {
            Ok(engine) => engine,
            Err(error) => return Err(error.to_string()),
        };
        if events {
            // The battle start's text, rendered again with events on.
            let mut again = super::start::start(config, seed, true).map_err(|e| e.to_string())?;
            again.events = Some(again.rendered.take().unwrap_or_default().into_iter().map(Event::Text).collect());
            engine = again;
        }
        Ok(Self::from_engine(engine, TurnEntry::TurnStart))
    }

    /// A battle whose current turn starts at `entry` in `base`.
    fn from_engine(base: Engine, entry: TurnEntry) -> FastBattle {
        let mut battle = FastBattle {
            current: base.clone(),
            base,
            entry,
            turn_decisions: Vec::new(),
            status: Status::Ended(Outcome::Draw),
            texts: Vec::new(),
            turn_texts_seen: 0,
            events: Vec::new(),
            turn_events_seen: 0,
        };
        // Events printed before the first turn (the battle start).
        if let Some(start) = battle.base.events.as_mut() {
            battle.events = std::mem::take(start);
        }
        battle.current = battle.base.clone();
        battle.advance();
        battle
    }

    pub fn status(&self) -> &Status {
        &self.status
    }

    /// The battle state now (at the pending decision or at the end).
    pub fn state(&self) -> BattleState {
        BattleState::read(&self.current.mem)
    }

    /// The engine itself (its memory is the original's WRAM layout).
    pub fn engine(&self) -> &Engine {
        &self.current
    }

    /// Text printed during the last step, as ROM text addresses.
    pub fn texts(&self) -> &[TextRef] {
        &self.texts
    }

    /// What the battle showed during the last step (or, before the first
    /// step, since the battle started). Empty unless started
    /// [`FastBattle::with_events`].
    pub fn events(&self) -> &[Event] {
        &self.events
    }

    /// Answer the pending decision.
    pub fn step(&mut self, action: Step) -> Result<&Status, Step> {
        match &self.status {
            Status::Decision { legal, .. } if legal.contains(&action) => {}
            _ => return Err(action),
        }
        self.turn_decisions.push(action);
        self.events.clear();
        self.advance();
        Ok(&self.status)
    }

    /// Run from the turn's start with the decisions so far until the next
    /// decision or the end, rolling the snapshot forward at turn boundaries.
    fn advance(&mut self) {
        self.texts.clear();
        loop {
            let mut engine = self.base.clone();
            engine.pending = self.turn_decisions.iter().copied().collect();
            let result = engine.battle_turn(self.entry);
            match result {
                Err(Suspend { site, legal }) => {
                    // A replay re-prints the turn so far; only the rest is new.
                    self.texts.extend_from_slice(&engine.texts[self.turn_texts_seen..]);
                    self.turn_texts_seen = engine.texts.len();
                    self.take_events(&engine);
                    self.current = engine;
                    self.status = Status::Decision { site, legal };
                    return;
                }
                Ok(TurnEnd::Quit) => {
                    self.texts.extend_from_slice(&engine.texts[self.turn_texts_seen..]);
                    self.take_events(&engine);
                    self.status = Status::Ended(outcome(&engine));
                    self.current = engine;
                    return;
                }
                Ok(TurnEnd::Continue) => {
                    self.texts.extend_from_slice(&engine.texts[self.turn_texts_seen..]);
                    self.turn_texts_seen = 0;
                    engine.texts.clear();
                    self.take_events(&engine);
                    self.turn_events_seen = 0;
                    if let Some(events) = engine.events.as_mut() {
                        events.clear();
                    }
                    engine.pending.clear();
                    self.base = engine;
                    self.entry = TurnEntry::TurnStart;
                    self.turn_decisions.clear();
                }
            }
        }
    }
}

impl FastBattle {
    /// Move the events `engine` produced in this turn that weren't reported yet.
    fn take_events(&mut self, engine: &Engine) {
        if let Some(events) = engine.events.as_ref() {
            self.events.extend_from_slice(&events[self.turn_events_seen..]);
            self.turn_events_seen = events.len();
        }
    }
}

/// `wBattleResult` → outcome.
fn outcome(engine: &Engine) -> Outcome {
    if engine.flags.caught_wild_mon {
        return Outcome::Caught;
    }
    // The low nybble is the result (`and $f`, as `ExitBattle` reads it); the
    // top bits are flags (`BATTLERESULT_BITMASK`).
    match engine.mem.rb(super::sym::W_BATTLE_RESULT) & 0x0f {
        0 => Outcome::Win,
        1 => Outcome::Loss,
        _ => Outcome::Draw,
    }
}
