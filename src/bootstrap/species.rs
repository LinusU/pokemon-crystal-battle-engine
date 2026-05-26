//! Species name <-> id lookup. Mirrors the order in
//! `scratch/pokecrystal/constants/pokemon_constants.asm`; first 251 entries are
//! the Gen 1 + Gen 2 dex (BULBASAUR..CELEBI), 252 is unused (`const_skip`),
//! 253 is EGG. Anything past that (Unown forms) belongs in a separate table.

pub const SPECIES_NAMES: &[&str] = &[
    "BULBASAUR", "IVYSAUR", "VENUSAUR", "CHARMANDER", "CHARMELEON", "CHARIZARD",
    "SQUIRTLE", "WARTORTLE", "BLASTOISE", "CATERPIE", "METAPOD", "BUTTERFREE",
    "WEEDLE", "KAKUNA", "BEEDRILL", "PIDGEY", "PIDGEOTTO", "PIDGEOT",
    "RATTATA", "RATICATE", "SPEAROW", "FEAROW", "EKANS", "ARBOK",
    "PIKACHU", "RAICHU", "SANDSHREW", "SANDSLASH", "NIDORAN_F", "NIDORINA",
    "NIDOQUEEN", "NIDORAN_M", "NIDORINO", "NIDOKING", "CLEFAIRY", "CLEFABLE",
    "VULPIX", "NINETALES", "JIGGLYPUFF", "WIGGLYTUFF", "ZUBAT", "GOLBAT",
    "ODDISH", "GLOOM", "VILEPLUME", "PARAS", "PARASECT", "VENONAT",
    "VENOMOTH", "DIGLETT", "DUGTRIO", "MEOWTH", "PERSIAN", "PSYDUCK",
    "GOLDUCK", "MANKEY", "PRIMEAPE", "GROWLITHE", "ARCANINE", "POLIWAG",
    "POLIWHIRL", "POLIWRATH", "ABRA", "KADABRA", "ALAKAZAM", "MACHOP",
    "MACHOKE", "MACHAMP", "BELLSPROUT", "WEEPINBELL", "VICTREEBEL",
    "TENTACOOL", "TENTACRUEL", "GEODUDE", "GRAVELER", "GOLEM", "PONYTA",
    "RAPIDASH", "SLOWPOKE", "SLOWBRO", "MAGNEMITE", "MAGNETON", "FARFETCH_D",
    "DODUO", "DODRIO", "SEEL", "DEWGONG", "GRIMER", "MUK",
    "SHELLDER", "CLOYSTER", "GASTLY", "HAUNTER", "GENGAR", "ONIX",
    "DROWZEE", "HYPNO", "KRABBY", "KINGLER", "VOLTORB", "ELECTRODE",
    "EXEGGCUTE", "EXEGGUTOR", "CUBONE", "MAROWAK", "HITMONLEE", "HITMONCHAN",
    "LICKITUNG", "KOFFING", "WEEZING", "RHYHORN", "RHYDON", "CHANSEY",
    "TANGELA", "KANGASKHAN", "HORSEA", "SEADRA", "GOLDEEN", "SEAKING",
    "STARYU", "STARMIE", "MR__MIME", "SCYTHER", "JYNX", "ELECTABUZZ",
    "MAGMAR", "PINSIR", "TAUROS", "MAGIKARP", "GYARADOS", "LAPRAS",
    "DITTO", "EEVEE", "VAPOREON", "JOLTEON", "FLAREON", "PORYGON",
    "OMANYTE", "OMASTAR", "KABUTO", "KABUTOPS", "AERODACTYL", "SNORLAX",
    "ARTICUNO", "ZAPDOS", "MOLTRES", "DRATINI", "DRAGONAIR", "DRAGONITE",
    "MEWTWO", "MEW",
    "CHIKORITA", "BAYLEEF", "MEGANIUM", "CYNDAQUIL", "QUILAVA", "TYPHLOSION",
    "TOTODILE", "CROCONAW", "FERALIGATR", "SENTRET", "FURRET", "HOOTHOOT",
    "NOCTOWL", "LEDYBA", "LEDIAN", "SPINARAK", "ARIADOS", "CROBAT",
    "CHINCHOU", "LANTURN", "PICHU", "CLEFFA", "IGGLYBUFF", "TOGEPI",
    "TOGETIC", "NATU", "XATU", "MAREEP", "FLAAFFY", "AMPHAROS",
    "BELLOSSOM", "MARILL", "AZUMARILL", "SUDOWOODO", "POLITOED", "HOPPIP",
    "SKIPLOOM", "JUMPLUFF", "AIPOM", "SUNKERN", "SUNFLORA", "YANMA",
    "WOOPER", "QUAGSIRE", "ESPEON", "UMBREON", "MURKROW", "SLOWKING",
    "MISDREAVUS", "UNOWN", "WOBBUFFET", "GIRAFARIG", "PINECO", "FORRETRESS",
    "DUNSPARCE", "GLIGAR", "STEELIX", "SNUBBULL", "GRANBULL", "QWILFISH",
    "SCIZOR", "SHUCKLE", "HERACROSS", "SNEASEL", "TEDDIURSA", "URSARING",
    "SLUGMA", "MAGCARGO", "SWINUB", "PILOSWINE", "CORSOLA", "REMORAID",
    "OCTILLERY", "DELIBIRD", "MANTINE", "SKARMORY", "HOUNDOUR", "HOUNDOOM",
    "KINGDRA", "PHANPY", "DONPHAN", "PORYGON2", "STANTLER", "SMEARGLE",
    "TYROGUE", "HITMONTOP", "SMOOCHUM", "ELEKID", "MAGBY", "MILTANK",
    "BLISSEY", "RAIKOU", "ENTEI", "SUICUNE", "LARVITAR", "PUPITAR",
    "TYRANITAR", "LUGIA", "HO_OH", "CELEBI",
];

/// Look up a species id (1..=251) by name. Case-insensitive; underscores are
/// optional so `"Mr. Mime"`, `"MR__MIME"`, `"Mr Mime"` all resolve. Returns
/// `None` for unknown names.
pub fn id_from_name(name: &str) -> Option<u8> {
    let normalized = normalize(name);
    SPECIES_NAMES
        .iter()
        .position(|canonical| normalize(canonical) == normalized)
        .map(|i| (i + 1) as u8)
}

fn normalize(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            '♀' => 'F',
            '♂' => 'M',
            other => other,
        })
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| c.to_ascii_uppercase())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_ids() {
        assert_eq!(id_from_name("BULBASAUR"), Some(1));
        assert_eq!(id_from_name("Pikachu"), Some(25));
        assert_eq!(id_from_name("pidgey"), Some(16));
        assert_eq!(id_from_name("MEW"), Some(151));
        assert_eq!(id_from_name("CELEBI"), Some(251));
    }

    #[test]
    fn punctuation_tolerant() {
        assert_eq!(id_from_name("mr. mime"), Some(122));
        assert_eq!(id_from_name("farfetch'd"), Some(83));
        assert_eq!(id_from_name("ho-oh"), Some(250));
        assert_eq!(id_from_name("NIDORAN♀"), Some(29));
    }

    #[test]
    fn unknown() {
        assert!(id_from_name("missingno").is_none());
    }

    #[test]
    fn list_length() {
        assert_eq!(SPECIES_NAMES.len(), 251);
    }
}
