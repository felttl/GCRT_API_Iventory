use std::{fs::File, ptr::NonNull};

use crate::inv_objs::item_type::ItemType;



pub trait Inventory {

    /// sort the inventory with alphabetical item names
    /// and merge duplicate items if they are not merged
    /// or not locked by user
    fn sort(&self);

    /// ?
    fn filter(&self);

    fn add_item(&mut self, item: ItemType, qty: u16);

    fn remove_item(&mut self, x: u16, y: u16);

    /// Description
    ///
    /// #### Arguments
    /// * `itype` - item type (abstract type)
    ///
    /// Returns true (found) | false if not found
    fn is_present(&self, itype: ItemType) -> bool;

    /// recover an item and remove it from the inventory <br>
    /// it is stackable remove one or more of the stack
    /// in the slot
    fn get_item(&self, index: usize) -> Option<&ItemType>;

    /// can lock and item if the item cell contains an item
    fn item_lock(&self, x: u16, y: u16);

    /// can unlock and item if the item cell contains an item
    /// and the item is already locked
    fn item_unlock(&self, x: u16, y: u16);

    /// can lock empty or non empty cells
    fn cell_lock(&mut self, x: u16, y: u16);

    /// can unlock empty or non empty cells
    fn cell_unlock(&mut self, x: u16, y: u16);


    /// get the entire data from inventory
    /// the most precise as possible for exportation
    fn get_snapshot(&self) -> NonNull<String>;



}



