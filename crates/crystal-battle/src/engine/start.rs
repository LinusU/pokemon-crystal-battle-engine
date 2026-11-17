//! Starting a battle without the emulator: from a [`BattleConfig`] and a seed
//! to the engine state at the top of the first `BattleTurn`, exactly as the
//! emulated original reaches it.
//!
//! Two layers, kept apart:
//!
//! * **Harness** (`seed_*`, [`boot_state`], the `launch_*` functions): what
//!   `bootstrap::run_with` and `bootstrap::launch_*` do — the WRAM a played
//!   game would have set up before the battle (player name and ID, badges,
//!   the bag, the party, battle type and trainer class), the few bytes the boot
//!   leaves behind, the RNG seed. The scenario format is the harness's, not the
//!   game's; this is plain Rust mirroring `src/bootstrap/mod.rs` line by line.
//! * **Original** (the `impl Engine` blocks): the game's own routines, one
//!   function per ASM routine — `TryAddMonToParty` / `GeneratePartyMonStats`,
//!   `StartBattle`, `BattleIntro`, `ClearBattleRAM`, `InitEnemyTrainer`,
//!   `ReadTrainerParty`, `InitEnemyWildmon`, `BattleStartMessage`, and the
//!   opening of `DoBattle` (send-outs, `SpikesDamage`, ...), ending where
//!   `DoBattle` jumps to `BattleTurn`.
//!
//! Dropped as graphics, sound or frame timing: the battle transition, pics,
//! palettes, HUD drawing, music and cries, `DelayFrames`. Their stray writes
//! to non-graphics WRAM are kept when later code could read them; the rest
//! (the boot's stack residue, OAM, `wPredef*` bookkeeping, window/menu data,
//! HUD scratch such as `wTempMon*`) is not modelled — see docs/engine.md.

use super::battle::{Engine, Flags, PARTY_LEN};
use super::mem::Mem;
use super::rsym as r;
use super::switch::FlagAction;
use super::sym::*;
use crate::config::{BattleConfig, BattleKind, ItemSpec, MonSpec};
use crate::data::{items, species, trainers};
use crate::defs::*;

/// A wild Unown with no unlocked letters: the original game's `LoadEnemyMon`
/// re-rolls its DVs forever (no letter a roll gives is unlocked), so the
/// battle never starts. The engine refuses it instead of hanging.
pub const UNOWN_FREEZE: &str =
    "a wild Unown needs an unlocked Unown letter (the original game re-rolls its DVs forever)";

/// Why a scenario can't start on the hand-written engine.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StartError {
    /// The scenario is malformed, or one the original game never starts (a
    /// wild Unown with no unlocked letters, [`UNOWN_FREEZE`]).
    Invalid(String),
    /// The scenario uses something only the emulated harness implements
    /// (e.g. a ROM-patching AI override).
    Unsupported(&'static str),
}

impl std::fmt::Display for StartError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StartError::Invalid(m) => f.write_str(m),
            StartError::Unsupported(m) => write!(f, "not supported by the engine: {m}"),
        }
    }
}

impl From<String> for StartError {
    fn from(m: String) -> Self {
        StartError::Invalid(m)
    }
}

/// What the harness decided about the trainer party and items before the
/// battle starts (the oracle keeps these on its `Cpu` / as ROM patches).
#[derive(Default)]
pub(super) struct Launch {
    /// `cpu.trainer_party_override`: `ReadTrainerParty` keeps the seeded party.
    pub(super) trainer_party_override: bool,
    /// `poke_trainer_attributes`: the `[[enemy_items]]` slots.
    pub(super) enemy_item_override: Option<[u8; 2]>,
}

/// `InitializeNPCNames`: Mom's, Red's and Green's names as the boot leaves
/// them in WRAM.
const NPC_NAMES: [(u16, [u8; 11]); 3] = [
    (W_MOMS_NAME, [0x8c, 0x8e, 0x8c, 0x50, 0xcd, 0x0f, 0x61, 0x3e, 0x02, 0x21, 0x29]),
    (W_REDS_NAME, [0x91, 0x84, 0x83, 0x50, 0x86, 0x91, 0x84, 0x84, 0x8d, 0x50, 0x8c]),
    (W_GREENS_NAME, [0x86, 0x91, 0x84, 0x84, 0x8d, 0x50, 0x8c, 0x8e, 0x8c, 0x50, 0xcd]),
];

/// `PARTYMON_STRUCT_LENGTH`.
const STRUCT: u16 = PARTYMON_STRUCT_LENGTH as u16;

/// Build the engine at the top of the first `BattleTurn` for `config` with
/// RNG seed `seed`. With `events`, the engine keeps what the battle shows
/// ([`Engine::events`]).
pub fn start(config: &BattleConfig, seed: u64, events: bool) -> Result<Engine, StartError> {
    let link = config.battle.kind == BattleKind::LinkBattle;
    // Cpu::seed_rng: a zero seed is mapped to the default constant.
    let rng = if seed == 0 { 0x9e37_79b9_7f4a_7c15 } else { seed };
    let mut engine = Engine::new(boot_state(config), rng, Flags {
        link_battle: link,
        external_enemy: config.battle.kind == BattleKind::TrainerBattle
            && config.battle.enemy_control == crate::config::EnemyControl::External,
        ..Flags::default()
    });
    if events {
        engine.events = Some(Vec::new());
    }
    let launch = match config.battle.kind {
        BattleKind::WildEncounter => engine.launch_wild_encounter(config)?,
        BattleKind::TrainerBattle => engine.launch_trainer_battle(config)?,
        BattleKind::LinkBattle => engine.launch_link_battle(config)?,
    };
    engine.start_battle(&launch)?;
    // `[enemy_trainer]` patches the trainer's AI flags in the ROM; nothing
    // before the first turn reads them.
    if let (Some(ov), BattleKind::TrainerBattle) = (&config.enemy_trainer, config.battle.kind) {
        let class = engine.mem.rb(W_TRAINER_CLASS);
        let mut patched = *engine.data.trainer_class(class);
        crate::config::apply_ai_flag_override(&mut patched, ov)?;
        engine.trainer_class_patch = Some((class, patched));
    }
    Ok(engine)
}

// ---------------------------------------------------------------------------
// Harness: the state the emulated boot leaves, and the scenario seeding
// ---------------------------------------------------------------------------

/// WRAM/HRAM after `entry_routine` with `skip_intro_to_battle` (the boot's
/// hardware init, no title screen), minus graphics, audio and stack residue:
/// the options the init sets, and `hSerialConnectionStatus` (0xff: no link).
/// Then `run_with` sets the Battle Scene bit if the scenario turns it off.
pub fn boot_state(config: &BattleConfig) -> Mem {
    boot_mem(config.battle.battle_scene != Some(false))
}

/// [`boot_state`] with the Battle Scene option `battle_scene`.
pub fn boot_mem(battle_scene: bool) -> Mem {
    let mut mem = Mem::default();
    mem.wb(W_OPTIONS, 0x03);
    mem.wb(W_TEXTBOX_FLAGS, 0x01);
    mem.wb(W_GB_PRINTER_BRIGHTNESS, 0x40);
    mem.wb(W_OPTIONS2, 0x01);
    mem.wb(H_SERIAL_CONNECTION_STATUS, 0xff);
    if !battle_scene {
        let options = mem.rb(W_OPTIONS);
        mem.wb(W_OPTIONS, options | 1 << BATTLE_SCENE);
    }
    mem
}

use super::names::Name;

impl Engine {
    /// `seed_player`: name, ID, gender, time of day, empty key/PC item lists.
    pub(super) fn seed_player(&mut self) {
        self.names.player = "RED".into();
        self.names.rival = "SILVER".into();
        // `InitializeNPCNames` copies 11 bytes of ROM to each (the name, its
        // `@`, and what follows). They are never printed in battle, but
        // `DoBattle`'s search for the enemy's first mon with HP runs past an
        // empty party into Mom's name.
        for (addr, bytes) in NPC_NAMES {
            for (i, b) in (0..).zip(bytes) {
                self.mem.wb(addr + i, b);
            }
        }
        self.mem.wb(W_PLAYER_ID, 0x12);
        self.mem.wb(W_PLAYER_ID + 1, 0x34);
        self.mem.wb(W_PLAYER_GENDER, 0);
        self.mem.wb(W_TIME_OF_DAY, DAY_F);
        self.mem.wb(W_TIME_OF_DAY_PAL, 0);
        self.mem.wb(W_LINK_MODE, 0);
        self.mem.wb(W_IN_BATTLE_TOWER_BATTLE, 0);
        self.mem.wb(W_OT_PARTY_COUNT, 0);
        for list in [W_NUM_KEY_ITEMS, W_NUM_PC_ITEMS] {
            self.mem.wb(list, 0);
            self.mem.wb(list + 1, 0xff);
        }
    }

    /// `seed_badges`.
    fn seed_badges(&mut self, config: &BattleConfig) {
        self.mem.wb(W_JOHTO_BADGES, config.battle.johto_badges.unwrap_or(0));
        self.mem.wb(W_KANTO_BADGES, config.battle.kanto_badges.unwrap_or(0));
    }

    /// `seed_player_items`: the Items and Balls pockets.
    fn seed_player_items(&mut self, specs: &[ItemSpec]) -> Result<(), StartError> {
        let (mut item_pocket, mut ball_pocket) = (Vec::new(), Vec::new());
        for spec in specs {
            let def = items::lookup(&spec.name).ok_or_else(|| format!("unknown item: {}", spec.name))?;
            if spec.quantity == 0 || spec.quantity > MAX_ITEM_STACK {
                return Err(format!("item {:?} quantity must be 1..={}, got {}", spec.name, MAX_ITEM_STACK, spec.quantity).into());
            }
            match def.pocket {
                items::Pocket::Item => item_pocket.push((def.id, spec.quantity)),
                items::Pocket::Ball => ball_pocket.push((def.id, spec.quantity)),
            }
        }
        if item_pocket.len() > MAX_ITEMS as usize {
            return Err(format!("too many items in Items pocket: {}", item_pocket.len()).into());
        }
        if ball_pocket.len() > MAX_BALLS as usize {
            return Err(format!("too many balls in Balls pocket: {}", ball_pocket.len()).into());
        }
        self.write_pocket(W_NUM_ITEMS, &item_pocket);
        self.write_pocket(W_NUM_BALLS, &ball_pocket);
        Ok(())
    }

    pub(super) fn write_pocket(&mut self, num_addr: u16, entries: &[(u8, u8)]) {
        self.mem.wb(num_addr, entries.len() as u8);
        let mut addr = num_addr + 1;
        for &(id, qty) in entries {
            self.mem.wb(addr, id);
            self.mem.wb(addr + 1, qty);
            addr += 2;
        }
        self.mem.wb(addr, 0xff);
    }

    /// `add_mon`: append one mon through the game's own `TryAddMonToParty`
    /// (the harness calls it with `wBattleMode = 0`: not in battle yet).
    fn add_mon(&mut self, species: u8, level: u8, mon_type: u8) {
        self.mem.wb(W_CUR_PARTY_SPECIES, species);
        self.mem.wb(W_CUR_PARTY_LEVEL, level);
        self.mem.wb(W_CUR_SPECIES, species);
        self.mem.wb(W_MON_TYPE, mon_type);
        self.mem.wb(W_BATTLE_MODE, 0);
        self.mem.wb(W_ENEMY_MON_ITEM, 0);
        self.try_add_mon_to_party();
    }

    /// `seed_party`: a whole party into the player's or the OT list, with the
    /// scenario's moveset / held item / OT ID overrides.
    fn seed_party(&mut self, party: &[MonSpec], mon_type: u8, count_addr: u16, mon1_addr: u16, label: &str) -> Result<(), StartError> {
        self.mem.wb(count_addr, 0);
        for (slot, mon) in party.iter().enumerate() {
            let id = species::id_from_name(&mon.species).ok_or_else(|| format!("unknown {label} species: {}", mon.species))?;
            let move_ids = crate::config::resolve_moves(mon)?;
            let item_id = crate::config::resolve_item(mon, label)?;
            self.add_mon(id, mon.level, mon_type);
            let base = mon1_addr + slot as u16 * STRUCT;
            if let Some(ids) = move_ids {
                // `apply_moveset`: full base PP for each given move, the rest zeroed.
                for i in 0..4usize {
                    let mv = ids.get(i).copied().unwrap_or(0);
                    self.mem.wb(base + u16::from(MON_MOVES) + i as u16, mv);
                    self.mem.wb(base + u16::from(MON_PP) + i as u16, crate::data::move_pp::MOVE_BASE_PP[usize::from(mv)]);
                }
            }
            if let Some(item) = item_id {
                self.mem.wb(base + u16::from(MON_ITEM), item);
            }
            if let (Some(ot_id), PARTYMON) = (mon.ot_id, mon_type) {
                self.mem.wb(base + u16::from(MON_OT_ID), (ot_id >> 8) as u8);
                self.mem.wb(base + u16::from(MON_OT_ID) + 1, ot_id as u8);
            }
            // `TryAddMonToParty` skips the nickname of OT mons (a link partner
            // would supply it): the harness seeds the species name.
            if mon_type == OTPARTYMON {
                self.mem.wb(W_NAMED_OBJECT_INDEX, id);
                self.get_pokemon_name();
                let name = self.data.names.species[usize::from(id)];
                self.names.ot_party.resize(slot, Default::default());
                self.names.ot_party.push(name.into());
            }
        }
        Ok(())
    }

    /// `launch_wild_encounter`.
    pub(super) fn launch_wild_encounter(&mut self, config: &BattleConfig) -> Result<Launch, StartError> {
        if config.player_party.is_empty() {
            return Err("wild_encounter requires at least one player_party entry".to_string().into());
        }
        if config.enemy_party.len() != 1 {
            return Err(format!("wild_encounter expects exactly one enemy_party entry, got {}", config.enemy_party.len()).into());
        }
        if !config.enemy_items.is_empty() {
            return Err("wild_encounter does not accept enemy_items".to_string().into());
        }
        self.seed_player();
        self.seed_badges(config);
        self.seed_player_items(&config.player_items)?;
        self.seed_party(&config.player_party, PARTYMON, W_PARTY_COUNT, W_PARTY_MON1, "player")?;
        let enemy = &config.enemy_party[0];
        let id = species::id_from_name(&enemy.species).ok_or_else(|| format!("unknown enemy species: {}", enemy.species))?;
        self.launch_wild(id, enemy.level)
    }

    /// A wild battle against `id` at `level` (the player already seeded).
    pub(super) fn launch_wild(&mut self, id: u8, level: u8) -> Result<Launch, StartError> {
        self.mem.wb(W_BATTLE_MODE, WILD_BATTLE);
        self.mem.wb(W_BATTLE_TYPE, BATTLETYPE_NORMAL);
        self.mem.wb(W_OTHER_TRAINER_CLASS, 0);
        self.mem.wb(W_TEMP_WILD_MON_SPECIES, id);
        self.mem.wb(W_TEMP_ENEMY_MON_SPECIES, id);
        self.mem.wb(W_CUR_PARTY_LEVEL, level);
        self.mem.wb(W_CUR_PARTY_SPECIES, id);
        if id == UNOWN && !self.any_unown_letter_unlocked() {
            return Err(UNOWN_FREEZE.to_string().into());
        }
        Ok(Launch::default())
    }

    /// `launch_trainer_battle`.
    pub(super) fn launch_trainer_battle(&mut self, config: &BattleConfig) -> Result<Launch, StartError> {
        if config.player_party.is_empty() {
            return Err("trainer_battle requires at least one player_party entry".to_string().into());
        }
        let name = config.battle.trainer.as_deref().ok_or_else(|| "trainer_battle requires a [battle] trainer = \"...\" field".to_string())?;
        let trainer = trainers::lookup(name).ok_or_else(|| format!("unknown trainer: {name}"))?;
        let mut launch = Launch::default();
        self.seed_player();
        self.seed_badges(config);
        self.seed_player_items(&config.player_items)?;
        self.seed_party(&config.player_party, PARTYMON, W_PARTY_COUNT, W_PARTY_MON1, "player")?;
        if !config.enemy_party.is_empty() {
            if config.enemy_party.len() > PARTY_LENGTH as usize {
                return Err(format!("trainer_battle enemy_party: at most {PARTY_LENGTH} entries, got {}", config.enemy_party.len()).into());
            }
            self.seed_party(&config.enemy_party, OTPARTYMON, W_OT_PARTY_COUNT, W_OT_PARTY_MON1, "enemy")?;
            self.mem.wb(W_CUR_OT_MON, 0);
            self.mem.wb(W_CUR_PARTY_MON, 0);
            // `cpu.trainer_party_override`: the HARNESS-PATCH in ReadTrainerParty.
            launch.trainer_party_override = true;
        }
        launch.enemy_item_override = enemy_items_override(&config.enemy_items)?;
        self.mem.wb(W_BATTLE_MODE, TRAINER_BATTLE);
        self.mem.wb(W_BATTLE_TYPE, BATTLETYPE_NORMAL);
        self.mem.wb(W_OTHER_TRAINER_CLASS, trainer.class);
        self.mem.wb(W_OTHER_TRAINER_ID, trainer.id);
        Ok(launch)
    }

    /// `launch_link_battle`.
    pub(super) fn launch_link_battle(&mut self, config: &BattleConfig) -> Result<Launch, StartError> {
        let max = PARTY_LENGTH as usize;
        if config.player_party.is_empty() || config.player_party.len() > max {
            return Err(format!("link_battle requires 1..={max} player_party entries, got {}", config.player_party.len()).into());
        }
        if config.enemy_party.is_empty() || config.enemy_party.len() > max {
            return Err(format!("link_battle requires 1..={max} enemy_party entries, got {}", config.enemy_party.len()).into());
        }
        if !config.player_items.is_empty() || !config.enemy_items.is_empty() {
            return Err("link_battle does not accept player_items / enemy_items (the bag is disabled in a link battle)".to_string().into());
        }
        self.seed_player();
        self.seed_badges(config);
        self.seed_player_items(&[])?;
        self.seed_party(&config.player_party, PARTYMON, W_PARTY_COUNT, W_PARTY_MON1, "player")?;
        self.seed_party(&config.enemy_party, OTPARTYMON, W_OT_PARTY_COUNT, W_OT_PARTY_MON1, "enemy")?;
        // (The harness names the other player "RIVAL".)
        self.launch_link_with("RIVAL")
    }

    /// A link battle against `name` (both parties already seeded).
    pub(super) fn launch_link_with(&mut self, name: &str) -> Result<Launch, StartError> {
        self.mem.wb(W_LINK_MODE, LINK_COLOSSEUM);
        self.mem.wb(H_SERIAL_CONNECTION_STATUS, USING_EXTERNAL_CLOCK);
        self.mem.wb(W_CUR_OT_MON, 0);
        self.mem.wb(W_CUR_PARTY_MON, 0);
        self.names.ot_player = name.into();
        self.names.ot_class = Name::Nickname(name.into());
        self.mem.wb(W_BATTLE_MODE, TRAINER_BATTLE);
        self.mem.wb(W_BATTLE_TYPE, BATTLETYPE_NORMAL);
        self.mem.wb(W_OTHER_TRAINER_CLASS, CAL);
        Ok(Launch::default())
    }
}

/// `poke_trainer_attributes`, the `[[enemy_items]]` half: the two item slots
/// the scenario wants in the trainer class's attributes (`None`: ROM's).
pub(super) fn enemy_items_override(specs: &[ItemSpec]) -> Result<Option<[u8; 2]>, StartError> {
    if specs.is_empty() {
        return Ok(None);
    }
    let mut slots = [NO_ITEM; 2];
    let mut n = 0;
    for spec in specs {
        let def = items::lookup(&spec.name).ok_or_else(|| format!("unknown enemy_items item: {}", spec.name))?;
        if spec.quantity == 0 {
            return Err(format!("enemy_items {}: quantity must be >= 1", spec.name).into());
        }
        for _ in 0..spec.quantity {
            if n >= 2 {
                return Err("enemy_items total quantity exceeds 2 (trainer has at most 2 item slots)".to_string().into());
            }
            slots[n] = def.id;
            n += 1;
        }
    }
    Ok(Some(slots))
}

// ---------------------------------------------------------------------------
// Original: TryAddMonToParty
// ---------------------------------------------------------------------------

impl Engine {
    /// `TryAddMonToParty` (engine/pokemon/move_mon.asm): append
    /// `wCurPartySpecies` at `wCurPartyLevel` to the player's party
    /// (`wMonType` = PARTYMON) or the OT party. Returns carry (success).
    pub fn try_add_mon_to_party(&mut self) -> bool {
        let opposing = self.mem.rb(W_MON_TYPE) & 0xf != 0;
        let count_addr = if opposing { W_OT_PARTY_COUNT } else { W_PARTY_COUNT };
        let count = self.mem.rb(count_addr).wrapping_add(1);
        if count > PARTY_LENGTH {
            return false;
        }
        self.mem.wb(count_addr, count);
        self.mem.wb(H_MOVE_MON, count);
        let species = self.mem.rb(W_CUR_PARTY_SPECIES);
        let de = count_addr + u16::from(count);
        self.mem.wb(de, species);
        self.mem.wb(de + 1, 0xff);
        // The OT name: the player's, for every mon.
        let ots = if opposing { &mut self.names.ot_party_ots } else { &mut self.names.party_ots };
        ots.resize(usize::from(count - 1), Default::default());
        ots.push(self.names.player.clone());
        // Only initialize the nickname for party mon.
        if self.mem.rb(W_MON_TYPE) == 0 {
            self.mem.wb(W_NAMED_OBJECT_INDEX, species);
            self.get_pokemon_name();
            let name = self.data.names.species[usize::from(species)];
            self.names.party.resize(usize::from(count - 1), Default::default());
            self.names.party.push(name.into());
        }
        let first = if opposing { W_OT_PARTY_MON1 } else { W_PARTY_MON1 };
        let mon = first + u16::from(count - 1) * STRUCT;
        self.generate_party_mon_stats(mon);
        true
    }

    /// `GeneratePartyMonStats`: fill the `party_struct` at `mon`.
    /// `wBattleMode` says whether it's a wild mon, `wMonType` whether it's an
    /// opposing mon; `wCurPartySpecies` / `wCurPartyLevel` the species and level.
    pub fn generate_party_mon_stats(&mut self, mon: u16) {
        let species = self.mem.rb(W_CUR_PARTY_SPECIES);
        self.mem.wb(W_CUR_SPECIES, species);
        self.get_base_data();
        let mut de = mon;
        let dex = self.mem.rb(W_BASE_DEX_NO);
        self.mem.wb(de, dex);
        de += 1;
        // Copy the item if it's a wild mon.
        let battle_mode = self.mem.rb(W_BATTLE_MODE);
        let item = if battle_mode == 0 { 0 } else { self.mem.rb(W_ENEMY_MON_ITEM) };
        self.mem.wb(de, item);
        de += 1;
        // Copy the moves if it's a wild mon.
        let moves = de;
        if battle_mode != 0 && self.mem.rb(W_MON_TYPE) == 0 {
            self.mem.copy(moves, W_ENEMY_MON_MOVES, u16::from(NUM_MOVES));
        } else {
            self.mem.fill(moves, u16::from(NUM_MOVES), 0);
            self.mem.wb(W_SKIP_MOVES_BEFORE_LEVEL_UP, 0);
            self.fill_moves(moves);
        }
        de = moves + u16::from(NUM_MOVES);
        // Initialize ID.
        let (id_hi, id_lo) = (self.mem.rb(W_PLAYER_ID), self.mem.rb(W_PLAYER_ID + 1));
        self.mem.wb(de, id_hi);
        self.mem.wb(de + 1, id_lo);
        de += 2;
        // Initialize Exp.
        let level = self.mem.rb(W_CUR_PARTY_LEVEL);
        self.calc_exp_at_level(level);
        for i in 0..3 {
            let b = self.mem.rb(H_PRODUCT + 1 + i);
            self.mem.wb(de + i, b);
        }
        de += 3;
        // Initialize stat experience.
        let stat_exp = u16::from(MON_DVS - MON_STAT_EXP);
        self.mem.fill(de, stat_exp, 0);
        de += stat_exp;
        let opposing = self.mem.rb(W_MON_TYPE) & 0xf != 0;
        let (b, c);
        let mut copy_wild_dvs = false;
        if opposing {
            (b, c) = self.get_trainer_dvs();
        } else {
            // .registerpokedex
            self.mem.wb(W_TEMP_SPECIES, species);
            // CheckCaughtMon only reads.
            self.small_far_flag_action(W_POKEDEX_CAUGHT, species.wrapping_sub(1), FlagAction::Set);
            self.small_far_flag_action(W_POKEDEX_SEEN, species.wrapping_sub(1), FlagAction::Set);
            if battle_mode != 0 {
                copy_wild_dvs = true;
                (b, c) = (0, 0);
            } else {
                b = self.random("GeneratePartyMonStats");
                c = self.random("GeneratePartyMonStats");
            }
        }
        if copy_wild_dvs {
            // .copywildmonDVs
            let (hi, lo) = (self.mem.rb(W_ENEMY_MON_D_VS), self.mem.rb(W_ENEMY_MON_D_VS + 1));
            self.mem.wb(de, hi);
            self.mem.wb(de + 1, lo);
            de += 2;
            self.mem.copy(de, W_ENEMY_MON_PP, u16::from(NUM_MOVES));
            de += u16::from(NUM_MOVES);
            self.mem.wb(de, BASE_HAPPINESS);
            self.mem.fill(de + 1, 3, 0);
            de += 4;
            self.mem.wb(de, level);
            de += 1;
            // wEnemyMonStatus (2 bytes), wEnemyMonHP (2 bytes)
            self.mem.copy(de, W_ENEMY_MON_STATUS, 2);
            de += 2;
            self.mem.copy(de, W_ENEMY_MON_HP, 2);
            de += 2;
        } else {
            self.mem.wb(de, b);
            self.mem.wb(de + 1, c);
            de += 2;
            // Initialize PP.
            self.fill_pp(mon + 2, de);
            de += u16::from(NUM_MOVES);
            // Initialize happiness, Pokerus, caught data, level, status.
            self.mem.wb(de, BASE_HAPPINESS);
            self.mem.fill(de + 1, 3, 0);
            de += 4;
            self.mem.wb(de, level);
            de += 1;
            self.mem.fill(de, 2, 0);
            de += 2;
            // Initialize HP.
            let hl = mon + u16::from(MON_STAT_EXP) - 1;
            self.calc_mon_stat_c(hl, 1, false);
            let (hi, lo) = (self.mem.rb(H_PRODUCT + 2), self.mem.rb(H_PRODUCT + 3));
            self.mem.wb(de, hi);
            self.mem.wb(de + 1, lo);
            de += 2;
        }
        // .initstats
        if battle_mode == 1 {
            self.mem.copy(de, W_ENEMY_MON_MAX_HP, STRUCT - u16::from(MON_MAXHP));
        } else {
            let hl = mon + u16::from(MON_STAT_EXP) - 1;
            self.calc_mon_stats(hl, de, false);
        }
        // .registerunowndex
        if !opposing && species == UNOWN {
            let slot = u16::from(self.mem.rb(W_PARTY_COUNT).wrapping_sub(1));
            self.get_unown_letter(W_PARTY_MON1_D_VS + slot * STRUCT);
            self.update_unown_dex();
        }
    }

    /// `UpdateUnownDex`: remember `wUnownLetter` in the Unown dex.
    fn update_unown_dex(&mut self) {
        let letter = self.mem.rb(W_UNOWN_LETTER);
        let mut hl = W_UNOWN_DEX;
        for _ in 0..NUM_UNOWN {
            let a = self.mem.rb(hl);
            if a == 0 {
                self.mem.wb(hl, letter);
                return;
            }
            if a == letter {
                return;
            }
            hl += 1;
        }
    }
}

// ---------------------------------------------------------------------------
// Original: StartBattle .. the first BattleTurn
// ---------------------------------------------------------------------------

impl Engine {
    /// `StartBattle` up to `DoBattle`'s `jp BattleTurn` (`ExitBattle` is the
    /// battle's end, not its start).
    pub(super) fn start_battle(&mut self, launch: &Launch) -> Result<(), StartError> {
        // This check prevents you from entering a battle without any Pokemon.
        if self.mem.rb(W_PARTY_COUNT) == 0 {
            return Err("battle needs at least one party mon".to_string().into());
        }
        self.battle_intro(launch)?;
        self.do_battle()
    }

    /// `BattleIntro`.
    fn battle_intro(&mut self, launch: &Launch) -> Result<(), StartError> {
        // LoadTrainerOrWildMonPic
        let mut a = self.mem.rb(W_OTHER_TRAINER_CLASS);
        if a == 0 {
            a = self.mem.rb(W_TEMP_WILD_MON_SPECIES);
            self.mem.wb(W_CUR_PARTY_SPECIES, a);
        }
        self.mem.wb(W_TEMP_ENEMY_MON_SPECIES, a);
        self.mem.wb(W_TEMP_BATTLE_MON_SPECIES, 0);
        self.mem.wb(W_BATTLE_MENU_CURSOR_POSITION, 0);
        self.mem.wb(H_MAP_ANIMS, 0);
        // PlayBattleMusic, ShowLinkBattleParticipants (patched out for link),
        // FindFirstAliveMonAndStartBattle (the transition): graphics and sound,
        // except that the transition respawns the overworld objects.
        self.hide_all_objects();
        self.clear_battle_ram();
        self.init_enemy(launch)?;
        // InitBattleDisplay: graphics.
        self.battle_start_message();
        // EmptyBattleTextbox, ClearBox, ClearSprites: graphics.
        if self.battle_mode() == WILD_BATTLE {
            self.update_enemy_hud();
        }
        Ok(())
    }

    /// `HideAllObjects` (from the battle transition's `RespawnPlayerAndOpponent`):
    /// every object struct faces `STANDING`. Not graphics: a wild battle's
    /// `DoBattle` walks `wOTPartyMon1HP` + 48n looking for a living mon and
    /// stops at the first of these bytes (it has no OT party), which fixes
    /// `wBattleAction`.
    fn hide_all_objects(&mut self) {
        for i in 0..u16::from(NUM_OBJECT_STRUCTS) {
            self.mem.wb(W_PLAYER_FACING + i * u16::from(OBJECT_LENGTH), STANDING as u8);
        }
    }

    /// `ClearBattleRAM`.
    fn clear_battle_ram(&mut self) {
        for addr in [W_BATTLE_PLAYER_ACTION, W_BATTLE_RESULT] {
            self.mem.wb(addr, 0);
        }
        self.mem.fill(W_PARTY_MENU_CURSOR, 4, 0);
        for addr in [
            W_MENU_SCROLL_POSITION, W_CRITICAL_HIT, W_BATTLE_MON_SPECIES, W_BATTLE_PARTICIPANTS_NOT_FAINTED,
            W_CUR_BATTLE_MON, W_FORCED_SWITCH, W_TIME_OF_DAY_PAL, W_PLAYER_TURNS_TAKEN, W_ENEMY_TURNS_TAKEN,
            W_EVOLVABLE_FLAGS,
        ] {
            self.mem.wb(addr, 0);
        }
        self.mem.fill(W_PLAYER_HP_PAL, 2, 0);
        self.mem.fill(W_BATTLE_MON_D_VS, 2, 0);
        self.mem.fill(W_ENEMY_MON_D_VS, 2, 0);
        // Clear the entire BattleMons area
        self.mem.fill(W_BATTLE, W_BATTLE_END - W_BATTLE, 0);
        self.reset_enemy_stat_levels();
        // ClearWindowData: menu data, not battle state.
    }

    /// `InitEnemy`.
    fn init_enemy(&mut self, launch: &Launch) -> Result<(), StartError> {
        let class = self.mem.rb(W_OTHER_TRAINER_CLASS);
        if class != 0 {
            self.init_enemy_trainer(launch, class)
        } else {
            self.init_enemy_wildmon();
            Ok(())
        }
    }

    /// `InitEnemyTrainer`.
    fn init_enemy_trainer(&mut self, launch: &Launch, class: u8) -> Result<(), StartError> {
        self.mem.wb(W_TRAINER_CLASS, class);
        self.mem.wb(W_TEMP_ENEMY_MON_SPECIES, 0);
        self.get_trainer_attributes(launch);
        self.read_trainer_party(launch)?;
        // RIVAL1's first mon has no held item
        if self.mem.rb(W_TRAINER_CLASS) == RIVAL1 {
            self.mem.wb(W_OT_PARTY_MON1_ITEM, 0);
        }
        // GetTrainerPic, PlaceGraphic: graphics.
        self.mem.wb(H_GRAPHIC_START_TILE, 0);
        self.mem.wb(W_ENEMY_ITEM_STATE, 0xff);
        self.mem.wb(W_CUR_OT_MON, 0xff);
        self.mem.wb(W_BATTLE_MODE, TRAINER_BATTLE);
        if !self.is_gym_leader() {
            return Ok(());
        }
        self.mem.wb(W_CUR_PARTY_MON, 0);
        let mut b = self.mem.rb(W_PARTY_COUNT);
        loop {
            let hp = self.get_party_param_location(MON_HP);
            if self.mem.rw(hp) != 0 {
                self.change_happiness(HAPPINESS_GYMBATTLE);
            }
            b = b.wrapping_sub(1);
            if b == 0 {
                return Ok(());
            }
            let cur = self.mem.rb(W_CUR_PARTY_MON);
            self.mem.wb(W_CUR_PARTY_MON, cur.wrapping_add(1));
        }
    }

    /// `IsGymLeader`: `wOtherTrainerClass` is in `GymLeaders` (which falls into
    /// `KantoGymLeaders`).
    fn is_gym_leader(&self) -> bool {
        let class = self.mem.rb(W_OTHER_TRAINER_CLASS);
        self.data.tables.gym_leaders.contains(&class)
    }

    /// `GetTrainerAttributes` (with `GetOTName`): the class name into
    /// `wOTClassName`, the class's two items and base reward.
    fn get_trainer_attributes(&mut self, launch: &Launch) {
        let class = self.mem.rb(W_TRAINER_CLASS);
        // GetOTName
        let src = if self.link_mode() != 0 {
            W_OT_PLAYER_NAME
        } else if class == RIVAL1 {
            W_RIVAL_NAME
        } else {
            self.mem.wb(W_CUR_SPECIES, class);
            self.mem.wb(W_NAMED_OBJECT_TYPE, TRAINER_NAME);
            self.get_name();
            W_STRING_BUFFER1
        };
        self.copy_name(W_OT_CLASS_NAME, src);
        let attributes = self.trainer_class(class);
        let [item1, item2] = launch.enemy_item_override.unwrap_or(attributes.items);
        self.mem.wb(W_ENEMY_TRAINER_ITEM1, item1);
        self.mem.wb(W_ENEMY_TRAINER_ITEM2, item2);
        let reward = attributes.base_money;
        self.mem.wb(W_ENEMY_TRAINER_BASE_REWARD, reward);
    }

    /// `ReadTrainerParty`: the trainer's party from ROM into `wOTParty*`.
    fn read_trainer_party(&mut self, launch: &Launch) -> Result<(), StartError> {
        if self.mem.bit(W_IN_BATTLE_TOWER_BATTLE, IN_BATTLE_TOWER_BATTLE_F) {
            return Ok(());
        }
        if self.link_mode() != 0 {
            return Ok(());
        }
        // HARNESS-PATCH (`trainer_party_override`): the scenario's own party.
        if launch.trainer_party_override {
            return Ok(());
        }
        self.mem.wb(W_OT_PARTY_COUNT, 0);
        self.mem.wb(W_OT_PARTY_COUNT + 1, 0xff);
        self.mem.fill(W_OT_PARTY_MONS, STRUCT * u16::from(PARTY_LENGTH), 0);
        let mut class = self.mem.rb(W_OTHER_TRAINER_CLASS);
        if class == CAL && self.mem.rb(W_OTHER_TRAINER_ID) == CAL2 {
            return Err(StartError::Unsupported("CAL2 reads the Mystery Gift trainer from SRAM"));
        }
        if class != CAL {
            class = self.mem.rb(W_OTHER_TRAINER_CLASS);
        }
        let id = self.mem.rb(W_OTHER_TRAINER_ID);
        let party = self.data.trainer_party(class, id).ok_or(StartError::Unsupported("a trainer id past the game's parties"))?;
        for mon in party.mons {
            self.trainer_mon(mon);
        }
        self.compute_trainer_reward();
        Ok(())
    }

    /// `TrainerType1`..`TrainerType4`: one mon of the party (level,
    /// species, and the item and moves the party's type has).
    fn trainer_mon(&mut self, data: &crate::data::game::TrainerMon) {
        self.mem.wb(W_CUR_PARTY_LEVEL, data.level);
        self.mem.wb(W_CUR_PARTY_SPECIES, data.species);
        self.mem.wb(W_MON_TYPE, OTPARTYMON);
        self.try_add_mon_to_party();
        let mon = W_OT_PARTY_MON1 + u16::from(self.mem.rb(W_OT_PARTY_COUNT).wrapping_sub(1)) * STRUCT;
        if let Some(item) = data.item {
            self.mem.wb(mon + u16::from(MON_ITEM), item);
        }
        if let Some(moves) = data.moves {
            for (i, mv) in (0..).zip(moves) {
                self.mem.wb(mon + u16::from(MON_MOVES) + i, mv);
            }
            // .copy_pp: stops at the first empty slot.
            let mut de = mon + u16::from(MON_PP);
            for mv in moves {
                if mv == 0 {
                    break;
                }
                let pp = self.data.mv(mv).pp;
                self.mem.wb(de, pp);
                de += 1;
            }
        }
    }

    /// `ComputeTrainerReward`: base reward × the last mon's level.
    fn compute_trainer_reward(&mut self) {
        for i in 0..3 {
            self.mem.wb(H_PRODUCT + i, 0);
        }
        let reward = self.mem.rb(W_ENEMY_TRAINER_BASE_REWARD);
        self.mem.wb(H_PRODUCT + 3, reward);
        let level = self.mem.rb(W_CUR_PARTY_LEVEL);
        self.mem.wb(H_PRODUCT + 4, level);
        self.multiply();
        self.mem.wb(W_BATTLE_REWARD, 0);
        let (a, b) = (self.mem.rb(H_PRODUCT + 2), self.mem.rb(H_PRODUCT + 3));
        self.mem.wb(W_BATTLE_REWARD + 1, a);
        self.mem.wb(W_BATTLE_REWARD + 2, b);
    }

    /// `InitEnemyWildmon`.
    fn init_enemy_wildmon(&mut self) {
        self.mem.wb(W_BATTLE_MODE, WILD_BATTLE);
        self.load_enemy_mon();
        self.mem.copy(W_WILD_MON_MOVES, W_ENEMY_MON_MOVES, u16::from(NUM_MOVES));
        self.mem.copy(W_WILD_MON_PP, W_ENEMY_MON_PP, u16::from(NUM_MOVES));
        self.get_unown_letter(W_ENEMY_MON_D_VS);
        if self.mem.rb(W_CUR_PARTY_SPECIES) == UNOWN && self.mem.rb(W_FIRST_UNOWN_SEEN) == 0 {
            let letter = self.mem.rb(W_UNOWN_LETTER);
            self.mem.wb(W_FIRST_UNOWN_SEEN, letter);
        }
        // GetAnimatedFrontpic, PlaceGraphic: graphics.
        self.mem.wb(W_TRAINER_CLASS, 0);
        self.mem.wb(H_GRAPHIC_START_TILE, 0);
    }

    /// `BattleStartMessage`.
    fn battle_start_message(&mut self) {
        let text = if self.battle_mode() == WILD_BATTLE {
            if self.battle_check_enemy_shininess() {
                self.mem.wb(W_BATTLE_AFTER_ANIM, 0);
                self.mem.wb(H_BATTLE_TURN, 1);
                self.mem.wb(W_BATTLE_ANIM_PARAM, 1);
                // Call_PlayBattleAnim(ANIM_SEND_OUT_MON)
                self.mem.wb(W_FX_ANIM_ID, ANIM_SEND_OUT_MON as u8);
                self.mem.wb(W_FX_ANIM_ID + 1, (ANIM_SEND_OUT_MON >> 8) as u8);
            }
            if !self.check_sleeping_tree_mon() && self.mem.bit(W_OPTIONS, BATTLE_SCENE) {
                // .cry_no_anim (the cry itself is sound)
                self.mem.wb(W_CRY_TRACKS, 0xf);
            }
            match self.mem.rb(W_BATTLE_TYPE) {
                BATTLETYPE_FISH => r::HOOKED_POKEMON_ATTACKED_TEXT,
                BATTLETYPE_TREE => r::POKEMON_FELL_FROM_TREE_TEXT,
                BATTLETYPE_CELEBI => r::WILD_CELEBI_APPEARED_TEXT,
                _ => r::WILD_POKEMON_APPEARED_TEXT,
            }
        } else {
            self.battle_get_trainer_name();
            r::WANTS_TO_BATTLE_TEXT
        };
        // BattleStart_TrainerHuds: graphics.
        self.text(text);
    }

    /// `Battle_GetTrainerName` (+ `GetTrainerName`, `CopyTrainerName`): the
    /// trainer's name into `wStringBuffer1`.
    pub fn battle_get_trainer_name(&mut self) {
        self.names.string_buffer1 = Name::Nickname(self.trainer_name().into());
    }

    /// The trainer's name, as `Battle_GetTrainerName` finds it.
    pub fn trainer_name(&self) -> String {
        if self.mem.bit(W_IN_BATTLE_TOWER_BATTLE, IN_BATTLE_TOWER_BATTLE_F) {
            return self.names.ot_player.to_string();
        }
        // (CAL: the Mystery Gift house flag lives in SRAM, which is blank.)
        let (id, class) = (self.mem.rb(W_OTHER_TRAINER_ID), self.mem.rb(W_OTHER_TRAINER_CLASS));
        self.data.trainer_party(class, id).map(|party| party.name.to_string()).unwrap_or_default()
    }

    /// `UpdateEnemyHUD` (wild battles): the memory side of `DrawEnemyHUD`.
    fn update_enemy_hud(&mut self) {
        let species = self.mem.rb(W_TEMP_ENEMY_MON_SPECIES);
        self.mem.wb(W_CUR_SPECIES, species);
        self.mem.wb(W_CUR_PARTY_SPECIES, species);
        self.get_base_data();
    }

    /// `UpdatePlayerHUD` and the `Call_PlayBattleAnim` of the send-out: the
    /// memory side (`wTempMon*` scratch, `wMonType`, `wWhichHPBar`,
    /// `wFXAnimID`), which later code never reads but the original leaves.
    fn update_player_hud(&mut self) {
        self.mem.wb(W_FX_ANIM_ID, ANIM_SEND_OUT_MON as u8);
        self.mem.wb(W_FX_ANIM_ID + 1, (ANIM_SEND_OUT_MON >> 8) as u8);
        // PrintPlayerHUD
        let cur = u16::from(self.mem.rb(W_CUR_BATTLE_MON));
        let dvs = W_PARTY_MON1_D_VS + cur * STRUCT;
        self.mem.copy(W_TEMP_MON_D_VS, dvs, 2);
        let tail = STRUCT - u16::from(MON_LEVEL);
        self.mem.copy(W_TEMP_MON_LEVEL, W_BATTLE_MON_LEVEL, tail);
        let species = self.mem.rb(W_PARTY_MON1_SPECIES + cur * STRUCT);
        self.mem.wb(W_CUR_PARTY_SPECIES, species);
        self.mem.wb(W_CUR_SPECIES, species);
        self.get_base_data();
        self.mem.wb(W_MON_TYPE, TEMPMON);
        let level = self.mem.rb(W_BATTLE_MON_LEVEL);
        self.mem.wb(W_TEMP_MON_LEVEL, level);
        // DrawPlayerHP
        self.mem.wb(W_MON_TYPE, PARTYMON);
        self.mem.wb(W_WHICH_HP_BAR, 1);
    }

    /// `DoBattle`, from its start to `jp BattleTurn`.
    fn do_battle(&mut self) -> Result<(), StartError> {
        self.mem.wb(W_BATTLE_PARTICIPANTS_NOT_FAINTED, 0);
        self.mem.wb(W_BATTLE_PARTICIPANTS_INCLUDING_FAINTED, 0);
        self.mem.wb(W_BATTLE_PLAYER_ACTION, 0);
        self.mem.wb(W_BATTLE_ENDED, 0);
        self.mem.wb(W_BATTLE_HAS_JUST_STARTED, 1);
        // The first OT mon with HP picks wBattleAction (a switch to it).
        let mut d = BATTLEACTION_SWITCH1 - 1;
        let mut hl = W_OT_PARTY_MON1_HP;
        loop {
            d = d.wrapping_add(1);
            if self.mem.rb(hl) | self.mem.rb(hl + 1) != 0 {
                break;
            }
            hl += STRUCT;
        }
        self.mem.wb(W_BATTLE_ACTION, d);
        let player_2 = self.link_mode() != 0 && self.mem.rb(H_SERIAL_CONNECTION_STATUS) == USING_INTERNAL_CLOCK;
        if !player_2 && self.battle_mode() != WILD_BATTLE {
            self.mem.wb(W_ENEMY_SWITCH_MON_INDEX, 0);
            self.new_enemy_mon_status();
            self.reset_enemy_stat_levels();
            self.break_attraction();
            // No decision is asked: wBattleHasJustStarted is set.
            if self.enemy_switch().is_err() {
                return Err(StartError::Unsupported("the first enemy send-out asked for a decision"));
            }
        }
        // LoadTilemapToTempTilemap: graphics.
        if self.check_player_party_for_fit_mon() == 0 {
            return Err(StartError::Unsupported("LostBattle: no party mon can fight"));
        }
        let battle_type = self.mem.rb(W_BATTLE_TYPE);
        if battle_type == BATTLETYPE_DEBUG || battle_type == BATTLETYPE_TUTORIAL {
            return Ok(());
        }
        self.mem.wb(W_CUR_PARTY_MON, 0);
        while !self.check_if_cur_party_mon_is_fit_to_fight() {
            let cur = self.mem.rb(W_CUR_PARTY_MON);
            self.mem.wb(W_CUR_PARTY_MON, cur.wrapping_add(1));
        }
        let last = self.mem.rb(W_CUR_BATTLE_MON);
        self.mem.wb(W_LAST_PLAYER_MON, last);
        let cur = self.mem.rb(W_CUR_PARTY_MON);
        self.mem.wb(W_CUR_BATTLE_MON, cur);
        let species = self.mem.rb(W_PARTY_SPECIES + u16::from(cur));
        self.mem.wb(W_CUR_PARTY_SPECIES, species);
        self.mem.wb(W_TEMP_BATTLE_MON_SPECIES, species);
        // SlideBattlePicOut: graphics.
        self.reset_battle_participants();
        self.init_battle_mon();
        self.reset_player_stat_levels();
        self.send_out_mon_text();
        self.new_battle_mon_status();
        self.break_attraction();
        self.send_out_player_mon();
        self.update_player_hud();
        self.set_player_turn();
        self.spikes_damage();
        // A link battle's second player (hSerialConnectionStatus ==
        // USING_INTERNAL_CLOCK) sends out the enemy here; the harness is
        // always the first.
        Ok(())
    }
}

#[allow(dead_code)]
const _: u16 = PARTY_LEN;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::EnemyControl;
    use crate::data::random::random_battle_config;

    /// A wild battle against `species`, from a random battle's player side.
    fn wild_against(species: &str) -> BattleConfig {
        let mut config = random_battle_config(1);
        config.battle.kind = BattleKind::WildEncounter;
        config.battle.trainer = None;
        config.battle.enemy_control = EnemyControl::default();
        config.enemy_items.clear();
        config.enemy_trainer = None;
        let mut mon = config.player_party[0].clone();
        mon.species = species.to_string();
        mon.moves = None;
        mon.item = None;
        config.enemy_party = vec![mon];
        config
    }

    #[test]
    fn refuses_a_wild_unown_with_no_unlocked_letters() {
        for seed in 1..20 {
            let error = start(&wild_against("Unown"), seed, false).err();
            assert_eq!(error, Some(StartError::Invalid(UNOWN_FREEZE.to_string())));
        }
    }

    #[test]
    fn starts_other_wild_battles() {
        assert!(start(&wild_against("Pidgey"), 1, false).is_ok());
    }
}
