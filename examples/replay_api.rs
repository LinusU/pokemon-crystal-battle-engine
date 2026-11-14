//! Replay fixture recordings through the library API (`battle::Battle`) and
//! check each ends with the recorded outcome after consuming exactly the
//! recorded steps.
//!
//! cargo run --release --example replay_api -- fixtures/*.toml

use rgb2rust_output::battle::{Action, Battle, Status};
use rgb2rust_output::bootstrap::player;

fn replay(path: &std::path::Path) -> Result<(), String> {
    let rec = player::load(path)?;
    let scenario = rec.scenario.ok_or("recording has no embedded scenario")?;
    let expected_outcome = rec
        .steps
        .last()
        .and_then(|s| s.expect.battle.as_ref())
        .and_then(|b| b.outcome);
    let mut battle = Battle::new(scenario, rec.seed).map_err(|e| e.to_string())?;
    for (i, play_step) in rec.steps.iter().enumerate() {
        let Status::Decision(decision) = battle.status() else {
            return Err(format!("battle ended before step #{}", i + 1));
        };
        let mut action = play_step.step;
        // Pre-`fight` recordings record FIGHT-without-a-menu as a move.
        if matches!(action, Action::Move { .. }) && decision.legal.contains(&Action::Fight) {
            action = Action::Fight;
        }
        battle.step(action).map_err(|e| format!("step #{}: {e}", i + 1))?;
    }
    match battle.status() {
        Status::Ended { outcome, .. } => match expected_outcome {
            Some(want) if want != outcome => Err(format!("outcome {outcome:?}, recorded {want:?}")),
            _ => Ok(()),
        },
        Status::Decision(d) => Err(format!("battle still running after the last step ({:?})", d.site)),
    }
}

fn main() {
    let paths: Vec<_> = std::env::args().skip(1).map(std::path::PathBuf::from).collect();
    let started = std::time::Instant::now();
    let results: Vec<_> = std::thread::scope(|s| {
        let chunks: Vec<_> = paths.chunks(paths.len().div_ceil(std::thread::available_parallelism().map_or(1, |n| n.get())).max(1)).collect();
        let handles: Vec<_> = chunks
            .into_iter()
            .map(|chunk| s.spawn(move || chunk.iter().map(|p| (p.clone(), replay(p))).collect::<Vec<_>>()))
            .collect();
        handles.into_iter().flat_map(|h| h.join().unwrap()).collect()
    });
    let mut failed = 0;
    for (path, r) in &results {
        if let Err(e) = r {
            failed += 1;
            println!("FAIL {}: {e}", path.display());
        }
    }
    println!(
        "{} passed, {failed} failed in {:.1}s",
        results.len() - failed,
        started.elapsed().as_secs_f64()
    );
    std::process::exit(if failed == 0 { 0 } else { 1 });
}
