# Phase 5: a trained battle AI

Goal: replace Crystal's trainer AI with one that plays well — using
everything the original's AI can (moves, switching, the trainer's items,
picking the next mon after a faint) — available as a library (`ai` cargo
feature) and as an opponent in the CLI and the `gui` window.

## How the AI plays the trainer

`[battle] enemy_control = "external"` (trainer battles) hands the trainer
AI's choices to `Enemy`-side decisions ([TOML schema](toml_schema.md#external-enemy-trainer)):

| Original routine | Replaced by | Legal actions |
| --- | --- | --- |
| `AIChooseMove` (turn start) | `EnemyTrainerAction` | usable moves (or `fight` when Encored / out of PP), switches unless trapped, the trainer's AI items |
| `AI_SwitchOrTryItem` | carries out that switch / item at the same point | — |
| `FindMonInOTPartyToSwitchIntoBattle` | `EnemySwitchIn` | every other alive mon |

Both engines implement the mode (HARNESS-PATCHes in the transpiled original,
[transpiled_code.md](transpiled_code.md)), and `engine_diff external` checks
them against each other. 20,000 random battles (seeds 100000–119999, all 251
moves, random AI items, random policy on both sides): **19,839 compared
exactly, 0 divergences**. Skipped: 122 the original freezes on or runs past
the tester's budget, and 39 against Cal (`CAL_2`), whose Mystery Gift party
lives in SRAM — the engine refuses to start it, and the original either
refuses too or reads a garbage party and crashes.

The original's AI-only rules (items only on its highest-level mon, the class's
switch / item flags, its random rolls) are not applied: the network decides.

## Strategy: why PPO self-play with a feed-forward network, not an RNN

Research summary:

- The strongest learned agents on Pokémon Showdown use **sequence models**:
  [Metamon](https://arxiv.org/abs/2504.04395) (offline RL on reconstructed
  human replays, Transformers, top 10% of human players in gens 1–4) and its
  follow-up the PokéAgent Challenge. They need memory because Showdown hides
  the opponent's moves, items, stats and team; the history *is* the state.
- **PPO self-play** with a feed-forward actor-critic works well when the state
  is observable: Huang & Lee (2019) reached ~72% GXE on Gen 7 random battles
  without search; Wang (MIT, 2024) added test-time MCTS on top of a PPO
  actor-critic for ~79.5% GXE on Gen 4.

Our situation differs from Showdown in the way that matters: **the AI sees
the battle state**, exactly as the original trainer AI does (which reads the
player's moves and stats straight from WRAM). `BattleState` is the
original's whole battle memory at the decision point — every counter, stat
stage, sub-status bit, both parties — so the state is Markov and a recurrent
network has nothing to remember. An RNN/Transformer would cost much more per
step and add BPTT complexity for no information gain. Hence:

- **Algorithm**: PPO (clipped, GAE λ=0.95, γ=0.995, entropy bonus), self-play.
- **Network**: MLP trunk (sparse input → 256 → 256, ReLU) with a value head and
  an **action-scoring head**: every legal action is encoded as its own feature
  vector and scored against the state embedding, then softmaxed over the
  legal list. This handles every decision site and any number of actions
  (four moves, up to five switches, each bag item × target × move slot) with
  one network, and masking is free.
- **Implementation**: plain Rust, no dependencies — the forward pass, the
  backward pass (checked against finite differences in the tests) and Adam
  are ~400 lines in [`crates/crystal-battle/src/ai/net.rs`](../crates/crystal-battle/src/ai/net.rs). Training runs on all
  cores with the hand-written engine (`FastBattle`, no text): an iteration
  (~17,000 decisions over ~850 battles, then 3 PPO epochs) takes ~1 second
  on an 11-core laptop.

If we later want to restrict the AI to what a human sees (hidden moves and
items), that is when a recurrent or attention-based encoder over the event
history earns its keep; the action-scoring head and PPO loop stay as they are.
Test-time search (cloning the battle with *re-randomised* RNG — cloning the
real stream would read the future) is the other natural next step.

## Features ([`crates/crystal-battle/src/ai/features.rs`](../crates/crystal-battle/src/ai/features.rs))

From the deciding side's point of view ("me" vs "foe"), so one network plays
both the player and the trainer:

- **Global**: the prompt site, battle mode, weather.
- **Per side**: the active mon (species, HP, level, in-battle stats, types,
  status, held item, stat stages, all 40 sub-status bits, every counter,
  screens) and its four moves (move id, effect id, type, power, accuracy, PP,
  disabled, and the matchup against the other active mon: type
  effectiveness, STAB, an estimated damage fraction); each party slot (HP,
  level, status, types, stats, its moves' types, its best damage on the foe's
  active mon and the foe's best on it).
- **Per action**: its kind; for a move, the move features; for a switch, the
  target's party features; for an item, the item id, its target mon and move.

Ids (species, moves, effects, items) are one-hot over 256: the input is
6,267 wide (`OBS_DIM`) but only a few hundred non-zeros, and the input layers only touch those rows.
Game data comes from the ROM tables (`Moves`, `BaseData`, `TypeMatchups`).

## Training ([`crates/crystal-battle/src/ai/train.rs`](../crates/crystal-battle/src/ai/train.rs))

Battles are `crystal_battle::data::random::random_battle_config` scenarios (a random
trainer, a random player party around its level, a bag of battle items), and
in 70% of them two random AI items for the trainer. Each iteration mixes:

- 45% **self-play**: the network plays both sides; both sides train.
- 30% **vs the original AI**: the network plays the player against the game's
  own trainer AI.
- 25% **vs the pool**: against one of the last 12 snapshots (taken every 10
  iterations), on a random side.

Reward: +1 win, −1 loss, 0 draw, plus potential-based shaping on the HP
balance (team HP fraction minus the foe's, weight 0.5), which leaves the
optimal policy unchanged. A battle that hits the 600-decision cap is a loss
for every learner, and the AI never picks Run in a trainer battle (the
original refuses it and reopens the menu): early runs learned to stall
behind that no-op.

```bash
cargo run --release -p crystal-battle --features ai --example ai_train -- run.bin 3000
cargo run --release -p crystal-battle --features ai --example ai_export -- run.bin crates/crystal-battle/models/crystal-ai.bin
```

The trainer saves an `f32` checkpoint every iteration (and every 50th under
its iteration number) and evaluates every 25 on 2,000 held-out battles.
Release builds here: training is CPU-bound in the engine and the network.

## Evaluation

Two questions, on the same held-out scenarios:

1. **Is it a better trainer than the original?** A fixed player policy plays
   the same battles against the original trainer AI and against ours; we
   compare the trainer's score (win = 1, draw = ½). Player policies: `greedy`
   (strongest move by power × STAB × effectiveness × accuracy, no voluntary
   switching or items) and `random`.
2. **Does it beat the original head-on?** The AI plays the player's side
   against the original trainer AI (with the player's random party and bag).

`cargo run --release -p crystal-battle --features ai --example ai_eval -- [model.bin] [games]`
prints both.

Shipped model (3,000 iterations, ~45 minutes on an 11-core laptop; 5,000
held-out battles per row, seeds 9,000,000+):

| Trainer | vs greedy player | vs random player |
| --- | ---: | ---: |
| original trainer AI | 40.1% | 61.6% |
| **trained AI** | **51.0%** | **72.5%** |

| Player side, against the original trainer AI | Player's score |
| --- | ---: |
| random | 38.4% |
| greedy | 59.9% |
| **trained AI** | **74.9%** |

The scenarios favour the player (a random party at the trainer's level plus a
bag), which is why the original trainer AI loses to the greedy baseline. The
trained AI as the trainer wins 11 points more of the same battles against both
baselines, and as the player it beats the original AI three times out of four.
Progress flattened after ~2,000 iterations; more capacity, opponent variety
(e.g. the original AI's own scripted styles as players) or test-time search
are the levers for more.

## Using it

Library (`--features ai`):

```rust
use crystal_battle::ai::Agent;
let mut ai = Agent::builtin(); // or Agent::load("model.bin")
let action = ai.choose(decision); // any Decision, either side
```

CLI (the AI plays the enemy trainer; you pick on stdin as usual):

```bash
cargo run --features "ai gui" -- --random --vs-ai --show-text
```

`--ai-model <path>` loads another model, `--ai-player` lets the AI play your
side too (watch it play itself). `--record` records both sides' decisions;
`--play` replays them without the AI.

The shipped model is [`crates/crystal-battle/models/crystal-ai.bin`](../crates/crystal-battle/models/crystal-ai.bin)
(1.8M parameters stored as bf16, 3.6 MB, `include_bytes!` into the library;
`ai_export` writes one from a training checkpoint). A model file stores the feature
version and layer sizes and refuses to load against a different encoding;
bump `features::FEATURES_VERSION` whenever the encoding changes.
