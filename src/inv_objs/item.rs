// Item.rs

use std::any::Any;
// use core::clone::Clone;

pub trait Item: Any {
    /// id of the type of the item
    fn type_id(&self) -> u32;
    /// name of the type of the item
    fn type_name(&self) -> &str;
    // clone the item
    fn clone_box(&self) -> Box<dyn Item>;
}


