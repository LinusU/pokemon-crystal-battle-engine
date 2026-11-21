//! What the battle says, typed: one [`Message`] per battle text, carrying
//! what the text mentions (mons, moves, items, numbers). The library never
//! renders text; a game renders a `Message` with its own text engine. Names
//! are passed through as given (a mon's nickname, the trainer's name) or are
//! ids into the game data (`mv`, `item`, `species`, `kind`, `stat`).
//!
//! Which messages appear, and in which order, is part of the battle's exact
//! behaviour. (The harness renders them to Crystal's English and checks them
//! against the original's text.)

use super::action::Side;

/// A mon a message names: its side and its name as the battle shows it
/// (`<USER>` / `<TARGET>` in the original's texts, which the game prefixes
/// with "Enemy " or "Wild " on the enemy side).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Mon {
    pub side: Side,
    pub name: String,
}

/// The enemy trainer as `<ENEMY>` names it: class and name ("BUG CATCHER"
/// "WADE"). In a link battle `class` is 0 and `name` the other player's.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Trainer {
    pub class: u8,
    pub name: String,
}

/// How the player calls a mon back (`WithdrawMonText`), by how much HP the
/// enemy lost while it was out.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Withdraw {
    /// "That's enough! Come back!" (little damage)
    ThatsEnough,
    /// "Come back!"
    ComeBack,
    /// "OK! Come back!"
    Ok,
    /// "Good! Come back!" (a lot of damage)
    Good,
}

/// How the player sends a mon out, by the enemy's remaining HP.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SendOut {
    /// "Go! …" (the enemy is at 70% or more)
    Go,
    /// "Do it! …"
    DoIt,
    /// "Go for it, …"
    GoForIt,
    /// "Your foe's weak! Get'm, …" (below 20%)
    FoesWeak,
}

/// One battle text. Variants are named after the original's texts.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Message {
    // --- the battle's start, end and send-outs -------------------------------------
    /// "Wild … appeared!"
    WildAppeared { mon: String },
    /// "… wants to battle!"
    WantsToBattle { trainer: Trainer },
    /// "… sent out …!"
    EnemySentOut { trainer: Trainer, mon: String },
    /// "Go! …" and its variants.
    PlayerSentOut { mon: String, how: SendOut },
    /// "…, come back!" and its variants.
    Withdraw { mon: String, how: Withdraw },
    /// "… withdrew …!"
    EnemyWithdrew { trainer: Trainer, mon: String },
    /// "The hooked … attacked!"
    HookedMonAttacked { mon: String },
    /// "… fell out of the tree!"
    FellFromTree { mon: String },
    /// "… was defeated!"
    EnemyWasDefeated { trainer: Trainer },
    /// "Lost against …!"
    LostAgainst { trainer: Trainer },
    /// "Tied against …!"
    TiedAgainst { trainer: Trainer },
    /// "… got ¥… for winning!"
    GotMoney { player: String, money: u32 },
    /// "… got ¥… for winning! Sent some to MOM!"
    GotMoneySentSomeToMom { player: String, money: u32 },
    SentHalfToMom,
    SentAllToMom,
    /// "Got away safely!"
    GotAwaySafely,
    /// "Can't escape!"
    CantEscape,
    /// "No! There's no running from a trainer battle!"
    NoRunningFromTrainer,
    /// "Wild … fled!"
    WildFled { mon: String },
    /// "Enemy … fled!"
    EnemyFled { mon: String },
    /// "… fled using a …!" (a held Smoke Ball)
    FledUsingItem { user: Mon, item: u8 },
    /// "… fled from battle!" (Roar, Whirlwind, Teleport)
    FledFromBattle { user: Mon },
    /// "… fainted!" (the player's mon)
    MonFainted { mon: String },
    /// "Enemy … fainted!"
    EnemyMonFainted { mon: String },
    /// "… has no moves left!"
    NoMovesLeft { mon: String },
    /// "… is already out."
    AlreadyOut { mon: String },
    /// "An EGG can't battle!"
    EggCantBattle,
    /// "There's no will to battle!"
    NoWillToBattle,

    // --- experience ----------------------------------------------------------------
    /// "… gained … EXP. Points!" (`boosted`: "a boosted …")
    GainedExp { mon: String, exp: u16, boosted: bool },
    /// "… grew to level …!"
    GrewToLevel { mon: String, level: u8 },
    /// "… learned …!"
    LearnedMove { mon: String, mv: u8 },
    /// "… is trying to learn …. But … can't learn more than four moves. …"
    AskForgetMove { mon: String, mv: u8 },
    /// "Which move should be forgotten?"
    WhichMoveToForget,
    /// "1, 2 and… Poof! … forgot …. And…"
    ForgotMove { mon: String, mv: u8 },

    // --- using a move ------------------------------------------------------------------
    /// "… used …!" (`instead`: a disobeying mon "used … instead,")
    UsedMove { user: Mon, mv: u8, instead: bool },
    /// "… won't obey!"
    WontObey { mon: String },
    /// "… ignored orders!" (the player's mon, by name)
    IgnoredOrders { mon: String },
    /// "… ignored orders!" (`<USER>`)
    IgnoredOrdersUser { user: Mon },
    /// "… ignored orders…sleeping!"
    IgnoredOrdersSleeping { mon: String },
    /// "… is loafing around."
    LoafingAround { mon: String },
    /// "… turned away!"
    TurnedAway { mon: String },
    /// "… began to nap!"
    BeganToNap { mon: String },
    /// "… has no PP left for …!"
    NoPpLeftFor { user: Mon, mv: u8 },
    /// "But no PP is left for the move!"
    NoPpLeft,
    /// "… dug a hole!" etc. (the turn a two-turn move charges)
    Charging { user: Mon, mv: u8 },
    /// "… must recharge!"
    MustRecharge { user: Mon },
    /// "… is storing energy!"
    StoringEnergy { user: Mon },
    /// "… unleashed energy!"
    UnleashedEnergy { user: Mon },
    /// "… kept going and crashed!"
    Crashed { user: Mon },
    /// "… flinched!"
    Flinched { user: Mon },
    /// "Magnitude …!"
    Magnitude { magnitude: u8 },
    /// "Hit … times!"
    HitTimes { times: u8 },
    /// "…'s attack!" (Beat Up, per party mon)
    BeatUpAttack { mon: String },

    // --- move results ------------------------------------------------------------------
    /// "…'s attack missed!"
    AttackMissed { user: Mon },
    /// "… evaded the attack!"
    Evaded { target: Mon },
    /// "A critical hit!"
    CriticalHit,
    /// "It's super-effective!"
    SuperEffective,
    /// "It's not very effective…"
    NotVeryEffective,
    /// "It didn't affect …!"
    DidntAffect { target: Mon },
    /// "It doesn't affect …!"
    DoesntAffect { target: Mon },
    /// "…'s unaffected!"
    Unaffected { target: Mon },
    /// "It's a one-hit KO!"
    OneHitKo,
    /// "But it failed!"
    ButItFailed,
    /// "It failed!"
    ItFailed,
    /// "But nothing happened."
    NothingHappened,
    /// "… ENDURED the hit!"
    Endured { target: Mon },
    /// "… hung on with …!"
    HungOn { target: Mon, item: u8 },
    /// "…'s hit with recoil!"
    Recoil { user: Mon },
    /// "Sucked health from …!"
    SuckedHealth { target: Mon },
    /// "…'s dream was eaten!"
    DreamEaten { target: Mon },
    /// "… regained health!"
    RegainedHealth { user: Mon },
    /// "…'s HP is full!"
    HpIsFull { user: Mon },
    /// "The battlers shared pain!"
    SharedPain,
    /// "… refused the gift!"
    RefusedGift { target: Mon },
    /// "… was hit by FUTURE SIGHT!"
    HitByFutureSight { target: Mon },
    /// "… foresaw an attack!"
    ForesawAttack { user: Mon },
    /// "… took aim!"
    TookAim { user: Mon },
    /// "… identified …!"
    Identified { user: Mon, target: Mon },
    /// "… took down with it, …!"
    TookDownWithIt { target: Mon, user: Mon },
    /// "…'s trying to take its opponent with it!"
    DestinyBond { user: Mon },
    /// "The MIRROR MOVE failed!"
    MirrorMoveFailed,
    /// "… learned …!" (Mimic)
    MimicLearned { user: Mon, mv: u8 },
    /// "… SKETCHED …!"
    Sketched { user: Mon, mv: u8 },
    /// "… TRANSFORMED into …!"
    Transformed { user: Mon, species: u8 },
    /// "… transformed into the …-type!" (Conversion)
    TransformedType { user: Mon, kind: u8 },
    /// "…'s … was reduced by …!" (Spite)
    Spite { target: Mon, mv: u8, amount: u8 },
    /// "… stole … from its foe!" (Thief)
    Stole { user: Mon, item: u8 },
    /// "Coins scattered everywhere!"
    CoinsScattered,
    /// "A bell chimed!"
    BellChimed,
    /// "… cut its HP and maximized ATTACK!"
    BellyDrum { user: Mon },
    /// "… copied the stat changes of …!"
    CopiedStats { user: Mon, target: Mon },
    /// "All stat changes were eliminated!"
    StatsEliminated,
    /// "Both #MON will faint in 3 turns!"
    StartPerish,
    /// "…'s PERISH count is …!"
    PerishCount { user: Mon, count: u8 },
    /// "… cut its own HP and put a CURSE on …!"
    PutCurse { user: Mon, target: Mon },
    /// "Too weak to make a SUBSTITUTE!"
    TooWeakForSubstitute,
    /// "… made a SUBSTITUTE!"
    MadeSubstitute { user: Mon },
    /// "… has a SUBSTITUTE!"
    HasSubstitute { user: Mon },
    /// "The SUBSTITUTE took damage for …!"
    SubstituteTookDamage { target: Mon },
    /// "…'s SUBSTITUTE faded!"
    SubstituteFaded { target: Mon },

    // --- stats ----------------------------------------------------------------------------
    /// "…'s … went up!" / "went way up!"
    StatRose { user: Mon, stat: u8, sharply: bool },
    /// "…'s … fell!" / "sharply fell!"
    StatFell { target: Mon, stat: u8, sharply: bool },
    /// "…'s … won't rise anymore!"
    WontRiseAnymore { user: Mon, stat: u8 },
    /// "…'s … won't drop anymore!"
    WontDropAnymore { target: Mon, stat: u8 },
    /// "…'s getting pumped!" (Focus Energy)
    GettingPumped { user: Mon },
    /// "…'s RAGE is building!"
    RageBuilding { user: Mon },

    // --- status ----------------------------------------------------------------------------
    /// "… fell asleep!"
    FellAsleep { target: Mon },
    /// "…'s already asleep!"
    AlreadyAsleep { target: Mon },
    /// "… is fast asleep!"
    FastAsleep { user: Mon },
    /// "… woke up!"
    WokeUp { user: Mon },
    /// "… fell asleep and became healthy!" (Rest)
    Rested { user: Mon },
    /// "… went to sleep!" (Rest)
    WentToSleep { user: Mon },
    /// "… was poisoned!"
    WasPoisoned { target: Mon },
    /// "…'s badly poisoned!"
    BadlyPoisoned { target: Mon },
    /// "…'s already poisoned!"
    AlreadyPoisoned { target: Mon },
    /// "… is hurt by poison!"
    HurtByPoison { user: Mon },
    /// "… was burned!"
    WasBurned { target: Mon },
    /// "…'s already burned!" (Crystal Legacy)
    AlreadyBurned { target: Mon },
    /// "…'s hurt by its burn!"
    HurtByBurn { user: Mon },
    /// "… was frozen solid!"
    WasFrozen { target: Mon },
    /// "…'s already frozen!" (Crystal Legacy)
    AlreadyFrozen { target: Mon },
    /// "… is frozen solid!"
    FrozenSolid { user: Mon },
    /// "… was defrosted!"
    WasDefrosted { user: Mon },
    /// "… was defrosted!" (the target, by a fire move)
    DefrostedOpponent { target: Mon },
    /// "…'s paralyzed! Maybe it can't attack!"
    Paralyzed { target: Mon },
    /// "…'s already paralyzed!"
    AlreadyParalyzed { target: Mon },
    /// "…'s fully paralyzed!"
    FullyParalyzed { user: Mon },
    /// "… became confused!"
    BecameConfused { target: Mon },
    /// "…'s already confused!"
    AlreadyConfused { target: Mon },
    /// "… is confused!"
    IsConfused { user: Mon },
    /// "It hurt itself in its confusion!"
    HurtItself,
    /// "…'s confused no more!"
    ConfusedNoMore { user: Mon },
    /// "… fell in love!"
    FellInLove { target: Mon },
    /// "… is in love with …!"
    InLoveWith { user: Mon, target: Mon },
    /// "…'s infatuation kept it from attacking!"
    Infatuation { user: Mon },
    /// "… started to have a NIGHTMARE!"
    StartedNightmare { target: Mon },
    /// "… has a NIGHTMARE!"
    HasNightmare { user: Mon },
    /// "…'s hurt by the CURSE!"
    HurtByCurse { user: Mon },
    /// "… was seeded!"
    WasSeeded { target: Mon },
    /// "LEECH SEED saps …!"
    LeechSeedSaps { user: Mon },
    /// "… shed LEECH SEED!" (Rapid Spin)
    ShedLeechSeed { user: Mon },
    /// "… is protected by SAFEGUARD!"
    SafeguardProtect { target: Mon },
    /// "…'s SAFEGUARD faded!"
    SafeguardFaded { user: Mon },
    /// "…'s covered by a veil!" (Safeguard)
    CoveredByVeil { user: Mon },
    /// "…'s protected by MIST."
    ProtectedByMist { target: Mon },
    /// "…'s shrouded in MIST!"
    Mist { user: Mon },
    /// "…'s protected by …!" (a held item)
    ProtectedByItem { target: Mon, item: u8 },
    /// "… fled in fear!"
    FledInFear { target: Mon },
    /// "… was blown away!"
    BlownAway { target: Mon },
    /// "… was dragged out!"
    DraggedOut { user: Mon },

    // --- trapping, disabling, encore -------------------------------------------------
    /// "… was WRAPPED by …!"
    WrappedBy { target: Mon, user: Mon },
    /// "… was CLAMPED by …!"
    ClampedBy { target: Mon, user: Mon },
    /// "… used BIND on …!"
    UsedBind { user: Mon, target: Mon },
    /// "… was trapped!" (Fire Spin, Whirlpool)
    Trapped { target: Mon },
    /// "…'s hurt by …!" (a trapping move)
    HurtByTrap { user: Mon, mv: u8 },
    /// "… was released from …!"
    ReleasedFromTrap { user: Mon, mv: u8 },
    /// "… was released by …!" (Rapid Spin)
    ReleasedBy { user: Mon, target: Mon },
    /// "… can't escape now!" (Mean Look)
    CantEscapeNow { target: Mon },
    /// "…'s … was DISABLED!"
    WasDisabled { target: Mon, mv: u8 },
    /// "…'s … is DISABLED!"
    DisabledMove { user: Mon, mv: u8 },
    /// "…'s disabled no more!"
    DisabledNoMore { user: Mon },
    /// "… got an ENCORE!"
    GotEncore { target: Mon },
    /// "…'s ENCORE ended!"
    EncoreEnded { target: Mon },

    // --- protection, screens -----------------------------------------------------
    /// "… PROTECTED itself!"
    ProtectedItself { user: Mon },
    /// "…'s PROTECTING itself!"
    ProtectingItself { target: Mon },
    /// "… braced itself!" (Endure)
    BracedItself { user: Mon },
    /// "…'s SPCL.DEF rose!" (Light Screen)
    LightScreen { user: Mon },
    /// "…'s DEFENSE rose!" (Reflect)
    Reflect { user: Mon },
    /// "Your/Enemy #MON's LIGHT SCREEN fell!"
    LightScreenFell { side: Side },
    /// "Your/Enemy #MON's REFLECT faded!"
    ReflectFaded { side: Side },

    // --- weather and spikes -------------------------------------------------------
    /// "A downpour started!"
    Downpour,
    /// "Rain continues to fall."
    RainContinues,
    /// "The rain stopped."
    RainStopped,
    /// "The sunlight got bright!"
    SunGotBright,
    /// "The sunlight is strong."
    SunlightIsStrong,
    /// "The sunlight faded."
    SunlightFaded,
    /// "A SANDSTORM brewed!"
    SandstormBrewed,
    /// "The SANDSTORM rages."
    SandstormRages,
    /// "The SANDSTORM hits …!"
    SandstormHits { user: Mon },
    /// "The SANDSTORM subsided."
    SandstormSubsided,
    /// "SPIKES scattered all around …!"
    Spikes { target: Mon },
    /// "…'s hurt by SPIKES!"
    HurtBySpikes { user: Mon },
    /// "… blew away SPIKES!"
    BlewSpikes { user: Mon },

    // --- held items -------------------------------------------------------------------
    /// "…'s … activated!"
    ItemActivated { user: Mon, item: u8 },
    /// "… recovered with …." (a held berry)
    RecoveredWithItem { target: Mon, item: u8 },
    /// "… recovered using a …!"
    RecoveredUsing { target: Mon, item: u8 },
    /// "… recovered PP using …."
    RecoveredPpUsing { user: Mon, item: u8 },
    /// "A … rid … of its confusion."
    ItemHealedConfusion { item: u8, target: Mon },

    // --- the bag ---------------------------------------------------------------------
    /// "… used the …."
    PlayerUsedItem { player: String, item: u8 },
    /// "… used … on …!"
    EnemyUsedItem { trainer: Trainer, item: u8, mon: String },
    /// "Items can't be used here."
    ItemsCantBeUsedHere,
    /// "OAK: …! This isn't the time to use that!"
    OakWarning { player: String },
    /// "It won't have any effect."
    WontHaveEffect,
    /// "That can't be used on an EGG."
    CantUseOnEgg,
    /// "It looks bitter…"
    LooksBitter,
    /// "Restore the PP of which move?"
    RestoreWhichMove,
    /// "PP was restored."
    PpRestored,
    /// "… recovered … HP!"
    PartyRecoveredHp { mon: String, hp: u16 },
    /// "…'s cured of poison."
    PartyCuredOfPoison { mon: String },
    /// "…'s burn was healed."
    PartyBurnHealed { mon: String },
    /// "… was defrosted."
    PartyDefrosted { mon: String },
    /// "… woke up."
    PartyWokeUp { mon: String },
    /// "…'s rid of paralysis."
    PartyRidOfParalysis { mon: String },
    /// "…'s health returned."
    PartyHealthReturned { mon: String },
    /// "… is revitalized."
    PartyRevitalized { mon: String },
    /// "… came to its senses."
    PartyCameToSenses { mon: String },
    /// "… grew to level …!" (the party menu's)
    PartyGrewToLevel { mon: String, level: u8 },

    // --- catching ----------------------------------------------------------------------
    /// "The trainer blocked the BALL!"
    BallBlocked,
    /// "Don't be a thief!"
    BallDontBeAThief,
    /// "Oh no! The #MON broke free!"
    BallBrokeFree,
    /// "Aww! It appeared to be caught!"
    BallAppearedCaught,
    /// "Aargh! Almost had it!"
    BallAlmostHadIt,
    /// "Shoot! It was so close too!"
    BallSoClose,
    /// "Gotcha! … was caught!"
    Caught { mon: String },
    /// "…'s data was newly added to the #DEX."
    NewDexData { mon: String },
}
