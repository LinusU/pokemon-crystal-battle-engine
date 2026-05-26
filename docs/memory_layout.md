# Memory layout cheat sheet

The ground truth for symbol addresses is `scratch/pokecrystal/pokecrystal.sym`, which lists every label and the bank/address it resolves to. This page lists only the WRAM regions the battle harness reads or writes today.

## WRAM addresses touched by `bootstrap`

| Address | Symbol | Bytes | Purpose |
| --- | --- | --- | --- |
| `0xc2dc` | `wLinkMode` | 1 | 0 = not a link battle. We force this to 0. |
| `0xcf5f` | `wMonType` | 1 | Whose mon are we operating on. Low nibble 0 = PARTYMON. |
| `0xcf60` | `wCurSpecies` | 1 | Scratch species id; many engine routines read this. |
| `0xcfc0` | `wInBattleTowerBattle` | 1 | Battle-Tower flag; 0 disables the special path. |
| `0xd108` | `wCurPartySpecies` | 1 | Species being added / opponent species. |
| `0xd143` | `wCurPartyLevel` | 1 | Level for the mon being added or the wild encounter. |
| `0xd204` | `wTempEnemyMonSpecies` | 1 | Set by `LoadTrainerOrWildMonPic`; we pre-seed it. |
| `0xd207` | `wEnemyMonItem` | 1 | Default 0; only meaningful in trainer fights. |
| `0xd22d` | `wBattleMode` | 1 | `0` = no battle, `1` = wild, `2` = trainer. |
| `0xd22e` | `wTempWildMonSpecies` | 1 | Species of the wild encounter. |
| `0xd22f` | `wOtherTrainerClass` | 1 | Trainer class id, or 0 for wild. |
| `0xd230` | `wBattleType` | 1 | `BATTLETYPE_NORMAL`, `BATTLETYPE_FISH`, etc. |
| `0xd231` | `wOtherTrainerID` | 1 | Which trainer of the class (`WHITNEY1`, `JOEY3`, …). |
| `0xd269` | `wTimeOfDay` | 1 | 0..3 (MORN/DAY/NITE/?). |
| `0xd280` | `wOTPartyCount` | 1 | Enemy trainer party size; 0 for wild. |
| `0xd472` | `wPlayerGender` | 1 | 0 = male, 1 = female. |
| `0xd47b` | `wPlayerID` | 2 | Player trainer id. Affects DV generation / catch math. |
| `0xd47d` | `wPlayerName` | 11 | `NAME_LENGTH` byte name in Crystal text encoding. |
| `0xd841` | `wTimeOfDayPal` | 1 | Palette id; saved/restored around `start_battle`. |
| `0xdcd7` | `wPartyCount` | 1 | Party size, terminator-prefix list begins after. |
| `0xdcdf` + N·48 | `wPartyMon{N}` | 48 | `party_struct` slot. Filled by `try_add_mon_to_party`. |
| `0xd892` | `wNumItems` | 1+ | Bag pocket count; slot 1 = `0xff` terminator. |
| `0xd8bc` | `wNumKeyItems` | 1+ | Key items pocket count + terminator. |
| `0xd8d7` | `wNumBalls` | 1+ | Balls pocket count + terminator. |
| `0xd8f1` | `wNumPCItems` | 1+ | PC items count + terminator. |

## Party struct (48 bytes, aka `PARTYMON_STRUCT_LENGTH`)

Defined by the `party_struct` macro in `scratch/pokecrystal/macros/ram.asm`. Layout per slot:

```
+0   Species   (1)
+1   Item      (1)
+2   Moves     (4 = NUM_MOVES)
+6   ID        (2)
+8   Exp       (3)
+11  HPExp     (2)
+13  AtkExp    (2)
+15  DefExp    (2)
+17  SpdExp    (2)
+19  SpcExp    (2)
+21  DVs       (2)
+23  PP        (4 = NUM_MOVES)
+27  Happiness (1)
+28  PokerusStatus (1)
+29  CaughtData    (1) — also aliased CaughtTime/CaughtLevel
+30  CaughtLocation (1) — also aliased CaughtGender
+31  Level     (1)
+32  Status    (1)
+33  Unused    (1)
+34  HP        (2)
+36  MaxHP     (2)
+38  Stats: Atk/Def/Spd/SpcAtk/SpcDef, each (2) — big-endian
```

We don't fill these by hand; `try_add_mon_to_party` + `GeneratePartyMonStats` does it.

## ROM bank summary (functions we call directly)

| Bank | Function | Purpose |
| --- | --- | --- |
| `0x03` | `TryAddMonToParty` / `GeneratePartyMonStats` | Build a `party_struct` slot from `wCurPartySpecies` + `wCurPartyLevel`. |
| `0x0f` | `StartBattle` / `BattleIntro` / `BattleStartMessage` | The wild/trainer battle entry. |

Look up new functions in `scratch/pokecrystal/pokecrystal.sym`. The bank prefix in each line (`0f:74dd BattleIntro`) is what you write to `hROMBank` and the MBC.
