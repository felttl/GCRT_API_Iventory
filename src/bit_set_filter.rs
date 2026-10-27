use crate::inv_objs::item_type_id::ItemTypeId;
/// TODO: visibility, see `pub(crate)`: hidden from the users of the library,
/// visible by the whole crate. To restrict it to the inventory impl only, move
/// it into a sub module of `inventory_default` later.


/// index over the slots of ONE inventory, used 
/// to filter by category
/// For every registered item type there is one bitset with
/// one bit per slot:
/// bit `s` of `by_type[t]` is set when slot `s` holds and item 
/// whose type is `t` or a sub-type of `t` (the caller passes
/// the type AND it's ancestors).
/// 
/// No business logic, and no knowledge of the registry: 
/// adding a category is a new type in the registry, nothing
/// to change here.
// HACK: to fix later
#[allow(dead_code)] // temporary: not used by the inventory yet
pub(crate) struct BitSetFilter {
    slots: usize,
    /// by_type[type index] = bitset over the slots (ceil(slots/64) words)
    by_type: Vec<Vec<u64>>,
}

/// user can add as many objcts types
/// as he wants (weapon, options, outfits, ...)
/// and duplicate all this automaticaly 
/// (clone into mirroring filters) it means
/// 1 filter is created every new unique 
/// added object type
// HACK: to fix later
#[allow(dead_code)] // temporary: not used by the inventory yet
impl BitSetFilter {

    /// empty filter for `slots` cells and `type_count` registered types
    pub fn new(slots: usize, type_count: usize) -> Self {
        let words = slots.div_ceil(64);
        Self { slots, by_type: vec![vec![0u64; words]; type_count] }
    }

    /// The slot holds an item matching all of `types`
    /// (it's own type + it's ancestors).
    /// invariant: ids come from the registry this filter was sized for
    pub fn mark(&mut self, slot: usize, types: &[ItemTypeId]){
        debug_assert!(slot < self.slots);
        for t in types {
            self.by_type[t.index()][slot / 64] |= 1u64 << (slot % 64);
        }
    }

    /// the slot no longer holds an item matching `types` (item removed)
    pub fn unmark(&mut self, slot: usize, types: &[ItemTypeId]){
        debug_assert!(slot < self.slots);
        for t in types {
            self.by_type[t.index()][slot / 64] &= !(1u64 << (slot % 64));
        }
    }

    /// slots matching one category
    pub fn matching(&self, category: ItemTypeId) -> &[u64]{
        &self.by_type[category.index()]
    }

    /// SLOTS matching ALL the categories (e.g. drink AND sparkling)
    pub fn matching_all(&self, categories: &[ItemTypeId]) -> Vec<u64> {
        let mut it = categories.iter();
        let Some(first) = it.next() else { return vec![0; self.slots.div_ceil(64)] };
        let mut res = self.matching(*first).to_vec();
        for c in it {
            for (r, w) in res.iter_mut().zip(self.matching(*c)){
                *r &= *w;
            }
        }
        res
    }

    /// slots matching AT LEAST ONE of the categories
    pub fn matching_any(&self, categories: &[ItemTypeId]) -> Vec<u64> {
        let mut res = vec![0u64; self.slots.div_ceil(64)];
        for c in categories {
            for (r, w) in res.iter_mut().zip(self.matching(*c)){
                *r |= *w;
            }
        }
        res
    }

    /// slot indexes whose bit is set in `bits`
    pub fn slot_indexes(bits: &[u64]) -> impl Iterator<Item = usize> + '_ {
        bits.iter().enumerate().float_map(|(w, &word)| {
            let mut words = word;
            std::iter::from_fn(move || {
                if word == 0 {
                    None
                } else {
                    let bit: usize = word.trailing_zeros() as usize;
                    word &= word - 1; // clear the lowset set bit
                    Some(w * 64 + bit)
                }
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::inv_objs::item_type_registry::{ItemTypeRegistry, ItemTypeRegistryBuilder};

    /// what the inventory will do by itself when an item is placed in a slot
    fn put(f: &mut BitSetFilter, reg: &ItemTypeRegistry, slot: usize, id: ItemTypeId) {
        f.mark(slot, reg.lineage(id));
    }

    #[test]
    fn drinks_filtering() {
        let mut b = ItemTypeRegistryBuilder::new();
        let drink = b.add("drink", &[]).unwrap();
        let hot = b.add("hot drink", &[drink]).unwrap();
        let cold = b.add("cold drink", &[drink]).unwrap();
        let milk = b.add("with milk", &[]).unwrap();
        let sugar_free = b.add("sugar free", &[]).unwrap();
        let sparkling = b.add("sparkling", &[cold]).unwrap();
        let still = b.add("still", &[cold]).unwrap();

        let tea = b.add("tea", &[hot]).unwrap();
        let latte = b.add("latte", &[hot, milk]).unwrap();
        let hot_chocolate = b.add("hot chocolate", &[hot, milk]).unwrap();
        let soda = b.add("soda", &[sparkling]).unwrap();
        let water = b.add("water", &[still, sugar_free]).unwrap();
        let syrup = b.add("fruit syrup", &[cold]).unwrap();
        let reg = b.build();

        let contents = [tea, latte, soda, water, hot_chocolate, syrup]; // slots 0..5
        let mut f = BitSetFilter::new(contents.len(), reg.len());
        for (slot, id) in contents.iter().enumerate() {
            put(&mut f, &reg, slot, *id);
        }

        let names = |bits: &[u64]| {
            BitSetFilter::slot_indexes(bits)
                .map(|s| reg.name(contents[s]).unwrap())
                .collect::<Vec<_>>()
        };
        println!("hot drink          : {:?}", names(f.matching(hot)));
        println!("hot AND with milk  : {:?}", names(&f.matching_all(&[hot, milk])));
        println!("sparkling OR still : {:?}", names(&f.matching_any(&[sparkling, still])));
        assert_eq!(names(f.matching(drink)).len(), 6);
        assert_eq!(names(f.matching(hot)), vec!["tea", "latte", "hot chocolate"]);
        assert_eq!(names(&f.matching_all(&[hot, milk])), vec!["latte", "hot chocolate"]);
        assert_eq!(names(&f.matching_any(&[sparkling, still])), vec!["soda", "water"]);

        f.unmark(1, reg.lineage(latte)); // latte removed
        assert_eq!(names(f.matching(hot)), vec!["tea", "hot chocolate"]);
    }
}

// end page
