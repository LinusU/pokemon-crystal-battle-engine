# Goal: phases 3 and 4

The current working goal. When everything here is ✅, the next step is to start
training a battle AI on top of the library.

The overall plan, in five phases:

1. ✅ Run the battle engine fully emulated.
2. ✅ Record enough battles to cover the engine (349 fixtures, ~80% of battle-code regions).
3. ⬜ **A library that drives just the battle logic** — `Battle::new(config)`,
   `step(action)`, typed state, legal actions, an ordered event stream.
4. ⬜ **Idiomatic Rust with excellent performance** — a hand-written engine that
   reproduces the original exactly, with logic separated from rendering.
5. Machine-learning trainer AI (out of scope here; this goal unblocks it).

## The exactness contract

"Exactly like the original" is defined as:

> Given the same scenario, the same decisions, and the same stream of random
> bytes, the engine produces the same sequence of battle states, the same
> battle text, and the same outcome as the original Crystal ROM code.

- **The reference is the original code, not the harness.** The transpiled engine
  in `src/game/` is the oracle because it *is* the ROM's code. Anything the
  harness hand-rolls instead of running the original routine is a fidelity gap
  (see below) and gets replaced with the original routine.
- **Randomness.** The original `Random` mixes `rDIV` (a hardware timer) into
  `hRandomAdd`/`hRandomSub`, so in the real game the random stream depends on
  frame timing and button-press timing — it is not reproducible even on real
  hardware. We keep a seeded source but preserve **every call site and every
  byte consumed** in the original order. The new engine must draw the same
  random bytes at the same points as the oracle; the differential tester checks
  the draw sequence, not only the end state.
- **Rendering never influences logic.** Events report what happened; nothing a
  renderer does feeds back into the battle.

## Verification

1. **Fixture corpus** (349 recordings, `scratch/validate_fixtures.py`, ~1.5 min in
   parallel with the release build). Must stay green throughout.
2. **Differential testing** (phase 4): the hand-written engine and the transpiled
   oracle run the same random battles in lockstep. At every decision point the
   battle state, the random-draw count, and the battle text so far must match.
   This covers far more than the fixtures — millions of battles, not hundreds.

## Fidelity gaps found in the harness (to fix in phase 3)

- **Hand-rolled bag items.** `bootstrap::cli::apply_item` applies Potions, status
  heals, revives and Ethers/Elixers with hand-written WRAM edits instead of the
  game's item routines (`engine/items/item_effects.asm`). It misses side effects
  the original has (for example the battle-mon/party-mon sync done by the real
  heal routines, stat recalculation after curing a status, and the
  original's exact text). Only the X items, Dire Hit, Guard Spec, Bitter Berry
  and Poké Balls currently run their real routines.
- **Hand-rolled switch.** The action-menu switch replicates only the WRAM writes
  of `TryPlayerSwitch`. To be checked line by line against the ASM; switch is
  logic-relevant (it decides Pursuit, trapping, etc.).
- **`Random` flags.** The RNG patch always clears carry on return; the original
  leaves carry from the final `sbc`. To be checked whether any caller reads it.

## Phase 3 — library API (on top of the transpiled engine)

- [ ] Split `bootstrap::cli` into an engine-facing layer (prompt kinds, legal
      options, applying an action) and front-ends (stdin, playback, library).
- [ ] `Battle` API: `Battle::new(&BattleConfig, seed)`, `prompt()`,
      `legal_actions()`, `step(Action)`, `state()`. The engine runs on its own
      thread and blocks at decision points; this keeps the original control
      flow untouched.
- [ ] Typed `BattleState` (both sides: active mon, party, stat stages, statuses,
      field effects) — what an AI needs to observe.
- [ ] `BattleEvent` stream from hooks: battle text (exact strings, from the text
      engine), move animations, HP changes, faint, switch-in.
- [ ] CLI text view that renders events, so a battle can be watched without a GPU.
- [ ] Replace the hand-rolled item and switch paths with the original routines.
- [ ] Fixture corpus green after every change; recordings that depended on a
      hand-rolled behaviour are re-recorded and the change is called out.

## Phase 4 — idiomatic, fast, logic separated from rendering

- [ ] Gate the emulated GPU's pixel output behind `gui`; measure battles/second.
- [ ] Hand-written engine (`src/engine/`): pure Rust, typed state, no `Cpu`,
      `Clone`-able (so an AI can search ahead), emits `BattleEvent`s. Game data
      (base stats, moves, type chart, trainer parties, item attributes, AI data)
      is read from the ROM image, so tables are exact by construction.
- [ ] Differential tester: lockstep comparison against the oracle over random
      battles (all battle kinds, all trainers, random parties, random policies).
- [ ] The library's `Battle` runs on the hand-written engine by default; the
      oracle stays available behind a feature for testing and for the window.
- [ ] Benchmarks: battles/second for the oracle vs the new engine.

## Definition of done

- `Battle` API documented, with a runnable example of a random-policy agent.
- Hand-written engine matches the oracle on every fixture and on at least one
  million differential battles with zero divergences.
- CLI can show a battle live from events; the `gui` window still works.
- Docs (`README`, `roadmap`, this page) describe the finished state.
