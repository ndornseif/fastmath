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
//! assert_eq!(rn.generate(), 0x064577751fa75998u64);
//! assert_eq!(rn.generate(), 0x2fffbd97d5f2f80au64);
//! assert_eq!(rn.generate(), 0x9981098d9584ad55u64);
//! ```

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
        let _ = new_rng.generate();
        let _ = new_rng.generate();
        new_rng
    }

    /// Generates a 'random' u64 and advances the generator state one step.
    pub fn generate(&mut self) -> u64 {
        self.state = self.state.wrapping_mul(Lehmer64::MUL_CONSTANT);
        (self.state >> 64) as u64
    }
}
