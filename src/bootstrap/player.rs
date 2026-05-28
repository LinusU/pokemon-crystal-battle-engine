//! Replay every player decision a previous run wrote out via [`super::recorder`].
//!
//! The harness opens a recording file once `bootstrap::run_with` has resolved
//! the seed (the recording's resolved seed wins — see [`load`]), parses every
//! `[[step]]` block into a [`Step`] from `super::recorder`, and hands the
//! resulting cursor to the [`Cpu`] in `cpu.player`. Each CLI prompt site in
//! [`super::cli`] consults `cpu.player` *before* falling back to stdin: if a
//! step is queued the prompt is skipped, the recorded answer is replayed,
//! and the rest of the engine never knows the difference.
//!
//! Errors at consume-time are fatal — a mismatched `kind` or an exhausted
//! script is treated as a corrupt replay, printed on stderr and turned into
//! a non-zero process exit. We never silently substitute, because a replay
//! that drifts is worse than one that stops.
//!
//! The format is the one [`super::recorder`] writes, *verbatim*. The
//! recording agent owns the schema; this module is a strict consumer.
//!
//! See [docs/toml_schema.md](../../docs/toml_schema.md) for the human-facing
//! description of the `--play` flag and the recording format.

use std::cell::RefCell;
use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::runtime::Cpu;

use super::recorder::Step;

/// Parsed `[recording]` header.
#[derive(Debug, Clone)]
pub struct Recording {
    /// Resolved RNG seed the original run used. Authoritative — overrides any
    /// `seed = ...` on the scenario.
    pub seed: u64,
    /// Path to the scenario TOML the original run was driven from, exactly as
    /// it appeared in the recording. Used by `--play` to load the same party
    /// and items if no scenario positional arg is supplied.
    pub source_scenario: Option<PathBuf>,
    /// The full `[[step]]` list in order.
    pub steps: Vec<Step>,
}

/// Read `path` and parse the `[recording]` header plus every `[[step]]` block.
/// The resolved seed and (optional) `source_scenario` are returned alongside
/// the parsed step list; install the steps on the cpu with [`install`].
pub fn load(path: &Path) -> Result<Recording, String> {
    let text = std::fs::read_to_string(path)
        .map_err(|e| format!("could not read recording {}: {}", path.display(), e))?;
    let raw: RawRecording = toml::from_str(&text)
        .map_err(|e| format!("could not parse recording {}: {}", path.display(), e))?;

    let seed = parse_seed(&raw.recording.seed).ok_or_else(|| {
        format!(
            "recording {}: invalid `seed = {:?}` in [recording] header (expected decimal or 0x-hex)",
            path.display(),
            raw.recording.seed,
        )
    })?;

    let mut steps = Vec::with_capacity(raw.step.len());
    for (idx, raw_step) in raw.step.into_iter().enumerate() {
        let step = raw_step
            .into_step()
            .map_err(|e| format!("recording {}: step #{}: {}", path.display(), idx, e))?;
        steps.push(step);
    }

    Ok(Recording {
        seed,
        source_scenario: raw.recording.source_scenario.map(PathBuf::from),
        steps,
    })
}

/// Install the parsed step list on `cpu.player`. The cursor starts at the
/// first step; each `next_*` call consumes one and advances. Co-exists with
/// [`super::recorder`] — installing both lets you re-record an edited replay.
pub fn install(cpu: &mut Cpu, steps: Vec<Step>) {
    cpu.player = Some(RefCell::new(Player { steps, cursor: 0 }));
}

/// Stateful cursor over the recording's step list. One instance lives on the
/// [`Cpu`] for the duration of the replayed battle.
#[derive(Debug)]
pub struct Player {
    steps: Vec<Step>,
    cursor: usize,
}

impl Player {
    /// Peek the next step without advancing. `None` when the recording is
    /// exhausted.
    pub fn peek(&self) -> Option<Step> {
        self.steps.get(self.cursor).copied()
    }
}

/// What kind of decision the CLI driver is asking for. Used for the
/// mismatch-error message when the recorded `kind` doesn't fit.
#[derive(Debug, Clone, Copy)]
pub enum PromptSite {
    BattleMenu,
    MoveSelection,
    ForcedSwitch,
    OfferSwitch,
}

impl PromptSite {
    fn label(self) -> &'static str {
        match self {
            PromptSite::BattleMenu => "battle menu (move/switch/item/run)",
            PromptSite::MoveSelection => "move re-selection (Disable / no-PP)",
            PromptSite::ForcedSwitch => "forced switch (active fainted)",
            PromptSite::OfferSwitch => "offer switch (enemy sending in next mon)",
        }
    }
}

/// If a recording is installed on `cpu`, consume the next step and return it.
/// On underflow or kind mismatch this prints a clear stderr message and
/// terminates the process with status 2 — silent substitution would let a
/// drift go unnoticed.
///
/// Returns `None` if no recording is installed, signalling the caller to fall
/// through to its stdin prompt.
pub fn consume(cpu: &Cpu, site: PromptSite, accepts: &[StepKind]) -> Option<Step> {
    let cell = cpu.player.as_ref()?;
    let mut player = cell.borrow_mut();
    let cursor = player.cursor;
    let Some(step) = player.steps.get(cursor).copied() else {
        eprintln!(
            "playback: ran out of steps at decision #{} ({}) — recording is incomplete",
            cursor + 1,
            site.label(),
        );
        std::process::exit(2);
    };
    let kind = step.kind();
    if !accepts.contains(&kind) {
        let expected: Vec<&'static str> = accepts.iter().map(|k| k.as_str()).collect();
        eprintln!(
            "playback: step #{} has kind = {:?} but the engine is asking for {} ({}); expected one of {:?}",
            cursor + 1,
            kind.as_str(),
            site.label(),
            site_str(site),
            expected,
        );
        std::process::exit(2);
    }
    player.cursor += 1;
    Some(step)
}

fn site_str(site: PromptSite) -> &'static str {
    match site {
        PromptSite::BattleMenu => "battle_menu",
        PromptSite::MoveSelection => "move_selection",
        PromptSite::ForcedSwitch => "forced_switch",
        PromptSite::OfferSwitch => "offer_switch",
    }
}

/// Tag for the variants in [`Step`]. Used at consume sites so each prompt can
/// declare which `kind`s it will accept without pattern-matching the whole
/// enum inline.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepKind {
    Move,
    Switch,
    Item,
    Run,
    ForcedSwitch,
    OfferSwitch,
}

impl StepKind {
    fn as_str(self) -> &'static str {
        match self {
            StepKind::Move => "move",
            StepKind::Switch => "switch",
            StepKind::Item => "item",
            StepKind::Run => "run",
            StepKind::ForcedSwitch => "forced_switch",
            StepKind::OfferSwitch => "offer_switch",
        }
    }
}

trait StepExt {
    fn kind(&self) -> StepKind;
}

impl StepExt for Step {
    fn kind(&self) -> StepKind {
        match self {
            Step::Move { .. } => StepKind::Move,
            Step::Switch { .. } => StepKind::Switch,
            Step::Item { .. } => StepKind::Item,
            Step::Run => StepKind::Run,
            Step::ForcedSwitch { .. } => StepKind::ForcedSwitch,
            Step::OfferSwitch { .. } => StepKind::OfferSwitch,
        }
    }
}

// ---------------------------------------------------------------------------
// TOML decoding
//
// We don't `#[derive(Deserialize)]` on [`Step`] directly — that type lives in
// `recorder` and stays serializer-free on purpose (the recorder hand-writes
// the lines). The shape here mirrors what the recorder emits.

#[derive(Debug, Deserialize)]
struct RawRecording {
    recording: RawHeader,
    #[serde(default)]
    step: Vec<RawStep>,
}

#[derive(Debug, Deserialize)]
struct RawHeader {
    /// String so we can accept the recorder's `0x...` quoted hex. Plain
    /// decimal works too via the same fallback path.
    seed: String,
    #[serde(default)]
    source_scenario: Option<String>,
}

#[derive(Debug, Deserialize)]
struct RawStep {
    kind: String,
    #[serde(default)]
    slot: Option<u8>,
    #[serde(default)]
    id: Option<u8>,
    #[serde(default)]
    target: Option<u8>,
    #[serde(default)]
    keep: Option<bool>,
}

impl RawStep {
    fn into_step(self) -> Result<Step, String> {
        match self.kind.as_str() {
            "move" => {
                let slot = self.slot.ok_or("`move` step needs `slot`")?;
                Ok(Step::Move { slot })
            }
            "switch" => {
                let slot = self.slot.ok_or("`switch` step needs `slot`")?;
                Ok(Step::Switch { slot })
            }
            "item" => {
                let id = self.id.ok_or("`item` step needs `id`")?;
                Ok(Step::Item {
                    id,
                    target: self.target,
                })
            }
            "run" => Ok(Step::Run),
            "forced_switch" => {
                let slot = self.slot.ok_or("`forced_switch` step needs `slot`")?;
                Ok(Step::ForcedSwitch { slot })
            }
            "offer_switch" => {
                let keep = self.keep.unwrap_or(false);
                if keep {
                    Ok(Step::OfferSwitch {
                        keep: true,
                        slot: None,
                    })
                } else {
                    let slot = self.slot.ok_or(
                        "`offer_switch` step needs either `keep = true` or `slot = N`",
                    )?;
                    Ok(Step::OfferSwitch {
                        keep: false,
                        slot: Some(slot),
                    })
                }
            }
            other => Err(format!("unknown `kind = {other:?}`")),
        }
    }
}

fn parse_seed(s: &str) -> Option<u64> {
    let trimmed = s.trim();
    if let Some(rest) = trimmed.strip_prefix("0x").or_else(|| trimmed.strip_prefix("0X")) {
        u64::from_str_radix(rest, 16).ok()
    } else {
        trimmed.parse::<u64>().ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Round-trip a recording from [`super::recorder`] through [`load`] to
    /// make sure the parser accepts the exact text the recorder writes.
    #[test]
    fn load_matches_recorder_output() {
        use super::super::recorder;

        let dir = std::env::temp_dir();
        let path = dir.join(format!(
            "battle-playback-test-{}-{}.toml",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
        ));

        let mut cpu = Cpu::default();
        recorder::install(&mut cpu, &path, 0xc0ffee, Some(Path::new("scratch/cli.toml")))
            .expect("install recorder");
        recorder::record(&cpu, Step::Move { slot: 0 });
        recorder::record(&cpu, Step::Switch { slot: 2 });
        recorder::record(&cpu, Step::Item { id: 0x12, target: Some(1) });
        recorder::record(&cpu, Step::Item { id: 0x04, target: None });
        recorder::record(&cpu, Step::Run);
        recorder::record(&cpu, Step::ForcedSwitch { slot: 3 });
        recorder::record(&cpu, Step::OfferSwitch { keep: true, slot: None });
        recorder::record(&cpu, Step::OfferSwitch { keep: false, slot: Some(4) });
        cpu.recorder = None;

        let recording = load(&path).expect("load recording");
        let _ = std::fs::remove_file(&path);

        assert_eq!(recording.seed, 0xc0ffee);
        assert_eq!(
            recording.source_scenario.as_deref(),
            Some(Path::new("scratch/cli.toml")),
        );
        assert_eq!(recording.steps.len(), 8);
        assert!(matches!(recording.steps[0], Step::Move { slot: 0 }));
        assert!(matches!(recording.steps[1], Step::Switch { slot: 2 }));
        assert!(matches!(
            recording.steps[2],
            Step::Item { id: 0x12, target: Some(1) }
        ));
        assert!(matches!(
            recording.steps[3],
            Step::Item { id: 0x04, target: None }
        ));
        assert!(matches!(recording.steps[4], Step::Run));
        assert!(matches!(recording.steps[5], Step::ForcedSwitch { slot: 3 }));
        assert!(matches!(
            recording.steps[6],
            Step::OfferSwitch { keep: true, slot: None }
        ));
        assert!(matches!(
            recording.steps[7],
            Step::OfferSwitch { keep: false, slot: Some(4) }
        ));
    }

    #[test]
    fn seed_decimal_and_hex_both_parse() {
        assert_eq!(parse_seed("0xc0ffee"), Some(0xc0ffee));
        assert_eq!(parse_seed("0X00000000000000FF"), Some(0xff));
        assert_eq!(parse_seed("42"), Some(42));
        assert_eq!(parse_seed("nonsense"), None);
    }

    #[test]
    fn unknown_kind_errors() {
        let text = r#"
[recording]
seed = "0x1"

[[step]]
kind = "teleport"
"#;
        let path = std::env::temp_dir().join(format!(
            "battle-playback-bad-{}-{}.toml",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
        ));
        std::fs::write(&path, text).unwrap();
        let err = load(&path).unwrap_err();
        let _ = std::fs::remove_file(&path);
        assert!(err.contains("teleport"), "got: {err}");
    }
}
