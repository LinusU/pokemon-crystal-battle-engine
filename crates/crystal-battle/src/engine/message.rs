//! Battle texts as [`Message`]s: what each text the battle code prints
//! (`StdBattleTextbox` / `PrintText` with a text label) says, read from the
//! state the text itself would read (the turn for `<USER>` / `<TARGET>`, the
//! name buffers for `text_ram`, the numbers for `text_decimal`, and the
//! choices its `text_asm` makes).

use super::battle::Engine;
use super::names::Name;
use super::rsym as r;
use super::sym::*;
use crate::battle::message::{Message, Mon, SendOut, Trainer, Withdraw};
use crate::battle::Side;
use crate::defs::{RIVAL1, RIVAL2};

/// Crystal Legacy's `AlreadyBurnedText`, which Crystal's ROM lacks: a label
/// past its end.
pub const ALREADY_BURNED_TEXT: usize = 0x20_0000;
/// Crystal Legacy's `AlreadyFrozenText`.
pub const ALREADY_FROZEN_TEXT: usize = 0x20_0001;

impl Engine {
    /// The mon a name buffer names, as the battle shows it.
    fn mon_name(&self, name: &Name) -> String {
        match name {
            Name::Nickname(s) => s.to_string(),
            Name::Species(id) => self.data.names.species[usize::from(*id)].to_string(),
            // Beat Up past the party: garbage the engine doesn't reproduce.
            Name::Empty => String::new(),
            other => panic!("engine: a mon's name expected, the buffer holds {other:?}"),
        }
    }

    fn side_mon(&self, side: Side) -> Mon {
        let name = match side {
            Side::Player => &self.names.battle_mon,
            Side::Enemy => &self.names.enemy_mon,
        };
        Mon { side, name: self.mon_name(name) }
    }

    /// `<USER>`.
    fn user(&self) -> Mon {
        self.side_mon(if self.turn() == 0 { Side::Player } else { Side::Enemy })
    }

    /// `<TARGET>`.
    fn target(&self) -> Mon {
        self.side_mon(if self.turn() == 0 { Side::Enemy } else { Side::Player })
    }

    /// `<ENEMY>`.
    fn trainer(&self) -> Trainer {
        let class = self.mem.rb(W_TRAINER_CLASS);
        let name = if self.link_mode() != 0 {
            match &self.names.ot_class {
                Name::Nickname(s) => s.to_string(),
                other => panic!("engine: a link trainer's name expected, wOTClassName holds {other:?}"),
            }
        } else if class == RIVAL1 || class == RIVAL2 {
            self.names.rival.to_string()
        } else {
            self.trainer_name()
        };
        Trainer { class, name }
    }

    fn battle_mon(&self) -> String {
        self.mon_name(&self.names.battle_mon)
    }

    fn enemy_mon(&self) -> String {
        self.mon_name(&self.names.enemy_mon)
    }

    fn item_in(&self, name: &Name) -> u8 {
        match name {
            Name::Item(id) => *id,
            other => panic!("engine: an item's name expected, the buffer holds {other:?}"),
        }
    }

    fn move_in(&self, name: &Name) -> u8 {
        match name {
            Name::Move(id) => *id,
            other => panic!("engine: a move's name expected, the buffer holds {other:?}"),
        }
    }

    fn stat_in(&self, name: &Name) -> u8 {
        match name {
            Name::Stat(id) => *id,
            other => panic!("engine: a stat's name expected, the buffer holds {other:?}"),
        }
    }

    /// `text_decimal` of `bytes` bytes at `addr` (big-endian, as printed).
    fn decimal(&self, addr: u16, bytes: u16) -> u32 {
        (0..bytes).fold(0, |v, i| v << 8 | u32::from(self.mem.rb(addr + i)))
    }

    /// The message the text at `label` prints now (`choices`: the
    /// `text_asm` choices the battle code made, as for [`Engine::text_with`]).
    pub fn message(&self, label: usize, choices: &[usize]) -> Message {
        let n = &self.names;
        let sb1 = || &n.string_buffer1;
        let sb2 = || &n.string_buffer2;
        use Message as M;
        match label {
            // --- start, end, send-outs
            r::WILD_POKEMON_APPEARED_TEXT | r::WILD_CELEBI_APPEARED_TEXT => M::WildAppeared { mon: self.enemy_mon() },
            r::WANTS_TO_BATTLE_TEXT => M::WantsToBattle { trainer: self.trainer() },
            r::BATTLE_TEXT_ENEMY_SENT_OUT => M::EnemySentOut { trainer: self.trainer(), mon: self.enemy_mon() },
            r::GO_MON_TEXT => M::PlayerSentOut { mon: self.battle_mon(), how: SendOut::Go },
            r::DO_IT_MON_TEXT => M::PlayerSentOut { mon: self.battle_mon(), how: SendOut::DoIt },
            r::GO_FOR_IT_MON_TEXT => M::PlayerSentOut { mon: self.battle_mon(), how: SendOut::GoForIt },
            r::YOUR_FOES_WEAK_GETM_MON_TEXT => M::PlayerSentOut { mon: self.battle_mon(), how: SendOut::FoesWeak },
            r::WITHDRAW_MON_TEXT_WITHDRAW_MON_TEXT => {
                let how = match choices.first().copied() {
                    Some(r::THATS_ENOUGH_COME_BACK_TEXT) => Withdraw::ThatsEnough,
                    Some(r::COME_BACK_TEXT) => Withdraw::ComeBack,
                    Some(r::OK_COME_BACK_TEXT) => Withdraw::Ok,
                    Some(r::GOOD_COME_BACK_TEXT) => Withdraw::Good,
                    other => panic!("engine: WithdrawMonText with choice {other:?}"),
                };
                M::Withdraw { mon: self.battle_mon(), how }
            }
            r::ENEMY_WITHDREW_TEXT => M::EnemyWithdrew { trainer: self.trainer(), mon: self.enemy_mon() },
            r::HOOKED_POKEMON_ATTACKED_TEXT => M::HookedMonAttacked { mon: self.enemy_mon() },
            r::POKEMON_FELL_FROM_TREE_TEXT => M::FellFromTree { mon: self.enemy_mon() },
            r::BATTLE_TEXT_ENEMY_WAS_DEFEATED => M::EnemyWasDefeated { trainer: self.trainer() },
            r::LOST_AGAINST_TEXT => M::LostAgainst { trainer: self.trainer() },
            r::TIED_AGAINST_TEXT => M::TiedAgainst { trainer: self.trainer() },
            r::GOT_MONEY_FOR_WINNING_TEXT => M::GotMoney { player: n.player.to_string(), money: self.decimal(W_BATTLE_REWARD, 3) },
            r::SENT_SOME_TO_MOM_TEXT => {
                M::GotMoneySentSomeToMom { player: n.player.to_string(), money: self.decimal(W_BATTLE_REWARD, 3) }
            }
            r::SENT_HALF_TO_MOM_TEXT => M::SentHalfToMom,
            r::SENT_ALL_TO_MOM_TEXT => M::SentAllToMom,
            r::BATTLE_TEXT_GOT_AWAY_SAFELY => M::GotAwaySafely,
            r::BATTLE_TEXT_CANT_ESCAPE | r::BATTLE_TEXT_CANT_ESCAPE2 => M::CantEscape,
            r::BATTLE_TEXT_THERES_NO_ESCAPE_FROM_TRAINER_BATTLE => M::NoRunningFromTrainer,
            r::BATTLE_TEXT_WILD_FLED => M::WildFled { mon: self.enemy_mon() },
            r::BATTLE_TEXT_ENEMY_FLED => M::EnemyFled { mon: self.enemy_mon() },
            r::BATTLE_TEXT_USER_FLED_USING_A_STRING_BUFFER1 => M::FledUsingItem { user: self.user(), item: self.item_in(sb1()) },
            r::FLED_FROM_BATTLE_TEXT => M::FledFromBattle { user: self.user() },
            r::BATTLE_TEXT_MON_FAINTED => M::MonFainted { mon: self.battle_mon() },
            r::BATTLE_TEXT_ENEMY_MON_FAINTED => M::EnemyMonFainted { mon: self.enemy_mon() },
            r::BATTLE_TEXT_MON_HAS_NO_MOVES_LEFT => M::NoMovesLeft { mon: self.battle_mon() },
            r::BATTLE_TEXT_MON_IS_ALREADY_OUT => M::AlreadyOut { mon: self.battle_mon() },
            r::BATTLE_TEXT_AN_EGG_CANT_BATTLE => M::EggCantBattle,
            r::BATTLE_TEXT_THERES_NO_WILL_TO_BATTLE => M::NoWillToBattle,

            // --- experience
            r::TEXT_MON_GAINED_EXP_POINT => M::GainedExp {
                mon: self.mon_name(sb1()),
                exp: self.decimal(W_STRING_BUFFER2, 2) as u16,
                boosted: self.mem.rb(W_STRING_BUFFER2 + 2) != 0,
            },
            r::BATTLE_TEXT_STRING_BUFFER1_GREW_TO_LEVEL => {
                M::GrewToLevel { mon: self.mon_name(sb1()), level: self.mem.rb(W_CUR_PARTY_LEVEL) }
            }
            r::LEARNED_MOVE_TEXT => M::LearnedMove { mon: self.mon_name(&n.mon_or_item), mv: self.move_in(sb2()) },
            r::ASK_FORGET_MOVE_TEXT => M::AskForgetMove { mon: self.mon_name(&n.mon_or_item), mv: self.move_in(sb2()) },
            r::MOVE_ASK_FORGET_TEXT => M::WhichMoveToForget,
            r::TEXT_1_2_AND_POOF => M::ForgotMove { mon: self.mon_name(&n.mon_or_item), mv: self.move_in(sb1()) },

            // --- using a move
            r::USED_MOVE_TEXT => M::UsedMove {
                user: self.user(),
                mv: self.move_in(sb2()),
                instead: self.mem.rb(W_ALREADY_DISOBEYED) != 0,
            },
            r::WONT_OBEY_TEXT => M::WontObey { mon: self.battle_mon() },
            r::IGNORED_ORDERS_TEXT => M::IgnoredOrders { mon: self.battle_mon() },
            r::IGNORED_ORDERS2_TEXT => M::IgnoredOrdersUser { user: self.user() },
            r::IGNORED_SLEEPING_TEXT => M::IgnoredOrdersSleeping { mon: self.battle_mon() },
            r::LOAFING_AROUND_TEXT => M::LoafingAround { mon: self.battle_mon() },
            r::TURNED_AWAY_TEXT => M::TurnedAway { mon: self.battle_mon() },
            r::BEGAN_TO_NAP_TEXT => M::BeganToNap { mon: self.battle_mon() },
            r::HAS_NO_PP_LEFT_TEXT => M::NoPpLeftFor { user: self.user(), mv: self.move_in(sb2()) },
            r::NO_PP_LEFT_TEXT => M::NoPpLeft,
            r::BATTLE_COMMAND_CHARGE_USED_TEXT => {
                M::Charging { user: self.user(), mv: self.battle_var(crate::defs::BATTLE_VARS_MOVE_ANIM) }
            }
            r::MUST_RECHARGE_TEXT => M::MustRecharge { user: self.user() },
            r::STORING_ENERGY_TEXT => M::StoringEnergy { user: self.user() },
            r::UNLEASHED_ENERGY_TEXT => M::UnleashedEnergy { user: self.user() },
            r::CRASHED_TEXT => M::Crashed { user: self.user() },
            r::FLINCHED_TEXT => M::Flinched { user: self.user() },
            r::MAGNITUDE_TEXT => M::Magnitude { magnitude: self.mem.rb(W_TEXT_DECIMAL_BYTE) },
            r::ENEMY_HIT_TIMES_TEXT => M::HitTimes { times: self.mem.rb(W_ENEMY_DAMAGE_TAKEN) },
            r::PLAYER_HIT_TIMES_TEXT => M::HitTimes { times: self.mem.rb(W_PLAYER_DAMAGE_TAKEN) },
            r::BEAT_UP_ATTACK_TEXT => M::BeatUpAttack { mon: self.mon_name(sb1()) },

            // --- move results
            r::ATTACK_MISSED_TEXT | r::ATTACK_MISSED2_TEXT => M::AttackMissed { user: self.user() },
            r::EVADED_TEXT => M::Evaded { target: self.target() },
            r::CRITICAL_HIT_TEXT => M::CriticalHit,
            r::SUPER_EFFECTIVE_TEXT => M::SuperEffective,
            r::NOT_VERY_EFFECTIVE_TEXT => M::NotVeryEffective,
            r::DIDNT_AFFECT1_TEXT | r::DIDNT_AFFECT2_TEXT => M::DidntAffect { target: self.target() },
            r::DOESNT_AFFECT_TEXT => M::DoesntAffect { target: self.target() },
            r::UNAFFECTED_TEXT => M::Unaffected { target: self.target() },
            r::ONE_HIT_KO_TEXT => M::OneHitKo,
            r::BUT_IT_FAILED_TEXT => M::ButItFailed,
            r::IT_FAILED_TEXT => M::ItFailed,
            r::NOTHING_HAPPENED_TEXT => M::NothingHappened,
            r::ENDURED_TEXT => M::Endured { target: self.target() },
            r::HUNG_ON_TEXT => M::HungOn { target: self.target(), item: self.item_in(sb1()) },
            r::RECOIL_TEXT => M::Recoil { user: self.user() },
            r::SUCKED_HEALTH_TEXT => M::SuckedHealth { target: self.target() },
            r::DREAM_EATEN_TEXT => M::DreamEaten { target: self.target() },
            r::REGAINED_HEALTH_TEXT => M::RegainedHealth { user: self.user() },
            r::HP_IS_FULL_TEXT => M::HpIsFull { user: self.user() },
            r::SHARED_PAIN_TEXT => M::SharedPain,
            r::PRESENT_FAILED_TEXT => M::RefusedGift { target: self.target() },
            r::BATTLE_TEXT_TARGET_WAS_HIT_BY_FUTURE_SIGHT => M::HitByFutureSight { target: self.target() },
            r::FORESAW_ATTACK_TEXT => M::ForesawAttack { user: self.user() },
            r::TOOK_AIM_TEXT => M::TookAim { user: self.user() },
            r::IDENTIFIED_TEXT => M::Identified { user: self.user(), target: self.target() },
            r::TOOK_DOWN_WITH_IT_TEXT => M::TookDownWithIt { target: self.target(), user: self.user() },
            r::DESTINY_BOND_EFFECT_TEXT => M::DestinyBond { user: self.user() },
            r::MIRROR_MOVE_FAILED_TEXT => M::MirrorMoveFailed,
            r::MIMIC_LEARNED_MOVE_TEXT => M::MimicLearned { user: self.user(), mv: self.move_in(sb1()) },
            r::SKETCHED_TEXT => M::Sketched { user: self.user(), mv: self.move_in(sb1()) },
            r::TRANSFORMED_TEXT => M::Transformed {
                user: self.user(),
                species: match sb1() {
                    Name::Species(id) => *id,
                    other => panic!("engine: TRANSFORMED_TEXT with {other:?}"),
                },
            },
            r::TRANSFORMED_TYPE_TEXT => M::TransformedType {
                user: self.user(),
                kind: match sb1() {
                    Name::Type(id) => *id,
                    other => panic!("engine: TRANSFORMED_TYPE_TEXT with {other:?}"),
                },
            },
            r::SPITE_EFFECT_TEXT => {
                M::Spite { target: self.target(), mv: self.move_in(sb1()), amount: self.mem.rb(W_TEXT_DECIMAL_BYTE) }
            }
            r::STOLE_TEXT => M::Stole { user: self.user(), item: self.item_in(sb1()) },
            r::COINS_SCATTERED_TEXT => M::CoinsScattered,
            r::BELL_CHIMED_TEXT => M::BellChimed,
            r::BELLY_DRUM_TEXT => M::BellyDrum { user: self.user() },
            r::COPIED_STATS_TEXT => M::CopiedStats { user: self.user(), target: self.target() },
            r::ELIMINATED_STATS_TEXT => M::StatsEliminated,
            r::START_PERISH_TEXT => M::StartPerish,
            r::PERISH_COUNT_TEXT => M::PerishCount { user: self.user(), count: self.mem.rb(W_TEXT_DECIMAL_BYTE) },
            r::PUT_A_CURSE_TEXT => M::PutCurse { user: self.user(), target: self.target() },
            r::TOO_WEAK_SUB_TEXT => M::TooWeakForSubstitute,
            r::MADE_SUBSTITUTE_TEXT => M::MadeSubstitute { user: self.user() },
            r::HAS_SUBSTITUTE_TEXT => M::HasSubstitute { user: self.user() },
            r::SUB_TOOK_DAMAGE_TEXT => M::SubstituteTookDamage { target: self.target() },
            r::SUB_FADED_TEXT => M::SubstituteFaded { target: self.target() },

            // --- stats
            r::BATTLE_COMMAND_STAT_UP_MESSAGE_STAT => M::StatRose {
                user: self.user(),
                stat: self.stat_in(sb2()),
                sharply: self.mem.rb(W_LOWERED_STAT) & 0xf0 != 0,
            },
            r::BATTLE_COMMAND_STAT_DOWN_MESSAGE_STAT => M::StatFell {
                target: self.target(),
                stat: self.stat_in(sb2()),
                sharply: self.mem.rb(W_LOWERED_STAT) & 0xf0 != 0,
            },
            r::WONT_RISE_ANYMORE_TEXT => M::WontRiseAnymore { user: self.user(), stat: self.stat_in(sb2()) },
            r::WONT_DROP_ANYMORE_TEXT => M::WontDropAnymore { target: self.target(), stat: self.stat_in(sb2()) },
            r::GETTING_PUMPED_TEXT => M::GettingPumped { user: self.user() },
            r::RAGE_BUILDING_TEXT => M::RageBuilding { user: self.user() },

            // --- status
            r::FELL_ASLEEP_TEXT => M::FellAsleep { target: self.target() },
            r::ALREADY_ASLEEP_TEXT => M::AlreadyAsleep { target: self.target() },
            r::FAST_ASLEEP_TEXT => M::FastAsleep { user: self.user() },
            r::WOKE_UP_TEXT => M::WokeUp { user: self.user() },
            r::RESTED_TEXT => M::Rested { user: self.user() },
            r::WENT_TO_SLEEP_TEXT => M::WentToSleep { user: self.user() },
            r::WAS_POISONED_TEXT => M::WasPoisoned { target: self.target() },
            r::BADLY_POISONED_TEXT => M::BadlyPoisoned { target: self.target() },
            r::ALREADY_POISONED_TEXT => M::AlreadyPoisoned { target: self.target() },
            r::HURT_BY_POISON_TEXT => M::HurtByPoison { user: self.user() },
            r::WAS_BURNED_TEXT => M::WasBurned { target: self.target() },
            ALREADY_BURNED_TEXT => M::AlreadyBurned { target: self.target() },
            ALREADY_FROZEN_TEXT => M::AlreadyFrozen { target: self.target() },
            r::HURT_BY_BURN_TEXT => M::HurtByBurn { user: self.user() },
            r::WAS_FROZEN_TEXT => M::WasFrozen { target: self.target() },
            r::FROZEN_SOLID_TEXT => M::FrozenSolid { user: self.user() },
            r::WAS_DEFROSTED_TEXT => M::WasDefrosted { user: self.user() },
            r::DEFROSTED_OPPONENT_TEXT => M::DefrostedOpponent { target: self.target() },
            r::PARALYZED_TEXT => M::Paralyzed { target: self.target() },
            r::ALREADY_PARALYZED_TEXT => M::AlreadyParalyzed { target: self.target() },
            r::FULLY_PARALYZED_TEXT => M::FullyParalyzed { user: self.user() },
            r::BECAME_CONFUSED_TEXT => M::BecameConfused { target: self.target() },
            r::ALREADY_CONFUSED_TEXT => M::AlreadyConfused { target: self.target() },
            r::IS_CONFUSED_TEXT => M::IsConfused { user: self.user() },
            r::HURT_ITSELF_TEXT => M::HurtItself,
            r::CONFUSED_NO_MORE_TEXT => M::ConfusedNoMore { user: self.user() },
            r::FELL_IN_LOVE_TEXT => M::FellInLove { target: self.target() },
            r::IN_LOVE_WITH_TEXT => M::InLoveWith { user: self.user(), target: self.target() },
            r::INFATUATION_TEXT => M::Infatuation { user: self.user() },
            r::STARTED_NIGHTMARE_TEXT => M::StartedNightmare { target: self.target() },
            r::HAS_A_NIGHTMARE_TEXT => M::HasNightmare { user: self.user() },
            r::HURT_BY_CURSE_TEXT => M::HurtByCurse { user: self.user() },
            r::WAS_SEEDED_TEXT => M::WasSeeded { target: self.target() },
            r::LEECH_SEED_SAPS_TEXT => M::LeechSeedSaps { user: self.user() },
            r::SHED_LEECH_SEED_TEXT => M::ShedLeechSeed { user: self.user() },
            r::SAFEGUARD_PROTECT_TEXT => M::SafeguardProtect { target: self.target() },
            r::BATTLE_TEXT_SAFEGUARD_FADED => M::SafeguardFaded { user: self.user() },
            r::COVERED_BY_VEIL_TEXT => M::CoveredByVeil { user: self.user() },
            r::PROTECTED_BY_MIST_TEXT => M::ProtectedByMist { target: self.target() },
            r::MIST_TEXT => M::Mist { user: self.user() },
            r::PROTECTED_BY_TEXT => M::ProtectedByItem { target: self.target(), item: self.item_in(sb1()) },
            r::FLED_IN_FEAR_TEXT => M::FledInFear { target: self.target() },
            r::BLOWN_AWAY_TEXT => M::BlownAway { target: self.target() },
            r::DRAGGED_OUT_TEXT => M::DraggedOut { user: self.user() },

            // --- trapping, disabling, encore
            r::WRAPPED_BY_TEXT => M::WrappedBy { target: self.target(), user: self.user() },
            r::CLAMPED_BY_TEXT => M::ClampedBy { target: self.target(), user: self.user() },
            r::USED_BIND_TEXT => M::UsedBind { user: self.user(), target: self.target() },
            r::FIRE_SPIN_TRAP_TEXT | r::WHIRLPOOL_TRAP_TEXT => M::Trapped { target: self.target() },
            r::BATTLE_TEXT_USERS_HURT_BY_STRING_BUFFER1 => M::HurtByTrap { user: self.user(), mv: self.move_in(sb1()) },
            r::BATTLE_TEXT_USER_WAS_RELEASED_FROM_STRING_BUFFER1 => {
                M::ReleasedFromTrap { user: self.user(), mv: self.move_in(sb1()) }
            }
            r::RELEASED_BY_TEXT => M::ReleasedBy { user: self.user(), target: self.target() },
            r::CANT_ESCAPE_NOW_TEXT => M::CantEscapeNow { target: self.target() },
            r::WAS_DISABLED_TEXT => M::WasDisabled { target: self.target(), mv: self.move_in(sb1()) },
            r::DISABLED_MOVE_TEXT => M::DisabledMove { user: self.user(), mv: self.move_in(sb1()) },
            r::DISABLED_NO_MORE_TEXT => M::DisabledNoMore { user: self.user() },
            r::GOT_AN_ENCORE_TEXT => M::GotEncore { target: self.target() },
            r::BATTLE_TEXT_TARGETS_ENCORE_ENDED => M::EncoreEnded { target: self.target() },

            // --- protection, screens
            r::PROTECTED_ITSELF_TEXT => M::ProtectedItself { user: self.user() },
            r::PROTECTING_ITSELF_TEXT => M::ProtectingItself { target: self.target() },
            r::BRACED_ITSELF_TEXT => M::BracedItself { user: self.user() },
            r::LIGHT_SCREEN_EFFECT_TEXT => M::LightScreen { user: self.user() },
            r::REFLECT_EFFECT_TEXT => M::Reflect { user: self.user() },
            r::BATTLE_TEXT_MONS_LIGHT_SCREEN_FELL => M::LightScreenFell { side: self.owner_in(sb1()) },
            r::BATTLE_TEXT_MONS_REFLECT_FADED => M::ReflectFaded { side: self.owner_in(sb1()) },

            // --- weather, spikes
            r::DOWNPOUR_TEXT => M::Downpour,
            r::BATTLE_TEXT_RAIN_CONTINUES_TO_FALL => M::RainContinues,
            r::BATTLE_TEXT_THE_RAIN_STOPPED => M::RainStopped,
            r::SUN_GOT_BRIGHT_TEXT => M::SunGotBright,
            r::BATTLE_TEXT_THE_SUNLIGHT_IS_STRONG => M::SunlightIsStrong,
            r::BATTLE_TEXT_THE_SUNLIGHT_FADED => M::SunlightFaded,
            r::SANDSTORM_BREWED_TEXT => M::SandstormBrewed,
            r::BATTLE_TEXT_THE_SANDSTORM_RAGES => M::SandstormRages,
            r::SANDSTORM_HITS_TEXT => M::SandstormHits { user: self.user() },
            r::BATTLE_TEXT_THE_SANDSTORM_SUBSIDED => M::SandstormSubsided,
            r::SPIKES_TEXT => M::Spikes { target: self.target() },
            r::BATTLE_TEXT_USER_HURT_BY_SPIKES => M::HurtBySpikes { user: self.user() },
            r::BLEW_SPIKES_TEXT => M::BlewSpikes { user: self.user() },

            // --- held items
            r::BATTLE_TEXT_USERS_STRING_BUFFER1_ACTIVATED => M::ItemActivated { user: self.user(), item: self.item_in(sb1()) },
            r::BATTLE_TEXT_TARGET_RECOVERED_WITH_ITEM => {
                M::RecoveredWithItem { target: self.target(), item: self.item_in(sb1()) }
            }
            r::RECOVERED_USING_TEXT => M::RecoveredUsing { target: self.target(), item: self.item_in(sb1()) },
            r::BATTLE_TEXT_USER_RECOVERED_PP_USING => M::RecoveredPpUsing { user: self.user(), item: self.item_in(sb1()) },
            r::BATTLE_TEXT_ITEM_HEALED_CONFUSION => {
                M::ItemHealedConfusion { item: self.item_in(sb1()), target: self.target() }
            }

            // --- the bag
            r::ITEM_USED_TEXT => M::PlayerUsedItem { player: n.player.to_string(), item: self.item_in(sb2()) },
            r::ENEMY_USED_ON_TEXT => {
                M::EnemyUsedItem { trainer: self.trainer(), item: self.item_in(&n.mon_or_item), mon: self.enemy_mon() }
            }
            r::BATTLE_TEXT_ITEMS_CANT_BE_USED_HERE => M::ItemsCantBeUsedHere,
            r::ITEM_OAK_WARNING_TEXT => M::OakWarning { player: n.player.to_string() },
            r::ITEM_WONT_HAVE_EFFECT_TEXT => M::WontHaveEffect,
            r::ITEM_CANT_USE_ON_EGG_TEXT => M::CantUseOnEgg,
            r::ITEM_LOOKS_BITTER_TEXT => M::LooksBitter,
            r::RESTORE_THE_PP_OF_WHICH_MOVE_TEXT => M::RestoreWhichMove,
            r::PP_RESTORED_TEXT => M::PpRestored,
            r::PRINT_PARTY_MENU_ACTION_TEXT_RECOVERED_SOME_HP_TEXT => {
                M::PartyRecoveredHp { mon: self.mon_name(sb1()), hp: self.decimal(W_CUR_HP_ANIM_DELTA_HP, 2) as u16 }
            }
            r::PRINT_PARTY_MENU_ACTION_TEXT_CURED_OF_POISON_TEXT => M::PartyCuredOfPoison { mon: self.mon_name(sb1()) },
            r::PRINT_PARTY_MENU_ACTION_TEXT_BURN_WAS_HEALED_TEXT => M::PartyBurnHealed { mon: self.mon_name(sb1()) },
            r::PRINT_PARTY_MENU_ACTION_TEXT_WAS_DEFROSTED_TEXT => M::PartyDefrosted { mon: self.mon_name(sb1()) },
            r::PRINT_PARTY_MENU_ACTION_TEXT_WOKE_UP_TEXT => M::PartyWokeUp { mon: self.mon_name(sb1()) },
            r::PRINT_PARTY_MENU_ACTION_TEXT_RID_OF_PARALYSIS_TEXT => M::PartyRidOfParalysis { mon: self.mon_name(sb1()) },
            r::PRINT_PARTY_MENU_ACTION_TEXT_HEALTH_RETURNED_TEXT => M::PartyHealthReturned { mon: self.mon_name(sb1()) },
            r::PRINT_PARTY_MENU_ACTION_TEXT_REVITALIZED_TEXT => M::PartyRevitalized { mon: self.mon_name(sb1()) },
            r::PRINT_PARTY_MENU_ACTION_TEXT_CAME_TO_ITS_SENSES_TEXT => M::PartyCameToSenses { mon: self.mon_name(sb1()) },
            r::PRINT_PARTY_MENU_ACTION_TEXT_GREW_TO_LEVEL_TEXT => {
                M::PartyGrewToLevel { mon: self.mon_name(sb1()), level: self.mem.rb(W_CUR_PARTY_LEVEL) }
            }

            // --- catching
            r::BALL_BLOCKED_TEXT => M::BallBlocked,
            r::BALL_DONT_BE_A_THIEF_TEXT => M::BallDontBeAThief,
            r::BALL_BROKE_FREE_TEXT => M::BallBrokeFree,
            r::BALL_APPEARED_CAUGHT_TEXT => M::BallAppearedCaught,
            r::BALL_ALMOST_HAD_IT_TEXT => M::BallAlmostHadIt,
            r::BALL_SO_CLOSE_TEXT => M::BallSoClose,
            r::TEXT_GOTCHA_MON_WAS_CAUGHT => M::Caught { mon: self.enemy_mon() },
            r::NEW_DEX_DATA_TEXT => M::NewDexData { mon: self.enemy_mon() },

            _ => panic!("engine: no message for the text at {label:#08x} ({})", super::rsym_name(label)),
        }
    }

    fn owner_in(&self, name: &Name) -> Side {
        match name {
            Name::Owner(side) => *side,
            other => panic!("engine: \"Your\" / \"Enemy\" expected, the buffer holds {other:?}"),
        }
    }
}
