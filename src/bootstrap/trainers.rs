//! Trainer-name lookup. Maps a display name like `"Whitney"` to the
//! `(wOtherTrainerClass, wOtherTrainerID)` pair the engine expects.
//!
//! Source of truth: `scratch/pokecrystal/constants/trainer_constants.asm`
//! (class indices) and `scratch/pokecrystal/data/trainers/parties.asm` (the
//! per-class list of trainer instances). The constants we resolve to are
//! exported in `src/defs.rs`.
//!
//! Two tables back this module:
//!
//! - [`trainers_data::ALL`](super::trainers_data::ALL) — every battleable
//!   trainer instance in the game (generated from `parties.asm`), keyed by a
//!   `CLASS_id` slug. [`all`] returns it, so `--random` can draw the whole cast.
//! - [`ALIASES`] — the curated friendly names (gym leaders, Elite Four,
//!   Champion, Red, Blue, the Eusine cameo) so a user can write
//!   `trainer = "Whitney"` and older recordings keep resolving.
//!
//! [`lookup`] checks the aliases first, then falls back to the full roster.

use crate::defs;

pub struct TrainerDef {
    pub name: &'static str,
    pub class: u8,
    pub id: u8,
    /// Average level of the trainer's canonical party, rounded to the nearest
    /// integer. Source: `scratch/pokecrystal/data/trainers/parties.asm`. Used
    /// by `bootstrap::random` to scale the generated player team.
    pub avg_level: u8,
    /// Number of mons in the trainer's canonical party, from the same source.
    pub party_count: u8,
}

/// Curated friendly names for the unique trainers. These are *aliases* on top
/// of [`trainers_data::ALL`](super::trainers_data::ALL): each resolves to the
/// same `(class, id)` as the corresponding `CLASS_id` slug there, but lets
/// callers (and older recordings) use a human name like `"Whitney"` or
/// `"Lance"`. [`lookup`] consults this list before the full roster.
const ALIASES: &[TrainerDef] = &[
    // Johto gym leaders
    TrainerDef { name: "Falkner",  class: defs::FALKNER,  id: defs::FALKNER1,  avg_level:  8, party_count: 2 },
    TrainerDef { name: "Bugsy",    class: defs::BUGSY,    id: defs::BUGSY1,    avg_level: 15, party_count: 3 },
    TrainerDef { name: "Whitney",  class: defs::WHITNEY,  id: defs::WHITNEY1,  avg_level: 19, party_count: 2 },
    TrainerDef { name: "Morty",    class: defs::MORTY,    id: defs::MORTY1,    avg_level: 23, party_count: 4 },
    TrainerDef { name: "Chuck",    class: defs::CHUCK,    id: defs::CHUCK1,    avg_level: 29, party_count: 2 },
    TrainerDef { name: "Jasmine",  class: defs::JASMINE,  id: defs::JASMINE1,  avg_level: 32, party_count: 3 },
    TrainerDef { name: "Pryce",    class: defs::PRYCE,    id: defs::PRYCE1,    avg_level: 29, party_count: 3 },
    TrainerDef { name: "Clair",    class: defs::CLAIR,    id: defs::CLAIR1,    avg_level: 38, party_count: 4 },

    // Elite Four + Champion
    TrainerDef { name: "Will",     class: defs::WILL,     id: defs::WILL1,     avg_level: 41, party_count: 5 },
    TrainerDef { name: "Koga",     class: defs::KOGA,     id: defs::KOGA1,     avg_level: 42, party_count: 5 },
    TrainerDef { name: "Bruno",    class: defs::BRUNO,    id: defs::BRUNO1,    avg_level: 43, party_count: 5 },
    TrainerDef { name: "Karen",    class: defs::KAREN,    id: defs::KAREN1,    avg_level: 44, party_count: 5 },
    TrainerDef { name: "Lance",    class: defs::CHAMPION, id: defs::LANCE,     avg_level: 47, party_count: 6 },

    // Kanto gym leaders
    TrainerDef { name: "Brock",    class: defs::BROCK,    id: defs::BROCK1,    avg_level: 42, party_count: 5 },
    TrainerDef { name: "Misty",    class: defs::MISTY,    id: defs::MISTY1,    avg_level: 44, party_count: 4 },
    TrainerDef { name: "Lt. Surge", class: defs::LT_SURGE, id: defs::LT_SURGE1, avg_level: 42, party_count: 5 },
    TrainerDef { name: "Erika",    class: defs::ERIKA,    id: defs::ERIKA1,    avg_level: 44, party_count: 4 },
    TrainerDef { name: "Janine",   class: defs::JANINE,   id: defs::JANINE1,   avg_level: 36, party_count: 5 },
    TrainerDef { name: "Sabrina",  class: defs::SABRINA,  id: defs::SABRINA1,  avg_level: 47, party_count: 3 },
    TrainerDef { name: "Blaine",   class: defs::BLAINE,   id: defs::BLAINE1,   avg_level: 47, party_count: 3 },

    // Misc unique
    TrainerDef { name: "Blue",     class: defs::BLUE,     id: defs::BLUE1,     avg_level: 57, party_count: 6 },
    TrainerDef { name: "Red",      class: defs::RED,      id: defs::RED1,      avg_level: 77, party_count: 6 },
    TrainerDef { name: "Eusine",   class: defs::MYSTICALMAN, id: defs::EUSINE, avg_level: 24, party_count: 3 },
];

/// Every battleable trainer instance in the game, in ROM order.
/// `bootstrap::random` picks one of these uniformly when `--random` is set.
pub fn all() -> &'static [TrainerDef] {
    super::trainers_data::ALL
}

/// Resolve a trainer name to `(class, id)`. Case-insensitive and ignores
/// punctuation, so `"Whitney"`, `"WHITNEY"`, `"lt surge"`, and `"Lt. Surge"`
/// all match the curated aliases, while `"YOUNGSTER_1"` and `"youngster 1"`
/// match the generated full roster. Aliases win on conflict.
pub fn lookup(name: &str) -> Option<&'static TrainerDef> {
    let normalized = normalize(name);
    ALIASES
        .iter()
        .chain(super::trainers_data::ALL)
        .find(|t| normalize(t.name) == normalized)
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
    fn whitney_resolves() {
        let t = lookup("Whitney").unwrap();
        assert_eq!(t.class, defs::WHITNEY);
        assert_eq!(t.id, defs::WHITNEY1);
    }

    #[test]
    fn case_and_punctuation_tolerant() {
        assert!(lookup("WHITNEY").is_some());
        assert!(lookup("whitney").is_some());

        let t = lookup("lt. surge").unwrap();
        assert_eq!(t.class, defs::LT_SURGE);
        let t2 = lookup("LT_SURGE").unwrap();
        assert_eq!(t2.class, defs::LT_SURGE);
    }

    #[test]
    fn lance_is_champion() {
        let t = lookup("Lance").unwrap();
        assert_eq!(t.class, defs::CHAMPION);
        assert_eq!(t.id, defs::LANCE);
    }

    #[test]
    fn unknown_trainer() {
        assert!(lookup("Giovanni").is_none());
    }

    /// The full roster must be present and every entry self-consistent: a
    /// well-formed `(class, id)`, plausible party stats, a unique slug, and a
    /// name that round-trips through `lookup` back to the same `(class, id)`.
    #[test]
    fn roster_is_complete_and_well_formed() {
        use std::collections::HashSet;
        let roster = all();
        // The game has 541 battleable trainer instances; guard against a
        // truncated or empty generated table without pinning the exact count.
        assert!(roster.len() >= 500, "roster shrank to {} entries", roster.len());

        let mut slugs = HashSet::new();
        for t in roster {
            assert!(slugs.insert(t.name), "duplicate trainer slug {}", t.name);
            assert!(t.class >= defs::FALKNER, "bad class for {}", t.name);
            assert!(t.id >= 1, "bad id for {}", t.name);
            assert!(t.party_count >= 1 && t.party_count <= 6, "bad party_count for {}", t.name);
            assert!(t.avg_level >= 1 && t.avg_level <= 100, "bad avg_level for {}", t.name);
            let resolved = lookup(t.name).expect("roster slug must resolve");
            assert_eq!((resolved.class, resolved.id), (t.class, t.id), "{} drifted", t.name);
        }
    }

    /// A generic, non-unique trainer (one of many Youngsters) is selectable —
    /// the whole point of widening `--random` past the curated uniques.
    #[test]
    fn generic_trainer_selectable() {
        let t = lookup("YOUNGSTER_1").expect("youngster 1 in roster");
        assert_eq!(t.class, defs::YOUNGSTER);
        assert_eq!(t.id, 1);
        assert!(lookup("youngster 1").is_some(), "normalization should accept spaces");
    }

    /// Every curated alias must point at a real roster entry, and the friendly
    /// name and that entry's slug must resolve to the *same* `(class, id)`.
    /// Since a trainer name's only battle effect is the `(class, id)` written to
    /// WRAM, this is what guarantees relabeling a recording's trainer from the
    /// friendly name to its roster slug (e.g. `"Lance"` -> `"CHAMPION_1"`)
    /// leaves the battle byte-identical. Ties the two tables together.
    #[test]
    fn aliases_match_the_roster() {
        for alias in ALIASES {
            let found = all()
                .iter()
                .find(|t| (t.class, t.id) == (alias.class, alias.id));
            let found = found.unwrap_or_else(|| {
                panic!(
                    "alias {} -> (class {}, id {}) has no roster entry",
                    alias.name, alias.class, alias.id,
                )
            });
            // The slug must round-trip to the same identity as the friendly name.
            let via_slug = lookup(found.name).expect("slug resolves");
            let via_alias = lookup(alias.name).expect("alias resolves");
            assert_eq!(
                (via_slug.class, via_slug.id),
                (via_alias.class, via_alias.id),
                "alias {} and slug {} disagree",
                alias.name, found.name,
            );
        }
    }
}
