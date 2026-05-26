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
- **Empty bag pockets**: `wNumItems` / `wNumKeyItems` / `wNumBalls` / `wNumPCItems` set to count=0 with a `0xff` terminator in the first slot (matches `_ResetWRAM.InitList`). Without the terminator the pack menu walks off the list and corrupts neighbouring WRAM — see [memory_layout.md](memory_layout.md).
- **Player party**: for each entry, set `wCurPartySpecies`, `wCurPartyLevel`, `wCurSpecies`, `wMonType = 0` (PARTYMON), `wBattleMode = 0` (so default item / random moves), then call `try_add_mon_to_party`. The engine fills in DVs, computes stats, picks the default moveset.
- **Encounter state**: `wBattleMode = WILD_BATTLE`, `wBattleType = BATTLETYPE_NORMAL`, `wOtherTrainerClass = 0`, `wTempWildMonSpecies`, `wTempEnemyMonSpecies`, `wCurPartyLevel` (= enemy level), `wCurPartySpecies` (= enemy species).
- Finally, call `start_battle`.

## ROM banks

Every transpiled function lives in a specific ROM bank. The transpiled code does **not** automatically bankswitch on Rust function entry — it assumes the right bank is already loaded. So before each top-level engine call, the harness writes to `hROMBank` (0xff9d) and the MBC bank-select (`R_ROMB` = 0x2000):

- `try_add_mon_to_party` → bank `0x03`
- `start_battle` → bank `0x0f`

If you add new direct-dispatch calls, look the symbol up in `scratch/pokecrystal/pokecrystal.sym` and switch to the bank shown in the prefix (e.g. `03:588c TryAddMonToParty`).

## What we deliberately skip

- **Title screen / New Game flow**: we don't simulate keypresses to drive the intro. The seeded player state is barebones — no Pokédex flags, no items, no money, no badges.
- **Intro cutscene**: `intro_sequence` (and onward to title) never runs.
- **Save loading**: `TryLoadSaveData` does run as part of `game_init`, but with no save file it falls through to the default "new game" state.
