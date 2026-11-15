//! Differential testing: run a battle on the original engine and on the
//! hand-written engine with the same decisions and compare them at every
//! decision point.

use crate::engine::battle::Engine;
use crate::engine::core::{TurnEnd, TurnEntry};
use crate::engine::mem::Mem;
use super::{Snapshot, Trace};
use crate::engine::start::{self, StartError};
use crate::engine::sym;
use crate::bootstrap::BattleConfig;

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
    // The bag (Items and Balls pockets) and what the item routines leave behind.
    (sym::W_NUM_ITEMS, sym::W_NUM_KEY_ITEMS),
    (sym::W_NUM_BALLS, sym::W_NUM_BALLS + 26),
    (sym::W_CUR_ITEM, sym::W_CUR_ITEM_QUANTITY + 1),
    (sym::W_WILD_MON, sym::W_WILD_MON + 1),
];

/// What the start-state check ([`check_start`]) leaves out of the WRAM
/// comparison: the WRAM the engine doesn't keep (graphics, sound, the live
/// stack, boot/UI scratch; never read by battle code, see docs/engine.md).
/// Everything else in WRAM is compared, byte for byte.
pub const START_IGNORED: &[(u16, u16)] = crate::engine::mem::NOT_KEPT;

/// HRAM bytes the start-state check compares (the rest of HRAM is the
/// boot's, graphics' and the math routines' scratch).
const START_HRAM: &[u16] = &[
    sym::H_BATTLE_TURN, sym::H_RANDOM_ADD, sym::H_RANDOM_SUB, sym::H_SERIAL_CONNECTION_STATUS,
];

/// What the first `BattleTurn` prologue and `AIChooseMove` write on top of
/// the start state. Excluded when the AI isn't ported yet and the start state
/// can only be compared before that turn runs ([`StartCheck::BeforeFirstTurn`]).
const FIRST_TURN_OUTPUTS: &[u16] = &[
    sym::W_TYPE_MODIFIER, sym::W_ATTACK_MISSED, sym::W_CUR_ENEMY_MOVE, sym::W_ENEMY_IS_SWITCHING,
    sym::W_ENEMY_AI_SWITCH_SCORE, sym::W_CUR_DAMAGE, sym::W_CUR_DAMAGE + 1, sym::W_BATTLE_HAS_JUST_STARTED,
    sym::W_NAMED_OBJECT_INDEX, sym::W_CUR_ENEMY_MOVE_NUM,
    // the enemy's move struct, filled in for the move the AI picked
    sym::W_ENEMY_MOVE_STRUCT, sym::W_ENEMY_MOVE_STRUCT + 1, sym::W_ENEMY_MOVE_STRUCT + 2,
    sym::W_ENEMY_MOVE_STRUCT + 3, sym::W_ENEMY_MOVE_STRUCT + 4, sym::W_ENEMY_MOVE_STRUCT + 5,
    sym::W_ENEMY_MOVE_STRUCT + 6,
    // the AI's random draws and its use of the turn byte
    sym::H_BATTLE_TURN, sym::H_RANDOM_ADD, sym::H_RANDOM_SUB,
];

/// The first way the engine differs from the original.
#[derive(Debug, Clone)]
pub enum Divergence {
    /// The engine hit a routine it doesn't have yet (or panicked).
    Panic { decision: usize, message: String },
    /// The engine's own start state (before the first decision) differs.
    Start { diffs: Vec<String> },
    /// The scenario can't start on the engine (see [`StartError::Unsupported`]).
    Unsupported { reason: String },
    Site { decision: usize, oracle: String, engine: String },
    Legal { decision: usize, oracle: String, engine: String },
    Rng { decision: usize, oracle_calls: usize, first_difference: String },
    Memory { decision: usize, diffs: Vec<String> },
    /// The battle text before this decision differs.
    Text { decision: usize, oracle: Vec<String>, engine: Vec<String> },
    /// One engine decided more times than the other.
    Length { oracle: usize, engine: usize },
    /// The battle's end (after the last decision) differs.
    End { what: String },
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

/// How far [`check_start`] could compare.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StartCheck {
    /// The engine reached the first decision: site, legal actions, RNG state,
    /// text and all of WRAM (but [`START_IGNORED`]) match the oracle's.
    Exact,
    /// The first turn needs the unported trainer AI, so the state before it
    /// was compared instead (memory except [`FIRST_TURN_OUTPUTS`], and text).
    BeforeFirstTurn,
}

/// Panic payload to message.
fn panic_message(payload: &(dyn std::any::Any + Send)) -> String {
    payload
        .downcast_ref::<String>()
        .cloned()
        .or_else(|| payload.downcast_ref::<&str>().map(|s| s.to_string()))
        .unwrap_or_default()
}

/// Memory differences over all WRAM except [`START_IGNORED`] (and the
/// [`START_HRAM`] bytes), minus `skip`: the start-state comparison.
pub fn start_mem_diffs(oracle: &Mem, engine: &Mem, skip: &[u16]) -> Vec<String> {
    let mut out = Vec::new();
    let ignored = |addr: u16| START_IGNORED.iter().any(|&(a, b)| (a..b).contains(&addr)) || skip.contains(&addr);
    for addr in (0xc000u16..0xe000).chain(START_HRAM.iter().copied()) {
        let (a, b) = (oracle.rb(addr), engine.rb(addr));
        if a != b && !ignored(addr) {
            out.push(format!("{} ({addr:#06x}): {a:#04x} → {b:#04x}", sym::ram_name(addr)));
        }
    }
    out
}

/// Start the battle on the engine and compare its state with the oracle's
/// first decision (`trace.decisions[0]`) before replaying anything: the
/// engine runs the first turn up to its first decision, then the site, legal
/// actions, RNG state, battle text so far and WRAM (see [`START_IGNORED`])
/// must be identical.
pub fn check_start(config: &BattleConfig, trace: &Trace, check_text: bool) -> Result<StartCheck, Divergence> {
    let Some(first) = trace.decisions.first() else { return Ok(StartCheck::Exact) };
    let seed = config.battle.seed.expect("differential scenarios carry a seed");
    let engine = match start::start(config, seed, check_text) {
        Ok(engine) => engine,
        Err(StartError::Unsupported(reason)) => return Err(Divergence::Unsupported { reason: reason.to_string() }),
        Err(StartError::Invalid(message)) => return Err(Divergence::Panic { decision: 0, message }),
    };
    let before = engine.clone();
    let run = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
        let mut engine = engine;
        let result = engine.battle_turn(TurnEntry::TurnStart);
        (engine, result)
    }));
    let mut diffs = Vec::new();
    let text_diff = |engine: &Engine, diffs: &mut Vec<String>| {
        if check_text {
            let texts = engine.rendered.clone().unwrap_or_default();
            if texts != first.texts {
                diffs.push(format!("text: {:?} → {texts:?}", first.texts));
            }
        }
    };
    match run {
        Ok((engine, Err(suspend))) => {
            if suspend.site != first.site {
                diffs.push(format!("site: {:?} → {:?}", first.site, suspend.site));
            }
            if suspend.legal != first.legal {
                diffs.push(format!("legal: {:?} → {:?}", first.legal, suspend.legal));
            }
            if engine.rng != first.rng {
                diffs.push(format!("rng: {:#x} → {:#x} ({} oracle draws)", first.rng, engine.rng, first.rng_calls));
            }
            text_diff(&engine, &mut diffs);
            diffs.extend(start_mem_diffs(&first.mem, &engine.mem, &[]));
            if diffs.is_empty() { Ok(StartCheck::Exact) } else { Err(Divergence::Start { diffs }) }
        }
        Ok((_, Ok(_))) => Err(Divergence::Start { diffs: vec!["the first turn ended without a decision".to_string()] }),
        Err(payload) => {
            let message = panic_message(&*payload);
            if !message.contains("unported routine AI") {
                return Err(Divergence::Panic { decision: 0, message });
            }
            text_diff(&before, &mut diffs);
            diffs.extend(start_mem_diffs(&first.mem, &before.mem, FIRST_TURN_OUTPUTS));
            if diffs.is_empty() { Ok(StartCheck::BeforeFirstTurn) } else { Err(Divergence::Start { diffs }) }
        }
    }
}

/// Replay `trace` on the engine from its own start state (`start::start`),
/// comparing every decision, the first included.
pub fn check(config: &BattleConfig, trace: &Trace, check_text: bool) -> Result<usize, Divergence> {
    if trace.decisions.is_empty() {
        return Ok(0);
    }
    let seed = config.battle.seed.expect("differential scenarios carry a seed");
    let mut engine = match start::start(config, seed, check_text) {
        Ok(engine) => engine,
        Err(StartError::Unsupported(reason)) => return Err(Divergence::Unsupported { reason: reason.to_string() }),
        Err(StartError::Invalid(message)) => return Err(Divergence::Panic { decision: 0, message }),
    };
    engine.pending = trace.decisions.iter().map(|d| d.action).collect();
    engine.decision_log = Some(Vec::new());
    engine.rng_trace = Some(Vec::new());
    let run = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let mut entry = TurnEntry::TurnStart;
        loop {
            match engine.battle_turn(entry) {
                Ok(TurnEnd::Continue) => entry = TurnEntry::TurnStart,
                Ok(TurnEnd::Quit) => return true,
                Err(_) => return false,
            }
        }
    }));
    // A panic (an unported routine) leaves the log of the decisions before it:
    // those are still compared, so everything up to the panic is verified.
    let ended = matches!(run, Ok(true));
    let panic = run.err().map(|payload| {
        payload
            .downcast_ref::<String>()
            .cloned()
            .or_else(|| payload.downcast_ref::<&str>().map(|s| s.to_string()))
            .unwrap_or_default()
    });
    let log = engine.decision_log.take().unwrap_or_default();
    let texts = std::mem::take(&mut engine.text_log);
    for (i, (snap, (site, legal, mem, rng))) in trace.decisions.iter().zip(&log).enumerate() {
        compare(i, snap, *site, legal, mem, *rng).map_err(|d| match d {
            Divergence::Rng { decision, oracle_calls, .. } => Divergence::Rng {
                decision,
                oracle_calls,
                first_difference: first_rng_difference(trace, engine.rng_trace.as_deref().unwrap_or_default()),
            },
            d => d,
        })?;
        if check_text && snap.texts != texts[i] {
            return Err(Divergence::Text { decision: i, oracle: snap.texts.clone(), engine: texts[i].clone() });
        }
    }
    if let Some(message) = panic {
        return Err(Divergence::Panic { decision: log.len(), message });
    }
    if log.len() != trace.decisions.len() {
        return Err(Divergence::Length { oracle: trace.decisions.len(), engine: log.len() });
    }
    // The last stretch: from the last decision to the end of the battle.
    if let Some(end) = &trace.end {
        if !ended {
            return Err(Divergence::End { what: "the oracle's battle ended, the engine's didn't".into() });
        }
        if end.rng != engine.rng {
            return Err(Divergence::End { what: format!("rng after {} calls", end.rng_calls) });
        }
        let diffs = mem_diffs(&end.mem, &engine.mem);
        if !diffs.is_empty() {
            return Err(Divergence::End { what: format!("memory: {diffs:?}") });
        }
        if check_text {
            let rendered = engine.rendered.take().unwrap_or_default();
            if end.texts != rendered {
                return Err(Divergence::End { what: format!("text: oracle {:?}, engine {:?}", end.texts, rendered) });
            }
        }
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
        return Err(Divergence::Rng { decision: i, oracle_calls: snap.rng_calls, first_difference: String::new() });
    }
    let diffs = mem_diffs(&snap.mem, mem);
    if !diffs.is_empty() {
        return Err(Divergence::Memory { decision: i, diffs });
    }
    Ok(())
}

/// The first random draw where the oracle and the engine differ: who drew
/// it on each side, and the bytes. The engine's trace starts at its first
/// decision; the oracle's log at power-on.
fn first_rng_difference(trace: &Trace, engine: &[(&'static str, u8)]) -> String {
    let Some(first) = trace.decisions.first() else { return String::new() };
    let oracle = &trace.rng_log[first.rng_calls..];
    let name = |d: &crate::runtime::RngDraw| crate::engine::rsym_name(crate::engine::rom::flat(d.bank, d.ret));
    for (k, pair) in oracle.iter().zip(engine).enumerate() {
        let (o, (e, ev)) = pair;
        if o.value != *ev {
            return format!("draw {k}: oracle {} → {:#04x}, engine {e} → {ev:#04x}", name(o), o.value);
        }
    }
    let context = |k: usize| {
        let from = k.saturating_sub(12);
        let o: Vec<String> = oracle[from..(k + 1).min(oracle.len())].iter().map(|d| format!("{}={:#04x}", name(d), d.value)).collect();
        let e: Vec<String> = engine[from..k.min(engine.len())].iter().map(|(n, v)| format!("{n}={v:#04x}")).collect();
        format!(" [oracle: {} | engine: {}]", o.join(", "), e.join(", "))
    };
    match (oracle.len(), engine.len()) {
        (a, b) if a > b => format!("draw {b}: oracle {} → {:#04x}, engine drew nothing more{}", name(&oracle[b]), oracle[b].value, context(b)),
        (a, b) if b > a => format!("draw {a}: engine {} → {:#04x}, oracle drew nothing more", engine[a].0, engine[a].1),
        _ => "the draws match; the generator state differs".to_string(),
    }
}

/// How printing a garbage name went on both engines ([`check_garbage_name`]).
#[derive(Debug)]
pub enum GarbageText {
    /// Same text, same memory.
    Exact,
    /// The original can't print it (it panics, freezes, or leaves WRAM bank
    /// 4 mapped); `engine` is what the engine did.
    OracleFailed { oracle: String, engine: String },
    /// The original printed it; the engine panicked.
    EngineFailed(String),
    Differs(String),
}

/// Print Beat Up's "<name>'s attack!" with the raw bytes `name` in
/// `wStringBuffer1` on both engines, from the same state, and compare the
/// battle text and memory: a check for the text engine on the garbage a
/// glitch can print (control characters, kana, `<MOBILE>` scripts).
pub fn check_garbage_name(bench: &mut super::TextBench, name: &[u8]) -> GarbageText {
    let oracle = bench.print_name(name);
    let mut mem = bench.mem().clone();
    for (i, &b) in name.iter().enumerate() {
        mem.wb(sym::W_STRING_BUFFER1 + i as u16, b);
    }
    let engine = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let mut engine = Engine::new(mem, 0, Default::default());
        let texts = engine.render_text(crate::engine::rsym::BEAT_UP_ATTACK_TEXT, &[]);
        (texts, engine.mem)
    }))
    .map_err(|payload| panic_message(&*payload));
    let (texts, after) = match oracle {
        Err(oracle) => {
            let engine = match engine {
                Ok(_) => "printed it".to_string(),
                Err(e) => format!("panicked: {e}"),
            };
            return GarbageText::OracleFailed { oracle, engine };
        }
        Ok(o) => o,
    };
    let (engine_texts, engine_mem) = match engine {
        Ok(e) => e,
        Err(e) => return GarbageText::EngineFailed(e),
    };
    if texts != engine_texts {
        return GarbageText::Differs(format!("text: oracle {texts:?}, engine {engine_texts:?}"));
    }
    let print_buffer = sym::H_PRINT_NUM_BUFFER..sym::H_PRINT_NUM_BUFFER + 10;
    let diffs: Vec<String> = (0xc000u16..0xe000)
        .filter(|&a| !crate::engine::mem::is_not_kept(a) && !crate::engine::mem::DRIFTS.iter().any(|&(lo, hi)| (lo..hi).contains(&a)))
        .chain(print_buffer)
        .filter(|&a| after.rb(a) != engine_mem.rb(a))
        .map(|a| format!("{} ({a:#06x}): {:#04x} → {:#04x}", sym::ram_name(a), after.rb(a), engine_mem.rb(a)))
        .collect();
    if !diffs.is_empty() {
        return GarbageText::Differs(format!("memory: {}", diffs.join(", ")));
    }
    GarbageText::Exact
}
