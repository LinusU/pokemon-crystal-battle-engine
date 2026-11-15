//! PPO self-play training, and evaluation against the original trainer AI.
//!
//! Each iteration plays random trainer battles on the hand-written engine
//! ([`FastBattle`], no text) across threads, collecting the decisions of the
//! side(s) being trained, then runs a few epochs of PPO on them. The
//! opponents are mixed (see [`TrainConfig`]):
//!
//! - **self-play**: the current network plays both the player and the
//!   externally controlled trainer, and both sides' decisions are trained on;
//! - **vs the original AI**: the network plays the player against the game's
//!   own trainer AI, so it keeps beating the thing it replaces;
//! - **vs the pool**: one side is the current network, the other a frozen
//!   earlier snapshot, so it doesn't forget how to beat old strategies.
//!
//! The reward is +1 / -1 / 0 for a win / loss / draw, plus potential-based
//! shaping on the HP balance ([`features::potential`]), which speeds up
//! learning without changing what the optimal policy is.

use super::agent::{argmax, considered, next, sample};
use super::features::{self, Sparse};
use super::net::{Adam, Net};
use crate::battle::{Action, BattleState, Outcome, PromptSite, Side};
use crate::config::{BattleConfig, EnemyControl, ItemSpec};
use crate::data::{names, random::random_battle_config};
use crate::engine::api::{FastBattle, Status};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

/// Training hyperparameters.
#[derive(Debug, Clone)]
pub struct TrainConfig {
    /// Learner decisions collected per iteration.
    pub samples_per_iter: usize,
    pub threads: usize,
    pub lr: f32,
    pub epochs: usize,
    pub minibatch: usize,
    pub clip: f32,
    pub vf_coef: f32,
    pub ent_coef: f32,
    pub max_grad_norm: f32,
    pub gamma: f32,
    pub lambda: f32,
    /// Weight of the HP-balance shaping potential.
    pub shaping: f32,
    /// A battle longer than this many decisions is stopped and scored a draw.
    pub max_decisions: usize,
    /// Share of battles that are self-play / vs the original AI / vs the pool
    /// (normalised; the pool share falls back to self-play while it's empty).
    pub mix: [f32; 3],
    /// Snapshot the network into the pool every this many iterations.
    pub pool_every: usize,
    pub pool_size: usize,
}

impl Default for TrainConfig {
    fn default() -> TrainConfig {
        TrainConfig {
            samples_per_iter: 16384,
            threads: std::thread::available_parallelism().map(|n| n.get()).unwrap_or(4),
            lr: 2e-4,
            epochs: 3,
            minibatch: 2048,
            clip: 0.2,
            vf_coef: 0.5,
            ent_coef: 0.01,
            max_grad_norm: 1.0,
            gamma: 0.995,
            lambda: 0.95,
            shaping: 0.5,
            max_decisions: 600,
            mix: [0.45, 0.30, 0.25],
            pool_every: 10,
            pool_size: 12,
        }
    }
}

/// Who plays a side.
#[derive(Clone)]
pub enum Policy {
    /// The game's own trainer AI (enemy side only; the scenario keeps
    /// `enemy_control = "original"`).
    Original,
    /// A network, sampling from its policy (`greedy = false`) or taking the
    /// most likely action.
    Net { net: Arc<Net>, greedy: bool },
    /// A uniformly random legal action.
    Random,
    /// The strongest-looking move by a damage estimate; never switches or
    /// uses items voluntarily. A simple scripted baseline.
    Greedy,
}

/// One learner decision.
#[derive(Debug, Clone)]
struct Sample {
    obs: Sparse,
    acts: Vec<Sparse>,
    action: usize,
    logp: f32,
    value: f32,
    adv: f32,
    ret: f32,
}

/// Battle results from one side's point of view.
#[derive(Debug, Clone, Copy, Default)]
pub struct Record {
    pub wins: usize,
    pub losses: usize,
    pub draws: usize,
}

impl Record {
    pub fn games(&self) -> usize {
        self.wins + self.losses + self.draws
    }

    /// Wins plus half the draws, over games.
    pub fn score(&self) -> f32 {
        if self.games() == 0 {
            return 0.0;
        }
        (self.wins as f32 + 0.5 * self.draws as f32) / self.games() as f32
    }

    fn add(&mut self, result: f32) {
        if result > 0.0 {
            self.wins += 1;
        } else if result < 0.0 {
            self.losses += 1;
        } else {
            self.draws += 1;
        }
    }

    fn merge(&mut self, other: Record) {
        self.wins += other.wins;
        self.losses += other.losses;
        self.draws += other.draws;
    }
}

impl std::fmt::Display for Record {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:5.1}% ({}W {}L {}D)", 100.0 * self.score(), self.wins, self.losses, self.draws)
    }
}

// --- scenarios ------------------------------------------------------------------

/// A random trainer battle for training / evaluation: [`random_battle_config`]
/// (a random trainer and a random player party around its level, with a
/// bag), and in seven battles out of ten two random AI items for the
/// trainer. `None` for scenarios the engine doesn't start (Cal's Mystery Gift
/// party lives in SRAM).
pub fn scenario(seed: u64, enemy: EnemyControl) -> Option<BattleConfig> {
    let mut config = random_battle_config(seed);
    if config.battle.trainer.as_deref().is_some_and(|t| t.starts_with("CAL")) {
        return None;
    }
    config.battle.enemy_control = enemy;
    let mut rng = seed ^ 0x6974_656d;
    if next(&mut rng) % 10 < 7 {
        let items = crate::battle::legal::ENEMY_TRAINER_ITEMS;
        config.enemy_items = (0..2)
            .map(|_| ItemSpec { name: names::item_name(items[(next(&mut rng) % items.len() as u64) as usize]).to_string(), quantity: 1 })
            .collect();
    }
    Some(config)
}

/// The outcome as a score for `side`.
fn result_for(outcome: Outcome, side: Side) -> f32 {
    let player = match outcome {
        Outcome::Win | Outcome::Caught => 1.0,
        Outcome::Loss => -1.0,
        Outcome::Draw => 0.0,
    };
    if side == Side::Player { player } else { -player }
}

/// Pick an action for a non-learning policy.
fn act(policy: &Policy, state: &BattleState, site: PromptSite, side: Side, legal: &[Action], rng: &mut u64) -> Action {
    if legal.len() == 1 {
        return legal[0];
    }
    match policy {
        Policy::Original => unreachable!("the original AI decides inside the engine"),
        Policy::Random => legal[(next(rng) % legal.len() as u64) as usize],
        Policy::Greedy => greedy(state, side, legal),
        Policy::Net { net, greedy } => {
            let obs = features::observe(state, site, side);
            let acts: Vec<Sparse> = legal.iter().map(|&a| features::action(state, side, a)).collect();
            let probs = net.forward(&obs, &acts).probs;
            legal[if *greedy { argmax(&probs) } else { sample(&probs, rng) }]
        }
    }
}

/// [`Policy::Greedy`].
fn greedy(state: &BattleState, side: Side, legal: &[Action]) -> Action {
    let (me, foe) = features::sides(state, side);
    let score = |a: &Action| -> f32 {
        match *a {
            Action::Move { slot } => {
                let mv = me.active.moves[slot as usize];
                let data = features::move_data(mv);
                let stab = if me.active.types.contains(&data.kind) { 1.5 } else { 1.0 };
                f32::from(data.power) * stab * features::effectiveness(data.kind, foe.active.types) * f32::from(data.accuracy) / 255.0 + 0.01
            }
            Action::Fight => 1.0,
            Action::ForcedSwitch { slot } | Action::Switch { slot } => {
                // Only when forced: the healthiest mon.
                let m = &me.party[slot as usize];
                f32::from(m.hp) / f32::from(m.max_hp.max(1)) * 0.001
            }
            Action::OfferSwitch { keep: true, .. } => 1.0,
            _ => -1.0,
        }
    };
    let voluntary_switch = |a: &Action| matches!(a, Action::Switch { .. }) && legal.iter().any(|l| matches!(l, Action::Move { .. } | Action::Fight));
    *legal
        .iter()
        .filter(|a| !voluntary_switch(a))
        .max_by(|a, b| score(a).total_cmp(&score(b)))
        .unwrap_or(&legal[0])
}

// --- rollouts ----------------------------------------------------------------------

/// One side's trajectory in one battle.
#[derive(Default)]
struct Trajectory {
    samples: Vec<Sample>,
    potentials: Vec<f32>,
}

/// Play one battle. `player` / `enemy` say who plays each side; `learn` is
/// the network being trained (its decisions are collected, sampled from the
/// policy) for the sides where `learning[side]` is set.
fn play(
    config: &BattleConfig,
    policies: [&Policy; 2],
    learn: Option<&Net>,
    learning: [bool; 2],
    max_decisions: usize,
    rng: &mut u64,
) -> Option<(f32, [Trajectory; 2], BattleState)> {
    let seed = config.battle.seed?;
    let mut battle = FastBattle::new(config, seed).ok()?;
    let mut trajectories: [Trajectory; 2] = Default::default();
    let mut decisions = 0;
    loop {
        let (site, legal) = match battle.status() {
            Status::Ended(outcome) => {
                let state = battle.state();
                return Some((result_for(*outcome, Side::Player), trajectories, state));
            }
            Status::Decision { site, legal } => (*site, legal.clone()),
        };
        decisions += 1;
        if decisions > max_decisions {
            return Some((STALLED, trajectories, battle.state()));
        }
        let side = site.side();
        let i = side as usize;
        let state = battle.state();
        let legal = considered(&state, &legal);
        let action = match learn {
            Some(net) if learning[i] && legal.len() > 1 => {
                let obs = features::observe(&state, site, side);
                let acts: Vec<Sparse> = legal.iter().map(|&a| features::action(&state, side, a)).collect();
                let t = net.forward(&obs, &acts);
                let k = sample(&t.probs, rng);
                trajectories[i].potentials.push(features::potential(&state, side));
                trajectories[i].samples.push(Sample { obs, acts, action: k, logp: t.probs[k].max(1e-12).ln(), value: t.value, adv: 0.0, ret: 0.0 });
                legal[k]
            }
            _ => act(policies[i], &state, site, side, &legal, rng),
        };
        battle.step(action).expect("a legal action");
    }
}

/// [`play`]'s result for a battle stopped at the decision cap.
const STALLED: f32 = f32::NAN;

/// Fill in advantages and returns (GAE) for one finished trajectory.
fn finish(tr: &mut Trajectory, result: f32, end_potential: f32, cfg: &TrainConfig) {
    let n = tr.samples.len();
    let mut next_value = 0.0;
    let mut adv = 0.0;
    for t in (0..n).rev() {
        let phi_next = if t + 1 < n { tr.potentials[t + 1] } else { end_potential };
        let mut reward = cfg.shaping * (phi_next - tr.potentials[t]);
        if t + 1 == n {
            reward += result;
        }
        let terminal_next = t + 1 == n;
        let v = tr.samples[t].value;
        let delta = reward + if terminal_next { 0.0 } else { cfg.gamma * next_value } - v;
        adv = delta + if terminal_next { 0.0 } else { cfg.gamma * cfg.lambda * adv };
        tr.samples[t].adv = adv;
        tr.samples[t].ret = adv + v;
        next_value = v;
    }
}

/// What a rollout phase produced.
#[derive(Default)]
pub struct Rollout {
    samples: Vec<Sample>,
    /// Learner results: self-play (player side), vs the original AI, vs the pool.
    pub vs_self: Record,
    pub vs_original: Record,
    pub vs_pool: Record,
    pub battles: usize,
    pub decisions: usize,
}

/// Collect at least `cfg.samples_per_iter` learner decisions.
fn rollout(net: &Arc<Net>, pool: &[Arc<Net>], cfg: &TrainConfig, seed_base: u64) -> Rollout {
    let collected = AtomicUsize::new(0);
    let next_game = AtomicUsize::new(0);
    let current = Policy::Net { net: net.clone(), greedy: false };
    let mut parts: Vec<Rollout> = std::thread::scope(|scope| {
        let handles: Vec<_> = (0..cfg.threads)
            .map(|_| {
                let (collected, next_game, current) = (&collected, &next_game, &current);
                scope.spawn(move || {
                    let mut out = Rollout::default();
                    while collected.load(Ordering::Relaxed) < cfg.samples_per_iter {
                        let game = next_game.fetch_add(1, Ordering::Relaxed) as u64;
                        let seed = seed_base.wrapping_mul(0x1_0000_0000).wrapping_add(game);
                        let mut rng = seed ^ 0xa5a5_5a5a_dead_beef;
                        let total: f32 = cfg.mix.iter().sum();
                        let mut roll = (next(&mut rng) >> 40) as f32 / (1u64 << 24) as f32 * total;
                        let mut mode = 0;
                        while mode < 2 && roll >= cfg.mix[mode] {
                            roll -= cfg.mix[mode];
                            mode += 1;
                        }
                        if mode == 2 && pool.is_empty() {
                            mode = 0;
                        }
                        let enemy = if mode == 1 { EnemyControl::Original } else { EnemyControl::External };
                        let Some(config) = scenario(seed, enemy) else { continue };
                        let snapshot;
                        let (policies, learning): ([&Policy; 2], [bool; 2]) = match mode {
                            0 => ([current, current], [true, true]),
                            1 => ([current, &Policy::Original], [true, false]),
                            _ => {
                                snapshot = Policy::Net { net: pool[(next(&mut rng) % pool.len() as u64) as usize].clone(), greedy: false };
                                if next(&mut rng) % 2 == 0 { ([current, &snapshot], [true, false]) } else { ([&snapshot, current], [false, true]) }
                            }
                        };
                        let Some((result, mut trajectories, end)) = play(&config, policies, Some(net), learning, cfg.max_decisions, &mut rng) else {
                            continue;
                        };
                        out.battles += 1;
                        for (i, tr) in trajectories.iter_mut().enumerate() {
                            let side = if i == 0 { Side::Player } else { Side::Enemy };
                            if !learning[i] {
                                continue;
                            }
                            // A stalled battle is a loss for every learner, so stalling never pays.
                            let r = if result.is_nan() { -1.0 } else if side == Side::Player { result } else { -result };
                            let shown = if result.is_nan() { 0.0 } else { r };
                            match mode {
                                0 if side == Side::Player => out.vs_self.add(shown),
                                1 => out.vs_original.add(shown),
                                2 => out.vs_pool.add(shown),
                                _ => {}
                            }
                            finish(tr, r, features::potential(&end, side), cfg);
                            out.decisions += tr.samples.len();
                            collected.fetch_add(tr.samples.len(), Ordering::Relaxed);
                            out.samples.append(&mut tr.samples);
                        }
                    }
                    out
                })
            })
            .collect();
        handles.into_iter().map(|h| h.join().expect("rollout worker")).collect()
    });
    let mut all = parts.pop().unwrap_or_default();
    for mut p in parts {
        all.samples.append(&mut p.samples);
        all.vs_self.merge(p.vs_self);
        all.vs_original.merge(p.vs_original);
        all.vs_pool.merge(p.vs_pool);
        all.battles += p.battles;
        all.decisions += p.decisions;
    }
    all
}

// --- PPO -----------------------------------------------------------------------------

/// Losses averaged over one update phase.
#[derive(Debug, Clone, Copy, Default)]
pub struct Losses {
    pub policy: f32,
    pub value: f32,
    pub entropy: f32,
    pub clip_fraction: f32,
    pub approx_kl: f32,
}

/// Gradient of the PPO loss over `batch`, summed into `grad`; returns the
/// summed loss terms.
fn batch_grad(net: &Net, batch: &[&Sample], cfg: &TrainConfig, scale: f32, grad: &mut [f32]) -> Losses {
    let mut l = Losses::default();
    for s in batch {
        let t = net.forward(&s.obs, &s.acts);
        let p = &t.probs;
        let logp = p[s.action].max(1e-12).ln();
        let ratio = (logp - s.logp).exp();
        let clipped = ratio.clamp(1.0 - cfg.clip, 1.0 + cfg.clip);
        let (surr1, surr2) = (ratio * s.adv, clipped * s.adv);
        l.policy += -surr1.min(surr2);
        if (ratio - clipped).abs() > 1e-6 {
            l.clip_fraction += 1.0;
        }
        l.approx_kl += s.logp - logp;
        // dL/dlogp: only through the unclipped term when it is the minimum.
        let g = if surr1 <= surr2 { -ratio * s.adv } else { 0.0 };
        let entropy: f32 = -p.iter().map(|&q| if q > 0.0 { q * q.ln() } else { 0.0 }).sum::<f32>();
        l.entropy += entropy;
        let dlogits: Vec<f32> = p
            .iter()
            .enumerate()
            .map(|(k, &q)| {
                let pg = g * (if k == s.action { 1.0 } else { 0.0 } - q);
                let eg = cfg.ent_coef * q * (q.max(1e-12).ln() + entropy);
                (pg + eg) * scale
            })
            .collect();
        let verr = t.value - s.ret;
        l.value += verr * verr;
        net.backward(&s.obs, &s.acts, &t, &dlogits, 2.0 * cfg.vf_coef * verr * scale, grad);
    }
    l
}

/// The trainer: network, optimiser, opponent pool.
pub struct Trainer {
    pub net: Net,
    pub adam: Adam,
    pub pool: Vec<Arc<Net>>,
    pub cfg: TrainConfig,
    pub iteration: usize,
    grads: Vec<Vec<f32>>,
}

/// What one iteration did.
pub struct IterStats {
    pub rollout: Rollout,
    pub losses: Losses,
    pub rollout_secs: f32,
    pub update_secs: f32,
}

impl Trainer {
    pub fn new(net: Net, cfg: TrainConfig) -> Trainer {
        let adam = Adam::new(net.len(), cfg.lr);
        Trainer { net, adam, pool: Vec::new(), cfg, iteration: 0, grads: Vec::new() }
    }

    /// Collect a batch of battles and run PPO on it.
    pub fn iterate(&mut self, seed: u64) -> IterStats {
        let started = std::time::Instant::now();
        let current = Arc::new(self.net.clone());
        let mut rollout = rollout(&current, &self.pool, &self.cfg, seed.wrapping_add(self.iteration as u64));
        let rollout_secs = started.elapsed().as_secs_f32();

        let started = std::time::Instant::now();
        let mut samples = std::mem::take(&mut rollout.samples);
        // Normalise advantages.
        let n = samples.len().max(1) as f32;
        let mean = samples.iter().map(|s| s.adv).sum::<f32>() / n;
        let std = (samples.iter().map(|s| (s.adv - mean).powi(2)).sum::<f32>() / n).sqrt().max(1e-6);
        for s in &mut samples {
            s.adv = (s.adv - mean) / std;
        }
        let threads = self.cfg.threads.max(1);
        if self.grads.len() != threads {
            self.grads = vec![vec![0.0; self.net.len()]; threads];
        }
        let mut order: Vec<usize> = (0..samples.len()).collect();
        let mut rng = seed ^ (self.iteration as u64).wrapping_mul(0x9e37);
        let mut total = Losses::default();
        let mut counted = 0.0;
        for _ in 0..self.cfg.epochs {
            // Shuffle.
            for i in (1..order.len()).rev() {
                let j = (next(&mut rng) % (i as u64 + 1)) as usize;
                order.swap(i, j);
            }
            for chunk in order.chunks(self.cfg.minibatch) {
                let batch: Vec<&Sample> = chunk.iter().map(|&i| &samples[i]).collect();
                let scale = 1.0 / batch.len() as f32;
                let per = batch.len().div_ceil(threads);
                let net = &self.net;
                let cfg = &self.cfg;
                let losses: Vec<Losses> = std::thread::scope(|scope| {
                    let handles: Vec<_> = self
                        .grads
                        .iter_mut()
                        .zip(batch.chunks(per.max(1)))
                        .map(|(grad, part)| {
                            scope.spawn(move || {
                                grad.iter_mut().for_each(|g| *g = 0.0);
                                batch_grad(net, part, cfg, scale, grad)
                            })
                        })
                        .collect();
                    handles.into_iter().map(|h| h.join().expect("gradient worker")).collect()
                });
                let used = batch.chunks(per.max(1)).count();
                // Sum the per-thread gradients into the first.
                let (first, rest) = self.grads.split_at_mut(1);
                for g in &rest[..used - 1] {
                    for (a, b) in first[0].iter_mut().zip(g) {
                        *a += b;
                    }
                }
                let grad = &mut first[0];
                let norm = grad.iter().map(|g| g * g).sum::<f32>().sqrt();
                if norm > self.cfg.max_grad_norm {
                    let k = self.cfg.max_grad_norm / norm;
                    grad.iter_mut().for_each(|g| *g *= k);
                }
                self.adam.step(&mut self.net.params, grad);
                for l in losses {
                    total.policy += l.policy;
                    total.value += l.value;
                    total.entropy += l.entropy;
                    total.clip_fraction += l.clip_fraction;
                    total.approx_kl += l.approx_kl;
                }
                counted += batch.len() as f32;
            }
        }
        let c = counted.max(1.0);
        let losses = Losses {
            policy: total.policy / c,
            value: total.value / c,
            entropy: total.entropy / c,
            clip_fraction: total.clip_fraction / c,
            approx_kl: total.approx_kl / c,
        };
        self.iteration += 1;
        if self.iteration % self.cfg.pool_every == 0 {
            self.pool.push(Arc::new(self.net.clone()));
            if self.pool.len() > self.cfg.pool_size {
                self.pool.remove(0);
            }
        }
        IterStats { rollout, losses, rollout_secs, update_secs: started.elapsed().as_secs_f32() }
    }
}

// --- evaluation ----------------------------------------------------------------------

/// Play `games` scenarios (seeds `first..`) with `player` against `trainer`
/// on `threads` threads; the result from the trainer's point of view.
/// `trainer` = [`Policy::Original`] runs the game's own AI.
pub fn evaluate(player: &Policy, trainer: &Policy, games: usize, first: u64, threads: usize) -> Record {
    let next_game = AtomicUsize::new(0);
    let records: Vec<Record> = std::thread::scope(|scope| {
        let handles: Vec<_> = (0..threads.max(1))
            .map(|_| {
                let next_game = &next_game;
                scope.spawn(move || {
                    let mut record = Record::default();
                    loop {
                        let g = next_game.fetch_add(1, Ordering::Relaxed);
                        if g >= games {
                            return record;
                        }
                        let seed = first + g as u64;
                        let enemy = if matches!(trainer, Policy::Original) { EnemyControl::Original } else { EnemyControl::External };
                        let Some(config) = scenario(seed, enemy) else { continue };
                        let mut rng = seed ^ 0x6576_616c;
                        if let Some((result, _, _)) = play(&config, [player, trainer], None, [false, false], 600, &mut rng) {
                            record.add(if result.is_nan() { 0.0 } else { -result });
                        }
                    }
                })
            })
            .collect();
        handles.into_iter().map(|h| h.join().expect("eval worker")).collect()
    });
    let mut total = Record::default();
    for r in records {
        total.merge(r);
    }
    total
}
