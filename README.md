# cj_bitmask_vec

[![Rust](https://github.com/cubicle-jockey/cj_bitmask_vec/actions/workflows/rust.yml/badge.svg)](https://github.com/cubicle-jockey/cj_bitmask_vec/actions/workflows/rust.yml)
[![Dependency Review](https://github.com/cubicle-jockey/cj_bitmask_vec/actions/workflows/dependency-review.yml/badge.svg)](https://github.com/cubicle-jockey/cj_bitmask_vec/actions/workflows/dependency-review.yml)
[![Crate](https://img.shields.io/crates/v/cj_bitmask_vec.svg)](https://crates.io/crates/cj_bitmask_vec)
[![API](https://docs.rs/cj_bitmask_vec/badge.svg)](https://docs.rs/cj_bitmask_vec)

BitmaskVec is a vector that pairs bitmasks with T. Bitmasks u8 through u128 are supported.<br>

Items can be added with or without supplying bitmasks. The bitmask will default to zero if not supplied.

Filtering iterator using bitmasks:

```rust
// filtering by bitmask
fn main() {
    use cj_bitmask_vec::prelude::*;

    let mut v = BitmaskVec::<u8, i32>::new();
    // Bitmasks hold whatever meaning the developer gives them.
    // In this example any u8 is a valid bitmask.
    //                (bitmask)  (T)      
    v.push_with_mask(0b00000000, 100);
    v.push_with_mask(0b00000010, 101);
    v.push_with_mask(0b00000011, 102);
    v.push_with_mask(0b00000100, 103);
    v.push_with_mask(0b00000110, 104);
    v.push(105);  // <- the bitmask will default to zero
    // or an easier way to add items   
    v += (0b00000000, 106);
    v += (0b00010000, 107);
    v += (0b00100000, 108);
    v += (0b00000100, 109);
    v += (0b10000001, 110);
    v += (0b00000001, 111);
    v += (0b00000000, 112);
    v += 113; // <- bitmask will default to zero

    assert_eq!(v[6], 106);

    // here we're going to iterate over all items that have bitmask bit 1 set
    let mut count = 0;
    let mut iter = v.iter_with_mask();
    //                                (mask with bit 1 set)
    //                                               V
    while let Some(pair) = iter.filter_mask(&0b00000010) {
        // only items 101, 102, and 104 in the Vec above have
        // bitmask bit 1 set.
        assert!([101, 102, 104].contains(&pair.item));
        count += 1;
    }
    assert_eq!(count, 3);
}
```

Iterating over T:

```rust
fn main() {
    use cj_bitmask_vec::prelude::*;

    let mut v = BitmaskVec::<u8, i32>::new();
    v.push_with_mask(0b00000000, 100);
    v.push_with_mask(0b00000010, 101);
    v.push_with_mask(0b00000010, 102);
    v.push_with_mask(0b00000110, 103);
    v.push_with_mask(0b00000001, 104);
    v.push_with_mask(0b00000001, 105);
    v.push_with_mask(0b00000000, 106);

    let mut total = 0;
    // iter excludes the bitmask
    for x in v.iter() {
        total += x;
    }
    assert_eq!(total, 721);
}
```

Iterating over T and bitmask:

```rust
fn main() {
    use cj_bitmask_vec::prelude::*;
    use cj_common::prelude::CjMatchesMask;

    let mut v = BitmaskVec::<u8, i32>::new();
    v.push_with_mask(0b00000000, 100);
    v.push_with_mask(0b00000010, 101);
    v.push_with_mask(0b00000010, 102);
    v.push_with_mask(0b00000110, 103);
    v.push_with_mask(0b00000001, 104);
    v.push_with_mask(0b00000001, 105);
    v.push_with_mask(0b00000000, 106);

    let mut total = 0;
    for x in v.iter_with_mask() {
        if x.matches_mask(&0b00000010) {
            total += x.item;
        }
    }
    assert_eq!(total, 306);
}
```

Mutably iterating over T:

```rust
fn main() {
    use cj_bitmask_vec::prelude::*;

    let mut v = BitmaskVec::<u8, i32>::new();
    v.push_with_mask(0b00000000, 100);
    v.push_with_mask(0b00000010, 101);
    v.push_with_mask(0b00000010, 102);
    v.push_with_mask(0b00000100, 103);
    v.push_with_mask(0b00000011, 104);
    v.push_with_mask(0b00000001, 105);
    v.push_with_mask(0b00000000, 106);

    let mut total = 0;
    // iter_mut excludes the bitmask
    let x = v.iter_mut();
    for z in x {
        // here we modify T
        total += *z;
        *z *= 2;
    }

    // verify the changes from above
    let mut total_2 = 0;
    let x = v.iter();
    for z in x {
        total_2 += *z;
    }

    assert_eq!(total_2, total * 2);
}
```

Mutably iterating over T and bitmask:

```rust
fn main() {
    use cj_bitmask_vec::prelude::*;
    use cj_common::prelude::{Bitflag, CjMatchesMask};

    let mut v = BitmaskVec::<u8, i32>::new();
    v.push_with_mask(0b00000000, 100);
    v.push_with_mask(0b00000010, 101);
    v.push_with_mask(0b00000010, 102);
    v.push_with_mask(0b00000100, 103);
    v.push_with_mask(0b00000011, 104);
    v.push_with_mask(0b00000001, 105);
    v.push_with_mask(0b00000000, 106);

    let mut total = 0;
    let x = v.iter_with_mask_mut();
    for z in x {
        total += z.item;
        // here we modify T
        z.item *= 2;

        // here we modify the 8th bit of the bitmask.
        // - Note that set_bit() only modifies a single bit,
        //   leaving the rest of the bitmask unchanged.
        z.bitmask.set_bit(7, true);
    }

    // verify the changes from above
    let mut total_2 = 0;
    let x = v.iter_with_mask();
    for z in x {
        total_2 += z.item;
        // test that the 8th bit is now set.
        assert!(z.matches_mask(&0b10000000));
    }
    // test that T was modified
    assert_eq!(total_2, total * 2);
}
```

## Benchmarks

This crate includes comprehensive benchmarks to measure the performance of various BitmaskVec operations. The benchmarks cover:

- **Basic Operations**: `new()`, `with_capacity()`, `push()`, `push_with_mask()`
- **Indexing Operations**: Index access, `pop()`, `pop_with_mask()`
- **Iteration Operations**: `iter()`, `iter_with_mask()`, `iter_mut()`, `iter_with_mask_mut()`
- **Filtering Operations**: `filter_mask()`, mask matching during iteration
- **Collection Operations**: `append()`, `clear()`, `resize()`, `resize_with_mask()`
- **Different Bitmask Types**: Performance comparison across u8, u16, u32, and u64 bitmasks

### Running Benchmarks

To run the benchmarks:

```bash
cargo bench
```

This will run all benchmarks and generate detailed performance reports. The benchmarks test with different data sizes (100, 1000, and 10000 elements) to show how performance scales.

### Benchmark Results

The benchmarks will generate HTML reports (if you have gnuplot installed) in the `target/criterion/` directory, providing detailed performance analysis including:

- Timing measurements with statistical analysis
- Performance comparisons across different input sizes
- Regression detection across benchmark runs
- Detailed plots and charts (with gnuplot)
