//! Crystal's English for the library's [`Message`]s, as the original prints
//! them: each message is mapped back to the text label it stands for, the
//! memory that text reads is rebuilt from the message, and the text engine's
//! port (`oracle::text`) renders it. The differential testers compare this
//! with the original's captured text, which checks both the messages (which,
//! when, and that they carry what the text needs) and this mapping.

use crate::battle::message::{Message, Mon, SendOut, Trainer, Withdraw};
use crate::battle::{Event, Side};
use crate::bootstrap::events::Shown;
use crate::config::{BattleConfig, BattleKind};
use crate::data::GameData;
use crate::engine::battle::{Engine, Flags};
use crate::engine::mem::Mem;
use crate::engine::rsym as r;
use crate::engine::sym::*;

/// What a message's text needs to know about the battle that the message
/// itself doesn't carry.
#[derive(Debug, Clone, Copy)]
pub struct Context {
    pub battle_mode: u8,
    pub link: bool,
}

impl Context {
    pub fn of(config: &BattleConfig) -> Context {
        Context {
            battle_mode: if config.battle.kind == BattleKind::WildEncounter {
                crate::defs::WILD_BATTLE
            } else {
                crate::defs::TRAINER_BATTLE
            },
            link: config.battle.kind == BattleKind::LinkBattle,
        }
    }
}

/// A name in the game's charmap, `@`-terminated.
pub fn encode(name: &str) -> Vec<u8> {
    let mut out = Vec::new();
    for c in name.chars() {
        let byte = match c {
            'A'..='Z' => 0x80 + (c as u8 - b'A'),
            'a'..='z' => 0xa0 + (c as u8 - b'a'),
            '0'..='9' => 0xf6 + (c as u8 - b'0'),
            ' ' => 0x7f,
            '“' => 0x72,
            '”' => 0x73,
            '·' => 0x74,
            '…' => 0x75,
            '(' => 0x9a,
            ')' => 0x9b,
            ':' => 0x9c,
            ';' => 0x9d,
            '[' => 0x9e,
            ']' => 0x9f,
            'Ä' => 0xc0,
            'Ö' => 0xc1,
            'Ü' => 0xc2,
            'ä' => 0xc3,
            'ö' => 0xc4,
            'ü' => 0xc5,
            '←' => 0xdf,
            '\'' => 0xe0,
            '-' => 0xe3,
            '?' => 0xe6,
            '!' => 0xe7,
            '.' => 0xe8,
            '&' => 0xe9,
            'é' => 0xea,
            '→' => 0xeb,
            '▷' => 0xec,
            '▶' => 0xed,
            '▼' => 0xee,
            '♂' => 0xef,
            '¥' => 0xf0,
            '×' => 0xf1,
            '/' => 0xf3,
            ',' => 0xf4,
            '♀' => 0xf5,
            _ => panic!("render: no charmap code for {c:?} in {name:?}"),
        };
        out.push(byte);
    }
    out.push(0x50);
    out
}

/// What the original's memory holds when a text prints, rebuilt from a
/// message.
struct Scene {
    engine: Engine,
}

impl Scene {
    fn new(ctx: Context) -> Scene {
        let mut engine = Engine::new(Mem::default(), 0, Flags::default());
        engine.mem.wb(W_BATTLE_MODE, ctx.battle_mode);
        engine.mem.wb(W_LINK_MODE, if ctx.link { crate::defs::LINK_COLOSSEUM } else { 0 });
        Scene { engine }
    }

    fn write(&mut self, addr: u16, name: &str) {
        for (i, b) in (0..).zip(encode(name)) {
            self.engine.mem.wb(addr + i, b);
        }
    }

    fn byte(&mut self, addr: u16, value: u8) {
        self.engine.mem.wb(addr, value);
    }

    /// A big-endian number of `bytes` bytes, as `text_decimal` reads it.
    fn number(&mut self, addr: u16, bytes: u16, value: u32) {
        for i in 0..bytes {
            self.byte(addr + i, (value >> (8 * (bytes - 1 - i))) as u8);
        }
    }

    fn side_name(&mut self, mon: &Mon) {
        let addr = match mon.side {
            Side::Player => W_BATTLE_MON_NICKNAME,
            Side::Enemy => W_ENEMY_MON_NICKNAME,
        };
        self.write(addr, &mon.name);
    }

    /// `<USER>` is `mon` (`hBattleTurn` is its side's).
    fn user(&mut self, mon: &Mon) {
        self.byte(H_BATTLE_TURN, u8::from(mon.side == Side::Enemy));
        self.side_name(mon);
    }

    /// `<TARGET>` is `mon`.
    fn target(&mut self, mon: &Mon) {
        self.byte(H_BATTLE_TURN, u8::from(mon.side == Side::Player));
        self.side_name(mon);
    }

    fn trainer(&mut self, trainer: &Trainer, data: &GameData) {
        self.byte(W_TRAINER_CLASS, trainer.class);
        if self.engine.link_mode() != 0 {
            self.write(W_OT_CLASS_NAME, &trainer.name);
        } else if trainer.class == crate::defs::RIVAL1 || trainer.class == crate::defs::RIVAL2 {
            self.write(W_RIVAL_NAME, &trainer.name);
        } else {
            self.write(W_OT_CLASS_NAME, data.names.trainer_classes[usize::from(trainer.class)]);
            // `oracle::text` places the trainer's name from here.
            self.write(W_OT_PLAYER_NAME, &trainer.name);
        }
    }
}

/// `GetMoveGrammar` for move `mv`.
fn move_grammar(data: &GameData, mv: u8) -> u8 {
    if mv == 0 {
        return 0;
    }
    data.tables.move_grammar.iter().position(|group| group.contains(&mv)).unwrap_or(data.tables.move_grammar.len()) as u8
}

/// The messages a [`Message`] prints, exactly as the original's text engine
/// places them (text-box lines separated by `\n`).
pub fn render(message: &Message, ctx: Context) -> Vec<String> {
    let data = GameData::crystal();
    let names = data.names;
    let mut s = Scene::new(ctx);
    let mut choices = Vec::new();
    use Message as M;
    let label = match message {
        M::WildAppeared { mon } => {
            s.write(W_ENEMY_MON_NICKNAME, mon);
            r::WILD_POKEMON_APPEARED_TEXT
        }
        M::WantsToBattle { trainer } => {
            s.trainer(trainer, data);
            r::WANTS_TO_BATTLE_TEXT
        }
        M::EnemySentOut { trainer, mon } => {
            s.trainer(trainer, data);
            s.write(W_ENEMY_MON_NICKNAME, mon);
            r::BATTLE_TEXT_ENEMY_SENT_OUT
        }
        M::PlayerSentOut { mon, how } => {
            s.write(W_BATTLE_MON_NICKNAME, mon);
            match how {
                SendOut::Go => r::GO_MON_TEXT,
                SendOut::DoIt => r::DO_IT_MON_TEXT,
                SendOut::GoForIt => r::GO_FOR_IT_MON_TEXT,
                SendOut::FoesWeak => r::YOUR_FOES_WEAK_GETM_MON_TEXT,
            }
        }
        M::Withdraw { mon, how } => {
            s.write(W_BATTLE_MON_NICKNAME, mon);
            choices.push(match how {
                Withdraw::ThatsEnough => r::THATS_ENOUGH_COME_BACK_TEXT,
                Withdraw::ComeBack => r::COME_BACK_TEXT,
                Withdraw::Ok => r::OK_COME_BACK_TEXT,
                Withdraw::Good => r::GOOD_COME_BACK_TEXT,
            });
            r::WITHDRAW_MON_TEXT_WITHDRAW_MON_TEXT
        }
        M::EnemyWithdrew { trainer, mon } => {
            s.trainer(trainer, data);
            s.write(W_ENEMY_MON_NICKNAME, mon);
            r::ENEMY_WITHDREW_TEXT
        }
        M::HookedMonAttacked { mon } => {
            s.write(W_ENEMY_MON_NICKNAME, mon);
            r::HOOKED_POKEMON_ATTACKED_TEXT
        }
        M::FellFromTree { mon } => {
            s.write(W_ENEMY_MON_NICKNAME, mon);
            r::POKEMON_FELL_FROM_TREE_TEXT
        }
        M::EnemyWasDefeated { trainer } => {
            s.trainer(trainer, data);
            r::BATTLE_TEXT_ENEMY_WAS_DEFEATED
        }
        M::LostAgainst { trainer } => {
            s.trainer(trainer, data);
            r::LOST_AGAINST_TEXT
        }
        M::TiedAgainst { trainer } => {
            s.trainer(trainer, data);
            r::TIED_AGAINST_TEXT
        }
        M::GotMoney { player, money } => {
            s.write(W_PLAYER_NAME, player);
            s.number(W_BATTLE_REWARD, 3, *money);
            r::GOT_MONEY_FOR_WINNING_TEXT
        }
        M::GotMoneySentSomeToMom { player, money } => {
            s.write(W_PLAYER_NAME, player);
            s.number(W_BATTLE_REWARD, 3, *money);
            r::SENT_SOME_TO_MOM_TEXT
        }
        M::SentHalfToMom => r::SENT_HALF_TO_MOM_TEXT,
        M::SentAllToMom => r::SENT_ALL_TO_MOM_TEXT,
        M::GotAwaySafely => r::BATTLE_TEXT_GOT_AWAY_SAFELY,
        M::CantEscape => r::BATTLE_TEXT_CANT_ESCAPE,
        M::NoRunningFromTrainer => r::BATTLE_TEXT_THERES_NO_ESCAPE_FROM_TRAINER_BATTLE,
        M::WildFled { mon } => {
            s.write(W_ENEMY_MON_NICKNAME, mon);
            r::BATTLE_TEXT_WILD_FLED
        }
        M::EnemyFled { mon } => {
            s.write(W_ENEMY_MON_NICKNAME, mon);
            r::BATTLE_TEXT_ENEMY_FLED
        }
        M::FledUsingItem { user, item } => {
            s.user(user);
            s.write(W_STRING_BUFFER1, names.items[usize::from(*item)]);
            r::BATTLE_TEXT_USER_FLED_USING_A_STRING_BUFFER1
        }
        M::FledFromBattle { user } => {
            s.user(user);
            r::FLED_FROM_BATTLE_TEXT
        }
        M::MonFainted { mon } => {
            s.write(W_BATTLE_MON_NICKNAME, mon);
            r::BATTLE_TEXT_MON_FAINTED
        }
        M::EnemyMonFainted { mon } => {
            s.write(W_ENEMY_MON_NICKNAME, mon);
            r::BATTLE_TEXT_ENEMY_MON_FAINTED
        }
        M::NoMovesLeft { mon } => {
            s.write(W_BATTLE_MON_NICKNAME, mon);
            r::BATTLE_TEXT_MON_HAS_NO_MOVES_LEFT
        }
        M::AlreadyOut { mon } => {
            s.write(W_BATTLE_MON_NICKNAME, mon);
            r::BATTLE_TEXT_MON_IS_ALREADY_OUT
        }
        M::EggCantBattle => r::BATTLE_TEXT_AN_EGG_CANT_BATTLE,
        M::NoWillToBattle => r::BATTLE_TEXT_THERES_NO_WILL_TO_BATTLE,

        M::GainedExp { mon, exp, boosted } => {
            s.write(W_STRING_BUFFER1, mon);
            s.number(W_STRING_BUFFER2, 2, u32::from(*exp));
            s.byte(W_STRING_BUFFER2 + 2, u8::from(*boosted));
            r::TEXT_MON_GAINED_EXP_POINT
        }
        M::GrewToLevel { mon, level } => {
            s.write(W_STRING_BUFFER1, mon);
            s.byte(W_CUR_PARTY_LEVEL, *level);
            r::BATTLE_TEXT_STRING_BUFFER1_GREW_TO_LEVEL
        }
        M::LearnedMove { mon, mv } => {
            s.write(W_MON_OR_ITEM_NAME_BUFFER, mon);
            s.write(W_STRING_BUFFER2, names.moves[usize::from(*mv)]);
            r::LEARNED_MOVE_TEXT
        }
        M::AskForgetMove { mon, mv } => {
            s.write(W_MON_OR_ITEM_NAME_BUFFER, mon);
            s.write(W_STRING_BUFFER2, names.moves[usize::from(*mv)]);
            r::ASK_FORGET_MOVE_TEXT
        }
        M::WhichMoveToForget => r::MOVE_ASK_FORGET_TEXT,
        M::ForgotMove { mon, mv } => {
            s.write(W_MON_OR_ITEM_NAME_BUFFER, mon);
            s.write(W_STRING_BUFFER1, names.moves[usize::from(*mv)]);
            r::TEXT_1_2_AND_POOF
        }

        M::UsedMove { user, mv, instead } => {
            s.user(user);
            s.write(W_STRING_BUFFER2, names.moves[usize::from(*mv)]);
            s.byte(W_MOVE_GRAMMAR, move_grammar(data, *mv));
            s.byte(W_ALREADY_DISOBEYED, u8::from(*instead));
            r::USED_MOVE_TEXT
        }
        M::WontObey { mon } => {
            s.write(W_BATTLE_MON_NICKNAME, mon);
            r::WONT_OBEY_TEXT
        }
        M::IgnoredOrders { mon } => {
            s.write(W_BATTLE_MON_NICKNAME, mon);
            r::IGNORED_ORDERS_TEXT
        }
        M::IgnoredOrdersUser { user } => {
            s.user(user);
            r::IGNORED_ORDERS2_TEXT
        }
        M::IgnoredOrdersSleeping { mon } => {
            s.write(W_BATTLE_MON_NICKNAME, mon);
            r::IGNORED_SLEEPING_TEXT
        }
        M::LoafingAround { mon } => {
            s.write(W_BATTLE_MON_NICKNAME, mon);
            r::LOAFING_AROUND_TEXT
        }
        M::TurnedAway { mon } => {
            s.write(W_BATTLE_MON_NICKNAME, mon);
            r::TURNED_AWAY_TEXT
        }
        M::BeganToNap { mon } => {
            s.write(W_BATTLE_MON_NICKNAME, mon);
            r::BEGAN_TO_NAP_TEXT
        }
        M::NoPpLeftFor { user, mv } => {
            s.user(user);
            s.write(W_STRING_BUFFER2, names.moves[usize::from(*mv)]);
            r::HAS_NO_PP_LEFT_TEXT
        }
        M::NoPpLeft => r::NO_PP_LEFT_TEXT,
        M::Charging { user, mv } => {
            s.user(user);
            s.byte(W_PLAYER_MOVE_STRUCT_ANIMATION, *mv);
            s.byte(W_ENEMY_MOVE_STRUCT_ANIMATION, *mv);
            r::BATTLE_COMMAND_CHARGE_USED_TEXT
        }
        M::MustRecharge { user } => user_text(&mut s, user, r::MUST_RECHARGE_TEXT),
        M::StoringEnergy { user } => user_text(&mut s, user, r::STORING_ENERGY_TEXT),
        M::UnleashedEnergy { user } => user_text(&mut s, user, r::UNLEASHED_ENERGY_TEXT),
        M::Crashed { user } => user_text(&mut s, user, r::CRASHED_TEXT),
        M::Flinched { user } => user_text(&mut s, user, r::FLINCHED_TEXT),
        M::Magnitude { magnitude } => {
            s.byte(W_TEXT_DECIMAL_BYTE, *magnitude);
            r::MAGNITUDE_TEXT
        }
        M::HitTimes { times } => {
            s.byte(W_ENEMY_DAMAGE_TAKEN, *times);
            r::ENEMY_HIT_TIMES_TEXT
        }
        M::BeatUpAttack { mon } => {
            s.write(W_STRING_BUFFER1, mon);
            r::BEAT_UP_ATTACK_TEXT
        }

        M::AttackMissed { user } => user_text(&mut s, user, r::ATTACK_MISSED_TEXT),
        M::Evaded { target } => target_text(&mut s, target, r::EVADED_TEXT),
        M::CriticalHit => r::CRITICAL_HIT_TEXT,
        M::SuperEffective => r::SUPER_EFFECTIVE_TEXT,
        M::NotVeryEffective => r::NOT_VERY_EFFECTIVE_TEXT,
        M::DidntAffect { target } => target_text(&mut s, target, r::DIDNT_AFFECT1_TEXT),
        M::DoesntAffect { target } => target_text(&mut s, target, r::DOESNT_AFFECT_TEXT),
        M::Unaffected { target } => target_text(&mut s, target, r::UNAFFECTED_TEXT),
        M::OneHitKo => r::ONE_HIT_KO_TEXT,
        M::ButItFailed => r::BUT_IT_FAILED_TEXT,
        M::ItFailed => r::IT_FAILED_TEXT,
        M::NothingHappened => r::NOTHING_HAPPENED_TEXT,
        M::Endured { target } => target_text(&mut s, target, r::ENDURED_TEXT),
        M::HungOn { target, item } => {
            s.target(target);
            s.write(W_STRING_BUFFER1, names.items[usize::from(*item)]);
            r::HUNG_ON_TEXT
        }
        M::Recoil { user } => user_text(&mut s, user, r::RECOIL_TEXT),
        M::SuckedHealth { target } => target_text(&mut s, target, r::SUCKED_HEALTH_TEXT),
        M::DreamEaten { target } => target_text(&mut s, target, r::DREAM_EATEN_TEXT),
        M::RegainedHealth { user } => user_text(&mut s, user, r::REGAINED_HEALTH_TEXT),
        M::HpIsFull { user } => user_text(&mut s, user, r::HP_IS_FULL_TEXT),
        M::SharedPain => r::SHARED_PAIN_TEXT,
        M::RefusedGift { target } => target_text(&mut s, target, r::PRESENT_FAILED_TEXT),
        M::HitByFutureSight { target } => target_text(&mut s, target, r::BATTLE_TEXT_TARGET_WAS_HIT_BY_FUTURE_SIGHT),
        M::ForesawAttack { user } => user_text(&mut s, user, r::FORESAW_ATTACK_TEXT),
        M::TookAim { user } => user_text(&mut s, user, r::TOOK_AIM_TEXT),
        M::Identified { user, target } => pair_text(&mut s, user, target, r::IDENTIFIED_TEXT),
        M::TookDownWithIt { target, user } => pair_text(&mut s, user, target, r::TOOK_DOWN_WITH_IT_TEXT),
        M::DestinyBond { user } => user_text(&mut s, user, r::DESTINY_BOND_EFFECT_TEXT),
        M::MirrorMoveFailed => r::MIRROR_MOVE_FAILED_TEXT,
        M::MimicLearned { user, mv } => {
            s.user(user);
            s.write(W_STRING_BUFFER1, names.moves[usize::from(*mv)]);
            r::MIMIC_LEARNED_MOVE_TEXT
        }
        M::Sketched { user, mv } => {
            s.user(user);
            s.write(W_STRING_BUFFER1, names.moves[usize::from(*mv)]);
            r::SKETCHED_TEXT
        }
        M::Transformed { user, species } => {
            s.user(user);
            s.write(W_STRING_BUFFER1, names.species[usize::from(*species)]);
            r::TRANSFORMED_TEXT
        }
        M::TransformedType { user, kind } => {
            s.user(user);
            s.write(W_STRING_BUFFER1, names.types[usize::from(*kind) % names.types.len()]);
            r::TRANSFORMED_TYPE_TEXT
        }
        M::Spite { target, mv, amount } => {
            s.target(target);
            s.write(W_STRING_BUFFER1, names.moves[usize::from(*mv)]);
            s.byte(W_TEXT_DECIMAL_BYTE, *amount);
            r::SPITE_EFFECT_TEXT
        }
        M::Stole { user, item } => {
            s.user(user);
            s.write(W_STRING_BUFFER1, names.items[usize::from(*item)]);
            r::STOLE_TEXT
        }
        M::CoinsScattered => r::COINS_SCATTERED_TEXT,
        M::BellChimed => r::BELL_CHIMED_TEXT,
        M::BellyDrum { user } => user_text(&mut s, user, r::BELLY_DRUM_TEXT),
        M::CopiedStats { user, target } => pair_text(&mut s, user, target, r::COPIED_STATS_TEXT),
        M::StatsEliminated => r::ELIMINATED_STATS_TEXT,
        M::StartPerish => r::START_PERISH_TEXT,
        M::PerishCount { user, count } => {
            s.user(user);
            s.byte(W_TEXT_DECIMAL_BYTE, *count);
            r::PERISH_COUNT_TEXT
        }
        M::PutCurse { user, target } => pair_text(&mut s, user, target, r::PUT_A_CURSE_TEXT),
        M::TooWeakForSubstitute => r::TOO_WEAK_SUB_TEXT,
        M::MadeSubstitute { user } => user_text(&mut s, user, r::MADE_SUBSTITUTE_TEXT),
        M::HasSubstitute { user } => user_text(&mut s, user, r::HAS_SUBSTITUTE_TEXT),
        M::SubstituteTookDamage { target } => target_text(&mut s, target, r::SUB_TOOK_DAMAGE_TEXT),
        M::SubstituteFaded { target } => target_text(&mut s, target, r::SUB_FADED_TEXT),

        M::StatRose { user, stat, sharply } => {
            s.user(user);
            s.write(W_STRING_BUFFER2, stat_name(data, *stat));
            s.byte(W_LOWERED_STAT, if *sharply { 0x10 } else { 0 });
            r::BATTLE_COMMAND_STAT_UP_MESSAGE_STAT
        }
        M::StatFell { target, stat, sharply } => {
            s.target(target);
            s.write(W_STRING_BUFFER2, stat_name(data, *stat));
            s.byte(W_LOWERED_STAT, if *sharply { 0x10 } else { 0 });
            r::BATTLE_COMMAND_STAT_DOWN_MESSAGE_STAT
        }
        M::WontRiseAnymore { user, stat } => {
            s.user(user);
            s.write(W_STRING_BUFFER2, stat_name(data, *stat));
            r::WONT_RISE_ANYMORE_TEXT
        }
        M::WontDropAnymore { target, stat } => {
            s.target(target);
            s.write(W_STRING_BUFFER2, stat_name(data, *stat));
            r::WONT_DROP_ANYMORE_TEXT
        }
        M::GettingPumped { user } => user_text(&mut s, user, r::GETTING_PUMPED_TEXT),
        M::RageBuilding { user } => user_text(&mut s, user, r::RAGE_BUILDING_TEXT),

        M::FellAsleep { target } => target_text(&mut s, target, r::FELL_ASLEEP_TEXT),
        M::AlreadyAsleep { target } => target_text(&mut s, target, r::ALREADY_ASLEEP_TEXT),
        M::FastAsleep { user } => user_text(&mut s, user, r::FAST_ASLEEP_TEXT),
        M::WokeUp { user } => user_text(&mut s, user, r::WOKE_UP_TEXT),
        M::Rested { user } => user_text(&mut s, user, r::RESTED_TEXT),
        M::WentToSleep { user } => user_text(&mut s, user, r::WENT_TO_SLEEP_TEXT),
        M::WasPoisoned { target } => target_text(&mut s, target, r::WAS_POISONED_TEXT),
        M::BadlyPoisoned { target } => target_text(&mut s, target, r::BADLY_POISONED_TEXT),
        M::AlreadyPoisoned { target } => target_text(&mut s, target, r::ALREADY_POISONED_TEXT),
        M::HurtByPoison { user } => user_text(&mut s, user, r::HURT_BY_POISON_TEXT),
        M::WasBurned { target } => target_text(&mut s, target, r::WAS_BURNED_TEXT),
        M::HurtByBurn { user } => user_text(&mut s, user, r::HURT_BY_BURN_TEXT),
        M::WasFrozen { target } => target_text(&mut s, target, r::WAS_FROZEN_TEXT),
        M::AlreadyBurned { .. } | M::AlreadyFrozen { .. } => {
            panic!("render: {message:?} is Crystal Legacy's text, which Crystal's ROM doesn't have")
        }
        M::FrozenSolid { user } => user_text(&mut s, user, r::FROZEN_SOLID_TEXT),
        M::WasDefrosted { user } => user_text(&mut s, user, r::WAS_DEFROSTED_TEXT),
        M::DefrostedOpponent { target } => target_text(&mut s, target, r::DEFROSTED_OPPONENT_TEXT),
        M::Paralyzed { target } => target_text(&mut s, target, r::PARALYZED_TEXT),
        M::AlreadyParalyzed { target } => target_text(&mut s, target, r::ALREADY_PARALYZED_TEXT),
        M::FullyParalyzed { user } => user_text(&mut s, user, r::FULLY_PARALYZED_TEXT),
        M::BecameConfused { target } => target_text(&mut s, target, r::BECAME_CONFUSED_TEXT),
        M::AlreadyConfused { target } => target_text(&mut s, target, r::ALREADY_CONFUSED_TEXT),
        M::IsConfused { user } => user_text(&mut s, user, r::IS_CONFUSED_TEXT),
        M::HurtItself => r::HURT_ITSELF_TEXT,
        M::ConfusedNoMore { user } => user_text(&mut s, user, r::CONFUSED_NO_MORE_TEXT),
        M::FellInLove { target } => target_text(&mut s, target, r::FELL_IN_LOVE_TEXT),
        M::InLoveWith { user, target } => pair_text(&mut s, user, target, r::IN_LOVE_WITH_TEXT),
        M::Infatuation { user } => user_text(&mut s, user, r::INFATUATION_TEXT),
        M::StartedNightmare { target } => target_text(&mut s, target, r::STARTED_NIGHTMARE_TEXT),
        M::HasNightmare { user } => user_text(&mut s, user, r::HAS_A_NIGHTMARE_TEXT),
        M::HurtByCurse { user } => user_text(&mut s, user, r::HURT_BY_CURSE_TEXT),
        M::WasSeeded { target } => target_text(&mut s, target, r::WAS_SEEDED_TEXT),
        M::LeechSeedSaps { user } => user_text(&mut s, user, r::LEECH_SEED_SAPS_TEXT),
        M::ShedLeechSeed { user } => user_text(&mut s, user, r::SHED_LEECH_SEED_TEXT),
        M::SafeguardProtect { target } => target_text(&mut s, target, r::SAFEGUARD_PROTECT_TEXT),
        M::SafeguardFaded { user } => user_text(&mut s, user, r::BATTLE_TEXT_SAFEGUARD_FADED),
        M::CoveredByVeil { user } => user_text(&mut s, user, r::COVERED_BY_VEIL_TEXT),
        M::ProtectedByMist { target } => target_text(&mut s, target, r::PROTECTED_BY_MIST_TEXT),
        M::Mist { user } => user_text(&mut s, user, r::MIST_TEXT),
        M::ProtectedByItem { target, item } => {
            s.target(target);
            s.write(W_STRING_BUFFER1, names.items[usize::from(*item)]);
            r::PROTECTED_BY_TEXT
        }
        M::FledInFear { target } => target_text(&mut s, target, r::FLED_IN_FEAR_TEXT),
        M::BlownAway { target } => target_text(&mut s, target, r::BLOWN_AWAY_TEXT),
        M::DraggedOut { user } => user_text(&mut s, user, r::DRAGGED_OUT_TEXT),

        M::WrappedBy { target, user } => pair_text(&mut s, user, target, r::WRAPPED_BY_TEXT),
        M::ClampedBy { target, user } => pair_text(&mut s, user, target, r::CLAMPED_BY_TEXT),
        M::UsedBind { user, target } => pair_text(&mut s, user, target, r::USED_BIND_TEXT),
        M::Trapped { target } => target_text(&mut s, target, r::FIRE_SPIN_TRAP_TEXT),
        M::HurtByTrap { user, mv } => {
            s.user(user);
            s.write(W_STRING_BUFFER1, names.moves[usize::from(*mv)]);
            r::BATTLE_TEXT_USERS_HURT_BY_STRING_BUFFER1
        }
        M::ReleasedFromTrap { user, mv } => {
            s.user(user);
            s.write(W_STRING_BUFFER1, names.moves[usize::from(*mv)]);
            r::BATTLE_TEXT_USER_WAS_RELEASED_FROM_STRING_BUFFER1
        }
        M::ReleasedBy { user, target } => pair_text(&mut s, user, target, r::RELEASED_BY_TEXT),
        M::CantEscapeNow { target } => target_text(&mut s, target, r::CANT_ESCAPE_NOW_TEXT),
        M::WasDisabled { target, mv } => {
            s.target(target);
            s.write(W_STRING_BUFFER1, names.moves[usize::from(*mv)]);
            r::WAS_DISABLED_TEXT
        }
        M::DisabledMove { user, mv } => {
            s.user(user);
            s.write(W_STRING_BUFFER1, names.moves[usize::from(*mv)]);
            r::DISABLED_MOVE_TEXT
        }
        M::DisabledNoMore { user } => user_text(&mut s, user, r::DISABLED_NO_MORE_TEXT),
        M::GotEncore { target } => target_text(&mut s, target, r::GOT_AN_ENCORE_TEXT),
        M::EncoreEnded { target } => target_text(&mut s, target, r::BATTLE_TEXT_TARGETS_ENCORE_ENDED),

        M::ProtectedItself { user } => user_text(&mut s, user, r::PROTECTED_ITSELF_TEXT),
        M::ProtectingItself { target } => target_text(&mut s, target, r::PROTECTING_ITSELF_TEXT),
        M::BracedItself { user } => user_text(&mut s, user, r::BRACED_ITSELF_TEXT),
        M::LightScreen { user } => user_text(&mut s, user, r::LIGHT_SCREEN_EFFECT_TEXT),
        M::Reflect { user } => user_text(&mut s, user, r::REFLECT_EFFECT_TEXT),
        M::LightScreenFell { side } => {
            s.write(W_STRING_BUFFER1, owner(*side));
            r::BATTLE_TEXT_MONS_LIGHT_SCREEN_FELL
        }
        M::ReflectFaded { side } => {
            s.write(W_STRING_BUFFER1, owner(*side));
            r::BATTLE_TEXT_MONS_REFLECT_FADED
        }

        M::Downpour => r::DOWNPOUR_TEXT,
        M::RainContinues => r::BATTLE_TEXT_RAIN_CONTINUES_TO_FALL,
        M::RainStopped => r::BATTLE_TEXT_THE_RAIN_STOPPED,
        M::SunGotBright => r::SUN_GOT_BRIGHT_TEXT,
        M::SunlightIsStrong => r::BATTLE_TEXT_THE_SUNLIGHT_IS_STRONG,
        M::SunlightFaded => r::BATTLE_TEXT_THE_SUNLIGHT_FADED,
        M::SandstormBrewed => r::SANDSTORM_BREWED_TEXT,
        M::SandstormRages => r::BATTLE_TEXT_THE_SANDSTORM_RAGES,
        M::SandstormHits { user } => user_text(&mut s, user, r::SANDSTORM_HITS_TEXT),
        M::SandstormSubsided => r::BATTLE_TEXT_THE_SANDSTORM_SUBSIDED,
        M::Spikes { target } => target_text(&mut s, target, r::SPIKES_TEXT),
        M::HurtBySpikes { user } => user_text(&mut s, user, r::BATTLE_TEXT_USER_HURT_BY_SPIKES),
        M::BlewSpikes { user } => user_text(&mut s, user, r::BLEW_SPIKES_TEXT),

        M::ItemActivated { user, item } => {
            s.user(user);
            s.write(W_STRING_BUFFER1, names.items[usize::from(*item)]);
            r::BATTLE_TEXT_USERS_STRING_BUFFER1_ACTIVATED
        }
        M::RecoveredWithItem { target, item } => {
            s.target(target);
            s.write(W_STRING_BUFFER1, names.items[usize::from(*item)]);
            r::BATTLE_TEXT_TARGET_RECOVERED_WITH_ITEM
        }
        M::RecoveredUsing { target, item } => {
            s.target(target);
            s.write(W_STRING_BUFFER1, names.items[usize::from(*item)]);
            r::RECOVERED_USING_TEXT
        }
        M::RecoveredPpUsing { user, item } => {
            s.user(user);
            s.write(W_STRING_BUFFER1, names.items[usize::from(*item)]);
            r::BATTLE_TEXT_USER_RECOVERED_PP_USING
        }
        M::ItemHealedConfusion { item, target } => {
            s.target(target);
            s.write(W_STRING_BUFFER1, names.items[usize::from(*item)]);
            r::BATTLE_TEXT_ITEM_HEALED_CONFUSION
        }

        M::PlayerUsedItem { player, item } => {
            s.write(W_PLAYER_NAME, player);
            s.write(W_STRING_BUFFER2, names.items[usize::from(*item)]);
            r::ITEM_USED_TEXT
        }
        M::EnemyUsedItem { trainer, item, mon } => {
            s.trainer(trainer, data);
            s.write(W_MON_OR_ITEM_NAME_BUFFER, names.items[usize::from(*item)]);
            s.write(W_ENEMY_MON_NICKNAME, mon);
            r::ENEMY_USED_ON_TEXT
        }
        M::ItemsCantBeUsedHere => r::BATTLE_TEXT_ITEMS_CANT_BE_USED_HERE,
        M::OakWarning { player } => {
            s.write(W_PLAYER_NAME, player);
            r::ITEM_OAK_WARNING_TEXT
        }
        M::WontHaveEffect => r::ITEM_WONT_HAVE_EFFECT_TEXT,
        M::CantUseOnEgg => r::ITEM_CANT_USE_ON_EGG_TEXT,
        M::LooksBitter => r::ITEM_LOOKS_BITTER_TEXT,
        M::RestoreWhichMove => r::RESTORE_THE_PP_OF_WHICH_MOVE_TEXT,
        M::PpRestored => r::PP_RESTORED_TEXT,
        M::PartyRecoveredHp { mon, hp } => {
            s.write(W_STRING_BUFFER1, mon);
            s.number(W_CUR_HP_ANIM_DELTA_HP, 2, u32::from(*hp));
            r::PRINT_PARTY_MENU_ACTION_TEXT_RECOVERED_SOME_HP_TEXT
        }
        M::PartyCuredOfPoison { mon } => party_text(&mut s, mon, r::PRINT_PARTY_MENU_ACTION_TEXT_CURED_OF_POISON_TEXT),
        M::PartyBurnHealed { mon } => party_text(&mut s, mon, r::PRINT_PARTY_MENU_ACTION_TEXT_BURN_WAS_HEALED_TEXT),
        M::PartyDefrosted { mon } => party_text(&mut s, mon, r::PRINT_PARTY_MENU_ACTION_TEXT_WAS_DEFROSTED_TEXT),
        M::PartyWokeUp { mon } => party_text(&mut s, mon, r::PRINT_PARTY_MENU_ACTION_TEXT_WOKE_UP_TEXT),
        M::PartyRidOfParalysis { mon } => party_text(&mut s, mon, r::PRINT_PARTY_MENU_ACTION_TEXT_RID_OF_PARALYSIS_TEXT),
        M::PartyHealthReturned { mon } => party_text(&mut s, mon, r::PRINT_PARTY_MENU_ACTION_TEXT_HEALTH_RETURNED_TEXT),
        M::PartyRevitalized { mon } => party_text(&mut s, mon, r::PRINT_PARTY_MENU_ACTION_TEXT_REVITALIZED_TEXT),
        M::PartyCameToSenses { mon } => party_text(&mut s, mon, r::PRINT_PARTY_MENU_ACTION_TEXT_CAME_TO_ITS_SENSES_TEXT),
        M::PartyGrewToLevel { mon, level } => {
            s.write(W_STRING_BUFFER1, mon);
            s.byte(W_CUR_PARTY_LEVEL, *level);
            r::PRINT_PARTY_MENU_ACTION_TEXT_GREW_TO_LEVEL_TEXT
        }

        M::BallBlocked => r::BALL_BLOCKED_TEXT,
        M::BallDontBeAThief => r::BALL_DONT_BE_A_THIEF_TEXT,
        M::BallBrokeFree => r::BALL_BROKE_FREE_TEXT,
        M::BallAppearedCaught => r::BALL_APPEARED_CAUGHT_TEXT,
        M::BallAlmostHadIt => r::BALL_ALMOST_HAD_IT_TEXT,
        M::BallSoClose => r::BALL_SO_CLOSE_TEXT,
        M::Caught { mon } => {
            s.write(W_ENEMY_MON_NICKNAME, mon);
            r::TEXT_GOTCHA_MON_WAS_CAUGHT
        }
        M::NewDexData { mon } => {
            s.write(W_ENEMY_MON_NICKNAME, mon);
            r::NEW_DEX_DATA_TEXT
        }
    };
    super::text::render_text(&mut s.engine, label, &choices)
}

fn user_text(s: &mut Scene, user: &Mon, label: usize) -> usize {
    s.user(user);
    label
}

fn target_text(s: &mut Scene, target: &Mon, label: usize) -> usize {
    s.target(target);
    label
}

/// A text naming both `<USER>` and `<TARGET>`.
fn pair_text(s: &mut Scene, user: &Mon, target: &Mon, label: usize) -> usize {
    s.user(user);
    s.side_name(target);
    label
}

fn party_text(s: &mut Scene, mon: &str, label: usize) -> usize {
    s.write(W_STRING_BUFFER1, mon);
    label
}

fn stat_name(data: &GameData, stat: u8) -> &'static str {
    data.names.stats[usize::from(stat.wrapping_sub(1)) % data.names.stats.len()]
}

fn owner(side: Side) -> &'static str {
    match side {
        Side::Player => "Your",
        Side::Enemy => "Enemy",
    }
}

/// What a battle's events show, as the harness captures the original's:
/// each message rendered, the HP bars as they are.
pub fn shown(events: &[Event], ctx: Context) -> Vec<Shown> {
    let mut out = Vec::new();
    for event in events {
        match event {
            Event::Message(message) => out.extend(render(message, ctx).into_iter().map(Shown::Text)),
            Event::HpBar { target, from, to, max } => {
                out.push(Shown::HpBar { target: *target, from: *from, to: *to, max: *max })
            }
            Event::Animation { anim, user, param } => {
                out.push(Shown::Animation { anim: *anim, turn: u8::from(*user == Side::Enemy), param: *param })
            }
            Event::Sound(sfx) => out.push(Shown::Sound { sfx: *sfx }),
            Event::Cry(species) => out.push(Shown::Cry { species: *species }),
            Event::Faint(side) => out.push(Shown::Faint { side: u8::from(*side == Side::Enemy) }),
            Event::SlideOut(side) => out.push(Shown::SlideOut { side: u8::from(*side == Side::Enemy) }),
            Event::TrainerSlidesIn => out.push(Shown::TrainerSlidesIn),
            Event::ExpBar { slot, gained } => out.push(Shown::ExpBar { slot: *slot, gained: *gained }),
            Event::StatsBox { slot } => out.push(Shown::StatsBox { slot: *slot }),
            Event::Frontpic(species) => out.push(Shown::Frontpic { species: *species }),
            Event::BallWobbles { wobbles, caught } => out.push(Shown::BallWobbles { wobbles: *wobbles, caught: *caught }),
        }
    }
    out
}

/// The text of rendered messages only.
pub fn texts(messages: &[Message], ctx: Context) -> Vec<String> {
    messages.iter().flat_map(|m| render(m, ctx)).collect()
}
