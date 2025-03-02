// fastmath - Various performance optimized math operations.
// Copyright 2025 N. Dornseif
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! sign - Sign functions.
//!
//! # Examples
//! ```
//! use fastmath::sign;
//!
//! assert_eq!(sign::i8_int_sign(-2), -1);
//! assert_eq!(sign::i16_int_sign(1), 1);
//! assert_eq!(sign::isize_int_sign(isize::MIN), -1);
//! assert_eq!(sign::i64_int_sign(0), 1);
//! assert_eq!(sign::i128_int_sign(i128::MAX), 1);
//! ```

use crate::traits::BaseInt;

/// Define a function for supplied datatype that
/// returns 1 if x > -1, -1 otherwise.
macro_rules! generic_int_sign {
    ($fnname:ident, $datatype:ty) => {
        /// Returns the sign of a signed integer.
        /// 1 if x > -1, -1 otherwise.
        /// Behaviour similar to .signum() except zero is treated as positive.
        pub fn $fnname(x: $datatype) -> i8 {
            const SIGNBIT_MASK: $datatype = 1 << <$datatype>::BITS - 1;
            const RIGHT_SHIFT_AMOUNT: u32 = <$datatype>::BITS - 2;
            1 - (x & SIGNBIT_MASK).rotate_right(RIGHT_SHIFT_AMOUNT) as i8
        }
    };
}

generic_int_sign!(i8_int_sign, i8);
generic_int_sign!(i16_int_sign, i16);
generic_int_sign!(i32_int_sign, i32);
generic_int_sign!(i64_int_sign, i64);
generic_int_sign!(i128_int_sign, i128);
generic_int_sign!(isize_int_sign, isize);

/// Generic sign comparison
#[inline]
pub fn int_opposite_sign<T: BaseInt>(x: T, y: T) -> bool {
    (x ^ y) < T::ZERO
}

/// Generic sign comparison
#[inline]
pub fn int_same_sign<T: BaseInt>(x: T, y: T) -> bool {
    !int_opposite_sign(x, y)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Defines a test function for integer sign function.
    macro_rules! test_int_sign {
        ($datatype:ty, $testfn:expr, $testname:ident) => {
            #[test]
            fn $testname() {
                assert_eq!($testfn(<$datatype>::MIN), -1, "Failed with x=MININT");
                assert_eq!($testfn(-1), -1, "Failed with x=-1");
                assert_eq!($testfn(0), 1, "Failed with x=0");
                assert_eq!($testfn(1), 1, "Failed with x=1");
                assert_eq!($testfn(<$datatype>::MAX), 1, "Failed with x=MAXINT");
            }
        };
    }

    test_int_sign!(i8, i8_int_sign, i8_int_sign_test);
    test_int_sign!(i16, i16_int_sign, i16_int_sign_test);
    test_int_sign!(i32, i32_int_sign, i32_int_sign_test);
    test_int_sign!(i64, i64_int_sign, i64_int_sign_test);
    test_int_sign!(i128, i128_int_sign, i128_int_sign_test);
    test_int_sign!(isize, isize_int_sign, isize_int_sign_test);

    /// Defines a test function for integer sign comparisons.
    macro_rules! test_sign_comparison {
        ($fnname_same:ident, $fnname_opposite:ident, $datatype:ty, $testname:ident) => {
            #[test]
            fn $testname() {
                assert!(
                    $fnname_opposite(-1 as $datatype, 0 as $datatype),
                    "Failed opposite sign with x=-1, y=0"
                );
                assert!(
                    $fnname_opposite(-1 as $datatype, 1 as $datatype),
                    "Failed opposite sign with x=-1, y=1"
                );
                assert!(
                    !$fnname_opposite(1 as $datatype, 0 as $datatype),
                    "Failed opposite sign withh x=1, y=0"
                );
                assert!(
                    !$fnname_opposite(0 as $datatype, 0 as $datatype),
                    "Failed opposite sign with x=0, y=0"
                );
                assert!(
                    !$fnname_opposite(<$datatype>::MAX, 0 as $datatype),
                    "Failed opposite sign with x=MAXINT, y=0"
                );
                assert!(
                    $fnname_opposite(<$datatype>::MIN, 0 as $datatype),
                    "Failed opposite sign with x=MININT, y=0"
                );
                assert!(
                    $fnname_opposite(<$datatype>::MIN, <$datatype>::MAX),
                    "Failed opposite sign with x=MININT, y=MAXINT"
                );

                assert!(
                    !$fnname_same(-1 as $datatype, 0 as $datatype),
                    "Failed same sign with x=-1, y=0"
                );
                assert!(
                    !$fnname_same(-1 as $datatype, 1 as $datatype),
                    "Failed same sign with x=-1, y=1"
                );
                assert!(
                    $fnname_same(1 as $datatype, 0 as $datatype),
                    "Failed same sign withh x=1, y=0"
                );
                assert!(
                    $fnname_same(0 as $datatype, 0 as $datatype),
                    "Failed same sign with x=0, y=0"
                );
                assert!(
                    $fnname_same(<$datatype>::MAX, 0 as $datatype),
                    "Failed same sign with x=MAXINT, y=0"
                );
                assert!(
                    !$fnname_same(<$datatype>::MIN, 0 as $datatype),
                    "Failed same sign with x=MININT, y=0"
                );
                assert!(
                    !$fnname_same(<$datatype>::MIN, <$datatype>::MAX),
                    "Failed same sign with x=MININT, y=MAXINT"
                );
            }
        };
    }
    test_sign_comparison!(
        int_same_sign,
        int_opposite_sign,
        i8,
        test_i8_sign_comparison
    );
    test_sign_comparison!(
        int_same_sign,
        int_opposite_sign,
        i16,
        test_i16_sign_comparison
    );
    test_sign_comparison!(
        int_same_sign,
        int_opposite_sign,
        i32,
        test_i32_sign_comparison
    );
    test_sign_comparison!(
        int_same_sign,
        int_opposite_sign,
        i64,
        test_i64_sign_comparison
    );
    test_sign_comparison!(
        int_same_sign,
        int_opposite_sign,
        i128,
        test_i128_sign_comparison
    );
    test_sign_comparison!(
        int_same_sign,
        int_opposite_sign,
        isize,
        test_isize_sign_comparison
    );
}
