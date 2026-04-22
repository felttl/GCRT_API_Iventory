
pub enum ItemData {
    Stackable {
        item_type: ItemType,
        quantity: u16,
    },
    Unique {
        item_type: ItemType,
        ammo: u16,
        durability: u16
    }
}

