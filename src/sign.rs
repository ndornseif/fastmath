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
//! assert_eq!(sign::int_sign_i8(-2), -1);
//! assert_eq!(sign::int_sign_i16(1), 1);
//! assert_eq!(sign::int_sign_i64(0), 1);
//! assert_eq!(sign::int_sign_i128(i128::MAX), 1);
//!
//! assert!(sign::int_opposite_sign_i8(0, -1));
//! assert!(sign::int_same_sign_isize(0, 1));
//! ```

/// Define a function that returns the sign of a signed integer.
macro_rules! generic_sign_function {
    ($fnname:ident, $datatype:ty) => {
        /// Returns the sign of a signed integer.
        /// 1 if x > -1, -1 otherwise.
        /// Behaviour similar to .signum() except zero is treated as positive.
        #[inline]
        pub fn $fnname(x: $datatype) -> $datatype {
            const MSB_MASK: $datatype = 1 << (<$datatype>::BITS - 1);
            const BITS_M_2: u32 = <$datatype>::BITS - 2;
            1 - (x & MSB_MASK).rotate_right(BITS_M_2)
        }
    };
}

generic_sign_function!(int_sign_i8, i8);
generic_sign_function!(int_sign_i16, i16);
generic_sign_function!(int_sign_i32, i32);
generic_sign_function!(int_sign_i64, i64);
generic_sign_function!(int_sign_i128, i128);
generic_sign_function!(int_sign_isize, isize);

/// Define a function that returns true if both supplied ints have opposite signs.
macro_rules! generic_sign_comparison_functions {
    ($fnname_opposite:ident, $fnname_same:ident, $datatype:ty) => {
        /// Returns true when x and y have opposite signs.
        /// Zero is considered positive.
        #[inline]
        pub fn $fnname_opposite(x: $datatype, y: $datatype) -> bool {
            (x ^ y) < 0
        }

        /// Returns true when x and y have the same sign.
        /// Zero is considered positive.
        #[inline]
        pub fn $fnname_same(x: $datatype, y: $datatype) -> bool {
            !$fnname_opposite(x, y)
        }
    };
}

generic_sign_comparison_functions!(int_opposite_sign_i8, int_same_sign_i8, i8);
generic_sign_comparison_functions!(int_opposite_sign_i16, int_same_sign_i16, i16);
generic_sign_comparison_functions!(int_opposite_sign_i32, int_same_sign_i32, i32);
generic_sign_comparison_functions!(int_opposite_sign_i64, int_same_sign_i64, i64);
generic_sign_comparison_functions!(int_opposite_sign_i128, int_same_sign_i128, i128);
generic_sign_comparison_functions!(int_opposite_sign_isize, int_same_sign_isize, isize);

#[cfg(test)]
mod tests {
    use super::*;

    /// Defines a test function for integer sign function.
    macro_rules! test_int_sign {
        ($testfn:expr, $datatype:ty, $testname:ident) => {
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

    test_int_sign!(int_sign_i8, i8, test_i8_int_sign);
    test_int_sign!(int_sign_i16, i16, test_i16_int_sign);
    test_int_sign!(int_sign_i32, i32, test_i32_int_sign);
    test_int_sign!(int_sign_i64, i64, test_i64_int_sign);
    test_int_sign!(int_sign_i128, i128, test_i128_int_sign);
    test_int_sign!(int_sign_isize, isize, test_isize_int_sign);

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
        int_same_sign_i8,
        int_opposite_sign_i8,
        i8,
        test_i8_sign_comparison
    );
    test_sign_comparison!(
        int_same_sign_i16,
        int_opposite_sign_i16,
        i16,
        test_i16_sign_comparison
    );
    test_sign_comparison!(
        int_same_sign_i32,
        int_opposite_sign_i32,
        i32,
        test_i32_sign_comparison
    );
    test_sign_comparison!(
        int_same_sign_i64,
        int_opposite_sign_i64,
        i64,
        test_i64_sign_comparison
    );
    test_sign_comparison!(
        int_same_sign_i128,
        int_opposite_sign_i128,
        i128,
        test_i128_sign_comparison
    );
    test_sign_comparison!(
        int_same_sign_isize,
        int_opposite_sign_isize,
        isize,
        test_isize_sign_comparison
    );
}
