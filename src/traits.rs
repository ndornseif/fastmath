// fastmath - Various performance optimized math operations.
// Copyright 2025 N. Dornseif
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! traits - Define traits for numeric values to allow easier generic programming.

use core::{cmp, ops};

/// A primitive integer.
pub trait BaseInt:
    Sized
    + ops::BitXor<Output = Self>
    + ops::Add<Output = Self>
    + ops::AddAssign
    + ops::BitAnd<Output = Self>
    + ops::BitAndAssign
    + cmp::PartialOrd
{
    /// The literal 0.
    const ZERO: Self;
    /// The literal 1.
    const ONE: Self;
}

macro_rules! impl_type_const {
    ($type:ty, $zero:expr, $one:expr) => {
        impl BaseInt for $type {
            const ZERO: $type = $zero;
            const ONE: $type = $one;
        }
    };
}

impl_type_const!(u8, 0, 1);
impl_type_const!(u16, 0, 1);
impl_type_const!(u32, 0, 1);
impl_type_const!(u64, 0, 1);
impl_type_const!(u128, 0, 1);
impl_type_const!(usize, 0, 1);

impl_type_const!(i8, 0, 1);
impl_type_const!(i16, 0, 1);
impl_type_const!(i32, 0, 1);
impl_type_const!(i64, 0, 1);
impl_type_const!(i128, 0, 1);
impl_type_const!(isize, 0, 1);
