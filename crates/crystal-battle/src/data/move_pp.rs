//! Base PP for every move, indexed by move id, and the game's max-PP formula.
//!
//! The party struct stores only *current* PP (low 6 bits) and the PP-Up count
//! (top 2 bits) per move slot — it does NOT store max PP. The battle item code
//! needs max PP to cap Elixer/Max Elixer restores, so we compute it from this
//! table the same way the game does.
//!
//! `MOVE_BASE_PP` is generated from `scratch/pokecrystal/data/moves/moves.asm`
//! (the `PP` field — 6th argument — of each `move` macro entry), ordered by
//! `scratch/pokecrystal/constants/move_constants.asm` so that the array index
//! equals the move id. Index 0 is a sentinel for "no move".
//!
//! [`max_pp`] replicates `ComputeMaxPP`
//! (`scratch/pokecrystal/engine/items/item_effects.asm`).

/// Base PP per move id. Index 0 is a sentinel (no move); ids 1..=251 are the
/// move constants in declaration order. Generated from
/// `scratch/pokecrystal/data/moves/moves.asm`.
pub const MOVE_BASE_PP: [u8; 252] = [
    0, 35, 25, 10, 15, 20, 20, 15, 15, 15, 35, 30, 5, 10, 30, 30,
    35, 35, 20, 15, 20, 20, 10, 20, 30, 5, 25, 15, 15, 15, 25, 20,
    5, 35, 15, 20, 20, 20, 15, 30, 35, 20, 20, 30, 25, 40, 20, 15,
    20, 20, 20, 30, 25, 15, 30, 25, 5, 15, 10, 5, 20, 20, 20, 5,
    35, 20, 25, 20, 20, 20, 15, 20, 10, 10, 40, 25, 10, 35, 30, 15,
    20, 40, 10, 15, 30, 15, 20, 10, 15, 10, 5, 10, 10, 25, 10, 20,
    40, 30, 30, 20, 20, 15, 10, 40, 15, 20, 30, 20, 20, 10, 40, 40,
    30, 30, 30, 20, 30, 10, 10, 20, 5, 10, 30, 20, 20, 20, 5, 15,
    10, 20, 15, 15, 35, 20, 15, 10, 20, 30, 15, 40, 20, 15, 10, 5,
    10, 30, 10, 15, 20, 15, 40, 40, 10, 5, 15, 10, 10, 10, 15, 30,
    30, 10, 10, 20, 10, 1, 1, 10, 10, 10, 5, 15, 25, 15, 10, 15,
    30, 5, 40, 15, 10, 25, 10, 30, 10, 20, 10, 10, 10, 10, 10, 20,
    5, 40, 5, 5, 15, 5, 10, 5, 15, 10, 5, 10, 20, 20, 40, 15,
    10, 20, 20, 25, 5, 15, 10, 5, 20, 15, 20, 25, 20, 5, 30, 5,
    10, 20, 40, 5, 20, 40, 20, 15, 35, 10, 5, 5, 5, 15, 5, 20,
    5, 5, 15, 20, 10, 5, 5, 15, 15, 15, 15, 10,
];

/// Highest valid move id (number of real moves; excludes the index-0 sentinel).
pub const NUM_MOVES: usize = MOVE_BASE_PP.len() - 1;

/// PP byte: low 6 bits = current PP.
pub const PP_MASK: u8 = 0x3f;
/// PP byte: top 2 bits = PP-Up count (0..=3).
const PP_UP_MASK: u8 = 0xc0;

/// Max PP of `move_id` given a PP byte whose top 2 bits hold the PP-Up count.
///
/// Replicates `ComputeMaxPP` (`scratch/pokecrystal/engine/items/item_effects.asm`):
///
/// ```text
/// ComputeMaxPP:
///     ; quotient = base_pp / 5      (integer divide)
///     ; c = num_pp_ups (bits 6-7 of the PP byte)
///     ; for each PP Up:
///     ;   inc = min(quotient, 7)    ; clamp so 40-PP moves cap at 61, not 64
///     ;   b += inc
///     ; max_pp = base_pp + b ... masked with PP_MASK
/// ```
///
/// The `min(quotient, 7)` clamp is the asm's `cp $8 / jr c / ld a, $7`: a 40-PP
/// move would otherwise reach 64 with three PP Ups and overflow into bit 6, so
/// the per-PP-Up bonus is capped at 7, yielding a maximum of 61.
///
/// This reads Pokémon Crystal's base PP; [`max_pp_of`] takes another game's.
pub fn max_pp(move_id: u8, pp_byte: u8) -> u8 {
    max_pp_of(MOVE_BASE_PP[move_id as usize], pp_byte)
}

/// [`max_pp`] for a move whose base PP is `base`.
pub fn max_pp_of(base: u8, pp_byte: u8) -> u8 {
    let num_pp_ups = (pp_byte & PP_UP_MASK) >> 6;
    let per_pp_up = (base / 5).min(7);
    (base.wrapping_add(per_pp_up.wrapping_mul(num_pp_ups))) & PP_MASK
}

/// Current PP (low 6 bits) of a PP byte.
pub fn current_pp(pp_byte: u8) -> u8 {
    pp_byte & PP_MASK
}

/// PP-Up bits (top 2 bits) of a PP byte, to be preserved across writes.
pub fn pp_up_bits(pp_byte: u8) -> u8 {
    pp_byte & PP_UP_MASK
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `ReadMem::base_pp` reads `GameData`; for Crystal that is this table.
    #[test]
    fn table_matches_crystal_game_data() {
        for id in 1..=NUM_MOVES as u8 {
            assert_eq!(MOVE_BASE_PP[usize::from(id)], crate::data::GameData::crystal().mv(id).pp, "move {id}");
        }
    }

    #[test]
    fn table_is_sized_and_indexed_by_move_id() {
        assert_eq!(MOVE_BASE_PP.len(), 252);
        assert_eq!(NUM_MOVES, 251);
        // Spot-check against moves.asm / move_constants.asm: id 1 = POUND (35),
        // id 2 = KARATE_CHOP (25), id 33 = TACKLE (35), id 251 = BEAT_UP (10).
        assert_eq!(MOVE_BASE_PP[1], 35);
        assert_eq!(MOVE_BASE_PP[2], 25);
        assert_eq!(MOVE_BASE_PP[33], 35);
        assert_eq!(MOVE_BASE_PP[251], 10);
        // No move has more than 40 base PP (the move macro asserts PP <= 40).
        assert!(MOVE_BASE_PP.iter().all(|&pp| pp <= 40));
    }

    #[test]
    fn max_pp_no_pp_ups_is_base() {
        // pp_byte with 0 PP-Ups (top bits clear) → max == base.
        assert_eq!(max_pp(1, 0), 35); // POUND
        assert_eq!(max_pp(1, 0x20), 35); // low bits ignored
    }

    #[test]
    fn max_pp_matches_compute_max_pp() {
        // Build a PP byte for `n` PP-Ups (top 2 bits) and assert against the
        // game's documented results. base/5 per PP-Up, clamped to +7 each.
        let pp = |n: u8| n << 6;
        // base 5: +1 each → 5,6,7,8
        let id5 = MOVE_BASE_PP.iter().position(|&p| p == 5).unwrap() as u8;
        assert_eq!(
            [max_pp(id5, pp(0)), max_pp(id5, pp(1)), max_pp(id5, pp(2)), max_pp(id5, pp(3))],
            [5, 6, 7, 8]
        );
        // base 10: +2 each → 10,12,14,16
        let id10 = MOVE_BASE_PP.iter().position(|&p| p == 10).unwrap() as u8;
        assert_eq!(
            [max_pp(id10, pp(0)), max_pp(id10, pp(1)), max_pp(id10, pp(2)), max_pp(id10, pp(3))],
            [10, 12, 14, 16]
        );
        // base 25: +5 each → 25,30,35,40
        let id25 = MOVE_BASE_PP.iter().position(|&p| p == 25).unwrap() as u8;
        assert_eq!(
            [max_pp(id25, pp(0)), max_pp(id25, pp(1)), max_pp(id25, pp(2)), max_pp(id25, pp(3))],
            [25, 30, 35, 40]
        );
        // base 40: +7 each (clamped, NOT +8) → 40,47,54,61 (the asm's max of 61).
        let id40 = MOVE_BASE_PP.iter().position(|&p| p == 40).unwrap() as u8;
        assert_eq!(
            [max_pp(id40, pp(0)), max_pp(id40, pp(1)), max_pp(id40, pp(2)), max_pp(id40, pp(3))],
            [40, 47, 54, 61]
        );
    }

    #[test]
    fn pp_byte_accessors_split_bits() {
        let byte = 0x80 | 23; // 2 PP-Ups, 23 current PP
        assert_eq!(current_pp(byte), 23);
        assert_eq!(pp_up_bits(byte), 0x80);
    }

    #[test]
    fn restorable_predicate_drives_targeting() {
        // The CLI offers a PP item only when some move has current PP < max PP.
        // POUND (id 1) base 35, no PP-Ups: full at 35, restorable below that.
        let id = 1u8;
        let full = 35; // 0 PP-Ups, current 35
        assert_eq!(max_pp(id, full), 35);
        assert!(!(current_pp(full) < max_pp(id, full)), "full PP — no effect");

        let drained = 0; // 0 PP-Ups, current 0
        assert!(current_pp(drained) < max_pp(id, drained), "0 PP — restorable");

        // With 3 PP-Ups, max is 56 (35 + 7*3); 35 current is now below max.
        let three_ups_at_base = 0xc0 | 35;
        assert_eq!(max_pp(id, three_ups_at_base), 56);
        assert!(
            current_pp(three_ups_at_base) < max_pp(id, three_ups_at_base),
            "PP-Ups raise max — 35/56 is restorable",
        );
    }

    #[test]
    fn elixer_caps_at_max_and_preserves_pp_ups() {
        // Replicates the apply path: new = pp_up_bits | min(cur + 10, max).
        let id = 1u8; // POUND, base 35
        let restore = |pp_byte: u8, add: u8| {
            let max = max_pp(id, pp_byte);
            let new = current_pp(pp_byte).saturating_add(add).min(max);
            pp_up_bits(pp_byte) | new
        };
        // 20/35, +10 → 30, no PP-Ups.
        assert_eq!(restore(20, 10), 30);
        // 30/35, +10 → capped at 35.
        assert_eq!(restore(30, 10), 35);
        // 30/35 with 2 PP-Ups (max 49), +10 → 40, PP-Up bits kept.
        let with_ups = 0x80 | 30;
        assert_eq!(max_pp(id, with_ups), 49);
        assert_eq!(restore(with_ups, 10), 0x80 | 40);
        // Max Elixer (full) on the same byte → 49, PP-Up bits kept.
        assert_eq!(pp_up_bits(with_ups) | max_pp(id, with_ups), 0x80 | 49);
    }
}
