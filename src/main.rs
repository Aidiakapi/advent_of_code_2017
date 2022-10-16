#![feature(byte_slice_trim_ascii)]
#![feature(never_type)]
#![feature(stmt_expr_attributes)]
#![feature(test)]

extern crate test;

mod prelude;
mod vec2;

framework::main!(
    day01,
    day02,
    day03,
);
