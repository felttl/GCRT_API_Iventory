use rand::{Rng, SeedableRng}; // toml
use rand_chacha::ChaCha8Rng; // toml
use rand::seq::SliceRandom; // (buitins)

/// radix sorting for `u32` data types
pub(crate) fn radixu32(data: &[u32]) -> Vec<u32> {
    if data.is_empty() { return data.to_vec(); }
    // determine le nombre de passes
    let max: u32 = *data.iter().max().unwrap();
    if max == 0 { return data.to_vec(); } // 1 or more zeroes
    let passes: u32 = max.ilog10() + 1; // value max of passes = 10 ((WOM)optimisation of grouping variables into a single variable is possible)
    let nbbuckets: u32 = 10; // waste of memory again (WOM)
    // create a vec of "boxes" (vecs of vec)
    let mut buckets: Vec<Vec<u32>> = vec![Vec::new();nbbuckets as usize];
    let mut flattened: Vec<u32> = data.to_vec();
    for p in 0..passes {
        for &d in &flattened { // ascending order
            let digit: usize = ((d / 10u32.pow(p)) % 10) as usize; // storage of only a digit ((WOM) 0 to 9 possible values (a big memory loss fo the rest of the memory to uptimize later))
            buckets[digit as usize].push(d);
        }
        // merge vec of vec into a vec
        flattened = buckets.iter_mut()
            .flat_map(|bucket| bucket.drain(..))
            .collect();
    }
    flattened
}


/// radix sorting for `u32` data types
/// with 256 buckets 2^8=256
pub(crate) fn radixu32_256b(data: &[u32]) -> Vec<u32> {
    if data.is_empty() { return data.to_vec(); }
    // determine le nombre de passes
    let max: u32 = *data.iter().max().unwrap();
    if max == 0 { return data.to_vec(); } // 1 or more zeroes
    let passes: u32 = (32 - max.leading_zeros()).div_ceil(8); // count the max bits of max value (waste of memory (WOM))
    // create a vec of "boxes" (vecs of vec)
    let nbbuckets: u32 = 256;
    let mut buckets: Vec<Vec<u32>> = vec![Vec::new();nbbuckets as usize];
    let mut flattened: Vec<u32> = data.to_vec();
    for p in 0..passes {
        for &d in &flattened { // ascending order
            let digit: usize = ((d / 10u32.pow(p)) % 10) as usize; // storage of only a digit ((WOM) 0 to 9 possible values (a big memory loss fo the rest of the memory to uptimize later))
            buckets[digit].push(d);
        }
        // merge vec of vec into a vec
        flattened = buckets.iter_mut()
            .flat_map(|bucket| bucket.drain(..))
            .collect();
    }
    flattened
}

/// radix sorting for `u32` data types
pub(crate) fn radixu32_adapt(data: &[u32], nbbuckets: u32) -> Vec<u32> {
    // empty
    if data.is_empty() { return Vec::new(); }
    // determine le nombre de passes
    let max: u32 = *data.iter().max().unwrap();
    if max == 0 { return data.to_vec(); } // 1 or more zeroes
    //let passes: u32 = (32 - max.leading_zeros()).div_ceil(8); // value max of passes = 10 ((WOM)optimisation of grouping variables into a single variable is possible)
    let bits_per_pass = nbbuckets.ilog2();
    let passes: u32 = (32 + bits_per_pass - 1) / bits_per_pass;
    // create a vec of "boxes" (vecs of vec)
    let mut buckets: Vec<Vec<u32>> = vec![Vec::new();nbbuckets as usize];
    let mut flattened: Vec<u32> = data.to_vec();
    let mask = nbbuckets - 1;
    for p in 0..passes {
        for &d in &flattened { // ascending order
            // let digit: usize = ((d / 10u32.pow(p)) % 10) as usize; // storage of only a digit ((WOM) 0 to 9 possible values (a big memory loss fo the rest of the memory to uptimize later))
            let digit: usize = ((d >> (bits_per_pass * p)) & mask) as usize; // storage of only a digit ((WOM) 0 to 9 possible values (a big memory loss fo the rest of the memory to uptimize later))
            buckets[digit].push(d);
        }
        // merge vec of vec into a vec
        flattened = buckets.iter_mut()
            .flat_map(|bucket| bucket.drain(..))
            .collect();
    }
    flattened
}

// why not doing the “flattening" only at the return of radix sorting function ? (try it out with a benchmark)

/// radix sorting for `u32` data types
/// variant2 with data duplication removal
/// - optimisation 1: remove `data` param duplication (when assigning into flattened for first read)
/// - optimisation 2: remove `flattened` re-creation of the variable (end of the 5th for loop)
/// - perf" penalty 1 : dynamic "passes" computed from selected buckets (do not include max digit of the max value)
/// - for impovements 1: optimisation 3:  remove computation of passes using input buckets number only, use the range of bits required depending on the MIN and MAX value of the serie (much better)
pub(crate) fn radixu32_adapt2(data: &[u32], nbbuckets: u32) -> Vec<u32> {
    if data.is_empty() { return Vec::new(); }
    let max: u32 = *data.iter().max().unwrap();
    if max == 0 { return data.to_vec(); } // 1 or more zeroes
    //let passes: u32 = (32 - max.leading_zeros()).div_ceil(8); // value max of passes = 10 ((WOM)optimisation of grouping variables into a single variable is possible)
    let bits_per_pass = nbbuckets.ilog2();
    let passes: u32 = (32 + bits_per_pass - 1) / bits_per_pass;
    // create a vec of "boxes" (vecs of vec)
    let n: usize = data.len();
    let mut buckets: Vec<Vec<u32>> = vec![Vec::new();nbbuckets as usize];
    let mut flattened: Vec<u32> = Vec::with_capacity(n);
    let mut output: Vec<u32> = Vec::with_capacity(n);
    let mask = nbbuckets - 1;
    for &d in data {
        // let digit: usize = ((d / 10u32.pow(p)) % 10) as usize; // storage of only a digit ((WOM) 0 to 9 possible values (a big memory loss fo the rest of the memory to uptimize later))
        let digit: usize = (d & mask) as usize; // storage of only a digit ((WOM) 0 to 9 possible values (a big memory loss fo the rest of the memory to uptimize later))
        buckets[digit].push(d);
    }
    for bucket in buckets.iter_mut() {
        output.extend(bucket.drain(..));
    }
    std::mem::swap(&mut flattened, &mut output); // swap ref pointed to memory only (very fast)
    for p in 1..passes {
        for &d in &flattened { // ascending order
            // let digit: usize = ((d / 10u32.pow(p)) % 10) as usize; // storage of only a digit ((WOM) 0 to 9 possible values (a big memory loss fo the rest of the memory to uptimize later))
            let digit: usize = ((d >> (bits_per_pass * p)) & mask) as usize; // storage of only a digit ((WOM) 0 to 9 possible values (a big memory loss fo the rest of the memory to uptimize later))
            buckets[digit].push(d);
        }
        output.clear();
        for bucket in buckets.iter_mut() {
            output.extend(bucket.drain(..));
        }
        std::mem::swap(&mut flattened, &mut output);
    }
    flattened
}

// to show nbbuckets and nb passes we will add 2 parameters that will be changed by the function
// to get the correct values (ref value edition)

/// radix sorting for `u32` data types
/// variant2 with data duplication removal
/// - optimisation 1: remove `data` param duplication (when assigning into flattened for first read)
/// - optimisation 2: remove `flattened` re-creation of the variable (end of the 5th for loop)
/// - optimisation 3: remove computation of passes using input buckets number only, use 
/// the range of bits required depending on the MIN and MAX value of the serie (much better)
/// (included too, with the chosen buckets number)
/// - perf" penalty 1 : dynamic "passes" computed from selected buckets (do not include max digit of the max value)
/// - for improvements 1: optimisation 4:  (next) remove flattening conversions
/// - for improvements 2: optimisation 5: (next) use continue in first if in the loop of minmax
/// - for improvements 3: optimisation 6: (next) use else in front of second if, in the loop of minmax 
/// (check difference beetween opti 5 and with differents profiles of data)
/// - for improvements 4: optimisation 7: (next) make algo multithread
/// - for improvements 4: optimisation 7: (next) make algo multi-types (scale first to 
/// all u* numbers (u8, u16, u32, ...) then i8, i16 etc.., f8,...,d8,...)
/// - or improvements 5: optimisation 8: (next) improve vector alloc (for lower input 
/// size try to reduce vec alloc because vec alloc > computation need fo 10 items (in example))
/// - last improvement: last opti: auto adapt buck count (remove input)
pub(crate) fn radixu32_adapt3(data: &[u32], nbbuckets: u32) -> Vec<u32> {
    if data.is_empty() { return Vec::new(); }
    let max: u32 = *data.iter().max().unwrap();
    if max == 0 { return data.to_vec(); } // 1 or more zeroes
    // get minmax in single parkour
    let mut min: u32 = data[0];
    let mut max: u32 = data[0];
    for &d in data {
        if d < min { min = d; }
        else if d > max { max = d; }
    }
    //let passes: u32 = (32 - max.leading_zeros()).div_ceil(8); // value max of passes = 10 ((WOM)optimisation of grouping variables into a single variable is possible)
    let bits_per_pass = nbbuckets.ilog2();
    let passes: u32 = (32 + bits_per_pass - 1) / bits_per_pass;
    // create a vec of "boxes" (vecs of vec)
    let n: usize = data.len();
    let mut buckets: Vec<Vec<u32>> = vec![Vec::new();nbbuckets as usize];
    let mut flattened: Vec<u32> = Vec::with_capacity(n);
    let mut output: Vec<u32> = Vec::with_capacity(n);
    let mask = nbbuckets - 1;
    for &d in data {
        // let digit: usize = ((d / 10u32.pow(p)) % 10) as usize; // storage of only a digit ((WOM) 0 to 9 possible values (a big memory loss fo the rest of the memory to uptimize later))
        let digit: usize = (d & mask) as usize; // storage of only a digit ((WOM) 0 to 9 possible values (a big memory loss fo the rest of the memory to uptimize later))
        buckets[digit].push(d);
    }
    for bucket in buckets.iter_mut() {
        output.extend(bucket.drain(..));
    }
    std::mem::swap(&mut flattened, &mut output);
    for p in 1..passes {
        for &d in &flattened { // ascending order
            // let digit: usize = ((d / 10u32.pow(p)) % 10) as usize; // storage of only a digit ((WOM) 0 to 9 possible values (a big memory loss fo the rest of the memory to uptimize later))
            let digit: usize = ((d >> (bits_per_pass * p)) & mask) as usize; // storage of only a digit ((WOM) 0 to 9 possible values (a big memory loss fo the rest of the memory to uptimize later))
            buckets[digit].push(d);
        }
        output.clear();
        for bucket in buckets.iter_mut() {
            output.extend(bucket.drain(..));
        }
        std::mem::swap(&mut flattened, &mut output);
    }
    flattened
}

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

use criterion::{
    criterion_group,
    criterion_main,
    BenchmarkId,
    Criterion,
};
use std::hint::black_box;

fn benchmarks(c: &mut Criterion){
    // variable + data declaration
    let mut group = c.benchmark_group("radix");
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
    let seed: u32 = 42;

    // benchmark here
    for size in (1..=7).map(|exp|10_usize.pow(exp)){
        for (profile_id, profile) in profiles.iter().enumerate(){
            let data = profile(size, seed);
            group.bench_with_input(
                BenchmarkId::new(
                    format!("radixu32/profile_{profile_id}"),
                    format!("{size}_items")
                ),
                &data,
                |b, data| {
                    b.iter(
                        || black_box(
                            radixu32(black_box(data.as_slice()))
                        )
                    )
                } 
            );
            group.bench_with_input(
                BenchmarkId::new(
                    format!("radixu32_256b/profile_{profile_id}"),
                    format!("{size}_items")
                ),
                &data,
                |b, data| {
                    b.iter(
                        || black_box(
                            radixu32_256b(black_box(data.as_slice()))
                        )
                    )
                } 
            );
            for i in 1..15 {
                let buckets = 2_u32.pow(i);
                group.bench_with_input(
                    BenchmarkId::new(
                        format!("radixu32_adapt/profile_{profile_id}"),
                        format!("{size}_items/{buckets}_buckets")
                    ),
                    &data,
                    |b, data| {
                        b.iter(|| {
                            black_box(radixu32_adapt(data.as_slice(), buckets))
                        })
                    } 
                );
                group.bench_with_input(
                    BenchmarkId::new(
                        format!("radixu32_adapt2/profile_{profile_id}"),
                        format!("{size}_items/{buckets}_buckets")
                    ),
                    &data,
                    |b, data| {
                        b.iter(|| {
                            black_box(radixu32_adapt2(data.as_slice(), buckets))
                        })
                    } 
                );                 
            }
            
        }
    }
    group.finish(); 
}

criterion_group!(benches, benchmarks);
criterion_main!(benches);


