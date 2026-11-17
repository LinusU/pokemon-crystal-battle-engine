//! Names. The original copies names around as text: `GetName` and friends
//! put a species', move's or item's name into `wStringBuffer1`, nicknames are
//! copied into `wBattleMonNickname`, and the battle texts print what the
//! buffers hold. The engine keeps what each buffer *names* instead
//! ([`Name`]), and the mons' nicknames as given; it never handles text.

use std::sync::Arc;

use super::battle::Engine;
use super::sym::*;
use crate::battle::Side;
use crate::defs::{ITEM_NAME, MON_NAME, MOVE_NAME, TM01, TRAINER_NAME};

const MON_NAME_LENGTH: u16 = 11;
const ITEM_NAME_LENGTH: u16 = 13;

/// What one of the original's name buffers holds.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum Name {
    #[default]
    Empty,
    Species(u8),
    Move(u8),
    Item(u8),
    Type(u8),
    /// A `StatNames` entry (1 = Attack).
    Stat(u8),
    TrainerClass(u8),
    /// A mon's nickname, or a player's name.
    Nickname(Arc<str>),
    /// "Your" / "Enemy" (`HandleScreens`).
    Owner(Side),
}

/// The battle's names: the ones the scenario gives, and what the original's
/// name buffers hold.
#[derive(Debug, Clone, Default)]
pub struct Names {
    pub player: Arc<str>,
    pub rival: Arc<str>,
    /// `wOTPlayerName`: the enemy trainer's name (or the rival's, or the
    /// other player's in a link battle).
    pub ot_player: Arc<str>,
    /// The player's party's nicknames, by slot.
    pub party: Vec<Arc<str>>,
    /// The enemy party's nicknames, by slot.
    pub ot_party: Vec<Arc<str>>,
    /// The player's party's original trainers' names, by slot.
    pub party_ots: Vec<Arc<str>>,
    /// The enemy party's original trainers' names, by slot.
    pub ot_party_ots: Vec<Arc<str>>,
    /// `wOTClassName`.
    pub ot_class: Name,
    pub string_buffer1: Name,
    pub string_buffer2: Name,
    pub mon_or_item: Name,
    /// `wBattleMonNickname`.
    pub battle_mon: Name,
    /// `wEnemyMonNickname`.
    pub enemy_mon: Name,
}

impl Engine {
    /// The name the original keeps at `addr` (a name buffer, a party
    /// nickname, a player's name).
    pub fn name_at(&self, addr: u16) -> Name {
        let names = &self.names;
        let nickname = |list: &[Arc<str>], base: u16| {
            let slot = usize::from((addr - base) / MON_NAME_LENGTH);
            Name::Nickname(list.get(slot).cloned().unwrap_or_default())
        };
        match addr {
            W_STRING_BUFFER1 => names.string_buffer1.clone(),
            W_STRING_BUFFER2 => names.string_buffer2.clone(),
            W_MON_OR_ITEM_NAME_BUFFER => names.mon_or_item.clone(),
            W_BATTLE_MON_NICKNAME => names.battle_mon.clone(),
            W_ENEMY_MON_NICKNAME => names.enemy_mon.clone(),
            W_OT_CLASS_NAME => names.ot_class.clone(),
            W_PLAYER_NAME => Name::Nickname(names.player.clone()),
            W_RIVAL_NAME => Name::Nickname(names.rival.clone()),
            W_OT_PLAYER_NAME => Name::Nickname(names.ot_player.clone()),
            a if (W_PARTY_MON_NICKNAMES..W_PARTY_MON_NICKNAMES_END).contains(&a) => nickname(&names.party, W_PARTY_MON_NICKNAMES),
            a if (W_OT_PARTY_MON_NICKNAMES..W_OT_PARTY_DATA_END).contains(&a) => nickname(&names.ot_party, W_OT_PARTY_MON_NICKNAMES),
            _ => panic!("engine: no name kept at {addr:#06x}"),
        }
    }

    /// Put `name` in the name buffer at `addr`.
    pub fn set_name_at(&mut self, addr: u16, name: Name) {
        let names = &mut self.names;
        let slot = match addr {
            W_STRING_BUFFER1 => &mut names.string_buffer1,
            W_STRING_BUFFER2 => &mut names.string_buffer2,
            W_MON_OR_ITEM_NAME_BUFFER => &mut names.mon_or_item,
            W_BATTLE_MON_NICKNAME => &mut names.battle_mon,
            W_ENEMY_MON_NICKNAME => &mut names.enemy_mon,
            W_OT_CLASS_NAME => &mut names.ot_class,
            _ => panic!("engine: no name buffer at {addr:#06x}"),
        };
        *slot = name;
    }

    /// Copy the name at `src` to the buffer at `dst` (`CopyBytes` of a name).
    pub fn copy_name(&mut self, dst: u16, src: u16) {
        let name = self.name_at(src);
        self.set_name_at(dst, name);
    }

    /// `GetPokemonName`: name of species `wNamedObjectIndex` into
    /// `wStringBuffer1`. Returns `de` (`wStringBuffer1`).
    pub fn get_pokemon_name(&mut self) -> u16 {
        let species = self.mem.rb(W_NAMED_OBJECT_INDEX);
        self.names.string_buffer1 = Name::Species(species);
        W_STRING_BUFFER1
    }

    /// `GetName`: name `wCurSpecies` of list `wNamedObjectType` into
    /// `wStringBuffer1`.
    pub fn get_name(&mut self) {
        let kind = self.mem.rb(W_NAMED_OBJECT_TYPE);
        let id = self.mem.rb(W_CUR_SPECIES);
        let de = if kind == MON_NAME {
            self.mem.wb(W_NAMED_OBJECT_INDEX, id);
            self.get_pokemon_name() + MON_NAME_LENGTH
        } else {
            self.names.string_buffer1 = match kind {
                MOVE_NAME => Name::Move(id),
                ITEM_NAME => Name::Item(id),
                TRAINER_NAME => Name::TrainerClass(id),
                _ => panic!("engine: GetName of list {kind}"),
            };
            W_STRING_BUFFER1 + ITEM_NAME_LENGTH
        };
        self.mem.ww_le(W_UNUSED_NAMES_POINTER, de);
    }

    /// `GetMoveName`: move `wNamedObjectIndex` into `wStringBuffer1`.
    pub fn get_move_name(&mut self) -> u16 {
        self.mem.wb(W_NAMED_OBJECT_TYPE, MOVE_NAME);
        let mv = self.mem.rb(W_NAMED_OBJECT_INDEX);
        self.mem.wb(W_CUR_SPECIES, mv);
        self.get_name();
        W_STRING_BUFFER1
    }

    /// `GetItemName`: item `wNamedObjectIndex` into `wStringBuffer1`.
    pub fn get_item_name(&mut self) -> u16 {
        let item = self.mem.rb(W_NAMED_OBJECT_INDEX);
        if item >= TM01 {
            self.get_tmhm_name();
            return W_STRING_BUFFER1;
        }
        self.mem.wb(W_CUR_SPECIES, item);
        self.mem.wb(W_NAMED_OBJECT_TYPE, ITEM_NAME);
        self.get_name();
        W_STRING_BUFFER1
    }

    /// `GetTMHMName`: "TM##" / "HM##" for item `wNamedObjectIndex` into
    /// `wStringBuffer1` (the item's name in the game data).
    pub fn get_tmhm_name(&mut self) {
        let item = self.mem.rb(W_NAMED_OBJECT_INDEX);
        self.names.string_buffer1 = Name::Item(item);
    }

    /// `CopyName1`: the name at `de` into `wStringBuffer2`.
    pub fn copy_name1(&mut self, de: u16) {
        self.copy_name2(de, W_STRING_BUFFER2);
    }

    /// `CopyName2`: the name at `de` to `hl`.
    pub fn copy_name2(&mut self, de: u16, hl: u16) {
        self.copy_name(hl, de);
    }

    /// The name `name` stands for, as the game's data spells it (a
    /// nickname as given).
    pub fn name_text(&self, name: &Name) -> String {
        let names = self.data.names;
        match name {
            Name::Empty => String::new(),
            Name::Species(id) => names.species[usize::from(*id)].to_string(),
            Name::Move(id) => names.moves[usize::from(*id)].to_string(),
            Name::Item(id) => names.items[usize::from(*id)].to_string(),
            Name::Type(id) => names.types[usize::from(*id) % names.types.len()].to_string(),
            Name::Stat(id) => names.stats[usize::from(id.wrapping_sub(1)) % names.stats.len()].to_string(),
            Name::TrainerClass(id) => names.trainer_classes[usize::from(*id)].to_string(),
            Name::Nickname(s) => s.to_string(),
            Name::Owner(side) => match side {
                Side::Player => "Your".to_string(),
                Side::Enemy => "Enemy".to_string(),
            },
        }
    }
}
