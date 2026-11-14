//! Typed view of the battle state, read straight from the engine's WRAM.
//!
//! Everything here is a copy of what the original game keeps in memory at a
//! decision point — nothing is derived or simplified — so an agent observing a
//! [`BattleState`] sees exactly what the engine will act on. Ids (species,
//! moves, items, types) are the game's own byte values; see
//! [`crate::bootstrap::names`] for display names.
//!
//! Addresses come from `scratch/pokecrystal/pokecrystal.sym`; struct layouts
//! from `macros/ram.asm` (`battle_struct`, `party_struct`).

use crate::runtime::Cpu;

/// `battle_struct` (`wBattleMon` / `wEnemyMon`): the active mon, including the
/// in-battle copies of its stats (which stat stages, paralysis and burn have
/// already modified).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BattleMon {
    pub species: u8,
    pub item: u8,
    pub moves: [u8; 4],
    /// Raw DV word (attack/defense nibbles, speed/special nibbles).
    pub dvs: u16,
    /// Raw PP bytes: low 6 bits current PP, top 2 bits PP Ups.
    pub pp: [u8; 4],
    pub happiness: u8,
    pub level: u8,
    /// Status byte: bits 0-2 sleep turns, then PSN, BRN, FRZ, PAR.
    pub status: u8,
    pub hp: u16,
    pub max_hp: u16,
    /// Attack, Defense, Speed, Sp. Atk, Sp. Def.
    pub stats: [u16; 5],
    pub types: [u8; 2],
}

/// `party_struct`: one party member (outside-battle stats).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PartyMon {
    pub species: u8,
    pub item: u8,
    pub moves: [u8; 4],
    pub dvs: u16,
    pub pp: [u8; 4],
    pub happiness: u8,
    pub level: u8,
    pub status: u8,
    pub hp: u16,
    pub max_hp: u16,
    /// Attack, Defense, Speed, Sp. Atk, Sp. Def.
    pub stats: [u16; 5],
}

/// One side's battle-only state: the active mon plus every per-side WRAM
/// counter and flag the engine keeps.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SideState {
    /// Party slot of the active mon (`wCurBattleMon` / `wCurOTMon`).
    pub active_slot: u8,
    pub active: BattleMon,
    pub party: Vec<PartyMon>,
    /// Absolute stat stage levels 1..=13 (7 is neutral) for Attack, Defense,
    /// Speed, Sp. Atk, Sp. Def, Accuracy, Evasion (`w*StatLevels`).
    pub stat_levels: [u8; 7],
    /// `w*SubStatus1..5` raw bytes (see `constants/battle_constants.asm`).
    pub substatus: [u8; 5],
    pub rollout_count: u8,
    pub confuse_count: u8,
    pub toxic_count: u8,
    /// High nibble: Disabled move's 1-based slot; low nibble: turns left.
    pub disable_count: u8,
    pub encore_count: u8,
    pub perish_count: u8,
    pub fury_cutter_count: u8,
    pub protect_count: u8,
    pub damage_taken: u16,
    pub substitute_hp: u8,
    pub wrap_count: u8,
    pub charging: u8,
    pub rage_counter: u8,
    pub future_sight_count: u8,
    /// `w*Screens`: bit 0 Spikes, bit 2 Safeguard, bit 3 Light Screen, bit 4 Reflect.
    pub screens: u8,
    pub safeguard_count: u8,
    pub light_screen_count: u8,
    pub reflect_count: u8,
    pub last_move: u8,
    pub last_counter_move: u8,
    pub minimized: u8,
    pub turns_taken: u8,
}

/// The whole battle at one decision point (or at the end).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BattleState {
    /// `wBattleMode`: 1 = wild, 2 = trainer.
    pub battle_mode: u8,
    /// `wLinkMode`: non-zero in a link battle.
    pub link_mode: u8,
    /// `wBattleWeather`: 0 none, 1 rain, 2 sun, 3 sandstorm.
    pub weather: u8,
    pub weather_count: u8,
    pub player: SideState,
    pub enemy: SideState,
}

// --- addresses ---------------------------------------------------------------

const W_BATTLE_MON: u16 = 0xc62c;
const W_ENEMY_MON: u16 = 0xd206;
const W_CUR_BATTLE_MON: u16 = 0xd0d4;
const W_CUR_OT_MON: u16 = 0xc663;
const W_PARTY_COUNT: u16 = 0xdcd7;
const W_PARTY_MON1: u16 = 0xdcdf;
const W_OT_PARTY_COUNT: u16 = 0xd280;
const W_OT_PARTY_MON1: u16 = 0xd288;
const PARTY_STRUCT_LEN: u16 = 48;
const W_BATTLE_MODE: u16 = 0xd22d;
const W_LINK_MODE: u16 = 0xc2dc;
const W_BATTLE_WEATHER: u16 = 0xc70a;
const W_WEATHER_COUNT: u16 = 0xc70b;

/// Per-side WRAM addresses. Player and enemy copies sit at fixed offsets from
/// each other for most counters, but not all, so each is listed explicitly.
struct SideAddrs {
    mon: u16,
    cur_slot: u16,
    party_count: u16,
    party_mon1: u16,
    stat_levels: u16,
    substatus1: u16,
    rollout: u16,
    damage_taken: u16,
    substitute_hp: u16,
    wrap: u16,
    charging: u16,
    rage: u16,
    future_sight: u16,
    screens: u16,
    safeguard: u16,
    light_screen: u16,
    reflect: u16,
    last_move: u16,
    last_counter_move: u16,
    minimized: u16,
    turns_taken: u16,
}

const PLAYER: SideAddrs = SideAddrs {
    mon: W_BATTLE_MON,
    cur_slot: W_CUR_BATTLE_MON,
    party_count: W_PARTY_COUNT,
    party_mon1: W_PARTY_MON1,
    stat_levels: 0xc6cc,
    substatus1: 0xc668,
    rollout: 0xc672,
    damage_taken: 0xc682,
    substitute_hp: 0xc6df,
    wrap: 0xc730,
    charging: 0xc732,
    rage: 0xc72b,
    future_sight: 0xc71d,
    screens: 0xc6ff,
    safeguard: 0xc701,
    light_screen: 0xc702,
    reflect: 0xc703,
    last_move: 0xc71b,
    last_counter_move: 0xc6f8,
    minimized: 0xc6fe,
    turns_taken: 0xc6dd,
};

const ENEMY: SideAddrs = SideAddrs {
    mon: W_ENEMY_MON,
    cur_slot: W_CUR_OT_MON,
    party_count: W_OT_PARTY_COUNT,
    party_mon1: W_OT_PARTY_MON1,
    stat_levels: 0xc6d4,
    substatus1: 0xc66d,
    rollout: 0xc67a,
    damage_taken: 0xc684,
    substitute_hp: 0xc6e0,
    wrap: 0xc731,
    charging: 0xc733,
    rage: 0xc72c,
    future_sight: 0xc71e,
    screens: 0xc700,
    safeguard: 0xc705,
    light_screen: 0xc706,
    reflect: 0xc707,
    last_move: 0xc71c,
    last_counter_move: 0xc6f9,
    minimized: 0xc6fa,
    turns_taken: 0xc6dc,
};

// --- reading -------------------------------------------------------------------

fn be16(cpu: &Cpu, addr: u16) -> u16 {
    u16::from(cpu.read_byte(addr)) << 8 | u16::from(cpu.read_byte(addr + 1))
}

fn bytes<const N: usize>(cpu: &Cpu, addr: u16) -> [u8; N] {
    std::array::from_fn(|i| cpu.read_byte(addr + i as u16))
}

fn stats(cpu: &Cpu, addr: u16) -> [u16; 5] {
    std::array::from_fn(|i| be16(cpu, addr + 2 * i as u16))
}

impl BattleMon {
    fn read(cpu: &Cpu, base: u16) -> Self {
        BattleMon {
            species: cpu.read_byte(base),
            item: cpu.read_byte(base + 1),
            moves: bytes(cpu, base + 2),
            dvs: be16(cpu, base + 6),
            pp: bytes(cpu, base + 8),
            happiness: cpu.read_byte(base + 12),
            level: cpu.read_byte(base + 13),
            status: cpu.read_byte(base + 14),
            hp: be16(cpu, base + 16),
            max_hp: be16(cpu, base + 18),
            stats: stats(cpu, base + 20),
            types: bytes(cpu, base + 30),
        }
    }
}

impl PartyMon {
    fn read(cpu: &Cpu, base: u16) -> Self {
        PartyMon {
            species: cpu.read_byte(base),
            item: cpu.read_byte(base + 1),
            moves: bytes(cpu, base + 2),
            dvs: be16(cpu, base + 21),
            pp: bytes(cpu, base + 23),
            happiness: cpu.read_byte(base + 27),
            level: cpu.read_byte(base + 31),
            status: cpu.read_byte(base + 32),
            hp: be16(cpu, base + 34),
            max_hp: be16(cpu, base + 36),
            stats: stats(cpu, base + 38),
        }
    }
}

impl SideState {
    fn read(cpu: &Cpu, a: &SideAddrs) -> Self {
        let count = cpu.read_byte(a.party_count).min(6);
        let rollout = a.rollout;
        SideState {
            active_slot: cpu.read_byte(a.cur_slot),
            active: BattleMon::read(cpu, a.mon),
            party: (0..u16::from(count))
                .map(|i| PartyMon::read(cpu, a.party_mon1 + i * PARTY_STRUCT_LEN))
                .collect(),
            stat_levels: bytes(cpu, a.stat_levels),
            substatus: bytes(cpu, a.substatus1),
            rollout_count: cpu.read_byte(rollout),
            confuse_count: cpu.read_byte(rollout + 1),
            toxic_count: cpu.read_byte(rollout + 2),
            disable_count: cpu.read_byte(rollout + 3),
            encore_count: cpu.read_byte(rollout + 4),
            perish_count: cpu.read_byte(rollout + 5),
            fury_cutter_count: cpu.read_byte(rollout + 6),
            protect_count: cpu.read_byte(rollout + 7),
            damage_taken: be16(cpu, a.damage_taken),
            substitute_hp: cpu.read_byte(a.substitute_hp),
            wrap_count: cpu.read_byte(a.wrap),
            charging: cpu.read_byte(a.charging),
            rage_counter: cpu.read_byte(a.rage),
            future_sight_count: cpu.read_byte(a.future_sight),
            screens: cpu.read_byte(a.screens),
            safeguard_count: cpu.read_byte(a.safeguard),
            light_screen_count: cpu.read_byte(a.light_screen),
            reflect_count: cpu.read_byte(a.reflect),
            last_move: cpu.read_byte(a.last_move),
            last_counter_move: cpu.read_byte(a.last_counter_move),
            minimized: cpu.read_byte(a.minimized),
            turns_taken: cpu.read_byte(a.turns_taken),
        }
    }
}

impl BattleState {
    /// Read the current battle state from the engine's memory.
    pub fn read(cpu: &Cpu) -> Self {
        BattleState {
            battle_mode: cpu.read_byte(W_BATTLE_MODE),
            link_mode: cpu.read_byte(W_LINK_MODE),
            weather: cpu.read_byte(W_BATTLE_WEATHER),
            weather_count: cpu.read_byte(W_WEATHER_COUNT),
            player: SideState::read(cpu, &PLAYER),
            enemy: SideState::read(cpu, &ENEMY),
        }
    }
}
