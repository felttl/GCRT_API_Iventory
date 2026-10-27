use crate::inv_objs::item_type::ItemType;
use crate::inv_objs::item_type::ItemTyped;
use crate::inv_objs::item_type_id::ItemTypeId;
use crate::inv_objs::item_type_registry::ItemTypeRegistry;
use crate::slot::{Slot, SlotDefault};
use crate::bit_set_filter::BitSetFilter;

use std:sync::Arc;

pub trait Inventory {

    /// sort the inventory with alphabetical item names
    /// and merge duplicate items if they are not merged
    /// or not locked by user
    fn sort(&mut self);

    /// fiter inventory by categories (show only concerned items)
    fn filter(&mut self); // generics required "compatible" (like in java)

    // filter by name (filter by the native name (name in the language choosed by user) and name in english)
    // resume: name can be any language, but once the language is chosen 
    // filter will only filter the word by the language chosen but english name still always be possible to filter
    fn filter_name(&mut self, name: String);

    /// item will be added (fastest add method)
    /// if the item is present in a slot and the stack slot is not 
    /// full it will increment the stack count, if the stack is full 
    /// another slot will be allocated for it, if no other occurences
    /// exists already it will be added at the first empty slot found,
    /// when the quantity is slightly higher then the max items per stack
    /// another slot will be taken for the rest of the items.
    /// (respects constraints)
    fn add_item(&mut self, item: ItemTypeId, qtt: u16);

    /// item will be added at specific location of the inventory
    /// (respects contraints)
    fn add_item_area(&mut self, item: ItemTypeId, qtt: u16, x: u16, y:u16);

    // remove item from location
    fn remove_item(&mut self, x: u16, y: u16);

    fn unstack(&self, x:u16, y:u16) -> Box<dyn Slot>; // return a slot not an inventory object (slot = inv object + quantity + check if slot locked)

    /// Description
    ///
    /// #### Arguments
    /// * `itype` - item type (abstract type) (reference)
    ///
    /// Returns true (found) | false if not found
    fn is_present(&self, itype: ItemTypeId) -> bool;

    /// recover an item and remove it from the inventory <br>
    /// it is stackable remove one or more of the stack
    /// in the slot
    /// Returns dyn pointer
    fn get_item(&self, index: usize) -> Option<ItemTypeId>;

    /// can lock and item if the item cell contains an item
    fn item_lock(&mut self, x: u16, y: u16);

    /// can unlock and item if the item cell contains an item
    /// and the item is already locked
    fn item_unlock(&mut self, x: u16, y: u16);

    /// can lock empty or non empty cells
    fn cell_lock(&mut self, x: u16, y: u16);

    /// can unlock empty or non empty cells
    fn cell_unlock(&mut self, x: u16, y: u16);

    /// get the entire data from inventory
    /// the most precise as possible for exportation
    fn get_snapshot(&mut self) -> String;

    /// fusion stacks of same object together
    /// remove white spaces (holes) after fusion
    /// holes: 0 or 1 extracted from the first column on right
    fn fusion(&mut self, holes: u8);

    /// merge another inventory into this one
    fn merge(&mut self, inventory: &Self) where Self:Sized;



}

/// ## Default API inventory that impl the `Inventory` Trait
/// default impl for inventory (name reversed
/// for a better access inventory classes variants)
pub struct InventoryDefault {
    registry: Arc<ItemTypeRegistry>,
    filter_engine: BitSetFilter,
    items: Vec<SlotDefault>,
    width: usize,
    height: usize
}

// do not put RustDoc here
impl InventoryDefault {

    pub fn new(registry: Arc<ItemTypeRegistry>, width: usize, height: usize) -> Self {
        let items: Vec<SlotDefault> = vec![
            SlotDefault::new(); width * height
        ];
        let filter: BitSetFilter = BitSetFilter::new(
            width * height, registry.len()
        );
        Self { registry, filter_engine: filter, items, width, height }
    }

    /// ## sorting method
    /// will sort the inventory within a criteria
    /// 
    /// key: is the lambda function that returns the citeria of each<br>
    /// objects in the inventory to sort on (example: `|a| a.name` will sort by `name`)
    fn sort<K, F>(&self, _inv: Vec<SlotDefault>, key: F) 
     where F: Fn(&SlotDefault)->K, K: Ord, {
        // TODO: impl here
    }
}

// do not put RustDoc here
impl Inventory for InventoryDefault {

    // fn pop() // will be interesting here (remove and item from inv and return it)

    fn sort(&mut self) {
        todo!()
    }
    
    fn filter(&mut self) {
        todo!()
    }
    
    fn add_item(&mut self, item: ItemTypeId, qty: u16) {
        todo!()
    }
    
    fn remove_item(&mut self, x: u16, y: u16) {
        todo!()
    }
    
    fn is_present(&self, itype: ItemTypeId) -> bool {
        todo!()
    }
    
    fn get_item(&self, index: usize) -> Option<ItemTypeId> {
        todo!()
    }
    
    fn item_lock(&mut self, x: u16, y: u16) {
        todo!()
    }
    
    fn item_unlock(&mut self, x: u16, y: u16) {
        todo!()
    }
    
    fn cell_lock(&mut self, x: u16, y: u16) {
        todo!()
    }
    
    fn cell_unlock(&mut self, x: u16, y: u16) {
        todo!()
    }
    
    fn get_snapshot(&mut self) -> String {
        todo!()
    }
    
    fn filter_name(&mut self, name: String) {
        todo!()
    }
    
    fn add_item_area(&mut self, item: ItemTypeId, qtt: u16, x: u16, y:u16) {
        todo!()
    }
    
    fn fusion(&mut self, holes: u8) {
        todo!()
    }
    
    fn merge(&mut self, other_inventory: &Self) {
        todo!()
    }

    fn unstack(&self, x:u16, y:u16) -> Box<dyn Slot>{
        todo!()
    }

}

// on devrait pouvoir ajouter dynamiquement les comportements
// au lieu de les fixer dans le trait pour éviter
// d'avoir des méthodes inutiles pour certaines applis (surface d'attaque + élevé et donc risque de sécurité plus grand que ce qui est utilisé)
// par contre on veut pouvoir avoir déja des outils (comme le verouillage) qui doit être présent par défaut (surtout en 2026)
// mais on doit pouvoir enlever ce que l'on utilise pas