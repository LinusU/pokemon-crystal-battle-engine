//! Drive the transpiled Crystal engine into a battle from external config.
//!
//! The strategy is "direct inject": run the hardware init from the normal boot
//! path (with `Cpu::skip_intro_to_battle = true` so we bail before the title /
//! intro sequence), then seed only the WRAM bytes the battle engine reads, and
//! call into `start_battle`.
//!
//! Long-term, the same seeding logic feeds the headless state machine; the
//! TOML + glium window in `main.rs` is just one consumer.

pub mod species;

use serde::Deserialize;

use crate::defs;
use crate::runtime::Cpu;

#[derive(Debug, Deserialize)]
pub struct BattleConfig {
    pub battle: BattleHeader,
    #[serde(default)]
    pub player_party: Vec<MonSpec>,
    #[serde(default)]
    pub enemy_party: Vec<MonSpec>,
}

#[derive(Debug, Deserialize)]
pub struct BattleHeader {
    #[serde(rename = "type")]
    pub kind: BattleKind,
}

#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BattleKind {
    WildEncounter,
}

#[derive(Debug, Deserialize)]
pub struct MonSpec {
    pub species: String,
    pub level: u8,
}

#[derive(Debug)]
pub enum LoadError {
    Io(std::io::Error),
    Parse(toml::de::Error),
}

impl std::fmt::Display for LoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoadError::Io(e) => write!(f, "could not read battle file: {e}"),
            LoadError::Parse(e) => write!(f, "could not parse battle file: {e}"),
        }
    }
}

impl std::error::Error for LoadError {}

pub fn load_battle(path: &std::path::Path) -> Result<BattleConfig, LoadError> {
    let text = std::fs::read_to_string(path).map_err(LoadError::Io)?;
    toml::from_str(&text).map_err(LoadError::Parse)
}

// WRAM addresses we touch. Source of truth: scratch/pokecrystal/pokecrystal.sym.
// Keep this in sync with docs/memory_layout.md.
const W_LINK_MODE: u16 = 0xc2dc;
const W_MON_TYPE: u16 = 0xcf5f;
const W_CUR_SPECIES: u16 = 0xcf60;
const W_IN_BATTLE_TOWER_BATTLE: u16 = 0xcfc0;
const W_CUR_PARTY_SPECIES: u16 = 0xd108;
const W_CUR_PARTY_LEVEL: u16 = 0xd143;
const W_TEMP_ENEMY_MON_SPECIES: u16 = 0xd204;
const W_ENEMY_MON_ITEM: u16 = 0xd207;
const W_BATTLE_MODE: u16 = 0xd22d;
const W_TEMP_WILD_MON_SPECIES: u16 = 0xd22e;
const W_OTHER_TRAINER_CLASS: u16 = 0xd22f;
const W_BATTLE_TYPE: u16 = 0xd230;
const W_TIME_OF_DAY: u16 = 0xd269;
const W_OT_PARTY_COUNT: u16 = 0xd280;
const W_PLAYER_GENDER: u16 = 0xd472;
const W_PLAYER_ID: u16 = 0xd47b;
const W_PLAYER_NAME: u16 = 0xd47d;
const W_TIME_OF_DAY_PAL: u16 = 0xd841;
const W_PARTY_COUNT: u16 = 0xdcd7;

// Bag pocket counts. Each list expects (count_byte, terminator_byte=0xff) at
// minimum so the menu code's "walk until 0xff" loops don't run off the end.
const W_NUM_ITEMS: u16 = 0xd892;
const W_NUM_KEY_ITEMS: u16 = 0xd8bc;
const W_NUM_BALLS: u16 = 0xd8d7;
const W_NUM_PC_ITEMS: u16 = 0xd8f1;

const CHARMAP_TERMINATOR: u8 = 0x50;

/// Banks of the engine routines we drive directly. Source: pokecrystal.sym.
const BANK_TRY_ADD_MON_TO_PARTY: u8 = 0x03;
const BANK_START_BATTLE: u8 = 0x0f;

/// Mirror what the engine's `Bankswitch` routine does: update the in-HRAM
/// shadow of the current bank (`hROMBank` = `0xff9d`) and the MBC bank-select
/// register so subsequent ROM reads from 0x4000..0x7fff hit the right bank.
fn switch_bank(cpu: &mut Cpu, bank: u8) {
    cpu.write_byte(0xff9d, bank);
    cpu.write_byte(crate::defs::R_ROMB as u16, bank);
}

/// Encode an ASCII string to Crystal's text encoding. Truncates and pads with
/// the `@` terminator to `NAME_LENGTH` (11 bytes).
fn encode_name(text: &str) -> [u8; defs::NAME_LENGTH as usize] {
    let mut out = [CHARMAP_TERMINATOR; defs::NAME_LENGTH as usize];
    let slot = (defs::NAME_LENGTH as usize) - 1; // reserve last byte for terminator
    for (i, c) in text.chars().take(slot).enumerate() {
        out[i] = match c {
            'A'..='Z' => 0x80 + (c as u8 - b'A'),
            'a'..='z' => 0xa0 + (c as u8 - b'a'),
            '0'..='9' => 0xf6 + (c as u8 - b'0'),
            ' ' => 0x7f,
            _ => CHARMAP_TERMINATOR,
        };
    }
    out
}

/// Seed the minimal "I'm a real player" state the battle engine reads:
/// name, ID, gender, time-of-day, and a few zero-init guards.
fn seed_player(cpu: &mut Cpu) {
    let name = encode_name("RED");
    for (i, b) in name.iter().enumerate() {
        cpu.write_byte(W_PLAYER_NAME + i as u16, *b);
    }
    cpu.write_byte(W_PLAYER_ID, 0x12);
    cpu.write_byte(W_PLAYER_ID + 1, 0x34);
    cpu.write_byte(W_PLAYER_GENDER, 0); // 0 = male; matters for some text only

    cpu.write_byte(W_TIME_OF_DAY, defs::DAY_F as u8);
    cpu.write_byte(W_TIME_OF_DAY_PAL, 0);

    cpu.write_byte(W_LINK_MODE, 0);
    cpu.write_byte(W_IN_BATTLE_TOWER_BATTLE, 0);
    cpu.write_byte(W_OT_PARTY_COUNT, 0);

    // Empty bag pockets, written like _ResetWRAM.InitList does:
    // count = 0, first slot = 0xff (terminator). Without the terminator the
    // pack menu walks past the end of the list and corrupts neighbouring WRAM.
    for list_addr in [W_NUM_ITEMS, W_NUM_KEY_ITEMS, W_NUM_BALLS, W_NUM_PC_ITEMS] {
        cpu.write_byte(list_addr, 0);
        cpu.write_byte(list_addr + 1, 0xff);
    }
}

/// Append one player party slot by driving the engine's own
/// `try_add_mon_to_party` so DVs, stats, moves, and PP all come out canonical.
fn add_player_mon(cpu: &mut Cpu, species: u8, level: u8) {
    cpu.write_byte(W_CUR_PARTY_SPECIES, species);
    cpu.write_byte(W_CUR_PARTY_LEVEL, level);
    cpu.write_byte(W_CUR_SPECIES, species);
    cpu.write_byte(W_MON_TYPE, 0); // PARTYMON
    cpu.write_byte(W_BATTLE_MODE, 0); // 0 = not in battle yet, so default item / random moves
    cpu.write_byte(W_ENEMY_MON_ITEM, 0);

    switch_bank(cpu, BANK_TRY_ADD_MON_TO_PARTY);
    let return_sentinel: u16 = 0xfffe;
    cpu.push_word(return_sentinel);
    crate::game::engine::pokemon::move_mon::try_add_mon_to_party(cpu);
}

fn launch_wild_encounter(cpu: &mut Cpu, config: &BattleConfig) -> Result<(), String> {
    if config.player_party.is_empty() {
        return Err("wild_encounter requires at least one player_party entry".into());
    }
    if config.enemy_party.len() != 1 {
        return Err(format!(
            "wild_encounter expects exactly one enemy_party entry, got {}",
            config.enemy_party.len()
        ));
    }

    seed_player(cpu);
    cpu.write_byte(W_PARTY_COUNT, 0);

    for mon in &config.player_party {
        let id = species::id_from_name(&mon.species)
            .ok_or_else(|| format!("unknown player species: {}", mon.species))?;
        add_player_mon(cpu, id, mon.level);
    }

    let enemy = &config.enemy_party[0];
    let enemy_id = species::id_from_name(&enemy.species)
        .ok_or_else(|| format!("unknown enemy species: {}", enemy.species))?;

    cpu.write_byte(W_BATTLE_MODE, defs::WILD_BATTLE as u8);
    cpu.write_byte(W_BATTLE_TYPE, defs::BATTLETYPE_NORMAL as u8);
    cpu.write_byte(W_OTHER_TRAINER_CLASS, 0);
    cpu.write_byte(W_TEMP_WILD_MON_SPECIES, enemy_id);
    cpu.write_byte(W_TEMP_ENEMY_MON_SPECIES, enemy_id);
    cpu.write_byte(W_CUR_PARTY_LEVEL, enemy.level);
    cpu.write_byte(W_CUR_PARTY_SPECIES, enemy_id);

    switch_bank(cpu, BANK_START_BATTLE);
    let return_sentinel: u16 = 0xfffd;
    cpu.push_word(return_sentinel);
    crate::game::engine::battle::core::start_battle(cpu);
    Ok(())
}

/// Drive the boot to the post-init "ready" state and then launch whatever
/// battle the config describes. Blocks for the duration of the battle.
pub fn run(cpu: &mut Cpu, config: &BattleConfig) -> Result<(), String> {
    cpu.skip_intro_to_battle = true;
    let entry = crate::entry_routine();
    (entry.func)(cpu);

    match config.battle.kind {
        BattleKind::WildEncounter => launch_wild_encounter(cpu, config)?,
    }
    Ok(())
}
