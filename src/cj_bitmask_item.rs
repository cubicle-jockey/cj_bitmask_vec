use cj_common::cj_binary::bitbuf::*;

/// BitmaskItem pairs T with a bitmask
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

    /// Returns true if all set flags in mask are matched in bitmask<br>
    ///   <i>(bitmask & mask) == mask</i>
    #[inline]
    pub fn matches_mask(&self, mask: &'a B) -> bool {
        self.bitmask.matches_mask(mask)
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
