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
| [src/game/home/random.rs](../src/game/home/random.rs) | `random` | — (unconditional) | Overwrite `hRandomAdd` / `hRandomSub` wholesale from a seedable xorshift64 (`Cpu::next_rng_byte`) instead of folding in the DIV register, so battle outcomes don't depend on frame timing or keypad cycles. `_battle_random` already collapses to `random` when `wLinkMode == 0`, which is always true in the harness, so one patch covers both. |
| [src/game/home/vblank.rs](../src/game/home/vblank.rs) | `v_blank_normal` | — (unconditional) | Skip the original asm's per-frame `rDIV → hRandomAdd/hRandomSub` stir. The seeded PRNG owns those bytes; without this patch every VBlank reintroduces frame-timing entropy and `random()` callers see drifting base values. PC and cycle counts are advanced to match the skipped 16-byte / 88-cycle block. |
| [src/game/home/joypad.rs](../src/game/home/joypad.rs) | `joy_wait_aor_b` + `joy_wait_aor_b_loop` | — (unconditional) | Skip "press A/B" text-continue waits when the CLI driver owns input. Both entry points are gated because some engine paths `become` straight into the loop. |
| [src/game/home/joypad.rs](../src/game/home/joypad.rs) | `get_joypad` | — (unconditional) | After the normal joypad read, force `0xffa7` (newly pressed) and `0xffa8` (held) to `PAD_A`. Every text-wait loop in joypad.rs (`simple_wait_press_aor_b`, `wait_press_aor_b_blink_cursor`, `prompt_button_input_wait_loop`) reads one of those (directly or via `joy_text_delay → 0xffa9`) and falls through immediately. Also makes the per-character text crawl render at "fast" speed. The interactive menus that would mis-trigger on auto-A are all already bypassed by their own CLI patches. |
| [src/game/engine/battle/core.rs](../src/game/engine/battle/core.rs) | `battle_menu` | — (unconditional) | Replace the action-choice menu (Fight/Pkmn/Pack/Run + submenus) with `bootstrap::cli::handle_battle_menu`, which prompts on stdin and dispatches via `become` into `battle_menu_fight` / `try_player_switch_try_switch` / `battle_menu_run` or applies a healing item directly. |
| [src/game/engine/battle/core.rs](../src/game/engine/battle/core.rs) | `move_selection_screen` | — (unconditional) | Consume `cpu.cli_pending_move` (stashed by the action menu) or call `bootstrap::cli::handle_move_selection` for the engine's Disable/No-PP re-pick path. Writes `wMenuCursorY` (0xcfa9), `wCurMoveNum` (0xd0d5), `wCurPlayerMove` (0xc6e3) — matching the contract of `move_selection_screen_use_move` → `_skip2`. |
| [src/game/engine/battle/core.rs](../src/game/engine/battle/core.rs) | `force_pick_party_mon_in_battle` | — (unconditional) | Replace the post-faint forced-switch UI with a terminal prompt. Writes `wCurPartyMon` (0xd109) and returns the success contract of `pick_party_mon_in_battle_loop`. |
| [src/game/engine/battle/core.rs](../src/game/engine/battle/core.rs) | `ask_use_next_pokemon` | — (unconditional) | Auto-answer "Yes" to "Use next Pokémon?" — the user's forced-switch is handled by the separate patch in `force_pick_party_mon_in_battle`. |
| [src/game/engine/battle/core.rs](../src/game/engine/battle/core.rs) | `offer_switch` | — (unconditional) | Replace the "Trainer is about to send in — switch?" yes/no + party menu with a single terminal list ("Keep using X" + alive party slots). Sets `wLastPlayerMon` (0xc71a) and `wCurBattleMon` (0xd0d4) on swap, and zeroes `wCurPlayerMove` (0xc6e3) / `wCurEnemyMove` (0xc6e4) like the asm's Accept path does — otherwise a stale move id can bleed into `BattleMonEntrance → PursuitSwitch`'s `GetBattleVar BATTLE_VARS_MOVE` check on the way to the new mon's entrance. Returns `flag_c = true` on keep — matches the engine's offer_switch contract. The asm's screen cleanup (`ClearPalettes`/`DelayFrame`/`_LoadHPBar`) is skipped on purpose; those expect a party-menu overlay we never opened. |

Update this table whenever you add or remove a `HARNESS-PATCH:`.
