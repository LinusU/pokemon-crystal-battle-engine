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

- ✅ **Hand-rolled bag items.** `bootstrap::cli::apply_item` used to apply
  Potions, status heals, revives and Ethers/Elixers with hand-written WRAM edits.
  It missed the original's side effects — for example Full Restore and Full Heal
  also cure confusion, and curing a status on the active mon clears Toxic and
  Nightmare and recalculates its stats. Items now run the real `BattleMenu_Pack`
  → `ItemSubmenu.Use` → effect routine; only the bag, party-menu and move-menu
  *selections* are hooked. Fixture 00030 had recorded the wrong behaviour (a
  confused Cubone kept hitting itself after a Full Restore) and was re-recorded.
- ✅ **Illegal actions accepted.** The harness let a Revive be used on a healthy
  mon (the original says "It won't have any effect" and returns to the menu);
  five fixtures (00002, 00003, 00021, 00028, 00036) had recorded that and were
  re-recorded. Every decision point now has a legal-step list mirroring the
  original menus, and playback rejects anything outside it.
- ✅ **No Struggle.** With no usable move the harness offered no FIGHT option at
  all (a one-mon trainer battle would loop on Run forever). FIGHT now exists as
  the `fight` step whenever the original shows no move menu (Encore, Bide,
  Struggle).
- ✅ **Legal item set.** X items are legal even at +6 (the original still uses
  them up), balls are legal in trainer battles (the original tosses the ball
  and spends the turn), heal-everything items are legal on a confused active
  mon, and no item is legal in a link battle.
- ✅ **Hand-rolled switch (checked).** The action-menu switch writes what
  `TryPlayerSwitch` writes (`wLastPlayerMon`, `wBattlePlayerAction`,
  `wCurPartyMon`, `wCurBattleMon`) and tail-calls the real `PlayerSwitch`. The
  only original writes it skips are UI (party-menu cursor, `wCurPartySpecies`,
  which the switch-in uses solely for the cry); its trap checks are
  `TryPlayerSwitch`'s.
- ✅ **`Random` flags (checked).** The RNG patch returns carry clear where the
  original returns the carry of its last `sbc`. None of the 229 `Random` /
  `BattleRandom` call sites reads carry before overwriting it, and z (result is
  zero) matches.

## Phase 3 — library API (on top of the transpiled engine)

- [x] Split `bootstrap::cli` into an engine-facing layer (legal steps per
      decision point) and decision sources (playback, `Decider`, stdin).
- [x] `Battle` API: `Battle::new(BattleConfig, seed)`, `status()`,
      `decision()`, `step(Action)`, `state()`, `outcome()`. The engine runs on
      its own thread and blocks at decision points; this keeps the original
      control flow untouched. All 349 fixtures replay through it.
- [x] Typed `BattleState` (both sides: active mon, party, stat stages,
      sub-statuses, every per-side counter, screens, weather).
- [x] `Event` stream from observe-only hooks: battle text (exact strings, from
      the text engine) and HP-bar changes. Faints, switch-ins and move use are
      in the text ("… fainted!", "Go! …!", "… used …!") and the state.
- [x] CLI text view that renders events (`--show-text`, `examples/watch`), so
      a battle can be watched without a GPU.
- [x] Replace the hand-rolled item path with the original routines (the switch
      path was checked and is equivalent).
- [x] Fixture corpus green after every change; recordings that depended on a
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
