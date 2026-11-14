//! Port of the player's bag items in battle: `BattleMenu_Pack` and its
//! `UseItem` continuation (`engine/battle/core.asm`), the battle pack's use of
//! an item (`engine/items/pack.asm`), and the effect routines of
//! `engine/items/item_effects.asm` (heals, status heals, revives, PP items,
//! X items, Dire Hit / Guard Spec / X Accuracy, berries, the Poké Doll and the
//! Poké Balls with their catch rate and wobble logic), plus the bag's
//! `TossItem` (`engine/items/items.asm`).
//!
//! The oracle replaces the interactive bag, party menu and move menu with the
//! harness's choices (HARNESS-PATCH `battle_pack_loop`, `party_menu_select`,
//! `move_selection_screen`, see docs/transpiled_code.md). That patched
//! behaviour is what is ported here: [`Pending`] carries the choices the
//! action menu made (the party target and the move of an Ether), and the
//! menus "return" them with the contracts of the patches.

use super::battle::{Engine, Flow, PARTY_LEN};
use super::rom;
use super::rsym as r;
use super::sym::*;
use super::switch::FlagAction;
use crate::defs::*;

/// `'@'`.
const TERMINATOR: u8 = 0x50;

/// `ITEMMENU_*` as `CheckItemContext` returns them for the battle.
const ITEMMENU_CURRENT: u8 = 4;
const ITEMMENU_PARTY: u8 = 5;
const ITEMMENU_CLOSE: u8 = 6;

/// The choices of the action menu that the item's own menus consume: the
/// party slot (`party_menu_select`) and the move slot of an Ether
/// (`move_selection_screen`, `wMoveSelectionMenuType == 2`).
#[derive(Debug, Clone, Copy)]
pub struct Pending {
    pub target: Option<u8>,
    pub move_slot: Option<u8>,
}

/// The ROM bank of the Pokédex entries of species `1 + 64 * n`, as
/// `HeavyBall_GetDexEntryBank` lists them ("Pokedex Entries 001-064", ...).
const POKEDEX_ENTRY_BANKS: [u8; 4] = [0x60, 0x6e, 0x73, 0x74];

impl Engine {
    // --- BattleMenu_Pack ---------------------------------------------------------------------

    /// `BattleMenu_Pack` for `Step::Item { id, target, move_slot }`. Returns
    /// carry (the battle is over).
    pub fn battle_menu_pack(&mut self, id: u8, target: Option<u8>, move_slot: Option<u8>) -> Flow<bool> {
        if self.link_mode() != 0 || self.mem.rb(W_IN_BATTLE_TOWER_BATTLE) != 0 {
            // .ItemsCantBeUsed
            self.text(r::BATTLE_TEXT_ITEMS_CANT_BE_USED_HERE);
            return self.battle_menu();
        }
        // call LoadStandardMenuHeader
        match self.mem.rb(W_BATTLE_TYPE) {
            BATTLETYPE_TUTORIAL => self.unported("BattleMenu_Pack (tutorial)"),
            BATTLETYPE_CONTEST => self.unported("BattleMenu_Pack (Bug-Catching Contest)"),
            _ => {}
        }
        let mut pending = Pending { target, move_slot };
        self.battle_pack(id, &mut pending)?;
        assert!(
            self.mem.rb(W_BATTLE_PLAYER_ACTION) != BATTLEPLAYERACTION_USEMOVE,
            "item {id:#04x} was offered as legal but its effect refused it"
        );
        // .got_item → .UseItem
        Ok(self.battle_menu_pack_use_item())
    }

    /// `BattleMenu_Pack.UseItem`. Returns carry (a catch ends the battle).
    fn battle_menu_pack_use_item(&mut self) -> bool {
        if self.mem.rb(W_WILD_MON) != 0 {
            // .run
            self.mem.wb(W_WILD_MON, 0);
            let result = self.mem.rb(W_BATTLE_RESULT) & BATTLERESULT_BITMASK;
            self.mem.wb(W_BATTLE_RESULT, result); // WIN
            return true;
        }
        // callfar CheckItemPocket
        self.check_item_pocket();
        self.mem.wb(H_BG_MAP_MODE, 0);
        // call UpdateBattleHUDs
        self.update_battle_huds();
        self.mem.wb(W_MENU_CURSOR_Y, 1);
        false
    }

    // --- BattlePack (patched) -----------------------------------------------------------------

    /// `BattlePack` with the `battle_pack_loop` HARNESS-PATCH: the item the
    /// action menu picked is used the way the bag's USE choice does
    /// (`ItemSubmenu.Use`), and the loop then ends as after a real selection.
    fn battle_pack(&mut self, id: u8, p: &mut Pending) -> Flow {
        self.mem.set_bit(W_OPTIONS, NO_TEXT_SCROLL);
        self.init_pack_buffers();

        // use_pending_item_in_pack
        self.mem.wb(W_CUR_ITEM, id);
        let ball = self.item_attr(id, ITEMATTR_POCKET) & 0xf == BALL;
        self.mem.wb(W_CUR_POCKET, if ball { BALL_POCKET } else { ITEM_POCKET });
        self.item_submenu_use(p)?;
        // Using the item moves the bag to PACKSTATE_QUITRUNSCRIPT; if it
        // didn't, the original refused the item and the legality predicates
        // should never have offered it.
        assert_eq!(
            self.mem.rb(W_JUMPTABLE_INDEX),
            PACKSTATE_QUITRUNSCRIPT,
            "item {id:#04x} (target {:?}, move {:?}) was offered as legal but its effect refused it",
            p.target,
            p.move_slot
        );
        assert!(p.target.is_none(), "item effect never asked for its target");
        assert!(p.move_slot.is_none(), "item effect never asked for its move");
        // A catch leaves the species in wWildMon (`Outcome::Caught`).
        if ball && self.mem.rb(W_WILD_MON) != 0 {
            self.flags.caught_wild_mon = true;
        }
        // The loop's next pass runs Pack_QuitRunScript, the one after exits.
        self.mem.set_bit(W_JUMPTABLE_INDEX, JUMPTABLE_EXIT_F);
        self.mem.wb(W_PACK_USED_ITEM, 1);
        // .end
        let pocket = self.mem.rb(W_CUR_POCKET);
        self.mem.wb(W_LAST_POCKET, pocket);
        self.mem.res_bit(W_OPTIONS, NO_TEXT_SCROLL);
        Ok(())
    }

    /// `InitPackBuffers`.
    fn init_pack_buffers(&mut self) {
        self.mem.wb(W_JUMPTABLE_INDEX, 0);
        let pocket = self.mem.rb(W_LAST_POCKET) & (NUM_POCKETS - 1);
        self.mem.wb(W_CUR_POCKET, pocket);
        // inc a; add a; dec a
        self.mem.wb(W_PACK_JUMPTABLE_INDEX, (pocket + 1) * 2 - 1);
        self.mem.wb(W_PACK_USED_ITEM, 0);
        self.mem.wb(W_SWITCH_ITEM, 0);
    }

    /// `ItemSubmenu.Use`.
    fn item_submenu_use(&mut self, p: &mut Pending) -> Flow {
        // farcall CheckItemContext
        let item = self.mem.rb(W_CUR_ITEM);
        let context = self.item_attr(item, ITEMATTR_HELP) & 0xf;
        self.mem.wb(W_ITEM_ATTRIBUTE_VALUE, context);
        match context {
            ITEMMENU_CURRENT => {
                // .Unused
                self.do_item_effect(p)?;
                if self.mem.rb(W_ITEM_EFFECT_SUCCEEDED) != 0 {
                    self.mem.wb(W_JUMPTABLE_INDEX, PACKSTATE_QUITRUNSCRIPT);
                }
            }
            ITEMMENU_PARTY => {
                // .BattleField
                self.do_item_effect(p)?;
                if self.mem.rb(W_ITEM_EFFECT_SUCCEEDED) != 0 {
                    self.mem.wb(W_JUMPTABLE_INDEX, PACKSTATE_QUITRUNSCRIPT);
                }
            }
            ITEMMENU_CLOSE => {
                // .BattleOnly
                self.do_item_effect(p)?;
                match self.mem.rb(W_ITEM_EFFECT_SUCCEEDED) {
                    0 => {} // .Oak
                    2 => self.mem.wb(W_ITEM_EFFECT_SUCCEEDED, 0), // .didnt_use_item
                    _ => self.mem.wb(W_JUMPTABLE_INDEX, PACKSTATE_QUITRUNSCRIPT),
                }
            }
            // .Oak: OakThisIsntTheTimeText
            _ => {}
        }
        Ok(())
    }

    /// `GetItemAttr`: attribute `attr` of item `item` (`wCurItem` in the
    /// original; this takes the item as an argument).
    fn item_attr(&self, item: u8, attr: u8) -> u8 {
        rom::byte(r::ITEM_ATTRIBUTES + usize::from(item - 1) * usize::from(ITEMATTR_STRUCT_LENGTH) + usize::from(attr))
    }

    /// `CheckItemPocket`: the pocket of `wCurItem` into `wItemAttributeValue`
    /// (1 = items, 2 = key items, 3 = balls, 4 = TMs / HMs).
    fn check_item_pocket(&mut self) -> u8 {
        let item = self.mem.rb(W_CUR_ITEM);
        let pocket = self.item_attr(item, ITEMATTR_POCKET) & 0xf;
        self.mem.wb(W_ITEM_ATTRIBUTE_VALUE, pocket);
        pocket
    }

    // --- the harness's menus --------------------------------------------------------------------

    /// `party_menu_select` (patched) while an item effect asks for its
    /// target: what the routine leaves after A on party slot `slot`
    /// (`bootstrap::cli::select_party_target`).
    fn select_party_target(&mut self, slot: u8) {
        self.mem.wb(W_MENU_CURSOR_Y, slot + 1);
        self.mem.wb(W_PARTY_MENU_CURSOR, slot + 1);
        self.mem.wb(W_CUR_PARTY_MON, slot);
        let species = self.mem.rb(W_PARTY_SPECIES + u16::from(slot));
        self.mem.wb(W_CUR_PARTY_SPECIES, species);
    }

    // --- DoItemEffect -----------------------------------------------------------------------------

    /// `_DoItemEffect`: run the effect of `wCurItem`.
    fn do_item_effect(&mut self, p: &mut Pending) -> Flow {
        let item = self.mem.rb(W_CUR_ITEM);
        self.mem.wb(W_NAMED_OBJECT_INDEX, item);
        let de = self.get_item_name();
        self.copy_name1(de);
        self.mem.wb(W_ITEM_EFFECT_SUCCEEDED, 1);
        match item {
            MASTER_BALL | ULTRA_BALL | GREAT_BALL | POKE_BALL | HEAVY_BALL | LEVEL_BALL | LURE_BALL | FAST_BALL
            | FRIEND_BALL | MOON_BALL | LOVE_BALL | PARK_BALL => self.poke_ball_effect(),
            ANTIDOTE | BURN_HEAL | ICE_HEAL | AWAKENING | PARLYZ_HEAL | FULL_HEAL | PSNCUREBERRY | PRZCUREBERRY
            | BURNT_BERRY | ICE_BERRY | MINT_BERRY | MIRACLEBERRY => {
                // StatusHealingEffect
                if self.use_item_select_mon(PARTYMENUACTION_HEALING_ITEM, p) {
                    return Ok(self.status_healer_exit_menu());
                }
                let a = self.use_status_healer();
                Ok(self.status_healer_jumptable(a))
            }
            HEAL_POWDER => {
                if self.use_item_select_mon(PARTYMENUACTION_HEALING_ITEM, p) {
                    return Ok(self.status_healer_exit_menu());
                }
                let mut a = self.use_status_healer();
                if a == 0 {
                    self.change_happiness(HAPPINESS_BITTERPOWDER);
                    self.text(r::ITEM_LOOKS_BITTER_TEXT);
                    a = 0;
                }
                Ok(self.status_healer_jumptable(a))
            }
            FULL_RESTORE => self.full_restore_effect(p),
            MAX_POTION | HYPER_POTION | SUPER_POTION | POTION | FRESH_WATER | SODA_POP | LEMONADE | MOOMOO_MILK
            | RAGECANDYBAR | BERRY_JUICE | BERRY | GOLD_BERRY => {
                // RestoreHPEffect
                let a = self.item_restore_hp(p);
                Ok(self.status_healer_jumptable(a))
            }
            ENERGYPOWDER | ENERGY_ROOT => {
                let c = if item == ENERGYPOWDER { HAPPINESS_BITTERPOWDER } else { HAPPINESS_ENERGYROOT };
                let mut a = self.item_restore_hp(p);
                if a == 0 {
                    self.change_happiness(c);
                    self.text(r::ITEM_LOOKS_BITTER_TEXT);
                    a = 0;
                }
                Ok(self.status_healer_jumptable(a))
            }
            REVIVE | MAX_REVIVE => {
                // ReviveEffect
                if self.use_item_select_mon(PARTYMENUACTION_HEALING_ITEM, p) {
                    return Ok(self.status_healer_exit_menu());
                }
                let a = self.revive_pokemon();
                Ok(self.status_healer_jumptable(a))
            }
            REVIVAL_HERB => {
                if self.use_item_select_mon(PARTYMENUACTION_HEALING_ITEM, p) {
                    return Ok(self.status_healer_exit_menu());
                }
                let mut a = self.revive_pokemon();
                if a == 0 {
                    self.change_happiness(HAPPINESS_REVIVALHERB);
                    self.text(r::ITEM_LOOKS_BITTER_TEXT);
                    a = 0;
                }
                Ok(self.status_healer_jumptable(a))
            }
            BITTER_BERRY => {
                // BitterBerryEffect
                let a = if self.mem.bit(W_PLAYER_SUB_STATUS3, SUBSTATUS_CONFUSED) {
                    self.mem.res_bit(W_PLAYER_SUB_STATUS3, SUBSTATUS_CONFUSED);
                    self.set_player_turn();
                    self.use_item_text();
                    self.text(r::CONFUSED_NO_MORE_TEXT);
                    0
                } else {
                    1
                };
                Ok(self.status_healer_jumptable(a))
            }
            X_ACCURACY => Ok(self.sub_status_item(SUBSTATUS_X_ACCURACY)),
            GUARD_SPEC => Ok(self.sub_status_item(SUBSTATUS_MIST)),
            DIRE_HIT => Ok(self.sub_status_item(SUBSTATUS_FOCUS_ENERGY)),
            X_ATTACK | X_DEFEND | X_SPEED | X_SPECIAL => self.x_item_effect(),
            POKE_DOLL => Ok(self.poke_doll_effect()),
            MAX_ELIXER | ETHER | MAX_ETHER | ELIXER | MYSTERYBERRY => self.restore_pp_effect(p),
            // NoEffect → IsntTheTimeMessage
            BRIGHTPOWDER | LUCKY_PUNCH | METAL_POWDER | NUGGET | EXP_SHARE => {
                self.text(r::ITEM_OAK_WARNING_TEXT);
                self.mem.wb(W_ITEM_EFFECT_SUCCEEDED, 0);
                Ok(())
            }
            _ => self.unported(&format!("item effect of item {item:#04x}")),
        }
    }

    /// `UseItemText` (and its fall-through, `UseDisposableItem`).
    fn use_item_text(&mut self) {
        self.text(r::ITEM_USED_TEXT);
        self.use_disposable_item();
    }

    /// `UseDisposableItem`: one item out of the bag.
    fn use_disposable_item(&mut self) {
        self.mem.wb(W_ITEM_QUANTITY_CHANGE, 1);
        self.toss_item(W_NUM_ITEMS);
    }

    /// `WontHaveAnyEffect_NotUsedMessage`.
    fn wont_have_any_effect_not_used(&mut self) {
        self.text(r::ITEM_WONT_HAVE_EFFECT_TEXT);
        // Item wasn't used.
        self.mem.wb(W_ITEM_EFFECT_SUCCEEDED, 2);
    }

    /// `CantUseItemMessage` with `WontHaveAnyEffectMessage`.
    fn wont_have_any_effect_message(&mut self) {
        self.mem.wb(W_ITEM_EFFECT_SUCCEEDED, 0);
        self.text(r::ITEM_WONT_HAVE_EFFECT_TEXT);
    }

    // --- X Accuracy, Guard Spec, Dire Hit, X items, Poké Doll ------------------------------------

    /// `XAccuracyEffect` / `GuardSpecEffect` / `DireHitEffect`: set `bit` of
    /// `wPlayerSubStatus4`.
    fn sub_status_item(&mut self, bit: u8) {
        if self.mem.bit(W_PLAYER_SUB_STATUS4, bit) {
            return self.wont_have_any_effect_not_used();
        }
        self.mem.set_bit(W_PLAYER_SUB_STATUS4, bit);
        self.use_item_text();
    }

    /// `XItemEffect`.
    fn x_item_effect(&mut self) -> Flow {
        self.use_item_text();
        let item = self.mem.rb(W_CUR_ITEM);
        // XItemStats
        let mut at = r::X_ITEM_STATS;
        while rom::byte(at) != item {
            at += 2;
        }
        let b = rom::byte(at + 1);
        self.set_player_turn();
        self.mem.wb(W_ATTACK_MISSED, 0);
        self.mem.wb(W_EFFECT_FAILED, 0);
        self.raise_stat(b);
        // call WaitSFX
        self.battle_command_stat_up_message()?;
        self.battle_command_stat_up_fail_text()?;
        let cur = self.mem.rb(W_CUR_BATTLE_MON);
        self.mem.wb(W_CUR_PARTY_MON, cur);
        self.change_happiness(HAPPINESS_USEDXITEM);
        Ok(())
    }

    /// `PokeDollEffect`.
    fn poke_doll_effect(&mut self) {
        if self.battle_mode() != WILD_BATTLE {
            self.mem.wb(W_ITEM_EFFECT_SUCCEEDED, 0);
            return;
        }
        self.mem.wb(W_FORCED_SWITCH, 1);
        let result = (self.mem.rb(W_BATTLE_RESULT) & BATTLERESULT_BITMASK) | DRAW;
        self.mem.wb(W_BATTLE_RESULT, result);
        self.use_item_text();
    }

    // --- party-targeted items -----------------------------------------------------------------------

    /// `GetPartyParamLocation`: the `off`th byte of `PartyMon[wCurPartyMon]`.
    fn party_param(&self, off: u8) -> u16 {
        W_PARTY_MONS + u16::from(off) + u16::from(self.mem.rb(W_CUR_PARTY_MON)) * PARTY_LEN
    }

    /// `UseItem_SelectMon`: `b` = the party menu action. Returns carry
    /// (cancelled, or an Egg).
    fn use_item_select_mon(&mut self, b: u8, p: &mut Pending) -> bool {
        // .SelectMon → ChooseMonToUseItemOn → PartyMenuSelect (patched)
        self.mem.wb(W_PARTY_MENU_ACTION_TEXT, b);
        let slot = p.target.take().expect("item effect asked for a party target the action menu didn't choose");
        self.select_party_target(slot);
        if self.mem.rb(W_CUR_PARTY_SPECIES) == EGG {
            // CantUseOnEggMessage
            self.mem.wb(W_ITEM_EFFECT_SUCCEEDED, 0);
            self.text(r::ITEM_CANT_USE_ON_EGG_TEXT);
            return true;
        }
        false
    }

    /// `StatusHealer_ExitMenu` (`StatusHealer_ClearPalettes` is graphics).
    fn status_healer_exit_menu(&mut self) {
        self.mem.wb(W_ITEM_EFFECT_SUCCEEDED, 0);
    }

    /// `StatusHealer_Jumptable`: 0 = used, 1 = no effect, 2 = cancelled.
    fn status_healer_jumptable(&mut self, a: u8) {
        match a {
            0 => {} // StatusHealer_ClearPalettes
            1 => self.wont_have_any_effect_message(), // StatusHealer_NoEffect
            _ => self.status_healer_exit_menu(),
        }
    }

    /// `IsItemUsedOnBattleMon`: carry.
    fn is_item_used_on_battle_mon(&self) -> bool {
        self.battle_mode() != 0 && self.mem.rb(W_CUR_PARTY_MON) == self.mem.rb(W_CUR_BATTLE_MON)
    }

    /// `GetItemHealingAction`: (`b` = party menu text, `c` = status mask).
    fn get_item_healing_action(&self) -> (u8, u8) {
        let item = self.mem.rb(W_CUR_ITEM);
        let mut at = r::STATUS_HEALING_ACTIONS;
        while rom::byte(at) != item {
            at += 3;
        }
        (rom::byte(at + 1), rom::byte(at + 2))
    }

    /// `UseStatusHealer`: 0 = used, 1 = no effect.
    fn use_status_healer(&mut self) -> u8 {
        if self.is_mon_fainted() {
            return 1;
        }
        let (mut b, c) = self.get_item_healing_action();
        let status = self.party_param(MON_STATUS);
        if self.mem.rb(status) & c == 0 {
            // IsItemUsedOnConfusedMon
            let confused =
                self.is_item_used_on_battle_mon() && self.mem.bit(W_PLAYER_SUB_STATUS3, SUBSTATUS_CONFUSED) && c == 0xff;
            if !confused {
                return 1;
            }
            b = PARTYMENUTEXT_HEAL_CONFUSION;
        }
        // .good
        self.mem.wb(status, 0);
        self.mem.wb(W_PARTY_MENU_ACTION_TEXT, b);
        self.heal_status();
        // Play_SFX_FULL_HEAL
        self.item_action_text_wait_button();
        self.use_disposable_item();
        0
    }

    /// `HealStatus`.
    fn heal_status(&mut self) {
        if !self.is_item_used_on_battle_mon() {
            return;
        }
        self.mem.wb(W_BATTLE_MON_STATUS, 0);
        self.mem.res_bit(W_PLAYER_SUB_STATUS5, SUBSTATUS_TOXIC);
        self.mem.res_bit(W_PLAYER_SUB_STATUS1, SUBSTATUS_NIGHTMARE);
        let (_, c) = self.get_item_healing_action();
        if c == 0xff {
            self.mem.res_bit(W_PLAYER_SUB_STATUS3, SUBSTATUS_CONFUSED);
        }
        self.calc_player_stats();
    }

    /// `BattlemonRestoreHealth`.
    fn battlemon_restore_health(&mut self) {
        if !self.is_item_used_on_battle_mon() {
            return;
        }
        let hp = self.mem.rw(self.party_param(MON_HP));
        self.mem.ww(W_BATTLE_MON_HP, hp);
    }

    /// `FullRestoreEffect`.
    fn full_restore_effect(&mut self, p: &mut Pending) -> Flow {
        if self.use_item_select_mon(PARTYMENUACTION_HEALING_ITEM, p) {
            return Ok(self.status_healer_exit_menu());
        }
        if self.is_mon_fainted() {
            return Ok(self.status_healer_jumptable(1)); // StatusHealer_NoEffect
        }
        if !self.is_mon_hurt() {
            // FullyHealStatus
            let a = self.use_status_healer();
            return Ok(self.status_healer_jumptable(a));
        }
        // .FullRestore
        self.mem.wb(W_LOW_HEALTH_ALARM, 0);
        self.revive_full_hp();
        let status = self.party_param(MON_STATUS);
        self.mem.wb(status, 0);
        self.mem.wb(status + 1, 0);
        self.heal_status();
        self.battlemon_restore_health();
        self.heal_hp_sfx_gfx();
        self.mem.wb(W_PARTY_MENU_ACTION_TEXT, PARTYMENUTEXT_HEAL_HP);
        self.item_action_text_wait_button();
        self.use_disposable_item();
        Ok(self.status_healer_jumptable(0))
    }

    /// `ItemRestoreHP`: 0 = used, 1 = no effect, 2 = cancelled.
    fn item_restore_hp(&mut self, p: &mut Pending) -> u8 {
        if self.use_item_select_mon(PARTYMENUACTION_HEALING_ITEM, p) {
            return 2;
        }
        if self.is_mon_fainted() {
            return 1;
        }
        if !self.is_mon_hurt() {
            return 1;
        }
        self.mem.wb(W_LOW_HEALTH_ALARM, 0);
        let amount = self.get_healing_item_amount();
        self.restore_health(amount);
        self.battlemon_restore_health();
        self.heal_hp_sfx_gfx();
        self.mem.wb(W_PARTY_MENU_ACTION_TEXT, PARTYMENUTEXT_HEAL_HP);
        self.item_action_text_wait_button();
        self.use_disposable_item();
        0
    }

    /// `RevivePokemon`: 0 = used, 1 = not fainted.
    fn revive_pokemon(&mut self) -> u8 {
        if !self.is_mon_fainted() {
            return 1;
        }
        if self.battle_mode() != 0 {
            let cur = self.mem.rb(W_CUR_PARTY_MON);
            if self.small_far_flag_action(W_BATTLE_PARTICIPANTS_INCLUDING_FAINTED, cur, FlagAction::Check) {
                self.small_far_flag_action(W_BATTLE_PARTICIPANTS_NOT_FAINTED, cur, FlagAction::Set);
            }
        }
        self.mem.wb(W_LOW_HEALTH_ALARM, 0);
        if self.mem.rb(W_CUR_ITEM) == REVIVE {
            self.revive_half_hp();
        } else {
            self.revive_full_hp();
        }
        self.heal_hp_sfx_gfx();
        self.mem.wb(W_PARTY_MENU_ACTION_TEXT, PARTYMENUTEXT_REVIVE);
        self.item_action_text_wait_button();
        self.use_disposable_item();
        0
    }

    /// `HealHP_SFX_GFX`: the party HP bar animation (its effect on memory).
    fn heal_hp_sfx_gfx(&mut self) {
        self.mem.wb(W_WHICH_HP_BAR, 2);
        self.animate_hp_bar();
    }

    /// `ItemActionTextWaitButton` → `ItemActionText` →
    /// `PrintPartyMenuActionText`: the party menu's message for
    /// `wPartyMenuActionText` ("X recovered N HP!", ...).
    fn item_action_text_wait_button(&mut self) {
        let cur = self.mem.rb(W_CUR_PARTY_MON);
        self.get_nickname(W_PARTY_MON_NICKNAMES, cur);
        let text = [
            r::PRINT_PARTY_MENU_ACTION_TEXT_CURED_OF_POISON_TEXT,
            r::PRINT_PARTY_MENU_ACTION_TEXT_BURN_WAS_HEALED_TEXT,
            r::PRINT_PARTY_MENU_ACTION_TEXT_WAS_DEFROSTED_TEXT,
            r::PRINT_PARTY_MENU_ACTION_TEXT_WOKE_UP_TEXT,
            r::PRINT_PARTY_MENU_ACTION_TEXT_RID_OF_PARALYSIS_TEXT,
            r::PRINT_PARTY_MENU_ACTION_TEXT_RECOVERED_SOME_HP_TEXT,
            r::PRINT_PARTY_MENU_ACTION_TEXT_HEALTH_RETURNED_TEXT,
            r::PRINT_PARTY_MENU_ACTION_TEXT_REVITALIZED_TEXT,
            r::PRINT_PARTY_MENU_ACTION_TEXT_GREW_TO_LEVEL_TEXT,
            r::PRINT_PARTY_MENU_ACTION_TEXT_CAME_TO_ITS_SENSES_TEXT,
        ][usize::from(self.mem.rb(W_PARTY_MENU_ACTION_TEXT) & 0xf)];
        self.text(text);
    }

    /// `ReviveHalfHP`.
    fn revive_half_hp(&mut self) {
        let hp = self.hp_buffer(W_HP_BUFFER1) >> 1;
        self.continue_revive(hp);
    }

    /// `ReviveFullHP`.
    fn revive_full_hp(&mut self) {
        let hp = self.hp_buffer(W_HP_BUFFER1);
        self.continue_revive(hp);
    }

    /// `ContinueRevive`: the party mon's HP = `hp`.
    fn continue_revive(&mut self, hp: u16) {
        let at = self.party_param(MON_HP);
        self.mem.ww(at, hp);
        self.load_cur_hp_into_buffer3();
    }

    /// `RestoreHealth` (`de` = the amount).
    fn restore_health(&mut self, de: u16) {
        let at = self.party_param(MON_HP + 1);
        let (lo, c1) = self.mem.rb(at).overflowing_add(de as u8);
        self.mem.wb(at, lo);
        let (hi, c2a) = self.mem.rb(at - 1).overflowing_add((de >> 8) as u8);
        let (hi, c2b) = hi.overflowing_add(u8::from(c1));
        self.mem.wb(at - 1, hi);
        if c2a || c2b {
            return self.revive_full_hp();
        }
        self.load_cur_hp_into_buffer3();
        // `sub`/`sbc` of the max HP: carry (HP < max) finishes.
        let hp = self.mem.rw(self.party_param(MON_HP));
        let max = self.mem.rw(self.party_param(MON_MAXHP));
        if hp >= max {
            self.revive_full_hp();
        }
    }

    /// `GetHealingItemAmount`.
    fn get_healing_item_amount(&self) -> u16 {
        let item = self.mem.rb(W_CUR_ITEM);
        let mut at = r::HEALING_HP_AMOUNTS;
        loop {
            let id = rom::byte(at);
            if id == 0xff || id == item {
                return rom::ptr(at + 1);
            }
            at += 3;
        }
    }

    /// A 16-bit value stored low byte first (`wHPBuffer1`..3).
    fn hp_buffer(&self, addr: u16) -> u16 {
        self.mem.rw_le(addr)
    }

    /// `LoadCurHPIntoBuffer3`.
    fn load_cur_hp_into_buffer3(&mut self) {
        let hp = self.mem.rw(self.party_param(MON_HP));
        self.mem.ww_le(W_HP_BUFFER3, hp);
    }

    /// `IsMonFainted` (with the buffers it loads): true when the HP is 0.
    fn is_mon_fainted(&mut self) -> bool {
        // LoadMaxHPIntoBuffer1, LoadCurHPIntoBuffer2
        let max = self.mem.rw(self.party_param(MON_MAXHP));
        self.mem.ww_le(W_HP_BUFFER1, max);
        let hp = self.mem.rw(self.party_param(MON_HP));
        self.mem.ww_le(W_HP_BUFFER2, hp);
        hp == 0
    }

    /// `IsMonAtFullHealth`'s carry: the HP is below the maximum.
    fn is_mon_hurt(&self) -> bool {
        self.hp_buffer(W_HP_BUFFER2) < self.hp_buffer(W_HP_BUFFER1)
    }

    // --- PP items -------------------------------------------------------------------------------------

    /// `RestorePPEffect` for every PP item but PP Up (never offered).
    fn restore_pp_effect(&mut self, p: &mut Pending) -> Flow {
        let item = self.mem.rb(W_CUR_ITEM);
        if item == PP_UP {
            self.unported("RestorePPEffect (PP Up)");
        }
        self.mem.wb(W_TEMP_RESTORE_PP_ITEM, item);
        // .loop
        if self.use_item_select_mon(PARTYMENUACTION_HEALING_ITEM, p) {
            return Ok(self.pp_restore_item_cancel());
        }
        if item == MAX_ELIXER || item == ELIXER {
            return Ok(self.elixer_restore_pp_of_all_moves());
        }
        self.text(r::RESTORE_THE_PP_OF_WHICH_MOVE_TEXT);
        let saved = self.mem.rb(W_CUR_MOVE_NUM);
        self.mem.wb(W_CUR_MOVE_NUM, 0);
        self.mem.wb(W_MOVE_SELECTION_MENU_TYPE, 2);
        // MoveSelectionScreen (patched, wMoveSelectionMenuType == 2)
        self.ether_move_menu(p);
        self.mem.wb(W_CUR_MOVE_NUM, saved);
        // GetMthMoveOfNthPartymon
        let mv_at = self.party_move_addr(W_PARTY_MON1_MOVES);
        let mv = self.mem.rb(mv_at);
        self.mem.wb(W_NAMED_OBJECT_INDEX, mv);
        let de = self.get_move_name();
        self.copy_name1(de);
        // Not_PP_Up
        if self.restore_pp() {
            self.battle_restore_pp();
        } else {
            self.pp_restore_item_no_effect();
        }
        Ok(())
    }

    /// `MoveSelectionScreen` (patched) for the Ether move menu: the A-button
    /// contract with the chosen slot.
    pub fn ether_move_menu(&mut self, p: &mut Pending) {
        let slot = p.move_slot.take().expect("Ether move menu opened without a CLI-chosen move slot");
        self.mem.wb(W_SWAPPING_MOVE, 0);
        self.mem.wb(W_MENU_CURSOR_Y, slot);
        self.mem.wb(W_CUR_MOVE_NUM, slot);
    }

    /// `GetMthMoveOfNthPartymon` with `hl` = `base` (the first party mon's
    /// field): the field of move `wMenuCursorY` of party mon `wCurPartyMon`.
    fn party_move_addr(&self, base: u16) -> u16 {
        base + u16::from(self.mem.rb(W_CUR_PARTY_MON)) * PARTY_LEN + u16::from(self.mem.rb(W_MENU_CURSOR_Y))
    }

    /// `Elixer_RestorePPofAllMoves`.
    fn elixer_restore_pp_of_all_moves(&mut self) {
        self.mem.wb(W_MENU_CURSOR_Y, 0);
        self.mem.wb(W_MENU_CURSOR_X, 0);
        for _ in 0..NUM_MOVES {
            let mv = self.mem.rb(self.party_move_addr(W_PARTY_MON1_MOVES));
            if mv != 0 && self.restore_pp() {
                let x = self.mem.rb(W_MENU_CURSOR_X).wrapping_add(1);
                self.mem.wb(W_MENU_CURSOR_X, x);
            }
            let y = self.mem.rb(W_MENU_CURSOR_Y).wrapping_add(1);
            self.mem.wb(W_MENU_CURSOR_Y, y);
        }
        if self.mem.rb(W_MENU_CURSOR_X) != 0 {
            return self.battle_restore_pp();
        }
        self.pp_restore_item_no_effect();
    }

    /// `PPRestoreItem_NoEffect`.
    fn pp_restore_item_no_effect(&mut self) {
        self.mem.wb(W_ITEM_EFFECT_SUCCEEDED, 0);
        self.text(r::ITEM_WONT_HAVE_EFFECT_TEXT);
        self.pp_restore_item_cancel();
    }

    /// `PPRestoreItem_Cancel`.
    fn pp_restore_item_cancel(&mut self) {
        self.mem.wb(W_ITEM_EFFECT_SUCCEEDED, 0);
    }

    /// `BattleRestorePP` (and its `FinishPPRestore`).
    fn battle_restore_pp(&mut self) {
        if self.battle_mode() != 0
            && self.mem.rb(W_CUR_PARTY_MON) == self.mem.rb(W_CUR_BATTLE_MON)
            && !self.mem.bit(W_PLAYER_SUB_STATUS5, SUBSTATUS_TRANSFORMED)
        {
            self.update_battle_mon_pp();
        }
        self.text(r::PP_RESTORED_TEXT);
        self.use_disposable_item();
    }

    /// `BattleRestorePP.UpdateBattleMonPP`.
    fn update_battle_mon_pp(&mut self) {
        let mut hl = W_PARTY_MON1_MOVES + u16::from(self.mem.rb(W_CUR_PARTY_MON)) * PARTY_LEN;
        let mut de = W_BATTLE_MON_MOVES;
        for _ in 0..NUM_MOVES {
            let a = self.mem.rb(de);
            if a == 0 {
                return;
            }
            if a == self.mem.rb(hl) {
                let pp = self.mem.rb(hl + u16::from(MON_PP - MON_MOVES));
                self.mem.wb(de + u16::from(NUM_MOVES) + 2, pp);
            }
            hl += 1;
            de += 1;
        }
    }

    /// `RestorePP`: restore PP of the move `wMenuCursorY` of party mon
    /// `wCurPartyMon`. Returns nz (`true`) when it restored something.
    fn restore_pp(&mut self) -> bool {
        self.mem.wb(W_MON_TYPE, 0); // PARTYMON
        self.get_max_pp_of_move();
        let at = self.party_move_addr(W_PARTY_MON1_PP);
        let b = self.mem.rb(W_TEMP_PP);
        let pp = self.mem.rb(at);
        if pp & PP_MASK >= b {
            // .dont_restore
            return false;
        }
        let item = self.mem.rb(W_TEMP_RESTORE_PP_ITEM);
        let mut b = b;
        if item != MAX_ELIXER && item != MAX_ETHER {
            let c = if item == MYSTERYBERRY { 5 } else { 10 };
            let sum = (pp & PP_MASK) + c;
            if sum < b {
                b = sum;
            }
        }
        // .restore_all
        let new = (pp & PP_UP_MASK) | b;
        self.mem.wb(at, new);
        new != 0
    }

    /// `GetMaxPPOfMove` (for `wMonType` = PARTYMON, the only type items use):
    /// the maximum PP of the move into `wTempPP`.
    fn get_max_pp_of_move(&mut self) {
        let sb0 = self.mem.rb(W_STRING_BUFFER1);
        let sb1 = self.mem.rb(W_STRING_BUFFER1 + 1);
        if self.mem.rb(W_MON_TYPE) != 0 {
            self.unported("GetMaxPPOfMove (not a party mon)");
        }
        let mv_at = self.party_move_addr(W_PARTY_MON1_MOVES);
        let mv = self.mem.rb(mv_at);
        let base_pp = rom::byte(r::MOVES + usize::from(mv.wrapping_sub(1)) * usize::from(MOVE_LENGTH) + usize::from(MOVE_PP));
        self.mem.wb(W_STRING_BUFFER1, base_pp);
        let ups = self.mem.rb(mv_at + u16::from(MON_PP - MON_MOVES)) & PP_UP_MASK;
        self.mem.wb(W_STRING_BUFFER1 + 1, ups | base_pp);
        self.mem.wb(W_TEMP_PP, 0);
        self.compute_max_pp(W_STRING_BUFFER1, W_STRING_BUFFER1 + 1);
        let max = self.mem.rb(W_STRING_BUFFER1 + 1) & PP_MASK;
        self.mem.wb(W_TEMP_PP, max);
        self.mem.wb(W_STRING_BUFFER1 + 1, sb1);
        self.mem.wb(W_STRING_BUFFER1, sb0);
    }

    /// `ComputeMaxPP` (`de` = the move's base PP, `hl` = its PP byte, which
    /// holds the PP Up count).
    fn compute_max_pp(&mut self, de: u16, hl: u16) {
        // Divide the base PP by 5.
        let base = self.mem.rb(de);
        self.mem.wb(H_DIVIDEND + 3, base);
        self.mem.wb(H_DIVIDEND, 0);
        self.mem.wb(H_DIVIDEND + 1, 0);
        self.mem.wb(H_DIVIDEND + 2, 0);
        self.mem.wb(H_DIVISOR, 5);
        self.divide(4);
        let mut b = self.mem.rb(hl);
        let mut c = b >> 6;
        if c != 0 {
            loop {
                let mut a = self.mem.rb(H_QUOTIENT + 3);
                if a >= 8 {
                    a = 7;
                }
                b = b.wrapping_add(a);
                if self.mem.rb(W_TEMP_PP).wrapping_sub(1) == 0 {
                    break;
                }
                c -= 1;
                if c == 0 {
                    break;
                }
            }
        }
        self.mem.wb(hl, b);
    }

    // --- TossItem ---------------------------------------------------------------------------------------

    /// `TossItem` with `hl`: the bag's pocket count (`wNumItems` selects the
    /// pocket by the item's attribute).
    fn toss_item(&mut self, hl: u16) -> bool {
        if hl != W_NUM_ITEMS {
            return self.remove_item_from_pocket(hl);
        }
        match self.check_item_pocket() {
            1 => self.remove_item_from_pocket(W_NUM_ITEMS),
            3 => self.remove_item_from_pocket(W_NUM_BALLS),
            2 => self.unported("TossKeyItem"),
            _ => self.unported("TossTMHM"),
        }
    }

    /// `RemoveItemFromPocket` (`hl` = the pocket's count byte, then
    /// (item, quantity) pairs ended by `$ff`). Returns carry (removed).
    fn remove_item_from_pocket(&mut self, de: u16) -> bool {
        let count = self.mem.rb(de);
        let item = self.mem.rb(W_CUR_ITEM);
        let hint = self.mem.rb(W_CUR_ITEM_QUANTITY);
        let mut hl = de + 1;
        let mut found = false;
        if hint < count {
            // The quantity doubles as a hint for the item's position.
            let at = hl + 2 * u16::from(hint);
            let hit = self.mem.rb(at) == item;
            if hit {
                hl = at + 1;
                found = true;
            }
        }
        if !found {
            loop {
                let a = self.mem.rb(hl);
                hl += 1;
                if a == item {
                    break;
                }
                if a == 0xff {
                    return false;
                }
                hl += 1;
            }
        }
        // .skip
        let change = self.mem.rb(W_ITEM_QUANTITY_CHANGE);
        let Some(left) = self.mem.rb(hl).checked_sub(change) else { return false };
        self.mem.wb(hl, left);
        self.mem.wb(W_ITEM_QUANTITY, left);
        if left != 0 {
            return true;
        }
        let mut bc = hl - 1;
        hl += 1;
        loop {
            let a = self.mem.rb(hl);
            hl += 1;
            self.mem.wb(bc, a);
            bc += 1;
            if a == 0xff {
                break;
            }
        }
        let n = self.mem.rb(de).wrapping_sub(1);
        self.mem.wb(de, n);
        true
    }

    // --- Poké Balls -------------------------------------------------------------------------------------

    /// `PokeBallEffect`.
    fn poke_ball_effect(&mut self) -> Flow {
        if self.battle_mode() != WILD_BATTLE {
            return self.use_ball_in_trainer_battle();
        }
        // A full party needs room in the box (SRAM, which the harness's
        // battles start empty): nothing to check until a catch.
        // .room_in_party
        self.mem.wb(W_WILD_MON, 0);
        // ReturnToBattle_UseBall (patched: nothing) for every ball but Park
        self.mem.res_bit(W_OPTIONS, NO_TEXT_SCROLL);
        self.text(r::ITEM_USED_TEXT);

        let mut b = self.mem.rb(W_ENEMY_MON_CATCH_RATE);
        let item = self.mem.rb(W_CUR_ITEM);
        let mut caught = true;
        if self.mem.rb(W_BATTLE_TYPE) != BATTLETYPE_TUTORIAL && item != MASTER_BALL {
            // BallMultiplierFunctionTable
            let mut at = r::BALL_MULTIPLIER_FUNCTION_TABLE;
            loop {
                let id = rom::byte(at);
                if id == 0xff {
                    break;
                }
                if id == item {
                    b = self.ball_multiplier(item, b);
                    break;
                }
                at += 3;
            }
            // .skip_or_return_from_ball_fn
            let a = if item == LEVEL_BALL { b } else { self.catch_rate_from_hp(b) };
            // .skip_hp_calc
            self.mem.wb(W_FINAL_CATCH_RATE, a);
            let roll = self.random("PokeBallEffect");
            caught = roll <= a;
        }
        let species = self.mem.rb(W_ENEMY_MON_SPECIES);
        self.mem.wb(W_WILD_MON, if caught { species } else { 0 });

        // The throw (DelayFrames dropped).
        let anim_param = if item < POKE_BALL + 1 { item } else { POKE_BALL };
        self.mem.wb(W_BATTLE_ANIM_PARAM, anim_param);
        self.play_fx_anim_id(ANIM_THROW_POKE_BALL as u8, (ANIM_THROW_POKE_BALL >> 8) as u8);
        self.set_player_turn();
        self.mem.wb(W_THROWN_BALL_WOBBLE_COUNT, 0);
        self.mem.wb(W_BATTLE_AFTER_ANIM, 0);
        // HARNESS-PATCH not_kurt_ball: the animation's `anim_checkpokeball`
        // loop is GetPokeBallWobble until it returns click (1) or break free (2).
        while self.get_poke_ball_wobble() == 0 {}

        if self.mem.rb(W_WILD_MON) == 0 {
            let text = match self.mem.rb(W_THROWN_BALL_WOBBLE_COUNT) {
                1 => Some(r::BALL_BROKE_FREE_TEXT),
                2 => Some(r::BALL_APPEARED_CAUGHT_TEXT),
                3 => Some(r::BALL_ALMOST_HAD_IT_TEXT),
                4 => Some(r::BALL_SO_CLOSE_TEXT),
                _ => None,
            };
            if let Some(text) = text {
                // .shake_and_break_free
                self.text(text);
                self.poke_ball_return_from_capture();
                return Ok(());
            }
        }
        self.poke_ball_caught()
    }

    /// `.return_from_capture`: the ball is used up.
    fn poke_ball_return_from_capture(&mut self) {
        let battle_type = self.mem.rb(W_BATTLE_TYPE);
        if battle_type == BATTLETYPE_TUTORIAL || battle_type == BATTLETYPE_DEBUG {
            return;
        }
        if battle_type == BATTLETYPE_CONTEST {
            let left = self.mem.rb(W_PARK_BALLS_REMAINING).wrapping_sub(1);
            self.mem.wb(W_PARK_BALLS_REMAINING, left);
            return;
        }
        if self.mem.rb(W_WILD_MON) != 0 {
            // call ClearBGPalettes / ClearTilemap
            self.unported("PokeBallEffect.toss after a catch (register left by ClearTilemap)");
        }
        self.mem.wb(W_ITEM_QUANTITY_CHANGE, 1);
        self.toss_item(W_NUM_ITEMS);
    }

    /// `PokeBallEffect.caught`: the wild mon was caught.
    fn poke_ball_caught(&mut self) -> Flow {
        // The status, HP, held item and Transformed bit survive LoadEnemyMon.
        let status = self.mem.rb(W_ENEMY_MON_STATUS);
        let hp = self.mem.rw(W_ENEMY_MON_HP);
        let item = self.mem.rb(W_ENEMY_MON_ITEM);
        let sub5 = self.mem.rb(W_ENEMY_SUB_STATUS5);
        self.mem.set_bit(W_ENEMY_SUB_STATUS5, SUBSTATUS_TRANSFORMED);
        // BUG: Catching a Transformed Pokémon always catches a Ditto.
        if sub5 & 1 << SUBSTATUS_TRANSFORMED != 0 {
            self.mem.wb(W_TEMP_ENEMY_MON_SPECIES, DITTO);
        } else {
            self.mem.set_bit(W_ENEMY_SUB_STATUS5, SUBSTATUS_TRANSFORMED);
            let (dv0, dv1) = (self.mem.rb(W_ENEMY_MON_D_VS), self.mem.rb(W_ENEMY_MON_D_VS + 1));
            self.mem.wb(W_ENEMY_BACKUP_D_VS, dv0);
            self.mem.wb(W_ENEMY_BACKUP_D_VS + 1, dv1);
        }
        let species = self.mem.rb(W_TEMP_ENEMY_MON_SPECIES);
        self.mem.wb(W_CUR_PARTY_SPECIES, species);
        let level = self.mem.rb(W_ENEMY_MON_LEVEL);
        self.mem.wb(W_CUR_PARTY_LEVEL, level);
        self.load_enemy_mon();
        self.mem.wb(W_ENEMY_SUB_STATUS5, sub5);
        self.mem.wb(W_ENEMY_MON_ITEM, item);
        self.mem.ww(W_ENEMY_MON_HP, hp);
        self.mem.wb(W_ENEMY_MON_STATUS, status);
        if !self.mem.bit(W_ENEMY_SUB_STATUS5, SUBSTATUS_TRANSFORMED) {
            self.mem.copy(W_ENEMY_MON_MOVES, W_WILD_MON_MOVES, u16::from(NUM_MOVES));
            self.mem.copy(W_ENEMY_MON_PP, W_WILD_MON_PP, u16::from(NUM_MOVES));
        }
        // .Transformed
        let species = self.mem.rb(W_ENEMY_MON_SPECIES);
        self.mem.wb(W_WILD_MON, species);
        self.mem.wb(W_CUR_PARTY_SPECIES, species);
        self.mem.wb(W_TEMP_SPECIES, species);
        if self.mem.rb(W_BATTLE_TYPE) == BATTLETYPE_TUTORIAL {
            self.text(r::TEXT_GOTCHA_MON_WAS_CAUGHT);
            return Ok(self.poke_ball_return_from_capture());
        }
        self.text(r::TEXT_GOTCHA_MON_WAS_CAUGHT);
        // CheckCaughtMon / SetSeenAndCaughtMon
        let index = species.wrapping_sub(1);
        let known = self.small_far_flag_action(W_POKEDEX_CAUGHT, index, FlagAction::Check);
        self.small_far_flag_action(W_POKEDEX_CAUGHT, index, FlagAction::Set);
        self.small_far_flag_action(W_POKEDEX_SEEN, index, FlagAction::Set);
        // CheckReceivedDex
        if !known && self.mem.bit(W_STATUS_FLAGS, STATUSFLAGS_POKEDEX_F) {
            self.text(r::NEW_DEX_DATA_TEXT);
            // predef NewPokedexEntry: graphics only
            self.mem.wb(W_TEMP_SPECIES, species);
        }
        match self.mem.rb(W_BATTLE_TYPE) {
            BATTLETYPE_CONTEST => self.unported("BugContest_SetCaughtContestMon"),
            BATTLETYPE_CELEBI => self.mem.set_bit(W_BATTLE_RESULT, BATTLERESULT_CAUGHT_CELEBI),
            _ => {}
        }
        // TryAddMonToParty / SendMonIntoBox, SetCaughtData: the caught mon
        // joins the party or the box (SRAM).
        self.unported("PokeBallEffect: TryAddMonToParty / SetCaughtData (party or box bookkeeping)")
    }

    /// `UseBallInTrainerBattle`.
    fn use_ball_in_trainer_battle(&mut self) -> Flow {
        // ReturnToBattle_UseBall (patched: nothing)
        self.play_fx_anim_id(ANIM_THROW_POKE_BALL as u8, (ANIM_THROW_POKE_BALL >> 8) as u8);
        self.mem.wb(W_BATTLE_ANIM_PARAM, 0);
        self.set_player_turn();
        self.mem.wb(W_BATTLE_AFTER_ANIM, 0);
        self.text(r::BALL_BLOCKED_TEXT);
        self.text(r::BALL_DONT_BE_A_THIEF_TEXT);
        self.use_disposable_item();
        Ok(())
    }

    /// `GetPokeBallWobble`: 0 = next wobble, 1 = caught, 2 = broke free.
    fn get_poke_ball_wobble(&mut self) -> u8 {
        let count = self.mem.rb(W_THROWN_BALL_WOBBLE_COUNT).wrapping_add(1);
        self.mem.wb(W_THROWN_BALL_WOBBLE_COUNT, count);
        // Wobble up to 3 times.
        if count == 3 + 1 {
            return if self.mem.rb(W_WILD_MON) != 0 { 1 } else { 2 };
        }
        if self.mem.rb(W_WILD_MON) != 0 {
            return 0;
        }
        let rate = self.mem.rb(W_FINAL_CATCH_RATE);
        let mut at = r::WOBBLE_PROBABILITIES;
        while rom::byte(at) < rate {
            at += 2;
        }
        let chance = rom::byte(at + 1);
        let roll = self.random("GetPokeBallWobble");
        if roll < chance { 0 } else { 2 }
    }

    /// The HP-dependent part of `PokeBallEffect` (from
    /// `.skip_or_return_from_ball_fn` to `.skip_hp_calc`): the catch rate `b`
    /// scaled by the enemy's HP, plus the status and held-item bonuses.
    fn catch_rate_from_hp(&mut self, rate: u8) -> u8 {
        self.mem.wb(H_MULTIPLICAND + 2, rate);
        let mut bc = self.mem.rw(W_ENEMY_MON_HP);
        let mut de = self.mem.rw(W_ENEMY_MON_MAX_HP);
        bc = bc.wrapping_shl(1);
        de = de.wrapping_add(de).wrapping_add(self.mem.rw(W_ENEMY_MON_MAX_HP));
        if de >> 8 != 0 {
            de >>= 2;
            bc >>= 2;
            // BUG: Catch rate formula breaks for Pokémon with max HP > 341.
            if bc as u8 == 0 {
                bc = bc & 0xff00 | 1;
            }
        }
        let e = de as u8;
        let c = bc as u8;
        self.mem.wb(H_MULTIPLIER, e.wrapping_sub(c));
        self.mem.wb(H_DIVIDEND, 0);
        self.mem.wb(H_MULTIPLICAND, 0);
        self.mem.wb(H_MULTIPLICAND + 1, 0);
        self.multiply();
        self.mem.wb(H_DIVISOR, e);
        self.divide(4);
        let mut a = self.mem.rb(H_QUOTIENT + 3);
        if a == 0 {
            a = 1;
        }
        // BUG: BRN/PSN/PAR do not affect catch rate.
        let bonus = if self.mem.rb(W_ENEMY_MON_STATUS) & (1 << FRZ | SLP_MASK) != 0 { 10 } else { 0 };
        let mut a = a.checked_add(bonus).unwrap_or(0xff);
        // BUG: HELD_CATCH_CHANCE has no effect (the parameter is read from `c`).
        let item = self.mem.rb(W_BATTLE_MON_ITEM);
        let (effect, param) = self.get_item_held_effect(item);
        if effect == HELD_CATCH_CHANCE {
            a = a.checked_add(param).unwrap_or(0xff);
        }
        a
    }

    /// The `BallMultiplierFunctionTable` routine of `item`, applied to the
    /// catch rate `b`.
    fn ball_multiplier(&mut self, item: u8, b: u8) -> u8 {
        match item {
            ULTRA_BALL => b.checked_mul(2).unwrap_or(0xff),
            GREAT_BALL | SAFARI_BALL | PARK_BALL => (b >> 1).checked_add(b).unwrap_or(0xff),
            HEAVY_BALL => self.heavy_ball_multiplier(b),
            LEVEL_BALL => {
                // x2 if the player's level is above the enemy's, x4 if its half is,
                // x8 if its quarter is
                let mut c = self.mem.rb(W_BATTLE_MON_LEVEL);
                let enemy = self.mem.rb(W_ENEMY_MON_LEVEL);
                if enemy >= c {
                    return b;
                }
                let Some(b) = b.checked_mul(2) else { return 0xff };
                c >>= 1;
                if enemy >= c {
                    return b;
                }
                let Some(b) = b.checked_mul(2) else { return 0xff };
                c >>= 1;
                if enemy >= c {
                    return b;
                }
                b.checked_mul(2).unwrap_or(0xff)
            }
            LURE_BALL => {
                if self.mem.rb(W_BATTLE_TYPE) != BATTLETYPE_FISH {
                    return b;
                }
                b.checked_mul(2).and_then(|d| d.checked_add(b)).unwrap_or(0xff)
            }
            FAST_BALL => {
                let species = self.mem.rb(W_TEMP_ENEMY_MON_SPECIES);
                let mut at = r::FLEE_MONS;
                // BUG: Fast Ball only boosts catch rate for three Pokémon.
                for _ in 0..3 {
                    let a = rom::byte(at);
                    at += 1;
                    if a != 0xff && a == species {
                        let Some(b) = b.checked_mul(2).and_then(|d| d.checked_mul(2)) else { return 0xff };
                        return b;
                    }
                }
                b
            }
            MOON_BALL => {
                let species = self.mem.rb(W_TEMP_ENEMY_MON_SPECIES);
                let bank = rom::bank_of(r::EVOS_ATTACKS_POINTERS);
                let ptr = rom::ptr(r::EVOS_ATTACKS_POINTERS + usize::from(species.wrapping_sub(1)) * 2);
                let evo = rom::flat(bank, ptr);
                if rom::byte(evo) != EVOLVE_ITEM {
                    return b;
                }
                // BUG: Moon Ball does not boost catch rate (the species byte
                // is compared with the Moon Stone's number).
                if rom::byte(evo + 3) != MOON_STONE_RED {
                    return b;
                }
                b.checked_mul(2).and_then(|d| d.checked_mul(2)).unwrap_or(0xff)
            }
            LOVE_BALL => self.love_ball_multiplier(b),
            _ => b,
        }
    }

    /// `HeavyBallMultiplier`.
    fn heavy_ball_multiplier(&mut self, b: u8) -> u8 {
        let species = self.mem.rb(W_ENEMY_MON_SPECIES);
        let mut hl = rom::ptr(r::POKEDEX_DATA_POINTER_TABLE + usize::from(species.wrapping_sub(1)) * 2);
        // BUG: Heavy Ball uses wrong weight value for three Pokémon.
        let bank = POKEDEX_ENTRY_BANKS[usize::from(species.rotate_left(2) & 3)];
        loop {
            let a = rom::byte(rom::flat(bank, hl));
            hl = hl.wrapping_add(1);
            if a == TERMINATOR {
                break;
            }
        }
        // skip the height
        hl = hl.wrapping_add(2);
        let weight = rom::ptr(rom::flat(bank, hl));
        let mut hl = weight >> 1;
        let mut bc = hl >> 4;
        hl = hl.wrapping_sub(bc);
        bc >>= 1;
        hl = hl.wrapping_sub(bc);
        let c = (hl >> 8) as u8;
        if c < (1024 >> 8) as u8 {
            // .lightmon
            return b.checked_sub(20).unwrap_or(1);
        }
        // .WeightsTable: (weight factor, boost)
        const TABLE: [(u8, u8); 4] = [(8, 0), (12, 20), (16, 30), (0xff, 40)];
        let mut at = 0;
        while at < 3 && c >= TABLE[at].0 {
            at += 1;
        }
        b.checked_add(TABLE[at].1).unwrap_or(0xff)
    }

    /// `LoveBallMultiplier`.
    fn love_ball_multiplier(&mut self, b: u8) -> u8 {
        // does species match?
        let wild = self.mem.rb(W_TEMP_ENEMY_MON_SPECIES);
        let mine = self.mem.rb(W_TEMP_BATTLE_MON_SPECIES);
        if mine != wild {
            return b;
        }
        // check player mon species
        self.mem.wb(W_CUR_PARTY_SPECIES, mine);
        self.mem.wb(W_MON_TYPE, 0); // PARTYMON
        let cur = self.mem.rb(W_CUR_BATTLE_MON);
        self.mem.wb(W_CUR_PARTY_MON, cur);
        let Some(male) = self.get_gender() else { return b }; // no effect on genderless
        let player = u8::from(!male);
        // check wild mon species
        self.mem.wb(W_CUR_PARTY_SPECIES, wild);
        self.mem.wb(W_MON_TYPE, WILDMON);
        let Some(male) = self.get_gender() else { return b };
        let wild_gender = u8::from(!male);
        // BUG: Love Ball boosts catch rate for the wrong gender.
        if wild_gender != player {
            return b;
        }
        b.checked_mul(2).and_then(|d| d.checked_mul(2)).and_then(|d| d.checked_mul(2)).unwrap_or(0xff)
    }
}
