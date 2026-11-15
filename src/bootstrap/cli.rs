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
use crate::runtime::Cpu;

use super::move_pp;
use super::names;
use super::player::{self, PromptSite, StepKind};
use super::recorder::{self, Side, Step};
pub use crate::battle::legal::*;

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

// `wPlayerSubStatus4` holds another byte of the active player mon's battle-only
// sub-statuses. The three in-battle "X" items that set a single bit here live in
// this byte: X Accuracy (`SUBSTATUS_X_ACCURACY`, bit 0), Guard Spec
// (`SUBSTATUS_MIST`, bit 1) and Dire Hit (`SUBSTATUS_FOCUS_ENERGY`, bit 2). The
// SubStatus4 bit list in `scratch/pokecrystal/constants/battle_constants.asm`
// (`const_def` from 0) is X_ACCURACY=0, MIST=1, FOCUS_ENERGY=2, then a
// `const_skip`, SUBSTITUTE=4, RECHARGE=5, RAGE=6, LEECH_SEED=7 — so the masks
// are `1 << 0`, `1 << 1`, `1 << 2`. `pokecrystal.sym`: `00:c66b wPlayerSubStatus4`.
const W_PLAYER_SUB_STATUS_4: u16 = 0xc66b;
// Bag-UI state the item hooks fill in. `00:cf63 wJumptableIndex`,
// `00:cf65 wCurPocket`, `01:d0d8 wPartyMenuCursor`, `01:d108
// wCurPartySpecies`, `01:dcd8 wPartySpecies` in `pokecrystal.sym`.
const W_JUMPTABLE_INDEX: u16 = 0xcf63;
const W_CUR_POCKET: u16 = 0xcf65;
const W_PARTY_MENU_CURSOR: u16 = 0xd0d8;
const W_CUR_PARTY_SPECIES: u16 = 0xd108;
const W_PARTY_SPECIES: u16 = 0xdcd8;
const W_LAST_PLAYER_MON: u16 = 0xc71a;

const W_BATTLE_MENU_CURSOR_POSITION: u16 = 0xd0d2;
const W_CUR_BATTLE_MON: u16 = 0xd0d4;
const W_MENU_CURSOR_Y: u16 = 0xcfa9;
const W_BATTLE_PLAYER_ACTION: u16 = 0xd0ec;
const W_CUR_ITEM: u16 = 0xd106;
const W_CUR_PARTY_MON: u16 = 0xd109;

const W_NUM_ITEMS: u16 = 0xd892;
const W_NUM_BALLS: u16 = 0xd8d7;

// Capture-path WRAM. Source: scratch/pokecrystal/pokecrystal.sym.
// `wWildMon` (0xc64e): non-zero (the caught species) after a successful catch,
// zero after a failed throw — this is what `BattleMenu_Pack.UseItem` keys on to
// decide caught-vs-broke-free, and what we key on too.
const W_WILD_MON: u16 = 0xc64e;

const W_ENEMY_MON_SPECIES: u16 = 0xd206;
const W_ENEMY_MON_ITEM: u16 = 0xd207; // held item id (`battle_struct` +1)
const W_ENEMY_MON_MOVES: u16 = 0xd208;
const W_ENEMY_MON_PP: u16 = 0xd20e;
const W_ENEMY_MON_LEVEL: u16 = 0xd213;
const W_ENEMY_MON_HP: u16 = 0xd216;
const W_ENEMY_MON_MAX_HP: u16 = 0xd218;
const W_ENEMY_MON_STATUS: u16 = 0xd214;
// Enemy active party slot, and the OT party list (same `party_struct` layout as
// the player party). `00:c663 wCurOTMon`, `01:d280 wOTPartyCount`,
// `01:d288 wOTPartyMon1` in `pokecrystal.sym`.
const W_CUR_OT_MON: u16 = 0xc663;
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
        if !decided_externally(cpu, PromptSite::BattleMenu) {
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

/// `AIChooseMove` patch for an externally controlled trainer
/// (`cpu.external_enemy`): one [`PromptSite::EnemyTrainerAction`] decision,
/// unless the enemy is locked in (`CheckEnemyLockedIn`). A move is written
/// the way `AIChooseMove` writes its pick (`wCurEnemyMove`,
/// `wCurEnemyMoveNum`); a switch or an item is stashed for the
/// `AI_SwitchOrTryItem` patch ([`external_enemy_switch_or_item`]).
pub fn handle_external_enemy_action(cpu: &mut Cpu) {
    cpu.external_enemy_choice = None;
    if enemy_locked_in(cpu) {
        return;
    }
    observe_decision_point(cpu);
    let step = obtain(
        cpu,
        PromptSite::EnemyTrainerAction,
        &[StepKind::Move, StepKind::Fight, StepKind::Switch, StepKind::Item],
        prompt_enemy_trainer_action,
    );
    recorder::record_side(cpu, Side::Enemy, step);
    let slot = external_enemy_move_slot(cpu, step);
    let mv = cpu.read_byte(W_ENEMY_MON_MOVES + u16::from(slot));
    cpu.write_byte(W_CUR_ENEMY_MOVE, mv);
    cpu.write_byte(W_CUR_ENEMY_MOVE_NUM, slot);
    cpu.external_enemy_choice = Some(step);
}


const W_CUR_ENEMY_MOVE: u16 = crate::engine::sym::W_CUR_ENEMY_MOVE;
const W_CUR_ENEMY_MOVE_NUM: u16 = crate::engine::sym::W_CUR_ENEMY_MOVE_NUM;

/// `FindMonInOTPartyToSwitchIntoBattle` patch for an externally controlled
/// trainer: one [`PromptSite::EnemySwitchIn`] decision. Returns the OT party
/// slot (the routine's `b`).
pub fn handle_external_enemy_switch_in(cpu: &mut Cpu) -> u8 {
    observe_decision_point(cpu);
    let step = obtain(cpu, PromptSite::EnemySwitchIn, &[StepKind::Switch], |cpu| {
        print_enemy_state(cpu);
        let current = cpu.read_byte(W_CUR_OT_MON);
        let slots = enemy_alive_party_slots(cpu, Some(current));
        let labels: Vec<String> = slots.iter().map(|&s| enemy_party_label(cpu, s)).collect();
        let pick = prompt_list("Enemy trainer: send in which mon?", &labels);
        Step::Switch { slot: slots[pick] }
    });
    recorder::record_side(cpu, Side::Enemy, step);
    let Step::Switch { slot } = step else { unreachable!("EnemySwitchIn consumes only Switch") };
    slot
}

/// What the `AI_SwitchOrTryItem` patch does for an external trainer's
/// stashed choice.
pub enum ExternalEnemyTurn {
    /// A move (or nothing to do): return with carry clear.
    Attack,
    /// Switch to this OT party slot (`AI_Switch`).
    Switch(u8),
    /// Use this AI item on the active mon.
    Item(u8),
}

/// The stashed [`handle_external_enemy_action`] choice, for the
/// `AI_SwitchOrTryItem` patch.
pub fn external_enemy_switch_or_item(cpu: &Cpu) -> ExternalEnemyTurn {
    match cpu.external_enemy_choice {
        Some(Step::Switch { slot }) => ExternalEnemyTurn::Switch(slot),
        Some(Step::Item { id, .. }) => ExternalEnemyTurn::Item(id),
        _ => ExternalEnemyTurn::Attack,
    }
}

/// `AI_TryItem.used_item` after an external trainer's item: empty the first
/// trainer item slot holding `id`, then the bookkeeping the original does
/// (`wEnemyGoesFirst`, Bide / Rage / Fury Cutter / Protect counters,
/// `wLastEnemyCounterMove`).
pub fn external_enemy_item_used(cpu: &mut Cpu, id: u8) {
    use crate::engine::sym as w;
    let de = if cpu.read_byte(w::W_ENEMY_TRAINER_ITEM1) == id { w::W_ENEMY_TRAINER_ITEM1 } else { w::W_ENEMY_TRAINER_ITEM2 };
    cpu.write_byte(de, 0);
    cpu.write_byte(w::W_ENEMY_GOES_FIRST, 1);
    let sub3 = cpu.read_byte(w::W_ENEMY_SUB_STATUS3);
    cpu.write_byte(w::W_ENEMY_SUB_STATUS3, sub3 & !(1 << defs::SUBSTATUS_BIDE));
    cpu.write_byte(w::W_ENEMY_FURY_CUTTER_COUNT, 0);
    cpu.write_byte(w::W_ENEMY_PROTECT_COUNT, 0);
    cpu.write_byte(w::W_ENEMY_RAGE_COUNTER, 0);
    let sub4 = cpu.read_byte(w::W_ENEMY_SUB_STATUS4);
    cpu.write_byte(w::W_ENEMY_SUB_STATUS4, sub4 & !(1 << defs::SUBSTATUS_RAGE));
    cpu.write_byte(w::W_LAST_ENEMY_COUNTER_MOVE, 0);
}

/// "SPECIES (HP x/y)" for OT party slot `slot`.
fn enemy_party_label(cpu: &Cpu, slot: u8) -> String {
    let species = read_ot_party_species(cpu, slot);
    let hp = read_word_be(cpu, ot_party_slot_addr(slot) + PARTY_OFFSET_HP);
    let max_hp = read_word_be(cpu, ot_party_slot_addr(slot) + PARTY_OFFSET_MAX_HP);
    format!("{} (HP {hp}/{max_hp})", species_name(species))
}

/// Print a numbered list under `title` and read a 0-based pick from stdin.
fn prompt_list(title: &str, labels: &[String]) -> usize {
    println!("\n[enemy] {title}");
    for (idx, label) in labels.iter().enumerate() {
        println!("  {}) {label}", idx + 1);
    }
    prompt_index("> ", labels.len())
}

/// Prompt for an external trainer's action on stdin.
fn prompt_enemy_trainer_action(cpu: &Cpu) -> Step {
    print_enemy_state(cpu);
    let legal = legal_enemy_trainer_steps(cpu);
    let labels: Vec<String> = legal
        .iter()
        .map(|step| match *step {
            Step::Move { slot } => {
                let move_id = cpu.read_byte(W_ENEMY_MON_MOVES + u16::from(slot));
                let pp = cpu.read_byte(W_ENEMY_MON_PP + u16::from(slot)) & defs::PP_MASK;
                format!("{} (PP {pp})", move_name(move_id))
            }
            Step::Fight => "FIGHT".to_string(),
            Step::Switch { slot } => format!("Switch to {}", enemy_party_label(cpu, slot)),
            Step::Item { id, .. } => format!("Use {}", names::item_name(id)),
            _ => unreachable!(),
        })
        .collect();
    legal[prompt_list("Enemy trainer: choose an action", &labels)]
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










// ---------------------------------------------------------------------------
// WRAM helpers







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
    if decided_externally(cpu, site) {
        let cell = cpu.decider.as_ref().expect("checked");
        let legal = legal_steps(cpu, site);
        let step = cell.borrow_mut().decide(cpu, site, &legal);
        assert!(legal.contains(&step), "decider chose illegal {step:?} at {site:?}; legal: {legal:?}");
        return step;
    }
    stdin(cpu)
}

/// Whether an installed [`super::decision::Decider`] answers `site`.
fn decided_externally(cpu: &Cpu, site: PromptSite) -> bool {
    cpu.decider.as_ref().is_some_and(|cell| cell.borrow().handles(site))
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
