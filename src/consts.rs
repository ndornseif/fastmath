// fastmath - Various performance optimized math operations.
// Copyright 2025 N. Dornseif
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! consts - Mathematical constants.
//!
//! Constants are available as f32 or f64.  
//! They are the closest available floating point value to the true value.
//!
//! # Examples
//! ```
//! use fastmath::consts;
//!
//! // True value to 20 decimal digits:
//! //                                        2.41421356237309504880
//! assert_eq!(consts::double::SQRT_2_PLUS_1, 2.41421356237309492343);
//! ```

// More digits for reference:
// SQRT_2_PLUS_1:
// 2.41421356237309504880168872420969807856967187537694807317667973799073247846

/// Double precision (f64) constants.
pub mod double {
    use core::mem::transmute;

    /// One plus the square root of two, also known as the silver ratio.
    /// Exact float representation: 2.41421356237309492343
    pub const SQRT_2_PLUS_1: f64 = unsafe { transmute::<u64, f64>(0x4003504f333f9de6) };
}

/// Single precision (f32) constants.
pub mod float {
    use core::mem::transmute;

    /// One plus the square root of two, also known as the silver ratio.
    /// Exact float representation: 2.41421365737915039063
    pub const SQRT_2_PLUS_1: f32 = unsafe { transmute::<u32, f32>(0x401a827a) };
}
