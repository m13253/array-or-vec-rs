/*
    Copyright © 2021 StarBrilliant <coder@poorlab.com>
    This work is free. You can redistribute it and/or modify it under the
    terms of the Do What The Fuck You Want To Public License, Version 2,
    as published by Sam Hocevar. See the COPYING file for more details.
*/

use core::alloc::Allocator;
use core::borrow::{Borrow, BorrowMut};
use core::ops::{
    Index, IndexMut, Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive,
};
use core::slice::{Iter, IterMut, SliceIndex};
use std::alloc::Global;

/// ArrayOrVec is a trait for generic functions that need either a compile-time array or a dynamic vector
pub trait ArrayOrVec<T, A = Global>:
    AsMut<[T]>
    + AsRef<[T]>
    + Borrow<[T]>
    + BorrowMut<[T]>
    + Into<Box<[T], A>>
    + Into<Vec<T, A>>
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
where
    A: Allocator,
{
    /// Item allows you to conveniently access the type `T`
    type Item;

    /// LEN can be either `Some(N)` or `None`, depending on whether the array is statically sized or not
    const LEN: Option<usize>;

    /// as_slice borrows it as a slice, so you can access methods not listed here
    fn as_slice(&self) -> &[T];

    /// as_mut_slice borrows it as a mutable slice, so you can access methods not listed here
    fn as_mut_slice(&mut self) -> &mut [T];

    /// iter creates an iterator
    fn iter(&self) -> Iter<'_, T>;

    /// iter_mut creates a mutable iterator
    fn iter_mut(&mut self) -> IterMut<'_, T>;

    /// get returns the item at the given index. If the index is out of bounds, it returns `None`.
    fn get<I>(&self, index: I) -> Option<&<I as SliceIndex<[T]>>::Output>
    where
        I: SliceIndex<[T]>;

    /// get_mut returns the item at the given index. If the index is out of bounds, it returns `None`.
    fn get_mut<I>(&mut self, index: I) -> Option<&mut <I as SliceIndex<[T]>>::Output>
    where
        I: SliceIndex<[T]>;

    /// get_unchecked returns the item at the given index without bound checks.
    unsafe fn get_unchecked<I>(&self, index: I) -> &<I as SliceIndex<[T]>>::Output
    where
        I: SliceIndex<[T]>;

    /// get_unchecked_mut returns the item at the given index without bound checks.
    unsafe fn get_unchecked_mut<I>(&mut self, index: I) -> &mut <I as SliceIndex<[T]>>::Output
    where
        I: SliceIndex<[T]>;

    /// len returns the length of the array or vector
    fn len(&self) -> usize;
}

impl<T, A, const N: usize> ArrayOrVec<T, A> for [T; N]
where
    A: Allocator,
    Box<[T], A>: From<[T; N]>,
    Vec<T, A>: From<[T; N]>,
{
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

impl<T, A> ArrayOrVec<T, A> for Vec<T, A>
where
    A: Allocator,
    Vec<T, A>: BorrowMut<[T]>,
{
    type Item = T;
    const LEN: Option<usize> = None;

    fn as_slice(&self) -> &[T] {
        <Vec<T, A>>::as_slice(self)
    }

    fn as_mut_slice(&mut self) -> &mut [T] {
        <Vec<T, A>>::as_mut_slice(self)
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
        <Vec<T, A>>::len(self)
    }
}
