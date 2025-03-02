// fastmath - Various performance optimized math operations.
// Copyright 2025 N. Dornseif
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! log - Logarithms and related functions.
//!
//! # Examples
//! ```
//! use fastmath::log;
//!
//! // Equivalent to 2^63 - 1
//! // = 9223372036854775807
//! let testval: u64 = (1 << 63) - 1;
//! assert_eq!(log::u64_log2_floor(testval), 62);
//!
//! // Float imprecision causes the wrong value
//! // to be returned for f64 (63 instead of 62).
//! assert_eq!((testval as f64).log2().floor() as u32, 63);
//! ```

/// Define a function for supplied datatype that is equivalent to floor(log2(x)).
macro_rules! generic_log2_floor {
    ($fnname:ident, $datatype:ty) => {
        /// Equivalent to floor(log2(x))
        /// Returns `u32::MAX` if x is zero.
        pub fn $fnname(x: $datatype) -> u32 {
            (<$datatype>::BITS - x.leading_zeros()).wrapping_sub(1)
        }
    };
}

generic_log2_floor!(u8_log2_floor, u8);
generic_log2_floor!(u16_log2_floor, u16);
generic_log2_floor!(u32_log2_floor, u32);
generic_log2_floor!(u64_log2_floor, u64);
generic_log2_floor!(u128_log2_floor, u128);
generic_log2_floor!(usize_log2_floor, usize);

mod tests {
    #[allow(unused_imports)]
    use super::*;

    /// Define a test function to test a log2_floor function.
    macro_rules! test_log2_floor {
        ($datatype:ty, $testfn:expr, $testname:ident) => {
            #[test]
            fn $testname() {
                // Powers of two and the adjacent numbers up to 2**(bits - 1)
                for exponent in 1..<$datatype>::BITS {
                    let base_value: $datatype = (1 << exponent);
                    let prev_value: $datatype = base_value - 1;
                    let next_value: $datatype = base_value + 1;

                    // Test 2**exponent - 1
                    assert_eq!(
                        $testfn(prev_value),
                        exponent - 1,
                        "Failed with x=2^{} - 1",
                        exponent
                    );

                    // Test 2**exponent
                    assert_eq!(
                        $testfn(base_value),
                        exponent,
                        "Failed with x=2^{}",
                        exponent
                    );

                    // Test 2**exponent + 1
                    assert_eq!(
                        $testfn(next_value),
                        exponent,
                        "Failed with x=2^{} + 1",
                        exponent
                    );
                }
                // Special edge cases
                assert_eq!($testfn(0), u32::MAX, "Failed with x=0");
                assert_eq!($testfn(1), 0, "Failed with x=1");
                assert_eq!(
                    $testfn(<$datatype>::MAX),
                    <$datatype>::BITS - 1,
                    "Failed with x=MAXINT"
                );
            }
        };
    }

    test_log2_floor!(u8, u8_log2_floor, u8_log2_floor_test);
    test_log2_floor!(u16, u16_log2_floor, u16_log2_floor_test);
    test_log2_floor!(u32, u32_log2_floor, u32_log2_floor_test);
    test_log2_floor!(u64, u64_log2_floor, u64_log2_floor_test);
    test_log2_floor!(u128, u128_log2_floor, u128_log2_floor_test);
    test_log2_floor!(usize, usize_log2_floor, usize_log2_floor_test);
}
