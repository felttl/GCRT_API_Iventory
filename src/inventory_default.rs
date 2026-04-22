// inventory.default.rs

use std::usize;

use crate::bit_set_filter::BitSetFilter;
use crate::inventory::Inventory;
use crate::inv_objs::item_type::ItemType;

#[derive(Clone)]
struct Slot {
    obj: Option<ItemType>,
    qtt: Option<u16>
}

impl Slot {

    pub fn new() -> Self {
        // fill Option with None type
        Self { 
            obj: None,
            qtt: None
        }
    }
    
}

/// default impl for Inventory
pub struct InventoryDefault {
    // filters

    filter_engine: BitSetFilter,
    items: Vec<Vec<Slot>>,
    width: usize,
    height: usize
}

/// default iml for inventory (name reversed
/// for a better access inventory classes variants)
impl InventoryDefault {

    pub fn new(width: usize, height: usize) -> Self {
        let items: Vec<Vec<Slot>> = vec![
            vec![Slot::new();width];
            height
        ];
        let filter: BitSetFilter = BitSetFilter::new();
        // full fill here
        Self { filter_engine: filter, items, width, height }
    }

    fn sort(&self, _inv: Vec<Vec<Slot>>) -> Vec<Vec<Slot>>{ // already private
        let mut res: Vec<Vec<Slot>> = Vec::new();

        res
    }
}

impl Inventory for InventoryDefault {

    fn sort(&self) {
        todo!()
    }
    
    fn filter(&self) {
        todo!()
    }
    
    fn add_item(&mut self, item: ItemType, qty: u16) {
        todo!()
    }
    
    fn remove_item(&mut self, x: u16, y: u16) {
        todo!()
    }
    
    fn is_present(&self, itype: ItemType) -> bool {
        todo!()
    }
    
    fn get_item(&self, index: usize) -> Option<&ItemType> {
        todo!()
    }
    
    fn item_lock(&self, x: u16, y: u16) {
        todo!()
    }
    
    fn item_unlock(&self, x: u16, y: u16) {
        todo!()
    }
    
    fn cell_lock(&mut self, x: u16, y: u16) {
        todo!()
    }
    
    fn cell_unlock(&mut self, x: u16, y: u16) {
        todo!()
    }
    
    fn get_snapshot(&self) -> std::ptr::NonNull<String> {
        todo!()
    }

}