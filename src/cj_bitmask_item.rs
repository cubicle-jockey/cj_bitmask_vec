use cj_common::cj_binary::bitbuf::*;
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
    pub fn new(bitmask: B, item: T) -> Self {
        Self { bitmask, item }
    }

    pub fn matches_mask(&self, mask: &'a B) -> bool {
        self.bitmask.matches_mask(mask)
    }
}

pub trait BitmaskItemX<'a, B> {
    fn match_mask(&self, mask: &'a B) -> bool;
}

impl<'a, B, T> BitmaskItemX<'a, B> for BitmaskItem<B, T>
where
    B: Bitflag + CjMatchesMask<'a, B>,
{
    fn match_mask(&self, mask: &'a B) -> bool {
        self.matches_mask(mask)
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
