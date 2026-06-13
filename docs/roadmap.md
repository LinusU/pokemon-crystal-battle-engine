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

## Step 4 — drop the GPU

- ✅ glium/winit are gated behind the `gui` feature: a default `cargo build` compiles neither, and the headless binary runs a battle without the screen/keypad channels (`Cpu::new_cgb(None, None)`).
- Still open: the in-`Cpu` GPU emulation (`runtime/gpu.rs`) keeps rendering pixels nobody reads in a headless run. Once `step` exposes state directly, that pixel path becomes dead weight and can be gated/removed too.
- The `gui` feature gives a glium/winit-free default build without a crate split, so splitting into `pokemon_crystal_battle` (library) + `pokemon_crystal_battle_play` (window harness) is now optional rather than required.

## Open design questions

- Do we keep transpiled engine code as the canonical implementation forever, or do we hand-rewrite parts that don't fit the state-machine model (e.g. the giant `do_battle` loop)? Hand-rewriting risks divergence; staying transpiled risks awkward control flow. Probably we live with transpiled for a long time.
- How do we want to handle RNG determinism? Crystal's `BattleRandom` reads from hRAM; we need to either intercept it or seed it explicitly.
- How do we want to deal with audio? `init_sound` runs during boot and the engine queues sfx; we're ignoring it but it's not free.
- Long-term, do we want save/load of `BattleState`, or is a turn-by-turn replay enough?
