# TOML schema

The harness reads a battle scenario from a TOML file. Today three `[battle] type` values are accepted: `"wild_encounter"`, `"trainer_battle"`, and `"link_battle"` (see [Link battles](#link-battles)).

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
moves = ["Flamethrower", "Quick Attack", "Swift", "Dig"]

[[player_party]]
species = "Geodude"
level = 18

[[player_items]]
name = "Potion"
quantity = 3
```

The optional `moves` array overrides a player mon's generated moveset (see [`[[player_party]]`](#player_party--one-or-more) below). Omit it — as on Geodude above — to keep the engine's level-up moveset.

For trainer battles the enemy party and the trainer's AI items can be overridden from the TOML. See [`[[enemy_party]]`](#enemy_party), [`[[enemy_items]]`](#enemy_items--trainer-battles-only) and [`[enemy_trainer]`](#enemy_trainer--trainer-battles-only) below. Example with all three override knobs:

```toml
[battle]
type = "trainer_battle"
trainer = "Whitney"

[[player_party]]
species = "Machop"
level = 25

# Override the enemy party (replaces Whitney's canonical Clefairy/Miltank)
[[enemy_party]]
species = "Caterpie"
level = 10
moves = ["Tackle", "String Shot"]

[[enemy_party]]
species = "Metapod"
level = 10

# Override the trainer's two AI item slots
[[enemy_items]]
name = "Super Potion"
quantity = 2

# Override AI behaviour flags
[enemy_trainer]
ai_move_flags = ["AI_BASIC", "AI_SMART"]
ai_item_flags = ["ALWAYS_USE", "CONTEXT_USE"]
```

## Link battles

```toml
[battle]
type = "link_battle"
seed = 0x1234

[[player_party]]
species = "Pikachu"
level = 30
moves = ["Thunderbolt", "Quick Attack", "Surf", "Thunder"]
item = "Light Ball"

[[enemy_party]]
species = "Geodude"
level = 30
moves = ["Rock Throw", "Magnitude", "Tackle", "Defense Curl"]
item = "Leftovers"
```

A `link_battle` is the engine's real two-player link mode driven entirely from the terminal: **both** sides are CLI-controlled, there is no AI, and **both parties are taken from the TOML** — unlike a trainer battle, nothing is loaded from ROM. So `[[enemy_party]]` is honored in full here (1..=6 mons, with `moves` and `item` overrides), exactly like `[[player_party]]` — held items work for both the player and the opponent. The harness seeds `wParty*` and `wOTParty*`, sets `wLinkMode` so the engine skips the AI and the ROM party load and reads the enemy's action from `wBattleAction`, and fills that byte from the CLI via the `link_battle_send_receive_action` hook (see [transpiled_code.md](transpiled_code.md)).

Each round prompts the **player** first (the normal action menu) and then the **enemy** (a parallel menu of the enemy mon's usable moves + its alive bench). When a side's mon faints it's prompted for a replacement. The bag is disabled in a link battle (as in the real game), so neither side is offered items, balls, or — for the enemy — run; `[[player_items]]` / `[[enemy_items]]` are rejected. `--record` / `--play` work as for any scenario; enemy decisions are written with `side = "enemy"` (see [Recording](#recording----record-path)). Like every recording, a link-battle `--record` embeds the full matchup, so a link fixture is a single self-contained file — no sibling scenario TOML is needed to replay it. Both parties are required (1..=6 each).

## Fields

### `[battle]`

| Field | Type | Required | Notes |
| --- | --- | --- | --- |
| `type` | string | yes | `"wild_encounter"`, `"trainer_battle"`, or `"link_battle"`. |
| `trainer` | string | for `trainer_battle` | Trainer name. Case- and punctuation-insensitive. Resolved via [`src/bootstrap/trainers.rs`](../src/bootstrap/trainers.rs): every battleable trainer instance in the game is addressable by a `CLASS_id` slug (e.g. `"YOUNGSTER_1"`, `"lt_surge 1"`), and the unique trainers also accept friendly aliases (`"Whitney"`, `"Lance"`, `"Red"`, `"Blue"`, `"Eusine"`, …). |
| `seed` | integer | no | 64-bit seed for the deterministic battle RNG. Accepts decimal or hex (`0xdeadbeef`). If omitted, the harness picks one from system entropy and logs it on stderr so the same run can be replayed. See [`src/runtime/cpu.rs`](../src/runtime/cpu.rs) (`Cpu::seed_rng`) and the `HARNESS-PATCH` in [`src/game/home/random.rs`](../src/game/home/random.rs). |
| `johto_badges` | integer | no | Badge bitfield written to `wJohtoBadges` (`0xd857`); `0xff` = all eight. Defaults to 0, so scenarios and recordings without the key behave as before. Badges feed the engine's badge damage boosts (`DoBadgeTypeBoosts` in [`misc.rs`](../src/game/engine/battle/misc.rs): +12.5% to the player's same-type moves per matching badge). |
| `kanto_badges` | integer | no | Badge bitfield written to `wKantoBadges` (`0xd858`). Defaults to 0. |

The battle is always driven from the terminal — see [`src/bootstrap/cli.rs`](../src/bootstrap/cli.rs). All in-battle menus (action / move / forced switch / "switch out enemy mon" prompt) become numbered lists on stdin; "press A to continue" waits are auto-advanced. The glium window still renders. Every item the original game lets you use during a battle is supported: HP heals (Potions, drinks, Moomoo Milk, herbs, HP berries), status heals (Antidote … Full Heal and the cure-berries), Full Restore, revives (Revive / Max Revive / Revival Herb), PP restores (Ether/Max Ether/Elixer/Max Elixer for the whole party, plus the single-move Ether/MysteryBerry which take an extra `move` slot), the in-battle stat boosters (X Attack/Defend/Speed/Special, X Accuracy, Dire Hit, Guard Spec), and confusion cure (Bitter Berry / Miracleberry). Items are only offered when they'd actually have an effect, mirroring the engine's "It wouldn't have any effect" guard. In a wild battle the action menu also lists each Poké Ball in the Balls pocket as a "Throw …" entry, recorded as an `item` step. Throwing one drives the engine's real catch routine: on a successful catch the wild mon is added to the party and the battle ends with `outcome = "caught"`; on a failed throw the ball is consumed and the turn passes to the enemy. Balls are never offered in a trainer battle. A Poké Doll in the Items pocket is likewise offered only in a wild battle as a "Use … (escape)" entry (also recorded as an `item` step): it ends the battle immediately as `outcome = "draw"` (the engine's `PokeDollEffect` sets `wForcedSwitch`, which the turn loop quits on). It's a no-op in a trainer battle, so it's never offered there.

## Random battles — `--random` (and `--seed <n>`)

Skip authoring a TOML and start a fresh randomized trainer battle:

```sh
cargo run -- --random                       # entropy seed, logged to stderr
cargo run -- --random --seed 0xdead         # deterministic, hex or decimal
cargo run -- --random --record /tmp/r.toml  # combine with recording
```

`--random` picks one trainer uniformly from the **entire game roster** — every battleable trainer instance, generated into [`src/bootstrap/trainers_data.rs`](../src/bootstrap/trainers_data.rs) from `parties.asm` (see [`all`](../src/bootstrap/trainers.rs)) — and generates a player party scaled to that trainer's canonical level / count (party size ±1 clamped to 1..=6, levels ±10% clamped to 2..=100). Each generated player mon has a small (~15%) chance to walk in holding an item from a curated `HELD_ITEMS` list (Leftovers, type boosters, Quick Claw, King's Rock, … — see [`src/bootstrap/random.rs`](../src/bootstrap/random.rs)). The bag gets 2..=4 entries drawn from a curated `BATTLE_ITEMS` list (potions, status heals, X-items) at 1..=3 each. The chosen matchup — held items included — is logged on stderr.

Pass `--seed <n>` to make the run reproducible (accepts decimal `42` or hex `0xdeadbeef`); otherwise the harness draws a base seed from system entropy and logs it. Two invocations with the same `--seed` produce identical matchups and identical battle outcomes for the same choices.

The base seed is split into **two independent seeds**: one drives matchup generation (trainer / party / items), the other drives the battle engine RNG (damage, crits, AI). They are never the same number, so the generator and the engine never share RNG state — changing how matchups are generated cannot perturb a battle's dynamics, and vice versa. This split also fixes a selection bias: feeding small or sequential seeds straight into the generator used to make trainer selection cluster onto a handful of trainers; mixing the base seed first spreads it across the whole roster. See [`src/bootstrap/random.rs`](../src/bootstrap/random.rs).

Mutual exclusions:

- `--random` cannot be combined with a positional `scenario.toml` (you'd be saying "scenario X and also random" — error).
- `--random` cannot be combined with `--play` (a recorded random run is replayed by passing the recording to `--play` *alone*; the recording carries the resolved matchup, so no regeneration is needed).
- `--seed` without `--random` is rejected — for scenario TOMLs the seed belongs in `[battle] seed = ...`.

As with every `--record` run, the recorder embeds the *resolved* matchup in the recording as a `[scenario.*]` block (see [Recording](#recording----record-path)). Because a generated run has no scenario file at all, this block is the *only* record of the matchup: `--play` reconstructs the battle from it and never re-runs the generator, so the generator can change freely without invalidating the recording. No TOML on disk is required.

## Recording — `--record <path>`

Pass `--record <out.toml>` on the command line and the harness appends every player decision to that file as it happens:

```sh
cargo run -- scratch/cli.toml --record /tmp/run.toml
```

The recording is a separate TOML document — not the scenario schema. It starts with a `[recording]` header and then one `[[step]]` table per decision the CLI driver resolves. Each step is flushed to disk immediately, so a mid-battle crash still leaves a usable script.

Every recording embeds a `[scenario.*]` block right after the header — it mirrors the standalone scenario schema one level down (`[scenario.battle]`, `[[scenario.player_party]]`, `[[scenario.player_items]]`, …) and holds the *resolved* matchup. This makes a recording self-contained for **every** battle kind: `--play` reconstructs the battle from the recording alone — no sibling scenario file is needed, and a generated (`--random`) run is never re-run through the generator. The engine seed stays in `[recording] seed`, so it is intentionally omitted from the embedded `[scenario.battle]`:

```toml
[recording]
seed = "0x48416938ef0dcf6f"      # the engine seed

[scenario.battle]
type = "trainer_battle"
trainer = "Karen"

[[scenario.player_party]]
species = "ARBOK"
level = 39

[[scenario.player_items]]
name = "Fresh Water"
quantity = 1
```

Each `[[step]]` table has two halves:

1. The **action** — `kind` plus its fields — written the instant the CLI resolves the decision.
2. An **expectation** block (`expect.*` keys) — the battle state that action *produced*, captured at the next decision point. This is the exact state the next step takes its action at; the final step's expectation is captured after the battle ends and also carries the battle `outcome`. See [Recording — assertions](#recording--assertions) below.

The embedded `[scenario.*]` block (shown above, omitted here for brevity) precedes the steps for every recording, file-driven or generated:

```toml
[recording]
seed = "0x0000000000c0ffee"            # always present; the resolved seed

[[step]]
kind = "move"
slot = 0
expect.player.active_slot = 0
expect.player.active.species = "QUILAVA"
expect.player.active.level = 20
expect.player.active.hp = 48
expect.player.active.max_hp = 62
expect.player.active.status = "none"
expect.player.active.pp = [24, 14, 30, 0]
expect.player.party = [{ species = "QUILAVA", level = 20, hp = 48, max_hp = 62, status = "none" }]
expect.enemy.active_slot = 0
expect.enemy.active.species = "CLEFAIRY"
expect.enemy.active.level = 18
expect.enemy.active.hp = 41
expect.enemy.active.max_hp = 55
expect.enemy.active.status = "paralysis"
expect.enemy.active.pp = [0, 0, 0, 0]
expect.enemy.party = [{ species = "CLEFAIRY", level = 18, hp = 41, max_hp = 55, status = "paralysis" }]

[[step]]
kind = "item"
id = 18
target = 0
# … expect block …

[[step]]
kind = "move"
slot = 2
# … expect block, plus the final outcome …
expect.battle.outcome = "win"
```

The `seed` field is always the *resolved* value: if the scenario omitted `seed = ...`, the harness draws one from system entropy and writes that resolved value into the recording so a later replay reproduces the run bit-for-bit.

`kind` is one of: `move`, `switch`, `item`, `run`, `forced_switch`, `offer_switch`. For `offer_switch`, the keep-current case writes `keep = true`; the swap-in case writes `slot = N` instead.

In a [link battle](#link-battles) both sides' decisions go into the one ordered `[[step]]` list, interleaved in the order the engine prompts them (player, then enemy, each round). The enemy's steps carry `side = "enemy"`; player steps omit the key. The enemy only ever produces `move` and `switch` steps. On playback each prompt site consumes a step of its own side in order — a step whose `side` doesn't fit the current prompt is structural drift (exit 2), the same as a `kind` mismatch. Recordings made before link battles existed have no `side` key and parse as `player`, so they replay unchanged.

```toml
[[step]]              # player's turn
kind = "move"
slot = 0
# … expect block …

[[step]]              # enemy's turn (the second player)
side = "enemy"
kind = "move"
slot = 2
# … expect block …
```

An `item` step carries `id = N` and, for items that target one party mon, `target = N` (0-based party slot). Single-move PP-restore items — Ether (`id = 63`), Max Ether (`id = 64`), and MysteryBerry (`id = 150`) — additionally carry `move = N`, the 0-based move slot whose PP to restore. The `move` key is omitted for every other item (whole-party PP restore, HP, status, revive); recordings made before this key existed have no `move` key and parse back as "no move slot", so older fixtures replay unchanged.

```toml
[[step]]
kind = "item"
id = 63        # Ether
target = 0     # party slot 0
move = 1       # restore the PP of move slot 1
# … expect block …
```

### Recording — assertions

The `expect.*` keys make a recording a self-checking regression fixture: replaying it (see [Playback](#playback----play-path)) re-derives the same state and compares it field-by-field. The recorder fills every field; the diff is *tolerant*, so a hand-edited recording may keep only the fields it cares about and the rest are ignored.

The captured shape, per step, mirrors [`src/bootstrap/snapshot.rs`](../src/bootstrap/snapshot.rs):

| Key | Meaning |
| --- | --- |
| `expect.<side>.active_slot` | Party slot index of the mon currently out (`<side>` is `player` or `enemy`). |
| `expect.<side>.active.{species,level,hp,max_hp,status}` | The active mon's vitals. `status` is one of `none`, `sleep`, `poison`, `burn`, `freeze`, `paralysis`, `toxic`. |
| `expect.<side>.active.pp` | A 4-element array of current PP (PP Ups masked off). Enemy PP isn't visible to the player, so the enemy's reads as `[0,0,0,0]`. |
| `expect.<side>.active.item` | Held item display name (`"Leftovers"`, `"King's Rock"`, …), `"none"` when unequipped, or `"#id"` for an id with no harness name. Tracks the active mon's item, so a consumed berry surfaces as a change to `"none"`. |
| `expect.<side>.party` | An array of inline tables, one per party slot in order: `{ species, level, hp, max_hp, status, item }`. Both the player's and the enemy's full parties are captured, so benched-mon changes (status spikes, healing, held items) are asserted too. |
| `expect.battle.outcome` | `win` / `loss` / `draw` / `caught`. Written only on the final step (after the battle ends). `caught` is a wild-mon capture with a Poké Ball. |

HP/Max-HP are decoded from the big-endian `wBattleMon` / `wEnemyMon` / `party_struct` fields; `outcome` from the low nibble of `wBattleResult` — except `caught`, which the engine reports as a WIN and the harness tracks via a side-channel set on the CLI capture path. See [Memory layout](memory_layout.md#in-battle-state-read-by-the-snapshot-reader).

## Playback — `--play <path>`

Pass `--play <recording.toml>` and the harness drives the battle from a previous run's `[[step]]` list instead of stdin:

```sh
cargo run -- --play /tmp/run.toml
```

The recording's `[recording] seed` always wins — replay uses that value to seed the engine RNG even if the loaded scenario carries its own `seed = ...`. The matchup comes from the recording's embedded `[scenario.*]` block, which every recording carries. Pass a positional `scratch/other.toml` to override it — it's checked first, so the recorded script re-runs against the edited scenario. `--record` and `--play` can be combined to re-record an edited playback (the new recording embeds its own resolved matchup).

Because the matchup travels inside the recording, a `--play` is self-contained: a recorded `--random` run is reconstructed without re-running the generator, and a file-driven (including link-battle) recording needs no scenario TOML on disk. Either the generator or a scenario file can change later without invalidating an existing recording. A recording with no embedded block (e.g. a hand-written script) needs a positional scenario passed alongside `--play`.

Each CLI prompt site consumes one step in order. The match is strict on `kind`: if the recorded action doesn't fit the current prompt site, the harness prints a clear stderr message naming the step index, the recorded `kind`, the prompt site, and the accepted kinds, then exits with status 2. The same exit-2 path fires when the step list is exhausted before the battle ends — playback never silently substitutes.

**Assertions.** As the battle replays, each step's recorded `expect.*` block (see [Recording — assertions](#recording--assertions)) is checked against the live state at the *next* decision point — and the last step's at battle end. Every disagreeing field is collected; if any mismatched, the harness prints one line per mismatch (`step #N: <path>: expected <x>, got <y>`) and exits with status **3**. A clean replay prints `playback: N step(s) replayed; all expectations held` and exits 0. This is what makes a recorded corpus usable as a regression suite while the transpiled engine is cleaned up into hand-written Rust.

Exit codes: `0` clean, `2` structural drift (kind mismatch / underflow), `3` expectation mismatch.

See [`src/bootstrap/player.rs`](../src/bootstrap/player.rs) and [`src/bootstrap/snapshot.rs`](../src/bootstrap/snapshot.rs).

### `[[player_party]]` — one or more

| Field | Type | Required | Notes |
| --- | --- | --- | --- |
| `species` | string | yes | Case-insensitive; punctuation ignored. `"Pikachu"`, `"PIKACHU"`, `"Mr. Mime"`, `"Farfetch'd"`, `"Ho-oh"` all work. |
| `level` | integer | yes | 1..100. |
| `moves` | array of strings | no | 1..=4 move names. When omitted, the engine's generated level-up moveset is kept. When present, it overrides the generated moves for that slot. Names are case-insensitive with punctuation ignored, so `"Thunderbolt"`, `"THUNDER WAVE"`, `"thunder_wave"`, `"Double-Edge"` all resolve (see [`src/bootstrap/names.rs`](../src/bootstrap/names.rs)). |
| `item` | string | no | Held item name (`"Leftovers"`, `"King's Rock"`, `"leftovers"`, …). Case-insensitive, punctuation ignored. When set, the mon walks into battle holding the item, so the engine's held-item mechanics (Leftovers' end-of-turn heal, type boosters, Quick Claw, flinch/crit items, …) all apply. Resolved via [`src/bootstrap/items.rs`](../src/bootstrap/items.rs) — the same table the bag uses. Unknown names are a hard error. |

DVs, PP, HP and stats are produced by the engine itself (`try_add_mon_to_party`/`GeneratePartyMonStats`) — they're not user-controllable yet. `moves` and `item` are the exceptions, both written into the slot after generation: `moves` overwrites the four move ids and refills PP to each move's base maximum (no PP Ups — a fresh mon has none; fewer than four moves leaves the trailing slots empty, id 0 / 0 PP), and `item` writes the `MON_ITEM` byte so the mon holds it walking into battle. Listing more than four moves, an empty `moves` list, an unknown move name, or an unknown `item` name is a hard error. For `[[enemy_party]]`: `moves` and `item` are honored for `trainer_battle` when a party override is present (1..=6 entries) and for `link_battle` (always). For a `wild_encounter` and a `trainer_battle` with no override block, the wild mon / ROM party gets its moves and items from the engine, so `moves` and `item` there are parsed but ignored.

### `[[enemy_party]]`

For `wild_encounter`, exactly one entry is required. Same field set as `player_party`.

For `trainer_battle`, **optional** (0..=6 entries):
- When **absent** (0 entries), the engine loads the trainer's canonical party from ROM via `ReadTrainerParty` — back-compat, all existing fixtures behave as before.
- When **present** (1..=6 entries), the entries fully replace the trainer's canonical party: `species` + `level` are required; `moves` (1..=4) and `item` (held item) are optional, honored exactly as for the player party. The harness seeds `wOTParty*` and an early-return HARNESS-PATCH in `read_trainer_party` prevents the ROM load from clobbering it. The rest of `InitEnemyTrainer` (attributes load, AI init, HUD) still runs normally.

For `link_battle`, **required** (1..=6 entries) and fully honored, including `moves` and `item` overrides — the enemy party is seeded into `wOTParty*` from the TOML, not loaded from ROM, so the opponent's held items work too. See [Link battles](#link-battles).

### `[[player_items]]` — zero or more

| Field | Type | Required | Notes |
| --- | --- | --- | --- |
| `name` | string | yes | Case-insensitive; punctuation ignored. `"Potion"`, `"POTION"`, `"super_potion"`, `"King's Rock"` all work. |
| `quantity` | integer | yes | 1..=99 (`MAX_ITEM_STACK`). |

Items are sorted into the engine's Items pocket (cap 20) or Balls pocket (cap 12) based on the item's pocket attribute. Key items, TMs, HMs, and mail are not exposed by the harness today — see [`src/bootstrap/items.rs`](../src/bootstrap/items.rs) for the supported list.

### `[[enemy_items]]` — trainer battles only

Same field set as `[[player_items]]`. Rejected for `wild_encounter` and `link_battle` (the bag is disabled in a link battle). For `trainer_battle`:
- When **absent**, the trainer's AI item slots (`wEnemyTrainerItem1` / `wEnemyTrainerItem2` at 0xc650/0xc651) are filled from the canonical `TrainerClassAttributes` ROM entry.
- When **present**, the entries are expanded by `quantity` in order into at most 2 item slots (e.g. one entry with `quantity = 2` fills both slots; `quantity = 1` fills slot 1 only; a second entry fills slot 2). Total quantity > 2 is a hard error. The bootstrap pokes the trainer class's ROM entry via `Cpu::poke_rom` before `start_battle`; `GetTrainerAttributes` then reads and copies the overridden bytes into WRAM.

The item name must resolve via [`src/bootstrap/items.rs`](../src/bootstrap/items.rs) (same table as the player bag). Unknown names are a hard error.

### `[enemy_trainer]` — trainer battles only

Optional table for overriding the trainer's AI behaviour. Each field, when present, replaces the corresponding 16-bit word in the trainer class's `TrainerClassAttributes` ROM entry (via `Cpu::poke_rom`) before `start_battle`. The ROM overlay is invisible to the canonical `TrainerClassAttributes` data — only the current battle session is affected.

| Field | Type | Required | Notes |
| --- | --- | --- | --- |
| `ai_move_flags` | array of strings | no | Replaces `TRNATTR_AI_MOVE_WEIGHTS`. Each name is one of: `AI_BASIC`, `AI_SETUP`, `AI_TYPES`, `AI_OFFENSIVE`, `AI_SMART`, `AI_OPPORTUNIST`, `AI_AGGRESSIVE`, `AI_CAUTIOUS`, `AI_STATUS`, `AI_RISKY`. An empty list (`[]`) is valid and means "no move-score AI" (word = 0). Unknown flag names are a hard error. |
| `ai_item_flags` | array of strings | no | Replaces `TRNATTR_AI_ITEM_SWITCH`. Each name is one of: `SWITCH_OFTEN`, `SWITCH_RARELY`, `SWITCH_SOMETIMES`, `ALWAYS_USE`, `UNKNOWN_USE`, `CONTEXT_USE`. An empty list is valid (word = 0). Unknown flag names are a hard error. |

Flag names are case-sensitive and must match the spellings in `scratch/pokecrystal/constants/trainer_data_constants.asm` exactly. Both fields may be omitted — only present fields are overridden.

The `[enemy_trainer]` block is embedded in recordings as `[scenario.enemy_trainer]` and round-trips through `--record`/`--play` so the overrides are reproduced exactly on replay.

## Species name resolution

See [`src/bootstrap/species.rs`](../src/bootstrap/species.rs). The table is the first 251 species in pokédex order — same order as `scratch/pokecrystal/constants/pokemon_constants.asm`. The lookup is case-insensitive, ignores punctuation, and maps `♀`/`♂` to `F`/`M`.

## Planned additions

- `nickname`, `dvs = "..."` per mon (will short-circuit `try_add_mon_to_party` for those fields). `moves = [...]` and `item = "..."` are done — see [`[[player_party]]`](#player_party--one-or-more).
- `[battle] battle_type` for `BATTLETYPE_FISH`, `BATTLETYPE_DEBUG`, etc.
- `[player]` block: `name`, `id`, `gender` instead of the hardcoded "RED" + 0x1234.
- Non-battle item categories (key items, TMs/HMs) — every *battle-usable* item is already supported.
