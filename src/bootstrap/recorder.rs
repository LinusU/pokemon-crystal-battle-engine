//! Record every player decision the CLI driver in [`super::cli`] resolves,
//! together with the battle state each decision produced.
//!
//! The harness opens a recording file once `bootstrap::run` has resolved the
//! seed, writes a `[recording]` header that always includes the *resolved*
//! seed (whether the scenario supplied one or it was drawn from system
//! entropy), then hands a writer to the [`Cpu`] in `cpu.recorder`.
//!
//! For generated battles (`--random`) the header is followed by an embedded
//! `[scenario.*]` block holding the resolved matchup, so the recording is
//! self-contained and `--play` never has to re-run the generator — see
//! [`write_scenario`] and [`super::player`].
//!
//! Each `[[step]]` table is written in two halves:
//!
//! 1. [`record`] appends the action (`kind` + its fields) the instant the CLI
//!    resolves it, and flushes — so a mid-battle panic still leaves a usable
//!    script on disk.
//! 2. [`observe`] appends that same step's `expect.*` keys at the *next*
//!    decision point — i.e. the battle state the action produced, which is the
//!    exact state the next step takes its action at. [`finalize`] does the same
//!    for the last step after the battle ends, and additionally records the
//!    battle outcome.
//!
//! Because nothing else is written between [`record`] and the following
//! [`observe`], the `expect.*` keys land inside the still-open `[[step]]` table
//! with no buffering: the file is valid TOML at every flush point, and a crash
//! at worst leaves the in-flight step without its `expect` block.
//!
//! No TOML serializer is pulled in here — the lines are short and fixed, and
//! writing them by hand keeps the on-disk output deterministic and easy to
//! diff. The companion tests parse the produced text with `toml` to make sure
//! we didn't drift, and [`super::player`] re-parses it on playback.
//!
//! See [docs/toml_schema.md](../../docs/toml_schema.md) for the human-facing
//! description of the `--record` flag and recording format.

use std::cell::RefCell;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use super::snapshot::{BattleSnapshot, Outcome, SideSnapshot};
use super::{BattleConfig, ItemSpec, MonSpec};
use crate::runtime::Cpu;

/// Which side a recorded decision belongs to. A normal (wild / trainer) battle
/// only ever records `Player` steps and writes no `side` key, so those
/// recordings — and every fixture made before link battles existed — parse back
/// as `Player`. A link battle additionally records `Enemy` steps for the second
/// CLI player's move / switch picks, tagged `side = "enemy"`. See
/// [`super::cli::handle_enemy_action`] and docs/toml_schema.md.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Player,
    Enemy,
}

impl Side {
    /// The `side` tag written into / parsed from the recording TOML. `Player`
    /// returns `None` so player steps stay key-less (back-compat); only `Enemy`
    /// writes `side = "enemy"`.
    pub fn tag(self) -> Option<&'static str> {
        match self {
            Side::Player => None,
            Side::Enemy => Some("enemy"),
        }
    }
}

/// One captured CLI decision. Each variant maps 1:1 to one of the prompt sites
/// in [`super::cli`]. This is the *action* half of a `[[step]]`; the *outcome*
/// half is the `expect.*` keys written by [`observe`] / [`finalize`]. The side
/// the decision belongs to is tracked separately (see [`Side`] / [`record_side`])
/// rather than on the variant, since the action shape is identical for both.
#[derive(Debug, Clone, Copy)]
pub enum Step {
    /// Action menu picked a move. `slot` is the 0-based move slot.
    Move { slot: u8 },
    /// Action menu picked a switch. `slot` is the 0-based party slot of
    /// the incoming mon.
    Switch { slot: u8 },
    /// Action menu picked an item. `target` is `Some(slot)` for items that
    /// target a specific party mon and `None` otherwise. `move_slot` is
    /// `Some(slot)` for single-move PP items (Ether / Max Ether / MysteryBerry)
    /// that target one move of the targeted mon, and `None` for every other
    /// item (whole-party PP, HP, status, revive). Older recordings have no
    /// `move` key, which parses back as `move_slot = None`.
    Item { id: u8, target: Option<u8>, move_slot: Option<u8> },
    /// Action menu picked Run.
    Run,
    /// `force_pick_party_mon_in_battle` — the active mon fainted and the
    /// engine forces us to send in another. `slot` is the chosen slot.
    ForcedSwitch { slot: u8 },
    /// `offer_switch` (trainer about to send in another mon). Either we
    /// keep the current mon (`keep = true`, `slot = None`) or we swap in
    /// the chosen slot.
    OfferSwitch { keep: bool, slot: Option<u8> },
}

/// Writer + the one bit of state we track between [`record`] and [`observe`]:
/// whether the most recent `[[step]]` is still open for its `expect.*` keys.
#[derive(Debug)]
pub struct Recorder {
    writer: BufWriter<File>,
    awaiting_expect: bool,
}

/// Open `path` for write (truncating any existing file), write the
/// `[recording]` header, and install the resulting writer on `cpu.recorder`.
/// The seed is always written even when the source scenario omitted one —
/// it's the resolved value used to seed the RNG so a replay is bit-identical.
///
/// When `scenario` is `Some`, the *resolved* matchup is embedded as a
/// `[scenario.*]` block right after the header. This makes recordings of
/// generated battles (`--random`) self-contained: `--play` reconstructs the
/// matchup from this block instead of re-running the generator, so the
/// generator can change without ever invalidating the recording.
pub fn install(
    cpu: &mut Cpu,
    path: &Path,
    seed: u64,
    source_scenario: Option<&Path>,
    scenario: Option<&BattleConfig>,
) -> Result<(), String> {
    let file = File::create(path)
        .map_err(|e| format!("could not open recording file {}: {}", path.display(), e))?;
    let mut writer = BufWriter::new(file);
    write_header(&mut writer, seed, source_scenario)
        .map_err(|e| format!("could not write recording header: {e}"))?;
    if let Some(config) = scenario {
        write_scenario(&mut writer, config)
            .map_err(|e| format!("could not write embedded scenario: {e}"))?;
    }
    writer
        .flush()
        .map_err(|e| format!("could not flush recording header: {e}"))?;
    cpu.recorder = Some(RefCell::new(Recorder { writer, awaiting_expect: false }));
    Ok(())
}

fn write_header(
    writer: &mut BufWriter<File>,
    seed: u64,
    source_scenario: Option<&Path>,
) -> std::io::Result<()> {
    writeln!(writer, "[recording]")?;
    if let Some(p) = source_scenario {
        // `to_string_lossy` is fine — paths the user typed at the CLI are
        // overwhelmingly UTF-8, and a non-UTF-8 byte just becomes U+FFFD
        // in the recorded label rather than crashing.
        writeln!(writer, "source_scenario = {}", toml_string(&p.to_string_lossy()))?;
    }
    writeln!(writer, "seed = \"0x{seed:016x}\"")?;
    Ok(())
}

/// Embed the resolved matchup as a `[scenario.*]` block. The shape mirrors the
/// standalone scenario TOML (`[battle]`, `[[player_party]]`, …) one level down
/// under `scenario`, so [`super::player`] decodes it straight into a
/// [`BattleConfig`]. The engine seed already lives in the `[recording]` header,
/// so `seed` is intentionally omitted here — there is one source of truth.
///
/// Written once at install time, before any `[[step]]`, so the file stays valid
/// TOML at every flush point.
fn write_scenario(writer: &mut BufWriter<File>, config: &BattleConfig) -> std::io::Result<()> {
    writeln!(writer)?;
    writeln!(writer, "[scenario.battle]")?;
    writeln!(writer, "type = {}", toml_string(config.battle.kind.as_str()))?;
    if let Some(trainer) = &config.battle.trainer {
        writeln!(writer, "trainer = {}", toml_string(trainer))?;
    }
    write_party(writer, "scenario.player_party", &config.player_party)?;
    write_party(writer, "scenario.enemy_party", &config.enemy_party)?;
    write_items(writer, "scenario.player_items", &config.player_items)?;
    write_items(writer, "scenario.enemy_items", &config.enemy_items)?;
    Ok(())
}

fn write_party(writer: &mut BufWriter<File>, table: &str, party: &[MonSpec]) -> std::io::Result<()> {
    for mon in party {
        writeln!(writer)?;
        writeln!(writer, "[[{table}]]")?;
        writeln!(writer, "species = {}", toml_string(&mon.species))?;
        writeln!(writer, "level = {}", mon.level)?;
        if let Some(moves) = &mon.moves {
            let rendered: Vec<String> = moves.iter().map(|m| toml_string(m)).collect();
            writeln!(writer, "moves = [{}]", rendered.join(", "))?;
        }
    }
    Ok(())
}

fn write_items(writer: &mut BufWriter<File>, table: &str, items: &[ItemSpec]) -> std::io::Result<()> {
    for item in items {
        writeln!(writer)?;
        writeln!(writer, "[[{table}]]")?;
        writeln!(writer, "name = {}", toml_string(&item.name))?;
        writeln!(writer, "quantity = {}", item.quantity)?;
    }
    Ok(())
}

/// Append a player-side `[[step]]` action block. Shorthand for
/// [`record_side`] with [`Side::Player`] — the common case, and what every
/// non-link battle records.
pub fn record(cpu: &Cpu, step: Step) {
    record_side(cpu, Side::Player, step);
}

/// Append a `[[step]]` action block (tagged with `side` when it's the enemy's)
/// and flush. Silently no-ops if the cpu has no recorder installed — callers
/// always invoke this; the runtime field is what gates recording.
///
/// The step is left "open": its `expect.*` keys are filled by the next
/// [`observe`] (or [`finalize`] if it turns out to be the last step).
pub fn record_side(cpu: &Cpu, side: Side, step: Step) {
    let Some(cell) = cpu.recorder.as_ref() else { return };
    let mut rec = cell.borrow_mut();
    if let Err(e) = write_step(&mut rec.writer, side, step) {
        // Don't bubble — we're mid-battle and the recording is best-effort.
        // The error is logged once so an unwriteable file doesn't go silent.
        eprintln!("recorder: failed to write step: {e}");
        return;
    }
    rec.awaiting_expect = true;
    if let Err(e) = rec.writer.flush() {
        eprintln!("recorder: failed to flush step: {e}");
    }
}

/// Append the open step's `expect.*` keys from `snapshot` — the state the just
/// recorded action produced. No-op if no step is open (e.g. the first decision
/// point, before any action) or no recorder is installed.
pub fn observe(cpu: &Cpu, snapshot: &BattleSnapshot) {
    write_outcome(cpu, snapshot, None);
}

/// Like [`observe`], but for the final step after the battle ends: also records
/// the battle `outcome`.
pub fn finalize(cpu: &Cpu, snapshot: &BattleSnapshot, outcome: Outcome) {
    write_outcome(cpu, snapshot, Some(outcome));
}

fn write_outcome(cpu: &Cpu, snapshot: &BattleSnapshot, outcome: Option<Outcome>) {
    let Some(cell) = cpu.recorder.as_ref() else { return };
    let mut rec = cell.borrow_mut();
    if !rec.awaiting_expect {
        return;
    }
    if let Err(e) = write_expect(&mut rec.writer, snapshot, outcome) {
        eprintln!("recorder: failed to write expect block: {e}");
        return;
    }
    rec.awaiting_expect = false;
    if let Err(e) = rec.writer.flush() {
        eprintln!("recorder: failed to flush expect block: {e}");
    }
}

fn write_step(writer: &mut BufWriter<File>, side: Side, step: Step) -> std::io::Result<()> {
    writeln!(writer)?;
    writeln!(writer, "[[step]]")?;
    // `side = "enemy"` only for the second CLI player's link-battle decisions;
    // player steps stay key-less so older recordings round-trip unchanged.
    if let Some(tag) = side.tag() {
        writeln!(writer, "side = \"{tag}\"")?;
    }
    match step {
        Step::Move { slot } => {
            writeln!(writer, "kind = \"move\"")?;
            writeln!(writer, "slot = {slot}")?;
        }
        Step::Switch { slot } => {
            writeln!(writer, "kind = \"switch\"")?;
            writeln!(writer, "slot = {slot}")?;
        }
        Step::Item { id, target, move_slot } => {
            writeln!(writer, "kind = \"item\"")?;
            writeln!(writer, "id = {id}")?;
            if let Some(t) = target {
                writeln!(writer, "target = {t}")?;
            }
            if let Some(m) = move_slot {
                writeln!(writer, "move = {m}")?;
            }
        }
        Step::Run => {
            writeln!(writer, "kind = \"run\"")?;
        }
        Step::ForcedSwitch { slot } => {
            writeln!(writer, "kind = \"forced_switch\"")?;
            writeln!(writer, "slot = {slot}")?;
        }
        Step::OfferSwitch { keep, slot } => {
            writeln!(writer, "kind = \"offer_switch\"")?;
            if keep {
                writeln!(writer, "keep = true")?;
            } else if let Some(s) = slot {
                writeln!(writer, "slot = {s}")?;
            }
        }
    }
    Ok(())
}

/// Write the `expect.*` dotted keys for one snapshot into the open `[[step]]`
/// table. Mirrors the shape [`super::snapshot::Expect`] parses back.
fn write_expect(
    writer: &mut BufWriter<File>,
    snapshot: &BattleSnapshot,
    outcome: Option<Outcome>,
) -> std::io::Result<()> {
    write_side(writer, "player", &snapshot.player)?;
    write_side(writer, "enemy", &snapshot.enemy)?;
    if let Some(o) = outcome {
        writeln!(writer, "expect.battle.outcome = {}", toml_string(o.as_str()))?;
    }
    Ok(())
}

fn write_side(writer: &mut BufWriter<File>, side: &str, s: &SideSnapshot) -> std::io::Result<()> {
    writeln!(writer, "expect.{side}.active_slot = {}", s.active_slot)?;
    let a = &s.active;
    writeln!(writer, "expect.{side}.active.species = {}", toml_string(&a.species))?;
    writeln!(writer, "expect.{side}.active.level = {}", a.level)?;
    writeln!(writer, "expect.{side}.active.hp = {}", a.hp)?;
    writeln!(writer, "expect.{side}.active.max_hp = {}", a.max_hp)?;
    writeln!(writer, "expect.{side}.active.status = {}", toml_string(a.status.as_str()))?;
    writeln!(
        writer,
        "expect.{side}.active.pp = [{}, {}, {}, {}]",
        a.pp[0], a.pp[1], a.pp[2], a.pp[3],
    )?;
    // Party as an inline array of inline tables — keeps the whole block inside
    // the open `[[step]]` table so streaming never needs a sub-table header.
    write!(writer, "expect.{side}.party = [")?;
    for (i, m) in s.party.iter().enumerate() {
        if i > 0 {
            write!(writer, ", ")?;
        }
        write!(
            writer,
            "{{ species = {}, level = {}, hp = {}, max_hp = {}, status = {} }}",
            toml_string(&m.species),
            m.level,
            m.hp,
            m.max_hp,
            toml_string(m.status.as_str()),
        )?;
    }
    writeln!(writer, "]")?;
    Ok(())
}

/// Quote a string as a TOML basic string. The recorded values are paths,
/// species names, and fixed tags with no backslash / control-char surprises,
/// so the only escapes we need are `"` and `\`.
fn toml_string(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bootstrap::snapshot::{ActiveSnapshot, PartySnapshot, SideSnapshot, Status};
    use std::path::PathBuf;

    fn temp_path(tag: &str) -> PathBuf {
        std::env::temp_dir().join(format!(
            "battle-recording-{tag}-{}-{}.toml",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
        ))
    }

    fn side(species: &str, hp: u16, slot: u8) -> SideSnapshot {
        SideSnapshot {
            active_slot: slot,
            active: ActiveSnapshot {
                species: species.to_string(),
                level: 10,
                hp,
                max_hp: 40,
                status: Status::None,
                pp: [25, 10, 0, 0],
            },
            party: vec![PartySnapshot {
                species: species.to_string(),
                level: 10,
                hp,
                max_hp: 40,
                status: Status::None,
            }],
        }
    }

    fn snap(player_hp: u16, enemy_hp: u16) -> BattleSnapshot {
        BattleSnapshot {
            player: side("QUILAVA", player_hp, 0),
            enemy: side("PIDGEY", enemy_hp, 0),
        }
    }

    /// The produced TOML must parse, contain the resolved seed, one `[[step]]`
    /// table per recorded decision with the right `kind`, and an `expect` block
    /// per step (with the battle outcome on the last one).
    #[test]
    fn steps_and_expects_round_trip_through_toml_parser() {
        let path = temp_path("rt");
        let mut cpu = Cpu::default();
        install(&mut cpu, &path, 0xc0ffee, Some(&PathBuf::from("scratch/cli.toml")), None)
            .expect("install");

        // Decision 1: a move. Its outcome is observed at decision 2.
        record(&cpu, Step::Move { slot: 0 });
        observe(&cpu, &snap(30, 22));
        // Decision 2: another move, ending the battle. Outcome from finalize.
        record(&cpu, Step::Move { slot: 1 });
        finalize(&cpu, &snap(30, 0), Outcome::Win);
        cpu.recorder = None;

        let text = std::fs::read_to_string(&path).expect("read recording");
        let parsed: toml::Value = toml::from_str(&text).expect("parse recording");
        let _ = std::fs::remove_file(&path);

        let recording = parsed.get("recording").expect("recording table");
        assert_eq!(recording.get("seed").and_then(|v| v.as_str()), Some("0x0000000000c0ffee"));

        let steps = parsed.get("step").and_then(|v| v.as_array()).expect("step array");
        assert_eq!(steps.len(), 2);

        // Step 0: move with an expect block, no outcome.
        assert_eq!(steps[0].get("kind").and_then(|v| v.as_str()), Some("move"));
        let exp0 = steps[0].get("expect").expect("step 0 expect");
        assert_eq!(
            exp0.get("enemy").and_then(|e| e.get("active")).and_then(|a| a.get("hp")).and_then(|v| v.as_integer()),
            Some(22),
        );
        assert!(exp0.get("battle").is_none(), "non-final step must not carry an outcome");

        // Step 1: final move, enemy fainted, outcome win.
        let exp1 = steps[1].get("expect").expect("step 1 expect");
        assert_eq!(
            exp1.get("enemy").and_then(|e| e.get("active")).and_then(|a| a.get("hp")).and_then(|v| v.as_integer()),
            Some(0),
        );
        assert_eq!(
            exp1.get("battle").and_then(|b| b.get("outcome")).and_then(|v| v.as_str()),
            Some("win"),
        );
        // Party array round-trips as an array of tables.
        let party = exp1
            .get("player")
            .and_then(|p| p.get("party"))
            .and_then(|v| v.as_array())
            .expect("player party array");
        assert_eq!(party.len(), 1);
        assert_eq!(party[0].get("species").and_then(|v| v.as_str()), Some("QUILAVA"));
    }

    /// A link battle records both player and enemy decisions into one ordered
    /// list. Enemy steps carry `side = "enemy"`; player steps stay key-less so
    /// pre-link recordings round-trip unchanged.
    #[test]
    fn enemy_side_is_tagged_player_side_is_not() {
        assert_eq!(Side::Player.tag(), None);
        assert_eq!(Side::Enemy.tag(), Some("enemy"));

        let path = temp_path("side");
        let mut cpu = Cpu::default();
        install(&mut cpu, &path, 1, None, None).expect("install");
        record(&cpu, Step::Move { slot: 0 }); // player
        record_side(&cpu, Side::Enemy, Step::Move { slot: 2 }); // enemy
        record_side(&cpu, Side::Enemy, Step::Switch { slot: 1 }); // enemy switch
        cpu.recorder = None;

        let text = std::fs::read_to_string(&path).expect("read");
        let parsed: toml::Value = toml::from_str(&text).expect("parse");
        let _ = std::fs::remove_file(&path);
        let steps = parsed.get("step").and_then(|v| v.as_array()).expect("steps");
        assert_eq!(steps.len(), 3);
        // Player step: no `side` key.
        assert!(steps[0].get("side").is_none(), "player step must not be tagged");
        // Enemy steps: side = "enemy".
        assert_eq!(steps[1].get("side").and_then(|v| v.as_str()), Some("enemy"));
        assert_eq!(steps[2].get("side").and_then(|v| v.as_str()), Some("enemy"));
        assert_eq!(steps[2].get("kind").and_then(|v| v.as_str()), Some("switch"));
    }

    /// An action recorded but never observed (a crash before the next decision)
    /// must still leave a parseable `[[step]]` — just without an `expect`.
    #[test]
    fn open_step_without_expect_is_still_valid_toml() {
        let path = temp_path("open");
        let mut cpu = Cpu::default();
        install(&mut cpu, &path, 1, None, None).expect("install");
        record(&cpu, Step::Run);
        cpu.recorder = None;

        let text = std::fs::read_to_string(&path).expect("read");
        let parsed: toml::Value = toml::from_str(&text).expect("parse");
        let _ = std::fs::remove_file(&path);
        let steps = parsed.get("step").and_then(|v| v.as_array()).expect("steps");
        assert_eq!(steps.len(), 1);
        assert_eq!(steps[0].get("kind").and_then(|v| v.as_str()), Some("run"));
        assert!(steps[0].get("expect").is_none());
    }
}
