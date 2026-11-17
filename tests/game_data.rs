//! The generated game data (`crystal_battle::data::crystal`) is the ROM's.

use crystal_battle_harness::data::GameData;
use crystal_battle_harness::defs::*;
use crystal_battle_harness::engine::rsym as r;
use crystal_battle_harness::rom;

fn at(base: usize, id: u8, size: usize) -> usize {
    base + usize::from(id.wrapping_sub(1)) * size
}

#[test]
fn matches_the_rom() {
    let data = GameData::crystal();
    for id in 0..=255u8 {
        let base = at(r::BASE_DATA, id, BASE_DATA_SIZE.into());
        let bytes: Vec<u8> = (0..32).map(|i| rom::byte(base + i)).collect();
        assert_eq!(data.species(id).base_data_bytes().as_slice(), bytes.as_slice(), "species {id}");
        let mv = at(r::MOVES, id, MOVE_LENGTH.into());
        let bytes: Vec<u8> = (0..7).map(|i| rom::byte(mv + i)).collect();
        assert_eq!(data.mv(id).bytes().as_slice(), bytes.as_slice(), "move {id}");
        let item = at(r::ITEM_ATTRIBUTES, id, ITEMATTR_STRUCT_LENGTH.into());
        let it = data.item(id);
        assert_eq!(it.price, rom::ptr(item), "item {id}");
        assert_eq!([it.effect, it.param, it.permissions, it.pocket, it.help], [2, 3, 4, 5, 6].map(|i| rom::byte(item + i)), "item {id}");
        let class = at(r::TRAINER_CLASS_ATTRIBUTES, id, NUM_TRAINER_ATTRIBUTES.into());
        let tc = data.trainer_class(id);
        let attrs: Vec<u8> = (0..7).map(|i| rom::byte(class + i)).collect();
        let mut ours = vec![tc.items[0], tc.items[1], tc.base_money];
        ours.extend(tc.ai_move_weights);
        ours.extend(tc.ai_item_switch);
        assert_eq!(ours, attrs, "trainer class {id}");
    }
}
