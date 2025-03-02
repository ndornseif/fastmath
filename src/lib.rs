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
//! use fastmath::{log, sign, consts};
//!
//! // Log examples
//! // Equivalent to 2**63 - 1
//! // = 9223372036854775807
//! let testval: u64 = (1 << 63) - 1;
//! assert_eq!(log::u64_log2_floor(testval), 62);
//!
//! // Sign examples
//! assert_eq!(sign::int_sign(isize::MIN), -1);
//! assert_eq!(sign::int_sign(0i64), 1);
//! assert_eq!(sign::int_sign(i128::MAX), 1);
//!
//! // Constant examples
//! assert_eq!(consts::double::SQRT_2_PLUS_1, 2.41421356237309492343);
//! ```

#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![no_std]

pub mod consts;
pub mod log;
pub mod rng;
pub mod sign;
pub mod traits;
