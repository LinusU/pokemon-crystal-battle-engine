//! Where battle decisions come from.
//!
//! Every place the engine waits for a choice (the action menu, a move re-pick,
//! a forced switch after a faint, the "switch before the trainer's next mon?"
//! offer, and the enemy's action in a link battle) is a *decision point*. At
//! each one the harness computes the legal choices ([`crate::bootstrap::cli::legal_steps`])
//! and asks, in order: a playback recording, then an installed [`Decider`],
//! then stdin.
//!
//! The library API ([`crate::battle`]) installs a [`Decider`] that hands the
//! decision to the caller of `Battle::step`.

use crate::runtime::Cpu;

pub use super::player::PromptSite;
pub use super::recorder::Step;

/// A source of battle decisions.
pub trait Decider: Send {
    /// Whether this decider answers decisions at `site`. Sites it doesn't
    /// handle fall through to stdin (e.g. an AI playing only the enemy side
    /// while a human plays the player). Defaults to every site.
    fn handles(&self, _site: PromptSite) -> bool {
        true
    }

    /// Pick one of `legal` (never empty) for the decision at `site`. The
    /// harness panics if the returned step is not in `legal`.
    fn decide(&mut self, cpu: &Cpu, site: PromptSite, legal: &[Step]) -> Step;

    /// The battle is over: the original is about to run `ExitBattle` (link
    /// result screen, Pay Day, evolution, Pokérus, `CleanUpBattleRAM`), which
    /// is the game's post-battle processing, not the battle. `cpu` holds the
    /// battle's final state.
    fn battle_ended(&mut self, _cpu: &Cpu) {}
}

/// `ExitBattle` (HARNESS-PATCH, observe only): tell the installed decider the
/// battle has ended.
pub fn battle_ended(cpu: &Cpu) {
    if let Some(cell) = cpu.decider.as_ref() {
        cell.borrow_mut().battle_ended(cpu);
    }
}

/// A [`Decider`] that plays the enemy side (and optionally the player side
/// too) with an [`Agent`](crate::ai::Agent) — the hook for the CLI and the
/// `gui` window (both run the emulated original), where the sides the AI
/// doesn't play are decided on stdin. Install it on `Cpu::decider`.
#[cfg(feature = "ai")]
pub struct AiDecider {
    pub agent: crate::ai::Agent,
    /// Also play the player's side (AI vs AI, to watch).
    pub player: bool,
}

#[cfg(feature = "ai")]
impl AiDecider {
    pub fn new(agent: crate::ai::Agent) -> AiDecider {
        AiDecider { agent, player: false }
    }
}

#[cfg(feature = "ai")]
impl Decider for AiDecider {
    fn handles(&self, site: PromptSite) -> bool {
        self.player || site.side() == crate::battle::Side::Enemy
    }

    fn decide(&mut self, cpu: &Cpu, site: PromptSite, legal: &[Step]) -> Step {
        let state = crate::battle::BattleState::read(cpu);
        self.agent.choose_at(&state, site, site.side(), legal)
    }
}
