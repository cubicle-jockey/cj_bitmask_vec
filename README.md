# cj_bitmask_vec
BitmaskVec is a vec that pairs bitmasks with T. Bitmasks u8 through u128 are supported.<br>

Items can be added with or without supplying bitmasks. Bitmask will default to zero if not supplied.
```rust
fn main() {
    use cj_bitmask_vec::prelude::*;
    
    let mut v = BitmaskVec::<u8, i32>::new();
    // bitmasks hold whatever meaning the developer gives them.
    // In this example any u8 is a valid bitmask.
    //                (bitmask)  (T)      
    v.push_with_mask(0b00000000, 100);
    v.push_with_mask(0b00000010, 101);
    v.push_with_mask(0b00000011, 102);
    v.push_with_mask(0b00000100, 103);
    v.push_with_mask(0b00000110, 104);
    v.push(105);  // <- bitmask will default to zero
    
    // example of iterating all items where bitmask bit 1 is set
    let mut count = 0;
    let mut iter = v.iter_with_mask();
    while let Some(pair) = iter.filter_mask(&0b00000010) {
        // only T 101, 102 and 104 in the Vec above have
        // bitmask bit one set.
        assert!([101, 102, 104].contains(&pair.item));
        count += 1;
    }
    assert_eq!(count, 3);
}
```
