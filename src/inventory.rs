use std::{fs::File, ptr::NonNull};
use std::any::Any;

use crate::inv_objs::item_type::ItemType;
use crate::inv_objs::item_type::ItemTyped;




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

    /// item will be added randomly (fastest add method)
    /// (respects constraints)
    fn add_item(&mut self, item: ItemTyped, qty: u16);

    /// item will be added at specific location of the inventory
    /// (respects contraints)
    fn add_item_area(&mut self, item: Box<dyn ItemType>, qty: u16);

    fn remove_item(&mut self, x: u16, y: u16);

    /// Description
    ///
    /// #### Arguments
    /// * `itype` - item type (abstract type) (reference)
    ///
    /// Returns true (found) | false if not found
    fn is_present(&self, itype: &dyn ItemType) -> bool;

    /// recover an item and remove it from the inventory <br>
    /// it is stackable remove one or more of the stack
    /// in the slot
    /// Returns dyn pointer
    fn get_item(&self, index: usize) -> Option<&ItemType>;

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
    fn fusion(&mut self, holes: bool);

    /// merge another inventory into this one
    fn merge(&mut self, inventory: &Self);



}

// on devrait pouvoir ajouter dynamiquement les comportements
// au lieu de les fixer dans le trait pour éviter
// d'avoir des méthodes inutiles pour certaines applis (surface d'attaque + élevé et donc risque de sécurité plus grand que ce qui est utilisé)
// par contre on veut pouvoir avoir déja des outils (comme le verouillage) qui doit être présent par défaut (surtout en 2026)
// mais on doit pouvoir enlever ce que l'on utilise pas