# The hand-written engine (`crates/crystal-battle/src/engine/`)

A port of pret/pokecrystal's battle engine to plain Rust, verified against the
original (the transpiled engine on the emulated CPU) by differential testing.
Goal and exactness contract: [goal.md](goal.md).

## Architecture

| File | What |
| --- | --- |
| `mem.rs` | `Mem`: the original's WRAM banks 0–1 and HRAM, byte for byte. All battle state lives here, in the original layout. |
| `rom.rs` | Read-only data straight from `pokecrystal.gbc` (`rom::byte`, `rom::ptr`). `Engine::rom_byte` adds the scenario's ROM overlay (the `[enemy_trainer]` AI-flag patches). |
| `sym.rs`, `rsym.rs` | Generated (`python3 tools/gen_engine_syms.py`, writes both): every WRAM/HRAM symbol (`W_BATTLE_MON_HP`, `H_BATTLE_TURN`, …) and every ROM label the battle code mentions, local text labels included (`r::MOVES`, `r::BATTLE_TEXT_…`, as flat ROM offsets). |
| `battle.rs` | `Engine` (memory + random stream + flags + optional logs) and shared primitives: `random` / `battle_random`, `battle_var*` (`GetBattleVar`), turn switching, `text` / `text_with`, `decide`. |
| `math.rs` | `Multiply` / `Divide` on the original's overlapping HRAM buffers. |
| `start.rs` | Starting a battle without the emulator: the harness's scenario seeding (player, rival name, bag, badges, parties; mirrors `bootstrap::launch_*`) and the game's `TryAddMonToParty`, `StartBattle` / `BattleIntro`, `ReadTrainerParty`, `InitEnemy*`, `BattleStartMessage` and `DoBattle`'s opening, up to the first `BattleTurn`. |
| `core.rs` | `engine/battle/core.asm`: the turn loop, action parsing, move order, between-turn effects, Berserk Gene, fleeing. |
| `commands.rs` | The move-effect script reader (`DoTurn` / `DoMove`), the dispatch of all 175 commands, `CheckTurn`, obedience, used-move text, PP, critical hits, STAB and type matchups, damage variation. |
| `damage.rs` | The damage pipeline (`CheckHit`, `DamageStats`, `DamageCalc`, applying damage, substitutes, loops, failure text, animation memory effects). |
| `residual.rs` | HP routines (`AnimateHPBar`'s memory side), residual damage, held items in residual effects. |
| `status.rs` | Status and stat-change commands, stat recalculation. |
| `misc.rs`, `effects_a.rs`, `effects_b.rs` | The remaining effect commands and `move_effects/*.asm` (attract..mist, nightmare..transform) with the small `effect_commands.asm` helpers they share. |
| `switch.rs` | Switching, fainting, send-outs, badge boosts, Spikes, `AI_Switch`. |
| `setup.rs` | Experience and level-ups, `LoadEnemyMon`, trainer-battle switching (`EnemySwitch`, `FindMonInOTPartyToSwitchIntoBattle`), prize money, `GetTMHMName`. |
| `ai.rs`, `ai_smart.rs`, `ai_switch.rs`, `ai_items.rs` | The trainer AI (`engine/battle/ai/`): `AIChooseMove` and the scoring layers, the `AI_Smart_*` handlers, `CheckAbleToSwitch` and friends, `AI_SwitchOrTryItem` and the item routines. |
| `items.rs` | `BattleMenu_Pack` and every bag item the harness offers (`engine/items/item_effects.asm`: heals, status heals, revives, PP items, X items, berries, Poké Doll, Poké Balls with the catch formula, wobble rolls and the party bookkeeping after a catch), `TossItem`. |
| `names.rs`, `text.rs` | Names into `wStringBuffer1`; the text command interpreter (renders a text label to the exact messages, `text_asm` handlers choose the next text). |
| `harness.rs` | The emulated harness's replacements for the original's menus (action menu, move selection, link actions), ported as the HARNESS-PATCHes run them. |
| `api.rs` | `FastBattle`: start, `step`, `status`, `state`, `events`, `Clone`. The library's `battle::Battle` wraps it. |

The oracle tooling lives in the harness crate (the repository root,
[`src/oracle/`](../src/oracle/)), since it runs the transpiled original:

| File | What |
| --- | --- |
| `oracle/mod.rs` | Runs the original engine and records memory, RNG state, legal actions, text and the action at every decision (random policy or a recording's script), the battle's end, and every `Random` call with its caller. |
| `oracle/diff.rs` | Differential checks: `check_start` (the start state, all of WRAM) and `check` (every decision and the end: site, legal actions, random stream, `COMPARED` memory, text). |
| `oracle/testgen.rs` | Random scenarios for differential testing: link / wild / trainer, random parties and moves, held items, bags, traded mons, badges. |

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
  `self.text(r::LABEL)`. A text whose `text_asm` picks the next text gets a
  handler in `text.rs` (`text_asm`); if the battle code already computed the
  choice, pass it with `self.text_with(label, &[choice])`. Printing is
  otherwise read-only, except where the original's text engine writes memory
  (`<ENEMY>` runs `Battle_GetTrainerName`; see `Engine::text_with`).
- **Text engine model** (`text.rs`): the renderer follows the original's tile
  cursor exactly — `hl` while placing, `bc` between text commands, and the
  line start `PlaceString` keeps on the stack (`<NEXT>`, `<LF>`, `<CR>` move
  from it) — including the registers routines clobber (`ld bc, 40` in
  `<NEXT>`, the dakuten mark in `b`, `TextScroll`'s `bc = 0`, a link
  battle's `<PROMPT>`). A character is battle text only if it lands in the
  text box, as in the capture. Normal text never leaves the box; garbage
  (Beat Up's names past the party) can, and then the characters it places
  past the tilemap are written to memory. `<MOBILE>` (`RunMobileScript`) is
  ported where it stays in WRAM banks 0-1 and ROM (skip commands, cursor
  moves, the index print with `_MobilePrintNum`, table strings); the
  commands that read SRAM or leave WRAM bank 4 mapped, a jump into garbage,
  and garbage reading or writing state the engine doesn't keep (the
  tilemap, sound, I/O, a ROM bank switch) panic instead.
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

Three tools compare the engine with the original (the transpiled code on the
emulated CPU). Build them in release (`CARGO_TARGET_DIR=target-plain cargo
build --release --examples`).

- `engine_diff [link|wild|trainer] <battles> <first_seed> <moves...>`: random
  battles (random parties, held items, bags, traded mons, badges; a random
  policy) on both engines in lockstep. Checks the start state, every decision
  (site, legal actions, random-stream state, memory, battle text) and the end
  of the battle (memory, random stream, text). A random-stream divergence names
  the first differing draw and who drew it on each side. `NO_TEXT=1` skips the
  text, `NO_ITEMS=1` the bag, `LONG_BATTLES=1` keeps the policy from ending a
  wild battle early.
- `engine_fixtures fixtures/*.toml`: the recorded fixture corpus, each
  recording's decisions driving both engines in lockstep (same checks).
- `api_diff [link|wild|trainer] <battles> <first_seed>`: the library API on
  both engines, `battle::Battle` against `battle::EmulatedBattle`: every
  `Decision` (site, side, legal actions, typed state, events) and the outcome.
- `text_diff [names] [first_seed]`: the text engine on garbage, which random
  battles reach too rarely to test: Beat Up's "<name>'s attack!" printed with
  random raw names (control characters, kana, `<MOBILE>` scripts) on both
  engines from a battle's first decision (`oracle::TextBench`,
  `diff::check_garbage_name`). Text and memory must match.

`engine_bench [battles]` measures battles per second on both.

Scenarios where the original game itself freezes (a wild Unown with no
unlocked letters) or the harness can't run it are skipped by a cycle
watchdog. Glitch reads outside the WRAM the engine keeps: echo RAM and ROM
bank 0 read like the original (`Mem::rb`); Beat Up's walk past the party
reads zeros for state neither engine keeps (see [goal.md](goal.md#known-gaps)).
Any other such read panics rather than guess.
