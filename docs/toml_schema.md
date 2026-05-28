# TOML schema

The harness reads a battle scenario from a TOML file. Today two `[battle] type` values are accepted: `"wild_encounter"` and `"trainer_battle"`.

## Example — wild encounter

```toml
[battle]
seed = 0xdeadbeef
type = "wild_encounter"

[[player_party]]
species = "Pikachu"
level = 10

[[enemy_party]]
species = "Pidgey"
level = 7

[[player_items]]
name = "Potion"
quantity = 2

[[player_items]]
name = "Super Potion"
quantity = 1
```

## Example — trainer battle

```toml
[battle]
type = "trainer_battle"
trainer = "Whitney"

[[player_party]]
species = "Quilava"
level = 20

[[player_party]]
species = "Geodude"
level = 18

[[player_items]]
name = "Potion"
quantity = 3
```

For trainer battles the enemy party and the trainer's AI items are loaded from ROM via `wOtherTrainerClass` / `wOtherTrainerID`. `[[enemy_party]]` and `[[enemy_items]]` entries are parsed (so scenarios can document the matchup) but the engine still uses the canonical ROM data — overriding them is on the [roadmap](roadmap.md).

## Fields

### `[battle]`

| Field | Type | Required | Notes |
| --- | --- | --- | --- |
| `type` | string | yes | `"wild_encounter"` or `"trainer_battle"`. |
| `trainer` | string | for `trainer_battle` | Trainer display name. Case- and punctuation-insensitive. Resolved via [`src/bootstrap/trainers.rs`](../src/bootstrap/trainers.rs) — today the 8 Johto + 8 Kanto gym leaders, Elite Four, Lance, Red, Blue, and Eusine. |
| `seed` | integer | no | 64-bit seed for the deterministic battle RNG. Accepts decimal or hex (`0xdeadbeef`). If omitted, the harness picks one from system entropy and logs it on stderr so the same run can be replayed. See [`src/runtime/cpu.rs`](../src/runtime/cpu.rs) (`Cpu::seed_rng`) and the `HARNESS-PATCH` in [`src/game/home/random.rs`](../src/game/home/random.rs). |

The battle is always driven from the terminal — see [`src/bootstrap/cli.rs`](../src/bootstrap/cli.rs). All in-battle menus (action / move / forced switch / "switch out enemy mon" prompt) become numbered lists on stdin; "press A to continue" waits are auto-advanced. The glium window still renders. Items currently support Potion-class (healing, status heals, revives) only — balls and X-items are not yet wired through.

### `[[player_party]]` — one or more

| Field | Type | Required | Notes |
| --- | --- | --- | --- |
| `species` | string | yes | Case-insensitive; punctuation ignored. `"Pikachu"`, `"PIKACHU"`, `"Mr. Mime"`, `"Farfetch'd"`, `"Ho-oh"` all work. |
| `level` | integer | yes | 1..100. |

DVs, default moves, PP, HP and stats are produced by the engine itself (`try_add_mon_to_party`/`GeneratePartyMonStats`) — they're not user-controllable yet.

### `[[enemy_party]]`

For `wild_encounter`, exactly one entry is required. Same field set as `player_party`.

For `trainer_battle`, optional and currently informational only — the engine loads the trainer's canonical party (species, levels, moves, items) from ROM via `ReadTrainerParty`.

### `[[player_items]]` — zero or more

| Field | Type | Required | Notes |
| --- | --- | --- | --- |
| `name` | string | yes | Case-insensitive; punctuation ignored. `"Potion"`, `"POTION"`, `"super_potion"`, `"King's Rock"` all work. |
| `quantity` | integer | yes | 1..=99 (`MAX_ITEM_STACK`). |

Items are sorted into the engine's Items pocket (cap 20) or Balls pocket (cap 12) based on the item's pocket attribute. Key items, TMs, HMs, and mail are not exposed by the harness today — see [`src/bootstrap/items.rs`](../src/bootstrap/items.rs) for the supported list.

### `[[enemy_items]]` — trainer battles only, currently informational

Same field set as `[[player_items]]`. Rejected for `wild_encounter`. For `trainer_battle`, parsed but ignored — the trainer's AI item slots (`wEnemyTrainerItem1` / `Item2`) come from `TrainerClassAttributes` in ROM. Wiring this up to override the ROM values is on the [roadmap](roadmap.md).

## Species name resolution

See [`src/bootstrap/species.rs`](../src/bootstrap/species.rs). The table is the first 251 species in pokédex order — same order as `scratch/pokecrystal/constants/pokemon_constants.asm`. The lookup is case-insensitive, ignores punctuation, and maps `♀`/`♂` to `F`/`M`.

## Planned additions

- `nickname`, `item`, `moves = [...]`, `dvs = "..."` per mon (will short-circuit `try_add_mon_to_party` for those fields).
- `[battle] battle_type` for `BATTLETYPE_FISH`, `BATTLETYPE_DEBUG`, etc.
- Honor `[[enemy_party]]` and `[[enemy_items]]` for trainer battles (override the ROM data instead of just documenting the matchup).
- `[player]` block: `name`, `id`, `gender` instead of the hardcoded "RED" + 0x1234.
- More item categories (key items, TMs/HMs).
