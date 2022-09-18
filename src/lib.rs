pub mod cj_bitmask_item;
pub mod cj_bitmask_vec;

pub mod prelude {
    pub use crate::cj_bitmask_item::*;
    pub use crate::cj_bitmask_vec::*;
}

#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
