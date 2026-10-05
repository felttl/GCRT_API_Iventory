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
UI part is made using Java (with JavaFX) another repository will be create for UI parts in rust.
(because actual solutions doesn't fit project requirements (2026))

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