//! Routines the port still needs. Each panics with the original routine's
//! name; delete an entry when its real port lands.

use super::battle::{Engine, Flow};

impl Engine {
    /// `GetItemHeldEffect`: (held effect `b`, parameter `c`) of item `b`.
    pub fn get_item_held_effect(&self, _item: u8) -> (u8, u8) { self.unported("GetItemHeldEffect") }
}
