# For coding agents working on this repo

Before doing anything non-trivial: read [docs/README.md](docs/README.md) and the pages it links to. They cover the project goal (headless Crystal battle engine), the bootstrap flow, the WRAM addresses we touch, the TOML schema, and the roadmap.

Keep those docs in sync. If you change the bootstrap, schema, memory layout, or the high-level plan, update the relevant page in `docs/` in the same change.

Other things worth knowing:

- **Build debug, not release.** Release takes several minutes to compile and runs too fast. `cargo build` / `cargo run` / `cargo test`, never `--release`.
- **Most of `src/game/` is transpiler output** from [pret/pokecrystal](https://github.com/pret/pokecrystal). Don't edit it for style or "cleanup." Small targeted patches (e.g. a runtime hook) are fine, but mark each one with a `HARNESS-PATCH:` comment and add a row to the table in [docs/transpiled_code.md](docs/transpiled_code.md) — that doc is the single source of truth for what's been modified and how the patches will eventually evolve into hand-written modules.
- **Source of truth** for symbol addresses, constants, and struct layouts is `scratch/pokecrystal/` — especially `pokecrystal.sym`, `constants/`, and `macros/ram.asm`. Look there before guessing.
- **Bank-switch before calling transpiled engine routines** from `src/bootstrap/`. Transpiled functions assume the right ROM bank is loaded; they don't switch on entry. Use the bank prefix in `pokecrystal.sym` (e.g. `0f:74dd BattleIntro` → bank `0x0f`) and `bootstrap::switch_bank`.
- **The end goal is a library, not an emulator.** Glium/winit are a harness. New abstractions should make sense for a headless `step(action) -> state` API even if the emulator is the only consumer today.
