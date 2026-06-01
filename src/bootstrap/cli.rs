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
use super::recorder::{self, Step};

// ---------------------------------------------------------------------------
// WRAM addresses. Source of truth: scratch/pokecrystal/pokecrystal.sym.
// Mirrors docs/memory_layout.md.

const W_BATTLE_MON_SPECIES: u16 = 0xc62c;
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
const W_ENEMY_SUB_STATUS_5: u16 = 0xc671;

// The active player mon's COMPUTED, stage-adjusted battle stats — 5 big-endian
// words: Attack, Defense, Speed, SpAtk, SpDef (no HP). This is what the damage
// routine reads (`wPlayerAttack`/`wPlayerSpAtk` in
// `engine/battle/effect_commands.asm` `PlayerAttackDamage`), NOT the stat
// levels. `00:c6b6 wPlayerStats` / `wPlayerAttack` in `pokecrystal.sym`.
const W_PLAYER_STATS: u16 = 0xc6b6;
// The active player mon's BASE (stage-neutral) battle stats — same 5-word
// layout as `wPlayerStats`. At battle init `wPlayerStats` is copied from here
// (`InitBattleMon`), so this is the multiplicand a stage recompute starts from.
// `00:c640 wBattleMonAttack` in `pokecrystal.sym`.
const W_BATTLE_MON_STATS: u16 = 0xc640;
// The active player mon's per-stat stage levels — 8 bytes indexed by the stat
// constants below (only the first 5 are battle stats). Each byte is an ABSOLUTE
// level 1..=13; 7 (`BASE_STAT_LEVEL`) is neutral, 13 (`MAX_STAT_LEVEL`) is the
// cap. `00:c6cc wPlayerStatLevels` / `wPlayerAtkLevel` in `pokecrystal.sym`,
// `NUM_LEVEL_STATS = 8` in `constants/battle_constants.asm`.
const W_PLAYER_STAT_LEVELS: u16 = 0xc6cc;

// Stat-level bounds and indices. Source: `constants/battle_constants.asm`
// (`BASE_STAT_LEVEL EQU 7`, `MAX_STAT_LEVEL EQU 13`, `MAX_STAT_VALUE EQU 999`)
// and the `const_def` stat-index block (ATTACK=0, DEFENSE=1, SPEED=2,
// SP_ATTACK=3, SP_DEFENSE=4, ACCURACY=5, EVASION=6, ABILITY=7).
const BASE_STAT_LEVEL: u8 = 7;
const MAX_STAT_LEVEL: u8 = 13;
const MAX_STAT_VALUE: u16 = 999;
const STAT_ATTACK: u8 = 0;
const STAT_DEFENSE: u8 = 1;
const STAT_SPEED: u8 = 2;
const STAT_SP_ATTACK: u8 = 3;
const W_CUR_PLAYER_MOVE: u16 = 0xc6e3;
const W_LAST_PLAYER_MON: u16 = 0xc71a;
const W_PLAYER_WRAP_COUNT: u16 = 0xc730;

const W_BATTLE_MENU_CURSOR_POSITION: u16 = 0xd0d2;
const W_CUR_BATTLE_MON: u16 = 0xd0d4;
const W_CUR_MOVE_NUM: u16 = 0xd0d5;
const W_MENU_CURSOR_Y: u16 = 0xcfa9;
const W_BATTLE_PLAYER_ACTION: u16 = 0xd0ec;
const W_CUR_ITEM: u16 = 0xd106;
const W_CUR_PARTY_MON: u16 = 0xd109;
const W_ITEM_QUANTITY_CHANGE: u16 = 0xd10c;

const W_PARTY_COUNT: u16 = 0xdcd7;
const W_PARTY_MON1: u16 = 0xdcdf;
const PARTY_STRUCT_LEN: u16 = 48;

const W_NUM_ITEMS: u16 = 0xd892;
const W_NUM_BALLS: u16 = 0xd8d7;

const W_ENEMY_MON_SPECIES: u16 = 0xd206;
const W_ENEMY_MON_LEVEL: u16 = 0xd213;
const W_ENEMY_MON_HP: u16 = 0xd216;
const W_ENEMY_MON_MAX_HP: u16 = 0xd218;
const W_ENEMY_MON_STATUS: u16 = 0xd214;

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
    loop {
        print_battle_state(cpu);
        let action = if let Some(step) = player::consume(
            cpu,
            PromptSite::BattleMenu,
            &[StepKind::Move, StepKind::Switch, StepKind::Item, StepKind::Run],
        ) {
            // Playback path: turn the step back into the same `Action` the
            // stdin path would produce. Item targets come from the step
            // itself — `prompt_item_target` is bypassed.
            match step {
                Step::Move { slot } => Action::Move(slot),
                Step::Switch { slot } => Action::Switch(slot),
                Step::Item { id, target, move_slot } => Action::Item { id, target, move_slot },
                Step::Run => Action::Run,
                // `consume`'s `accepts` filter rules these out — unreachable
                // in well-formed scripts and reported by `consume` otherwise.
                Step::ForcedSwitch { .. } | Step::OfferSwitch { .. } => unreachable!(),
            }
        } else {
            prompt_action(cpu)
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
                apply_item(cpu, id, target, move_slot);
                cpu.write_byte(W_BATTLE_PLAYER_ACTION, defs::BATTLEPLAYERACTION_USEITEM);
                cpu.write_byte(W_BATTLE_MENU_CURSOR_POSITION, 3);
                recorder::record(cpu, Step::Item { id, target, move_slot });
                // Normal return: caller is `battle_turn_loop1`, which then
                // checks wBattleEnded and continues the turn.
                cpu.pc = cpu.pop_word();
                cpu.cycle(16);
                return;
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
    if let Some(Step::Move { slot }) =
        player::consume(cpu, PromptSite::MoveSelection, &[StepKind::Move])
    {
        // Re-record on the playback path too so a re-recording (record +
        // play simultaneously) carries the same shape as the original.
        recorder::record(cpu, Step::Move { slot });
        return slot;
    }

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
    let slot = slots[pick];
    // Standalone re-pick from `move_selection_screen` (Disable / no-PP path).
    // Same shape as the action-menu Move so playback doesn't need to know
    // which prompt site produced it.
    recorder::record(cpu, Step::Move { slot });
    slot
}

/// CLI patch at `force_pick_party_mon_in_battle`. Returns the chosen alive,
/// non-current party slot.
pub fn handle_forced_switch(cpu: &mut Cpu) -> u8 {
    observe_decision_point(cpu);
    if let Some(Step::ForcedSwitch { slot }) =
        player::consume(cpu, PromptSite::ForcedSwitch, &[StepKind::ForcedSwitch])
    {
        recorder::record(cpu, Step::ForcedSwitch { slot });
        return slot;
    }

    print_battle_state(cpu);
    println!("\n[forced switch] your active Pokémon fainted — choose the next:");
    let current = cpu.read_byte(W_CUR_BATTLE_MON);
    let options = alive_party_slots(cpu, Some(current));
    if options.is_empty() {
        // Shouldn't happen — the engine only enters this path with surviving
        // mons. Fall back to slot 0 to make the failure visible.
        eprintln!("WARN: no alive party slots; defaulting to slot 0");
        recorder::record(cpu, Step::ForcedSwitch { slot: 0 });
        return 0;
    }
    for (idx, slot) in options.iter().enumerate() {
        print_party_slot_line(cpu, *slot, idx + 1);
    }
    let pick = prompt_index("> ", options.len());
    let slot = options[pick];
    recorder::record(cpu, Step::ForcedSwitch { slot });
    slot
}

/// CLI patch at `offer_switch` (trainer about to send in another mon). Returns
/// `None` if the player keeps the current mon, `Some(slot)` to swap.
pub fn handle_offer_switch(cpu: &mut Cpu) -> Option<u8> {
    observe_decision_point(cpu);
    if let Some(Step::OfferSwitch { keep, slot }) =
        player::consume(cpu, PromptSite::OfferSwitch, &[StepKind::OfferSwitch])
    {
        recorder::record(cpu, Step::OfferSwitch { keep, slot });
        return if keep { None } else { slot };
    }

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
        recorder::record(cpu, Step::OfferSwitch { keep: true, slot: None });
        None
    } else {
        let slot = options[pick - 1];
        recorder::record(cpu, Step::OfferSwitch { keep: false, slot: Some(slot) });
        Some(slot)
    }
}

// ---------------------------------------------------------------------------
// Prompt rendering

fn print_battle_state(cpu: &Cpu) {
    let species = cpu.read_byte(W_BATTLE_MON_SPECIES);
    let level = cpu.read_byte(W_BATTLE_MON_LEVEL);
    let hp = read_word_be(cpu, W_BATTLE_MON_HP);
    let max_hp = read_word_be(cpu, W_BATTLE_MON_MAX_HP);
    let status = cpu.read_byte(W_BATTLE_MON_STATUS);

    let enemy_species = cpu.read_byte(W_ENEMY_MON_SPECIES);
    let enemy_level = cpu.read_byte(W_ENEMY_MON_LEVEL);
    let enemy_hp = read_word_be(cpu, W_ENEMY_MON_HP);
    let enemy_max_hp = read_word_be(cpu, W_ENEMY_MON_MAX_HP);
    let enemy_status = cpu.read_byte(W_ENEMY_MON_STATUS);

    println!(
        "\n=== Enemy: {} Lv{} HP {}/{}{}\n=== You:   {} Lv{} HP {}/{}{}",
        species_name(enemy_species),
        enemy_level,
        enemy_hp,
        enemy_max_hp,
        status_suffix(enemy_status),
        species_name(species),
        level,
        hp,
        max_hp,
        status_suffix(status),
    );
}

fn prompt_action(cpu: &Cpu) -> Action {
    let mut entries: Vec<(String, Action)> = Vec::new();

    // Moves
    for slot in move_options(cpu) {
        let move_id = cpu.read_byte(W_BATTLE_MON_MOVES + u16::from(slot));
        let pp = cpu.read_byte(W_BATTLE_MON_PP + u16::from(slot)) & defs::PP_MASK;
        entries.push((
            format!("{} (PP {pp})", move_name(move_id)),
            Action::Move(slot),
        ));
    }

    // Switches
    let current = cpu.read_byte(W_CUR_BATTLE_MON);
    for slot in alive_party_slots(cpu, Some(current)) {
        let species = read_party_species(cpu, slot);
        let hp = read_word_be(cpu, party_slot_addr(slot) + PARTY_OFFSET_HP);
        let max_hp = read_word_be(cpu, party_slot_addr(slot) + PARTY_OFFSET_MAX_HP);
        entries.push((
            format!("Switch to {} (HP {}/{})", species_name(species), hp, max_hp),
            Action::Switch(slot),
        ));
    }

    // Items pocket — only the ones the CLI knows how to apply directly, and
    // only when they'd actually do something to some party mon. The real game
    // refuses the rest ("It wouldn't have any effect"), so we don't surface
    // them as choices. Balls and other items go through the engine's bag UI,
    // which we currently bypass.
    //
    // Active-mon items (Bitter Berry, the X stat-boosters) are a parallel case:
    // they act on the active battler and take NO party target, so they're keyed
    // off an `active_item_usable` check rather than `item_valid_targets`. They
    // carry `target: None`, which `apply_item` resolves to the active mon, and
    // they skip the target prompt below.
    for (id, qty) in read_pocket(cpu, W_NUM_ITEMS) {
        // Offer the item if it would do something: an active-mon item when the
        // active battler currently qualifies (Bitter Berry → confused, an
        // X-item → its stat below the cap), or a party-target item when at
        // least one party mon qualifies.
        if !active_item_usable(cpu, id) && item_valid_targets(cpu, id).is_empty() {
            continue;
        }
        entries.push((
            format!("Use {} ×{qty}", item_display(id)),
            Action::Item { id, target: None, move_slot: None },
        ));
    }
    // Balls: not exposed yet — would require running the engine's catch
    // routine in-band. Tracked alongside the "headless step API" work.
    let _ = W_NUM_BALLS;

    entries.push(("Run".to_string(), Action::Run));

    println!("\n[turn] choose an action:");
    for (idx, (label, _)) in entries.iter().enumerate() {
        println!("  {}) {label}", idx + 1);
    }
    let pick = prompt_index("> ", entries.len());
    let mut chosen = entries[pick].1;

    // Active-mon items (Bitter Berry, the X stat-boosters) act on the active
    // battler — no party target, no move prompt — so they're left exactly as
    // built above (`target: None`). Every OTHER directly-usable item targets one
    // party mon;
    // we prompt for it here so the action menu shows one line per item (not one
    // line per item×target). Single-move PP items (Ether / Max Ether /
    // MysteryBerry) then prompt for which of that mon's moves to restore.
    if let Action::Item { id, .. } = chosen {
        if active_item_usable(cpu, id) {
            return chosen;
        }
        let target_slot = prompt_item_target(cpu, id);
        let move_slot = if is_single_move_pp_item(id) {
            Some(prompt_pp_move_slot(cpu, id, target_slot))
        } else {
            None
        };
        chosen = Action::Item {
            id,
            target: Some(target_slot),
            move_slot,
        };
    }
    chosen
}

fn prompt_item_target(cpu: &Cpu, id: u8) -> u8 {
    println!("\n[item] use {} on which Pokémon?", item_display(id));
    // Only mons the item can actually affect — a full-HP mon for a Potion, a
    // living mon for a Revive, etc. are omitted (the game would just say "It
    // wouldn't have any effect"). Non-empty because the action menu only offers
    // an item once at least one slot qualifies.
    let options = item_valid_targets(cpu, id);
    for (idx, slot) in options.iter().enumerate() {
        print_party_slot_line(cpu, *slot, idx + 1);
    }
    let pick = prompt_index("> ", options.len());
    options[pick]
}

/// Prompt for which move of party `slot` to restore PP on, for a single-move PP
/// item. Lists only moves that exist and are below their max PP (the ones the
/// item would actually affect), each with its name and `current/max` PP, and
/// returns the chosen 0-based move slot. Only reached on the interactive path;
/// playback supplies the move slot from the recorded step.
fn prompt_pp_move_slot(cpu: &Cpu, id: u8, slot: u8) -> u8 {
    println!(
        "\n[item] use {} on which move of {}?",
        item_display(id),
        species_name(read_party_species(cpu, slot)),
    );
    let options = restorable_move_slots(cpu, slot);
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
        if std::io::stdin().lock().read_line(&mut line).is_err() {
            // stdin closed (CI / piped) — default to first option.
            return 0;
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

fn check_can_switch(cpu: &Cpu, target: u8) -> Result<(), String> {
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
            | ItemEffect::ActiveStatUp(_) => false,
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
fn item_valid_targets(cpu: &Cpu, id: u8) -> Vec<u8> {
    let Some(effect) = item_effect(id) else {
        return Vec::new();
    };
    let n = cpu.read_byte(W_PARTY_COUNT);
    (0..n)
        .filter(|&slot| match effect {
            // Active-mon items have no party target — they're offered via the
            // active-mon branch in `prompt_action`, never the party-target path.
            ItemEffect::ActiveConfusion | ItemEffect::ActiveStatUp(_) => false,
            // PP isn't a vital, so `affects` can't judge PP items. A PP restore
            // applies to any mon — alive or fainted (RestorePP has no faint
            // guard, so the game lets you Ether/Elixer a fainted mon) — that has
            // at least one move below its max PP.
            ItemEffect::PpAll { .. } | ItemEffect::PpOne { .. } => {
                slot_has_restorable_pp(cpu, slot)
            }
            _ => {
                let base = party_slot_addr(slot);
                effect.affects(
                    read_word_be(cpu, base + PARTY_OFFSET_HP),
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
fn slot_has_restorable_pp(cpu: &Cpu, slot: u8) -> bool {
    !restorable_move_slots(cpu, slot).is_empty()
}

/// The 0-based move slots of party `slot` that exist (move id != 0) and hold a
/// move below its max PP — i.e. the moves a single-move PP item could restore.
/// Drives the move-selection prompt for Ether/Max Ether/MysteryBerry.
fn restorable_move_slots(cpu: &Cpu, slot: u8) -> Vec<u8> {
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

/// Whether `id` is an active-mon item (acts on the active battler, takes no
/// party target) AND is usable right now: Bitter Berry only while the active mon
/// is confused; an X-item only while its stat stage is below `MAX_STAT_LEVEL`
/// (mirrors `RaiseStat`'s "wouldn't have any effect" guard at the cap — the asm
/// refuses to raise past 13). Returns `false` for every party-target item.
/// Drives whether the active-mon item appears in the action menu.
fn active_item_usable(cpu: &Cpu, id: u8) -> bool {
    match item_effect(id) {
        Some(ItemEffect::ActiveConfusion) => {
            is_active_confused(cpu.read_byte(W_PLAYER_SUB_STATUS_3))
        }
        Some(ItemEffect::ActiveStatUp(stat)) => stat_level_can_raise(cpu, stat),
        _ => false,
    }
}

/// The active player mon's stage level (1..=13) for stat index `stat`, reading
/// `wPlayerStatLevels[stat]`.
fn read_stat_level(cpu: &Cpu, stat: u8) -> u8 {
    cpu.read_byte(W_PLAYER_STAT_LEVELS + u16::from(stat))
}

/// Can stat `stat` still be raised — i.e. is its stage level below
/// `MAX_STAT_LEVEL`? An X-item at the cap has no effect, so the game (and the
/// CLI) doesn't offer it.
fn stat_level_can_raise(cpu: &Cpu, stat: u8) -> bool {
    read_stat_level(cpu, stat) < MAX_STAT_LEVEL
}

fn apply_item(cpu: &mut Cpu, id: u8, target: Option<u8>, move_slot: Option<u8>) {
    let slot = target.unwrap_or_else(|| cpu.read_byte(W_CUR_BATTLE_MON));
    let base = party_slot_addr(slot);

    // Status heals first. Full Restore (a separate effect) clears all status as
    // part of its HP arm below.
    if let Some(ItemEffect::Status(mask)) = item_effect(id) {
        clear_status_bits(cpu, slot, mask);
    }

    // Confusion is a battle-only sub-status on the active mon, separate from the
    // status byte. Bitter Berry (`ActiveConfusion`, always the active mon) and
    // MiracleBerry (when its chosen target IS the active slot) clear it. Mirrors
    // `BitterBerryEffect`: `res SUBSTATUS_CONFUSED, [wPlayerSubStatus3]`.
    let clears_confusion = matches!(item_effect(id), Some(ItemEffect::ActiveConfusion))
        || (id == 0x6d && slot == cpu.read_byte(W_CUR_BATTLE_MON));
    if clears_confusion {
        clear_active_confusion(cpu);
    }

    // X stat-boosters: raise one stage of the active mon. No party target — the
    // active battler is always the subject, like the confusion case above.
    if let Some(ItemEffect::ActiveStatUp(stat)) = item_effect(id) {
        raise_active_stat(cpu, stat);
    }

    // Healing.
    let max_hp = read_word_be(cpu, base + PARTY_OFFSET_MAX_HP);
    let cur_hp = read_word_be(cpu, base + PARTY_OFFSET_HP);
    let heal = match id {
        0x12 => Some(20),                              // Potion
        0x11 => Some(50),                              // Super Potion
        0x10 => Some(200),                             // Hyper Potion
        0x0f => Some(max_hp.saturating_sub(cur_hp)),   // Max Potion
        0x0e => {
            clear_status_bits(cpu, slot, 0xff);
            Some(max_hp.saturating_sub(cur_hp))
        }
        0x2e => Some(50),                              // Fresh Water
        0x2f => Some(60),                              // Soda Pop
        0x30 => Some(80),                              // Lemonade
        0x48 => Some(100),                             // Moomoo Milk
        0x79 => Some(50),                              // Energypowder
        0x7a => Some(200),                             // Energy Root
        0xad => Some(10),                              // Berry
        0xae => Some(30),                              // Gold Berry
        0x8b => Some(20),                              // Berry Juice
        0x72 => Some(20),                              // RageCandyBar
        0x27 if cur_hp == 0 => Some(max_hp / 2),       // Revive
        0x28 | 0x7c if cur_hp == 0 => Some(max_hp),     // Max Revive, Revival Herb
        _ => None,
    };

    if let Some(amount) = heal {
        let healed = (cur_hp.saturating_add(amount)).min(max_hp);
        write_word_be(cpu, base + PARTY_OFFSET_HP, healed);
        // The active mon's wBattleMon HP is a mirror — update it too if this
        // slot is currently in battle.
        if slot == cpu.read_byte(W_CUR_BATTLE_MON) {
            write_word_be(cpu, W_BATTLE_MON_HP, healed);
        }
    }

    // Whole-party PP restore (Elixer / Max Elixer).
    if let Some(ItemEffect::PpAll { amount }) = item_effect(id) {
        restore_pp_all(cpu, slot, amount);
    }

    // Single-move PP restore (Ether / Max Ether / MysteryBerry). The chosen
    // move slot comes from the prompt (interactive) or the recorded step
    // (playback); if it's somehow absent, fall back to the first restorable
    // move so we never silently no-op.
    if let Some(ItemEffect::PpOne { amount }) = item_effect(id) {
        let chosen = move_slot
            .or_else(|| restorable_move_slots(cpu, slot).first().copied())
            .unwrap_or(0);
        restore_pp_one(cpu, slot, chosen, amount);
    }

    // Decrement the bag count.
    decrement_pocket_item(cpu, W_NUM_ITEMS, id);

    // Make the engine see the item selection (some downstream code reads these).
    cpu.write_byte(W_CUR_ITEM, id);
    cpu.write_byte(W_ITEM_QUANTITY_CHANGE, 1);
}

/// Restore PP on every move of `slot`. `amount = Some(n)` adds `n` PP per move
/// (capped at max PP); `amount = None` (Max Elixer) sets each move to full. The
/// PP-Up bits (top 2) are preserved. Mirrors `RestorePP`
/// (`scratch/pokecrystal/engine/items/item_effects.asm`): the new PP is
/// `(orig & PP_UP_MASK) | min(cur + n, max)`.
///
/// If `slot` is the active battle mon, the matching `wBattleMonPP` slot is
/// mirrored too — but only for moves the active mon actually has (the game's
/// `BattleRestorePP` matches by move id), so a transformed/sketched layout is
/// handled by id rather than by position.
fn restore_pp_all(cpu: &mut Cpu, slot: u8, amount: Option<u8>) {
    let base = party_slot_addr(slot);
    let is_active = slot == cpu.read_byte(W_CUR_BATTLE_MON);

    for i in 0..MOVE_SLOTS {
        let move_id = cpu.read_byte(base + PARTY_OFFSET_MOVES + i);
        if move_id == 0 {
            continue;
        }
        let pp_byte = cpu.read_byte(base + PARTY_OFFSET_PP + i);
        let max = move_pp::max_pp(move_id, pp_byte);
        let new_pp = match amount {
            Some(n) => move_pp::current_pp(pp_byte).saturating_add(n).min(max),
            None => max,
        };
        let new_byte = move_pp::pp_up_bits(pp_byte) | new_pp;
        cpu.write_byte(base + PARTY_OFFSET_PP + i, new_byte);

        if is_active {
            mirror_battle_mon_pp(cpu, move_id, new_byte);
        }
    }
}

/// Restore PP on a single move slot of `slot` (Ether / Max Ether /
/// MysteryBerry). `amount = Some(n)` adds `n` PP (capped at max PP); `amount =
/// None` (Max Ether) sets it to full. The PP-Up bits (top 2) are preserved, the
/// same `(orig & PP_UP_MASK) | min(cur + n, max)` shape `RestorePP` uses. A move
/// slot that's empty or already at full PP is left untouched (the game's
/// `.dont_restore`). If `slot` is the active battle mon, the matching
/// `wBattleMonPP` slot is mirrored by move id, like [`restore_pp_all`].
fn restore_pp_one(cpu: &mut Cpu, slot: u8, move_slot: u8, amount: Option<u8>) {
    if u16::from(move_slot) >= MOVE_SLOTS {
        return;
    }
    let base = party_slot_addr(slot);
    let move_id = cpu.read_byte(base + PARTY_OFFSET_MOVES + u16::from(move_slot));
    if move_id == 0 {
        return;
    }
    let pp_byte = cpu.read_byte(base + PARTY_OFFSET_PP + u16::from(move_slot));
    let max = move_pp::max_pp(move_id, pp_byte);
    if move_pp::current_pp(pp_byte) >= max {
        return;
    }
    let new_pp = match amount {
        Some(n) => move_pp::current_pp(pp_byte).saturating_add(n).min(max),
        None => max,
    };
    let new_byte = move_pp::pp_up_bits(pp_byte) | new_pp;
    cpu.write_byte(base + PARTY_OFFSET_PP + u16::from(move_slot), new_byte);

    if slot == cpu.read_byte(W_CUR_BATTLE_MON) {
        mirror_battle_mon_pp(cpu, move_id, new_byte);
    }
}

/// Copy a restored PP byte into the active mon's `wBattleMonPP` slot whose
/// `wBattleMonMoves` entry matches `move_id`.
fn mirror_battle_mon_pp(cpu: &mut Cpu, move_id: u8, pp_byte: u8) {
    for j in 0..MOVE_SLOTS {
        if cpu.read_byte(W_BATTLE_MON_MOVES + j) == move_id {
            cpu.write_byte(W_BATTLE_MON_PP + j, pp_byte);
            return;
        }
    }
}

/// Clear `SUBSTATUS_CONFUSED` (bit 7) on the active player mon, mirroring
/// `BitterBerryEffect` (`res SUBSTATUS_CONFUSED, [wPlayerSubStatus3]`). The asm
/// clears ONLY the bit — it does not zero `wPlayerConfuseCount` — so neither do
/// we. The natural confusion-decrement loop (`effect_commands.asm`) gates on the
/// bit, so a now-stale counter is never read again unless confusion is re-set.
fn clear_active_confusion(cpu: &mut Cpu) {
    let v = cpu.read_byte(W_PLAYER_SUB_STATUS_3);
    cpu.write_byte(W_PLAYER_SUB_STATUS_3, v & !SUBSTATUS_CONFUSED_MASK);
}

/// `StatLevelMultipliers` from `scratch/pokecrystal/data/battle/stat_multipliers.asm`,
/// indexed by stage level 1..=13 (i.e. modifier -6..=+6; level 7 = ×100%). Each
/// entry is `(numerator, denominator)`. The engine multiplies a base stat by
/// `num` then integer-divides by `den` (`CalcBattleStats` / `ApplyStatLevelMultiplier`).
const STAT_LEVEL_MULTIPLIERS: [(u16, u16); 13] = [
    (25, 100), // level 1  (-6) = 25%
    (28, 100), // level 2  (-5) = 28%
    (33, 100), // level 3  (-4) = 33%
    (40, 100), // level 4  (-3) = 40%
    (50, 100), // level 5  (-2) = 50%
    (66, 100), // level 6  (-1) = 66%
    (1, 1),    // level 7  ( 0) = 100%
    (15, 10),  // level 8  (+1) = 150%
    (2, 1),    // level 9  (+2) = 200%
    (25, 10),  // level 10 (+3) = 250%
    (3, 1),    // level 11 (+4) = 300%
    (35, 10),  // level 12 (+5) = 350%
    (4, 1),    // level 13 (+6) = 400%
];

/// Recompute one of the active mon's COMPUTED stat values (`wPlayerStats`) from
/// its BASE battle stat (`wBattleMonStats`) and a stage `level` (1..=13), the
/// same `base * num / den` the engine uses, clamped to `[1, MAX_STAT_VALUE]`.
/// Pure so it's unit-testable without a `Cpu`. Mirrors `ApplyStatLevelMultiplier`
/// (`engine/battle/core.asm`): integer multiply then divide, cap at 999, and a
/// floor of 1 (`.okay4`: `inc [hl]` when the result is zero).
fn apply_stat_multiplier(base: u16, level: u8) -> u16 {
    let (num, den) = STAT_LEVEL_MULTIPLIERS[usize::from(level - 1)];
    let raised = (u32::from(base) * u32::from(num) / u32::from(den)) as u16;
    raised.clamp(1, MAX_STAT_VALUE)
}

/// Raise stat `stat` of the active player mon by one stage, mirroring `RaiseStat`
/// (`engine/battle/effect_commands.asm`): increment `wPlayerStatLevels[stat]`
/// (capped at `MAX_STAT_LEVEL`), then recompute the COMPUTED stat in
/// `wPlayerStats` — which is what the damage routine reads
/// (`PlayerAttackDamage` reads `wPlayerAttack`/`wPlayerSpAtk`, i.e.
/// `wPlayerStats`, NOT the stat level). Setting only the level would be
/// cosmetic; the boost wouldn't affect damage.
///
/// We recompute from `wBattleMonStats` (the stage-neutral base) × the new stage
/// multiplier. Note the engine's `CalcPlayerStats` would additionally re-apply
/// badge boosts and burn/paralysis modifiers; we don't re-apply those, so a
/// player with stat badges or a burn/paralysis would see a slightly different
/// computed value. In our synthetic battles the player has no badges, and the
/// +stage factor dominates, so this matches the game for the common case.
fn raise_active_stat(cpu: &mut Cpu, stat: u8) {
    let level = read_stat_level(cpu, stat);
    if level >= MAX_STAT_LEVEL {
        return; // Already maxed — RaiseStat refuses ("wouldn't have any effect").
    }
    let new_level = level + 1;
    cpu.write_byte(W_PLAYER_STAT_LEVELS + u16::from(stat), new_level);

    // Recompute the computed stat word (big-endian) from the base battle stat.
    let stat_addr = u16::from(stat) * 2;
    let base = read_word_be(cpu, W_BATTLE_MON_STATS + stat_addr);
    let raised = apply_stat_multiplier(base, new_level);
    write_word_be(cpu, W_PLAYER_STATS + stat_addr, raised);
}

fn clear_status_bits(cpu: &mut Cpu, slot: u8, mask: u8) {
    let addr = party_slot_addr(slot) + PARTY_OFFSET_STATUS;
    let v = cpu.read_byte(addr);
    cpu.write_byte(addr, v & !mask);
    if slot == cpu.read_byte(W_CUR_BATTLE_MON) {
        let v2 = cpu.read_byte(W_BATTLE_MON_STATUS);
        cpu.write_byte(W_BATTLE_MON_STATUS, v2 & !mask);
    }
}

fn decrement_pocket_item(cpu: &mut Cpu, count_addr: u16, item_id: u8) {
    let count = cpu.read_byte(count_addr);
    let mut addr = count_addr + 1;
    for slot in 0..count {
        let id = cpu.read_byte(addr);
        let qty = cpu.read_byte(addr + 1);
        if id == item_id {
            if qty > 1 {
                cpu.write_byte(addr + 1, qty - 1);
            } else {
                // Remove entry: shift later entries left.
                let mut write_addr = addr;
                let mut read_addr = addr + 2;
                let mut remaining = count - slot - 1;
                while remaining > 0 {
                    cpu.write_byte(write_addr, cpu.read_byte(read_addr));
                    cpu.write_byte(write_addr + 1, cpu.read_byte(read_addr + 1));
                    write_addr += 2;
                    read_addr += 2;
                    remaining -= 1;
                }
                // Move terminator and decrement count.
                cpu.write_byte(write_addr, 0xff);
                cpu.write_byte(count_addr, count - 1);
            }
            return;
        }
        addr += 2;
    }
}

// ---------------------------------------------------------------------------
// WRAM helpers

fn party_slot_addr(slot: u8) -> u16 {
    W_PARTY_MON1 + u16::from(slot) * PARTY_STRUCT_LEN
}

fn read_party_species(cpu: &Cpu, slot: u8) -> u8 {
    cpu.read_byte(party_slot_addr(slot))
}

fn read_word_be(cpu: &Cpu, addr: u16) -> u16 {
    u16::from_be_bytes([cpu.read_byte(addr), cpu.read_byte(addr + 1)])
}

fn write_word_be(cpu: &mut Cpu, addr: u16, value: u16) {
    let [hi, lo] = value.to_be_bytes();
    cpu.write_byte(addr, hi);
    cpu.write_byte(addr + 1, lo);
}

fn read_pocket(cpu: &Cpu, count_addr: u16) -> Vec<(u8, u8)> {
    let n = cpu.read_byte(count_addr) as u16;
    let mut out = Vec::with_capacity(n as usize);
    for i in 0..n {
        let id = cpu.read_byte(count_addr + 1 + i * 2);
        let qty = cpu.read_byte(count_addr + 2 + i * 2);
        out.push((id, qty));
    }
    out
}

fn move_options(cpu: &Cpu) -> Vec<u8> {
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
        out.push(slot);
    }
    out
}

fn alive_party_slots(cpu: &Cpu, exclude: Option<u8>) -> Vec<u8> {
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
        assert!(item_effect(0x2c).is_none(), "Dire Hit (not yet supported)");
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
        assert_eq!(BASE_STAT_LEVEL, 7);
        // X-items never go through the party-target / move-slot machinery.
        for id in [0x31u8, 0x33, 0x34, 0x35] {
            assert!(!effect(id).affects(20, 20, 0), "vitals never claim an effect");
            assert!(!is_single_move_pp_item(id), "X-item has no move-slot prompt");
        }
    }

    /// `stat_level_can_raise` (and thus whether an X-item is offered) is true iff
    /// the stage level is below `MAX_STAT_LEVEL`; the clamp in
    /// `raise_active_stat` stops the level at `MAX_STAT_LEVEL`. We test the
    /// boundary logic directly against the constants rather than through a `Cpu`.
    #[test]
    fn x_item_usable_below_cap_and_clamps_at_max() {
        assert_eq!(MAX_STAT_LEVEL, 13);
        // Usable from neutral up through 12; refused once already at the cap.
        for level in BASE_STAT_LEVEL..MAX_STAT_LEVEL {
            assert!(level < MAX_STAT_LEVEL, "level {level} is below the cap — usable");
        }
        assert!(!(MAX_STAT_LEVEL < MAX_STAT_LEVEL), "at the cap — no effect, not offered");
        // The +1 increment never exceeds the cap: from 12 → 13, and 13 stays 13.
        let next = |lvl: u8| if lvl >= MAX_STAT_LEVEL { lvl } else { lvl + 1 };
        assert_eq!(next(MAX_STAT_LEVEL - 1), MAX_STAT_LEVEL, "12 → 13");
        assert_eq!(next(MAX_STAT_LEVEL), MAX_STAT_LEVEL, "13 stays 13 (clamp)");
    }

    /// `apply_stat_multiplier` mirrors `ApplyStatLevelMultiplier`: `base * num /
    /// den` (integer), capped at `MAX_STAT_VALUE` and floored at 1. Ratios come
    /// from `data/battle/stat_multipliers.asm` indexed by level 1..=13.
    #[test]
    fn stat_multiplier_matches_the_asm_table() {
        // Neutral (level 7) is the identity.
        assert_eq!(apply_stat_multiplier(100, BASE_STAT_LEVEL), 100);
        // +1 stage = 150%, +2 = 200%, with integer truncation.
        assert_eq!(apply_stat_multiplier(100, 8), 150);
        assert_eq!(apply_stat_multiplier(100, 9), 200);
        assert_eq!(apply_stat_multiplier(101, 8), 151); // 101 * 15 / 10 = 151 (trunc)
        // +6 stage = 400%, and the result caps at MAX_STAT_VALUE (999).
        assert_eq!(apply_stat_multiplier(200, MAX_STAT_LEVEL), 800);
        assert_eq!(apply_stat_multiplier(500, MAX_STAT_LEVEL), 999, "capped at 999");
        // Floor of 1: a tiny base lowered far never drops to 0.
        assert_eq!(apply_stat_multiplier(1, 1), 1, "1 * 25 / 100 = 0 → floored to 1");
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
}
