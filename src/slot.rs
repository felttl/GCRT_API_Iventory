use crate::inv_objs::item_type_id::ItemTypeId;

pub trait Slot {

    /// get identifier of the object in the slot 
    /// (None if empty)
    fn obj(&self) -> Option<ItemTypeId>;

    // can be good to start get methods with g 
    /// instead of get or anything...

    /// get quantity of items in the slot
    /// (None if empty)
    fn qtt(&self) -> Option<u16>;


}

#[derive(Clone)]
pub struct SlotDefault {
    pub(crate) obj: Option<ItemTypeId>,
    pub(crate) qtt: Option<u16>,
}

impl SlotDefault {

    pub fn new() -> Self {
        Self {
            obj: None,
            qtt: None,
        }
    }

}

impl Slot for SlotDefault {
    fn obj(&self) -> Option<ItemTypeId> {
        self.obj
    }
    fn qtt(&self) -> Option<u16> {
        self.qtt
    }
}
