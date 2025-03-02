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
//! // True value to 22 decimal digits:
//! //                                       2.414213562373095048801
//! assert_eq!(consts::float::SQRT_2_PLUS_1, 2.414213657379150390625);
//! ```

// More digits for reference:
// SQRT_2_PLUS_1:
// 2.41421356237309504880168872420969807856967187537694807317667973799073247846
//
// CBRT_2:
// 1.25992104989487316476721060727822835057025146470150798008197511215529967651
//
// CBRT_3:
// 1.44224957030740838232163831078010958839186925349935057754641619454168759682
//
// SUPERGOLDEN_RATIO:
// 1.46557123187676802665673122521993910802557756847228570164318311124926299668
//
// SUPERSILVER_RATIO:
// 2.20556943040059031170202861778382342637710891959769944047055220355183479035

/// Double precision (f64) constants.
pub mod double {
    use core::mem::transmute;

    /// One plus the square root of two, also known as the silver ratio.
    /// The positive solution of the equation x^2 = 2x + 1.
    /// Exact double representation: 2.41421356237309492343001693370752036571502685546875
    pub const SQRT_2_PLUS_1: f64 = unsafe { transmute::<u64, f64>(0x4003504f333f9de6) };

    /// The cube root of two  
    /// Exact double representation: 1.2599210498948731906665443602832965552806854248046875
    pub const CBRT_2: f64 = unsafe { transmute::<u64, f64>(0x3ff428a2f98d728b) };

    /// The cube root of three  
    /// Exact double representation: 1.442249570307408301772511549643240869045257568359375
    pub const CBRT_3: f64 = unsafe { transmute::<u64, f64>(0x3ff7137449123ef6) };

    /// The supergolden ratio. The real solution of the equation x^3 = x^2 + 1.  
    /// Exact double representation: 1.4655712318767679658293445754679851233959197998046875
    pub const SUPERGOLDEN_RATIO: f64 = unsafe { transmute::<u64, f64>(0x3ff772fad1ede80b) };

    /// The supersilver ratio. The real solution of the equation x^3 = 2x^2 + 1.  
    /// Exact double representation: 2.205569430400590391627702047117054462432861328125
    pub const SUPERSILVER_RATIO: f64 = unsafe { transmute::<u64, f64>(0x4001a50195e505e8) };
}

/// Single precision (f32) constants.
pub mod float {
    use core::mem::transmute;

    /// One plus the square root of two, also known as the silver ratio.  
    /// Exact float representation: 2.414213657379150390625
    pub const SQRT_2_PLUS_1: f32 = unsafe { transmute::<u32, f32>(0x401a827a) };

    /// The cube root of two  
    /// Exact float representation: 1.25992107391357421875
    pub const CBRT_2: f32 = unsafe { transmute::<u32, f32>(0x3fa14518) };

    /// The cube root of three  
    /// Exact float representation: 1.4422495365142822265625
    pub const CBRT_3: f32 = unsafe { transmute::<u32, f32>(0x3fb89ba2) };

    /// The supergolden ratio. The real solution of the equation x^3 = x^2 + 1.  
    /// Exact float representation: 1.46557128429412841796875
    pub const SUPERGOLDEN_RATIO: f32 = unsafe { transmute::<u32, f32>(0x3fbb97d7) };

    /// The supersilver ratio. The real solution of the equation x^3 = 2x^2 + 1.  
    /// Exact float representation: 2.2055695056915283203125
    pub const SUPERSILVER_RATIO: f32 = unsafe { transmute::<u32, f32>(0x400d280d) };
}
