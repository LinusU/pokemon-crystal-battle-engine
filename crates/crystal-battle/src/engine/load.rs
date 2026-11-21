//! A battle from a [`BattleSetup`]: what the game holds, written into the
//! engine's state the way the game keeps it, then the game's own battle
//! start. And the reverse for a scenario: [`generate`] makes the setup a
//! [`BattleConfig`] describes, the way the game makes new mons.

use std::sync::Arc;

use super::battle::{Engine, Flags};
use super::start::{Launch, StartError};
use super::sym::*;
use crate::battle::result::BattleResult;
use crate::battle::setup::{Bag, BattleSetup, Opponent, Options, Player, Pokemon, World};
use crate::battle::Outcome;
use crate::config::{BattleConfig, BattleKind, EnemyControl};
use crate::data::GameData;
use crate::defs::*;

const STRUCT: u16 = PARTYMON_STRUCT_LENGTH as u16;

/// Start a battle from `setup` with the random stream at `rng`.
pub fn start(data: &'static GameData, setup: &BattleSetup, rng: u64, events: bool) -> Result<Engine, StartError> {
    let (link, external) = match &setup.opponent {
        Opponent::Link { .. } => (true, false),
        Opponent::Trainer { control, .. } => (false, *control == EnemyControl::External),
        Opponent::Wild { .. } => (false, false),
    };
    let rng = if rng == 0 { 0x9e37_79b9_7f4a_7c15 } else { rng };
    let mut engine = Engine::new(super::start::boot_mem(setup.options.battle_scene), rng, Flags {
        link_battle: link,
        external_enemy: external,
        ..Flags::default()
    });
    engine.data = data;
    if events {
        engine.events = Some(Vec::new());
    }
    engine.seed_player_from(&setup.player, &setup.world);
    engine.write_bag(&setup.bag)?;
    engine.write_party(&setup.party, false)?;
    let launch = engine.launch_opponent(setup)?;
    engine.start_battle(&launch)?;
    if let Opponent::Trainer { ai_flags: Some((moves, items)), .. } = setup.opponent {
        let class = engine.mem.rb(W_TRAINER_CLASS);
        let mut patched = *engine.data.trainer_class(class);
        patched.ai_move_weights = moves.to_le_bytes();
        patched.ai_item_switch = items.to_le_bytes();
        engine.trainer_class_patch = Some((class, patched));
    }
    Ok(engine)
}

impl Engine {
    /// The player, as `seed_player` leaves them, from the setup.
    fn seed_player_from(&mut self, player: &Player, world: &World) {
        self.seed_player();
        self.names.player = player.name.as_str().into();
        self.names.rival = world.rival_name.as_str().into();
        self.mem.ww(W_PLAYER_ID, player.id);
        self.mem.wb(W_PLAYER_GENDER, player.gender);
        self.mem.wb(W_TIME_OF_DAY, world.time_of_day);
        self.mem.wb(W_UNLOCKED_UNOWNS, world.unlocked_unowns);
        self.location = world.location;
        for (list, addr) in [(&world.seen, W_POKEDEX_SEEN), (&world.caught, W_POKEDEX_CAUGHT)] {
            for &species in list {
                let i = u16::from(species.wrapping_sub(1));
                let byte = addr + i / 8;
                let v = self.mem.rb(byte) | 1 << (i % 8);
                self.mem.wb(byte, v);
            }
        }
        for (i, &letter) in (0..).zip(world.unown_dex.iter().take(usize::from(NUM_UNOWN))) {
            self.mem.wb(W_UNOWN_DEX + i, letter);
        }
        let money = player.money.to_be_bytes();
        for i in 0..3 {
            self.mem.wb(W_MONEY + i, money[usize::from(i) + 1]);
        }
        self.mem.wb(W_MOM_SAVING_MONEY, player.mom_saving);
        let mom = player.mom_money.to_be_bytes();
        for i in 0..3 {
            self.mem.wb(W_MOMS_MONEY + i, mom[usize::from(i) + 1]);
        }
        self.mem.wb(W_JOHTO_BADGES, player.johto_badges);
        self.mem.wb(W_KANTO_BADGES, player.kanto_badges);
    }

    fn write_bag(&mut self, bag: &Bag) -> Result<(), StartError> {
        if bag.items.len() > usize::from(MAX_ITEMS) || bag.balls.len() > usize::from(MAX_BALLS) {
            return Err("too many entries in a bag pocket".to_string().into());
        }
        self.write_pocket(W_NUM_ITEMS, &bag.items);
        self.write_pocket(W_NUM_BALLS, &bag.balls);
        Ok(())
    }

    /// A party into the player's (or, `opposing`, the enemy's) party.
    fn write_party(&mut self, party: &[Pokemon], opposing: bool) -> Result<(), StartError> {
        if party.is_empty() || party.len() > usize::from(PARTY_LENGTH) {
            return Err(format!("a party has 1..={PARTY_LENGTH} mons, got {}", party.len()).into());
        }
        let (count, mon1) = if opposing { (W_OT_PARTY_COUNT, W_OT_PARTY_MON1) } else { (W_PARTY_COUNT, W_PARTY_MON1) };
        self.mem.wb(count, party.len() as u8);
        for (slot, mon) in (0u16..).zip(party) {
            self.mem.wb(count + 1 + slot, mon.species);
            for (i, b) in (0..).zip(mon.to_bytes()) {
                self.mem.wb(mon1 + slot * STRUCT + i, b);
            }
        }
        self.mem.wb(count + 1 + party.len() as u16, 0xff);
        let nicknames: Vec<Arc<str>> = party.iter().map(|m| m.nickname.as_str().into()).collect();
        let ots: Vec<Arc<str>> = party.iter().map(|m| m.ot_name.as_str().into()).collect();
        if opposing {
            self.names.ot_party = nicknames;
            self.names.ot_party_ots = ots;
        } else {
            self.names.party = nicknames;
            self.names.party_ots = ots;
        }
        Ok(())
    }

    fn launch_opponent(&mut self, setup: &BattleSetup) -> Result<Launch, StartError> {
        match &setup.opponent {
            Opponent::Wild { species, level } => self.launch_wild(*species, *level),
            Opponent::Trainer { class, id, party, items, .. } => {
                let mut launch = Launch::default();
                if let Some(party) = party {
                    self.write_party(party, true)?;
                    self.mem.wb(W_CUR_OT_MON, 0);
                    self.mem.wb(W_CUR_PARTY_MON, 0);
                    launch.trainer_party_override = true;
                }
                launch.enemy_item_override = *items;
                self.mem.wb(W_BATTLE_MODE, TRAINER_BATTLE);
                self.mem.wb(W_BATTLE_TYPE, BATTLETYPE_NORMAL);
                self.mem.wb(W_OTHER_TRAINER_CLASS, *class);
                self.mem.wb(W_OTHER_TRAINER_ID, *id);
                Ok(launch)
            }
            Opponent::Link { name, party } => {
                self.write_party(party, true)?;
                self.launch_link_with(name)
            }
        }
    }
}

/// The setup `config` describes, made the way the game makes new mons
/// (`TryAddMonToParty`), and the random stream after it (making mons draws
/// DVs). Starting [`start`] from both is the same battle as
/// `start::start(config, seed, ..)`.
pub fn generate(config: &BattleConfig, seed: u64) -> Result<(BattleSetup, u64), StartError> {
    generate_in(crate::data::GameData::crystal(), config, seed)
}

/// [`generate`], making the mons from `data`.
pub fn generate_in(data: &'static crate::data::GameData, config: &BattleConfig, seed: u64) -> Result<(BattleSetup, u64), StartError> {
    let rng = if seed == 0 { 0x9e37_79b9_7f4a_7c15 } else { seed };
    let mut engine = Engine::new(super::start::boot_state(config), rng, Flags::default());
    engine.data = data;
    match config.battle.kind {
        BattleKind::WildEncounter => engine.launch_wild_encounter(config)?,
        BattleKind::TrainerBattle => engine.launch_trainer_battle(config)?,
        BattleKind::LinkBattle => engine.launch_link_battle(config)?,
    };
    let party = engine.read_party(false);
    let player = Player {
        name: engine.names.player.to_string(),
        id: engine.mem.rw(W_PLAYER_ID),
        gender: engine.mem.rb(W_PLAYER_GENDER),
        money: engine.money_at(W_MONEY),
        johto_badges: engine.mem.rb(W_JOHTO_BADGES),
        kanto_badges: engine.mem.rb(W_KANTO_BADGES),
        mom_saving: engine.mem.rb(W_MOM_SAVING_MONEY),
        mom_money: engine.money_at(W_MOMS_MONEY),
    };
    let bag = Bag { items: engine.read_pocket(W_NUM_ITEMS), balls: engine.read_pocket(W_NUM_BALLS) };
    let opponent = match config.battle.kind {
        BattleKind::WildEncounter => {
            Opponent::Wild { species: engine.mem.rb(W_TEMP_WILD_MON_SPECIES), level: engine.mem.rb(W_CUR_PARTY_LEVEL) }
        }
        BattleKind::TrainerBattle => {
            let ai_flags = match &config.enemy_trainer {
                Some(ov) => {
                    let class = engine.mem.rb(W_OTHER_TRAINER_CLASS);
                    let mut patched = *engine.data.trainer_class(class);
                    crate::config::apply_ai_flag_override(&mut patched, ov)?;
                    Some((u16::from_le_bytes(patched.ai_move_weights), u16::from_le_bytes(patched.ai_item_switch)))
                }
                None => None,
            };
            Opponent::Trainer {
                class: engine.mem.rb(W_OTHER_TRAINER_CLASS),
                id: engine.mem.rb(W_OTHER_TRAINER_ID),
                party: (!config.enemy_party.is_empty()).then(|| engine.read_party(true)),
                items: super::start::enemy_items_override(&config.enemy_items)?,
                ai_flags,
                control: config.battle.enemy_control,
            }
        }
        BattleKind::LinkBattle => Opponent::Link { name: engine.names.ot_player.to_string(), party: engine.read_party(true) },
    };
    let setup = BattleSetup {
        player,
        party,
        bag,
        opponent,
        options: Options { battle_scene: config.battle.battle_scene != Some(false) },
        world: engine.read_world(),
    };
    Ok((setup, engine.rng))
}

impl Engine {
    /// The player's (or, `opposing`, the enemy's) party as [`Pokemon`].
    pub fn read_party(&self, opposing: bool) -> Vec<Pokemon> {
        let (count, mon1, names, ots) = if opposing {
            (W_OT_PARTY_COUNT, W_OT_PARTY_MON1, &self.names.ot_party, &self.names.ot_party_ots)
        } else {
            (W_PARTY_COUNT, W_PARTY_MON1, &self.names.party, &self.names.party_ots)
        };
        (0..u16::from(self.mem.rb(count).min(PARTY_LENGTH)))
            .map(|slot| {
                let bytes: [u8; 48] = std::array::from_fn(|i| self.mem.rb(mon1 + slot * STRUCT + i as u16));
                let name = |list: &Vec<Arc<str>>| list.get(usize::from(slot)).map(|s| s.to_string()).unwrap_or_default();
                Pokemon::from_bytes(&bytes, name(names), name(ots))
            })
            .collect()
    }

    fn read_pocket(&self, num: u16) -> Vec<(u8, u8)> {
        (0..u16::from(self.mem.rb(num))).map(|i| (self.mem.rb(num + 1 + 2 * i), self.mem.rb(num + 2 + 2 * i))).collect()
    }
}

impl Engine {
    /// What the battle, ended with `outcome`, leaves behind.
    pub fn result(&self, outcome: Outcome) -> BattleResult {
        let party = self.read_party(false);
        let flags = |addr: u16, count: u16| -> Vec<u16> {
            (0..count).filter(|&i| self.mem.rb(addr + i / 8) & 1 << (i % 8) != 0).collect()
        };
        BattleResult {
            outcome,
            caught: (outcome == Outcome::Caught).then(|| party.len() - 1),
            party,
            money: self.money_at(W_MONEY),
            mom_money: self.money_at(W_MOMS_MONEY),
            pay_day: self.money_at(W_PAY_DAY_MONEY),
            bag: Bag { items: self.read_pocket(W_NUM_ITEMS), balls: self.read_pocket(W_NUM_BALLS) },
            may_evolve: flags(W_EVOLVABLE_FLAGS, u16::from(PARTY_LENGTH)).into_iter().map(usize::from).collect(),
            world: self.read_world(),
        }
    }

    /// The world state the battle keeps.
    fn read_world(&self) -> World {
        let species = |addr: u16| -> Vec<u8> {
            (0..u16::from(NUM_POKEMON)).filter(|&i| self.mem.rb(addr + i / 8) & 1 << (i % 8) != 0).map(|i| i as u8 + 1).collect()
        };
        World {
            time_of_day: self.mem.rb(W_TIME_OF_DAY),
            unlocked_unowns: self.mem.rb(W_UNLOCKED_UNOWNS),
            rival_name: self.names.rival.to_string(),
            seen: species(W_POKEDEX_SEEN),
            caught: species(W_POKEDEX_CAUGHT),
            unown_dex: (0..u16::from(NUM_UNOWN)).map(|i| self.mem.rb(W_UNOWN_DEX + i)).take_while(|&l| l != 0).collect(),
            location: self.location,
        }
    }
}
