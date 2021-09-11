/*
    Copyright © 2021 StarBrilliant <coder@poorlab.com>
    This work is free. You can redistribute it and/or modify it under the
    terms of the Do What The Fuck You Want To Public License, Version 2,
    as published by Sam Hocevar. See the COPYING file for more details.
*/

#![cfg(test)]

use super::*;
use core::ops::Add;

fn sum<'a, T, A>(a: &'a A) -> T
where
    T: 'a + Default + Add<&'a T, Output = T>,
    A: ArrayOrVec<T>,
{
    a.iter().fold(T::default(), |acc, x| acc + x)
}

#[test]
fn test_sum_array() {
    let result = sum(&[1, 2, 3]);
    assert_eq!(result, 6);
}

#[test]
fn test_sum_vec() {
    let result = sum(&vec![1, 2, 3]);
    assert_eq!(result, 6);
}
