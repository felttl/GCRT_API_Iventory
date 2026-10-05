/// Opaque identifier for an item type, handed out by the registry
/// when a type is registered. The field is private: nobody else
/// can forge an id.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ItemTypeId(u32);

// u32 = 2^(32) allows 4 billion types, means 4b different identifiers
// but it cost the number n of types squarred in memory due to bit comparison
// example: 1e4 types = 1e8 bits = aprox 12.5Mo (100 000 types = 1.25Go of data)

///
/// 
/// 
/// 
impl ItemTypeId {
    /// only the registry (same create) crates ids
    pub(crate) fn new(index: u32) -> Self {
        Self(index)
    }
    /// position of the type in the registry 
    /// (index of it's bitset in the filter)
    pub(crate) fn index(self) -> usize{
        self.0 as usize
    }

}

