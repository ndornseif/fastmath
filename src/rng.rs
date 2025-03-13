// fastmath - Various performance optimized math operations.
// Copyright 2025 N. Dornseif
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! rng - Optimized RRNG.
//!
//! # Examples
//! ```
//! use fastmath::rng;
//!
//! // Zero is a weak seed, but is internaly replaced with a strong default.
//! let mut rn = rng::Lehmer64::new(0);
//! assert_eq!(rn.generate_u64(), 0x064577751fa75998u64);
//! assert_eq!(rn.generate_u8(), 0x0au8);
//! assert_eq!(rn.generate_isize(), -0x667ef6726a7b52abisize);
//! ```


/// Define a function that generates a random result of the specified datatype.
macro_rules! generic_generation_function {
    ($fnname:ident, $datatype:ty) => {
        /// Generates a 'random' integer and advances the generator state one step.
        pub fn $fnname(&mut self) -> $datatype {
            self.advance();
            (self.state >> 64) as $datatype
        }
    };
}

#[derive(Debug, Copy, Clone)]
/// Fast high quality LCG PRNG
/// but NOT cryptographically secure.
pub struct Lehmer64 {
    state: u128,
}
impl Lehmer64 {
    const DEFAULT_SEED: u128 = 0xfe1f873c7fc74fa65743b339f566f7bb;
    const MUL_CONSTANT: u128 = 0xda942042e4dd58b5;
    /// Initalize a new RNG with the specified seed.
    /// Where the seed is the intial internal state.
    pub fn new(seed: u128) -> Self {
        // If seed is zero the generator only produces zeroes.
        let mut new_rng = if seed == 0 {
            Lehmer64 {
                state: Lehmer64::DEFAULT_SEED,
            }
        } else {
            Lehmer64 { state: seed }
        };
        // Shuffle the internal state twice.
        // This prevents the first value from being low
        // if the seed was a small number.
        let _ = new_rng.advance();
        let _ = new_rng.advance();
        new_rng
    }

    /// Advances the generator state one step.
    #[inline(always)]
    fn advance(&mut self) {
        self.state = self.state.wrapping_mul(Lehmer64::MUL_CONSTANT);
    }

    generic_generation_function!(generate_u8, u8);
    generic_generation_function!(generate_u16, u16);
    generic_generation_function!(generate_u32, u32);
    generic_generation_function!(generate_u64, u64);
    generic_generation_function!(generate_usize, usize);


    generic_generation_function!(generate_i8, i8);
    generic_generation_function!(generate_i16, i16);
    generic_generation_function!(generate_i32, i32);
    generic_generation_function!(generate_i64, i64);
    generic_generation_function!(generate_isize, isize);

}
