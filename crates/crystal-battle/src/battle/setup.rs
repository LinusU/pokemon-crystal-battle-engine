//! What a battle starts from, as a game holds it: the player, their party
//! (every field of every mon), their bag, the opponent, the options and the
//! bits of world state the battle reads. A game builds a [`BattleSetup`]
//! from its save; a scenario ([`crate::config::BattleConfig`]) makes one
//! the way the game itself makes new mons.

use crate::config::EnemyControl;

/// One Pokémon as a save holds it: the game's `party_struct`, its nickname
/// and its original trainer's name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pokemon {
    pub species: u8,
    pub item: u8,
    pub moves: [u8; 4],
    /// The original trainer's ID; another ID than the player's makes the mon
    /// traded (it gains more experience and may disobey).
    pub ot_id: u16,
    /// Total experience (a 24-bit number).
    pub exp: u32,
    /// Stat experience: HP, Attack, Defense, Speed, Special.
    pub stat_exp: [u16; 5],
    /// DVs: Attack and Defense nibbles, then Speed and Special.
    pub dvs: [u8; 2],
    /// Current PP per move; the top two bits are the PP Ups.
    pub pp: [u8; 4],
    pub happiness: u8,
    pub pokerus: u8,
    /// Caught data: time and level, then OT gender and location.
    pub caught: [u8; 2],
    pub level: u8,
    /// Status: bits 0-2 sleep turns, then poison, burn, freeze, paralysis.
    pub status: u8,
    pub hp: u16,
    /// Max HP, Attack, Defense, Speed, Special Attack, Special Defense.
    pub stats: [u16; 6],
    pub nickname: String,
    pub ot_name: String,
}

impl Pokemon {
    /// The mon's `party_struct` (48 bytes).
    pub fn to_bytes(&self) -> [u8; 48] {
        let mut b = [0u8; 48];
        b[0] = self.species;
        b[1] = self.item;
        b[2..6].copy_from_slice(&self.moves);
        b[6..8].copy_from_slice(&self.ot_id.to_be_bytes());
        b[8..11].copy_from_slice(&self.exp.to_be_bytes()[1..]);
        for (i, v) in self.stat_exp.iter().enumerate() {
            b[11 + 2 * i..13 + 2 * i].copy_from_slice(&v.to_be_bytes());
        }
        b[21..23].copy_from_slice(&self.dvs);
        b[23..27].copy_from_slice(&self.pp);
        b[27] = self.happiness;
        b[28] = self.pokerus;
        b[29..31].copy_from_slice(&self.caught);
        b[31] = self.level;
        b[32] = self.status;
        b[34..36].copy_from_slice(&self.hp.to_be_bytes());
        for (i, v) in self.stats.iter().enumerate() {
            b[36 + 2 * i..38 + 2 * i].copy_from_slice(&v.to_be_bytes());
        }
        b
    }

    /// A mon from its `party_struct` and names.
    pub fn from_bytes(b: &[u8; 48], nickname: String, ot_name: String) -> Pokemon {
        let be = |i: usize| u16::from_be_bytes([b[i], b[i + 1]]);
        Pokemon {
            species: b[0],
            item: b[1],
            moves: [b[2], b[3], b[4], b[5]],
            ot_id: be(6),
            exp: u32::from_be_bytes([0, b[8], b[9], b[10]]),
            stat_exp: std::array::from_fn(|i| be(11 + 2 * i)),
            dvs: [b[21], b[22]],
            pp: [b[23], b[24], b[25], b[26]],
            happiness: b[27],
            pokerus: b[28],
            caught: [b[29], b[30]],
            level: b[31],
            status: b[32],
            hp: be(34),
            stats: std::array::from_fn(|i| be(36 + 2 * i)),
            nickname,
            ot_name,
        }
    }
}

/// The player.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub name: String,
    pub id: u16,
    /// 0 boy, 1 girl.
    pub gender: u8,
    pub money: u32,
    /// One bit per badge (Zephyr first).
    pub johto_badges: u8,
    /// One bit per badge (Boulder first).
    pub kanto_badges: u8,
    /// Whether Mom saves part of the prize money (`wMomSavingMoney`).
    pub mom_saving: u8,
    /// Mom's savings.
    pub mom_money: u32,
}

/// The bag: the pockets a battle can use, as (item, quantity).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Bag {
    pub items: Vec<(u8, u8)>,
    pub balls: Vec<(u8, u8)>,
}

/// Who the player battles.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Opponent {
    /// A wild mon of `species` at `level` (the battle generates the rest).
    Wild { species: u8, level: u8 },
    /// A trainer: class and id pick the game's own party, items and AI,
    /// unless overridden.
    Trainer {
        class: u8,
        id: u8,
        /// A party of the scenario's own instead of the game's.
        party: Option<Vec<Pokemon>>,
        /// The two items the AI may use instead of the class's.
        items: Option<[u8; 2]>,
        /// The AI's move-scoring and item/switch flags instead of the
        /// class's (`TRNATTR_AI_MOVE_WEIGHTS`, `TRNATTR_AI_ITEM_SWITCH`).
        ai_flags: Option<(u16, u16)>,
        /// Whether the game's AI decides, or the caller.
        control: EnemyControl,
    },
    /// Another player, both sides decided by the caller.
    Link { name: String, party: Vec<Pokemon> },
}

/// The options the battle reads.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Options {
    /// Battle animations on (they decide some of what is shown).
    pub battle_scene: bool,
}

impl Default for Options {
    fn default() -> Self {
        Options { battle_scene: true }
    }
}

/// The world state a battle reads.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct World {
    /// `MORN_F`, `DAY_F` or `NITE_F`.
    pub time_of_day: u8,
    /// The Unown letter sets unlocked (`wUnlockedUnowns`).
    pub unlocked_unowns: u8,
    /// The rival's name (`<RIVAL>`, the rival trainer classes).
    pub rival_name: String,
    /// Species the player has seen (the Pokédex).
    pub seen: Vec<u8>,
    /// Species the player has caught (a catch of a new one is announced).
    pub caught: Vec<u8>,
    /// The Unown letters caught, in order (`wUnownDex`).
    pub unown_dex: Vec<u8>,
    /// The Pokégear location of the map the battle is on (`LANDMARK_*`; for
    /// the Pokémon Center 2F, of the map before it): a caught mon's caught
    /// data, and a level-up at its home location.
    pub location: u8,
}

impl Default for World {
    fn default() -> Self {
        World {
            time_of_day: crate::defs::DAY_F,
            unlocked_unowns: 0,
            rival_name: "SILVER".into(),
            seen: Vec::new(),
            caught: Vec::new(),
            unown_dex: Vec::new(),
            location: crate::engine::battle::HARNESS_MAP_LOCATION,
        }
    }
}

/// Everything a battle starts from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BattleSetup {
    pub player: Player,
    pub party: Vec<Pokemon>,
    pub bag: Bag,
    pub opponent: Opponent,
    pub options: Options,
    pub world: World,
}

impl BattleSetup {
    /// The setup a scenario describes, made the way the game makes new mons
    /// (which draws random numbers), and the random stream after it: start
    /// the battle with that stream ([`Battle::start`](super::Battle::start))
    /// to play the scenario's battle with engine seed `seed`.
    pub fn from_config(config: &crate::config::BattleConfig, seed: u64) -> Result<(BattleSetup, u64), String> {
        crate::engine::load::generate(config, seed).map_err(|error| error.to_string())
    }
}
