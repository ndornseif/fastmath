// fastmath - Various performance optimized math operations.
// Copyright 2025 N. Dornseif
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! # fastmath
//! Performance optimized math operations.
//!
//! ## Description
//! Bit level math functions, also includes mathematical constants.
//! Optimized for size and performance. Only uses rust core, no std library or other dependencies.
//!
//! ## Examples
//! ```
//! use fastmath::{log, sign, consts, rng};
//!
//! // Log examples
//! // Equivalent to 2**63 - 1
//! // = 9223372036854775807
//! let testval: u64 = (1 << 63) - 1;
//! assert_eq!(log::u64_log2_floor(testval), 62);
//!
//! // Sign examples
//! assert_eq!(sign::int_sign_isize(isize::MIN), -1);
//! assert_eq!(sign::int_sign_i64(0), 1);
//! assert_eq!(sign::int_sign_i128(i128::MAX), 1);
//!
//! // Constant examples
//! assert_eq!(consts::double::SQRT_2_PLUS_1, 2.41421356237309492343);
//!
//! // Random number generator examples
//! let mut rn = rng::Lehmer64::new(0);
//! assert_eq!(rn.generate_u64(), 0x064577751fa75998);
//! ```

#![warn(missing_docs, missing_debug_implementations, rust_2018_idioms)]
#![no_std]

pub mod consts;
pub mod log;
pub mod rng;
pub mod sign;
pub mod traits;
