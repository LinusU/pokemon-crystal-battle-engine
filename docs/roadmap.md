# Roadmap

End-goal: a **headless Pokémon Crystal battle engine library** that exposes `BattleState` and `step(action) -> Outcome`. The emulator window (`main.rs` + glium/winit, now behind the off-by-default `gui` cargo feature) is a stepping stone for verification and demos; it isn't the long-term API surface.

## Step 1 — done

Load a TOML scenario, jump into a wild battle, play it in the window.

## Step 2 — make the harness usable

- ✅ Driver for trainer battles (`type = "trainer_battle"`, `trainer = "..."`). Today the engine's `InitEnemyTrainer` loads the canonical party and held items from ROM via `wOtherTrainerClass` / `wOtherTrainerID`.
- ✅ Link battles (`type = "link_battle"`). Both parties are seeded from the TOML (species, level, moves) and both sides are CLI-driven — the engine's real `wLinkMode` path with the AI and ROM party-load skipped, and the enemy action sourced from the CLI via the `link_battle_send_receive_action` hook. See [bootstrap.md](bootstrap.md#link-battles) and [toml_schema.md](toml_schema.md#link-battles). (This also demonstrates the "seed `wOTParty*` instead of loading from ROM" mechanism the trainer-battle enemy-override below still wants.)
- ✅ TOML-driven overrides for the enemy party (`[[enemy_party]]`), trainer AI items (`[[enemy_items]]`), and AI flags (`[enemy_trainer]`). The party override seeds `wOTParty*` and early-returns out of `ReadTrainerParty` via a HARNESS-PATCH; item/flag overrides poke the `TrainerClassAttributes` ROM entry via `Cpu::poke_rom` so `GetTrainerAttributes` and the AI flag readers see the overridden data. All three round-trip through `--record`/`--play`.
- Player overrides via TOML: name, id, gender, money, badges (for stat boosts).
- Per-mon overrides: nickname, DVs. (Player-party `moves` and `item` are done — `[[player_party]] moves = [...]` / `item = "..."` overwrite the generated moveset / `MON_ITEM` byte after `try_add_mon_to_party`; `item` also works on the link-battle enemy party. See [TOML schema](toml_schema.md#player_party--one-or-more).)
- A `[battle]` `seed` field for `wBattleRandom` reproducibility.

## Step 2.5 — make runs reproducible (prerequisite for tests)

Testing is blocked on two things:

1. **Seedable, deterministic RNG.** Crystal's `Random` / `_battle_random` (in `src/game/home/random.rs`) currently derive entropy from `rDIV` and `hRandomAdd` / `hRandomSub`. In theory that's deterministic given a fixed input trace, but cycle counts will drift as we add hooks, so we don't want to rely on it. Plan: replace the two functions' bodies with a tiny LCG step that reads/writes a new `cpu.rng_state` field. Two `HARNESS-PATCH:` markers, ~5 lines each. Seed from `BattleConfig` (optional `seed = N` in TOML).
2. **Programmatic action input.** Right now the only way to choose a move is via the keypad channel. Plan: a `HARNESS-PATCH:` at the top of `BattleMenu` that, if `cpu.scripted_action` is set, writes `wBattlePlayerAction` / `wCurMoveNum` / `wCurPartyMon` directly and skips the menu loop. Same shape as the `skip_intro_to_battle` hook.

These are deliberately small patches, not graduations to hand-written. See [transpiled_code.md](transpiled_code.md).

## Step 3 — first cut at the state machine

- Extract a `BattleState` snapshot from `Cpu` (the relevant WRAM regions + a few hRAM bytes).
- Define `Action` = `{ Move(slot), Switch(slot), Item(id), Run }`.
- Define `Outcome` = `{ NextTurn(BattleState), AwaitingPlayer { prompts }, Ended { winner, ... } }`.
- Implement `step` by setting `wBattlePlayerAction` / `wCurMoveNum` / etc. and driving the relevant engine sub-routine for one "tick" of `do_battle`. Open question: where the natural reentry points are. Likely candidates are between `BattleTurn` iterations.
- Define an ordered `BattleEvent` stream as part of the API (see [Step 4](#step-4--separate-battle-logic-from-rendering)). Emit events from small hooks around the transpiled presentation calls, the same shape as existing `HARNESS-PATCH` hooks.

## Step 4 — separate battle logic from rendering

Goal: be able to watch a battle live without the battle logic depending on any renderer. We separate logic from rendering rather than ripping rendering out, because the separation boundary *is* the library API — Rustic Crystal and Rustic Yellow both need to draw a battle the library drives, so this is the integration work for both, not a side project.

**Shape.** The logic emits an ordered stream of presentation events; renderers are consumers:

```rust
enum BattleEvent {
    Text(BattleText),            // "Foe GENGAR used HYPNOSIS!"
    MoveAnimation { side, move_id },
    HpChanged { side, from, to },
    StatusChanged { side, status },
    StatChanged { side, stat, stages },
    Fainted { side },
    SwitchedIn { side, slot, species },
    Sfx(SfxId),
    AwaitingInput(Prompt),       // the existing CLI chokepoints
}
```

Consumers:

- **Training / headless:** ignore the stream. With a generic no-op sink, monomorphization removes it entirely.
- **CLI text view:** prints the event stream as a readable battle log, so a battle can be watched with no GPU at all.
- **Graphical renderer** (behind the `gui` feature): draws the battle screen from events.
- **Rustic Crystal / Rustic Yellow:** each brings its own renderer. Yellow lifts the Gen 2 engine into Gen 1 with Gen 1 presentation, which is exactly why the logic must not know how it is drawn.
- Because recordings replay deterministically, any recorded battle (including ML self-play) can be watched after the fact.

**Why it can't be a clean cut today.** In the transpiled code presentation is interleaved with logic — `core.rs` alone has ~52 `std_battle_textbox`, ~28 tilemap loads, ~26 `delay_frames`, ~12 `wait_bg_map`, and ~10 `play_sfx` calls next to the damage and turn-order logic. The split therefore happens incrementally, alongside graduating routines to hand-written code.

**Order of work:**

1. ✅ glium/winit are gated behind the `gui` feature: a default `cargo build` compiles neither, and the headless binary runs a battle without the screen/keypad channels (`Cpu::new_cgb(None, None)`).
2. Gate the pixel output of the in-`Cpu` GPU emulation (`runtime/gpu.rs`) behind `gui` too. The transpiled code keeps writing tilemaps/VRAM (cheap memory writes), but headless builds stop producing pixels nobody reads. The window keeps working unchanged and stays the visual reference.
3. ✅ With Step 3: emit `BattleEvent`s from hooks around the presentation calls (battle textbox, HP-bar updates) and build the CLI text view on them (`--show-text`, `examples/watch`). Today's `battle::Event` is `Text` and `HpBar`; animations, sfx and the finer-grained events above are still to add.
4. ✅ The logic has graduated: the hand-written engine (`crates/crystal-battle/src/engine/`, [engine.md](engine.md)) contains no rendering at all and is what `battle::Battle` runs on. It produces the same events (exact text, HP-bar changes) as the hooks on the original. As logic graduates to hand-written code, presentation calls leave the logic and only event emissions remain. The renderer can keep reusing the *transpiled presentation routines* (textbox, battle anims, HUD), now driven by events instead of being called from inside the logic — likely the same shape Rustic Crystal ends up with.
5. Once the event-driven renderer matches the emulated GPU visually, the emulated GPU can be dropped from the battle path.

**Invariant: rendering never influences logic.** Events describe what happened; they never feed back. In real Crystal, frame timing can leak into randomness (`rDIV`, `hRandomAdd`), so removing `delay_frames` and similar calls must not change outcomes. The fixture corpus enforces this — any rendering change that moves a recording's per-step expectations is a bug.

## Step 5 — a library for Rustic Crystal / Rustic Yellow

- ✅ Crate split: `crystal-battle` (`crates/crystal-battle/`: engine, `Battle`, config, data, AI; `serde` only, stable Rust) and `crystal-battle-harness` (the repository root: transpiled original, emulator, CLI, recorder, `EmulatedBattle`, testers, `gui`). The library never runs the emulator: a scenario the engine can't start is an error, not a fallback to the oracle.
- ✅ Game data without the ROM: typed `GameData` generated by `tools/gen_game_data.py` ([library.md](library.md)); the library builds without `pokecrystal.gbc`. (Rustic Crystal ships the 1.1 ROM; the generator can read it with its own .sym.)
- ✅ Crystal Legacy: its data, `GameData::crystal_legacy()` (feature `crystal-legacy`), and its battle code's changes as `Rules` ([library.md](library.md#crystal-legacy)). Still to do: its trainer party format, hard mode.
- ✅ Full party input: `BattleSetup` (every party field, nicknames and OT names, the player, the bag, options, the opponent, world state).
- ✅ A typed post-battle result: `BattleResult`.
- ✅ Events for a renderer: typed messages, animations, sound effects, cries, faints, slides, the experience bar, level-up stats, ball wobbles, each verified against the original.
- Battle types the harness never offered: the Bug-Catching Contest, the tutorial battle, the Battle Tower; catching with a full party and a full box.
- Rustic Yellow: mapping Gen 1 party data (internal species indices, item ids, no held items) to Crystal's and back.

## Open design questions

- Do we keep transpiled engine code as the canonical implementation forever, or do we hand-rewrite parts that don't fit the state-machine model (e.g. the giant `do_battle` loop)? Hand-rewriting risks divergence; staying transpiled risks awkward control flow. Probably we live with transpiled for a long time.
- How do we want to handle RNG determinism? Crystal's `BattleRandom` reads from hRAM; we need to either intercept it or seed it explicitly.
- How do we want to deal with audio? `init_sound` runs during boot and the engine queues sfx; we're ignoring it but it's not free.
- Long-term, do we want save/load of `BattleState`, or is a turn-by-turn replay enough?
