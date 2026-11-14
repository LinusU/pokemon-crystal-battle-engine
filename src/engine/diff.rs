//! Differential testing: run a battle on the original engine and on the
//! hand-written engine with the same decisions and compare them at every
//! decision point.

use super::battle::{Engine, Flags};
use super::core::{TurnEnd, TurnEntry};
use super::mem::Mem;
use super::oracle::{Snapshot, Trace};
use super::sym;

/// Memory compared at each decision: battle state, both parties, and the
/// handful of battle variables outside `wBattle`.
pub const COMPARED: &[(u16, u16)] = &[
    (sym::W_BATTLE, sym::W_BATTLE_END),
    (sym::W_ENEMY_MON, sym::W_ENEMY_MON_END),
    (sym::W_OT_PARTY_COUNT, sym::W_OT_PARTY_MON_NICKNAMES),
    (sym::W_PARTY_COUNT, sym::W_PARTY_MON_O_TS),
    (sym::W_CUR_BATTLE_MON, sym::W_CUR_BATTLE_MON + 1),
    (sym::W_BATTLE_PLAYER_ACTION, sym::W_BATTLE_PLAYER_ACTION + 1),
    (sym::W_BATTLE_RESULT, sym::W_BATTLE_RESULT + 1),
    (sym::W_BATTLE_ACTION, sym::W_BATTLE_ACTION + 1),
    (sym::H_BATTLE_TURN, sym::H_BATTLE_TURN + 1),
];

/// The first way the engine differs from the original.
#[derive(Debug, Clone)]
pub enum Divergence {
    /// The engine hit a routine it doesn't have yet (or panicked).
    Panic { decision: usize, message: String },
    Site { decision: usize, oracle: String, engine: String },
    Legal { decision: usize, oracle: String, engine: String },
    Rng { decision: usize, oracle_calls: usize },
    Memory { decision: usize, diffs: Vec<String> },
    /// One engine decided more times than the other.
    Length { oracle: usize, engine: usize },
}

/// Memory differences over [`COMPARED`], as `symbol: oracle → engine`.
pub fn mem_diffs(oracle: &Mem, engine: &Mem) -> Vec<String> {
    let mut out = Vec::new();
    for &(start, end) in COMPARED {
        for addr in start..end {
            let (a, b) = (oracle.rb(addr), engine.rb(addr));
            if a != b {
                out.push(format!("{}: {a:#04x} → {b:#04x}", sym::ram_name(addr)));
            }
        }
    }
    out
}

/// Replay `trace` on the engine, starting from the oracle's first decision.
pub fn check(trace: &Trace, link: bool) -> Result<usize, Divergence> {
    let Some(first) = trace.decisions.first() else { return Ok(0) };
    let mut engine = Engine::new(first.mem.clone(), first.rng, Flags { link_battle: link, ..Flags::default() });
    engine.pending = trace.decisions.iter().map(|d| d.action).collect();
    engine.decision_log = Some(Vec::new());
    let run = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let mut entry = TurnEntry::BattleMenu;
        loop {
            match engine.battle_turn(entry) {
                Ok(TurnEnd::Continue) => entry = TurnEntry::TurnStart,
                Ok(TurnEnd::Quit) | Err(_) => break,
            }
        }
        engine
    }));
    let (log, panic) = match run {
        Ok(engine) => (engine.decision_log.unwrap(), None),
        Err(payload) => {
            let msg = payload
                .downcast_ref::<String>()
                .cloned()
                .or_else(|| payload.downcast_ref::<&str>().map(|s| s.to_string()))
                .unwrap_or_default();
            (Vec::new(), Some(msg))
        }
    };
    if let Some(message) = panic {
        // We lost the log; re-run up to the panic to find how far it got.
        return Err(Divergence::Panic { decision: usize::MAX, message });
    }
    for (i, (snap, (site, legal, mem, rng))) in trace.decisions.iter().zip(&log).enumerate() {
        compare(i, snap, *site, legal, mem, *rng)?;
    }
    if log.len() != trace.decisions.len() {
        return Err(Divergence::Length { oracle: trace.decisions.len(), engine: log.len() });
    }
    Ok(log.len())
}

fn compare(
    i: usize,
    snap: &Snapshot,
    site: crate::bootstrap::decision::PromptSite,
    legal: &[crate::bootstrap::decision::Step],
    mem: &Mem,
    rng: u64,
) -> Result<(), Divergence> {
    if snap.site != site {
        return Err(Divergence::Site { decision: i, oracle: format!("{:?}", snap.site), engine: format!("{site:?}") });
    }
    if snap.legal != legal {
        return Err(Divergence::Legal { decision: i, oracle: format!("{:?}", snap.legal), engine: format!("{legal:?}") });
    }
    if snap.rng != rng {
        return Err(Divergence::Rng { decision: i, oracle_calls: snap.rng_calls });
    }
    let diffs = mem_diffs(&snap.mem, mem);
    if !diffs.is_empty() {
        return Err(Divergence::Memory { decision: i, diffs });
    }
    Ok(())
}
