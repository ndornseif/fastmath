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
    /// Number of bits in binary representation minus 1
    const BITS_M_1: u32;
    /// Number of bits in binary representation minus 2
    const BITS_M_2: u32;
    /// Largest representable value
    const MAX: Self;
    /// Smallest representable value
    const MIN: Self;
    /// Masks of the MSB, signbit for signed types
    const MSB: Self;
    /// Shifts the bits to the right by a specified amount, n,
    /// wrapping the truncated bits to the end of the resulting integer.
    fn rotate_right(self, n: u32) -> Self;
    /// Shifts the bits to the left by a specified amount, n,
    /// wrapping the truncated bits to the end of the resulting integer.
    fn rotate_left(self, n: u32) -> Self;
    /// Performs primitive typecast from u64 to T.
    fn from_u64(n: u64) -> Self;
}

macro_rules! impl_type_const {
    ($($type:ty),*) => {
        $(impl BaseInt for $type {
            const ZERO: $type = 0;
            const ONE: $type = 1;
            const BITS: u32 = <$type>::BITS;
            const BITS_M_1: u32 = <$type>::BITS - 1;
            const BITS_M_2: u32 = <$type>::BITS - 2;
            const MAX: $type = <$type>::MAX;
            const MIN: $type = <$type>::MIN;
            const MSB: $type = 1 << (<$type>::BITS - 1);
            #[inline]
            fn rotate_right(self, n: u32) -> Self {
                self.rotate_right(n)
            }
            #[inline]
            fn rotate_left(self, n: u32) -> Self {
                self.rotate_left(n)
            }
            #[inline]
            fn from_u64(n: u64) -> Self {
                n as $type
            }
        }
    )*};
}

impl_type_const!(u8, u16, u32, u64, u128, usize);
impl_type_const!(i8, i16, i32, i64, i128, isize);
