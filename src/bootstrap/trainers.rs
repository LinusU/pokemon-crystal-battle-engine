//! Trainer-name lookup. Maps a display name like `"Whitney"` to the
//! `(wOtherTrainerClass, wOtherTrainerID)` pair the engine expects.
//!
//! Source of truth: `scratch/pokecrystal/constants/trainer_constants.asm`
//! (class indices) and `scratch/pokecrystal/data/trainers/parties.asm` (the
//! per-class list of trainer instances). The constants we resolve to are
//! exported in `src/defs.rs`.
//!
//! Only the unique trainers — gym leaders, Elite Four, Champion, Red, Blue,
//! and the Eusine cameo — are listed today. Extend as needed.

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

const TRAINERS: &[TrainerDef] = &[
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

/// All harness-supported unique trainers, in the order they're declared above.
/// `bootstrap::random` picks one of these uniformly when `--random` is set.
pub fn all() -> &'static [TrainerDef] {
    TRAINERS
}

/// Resolve a trainer name to `(class, id)`. Case-insensitive and ignores
/// punctuation, so `"Whitney"`, `"WHITNEY"`, `"lt surge"`, and `"Lt. Surge"`
/// all match.
pub fn lookup(name: &str) -> Option<&'static TrainerDef> {
    let normalized = normalize(name);
    TRAINERS.iter().find(|t| normalize(t.name) == normalized)
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
}
