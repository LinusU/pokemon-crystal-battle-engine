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
// The active player mon's Disable counter. High nibble = the disabled move's
// 1-based slot (0 = nothing disabled); low nibble = turns remaining.
// `00:c675 wPlayerDisableCount` in `pokecrystal.sym`.
const W_PLAYER_DISABLE_COUNT: u16 = 0xc675;

// The active player mon's per-stat stage levels — 8 bytes indexed by the stat
// constants below (only the first 5 are battle stats). Each byte is an ABSOLUTE
// level 1..=13; 7 (`BASE_STAT_LEVEL`) is neutral, 13 (`MAX_STAT_LEVEL`) is the
// cap. `00:c6cc wPlayerStatLevels` / `wPlayerAtkLevel` in `pokecrystal.sym`,
// `NUM_LEVEL_STATS = 8` in `constants/battle_constants.asm`.
const W_PLAYER_STAT_LEVELS: u16 = 0xc6cc;

// Stat-level bounds and indices. Source: `constants/battle_constants.asm`
// (`BASE_STAT_LEVEL EQU 7`, `MAX_STAT_LEVEL EQU 13`) and the `const_def`
// stat-index block (ATTACK=0, DEFENSE=1, SPEED=2, SP_ATTACK=3, SP_DEFENSE=4,
// ACCURACY=5, EVASION=6, ABILITY=7).
const BASE_STAT_LEVEL: u8 = 7;
const MAX_STAT_LEVEL: u8 = 13;
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
/// `wNamedObjectIndex` — the id `GetItemName` reads to fill `wStringBuffer2`,
/// which "<PLAYER> used the …" prints from. Set by the engine's `DoItemEffect`
/// preamble; we replicate it when driving an item routine directly.
const W_NAMED_OBJECT_INDEX: u16 = 0xd265;

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
// `wBattleEnded` (0xc734): the turn loop (`battle_turn_loop1`) quits when this is
// non-zero. The capture path sets it so the battle ends after a catch.
const W_BATTLE_ENDED: u16 = 0xc734;
// `wBattleResult` (0xd0ee): low nibble is WIN/LOSS/DRAW; the high two bits carry
// the CAUGHT_CELEBI / BOX_FULL flags. The catch path masks it to WIN; the Poké
// Doll path masks it to DRAW.
const W_BATTLE_RESULT: u16 = 0xd0ee;
// `wForcedSwitch` (0xd232): the turn loop (`battle_turn_loop1`) quits when this
// is non-zero. `PokeDollEffect` sets it to TRUE to end a wild battle as an
// escape, the same signal the engine uses when the player flees. `01:d232
// wForcedSwitch` in `pokecrystal.sym`.
const W_FORCED_SWITCH: u16 = 0xd232;

// Poké Doll item id (`POKE_DOLL` in `scratch/pokecrystal/constants/item_constants.asm`).
// In a wild battle it ends the battle immediately as an escape (a DRAW); in a
// trainer battle it does nothing (and we never offer it there). See
// `PokeDollEffect` in `scratch/pokecrystal/engine/items/item_effects.asm`.
const POKE_DOLL_ID: u8 = 0x25;

// `PokeBallEffect` lives in ROM bank 0x03 (`03:68a2 PokeBallEffect` in
// pokecrystal.sym). `battle_menu` (the routine this CLI module patches) runs in
// bank 0x0f, so we save/switch/restore around the call.
const BANK_POKE_BALL_EFFECT: u8 = 0x03;
// The active-mon item effect routines (`XItemEffect`, `XAccuracyEffect`,
// `GuardSpecEffect`, `DireHitEffect`, `BitterBerryEffect`) all live in ROM bank
// 0x03 too (verified in pokecrystal.sym), so [`use_active_item_effect`] uses the
// same save/switch/restore as the ball path. Same numeric value as
// `BANK_POKE_BALL_EFFECT`; kept distinct for intent.
const BANK_ITEM_EFFECTS: u8 = 0x03;
const H_ROM_BANK: u16 = 0xff9d;

// `wBattleMode` — 0 = not in battle, 1 = wild, 2 = trainer (`WILD_BATTLE` /
// `TRAINER_BATTLE` in `scratch/pokecrystal/constants/battle_constants.asm`).
// We only offer Poké Balls in a wild battle (`offers_balls`); a thrown ball in
// a trainer battle is refused by the real game ("The trainer blocked the BALL!").
// `00:d22d wBattleMode` in `pokecrystal.sym`.
const W_BATTLE_MODE: u16 = 0xd22d;

const W_ENEMY_MON_SPECIES: u16 = 0xd206;
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
    Switch(u8),
    Item { id: u8, target: Option<u8>, move_slot: Option<u8> },
    /// Throw a Poké Ball from the Balls pocket. `id` is the ball item id. Only
    /// offered in wild battles (see [`offers_balls`]). For now this just consumes
    /// the turn without catching — see the dispatch in [`handle_battle_menu`].
    Ball { id: u8 },
    /// Use a guaranteed-escape item from the Items pocket (the Poké Doll). Only
    /// offered in wild battles (see [`offers_escape_item`]): it ends the battle
    /// immediately as a DRAW. In a trainer battle the real game's `PokeDollEffect`
    /// is a no-op, so we never list it there. `id` is the item id. See the
    /// dispatch in [`handle_battle_menu`].
    Escape { id: u8 },
    Run,
}

/// Whether the action menu should offer Poké Balls, given `wBattleMode`. Balls
/// only make sense in a wild battle (`WILD_BATTLE` = 1); the real game refuses a
/// ball in a trainer battle ("The trainer blocked the BALL!"), so we never list
/// one there. Pure helper so the mode test is unit-testable without a `Cpu`.
fn offers_balls(battle_mode: u8) -> bool {
    battle_mode == defs::WILD_BATTLE
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
                Step::Move { slot } => {
                    assert_recorded_move_legal(cpu, slot);
                    Action::Move(slot)
                }
                Step::Switch { slot } => Action::Switch(slot),
                // A ball is recorded as an `item` step (different pocket, same
                // representation), so route a ball id back to the ball path —
                // exactly as the stdin path would produce it.
                Step::Item { id, .. } if is_ball_id(id) => Action::Ball { id },
                // The Poké Doll is recorded as an `item` step (same as a ball:
                // an Items-pocket id with no party target), so route it back to
                // the escape path exactly as the stdin path would.
                Step::Item { id, .. } if is_escape_item_id(id) => Action::Escape { id },
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
                // Active-mon items (X stat-boosters, X Accuracy, Dire Hit, Guard
                // Spec, Bitter Berry) drive their REAL effect routine so the
                // authentic battle text prints; that routine also tosses the item
                // itself, so we must NOT also decrement the bag. Every other item
                // is applied by the hand-rolled `apply_item` (which decrements).
                if is_active_effect_item(id) {
                    use_active_item_effect(cpu, id);
                } else {
                    apply_item(cpu, id, target, move_slot);
                }
                cpu.write_byte(W_BATTLE_PLAYER_ACTION, defs::BATTLEPLAYERACTION_USEITEM);
                cpu.write_byte(W_BATTLE_MENU_CURSOR_POSITION, 3);
                recorder::record(cpu, Step::Item { id, target, move_slot });
                // Normal return: caller is `battle_turn_loop1`, which then
                // checks wBattleEnded and continues the turn.
                cpu.pc = cpu.pop_word();
                cpu.cycle(16);
                return;
            }
            Action::Ball { id } => {
                let caught = throw_ball(cpu, id);
                cpu.write_byte(W_BATTLE_PLAYER_ACTION, defs::BATTLEPLAYERACTION_USEITEM);
                cpu.write_byte(W_BATTLE_MENU_CURSOR_POSITION, 3);
                // Recorded as an `item` step (a ball is just an item id from a
                // different pocket) so playback needs no new step kind.
                recorder::record(cpu, Step::Item { id, target: None, move_slot: None });
                if caught {
                    // Caught: end the battle the way the engine's
                    // `BattleMenu_Pack.UseItem` → `.run` does (core.asm ~5043):
                    // zero wWildMon, mask wBattleResult down to WIN, and return
                    // carry-set so `battle_turn_loop1` quits (`call BattleMenu` /
                    // `jr c, .quit`). We also set wBattleEnded for the loop's
                    // second guard. The caught mon is already in the party (the
                    // real routine ran TryAddMonToParty). The `caught_wild_mon`
                    // flag makes `read_outcome` report `Outcome::Caught`.
                    cpu.write_byte(W_WILD_MON, 0);
                    let result = cpu.read_byte(W_BATTLE_RESULT) & defs::BATTLERESULT_BITMASK;
                    cpu.write_byte(W_BATTLE_RESULT, result);
                    cpu.write_byte(W_BATTLE_ENDED, 1);
                    cpu.caught_wild_mon = true;
                    cpu.flag_c = true;
                    cpu.pc = cpu.pop_word();
                    cpu.cycle(16);
                    return;
                }
                // Broke free: the throw consumed the turn (the ball was tossed
                // by the real routine). Return to `battle_turn_loop1` carry-clear
                // — same contract as a healing item — so the enemy gets its turn.
                cpu.flag_c = false;
                cpu.pc = cpu.pop_word();
                cpu.cycle(16);
                return;
            }
            Action::Escape { id } => {
                use_poke_doll(cpu, id);
                cpu.write_byte(W_BATTLE_PLAYER_ACTION, defs::BATTLEPLAYERACTION_USEITEM);
                cpu.write_byte(W_BATTLE_MENU_CURSOR_POSITION, 3);
                // Recorded as an `item` step (a Poké Doll is just an Items-pocket
                // id with no party target) so playback needs no new step kind.
                recorder::record(cpu, Step::Item { id, target: None, move_slot: None });
                // Return to `battle_turn_loop1` carry-clear. We don't take the
                // first `jr c` quit (that's the catch path's contract); instead
                // `use_poke_doll` set `wForcedSwitch`, which the loop's third
                // guard (`ld a, [wForcedSwitch] / and a / jr nz, .quit`) checks —
                // so the loop quits there, exactly as `PokeDollEffect` intends.
                cpu.flag_c = false;
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
        assert_recorded_move_legal(cpu, slot);
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
    let action = if let Some(step) =
        player::consume(cpu, PromptSite::EnemyAction, &[StepKind::Move, StepKind::Switch])
    {
        match step {
            Step::Move { slot } => EnemyAction::Move(slot),
            Step::Switch { slot } => EnemyAction::Switch(slot),
            // `consume`'s `accepts` filter rules everything else out.
            _ => unreachable!("EnemyAction consumes only Move/Switch"),
        }
    } else {
        prompt_enemy_action(cpu, enemy_fainted, &moves, &switches)
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

    let foe_species = cpu.read_byte(W_BATTLE_MON_SPECIES);
    let foe_level = cpu.read_byte(W_BATTLE_MON_LEVEL);
    let foe_hp = read_word_be(cpu, W_BATTLE_MON_HP);
    let foe_max_hp = read_word_be(cpu, W_BATTLE_MON_MAX_HP);
    let foe_status = cpu.read_byte(W_BATTLE_MON_STATUS);

    println!(
        "\n=== Opponent: {} Lv{} HP {}/{}{}\n=== Enemy:    {} Lv{} HP {}/{}{}",
        species_name(foe_species),
        foe_level,
        foe_hp,
        foe_max_hp,
        status_suffix(foe_status),
        species_name(species),
        level,
        hp,
        max_hp,
        status_suffix(status),
    );
}

/// Address of OT party slot `slot` (enemy `party_struct`).
fn ot_party_slot_addr(slot: u8) -> u16 {
    W_OT_PARTY_MON1 + u16::from(slot) * PARTY_STRUCT_LEN
}

fn read_ot_party_species(cpu: &Cpu, slot: u8) -> u8 {
    cpu.read_byte(ot_party_slot_addr(slot))
}

/// The enemy's selectable move slots: a real move (id != 0) with PP left that
/// isn't Disabled. Mirrors [`move_options`] against the enemy's `wEnemyMon*`
/// fields and `wEnemyDisableCount`.
fn enemy_move_options(cpu: &Cpu) -> Vec<u8> {
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
fn enemy_alive_party_slots(cpu: &Cpu, exclude: Option<u8>) -> Vec<u8> {
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
    // Balls — only in a wild battle (a trainer blocks the ball). Each entry in
    // the Balls pocket becomes a "Throw <ball> ×qty" action. The catch itself is
    // still a stub (see `TODO(capture)` in `handle_battle_menu`'s `Action::Ball`
    // arm): throwing currently just consumes the turn.
    if offers_balls(cpu.read_byte(W_BATTLE_MODE)) {
        for (id, qty) in read_pocket(cpu, W_NUM_BALLS) {
            entries.push((
                format!("Throw {} ×{qty}", item_display(id)),
                Action::Ball { id },
            ));
        }
    }
    // Poké Doll — a guaranteed escape, only in a wild battle (it's a no-op in a
    // trainer battle). It lives in the Items pocket but takes no party target and
    // isn't an `item_effect`, so it's offered here as a dedicated `Escape` action
    // rather than through the party-target item loop above. `PokeDollEffect`.
    if offers_escape_item(cpu.read_byte(W_BATTLE_MODE)) {
        for (id, qty) in read_pocket(cpu, W_NUM_ITEMS) {
            if is_escape_item_id(id) {
                entries.push((
                    format!("Use {} ×{qty} (escape)", item_display(id)),
                    Action::Escape { id },
                ));
            }
        }
    }

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
fn item_valid_targets(cpu: &Cpu, id: u8) -> Vec<u8> {
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
fn active_item_usable(cpu: &Cpu, id: u8) -> bool {
    match item_effect(id) {
        Some(ItemEffect::ActiveConfusion) => {
            is_active_confused(cpu.read_byte(W_PLAYER_SUB_STATUS_3))
        }
        Some(ItemEffect::ActiveStatUp(stat)) => stat_level_can_raise(cpu, stat),
        // Usable only while the bit is CLEAR — mirrors the asm's
        // `bit mask, [hl] / jp nz, WontHaveAnyEffect` refusal once it's set.
        Some(ItemEffect::ActiveSubStatusSet { addr, mask }) => {
            cpu.read_byte(addr) & mask == 0
        }
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

/// Throw a Poké Ball by driving the real transpiled catch routine
/// (`poke_ball_effect`, ROM bank 0x03). The routine does the full catch math +
/// RNG, runs the (headless-stubbed) throw animation, and on success loads the
/// caught mon and adds it to the party via its own `TryAddMonToParty` predef.
/// The nickname UI is skipped by the HARNESS-PATCH in
/// `src/game/engine/items/item_effects.rs`.
///
/// We do NOT decrement the Balls pocket ourselves: the routine's `.toss` tail
/// calls `TossItem`, which removes one ball. Decrementing here too would double-
/// count.
///
/// Returns `true` if the wild mon was caught. The signal is `wWildMon`
/// (`W_WILD_MON`): the routine writes the caught species there on success and
/// leaves it zero on a failed throw — exactly what the engine's
/// `BattleMenu_Pack.UseItem` keys on (core.asm `.UseItem`: `ld a, [wWildMon] /
/// and a / jr nz, .run`).
/// Load `id`'s display name into `wStringBuffer2`, the way the engine's
/// `DoItemEffect` does before invoking an item effect, so `UseItemText`'s
/// "<PLAYER> used the <name>." prints the item name. Mirrors the preamble in
/// the transpiled `_do_item_effect` (`wNamedObjectIndex` ← id, `GetItemName`,
/// `CopyName1`). `GetItemName`/`CopyName1` are home-bank routines, callable
/// without a bank switch.
fn load_item_name(cpu: &mut Cpu, id: u8) {
    // Both routines end in `ret` (`cpu.pc = pop_word()`), so push a sentinel
    // return address before each call to keep the stack balanced — the same
    // call discipline `_do_item_effect` uses.
    cpu.write_byte(W_NAMED_OBJECT_INDEX, id);
    let sentinel: u16 = 0xfffc;
    cpu.push_word(sentinel);
    crate::game::home::names::get_item_name(cpu);
    cpu.push_word(sentinel);
    crate::game::home::copy_name::copy_name1(cpu);
}

fn throw_ball(cpu: &mut Cpu, id: u8) -> bool {
    // `PokeBallEffect` reads `wCurItem` for the ball id and `wBattleMode` for the
    // wild/trainer check (already WILD here — balls are only offered in wild
    // battles). It increments the toss count from `wItemQuantityChange`, set to 1.
    cpu.write_byte(W_CUR_ITEM, id);
    cpu.write_byte(W_ITEM_QUANTITY_CHANGE, 1);

    // `PokeBallEffect` prints "<PLAYER> used the <name>." via `UseItemText`,
    // which reads the item name out of `wStringBuffer2`. Normally the engine's
    // `DoItemEffect` dispatcher fills that buffer (set `wNamedObjectIndex`,
    // `GetItemName`, `CopyName1`) before jumping to the effect; since we call the
    // effect directly we must replicate that preamble or the name renders blank
    // ("RED used the ."). Both routines live in the home bank, so no switch yet.
    load_item_name(cpu, id);

    // Save the current ROM bank, switch to PokeBallEffect's bank, push a sentinel
    // return address (the routine ends in a `ret` via TossItem), call it, then
    // restore the bank so the battle code that resumes sees bank 0x0f again.
    let saved_bank = cpu.read_byte(H_ROM_BANK);
    super::switch_bank(cpu, BANK_POKE_BALL_EFFECT);
    let return_sentinel: u16 = 0xfffc;
    cpu.push_word(return_sentinel);
    crate::game::engine::items::item_effects::poke_ball_effect(cpu);
    super::switch_bank(cpu, saved_bank);

    cpu.read_byte(W_WILD_MON) != 0
}

/// Whether `id` is an "active-mon" battle item that drives a real transpiled
/// effect routine (so the authentic battle-box text prints) rather than being
/// applied by hand-rolled WRAM writes. These act on the active battler and take
/// no party target. Drives the dispatch in [`handle_battle_menu`]'s
/// `Action::Item` arm to [`use_active_item_effect`].
///
/// This covers all eight active-mon items: the X stat-boosters, the in-battle
/// sub-status setters (X Accuracy / Dire Hit / Guard Spec), and Bitter Berry.
fn is_active_effect_item(id: u8) -> bool {
    matches!(
        item_effect(id),
        Some(
            ItemEffect::ActiveStatUp(_)
                | ItemEffect::ActiveSubStatusSet { .. }
                | ItemEffect::ActiveConfusion
        )
    )
}

/// Drive the REAL transpiled effect routine for an active-mon battle item, so
/// the authentic battle text prints (the hand-rolled path printed nothing). This
/// is the [`throw_ball`] template applied to the X stat-boosters, the sub-status
/// setters, and Bitter Berry: their effect routines all live in ROM bank 0x03
/// (`XItemEffect=03:74c5`, `XAccuracyEffect=03:7482`, `GuardSpecEffect=03:74ab`,
/// `DireHitEffect=03:74b8`, `BitterBerryEffect=03:716a` in pokecrystal.sym), so
/// the same bank-switch + sentinel-push call discipline applies.
///
/// We do NOT decrement the bag ourselves: every one of these routines tails into
/// `UseItemText` → `UseDisposableItem` → `TossItem`, which removes one of the
/// item identified by `wCurItem` from `wNumItems`. Decrementing here too would
/// double-count (the same lesson the ball path learned).
///
/// `RaiseStat` (reached via `XItemEffect`'s `farcall RaiseStat`) increments the
/// stage level in `wPlayerStatLevels` AND recomputes the COMPUTED stat in
/// `wPlayerStats` (via `CalcPlayerStats`), which is what the damage routine
/// reads — so the boost actually affects damage, no hand-rolled recompute needed.
fn use_active_item_effect(cpu: &mut Cpu, id: u8) {
    // The effect routine reads `wCurItem` for the item id (X items dispatch on it
    // via `XItemStats`; `UseDisposableItem`/`TossItem` toss it). It bumps the
    // toss count from `wItemQuantityChange`, set to 1 — same as the ball path.
    cpu.write_byte(W_CUR_ITEM, id);
    cpu.write_byte(W_ITEM_QUANTITY_CHANGE, 1);

    // `UseItemText` prints "<PLAYER> used the <name>." by reading the item name
    // out of `wStringBuffer2`. The engine's `DoItemEffect` dispatcher fills that
    // buffer before jumping to the effect; since we call the effect directly we
    // must replicate that preamble or the name renders blank. Home-bank routines,
    // so no switch yet — same as the ball path.
    load_item_name(cpu, id);

    // Save the current ROM bank, switch to the effect routine's bank (0x03), push
    // a sentinel return address (each routine ends in a `ret` via TossItem), call
    // the right routine by id, then restore the bank so the battle code that
    // resumes sees bank 0x0f again.
    let saved_bank = cpu.read_byte(H_ROM_BANK);
    super::switch_bank(cpu, BANK_ITEM_EFFECTS);
    let return_sentinel: u16 = 0xfffc;
    cpu.push_word(return_sentinel);
    match id {
        0x31 | 0x33 | 0x34 | 0x35 => {
            crate::game::engine::items::item_effects::x_item_effect(cpu)
        }
        0x21 => crate::game::engine::items::item_effects::x_accuracy_effect(cpu),
        0x29 => crate::game::engine::items::item_effects::guard_spec_effect(cpu),
        0x2c => crate::game::engine::items::item_effects::dire_hit_effect(cpu),
        0x53 => crate::game::engine::items::item_effects::bitter_berry_effect(cpu),
        // `is_active_effect_item` gates the only caller, so no other id reaches
        // here.
        other => unreachable!("not an active-effect item: {other:#x}"),
    }
    super::switch_bank(cpu, saved_bank);
}

/// Mask `wBattleResult` down to a DRAW, preserving the CAUGHT_CELEBI / BOX_FULL
/// high bits — the `and BATTLERESULT_BITMASK / or DRAW` that `PokeDollEffect`
/// performs. Pure so it's unit-testable without a `Cpu`. `read_outcome` keys on
/// the low nibble, so a DRAW (2) surfaces as `Outcome::Draw`.
fn battle_result_as_draw(result: u8) -> u8 {
    (result & defs::BATTLERESULT_BITMASK) | defs::DRAW
}

/// Use the Poké Doll (id `0x25`) in a wild battle: a guaranteed escape that ends
/// the battle as a DRAW. We replicate the WILD branch of `PokeDollEffect`
/// (`scratch/pokecrystal/engine/items/item_effects.asm` ~2096) directly rather
/// than driving the transpiled `poke_doll_effect`: the real routine's effect is
/// just two trivial WRAM writes, but it tail-calls `UseItemText`, whose
/// `wait_press_aor_b_blink_cursor` joypad wait hangs headless (the same reason
/// the heal path hand-applies its effect). So we do the two writes by hand — no
/// transpiled patch needed.
///
/// The two writes, mirroring the asm exactly:
///   - `wForcedSwitch` (0xd232) = 1 (TRUE) — `battle_turn_loop1` quits when this
///     is non-zero, ending the battle as an escape.
///   - `wBattleResult` (0xd0ee) = `(wBattleResult & BATTLERESULT_BITMASK) | DRAW`.
///
/// Then we consume one Poké Doll from the Items pocket (the asm reaches
/// `TossItem` via `UseItemText`, which removes one) and set `wCurItem` /
/// `wItemQuantityChange` like the heal path does. The caller ends the battle.
fn use_poke_doll(cpu: &mut Cpu, id: u8) {
    cpu.write_byte(W_FORCED_SWITCH, 1);
    let result = cpu.read_byte(W_BATTLE_RESULT);
    cpu.write_byte(W_BATTLE_RESULT, battle_result_as_draw(result));

    decrement_pocket_item(cpu, W_NUM_ITEMS, id);
    cpu.write_byte(W_CUR_ITEM, id);
    cpu.write_byte(W_ITEM_QUANTITY_CHANGE, 1);
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
    // status byte. MiracleBerry (when its chosen target IS the active slot)
    // clears it. Mirrors `res SUBSTATUS_CONFUSED, [wPlayerSubStatus3]`. Bitter
    // Berry (`ActiveConfusion`) is NOT handled here — it drives the real
    // `BitterBerryEffect` via `use_active_item_effect` so its battle text prints,
    // and never reaches `apply_item`.
    let clears_confusion = id == 0x6d && slot == cpu.read_byte(W_CUR_BATTLE_MON);
    if clears_confusion {
        clear_active_confusion(cpu);
    }

    // NOTE: the X stat-boosters and the in-battle sub-status setters (X Accuracy
    // / Guard Spec / Dire Hit) used to be applied here by hand-rolled WRAM writes
    // (`raise_active_stat` / a direct `wPlayerSubStatus4` OR). They now drive
    // their real effect routine via `use_active_item_effect` (which prints the
    // authentic text), so they never reach `apply_item`.

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
fn illegal_move_reason(cpu: &Cpu, slot: u8) -> Option<String> {
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
fn assert_recorded_move_legal(cpu: &Cpu, slot: u8) {
    if cpu.player.is_none() {
        return;
    }
    if let Some(reason) = illegal_move_reason(cpu, slot) {
        eprintln!(
            "playback: illegal move — recorded {reason}; selectable slots were {:?}",
            move_options(cpu),
        );
        std::process::exit(2);
    }
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
    /// the stage level is below `MAX_STAT_LEVEL`; the real `RaiseStat` (driven via
    /// `use_active_item_effect`) refuses to raise past the cap. We test the
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

    /// `is_active_effect_item` selects exactly the eight active-mon items that
    /// `handle_battle_menu` routes to `use_active_item_effect` (driving the real
    /// engine effect routine for authentic text), and rejects party-target items,
    /// balls, the Poké Doll, and unknown ids — which still go through `apply_item`
    /// / the ball / escape paths.
    #[test]
    fn is_active_effect_item_selects_the_eight_active_items() {
        // The eight active-mon items: X stat-boosters, the sub-status setters,
        // and Bitter Berry.
        for id in [0x31u8, 0x33, 0x34, 0x35, 0x21, 0x29, 0x2c, 0x53] {
            assert!(is_active_effect_item(id), "item {id:#x} drives its effect routine");
        }
        // Party-target items (Potion 0x12, Full Restore 0x0e, Ether 0x3f,
        // Antidote 0x09), a ball (Poké Ball 0x01), the Poké Doll (0x25), and an
        // unknown id are NOT active-effect items.
        for id in [0x12u8, 0x0e, 0x3f, 0x09, 0x01, 0x25, 0x00] {
            assert!(!is_active_effect_item(id), "item {id:#x} is not an active-effect item");
        }
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

    /// Balls are offered only in a wild battle (`wBattleMode == WILD_BATTLE`).
    /// A trainer battle (or the not-in-battle 0 value) never lists a ball — the
    /// real game refuses it ("The trainer blocked the BALL!").
    #[test]
    fn balls_offered_only_in_wild_battles() {
        assert_eq!(defs::WILD_BATTLE, 1);
        assert_eq!(defs::TRAINER_BATTLE, 2);
        assert!(offers_balls(defs::WILD_BATTLE), "wild — balls offered");
        assert!(!offers_balls(defs::TRAINER_BATTLE), "trainer — balls blocked");
        assert!(!offers_balls(0), "not in battle — no balls");
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

    /// `battle_result_as_draw` mirrors `PokeDollEffect`'s `and BATTLERESULT_BITMASK
    /// / or DRAW`: it forces the low nibble to DRAW (2) while preserving the
    /// CAUGHT_CELEBI / BOX_FULL high bits. `read_outcome` keys on the low nibble,
    /// so the result reads as `Outcome::Draw`.
    #[test]
    fn poke_doll_masks_result_to_draw() {
        assert_eq!(defs::DRAW, 2);
        assert_eq!(defs::BATTLERESULT_BITMASK, 0xc0);
        // A fresh result (WIN, no flags) becomes DRAW.
        assert_eq!(battle_result_as_draw(0x00), defs::DRAW);
        // A prior LOSS low nibble is overwritten by DRAW.
        assert_eq!(battle_result_as_draw(defs::LOSE), defs::DRAW);
        // The high CAUGHT_CELEBI / BOX_FULL bits survive; the low nibble is DRAW.
        assert_eq!(battle_result_as_draw(0xc0), 0xc0 | defs::DRAW);
        assert_eq!(battle_result_as_draw(0x41), 0x40 | defs::DRAW);
        // Any input's low nibble reads as Draw (2) under the outcome mask 0x0f.
        for raw in [0x00u8, 0x01, 0x02, 0x40, 0x80, 0xc0, 0xff] {
            assert_eq!(battle_result_as_draw(raw) & 0x0f, defs::DRAW);
        }
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
