# The hand-written engine (`crates/crystal-battle/src/engine/`)

A port of pret/pokecrystal's battle engine to plain Rust, verified against the
original (the transpiled engine on the emulated CPU) by differential testing.
Goal and exactness contract: [goal.md](goal.md).

## Architecture

| File | What |
| --- | --- |
| `mem.rs` | `Mem`: the original's WRAM banks 0–1 and HRAM, byte for byte. All battle state lives here, in the original layout. |
| `../data/game.rs`, `../data/crystal.rs` | `GameData`: species, moves, items, trainer classes, the type chart, move effect scripts, growth rates and the mechanics' small tables, typed. `crystal.rs` is generated from the ROM by `tools/gen_game_data.py`; tables indexed by an id cover all 256 values, past the real range holding what the original reads there. `Engine::data` points at it; `Engine::trainer_class` applies the scenario's `[enemy_trainer]` AI-flag override. |
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
| `names.rs`, `message.rs` | Names: what each of the original's name buffers names (`Name`: a species, move, item, … or a nickname, kept as a string), never text; the name bytes stay zero (`mem::NAMES`). Messages: each text label the battle prints, as a typed `battle::Message` read from the state the text reads. |
| `harness.rs` | The emulated harness's replacements for the original's menus (action menu, move selection, link actions), ported as the HARNESS-PATCHes run them. |
| `load.rs` | Starting from a game's `BattleSetup` (its parties, player, bag, opponent, world written into the original's layout, then the game's own battle start); `generate`, the setup a `BattleConfig` scenario describes; reading the `BattleResult` back. |
| `api.rs` | `FastBattle`: start (from a scenario or a setup), `step`, `status`, `state`, `events`, `result`, `Clone`. The library's `battle::Battle` wraps it. |

The oracle tooling lives in the harness crate (the repository root,
[`src/oracle/`](../src/oracle/)), since it runs the transpiled original:

| File | What |
| --- | --- |
| `oracle/mod.rs` | Runs the original engine and records memory, RNG state, legal actions, text and the action at every decision (random policy or a recording's script), the battle's end, and every `Random` call with its caller. |
| `oracle/diff.rs` | Differential checks: `check_start` (the start state, all of WRAM) and `check` (every decision and the end: site, legal actions, random stream, `COMPARED` memory, text). |
| `oracle/testgen.rs` | Random scenarios for differential testing: link / wild / trainer, random parties and moves, held items, bags, traded mons, badges. |
| `oracle/render.rs`, `oracle/text.rs` | The library's messages rendered back to the original's exact English: each message's text label and the memory it reads, through the ported text engine (`text.rs`). The library never renders; the testers compare this with the original's captured text. |

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
- **Show, don't draw**: what the original plays becomes a `battle::Event`
  at the same point: `PlayBattleAnim` (and `Call_PlayBattleAnim*`,
  `FarPlayBattleAnimation`, `PlayFXAnimID`, `LoadAnim`) → `play_battle_anim` /
  `call_play_battle_anim` / `far_play_battle_animation` / `play_fx_anim_id`;
  `PlaySFX` → `self.show(Event::Sound(..))`; `PlayStereoCry` → `Event::Cry`;
  the faint slides, `SlideBattlePicOut`, the trainer sliding back in,
  `AnimateExpBar`, `PrintTempMonStats`, `AnimateFrontpic` → their events. The
  testers compare the stream with hooks on those routines. Drop what is only
  drawing and timing: `DelayFrames`, `WaitBGMap`, HUD redraws, `ClearBox`,
  tilemap work (they never affect the battle; verified: the corpus replays
  unchanged with Battle Scene off). Keep any memory writes those routines
  make *outside* graphics/sound RAM.
- **Text**: `StdBattleTextbox` / `PrintText` with `hl = Label` →
  `self.text(r::LABEL)`, which emits the label's `battle::Message`
  (`message.rs`). A text whose `text_asm` choice the battle code computed
  passes it with `self.text_with(label, &[choice])`. Printing is otherwise
  read-only, except where the original's text engine runs battle code
  (`<ENEMY>` runs `Battle_GetTrainerName`; see `Engine::text_with`).
- **Names**: where the original copies a name into a buffer (`GetName`,
  `GetNickname`, `CopyBytes` of a nickname), set the buffer's `Name`
  (`names.rs`: `copy_name`, `set_name_at`) instead of writing bytes.
- **Text engine model** (the harness's [`src/oracle/text.rs`](../src/oracle/text.rs),
  which renders messages back to text for the testers): the renderer follows the original's tile
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
watchdog. The engine refuses the Unown one at the start (`start::UNOWN_FREEZE`,
a setup error) rather than re-roll forever; `api_diff` reports scenarios the
engine refuses as skipped. Glitch reads outside the WRAM the engine keeps: echo RAM reads like the
original (`Mem::rb`); a read of ROM bank 0 (the game's code, which the
library doesn't have) is an unsupported glitch and stops the engine; Beat Up's walk past the party
reads zeros for state neither engine keeps (see [goal.md](goal.md#known-gaps)).
Any other such read panics rather than guess.
