//! The engine's public face: a battle you drive decision by decision, that
//! you can clone to search ahead.
//!
//! Each step replays the current turn from a snapshot taken at its start with
//! the decisions made so far plus the new one (see docs/engine.md): the
//! port stays a straight copy of the original's sequential code, and cloning
//! is copying the snapshot.

use super::battle::{Engine, Flags, Suspend, TextRef};
use super::core::{TurnEnd, TurnEntry};
use crate::battle::BattleState;
use crate::bootstrap::BattleConfig;
use crate::bootstrap::decision::{PromptSite, Step};
use crate::bootstrap::snapshot::Outcome;

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
}

impl FastBattle {
    /// Start a battle. The original engine sets it up (the hand-written
    /// engine takes over at the first decision).
    pub fn new(config: &BattleConfig, seed: u64) -> Option<FastBattle> {
        let first = super::oracle::first_decision(config, seed)?;
        let flags = Flags {
            link_battle: config.battle.kind == crate::bootstrap::BattleKind::LinkBattle,
            ..Flags::default()
        };
        let base = Engine::new(first.mem, first.rng, flags);
        let mut battle = FastBattle {
            current: base.clone(),
            base,
            entry: TurnEntry::BattleMenu,
            turn_decisions: Vec::new(),
            status: Status::Ended(Outcome::Draw),
            texts: Vec::new(),
            turn_texts_seen: 0,
        };
        battle.advance();
        Some(battle)
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

    /// Answer the pending decision.
    pub fn step(&mut self, action: Step) -> Result<&Status, Step> {
        match &self.status {
            Status::Decision { legal, .. } if legal.contains(&action) => {}
            _ => return Err(action),
        }
        self.turn_decisions.push(action);
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
                    self.current = engine;
                    self.status = Status::Decision { site, legal };
                    return;
                }
                Ok(TurnEnd::Quit) => {
                    self.texts.extend_from_slice(&engine.texts[self.turn_texts_seen..]);
                    self.status = Status::Ended(outcome(&engine));
                    self.current = engine;
                    return;
                }
                Ok(TurnEnd::Continue) => {
                    self.texts.extend_from_slice(&engine.texts[self.turn_texts_seen..]);
                    self.turn_texts_seen = 0;
                    engine.texts.clear();
                    engine.pending.clear();
                    self.base = engine;
                    self.entry = TurnEntry::TurnStart;
                    self.turn_decisions.clear();
                }
            }
        }
    }
}

/// `wBattleResult` → outcome.
fn outcome(engine: &Engine) -> Outcome {
    if engine.flags.caught_wild_mon {
        return Outcome::Caught;
    }
    match engine.mem.rb(super::sym::W_BATTLE_RESULT) & crate::defs::BATTLERESULT_BITMASK {
        0 => Outcome::Win,
        1 => Outcome::Loss,
        _ => Outcome::Draw,
    }
}
