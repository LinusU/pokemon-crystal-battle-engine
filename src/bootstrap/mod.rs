//! Drive the transpiled Crystal engine into a battle from external config.
//!
//! The strategy is "direct inject": run the hardware init from the normal boot
//! path (with `Cpu::skip_intro_to_battle = true` so we bail before the title /
//! intro sequence), then seed only the WRAM bytes the battle engine reads, and
//! call into `start_battle`.
//!
//! Long-term, the same seeding logic feeds the headless state machine; the
//! TOML + glium window in `main.rs` is just one consumer.

pub mod cli;
pub mod items;
pub mod move_pp;
pub mod names;
pub mod player;
pub mod random;
pub mod recorder;
pub mod snapshot;
pub mod species;
pub mod trainers;
pub mod trainers_data;

use serde::Deserialize;

use crate::defs;
use crate::runtime::Cpu;

#[derive(Debug, Clone, Deserialize)]
pub struct BattleConfig {
    pub battle: BattleHeader,
    #[serde(default)]
    pub player_party: Vec<MonSpec>,
    #[serde(default)]
    pub enemy_party: Vec<MonSpec>,
    #[serde(default)]
    pub player_items: Vec<ItemSpec>,
    #[serde(default)]
    pub enemy_items: Vec<ItemSpec>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BattleHeader {
    #[serde(rename = "type")]
    pub kind: BattleKind,
    /// Required for `trainer_battle`. Resolved via `bootstrap::trainers`.
    pub trainer: Option<String>,
    /// Seed for the deterministic battle RNG. If unset, the harness picks
    /// one from system entropy and logs it. Accepts decimal (`42`) or hex
    /// (`0xdeadbeef`) literals via TOML's integer parser.
    pub seed: Option<u64>,
}

#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BattleKind {
    WildEncounter,
    TrainerBattle,
    /// A headless link battle: both parties are fully specified in the TOML and
    /// BOTH sides are CLI-driven (no AI). The harness seeds both `wParty*` and
    /// `wOTParty*`, sets `wLinkMode` so the engine takes its real link path, and
    /// feeds the enemy's per-round action via the `link_battle_send_receive_action`
    /// hook. See [`launch_link_battle`].
    LinkBattle,
}

impl BattleKind {
    /// The `[battle] type` string this kind deserializes from — the inverse of
    /// the `snake_case` serde rename. Used by [`recorder`] to write an embedded
    /// scenario block that [`load_battle`] / [`player`] can read straight back.
    pub fn as_str(self) -> &'static str {
        match self {
            BattleKind::WildEncounter => "wild_encounter",
            BattleKind::TrainerBattle => "trainer_battle",
            BattleKind::LinkBattle => "link_battle",
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct MonSpec {
    pub species: String,
    pub level: u8,
    /// Optional explicit moveset (1..=4 move names). When omitted, the engine's
    /// generated level-up moveset is kept. When present, it overrides the
    /// generated moves of a `[[player_party]]` slot and refills PP to each
    /// move's base maximum. Honored only for the player party — see
    /// [`seed_player_party`]. Resolved via [`names::move_id_from_name`].
    #[serde(default)]
    pub moves: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ItemSpec {
    pub name: String,
    pub quantity: u8,
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
const W_OTHER_TRAINER_ID: u16 = 0xd231;
const W_TIME_OF_DAY: u16 = 0xd269;
const W_OT_PARTY_COUNT: u16 = 0xd280;
const W_PLAYER_GENDER: u16 = 0xd472;
const W_PLAYER_ID: u16 = 0xd47b;
const W_PLAYER_NAME: u16 = 0xd47d;
const W_TIME_OF_DAY_PAL: u16 = 0xd841;
const W_PARTY_COUNT: u16 = 0xdcd7;
const W_PARTY_MON1: u16 = 0xdcdf;
// OT (enemy) party list — same `party_struct` layout as the player party. Used
// by the link-battle path to pre-seed the enemy's whole party.
const W_OT_PARTY_MON1: u16 = 0xd288;
// `wCurOTMon` (0xc663): the enemy's active party slot. `wCurPartyMon` (0xd109):
// the scratch "current mon" slot `InitEnemyMon` copies into `wCurOTMon` when the
// first enemy mon is sent out. Both are normally zeroed by the ROM trainer-party
// load, which the link path early-returns out of — so we seed them to 0
// ourselves, else `InitEnemyMon` makes the enemy's active slot a garbage index
// and the engine reads/writes the OT party far out of bounds. `00:c663
// wCurOTMon`, `00:d109 wCurPartyMon` in `pokecrystal.sym`.
const W_CUR_OT_MON: u16 = 0xc663;
const W_CUR_PARTY_MON: u16 = 0xd109;
// The opponent's class name (`wOTClassName`, 0xc656) and link player name
// (`wOTPlayerName`, 0xd26b). In a real link battle these are filled by the link
// setup / `GetTrainerAttributes`; in our headless setup `GetTrainerAttributes`
// early-returns on `wLinkMode != 0`, so they hold unterminated garbage. The
// battle-start / "sent out" textboxes then walk the garbage past the tilemap and
// clobber neighbouring battle WRAM (`wOTClassName` is only 13 bytes below
// `wCurOTMon`). We seed terminated names so the text stays bounded.
const W_OT_CLASS_NAME: u16 = 0xc656;
const W_OT_PLAYER_NAME: u16 = 0xd26b;
// Per-mon nickname lists. `try_add_mon_to_party` fills the player list
// (`wPartyMonNicknames`) but SKIPS it for OT mons (`wMonType != PARTYMON`) — in
// a real link battle the partner supplies those names. Headless we must seed
// `wOTPartyMonNicknames` ourselves or the enemy mon's name renders as garbage.
// `get_pokemon_name` writes the ROM species name into `wStringBuffer1`.
// `01:d3ea wOTPartyMonNicknames`, `01:d073 wStringBuffer1`, `01:d265
// wNamedObjectIndex` in `pokecrystal.sym`.
const W_OT_PARTY_MON_NICKNAMES: u16 = 0xd3ea;
const W_STRING_BUFFER1: u16 = 0xd073;
const W_NAMED_OBJECT_INDEX: u16 = 0xd265;

// `hSerialConnectionStatus` (0xffcb). In a link battle `do_battle` reads this:
// `== USING_INTERNAL_CLOCK` makes the local side player 2 (its enemy mon is
// assumed already set up by the partner). We want the local side to be player 1
// so the engine runs the first-enemy-mon send-out itself, so we seed
// `USING_EXTERNAL_CLOCK`. Turn order on a speed tie also keys off this.
const H_SERIAL_CONNECTION_STATUS: u16 = 0xffcb;

// `wMonType` values (constants/ram_constants.asm): which party `try_add_mon_to_party`
// appends to. 0 = the player party (`wParty*`), 1 = the OT party (`wOTParty*`).
const MONTYPE_PARTYMON: u8 = 0;
const MONTYPE_OTPARTYMON: u8 = 1;

// `party_struct` geometry, for the moveset override (see docs/memory_layout.md).
const PARTY_STRUCT_LEN: u16 = 48;
const MON_OFFSET_MOVES: u16 = 2; // MON_MOVES: 4 bytes, one id per slot
const MON_OFFSET_PP: u16 = 23; // MON_PP: 4 bytes, current PP per slot
const NUM_MOVE_SLOTS: usize = 4;

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
pub(crate) fn switch_bank(cpu: &mut Cpu, bank: u8) {
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

    // Key items and PC items are always empty in this harness. The Items and
    // Balls pockets are populated by `seed_player_items`, which writes the
    // count + 0xff terminator even for an empty config.
    for list_addr in [W_NUM_KEY_ITEMS, W_NUM_PC_ITEMS] {
        cpu.write_byte(list_addr, 0);
        cpu.write_byte(list_addr + 1, 0xff);
    }
}

/// Write the player's items into the Items and Balls pockets. Each pocket
/// stores `[count][id, qty]*[0xff]`, matching `_ResetWRAM.InitList`. The
/// terminator is mandatory — without it, the pack menu walks past the end of
/// the list and corrupts neighbouring WRAM.
fn seed_player_items(cpu: &mut Cpu, specs: &[ItemSpec]) -> Result<(), String> {
    let mut item_pocket: Vec<(u8, u8)> = Vec::new();
    let mut ball_pocket: Vec<(u8, u8)> = Vec::new();

    for spec in specs {
        let def = items::lookup(&spec.name)
            .ok_or_else(|| format!("unknown item: {}", spec.name))?;
        if spec.quantity == 0 || spec.quantity > defs::MAX_ITEM_STACK {
            return Err(format!(
                "item {:?} quantity must be 1..={}, got {}",
                spec.name,
                defs::MAX_ITEM_STACK,
                spec.quantity
            ));
        }
        let bucket = match def.pocket {
            items::Pocket::Item => &mut item_pocket,
            items::Pocket::Ball => &mut ball_pocket,
        };
        bucket.push((def.id, spec.quantity));
    }

    if item_pocket.len() > defs::MAX_ITEMS as usize {
        return Err(format!(
            "too many items in Items pocket: {} (max {})",
            item_pocket.len(),
            defs::MAX_ITEMS
        ));
    }
    if ball_pocket.len() > defs::MAX_BALLS as usize {
        return Err(format!(
            "too many balls in Balls pocket: {} (max {})",
            ball_pocket.len(),
            defs::MAX_BALLS
        ));
    }

    write_pocket(cpu, W_NUM_ITEMS, &item_pocket);
    write_pocket(cpu, W_NUM_BALLS, &ball_pocket);
    Ok(())
}

fn write_pocket(cpu: &mut Cpu, num_addr: u16, entries: &[(u8, u8)]) {
    cpu.write_byte(num_addr, entries.len() as u8);
    let mut addr = num_addr + 1;
    for (id, qty) in entries {
        cpu.write_byte(addr, *id);
        cpu.write_byte(addr + 1, *qty);
        addr += 2;
    }
    cpu.write_byte(addr, 0xff);
}

/// Append one party slot by driving the engine's own `try_add_mon_to_party` so
/// DVs, stats, moves, and PP all come out canonical. `mon_type` selects which
/// party (`MONTYPE_PARTYMON` → `wParty*`, `MONTYPE_OTPARTYMON` → `wOTParty*`).
fn add_mon(cpu: &mut Cpu, species: u8, level: u8, mon_type: u8) {
    cpu.write_byte(W_CUR_PARTY_SPECIES, species);
    cpu.write_byte(W_CUR_PARTY_LEVEL, level);
    cpu.write_byte(W_CUR_SPECIES, species);
    cpu.write_byte(W_MON_TYPE, mon_type);
    cpu.write_byte(W_BATTLE_MODE, 0); // 0 = not in battle yet, so default item / random moves
    cpu.write_byte(W_ENEMY_MON_ITEM, 0);

    switch_bank(cpu, BANK_TRY_ADD_MON_TO_PARTY);
    let return_sentinel: u16 = 0xfffe;
    cpu.push_word(return_sentinel);
    crate::game::engine::pokemon::move_mon::try_add_mon_to_party(cpu);
}

fn seed_player_party(cpu: &mut Cpu, party: &[MonSpec]) -> Result<(), String> {
    seed_party(cpu, party, MONTYPE_PARTYMON, W_PARTY_COUNT, W_PARTY_MON1, "player")
}

/// Seed a whole party into either the player's or the OT (enemy's) list,
/// honoring each mon's optional `moves` override. `count_addr` / `mon1_addr`
/// are the destination party's count byte and first `party_struct`; `mon_type`
/// routes `try_add_mon_to_party` to the matching list. `label` names the side in
/// error messages. Shared by the player party and the link-battle enemy party.
fn seed_party(
    cpu: &mut Cpu,
    party: &[MonSpec],
    mon_type: u8,
    count_addr: u16,
    mon1_addr: u16,
    label: &str,
) -> Result<(), String> {
    cpu.write_byte(count_addr, 0);
    for (slot, mon) in party.iter().enumerate() {
        let id = species::id_from_name(&mon.species)
            .ok_or_else(|| format!("unknown {label} species: {}", mon.species))?;
        // Resolve the optional moveset before driving the engine so a bad move
        // name fails fast, before any WRAM is touched for this slot.
        let move_ids = resolve_moves(mon)?;
        add_mon(cpu, id, mon.level, mon_type);
        if let Some(ids) = move_ids {
            apply_moveset(cpu, mon1_addr, slot as u8, &ids);
        }
        // `try_add_mon_to_party` sets the nickname only for the player party;
        // for OT mons it skips it (the link partner would supply the name), so
        // seed the OT nickname to the species name ourselves.
        if mon_type == MONTYPE_OTPARTYMON {
            seed_ot_nickname(cpu, slot as u8, id);
        }
    }
    Ok(())
}

/// Set OT party slot `slot`'s nickname to `species`'s ROM name, mirroring what
/// `try_add_mon_to_party` does for the player party (which it skips for OT
/// mons). Drives the engine's `get_pokemon_name` to fetch the exact name into
/// `wStringBuffer1`, then copies it into `wOTPartyMonNicknames[slot]`. Without
/// this the enemy mon's name in the battle HUD renders as uninitialised garbage.
fn seed_ot_nickname(cpu: &mut Cpu, slot: u8, species: u8) {
    cpu.write_byte(W_NAMED_OBJECT_INDEX, species);
    // `get_pokemon_name` saves/restores the ROM bank itself, so no switch needed.
    let return_sentinel: u16 = 0xfffc;
    cpu.push_word(return_sentinel);
    crate::game::home::names::get_pokemon_name(cpu);

    let dest = W_OT_PARTY_MON_NICKNAMES + slot as u16 * defs::MON_NAME_LENGTH as u16;
    for i in 0..defs::MON_NAME_LENGTH as u16 {
        let b = cpu.read_byte(W_STRING_BUFFER1 + i);
        cpu.write_byte(dest + i, b);
    }
}

/// Resolve a mon's optional `moves` list to move ids, validating the count and
/// every name. `None` means "keep the engine's generated moveset" and leaves
/// the slot untouched.
fn resolve_moves(mon: &MonSpec) -> Result<Option<Vec<u8>>, String> {
    let Some(names) = &mon.moves else {
        return Ok(None);
    };
    if names.is_empty() || names.len() > NUM_MOVE_SLOTS {
        return Err(format!(
            "{}: moves must list 1..={} entries, got {}",
            mon.species,
            NUM_MOVE_SLOTS,
            names.len()
        ));
    }
    let mut ids = Vec::with_capacity(names.len());
    for name in names {
        let id = names::move_id_from_name(name)
            .ok_or_else(|| format!("unknown move for {}: {}", mon.species, name))?;
        ids.push(id);
    }
    Ok(Some(ids))
}

/// Overwrite a freshly added party slot's moves and PP with an explicit
/// moveset. `try_add_mon_to_party` has already filled the slot with canonical
/// DVs, stats, and the level-up moveset; this touches only the `MON_MOVES` and
/// `MON_PP` bytes. Each given move gets full PP (its base maximum — a fresh mon
/// has no PP Ups); any leftover move slot is zeroed so the engine sees fewer
/// than four moves rather than a stale generated one.
fn apply_moveset(cpu: &mut Cpu, party_mon1_addr: u16, slot: u8, move_ids: &[u8]) {
    let base = party_mon1_addr + slot as u16 * PARTY_STRUCT_LEN;
    for i in 0..NUM_MOVE_SLOTS {
        let id = move_ids.get(i).copied().unwrap_or(0);
        cpu.write_byte(base + MON_OFFSET_MOVES + i as u16, id);
        cpu.write_byte(base + MON_OFFSET_PP + i as u16, move_pp::MOVE_BASE_PP[id as usize]);
    }
}

fn start_battle(cpu: &mut Cpu) {
    switch_bank(cpu, BANK_START_BATTLE);
    let return_sentinel: u16 = 0xfffd;
    cpu.push_word(return_sentinel);
    crate::game::engine::battle::core::start_battle(cpu);
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
    if !config.enemy_items.is_empty() {
        return Err("wild_encounter does not accept enemy_items".into());
    }

    seed_player(cpu);
    seed_player_items(cpu, &config.player_items)?;
    seed_player_party(cpu, &config.player_party)?;

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

    start_battle(cpu);
    Ok(())
}

fn launch_trainer_battle(cpu: &mut Cpu, config: &BattleConfig) -> Result<(), String> {
    if config.player_party.is_empty() {
        return Err("trainer_battle requires at least one player_party entry".into());
    }
    let trainer_name = config
        .battle
        .trainer
        .as_deref()
        .ok_or("trainer_battle requires a [battle] trainer = \"...\" field")?;
    let trainer = trainers::lookup(trainer_name)
        .ok_or_else(|| format!("unknown trainer: {trainer_name}"))?;

    seed_player(cpu);
    seed_player_items(cpu, &config.player_items)?;
    seed_player_party(cpu, &config.player_party)?;

    // The engine's `InitEnemyTrainer` reads `wOtherTrainerClass` /
    // `wOtherTrainerID` and uses them to load the canonical party from
    // `TrainerGroups` and the held items / base reward from
    // `TrainerClassAttributes`. The `[[enemy_party]]` and `[[enemy_items]]`
    // entries in the TOML are parsed but the engine still uses ROM data for
    // both — see docs/toml_schema.md.
    cpu.write_byte(W_BATTLE_MODE, defs::TRAINER_BATTLE as u8);
    cpu.write_byte(W_BATTLE_TYPE, defs::BATTLETYPE_NORMAL as u8);
    cpu.write_byte(W_OTHER_TRAINER_CLASS, trainer.class);
    cpu.write_byte(W_OTHER_TRAINER_ID, trainer.id);
    // wTempWildMonSpecies is left as-is; LoadTrainerOrWildMonPic checks
    // wOtherTrainerClass first and skips the wild path entirely.

    start_battle(cpu);
    Ok(())
}

/// Launch a link battle: both parties are fully specified and BOTH sides are
/// CLI-driven. Unlike a trainer battle, the enemy party is NOT loaded from ROM —
/// we seed `wOTParty*` ourselves and set `wLinkMode` so the engine takes its
/// real link path, which skips the AI, early-returns out of `ReadTrainerParty` /
/// `GetTrainerAttributes` (preserving our seeded party), and reads the enemy's
/// per-round action from `wBattleAction`. The `link_battle_send_receive_action`
/// HARNESS-PATCH fills that byte from the CLI / playback (see
/// [`cli::handle_enemy_action`]). See docs/bootstrap.md.
fn launch_link_battle(cpu: &mut Cpu, config: &BattleConfig) -> Result<(), String> {
    let max = defs::PARTY_LENGTH as usize;
    if config.player_party.is_empty() || config.player_party.len() > max {
        return Err(format!(
            "link_battle requires 1..={max} player_party entries, got {}",
            config.player_party.len()
        ));
    }
    if config.enemy_party.is_empty() || config.enemy_party.len() > max {
        return Err(format!(
            "link_battle requires 1..={max} enemy_party entries, got {}",
            config.enemy_party.len()
        ));
    }
    // A Gen-II link battle disables the bag — both sides only move and switch.
    if !config.player_items.is_empty() || !config.enemy_items.is_empty() {
        return Err(
            "link_battle does not accept player_items / enemy_items (the bag is disabled in a link battle)"
                .into(),
        );
    }

    seed_player(cpu);
    // No bag items, but the pack-menu pocket terminators must still be written.
    seed_player_items(cpu, &[])?;
    seed_player_party(cpu, &config.player_party)?;
    seed_party(
        cpu,
        &config.enemy_party,
        MONTYPE_OTPARTYMON,
        W_OT_PARTY_COUNT,
        W_OT_PARTY_MON1,
        "enemy",
    )?;

    // Switch the engine onto its real link path. `cpu.link_battle` gates the
    // three HARNESS-PATCHes (serial exchange → CLI, RNG → seeded LCG, battle end
    // → non-linked teardown). `wOtherTrainerClass = CAL` mirrors the real
    // colosseum-battle setup (engine/link/link.asm) so `InitEnemy` routes to the
    // multi-mon trainer path; `ReadTrainerParty` / `GetTrainerAttributes` then
    // early-return on `wLinkMode != 0`, leaving the party we seeded intact.
    // `USING_EXTERNAL_CLOCK` makes the local side player 1, so `DoBattle` sends
    // out the first enemy mon itself (player 2 assumes its partner did).
    cpu.link_battle = true;
    cpu.write_byte(W_LINK_MODE, defs::LINK_COLOSSEUM);
    cpu.write_byte(H_SERIAL_CONNECTION_STATUS, defs::USING_EXTERNAL_CLOCK);
    // The active-enemy-mon slot (and the scratch slot InitEnemyMon copies from)
    // are normally zeroed by the ROM party load we skip.
    cpu.write_byte(W_CUR_OT_MON, 0);
    cpu.write_byte(W_CUR_PARTY_MON, 0);
    // Seed terminated opponent names so the battle-start text stays bounded
    // (GetTrainerAttributes — which fills these — early-returns in link mode).
    let ot_name = encode_name("RIVAL");
    for (i, b) in ot_name.iter().enumerate() {
        cpu.write_byte(W_OT_PLAYER_NAME + i as u16, *b);
        cpu.write_byte(W_OT_CLASS_NAME + i as u16, *b);
    }
    cpu.write_byte(W_BATTLE_MODE, defs::TRAINER_BATTLE as u8);
    cpu.write_byte(W_BATTLE_TYPE, defs::BATTLETYPE_NORMAL as u8);
    cpu.write_byte(W_OTHER_TRAINER_CLASS, defs::CAL);

    start_battle(cpu);
    Ok(())
}

/// Drive the boot to the post-init "ready" state and then launch whatever
/// battle the config describes. Blocks for the duration of the battle.
pub fn run(cpu: &mut Cpu, config: &BattleConfig) -> Result<(), String> {
    run_with(cpu, config, RunOptions::default())
}

/// Runtime knobs for [`run`] that don't belong on the scenario TOML —
/// today the optional `--record` target path, the source scenario path used
/// for the recording header, and the parsed `--play` recording. All default
/// to `None`.
#[derive(Default)]
pub struct RunOptions<'a> {
    /// If `Some`, every player decision is appended as a `[[step]]` block
    /// to the file at this path. See [`recorder`].
    pub record_path: Option<&'a std::path::Path>,
    /// If `Some`, written into the recording header as `source_scenario`
    /// so the recorded file documents what it was driven from.
    pub source_scenario: Option<&'a std::path::Path>,
    /// If `true`, the resolved matchup is embedded in the recording as a
    /// `[scenario.*]` block (see [`recorder::install`]). Set for generated
    /// (`--random`) runs so the recording is self-contained and `--play`
    /// reconstructs the matchup without re-running the generator.
    pub embed_scenario: bool,
    /// If `Some`, the seed is forced to this value (the recording's
    /// resolved seed) instead of whatever the scenario carries. Set by
    /// `--play` so playback is bit-identical to the original run.
    pub seed_override: Option<u64>,
    /// If `Some`, the parsed playback step list is installed before any CLI
    /// prompt fires; each prompt site consumes one step instead of reading
    /// stdin, and the recorded `expect`s are asserted as the battle replays.
    /// See [`player`].
    pub playback: Option<Vec<player::PlayStep>>,
}

pub fn run_with(
    cpu: &mut Cpu,
    config: &BattleConfig,
    options: RunOptions<'_>,
) -> Result<(), String> {
    let seed = match options.seed_override {
        Some(s) => {
            eprintln!("battle: using seed 0x{s:016x} from recording");
            s
        }
        None => resolve_seed(config.battle.seed),
    };
    cpu.seed_rng(seed);

    // Install the recorder *after* the seed is resolved so the resolved
    // value lands in the header even when the scenario omitted `seed = ...`.
    if let Some(path) = options.record_path {
        let embedded = options.embed_scenario.then_some(config);
        recorder::install(cpu, path, seed, options.source_scenario, embedded)?;
        eprintln!("battle: recording to {}", path.display());
    }

    // Install playback before any CLI prompt fires. Recording + playback can
    // both be installed at once — useful for re-recording an edited replay.
    if let Some(steps) = options.playback {
        eprintln!("battle: replaying {} step(s)", steps.len());
        player::install(cpu, steps);
    }

    cpu.skip_intro_to_battle = true;
    let entry = crate::entry_routine();
    (entry.func)(cpu);

    match config.battle.kind {
        BattleKind::WildEncounter => launch_wild_encounter(cpu, config)?,
        BattleKind::TrainerBattle => launch_trainer_battle(cpu, config)?,
        BattleKind::LinkBattle => launch_link_battle(cpu, config)?,
    }

    // The battle has ended. Capture the final state + outcome so the recorder
    // closes out the last step's `expect` block and playback asserts the
    // result. `wBattleMon` / `wEnemyMon` stay valid after `start_battle`
    // returns, and `wBattleResult` now holds the WIN/LOSE/DRAW flag.
    if cpu.recorder.is_some() || cpu.player.is_some() {
        let final_snapshot = snapshot::BattleSnapshot::read(cpu);
        let outcome = snapshot::read_outcome(cpu);
        recorder::finalize(cpu, &final_snapshot, outcome);
        player::finalize(cpu, &final_snapshot, outcome);
    }
    Ok(())
}

/// Pick a seed for the battle RNG: honour the TOML value if present, otherwise
/// draw one from system entropy and log it so the run is still reproducible.
fn resolve_seed(from_config: Option<u64>) -> u64 {
    if let Some(seed) = from_config {
        eprintln!("battle: using seed 0x{seed:016x} from scenario");
        return seed;
    }
    let seed = system_entropy_seed();
    eprintln!("battle: no seed in scenario, generated 0x{seed:016x}");
    seed
}

/// Mix `SystemTime` nanos with the process id to get a non-repeating seed
/// without taking on a `rand` dependency just for this.
fn system_entropy_seed() -> u64 {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(0);
    let pid = std::process::id() as u64;
    let mut seed = nanos ^ pid.wrapping_mul(0x9e37_79b9_7f4a_7c15);
    if seed == 0 {
        seed = 0x9e37_79b9_7f4a_7c15;
    }
    seed
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mon(moves: Option<&[&str]>) -> MonSpec {
        MonSpec {
            species: "Pikachu".into(),
            level: 10,
            moves: moves.map(|m| m.iter().map(|s| s.to_string()).collect()),
        }
    }

    #[test]
    fn no_moves_key_keeps_generated_moveset() {
        assert_eq!(resolve_moves(&mon(None)).unwrap(), None);
    }

    #[test]
    fn moves_resolve_to_ids_in_order() {
        // Thunderbolt=85, Surf=57, Quick Attack=98.
        let ids = resolve_moves(&mon(Some(&["Thunderbolt", "surf", "QUICK_ATTACK"])))
            .unwrap()
            .unwrap();
        assert_eq!(ids, vec![85, 57, 98]);
    }

    #[test]
    fn empty_list_is_rejected() {
        assert!(resolve_moves(&mon(Some(&[]))).is_err());
    }

    #[test]
    fn more_than_four_moves_is_rejected() {
        let err = resolve_moves(&mon(Some(&["Cut", "Fly", "Dig", "Bite", "Surf"]))).unwrap_err();
        assert!(err.contains("1..=4"), "{err}");
    }

    #[test]
    fn unknown_move_name_is_rejected() {
        let err = resolve_moves(&mon(Some(&["Surf", "Notamove"]))).unwrap_err();
        assert!(err.contains("Notamove"), "{err}");
    }
}
