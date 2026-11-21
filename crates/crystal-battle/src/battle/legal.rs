//! The legal choices at every decision point, mirroring the original game's
//! menus exactly: which moves are selectable, which mons can switch in, which
//! bag items would have an effect and on whom. This is the action space of
//! [`crate::battle::Battle`]; the emulated harness checks recordings and
//! deciders against the same lists.

use crate::battle::action::{PromptSite, Step};
use crate::data::{move_pp, names};
use crate::defs;
use crate::engine::mem::ReadMem;
use crate::engine::sym;

const W_ENEMY_TRAINER_ITEM1: u16 = sym::W_ENEMY_TRAINER_ITEM1;
const W_ENEMY_SUB_STATUS_4: u16 = sym::W_ENEMY_SUB_STATUS4;
const W_ENEMY_SUB_STATUS_3: u16 = sym::W_ENEMY_SUB_STATUS3;
const W_ENEMY_WRAP_COUNT: u16 = sym::W_ENEMY_WRAP_COUNT;
// ---------------------------------------------------------------------------
// WRAM addresses. Source of truth: scratch/pokecrystal/pokecrystal.sym.
// Mirrors docs/memory_layout.md.

const W_BATTLE_MON_MOVES: u16 = 0xc62e;
const W_BATTLE_MON_PP: u16 = 0xc634;

// `wPlayerSubStatus3` holds the active player mon's battle-only sub-statuses.
// Confusion (`SUBSTATUS_CONFUSED`, bit 7) lives here, NOT in the party/battle
// status byte — it only exists on the active battler. See
// `scratch/pokecrystal/constants/battle_constants.asm` (SubStatus3 bits) and
// `pokecrystal.sym` (`00:c66a wPlayerSubStatus3`).
const W_PLAYER_SUB_STATUS_3: u16 = 0xc66a;
// `wPlayerSubStatus4` holds another byte of the active player mon's battle-only
// sub-statuses. The three in-battle "X" items that set a single bit here live in
// this byte: X Accuracy (`SUBSTATUS_X_ACCURACY`, bit 0), Guard Spec
// (`SUBSTATUS_MIST`, bit 1) and Dire Hit (`SUBSTATUS_FOCUS_ENERGY`, bit 2). The
// SubStatus4 bit list in `scratch/pokecrystal/constants/battle_constants.asm`
// (`const_def` from 0) is X_ACCURACY=0, MIST=1, FOCUS_ENERGY=2, then a
// `const_skip`, SUBSTITUTE=4, RECHARGE=5, RAGE=6, LEECH_SEED=7 — so the masks
// are `1 << 0`, `1 << 1`, `1 << 2`. `pokecrystal.sym`: `00:c66b wPlayerSubStatus4`.
const W_PLAYER_SUB_STATUS_4: u16 = 0xc66b;
const SUBSTATUS_X_ACCURACY_MASK: u8 = 1 << 0;
const SUBSTATUS_MIST_MASK: u8 = 1 << 1;
const SUBSTATUS_FOCUS_ENERGY_MASK: u8 = 1 << 2;
const W_ENEMY_SUB_STATUS_5: u16 = 0xc671;
// `wPlayerSubStatus5` (Encore lives here, `SUBSTATUS_ENCORED`) and
// `wLastPlayerMove` (the move an Encore repeats). `00:c66c wPlayerSubStatus5`,
// `00:c71b wLastPlayerMove` in `pokecrystal.sym`.
const W_PLAYER_SUB_STATUS_5: u16 = 0xc66c;
// `wLinkMode` — non-zero in a link battle. `00:c2dc wLinkMode`.
const W_LINK_MODE: u16 = 0xc2dc;
// The active player mon's Disable counter. High nibble = the disabled move's
// 1-based slot (0 = nothing disabled); low nibble = turns remaining.
// `00:c675 wPlayerDisableCount` in `pokecrystal.sym`.
const W_PLAYER_DISABLE_COUNT: u16 = 0xc675;

const STAT_ATTACK: u8 = 0;
const STAT_DEFENSE: u8 = 1;
const STAT_SPEED: u8 = 2;
const STAT_SP_ATTACK: u8 = 3;
const W_PLAYER_WRAP_COUNT: u16 = 0xc730;

const W_CUR_BATTLE_MON: u16 = 0xd0d4;

const W_PARTY_COUNT: u16 = 0xdcd7;
const W_PARTY_MON1: u16 = 0xdcdf;
const PARTY_STRUCT_LEN: u16 = 48;

const W_NUM_ITEMS: u16 = 0xd892;
const W_NUM_BALLS: u16 = 0xd8d7;

// Poké Doll item id (`POKE_DOLL` in `scratch/pokecrystal/constants/item_constants.asm`).
// In a wild battle it ends the battle immediately as an escape (a DRAW); in a
// trainer battle it does nothing (and we never offer it there). See
// `PokeDollEffect` in `scratch/pokecrystal/engine/items/item_effects.asm`.
const POKE_DOLL_ID: u8 = 0x25;

// `wBattleMode` — 0 = not in battle, 1 = wild, 2 = trainer (`WILD_BATTLE` /
// `TRAINER_BATTLE` in `scratch/pokecrystal/constants/battle_constants.asm`).
// We only offer Poké Balls in a wild battle (`offers_balls`); a thrown ball in
// a trainer battle is refused by the real game ("The trainer blocked the BALL!").
// `00:d22d wBattleMode` in `pokecrystal.sym`.
const W_BATTLE_MODE: u16 = 0xd22d;

const W_ENEMY_MON_MOVES: u16 = 0xd208;
const W_ENEMY_MON_PP: u16 = 0xd20e;
const W_ENEMY_MON_HP: u16 = 0xd216;
// The active enemy mon's Disable counter — same packing as wPlayerDisableCount
// (high nibble = disabled move's 1-based slot, 0 = nothing). `00:c67d
// wEnemyDisableCount` in `pokecrystal.sym`.
const W_ENEMY_DISABLE_COUNT: u16 = 0xc67d;
// Enemy active party slot, and the OT party list (same `party_struct` layout as
// the player party). `00:c663 wCurOTMon`, `01:d280 wOTPartyCount`,
// `01:d288 wOTPartyMon1` in `pokecrystal.sym`.
const W_CUR_OT_MON: u16 = 0xc663;
const W_OT_PARTY_COUNT: u16 = 0xd280;
const W_OT_PARTY_MON1: u16 = 0xd288;

// Party struct field offsets (see docs/memory_layout.md).
const PARTY_OFFSET_MOVES: u16 = 2;
const PARTY_OFFSET_PP: u16 = 23;
const PARTY_OFFSET_STATUS: u16 = 32;
const PARTY_OFFSET_HP: u16 = 34;
const PARTY_OFFSET_MAX_HP: u16 = 36;

// A party mon (and the active battle-mon mirror) holds 4 move + 4 PP slots.
const MOVE_SLOTS: u16 = 4;

// Bit of `SUBSTATUS_CONFUSED` within `wPlayerSubStatus3`. The SubStatus3 bit
// list in `constants/battle_constants.asm` is BIDE=0, RAMPAGE=1, IN_LOOP=2,
// FLINCHED=3, CHARGED=4, UNDERGROUND=5, FLYING=6, CONFUSED=7, so the mask is
// `1 << 7`.
const SUBSTATUS_CONFUSED_MASK: u8 = 1 << 7;

/// Is the active player mon confused, given its `wPlayerSubStatus3` byte?
/// Pure helper so the bit test is unit-testable without a `Cpu`.
pub fn is_active_confused(sub_status3: u8) -> bool {
    sub_status3 & SUBSTATUS_CONFUSED_MASK != 0
}

/// Whether the action menu should offer the Poké Doll (a guaranteed escape),
/// given `wBattleMode`. Like a ball, it only does anything in a wild battle
/// (`WILD_BATTLE` = 1): `PokeDollEffect` is a no-op in a trainer battle (it sets
/// `wItemEffectSucceeded = 0` and returns), so we never list it there. Pure
/// helper so the mode test is unit-testable without a `Cpu`.
pub fn offers_escape_item(battle_mode: u8) -> bool {
    battle_mode == defs::WILD_BATTLE
}

/// Whether `id` is a guaranteed-escape item (the Poké Doll) — an Items-pocket
/// item that ends a wild battle as an escape rather than healing a party mon. It
/// takes no party target and isn't classified by [`item_effect`], so the
/// CLI's item-offering loop (`prompt_action`) routes it to its own `Action::Escape`
/// branch (and playback routes the recorded item id back the same way).
pub fn is_escape_item_id(id: u8) -> bool {
    id == POKE_DOLL_ID
}

/// The move slot an external trainer's `EnemyTrainerAction` leaves in
/// `wCurEnemyMoveNum` / `wCurEnemyMove`. `AIChooseMove` always leaves a
/// pick, and the rest of the turn reads it even when the trainer then
/// switches or uses an item (`ParseEnemyAction` loads its move data). So a
/// switch, an item, or FIGHT (Encore / Struggle, which `ParseEnemyAction`
/// overrides anyway) leaves the first usable move, or slot 0 if none.
pub fn external_enemy_move_slot(cpu: &impl ReadMem, step: Step) -> u8 {
    match step {
        Step::Move { slot } => slot,
        _ => enemy_move_options(cpu).first().copied().unwrap_or(0),
    }
}

/// Address of OT party slot `slot` (enemy `party_struct`).
pub fn ot_party_slot_addr(slot: u8) -> u16 {
    W_OT_PARTY_MON1 + u16::from(slot) * PARTY_STRUCT_LEN
}

pub fn read_ot_party_species(cpu: &impl ReadMem, slot: u8) -> u8 {
    cpu.read_byte(ot_party_slot_addr(slot))
}

/// The enemy's selectable move slots: a real move (id != 0) with PP left that
/// isn't Disabled. Mirrors [`move_options`] against the enemy's `wEnemyMon*`
/// fields and `wEnemyDisableCount`.
pub fn enemy_move_options(cpu: &impl ReadMem) -> Vec<u8> {
    let disabled_slot = (cpu.read_byte(W_ENEMY_DISABLE_COUNT) >> 4).wrapping_sub(1);
    let mut out = Vec::with_capacity(defs::NUM_MOVES as usize);
    for slot in 0..defs::NUM_MOVES {
        let move_id = cpu.read_byte(W_ENEMY_MON_MOVES + u16::from(slot));
        if move_id == 0 {
            continue;
        }
        let pp = cpu.read_byte(W_ENEMY_MON_PP + u16::from(slot)) & defs::PP_MASK;
        if pp == 0 {
            continue;
        }
        if slot == disabled_slot {
            continue;
        }
        out.push(slot);
    }
    out
}

/// The enemy's alive, non-current OT party slots. Mirrors [`alive_party_slots`]
/// against `wOTPartyCount` / the OT party list.
pub fn enemy_alive_party_slots(cpu: &impl ReadMem, exclude: Option<u8>) -> Vec<u8> {
    let n = cpu.read_byte(W_OT_PARTY_COUNT);
    let mut out = Vec::with_capacity(n as usize);
    for slot in 0..n {
        if Some(slot) == exclude {
            continue;
        }
        if read_word_be(cpu, ot_party_slot_addr(slot) + PARTY_OFFSET_HP) == 0 {
            continue;
        }
        out.push(slot);
    }
    out
}

pub fn check_can_switch(cpu: &impl ReadMem, target: u8) -> Result<(), String> {
    let current = cpu.read_byte(W_CUR_BATTLE_MON);
    if target == current {
        return Err(format!("That Pokémon is already in battle."));
    }
    if cpu.read_byte(W_PLAYER_WRAP_COUNT) != 0 {
        return Err("You're trapped by a multi-turn attack — can't switch.".to_string());
    }
    let enemy_sub5 = cpu.read_byte(W_ENEMY_SUB_STATUS_5);
    if enemy_sub5 & (1 << defs::SUBSTATUS_CANT_RUN) != 0 {
        return Err("You're locked in — can't switch.".to_string());
    }
    let hp = read_word_be(cpu, party_slot_addr(target) + PARTY_OFFSET_HP);
    if hp == 0 {
        return Err(format!("Slot {target} is fainted."));
    }
    Ok(())
}

/// What a battle-usable item does, abstracted over the specific item id. The
/// single source of truth for which items the CLI applies directly, which party
/// mons a given item can affect, and the status mask the item routine clears.
/// Items not classified here aren't offered in the action menu (balls,
/// evolution stones, ...).
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)] // the payloads document each effect; the real routines apply them
pub enum ItemEffect {
    /// Restores HP — only affects a living mon below max HP.
    Hp,
    /// Revives a fainted mon (HP == 0).
    Revive,
    /// Clears the given status bits — only affects a living, statused mon.
    Status(u8),
    /// Full Restore: HP plus every status — affects a living mon that's hurt or
    /// statused.
    FullRestore,
    /// Restores PP to every move of the target. `amount: Some(n)` adds `n` PP to
    /// each move (capped at max PP); `amount: None` (Max Elixer) restores every
    /// move to full. Unlike the vitals-based effects, whether this "would have an
    /// effect" depends on per-move PP, so [`ItemEffect::affects`] can't decide
    /// it — PP targeting is special-cased in [`item_valid_targets`].
    PpAll { amount: Option<u8> },
    /// Restores PP to ONE chosen move of the target (Ether / Max Ether /
    /// MysteryBerry). `amount: Some(n)` adds `n` PP to the chosen move (capped at
    /// max PP); `amount: None` (Max Ether) restores it to full. Like
    /// [`ItemEffect::PpAll`], whether it "would have an effect" depends on
    /// per-move PP, so it's special-cased in [`item_valid_targets`] /
    /// [`ItemEffect::affects`] rather than judged by vitals.
    PpOne { amount: Option<u8> },
    /// Clears `SUBSTATUS_CONFUSED` on the active player mon (Bitter Berry). Unlike
    /// every other effect, confusion is a battle-only sub-status that lives in
    /// `wPlayerSubStatus3`, not the party struct — so this item has NO party
    /// target (it always acts on the active battler) and its validity is judged
    /// from the sub-status byte, not vitals. It's surfaced as an "active-mon item"
    /// in the CLI's `prompt_action` rather than going through the party-target machinery.
    ActiveConfusion,
    /// Raises one stat stage of the active player mon by one level (X Attack /
    /// X Defend / X Speed / X Special). The payload is the stat index into
    /// `wPlayerStatLevels` (ATTACK=0, DEFENSE=1, SPEED=2, SP_ATTACK=3). Like
    /// [`ItemEffect::ActiveConfusion`] this acts on the active battler with no
    /// party target; its validity is judged from the stage level (usable only
    /// while below `MAX_STAT_LEVEL`), so it's an "active-mon item" in
    /// the CLI's `prompt_action`. See `XItemEffect` / `RaiseStat`
    /// (`scratch/pokecrystal/engine/items/item_effects.asm`,
    /// `engine/battle/effect_commands.asm`).
    ActiveStatUp(u8),
    /// Sets a single sub-status bit `mask` in the byte at `addr` on the active
    /// player mon, refusing if it's already set (X Accuracy → `SUBSTATUS_X_ACCURACY`,
    /// Guard Spec → `SUBSTATUS_MIST`, Dire Hit → `SUBSTATUS_FOCUS_ENERGY`, all in
    /// `wPlayerSubStatus4`). Like [`ItemEffect::ActiveConfusion`] (which is the
    /// inverse — a bit-CLEAR) this acts on the active battler with no party
    /// target; its validity is judged from the sub-status byte (usable only while
    /// the bit is CLEAR), so it's an "active-mon item" in the CLI's `prompt_action`. See
    /// `XAccuracyEffect` / `GuardSpecEffect` / `DireHitEffect`
    /// (`scratch/pokecrystal/engine/items/item_effects.asm`), each literally a
    /// `bit`/`set` of its mask in `wPlayerSubStatus4`. There is no separate Mist
    /// timer in Gen II — Guard Spec just sets the MIST bit.
    ActiveSubStatusSet { addr: u16, mask: u8 },
}

impl ItemEffect {
    /// Would applying this item to a mon with these vitals change anything?
    /// Mirrors the game's "It wouldn't have any effect" guard, so the CLI only
    /// offers items — and target slots — that actually do something.
    ///
    /// This only sees HP/status, which is enough for every effect EXCEPT
    /// [`ItemEffect::PpAll`] (PP isn't a vital). PP items never reach this method
    /// — [`item_valid_targets`] handles them with a per-move PP check — so this
    /// arm returns `false` defensively.
    fn affects(self, hp: u16, max_hp: u16, status: u8) -> bool {
        match self {
            ItemEffect::Hp => hp != 0 && hp < max_hp,
            ItemEffect::Revive => hp == 0,
            ItemEffect::Status(mask) => hp != 0 && status & mask != 0,
            ItemEffect::FullRestore => hp != 0 && (hp < max_hp || status != 0),
            // PP, confusion and stat stages aren't vitals — these never reach
            // `affects` (`item_valid_targets` / the active-mon branch judge
            // them), so this returns `false` defensively.
            ItemEffect::PpAll { .. }
            | ItemEffect::PpOne { .. }
            | ItemEffect::ActiveConfusion
            | ItemEffect::ActiveStatUp(_)
            | ItemEffect::ActiveSubStatusSet { .. } => false,
        }
    }
}

/// Classify an item id into the effect the CLI knows how to apply, or `None`
/// for items we don't surface in the action menu. Status masks follow
/// `scratch/pokecrystal/data/items/heal_status.asm`.
pub fn item_effect(id: u8) -> Option<ItemEffect> {
    Some(match id {
        // HP-restoring: Potion, Super, Hyper, Max, drinks, Moomoo Milk, herbs,
        // berries, Berry Juice, RageCandyBar.
        0x12 | 0x11 | 0x10 | 0x0f | 0x2e | 0x2f | 0x30 | 0x48 | 0x79 | 0x7a | 0xad
        | 0xae | 0x8b | 0x72 => ItemEffect::Hp,
        // Revive, Max Revive, Revival Herb.
        0x27 | 0x28 | 0x7c => ItemEffect::Revive,
        // Full Restore — HP plus all status.
        0x0e => ItemEffect::FullRestore,
        // Whole-party PP restore. Elixer adds 10 PP to every move; Max Elixer
        // refills every move. See `RestorePP` in
        // `scratch/pokecrystal/engine/items/item_effects.asm`.
        0x41 => ItemEffect::PpAll { amount: Some(10) }, // Elixer
        0x15 => ItemEffect::PpAll { amount: None },     // Max Elixer
        // Single-move PP restore. Ether adds 10 PP to one move; Max Ether
        // refills one move; MysteryBerry adds 5 PP to one move. See `RestorePP`
        // (`.restore_one` path) in
        // `scratch/pokecrystal/engine/items/item_effects.asm`.
        0x3f => ItemEffect::PpOne { amount: Some(10) }, // Ether
        0x40 => ItemEffect::PpOne { amount: None },     // Max Ether
        0x96 => ItemEffect::PpOne { amount: Some(5) },  // MysteryBerry
        // Status heals.
        0x09 => ItemEffect::Status(1 << defs::PSN as u8), // Antidote — poison
        0x0a => ItemEffect::Status(1 << defs::BRN as u8), // Burn Heal
        0x0b => ItemEffect::Status(1 << defs::FRZ as u8), // Ice Heal
        0x0c => ItemEffect::Status(defs::SLP_MASK),       // Awakening — sleep counter
        0x0d => ItemEffect::Status(1 << defs::PAR as u8), // Parlyz Heal
        0x26 => ItemEffect::Status(0xff),                 // Full Heal — any status
        0x4a => ItemEffect::Status(1 << defs::PSN as u8), // PSNCureBerry — poison
        0x4e => ItemEffect::Status(1 << defs::PAR as u8), // PRZCureBerry — paralysis
        0x4f => ItemEffect::Status(1 << defs::FRZ as u8), // Burnt Berry — freeze
        0x50 => ItemEffect::Status(1 << defs::BRN as u8), // Ice Berry — burn
        0x54 => ItemEffect::Status(defs::SLP_MASK),       // Mint Berry — sleep counter
        // Bitter Berry — clears confusion on the active mon. Battle-only
        // sub-status; no party target. See `BitterBerryEffect`
        // (`scratch/pokecrystal/engine/items/item_effects.asm`).
        0x53 => ItemEffect::ActiveConfusion,
        // X stat-boosters — raise one stat stage of the active mon by one
        // level. Item→stat mapping from `data/items/x_stats.asm` (`XItemStats`):
        // X_ATTACK→ATTACK, X_DEFEND→DEFENSE, X_SPEED→SPEED, X_SPECIAL→SP_ATTACK.
        0x31 => ItemEffect::ActiveStatUp(STAT_ATTACK),    // X Attack
        0x33 => ItemEffect::ActiveStatUp(STAT_DEFENSE),   // X Defend
        0x34 => ItemEffect::ActiveStatUp(STAT_SPEED),     // X Speed
        0x35 => ItemEffect::ActiveStatUp(STAT_SP_ATTACK), // X Special
        // In-battle sub-status setters — set one bit of `wPlayerSubStatus4` on
        // the active mon, refusing if already set. See `XAccuracyEffect` /
        // `GuardSpecEffect` / `DireHitEffect`
        // (`scratch/pokecrystal/engine/items/item_effects.asm`).
        0x21 => ItemEffect::ActiveSubStatusSet {
            addr: W_PLAYER_SUB_STATUS_4,
            mask: SUBSTATUS_X_ACCURACY_MASK,
        }, // X Accuracy
        0x29 => ItemEffect::ActiveSubStatusSet {
            addr: W_PLAYER_SUB_STATUS_4,
            mask: SUBSTATUS_MIST_MASK,
        }, // Guard Spec (sets MIST — no separate timer in Gen II)
        0x2c => ItemEffect::ActiveSubStatusSet {
            addr: W_PLAYER_SUB_STATUS_4,
            mask: SUBSTATUS_FOCUS_ENERGY_MASK,
        }, // Dire Hit
        // MiracleBerry — any status byte bit, AND confusion on the active mon
        // (handled in `apply_item`). See `MysteryBerryEffect` / `StatusHealer`.
        0x6d => ItemEffect::Status(0xff),                 // MiracleBerry — any status
        0x7b => ItemEffect::Status(0xff),                 // Heal Powder — any status
        _ => return None,
    })
}

/// Party slots the item would actually affect right now. Empty means the game
/// would refuse it ("It wouldn't have any effect"), so we neither offer the
/// item nor list a target for it. Drives both the action menu and the target
/// prompt.
pub fn item_valid_targets(cpu: &impl ReadMem, id: u8) -> Vec<u8> {
    let Some(effect) = item_effect(id) else {
        return Vec::new();
    };
    let n = cpu.read_byte(W_PARTY_COUNT);
    (0..n)
        .filter(|&slot| match effect {
            // Active-mon items have no party target — they're offered via the
            // active-mon branch in `prompt_action`, never the party-target path.
            ItemEffect::ActiveConfusion
            | ItemEffect::ActiveStatUp(_)
            | ItemEffect::ActiveSubStatusSet { .. } => false,
            // PP isn't a vital, so `affects` can't judge PP items. A PP restore
            // applies to any mon — alive or fainted (RestorePP has no faint
            // guard, so the game lets you Ether/Elixer a fainted mon) — that has
            // at least one move below its max PP.
            ItemEffect::PpAll { .. } | ItemEffect::PpOne { .. } => {
                slot_has_restorable_pp(cpu, slot)
            }
            _ => {
                let base = party_slot_addr(slot);
                let hp = read_word_be(cpu, base + PARTY_OFFSET_HP);
                // `UseStatusHealer` also accepts a heal-everything item (status
                // mask %11111111) on the active mon when it is only confused
                // (`IsItemUsedOnConfusedMon`). Full Restore at full HP falls
                // through to the same routine.
                let cures_active_confusion = matches!(effect, ItemEffect::Status(0xff) | ItemEffect::FullRestore)
                    && hp != 0
                    && slot == cpu.read_byte(W_CUR_BATTLE_MON)
                    && is_active_confused(cpu.read_byte(W_PLAYER_SUB_STATUS_3));
                cures_active_confusion
                    || effect.affects(
                        hp,
                        read_word_be(cpu, base + PARTY_OFFSET_MAX_HP),
                        cpu.read_byte(base + PARTY_OFFSET_STATUS),
                    )
            }
        })
        .collect()
}

/// Does any of `slot`'s 4 move slots hold a move whose current PP is below its
/// max PP? Drives PP-item targeting (the game's "It wouldn't have any effect"
/// guard for Elixer/Max Elixer/Ether/Max Ether/MysteryBerry).
pub fn slot_has_restorable_pp(cpu: &impl ReadMem, slot: u8) -> bool {
    !restorable_move_slots(cpu, slot).is_empty()
}

/// The 0-based move slots of party `slot` that exist (move id != 0) and hold a
/// move below its max PP — i.e. the moves a single-move PP item could restore.
/// Drives the move-selection prompt for Ether/Max Ether/MysteryBerry.
pub fn restorable_move_slots(cpu: &impl ReadMem, slot: u8) -> Vec<u8> {
    let base = party_slot_addr(slot);
    (0..MOVE_SLOTS)
        .filter(|&i| {
            let move_id = cpu.read_byte(base + PARTY_OFFSET_MOVES + i);
            if move_id == 0 {
                return false;
            }
            let pp_byte = cpu.read_byte(base + PARTY_OFFSET_PP + i);
            move_pp::current_pp(pp_byte) < move_pp::max_pp_of(cpu.base_pp(move_id), pp_byte)
        })
        .map(|i| i as u8)
        .collect()
}

/// Whether `id` is a single-move PP item (Ether / Max Ether / MysteryBerry),
/// which needs a move-slot prompt after the target mon is chosen.
pub fn is_single_move_pp_item(id: u8) -> bool {
    matches!(item_effect(id), Some(ItemEffect::PpOne { .. }))
}

/// Whether `id` lives in the Balls pocket (Poké Ball / Great Ball / the Apricorn
/// balls / Master Ball / …). A ball is just an item id from a different pocket,
/// so it's recorded as an `item` step; this lets the dispatch route it to the
/// ball path (decrement the Balls pocket, throw it) rather than the party-target
/// item path. Source: the `Pocket::Ball` rows in [`crate::data::items`].
pub fn is_ball_id(id: u8) -> bool {
    matches!(crate::data::items::lookup_id(id), Some(d) if d.pocket == crate::data::items::Pocket::Ball)
}

/// Whether `id` is an active-mon item (acts on the active battler, takes no
/// party target) AND is usable right now: Bitter Berry only while the active mon
/// is confused; an X-item only while its stat stage is below `MAX_STAT_LEVEL`
/// (mirrors `RaiseStat`'s "wouldn't have any effect" guard at the cap — the asm
/// refuses to raise past 13). Returns `false` for every party-target item.
/// Drives whether the active-mon item appears in the action menu.
pub fn active_item_usable(cpu: &impl ReadMem, id: u8) -> bool {
    match item_effect(id) {
        Some(ItemEffect::ActiveConfusion) => {
            is_active_confused(cpu.read_byte(W_PLAYER_SUB_STATUS_3))
        }
        // `XItemEffect` never refuses: at +6 the item is still used up and the
        // turn spent ("…won't rise anymore!"). So it is always legal.
        Some(ItemEffect::ActiveStatUp(_)) => true,
        // Usable only while the bit is CLEAR — mirrors the asm's
        // `bit mask, [hl] / jp nz, WontHaveAnyEffect` refusal once it's set.
        Some(ItemEffect::ActiveSubStatusSet { addr, mask }) => {
            cpu.read_byte(addr) & mask == 0
        }
        _ => false,
    }
}

pub fn party_slot_addr(slot: u8) -> u16 {
    W_PARTY_MON1 + u16::from(slot) * PARTY_STRUCT_LEN
}

pub fn read_party_species(cpu: &impl ReadMem, slot: u8) -> u8 {
    cpu.read_byte(party_slot_addr(slot))
}

pub fn read_word_be(cpu: &impl ReadMem, addr: u16) -> u16 {
    u16::from_be_bytes([cpu.read_byte(addr), cpu.read_byte(addr + 1)])
}

pub fn read_pocket(cpu: &impl ReadMem, count_addr: u16) -> Vec<(u8, u8)> {
    let n = cpu.read_byte(count_addr) as u16;
    let mut out = Vec::with_capacity(n as usize);
    for i in 0..n {
        let id = cpu.read_byte(count_addr + 1 + i * 2);
        let qty = cpu.read_byte(count_addr + 2 + i * 2);
        out.push((id, qty));
    }
    out
}

pub fn move_options(cpu: &impl ReadMem) -> Vec<u8> {
    // A move disabled by Disable is not selectable, same as the engine's
    // move_selection_screen_use_move `.move_disabled` rejection. wPlayerDisableCount
    // packs the disabled move's 1-based slot in its high nibble (0 = nothing
    // disabled); recover the 0-based slot with the engine's `swap a / and $f / dec a`.
    // When nothing is disabled the high nibble is 0, so the `dec` wraps to 0xff and
    // never matches a real slot.
    let disabled_slot = (cpu.read_byte(W_PLAYER_DISABLE_COUNT) >> 4).wrapping_sub(1);
    let mut out = Vec::with_capacity(defs::NUM_MOVES as usize);
    for slot in 0..defs::NUM_MOVES {
        let move_id = cpu.read_byte(W_BATTLE_MON_MOVES + u16::from(slot));
        if move_id == 0 {
            continue;
        }
        let pp = cpu.read_byte(W_BATTLE_MON_PP + u16::from(slot)) & defs::PP_MASK;
        if pp == 0 {
            continue;
        }
        if slot == disabled_slot {
            continue;
        }
        out.push(slot);
    }
    out
}

/// Why a recorded `move` slot could not have been legally chosen, if any.
/// `None` means the slot is selectable (it's in [`move_options`]). The string
/// is a human-readable reason for the diagnostic.
pub fn illegal_move_reason(cpu: &impl ReadMem, slot: u8) -> Option<String> {
    if move_options(cpu).contains(&slot) {
        return None;
    }
    let move_id = cpu.read_byte(W_BATTLE_MON_MOVES + u16::from(slot));
    let pp = cpu.read_byte(W_BATTLE_MON_PP + u16::from(slot)) & defs::PP_MASK;
    let disabled_slot = (cpu.read_byte(W_PLAYER_DISABLE_COUNT) >> 4).wrapping_sub(1);
    let why = if move_id == 0 {
        "no move in that slot".to_string()
    } else if slot == disabled_slot {
        "Disabled".to_string()
    } else if pp == 0 {
        "no PP left".to_string()
    } else {
        "not selectable".to_string()
    };
    Some(format!("slot {slot} ({}) is {why}", names::move_name(move_id)))
}

/// Every choice the original game's menus allow at decision point `site`, in
/// a stable order. This is the action space the library API exposes.
pub fn legal_steps(cpu: &impl ReadMem, site: PromptSite) -> Vec<Step> {
    match site {
        PromptSite::BattleMenu => legal_battle_menu_steps(cpu),
        PromptSite::MoveSelection => {
            move_options(cpu).into_iter().map(|slot| Step::Move { slot }).collect()
        }
        PromptSite::ForcedSwitch => {
            let current = cpu.read_byte(W_CUR_BATTLE_MON);
            alive_party_slots(cpu, Some(current))
                .into_iter()
                .map(|slot| Step::ForcedSwitch { slot })
                .collect()
        }
        PromptSite::OfferSwitch => {
            let current = cpu.read_byte(W_CUR_BATTLE_MON);
            let mut out = vec![Step::OfferSwitch { keep: true, slot: None }];
            out.extend(
                alive_party_slots(cpu, Some(current))
                    .into_iter()
                    .map(|slot| Step::OfferSwitch { keep: false, slot: Some(slot) }),
            );
            out
        }
        PromptSite::EnemyAction => {
            let enemy_fainted = read_word_be(cpu, W_ENEMY_MON_HP) == 0;
            let current = cpu.read_byte(W_CUR_OT_MON);
            let mut out: Vec<Step> = if enemy_fainted {
                Vec::new()
            } else {
                enemy_move_options(cpu).into_iter().map(|slot| Step::Move { slot }).collect()
            };
            out.extend(
                enemy_alive_party_slots(cpu, Some(current))
                    .into_iter()
                    .map(|slot| Step::Switch { slot }),
            );
            out
        }
        PromptSite::EnemyTrainerAction => legal_enemy_trainer_steps(cpu),
        PromptSite::EnemySwitchIn => {
            let current = cpu.read_byte(W_CUR_OT_MON);
            enemy_alive_party_slots(cpu, Some(current)).into_iter().map(|slot| Step::Switch { slot }).collect()
        }
    }
}

/// The items an externally controlled trainer can use: the `AI_Items` table
/// (`engine/battle/ai/items.asm`), each used on the active mon.
pub const ENEMY_TRAINER_ITEMS: [u8; 13] = [
    defs::FULL_RESTORE,
    defs::MAX_POTION,
    defs::HYPER_POTION,
    defs::SUPER_POTION,
    defs::POTION,
    defs::X_ACCURACY,
    defs::FULL_HEAL,
    defs::GUARD_SPEC,
    defs::DIRE_HIT,
    defs::X_ATTACK,
    defs::X_DEFEND,
    defs::X_SPEED,
    defs::X_SPECIAL,
];

/// Whether an externally controlled trainer's action is forced this turn, the
/// way `CheckEnemyLockedIn` forces it (recharging, a charged / rampaging /
/// Bide move, Rollout). No [`PromptSite::EnemyTrainerAction`] is asked then.
pub fn enemy_locked_in(cpu: &impl ReadMem) -> bool {
    if cpu.read_byte(W_ENEMY_SUB_STATUS_4) & 1 << defs::SUBSTATUS_RECHARGE != 0 {
        return true;
    }
    let sub3 = cpu.read_byte(W_ENEMY_SUB_STATUS_3);
    if sub3 & (1 << defs::SUBSTATUS_CHARGED | 1 << defs::SUBSTATUS_RAMPAGE | 1 << defs::SUBSTATUS_BIDE) != 0 {
        return true;
    }
    cpu.read_byte(crate::engine::sym::W_ENEMY_SUB_STATUS1) & 1 << defs::SUBSTATUS_ROLLOUT != 0
}

/// [`PromptSite::EnemyTrainerAction`]: FIGHT when Encored or out of usable
/// moves (the engine then picks the Encored move or Struggle), else each
/// usable move; a switch to every other alive mon unless trapped (the
/// player's Mean Look / Spider Web, or a binding move, the same checks as
/// `AI_SwitchOrTryItem`); and each AI item the trainer still holds.
pub fn legal_enemy_trainer_steps(cpu: &impl ReadMem) -> Vec<Step> {
    let moves = enemy_move_options(cpu);
    let mut out: Vec<Step> = if moves.is_empty() || cpu.read_byte(W_ENEMY_SUB_STATUS_5) & 1 << defs::SUBSTATUS_ENCORED != 0 {
        vec![Step::Fight]
    } else {
        moves.into_iter().map(|slot| Step::Move { slot }).collect()
    };
    let trapped = cpu.read_byte(W_PLAYER_SUB_STATUS_5) & 1 << defs::SUBSTATUS_CANT_RUN != 0
        || cpu.read_byte(W_ENEMY_WRAP_COUNT) != 0;
    if !trapped {
        let current = cpu.read_byte(W_CUR_OT_MON);
        out.extend(enemy_alive_party_slots(cpu, Some(current)).into_iter().map(|slot| Step::Switch { slot }));
    }
    for i in 0..2 {
        let id = cpu.read_byte(W_ENEMY_TRAINER_ITEM1 + i);
        let step = Step::Item { id, target: None, move_slot: None };
        if ENEMY_TRAINER_ITEMS.contains(&id) && !out.contains(&step) {
            out.push(step);
        }
    }
    out
}

pub fn legal_battle_menu_steps(cpu: &impl ReadMem) -> Vec<Step> {
    let mut out = Vec::new();
    // FIGHT.
    if fight_skips_move_menu(cpu) || player_must_struggle(cpu) {
        out.push(Step::Fight);
    } else {
        out.extend(move_options(cpu).into_iter().map(|slot| Step::Move { slot }));
    }
    // PKMN.
    let current = cpu.read_byte(W_CUR_BATTLE_MON);
    out.extend(
        alive_party_slots(cpu, Some(current))
            .into_iter()
            .filter(|&slot| check_can_switch(cpu, slot).is_ok())
            .map(|slot| Step::Switch { slot }),
    );
    // PACK — every bag item whose real effect would succeed (and so use the
    // turn), expanded per target / move. `BattleMenu_Pack` refuses all items
    // in a link battle ("Items can't be used here").
    let items_allowed = cpu.read_byte(W_LINK_MODE) == 0;
    for (id, _qty) in read_pocket(cpu, W_NUM_ITEMS).into_iter().filter(|_| items_allowed) {
        if active_item_usable(cpu, id) {
            out.push(Step::Item { id, target: None, move_slot: None });
            continue;
        }
        for target in item_valid_targets(cpu, id) {
            if is_single_move_pp_item(id) {
                for m in restorable_move_slots(cpu, target) {
                    out.push(Step::Item { id, target: Some(target), move_slot: Some(m) });
                }
            } else {
                out.push(Step::Item { id, target: Some(target), move_slot: None });
            }
        }
    }
    // Balls work in every battle: in a trainer battle `UseBallInTrainerBattle`
    // still tosses the ball and uses the turn ("The trainer blocked the BALL!").
    // (A full party AND a full box would refuse it; a harness battle starts
    // with an empty box and can catch at most one mon.)
    if items_allowed {
        for (id, _qty) in read_pocket(cpu, W_NUM_BALLS) {
            out.push(Step::Item { id, target: None, move_slot: None });
        }
    }
    if items_allowed && offers_escape_item(cpu.read_byte(W_BATTLE_MODE)) {
        for (id, _qty) in read_pocket(cpu, W_NUM_ITEMS) {
            if is_escape_item_id(id) {
                out.push(Step::Item { id, target: None, move_slot: None });
            }
        }
    }
    // RUN.
    out.push(Step::Run);
    out
}

/// FIGHT goes straight to a move without a menu: `ParsePlayerAction` checks
/// Encore (`SUBSTATUS_ENCORED` → repeat `wLastPlayerMove`) and then Bide
/// (`SUBSTATUS_BIDE` → `.locked_in`) before it would call `MoveSelectionScreen`.
pub fn fight_skips_move_menu(cpu: &impl ReadMem) -> bool {
    cpu.read_byte(W_PLAYER_SUB_STATUS_5) & (1 << defs::SUBSTATUS_ENCORED) != 0
        || cpu.read_byte(W_PLAYER_SUB_STATUS_3) & (1 << defs::SUBSTATUS_BIDE) != 0
}

/// Whether `CheckPlayerHasUsableMoves` would force Struggle. A read-only copy
/// of its predicate, including its bug: with a move Disabled, the remaining PP
/// bytes are OR'd *without* masking off the PP-Up bits, so a 0-PP move with PP
/// Ups counts as usable (see `docs/bugs_and_glitches.md` in pokecrystal).
pub fn player_must_struggle(cpu: &impl ReadMem) -> bool {
    let pp = |i: u16| cpu.read_byte(W_BATTLE_MON_PP + i);
    let disable = cpu.read_byte(W_PLAYER_DISABLE_COUNT);
    if disable == 0 {
        return (pp(0) | pp(1) | pp(2) | pp(3)) & defs::PP_MASK == 0;
    }
    let disabled = u16::from(disable >> 4); // 1-based slot
    let mut acc = 0u8;
    for i in 0..MOVE_SLOTS {
        if i + 1 != disabled {
            acc |= pp(i);
        }
    }
    acc == 0
}

pub fn alive_party_slots(cpu: &impl ReadMem, exclude: Option<u8>) -> Vec<u8> {
    let n = cpu.read_byte(W_PARTY_COUNT);
    let mut out = Vec::with_capacity(n as usize);
    for slot in 0..n {
        if Some(slot) == exclude {
            continue;
        }
        let hp = read_word_be(cpu, party_slot_addr(slot) + PARTY_OFFSET_HP);
        if hp == 0 {
            continue;
        }
        out.push(slot);
    }
    out
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::mem::Mem;

    fn write_word_be(cpu: &mut Mem, addr: u16, value: u16) {
        cpu.wb(addr, (value >> 8) as u8);
        cpu.wb(addr + 1, value as u8);
    }

    fn effect(id: u8) -> ItemEffect {
        item_effect(id).expect("item should be classified")
    }

    #[test]
    fn hp_items_need_a_hurt_living_mon() {
        let potion = effect(0x12);
        assert!(!potion.affects(20, 20, 0), "full HP — no effect");
        assert!(potion.affects(10, 20, 0), "below max — heals");
        assert!(!potion.affects(0, 20, 0), "fainted — Potion can't revive");
    }

    #[test]
    fn berries_and_treats_restore_hp() {
        // Berry, Gold Berry, Berry Juice, RageCandyBar all classify as HP heals.
        for id in [0xad, 0xae, 0x8b, 0x72] {
            assert!(matches!(effect(id), ItemEffect::Hp), "item {id:#x} heals HP");
            assert!(effect(id).affects(10, 20, 0), "below max — heals");
            assert!(!effect(id).affects(20, 20, 0), "full HP — no effect");
        }
    }

    #[test]
    fn revives_need_a_fainted_mon() {
        let revive = effect(0x27);
        assert!(revive.affects(0, 20, 0), "fainted — revives");
        assert!(!revive.affects(10, 20, 0), "alive — no effect");
        assert!(!revive.affects(20, 20, 0), "full HP — no effect");
    }

    #[test]
    fn revival_herb_revives_a_fainted_mon() {
        let herb = effect(0x7c);
        assert!(matches!(herb, ItemEffect::Revive), "Revival Herb revives");
        assert!(herb.affects(0, 20, 0), "fainted — revives");
        assert!(!herb.affects(10, 20, 0), "alive — no effect");
        assert!(!herb.affects(20, 20, 0), "full HP — no effect");
    }

    #[test]
    fn status_heals_match_their_condition() {
        let antidote = effect(0x09);
        let psn = 1 << defs::PSN as u8;
        let par = 1 << defs::PAR as u8;
        assert!(antidote.affects(20, 20, psn), "poisoned — cures even at full HP");
        assert!(!antidote.affects(20, 20, par), "paralyzed only — Antidote does nothing");
        assert!(!antidote.affects(20, 20, 0), "no status — no effect");
        assert!(!antidote.affects(0, 20, psn), "fainted — no effect");

        let full_heal = effect(0x26);
        assert!(full_heal.affects(20, 20, par), "any status — cures");
        assert!(!full_heal.affects(20, 20, 0), "no status — no effect");
    }

    #[test]
    fn curing_berries_match_their_status() {
        let psn = 1 << defs::PSN as u8;
        let par = 1 << defs::PAR as u8;
        let frz = 1 << defs::FRZ as u8;
        let brn = 1 << defs::BRN as u8;
        let slp = defs::SLP_MASK;

        // Each single-status berry cures its own status and nothing else.
        for (id, own, other) in [
            (0x4a, psn, par), // PSNCureBerry — poison
            (0x4e, par, psn), // PRZCureBerry — paralysis
            (0x4f, frz, brn), // Burnt Berry — freeze
            (0x50, brn, frz), // Ice Berry — burn
            (0x54, slp, psn), // Mint Berry — sleep
        ] {
            assert!(effect(id).affects(20, 20, own), "item {id:#x} cures its status");
            assert!(!effect(id).affects(20, 20, other), "item {id:#x} ignores others");
        }

        // MiracleBerry and Heal Powder clear any status.
        for id in [0x6d, 0x7b] {
            assert!(effect(id).affects(20, 20, par), "item {id:#x} cures any status");
            assert!(!effect(id).affects(20, 20, 0), "item {id:#x} no status — no effect");
        }
    }

    #[test]
    fn full_restore_covers_hp_or_status() {
        let fr = effect(0x0e);
        let brn = 1 << defs::BRN as u8;
        assert!(fr.affects(10, 20, 0), "hurt only — works");
        assert!(fr.affects(20, 20, brn), "statused only — works");
        assert!(!fr.affects(20, 20, 0), "full HP and clean — no effect");
        assert!(!fr.affects(0, 20, 0), "fainted — no effect");
    }

    #[test]
    fn unsupported_items_are_unclassified() {
        assert!(item_effect(0x01).is_none(), "Master Ball");
        assert!(item_effect(0x20).is_none(), "Rare Candy");
        assert!(item_effect(0x32).is_none(), "ITEM_32 (no in-battle effect)");
    }

    #[test]
    fn pp_items_classify_as_pp_all() {
        // Elixer adds 10 PP to every move.
        assert!(
            matches!(effect(0x41), ItemEffect::PpAll { amount: Some(10) }),
            "Elixer adds 10 PP to all moves",
        );
        // Max Elixer refills every move (no fixed amount).
        assert!(
            matches!(effect(0x15), ItemEffect::PpAll { amount: None }),
            "Max Elixer fully restores all moves",
        );
    }

    #[test]
    fn single_move_pp_items_classify_as_pp_one() {
        // Ether adds 10 PP to one move.
        assert!(
            matches!(effect(0x3f), ItemEffect::PpOne { amount: Some(10) }),
            "Ether adds 10 PP to one move",
        );
        // Max Ether refills one move (no fixed amount).
        assert!(
            matches!(effect(0x40), ItemEffect::PpOne { amount: None }),
            "Max Ether fully restores one move",
        );
        // MysteryBerry adds 5 PP to one move (asm `ld c, 5`).
        assert!(
            matches!(effect(0x96), ItemEffect::PpOne { amount: Some(5) }),
            "MysteryBerry adds 5 PP to one move",
        );
        // All three are single-move PP items needing a move-slot prompt.
        for id in [0x3f, 0x40, 0x96] {
            assert!(is_single_move_pp_item(id), "item {id:#x} is single-move PP");
        }
        // Whole-party PP items (Elixer / Max Elixer) and HP items are NOT.
        for id in [0x41, 0x15, 0x12] {
            assert!(!is_single_move_pp_item(id), "item {id:#x} is not single-move PP");
        }
    }

    /// The single-move restore math mirrors `RestorePP` (`.restore_some` /
    /// `.restore_all`): `new = (orig & PP_UP_MASK) | min(cur + n, max)`, with
    /// `amount: None` (Max Ether) restoring to full and PP-Up bits preserved.
    #[test]
    fn single_move_restore_amount_and_cap() {
        let id = 1u8; // POUND, base 35
        let restore = |pp_byte: u8, amount: Option<u8>| {
            let max = move_pp::max_pp(id, pp_byte);
            let new = match amount {
                Some(n) => move_pp::current_pp(pp_byte).saturating_add(n).min(max),
                None => max,
            };
            move_pp::pp_up_bits(pp_byte) | new
        };
        // Ether: +10, below cap.
        assert_eq!(restore(20, Some(10)), 30);
        // Ether: +10 capped at max (35).
        assert_eq!(restore(30, Some(10)), 35);
        // MysteryBerry: +5, below cap.
        assert_eq!(restore(20, Some(5)), 25);
        // MysteryBerry: +5 capped at max.
        assert_eq!(restore(33, Some(5)), 35);
        // Max Ether: restore to full, no PP-Ups.
        assert_eq!(restore(0, None), 35);
        // PP-Up bits preserved, and the higher max is respected.
        let with_ups = 0x80 | 30; // 2 PP-Ups → max 49 for base 35
        assert_eq!(move_pp::max_pp(id, with_ups), 49);
        assert_eq!(restore(with_ups, Some(10)), 0x80 | 40);
        assert_eq!(restore(with_ups, None), 0x80 | 49);
    }

    #[test]
    fn bitter_berry_is_an_active_confusion_item() {
        // Bitter Berry (0x53) clears confusion on the active mon.
        assert!(
            matches!(effect(0x53), ItemEffect::ActiveConfusion),
            "Bitter Berry is an active-mon confusion item",
        );
        // It's never judged by vitals and never offered as a party-target item.
        assert!(!effect(0x53).affects(20, 20, 0), "vitals never claim an effect");
        assert!(!effect(0x53).affects(0, 20, 0), "vitals never claim an effect");
        // It is NOT a single-move PP item (no move-slot prompt).
        assert!(!is_single_move_pp_item(0x53), "Bitter Berry has no move-slot prompt");
    }

    /// X Attack/Defend/Speed/Special map to the stat indices in
    /// `data/items/x_stats.asm` (`XItemStats`): X_ATTACK→ATTACK(0),
    /// X_DEFEND→DEFENSE(1), X_SPEED→SPEED(2), X_SPECIAL→SP_ATTACK(3).
    #[test]
    fn x_items_map_to_their_stat_index() {
        for (id, stat) in [
            (0x31u8, STAT_ATTACK),    // X Attack → ATTACK
            (0x33, STAT_DEFENSE),     // X Defend → DEFENSE
            (0x34, STAT_SPEED),       // X Speed → SPEED
            (0x35, STAT_SP_ATTACK),   // X Special → SP_ATTACK
        ] {
            assert!(
                matches!(effect(id), ItemEffect::ActiveStatUp(s) if s == stat),
                "item {id:#x} raises stat index {stat}",
            );
        }
        // The stat indices match the `const_def` block (ATTACK=0..SP_ATTACK=3),
        // and neutral is `BASE_STAT_LEVEL`.
        assert_eq!((STAT_ATTACK, STAT_DEFENSE, STAT_SPEED, STAT_SP_ATTACK), (0, 1, 2, 3));
        // X-items never go through the party-target / move-slot machinery.
        for id in [0x31u8, 0x33, 0x34, 0x35] {
            assert!(!effect(id).affects(20, 20, 0), "vitals never claim an effect");
            assert!(!is_single_move_pp_item(id), "X-item has no move-slot prompt");
        }
    }

    /// X Accuracy / Guard Spec / Dire Hit each set one bit of `wPlayerSubStatus4`
    /// on the active mon. Item→(addr, mask) mapping from `XAccuracyEffect` /
    /// `GuardSpecEffect` / `DireHitEffect` (`engine/items/item_effects.asm`) and
    /// the SubStatus4 `const_def` (X_ACCURACY=0, MIST=1, FOCUS_ENERGY=2).
    #[test]
    fn substatus_set_items_map_to_addr_and_mask() {
        // The masks are the first three SubStatus4 bits.
        assert_eq!(SUBSTATUS_X_ACCURACY_MASK, 0x01);
        assert_eq!(SUBSTATUS_MIST_MASK, 0x02);
        assert_eq!(SUBSTATUS_FOCUS_ENERGY_MASK, 0x04);
        for (id, mask) in [
            (0x21u8, SUBSTATUS_X_ACCURACY_MASK),   // X Accuracy → X_ACCURACY
            (0x29, SUBSTATUS_MIST_MASK),           // Guard Spec → MIST
            (0x2c, SUBSTATUS_FOCUS_ENERGY_MASK),   // Dire Hit → FOCUS_ENERGY
        ] {
            assert!(
                matches!(
                    effect(id),
                    ItemEffect::ActiveSubStatusSet { addr, mask: m }
                        if addr == W_PLAYER_SUB_STATUS_4 && m == mask
                ),
                "item {id:#x} sets bit {mask:#x} of wPlayerSubStatus4",
            );
            // Never judged by vitals, never a single-move PP item.
            assert!(!effect(id).affects(20, 20, 0), "vitals never claim an effect");
            assert!(!effect(id).affects(0, 20, 0), "vitals never claim an effect");
            assert!(!is_single_move_pp_item(id), "no move-slot prompt");
        }
    }

    /// "Usable iff the bit is clear" and "applying ORs in the bit, preserving the
    /// others" — the two halves of `bit`/`set mask, [wPlayerSubStatus4]`. Tested
    /// against the masks directly (the same predicate `active_item_usable` and
    /// `apply_item` use) so no `Cpu` is needed.
    #[test]
    fn substatus_set_usable_iff_clear_and_apply_preserves_others() {
        let usable = |byte: u8, mask: u8| byte & mask == 0;
        let apply = |byte: u8, mask: u8| byte | mask;
        for mask in [
            SUBSTATUS_X_ACCURACY_MASK,
            SUBSTATUS_MIST_MASK,
            SUBSTATUS_FOCUS_ENERGY_MASK,
        ] {
            // Clear → usable; once set → refused (hidden).
            assert!(usable(0x00, mask), "bit clear — usable");
            assert!(!usable(mask, mask), "bit already set — refused");
            assert!(!usable(0xff, mask), "all bits set — refused");
            // Applying sets the bit and leaves every OTHER bit untouched.
            let other = 0xffu8 & !mask; // all the bits this item must not touch
            assert_eq!(apply(other, mask), 0xff, "OR sets our bit, keeps the rest");
            assert_eq!(apply(0x00, mask), mask, "from clear, only our bit is set");
            // Re-applying when already set is a no-op (idempotent OR).
            assert_eq!(apply(mask, mask), mask, "already set — unchanged");
        }
        // The three masks are distinct bits, so the items never interfere.
        assert_eq!(
            SUBSTATUS_X_ACCURACY_MASK | SUBSTATUS_MIST_MASK | SUBSTATUS_FOCUS_ENERGY_MASK,
            0x07,
        );
    }

    /// `SUBSTATUS_CONFUSED` is bit 7 of `wPlayerSubStatus3`; the helper reports
    /// confusion iff that bit is set, ignoring the other SubStatus3 bits.
    #[test]
    fn confusion_mask_is_bit_7() {
        assert_eq!(SUBSTATUS_CONFUSED_MASK, 0x80);
        assert!(is_active_confused(0x80), "bit 7 set — confused");
        assert!(is_active_confused(0xff), "all bits set — confused");
        assert!(!is_active_confused(0x00), "no bits — not confused");
        // Every other SubStatus3 bit (BIDE..FLYING = bits 0..6) without bit 7
        // must NOT read as confused.
        for bit in 0..7u8 {
            assert!(
                !is_active_confused(1 << bit),
                "bit {bit} alone is not confusion",
            );
        }
        // Clearing bit 7 mirrors `res SUBSTATUS_CONFUSED, [hl]` and leaves the
        // other bits intact.
        let with_other = 0x80 | (1 << 6); // CONFUSED + FLYING
        assert_eq!(with_other & !SUBSTATUS_CONFUSED_MASK, 1 << 6);
    }

    /// The Poké Doll (a guaranteed escape) is offered only in a wild battle, the
    /// same gate as a ball: `PokeDollEffect` is a no-op in a trainer battle. And
    /// `is_escape_item_id` recognizes exactly the Poké Doll id — it is NOT a
    /// classified `item_effect` (so the party-target item loop skips it) and NOT
    /// a ball, so the two routing predicates never overlap.
    #[test]
    fn poke_doll_offered_only_in_wild_battles() {
        assert_eq!(POKE_DOLL_ID, 0x25);
        assert!(offers_escape_item(defs::WILD_BATTLE), "wild — Poké Doll offered");
        assert!(!offers_escape_item(defs::TRAINER_BATTLE), "trainer — Poké Doll is a no-op");
        assert!(!offers_escape_item(0), "not in battle — no Poké Doll");

        assert!(is_escape_item_id(POKE_DOLL_ID), "the Poké Doll is the escape item");
        // It's neither a classified party-target item nor a ball, so it routes
        // only through the dedicated escape branch.
        assert!(item_effect(POKE_DOLL_ID).is_none(), "Poké Doll has no party-heal effect");
        assert!(!is_ball_id(POKE_DOLL_ID), "Poké Doll is not a ball");
        // A Potion is not an escape item.
        assert!(!is_escape_item_id(0x12), "Potion is not the escape item");
    }

    /// `is_ball_id` is true for exactly the Balls-pocket ids and false for
    /// Items-pocket ids (and unknown ids), so a ball routes to the ball path
    /// while a normal item routes to `apply_item`.
    #[test]
    fn ball_ids_are_recognized() {
        // Poké Ball, Great Ball, Ultra Ball, Master Ball, an Apricorn ball.
        for id in [0x05, 0x04, 0x02, 0x01, 0xa0] {
            assert!(is_ball_id(id), "item {id:#x} is a ball");
        }
        // Potion, Full Restore, X Attack, Ether — Items pocket, not balls.
        for id in [0x12, 0x0e, 0x31, 0x3f] {
            assert!(!is_ball_id(id), "item {id:#x} is not a ball");
        }
        // An id our table doesn't know isn't a ball.
        assert!(!is_ball_id(0x00), "unknown id is not a ball");
    }

    #[test]
    fn pp_all_is_not_judged_by_vitals() {
        // PP isn't a vital, so the HP/status `affects` guard never claims a PP
        // item has an effect — `item_valid_targets` decides via per-move PP.
        // (A full-HP, unstatused mon must still be a valid Elixer target if its
        // moves are below max PP.)
        let elixer = effect(0x41);
        assert!(!elixer.affects(20, 20, 0), "full HP/clean — vitals say no");
        assert!(!elixer.affects(0, 20, 0), "fainted — vitals say no");
        assert!(!elixer.affects(10, 20, 1), "hurt+statused — vitals still say no");

        // Single-move PP items are judged the same way: vitals never claim an
        // effect, and a fainted mon is still a valid target (no faint guard).
        let ether = effect(0x3f);
        assert!(!ether.affects(20, 20, 0), "full HP/clean — vitals say no");
        assert!(!ether.affects(0, 20, 0), "fainted — vitals say no");
    }

    #[test]
    fn disabled_move_is_not_selectable() {
        let mut cpu = Mem::default();
        // Four real moves, each with PP to spare.
        for slot in 0..4u16 {
            cpu.wb(W_BATTLE_MON_MOVES + slot, slot as u8 + 1);
            cpu.wb(W_BATTLE_MON_PP + slot, 10);
        }

        // Nothing disabled: all four moves are selectable.
        cpu.wb(W_PLAYER_DISABLE_COUNT, 0);
        assert_eq!(move_options(&cpu), vec![0, 1, 2, 3]);

        // Disable the move in slot 1 — high nibble = its 1-based slot (2), low
        // nibble = turns remaining (5). It drops out of the selectable list,
        // matching the engine's `.move_disabled` rejection.
        cpu.wb(W_PLAYER_DISABLE_COUNT, 0x25);
        assert_eq!(move_options(&cpu), vec![0, 2, 3]);

        // Counter cleared (Disable expired): the move is selectable again.
        cpu.wb(W_PLAYER_DISABLE_COUNT, 0);
        assert_eq!(move_options(&cpu), vec![0, 1, 2, 3]);
    }

    /// The enemy move menu (link battles) mirrors the player's: a real move
    /// (id != 0) with PP that isn't Disabled. Same logic as
    /// `disabled_move_is_not_selectable`, against the `wEnemyMon*` fields and
    /// `wEnemyDisableCount`.
    #[test]
    fn enemy_move_options_filter_pp_and_disable() {
        let mut cpu = Mem::default();
        // slot 0: real move, PP. slot 1: real move, 0 PP. slot 2: real move, PP
        // (we'll Disable it). slot 3: empty.
        cpu.wb(W_ENEMY_MON_MOVES + 0, 10);
        cpu.wb(W_ENEMY_MON_PP + 0, 5);
        cpu.wb(W_ENEMY_MON_MOVES + 1, 20);
        cpu.wb(W_ENEMY_MON_PP + 1, 0);
        cpu.wb(W_ENEMY_MON_MOVES + 2, 30);
        cpu.wb(W_ENEMY_MON_PP + 2, 8);
        cpu.wb(W_ENEMY_MON_MOVES + 3, 0);
        cpu.wb(W_ENEMY_DISABLE_COUNT, 0); // nothing disabled
        assert_eq!(enemy_move_options(&cpu), vec![0, 2]); // slot 1 has no PP

        // Disable slot 2 (high nibble = its 1-based slot 3): drops out.
        cpu.wb(W_ENEMY_DISABLE_COUNT, 0x34);
        assert_eq!(enemy_move_options(&cpu), vec![0]);
    }

    /// The enemy switch list (link battles) is the alive OT party slots minus
    /// the active one — so a single-mon enemy offers no switch.
    #[test]
    fn enemy_alive_party_slots_exclude_active_and_fainted() {
        let mut cpu = Mem::default();
        cpu.wb(W_OT_PARTY_COUNT, 3);
        // slot 0 active+alive, slot 1 alive, slot 2 fainted.
        write_word_be(&mut cpu, ot_party_slot_addr(0) + PARTY_OFFSET_HP, 20);
        write_word_be(&mut cpu, ot_party_slot_addr(1) + PARTY_OFFSET_HP, 15);
        write_word_be(&mut cpu, ot_party_slot_addr(2) + PARTY_OFFSET_HP, 0);
        assert_eq!(enemy_alive_party_slots(&cpu, Some(0)), vec![1]);
        // Single-mon enemy: nothing to switch to.
        cpu.wb(W_OT_PARTY_COUNT, 1);
        assert_eq!(enemy_alive_party_slots(&cpu, Some(0)), Vec::<u8>::new());
    }

    /// The action byte the enemy hook writes to `wBattleAction`: a move slot is
    /// the slot index (BATTLEACTION_MOVE1 == 0), a switch is
    /// `BATTLEACTION_SWITCH1 + slot`. These are what `parse_enemy_action`'s link
    /// path decodes.
    #[test]
    fn enemy_action_byte_encoding() {
        assert_eq!(defs::BATTLEACTION_MOVE1, 0);
        assert_eq!(defs::BATTLEACTION_MOVE1 + 3, 3); // move slot 3
        assert_eq!(defs::BATTLEACTION_SWITCH1, 4);
        assert_eq!(defs::BATTLEACTION_SWITCH1 + 0, 4); // switch to slot 0
        assert_eq!(defs::BATTLEACTION_SWITCH1 + 5, 9); // switch to slot 5
        // The two ranges never overlap (4 move slots, then switches).
        assert!(defs::BATTLEACTION_MOVE1 + (defs::NUM_MOVES as u8 - 1) < defs::BATTLEACTION_SWITCH1);
    }

    #[test]
    fn illegal_move_reason_flags_unselectable_slots() {
        let mut cpu = Mem::default();
        // slot 0: real move, full PP. slot 1: real move, 0 PP. slot 2: real
        // move, full PP (we'll Disable it). slot 3: empty.
        cpu.wb(W_BATTLE_MON_MOVES + 0, 1);
        cpu.wb(W_BATTLE_MON_PP + 0, 10);
        cpu.wb(W_BATTLE_MON_MOVES + 1, 2);
        cpu.wb(W_BATTLE_MON_PP + 1, 0);
        cpu.wb(W_BATTLE_MON_MOVES + 2, 3);
        cpu.wb(W_BATTLE_MON_PP + 2, 10);
        cpu.wb(W_BATTLE_MON_MOVES + 3, 0);
        // Disable slot 2 (1-based slot 3 in the high nibble).
        cpu.wb(W_PLAYER_DISABLE_COUNT, 0x34);

        assert_eq!(illegal_move_reason(&cpu, 0), None, "real move with PP is legal");
        assert!(illegal_move_reason(&cpu, 1).unwrap().contains("no PP left"));
        assert!(illegal_move_reason(&cpu, 2).unwrap().contains("Disabled"));
        assert!(illegal_move_reason(&cpu, 3).unwrap().contains("no move in that slot"));
    }
}
