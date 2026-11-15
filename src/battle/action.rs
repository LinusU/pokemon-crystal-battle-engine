//! The decisions a battle asks for: where it asks ([`PromptSite`]), whose
//! decision it is ([`Side`]) and the choice itself ([`Step`]).

/// Which side a recorded decision belongs to. A normal (wild / trainer) battle
/// only ever records `Player` steps and writes no `side` key, so those
/// recordings — and every fixture made before link battles existed — parse back
/// as `Player`. A link battle additionally records `Enemy` steps for the second
/// CLI player's move / switch picks, tagged `side = "enemy"`. See
/// [`super::cli::handle_enemy_action`] and docs/toml_schema.md.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Player,
    Enemy,
}

impl Side {
    /// The `side` tag written into / parsed from the recording TOML. `Player`
    /// returns `None` so player steps stay key-less (back-compat); only `Enemy`
    /// writes `side = "enemy"`.
    pub fn tag(self) -> Option<&'static str> {
        match self {
            Side::Player => None,
            Side::Enemy => Some("enemy"),
        }
    }
}

/// One captured CLI decision. Each variant maps 1:1 to one of the prompt sites
/// in [`super::cli`]. This is the *action* half of a `[[step]]`; the *outcome*
/// half is the `expect.*` keys written by [`observe`] / [`finalize`]. The side
/// the decision belongs to is tracked separately (see [`Side`] / [`record_side`])
/// rather than on the variant, since the action shape is identical for both.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Step {
    /// Action menu picked a move. `slot` is the 0-based move slot.
    Move { slot: u8 },
    /// Action menu picked a switch. `slot` is the 0-based party slot of
    /// the incoming mon.
    Switch { slot: u8 },
    /// Action menu picked an item. `target` is `Some(slot)` for items that
    /// target a specific party mon and `None` otherwise. `move_slot` is
    /// `Some(slot)` for single-move PP items (Ether / Max Ether / MysteryBerry)
    /// that target one move of the targeted mon, and `None` for every other
    /// item (whole-party PP, HP, status, revive). Older recordings have no
    /// `move` key, which parses back as `move_slot = None`.
    Item { id: u8, target: Option<u8>, move_slot: Option<u8> },
    /// Action menu picked Run.
    Run,
    /// Action menu picked FIGHT in a state where the original game shows no
    /// move menu and the engine picks the move itself: the mon is Encored
    /// (`ParsePlayerAction` uses `wLastPlayerMove`), is storing energy with
    /// Bide, or has no usable move (`CheckPlayerHasUsableMoves` forces
    /// Struggle). Offered instead of the `Move` options in exactly those
    /// states.
    Fight,
    /// `force_pick_party_mon_in_battle` — the active mon fainted and the
    /// engine forces us to send in another. `slot` is the chosen slot.
    ForcedSwitch { slot: u8 },
    /// `offer_switch` (trainer about to send in another mon). Either we
    /// keep the current mon (`keep = true`, `slot = None`) or we swap in
    /// the chosen slot.
    OfferSwitch { keep: bool, slot: Option<u8> },
}

/// What kind of decision the CLI driver is asking for. Used for the
/// mismatch-error message when the recorded `kind` doesn't fit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PromptSite {
    BattleMenu,
    MoveSelection,
    ForcedSwitch,
    OfferSwitch,
    /// The second CLI player's per-round action in a link battle (move or
    /// switch). Reached from the `link_battle_send_receive_action` hook rather
    /// than an engine menu. Consumes only `Enemy`-side steps.
    EnemyAction,
    /// The enemy trainer's per-turn action when the trainer is controlled
    /// from outside (`enemy_control = "external"`): a move, FIGHT, a switch,
    /// or one of the trainer's AI items. Replaces `AIChooseMove` and the
    /// switch/item choice of `AI_SwitchOrTryItem`. Consumes `Enemy` steps.
    EnemyTrainerAction,
    /// Which mon an externally controlled trainer sends in next (after a
    /// faint, or Baton Pass). Replaces `FindMonInOTPartyToSwitchIntoBattle`.
    /// Consumes `Enemy` steps.
    EnemySwitchIn,
}

impl PromptSite {
    /// A human-readable name for the site.
    pub fn label(self) -> &'static str {
        match self {
            PromptSite::BattleMenu => "battle menu (move/switch/item/run)",
            PromptSite::MoveSelection => "move re-selection (Disable / no-PP)",
            PromptSite::ForcedSwitch => "forced switch (active fainted)",
            PromptSite::OfferSwitch => "offer switch (enemy sending in next mon)",
            PromptSite::EnemyAction => "enemy action (link battle, move/switch)",
            PromptSite::EnemyTrainerAction => "enemy trainer action (move/switch/item)",
            PromptSite::EnemySwitchIn => "enemy trainer switch-in",
        }
    }

    /// Which side's recorded steps this prompt site consumes. Every player-menu
    /// site takes `Player` steps; the link-battle enemy hook takes `Enemy`
    /// steps. Playback uses this to keep the interleaved step stream in lockstep
    /// with the engine's alternating player/enemy prompts.
    pub fn side(self) -> Side {
        match self {
            PromptSite::EnemyAction | PromptSite::EnemyTrainerAction | PromptSite::EnemySwitchIn => Side::Enemy,
            _ => Side::Player,
        }
    }
}

