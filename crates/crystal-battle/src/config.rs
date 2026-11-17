//! A battle's configuration: who fights whom, with what. The TOML scenario
//! format (`docs/toml_schema.md`) deserializes straight into [`BattleConfig`];
//! the engine ([`crate::battle::Battle`]) starts a battle from one.

use serde::Deserialize;

use crate::data::{items, names};

/// Move slots per mon.
const NUM_MOVE_SLOTS: usize = 4;

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
    /// Optional `[enemy_trainer]` table for trainer battles. When present,
    /// overrides the trainer class's AI move-flags and/or AI item-flags
    /// (and thus enemy item usage strategy) by poking the `TrainerClassAttributes`
    /// ROM entry before `start_battle`. See [`EnemyTrainerOverride`].
    pub enemy_trainer: Option<EnemyTrainerOverride>,
}

/// Optional per-battle AI overrides for trainer battles. Each field, when
/// `Some`, replaces the corresponding word in the trainer class's
/// `TrainerClassAttributes` ROM entry. The flag names are case-sensitive and
/// must match the spellings in `scratch/pokecrystal/constants/trainer_data_constants.asm`.
#[derive(Debug, Clone, Deserialize)]
pub struct EnemyTrainerOverride {
    /// When `Some`, replaces `TRNATTR_AI_MOVE_WEIGHTS` (the 16-bit move-AI
    /// flag word) for the trainer's class. Valid flag names (bit positions):
    /// `AI_BASIC` (0), `AI_SETUP` (1), `AI_TYPES` (2), `AI_OFFENSIVE` (3),
    /// `AI_SMART` (4), `AI_OPPORTUNIST` (5), `AI_AGGRESSIVE` (6),
    /// `AI_CAUTIOUS` (7), `AI_STATUS` (8), `AI_RISKY` (9). An empty list is
    /// valid and means "no AI move scoring" (word = 0).
    pub ai_move_flags: Option<Vec<String>>,
    /// When `Some`, replaces `TRNATTR_AI_ITEM_SWITCH` (the 16-bit item/switch
    /// AI flag word) for the trainer's class. Valid flag names (bit positions):
    /// `SWITCH_OFTEN` (0), `SWITCH_RARELY` (1), `SWITCH_SOMETIMES` (2),
    /// `ALWAYS_USE` (4), `UNKNOWN_USE` (5), `CONTEXT_USE` (6). An empty list
    /// is valid and means "never use items / switch" (word = 0).
    pub ai_item_flags: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BattleHeader {
    #[serde(rename = "type")]
    pub kind: BattleKind,
    /// Required for `trainer_battle`. Resolved via [`crate::data::trainers`].
    pub trainer: Option<String>,
    /// Seed for the deterministic battle RNG. If unset, the harness picks
    /// one from system entropy and logs it. Accepts decimal (`42`) or hex
    /// (`0xdeadbeef`) literals via TOML's integer parser.
    pub seed: Option<u64>,
    /// Johto badge bitfield (`wJohtoBadges`, 0xd857). Each bit corresponds to
    /// one badge; `0xff` = all 8 Johto badges held. When omitted, defaults to 0
    /// (no badges). The badge boost in `misc.rs` applies a stat multiplier for
    /// player Pokémon of certain types when the matching badge is held.
    #[serde(default)]
    pub johto_badges: Option<u8>,
    /// Kanto badge bitfield (`wKantoBadges`, 0xd858). Each bit corresponds to
    /// one badge; `0xff` = all 8 Kanto badges held. When omitted, defaults to 0.
    #[serde(default)]
    pub kanto_badges: Option<u8>,
    /// The game's Battle Scene option (`wOptions` bit `BATTLE_SCENE`). `true`
    /// (the game's default, and the default when omitted) plays move
    /// animations; `false` skips them. Battle logic is identical either way
    /// (the whole fixture corpus replays unchanged with it off); `false` is
    /// ~25% faster headless. Text differs in one place: with the scene off,
    /// Present on a fully healed target also prints "…can't receive the gift".
    #[serde(default)]
    pub battle_scene: Option<bool>,
    /// Who decides for the enemy trainer (trainer battles only). `original`
    /// (the default) runs the game's own trainer AI. `external` replaces its
    /// three choices — the move (`AIChooseMove`), switching or using an item
    /// (`AI_SwitchOrTryItem`), and the next mon to send in
    /// (`FindMonInOTPartyToSwitchIntoBattle`) — with `Enemy`-side decisions
    /// ([`PromptSite::EnemyTrainerAction`](crate::battle::PromptSite::EnemyTrainerAction),
    /// [`PromptSite::EnemySwitchIn`](crate::battle::PromptSite::EnemySwitchIn)), so
    /// another policy (a trained AI, a second player) plays the trainer.
    #[serde(default)]
    pub enemy_control: EnemyControl,
}

/// See [`BattleHeader::enemy_control`].
#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum EnemyControl {
    #[default]
    Original,
    External,
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
    /// hook (the harness's `launch_link_battle`).
    LinkBattle,
}

impl BattleKind {
    /// The `[battle] type` string this kind deserializes from — the inverse of
    /// the `snake_case` serde rename. Used by the harness's recorder to write an embedded
    /// scenario block that the harness can read straight back.
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
    /// the harness's `seed_player_party`. Resolved via [`names::move_id_from_name`].
    #[serde(default)]
    pub moves: Option<Vec<String>>,
    /// Optional held item name. When set, the item id is written into the slot's
    /// `MON_ITEM` byte after generation, so the mon walks into battle holding it
    /// (Leftovers, King's Rock, type boosters, …). Honored for the player party
    /// (every battle kind) and for the link-battle enemy party — both go through
    /// the harness's `seed_party`. A wild/trainer enemy's item comes from the engine / ROM,
    /// so `item` on those `[[enemy_party]]` entries is ignored, like `moves`.
    /// Resolved via [`items::lookup`].
    #[serde(default)]
    pub item: Option<String>,
    /// Optional original-trainer ID. The player's own ID is `0x1234`; any other
    /// value makes the mon traded, which the engine treats differently (it may
    /// disobey above the badge level cap, and it gains boosted experience).
    /// Honored for the player party only.
    #[serde(default)]
    pub ot_id: Option<u16>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ItemSpec {
    pub name: String,
    pub quantity: u8,
}



/// Valid `ai_move_flags` names from `trainer_data_constants.asm` and their bit
/// positions within the `TRNATTR_AI_MOVE_WEIGHTS` 16-bit word.
pub const AI_MOVE_FLAG_NAMES: &[(&str, u16)] = &[
    ("AI_BASIC",       1 << 0),
    ("AI_SETUP",       1 << 1),
    ("AI_TYPES",       1 << 2),
    ("AI_OFFENSIVE",   1 << 3),
    ("AI_SMART",       1 << 4),
    ("AI_OPPORTUNIST", 1 << 5),
    ("AI_AGGRESSIVE",  1 << 6),
    ("AI_CAUTIOUS",    1 << 7),
    ("AI_STATUS",      1 << 8),
    ("AI_RISKY",       1 << 9),
];

/// Valid `ai_item_flags` names from `trainer_data_constants.asm` and their bit
/// positions within the `TRNATTR_AI_ITEM_SWITCH` 16-bit word.
pub const AI_ITEM_FLAG_NAMES: &[(&str, u16)] = &[
    ("SWITCH_OFTEN",     1 << 0),
    ("SWITCH_RARELY",    1 << 1),
    ("SWITCH_SOMETIMES", 1 << 2),
    // bit 3 is reserved (const_skip in the asm)
    ("ALWAYS_USE",       1 << 4),
    ("UNKNOWN_USE",      1 << 5),
    ("CONTEXT_USE",      1 << 6),
];

/// Resolve a list of AI flag name strings to a 16-bit flag word. Each name must
/// exist in `table`. Unknown names are a hard error. An empty list is valid and
/// produces word = 0.
pub fn resolve_ai_flags(names: &[String], table: &[(&str, u16)], field: &str) -> Result<u16, String> {
    let mut word: u16 = 0;
    for name in names {
        let Some(&(_, bit)) = table.iter().find(|&&(n, _)| n == name.as_str()) else {
            let valid: Vec<&str> = table.iter().map(|&(n, _)| n).collect();
            return Err(format!(
                "unknown {field} flag {:?}; valid flags are: {}",
                name,
                valid.join(", ")
            ));
        };
        word |= bit;
    }
    Ok(word)
}



/// Apply an `[enemy_trainer]` override's AI flags to a trainer class's
/// attributes.
pub fn apply_ai_flag_override(class: &mut crate::data::game::TrainerClass, ov: &EnemyTrainerOverride) -> Result<(), String> {
    if let Some(names) = &ov.ai_move_flags {
        class.ai_move_weights = resolve_ai_flags(names, AI_MOVE_FLAG_NAMES, "ai_move_flags")?.to_le_bytes();
    }
    if let Some(names) = &ov.ai_item_flags {
        class.ai_item_switch = resolve_ai_flags(names, AI_ITEM_FLAG_NAMES, "ai_item_flags")?.to_le_bytes();
    }
    Ok(())
}

/// Resolve a mon's optional `moves` list to move ids, validating the count and
/// every name. `None` means "keep the engine's generated moveset" and leaves
/// the slot untouched.
pub fn resolve_moves(mon: &MonSpec) -> Result<Option<Vec<u8>>, String> {
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

/// Resolve a mon's optional `item` name to its item id, validating the name.
/// `None` means "no held item" and leaves the slot's `MON_ITEM` byte as
/// `try_add_mon_to_party` left it (0, since we add mons with `wBattleMode = 0`).
pub fn resolve_item(mon: &MonSpec, label: &str) -> Result<Option<u8>, String> {
    let Some(name) = &mon.item else {
        return Ok(None);
    };
    let def = items::lookup(name)
        .ok_or_else(|| format!("unknown held item for {label} {}: {}", mon.species, name))?;
    Ok(Some(def.id))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mon(moves: Option<&[&str]>) -> MonSpec {
        MonSpec {
            species: "Pikachu".into(),
            level: 10,
            moves: moves.map(|m| m.iter().map(|s| s.to_string()).collect()),
            item: None,
            ot_id: None,
        }
    }

    fn mon_with_item(item: Option<&str>) -> MonSpec {
        MonSpec {
            species: "Pikachu".into(),
            level: 10,
            moves: None,
            item: item.map(|s| s.to_string()),
            ot_id: None,
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

    #[test]
    fn no_item_key_leaves_slot_itemless() {
        assert_eq!(resolve_item(&mon_with_item(None), "player").unwrap(), None);
    }

    #[test]
    fn item_resolves_to_id() {
        // Leftovers=0x92, King's Rock=0x52 — case/punctuation insensitive.
        assert_eq!(resolve_item(&mon_with_item(Some("Leftovers")), "player").unwrap(), Some(0x92));
        assert_eq!(resolve_item(&mon_with_item(Some("kings rock")), "enemy").unwrap(), Some(0x52));
    }

    #[test]
    fn unknown_item_name_is_rejected() {
        let err = resolve_item(&mon_with_item(Some("Master Sword")), "player").unwrap_err();
        assert!(err.contains("Master Sword"), "{err}");
    }

    // -------------------------------------------------------------------------
    // AI flag resolution
    // -------------------------------------------------------------------------

    fn flags(names: &[&str]) -> Vec<String> {
        names.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn ai_move_empty_list_gives_zero() {
        assert_eq!(resolve_ai_flags(&[], AI_MOVE_FLAG_NAMES, "ai_move_flags").unwrap(), 0);
    }

    #[test]
    fn ai_move_known_flags_round_trip() {
        // AI_BASIC=bit0, AI_SMART=bit4 → 0x11 (17 dec)
        let word = resolve_ai_flags(
            &flags(&["AI_BASIC", "AI_SMART"]),
            AI_MOVE_FLAG_NAMES,
            "ai_move_flags",
        )
        .unwrap();
        assert_eq!(word, (1 << 0) | (1 << 4));
        assert_eq!(word, 0x0011);
    }

    #[test]
    fn ai_item_known_flags_round_trip() {
        // CONTEXT_USE=bit6=0x40, SWITCH_SOMETIMES=bit2=0x04 → 0x44
        let word = resolve_ai_flags(
            &flags(&["CONTEXT_USE", "SWITCH_SOMETIMES"]),
            AI_ITEM_FLAG_NAMES,
            "ai_item_flags",
        )
        .unwrap();
        assert_eq!(word, (1 << 6) | (1 << 2));
        assert_eq!(word, 0x0044);
    }

    #[test]
    fn ai_move_unknown_flag_is_rejected() {
        let err = resolve_ai_flags(
            &flags(&["AI_BASIC", "AI_INVALID"]),
            AI_MOVE_FLAG_NAMES,
            "ai_move_flags",
        )
        .unwrap_err();
        assert!(err.contains("AI_INVALID"), "{err}");
        assert!(err.contains("ai_move_flags"), "{err}");
    }

    #[test]
    fn ai_item_unknown_flag_is_rejected() {
        let err = resolve_ai_flags(
            &flags(&["SWITCH_OFTEN", "NOTAFLAG"]),
            AI_ITEM_FLAG_NAMES,
            "ai_item_flags",
        )
        .unwrap_err();
        assert!(err.contains("NOTAFLAG"), "{err}");
    }

    // -------------------------------------------------------------------------
    // Attribute poke offset correctness
    // -------------------------------------------------------------------------

}
