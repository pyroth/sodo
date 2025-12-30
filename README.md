# sodo

Fast Sudoku solver and generator in Rust.

## Installation

```bash
cargo install sodo
```

## Usage

```bash
# Generate a puzzle
sodo g -d hard

# Solve a puzzle
sodo s <puzzle>

# Get a hint
sodo h <puzzle>

# Validate
sodo v <puzzle>
```

## Library

```rust
use sodo::{Sudoku, Solver};

let puzzle = Sudoku::from_string(
    "..76..23.1.29.4.5.695..34...1...5.6.....6.......3...4...12..38.45.1.87.6.28..61..",
    9
).unwrap();

let mut solver = Solver::new();
let solution = solver.solve(puzzle).unwrap();
println!("{}", solution);
```

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <https://opensource.org/licenses/MIT>)

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in sodo by you, as defined in the Apache-2.0 license, shall be dually licensed as above, without any additional terms or conditions.
