//! A trained network as a battle player.

use super::features::{self, Sparse};
use super::net::{Net, Trace};
use crate::battle::{Action, BattleState, Decision, PromptSite, Side};

/// How an [`Agent`] turns its policy into one action.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Play {
    /// Always the most likely action (the strongest, fully predictable).
    Greedy,
    /// Sample from the policy at this temperature (1.0 = as trained).
    Sample { temperature: f32 },
}

/// A battle AI: plays either side of any [`Decision`].
#[derive(Debug, Clone)]
pub struct Agent {
    net: Net,
    play: Play,
    /// splitmix64 state for [`Play::Sample`].
    rng: u64,
}

/// The model shipped with the crate (`models/crystal-ai.bin`).
const BUILTIN: &[u8] = include_bytes!("../../models/crystal-ai.bin");

impl Agent {
    /// Wrap a network. Plays [`Play::Greedy`].
    pub fn new(net: Net) -> Agent {
        Agent { net, play: Play::Greedy, rng: 0x5eed }
    }

    /// The model trained for this crate (see docs/ai.md).
    pub fn builtin() -> Agent {
        Agent::new(Net::from_bytes(BUILTIN).expect("the built-in model matches this build's features"))
    }

    /// Load a model saved by the trainer (`ai_train`).
    pub fn load(path: impl AsRef<std::path::Path>) -> Result<Agent, String> {
        let bytes = std::fs::read(path.as_ref()).map_err(|e| format!("{}: {e}", path.as_ref().display()))?;
        Ok(Agent::new(Net::from_bytes(&bytes)?))
    }

    /// Change how actions are picked; `seed` seeds sampling.
    pub fn with_play(mut self, play: Play, seed: u64) -> Agent {
        self.play = play;
        self.rng = seed;
        self
    }

    pub fn net(&self) -> &Net {
        &self.net
    }

    /// The policy at a decision: one probability per action in `legal`, and
    /// the value estimate (about +1 = `side` wins, -1 = loses).
    pub fn evaluate(&self, state: &BattleState, site: PromptSite, side: Side, legal: &[Action]) -> (Vec<f32>, f32) {
        let trace = self.trace(state, site, side, legal).2;
        (trace.probs, trace.value)
    }

    /// Encode and run the network.
    pub(crate) fn trace(&self, state: &BattleState, site: PromptSite, side: Side, legal: &[Action]) -> (Sparse, Vec<Sparse>, Trace) {
        let obs = features::observe(state, site, side);
        let acts: Vec<Sparse> = legal.iter().map(|&a| features::action(state, side, a)).collect();
        let trace = self.net.forward(&obs, &acts);
        (obs, acts, trace)
    }

    /// Pick an action for a decision.
    pub fn choose(&mut self, decision: &Decision) -> Action {
        self.choose_at(&decision.state, decision.site, decision.side, &decision.legal)
    }

    /// Pick one of `legal` for `side` deciding at `site` in `state`.
    pub fn choose_at(&mut self, state: &BattleState, site: PromptSite, side: Side, legal: &[Action]) -> Action {
        let legal = &considered(state, legal);
        if legal.len() == 1 {
            return legal[0];
        }
        let (probs, _) = self.evaluate(state, site, side, legal);
        let pick = match self.play {
            Play::Greedy => argmax(&probs),
            Play::Sample { temperature } => {
                let weights: Vec<f32> = probs.iter().map(|p| p.max(1e-12).powf(1.0 / temperature.max(1e-3))).collect();
                sample(&weights, &mut self.rng)
            }
        };
        legal[pick]
    }
}

/// The actions the AI chooses among: `legal` minus Run in a trainer battle,
/// which the original refuses ("No! There's no running from a trainer
/// battle!") and returns to the same menu — a no-op that would let a policy
/// stall forever.
pub fn considered(state: &BattleState, legal: &[Action]) -> Vec<Action> {
    let trainer = state.battle_mode == crate::defs::TRAINER_BATTLE;
    let kept: Vec<Action> = legal.iter().copied().filter(|&a| !(trainer && a == Action::Run)).collect();
    if kept.is_empty() { legal.to_vec() } else { kept }
}

pub(crate) fn argmax(v: &[f32]) -> usize {
    let mut best = 0;
    for (i, &x) in v.iter().enumerate() {
        if x > v[best] {
            best = i;
        }
    }
    best
}

/// splitmix64.
pub(crate) fn next(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}

/// Index drawn proportionally to `weights`.
pub(crate) fn sample(weights: &[f32], rng: &mut u64) -> usize {
    let total: f32 = weights.iter().sum();
    let mut x = (next(rng) >> 40) as f32 / (1u64 << 24) as f32 * total;
    for (i, &w) in weights.iter().enumerate() {
        if x < w {
            return i;
        }
        x -= w;
    }
    weights.len() - 1
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::battle::{Battle, Status};
    use crate::config::EnemyControl;
    use crate::data::random::random_battle_config;

    /// The built-in model loads and plays a whole trainer battle on both sides,
    /// trainer items and switches included.
    #[test]
    fn builtin_model_plays_a_trainer_battle() {
        let mut ai = Agent::builtin();
        for seed in [3, 4] {
            let mut config = random_battle_config(seed);
            config.battle.enemy_control = EnemyControl::External;
            let engine_seed = config.battle.seed.unwrap();
            let mut battle = Battle::new(config, engine_seed).unwrap();
            let mut steps = 0;
            while let Status::Decision(decision) = battle.status() {
                let action = ai.choose(decision);
                assert!(decision.legal.contains(&action));
                battle.step(action).unwrap();
                steps += 1;
                assert!(steps < 1000, "the battle never ended");
            }
        }
    }

    #[test]
    fn never_runs_from_a_trainer() {
        let config = random_battle_config(3);
        let engine_seed = config.battle.seed.unwrap();
        let battle = Battle::new(config, engine_seed).unwrap();
        let decision = battle.decision().unwrap();
        assert!(decision.legal.contains(&Action::Run));
        assert!(!considered(&decision.state, &decision.legal).contains(&Action::Run));
    }
}
