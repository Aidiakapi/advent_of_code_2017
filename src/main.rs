#![feature(byte_slice_trim_ascii)]
#![feature(never_type)]
#![feature(custom_test_frameworks)]
#![cfg_attr(feature = "criterion", test_runner(criterion::runner))]

mod prelude;

framework::main!(
    day01,
    day02,
    day03,
    day04,
    day05,
);
