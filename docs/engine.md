# The hand-written engine (`src/engine/`)

A port of pret/pokecrystal's battle engine to plain Rust, verified against the
original (the transpiled engine on the emulated CPU) by differential testing.
Goal and exactness contract: [goal.md](goal.md).

## Architecture

| File | What |
| --- | --- |
| `mem.rs` | `Mem`: the original's WRAM banks 0–1 and HRAM, byte for byte. All battle state lives here, in the original layout. |
| `rom.rs` | Read-only data straight from `pokecrystal.gbc` (`rom::byte`, `rom::ptr`). |
| `sym.rs` | Generated (`python3 tools/gen_engine_syms.py > src/engine/sym.rs`): every WRAM/HRAM symbol (`W_BATTLE_MON_HP`, `H_BATTLE_TURN`, …) and every ROM label the battle code mentions (`MOVES`, `BATTLE_TEXT_…`, as flat ROM offsets). |
| `battle.rs` | `Engine` (memory + random stream + flags) and shared primitives: `random` / `battle_random`, `battle_var*` (`GetBattleVar`), turn switching, `text`, `decide`. |
| `math.rs` | `Multiply` / `Divide` on the original's overlapping HRAM buffers. |
| `core.rs` | `engine/battle/core.asm`. |
| `ai.rs`, `ai_smart.rs`, `ai_switch.rs`, `ai_items.rs` | The trainer AI (`engine/battle/ai/`): `AIChooseMove` and the scoring layers, the `AI_Smart_*` handlers, `CheckAbleToSwitch` and friends, `AI_SwitchOrTryItem` and the item routines. AI flags and items are read from `TrainerClassAttributes` in the plain ROM (the harness' `poke_rom` overrides for `[enemy_trainer]` / `[[enemy_items]]` are not modelled yet). HUD redraws and sound are dropped. |
| `todo.rs` | Stubs for routines not ported yet; each panics with the original's name. |
| `oracle.rs` | Runs the original engine and records memory, RNG state, legal actions and a random policy's action at every decision, plus every `Random` call with its caller. |
| `testgen.rs` | Random scenarios for differential testing. |

### Decisions, `step`, and cloning

Routines return `Flow<T> = Result<T, Suspend>`. A routine that needs a
decision calls `self.decide(site)?`, which either pops the next queued
decision or returns `Err(Suspend { site, legal })`; `?` carries it to the top
of the turn. The public API keeps a snapshot of the engine at the start of the
current turn plus the decisions made in this turn so far; `step(action)`
restores the snapshot and replays the turn with one more decision. A turn is
microseconds of work, so this is cheap, and it keeps every routine a straight
port of the original's sequential code. Cloning a battle is cloning the
snapshot (about 8 KB).

## Porting a routine

- **One Rust function per ASM routine**, named in snake_case after it
  (`BattleCommand_CheckHit` → `battle_command_check_hit`), with a doc comment
  naming the routine. Local labels become control flow inside the function.
- **Memory**: read/write through `self.mem` with the `sym` constants.
  `GetBattleVar(BATTLE_VARS_X)` → `self.battle_var(BATTLE_VARS_X)`;
  `GetBattleVarAddr` → `self.battle_var_addr(..)`.
- **Registers and flags** become parameters and return values. If a caller
  relies on a register a routine leaves behind (common!), return it.
- **Keep every write to battle memory**, including ones that look cosmetic
  (`wNamedObjectIndex`, `wStringBuffer1`, `wFXAnimID`): later code or text may
  read them, and the differential tester compares memory.
- **Drop graphics, sound and frame timing**: `DelayFrames`, `WaitBGMap`,
  animations (`PlayBattleAnim`, `Call_PlayBattleAnim*`, `FarPlayBattleAnimation`),
  HUD redraws, `PlaySFX`/cries, `ClearBox`, tilemap work. These never affect the
  battle (verified: the corpus replays unchanged with Battle Scene off). Keep
  any memory writes those routines make *outside* graphics/sound RAM.
- **Text**: `StdBattleTextbox` / `PrintText` with `hl = Label` →
  `self.text(LABEL)`. Text with `text_asm` logic is ported when text rendering
  lands; until then record the label.
- **Random numbers**: `call BattleRandom` → `self.battle_random("Routine")`,
  `call Random` → `self.random("Routine")`, with the routine's name (used by
  the differential tester to say which routine drew a byte).
- **`farcall` / `callfar` / `predef`** are plain calls. Bank switching
  disappears.
- **Bugs are features**: port the original's behaviour, glitches included
  (pokecrystal marks them `; BUG:`). Never fix one.
- **Harness patches**: where the emulated harness replaces an original routine
  (a `HARNESS-PATCH:` in `src/game/`, listed in [transpiled_code.md](transpiled_code.md)),
  port the *patched* behaviour (that is what the oracle runs) and say so.

## Verification

`examples/engine_diff.rs` (coming next) runs random battles on both engines in
lockstep: same scenario, same seed, same decisions. At each decision it
compares the legal actions, the random-stream state and the battle memory
(a whitelist of logic-relevant ranges), and reports the first divergence with
symbol names and the oracle's `Random` caller log.
