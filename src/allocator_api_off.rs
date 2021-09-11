use core::borrow::{Borrow, BorrowMut};
use core::ops::{
    Index, IndexMut, Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive,
};
use core::slice::{Iter, IterMut, SliceIndex};

pub trait ArrayOrVec<T>:
    AsMut<[T]>
    + AsRef<[T]>
    + Borrow<[T]>
    + BorrowMut<[T]>
    + Into<Box<[T]>>
    + Into<Vec<T>>
    + Index<usize>
    + Index<Range<usize>>
    + Index<RangeFrom<usize>>
    + Index<RangeFull>
    + Index<RangeInclusive<usize>>
    + Index<RangeTo<usize>>
    + Index<RangeToInclusive<usize>>
    + IndexMut<usize>
    + IndexMut<Range<usize>>
    + IndexMut<RangeFrom<usize>>
    + IndexMut<RangeFull>
    + IndexMut<RangeInclusive<usize>>
    + IndexMut<RangeTo<usize>>
    + IndexMut<RangeToInclusive<usize>>
{
    type Item;
    const LEN: Option<usize>;

    fn as_slice(&self) -> &[T];

    fn as_mut_slice(&mut self) -> &mut [T];

    fn iter(&self) -> Iter<'_, T>;

    fn iter_mut(&mut self) -> IterMut<'_, T>;

    fn get<I>(&self, index: I) -> Option<&<I as SliceIndex<[T]>>::Output>
    where
        I: SliceIndex<[T]>;

    fn get_mut<I>(&mut self, index: I) -> Option<&mut <I as SliceIndex<[T]>>::Output>
    where
        I: SliceIndex<[T]>;

    unsafe fn get_unchecked<I>(&self, index: I) -> &<I as SliceIndex<[T]>>::Output
    where
        I: SliceIndex<[T]>;

    unsafe fn get_unchecked_mut<I>(&mut self, index: I) -> &mut <I as SliceIndex<[T]>>::Output
    where
        I: SliceIndex<[T]>;

    fn len(&self) -> usize;
}

impl<T, const N: usize> ArrayOrVec<T> for [T; N] {
    type Item = T;
    const LEN: Option<usize> = Some(N);

    fn as_slice(&self) -> &[T] {
        self
    }

    fn as_mut_slice(&mut self) -> &mut [T] {
        self
    }

    fn iter(&self) -> Iter<'_, T> {
        <[T]>::iter(self)
    }

    fn iter_mut(&mut self) -> IterMut<'_, T> {
        <[T]>::iter_mut(self)
    }

    fn get<I>(&self, index: I) -> Option<&<I as SliceIndex<[T]>>::Output>
    where
        I: SliceIndex<[T]>,
    {
        <[T]>::get(self, index)
    }

    fn get_mut<I>(&mut self, index: I) -> Option<&mut <I as SliceIndex<[T]>>::Output>
    where
        I: SliceIndex<[T]>,
    {
        <[T]>::get_mut(self, index)
    }

    unsafe fn get_unchecked<I>(&self, index: I) -> &<I as SliceIndex<[T]>>::Output
    where
        I: SliceIndex<[T]>,
    {
        <[T]>::get_unchecked(self, index)
    }

    unsafe fn get_unchecked_mut<I>(&mut self, index: I) -> &mut <I as SliceIndex<[T]>>::Output
    where
        I: SliceIndex<[T]>,
    {
        <[T]>::get_unchecked_mut(self, index)
    }

    fn len(&self) -> usize {
        N
    }
}

impl<T> ArrayOrVec<T> for Vec<T> {
    type Item = T;
    const LEN: Option<usize> = None;

    fn as_slice(&self) -> &[T] {
        <Vec<T>>::as_slice(self)
    }

    fn as_mut_slice(&mut self) -> &mut [T] {
        <Vec<T>>::as_mut_slice(self)
    }

    fn iter(&self) -> Iter<'_, T> {
        <[T]>::iter(self)
    }

    fn iter_mut(&mut self) -> IterMut<'_, T> {
        <[T]>::iter_mut(self)
    }

    fn get<I>(&self, index: I) -> Option<&<I as SliceIndex<[T]>>::Output>
    where
        I: SliceIndex<[T]>,
    {
        <[T]>::get(self, index)
    }

    fn get_mut<I>(&mut self, index: I) -> Option<&mut <I as SliceIndex<[T]>>::Output>
    where
        I: SliceIndex<[T]>,
    {
        <[T]>::get_mut(self, index)
    }

    unsafe fn get_unchecked<I>(&self, index: I) -> &<I as SliceIndex<[T]>>::Output
    where
        I: SliceIndex<[T]>,
    {
        <[T]>::get_unchecked(self, index)
    }

    unsafe fn get_unchecked_mut<I>(&mut self, index: I) -> &mut <I as SliceIndex<[T]>>::Output
    where
        I: SliceIndex<[T]>,
    {
        <[T]>::get_unchecked_mut(self, index)
    }

    fn len(&self) -> usize {
        <Vec<T>>::len(self)
    }
}
