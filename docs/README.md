# pokemon-crystal-battle-engine — living docs

A **headless Pokémon Crystal battle engine**: the battle as a state machine — feed actions in, get the next state out, repeat until the battle ends. It started as a Rust port of the [pret/pokecrystal](https://github.com/pret/pokecrystal) disassembly, mechanically produced by a custom transpiler; the battle has since been ported by hand and verified against that transpiled original.

The repository is a Cargo workspace of two crates:

- **`crystal-battle`** ([`crates/crystal-battle/`](../crates/crystal-battle/)) — the library: the hand-written engine, the `Battle` API, the scenario types (`config`), the game-data tables (`data`), and the trained AI (`ai` feature). It depends on `serde` alone and builds on stable Rust. This is what Rustic Crystal / Rustic Yellow depend on.
- **`crystal-battle-harness`** (the repository root) — the reference the library is verified against: the transpiled original (`src/game/`, `src/dispatch.rs`) on an emulated Game Boy (`src/runtime/`), the battle-launch harness and CLI (`src/bootstrap/`), recording / playback, `EmulatedBattle`, the differential testers (`src/oracle/`, `examples/`), the fixture corpus and the `gui` window. It needs nightly (the transpiled code uses `become`).

We avoid editing the transpiled code where we can — when we have to (e.g. boot-flow hooks), we patch a single line and document why in [transpiled_code.md](transpiled_code.md). The generated constants it shares with the engine (`crates/crystal-battle/src/defs.rs`) live in the library.

## Where we are

- ✅ The transpiled engine boots end-to-end — the battle harness runs the real boot path (hardware init, title/intro skipped) before jumping straight into the battle.
- ✅ TOML-driven launch: `cargo run -- scratch/first.toml` skips the title/intro and starts a battle.
- ✅ Wild encounters supported with `[battle] type = "wild_encounter"`, one player party, one enemy mon.
- ✅ Bag items via `[[player_items]]` — sorted into the engine's Items / Balls pockets.
- ✅ Every battle-usable item works from the action menu: HP/status/PP heals, revives, X-items, Dire Hit, Guard Spec, confusion cure. In wild battles you can also throw a Poké Ball to **catch** the wild mon (the caught mon joins the party, `outcome = "caught"`) or use a Poké Doll to escape (`outcome = "draw"`).
- ✅ Trainer battles via `[battle] type = "trainer_battle"`, `trainer = "Whitney"` — the engine loads the canonical party and AI items from ROM.
- ✅ Link battles via `[battle] type = "link_battle"` — both parties are fully specified (species, level, moves) and **both sides are CLI-driven** (no AI). The harness seeds `wParty*` and `wOTParty*`, sets `wLinkMode` so the engine takes its real link path, and feeds the enemy's per-round action through the `link_battle_send_receive_action` hook. Each round prompts the player then the enemy; recording tags enemy decisions `side = "enemy"`. See [TOML schema](toml_schema.md#link-battles).
- ✅ Per-mon overrides: player-party `moves = [...]` and `item = "..."` (held item) are supported, and `item` also works on the link-battle enemy party so both sides can hold items (see [TOML schema](toml_schema.md#player_party--one-or-more)); DVs and nicknames are still on the [roadmap](roadmap.md).
- ✅ Trainer-battle enemy overrides via `[[enemy_party]]`, `[[enemy_items]]`, and `[enemy_trainer]` — the trainer's canonical party, AI item slots, and AI flags can all be replaced from the TOML. Recordings embed the overrides and `--play` reproduces them exactly. See [TOML schema](toml_schema.md#enemy_party).
- ✅ CLI-driven actions — every in-battle menu (action / move / forced switch / "switch out enemy mon" prompt) is a numbered list on stdin. See [`src/bootstrap/cli.rs`](../src/bootstrap/cli.rs).
- ✅ Recording with `--record <out.toml>` — every CLI decision (plus the resolved seed) is appended as a `[[step]]` block to a TOML file for later replay. The resolved matchup is embedded as a `[scenario.*]` block, so a recording is self-contained for every battle kind. See [TOML schema](toml_schema.md#recording----record-path).
- ✅ Playback with `--play <recording.toml>` — drives the battle from a recorded `[[step]]` list with no stdin prompts; the recording's seed is authoritative and its embedded `[scenario.*]` block reconstructs the matchup, so no other file is needed. See [TOML schema](toml_schema.md#playback----play-path).
- ✅ Random battles with `--random` (optionally `--seed <n>`) — skip authoring TOML and start a deterministic randomized trainer battle; the recording embeds the resolved matchup and replays from the seed, so the generator is never re-run. See [TOML schema](toml_schema.md#random-battles----random-and---seed-n).
- ✅ Headless by default — glium/winit sit behind the off-by-default `gui` cargo feature, so a plain `cargo run` / `cargo build` compiles no graphics deps and drives the battle straight to completion with no window. `--features gui` adds an opt-in window that shows the same battle. Either way a battle must be specified (`<scenario.toml>` / `--random` / `--play`); the binary always runs a single battle and never boots into the overworld.
- ✅ Battle events: typed messages (`battle::Message`: one per battle text, with the mons, moves, items and numbers it mentions; the library never renders text) HP-bar changes, and everything else the screen shows (battle animations, sound effects, cries, faints, pictures sliding, the experience bar, the level-up stats box, ball wobbles), as `battle::Event`s on every `Decision` / `Battle::events()`, each where the original plays it. The harness renders messages to the original's exact English (`oracle::render`) for the differential testers. `--show-text` prints them live in the CLI, so any battle or `--play` replay can be watched in the terminal; `cargo run --release -p crystal-battle --example watch` does the same through the library.
- ✅ External enemy trainer: `[battle] enemy_control = "external"` hands the trainer AI's choices (its move, switching, its AI items, the next mon to send in) to `Enemy`-side decisions, on both engines (`engine_diff external`). See [TOML schema](toml_schema.md#external-enemy-trainer).
- ✅ A trained battle AI (`ai` cargo feature): a policy/value network trained by PPO self-play on the hand-written engine, playing either side of any decision — moves, switches, bag and trainer items. `--vs-ai` makes it the enemy trainer in the CLI (and the `gui` window). See [ai.md](ai.md).
- ✅ Library API: [`battle::Battle`](../crates/crystal-battle/src/battle/mod.rs) — `Battle::new(config, seed)`, `status()` (a `Decision` with the legal actions, a typed [`BattleState`](../crates/crystal-battle/src/battle/state.rs) and the events since the last decision, or the outcome), `step(action)`, and `Clone` (copy a battle to search ahead). It runs on the **hand-written engine** ([`crates/crystal-battle/src/engine/`](../crates/crystal-battle/src/engine/), [engine.md](engine.md)): plain Rust, no emulator, verified to reproduce the original exactly (states, random draws, text, outcome). The harness's [`battle::EmulatedBattle`](../src/battle/emulated.rs) has the same API on the original code (the transpiled engine on its own thread), the reference it is tested against. Legal actions mirror the original menus exactly (see [TOML schema](toml_schema.md#legal-steps)). Library examples: `game` (a battle driven the way a game drives it: a `BattleSetup` in, events out, the `BattleResult` back), `random_agent`, `watch` (`-p crystal-battle`); harness examples: `replay_api -- fixtures/*.toml`, `api_diff`, `engine_bench`.

## Layout

| Path | What's there |
| --- | --- |
| [`crates/crystal-battle/src/engine/`](../crates/crystal-battle/src/engine/) | The hand-written battle engine. See [engine.md](engine.md). |
| [`crates/crystal-battle/src/battle/`](../crates/crystal-battle/src/battle/) | The library API: `Battle`, actions and legal actions, events, typed state, outcome. |
| [`crates/crystal-battle/src/config.rs`](../crates/crystal-battle/src/config.rs) | `BattleConfig`: the scenario a battle starts from (the TOML schema deserializes into it). |
| [`crates/crystal-battle/src/data/`](../crates/crystal-battle/src/data/) | Species, move, item and trainer tables; base PP; the random-battle generator. |
| [`crates/crystal-battle/src/ai/`](../crates/crystal-battle/src/ai/) | The trained battle AI (`ai` feature): features, network, PPO trainer, `Agent`. See [ai.md](ai.md). |
| `crates/crystal-battle/models/` | The shipped AI model (`crystal-ai.bin`). |
| [`src/bootstrap/`](../src/bootstrap/) | Battle-launch harness — WRAM seeding, dispatch to `start_battle`, the CLI, recording and playback. |
| [`src/runtime/`](../src/runtime/) | The Game Boy hardware emulation (CPU, GPU, MBC, keypad). The transpiled code shares the same `Cpu` struct. |
| [`src/game/`](../src/game/) | Transpiled engine code. Mirrors the directory layout of pret/pokecrystal. |
| [`src/battle/emulated.rs`](../src/battle/emulated.rs) | `EmulatedBattle`: the library API on the original code. |
| [`src/oracle/`](../src/oracle/) | Running the original for comparison, the differential checks, random test scenarios. |
| [`src/main.rs`](../src/main.rs) | CLI parsing + headless battle runner; the glium/winit window + cpu thread live in a `gui`-gated module. |
| `pokecrystal.gbc` | The ROM (gitignored). Only the harness needs it: the emulated original runs it, and `tools/gen_game_data.py` generates the library's game data (`crates/crystal-battle/src/data/crystal.rs`) from it (`crystal_legacy.rs` comes from a Crystal Legacy build the same way). The library builds without it. |
| `scratch/pokecrystal/` | Vendored copy of pret/pokecrystal (gitignored). Source of truth for symbols, constants, struct layouts. |
| `scratch/*.toml` | Battle scenarios for development. |

## Further reading

- [Bootstrap flow](bootstrap.md) — how we go from `cargo run -- foo.toml` to a running battle.
- [Memory layout](memory_layout.md) — WRAM addresses we touch, and where to look them up.
- [TOML schema](toml_schema.md) — what a battle config looks like today.
- [Transpiled code](transpiled_code.md) — the rules for editing `src/game/`, the `HARNESS-PATCH:` convention, and the "graduate to hand-written" model.
- [Roadmap](roadmap.md) — what step 2+ looks like, open questions to resolve.
- [Goal](goal.md) — phases 3 and 4 (done), the exactness contract, how it is verified, and known gaps.
- [Engine](engine.md) — the hand-written engine: architecture, porting rules, verification tools.
- [AI](ai.md) — phase 5: the trained battle AI, why PPO + a feed-forward network, features, training, evaluation.
- [Library](library.md) — the library as a game component: ROM-free game data, full setup in, typed events, the result out (in progress).
