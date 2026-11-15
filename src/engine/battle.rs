//! The engine's state and the primitives every ported routine uses: the
//! random stream, battle variables (`GetBattleVar`), turn switching, battle
//! text, and decisions.

use std::collections::VecDeque;

use super::mem::Mem;
use super::sym::*;
use crate::bootstrap::decision::{PromptSite, Step};
use crate::defs;

/// A routine either finishes or stops because the engine needs a decision it
/// doesn't have yet. `?` propagates the stop to the top of the turn; the
/// caller then re-runs the turn from its start with the decision supplied (see
/// [`super::Battle`]).
pub type Flow<T = ()> = Result<T, Suspend>;

/// "Waiting for a decision at `site`."
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Suspend {
    pub site: PromptSite,
    pub legal: Vec<Step>,
}

/// One message the battle would print: the ROM address of its text.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextRef(pub usize);

/// Harness-level flags the emulated harness keeps on its `Cpu`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Flags {
    /// Both sides are decided by the caller (`wLinkMode` link path).
    pub link_battle: bool,
    /// A thrown ball caught the wild mon (`Outcome::Caught`).
    pub caught_wild_mon: bool,
    /// Move slot picked at the battle menu, for the patched
    /// `MoveSelectionScreen` (`Cpu::cli_pending_move`).
    pub pending_move: Option<u8>,
    /// FIGHT with no usable move (`Cpu::cli_pending_struggle`).
    pub pending_struggle: bool,
    /// The enemy trainer is decided by the caller (`enemy_control =
    /// "external"`, `Cpu::external_enemy`).
    pub external_enemy: bool,
    /// The external trainer's action this turn, for `AI_SwitchOrTryItem`
    /// (`Cpu::external_enemy_choice`).
    pub enemy_choice: Option<Step>,
}

/// The whole battle: memory, the random stream, and harness flags. `Clone` is
/// cheap (~8 KB), which is what lets an agent search ahead.
#[derive(Debug, Clone)]
pub struct Engine {
    pub mem: Mem,
    /// xorshift64 state — the same stream as `Cpu::rng_state`.
    pub rng: u64,
    pub flags: Flags,
    /// Decisions to hand out before suspending (filled when replaying a turn).
    pub pending: VecDeque<Step>,
    /// Every message printed since the log was last taken.
    pub texts: Vec<TextRef>,
    /// Optional record of every random draw (differential testing): the
    /// routine that asked, and the byte.
    pub rng_trace: Option<Vec<(&'static str, u8)>>,
    /// Optional record of the engine at every decision point (differential
    /// testing): the site, legal steps, memory and random-stream state.
    pub decision_log: Option<Vec<(PromptSite, Vec<Step>, Mem, u64)>>,
    /// The registers `b`, `c`, `d`, `e` that `PlayerAttackDamage` /
    /// `EnemyAttackDamage` / `HitSelfInConfusion` leave for
    /// `BattleCommand_DamageCalc` (attack, defense, move power, level). The
    /// original's script reader preserves them between commands.
    pub damage_regs: [u8; 4],
    /// When `Some`, every printed text is rendered to its messages here
    /// (exact strings, see `text.rs`).
    pub rendered: Option<Vec<String>>,
    /// Rendered messages per decision, when both logs are on.
    pub text_log: Vec<Vec<String>>,
    /// ROM bytes a scenario overrides (flat offset, value): the harness's
    /// `Cpu::poke_rom` patches of a trainer's AI flags. Read through
    /// [`Engine::rom_byte`].
    pub rom_overlay: Vec<(usize, u8)>,
    /// When `Some`, what the battle shows, in the emulated harness's format
    /// (`bootstrap::events`): every text message and every HP-bar change.
    pub events: Option<Vec<crate::bootstrap::events::Event>>,
}

impl Engine {
    pub fn new(mem: Mem, rng: u64, flags: Flags) -> Self {
        Engine { mem, rng, flags, pending: VecDeque::new(), texts: Vec::new(), rng_trace: None, decision_log: None, damage_regs: [0; 4], rendered: None, text_log: Vec::new(), rom_overlay: Vec::new(), events: None }
    }

    /// A ROM byte, as patched by the scenario's [`Engine::rom_overlay`].
    pub fn rom_byte(&self, addr: usize) -> u8 {
        match self.rom_overlay.iter().find(|&&(a, _)| a == addr) {
            Some(&(_, v)) => v,
            None => super::rom::byte(addr),
        }
    }

    // --- the random stream ----------------------------------------------------

    fn next_rng_byte(&mut self) -> u8 {
        let mut x = self.rng;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.rng = x;
        x as u8
    }

    /// `Random`: two bytes from the stream into `hRandomAdd` / `hRandomSub`;
    /// returns the second. (The harness replaces the original's DIV mixing
    /// with this seeded stream; see docs/goal.md.)
    pub fn random(&mut self, caller: &'static str) -> u8 {
        let add = self.next_rng_byte();
        self.mem.wb(H_RANDOM_ADD, add);
        let sub = self.next_rng_byte();
        self.mem.wb(H_RANDOM_SUB, sub);
        if let Some(trace) = self.rng_trace.as_mut() {
            trace.push((caller, sub));
        }
        sub
    }

    /// `BattleRandom`: `Random` (non-link and harness link battles alike),
    /// leaving the value in `wPredefHL + 1` like the original.
    pub fn battle_random(&mut self, caller: &'static str) -> u8 {
        let a = self.random(caller);
        self.mem.wb(W_PREDEF_HL + 1, a);
        a
    }

    // --- turns and battle variables ---------------------------------------

    /// `hBattleTurn`: 0 = player, 1 = enemy.
    pub fn turn(&self) -> u8 {
        self.mem.rb(H_BATTLE_TURN)
    }

    pub fn is_player_turn(&self) -> bool {
        self.turn() == 0
    }

    pub fn set_player_turn(&mut self) {
        self.mem.wb(H_BATTLE_TURN, 0);
    }

    pub fn set_enemy_turn(&mut self) {
        self.mem.wb(H_BATTLE_TURN, 1);
    }

    /// `SwitchTurnCore`.
    pub fn switch_turn(&mut self) {
        let t = self.turn() ^ 1;
        self.mem.wb(H_BATTLE_TURN, t);
    }

    /// `GetBattleVarAddr`.
    pub fn battle_var_addr(&self, var: u8) -> u16 {
        // BattleVarLocations: (player, enemy) per location.
        const LOCATIONS: [(u16, u16); 13] = [
            (W_PLAYER_SUB_STATUS1, W_ENEMY_SUB_STATUS1),
            (W_PLAYER_SUB_STATUS2, W_ENEMY_SUB_STATUS2),
            (W_PLAYER_SUB_STATUS3, W_ENEMY_SUB_STATUS3),
            (W_PLAYER_SUB_STATUS4, W_ENEMY_SUB_STATUS4),
            (W_PLAYER_SUB_STATUS5, W_ENEMY_SUB_STATUS5),
            (W_BATTLE_MON_STATUS, W_ENEMY_MON_STATUS),
            (W_PLAYER_MOVE_STRUCT_ANIMATION, W_ENEMY_MOVE_STRUCT_ANIMATION),
            (W_PLAYER_MOVE_STRUCT_EFFECT, W_ENEMY_MOVE_STRUCT_EFFECT),
            (W_PLAYER_MOVE_STRUCT_POWER, W_ENEMY_MOVE_STRUCT_POWER),
            (W_PLAYER_MOVE_STRUCT_TYPE, W_ENEMY_MOVE_STRUCT_TYPE),
            (W_CUR_PLAYER_MOVE, W_CUR_ENEMY_MOVE),
            (W_LAST_PLAYER_COUNTER_MOVE, W_LAST_ENEMY_COUNTER_MOVE),
            (W_LAST_PLAYER_MOVE, W_LAST_ENEMY_MOVE),
        ];
        // BattleVarPairs: location index, and whether it is the opponent's.
        let (loc, opp) = match var {
            0..=4 => (var as usize, false),
            5..=9 => ((var - 5) as usize, true),
            10 => (5, false),
            11 => (5, true),
            12..=16 => ((var - 6) as usize, false),
            17 => (11, false),
            18 => (11, true),
            19 => (12, false),
            20 => (12, true),
            _ => panic!("bad battle var {var}"),
        };
        let (player, enemy) = LOCATIONS[loc];
        if (self.turn() != 0) != opp { enemy } else { player }
    }

    /// `GetBattleVar`.
    pub fn battle_var(&self, var: u8) -> u8 {
        self.mem.rb(self.battle_var_addr(var))
    }

    pub fn set_battle_var(&mut self, var: u8, value: u8) {
        let addr = self.battle_var_addr(var);
        self.mem.wb(addr, value);
    }

    pub fn battle_var_bit(&self, var: u8, bit: u8) -> bool {
        self.battle_var(var) & 1 << bit != 0
    }

    pub fn set_battle_var_bit(&mut self, var: u8, bit: u8) {
        let addr = self.battle_var_addr(var);
        self.mem.set_bit(addr, bit);
    }

    pub fn res_battle_var_bit(&mut self, var: u8, bit: u8) {
        let addr = self.battle_var_addr(var);
        self.mem.res_bit(addr, bit);
    }

    /// The user's / target's `battle_struct` (`wBattleMon` or `wEnemyMon`).
    pub fn user_mon(&self) -> u16 {
        if self.is_player_turn() { W_BATTLE_MON } else { W_ENEMY_MON }
    }

    pub fn target_mon(&self) -> u16 {
        if self.is_player_turn() { W_ENEMY_MON } else { W_BATTLE_MON }
    }

    pub fn link_mode(&self) -> u8 {
        self.mem.rb(W_LINK_MODE)
    }

    pub fn battle_mode(&self) -> u8 {
        self.mem.rb(W_BATTLE_MODE)
    }

    // --- text -------------------------------------------------------------------

    /// `StdBattleTextbox` / `PrintText` with the text at `label`.
    pub fn text(&mut self, label: usize) {
        self.text_with(label, &[]);
    }

    /// `text`, for a text whose `text_asm` choices the battle code computed
    /// (in order).
    pub fn text_with(&mut self, label: usize, choices: &[usize]) {
        // PlaceEnemysName in a (non-rival) trainer battle runs
        // Battle_GetTrainerName as the text prints.
        // (Beat Up's text, which can print garbage, does that as it renders.)
        let side_effects = label == super::rsym::BEAT_UP_ATTACK_TEXT;
        if !side_effects
            && self.mem.rb(W_BATTLE_MODE) == crate::defs::TRAINER_BATTLE
            && self.link_mode() == 0
            && !matches!(self.mem.rb(W_TRAINER_CLASS), crate::defs::RIVAL1 | crate::defs::RIVAL2)
            && self.text_places_trainer_name(label, choices)
        {
            self.battle_get_trainer_name();
        }
        self.texts.push(TextRef(label));
        // Printing only writes memory through glitches: Beat Up's names past
        // the party can hold a <MOBILE> script (see text.rs), so that text is
        // always run.
        if self.rendered.is_some() || self.events.is_some() || side_effects {
            let messages = self.render_text(label, choices);
            if let Some(events) = self.events.as_mut() {
                events.extend(messages.iter().cloned().map(crate::bootstrap::events::Event::Text));
            }
            if let Some(rendered) = self.rendered.as_mut() {
                rendered.extend(messages);
            }
        }
    }

    // --- decisions --------------------------------------------------------------

    /// Take the next decision for `site`, or suspend if none is queued.
    pub fn decide(&mut self, site: PromptSite) -> Flow<Step> {
        let legal = crate::bootstrap::cli::legal_steps(&self.mem, site);
        if let Some(log) = self.decision_log.as_mut() {
            log.push((site, legal.clone(), self.mem.clone(), self.rng));
            if let Some(r) = self.rendered.as_mut() {
                self.text_log.push(std::mem::take(r));
            }
        }
        match self.pending.pop_front() {
            Some(step) => {
                assert!(legal.contains(&step), "illegal {step:?} at {site:?}; legal: {legal:?}");
                Ok(step)
            }
            None => Err(Suspend { site, legal }),
        }
    }
}

/// `PP_MASK` and other small constants used across the port.
pub const PP_MASK: u8 = defs::PP_MASK;

/// `PARTYMON_STRUCT_LENGTH` as an address offset.
pub const PARTY_LEN: u16 = defs::PARTYMON_STRUCT_LENGTH as u16;

/// rgbasm's `n percent`: `n * $ff / 100`.
pub const fn percent(n: u32) -> u8 {
    (n * 0xff / 100) as u8
}
