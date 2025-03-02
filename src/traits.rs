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
    + ops::BitXorAssign
    + ops::BitAnd<Output = Self>
    + ops::BitAndAssign
    + ops::BitOr<Output = Self>
    + ops::BitOrAssign
    + ops::Not<Output = Self>
    + ops::Shl<Output = Self>
    + ops::ShlAssign
    + ops::Shr<Output = Self>
    + ops::ShrAssign
    + ops::Add<Output = Self>
    + ops::AddAssign
    + ops::Sub<Output = Self>
    + ops::SubAssign
    + ops::Mul<Output = Self>
    + ops::MulAssign
    + ops::Div<Output = Self>
    + ops::DivAssign
    + cmp::PartialOrd
{
    /// The literal 0.
    const ZERO: Self;
    /// The literal 1.
    const ONE: Self;
    /// Number of bits in binary representation
    const BITS: u32;
    /// Largest representable value
    const MAX: Self;
    /// Smallest representable value
    const MIN: Self;
}

macro_rules! impl_type_const {
    ($type:ty) => {
        impl BaseInt for $type {
            const ZERO: $type = 0;
            const ONE: $type = 1;
            const BITS: u32 = <$type>::BITS;
            const MAX: $type = <$type>::MAX;
            const MIN: $type = <$type>::MIN;
        }
    };
}

impl_type_const!(u8);
impl_type_const!(u16);
impl_type_const!(u32);
impl_type_const!(u64);
impl_type_const!(u128);
impl_type_const!(usize);

impl_type_const!(i8);
impl_type_const!(i16);
impl_type_const!(i32);
impl_type_const!(i64);
impl_type_const!(i128);
impl_type_const!(isize);
