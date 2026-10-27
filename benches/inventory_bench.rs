use GCRT_API_Inventory::inventory::InventoryDefault;

use GCRT_API_Inventory::inv_objs::item_type::ItemType;
use GCRT_API_Inventory::inv_objs::item_type::ItemTyped;
use GCRT_API_Inventory::inv_objs::item_type_id::ItemTypeId;
use GCRT_API_Inventory::inv_objs::item_type_registry::ItemTypeRegistry;
use GCRT_API_Inventory::slot::{Slot, SlotDefault};
use GCRT_API_Inventory::bit_set_filter::BitSetFilter;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

// 1er truc : méthode de tri placées et utilisables ici
// 2 : savoir 

// re-write the inventory here because things will change
struct InventoryBenchmark {
    registry: Arc<ItemTypeRegistry>,
    filter_engine: BitSetFilter,
    items: Vec<SlotDefault>,
    width: usize,
    height: usize
}

impl InventoryBenchmark {

    pub fn new(registry: Arc<ItemTypeRegistry>, width: usize, height: usize) -> Self {
        let items: Vec<SlotDefault> = vec![
            SlotDefault::new(); width * height
        ];
        let filter: BitSetFilter = BitSetFilter::new(
            width * height, registry.len()
        );
        Self { registry, filter_engine: filter, items, width, height }
    }

    /// arg1: criteria for sorting (a lambda function)
    /// arg2: sorting algorithm (another lambda)
    fn sort<K, F, A>(&mut self, criteria: F, algo: A) 
     where 
      F: Fn(&SlotDefault)->K, 
      K: Ord, 
      A: FnOnce(&mut Vec<SlotDefault>, F), {
        algo(&mut self.items, criteria);
    }
}

// IMPL the RADIX algorithm
// rust create already exists to use radix
// see : https://docs.rs/rdxsort/latest/rdxsort/

fn looop<F>(n: usize, seed: u32,mut job: F) -> Vec<u32> 
 where
  F: FnMut(&mut ChaCha8Rng) -> u32, {
    let mut rng = ChaCha8Rng::seed_from_u32(seed);
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
    let profiles: Vec<Box<dyn Fn(u32, u32) -> u32>> = vec![
        Box::new(|n, seed| looop(n,seed,|rng|rng.gen())), // random
        Box::new(|n, seed| { // sorted
            let mut data = looop(n, seed, |rng| rng.gen());
            data.sort();
            data
        }),
        Box::new(|n, seed| { // reversed
            let mut data = looop(n, seed, |rng| rng.gen());
            data.sort_by(|a,b| b.cmp(a));
            data
        }),
        Box::new(|n, seed| { // almost sorted
            let mut data = looop(n, seed, |rng| rng.gen());
            data.sort();
            for i in (1..n).step_by(100) {
                data.swap(i-1,i);
            }
            data
        }),
        Box::new(|n, seed| looop(n,seed,|rng|rng.gen_range(0..10))), // many duplicates
        Box::new(|n, seed| looop(n,seed,|rng|rng.gen_range(0..100))), // focused values
        Box::new(|n, seed| {  // high spread
            let mut data: vec<u32> = (0..n)
             .map(|i| ((i as u64 * u32::MAX as u64)/n as u64) as u32)
             .collect();
            let mut rng: ChaCha8Rng = ChaCha8Rng::seed_from_u32(seed);
            data.shuffle(&mut rng);
            data
        }),
        Box::new(|n, seed| (0..n).map(|i| (i%4) as u32).collect()), // repeating patterns
    ];
    let mut datasets: Vec<Vec<u32>> = Vec::new();
    // memory size = sum(sum(6 * 8 * (n+1)**10 * sizeof(u32)) for n in range 6)
    // = around 400 Mo
    for scale in 1..=6 {
        let size = 10_usize.pow(scale);
        for profile in &profiles {
            data.push(profile(size, seed));
        }
    }

    // lets sort by value

    // bon je parles en français ça sera plus simple
    // apres chaque génération des données on va faire un trie sur les données et on va vérifier
    // mais on ne vas pas sauvegarder les données triées car ça ne sers a rien la vérification suffira amplement.

    // storage size : 8*6*10^6

    self.sort(
        |slot| 0, // 0 TEMPORARILY
        |items, criteria| items.sort_by_key(criteria)
    );

}

