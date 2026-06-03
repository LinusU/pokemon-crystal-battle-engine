# pokemon-crystal-battle-engine — living docs

This crate is a Rust port of the [pret/pokecrystal](https://github.com/pret/pokecrystal) Pokémon Crystal disassembly, mechanically produced by a custom transpiler. The current goal is to evolve it into a **headless Pokémon Crystal battle engine** that exposes the battle as a state machine: feed actions in, get the next state out, repeat until the battle ends.

The transpiled code is in `src/game/`, `src/dispatch.rs`, `src/defs.rs`. We avoid editing it where we can — when we have to (e.g. boot-flow hooks), we patch a single line and document why here.

## Where we are

- ✅ The transpiled engine boots end-to-end — the battle harness runs the real boot path (hardware init, title/intro skipped) before jumping straight into the battle.
- ✅ TOML-driven launch: `cargo run -- scratch/first.toml` skips the title/intro and starts a battle.
- ✅ Wild encounters supported with `[battle] type = "wild_encounter"`, one player party, one enemy mon.
- ✅ Bag items via `[[player_items]]` — sorted into the engine's Items / Balls pockets.
- ✅ Every battle-usable item works from the action menu: HP/status/PP heals, revives, X-items, Dire Hit, Guard Spec, confusion cure. In wild battles you can also throw a Poké Ball to **catch** the wild mon (the caught mon joins the party, `outcome = "caught"`) or use a Poké Doll to escape (`outcome = "draw"`).
- ✅ Trainer battles via `[battle] type = "trainer_battle"`, `trainer = "Whitney"` — the engine loads the canonical party and AI items from ROM.
- ✅ Link battles via `[battle] type = "link_battle"` — both parties are fully specified (species, level, moves) and **both sides are CLI-driven** (no AI). The harness seeds `wParty*` and `wOTParty*`, sets `wLinkMode` so the engine takes its real link path, and feeds the enemy's per-round action through the `link_battle_send_receive_action` hook. Each round prompts the player then the enemy; recording tags enemy decisions `side = "enemy"`. See [TOML schema](toml_schema.md#link-battles).
- 🟡 Per-mon overrides: player-party `moves = [...]` is supported (see [TOML schema](toml_schema.md#player_party--one-or-more)); DVs, held items, and per-trainer enemy-party / `wEnemyTrainerItem1`/`Item2` overrides are still on the [roadmap](roadmap.md).
- ✅ CLI-driven actions — every in-battle menu (action / move / forced switch / "switch out enemy mon" prompt) is a numbered list on stdin. See [`src/bootstrap/cli.rs`](../src/bootstrap/cli.rs).
- ✅ Recording with `--record <out.toml>` — every CLI decision (plus the resolved seed) is appended as a `[[step]]` block to a TOML file for later replay. The resolved matchup is embedded as a `[scenario.*]` block, so a recording is self-contained for every battle kind. See [TOML schema](toml_schema.md#recording----record-path).
- ✅ Playback with `--play <recording.toml>` — drives the battle from a recorded `[[step]]` list with no stdin prompts; the recording's seed is authoritative and its embedded `[scenario.*]` block reconstructs the matchup, so no other file is needed. See [TOML schema](toml_schema.md#playback----play-path).
- ✅ Random battles with `--random` (optionally `--seed <n>`) — skip authoring TOML and start a deterministic randomized trainer battle; the recording embeds the resolved matchup and replays from the seed, so the generator is never re-run. See [TOML schema](toml_schema.md#random-battles----random-and---seed-n).
- ✅ Headless by default — glium/winit sit behind the off-by-default `gui` cargo feature, so a plain `cargo run` / `cargo build` compiles no graphics deps and drives the battle straight to completion with no window. `--features gui` adds an opt-in window that shows the same battle. Either way a battle must be specified (`<scenario.toml>` / `--random` / `--play`); the binary always runs a single battle and never boots into the overworld.
- ⬜ A typed `step(action) -> Outcome` API. The CLI driver already sits at the action chokepoints; lifting it into a library surface is mostly relayout.

## Layout

| Path | What's there |
| --- | --- |
| [`src/bootstrap/`](../src/bootstrap/) | Battle-launch harness — TOML schema, species table, WRAM seeding, dispatch to `start_battle`. |
| [`src/runtime/`](../src/runtime/) | The Game Boy hardware emulation (CPU, GPU, MBC, keypad). The transpiled code shares the same `Cpu` struct. |
| [`src/game/`](../src/game/) | Transpiled engine code. Mirrors the directory layout of pret/pokecrystal. |
| [`src/main.rs`](../src/main.rs) | CLI parsing + headless battle runner; the glium/winit window + cpu thread live in a `gui`-gated module. |
| `scratch/pokecrystal/` | Vendored copy of pret/pokecrystal (gitignored). Source of truth for symbols, constants, struct layouts. |
| `scratch/*.toml` | Battle scenarios for development. |

## Further reading

- [Bootstrap flow](bootstrap.md) — how we go from `cargo run -- foo.toml` to a running battle.
- [Memory layout](memory_layout.md) — WRAM addresses we touch, and where to look them up.
- [TOML schema](toml_schema.md) — what a battle config looks like today.
- [Transpiled code](transpiled_code.md) — the rules for editing `src/game/`, the `HARNESS-PATCH:` convention, and the "graduate to hand-written" model.
- [Roadmap](roadmap.md) — what step 2+ looks like, open questions to resolve.
