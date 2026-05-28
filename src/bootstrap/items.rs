//! Item name <-> id lookup with pocket assignments. Ids come from
//! `scratch/pokecrystal/constants/item_constants.asm`; pockets from
//! `scratch/pokecrystal/data/items/attributes.asm`.
//!
//! Only items that go in the `ITEM` and `BALL` pockets are listed — key items,
//! TMs, HMs, mail, and unused `ITEM_XX` slots are intentionally absent. Extend
//! as needed.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pocket {
    /// `ITEM_POCKET`. Stored at `wNumItems` / `wItems`. Cap is `MAX_ITEMS` (20).
    Item,
    /// `BALL_POCKET`. Stored at `wNumBalls` / `wBalls`. Cap is `MAX_BALLS` (12).
    Ball,
}

pub struct ItemDef {
    pub name: &'static str,
    pub id: u8,
    pub pocket: Pocket,
}

const ITEMS: &[ItemDef] = &[
    // Balls
    ItemDef { name: "Master Ball",   id: 0x01, pocket: Pocket::Ball },
    ItemDef { name: "Ultra Ball",    id: 0x02, pocket: Pocket::Ball },
    ItemDef { name: "Great Ball",    id: 0x04, pocket: Pocket::Ball },
    ItemDef { name: "Poke Ball",     id: 0x05, pocket: Pocket::Ball },
    ItemDef { name: "Heavy Ball",    id: 0x9d, pocket: Pocket::Ball },
    ItemDef { name: "Level Ball",    id: 0x9f, pocket: Pocket::Ball },
    ItemDef { name: "Lure Ball",     id: 0xa0, pocket: Pocket::Ball },
    ItemDef { name: "Fast Ball",     id: 0xa1, pocket: Pocket::Ball },
    ItemDef { name: "Friend Ball",   id: 0xa4, pocket: Pocket::Ball },
    ItemDef { name: "Moon Ball",     id: 0xa5, pocket: Pocket::Ball },
    ItemDef { name: "Love Ball",     id: 0xa6, pocket: Pocket::Ball },
    ItemDef { name: "Park Ball",     id: 0xb1, pocket: Pocket::Ball },

    // HP-restoring items
    ItemDef { name: "Potion",        id: 0x12, pocket: Pocket::Item },
    ItemDef { name: "Super Potion",  id: 0x11, pocket: Pocket::Item },
    ItemDef { name: "Hyper Potion",  id: 0x10, pocket: Pocket::Item },
    ItemDef { name: "Max Potion",    id: 0x0f, pocket: Pocket::Item },
    ItemDef { name: "Full Restore",  id: 0x0e, pocket: Pocket::Item },
    ItemDef { name: "Fresh Water",   id: 0x2e, pocket: Pocket::Item },
    ItemDef { name: "Soda Pop",      id: 0x2f, pocket: Pocket::Item },
    ItemDef { name: "Lemonade",      id: 0x30, pocket: Pocket::Item },
    ItemDef { name: "Moomoo Milk",   id: 0x48, pocket: Pocket::Item },
    ItemDef { name: "Berry Juice",   id: 0x8b, pocket: Pocket::Item },
    ItemDef { name: "Ragecandybar",  id: 0x72, pocket: Pocket::Item },
    ItemDef { name: "Energypowder",  id: 0x79, pocket: Pocket::Item },
    ItemDef { name: "Energy Root",   id: 0x7a, pocket: Pocket::Item },
    ItemDef { name: "Heal Powder",   id: 0x7b, pocket: Pocket::Item },
    ItemDef { name: "Revival Herb",  id: 0x7c, pocket: Pocket::Item },

    // Status heals
    ItemDef { name: "Antidote",      id: 0x09, pocket: Pocket::Item },
    ItemDef { name: "Burn Heal",     id: 0x0a, pocket: Pocket::Item },
    ItemDef { name: "Ice Heal",      id: 0x0b, pocket: Pocket::Item },
    ItemDef { name: "Awakening",     id: 0x0c, pocket: Pocket::Item },
    ItemDef { name: "Parlyz Heal",   id: 0x0d, pocket: Pocket::Item },
    ItemDef { name: "Full Heal",     id: 0x26, pocket: Pocket::Item },

    // Revives
    ItemDef { name: "Revive",        id: 0x27, pocket: Pocket::Item },
    ItemDef { name: "Max Revive",    id: 0x28, pocket: Pocket::Item },
    ItemDef { name: "Sacred Ash",    id: 0x9c, pocket: Pocket::Item },

    // PP recovery
    ItemDef { name: "Ether",         id: 0x3f, pocket: Pocket::Item },
    ItemDef { name: "Max Ether",     id: 0x40, pocket: Pocket::Item },
    ItemDef { name: "Elixer",        id: 0x41, pocket: Pocket::Item },
    ItemDef { name: "Max Elixer",    id: 0x15, pocket: Pocket::Item },
    ItemDef { name: "PP Up",         id: 0x3e, pocket: Pocket::Item },

    // Battle stat boosters
    ItemDef { name: "X Attack",      id: 0x31, pocket: Pocket::Item },
    ItemDef { name: "X Defend",      id: 0x33, pocket: Pocket::Item },
    ItemDef { name: "X Speed",       id: 0x34, pocket: Pocket::Item },
    ItemDef { name: "X Special",     id: 0x35, pocket: Pocket::Item },
    ItemDef { name: "X Accuracy",    id: 0x21, pocket: Pocket::Item },
    ItemDef { name: "Dire Hit",      id: 0x2c, pocket: Pocket::Item },
    ItemDef { name: "Guard Spec",    id: 0x29, pocket: Pocket::Item },

    // Escape / repel
    ItemDef { name: "Escape Rope",   id: 0x13, pocket: Pocket::Item },
    ItemDef { name: "Repel",         id: 0x14, pocket: Pocket::Item },
    ItemDef { name: "Super Repel",   id: 0x2a, pocket: Pocket::Item },
    ItemDef { name: "Max Repel",     id: 0x2b, pocket: Pocket::Item },
    ItemDef { name: "Poke Doll",     id: 0x25, pocket: Pocket::Item },
    ItemDef { name: "Smoke Ball",    id: 0x6a, pocket: Pocket::Item },

    // Evolution stones
    ItemDef { name: "Moon Stone",    id: 0x08, pocket: Pocket::Item },
    ItemDef { name: "Fire Stone",    id: 0x16, pocket: Pocket::Item },
    ItemDef { name: "Thunderstone",  id: 0x17, pocket: Pocket::Item },
    ItemDef { name: "Water Stone",   id: 0x18, pocket: Pocket::Item },
    ItemDef { name: "Leaf Stone",    id: 0x22, pocket: Pocket::Item },
    ItemDef { name: "Sun Stone",     id: 0xa9, pocket: Pocket::Item },
    ItemDef { name: "Everstone",     id: 0x70, pocket: Pocket::Item },

    // Permanent stat boosters
    ItemDef { name: "HP Up",         id: 0x1a, pocket: Pocket::Item },
    ItemDef { name: "Protein",       id: 0x1b, pocket: Pocket::Item },
    ItemDef { name: "Iron",          id: 0x1c, pocket: Pocket::Item },
    ItemDef { name: "Carbos",        id: 0x1d, pocket: Pocket::Item },
    ItemDef { name: "Calcium",       id: 0x1f, pocket: Pocket::Item },
    ItemDef { name: "Rare Candy",    id: 0x20, pocket: Pocket::Item },

    // Valuables
    ItemDef { name: "Nugget",        id: 0x24, pocket: Pocket::Item },
    ItemDef { name: "Pearl",         id: 0x6e, pocket: Pocket::Item },
    ItemDef { name: "Big Pearl",     id: 0x6f, pocket: Pocket::Item },
    ItemDef { name: "Stardust",      id: 0x83, pocket: Pocket::Item },
    ItemDef { name: "Star Piece",    id: 0x84, pocket: Pocket::Item },
    ItemDef { name: "Tinymushroom",  id: 0x56, pocket: Pocket::Item },
    ItemDef { name: "Big Mushroom",  id: 0x57, pocket: Pocket::Item },

    // Berries
    ItemDef { name: "Berry",         id: 0xad, pocket: Pocket::Item },
    ItemDef { name: "Gold Berry",    id: 0xae, pocket: Pocket::Item },
    ItemDef { name: "Psncureberry",  id: 0x4a, pocket: Pocket::Item },
    ItemDef { name: "Przcureberry",  id: 0x4e, pocket: Pocket::Item },
    ItemDef { name: "Burnt Berry",   id: 0x4f, pocket: Pocket::Item },
    ItemDef { name: "Ice Berry",     id: 0x50, pocket: Pocket::Item },
    ItemDef { name: "Bitter Berry",  id: 0x53, pocket: Pocket::Item },
    ItemDef { name: "Mint Berry",    id: 0x54, pocket: Pocket::Item },
    ItemDef { name: "Miracleberry",  id: 0x6d, pocket: Pocket::Item },
    ItemDef { name: "Mysteryberry",  id: 0x96, pocket: Pocket::Item },

    // Held items (live in the ITEM pocket; the engine only cares once equipped)
    ItemDef { name: "Brightpowder",  id: 0x03, pocket: Pocket::Item },
    ItemDef { name: "Metal Powder",  id: 0x23, pocket: Pocket::Item },
    ItemDef { name: "Lucky Punch",   id: 0x1e, pocket: Pocket::Item },
    ItemDef { name: "Quick Claw",    id: 0x49, pocket: Pocket::Item },
    ItemDef { name: "Soft Sand",     id: 0x4c, pocket: Pocket::Item },
    ItemDef { name: "Sharp Beak",    id: 0x4d, pocket: Pocket::Item },
    ItemDef { name: "Poison Barb",   id: 0x51, pocket: Pocket::Item },
    ItemDef { name: "King's Rock",   id: 0x52, pocket: Pocket::Item },
    ItemDef { name: "Silverpowder",  id: 0x58, pocket: Pocket::Item },
    ItemDef { name: "Amulet Coin",   id: 0x5b, pocket: Pocket::Item },
    ItemDef { name: "Cleanse Tag",   id: 0x5e, pocket: Pocket::Item },
    ItemDef { name: "Mystic Water",  id: 0x5f, pocket: Pocket::Item },
    ItemDef { name: "Twistedspoon",  id: 0x60, pocket: Pocket::Item },
    ItemDef { name: "Blackbelt",     id: 0x62, pocket: Pocket::Item },
    ItemDef { name: "Blackglasses",  id: 0x66, pocket: Pocket::Item },
    ItemDef { name: "Slowpoketail",  id: 0x67, pocket: Pocket::Item },
    ItemDef { name: "Pink Bow",      id: 0x68, pocket: Pocket::Item },
    ItemDef { name: "Stick",         id: 0x69, pocket: Pocket::Item },
    ItemDef { name: "Nevermeltice",  id: 0x6b, pocket: Pocket::Item },
    ItemDef { name: "Magnet",        id: 0x6c, pocket: Pocket::Item },
    ItemDef { name: "Spell Tag",     id: 0x71, pocket: Pocket::Item },
    ItemDef { name: "Miracle Seed",  id: 0x75, pocket: Pocket::Item },
    ItemDef { name: "Thick Club",    id: 0x76, pocket: Pocket::Item },
    ItemDef { name: "Focus Band",    id: 0x77, pocket: Pocket::Item },
    ItemDef { name: "Hard Stone",    id: 0x7d, pocket: Pocket::Item },
    ItemDef { name: "Lucky Egg",     id: 0x7e, pocket: Pocket::Item },
    ItemDef { name: "Charcoal",      id: 0x8a, pocket: Pocket::Item },
    ItemDef { name: "Scope Lens",    id: 0x8c, pocket: Pocket::Item },
    ItemDef { name: "Metal Coat",    id: 0x8f, pocket: Pocket::Item },
    ItemDef { name: "Dragon Fang",   id: 0x90, pocket: Pocket::Item },
    ItemDef { name: "Leftovers",     id: 0x92, pocket: Pocket::Item },
    ItemDef { name: "Dragon Scale",  id: 0x97, pocket: Pocket::Item },
    ItemDef { name: "Berserk Gene",  id: 0x98, pocket: Pocket::Item },
    ItemDef { name: "Light Ball",    id: 0xa3, pocket: Pocket::Item },
    ItemDef { name: "Polkadot Bow",  id: 0xaa, pocket: Pocket::Item },
    ItemDef { name: "Up-Grade",      id: 0xac, pocket: Pocket::Item },
    ItemDef { name: "Brick Piece",   id: 0xb4, pocket: Pocket::Item },
];

/// Look up an item definition by display name. Case-insensitive; punctuation
/// and whitespace are ignored, so `"Super Potion"`, `"super potion"`,
/// `"SUPER_POTION"`, and `"superpotion"` all resolve. Returns `None` for
/// unknown or unsupported names.
pub fn lookup(name: &str) -> Option<&'static ItemDef> {
    let normalized = normalize(name);
    ITEMS.iter().find(|d| normalize(d.name) == normalized)
}

/// Reverse lookup — id to display name. Returns `None` for ids not in our
/// table (key items, TMs, HMs, mail, unused slots).
pub fn name_from_id(id: u8) -> Option<&'static str> {
    ITEMS.iter().find(|d| d.id == id).map(|d| d.name)
}

fn normalize(s: &str) -> String {
    s.chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| c.to_ascii_uppercase())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn potion_lookups() {
        let p = lookup("Potion").unwrap();
        assert_eq!(p.id, 0x12);
        assert_eq!(p.pocket, Pocket::Item);

        let sp = lookup("Super Potion").unwrap();
        assert_eq!(sp.id, 0x11);
        assert_eq!(sp.pocket, Pocket::Item);
    }

    #[test]
    fn ball_pocket() {
        let pb = lookup("poke ball").unwrap();
        assert_eq!(pb.id, 0x05);
        assert_eq!(pb.pocket, Pocket::Ball);
    }

    #[test]
    fn punctuation_tolerant() {
        assert!(lookup("SUPER_POTION").is_some());
        assert!(lookup("superpotion").is_some());
        assert_eq!(lookup("kings rock").unwrap().id, 0x52);
        assert_eq!(lookup("Up Grade").unwrap().id, 0xac);
    }

    #[test]
    fn unknown() {
        assert!(lookup("Master Sword").is_none());
    }

    #[test]
    fn no_duplicate_ids() {
        let mut ids: Vec<u8> = ITEMS.iter().map(|d| d.id).collect();
        ids.sort();
        let len_before = ids.len();
        ids.dedup();
        assert_eq!(len_before, ids.len(), "duplicate item ids in table");
    }
}
