//! Replay every player decision a previous run wrote out via [`super::recorder`],
//! and assert the battle state each decision produced.
//!
//! The harness opens a recording file once `bootstrap::run_with` has resolved
//! the seed (the recording's resolved seed wins — see [`load`]), parses every
//! `[[step]]` block into a [`PlayStep`] (action + recorded [`Expect`]), and
//! hands the resulting cursor to the [`Cpu`] in `cpu.player`. Each CLI prompt
//! site in [`super::cli`] consults `cpu.player` *before* falling back to stdin:
//! if a step is queued the prompt is skipped and the recorded answer replayed.
//!
//! Two failure modes are surfaced loudly:
//!
//! - **Structural drift** (a mismatched `kind` or an exhausted script) is fatal
//!   at consume-time — printed on stderr, process exits with status 2. We never
//!   silently substitute, because a replay that drifts is worse than one that
//!   stops.
//! - **Expectation mismatches** (the observed state disagrees with a recorded
//!   `expect.*`) are collected as the battle replays — checked at the *next*
//!   decision point for each step and at battle end for the last — then printed
//!   together with a process exit of status 3. This is what makes a recording
//!   usable as a regression fixture.
//!
//! The format is the one [`super::recorder`] writes, *verbatim*; this module is
//! a strict consumer of it.
//!
//! See [docs/toml_schema.md](../../docs/toml_schema.md) for the human-facing
//! description of the `--play` flag and the recording format.

use std::cell::RefCell;
use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::runtime::Cpu;

use super::recorder::Step;
use super::snapshot::{self, BattleSnapshot, Expect, Outcome};

/// One replayable step: the recorded action plus the tolerant expectations for
/// the state it produced. The `expect` is empty for steps a recording omitted
/// it on (e.g. the in-flight last step of a crashed run, or a hand-written
/// script that only asserts some turns).
#[derive(Debug, Clone)]
pub struct PlayStep {
    pub step: Step,
    pub expect: Expect,
}

/// Parsed `[recording]` header plus its step list.
#[derive(Debug, Clone)]
pub struct Recording {
    /// Resolved RNG seed the original run used. Authoritative — overrides any
    /// `seed = ...` on the scenario.
    pub seed: u64,
    /// Path to the scenario TOML the original run was driven from, exactly as
    /// it appeared in the recording. Used by `--play` to load the same party
    /// and items if no scenario positional arg is supplied.
    pub source_scenario: Option<PathBuf>,
    /// The matchup embedded in the recording (`[scenario.*]`), present for
    /// generated (`--random`) runs. When set, `--play` reconstructs the battle
    /// from this directly instead of loading `source_scenario` or re-running
    /// the generator.
    pub scenario: Option<super::BattleConfig>,
    /// The full `[[step]]` list in order.
    pub steps: Vec<PlayStep>,
}

/// Read `path` and parse the `[recording]` header plus every `[[step]]` block
/// (action + `expect`). Install the steps on the cpu with [`install`].
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
            .into_play_step()
            .map_err(|e| format!("recording {}: step #{}: {}", path.display(), idx, e))?;
        steps.push(step);
    }

    Ok(Recording {
        seed,
        source_scenario: raw.recording.source_scenario.map(PathBuf::from),
        scenario: raw.scenario,
        steps,
    })
}

/// Install the parsed step list on `cpu.player`. The cursor starts at the
/// first step; each [`consume`] call consumes one and advances. Co-exists with
/// [`super::recorder`] — installing both lets you re-record an edited replay.
pub fn install(cpu: &mut Cpu, steps: Vec<PlayStep>) {
    cpu.player = Some(RefCell::new(Player {
        steps,
        cursor: 0,
        mismatches: Vec::new(),
    }));
}

/// Stateful cursor over the recording's step list, plus the running tally of
/// expectation mismatches. One instance lives on the [`Cpu`] for the duration
/// of the replayed battle.
#[derive(Debug)]
pub struct Player {
    steps: Vec<PlayStep>,
    cursor: usize,
    mismatches: Vec<snapshot::Mismatch>,
}

impl Player {
    /// Peek the next action without advancing. `None` when the recording is
    /// exhausted.
    pub fn peek(&self) -> Option<Step> {
        self.steps.get(self.cursor).map(|s| s.step)
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

/// If a recording is installed on `cpu`, consume the next step and return its
/// action. On underflow or kind mismatch this prints a clear stderr message and
/// terminates the process with status 2 — silent substitution would let a
/// drift go unnoticed.
///
/// Returns `None` if no recording is installed, signalling the caller to fall
/// through to its stdin prompt.
pub fn consume(cpu: &Cpu, site: PromptSite, accepts: &[StepKind]) -> Option<Step> {
    let cell = cpu.player.as_ref()?;
    let mut player = cell.borrow_mut();
    let cursor = player.cursor;
    let Some(step) = player.steps.get(cursor).map(|s| s.step) else {
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

/// Assert the *previous* step's recorded expectations against the current
/// battle state. Called at the top of each CLI prompt site (the state there is
/// the outcome of the previous decision). No-op if no recording is installed or
/// we're at the first decision (no previous step yet).
pub fn observe(cpu: &Cpu, snapshot: &BattleSnapshot) {
    let Some(cell) = cpu.player.as_ref() else { return };
    let mut player = cell.borrow_mut();
    check_previous_step(&mut player, snapshot, None);
}

/// Assert the last consumed step's expectations against the final battle state
/// and outcome, then print the verdict. Exits the process with status 3 if any
/// expectation was violated. No-op if no recording is installed.
pub fn finalize(cpu: &Cpu, snapshot: &BattleSnapshot, outcome: Outcome) {
    let Some(cell) = cpu.player.as_ref() else { return };
    let mut player = cell.borrow_mut();
    check_previous_step(&mut player, snapshot, Some(outcome));

    let step_count = player.cursor;
    if player.mismatches.is_empty() {
        eprintln!("playback: {step_count} step(s) replayed; all expectations held");
        return;
    }
    eprintln!(
        "playback: {} expectation mismatch(es) across {step_count} step(s):",
        player.mismatches.len(),
    );
    for m in &player.mismatches {
        eprintln!("  {}: expected {}, got {}", m.path, m.expected, m.actual);
    }
    std::process::exit(3);
}

/// Diff the step at `cursor - 1` (the most recently consumed one) against
/// `snapshot`, tagging each mismatch with its step number. `outcome` is passed
/// through to the diff so `expect.battle.outcome` is only checked at battle end.
fn check_previous_step(player: &mut Player, snapshot: &BattleSnapshot, outcome: Option<Outcome>) {
    let Player { steps, cursor, mismatches } = player;
    if *cursor == 0 {
        return;
    }
    let idx = *cursor - 1;
    let expect = &steps[idx].expect;
    if expect.is_empty() {
        return;
    }
    let mut found = snapshot::diff(expect, snapshot, outcome);
    for m in &mut found {
        m.path = format!("step #{}: {}", idx + 1, m.path);
    }
    mismatches.append(&mut found);
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
// the lines). The shape here mirrors what the recorder emits: the flat action
// fields plus the nested `expect` table.

#[derive(Debug, Deserialize)]
struct RawRecording {
    recording: RawHeader,
    /// The embedded matchup (`[scenario.*]`). Decodes straight into the same
    /// [`BattleConfig`] the standalone scenario TOML uses. Absent for
    /// recordings driven from a scenario file (those carry `source_scenario`).
    #[serde(default)]
    scenario: Option<super::BattleConfig>,
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
    /// Single-move PP items (Ether / Max Ether / MysteryBerry) record the
    /// 0-based move slot to restore. `move` is a Rust keyword, hence the rename.
    /// Absent on older recordings and every non-PP-single item, where it parses
    /// back as `None`.
    #[serde(default, rename = "move")]
    move_slot: Option<u8>,
    #[serde(default)]
    keep: Option<bool>,
    #[serde(default)]
    expect: Expect,
}

impl RawStep {
    fn into_play_step(self) -> Result<PlayStep, String> {
        let RawStep { kind, slot, id, target, move_slot, keep, expect } = self;
        let step = match kind.as_str() {
            "move" => {
                let slot = slot.ok_or("`move` step needs `slot`")?;
                Step::Move { slot }
            }
            "switch" => {
                let slot = slot.ok_or("`switch` step needs `slot`")?;
                Step::Switch { slot }
            }
            "item" => {
                let id = id.ok_or("`item` step needs `id`")?;
                Step::Item { id, target, move_slot }
            }
            "run" => Step::Run,
            "forced_switch" => {
                let slot = slot.ok_or("`forced_switch` step needs `slot`")?;
                Step::ForcedSwitch { slot }
            }
            "offer_switch" => {
                let keep = keep.unwrap_or(false);
                if keep {
                    Step::OfferSwitch { keep: true, slot: None }
                } else {
                    let slot = slot
                        .ok_or("`offer_switch` step needs either `keep = true` or `slot = N`")?;
                    Step::OfferSwitch { keep: false, slot: Some(slot) }
                }
            }
            other => return Err(format!("unknown `kind = {other:?}`")),
        };
        Ok(PlayStep { step, expect })
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
    use crate::bootstrap::snapshot::{ActiveSnapshot, PartySnapshot, SideSnapshot, Status};

    fn temp_path(tag: &str) -> PathBuf {
        std::env::temp_dir().join(format!(
            "battle-playback-{tag}-{}-{}.toml",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
        ))
    }

    /// Round-trip a recording from [`super::super::recorder`] through [`load`],
    /// including the `expect` blocks the recorder now writes.
    #[test]
    fn load_matches_recorder_output() {
        use super::super::recorder;

        fn side(species: &str, hp: u16) -> SideSnapshot {
            SideSnapshot {
                active_slot: 0,
                active: ActiveSnapshot {
                    species: species.into(),
                    level: 10,
                    hp,
                    max_hp: 40,
                    status: Status::None,
                    pp: [25, 0, 0, 0],
                },
                party: vec![PartySnapshot {
                    species: species.into(),
                    level: 10,
                    hp,
                    max_hp: 40,
                    status: Status::None,
                }],
            }
        }
        let snap = |php, ehp| BattleSnapshot { player: side("QUILAVA", php), enemy: side("PIDGEY", ehp) };

        let path = temp_path("rt");
        let mut cpu = Cpu::default();
        recorder::install(&mut cpu, &path, 0xc0ffee, Some(Path::new("scratch/cli.toml")), None)
            .expect("install recorder");
        recorder::record(&cpu, Step::Move { slot: 0 });
        recorder::observe(&cpu, &snap(30, 22));
        recorder::record(&cpu, Step::Item { id: 0x12, target: Some(1), move_slot: None });
        recorder::finalize(&cpu, &snap(30, 0), Outcome::Win);
        cpu.recorder = None;

        let recording = load(&path).expect("load recording");
        let _ = std::fs::remove_file(&path);

        assert_eq!(recording.seed, 0xc0ffee);
        assert_eq!(recording.source_scenario.as_deref(), Some(Path::new("scratch/cli.toml")));
        assert_eq!(recording.steps.len(), 2);
        assert!(matches!(recording.steps[0].step, Step::Move { slot: 0 }));
        assert!(matches!(
            recording.steps[1].step,
            Step::Item { id: 0x12, target: Some(1), move_slot: None }
        ));

        // The first step's expect carries the observed enemy HP, no outcome.
        let e0 = &recording.steps[0].expect;
        assert_eq!(e0.enemy.as_ref().unwrap().active.as_ref().unwrap().hp, Some(22));
        assert!(e0.battle.is_none());

        // The last step carries the final state and the outcome.
        let e1 = &recording.steps[1].expect;
        assert_eq!(e1.enemy.as_ref().unwrap().active.as_ref().unwrap().hp, Some(0));
        assert_eq!(e1.battle.as_ref().unwrap().outcome, Some(Outcome::Win));
    }

    /// A single-move PP item records a `move = N` key, which must round-trip
    /// through the recorder and back into `Step::Item { move_slot: Some(N) }`.
    #[test]
    fn item_step_with_move_slot_round_trips() {
        use super::super::recorder;

        let text = r#"
[recording]
seed = "0x1"

[[step]]
kind = "item"
id = 63
target = 2
move = 1
"#;
        let path = temp_path("movekey");
        std::fs::write(&path, text).unwrap();
        let rec = load(&path).expect("load");
        let _ = std::fs::remove_file(&path);
        assert_eq!(rec.steps.len(), 1);
        assert!(matches!(
            rec.steps[0].step,
            Step::Item { id: 0x3f, target: Some(2), move_slot: Some(1) }
        ));

        // And the recorder writes a key the parser reads back identically.
        let path = temp_path("movekey-rt");
        let mut cpu = Cpu::default();
        recorder::install(&mut cpu, &path, 1, None, None).expect("install");
        recorder::record(&cpu, Step::Item { id: 0x40, target: Some(0), move_slot: Some(3) });
        cpu.recorder = None;
        let rec = load(&path).expect("load rt");
        let _ = std::fs::remove_file(&path);
        assert!(matches!(
            rec.steps[0].step,
            Step::Item { id: 0x40, target: Some(0), move_slot: Some(3) }
        ));
    }

    /// CRITICAL backward-compat: an item step written before the `move` key
    /// existed (no `move = N`) must still parse, with `move_slot = None`.
    #[test]
    fn item_step_without_move_key_parses_as_none() {
        let text = r#"
[recording]
seed = "0x1"

[[step]]
kind = "item"
id = 18
target = 0
"#;
        let path = temp_path("nomovekey");
        std::fs::write(&path, text).unwrap();
        let rec = load(&path).expect("load");
        let _ = std::fs::remove_file(&path);
        assert_eq!(rec.steps.len(), 1);
        assert!(matches!(
            rec.steps[0].step,
            Step::Item { id: 18, target: Some(0), move_slot: None }
        ));
    }

    #[test]
    fn seed_decimal_and_hex_both_parse() {
        assert_eq!(parse_seed("0xc0ffee"), Some(0xc0ffee));
        assert_eq!(parse_seed("0X00000000000000FF"), Some(0xff));
        assert_eq!(parse_seed("42"), Some(42));
        assert_eq!(parse_seed("nonsense"), None);
    }

    /// An embedded `[scenario.*]` block written by the recorder must round-trip
    /// back into `Recording.scenario` — this is what lets `--play` of a recorded
    /// `--random` run reconstruct the matchup without re-running the generator.
    #[test]
    fn embedded_scenario_round_trips() {
        use super::super::recorder;
        use super::super::{BattleConfig, BattleHeader, BattleKind, ItemSpec, MonSpec};

        fn side(species: &str) -> SideSnapshot {
            SideSnapshot {
                active_slot: 0,
                active: ActiveSnapshot {
                    species: species.into(),
                    level: 20,
                    hp: 0,
                    max_hp: 60,
                    status: Status::None,
                    pp: [0, 0, 0, 0],
                },
                party: vec![PartySnapshot {
                    species: species.into(),
                    level: 20,
                    hp: 0,
                    max_hp: 60,
                    status: Status::None,
                }],
            }
        }

        let config = BattleConfig {
            battle: BattleHeader {
                kind: BattleKind::TrainerBattle,
                trainer: Some("Whitney".to_string()),
                // Engine seed lives in the recording header, not here.
                seed: None,
            },
            player_party: vec![
                MonSpec { species: "QUILAVA".into(), level: 20 },
                MonSpec { species: "GEODUDE".into(), level: 18 },
            ],
            enemy_party: vec![],
            player_items: vec![ItemSpec { name: "Potion".into(), quantity: 3 }],
            enemy_items: vec![],
        };

        let path = temp_path("scn");
        let mut cpu = Cpu::default();
        recorder::install(&mut cpu, &path, 0xc0ffee, None, Some(&config)).expect("install");
        recorder::record(&cpu, Step::Move { slot: 0 });
        recorder::finalize(
            &cpu,
            &BattleSnapshot { player: side("QUILAVA"), enemy: side("CLEFAIRY") },
            Outcome::Win,
        );
        cpu.recorder = None;

        let rec = load(&path).expect("load recording");
        let _ = std::fs::remove_file(&path);

        // The engine seed stays in the header; it is not duplicated in scenario.
        assert_eq!(rec.seed, 0xc0ffee);
        let scn = rec.scenario.expect("embedded scenario present");
        assert!(scn.battle.seed.is_none());
        assert!(matches!(scn.battle.kind, BattleKind::TrainerBattle));
        assert_eq!(scn.battle.trainer.as_deref(), Some("Whitney"));
        assert_eq!(scn.player_party.len(), 2);
        assert_eq!(scn.player_party[0].species, "QUILAVA");
        assert_eq!(scn.player_party[1].level, 18);
        assert_eq!(scn.player_items.len(), 1);
        assert_eq!(scn.player_items[0].name, "Potion");
        assert_eq!(scn.player_items[0].quantity, 3);
    }

    #[test]
    fn steps_without_expect_parse_with_empty_expect() {
        let text = r#"
[recording]
seed = "0x1"

[[step]]
kind = "move"
slot = 2
"#;
        let path = temp_path("noexp");
        std::fs::write(&path, text).unwrap();
        let rec = load(&path).expect("load");
        let _ = std::fs::remove_file(&path);
        assert_eq!(rec.steps.len(), 1);
        assert!(rec.steps[0].expect.is_empty());
    }

    #[test]
    fn unknown_kind_errors() {
        let text = r#"
[recording]
seed = "0x1"

[[step]]
kind = "teleport"
"#;
        let path = temp_path("bad");
        std::fs::write(&path, text).unwrap();
        let err = load(&path).unwrap_err();
        let _ = std::fs::remove_file(&path);
        assert!(err.contains("teleport"), "got: {err}");
    }
}
