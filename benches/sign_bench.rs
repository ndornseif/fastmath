// fastmath - Various performance optimized math operations.
// Copyright 2025 N. Dornseif
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! benchmarks

use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

const SCRATCHPAD_SIZE: usize = 1_000_00;

macro_rules! prepare_scratchpad {
    ($type:tt) => {{
        let mut rn = fastmath::rng::Lehmer64::new(0);
        let mut pad = [0 as $type; SCRATCHPAD_SIZE];
        for val in &mut pad {
            *val = rn.generate() as $type;
        }
        pad
    }};
}

macro_rules! make_sign_bench {
    // Arguments are test type, test function name and test name.
    ($type:tt, $fn_name:ident, $test_name:expr) => {
        paste::item! {
            fn [< bench _ $fn_name >](pad: [$type; SCRATCHPAD_SIZE]) {
                use fastmath::sign;
                for val in pad.chunks(2) {
                    black_box(sign::int_same_sign(val[0], val[1]));
                }
            }

            fn $fn_name(c: &mut Criterion) {
                let pad = prepare_scratchpad!($type);
                c.bench_function($test_name, |b| b.iter(|| [< bench _ $fn_name >](pad)));
            }
        }
    };
}

make_sign_bench!(i8, bench_i8_same_sign, "Benchmark int_same_sign for i8");
make_sign_bench!(i16, bench_i16_same_sign, "Benchmark int_same_sign for i16");
make_sign_bench!(i32, bench_i32_same_sign, "Benchmark int_same_sign for i32");
make_sign_bench!(i64, bench_i64_same_sign, "Benchmark int_same_sign for i64");
make_sign_bench!(
    i128,
    bench_i128_same_sign,
    "Benchmark int_same_sign for i128"
);
make_sign_bench!(
    isize,
    bench_isize_same_sign,
    "Benchmark int_same_sign for isize"
);

criterion_group!(
    benches,
    bench_i8_same_sign,
    bench_i16_same_sign,
    bench_i32_same_sign,
    bench_i64_same_sign,
    bench_i128_same_sign,
    bench_isize_same_sign
);

criterion_main!(benches);
