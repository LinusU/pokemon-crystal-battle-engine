# Bootstrap flow

Goal: jump straight into a battle described in a TOML file, in roughly the same state the engine would be in if the player walked into the encounter from the overworld.

## The pieces

1. **`main.rs`** parses `argv[1]` as a TOML path (if present), spawns the CPU thread, and renders the GPU output into a glium window. Without a path, it just boots Crystal as normal.
2. **`bootstrap::load_battle`** parses the TOML into a `BattleConfig`.
3. **`bootstrap::run`** is what the CPU thread invokes:
   1. Set `cpu.skip_intro_to_battle = true`.
   2. Call the normal entry routine (`header::start`).
   3. That tail-calls all the way down to `init_no_double_speed`, which now bails out before `game_init` thanks to the flag — so the hardware (LCDC, palettes, vblank, audio, OAM-DMA stub, etc.) is fully initialised but the title/intro sequence never runs.
   4. With control back in Rust, the harness seeds WRAM and calls `start_battle`.

The single line we patched in transpiled code is at the top of `intro_sequence`:

```rust
pub(crate) fn intro_sequence(cpu: &mut Cpu) {
    if cpu.skip_intro_to_battle {
        return;
    }
    // ... original body
}
```

This bails out *after* `game_init` has done its display init (`TryLoadSaveData`, `ClearTilemap`, `ClearBGPalettes`, set `hBGMapAddress = 0x9800`, zero a couple of HRAM bytes). We need that display state — without it, opening the bag mid-battle queues a 2bpp tile transfer to a stale destination and corrupts HRAM.

If you re-run the transpiler from upstream, re-apply this hook.

## Seeding the battle

For a wild encounter, the harness writes these WRAM bytes (see [memory_layout.md](memory_layout.md)):

- **Player identity**: `wPlayerName`, `wPlayerID`, `wPlayerGender`, `wTimeOfDay`, `wTimeOfDayPal`. Just enough that engine routines that read them get sane values.
- **Guards**: `wLinkMode = 0`, `wInBattleTowerBattle = 0`, `wOTPartyCount = 0`.
- **Bag pockets**: `wNumKeyItems` / `wNumPCItems` are always empty (count=0 + `0xff` terminator, matching `_ResetWRAM.InitList`). `wNumItems` and `wNumBalls` are filled from `[[player_items]]` in the TOML — each entry is resolved via [`src/bootstrap/items.rs`](../src/bootstrap/items.rs) and routed to the matching pocket; both pockets always end with the `0xff` terminator. Without the terminator the pack menu walks off the list and corrupts neighbouring WRAM — see [memory_layout.md](memory_layout.md).
- **Player party**: for each entry, set `wCurPartySpecies`, `wCurPartyLevel`, `wCurSpecies`, `wMonType = 0` (PARTYMON), `wBattleMode = 0` (so default item / random moves), then call `try_add_mon_to_party`. The engine fills in DVs, computes stats, picks the default moveset.
- **Encounter state**: `wBattleMode = WILD_BATTLE`, `wBattleType = BATTLETYPE_NORMAL`, `wOtherTrainerClass = 0`, `wTempWildMonSpecies`, `wTempEnemyMonSpecies`, `wCurPartyLevel` (= enemy level), `wCurPartySpecies` (= enemy species).
- Finally, call `start_battle`.

## Trainer battles

For `type = "trainer_battle"`, the player-side seeding is identical to a wild encounter (identity, items, party). The encounter-state bytes differ:

- **Trainer identity**: `wOtherTrainerClass` + `wOtherTrainerID`. The harness resolves `trainer = "Whitney"` (etc.) via [`src/bootstrap/trainers.rs`](../src/bootstrap/trainers.rs).
- **Mode**: `wBattleMode = TRAINER_BATTLE`, `wBattleType = BATTLETYPE_NORMAL`.

Once those are set, `start_battle → BattleIntro → InitEnemy` notices nonzero `wOtherTrainerClass` and routes to `InitEnemyTrainer`. That routine does the heavy lifting for us: `GetTrainerAttributes` copies the AI items + base reward out of `TrainerClassAttributes`, and `ReadTrainerParty` walks `TrainerGroups` for the matching `(class, id)` pair and calls `try_add_mon_to_party` once per OT mon, filling `wOTPartyCount` / `wOTPartyMon{n}` with the canonical party.

Because the engine owns the enemy-party load for trainer battles, the TOML's `[[enemy_party]]` and `[[enemy_items]]` are currently informational only. Overriding them would need a `HARNESS-PATCH:` in `init_enemy_trainer` that skips the two `callfar`s when a `cpu.skip_trainer_party_load` flag is set, plus harness-side pre-seeding of `wOTParty*` (using `wMonType = OTPARTYMON`) and `wEnemyTrainerItem1` / `Item2`. See [roadmap.md](roadmap.md).

## ROM banks

Every transpiled function lives in a specific ROM bank. The transpiled code does **not** automatically bankswitch on Rust function entry — it assumes the right bank is already loaded. So before each top-level engine call, the harness writes to `hROMBank` (0xff9d) and the MBC bank-select (`R_ROMB` = 0x2000):

- `try_add_mon_to_party` → bank `0x03`
- `start_battle` → bank `0x0f`

The trainer-battle path only needs the same banks: `start_battle` (`0x0f`) covers `BattleIntro` / `InitEnemyTrainer` / `GetTrainerAttributes` / `ReadTrainerParty`, which are all in bank `0x0e` and reached via `callfar` (so the engine handles its own bankswitching inside).

If you add new direct-dispatch calls, look the symbol up in `scratch/pokecrystal/pokecrystal.sym` and switch to the bank shown in the prefix (e.g. `03:588c TryAddMonToParty`).

## What we deliberately skip

- **Title screen / New Game flow**: we don't simulate keypresses to drive the intro. The seeded player state is barebones — no Pokédex flags, no items, no money, no badges.
- **Intro cutscene**: `intro_sequence` (and onward to title) never runs.
- **Save loading**: `TryLoadSaveData` does run as part of `game_init`, but with no save file it falls through to the default "new game" state.
