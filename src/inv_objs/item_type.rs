// item_type.rs
// use std::slice;
use std::str;

// TODO:  old example to replace with direct 
// types put in the main program 
// pub enum ItemType {
//     WEAPON,
//     OUTFIT,
//     POTION,
//     AMMO, 
//     HOUSING,
//     VEGETABLES
// }

/// #### structure for ItemType representing its data
pub struct ItemTyped {
    name: String

}

pub trait ItemType {
    /// #### get type name 
    /// get a String (struct)
    fn get_name(&self) -> &str;

}

impl ItemType for ItemTyped {


    fn get_name(&self) -> &str {
        &self.name
    }



}

