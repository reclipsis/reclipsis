use std::collections::HashMap;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component, Serialize, Deserialize, Reflect, Clone, Debug, PartialEq)]
pub struct Inventory {
    // HashMap<InventorySlot, ItemId>
    // Slot 0 is ignored
    pub inventory: HashMap<u8, ItemId>,
    pub equipped_item: Option<ItemId>,
}

impl Default for Inventory {
    fn default() -> Self {
        Self {
            inventory: HashMap::new(),
            equipped_item: None,
        }
    }
}

#[derive(Serialize, Deserialize, Reflect, Clone, Copy, Debug, PartialEq)]
pub struct ItemId(pub u32);

#[derive(Clone, Debug, PartialEq)]
pub struct Item {
    pub id: ItemId,
    pub name: String,
}
