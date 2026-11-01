# GCRT_API_Iventory
###### Game Core Reusable Tool API Inventory
a simple intuitive Inventory <u>back-end</u> with a lof of features (for games primarily, can be used for other purposes like stock management)

> [!note]
> - cross platform
> - high performance (closer to math models)
> - sourced (scientifc papers for optimization)
> - documented (with code example)

UI directory is intended to provide a UI to test the inventory and is not a part of this project.
Therefore no documentation is provided for it.
UI part is made using TypeScript (with Tauri).

> [!important]
> test for rust code is include in the source code directly according to Rust rules (but i don't agree with this choice i prefer a dedicated repertory for tests to separate the real used code to test code...)

### Execution and Running

```bash
# compile & run
cargo run
# compile
cargo build
# 
```
for single files use : `rustc main.rs` and then run prog using `./main`.


### Benchmarks
<!-- will be splitted later into a specific file with an URL placed here -->
RADIX sorting algorithm (without Inventory)

made of :

- 8 profiles (data with high spread, patterns, pseudo-random, pre-sorted, reversed sort, patterns, duplicates, focused)
- 4 algorithms (for now) for radix impl (2 before loop and 2 after)
- 7 sizes for data (1e1, 1e2, 1e3, ..., 1e7)
- 14 buckets size (from 2e1, 2e2, ..., 2e14)
- 100 samples (by default criterion, not changed)

total benchmarks :
= 100\*(8\*7\*(2+14\*2)) 
= 100\*1680 
= 168 000 
(it means: at least function call counts by Criterion)

next things to test: 
- add: bitscale depending on the range of the value data spread in the `adapt` algorithm versions(if the MIN and MAX value does not cover the full range of a value type, example : u32::MAX is 4_294_967_296 but max value of a serie can be lower than this)
- add different datatypes (now include only `u32`)
- add radix impl from external libs for comparison
- add driftsort impl + external driftsort for comparison
- current mode : monothread, add also : concurrency + parallelism
- add inventory benchmark using all this work (+ last elements noticed in previous points)

to run this benchmark only (outside `src/` in the project directly)
```bash
cargo bench --bench radix_bench
```

to see all reports see : `target/criterion/`

>[!Warning] 
> the computer or device
> used for benchmarks where using other
> apps during benchmarks so noise is present too
> to remove nois if you run benchmarks on your own
> device try to plug on and switch off all other apps
> to reduce noise. some benchkarms have been done when
> the divice was on battery and not pluged on so differences
> can be seen in results due to this too.
> No series have 100% been done when pluged on yet...

### Project Contribution

CLOSED (fork if you want to contribute)

### project rules

#### software constraints

- NRE (Not Ripple Effect)
- ECS oriented Data and (sub) Systems
- math opti bitset + object representation
- Hot Reload (put update when the software is currently running)

#### Cases

- camelCase for types 
- snake_case for file names

<!-- very small programm because very high complexity/difficulty to make -->