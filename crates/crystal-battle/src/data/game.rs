//! The game's data, typed: species, moves, items, trainer classes, the type
//! chart, move effect scripts and the small tables the battle mechanics
//! consult. [`GameData::crystal`] is Pokémon Crystal's, generated from the
//! ROM by `tools/gen_game_data.py` (see [`super::crystal`]);
//! `GameData::crystal_legacy` (feature `crystal-legacy`) is Crystal Legacy's,
//! generated the same way.
//!
//! Tables indexed by an id (species, move, item, trainer class, …) have one
//! entry for every `u8`. Entries outside the real range (species 0 or 252+,
//! move 0, …) hold what the original reads there: its tables are indexed
//! with `id - 1` and no bounds check, and some battles do reach them (a move
//! 0 in a glitched move slot, Present's or Magnitude's lookups). A data set
//! that isn't generated from a ROM can fill them with anything.
//!
//! The library never needs a `GameData` that outlives a battle less than
//! `'static`: build one once (or use [`GameData::crystal`]) and leak it.

/// Everything the battle engine reads from the game's data.
#[derive(Debug)]
pub struct GameData {
    /// Per species id (256 entries).
    pub species: &'static [Species],
    /// Per move id (256 entries).
    pub moves: &'static [Move],
    /// Per item id (256 entries).
    pub items: &'static [Item],
    /// Per trainer class id (256 entries).
    pub trainer_classes: &'static [TrainerClass],
    /// `TypeMatchups`, in order; [`TypeMatchup::Foresight`] marks where the
    /// matchups Foresight ignores begin.
    pub type_matchups: &'static [TypeMatchup],
    /// Per move effect id (256 entries): the effect's battle-command script,
    /// without its closing `endmove`. Empty for an effect id whose script
    /// the original can't run.
    pub effect_scripts: &'static [&'static [u8]],
    /// Per growth rate (256 entries, `GrowthRates` indexed by `rate * 4`
    /// as a byte, like `CalcExpAtLevel`).
    pub growth_rates: &'static [GrowthRate],
    /// The mechanics' small tables.
    pub tables: &'static Tables,
    /// Names, as the game spells them.
    pub names: &'static Names,
    /// Every trainer's party, in the original's order (`TrainerGroups`).
    pub trainer_parties: &'static [TrainerParty],
    /// Per trainer class id (256 entries): the index of its first party in
    /// [`GameData::trainer_parties`], `u16::MAX` for none.
    pub trainer_class_first_party: &'static [u16],
    /// What the game's battle code does differently from Crystal's.
    pub rules: Rules,
}

/// The ways a game's battle code differs from Pokémon Crystal's: the
/// changes a pokecrystal fork makes to the code rather than to its data.
/// Each flag is off in [`Rules::CRYSTAL`]; [`Rules::CRYSTAL_LEGACY`] turns on
/// Crystal Legacy's (V1.3.1) changes, ported from its source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Rules {
    /// Ghost moves are special and Dark moves physical, wherever the battle
    /// asks: damage, critical hits, Counter, Mirror Coat and the AI. Crystal
    /// goes by the type's place in the type list, special from Fire on.
    pub ghost_special_dark_physical: bool,
    /// Crystal Legacy's fixes to the battle's known bugs (pokecrystal's
    /// `docs/bugs_and_glitches.md`): Return and Frustration have at least 1
    /// power.
    pub bug_fixes: bool,
}

impl Rules {
    /// Pokémon Crystal's own code.
    pub const CRYSTAL: Rules = Rules { ghost_special_dark_physical: false, bug_fixes: false };
    /// Crystal Legacy's code.
    pub const CRYSTAL_LEGACY: Rules = Rules { ghost_special_dark_physical: true, bug_fixes: true };
}

/// A trainer's party (`data/trainers/parties.asm`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TrainerParty {
    pub name: &'static str,
    /// `TRAINERTYPE_*`: whether the mons have items and moves.
    pub kind: u8,
    pub mons: &'static [TrainerMon],
}

/// One mon of a trainer's party.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TrainerMon {
    pub level: u8,
    pub species: u8,
    pub item: Option<u8>,
    pub moves: Option<[u8; 4]>,
}

/// Names (species, moves, …), as the game spells them. Indexed like the
/// tables: 256 entries per id, past the real range what the original reads.
#[derive(Debug)]
pub struct Names {
    pub species: &'static [&'static str],
    pub moves: &'static [&'static str],
    pub items: &'static [&'static str],
    pub trainer_classes: &'static [&'static str],
    /// Per type id (32 entries).
    pub types: &'static [&'static str],
    /// `StatNames`: `stats[0]` is stat 1 (ATTACK), through "ABILITY".
    pub stats: &'static [&'static str],
}

/// A species' base data (`BaseData`), its evolutions and level-up moves
/// (`EvosAttacks`), and the weight the Heavy Ball reads.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Species {
    pub dex_no: u8,
    /// HP, Attack, Defense, Speed, Special Attack, Special Defense.
    pub base_stats: [u8; 6],
    pub types: [u8; 2],
    pub catch_rate: u8,
    pub base_exp: u8,
    /// The items a wild mon of this species may hold (common, rare).
    pub items: [u8; 2],
    pub gender_ratio: u8,
    pub unknown1: u8,
    pub egg_steps: u8,
    pub unknown2: u8,
    pub pic_size: u8,
    /// The unused front/back sprite pointers (copied into `wBaseData`).
    pub beta_pics: [u8; 4],
    pub growth_rate: u8,
    pub egg_groups: u8,
    /// TM/HM compatibility bits.
    pub tmhm: [u8; 8],
    pub evolutions: &'static [Evolution],
    /// Level-up moves, `(level, move)`, as `LearnLevelMoves` / `FillMoves`
    /// read them: they skip the evolutions byte by byte to the first 0.
    pub learnset: &'static [(u8, u8)],
    /// The weight `HeavyBallMultiplier` reads, in hectograms. For three
    /// species it is not the Pokédex weight (BUG: the wrong Pokédex bank).
    pub heavy_ball_weight: u16,
}

impl Species {
    /// The species' 32 bytes of `BaseData`, as `GetBaseData` copies them to
    /// `wCurBaseData`.
    pub fn base_data_bytes(&self) -> [u8; 32] {
        let mut b = [0u8; 32];
        b[0] = self.dex_no;
        b[1..7].copy_from_slice(&self.base_stats);
        b[7..9].copy_from_slice(&self.types);
        b[9] = self.catch_rate;
        b[10] = self.base_exp;
        b[11..13].copy_from_slice(&self.items);
        b[13] = self.gender_ratio;
        b[14] = self.unknown1;
        b[15] = self.egg_steps;
        b[16] = self.unknown2;
        b[17] = self.pic_size;
        b[18..22].copy_from_slice(&self.beta_pics);
        b[22] = self.growth_rate;
        b[23] = self.egg_groups;
        b[24..32].copy_from_slice(&self.tmhm);
        b
    }
}

/// One `EvosAttacks` evolution entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Evolution {
    /// `EVOLVE_LEVEL`.
    Level { level: u8, species: u8 },
    /// `EVOLVE_ITEM`.
    Item { item: u8, species: u8 },
    /// `EVOLVE_TRADE`, holding `item` (or any).
    Trade { item: u8, species: u8 },
    /// `EVOLVE_HAPPINESS` at `time` of day (`TR_*`).
    Happiness { time: u8, species: u8 },
    /// `EVOLVE_STAT`: at `level`, when Attack compares to Defense as `cmp`
    /// (`ATK_*_DEF`).
    Stat { level: u8, cmp: u8, species: u8 },
}

impl Evolution {
    /// The `EVOLVE_*` id that starts the entry.
    pub fn method(&self) -> u8 {
        use crate::defs::*;
        match self {
            Evolution::Level { .. } => EVOLVE_LEVEL,
            Evolution::Item { .. } => EVOLVE_ITEM,
            Evolution::Trade { .. } => EVOLVE_TRADE,
            Evolution::Happiness { .. } => EVOLVE_HAPPINESS,
            Evolution::Stat { .. } => EVOLVE_STAT,
        }
    }
}

/// A move (`Moves`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Move {
    pub animation: u8,
    pub effect: u8,
    pub power: u8,
    pub kind: u8,
    pub accuracy: u8,
    pub pp: u8,
    pub effect_chance: u8,
}

impl Move {
    /// The move's 7 bytes, as the battle copies them into a move struct.
    pub fn bytes(&self) -> [u8; 7] {
        [self.animation, self.effect, self.power, self.kind, self.accuracy, self.pp, self.effect_chance]
    }

    /// The field at a `MOVE_*` offset.
    pub fn field(&self, offset: u8) -> u8 {
        self.bytes()[usize::from(offset)]
    }
}

/// An item's attributes (`ItemAttributes`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Item {
    pub price: u16,
    /// Held effect (`HELD_*`).
    pub effect: u8,
    pub param: u8,
    pub permissions: u8,
    pub pocket: u8,
    pub help: u8,
}

/// A trainer class: `TrainerClassAttributes` and `TrainerClassDVs`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TrainerClass {
    /// The two items the class's AI may use.
    pub items: [u8; 2],
    pub base_money: u8,
    /// AI move-scoring layers (`TRNATTR_AI_MOVE_WEIGHTS`), bit `n` of the
    /// little-endian pair = layer `n`.
    pub ai_move_weights: [u8; 2],
    /// AI item/switch flags (`TRNATTR_AI_ITEM_SWITCH`).
    pub ai_item_switch: [u8; 2],
    /// `TrainerClassDVs`: Attack/Defense, Speed/Special.
    pub dvs: [u8; 2],
}

/// One `TypeMatchups` entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TypeMatchup {
    /// `attacker` against `defender` is multiplied by `multiplier / 10`.
    Matchup { attacker: u8, defender: u8, multiplier: u8 },
    /// The Foresight marker: an identified target skips the entries after it.
    Foresight,
}

/// One growth rate (`GrowthRates`): exp at level `n` is
/// `(a/b)·n³ + c·n² + d·n − e`, `c` signed by bit 7.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GrowthRate {
    /// `a` in the high nibble, `b` in the low one.
    pub cubic: u8,
    /// `c`: bit 7 is the sign, bits 0..=6 the magnitude.
    pub quadratic: u8,
    pub linear: u8,
    pub constant: u8,
}

/// The mechanics' small tables. Lists the original ends with `-1` are
/// given without it; tables the original walks into their terminator keep
/// that entry (documented on each).
#[derive(Debug)]
pub struct Tables {
    /// `MoveEffectPriorities`: (effect, priority), ending with the `-1`
    /// entry and the byte after it (`GetMovePriority` returns it for effect
    /// `$ff`).
    pub move_effect_priorities: &'static [(u8, u8)],
    pub always_flee_mons: &'static [u8],
    pub often_flee_mons: &'static [u8],
    pub sometimes_flee_mons: &'static [u8],
    /// The first three bytes of `FleeMons`, which the Fast Ball checks
    /// (BUG: only three species).
    pub fast_ball_species: [u8; 3],
    pub asleep_tree_mons_morn: &'static [u8],
    pub asleep_tree_mons_day: &'static [u8],
    pub asleep_tree_mons_nite: &'static [u8],
    /// `StatLevelMultipliers` by stat level (256 entries; `level - 1` as a
    /// byte indexes the table).
    pub stat_level_multipliers: &'static [(u8, u8)],
    /// `StatLevelMultipliers_Applied`, the same way.
    pub stat_level_multipliers_applied: &'static [(u8, u8)],
    /// `AccuracyLevelMultipliers` by accuracy level (256 entries; the
    /// original indexes it with `(level - 1) * 2` as a byte).
    pub accuracy_level_multipliers: &'static [(u8, u8)],
    /// `CriticalHitChances` by stage (16 entries; stages past the table read
    /// what follows it).
    pub critical_hit_chances: &'static [u8],
    pub critical_hit_moves: &'static [u8],
    /// `HappinessChanges` by `HAPPINESS_*` id (256 entries; `id - 1` as a
    /// byte), per happiness bracket.
    pub happiness_changes: &'static [[u8; 3]],
    /// `AI_Items`: the items the trainer AI knows how to use, in order.
    pub ai_items: &'static [u8],
    pub status_only_effects: &'static [u8],
    pub stall_moves: &'static [u8],
    pub reckless_moves: &'static [u8],
    pub constant_damage_effects: &'static [u8],
    pub residual_moves: &'static [u8],
    pub risky_effects: &'static [u8],
    pub useful_moves: &'static [u8],
    pub encore_moves: &'static [u8],
    pub rain_dance_moves: &'static [u8],
    pub sunny_day_moves: &'static [u8],
    /// `MagnitudePower`: (threshold, power, magnitude); the last threshold is
    /// `$ff`.
    pub magnitude_power: &'static [(u8, u8, u8)],
    /// `PresentPower`: (threshold, power).
    pub present_power: &'static [(u8, u8)],
    /// `FlailReversalPower`: (threshold, power); the last threshold covers
    /// every value a battle computes.
    pub flail_reversal_power: &'static [(u8, u8)],
    /// `WeatherTypeModifiers`: (weather, type, multiplier ×10).
    pub weather_type_modifiers: &'static [(u8, u8, u8)],
    /// `WeatherMoveModifiers`: (weather, effect, multiplier ×10).
    pub weather_move_modifiers: &'static [(u8, u8, u8)],
    /// `BadgeTypeBoosts`: the type each badge boosts, Johto then Kanto.
    pub badge_type_boosts: &'static [u8],
    /// `TypeBoostItems`: (held effect, type).
    pub type_boost_items: &'static [(u8, u8)],
    pub metronome_excepts: &'static [u8],
    pub mail_items: &'static [u8],
    /// `ConsumableEffects`: held effects used up when they trigger.
    pub consumable_effects: &'static [u8],
    /// `XItemStats`: (item, stat).
    pub x_item_stats: &'static [(u8, u8)],
    /// `StatusHealingActions`: (item, party-menu action, status mask).
    pub status_healing_actions: &'static [(u8, u8, u8)],
    /// `HealingHPAmounts`: (item, HP), ending with the `-1` entry and the
    /// word after it (`GetHealingItemAmount` returns it for unknown items).
    pub healing_hp_amounts: &'static [(u8, u16)],
    /// `BallMultiplierFunctionTable`: the balls with a catch-rate function.
    pub ball_multiplier_items: &'static [u8],
    /// `WobbleProbabilities`: (catch rate, chance); the last rate is `$ff`.
    pub wobble_probabilities: &'static [(u8, u8)],
    /// `UnlockedUnownLetterSets`: the letters each unlock flag allows.
    pub unlocked_unown_letter_sets: &'static [&'static [u8]],
    /// `MagikarpLengths`: (threshold, divisor), 14 entries.
    pub magikarp_lengths: &'static [(u16, u8)],
    /// `IsHMMove.HMMoves`.
    pub hm_moves: &'static [u8],
    /// `GymLeaders` (trainer classes).
    pub gym_leaders: &'static [u8],
    /// `MoveGrammar`: the move groups that pick each "used" text.
    pub move_grammar: &'static [&'static [u8]],
}

impl GameData {
    /// Pokémon Crystal's data.
    pub fn crystal() -> &'static GameData {
        &super::crystal::CRYSTAL
    }

    /// Pokémon Crystal Legacy's data (V1.3.1). It has no trainer parties: a
    /// trainer battle brings its own.
    #[cfg(feature = "crystal-legacy")]
    pub fn crystal_legacy() -> &'static GameData {
        &super::crystal_legacy::CRYSTAL_LEGACY
    }

    pub fn species(&self, id: u8) -> &Species {
        &self.species[usize::from(id)]
    }

    pub fn mv(&self, id: u8) -> &Move {
        &self.moves[usize::from(id)]
    }

    pub fn item(&self, id: u8) -> &Item {
        &self.items[usize::from(id)]
    }

    pub fn trainer_class(&self, id: u8) -> &TrainerClass {
        &self.trainer_classes[usize::from(id)]
    }

    pub fn growth_rate(&self, id: u8) -> &GrowthRate {
        &self.growth_rates[usize::from(id)]
    }

    /// Party `id` of trainer class `class`, the way `ReadTrainerParty` finds
    /// it: counting parties from the class's first (an id past the class's
    /// last party is the next class's).
    pub fn trainer_party(&self, class: u8, id: u8) -> Option<&'static TrainerParty> {
        let first = *self.trainer_class_first_party.get(usize::from(class))?;
        if first == u16::MAX {
            return None;
        }
        self.trainer_parties.get(usize::from(first) + usize::from(id.wrapping_sub(1)))
    }

    /// The battle-command script of move effect `effect`. Panics for an
    /// effect id whose script the original can't run (a glitch move).
    pub fn effect_script(&self, effect: u8) -> &'static [u8] {
        let script = self.effect_scripts[usize::from(effect)];
        assert!(!script.is_empty(), "unsupported glitch: move effect {effect:#04x} has no script");
        script
    }
}

