//! Where the emulated harness patches the original's ROM: the
//! `[enemy_trainer]` and `[[enemy_items]]` overrides write into the trainer
//! class's `TrainerClassAttributes` entry, and the boot's NPC names come from
//! ROM addresses.

use crate::config::{AI_ITEM_FLAG_NAMES, AI_MOVE_FLAG_NAMES, EnemyTrainerOverride, resolve_ai_flags};
use crate::defs;

/// `(wMomsName, InitializeNPCNames.Mom)`, Red's, Green's (bank 01).
pub const NPC_NAMES: [(u16, u16); 3] = [(0xd488, 0x5d1f), (0xd49e, 0x5d15), (0xd4a9, 0x5d19)];

// `TrainerClassAttributes` table.
// Symbol `0e:559c TrainerClassAttributes` from scratch/pokecrystal/pokecrystal.sym.
// Flat ROM offset = bank * 0x4000 + (addr - 0x4000) = 0x0e * 0x4000 + (0x559c - 0x4000).
// Each entry is NUM_TRAINER_ATTRIBUTES (7) bytes: item1, item2, basemoney,
// ai_move_lo, ai_move_hi, ai_item_lo, ai_item_hi (little-endian words).
pub const TRNATTR_ROM_FLAT_BASE: usize = 0x0e * 0x4000 + (0x559c - 0x4000); // = 0x3959c

// Byte offsets within one entry (mirrors TRNATTR_* in defs.rs / attributes.asm).
pub const TRNATTR_OFF_ITEM1: usize = 0;
pub const TRNATTR_OFF_ITEM2: usize = 1;
pub const TRNATTR_OFF_AI_MOVE_LO: usize = 3; // TRNATTR_AI_MOVE_WEIGHTS low byte
pub const TRNATTR_OFF_AI_MOVE_HI: usize = 4;
pub const TRNATTR_OFF_AI_ITEM_LO: usize = 5; // TRNATTR_AI_ITEM_SWITCH low byte
pub const TRNATTR_OFF_AI_ITEM_HI: usize = 6;

/// Compute the flat ROM offset of a trainer class's `TrainerClassAttributes`
/// entry. `trainer_class` is 1-based (class 1 = Falkner). The entry is
/// `NUM_TRAINER_ATTRIBUTES` (7) bytes; fields are at `entry_base + offset`.
pub fn trnattr_entry_base(trainer_class: u8) -> usize {
    TRNATTR_ROM_FLAT_BASE + (trainer_class as usize - 1) * defs::NUM_TRAINER_ATTRIBUTES as usize
}

/// The ROM bytes (flat offset, value) an `[enemy_trainer]` override patches in
/// `trainer_class`'s `TrainerClassAttributes` entry: the AI move-flag and AI
/// item/switch-flag words. (The engine patches a typed `TrainerClass`: `config::apply_ai_flag_override`.)
pub fn ai_flag_patches(
    trainer_class: u8,
    ov: &EnemyTrainerOverride,
) -> Result<Vec<(usize, u8)>, String> {
    let entry_base = trnattr_entry_base(trainer_class);
    let mut patches = Vec::new();
    if let Some(names) = &ov.ai_move_flags {
        let word = resolve_ai_flags(names, AI_MOVE_FLAG_NAMES, "ai_move_flags")?;
        patches.push((entry_base + TRNATTR_OFF_AI_MOVE_LO, (word & 0xff) as u8));
        patches.push((entry_base + TRNATTR_OFF_AI_MOVE_HI, (word >> 8) as u8));
    }
    if let Some(names) = &ov.ai_item_flags {
        let word = resolve_ai_flags(names, AI_ITEM_FLAG_NAMES, "ai_item_flags")?;
        patches.push((entry_base + TRNATTR_OFF_AI_ITEM_LO, (word & 0xff) as u8));
        patches.push((entry_base + TRNATTR_OFF_AI_ITEM_HI, (word >> 8) as u8));
    }
    Ok(patches)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The flat offset formula must land exactly where `get_trainer_attributes`
    /// indexes: (class-1) * NUM_TRAINER_ATTRIBUTES bytes from the table base.
    /// Verified against the canonical ROM bytes for Falkner (class 1) and
    /// Whitney (class 2).
    #[test]
    fn trnattr_entry_base_matches_canonical_rom() {
        let rom = crate::rom::ROM;

        // Falkner (class 1): item1=0, item2=0, basemoney=0x19
        let falkner_base = trnattr_entry_base(1);
        assert_eq!(rom[falkner_base], 0x00, "Falkner item1");
        assert_eq!(rom[falkner_base + 1], 0x00, "Falkner item2");
        assert_eq!(rom[falkner_base + 2], 0x19, "Falkner basemoney");
        // ai_move word (little-endian): 0x03d3 (AI_BASIC|AI_SETUP|AI_SMART|
        // AI_AGGRESSIVE|AI_CAUTIOUS|AI_STATUS|AI_RISKY)
        let ai_move = u16::from_le_bytes([rom[falkner_base + 3], rom[falkner_base + 4]]);
        assert_eq!(ai_move, 0x03d3, "Falkner ai_move_flags");
        // ai_item word: 0x0044 (CONTEXT_USE|SWITCH_SOMETIMES)
        let ai_item = u16::from_le_bytes([rom[falkner_base + 5], rom[falkner_base + 6]]);
        assert_eq!(ai_item, 0x0044, "Falkner ai_item_flags");

        // Whitney (class 2): same AI flags as Falkner (identical canonical entry)
        let whitney_base = trnattr_entry_base(2);
        assert_eq!(whitney_base, falkner_base + 7, "Whitney entry immediately follows Falkner");
        assert_eq!(rom[whitney_base + 2], 0x19, "Whitney basemoney");
        let ai_move_w = u16::from_le_bytes([rom[whitney_base + 3], rom[whitney_base + 4]]);
        assert_eq!(ai_move_w, 0x03d3, "Whitney ai_move_flags");
    }
}
