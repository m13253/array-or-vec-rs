/*
    Copyright © 2021 StarBrilliant <coder@poorlab.com>
    This work is free. You can redistribute it and/or modify it under the
    terms of the Do What The Fuck You Want To Public License, Version 2,
    as published by Sam Hocevar. See the COPYING file for more details.
*/

use std::borrow::{Borrow, BorrowMut};
use std::ops::{Index, Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};
use std::slice::SliceIndex;

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
{
    type Item;
    const LEN: Option<usize>;

    fn as_slice(&self) -> &[Self::Item];

    fn as_mut_slice(&mut self) -> &mut [Self::Item];

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

    fn as_slice(&self) -> &[Self::Item] {
        self
    }

    fn as_mut_slice(&mut self) -> &mut [Self::Item] {
        self
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

    fn as_slice(&self) -> &[Self::Item] {
        <Vec<T>>::as_slice(self)
    }

    fn as_mut_slice(&mut self) -> &mut [Self::Item] {
        <Vec<T>>::as_mut_slice(self)
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
