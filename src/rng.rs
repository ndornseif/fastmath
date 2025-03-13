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

use crate::consts::double::INV_2POW53;
use crate::consts::float::INV_2POW24;

/// Define a function that generates a random result of the specified datatype.
macro_rules! generic_generation_function {
    ($fnname:ident, $datatype:ty) => {
        /// Generates a 'random' integer and advances the generator state one step.
        #[inline]
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
    /// If the seed is zero, it is replaced with a predefined strong default.
    pub fn new(seed: u128) -> Self {
        let state = if seed == 0 { Self::DEFAULT_SEED } else { seed };
        let mut new_rng = Lehmer64 { state };
        // Shuffle the internal state twice.
        // This prevents the first value from being low if the seed was a small number.
        new_rng.advance();
        new_rng.advance();
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

    // We define a seperate function for 128bit datatypes since they need two generator steps.
    /// Generates a 'random' u128 and advances the generator state two steps.
    #[inline]
    pub fn generate_u128(&mut self) -> u128 {
        self.advance();
        let high_bits = self.state >> 64;
        self.advance();
        (high_bits << 64) | (self.state >> 64)
    }

    generic_generation_function!(generate_i8, i8);
    generic_generation_function!(generate_i16, i16);
    generic_generation_function!(generate_i32, i32);
    generic_generation_function!(generate_i64, i64);
    generic_generation_function!(generate_isize, isize);

    // We define a seperate function for 128bit datatypes since they need two generator steps.
    /// Generates a 'random' i128 and advances the generator state two steps.
    #[inline]
    pub fn generate_i128(&mut self) -> i128 {
        self.advance();
        let high_bits = self.state >> 64;
        self.advance();
        ((high_bits << 64) | (self.state >> 64)) as i128
    }

    /// Generate a 'random' f64 in the range [0; 1).  
    /// Has 53 bits of effective entropy
    /// and does not produce all possible values in the range.
    #[inline]
    pub fn generate_f64(&mut self) -> f64 {
        (self.generate_u64() >> 11) as f64 * INV_2POW53
    }

    /// Generate a 'random' f32 in the range [0; 1).  
    /// Has 24 bits of effective entropy
    /// and does not produce all possible values in the range.
    #[inline]
    pub fn generate_f32(&mut self) -> f32 {
        (self.generate_u64() >> 40) as f32 * INV_2POW24
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    /// test that two u64 are correctly used when genrating a u128.
    #[test]
    fn u128_concatenation_test() {
        let mut rn = Lehmer64::new(0);
        let full_integer = rn.generate_u128();

        // Reset the generator state here.
        let mut rn = Lehmer64::new(0);
        assert_eq!((full_integer >> 64) as u64, rn.generate_u64());
        assert_eq!(full_integer as u64, rn.generate_u64());

        let mut rn = Lehmer64::new(0);
        let full_integer = rn.generate_i128();

        // Reset the generator state here.
        let mut rn = Lehmer64::new(0);
        assert_eq!((full_integer >> 64) as i64, rn.generate_i64());
        assert_eq!(full_integer as i64, rn.generate_i64());
    }
}
