use crate::cj_bitmask_item::BitmaskItem;
use cj_common::cj_binary::bitbuf::*;
use std::ops::{Index, IndexMut};
use std::slice::Iter;

pub struct BitmaskVec<B, T>
where
    B: Bitflag,
{
    inner: Vec<BitmaskItem<B, T>>,
}

impl<'a, B, T> BitmaskVec<B, T>
where
    B: Bitflag + CjMatchesMask<'a, B> + Clone + Default,
{
    pub fn new() -> Self {
        Self {
            inner: Vec::<BitmaskItem<B, T>>::new(),
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    #[inline]
    pub fn push(&mut self, bitmask: B, value: T) {
        self.inner.push(BitmaskItem::new(bitmask, value));
    }

    #[inline]
    pub fn pop(&mut self) -> Option<T> {
        if let Some(item) = self.inner.pop() {
            Some(item.item)
        } else {
            None
        }
    }

    #[inline]
    pub fn pop_with_mask(&mut self) -> Option<BitmaskItem<B, T>> {
        self.inner.pop()
    }

    #[inline]
    pub fn iter(&'a mut self) -> BitmaskVecIter<'a, B, T> {
        BitmaskVecIter::new(self.inner.iter())
    }

    #[inline]
    pub fn iter_with_mask(&'a mut self) -> BitmaskVecIterWithMask<'a, B, T> {
        BitmaskVecIterWithMask::new(self.inner.iter())
    }
}

impl<'a, B, T> Index<usize> for BitmaskVec<B, T>
where
    B: Bitflag + CjMatchesMask<'a, B>,
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.inner[index].item
    }
}

impl<'a, B, T> IndexMut<usize> for BitmaskVec<B, T>
where
    B: Bitflag + CjMatchesMask<'a, B>,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.inner[index].item
    }
}

pub struct BitmaskVecIter<'a, B, T>
where
    B: Bitflag + CjMatchesMask<'a, B>,
{
    inner: Iter<'a, BitmaskItem<B, T>>,
}

impl<'a, B, T> BitmaskVecIter<'a, B, T>
where
    B: Bitflag + CjMatchesMask<'a, B>,
{
    pub fn new(i: Iter<'a, BitmaskItem<B, T>>) -> Self {
        Self { inner: i }
    }

    #[inline]
    fn next_inner(&mut self) -> Option<&'a T> {
        if let Some(item) = self.inner.next() {
            return Some(&item.item);
        }
        None
    }
}

impl<'a, B, T> Iterator for BitmaskVecIter<'a, B, T>
where
    B: Bitflag + CjMatchesMask<'a, B>,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_inner()
    }
}

pub struct BitmaskVecIterWithMask<'a, B, T>
where
    B: Bitflag + CjMatchesMask<'a, B> + Clone + Default,
{
    inner: Iter<'a, BitmaskItem<B, T>>,
}

impl<'a, B, T> BitmaskVecIterWithMask<'a, B, T>
where
    B: Bitflag + CjMatchesMask<'a, B> + Clone + Default,
{
    pub fn new(i: Iter<'a, BitmaskItem<B, T>>) -> Self {
        Self { inner: i }
    }
    #[inline]
    fn next_inner(&mut self) -> Option<&'a BitmaskItem<B, T>> {
        if let Some(item) = self.inner.next() {
            return Some(&item);
        }
        None
    }

    pub fn filter_mask(&mut self, mask: &'a B) -> Option<&'a BitmaskItem<B, T>> {
        while let Some(item) = self.inner.next() {
            if item.matches_mask(mask) {
                return Some(&item);
            }
        }
        None
    }
}

impl<'a, B, T> Iterator for BitmaskVecIterWithMask<'a, B, T>
where
    B: Bitflag + CjMatchesMask<'a, B> + Clone + Default,
{
    type Item = &'a BitmaskItem<B, T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_inner()
    }
}

// pub struct BitmaskVecIterFilter<'a, B, T>
// where
//     B: Bitflag + CjMatchesMask<'a, B> + Default + Clone,
//     Self: Sized,
// {
//     inner: BitmaskVecIterWithMask<'a, B, T>,
//     the_mask: B,
// }
//
// impl<'a, B, T> BitmaskVecIterFilter<'a, B, T>
// where
//     B: Bitflag + CjMatchesMask<'a, B> + Default + Clone,
// {
//     pub fn new(mut i: BitmaskVecIterWithMask<'a, B, T>) -> Self {
//         Self {
//             inner: i,
//             the_mask: B::default(),
//         }
//     }
//
//     pub fn filter_X(
//         mut self,
//         mask: &'a B,
//     ) -> Filter<Iter<'a, BitmaskItem<B, T>>, impl FnMut(&'a &'a BitmaskItem<B, T>) -> bool> {
//         self.the_mask = mask.clone();
//
//         self.inner.inner.filter(|f| f.matches_mask(mask))
//     }
//
//     #[inline]
//     fn next_inner(&mut self) -> Option<&'a BitmaskItem<B, T>> {
//         if let Some(item) = self.inner.next() {
//             return Some(&item);
//         }
//         None
//     }
// }
//
// impl<'a, B, T> Iterator for BitmaskVecIterFilter<'a, B, T>
// where
//     B: Bitflag + CjMatchesMask<'a, B> + Clone + Default,
// {
//     type Item = &'a BitmaskItem<B, T>;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         self.next_inner()
//     }
// }

// pub trait BitmaskVecFilter<'a, B, T, F>
// where
//     B: Bitflag + CjMatchesMask<'a, B> + Clone + Default + Sized + 'a,
//     T: Sized + 'a,
//     F : FnMut(&'a &'a BitmaskItem<B, T>) -> bool,
// {
//     fn filter(
//         &'a mut self,
//         mask: B,
//     ) -> Filter<Iter<'a, BitmaskItem<B, T>>, F>;
//
//
// }
//
// impl <'a, B, T, F>BitmaskVecFilter<'a, B, T, F> for BitmaskVecIterWithMask<'a, B, T>
// where
// B: Bitflag + CjMatchesMask<'a, B> + Clone + Default + Sized + 'a,
// T: Sized + 'a,
// F : FnMut(&'a &'a BitmaskItem<B, T>) -> bool,
// {
//     fn filter(&'a mut self, mask: B) -> Filter<Iter<'a, BitmaskItem<B, T>>, F> {
//         self.filter_x(mask)
//     }
// }

#[cfg(test)]
mod test {
    use crate::cj_bitmask_vec::BitmaskVec;

    #[test]
    fn test_bitmask_vec() {
        let _ = BitmaskVec::<u8, i32>::new();
    }

    #[test]
    fn test_bitmask_vec_push() {
        let mut v = BitmaskVec::<u8, i32>::new();
        v.push(0, 100);
        v.push(1, 400);
        v.push(2, 0);

        assert_eq!(v.len(), 3);
    }

    #[test]
    fn test_bitmask_vec_pop() {
        let mut v = BitmaskVec::<u8, i32>::new();
        v.push(0, 100);
        v.push(1, 400);
        v.push(2, 999);

        let x = v.pop();

        assert_eq!(x, Some(999));
    }

    #[test]
    fn test_bitmask_vec_pop_with_mask() {
        let mut v = BitmaskVec::<u8, i32>::new();
        v.push(0, 100);
        v.push(1, 400);
        v.push(2, 999);

        let x = v.pop_with_mask();
        assert!(x.is_some());
        let x = x.unwrap();
        assert_eq!(x.bitmask, 2);
        assert_eq!(x.item, 999);
    }

    #[test]
    fn test_bitmask_vec_index() {
        let mut v = BitmaskVec::<u8, i32>::new();
        v.push(0, 100);
        v.push(1, 400);
        v.push(2, 999);

        let x = v[1];
        assert_eq!(x, 400);
    }

    #[test]
    fn test_bitmask_vec_index_mut() {
        let mut v = BitmaskVec::<u8, i32>::new();
        v.push(0, 100);
        v.push(1, 400);
        v.push(2, 999);

        v[1] = 800;
        let x = v[1];
        assert_eq!(x, 800);
    }

    #[test]
    fn test_bitmask_vec_iter() {
        let mut v = BitmaskVec::<u8, i32>::new();
        v.push(0b00000000, 100);
        v.push(0b00000010, 101);
        v.push(0b00000010, 102);
        v.push(0b00000110, 103);
        v.push(0b00000001, 104);
        v.push(0b00000001, 105);
        v.push(0b00000000, 106);

        assert_eq!(v.iter().count(), 7);
    }

    #[test]
    fn test_bitmask_vec_iter_with_mask() {
        let mut v = BitmaskVec::<u8, i32>::new();
        v.push(0b00000000, 100);
        v.push(0b00000010, 101);
        v.push(0b00000010, 102);
        v.push(0b00000110, 103);
        v.push(0b00000001, 104);
        v.push(0b00000001, 105);
        v.push(0b00000000, 106);

        assert_eq!(v.iter_with_mask().count(), 7);
    }

    #[test]
    fn test_bitmask_vec_with_mask_match() {
        let mut v = BitmaskVec::<u8, i32>::new();
        v.push(0b00000000, 100);
        v.push(0b00000010, 101);
        v.push(0b00000010, 102);
        v.push(0b00000110, 103);
        v.push(0b00000001, 104);
        v.push(0b00000001, 105);
        v.push(0b00000000, 106);

        let mut count = 0;
        let mut match_count = 0;
        for x in v.iter_with_mask() {
            count += 1;
            if x.matches_mask(&0b00000010) {
                match_count += 1;
            }
        }

        assert_eq!(count, 7,);
        assert_eq!(match_count, 3);
    }

    #[test]
    fn test_bitmask_vec_with_mask_filter() {
        let mut v = BitmaskVec::<u8, i32>::new();
        v.push(0b00000000, 100);
        v.push(0b00000010, 101);
        v.push(0b00000010, 102);
        v.push(0b00000110, 103);
        v.push(0b00000001, 104);
        v.push(0b00000001, 105);
        v.push(0b00000000, 106);

        let mut count = 0;
        let mut z = v.iter_with_mask();
        while let Some(_) = z.filter_mask(&0b00000010) {
            count += 1;
        }

        assert_eq!(count, 3);
    }
}
