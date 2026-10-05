// Item.rs

use std::any::Any;
use core::clone::Clone;



pub struct ItemSru {
}

pub trait Item: Clone + Any {

    /// id of the type of the item
    fn type_id(&self) -> u32;

    /// name of the type of the item
    fn type_name(&self) -> &str;

    fn Clone(&self) -> Box<dyn ItemSru>;

}


/// default impl of clone_box for all sub conctrete types
// impl<T: Item + Clone + 'static> Item for T {
//     fn clone_box(&self) -> Box<dyn Item> {
//         Box::new(self.clone())
//     }
// }

impl Item for ItemSru {

    fn type_id(&self) -> u32 {
        todo!()
    }

    fn type_name(&self) -> &str {
        todo!()
    }

    fn Clone(&self) -> Box<dyn Item> {
        &self.clone()
    }
}

