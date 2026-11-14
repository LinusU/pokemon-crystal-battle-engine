# Memory layout cheat sheet

The ground truth for symbol addresses is `scratch/pokecrystal/pokecrystal.sym`, which lists every label and the bank/address it resolves to. This page lists only the WRAM regions the battle harness reads or writes today.

## WRAM addresses touched by `bootstrap`

| Address | Symbol | Bytes | Purpose |
| --- | --- | --- | --- |
| `0xc2dc` | `wLinkMode` | 1 | 0 = not a link battle. We force this to 0. |
| `0xcf5f` | `wMonType` | 1 | Whose mon are we operating on. Low nibble 0 = PARTYMON. |
| `0xcf60` | `wCurSpecies` | 1 | Scratch species id; many engine routines read this. |
| `0xcfc0` | `wInBattleTowerBattle` | 1 | Battle-Tower flag; 0 disables the special path. |
| `0xcfcc` | `wOptions` | 1 | Game options. With `[battle] battle_scene = false` we set bit 7 (`BATTLE_SCENE`, set = animations off). |
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
| `0xd857` | `wJohtoBadges` | 1 | Badge bitfield from `[battle] johto_badges` (default 0). Read by `DoBadgeTypeBoosts`. |
| `0xd858` | `wKantoBadges` | 1 | Badge bitfield from `[battle] kanto_badges` (default 0). |
| `0xdcd7` | `wPartyCount` | 1 | Party size, terminator-prefix list begins after. |
| `0xdcdf` + N·48 | `wPartyMon{N}` | 48 | `party_struct` slot. Filled by `try_add_mon_to_party`. |

### Link-battle seeding (`launch_link_battle`)

A `link_battle` additionally seeds the enemy party and switches the engine onto its real link path (see [bootstrap.md](bootstrap.md#link-battles)). These are the extra bytes it writes:

| Address | Symbol | Bytes | Purpose |
| --- | --- | --- | --- |
| `0xc2dc` | `wLinkMode` | 1 | Set to `LINK_COLOSSEUM` (3) — engine takes the link path: no AI, no ROM party load, enemy action from `wBattleAction`. |
| `0xffcb` | `hSerialConnectionStatus` | 1 | `USING_EXTERNAL_CLOCK` (1) → local side is player 1, so `DoBattle` sends out the first enemy mon itself. |
| `0xd280` | `wOTPartyCount` | 1 | Enemy party size; set by `try_add_mon_to_party` as each enemy mon is added (`wMonType = OTPARTYMON`). |
| `0xd288` + N·48 | `wOTPartyMon{N}` | 48 | Enemy `party_struct` slots, seeded from `[[enemy_party]]` (same path as the player party). |
| `0xc663` | `wCurOTMon` | 1 | Active enemy slot, zeroed (the ROM load that normally does this is skipped). |
| `0xd109` | `wCurPartyMon` | 1 | Scratch slot `InitEnemyMon` copies into `wCurOTMon`; zeroed for the same reason. |
| `0xc656` | `wOTClassName` | 11 | Opponent class name, seeded with a terminated string so battle-start text stays bounded (it's only 13 bytes below `wCurOTMon`). |
| `0xd26b` | `wOTPlayerName` | 11 | Opponent player name, seeded for the same reason. |
| `0xd22f` | `wOtherTrainerClass` | 1 | `CAL` (12) — routes `InitEnemy` to the multi-mon trainer path, as the real colosseum battle does. |

The link path's enemy action lives in `wBattleAction` (`0xd430`); the `link_battle_send_receive_action` hook fills it from the CLI (see the in-battle table — `cli.rs` reads/writes it). RNG would otherwise come from `wLinkBattleRNs` (`0xd1fa`, 10 bytes), but the `_battle_random` patch keeps link battles on the seeded LCG instead.

## In-battle WRAM (read or written by the CLI driver)

The CLI driver in [`src/bootstrap/cli.rs`](../src/bootstrap/cli.rs) reads battle state out of these addresses and writes the engine's "post-menu" state back into them when the player picks an action. The patches in [`src/game/engine/battle/core.rs`](../src/game/engine/battle/core.rs) (`battle_menu`, `move_selection_screen`, `force_pick_party_mon_in_battle`, `ask_use_next_pokemon`, `offer_switch`) target these same fields.

| Address | Symbol | Bytes | Purpose |
| --- | --- | --- | --- |
| `0xc62c` | `wBattleMon` (struct start) | 32 | Active player mon: species, item, moves, PP, level, status, HP, MaxHP, stats, types. |
| `0xc63c` | `wBattleMonHP` | 2 | Current HP of active player mon (big-endian). |
| `0xc63e` | `wBattleMonMaxHP` | 2 | Max HP (big-endian). |
| `0xc640` | `wBattleMonStats` (`wBattleMonAttack`…) | 10 | Active player mon BASE (stage-neutral) battle stats: 5 big-endian words Atk/Def/Spd/SpA/SpD. The multiplicand an X-item stat recompute starts from. |
| `0xc66a` | `wPlayerSubStatus3` | 1 | Active player mon battle-only sub-statuses. Bit 7 `SUBSTATUS_CONFUSED`: gates the Bitter Berry / MiracleBerry confusion cure (cleared on use). |
| `0xc6b6` | `wPlayerStats` (`wPlayerAttack`…) | 10 | Active player mon COMPUTED, stage-adjusted battle stats (same 5-word layout). What the damage routine reads — an X-item must rewrite this, not just the stat level, to affect damage. |
| `0xc6cc` | `wPlayerStatLevels` (`wPlayerAtkLevel`…) | 8 | Per-stat stage levels (ATTACK=0…ABILITY=7); each an absolute level 1..=13, 7 neutral, 13 cap. An X-item raises one by 1 (capped at 13). |
| `0xc671` | `wEnemySubStatus5` | 1 | Bit `SUBSTATUS_CANT_RUN` blocks switching (Mean Look / Spider Web). |
| `0xc6e3` | `wCurPlayerMove` | 1 | The move id the player picked this turn. Zeroed by the `offer_switch` patch on Accept (mirrors the asm) so `PursuitSwitch` can't read a stale value during the new mon's entrance. |
| `0xc6e4` | `wCurEnemyMove` | 1 | The move id the enemy picked this turn. Same zero-on-Accept story as `wCurPlayerMove`. |
| `0xc71a` | `wLastPlayerMon` | 1 | Party slot of the *outgoing* mon when a switch happens. |
| `0xc730` | `wPlayerWrapCount` | 1 | Non-zero = Wrap/Whirlpool/Bind in progress; player can't switch. |
| `0xcfa9` | `wMenuCursorY` | 1 | Generic cursor; the move-select path uses it as a 0-based slot index. |
| `0xd0d2` | `wBattleMenuCursorPosition` | 1 | 1=Fight, 2=Pkmn, 3=Pack, 4=Run. Read by `battle_menu_next` to dispatch. |
| `0xd0d4` | `wCurBattleMon` | 1 | Party slot of the *active* player mon. |
| `0xd0d5` | `wCurMoveNum` | 1 | The slot (0..3) of the chosen move. |
| `0xd0ec` | `wBattlePlayerAction` | 1 | 0=`USEMOVE`, 1=`USEITEM`, 2=`SWITCH`. |
| `0xd106` | `wCurItem` | 1 | Selected item id (for engine paths we hand off to). |
| `0xd109` | `wCurPartyMon` | 1 | Selected party slot (switch target / item target). |
| `0xd10c` | `wItemQuantityChange` | 1 | Engine bag bookkeeping; we keep it consistent. |
| `0xd206` | `wEnemyMon` (struct start) | — | Same shape as `wBattleMon` for the opposing mon. |
| `0xd216` | `wEnemyMonHP` | 2 | Big-endian. |
| `0xd892` | `wNumItems` | 1+ | Bag pocket count; slot 1 = `0xff` terminator. |
| `0xd8bc` | `wNumKeyItems` | 1+ | Key items pocket count + terminator. |
| `0xd8d7` | `wNumBalls` | 1+ | Balls pocket count + terminator. |
| `0xd8f1` | `wNumPCItems` | 1+ | PC items count + terminator. |

## In-battle state read by the snapshot reader

[`src/bootstrap/snapshot.rs`](../src/bootstrap/snapshot.rs) observes a comprehensive slice of battle state at each decision point and at battle end, for the recording's `expect.*` assertions (see [TOML schema](toml_schema.md#recording--assertions)). It is a pure reader — it never writes WRAM. In addition to the active-mon `wBattleMon` / `wEnemyMon` fields above, it reads:

| Address | Symbol | Bytes | Purpose |
| --- | --- | --- | --- |
| `0xc62c` | `wBattleMonSpecies` | 1 | Active player mon species id. |
| `0xc62d` | `wBattleMonItem` | 1 | Active player mon held item id (`battle_struct` +1). Decoded to a name for the `expect.player.active.item` assertion. |
| `0xc634` | `wBattleMonPP` | 4 | Active player mon PP, one per move slot. Low 6 bits = current PP; top 2 = PP Ups (masked off). |
| `0xc639` | `wBattleMonLevel` | 1 | Active player mon level. |
| `0xc63a` | `wBattleMonStatus` | 1 | Status byte. Bits 0–2 = sleep counter; bit 3 PSN, 4 BRN, 5 FRZ, 6 PAR, 7 TOX. |
| `0xc663` | `wCurOTMon` | 1 | Party slot of the *active* enemy mon. |
| `0xd206` | `wEnemyMonSpecies` | 1 | Active enemy mon species id. |
| `0xd207` | `wEnemyMonItem` | 1 | Active enemy mon held item id (`battle_struct` +1). Decoded for the `expect.enemy.active.item` assertion. |
| `0xd20e` | `wEnemyMonPP` | 4 | Active enemy mon PP (not asserted — invisible to the player). |
| `0xd213` | `wEnemyMonLevel` | 1 | Active enemy mon level. |
| `0xd214` | `wEnemyMonStatus` | 1 | Enemy status byte (same bit layout as player). |
| `0xd218` | `wEnemyMonMaxHP` | 2 | Enemy max HP (big-endian). |
| `0xd280` | `wOTPartyCount` | 1 | Enemy trainer party size. |
| `0xd288` + N·48 | `wOTPartyMon{N}` | 48 | Enemy `party_struct` slots — same layout as `wPartyMon{N}`. |
| `0xd0ee` | `wBattleResult` | 1 | Battle outcome. Low nibble: 0 = win, 1 = loss, else draw (`BATTLERESULT_BITMASK = 0xc0` carries CAUGHT_CELEBI / BOX_FULL, ignored). |
| `0xd232` | `wForcedSwitch` | 1 | Non-zero ends the turn loop (`battle_turn_loop1` quits when set). `PokeDollEffect` sets it to 1 to end a wild battle as an escape (a DRAW); the CLI Poké Doll path replicates that. |

`wCurBattleMon` (`0xd0d4`, player active slot) is listed in the in-battle table above. Per-slot party reads use the `party_struct` offsets below (species `+0`, item `+1`, level `+31`, status `+32`, HP `+34`, MaxHP `+36`) against both `wPartyMon1` (`0xdcdf`) and `wOTPartyMon1` (`0xd288`). The harness also *writes* item `+1` when seeding a `[[player_party]]` / link `[[enemy_party]]` mon with an `item = "..."` override.

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
