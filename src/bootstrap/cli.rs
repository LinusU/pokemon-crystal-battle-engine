#![warn(dead_code, unused_imports, unused_variables)]
//! Terminal-driven battle input. Replaces the engine's keypad menus with stdin
//! prompts that read WRAM directly. Always on — see the unconditional
//! `HARNESS-PATCH` sites listed below.
//!
//! The HARNESS-PATCH sites that call into this module are:
//!
//! - `battle_menu` → [`handle_battle_menu`]
//! - `move_selection_screen` → [`handle_move_selection`]
//! - `force_pick_party_mon_in_battle` → [`handle_forced_switch`]
//! - `offer_switch` → [`handle_offer_switch`]
//!
//! `ask_use_next_pokemon` and `joy_wait_aor_b` are short-circuited inline; they
//! don't reach this module.
//!
//! The module operates entirely on WRAM, with no API surface beyond `&mut Cpu`.
//! Long-term this is the shape a `step(action) -> Outcome` library API will
//! sit on top of — see [docs/roadmap.md](../../docs/roadmap.md).

use std::io::{BufRead, Write};

use crate::defs;
use crate::runtime::{Cpu, ReadMem};

use super::move_pp;
use super::names;
use super::player::{self, PromptSite, StepKind};
use super::recorder::{self, Side, Step};

// ---------------------------------------------------------------------------
// WRAM addresses. Source of truth: scratch/pokecrystal/pokecrystal.sym.
// Mirrors docs/memory_layout.md.

const W_BATTLE_MON_SPECIES: u16 = 0xc62c;
const W_BATTLE_MON_ITEM: u16 = 0xc62d; // held item id (`battle_struct` +1)
const W_BATTLE_MON_MOVES: u16 = 0xc62e;
const W_BATTLE_MON_PP: u16 = 0xc634;
const W_BATTLE_MON_LEVEL: u16 = 0xc639;
const W_BATTLE_MON_STATUS: u16 = 0xc63a;
const W_BATTLE_MON_HP: u16 = 0xc63c;
const W_BATTLE_MON_MAX_HP: u16 = 0xc63e;

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
// Bag-UI state the item hooks fill in. `00:cf63 wJumptableIndex`,
// `00:cf65 wCurPocket`, `01:d0d8 wPartyMenuCursor`, `01:d108
// wCurPartySpecies`, `01:dcd8 wPartySpecies` in `pokecrystal.sym`.
const W_JUMPTABLE_INDEX: u16 = 0xcf63;
const W_CUR_POCKET: u16 = 0xcf65;
const W_PARTY_MENU_CURSOR: u16 = 0xd0d8;
const W_CUR_PARTY_SPECIES: u16 = 0xd108;
const W_PARTY_SPECIES: u16 = 0xdcd8;
// The active player mon's Disable counter. High nibble = the disabled move's
// 1-based slot (0 = nothing disabled); low nibble = turns remaining.
// `00:c675 wPlayerDisableCount` in `pokecrystal.sym`.
const W_PLAYER_DISABLE_COUNT: u16 = 0xc675;

const STAT_ATTACK: u8 = 0;
const STAT_DEFENSE: u8 = 1;
const STAT_SPEED: u8 = 2;
const STAT_SP_ATTACK: u8 = 3;
const W_LAST_PLAYER_MON: u16 = 0xc71a;
const W_PLAYER_WRAP_COUNT: u16 = 0xc730;

const W_BATTLE_MENU_CURSOR_POSITION: u16 = 0xd0d2;
const W_CUR_BATTLE_MON: u16 = 0xd0d4;
const W_MENU_CURSOR_Y: u16 = 0xcfa9;
const W_BATTLE_PLAYER_ACTION: u16 = 0xd0ec;
const W_CUR_ITEM: u16 = 0xd106;
const W_CUR_PARTY_MON: u16 = 0xd109;

const W_PARTY_COUNT: u16 = 0xdcd7;
const W_PARTY_MON1: u16 = 0xdcdf;
const PARTY_STRUCT_LEN: u16 = 48;

const W_NUM_ITEMS: u16 = 0xd892;
const W_NUM_BALLS: u16 = 0xd8d7;

// Capture-path WRAM. Source: scratch/pokecrystal/pokecrystal.sym.
// `wWildMon` (0xc64e): non-zero (the caught species) after a successful catch,
// zero after a failed throw — this is what `BattleMenu_Pack.UseItem` keys on to
// decide caught-vs-broke-free, and what we key on too.
const W_WILD_MON: u16 = 0xc64e;

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

const W_ENEMY_MON_SPECIES: u16 = 0xd206;
const W_ENEMY_MON_ITEM: u16 = 0xd207; // held item id (`battle_struct` +1)
const W_ENEMY_MON_MOVES: u16 = 0xd208;
const W_ENEMY_MON_PP: u16 = 0xd20e;
const W_ENEMY_MON_LEVEL: u16 = 0xd213;
const W_ENEMY_MON_HP: u16 = 0xd216;
const W_ENEMY_MON_MAX_HP: u16 = 0xd218;
const W_ENEMY_MON_STATUS: u16 = 0xd214;
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
// `wBattleAction` (0xd430): the enemy's action for the round, where a serial
// link partner would have written it. The link path's `parse_enemy_action`
// reads it as a move slot (0..3 = BATTLEACTION_MOVE1..4), `BATTLEACTION_SWITCH1
// + slot` for a switch, or `BATTLEACTION_STRUGGLE`. The link-battle CLI
// (`handle_enemy_action`) fills it in place of the cable.
const W_BATTLE_ACTION: u16 = 0xd430;

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
fn is_active_confused(sub_status3: u8) -> bool {
    sub_status3 & SUBSTATUS_CONFUSED_MASK != 0
}

// ---------------------------------------------------------------------------

/// The action the player picked from the CLI menu. Used internally; the patch
/// at `battle_menu` calls [`handle_battle_menu`] which dispatches it.
#[derive(Debug, Clone, Copy)]
enum Action {
    Move(u8),
    /// FIGHT with no move menu (Encore / Bide / Struggle). See [`Step::Fight`].
    Fight,
    Switch(u8),
    Item { id: u8, target: Option<u8>, move_slot: Option<u8> },
    Run,
}

/// Whether the action menu should offer the Poké Doll (a guaranteed escape),
/// given `wBattleMode`. Like a ball, it only does anything in a wild battle
/// (`WILD_BATTLE` = 1): `PokeDollEffect` is a no-op in a trainer battle (it sets
/// `wItemEffectSucceeded = 0` and returns), so we never list it there. Pure
/// helper so the mode test is unit-testable without a `Cpu`.
fn offers_escape_item(battle_mode: u8) -> bool {
    battle_mode == defs::WILD_BATTLE
}

/// Whether `id` is a guaranteed-escape item (the Poké Doll) — an Items-pocket
/// item that ends a wild battle as an escape rather than healing a party mon. It
/// takes no party target and isn't classified by [`item_effect`], so the
/// item-offering loop in [`prompt_action`] routes it to its own [`Action::Escape`]
/// branch (and playback routes the recorded item id back the same way).
fn is_escape_item_id(id: u8) -> bool {
    id == POKE_DOLL_ID
}

/// Snapshot the battle and hand it to the recorder and the player. Called at
/// the top of every prompt site: the state observed here is the outcome of the
/// previous decision — the exact state this decision will act at — so the
/// recorder writes it as the previous step's `expect` block and the player
/// asserts the previous step's recorded expectations against it.
fn observe_decision_point(cpu: &Cpu) {
    // Only the recorder and the player consume the snapshot — skip the read +
    // allocation entirely on a plain interactive run with neither installed.
    if cpu.recorder.is_none() && cpu.player.is_none() {
        return;
    }
    let snapshot = super::snapshot::BattleSnapshot::read(cpu);
    recorder::observe(cpu, &snapshot);
    player::observe(cpu, &snapshot);
}

/// CLI patch at `battle_menu`. Reads battle state from WRAM, either consumes
/// the next playback step or prompts the user for an action, then dispatches
/// it (writing WRAM and either returning to caller or tail-calling into an
/// engine sub-routine via `become`).
pub fn handle_battle_menu(cpu: &mut Cpu) {
    observe_decision_point(cpu);
    // Never let a move picked at an earlier menu leak into this one.
    cpu.cli_pending_move = None;
    cpu.cli_pending_struggle = false;
    loop {
        if cpu.decider.is_none() {
            print_battle_state(cpu);
        }
        let step = obtain(
            cpu,
            PromptSite::BattleMenu,
            &[StepKind::Move, StepKind::Fight, StepKind::Switch, StepKind::Item, StepKind::Run],
            |cpu| action_to_step(prompt_action(cpu)),
        );
        let action = {
            // Turn the step into the dispatch `Action`. Item targets come from
            // the step itself.
            match step {
                Step::Move { slot } => Action::Move(slot),
                Step::Fight => Action::Fight,
                Step::Switch { slot } => Action::Switch(slot),
                Step::Item { id, target, move_slot } => Action::Item { id, target, move_slot },
                Step::Run => Action::Run,
                // `obtain`'s `accepts` filter rules these out.
                Step::ForcedSwitch { .. } | Step::OfferSwitch { .. } => unreachable!(),
            }
        };
        match action {
            Action::Move(slot) => {
                cpu.cli_pending_move = Some(slot);
                cpu.write_byte(W_BATTLE_PLAYER_ACTION, defs::BATTLEPLAYERACTION_USEMOVE);
                cpu.write_byte(W_BATTLE_MENU_CURSOR_POSITION, 1);
                recorder::record(cpu, Step::Move { slot });
                // Tail-call battle_menu_fight: clears tilemap, sets and-a-a flags, returns.
                return crate::game::engine::battle::core::battle_menu_fight(cpu);
            }
            Action::Fight => {
                // FIGHT where the original shows no move menu. For Encore and
                // Bide `ParsePlayerAction` never calls `MoveSelectionScreen`;
                // for no-usable-moves it does, and the patched screen runs the
                // engine's own `CheckPlayerHasUsableMoves` to force Struggle.
                cpu.cli_pending_struggle = !fight_skips_move_menu(cpu);
                cpu.write_byte(W_BATTLE_PLAYER_ACTION, defs::BATTLEPLAYERACTION_USEMOVE);
                cpu.write_byte(W_BATTLE_MENU_CURSOR_POSITION, 1);
                recorder::record(cpu, Step::Fight);
                return crate::game::engine::battle::core::battle_menu_fight(cpu);
            }
            Action::Switch(slot) => {
                if let Err(msg) = check_can_switch(cpu, slot) {
                    println!("{msg}");
                    continue;
                }
                // Replicate just the state writes that `try_player_switch_try_switch`
                // performs (lines ~21219-21270 of core.rs) and skip the UI cleanup
                // calls (`clear_palettes`, `_load_hp_bar`, `close_window`, ...).
                // Those expect a party-menu overlay to be open — which it isn't,
                // because our `battle_menu` patch never opened one. Calling
                // them anyway hangs on a white screen.
                let old = cpu.read_byte(W_CUR_BATTLE_MON);
                cpu.write_byte(W_LAST_PLAYER_MON, old);
                cpu.write_byte(W_BATTLE_PLAYER_ACTION, defs::BATTLEPLAYERACTION_SWITCH);
                cpu.write_byte(W_CUR_PARTY_MON, slot);
                cpu.write_byte(W_CUR_BATTLE_MON, slot);
                cpu.write_byte(W_BATTLE_MENU_CURSOR_POSITION, 2);
                recorder::record(cpu, Step::Switch { slot });
                // `player_switch` does its own enemy-action parse + switch
                // animation, and ends by returning to whoever called us.
                return crate::game::engine::battle::core::player_switch(cpu);
            }
            Action::Item { id, target, move_slot } => {
                // Run the engine's real `BattleMenu_Pack`. Its bag UI is
                // replaced by the `battle_pack_loop` HARNESS-PATCH, which uses
                // this item exactly as the bag's USE choice does; the item's
                // own effect routine then asks the patched party menu / move
                // menu for the target and move picked here. Every side effect,
                // text, the catch / escape exits and the turn bookkeeping are
                // the original code's.
                cpu.cli_pending_item = Some((id, target, move_slot));
                recorder::record(cpu, Step::Item { id, target, move_slot });
                return crate::game::engine::battle::core::battle_menu_pack(cpu);
            }
            Action::Run => {
                cpu.write_byte(W_BATTLE_MENU_CURSOR_POSITION, 4);
                recorder::record(cpu, Step::Run);
                return crate::game::engine::battle::core::battle_menu_run(cpu);
            }
        }
    }
}

/// CLI patch at `move_selection_screen` for the engine-driven re-pick path
/// (Disable / No-PP). The normal player-action path stashes `cli_pending_move`
/// in [`handle_battle_menu`], so this prompt is rare.
pub fn handle_move_selection(cpu: &mut Cpu) -> u8 {
    observe_decision_point(cpu);
    let step = obtain(cpu, PromptSite::MoveSelection, &[StepKind::Move], prompt_move_selection);
    let Step::Move { slot } = step else { unreachable!("move selection only accepts moves") };
    // Same shape as the action-menu Move so playback doesn't need to know
    // which prompt site produced it.
    recorder::record(cpu, Step::Move { slot });
    slot
}

fn prompt_move_selection(cpu: &Cpu) -> Step {
    let species = cpu.read_byte(W_BATTLE_MON_SPECIES);
    println!(
        "\n[move] {} (Lv {}) — pick a move:",
        species_name(species),
        cpu.read_byte(W_BATTLE_MON_LEVEL)
    );
    let slots = move_options(cpu);
    for (idx, slot) in slots.iter().enumerate() {
        let move_id = cpu.read_byte(W_BATTLE_MON_MOVES + u16::from(*slot));
        let pp = cpu.read_byte(W_BATTLE_MON_PP + u16::from(*slot)) & defs::PP_MASK;
        println!("  {}) {} (PP {pp})", idx + 1, move_name(move_id));
    }
    let pick = prompt_index("> ", slots.len());
    Step::Move { slot: slots[pick] }
}

/// CLI patch at `force_pick_party_mon_in_battle`. Returns the chosen alive,
/// non-current party slot.
pub fn handle_forced_switch(cpu: &mut Cpu) -> u8 {
    observe_decision_point(cpu);
    let step = obtain(cpu, PromptSite::ForcedSwitch, &[StepKind::ForcedSwitch], prompt_forced_switch);
    let Step::ForcedSwitch { slot } = step else { unreachable!("forced switch only accepts forced_switch") };
    recorder::record(cpu, Step::ForcedSwitch { slot });
    slot
}

fn prompt_forced_switch(cpu: &Cpu) -> Step {
    print_battle_state(cpu);
    println!("\n[forced switch] your active Pokémon fainted — choose the next:");
    let current = cpu.read_byte(W_CUR_BATTLE_MON);
    let options = alive_party_slots(cpu, Some(current));
    if options.is_empty() {
        // Shouldn't happen — the engine only enters this path with surviving
        // mons. Fall back to slot 0 to make the failure visible.
        eprintln!("WARN: no alive party slots; defaulting to slot 0");
        return Step::ForcedSwitch { slot: 0 };
    }
    for (idx, slot) in options.iter().enumerate() {
        print_party_slot_line(cpu, *slot, idx + 1);
    }
    let pick = prompt_index("> ", options.len());
    Step::ForcedSwitch { slot: options[pick] }
}

/// CLI patch at `offer_switch` (trainer about to send in another mon). Returns
/// `None` if the player keeps the current mon, `Some(slot)` to swap.
pub fn handle_offer_switch(cpu: &mut Cpu) -> Option<u8> {
    observe_decision_point(cpu);
    let step = obtain(cpu, PromptSite::OfferSwitch, &[StepKind::OfferSwitch], prompt_offer_switch);
    let Step::OfferSwitch { keep, slot } = step else { unreachable!("offer switch only accepts offer_switch") };
    recorder::record(cpu, Step::OfferSwitch { keep, slot });
    if keep { None } else { slot }
}

fn prompt_offer_switch(cpu: &Cpu) -> Step {
    print_battle_state(cpu);
    let current = cpu.read_byte(W_CUR_BATTLE_MON);
    let current_species = read_party_species(cpu, current);
    println!(
        "\n[switch?] enemy is sending in another Pokémon — switch?\n  1) Keep using {}",
        species_name(current_species)
    );
    let options = alive_party_slots(cpu, Some(current));
    for (idx, slot) in options.iter().enumerate() {
        print_party_slot_line(cpu, *slot, idx + 2);
    }
    let pick = prompt_index("> ", 1 + options.len());
    if pick == 0 {
        Step::OfferSwitch { keep: true, slot: None }
    } else {
        Step::OfferSwitch { keep: false, slot: Some(options[pick - 1]) }
    }
}

// ---------------------------------------------------------------------------
// Enemy-side input (link battles)
//
// In a link battle both sides are CLI-driven. The player side goes through the
// engine's normal menus (`handle_battle_menu` etc.). The enemy side has no menu
// — the engine's link path reads the enemy's action from `wBattleAction`
// (0xd430), exactly where a serial partner would have written it. The
// `link_battle_send_receive_action` HARNESS-PATCH calls [`handle_enemy_action`]
// to fill that byte from the CLI / playback, so the second player drives the
// enemy with the same move/switch vocabulary the player has. Items, balls, and
// run aren't offered to the enemy: a Gen-II link battle disables the bag, and
// the action byte can only encode a move or a switch anyway.

/// The enemy's chosen action for the round. Mirrors the move/switch subset of
/// [`Action`]; the enemy never uses items, balls, or run.
#[derive(Debug, Clone, Copy)]
enum EnemyAction {
    Move(u8),
    Switch(u8),
}

/// CLI patch reached from the `link_battle_send_receive_action` HARNESS-PATCH in
/// a link battle: the second player picks the enemy's action for this round and
/// it is encoded into `wBattleAction` (0xd430) — the byte the engine's link path
/// reads in place of an AI decision or a serial partner. The single hook fires
/// from four engine sites; the context is read from WRAM:
///
/// - **Player forced-replacement sync** (`wBattlePlayerAction == USEITEM`, enemy
///   alive): only the player's mon fainted. `force_player_mon_choice` exchanges
///   purely to stay in lockstep with the (real) partner and never reads
///   `wBattleAction` back, so the enemy doesn't act — write a harmless SKIPTURN
///   and return WITHOUT making this a decision point (no observe / consume /
///   record), or the recorded step stream would gain a phantom enemy step.
/// - **Enemy must replace a fainted mon** (`wEnemyMonHP == 0`): offer only the
///   enemy's alive bench → `BATTLEACTION_SWITCH1 + slot`.
/// - **Normal turn / player switch**: offer the enemy's usable moves + alive
///   bench → a move slot (`BATTLEACTION_MOVE1 + slot`) or a switch.
/// - **Forced Struggle** (no usable move and no mon to switch to): not a real
///   choice, so it is not a decision point either — hand the engine
///   `BATTLEACTION_STRUGGLE` and return.
pub fn handle_enemy_action(cpu: &mut Cpu) {
    let enemy_fainted = read_word_be(cpu, W_ENEMY_MON_HP) == 0;

    // Player-only faint: the enemy isn't acting and the engine ignores the byte
    // here. Not a decision point — keep the record/playback step stream aligned.
    if cpu.read_byte(W_BATTLE_PLAYER_ACTION) == defs::BATTLEPLAYERACTION_USEITEM && !enemy_fainted {
        cpu.write_byte(W_BATTLE_ACTION, defs::BATTLEACTION_SKIPTURN);
        return;
    }

    let current = cpu.read_byte(W_CUR_OT_MON);
    let moves = if enemy_fainted { Vec::new() } else { enemy_move_options(cpu) };
    let switches = enemy_alive_party_slots(cpu, Some(current));

    // No usable move and nowhere to switch: forced Struggle. Like the sync case
    // this is not a choice, so it is recorded/replayed implicitly (the same
    // deterministic state reproduces it) rather than as a step.
    if moves.is_empty() && switches.is_empty() {
        cpu.write_byte(W_BATTLE_ACTION, defs::BATTLEACTION_STRUGGLE);
        return;
    }

    observe_decision_point(cpu);
    let step = obtain(cpu, PromptSite::EnemyAction, &[StepKind::Move, StepKind::Switch], |cpu| {
        match prompt_enemy_action(cpu, enemy_fainted, &moves, &switches) {
            EnemyAction::Move(slot) => Step::Move { slot },
            EnemyAction::Switch(slot) => Step::Switch { slot },
        }
    });
    let action = match step {
        Step::Move { slot } => EnemyAction::Move(slot),
        Step::Switch { slot } => EnemyAction::Switch(slot),
        // `obtain`'s `accepts` filter rules everything else out.
        _ => unreachable!("EnemyAction consumes only Move/Switch"),
    };

    let byte = match action {
        EnemyAction::Move(slot) => {
            recorder::record_side(cpu, Side::Enemy, Step::Move { slot });
            // BATTLEACTION_MOVE1 == 0, so the move slot index *is* the action byte.
            defs::BATTLEACTION_MOVE1 + slot
        }
        EnemyAction::Switch(slot) => {
            recorder::record_side(cpu, Side::Enemy, Step::Switch { slot });
            defs::BATTLEACTION_SWITCH1 + slot
        }
    };
    cpu.write_byte(W_BATTLE_ACTION, byte);
}

/// Prompt the second player for the enemy's action. `moves` / `switches` are the
/// already-computed legal options (so the empty/forced cases are handled by the
/// caller). When `enemy_fainted` the enemy is sending in a replacement, so only
/// switches are offered.
fn prompt_enemy_action(cpu: &Cpu, enemy_fainted: bool, moves: &[u8], switches: &[u8]) -> EnemyAction {
    print_enemy_state(cpu);
    let mut entries: Vec<(String, EnemyAction)> = Vec::new();
    if !enemy_fainted {
        for &slot in moves {
            let move_id = cpu.read_byte(W_ENEMY_MON_MOVES + u16::from(slot));
            let pp = cpu.read_byte(W_ENEMY_MON_PP + u16::from(slot)) & defs::PP_MASK;
            entries.push((format!("{} (PP {pp})", move_name(move_id)), EnemyAction::Move(slot)));
        }
    }
    for &slot in switches {
        let species = read_ot_party_species(cpu, slot);
        let hp = read_word_be(cpu, ot_party_slot_addr(slot) + PARTY_OFFSET_HP);
        let max_hp = read_word_be(cpu, ot_party_slot_addr(slot) + PARTY_OFFSET_MAX_HP);
        entries.push((
            format!("Switch to {} (HP {}/{})", species_name(species), hp, max_hp),
            EnemyAction::Switch(slot),
        ));
    }

    if enemy_fainted {
        println!("\n[enemy] your Pokémon fainted — choose the next:");
    } else {
        println!("\n[enemy] choose an action:");
    }
    for (idx, (label, _)) in entries.iter().enumerate() {
        println!("  {}) {label}", idx + 1);
    }
    let pick = prompt_index("> ", entries.len());
    entries[pick].1
}

/// Print the battle from the enemy player's perspective: their active mon as
/// "You", the player's mon as "Opponent". Mirrors [`print_battle_state`].
fn print_enemy_state(cpu: &Cpu) {
    let species = cpu.read_byte(W_ENEMY_MON_SPECIES);
    let level = cpu.read_byte(W_ENEMY_MON_LEVEL);
    let hp = read_word_be(cpu, W_ENEMY_MON_HP);
    let max_hp = read_word_be(cpu, W_ENEMY_MON_MAX_HP);
    let status = cpu.read_byte(W_ENEMY_MON_STATUS);
    let item = cpu.read_byte(W_ENEMY_MON_ITEM);

    let foe_species = cpu.read_byte(W_BATTLE_MON_SPECIES);
    let foe_level = cpu.read_byte(W_BATTLE_MON_LEVEL);
    let foe_hp = read_word_be(cpu, W_BATTLE_MON_HP);
    let foe_max_hp = read_word_be(cpu, W_BATTLE_MON_MAX_HP);
    let foe_status = cpu.read_byte(W_BATTLE_MON_STATUS);
    let foe_item = cpu.read_byte(W_BATTLE_MON_ITEM);

    println!(
        "\n=== Opponent: {} Lv{} HP {}/{}{}{}\n=== Enemy:    {} Lv{} HP {}/{}{}{}",
        species_name(foe_species),
        foe_level,
        foe_hp,
        foe_max_hp,
        status_suffix(foe_status),
        item_suffix(foe_item),
        species_name(species),
        level,
        hp,
        max_hp,
        status_suffix(status),
        item_suffix(item),
    );
}

/// Address of OT party slot `slot` (enemy `party_struct`).
fn ot_party_slot_addr(slot: u8) -> u16 {
    W_OT_PARTY_MON1 + u16::from(slot) * PARTY_STRUCT_LEN
}

fn read_ot_party_species(cpu: &impl ReadMem, slot: u8) -> u8 {
    cpu.read_byte(ot_party_slot_addr(slot))
}

/// The enemy's selectable move slots: a real move (id != 0) with PP left that
/// isn't Disabled. Mirrors [`move_options`] against the enemy's `wEnemyMon*`
/// fields and `wEnemyDisableCount`.
fn enemy_move_options(cpu: &impl ReadMem) -> Vec<u8> {
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
fn enemy_alive_party_slots(cpu: &impl ReadMem, exclude: Option<u8>) -> Vec<u8> {
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

// ---------------------------------------------------------------------------
// Prompt rendering

fn print_battle_state(cpu: &Cpu) {
    let species = cpu.read_byte(W_BATTLE_MON_SPECIES);
    let level = cpu.read_byte(W_BATTLE_MON_LEVEL);
    let hp = read_word_be(cpu, W_BATTLE_MON_HP);
    let max_hp = read_word_be(cpu, W_BATTLE_MON_MAX_HP);
    let status = cpu.read_byte(W_BATTLE_MON_STATUS);
    let item = cpu.read_byte(W_BATTLE_MON_ITEM);

    let enemy_species = cpu.read_byte(W_ENEMY_MON_SPECIES);
    let enemy_level = cpu.read_byte(W_ENEMY_MON_LEVEL);
    let enemy_hp = read_word_be(cpu, W_ENEMY_MON_HP);
    let enemy_max_hp = read_word_be(cpu, W_ENEMY_MON_MAX_HP);
    let enemy_status = cpu.read_byte(W_ENEMY_MON_STATUS);
    let enemy_item = cpu.read_byte(W_ENEMY_MON_ITEM);

    println!(
        "\n=== Enemy: {} Lv{} HP {}/{}{}{}\n=== You:   {} Lv{} HP {}/{}{}{}",
        species_name(enemy_species),
        enemy_level,
        enemy_hp,
        enemy_max_hp,
        status_suffix(enemy_status),
        item_suffix(enemy_item),
        species_name(species),
        level,
        hp,
        max_hp,
        status_suffix(status),
        item_suffix(item),
    );
}

fn prompt_action(cpu: &Cpu) -> Action {
    // One menu line per move / switch / run, and one per distinct bag item;
    // the item's target and move are asked for afterwards. Everything offered
    // comes from `legal_battle_menu_steps`, so stdin can only pick what the
    // original menus accept.
    let legal = legal_battle_menu_steps(cpu);
    let mut entries: Vec<(String, Step)> = Vec::new();
    for step in &legal {
        let label = match *step {
            Step::Move { slot } => {
                let move_id = cpu.read_byte(W_BATTLE_MON_MOVES + u16::from(slot));
                let pp = cpu.read_byte(W_BATTLE_MON_PP + u16::from(slot)) & defs::PP_MASK;
                format!("{} (PP {pp})", move_name(move_id))
            }
            Step::Fight => "Fight".to_string(),
            Step::Switch { slot } => {
                let species = read_party_species(cpu, slot);
                let hp = read_word_be(cpu, party_slot_addr(slot) + PARTY_OFFSET_HP);
                let max_hp = read_word_be(cpu, party_slot_addr(slot) + PARTY_OFFSET_MAX_HP);
                format!("Switch to {} (HP {}/{})", species_name(species), hp, max_hp)
            }
            Step::Item { id, .. } => {
                if entries.iter().any(|(_, s)| matches!(s, Step::Item { id: other, .. } if *other == id)) {
                    continue;
                }
                let qty = item_quantity(cpu, id);
                if is_ball_id(id) {
                    format!("Throw {} ×{qty}", item_display(id))
                } else if is_escape_item_id(id) {
                    format!("Use {} ×{qty} (escape)", item_display(id))
                } else {
                    format!("Use {} ×{qty}", item_display(id))
                }
            }
            Step::Run => "Run".to_string(),
            Step::ForcedSwitch { .. } | Step::OfferSwitch { .. } => unreachable!(),
        };
        entries.push((label, *step));
    }

    println!("\n[turn] choose an action:");
    for (idx, (label, _)) in entries.iter().enumerate() {
        println!("  {}) {label}", idx + 1);
    }
    let pick = prompt_index("> ", entries.len());
    let chosen = entries[pick].1;

    let Step::Item { id, .. } = chosen else {
        return match chosen {
            Step::Move { slot } => Action::Move(slot),
            Step::Fight => Action::Fight,
            Step::Switch { slot } => Action::Switch(slot),
            _ => Action::Run,
        };
    };
    // The item's legal (target, move) pairs.
    let uses: Vec<(Option<u8>, Option<u8>)> = legal
        .iter()
        .filter_map(|s| match *s {
            Step::Item { id: other, target, move_slot } if other == id => Some((target, move_slot)),
            _ => None,
        })
        .collect();
    let mut targets: Vec<Option<u8>> = Vec::new();
    for (t, _) in &uses {
        if !targets.contains(t) {
            targets.push(*t);
        }
    }
    let target = match targets.as_slice() {
        [None] => None,
        _ => Some(prompt_item_target(cpu, id, &targets.iter().flatten().copied().collect::<Vec<_>>())),
    };
    let moves: Vec<u8> = uses.iter().filter(|(t, _)| *t == target).filter_map(|(_, m)| *m).collect();
    let move_slot = match (target, moves.is_empty()) {
        (Some(t), false) => Some(prompt_pp_move_slot(cpu, id, t, &moves)),
        _ => None,
    };
    Action::Item { id, target, move_slot }
}

fn item_quantity(cpu: &Cpu, id: u8) -> u8 {
    read_pocket(cpu, W_NUM_ITEMS)
        .into_iter()
        .chain(read_pocket(cpu, W_NUM_BALLS))
        .find(|&(other, _)| other == id)
        .map_or(0, |(_, qty)| qty)
}

fn prompt_item_target(cpu: &Cpu, id: u8, options: &[u8]) -> u8 {
    println!("\n[item] use {} on which Pokémon?", item_display(id));
    for (idx, slot) in options.iter().enumerate() {
        print_party_slot_line(cpu, *slot, idx + 1);
    }
    let pick = prompt_index("> ", options.len());
    options[pick]
}

/// Prompt for which move of party `slot` to restore PP on, for a single-move PP
/// item. `options` are the move slots the item would affect.
fn prompt_pp_move_slot(cpu: &Cpu, id: u8, slot: u8, options: &[u8]) -> u8 {
    println!(
        "\n[item] use {} on which move of {}?",
        item_display(id),
        species_name(read_party_species(cpu, slot)),
    );
    let base = party_slot_addr(slot);
    for (idx, move_slot) in options.iter().enumerate() {
        let move_id = cpu.read_byte(base + PARTY_OFFSET_MOVES + u16::from(*move_slot));
        let pp_byte = cpu.read_byte(base + PARTY_OFFSET_PP + u16::from(*move_slot));
        println!(
            "  {}) {} (PP {}/{})",
            idx + 1,
            move_name(move_id),
            move_pp::current_pp(pp_byte),
            move_pp::max_pp(move_id, pp_byte),
        );
    }
    let pick = prompt_index("> ", options.len());
    options[pick]
}

fn print_party_slot_line(cpu: &Cpu, slot: u8, idx: usize) {
    let species = read_party_species(cpu, slot);
    let hp = read_word_be(cpu, party_slot_addr(slot) + PARTY_OFFSET_HP);
    let max_hp = read_word_be(cpu, party_slot_addr(slot) + PARTY_OFFSET_MAX_HP);
    let status = cpu.read_byte(party_slot_addr(slot) + PARTY_OFFSET_STATUS);
    let fainted = if hp == 0 { " [FAINTED]" } else { "" };
    println!(
        "  {idx}) {} (HP {}/{}{}){}",
        species_name(species),
        hp,
        max_hp,
        status_suffix(status),
        fainted,
    );
}

fn prompt_index(prompt: &str, len: usize) -> usize {
    loop {
        print!("{prompt}");
        let _ = std::io::stdout().flush();
        let mut line = String::new();
        // EOF is `Ok(0)`, not `Err`: a closed stdin (dead driver, `</dev/null`)
        // must abort, or this loop spins forever re-prompting at 100% CPU.
        match std::io::stdin().lock().read_line(&mut line) {
            Ok(0) | Err(_) => {
                eprintln!("error: stdin closed while waiting for a battle decision");
                std::process::exit(2);
            }
            Ok(_) => {}
        }
        let trimmed = line.trim();
        if let Ok(n) = trimmed.parse::<usize>() {
            if n >= 1 && n <= len {
                return n - 1;
            }
        }
        println!("Enter a number 1..={len}.");
    }
}

// ---------------------------------------------------------------------------
// Switch validation

fn check_can_switch(cpu: &impl ReadMem, target: u8) -> Result<(), String> {
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

// ---------------------------------------------------------------------------
// Bag items through the real engine routines

/// Called at the top of `battle_pack_loop` (HARNESS-PATCH). If the action menu
/// picked an item, use it exactly as the bag's USE choice does: set the
/// selection the pocket menu would have left (`wCurItem`, `wCurPocket`) and run
/// `ItemSubmenu.Use`, which looks up the item's battle context and calls its
/// real effect routine through `DoItemEffect`. The effect asks for its target
/// and move through the patched party menu / move menu.
pub fn use_pending_item_in_pack(cpu: &mut Cpu) {
    let Some((id, target, move_slot)) = cpu.cli_pending_item.take() else {
        return;
    };
    cpu.write_byte(W_CUR_ITEM, id);
    let pocket = if is_ball_id(id) { defs::BALL_POCKET } else { defs::ITEM_POCKET };
    cpu.write_byte(W_CUR_POCKET, pocket);
    cpu.cli_pending_party_target = target;
    cpu.cli_pending_item_move = move_slot;

    let before = cpu.read_byte(W_PLAYER_SUB_STATUS_4);
    let caller_pc = cpu.pc;
    let return_sentinel: u16 = 0xfffa;
    cpu.push_word(return_sentinel);
    crate::game::engine::items::pack::tmhm_submenu_use(cpu);
    assert_eq!(cpu.pc, return_sentinel);
    // Back at the caller: it computes its return addresses from `pc`, which
    // would otherwise run past 0xffff.
    cpu.pc = caller_pc;

    // Using the item moves the bag to PACKSTATE_QUITRUNSCRIPT (whose handler
    // then sets JUMPTABLE_EXIT_F on the loop's next pass). If it didn't, the
    // original refused the item ("won't have any effect" / "isn't the time")
    // — and `legal_steps` should never have offered it. That is a bug in the
    // legality predicates, not a playable state.
    let used = cpu.read_byte(W_JUMPTABLE_INDEX) == defs::PACKSTATE_QUITRUNSCRIPT;
    assert!(
        used,
        "item {} (target {target:?}, move {move_slot:?}) was offered as legal but its effect refused it \
         (jumptable {:#x}, substatus4 {:#x} before {before:#x}, battle type {:#x})",
        item_display(id),
        cpu.read_byte(W_JUMPTABLE_INDEX),
        cpu.read_byte(W_PLAYER_SUB_STATUS_4),
        cpu.read_byte(0xd230),
    );
    assert!(cpu.cli_pending_party_target.is_none(), "item effect never asked for its target");
    assert!(cpu.cli_pending_item_move.is_none(), "item effect never asked for its move");
    // A catch leaves the species in wWildMon; `BattleMenu_Pack.UseItem` then
    // ends the battle as a WIN, indistinguishable in WRAM from a KO.
    if is_ball_id(id) && cpu.read_byte(W_WILD_MON) != 0 {
        cpu.caught_wild_mon = true;
    }
}

/// `party_menu_select` (HARNESS-PATCH) while an item effect is choosing its
/// target: leave what the routine leaves after A on party slot `slot` — 1-based
/// `wMenuCursorY` / `wPartyMenuCursor`, `wCurPartyMon`, `wCurPartySpecies` —
/// and return carry clear. The joypad read and menu sound effects are skipped.
pub fn select_party_target(cpu: &mut Cpu, slot: u8) {
    cpu.write_byte(W_MENU_CURSOR_Y, slot + 1);
    cpu.write_byte(W_PARTY_MENU_CURSOR, slot + 1);
    cpu.write_byte(W_CUR_PARTY_MON, slot);
    let species = cpu.read_byte(W_PARTY_SPECIES + u16::from(slot));
    cpu.write_byte(W_CUR_PARTY_SPECIES, species);
    cpu.a = species;
    cpu.flag_z = species == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

// ---------------------------------------------------------------------------
// Item application
//
// For healing items we apply the effect directly to the target party slot so we
// don't have to re-enter the engine's bag UI (which itself opens the party
// menu for target selection). For balls we hand control back to the engine
// via `battle_menu_pack_ball` — but [`apply_item`] is only invoked for the
// direct-apply types; balls take the path inside [`handle_battle_menu`].

/// What a battle-usable item does, abstracted over the specific item id. The
/// single source of truth for which items the CLI applies directly, which party
/// mons a given item can affect, and the status mask [`apply_item`] clears.
/// Items not classified here aren't offered in the action menu (balls,
/// evolution stones, ...).
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)] // the payloads document each effect; the real routines apply them
enum ItemEffect {
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
    /// in [`prompt_action`] rather than going through the party-target machinery.
    ActiveConfusion,
    /// Raises one stat stage of the active player mon by one level (X Attack /
    /// X Defend / X Speed / X Special). The payload is the stat index into
    /// `wPlayerStatLevels` (ATTACK=0, DEFENSE=1, SPEED=2, SP_ATTACK=3). Like
    /// [`ItemEffect::ActiveConfusion`] this acts on the active battler with no
    /// party target; its validity is judged from the stage level (usable only
    /// while below `MAX_STAT_LEVEL`), so it's an "active-mon item" in
    /// [`prompt_action`]. See `XItemEffect` / `RaiseStat`
    /// (`scratch/pokecrystal/engine/items/item_effects.asm`,
    /// `engine/battle/effect_commands.asm`).
    ActiveStatUp(u8),
    /// Sets a single sub-status bit `mask` in the byte at `addr` on the active
    /// player mon, refusing if it's already set (X Accuracy → `SUBSTATUS_X_ACCURACY`,
    /// Guard Spec → `SUBSTATUS_MIST`, Dire Hit → `SUBSTATUS_FOCUS_ENERGY`, all in
    /// `wPlayerSubStatus4`). Like [`ItemEffect::ActiveConfusion`] (which is the
    /// inverse — a bit-CLEAR) this acts on the active battler with no party
    /// target; its validity is judged from the sub-status byte (usable only while
    /// the bit is CLEAR), so it's an "active-mon item" in [`prompt_action`]. See
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
fn item_effect(id: u8) -> Option<ItemEffect> {
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
fn item_valid_targets(cpu: &impl ReadMem, id: u8) -> Vec<u8> {
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
fn slot_has_restorable_pp(cpu: &impl ReadMem, slot: u8) -> bool {
    !restorable_move_slots(cpu, slot).is_empty()
}

/// The 0-based move slots of party `slot` that exist (move id != 0) and hold a
/// move below its max PP — i.e. the moves a single-move PP item could restore.
/// Drives the move-selection prompt for Ether/Max Ether/MysteryBerry.
fn restorable_move_slots(cpu: &impl ReadMem, slot: u8) -> Vec<u8> {
    let base = party_slot_addr(slot);
    (0..MOVE_SLOTS)
        .filter(|&i| {
            let move_id = cpu.read_byte(base + PARTY_OFFSET_MOVES + i);
            if move_id == 0 {
                return false;
            }
            let pp_byte = cpu.read_byte(base + PARTY_OFFSET_PP + i);
            move_pp::current_pp(pp_byte) < move_pp::max_pp(move_id, pp_byte)
        })
        .map(|i| i as u8)
        .collect()
}

/// Whether `id` is a single-move PP item (Ether / Max Ether / MysteryBerry),
/// which needs a move-slot prompt after the target mon is chosen.
fn is_single_move_pp_item(id: u8) -> bool {
    matches!(item_effect(id), Some(ItemEffect::PpOne { .. }))
}

/// Whether `id` lives in the Balls pocket (Poké Ball / Great Ball / the Apricorn
/// balls / Master Ball / …). A ball is just an item id from a different pocket,
/// so it's recorded as an `item` step; this lets the dispatch route it to the
/// ball path (decrement the Balls pocket, throw it) rather than the party-target
/// item path. Source: the `Pocket::Ball` rows in [`super::items`].
fn is_ball_id(id: u8) -> bool {
    matches!(super::items::lookup_id(id), Some(d) if d.pocket == super::items::Pocket::Ball)
}

/// Whether `id` is an active-mon item (acts on the active battler, takes no
/// party target) AND is usable right now: Bitter Berry only while the active mon
/// is confused; an X-item only while its stat stage is below `MAX_STAT_LEVEL`
/// (mirrors `RaiseStat`'s "wouldn't have any effect" guard at the cap — the asm
/// refuses to raise past 13). Returns `false` for every party-target item.
/// Drives whether the active-mon item appears in the action menu.
fn active_item_usable(cpu: &impl ReadMem, id: u8) -> bool {
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

// ---------------------------------------------------------------------------
// WRAM helpers

fn party_slot_addr(slot: u8) -> u16 {
    W_PARTY_MON1 + u16::from(slot) * PARTY_STRUCT_LEN
}

fn read_party_species(cpu: &impl ReadMem, slot: u8) -> u8 {
    cpu.read_byte(party_slot_addr(slot))
}

fn read_word_be(cpu: &impl ReadMem, addr: u16) -> u16 {
    u16::from_be_bytes([cpu.read_byte(addr), cpu.read_byte(addr + 1)])
}

fn read_pocket(cpu: &impl ReadMem, count_addr: u16) -> Vec<(u8, u8)> {
    let n = cpu.read_byte(count_addr) as u16;
    let mut out = Vec::with_capacity(n as usize);
    for i in 0..n {
        let id = cpu.read_byte(count_addr + 1 + i * 2);
        let qty = cpu.read_byte(count_addr + 2 + i * 2);
        out.push((id, qty));
    }
    out
}

fn move_options(cpu: &impl ReadMem) -> Vec<u8> {
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
fn illegal_move_reason(cpu: &impl ReadMem, slot: u8) -> Option<String> {
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
    Some(format!("slot {slot} ({}) is {why}", move_name(move_id)))
}

/// During playback, a recorded `move` step must name a slot the player could
/// legally have chosen at this decision point — the same set the engine's move
/// menu would offer ([`move_options`]: a real move, with PP, not Disabled). A
/// recording that selects an illegal move (e.g. one Disabled by the enemy,
/// captured before `move_options` filtered Disable) encodes play the real game
/// forbids, so fail loudly to get it regenerated rather than silently replaying
/// impossible moves. No-op outside playback: the live menu only ever offers
/// legal slots, so recording can't produce one.
// ---------------------------------------------------------------------------
// Decision points: legal choices and where the choice comes from

/// Get the decision for `site`: from the playback script if one is installed,
/// else from the installed [`super::decision::Decider`], else from stdin via
/// `stdin`. Playback and decider steps are checked against [`legal_steps`];
/// an illegal recorded step exits with status 2 (structural drift), an
/// illegal decider step panics.
fn obtain(
    cpu: &Cpu,
    site: PromptSite,
    accepts: &[StepKind],
    stdin: impl FnOnce(&Cpu) -> Step,
) -> Step {
    if let Some(step) = player::consume(cpu, site, accepts) {
        let legal = legal_steps(cpu, site);
        if legal.contains(&step) {
            return step;
        }
        // Recordings made before `fight` existed record FIGHT-without-a-menu
        // (Encore / Bide) as whatever move slot was picked; the engine ignored
        // it. Read those back as the `fight` they were.
        if matches!(step, Step::Move { .. }) && legal.contains(&Step::Fight) {
            return Step::Fight;
        }
        if let Step::Move { slot } = step {
            if let Some(reason) = illegal_move_reason(cpu, slot) {
                eprintln!(
                    "playback: illegal move — recorded {reason}; selectable slots were {:?}",
                    move_options(cpu),
                );
                std::process::exit(2);
            }
        }
        eprintln!("playback: illegal step at {site:?} — recorded {step:?}; legal steps were {legal:?}");
        std::process::exit(2);
    }
    if let Some(cell) = cpu.decider.as_ref() {
        let legal = legal_steps(cpu, site);
        let step = cell.borrow_mut().decide(cpu, site, &legal);
        assert!(legal.contains(&step), "decider chose illegal {step:?} at {site:?}; legal: {legal:?}");
        return step;
    }
    stdin(cpu)
}

fn action_to_step(action: Action) -> Step {
    match action {
        Action::Move(slot) => Step::Move { slot },
        Action::Fight => Step::Fight,
        Action::Switch(slot) => Step::Switch { slot },
        Action::Item { id, target, move_slot } => Step::Item { id, target, move_slot },
        Action::Run => Step::Run,
    }
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
    }
}

fn legal_battle_menu_steps(cpu: &impl ReadMem) -> Vec<Step> {
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

fn alive_party_slots(cpu: &impl ReadMem, exclude: Option<u8>) -> Vec<u8> {
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

fn species_name(id: u8) -> &'static str {
    names::species_name(id)
}

fn move_name(id: u8) -> &'static str {
    names::move_name(id)
}

fn item_display(id: u8) -> &'static str {
    names::item_name(id)
}

/// `" @ItemName"` when the mon holds an item, empty otherwise. Appended to the
/// battle status line so a held item (Leftovers, type boosters, …) is visible.
fn item_suffix(item: u8) -> String {
    if item == 0 {
        String::new()
    } else {
        format!(" @{}", item_display(item))
    }
}

fn status_suffix(status: u8) -> String {
    if status == 0 {
        return String::new();
    }
    let mut tags: Vec<&str> = Vec::new();
    if status & 0b0000_0111 != 0 {
        tags.push("SLP");
    }
    if status & (1 << defs::PSN as u8) != 0 {
        tags.push("PSN");
    }
    if status & (1 << defs::BRN as u8) != 0 {
        tags.push("BRN");
    }
    if status & (1 << defs::FRZ as u8) != 0 {
        tags.push("FRZ");
    }
    if status & (1 << defs::PAR as u8) != 0 {
        tags.push("PAR");
    }
    if tags.is_empty() {
        return String::new();
    }
    format!(" [{}]", tags.join(","))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn write_word_be(cpu: &mut Cpu, addr: u16, value: u16) {
        cpu.write_byte(addr, (value >> 8) as u8);
        cpu.write_byte(addr + 1, value as u8);
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
        let mut cpu = Cpu::new_cgb(None, None);
        // Four real moves, each with PP to spare.
        for slot in 0..4u16 {
            cpu.write_byte(W_BATTLE_MON_MOVES + slot, slot as u8 + 1);
            cpu.write_byte(W_BATTLE_MON_PP + slot, 10);
        }

        // Nothing disabled: all four moves are selectable.
        cpu.write_byte(W_PLAYER_DISABLE_COUNT, 0);
        assert_eq!(move_options(&cpu), vec![0, 1, 2, 3]);

        // Disable the move in slot 1 — high nibble = its 1-based slot (2), low
        // nibble = turns remaining (5). It drops out of the selectable list,
        // matching the engine's `.move_disabled` rejection.
        cpu.write_byte(W_PLAYER_DISABLE_COUNT, 0x25);
        assert_eq!(move_options(&cpu), vec![0, 2, 3]);

        // Counter cleared (Disable expired): the move is selectable again.
        cpu.write_byte(W_PLAYER_DISABLE_COUNT, 0);
        assert_eq!(move_options(&cpu), vec![0, 1, 2, 3]);
    }

    /// The enemy move menu (link battles) mirrors the player's: a real move
    /// (id != 0) with PP that isn't Disabled. Same logic as
    /// `disabled_move_is_not_selectable`, against the `wEnemyMon*` fields and
    /// `wEnemyDisableCount`.
    #[test]
    fn enemy_move_options_filter_pp_and_disable() {
        let mut cpu = Cpu::new_cgb(None, None);
        // slot 0: real move, PP. slot 1: real move, 0 PP. slot 2: real move, PP
        // (we'll Disable it). slot 3: empty.
        cpu.write_byte(W_ENEMY_MON_MOVES + 0, 10);
        cpu.write_byte(W_ENEMY_MON_PP + 0, 5);
        cpu.write_byte(W_ENEMY_MON_MOVES + 1, 20);
        cpu.write_byte(W_ENEMY_MON_PP + 1, 0);
        cpu.write_byte(W_ENEMY_MON_MOVES + 2, 30);
        cpu.write_byte(W_ENEMY_MON_PP + 2, 8);
        cpu.write_byte(W_ENEMY_MON_MOVES + 3, 0);
        cpu.write_byte(W_ENEMY_DISABLE_COUNT, 0); // nothing disabled
        assert_eq!(enemy_move_options(&cpu), vec![0, 2]); // slot 1 has no PP

        // Disable slot 2 (high nibble = its 1-based slot 3): drops out.
        cpu.write_byte(W_ENEMY_DISABLE_COUNT, 0x34);
        assert_eq!(enemy_move_options(&cpu), vec![0]);
    }

    /// The enemy switch list (link battles) is the alive OT party slots minus
    /// the active one — so a single-mon enemy offers no switch.
    #[test]
    fn enemy_alive_party_slots_exclude_active_and_fainted() {
        let mut cpu = Cpu::new_cgb(None, None);
        cpu.write_byte(W_OT_PARTY_COUNT, 3);
        // slot 0 active+alive, slot 1 alive, slot 2 fainted.
        write_word_be(&mut cpu, ot_party_slot_addr(0) + PARTY_OFFSET_HP, 20);
        write_word_be(&mut cpu, ot_party_slot_addr(1) + PARTY_OFFSET_HP, 15);
        write_word_be(&mut cpu, ot_party_slot_addr(2) + PARTY_OFFSET_HP, 0);
        assert_eq!(enemy_alive_party_slots(&cpu, Some(0)), vec![1]);
        // Single-mon enemy: nothing to switch to.
        cpu.write_byte(W_OT_PARTY_COUNT, 1);
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
        let mut cpu = Cpu::new_cgb(None, None);
        // slot 0: real move, full PP. slot 1: real move, 0 PP. slot 2: real
        // move, full PP (we'll Disable it). slot 3: empty.
        cpu.write_byte(W_BATTLE_MON_MOVES + 0, 1);
        cpu.write_byte(W_BATTLE_MON_PP + 0, 10);
        cpu.write_byte(W_BATTLE_MON_MOVES + 1, 2);
        cpu.write_byte(W_BATTLE_MON_PP + 1, 0);
        cpu.write_byte(W_BATTLE_MON_MOVES + 2, 3);
        cpu.write_byte(W_BATTLE_MON_PP + 2, 10);
        cpu.write_byte(W_BATTLE_MON_MOVES + 3, 0);
        // Disable slot 2 (1-based slot 3 in the high nibble).
        cpu.write_byte(W_PLAYER_DISABLE_COUNT, 0x34);

        assert_eq!(illegal_move_reason(&cpu, 0), None, "real move with PP is legal");
        assert!(illegal_move_reason(&cpu, 1).unwrap().contains("no PP left"));
        assert!(illegal_move_reason(&cpu, 2).unwrap().contains("Disabled"));
        assert!(illegal_move_reason(&cpu, 3).unwrap().contains("no move in that slot"));
    }
}
