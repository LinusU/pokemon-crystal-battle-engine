//! Run the original engine (the transpiled ROM code on the emulated Cpu) and
//! record what it does at every decision point — the reference the
//! hand-written engine is compared against.

use std::sync::{Arc, Mutex};

use super::mem::Mem;
use crate::bootstrap::BattleConfig;
use crate::bootstrap::decision::{Decider, PromptSite, Step};
use crate::runtime::Cpu;

/// The original engine's state at one decision point.
#[derive(Debug, Clone)]
pub struct Snapshot {
    pub site: PromptSite,
    pub legal: Vec<Step>,
    /// The action the policy picked here.
    pub action: Step,
    pub mem: Mem,
    /// The random-stream state (`Cpu::rng_state`).
    pub rng: u64,
    /// How many `Random` calls had happened by this decision (an index into
    /// [`Trace::rng_log`]).
    pub rng_calls: usize,
    /// Battle text printed since the previous decision (exact strings).
    pub texts: Vec<String>,
}

/// A full oracle run.
#[derive(Debug, Clone)]
pub struct Trace {
    pub decisions: Vec<Snapshot>,
    /// The battle's end (just before `ExitBattle`): memory, RNG, the battle
    /// text since the last decision, and the outcome. `site`, `legal` and
    /// `action` are unused.
    pub end: Option<Snapshot>,
    /// Memory and RNG after the game finished (`ExitBattle` included).
    pub end_mem: Mem,
    pub end_rng: u64,
    /// Every `Random` call of the battle, with its caller.
    pub rng_log: Vec<crate::runtime::RngDraw>,
}

/// splitmix64.
pub fn mix(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}

struct RecordingDecider {
    out: Arc<Mutex<Vec<Snapshot>>>,
    end: Arc<Mutex<Option<Snapshot>>>,
    policy: u64,
    max: usize,
    /// When set, the decisions to make (in order) instead of the random policy.
    script: Option<std::collections::VecDeque<Step>>,
}

struct TooLong;

impl Decider for RecordingDecider {
    fn decide(&mut self, cpu: &Cpu, site: PromptSite, legal: &[Step]) -> Step {
        let mut out = self.out.lock().unwrap_or_else(|e| e.into_inner());
        if out.len() >= self.max {
            drop(out);
            std::panic::resume_unwind(Box::new(TooLong));
        }
        // The next decision must come within this many cycles (about two
        // minutes of game time), or the original game has frozen.
        cpu.cycle_limit.set(cpu.cycles + (1 << 30));
        // LONG_BATTLES=1: leave out the actions that end a wild battle (Run, the
        // Poké Doll, balls) so battles go on and use more items.
        let mut pool: Vec<Step> = legal.to_vec();
        if std::env::var_os("LONG_BATTLES").is_some() {
            let ends = |s: &Step| match *s {
                Step::Run => true,
                Step::Item { id, .. } => id == crate::defs::POKE_DOLL || matches!(id, 1..=5 | 0x9d..=0xa6 | 0xb1),
                _ => false,
            };
            let kept: Vec<Step> = pool.iter().copied().filter(|s| !ends(s)).collect();
            if !kept.is_empty() {
                pool = kept;
            }
        }
        let action = match self.script.as_mut() {
            Some(script) => {
                let mut action = script.pop_front().expect("the script ran out of decisions");
                // As in playback: old recordings store FIGHT-without-a-menu
                // (Encore / Bide / Struggle) as a move slot.
                if matches!(action, Step::Move { .. }) && !legal.contains(&action) && legal.contains(&Step::Fight) {
                    action = Step::Fight;
                }
                assert!(legal.contains(&action), "scripted decision {action:?} is not legal at {site:?}");
                action
            }
            None => pool[(mix(&mut self.policy) % pool.len() as u64) as usize],
        };
        out.push(snapshot(cpu, site, legal.to_vec(), action));
        action
    }

    fn battle_ended(&mut self, cpu: &Cpu) {
        let snap = snapshot(cpu, PromptSite::BattleMenu, Vec::new(), Step::Run);
        *self.end.lock().unwrap_or_else(|e| e.into_inner()) = Some(snap);
    }
}

/// The original's state now, as a [`Snapshot`] (texts: everything printed
/// since the last one).
fn snapshot(cpu: &Cpu, site: PromptSite, legal: Vec<Step>, action: Step) -> Snapshot {
    Snapshot {
        site,
        legal,
        action,
        mem: Mem::copy_from(cpu),
        rng: cpu.rng_state(),
        rng_calls: cpu.rng_log.as_ref().map_or(0, Vec::len),
        texts: crate::bootstrap::events::take_events(cpu)
            .into_iter()
            .filter_map(|e| match e {
                crate::bootstrap::events::Event::Text(t) => Some(t),
                _ => None,
            })
            .collect(),
    }
}

/// Run `config` with engine seed `seed` on the original engine, picking a
/// uniformly random legal action (seeded by `policy_seed`) at every decision.
/// `None` if the battle needed more than `max_decisions` decisions.
pub fn run(config: &BattleConfig, seed: u64, policy_seed: u64, max_decisions: usize) -> Option<Trace> {
    run_with_decider(config, seed, policy_seed, max_decisions, None)
}

/// Run `config` on the original engine making exactly the decisions in
/// `script` (a recording's steps), in order. Panics if the script runs out or
/// a decision isn't legal.
pub fn run_scripted(config: &BattleConfig, seed: u64, script: Vec<Step>) -> Option<Trace> {
    let max = script.len() + 1;
    run_with_decider(config, seed, 0, max, Some(script.into()))
}

fn run_with_decider(
    config: &BattleConfig,
    seed: u64,
    policy_seed: u64,
    max_decisions: usize,
    script: Option<std::collections::VecDeque<Step>>,
) -> Option<Trace> {
    let out = Arc::new(Mutex::new(Vec::new()));
    let end = Arc::new(Mutex::new(None));
    let mut cpu = Cpu::new_cgb(None, None);
    cpu.interrupt_vectors = crate::interrupt_vectors();
    cpu.rng_log = Some(Vec::new());
    cpu.cycle_limit.set(1 << 31);
    cpu.events = Some(std::cell::RefCell::new(Default::default()));
    cpu.decider = Some(std::cell::RefCell::new(Box::new(RecordingDecider {
        out: out.clone(),
        end: end.clone(),
        policy: policy_seed,
        max: max_decisions,
        script,
    })));
    let options = crate::bootstrap::RunOptions { seed_override: Some(seed), quiet: true, ..Default::default() };
    let finished = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        crate::bootstrap::run_with(&mut cpu, config, options).expect("battle setup");
    }));
    match finished {
        Ok(()) => {}
        Err(payload)
            if payload.downcast_ref::<TooLong>().is_some()
                || payload.downcast_ref::<crate::runtime::CycleLimit>().is_some() =>
        {
            let first = out.lock().unwrap_or_else(|e| e.into_inner()).first().cloned();
            FIRST.with(|f| *f.borrow_mut() = first);
            return None;
        }
        Err(payload) => std::panic::resume_unwind(payload),
    }
    let decisions = std::mem::take(&mut *out.lock().unwrap_or_else(|e| e.into_inner()));
    let rng_log = cpu.rng_log.take().unwrap_or_default();
    let end = end.lock().unwrap_or_else(|e| e.into_inner()).take();
    Some(Trace { decisions, end, end_mem: Mem::copy_from(&cpu), end_rng: cpu.rng_state(), rng_log })
}

/// Run `config` on the original engine up to its first decision and return
/// that state (how the hand-written engine starts a battle until battle
/// setup is ported).
pub fn first_decision(config: &BattleConfig, seed: u64) -> Option<Snapshot> {
    match run(config, seed, 0, 1) {
        Some(trace) => trace.decisions.into_iter().next(),
        None => FIRST.with(|f| f.borrow_mut().take()),
    }
}

thread_local! {
    static FIRST: std::cell::RefCell<Option<Snapshot>> = const { std::cell::RefCell::new(None) };
}
