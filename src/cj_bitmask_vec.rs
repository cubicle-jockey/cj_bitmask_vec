use crate::cj_bitmask_item::BitmaskItem;
use cj_common::cj_binary::bitbuf::*;
use std::ops::{Index, IndexMut};
use std::slice::{Iter, IterMut};

/// BitmaskVec is a vec that pairs bitmasks with T. Bitmasks u8 through u128 are supported.<br>
///
/// Items can be added with or without supplying bitmasks. Bitmask will default to zero if not supplied.
/// ```
/// # use cj_bitmask_vec::{cj_bitmask_vec::*, cj_bitmask_item::*};
/// let mut v = BitmaskVec::<u8, i32>::new();
/// // bitmasks hold whatever meaning the developer gives them.
/// // In this example any u8 is a valid bitmask.
/// //                (bitmask)  (T)      
/// v.push_with_mask(0b00000000, 100);
/// v.push_with_mask(0b00000010, 101);
/// v.push_with_mask(0b00000011, 102);
/// v.push_with_mask(0b00000100, 103);
/// v.push_with_mask(0b00000110, 104);
/// v.push(105);  // <- bitmask will default to zero
///
/// // example of iterating all items where bitmask bit 1 is set
/// let mut count = 0;
/// let mut iter = v.iter_with_mask();
/// while let Some(pair) = iter.filter_mask(&0b00000010) {
///     // only T 101, 102 and 104 in the Vec above have
///     // bitmask bit one set.
///     assert!([101, 102, 104].contains(&pair.item));
///     count += 1;
/// }
/// assert_eq!(count, 3);
/// ```
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

    /// Constructs a new, empty Vec with at least the specified capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: Vec::<BitmaskItem<B, T>>::with_capacity(capacity),
        }
    }

    /// Returns the number of elements the vector can hold without reallocating.
    #[inline]
    pub fn capacity(&self) -> usize {
        self.inner.capacity()
    }

    /// Moves all the elements of other into self, leaving other empty.
    #[inline]
    pub fn append(&mut self, other: &mut Self) {
        self.inner.append(&mut other.inner)
    }

    /// Extracts a slice containing the entire vector.
    #[inline]
    pub fn as_slice(&self) -> &[BitmaskItem<B, T>] {
        self.inner.as_slice()
    }

    /// Extracts a mutable slice containing the entire vector.
    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [BitmaskItem<B, T>] {
        self.inner.as_mut_slice()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Pushes T and a default bitmask of zero.
    #[inline]
    pub fn push(&mut self, value: T) {
        self.inner.push(BitmaskItem::new(B::default(), value));
    }

    /// Pushes T and the supplied bitmask
    /// ```
    /// # use cj_bitmask_vec::{cj_bitmask_vec::*, cj_bitmask_item::*};
    /// let mut v = BitmaskVec::<u8, i32>::new();
    /// // bitmasks hold whatever meaning the developer gives them.
    /// // In this example any u8 is a valid bitmask.
    /// //                (bitmask)  (T)      
    /// v.push_with_mask(0b00000000, 100);
    /// v.push_with_mask(0b00000010, 101);
    /// v.push_with_mask(0b00000011, 102);
    /// ```
    #[inline]
    pub fn push_with_mask(&mut self, bitmask: B, value: T) {
        self.inner.push(BitmaskItem::new(bitmask, value));
    }

    /// Pops T from the Vec without the bitmask.  If both T and bitmask are wanted,
    /// use pop_with_mask() instead.
    #[inline]
    pub fn pop(&mut self) -> Option<T> {
        if let Some(item) = self.inner.pop() {
            Some(item.item)
        } else {
            None
        }
    }

    /// Pops BitmaskItem containing both T and bitmask from Vec.
    /// ```
    /// # use cj_bitmask_vec::{cj_bitmask_vec::*, cj_bitmask_item::*};
    /// let mut v = BitmaskVec::<u8, i32>::new();
    /// v.push_with_mask(0b00000000, 100);
    /// v.push_with_mask(0b00000001, 400);
    /// v.push_with_mask(0b00000010, 999);
    ///
    /// let x = v.pop_with_mask();
    /// assert!(x.is_some());
    /// let x = x.unwrap();
    /// assert_eq!(x.bitmask, 0b00000010);
    /// assert_eq!(x.item, 999);
    /// ```
    #[inline]
    pub fn pop_with_mask(&mut self) -> Option<BitmaskItem<B, T>> {
        self.inner.pop()
    }

    /// Returns a BitmaskVecIter for iterating over T.
    /// * this iter excludes bitmask. Use iter_with_mask() instead if both T and bitmask are wanted.
    /// ```
    /// # use cj_bitmask_vec::{cj_bitmask_vec::*, cj_bitmask_item::*};
    /// let mut v = BitmaskVec::<u8, i32>::new();
    /// v.push_with_mask(0b00000000, 100);
    /// v.push_with_mask(0b00000010, 101);
    /// v.push_with_mask(0b00000010, 102);
    /// v.push_with_mask(0b00000110, 103);
    /// v.push_with_mask(0b00000001, 104);
    /// v.push_with_mask(0b00000001, 105);
    /// v.push_with_mask(0b00000000, 106);
    ///
    /// let mut total = 0;
    /// for x in v.iter() {
    ///     total += x;
    /// }
    /// assert_eq!(total, 721);
    /// ```
    #[inline]
    pub fn iter(&'a mut self) -> BitmaskVecIter<'a, B, T> {
        BitmaskVecIter::new(self.inner.iter())
    }

    /// Returns a BitmaskVecIterWithMask for iterating over T and bitmask.    
    /// ```
    /// # use cj_common::prelude::CjMatchesMask;
    /// # use cj_bitmask_vec::{cj_bitmask_vec::*, cj_bitmask_item::*};
    /// let mut v = BitmaskVec::<u8, i32>::new();
    /// v.push_with_mask(0b00000000, 100);
    /// v.push_with_mask(0b00000010, 101);
    /// v.push_with_mask(0b00000010, 102);
    /// v.push_with_mask(0b00000110, 103);
    /// v.push_with_mask(0b00000001, 104);
    /// v.push_with_mask(0b00000001, 105);
    /// v.push_with_mask(0b00000000, 106);
    ///
    /// let mut total = 0;
    /// for x in v.iter_with_mask() {
    ///     if x.matches_mask(&0b00000010) {
    ///         total += x.item;
    ///     }  
    /// }
    /// assert_eq!(total, 306);
    /// ```
    #[inline]
    pub fn iter_with_mask(&'a mut self) -> BitmaskVecIterWithMask<'a, B, T> {
        BitmaskVecIterWithMask::new(self.inner.iter())
    }

    /// Returns a BitmaskVecIterMut for mutable iteration over T.
    /// * this iter excludes bitmask. Use iter_with_mask_mut() instead if both T and bitmask are wanted.
    /// ```
    /// # use cj_bitmask_vec::{cj_bitmask_vec::*, cj_bitmask_item::*};
    /// let mut v = BitmaskVec::<u8, i32>::new();
    /// v.push_with_mask(0b00000000, 100);
    /// v.push_with_mask(0b00000010, 101);
    /// v.push_with_mask(0b00000010, 102);
    /// v.push_with_mask(0b00000100, 103);
    /// v.push_with_mask(0b00000011, 104);
    /// v.push_with_mask(0b00000001, 105);
    /// v.push_with_mask(0b00000000, 106);
    ///
    /// let mut total = 0;
    /// let x = v.iter_mut();
    /// for z in x {
    ///     // here we modify T
    ///     total += *z;
    ///     *z *= 2;
    /// }
    ///
    /// let mut total_2 = 0;
    /// let x = v.iter();
    /// for z in x {
    ///     total_2 += *z;
    /// }
    ///
    /// assert_eq!(total_2, total * 2)
    /// ```
    #[inline]
    pub fn iter_mut(&'a mut self) -> BitmaskVecIterMut<'a, B, T> {
        BitmaskVecIterMut::new(self.inner.iter_mut())
    }

    /// Returns a BitmaskVecIterWithMaskMut for mutable iteration over T and bitmask.    
    /// ```
    /// # use cj_common::prelude::{Bitflag, CjMatchesMask};
    /// # use cj_bitmask_vec::{cj_bitmask_vec::*, cj_bitmask_item::*};
    /// let mut v = BitmaskVec::<u8, i32>::new();
    /// v.push_with_mask(0b00000000, 100);
    /// v.push_with_mask(0b00000010, 101);
    /// v.push_with_mask(0b00000010, 102);
    /// v.push_with_mask(0b00000100, 103);
    /// v.push_with_mask(0b00000011, 104);
    /// v.push_with_mask(0b00000001, 105);
    /// v.push_with_mask(0b00000000, 106);
    ///
    /// let mut total = 0;
    /// let x = v.iter_with_mask_mut();
    /// for z in x {
    ///     total += z.item;
    ///     // here we modify T
    ///     z.item *= 2;
    ///
    ///     // and here we modify the 8th bit of the bitmask.
    ///     // - note that set_bit() only modifies a single bit,
    ///     //   leaving the rest of bitmask unchanged.
    ///     z.bitmask.set_bit(7, true);
    /// }
    /// // verify the changes from above
    /// let mut total_2 = 0;
    /// let x = v.iter_with_mask();
    /// for z in x {
    ///     total_2 += z.item;
    ///     // test that the 8th bit is now set.
    ///     assert!(z.matches_mask(&0b10000000));
    /// }
    /// // test that T was modified
    /// assert_eq!(total_2, total * 2);
    ///
    /// ```
    #[inline]
    pub fn iter_with_mask_mut(&'a mut self) -> BitmaskVecIterWithMaskMut<'a, B, T> {
        BitmaskVecIterWithMaskMut::new(self.inner.iter_mut())
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

// =================================================================================================
/// Iter that returns T (excludes bitmask)
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

// =================================================================================================
/// Iter that returns BitmaskItem, containing both T and bitmask.
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

// =================================================================================================
/// Iter that returns mutable T (excludes bitmask)
pub struct BitmaskVecIterMut<'a, B, T>
where
    B: Bitflag + CjMatchesMask<'a, B>,
{
    inner: IterMut<'a, BitmaskItem<B, T>>,
}

impl<'a, B, T> BitmaskVecIterMut<'a, B, T>
where
    B: Bitflag + CjMatchesMask<'a, B>,
{
    pub fn new(i: IterMut<'a, BitmaskItem<B, T>>) -> Self {
        Self { inner: i }
    }

    #[inline]
    fn next_inner_mut(&mut self) -> Option<&'a mut T> {
        if let Some(item) = self.inner.next() {
            return Some(&mut item.item);
        }
        None
    }
}

impl<'a, B, T> Iterator for BitmaskVecIterMut<'a, B, T>
where
    B: Bitflag + CjMatchesMask<'a, B>,
{
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_inner_mut()
    }
}

// =================================================================================================
/// Iter that returns mutable BitmaskItem, containing both T and bitmask.
pub struct BitmaskVecIterWithMaskMut<'a, B, T>
where
    B: Bitflag + CjMatchesMask<'a, B> + Clone + Default,
{
    inner: IterMut<'a, BitmaskItem<B, T>>,
}

impl<'a, B, T> BitmaskVecIterWithMaskMut<'a, B, T>
where
    B: Bitflag + CjMatchesMask<'a, B> + Clone + Default,
{
    pub fn new(i: IterMut<'a, BitmaskItem<B, T>>) -> Self {
        Self { inner: i }
    }
    #[inline]
    fn next_inner_mut(&mut self) -> Option<&'a mut BitmaskItem<B, T>> {
        if let Some(item) = self.inner.next() {
            return Some(item);
        }
        None
    }

    pub fn filter_mask(&mut self, mask: &'a B) -> Option<&'a mut BitmaskItem<B, T>> {
        while let Some(item) = self.inner.next() {
            if item.matches_mask(mask) {
                return Some(item);
            }
        }
        None
    }
}

impl<'a, B, T> Iterator for BitmaskVecIterWithMaskMut<'a, B, T>
where
    B: Bitflag + CjMatchesMask<'a, B> + Clone + Default,
{
    type Item = &'a mut BitmaskItem<B, T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_inner_mut()
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
    use cj_common::prelude::Bitflag;

    #[test]
    fn test_bitmask_vec() {
        let _ = BitmaskVec::<u8, i32>::new();
    }

    #[test]
    fn test_bitmask_vec_push() {
        let mut v = BitmaskVec::<u8, i32>::new();
        v.push_with_mask(0, 100);
        v.push_with_mask(1, 400);
        v.push_with_mask(2, 0);

        assert_eq!(v.len(), 3);
    }

    #[test]
    fn test_bitmask_vec_pop() {
        let mut v = BitmaskVec::<u8, i32>::new();
        v.push_with_mask(0, 100);
        v.push_with_mask(1, 400);
        v.push_with_mask(2, 999);

        let x = v.pop();

        assert_eq!(x, Some(999));
    }

    #[test]
    fn test_bitmask_vec_pop_with_mask() {
        let mut v = BitmaskVec::<u8, i32>::new();
        v.push_with_mask(0, 100);
        v.push_with_mask(1, 400);
        v.push_with_mask(2, 999);

        let x = v.pop_with_mask();
        assert!(x.is_some());
        let x = x.unwrap();
        assert_eq!(x.bitmask, 0b00000010);
        assert_eq!(x.item, 999);
    }

    #[test]
    fn test_bitmask_vec_index() {
        let mut v = BitmaskVec::<u8, i32>::new();
        v.push_with_mask(0, 100);
        v.push_with_mask(1, 400);
        v.push_with_mask(2, 999);

        let x = v[1];
        assert_eq!(x, 400);
    }

    #[test]
    fn test_bitmask_vec_index_mut() {
        let mut v = BitmaskVec::<u8, i32>::new();
        v.push_with_mask(0, 100);
        v.push_with_mask(1, 400);
        v.push_with_mask(2, 999);

        v[1] = 800;
        let x = v[1];
        assert_eq!(x, 800);
    }

    #[test]
    fn test_bitmask_vec_iter() {
        let mut v = BitmaskVec::<u8, i32>::new();
        v.push_with_mask(0b00000000, 100);
        v.push_with_mask(0b00000010, 101);
        v.push_with_mask(0b00000010, 102);
        v.push_with_mask(0b00000110, 103);
        v.push_with_mask(0b00000001, 104);
        v.push_with_mask(0b00000001, 105);
        v.push_with_mask(0b00000000, 106);

        assert_eq!(v.iter().count(), 7);
    }

    #[test]
    fn test_bitmask_vec_iter_2() {
        let mut v = BitmaskVec::<u8, i32>::new();
        v.push_with_mask(0b00000000, 100);
        v.push_with_mask(0b00000010, 101);
        v.push_with_mask(0b00000010, 102);
        v.push_with_mask(0b00000110, 103);
        v.push_with_mask(0b00000001, 104);
        v.push_with_mask(0b00000001, 105);
        v.push_with_mask(0b00000000, 106);

        let mut total = 0;
        for x in v.iter() {
            total += x;
        }
        assert_eq!(total, 721);
    }

    #[test]
    fn test_bitmask_vec_iter_with_mask() {
        let mut v = BitmaskVec::<u8, i32>::new();
        v.push_with_mask(0b00000000, 100);
        v.push_with_mask(0b00000010, 101);
        v.push_with_mask(0b00000010, 102);
        v.push_with_mask(0b00000110, 103);
        v.push_with_mask(0b00000001, 104);
        v.push_with_mask(0b00000001, 105);
        v.push_with_mask(0b00000000, 106);

        assert_eq!(v.iter_with_mask().count(), 7);
    }

    #[test]
    fn test_bitmask_vec_with_mask_match() {
        let mut v = BitmaskVec::<u8, i32>::new();
        v.push_with_mask(0b00000000, 100);
        v.push_with_mask(0b00000010, 101);
        v.push_with_mask(0b00000010, 102);
        v.push_with_mask(0b00000110, 103);
        v.push_with_mask(0b00000001, 104);
        v.push_with_mask(0b00000001, 105);
        v.push_with_mask(0b00000000, 106);

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
        v.push_with_mask(0b00000000, 100);
        v.push_with_mask(0b00000010, 101);
        v.push_with_mask(0b00000010, 102);
        v.push_with_mask(0b00000100, 103);
        v.push_with_mask(0b00000011, 104);
        v.push_with_mask(0b00000001, 105);
        v.push_with_mask(0b00000000, 106);

        let mut count = 0;
        let mut z = v.iter_with_mask();
        while let Some(pair) = z.filter_mask(&0b00000010) {
            assert!([101, 102, 104].contains(&pair.item));
            count += 1;
        }
        assert_eq!(count, 3);
    }

    #[test]
    fn test_bitmask_vec_iter_mut() {
        let mut v = BitmaskVec::<u8, i32>::new();
        v.push_with_mask(0b00000000, 100);
        v.push_with_mask(0b00000010, 101);
        v.push_with_mask(0b00000010, 102);
        v.push_with_mask(0b00000100, 103);
        v.push_with_mask(0b00000011, 104);
        v.push_with_mask(0b00000001, 105);
        v.push_with_mask(0b00000000, 106);

        let mut total = 0;
        let x = v.iter_mut();
        for z in x {
            total += *z;
            *z *= 2;
        }

        let mut total_2 = 0;
        let x = v.iter();
        for z in x {
            total_2 += *z;
        }

        assert_eq!(total_2, total * 2)
    }

    #[test]
    fn test_bitmask_vec_iter_masked_mut() {
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
            z.item *= 2;

            z.bitmask.set_bit(7, true);
        }

        let mut total_2 = 0;
        let x = v.iter_with_mask();
        for z in x {
            total_2 += z.item;
            assert!(z.matches_mask(&0b10000000));
        }

        assert_eq!(total_2, total * 2)
    }

    #[test]
    fn test_bitmask_vec_with_capacity() {
        let mut v = BitmaskVec::<u8, i32>::with_capacity(10);

        assert_eq!(10, v.capacity())
    }

    #[test]
    fn test_bitmask_vec_append() {
        let mut v = BitmaskVec::<u8, i32>::new();
        v.push_with_mask(0b00000000, 100);
        v.push_with_mask(0b00000010, 101);
        v.push_with_mask(0b00000010, 102);
        v.push_with_mask(0b00000100, 103);
        v.push_with_mask(0b00000011, 104);
        v.push_with_mask(0b00000001, 105);
        v.push_with_mask(0b00000000, 106);

        let mut v2 = BitmaskVec::<u8, i32>::new();
        v2.push_with_mask(0b00000000, 100);
        v2.push_with_mask(0b00000010, 101);
        v2.push_with_mask(0b00000010, 102);
        v2.push_with_mask(0b00000100, 103);
        v2.push_with_mask(0b00000011, 104);
        v2.push_with_mask(0b00000001, 105);
        v2.push_with_mask(0b00000000, 106);

        v.append(&mut v2);

        assert_eq!(v.len(), 14);
        assert_eq!(v2.len(), 0);
    }

    #[test]
    fn test_bitmask_vec_as_slice() {
        let mut v = BitmaskVec::<u8, i32>::new();
        v.push_with_mask(0b00000000, 100);
        v.push_with_mask(0b00000010, 101);
        v.push_with_mask(0b00000010, 102);
        v.push_with_mask(0b00000100, 103);
        v.push_with_mask(0b00000011, 104);
        v.push_with_mask(0b00000001, 105);
        v.push_with_mask(0b00000000, 106);

        assert_eq!(v.as_slice().len(), 7);
    }

    #[test]
    fn test_bitmask_vec_as_mut_slice() {
        let mut v = BitmaskVec::<u8, i32>::new();
        v.push_with_mask(0b00000000, 100);
        v.push_with_mask(0b00000010, 101);
        v.push_with_mask(0b00000010, 102);
        v.push_with_mask(0b00000100, 103);
        v.push_with_mask(0b00000011, 104);
        v.push_with_mask(0b00000001, 105);
        v.push_with_mask(0b00000000, 106);
        {
            let s = v.as_mut_slice();
            s[1].item = 500;
            s[1].bitmask.set_bit(7, true);
        }
        assert_eq!(v[1], 500);
        // hmmm.  TO.DO. maybe i should change index/indexmut to return BitmaskItem instead of just T...
        assert_eq!(v.iter_with_mask().nth(1).unwrap().bitmask, 0b10000010);
    }
}
