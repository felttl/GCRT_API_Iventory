
/// TODO: issue here the struct "BitSetFilter" must
/// be public only for Inventory impl no the rest 
/// of the project so it might not be public (how to do this?)

/// provide a filter for object matrices
/// no business logic
pub(crate) struct BitSetFilter {

    weapon: Vec<u64>,
    equipped: Vec<u64>,
    consumable: Vec<u64>,
    mask_weapon: Vec<u64>
    
}

/// user can add as many objcts types
/// as he wants (weapon, options, outfits, ...)
/// and duplicate all this automaticaly 
/// (clone into mirroring filters) it means
/// 1 filter is created every new unique 
/// added object type
impl BitSetFilter {

    /// initialise empty filter
    pub fn new() -> Self {
        // a
    }

}
