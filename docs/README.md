# pokemon-crystal-battle-engine — living docs

This crate is a Rust port of the [pret/pokecrystal](https://github.com/pret/pokecrystal) Pokémon Crystal disassembly, mechanically produced by a custom transpiler. The current goal is to evolve it into a **headless Pokémon Crystal battle engine** that exposes the battle as a state machine: feed actions in, get the next state out, repeat until the battle ends.

The transpiled code is in `src/game/`, `src/dispatch.rs`, `src/defs.rs`. We avoid editing it where we can — when we have to (e.g. boot-flow hooks), we patch a single line and document why here.

## Where we are

- ✅ The transpiled game still boots end-to-end (`cargo run` with no args plays Crystal).
- ✅ TOML-driven launch: `cargo run -- scratch/first.toml` skips the title/intro and starts a battle.
- ✅ Wild encounters supported with `[battle] type = "wild_encounter"`, one player party, one enemy mon.
- ✅ Bag items via `[[player_items]]` — sorted into the engine's Items / Balls pockets.
- ✅ Trainer battles via `[battle] type = "trainer_battle"`, `trainer = "Whitney"` — the engine loads the canonical party and AI items from ROM.
- ⬜ Per-mon overrides (moves, DVs, items) and per-trainer overrides for the enemy party and `wEnemyTrainerItem1`/`Item2`.
- ✅ CLI-driven actions via `[battle] cli = true` — every in-battle menu (action / move / forced switch / "switch out enemy mon" prompt) becomes a numbered list on stdin. See [`src/bootstrap/cli.rs`](../src/bootstrap/cli.rs) and [TOML schema](toml_schema.md).
- ⬜ Headless mode (skip glium/winit, just iterate the state machine).
- ⬜ A typed `step(action) -> Outcome` API. The CLI driver already sits at the action chokepoints; lifting it into a library surface is mostly relayout.

## Layout

| Path | What's there |
| --- | --- |
| [`src/bootstrap/`](../src/bootstrap/) | Battle-launch harness — TOML schema, species table, WRAM seeding, dispatch to `start_battle`. |
| [`src/runtime/`](../src/runtime/) | The Game Boy hardware emulation (CPU, GPU, MBC, keypad). The transpiled code shares the same `Cpu` struct. |
| [`src/game/`](../src/game/) | Transpiled engine code. Mirrors the directory layout of pret/pokecrystal. |
| [`src/main.rs`](../src/main.rs) | glium/winit window + cpu thread. |
| `scratch/pokecrystal/` | Vendored copy of pret/pokecrystal (gitignored). Source of truth for symbols, constants, struct layouts. |
| `scratch/*.toml` | Battle scenarios for development. |

## Further reading

- [Bootstrap flow](bootstrap.md) — how we go from `cargo run -- foo.toml` to a running battle.
- [Memory layout](memory_layout.md) — WRAM addresses we touch, and where to look them up.
- [TOML schema](toml_schema.md) — what a battle config looks like today.
- [Transpiled code](transpiled_code.md) — the rules for editing `src/game/`, the `HARNESS-PATCH:` convention, and the "graduate to hand-written" model.
- [Roadmap](roadmap.md) — what step 2+ looks like, open questions to resolve.
