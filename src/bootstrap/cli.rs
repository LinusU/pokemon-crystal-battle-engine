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

const W_ENEMY_SUB_STATUS_5: u16 = 0xc671;
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
const PARTY_OFFSET_STATUS: u16 = 32;
const PARTY_OFFSET_HP: u16 = 34;
const PARTY_OFFSET_MAX_HP: u16 = 36;

// ---------------------------------------------------------------------------

/// The action the player picked from the CLI menu. Used internally; the patch
/// at `battle_menu` calls [`handle_battle_menu`] which dispatches it.
#[derive(Debug, Clone, Copy)]
enum Action {
    Move(u8),
    Switch(u8),
    Item { id: u8, target: Option<u8> },
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
                Step::Item { id, target } => Action::Item { id, target },
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
            Action::Item { id, target } => {
                apply_item(cpu, id, target);
                cpu.write_byte(W_BATTLE_PLAYER_ACTION, defs::BATTLEPLAYERACTION_USEITEM);
                cpu.write_byte(W_BATTLE_MENU_CURSOR_POSITION, 3);
                recorder::record(cpu, Step::Item { id, target });
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
    for (id, qty) in read_pocket(cpu, W_NUM_ITEMS) {
        if item_valid_targets(cpu, id).is_empty() {
            continue;
        }
        entries.push((
            format!("Use {} ×{qty}", item_display(id)),
            Action::Item { id, target: None },
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

    // Every directly-usable item targets one party mon. We prompt for it here
    // so the action menu shows one line per item (not one line per item×target).
    if let Action::Item { id, .. } = chosen {
        let target_slot = prompt_item_target(cpu, id);
        chosen = Action::Item {
            id,
            target: Some(target_slot),
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
/// Items not classified here aren't offered in the action menu (balls, X-items,
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
}

impl ItemEffect {
    /// Would applying this item to a mon with these vitals change anything?
    /// Mirrors the game's "It wouldn't have any effect" guard, so the CLI only
    /// offers items — and target slots — that actually do something.
    fn affects(self, hp: u16, max_hp: u16, status: u8) -> bool {
        match self {
            ItemEffect::Hp => hp != 0 && hp < max_hp,
            ItemEffect::Revive => hp == 0,
            ItemEffect::Status(mask) => hp != 0 && status & mask != 0,
            ItemEffect::FullRestore => hp != 0 && (hp < max_hp || status != 0),
        }
    }
}

/// Classify an item id into the effect the CLI knows how to apply, or `None`
/// for items we don't surface in the action menu. Status masks follow
/// `scratch/pokecrystal/data/items/heal_status.asm`.
fn item_effect(id: u8) -> Option<ItemEffect> {
    Some(match id {
        // HP-restoring: Potion, Super, Hyper, Max, drinks, Moomoo Milk, herbs.
        0x12 | 0x11 | 0x10 | 0x0f | 0x2e | 0x2f | 0x30 | 0x48 | 0x79 | 0x7a => {
            ItemEffect::Hp
        }
        // Revive, Max Revive.
        0x27 | 0x28 => ItemEffect::Revive,
        // Full Restore — HP plus all status.
        0x0e => ItemEffect::FullRestore,
        // Status heals.
        0x09 => ItemEffect::Status(1 << defs::PSN as u8), // Antidote — poison
        0x0a => ItemEffect::Status(1 << defs::BRN as u8), // Burn Heal
        0x0b => ItemEffect::Status(1 << defs::FRZ as u8), // Ice Heal
        0x0c => ItemEffect::Status(defs::SLP_MASK),       // Awakening — sleep counter
        0x0d => ItemEffect::Status(1 << defs::PAR as u8), // Parlyz Heal
        0x26 => ItemEffect::Status(0xff),                 // Full Heal — any status
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
        .filter(|&slot| {
            let base = party_slot_addr(slot);
            effect.affects(
                read_word_be(cpu, base + PARTY_OFFSET_HP),
                read_word_be(cpu, base + PARTY_OFFSET_MAX_HP),
                cpu.read_byte(base + PARTY_OFFSET_STATUS),
            )
        })
        .collect()
}

fn apply_item(cpu: &mut Cpu, id: u8, target: Option<u8>) {
    let slot = target.unwrap_or_else(|| cpu.read_byte(W_CUR_BATTLE_MON));
    let base = party_slot_addr(slot);

    // Status heals first. Full Restore (a separate effect) clears all status as
    // part of its HP arm below.
    if let Some(ItemEffect::Status(mask)) = item_effect(id) {
        clear_status_bits(cpu, slot, mask);
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
        0x27 if cur_hp == 0 => Some(max_hp / 2),       // Revive
        0x28 if cur_hp == 0 => Some(max_hp),           // Max Revive
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

    // Decrement the bag count.
    decrement_pocket_item(cpu, W_NUM_ITEMS, id);

    // Make the engine see the item selection (some downstream code reads these).
    cpu.write_byte(W_CUR_ITEM, id);
    cpu.write_byte(W_ITEM_QUANTITY_CHANGE, 1);
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
    fn revives_need_a_fainted_mon() {
        let revive = effect(0x27);
        assert!(revive.affects(0, 20, 0), "fainted — revives");
        assert!(!revive.affects(10, 20, 0), "alive — no effect");
        assert!(!revive.affects(20, 20, 0), "full HP — no effect");
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
        assert!(item_effect(0x31).is_none(), "X Attack");
    }
}
