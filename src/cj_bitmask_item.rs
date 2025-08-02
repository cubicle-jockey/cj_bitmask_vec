use cj_common::cj_binary::bitbuf::*;

/// BitmaskItem pairs T with a bitmask
///
/// This struct stores an item of type `T` along with an associated bitmask of type `B`.
/// The bitmask can be used for filtering, categorization, or any other bit-based operations.
///
/// # Examples
///
/// ```
/// # use cj_bitmask_vec::prelude::*;
/// let item = BitmaskItem::new(0b00000101u8, "hello");
/// assert_eq!(item.bitmask, 0b00000101u8);
/// assert_eq!(item.item, "hello");
///
/// // Access fields directly
/// let mut item = BitmaskItem { bitmask: 0b00000001u8, item: 42i32 };
/// item.item = 100;
/// assert_eq!(item.item, 100);
/// ```
#[derive(Debug, Clone)]
pub struct BitmaskItem<B, T>
where
    B: Bitflag,
{
    pub bitmask: B,
    pub item: T,
}

impl<'a, B, T> BitmaskItem<B, T>
where
    B: Bitflag + CjMatchesMask<'a, B>,
{
    /// Creates a new `BitmaskItem<B, T>` with the specified bitmask and item.
    ///
    /// # Arguments
    ///
    /// * `bitmask` - The bitmask to associate with the item
    /// * `item` - The item to store
    ///
    /// # Examples
    ///
    /// ```
    /// # use cj_bitmask_vec::prelude::*;
    /// let item = BitmaskItem::new(0b00000101u8, 42i32);
    /// assert_eq!(item.bitmask, 0b00000101u8);
    /// assert_eq!(item.item, 42i32);
    /// ```
    #[inline]
    pub fn new(bitmask: B, item: T) -> Self {
        Self { bitmask, item }
    }

    /// Returns true if all set flags in the mask are matched in the bitmask<br>
    ///   <i>(bitmask & mask) == mask</i>
    ///
    /// # Examples
    ///
    /// ```
    /// # use cj_bitmask_vec::prelude::*;
    /// let item = BitmaskItem::new(0b00000111u8, 42i32);
    ///
    /// // Check if bit 0 is set
    /// assert!(item.matches_mask(&0b00000001u8));
    ///
    /// // Check if bits 0 and 1 are both set
    /// assert!(item.matches_mask(&0b00000011u8));
    ///
    /// // Check if bit 3 is set (it's not)
    /// assert!(!item.matches_mask(&0b00001000u8));
    /// ```
    #[inline]
    pub fn matches_mask(&self, mask: &'a B) -> bool {
        self.bitmask.matches_mask(mask)
    }

    /// Sets the bitmask to a new value.
    /// # Arguments
    /// * `bitmask` - The new bitmask to set
    #[inline]
    pub fn set_mask(&mut self, bitmask: B) {
        self.bitmask = bitmask;
    }
}

#[cfg(test)]
mod test {
    use crate::cj_bitmask_item::BitmaskItem;
    use cj_common::prelude::CjMatchesMask;

    #[test]
    fn test_bitmask_item() {
        let x = BitmaskItem {
            bitmask: 2u8,
            item: 1000,
        };

        assert!(x.bitmask.matches_mask(&0b00000010u8));
    }

    #[test]
    fn test_bitmask_item_new() {
        let x = BitmaskItem::<u8, i32>::new(2u8, 1000);

        assert!(x.bitmask.matches_mask(&0b00000010u8));
    }

    #[test]
    fn test_bitmask_item_new_infer() {
        let x = BitmaskItem::new(2u8, 1000);

        assert!(x.bitmask.matches_mask(&0b00000010u8));
    }

    #[test]
    fn test_bitmask_item_matches_mask() {
        let x = BitmaskItem::new(2u8, 1000);

        assert!(x.matches_mask(&0b00000010u8));
    }
}
