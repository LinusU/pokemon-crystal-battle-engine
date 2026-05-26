# Living with the transpiled code

Nearly all of `src/game/` is mechanically transpiled from [pret/pokecrystal](https://github.com/pret/pokecrystal). The long-term plan is to replace it with hand-written Rust as the battle engine matures, but for now we have ~2.7 MB of generated code that we want to keep retranspile-friendly.

There are three states a chunk of code can be in. Treat them differently.

## 1. Transpiled (default)

What `src/game/` looks like today. Functions correspond 1:1 to assembly routines, share the same `Cpu` state, push/pop the simulated stack, etc.

- **Don't edit for style, cleanup, naming, or refactoring.** Any of those changes get blown away on the next transpile run.
- **Don't add asserts or invariants** "just in case." If you genuinely need to verify something at runtime, do it from `src/runtime/` or `src/bootstrap/` where it survives a retranspile.

## 2. Transpiled with a harness hook

When a real bug or a needed runtime hook forces an edit, keep it tiny and conventional.

- **Mark every patch** with the literal token `HARNESS-PATCH:` in a comment immediately above the inserted code. `rg HARNESS-PATCH src/` should turn up every modification we've made to generated code — that's the single source of truth for "what do I need to re-apply after a retranspile."
- **Keep the patch shape boring.** Prefer a one-line `if cpu.<flag> { return; }` or `if cpu.<flag> { ... }` at a function boundary over weaving logic into the middle of a routine. Easier to re-apply, easier to read.
- **Put the rationale and the WRAM/HRAM/symbol context in the comment**, plus a link to the relevant doc page. The patch site is the only place a future reader sees this; don't make them grep git history.
- **Add a row to the "Active patches" table below** in the same change.

Example (the only patch we have today, in [src/game/engine/menus/intro_menu.rs:3188](../src/game/engine/menus/intro_menu.rs:3188)):

```rust
pub(crate) fn intro_sequence(cpu: &mut Cpu) {
    // HARNESS-PATCH: bail out of the boot flow before the title/intro sequence
    // so the battle-engine consumer can take over. `game_init` has already
    // done its display init ... See docs/transpiled_code.md.
    if cpu.skip_intro_to_battle {
        return;
    }
    // ... original body
}
```

## 3. Hand-written (graduated)

Eventually some modules — likely the battle turn loop and RNG first — will be useful enough to rewrite by hand. When that happens:

- **Move the file out of `src/game/`** into something like `src/handwritten/` (or rename and update the module tree), so the boundary is visible at a glance.
- **Stop treating it as transpiler output.** Refactor freely, add invariants, change shape, add unit tests.
- **The transpiled version stays around** as a cross-check until we trust the rewrite — keep a copy under `src/game/.archive/` or similar for reference.

We're not graduating anything yet. The prerequisite is a test/replay harness solid enough to detect regressions: without that, every rewrite is a leap of faith. See [roadmap.md](roadmap.md).

## Active patches

| File | Function | Flag | Purpose |
| --- | --- | --- | --- |
| [src/game/engine/menus/intro_menu.rs](../src/game/engine/menus/intro_menu.rs) | `intro_sequence` | `cpu.skip_intro_to_battle` | Skip title/intro sequence so the harness can drive straight into a battle. |
| [src/game/engine/battle/ai/scoring.rs](../src/game/engine/battle/ai/scoring.rs) | `ai_smart_checkmove` | — (unconditional) | Dispatch the AI_Smart per-effect handler directly. The asm uses `push de; ret` as an indirect `jp de` into bank 0x0e, which the transpiler can't follow — without the patch, AIChooseMove returns with PC pointing at the handler and FarCall_hl trips its assert. |

Update this table whenever you add or remove a `HARNESS-PATCH:`.
