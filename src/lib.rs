//! BitmaskVec is a vec that pairs bitmasks with T. Bitmasks u8 through u128 are supported.
//!
//! Items can be added with or without supplying bitmasks. Bitmask will default to zero if not supplied.
//!
//! Filtering iterator using bitmasks
//! ```
//!     use cj_bitmask_vec::prelude::*;
//!    
//!     let mut v = BitmaskVec::<u8, i32>::new();
//!     // bitmasks hold whatever meaning the developer gives them.
//!     // In this example any u8 is a valid bitmask.
//!     //                (bitmask)  (T)      
//!     v.push_with_mask(0b00000000, 100);
//!     v.push_with_mask(0b00000010, 101);
//!     v.push_with_mask(0b00000011, 102);
//!     v.push_with_mask(0b00000100, 103);
//!     v.push_with_mask(0b00000110, 104);
//!     v.push(105);  // <- bitmask will default to zero
//!     
//!     // here we're going to iterate all items that have bitmask bit 1 set
//!     let mut count = 0;
//!     let mut iter = v.iter_with_mask();
//!     //                                (mask with bit 1 set)
//!     //                                               V
//!     while let Some(pair) = iter.filter_mask(&0b00000010) {
//!         // only T 101, 102 and 104 in the Vec above have
//!         // bitmask bit one set.
//!         assert!([101, 102, 104].contains(&pair.item));
//!         count += 1;
//!     }
//!     assert_eq!(count, 3);
//! ```
//! Iterating over T
//! ```
//!     use cj_bitmask_vec::prelude::*;
//!     
//!     let mut v = BitmaskVec::<u8, i32>::new();
//!     v.push_with_mask(0b00000000, 100);
//!     v.push_with_mask(0b00000010, 101);
//!     v.push_with_mask(0b00000010, 102);
//!     v.push_with_mask(0b00000110, 103);
//!     v.push_with_mask(0b00000001, 104);
//!     v.push_with_mask(0b00000001, 105);
//!     v.push_with_mask(0b00000000, 106);
//!
//!     let mut total = 0;
//!     // iter excludes the bitmask
//!     for x in v.iter() {
//!         total += x;
//!     }
//!     assert_eq!(total, 721);
//! ```
//! Iterating over T and bitmask.
//! ```
//!     use cj_bitmask_vec::prelude::*;
//!     use cj_common::prelude::CjMatchesMask;
//!     
//!     let mut v = BitmaskVec::<u8, i32>::new();
//!     v.push_with_mask(0b00000000, 100);
//!     v.push_with_mask(0b00000010, 101);
//!     v.push_with_mask(0b00000010, 102);
//!     v.push_with_mask(0b00000110, 103);
//!     v.push_with_mask(0b00000001, 104);
//!     v.push_with_mask(0b00000001, 105);
//!     v.push_with_mask(0b00000000, 106);
//!
//!     let mut total = 0;
//!     for x in v.iter_with_mask() {
//!         if x.matches_mask(&0b00000010) {
//!             total += x.item;
//!         }
//!     }
//!     assert_eq!(total, 306);
//! ```
//! Mutably iterating over T
//! ```
//!     use cj_bitmask_vec::prelude::*;
//!
//!     let mut v = BitmaskVec::<u8, i32>::new();
//!     v.push_with_mask(0b00000000, 100);
//!     v.push_with_mask(0b00000010, 101);
//!     v.push_with_mask(0b00000010, 102);
//!     v.push_with_mask(0b00000100, 103);
//!     v.push_with_mask(0b00000011, 104);
//!     v.push_with_mask(0b00000001, 105);
//!     v.push_with_mask(0b00000000, 106);
//!
//!     let mut total = 0;
//!     // iter_mut exludes the bitmask
//!     let x = v.iter_mut();
//!     for z in x {
//!         // here we modify T
//!         total += *z;
//!         *z *= 2;
//!     }
//!
//!     // verify the changes from above
//!     let mut total_2 = 0;
//!     let x = v.iter();
//!     for z in x {
//!         total_2 += *z;
//!     }
//!
//!     assert_eq!(total_2, total * 2);
//! ```
//! Mutably iterating over T and bitmask
//! ```
//!     use cj_bitmask_vec::prelude::*;
//!     use cj_common::prelude::{Bitflag, CjMatchesMask};
//!
//!     let mut v = BitmaskVec::<u8, i32>::new();
//!     v.push_with_mask(0b00000000, 100);
//!     v.push_with_mask(0b00000010, 101);
//!     v.push_with_mask(0b00000010, 102);
//!     v.push_with_mask(0b00000100, 103);
//!     v.push_with_mask(0b00000011, 104);
//!     v.push_with_mask(0b00000001, 105);
//!     v.push_with_mask(0b00000000, 106);
//!
//!     let mut total = 0;
//!     let x = v.iter_with_mask_mut();
//!     for z in x {
//!         total += z.item;
//!         // here we modify T
//!         z.item *= 2;
//!
//!         // here we modify the 8th bit of the bitmask.
//!         // - note that set_bit() only modifies a single bit,
//!         //   leaving the rest of bitmask unchanged.
//!         z.bitmask.set_bit(7, true);
//!     }
//!     
//!     // verify the changes from above
//!     let mut total_2 = 0;
//!     let x = v.iter_with_mask();
//!     for z in x {
//!         total_2 += z.item;
//!         // test that the 8th bit is now set.
//!         assert!(z.matches_mask(&0b10000000));
//!     }
//!     // test that T was modified
//!     assert_eq!(total_2, total * 2);
//! ```
pub mod cj_bitmask_item;
pub mod cj_bitmask_vec;

pub mod prelude {
    pub use crate::cj_bitmask_item::*;
    pub use crate::cj_bitmask_vec::*;
}

#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
