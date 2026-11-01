use std::sync::Arc;

// use gcrt_api_inventory::inventory::InventoryDefault;
// use gcrt_api_inventory::inv_objs::item_type::ItemType;
// use gcrt_api_inventory::inv_objs::item_type::ItemTyped;
// use gcrt_api_inventory::inv_objs::item_type_id::ItemTypeId;
use gcrt_api_inventory::inv_objs::item_type_registry::ItemTypeRegistry;
use gcrt_api_inventory::slot::{Slot, SlotDefault};
use rand::{Rng, SeedableRng}; // toml
use rand_chacha::ChaCha8Rng; // toml
use rand::seq::SliceRandom; // (buitins)


// CANNOT copy-paste parts of API real code to break visibility 
// we are forced to test all implemented things inside the API 
// from outside like the external user will do, we have no other choices
// like "public only to package using default accessibility" in Java
// to do it cleanly

// impl InventoryBenchmark {

//     pub fn new(registry: Arc<ItemTypeRegistry>, width: usize, height: usize) -> Self {
//         let items: Vec<SlotDefault> = vec![
//             SlotDefault::new(); width * height
//         ];
//         let filter: BitSetFilter = BitSetFilter::new(
//             width * height, registry.len()
//         );
//         Self { registry, filter_engine: filter, items, width, height }
//     }

//     /// arg1: criteria for sorting (a lambda function)
//     /// arg2: sorting algorithm (another lambda)
//     fn sort<K, F, A>(&mut self, criteria: F, algo: A) 
//      where 
//       F: Fn(&SlotDefault)->K, 
//       K: Ord, 
//       A: FnOnce(&mut Vec<SlotDefault>, F), {
//         algo(&mut self.items, criteria);
//     }
// }

// IMPL the RADIX algorithm
// rust create already exists to use radix
// see : https://docs.rs/rdxsort/latest/rdxsort/

/// u: number of wanted items
/// seed: seed value for making the data same again
/// job: function to be called n times for generating all the data into the final vector
/// return: a vector of u32 values
fn looop<F>(n: usize, seed: u32,mut job: F) -> Vec<u32> 
 where
  F: FnMut(&mut ChaCha8Rng) -> u32, {
    let mut rng = ChaCha8Rng::seed_from_u64(seed as u64);
    (0..n).map(|_| job(&mut rng)).collect()
}

fn main(){

    // create the data

    // --- Profiles ---
    // 
    //  1 random
    //  2 already sorted
    //  3 reversed
    //  4 almost sorted
    //  5 many duplications
    //  6 many focused values
    //  7 high spread
    //  8 patterns

    // initialize generators
    let seed: u32 = 42;
    // put all profiles generators into a vector for calling into a loop
    let profiles: Vec<Box<dyn Fn(usize, u32) -> Vec<u32>>> = vec![
        Box::new(|n, seed| looop(n,seed,|rng|rng.random())), // random
        Box::new(|n, seed| { // sorted
            let mut data = looop(n, seed, |rng| rng.random());
            data.sort();
            data
        }),
        Box::new(|n, seed| { // reversed
            let mut data = looop(n, seed, |rng| rng.random());
            data.sort_by(|a,b| b.cmp(a));
            data
        }),
        Box::new(|n, seed| { // almost sorted
            let mut data = looop(n, seed, |rng| rng.random());
            data.sort();
            for i in (1..n).step_by(100) {
                data.swap(i-1,i);
            }
            data
        }),
        Box::new(|n, seed| looop(n,seed,|rng|rng.random_range(0..10))), // many duplicates
        Box::new(|n, seed| looop(n,seed,|rng|rng.random_range(0..100))), // focused values
        Box::new(|n, seed| {  // high spread
            let mut data: Vec<u32> = (0..n)
             .map(|i| ((i as u64 * u32::MAX as u64)/n as u64) as u32)
             .collect();
            let mut rng: ChaCha8Rng = ChaCha8Rng::seed_from_u64(seed as u64);
            data.shuffle(&mut rng);
            data
        }),
        Box::new(|n, seed| { // repeating patterns
            let mut rng = ChaCha8Rng::seed_from_u64(seed as u64);
            let pattern_len = rng.random_range(2..20);
            let pattern: Vec<u32> = (0..pattern_len)
             .map(|_| rng.random_range(0..100))
             .collect();
            let offset = rng.random_range(0..pattern_len);
            (0..n)
             .map(|i| pattern[(i + offset) % pattern_len])
             .collect()
        }),
    ];

    // lets sort by value

    // bon je parles en français ça sera plus simple
    // apres chaque génération des données on va faire un trie sur les données et on va vérifier
    // mais on ne vas pas sauvegarder les données triées car ça ne sers a rien la vérification suffira amplement.

    // storage size : 8*6*10^6

    // self.sort(
    //     |slot| 0, // 0 TEMPORARILY
    //     |items, criteria| items.sort_by_key(criteria)
    // );

}

