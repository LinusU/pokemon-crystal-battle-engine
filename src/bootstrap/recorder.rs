//! Record every player decision the CLI driver in [`super::cli`] resolves.
//!
//! The harness opens a recording file once `bootstrap::run` has resolved the
//! seed, writes a `[recording]` header that always includes the *resolved*
//! seed (whether the scenario supplied one or it was drawn from system
//! entropy), then hands a writer to the [`Cpu`] in `cpu.recorder`. Each CLI
//! prompt site calls one of the `record_*` helpers below after the prompt
//! returns, which appends a single `[[step]]` table and flushes — so a
//! mid-battle panic still leaves a usable script on disk.
//!
//! The schema is intentionally tiny: a `kind` tag plus the few fields the
//! action carries. Playback (a separate change) reads `[[step]]` entries
//! back and feeds them into the same WRAM writes that the live CLI does.
//!
//! No TOML serializer is pulled in here — the lines are short and fixed,
//! and writing them by hand keeps the on-disk output deterministic and easy
//! to diff. The companion test parses the produced text with `toml` to make
//! sure we didn't drift.
//!
//! See [docs/toml_schema.md](../../docs/toml_schema.md) for the human-facing
//! description of the `--record` flag and recording format.

use std::cell::RefCell;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use crate::runtime::Cpu;

/// One captured CLI decision. Each variant maps 1:1 to one of the four
/// prompt sites in [`super::cli`].
#[derive(Debug, Clone, Copy)]
pub enum Step {
    /// Action menu picked a move. `slot` is the 0-based move slot.
    Move { slot: u8 },
    /// Action menu picked a switch. `slot` is the 0-based party slot of
    /// the incoming mon.
    Switch { slot: u8 },
    /// Action menu picked an item. `target` is `Some(slot)` for items that
    /// target a specific party mon and `None` otherwise.
    Item { id: u8, target: Option<u8> },
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

/// Open `path` for write (truncating any existing file), write the
/// `[recording]` header, and install the resulting writer on `cpu.recorder`.
/// The seed is always written even when the source scenario omitted one —
/// it's the resolved value used to seed the RNG so a replay is bit-identical.
pub fn install(
    cpu: &mut Cpu,
    path: &Path,
    seed: u64,
    source_scenario: Option<&Path>,
) -> Result<(), String> {
    let file = File::create(path)
        .map_err(|e| format!("could not open recording file {}: {}", path.display(), e))?;
    let mut writer = BufWriter::new(file);
    write_header(&mut writer, seed, source_scenario)
        .map_err(|e| format!("could not write recording header: {e}"))?;
    writer
        .flush()
        .map_err(|e| format!("could not flush recording header: {e}"))?;
    cpu.recorder = Some(RefCell::new(writer));
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

/// Append a `[[step]]` entry and flush. Silently no-ops if the cpu has no
/// recorder installed — callers always invoke this; the runtime flag is
/// what gates recording.
pub fn record(cpu: &Cpu, step: Step) {
    let Some(cell) = cpu.recorder.as_ref() else { return };
    let mut writer = cell.borrow_mut();
    if let Err(e) = write_step(&mut writer, step) {
        // Don't bubble — we're mid-battle and the recording is best-effort.
        // The error is logged once so an unwriteable file doesn't go silent.
        eprintln!("recorder: failed to write step: {e}");
        return;
    }
    if let Err(e) = writer.flush() {
        eprintln!("recorder: failed to flush step: {e}");
    }
}

fn write_step(writer: &mut BufWriter<File>, step: Step) -> std::io::Result<()> {
    writeln!(writer)?;
    writeln!(writer, "[[step]]")?;
    match step {
        Step::Move { slot } => {
            writeln!(writer, "kind = \"move\"")?;
            writeln!(writer, "slot = {slot}")?;
        }
        Step::Switch { slot } => {
            writeln!(writer, "kind = \"switch\"")?;
            writeln!(writer, "slot = {slot}")?;
        }
        Step::Item { id, target } => {
            writeln!(writer, "kind = \"item\"")?;
            writeln!(writer, "id = {id}")?;
            if let Some(t) = target {
                writeln!(writer, "target = {t}")?;
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

/// Quote a string as a TOML basic string. The recorded values are paths and
/// have no backslash / control-char surprises, so the only escape we need is
/// `"`.
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
    use std::path::PathBuf;

    /// The produced TOML must parse, contain the resolved seed under
    /// `[recording]`, and one `[[step]]` table per recorded decision with
    /// the right `kind` discriminator.
    #[test]
    fn header_and_steps_round_trip_through_toml_parser() {
        let dir = std::env::temp_dir();
        let path = dir.join(format!(
            "battle-recording-test-{}-{}.toml",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
        ));
        let mut cpu = Cpu::default();
        install(&mut cpu, &path, 0xc0ffee, Some(&PathBuf::from("scratch/cli.toml")))
            .expect("install");
        record(&cpu, Step::Move { slot: 0 });
        record(&cpu, Step::Switch { slot: 2 });
        record(&cpu, Step::Item { id: 0x12, target: Some(1) });
        record(&cpu, Step::Item { id: 0x04, target: None });
        record(&cpu, Step::Run);
        record(&cpu, Step::ForcedSwitch { slot: 3 });
        record(&cpu, Step::OfferSwitch { keep: true, slot: None });
        record(&cpu, Step::OfferSwitch { keep: false, slot: Some(4) });
        // Drop the writer so the BufWriter flushes on close.
        cpu.recorder = None;

        let text = std::fs::read_to_string(&path).expect("read recording");
        let parsed: toml::Value = toml::from_str(&text).expect("parse recording");
        let _ = std::fs::remove_file(&path);

        let recording = parsed.get("recording").expect("recording table");
        assert_eq!(recording.get("seed").and_then(|v| v.as_str()), Some("0x0000000000c0ffee"));
        assert_eq!(
            recording.get("source_scenario").and_then(|v| v.as_str()),
            Some("scratch/cli.toml"),
        );

        let steps = parsed.get("step").and_then(|v| v.as_array()).expect("step array");
        assert_eq!(steps.len(), 8);
        assert_eq!(steps[0].get("kind").and_then(|v| v.as_str()), Some("move"));
        assert_eq!(steps[0].get("slot").and_then(|v| v.as_integer()), Some(0));
        assert_eq!(steps[1].get("kind").and_then(|v| v.as_str()), Some("switch"));
        assert_eq!(steps[2].get("kind").and_then(|v| v.as_str()), Some("item"));
        assert_eq!(steps[2].get("id").and_then(|v| v.as_integer()), Some(0x12));
        assert_eq!(steps[2].get("target").and_then(|v| v.as_integer()), Some(1));
        assert_eq!(steps[3].get("target"), None, "no-target items must omit the field");
        assert_eq!(steps[4].get("kind").and_then(|v| v.as_str()), Some("run"));
        assert_eq!(steps[5].get("kind").and_then(|v| v.as_str()), Some("forced_switch"));
        assert_eq!(steps[6].get("keep").and_then(|v| v.as_bool()), Some(true));
        assert_eq!(steps[6].get("slot"), None);
        assert_eq!(steps[7].get("keep"), None);
        assert_eq!(steps[7].get("slot").and_then(|v| v.as_integer()), Some(4));
    }
}
