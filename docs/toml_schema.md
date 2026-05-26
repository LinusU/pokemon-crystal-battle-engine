# TOML schema

The harness reads a battle scenario from a TOML file. Today only wild encounters are supported; everything else is rejected.

## Example

```toml
[battle]
type = "wild_encounter"

[[player_party]]
species = "Pikachu"
level = 10

[[enemy_party]]
species = "Pidgey"
level = 7
```

## Fields

### `[battle]`

| Field | Type | Required | Notes |
| --- | --- | --- | --- |
| `type` | string | yes | Currently only `"wild_encounter"`. |

### `[[player_party]]` — one or more

| Field | Type | Required | Notes |
| --- | --- | --- | --- |
| `species` | string | yes | Case-insensitive; punctuation ignored. `"Pikachu"`, `"PIKACHU"`, `"Mr. Mime"`, `"Farfetch'd"`, `"Ho-oh"` all work. |
| `level` | integer | yes | 1..100. |

DVs, default moves, PP, HP and stats are produced by the engine itself (`try_add_mon_to_party`/`GeneratePartyMonStats`) — they're not user-controllable yet.

### `[[enemy_party]]`

For `wild_encounter`, exactly one entry is required. Same field set as `player_party`.

## Species name resolution

See [`src/bootstrap/species.rs`](../src/bootstrap/species.rs). The table is the first 251 species in pokédex order — same order as `scratch/pokecrystal/constants/pokemon_constants.asm`. The lookup is case-insensitive, ignores punctuation, and maps `♀`/`♂` to `F`/`M`.

## Planned additions

- `nickname`, `item`, `moves = [...]`, `dvs = "..."` per mon (will short-circuit `try_add_mon_to_party` for those fields).
- `[battle] battle_type` for `BATTLETYPE_FISH`, `BATTLETYPE_DEBUG`, etc.
- Trainer-class encounters (`type = "trainer_battle"`, `trainer_class = "...", trainer_id = ...`).
- `[player]` block: `name`, `id`, `gender` instead of the hardcoded "RED" + 0x1234.
