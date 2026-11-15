//! The policy/value network: a small MLP with hand-written backprop.
//!
//! ```text
//! obs (sparse) ─ W1 ─ relu ─ W2 ─ relu ─ h ──┬─ wv ─ value
//!                                            └─ Wh ─┐
//! action (sparse) ─────────────────────────── Wa ───┴─ + ─ relu ─ wl ─ logit
//! ```
//!
//! Each legal action gets a logit from its own features plus the state
//! embedding `h`, and the policy is the softmax over the legal actions, so
//! one network handles every decision site and any number of actions (moves,
//! switches, each bag item and target). `Wh·h` is computed once per state.
//!
//! Inputs are sparse ([`Sparse`]), so the input layers cost O(non-zeros).
//! Weights are input-major (`W[i]` is the row a non-zero input `i` adds), which
//! makes both the sparse forward pass and the sparse gradient a row `axpy`.

use super::features::{ACT_DIM, FEATURES_VERSION, OBS_DIM, Sparse};

/// Layer sizes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Shape {
    pub obs: usize,
    pub act: usize,
    /// Trunk width.
    pub hidden: usize,
    /// Action-head width.
    pub head: usize,
}

impl Shape {
    pub const DEFAULT: Shape = Shape { obs: OBS_DIM as usize, act: ACT_DIM as usize, hidden: 256, head: 128 };
}

/// Offsets of each parameter block in the flat parameter vector.
#[derive(Debug, Clone, Copy)]
struct Layout {
    w1: usize,
    b1: usize,
    w2: usize,
    b2: usize,
    wv: usize,
    bv: usize,
    wa: usize,
    wh: usize,
    ba: usize,
    wl: usize,
    len: usize,
}

impl Layout {
    fn new(s: Shape) -> Layout {
        let w1 = 0;
        let b1 = w1 + s.obs * s.hidden;
        let w2 = b1 + s.hidden;
        let b2 = w2 + s.hidden * s.hidden;
        let wv = b2 + s.hidden;
        let bv = wv + s.hidden;
        let wa = bv + 1;
        let wh = wa + s.act * s.head;
        let ba = wh + s.hidden * s.head;
        let wl = ba + s.head;
        let len = wl + s.head;
        Layout { w1, b1, w2, b2, wv, bv, wa, wh, ba, wl, len }
    }
}

/// The network's parameters.
#[derive(Debug, Clone)]
pub struct Net {
    pub shape: Shape,
    layout: Layout,
    /// Every parameter, flat.
    pub params: Vec<f32>,
}

/// What a forward pass keeps for the backward pass.
#[derive(Debug, Clone, Default)]
pub struct Trace {
    h1: Vec<f32>,
    h2: Vec<f32>,
    /// Pre-activation of the action head, per action.
    z: Vec<Vec<f32>>,
    /// Post-activation of the action head, per action.
    u: Vec<Vec<f32>>,
    pub logits: Vec<f32>,
    pub probs: Vec<f32>,
    pub value: f32,
}

/// splitmix64, for initialisation.
fn mix(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}

fn axpy(y: &mut [f32], a: f32, x: &[f32]) {
    for (y, x) in y.iter_mut().zip(x) {
        *y += a * x;
    }
}

fn dot(a: &[f32], b: &[f32]) -> f32 {
    a.iter().zip(b).map(|(a, b)| a * b).sum()
}

impl Net {
    /// A freshly initialised network (He-uniform weights, zero biases).
    pub fn new(shape: Shape, seed: u64) -> Net {
        let layout = Layout::new(shape);
        let mut params = vec![0.0f32; layout.len];
        let mut rng = seed;
        let mut init = |params: &mut [f32], fan_in: f32, gain: f32| {
            let bound = gain * (6.0 / fan_in).sqrt();
            for p in params {
                let u = (mix(&mut rng) >> 40) as f32 / (1u64 << 24) as f32;
                *p = (2.0 * u - 1.0) * bound;
            }
        };
        let l = layout;
        // The sparse inputs have a few hundred non-zeros, not `obs` of them.
        init(&mut params[l.w1..l.b1], 300.0, 1.0);
        init(&mut params[l.w2..l.b2], shape.hidden as f32, 1.0);
        init(&mut params[l.wv..l.bv], shape.hidden as f32, 0.1);
        init(&mut params[l.wa..l.wh], 60.0, 1.0);
        init(&mut params[l.wh..l.ba], shape.hidden as f32, 1.0);
        init(&mut params[l.wl..l.len], shape.head as f32, 0.1);
        Net { shape, layout, params }
    }

    /// The default shape, freshly initialised.
    pub fn default_shape(seed: u64) -> Net {
        Net::new(Shape::DEFAULT, seed)
    }

    pub fn len(&self) -> usize {
        self.layout.len
    }

    /// The state embedding `h` (`h1`, `h2`).
    fn trunk(&self, obs: &Sparse) -> (Vec<f32>, Vec<f32>) {
        let (s, l, p) = (self.shape, self.layout, &self.params);
        let mut z1 = p[l.b1..l.b1 + s.hidden].to_vec();
        for (&i, &v) in obs.idx.iter().zip(&obs.val) {
            let row = l.w1 + i as usize * s.hidden;
            axpy(&mut z1, v, &p[row..row + s.hidden]);
        }
        let h1: Vec<f32> = z1.iter().map(|&z| z.max(0.0)).collect();
        let mut z2 = p[l.b2..l.b2 + s.hidden].to_vec();
        for (i, &v) in h1.iter().enumerate() {
            if v != 0.0 {
                let row = l.w2 + i * s.hidden;
                axpy(&mut z2, v, &p[row..row + s.hidden]);
            }
        }
        let h2: Vec<f32> = z2.iter().map(|&z| z.max(0.0)).collect();
        (h1, h2)
    }

    /// Forward pass: the value of `obs` and a probability per action in `acts`.
    pub fn forward(&self, obs: &Sparse, acts: &[Sparse]) -> Trace {
        let (s, l, p) = (self.shape, self.layout, &self.params);
        let (h1, h2) = self.trunk(obs);
        let value = dot(&p[l.wv..l.wv + s.hidden], &h2) + p[l.bv];
        // q = Wh·h + ba, shared by every action.
        let mut q = p[l.ba..l.ba + s.head].to_vec();
        for (i, &v) in h2.iter().enumerate() {
            if v != 0.0 {
                let row = l.wh + i * s.head;
                axpy(&mut q, v, &p[row..row + s.head]);
            }
        }
        let wl = &p[l.wl..l.wl + s.head];
        let mut z = Vec::with_capacity(acts.len());
        let mut u = Vec::with_capacity(acts.len());
        let mut logits = Vec::with_capacity(acts.len());
        for a in acts {
            let mut za = q.clone();
            for (&i, &v) in a.idx.iter().zip(&a.val) {
                let row = l.wa + i as usize * s.head;
                axpy(&mut za, v, &p[row..row + s.head]);
            }
            let ua: Vec<f32> = za.iter().map(|&x| x.max(0.0)).collect();
            logits.push(dot(wl, &ua));
            z.push(za);
            u.push(ua);
        }
        let probs = softmax(&logits);
        Trace { h1, h2, z, u, logits, probs, value }
    }

    /// Backward pass for one sample. `dlogits` is dL/dlogit per action and
    /// `dvalue` dL/dvalue; the gradient is added into `grad` (same layout as
    /// `params`).
    pub fn backward(&self, obs: &Sparse, acts: &[Sparse], t: &Trace, dlogits: &[f32], dvalue: f32, grad: &mut [f32]) {
        let (s, l, p) = (self.shape, self.layout, &self.params);
        let mut dh2 = vec![0.0f32; s.hidden];
        // Value head.
        axpy(&mut grad[l.wv..l.wv + s.hidden], dvalue, &t.h2);
        grad[l.bv] += dvalue;
        axpy(&mut dh2, dvalue, &p[l.wv..l.wv + s.hidden]);
        // Action head.
        let mut dq = vec![0.0f32; s.head];
        for (k, a) in acts.iter().enumerate() {
            let dl = dlogits[k];
            if dl == 0.0 {
                continue;
            }
            axpy(&mut grad[l.wl..l.wl + s.head], dl, &t.u[k]);
            let mut dz = vec![0.0f32; s.head];
            for j in 0..s.head {
                if t.z[k][j] > 0.0 {
                    dz[j] = dl * p[l.wl + j];
                }
            }
            for (&i, &v) in a.idx.iter().zip(&a.val) {
                let row = l.wa + i as usize * s.head;
                axpy(&mut grad[row..row + s.head], v, &dz);
            }
            axpy(&mut dq, 1.0, &dz);
        }
        axpy(&mut grad[l.ba..l.ba + s.head], 1.0, &dq);
        for i in 0..s.hidden {
            let v = t.h2[i];
            let row = l.wh + i * s.head;
            if v != 0.0 {
                axpy(&mut grad[row..row + s.head], v, &dq);
                dh2[i] += dot(&p[row..row + s.head], &dq);
            }
        }
        // Trunk: relu(z2) — h2 > 0 exactly where z2 > 0.
        let dz2: Vec<f32> = dh2.iter().zip(&t.h2).map(|(&d, &h)| if h > 0.0 { d } else { 0.0 }).collect();
        axpy(&mut grad[l.b2..l.b2 + s.hidden], 1.0, &dz2);
        let mut dh1 = vec![0.0f32; s.hidden];
        for i in 0..s.hidden {
            let v = t.h1[i];
            if v != 0.0 {
                let row = l.w2 + i * s.hidden;
                axpy(&mut grad[row..row + s.hidden], v, &dz2);
                dh1[i] = dot(&p[row..row + s.hidden], &dz2);
            }
        }
        let dz1: Vec<f32> = dh1.iter().zip(&t.h1).map(|(&d, &h)| if h > 0.0 { d } else { 0.0 }).collect();
        axpy(&mut grad[l.b1..l.b1 + s.hidden], 1.0, &dz1);
        for (&i, &v) in obs.idx.iter().zip(&obs.val) {
            let row = l.w1 + i as usize * s.hidden;
            axpy(&mut grad[row..row + s.hidden], v, &dz1);
        }
    }

    // --- saving -----------------------------------------------------------------

    const MAGIC: &'static [u8; 4] = b"PKAI";

    /// Serialise with full `f32` parameters (training checkpoints).
    pub fn to_bytes(&self) -> Vec<u8> {
        self.serialise(Precision::F32)
    }

    /// Serialise with `bf16` parameters (the top half of each `f32`, rounded):
    /// half the size, for shipping. Playing strength is unaffected.
    pub fn to_bytes_bf16(&self) -> Vec<u8> {
        self.serialise(Precision::Bf16)
    }

    /// Magic, precision, features version, shape, then the parameters
    /// little-endian.
    fn serialise(&self, precision: Precision) -> Vec<u8> {
        let mut out = Vec::with_capacity(28 + 4 * self.params.len());
        out.extend_from_slice(Self::MAGIC);
        out.extend_from_slice(&(precision as u32).to_le_bytes());
        out.extend_from_slice(&FEATURES_VERSION.to_le_bytes());
        for d in [self.shape.obs, self.shape.act, self.shape.hidden, self.shape.head] {
            out.extend_from_slice(&(d as u32).to_le_bytes());
        }
        for p in &self.params {
            match precision {
                Precision::F32 => out.extend_from_slice(&p.to_le_bytes()),
                Precision::Bf16 => {
                    // Round to nearest even on the dropped half.
                    let bits = p.to_bits();
                    let rounded = bits.wrapping_add(0x7fff + ((bits >> 16) & 1));
                    out.extend_from_slice(&((rounded >> 16) as u16).to_le_bytes());
                }
            }
        }
        out
    }

    /// Parse [`Net::to_bytes`] / [`Net::to_bytes_bf16`] output.
    pub fn from_bytes(bytes: &[u8]) -> Result<Net, String> {
        let word = |i: usize| -> Result<u32, String> {
            bytes.get(i..i + 4).map(|b| u32::from_le_bytes(b.try_into().unwrap())).ok_or_else(|| "truncated model file".to_string())
        };
        if bytes.get(..4) != Some(Self::MAGIC.as_slice()) {
            return Err("not a model file (bad magic)".into());
        }
        let precision = match word(4)? {
            1 => Precision::F32,
            2 => Precision::Bf16,
            p => return Err(format!("unknown parameter precision {p}")),
        };
        let version = word(8)?;
        if version != FEATURES_VERSION {
            return Err(format!("model was trained on feature version {version}, this build encodes version {FEATURES_VERSION}"));
        }
        let shape = Shape { obs: word(12)? as usize, act: word(16)? as usize, hidden: word(20)? as usize, head: word(24)? as usize };
        if shape.obs != OBS_DIM as usize || shape.act != ACT_DIM as usize {
            return Err(format!("model input sizes {}/{} don't match this build's {OBS_DIM}/{ACT_DIM}", shape.obs, shape.act));
        }
        let layout = Layout::new(shape);
        let body = &bytes[28..];
        let width = match precision {
            Precision::F32 => 4,
            Precision::Bf16 => 2,
        };
        if body.len() != width * layout.len {
            return Err(format!("model file has {} parameter bytes, expected {}", body.len(), width * layout.len));
        }
        let params = match precision {
            Precision::F32 => body.chunks_exact(4).map(|c| f32::from_le_bytes(c.try_into().unwrap())).collect(),
            Precision::Bf16 => body.chunks_exact(2).map(|c| f32::from_bits(u32::from(u16::from_le_bytes(c.try_into().unwrap())) << 16)).collect(),
        };
        Ok(Net { shape, layout, params })
    }
}

/// How parameters are stored in a model file.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Precision {
    F32 = 1,
    Bf16 = 2,
}

/// Softmax of `logits` (max-shifted).
pub fn softmax(logits: &[f32]) -> Vec<f32> {
    let max = logits.iter().copied().fold(f32::NEG_INFINITY, f32::max);
    let exp: Vec<f32> = logits.iter().map(|&l| (l - max).exp()).collect();
    let sum: f32 = exp.iter().sum();
    exp.into_iter().map(|e| e / sum).collect()
}

/// Adam over the flat parameter vector.
#[derive(Debug, Clone)]
pub struct Adam {
    m: Vec<f32>,
    v: Vec<f32>,
    t: i32,
    pub lr: f32,
    pub beta1: f32,
    pub beta2: f32,
    pub eps: f32,
}

impl Adam {
    pub fn new(len: usize, lr: f32) -> Adam {
        Adam { m: vec![0.0; len], v: vec![0.0; len], t: 0, lr, beta1: 0.9, beta2: 0.999, eps: 1e-8 }
    }

    /// One step: `params -= lr * m̂ / (√v̂ + ε)`.
    pub fn step(&mut self, params: &mut [f32], grad: &[f32]) {
        self.t += 1;
        let (b1, b2) = (self.beta1, self.beta2);
        let c1 = 1.0 - b1.powi(self.t);
        let c2 = 1.0 - b2.powi(self.t);
        let lr = self.lr * c2.sqrt() / c1;
        for i in 0..params.len() {
            let g = grad[i];
            // Untouched sparse rows: let their moments decay without a step.
            if g == 0.0 && self.m[i] == 0.0 {
                continue;
            }
            self.m[i] = b1 * self.m[i] + (1.0 - b1) * g;
            self.v[i] = b2 * self.v[i] + (1.0 - b2) * g * g;
            params[i] -= lr * self.m[i] / (self.v[i].sqrt() + self.eps);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sparse(pairs: &[(u32, f32)]) -> Sparse {
        Sparse { idx: pairs.iter().map(|p| p.0).collect(), val: pairs.iter().map(|p| p.1).collect() }
    }

    /// The analytic gradient matches finite differences on a tiny network.
    #[test]
    fn backward_matches_finite_differences() {
        let shape = Shape { obs: 7, act: 5, hidden: 6, head: 4 };
        let mut net = Net::new(shape, 3);
        // Positive biases so most units are active (relu kinks break the check).
        let l = net.layout;
        for i in l.b1..l.b1 + 6 {
            net.params[i] = 0.3;
        }
        for i in l.b2..l.b2 + 6 {
            net.params[i] = 0.3;
        }
        for i in l.ba..l.ba + 4 {
            net.params[i] = 0.3;
        }
        let obs = sparse(&[(0, 1.0), (3, -0.5), (6, 0.7)]);
        let acts = vec![sparse(&[(0, 1.0), (2, 0.4)]), sparse(&[(1, 1.0)]), sparse(&[(3, 0.9), (4, -0.2)])];
        // L = -log p[1] + 0.5 * value^2
        let loss = |net: &Net| -> f32 {
            let t = net.forward(&obs, &acts);
            -t.probs[1].ln() + 0.5 * t.value * t.value
        };
        let t = net.forward(&obs, &acts);
        let dlogits: Vec<f32> = t.probs.iter().enumerate().map(|(k, &p)| p - if k == 1 { 1.0 } else { 0.0 }).collect();
        let mut grad = vec![0.0; net.len()];
        net.backward(&obs, &acts, &t, &dlogits, t.value, &mut grad);
        let eps = 1e-3;
        let mut worst = 0.0f32;
        for i in 0..net.len() {
            let old = net.params[i];
            net.params[i] = old + eps;
            let up = loss(&net);
            net.params[i] = old - eps;
            let down = loss(&net);
            net.params[i] = old;
            let numeric = (up - down) / (2.0 * eps);
            worst = worst.max((numeric - grad[i]).abs());
        }
        assert!(worst < 2e-3, "worst gradient error {worst}");
    }

    #[test]
    fn round_trips_through_bytes() {
        let net = Net::default_shape(1);
        let back = Net::from_bytes(&net.to_bytes()).unwrap();
        assert_eq!(back.params, net.params);
        let half = Net::from_bytes(&net.to_bytes_bf16()).unwrap();
        for (a, b) in half.params.iter().zip(&net.params) {
            assert!((a - b).abs() <= b.abs() / 128.0, "{a} vs {b}");
        }
    }
}
