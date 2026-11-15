# Goal: phases 3 and 4

Phases 3 and 4 are done: the next step is training a battle AI on top of the
library (`battle::Battle`).

The overall plan, in five phases:

1. ✅ Run the battle engine fully emulated.
2. ✅ Record enough battles to cover the engine (349 fixtures, ~80% of battle-code regions).
3. ✅ **A library that drives just the battle logic** — `Battle::new(config)`,
   `step(action)`, typed state, legal actions, an ordered event stream.
4. ✅ **Idiomatic Rust with excellent performance** — a hand-written engine that
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

- [x] Headless speed for the oracle: no scanline rendering without a screen,
      halts fast-forward to the next GPU event, GPU updates deferred until
      observable (cycle-identical), keypad not polled without input, and the
      `battle_scene = false` option. Longest fixture: 4.7s → 1.3s; the corpus
      replays in ~1 min on 11 cores. Remaining cost is the per-instruction
      emulation itself (~250M emulated instructions for a 120-decision battle).
- [x] Fidelity: the Poké Ball wobble rolls (`GetPokeBallWobble` draws
      `Random`) were skipped with the ball animation; they now run.
- [x] Hand-written engine (`src/engine/`, guide: [engine.md](engine.md)):
      plain Rust, no `Cpu`, the original's memory layout (`Mem`, WRAM banks
      0–1 + HRAM), game data read from the ROM image, `Clone` (about 8 KB) so
      an AI can search ahead. Every routine a battle reaches is ported: battle
      start (scenario seeding, `StartBattle`, trainer parties, send-outs), the
      turn loop, all 175 effect commands and move effects, damage, switching,
      fainting, experience and level-ups, the trainer AI, bag items and
      catching, obedience, Berserk Gene, battle text (`text_asm` included) and
      the HP-bar events.
- [x] Differential tester: `engine_diff` (random link, wild and trainer
      battles: random parties and moves from all 251, held items, bags,
      traded mons, badges, random policy) compares the start state, every
      decision (site, legal actions, random-stream state, memory, text) and
      the end of the battle. `engine_fixtures` does the same for the
      recorded corpus, `api_diff` for the library API on both engines.
- [x] The library's `Battle` runs on the hand-written engine;
      `EmulatedBattle` keeps the same API on the original for testing and the
      window. (Both are compiled; there is no cargo feature, since the
      engine's tests use the emulated original.)
- [x] Benchmarks (`examples/engine_bench`, one thread, random policy, all
      moves, setup included):

      | kind | `FastBattle` (state only) | `Battle` (with events) | `EmulatedBattle` | speedup |
      | --- | ---: | ---: | ---: | ---: |
      | link | 2,707/s | 1,769/s | 17.4/s | 156× |
      | wild | 11,267/s | 6,337/s | 21.8/s | 517× |
      | trainer | 3,029/s | 2,084/s | 3.7/s | 813× |

### Fidelity fixes found by differential testing

The tester found harness gaps as well as port bugs; the harness gaps were
fixed in the harness (so the oracle got closer to the real game):

- **Rival name.** Rival battles printed an unset `wRivalName`, and the text
  engine wrote `?` bytes across WRAM (the window stack, `wPlayerSubStatus4`).
  The harness now names the rival SILVER; fixtures 00276, 00289 and 00290
  had recorded the corrupted battles and were re-recorded.
- **Move menu rows.** The harness skipped `MoveSelectionScreen`, leaving
  `w2DMenuNumRows` stale, and a disobedient traded mon's random-move pick
  could loop forever. The patch now leaves the menu's row count.
- **Battle-end hook.** The end of a battle is defined as the moment before
  `ExitBattle` (post-battle processing: link result screen, Pay Day,
  evolution, Pokérus, `CleanUpBattleRAM`). Both APIs report the outcome and
  final state there, and the tester compares it.
- **Text capture.** A full party's party-menu "CANCEL" is no longer captured
  as battle text.
- **NPC names.** The harness never ran a new game's `InitializeNPCNames`,
  leaving Mom's, Red's and Green's names zero; garbage text that printed
  `<MOM>` / `<RED>` / `<GREEN>` then ran on through WRAM. They are now set
  like a new game sets them.
- **HP bar animation.** Two of the original's `; BUG:`s had been ported as
  the fixed code: the long animation's pixel check compares the low byte of
  the HP before the step (so it redraws almost every step and leaves that
  byte in `wCurHPBarPixels`), and the short animation clamps the HP it shows
  to `wCurHPAnimLowHP`..`HighHP`. Neither reached compared memory, but the HP
  number they leave in `wStringBuffer2` is what garbage text reads after an
  unterminated name.

### Toolchain

The crate needs nightly (the transpiled code uses `become` tail calls). The
`gui` feature moved to glium 0.36 / winit 0.30: newer nightlies no longer
compile the `objc2` 0.4 that glium 0.34 pulled in.

### Known gaps

- **Beat Up's walk past the party** is pinned, not original. A Beat Up that
  misses leaves its loop flag set; the next one underflows the loop counter
  and walks the party index through all 256 values: past the party, through
  echo RAM, OAM and I/O, wrapping around to ROM (about 1 in 2,500 random
  battles). Where a read lands on WRAM both engines keep, or on ROM, it is
  exact. Where it lands on state neither keeps the way real Crystal does (the
  live stack, sound, graphics and menu scratch, OAM, I/O: see
  `engine::mem::NOT_KEPT` / `DRIFTS`), the transpiled original holds an
  artifact of itself (what the transpiled code pushed, the emulator's
  timing), not real hardware state. Both engines read zeros there instead (a
  HARNESS-PATCH in `GetBeatupMonLocation` and Beat Up's nickname lookup,
  [transpiled_code.md](transpiled_code.md)).
- **`<MOBILE>` in garbage text** (`RunMobileScript`, reachable through
  an enemy Beat Up's names past the party in link battles) is ported where
  it stays in the memory the engine models; its SRAM commands, the command
  that leaves WRAM bank 4 mapped, and garbage reaching the tilemap, sound,
  I/O or a ROM bank switch stop the engine (`Error::Engine`). `text_diff`:
  12,000 random garbage names, zero differences; the rest stop on purpose
  or are ones the original can't print either.
- **Scenarios the original itself freezes on** (a wild Unown with no unlocked
  letters) are skipped by the tester's cycle watchdog.
- **Not offered by the harness, so not ported:** Bug-Catching Contest and
  tutorial battles, Battle Tower, mobile link battles, PP Up / key items /
  TMs in battle, a full party *and* full PC box when catching (the box lives
  in SRAM).

## Definition of done

- `Battle` API documented, with a runnable example of a random-policy agent.
- Hand-written engine matches the oracle on every fixture and on at least one
  million differential battles with zero divergences.
- CLI can show a battle live from events; the `gui` window still works.
- Docs (`README`, `roadmap`, this page) describe the finished state.
