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

## Random battles — `--random` (and `--seed <n>`)

Skip authoring a TOML and start a fresh randomized trainer battle:

```sh
cargo run -- --random                       # entropy seed, logged to stderr
cargo run -- --random --seed 0xdead         # deterministic, hex or decimal
cargo run -- --random --record /tmp/r.toml  # combine with recording
```

`--random` picks one trainer from [`src/bootstrap/trainers.rs`](../src/bootstrap/trainers.rs) and generates a player party scaled to that trainer's canonical level / count (party size ±1 clamped to 1..=6, levels ±10% clamped to 2..=100). The bag gets 2..=4 entries drawn from a curated `BATTLE_ITEMS` list (potions, status heals, X-items — see [`src/bootstrap/random.rs`](../src/bootstrap/random.rs)) at 1..=3 each. The chosen matchup is logged on stderr.

The generator and the battle RNG are both keyed off the same `u64` — pass `--seed <n>` to pin it (accepts decimal `42` or hex `0xdeadbeef`); otherwise the harness draws one from system entropy and logs it so the run can be reproduced. Two invocations with the same `--seed` produce identical matchups and identical battle outcomes for the same choices.

Mutual exclusions:

- `--random` cannot be combined with a positional `scenario.toml` (you'd be saying "scenario X and also random" — error).
- `--random` cannot be combined with `--play` (a recorded random run is replayed by passing the recording to `--play` *alone*; the playback path notices the `source_scenario = "random"` marker the recorder wrote and regenerates the scenario from the recording's seed).
- `--seed` without `--random` is rejected — for scenario TOMLs the seed belongs in `[battle] seed = ...`.

When `--random` is combined with `--record`, the recording's `[recording] source_scenario` is set to the literal string `"random"`. `--play` of such a recording re-generates the same matchup from the recording's seed; no TOML on disk is required.

## Recording — `--record <path>`

Pass `--record <out.toml>` on the command line and the harness appends every player decision to that file as it happens:

```sh
cargo run -- scratch/cli.toml --record /tmp/run.toml
```

The recording is a separate TOML document — not the scenario schema. It starts with a `[recording]` header and then one `[[step]]` table per decision the CLI driver resolves. Each step is flushed to disk immediately, so a mid-battle crash still leaves a usable script.

```toml
[recording]
source_scenario = "scratch/cli.toml"   # if the run was driven from a TOML file
seed = "0x0000000000c0ffee"            # always present; the resolved seed

[[step]]
kind = "move"
slot = 0

[[step]]
kind = "switch"
slot = 1

[[step]]
kind = "item"
id = 18
target = 0

[[step]]
kind = "forced_switch"
slot = 1

[[step]]
kind = "offer_switch"
keep = true
```

The `seed` field is always the *resolved* value: if the scenario omitted `seed = ...`, the harness draws one from system entropy and writes that resolved value into the recording so a later replay reproduces the run bit-for-bit.

`kind` is one of: `move`, `switch`, `item`, `run`, `forced_switch`, `offer_switch`. For `offer_switch`, the keep-current case writes `keep = true`; the swap-in case writes `slot = N` instead.

## Playback — `--play <path>`

Pass `--play <recording.toml>` and the harness drives the battle from a previous run's `[[step]]` list instead of stdin:

```sh
cargo run -- --play /tmp/run.toml
```

The recording's `[recording] seed` always wins — replay uses that value to seed the RNG even if the loaded scenario carries its own `seed = ...`. The scenario path comes from the recording's `source_scenario` header by default; pass a positional `scratch/other.toml` to override (useful for re-running the same script against an edited scenario). `--record` and `--play` can be combined to re-record an edited playback.

If the recording's `source_scenario` is the literal string `"random"`, the scenario is regenerated from the recording's seed via `bootstrap::random::random_battle_config` rather than loaded from disk — see [Random battles](#random-battles----random-and---seed-n).

Each CLI prompt site consumes one step in order. The match is strict on `kind`: if the recorded action doesn't fit the current prompt site, the harness prints a clear stderr message naming the step index, the recorded `kind`, the prompt site, and the accepted kinds, then exits with status 2. The same exit-2 path fires when the step list is exhausted before the battle ends — playback never silently substitutes.

See [`src/bootstrap/player.rs`](../src/bootstrap/player.rs).

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
