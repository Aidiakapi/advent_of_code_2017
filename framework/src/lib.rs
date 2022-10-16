#![feature(auto_traits)]
#![feature(maybe_uninit_array_assume_init)]
#![feature(maybe_uninit_uninit_array)]
#![feature(negative_impls)]

pub mod error;
pub mod inputs;
pub mod parsers;
pub mod prelude;
pub mod result;
pub mod runner;

#[macro_export]
macro_rules! main {
    ($($day:ident),*$(,)?) => {
        $(mod $day;)*

        fn main() -> framework::result::Result<()> {
            $crate::runner::run(&[
                $(Box::new($day::day_metadata()),)*
            ])
        }
    };
}

#[macro_export]
macro_rules! day {
    ($day_nr:literal, true, $parse_fn:ident => $($part_fn:ident),+$(,)?) => {
        $crate::day!(@primary, $day_nr, $parse_fn => $($part_fn),+);
        $crate::day!(@bench, $day_nr, $parse_fn => $($part_fn),+);
    };
    ($day_nr:literal, false, $parse_fn:ident => $($part_fn:ident),+$(,)?) => {
        $crate::day!(@primary, $day_nr, $parse_fn => $($part_fn),+);
    };
    (@primary, $day_nr:literal, $parse_fn:ident => $($part_fn:ident),+) => {
use super::prelude::*;
pub fn day_metadata() -> impl $crate::runner::SpecificDayMetadata {
    $crate::runner::DayMetadata {
        number: $day_nr,
        parse_fn: Box::new(|input| {
            $crate::result::IntoResult::into_result($parse_fn(input))
        }),
        parts: vec![$(
            $crate::runner::DayPart {
                name: stringify!($part_fn),
                function: Box::new(|input| {
                    $crate::result::IntoResult::into_result($part_fn(input))
                        .map(|result| std::string::ToString::to_string(&result))
                }),
            },
        )+],
    }
}
    };
    (@bench, $day_nr:literal, $parse_fn:ident => $($part_fn:ident),+) => {
#[cfg(test)]
pub mod benchmarks {
    use ::test::{black_box, Bencher};
    #[bench]
    pub fn $parse_fn(b: &mut Bencher) {
        let mut inputs = $crate::inputs::Inputs::new();
        let input = inputs.get($day_nr).expect("could not get input");
        b.iter(|| super::$parse_fn(&input));
    }
    $(
        #[bench]
        pub fn $part_fn(b: &mut Bencher) {
            let mut inputs = $crate::inputs::Inputs::new();
            let input = inputs.get($day_nr).expect("could not get input");
            let parsed = super::$parse_fn(&input).expect("could not parse input");
            b.iter(|| super::$part_fn(&parsed));
        }
    )*
}
    };
}

#[macro_export]
macro_rules! tests {
    ($($x:tt)*) => {
        #[cfg(test)]
        mod tests {
            use super::*;
            use $crate::test_pt;

            $($x)*
        }
    };
}

#[macro_export]
macro_rules! test_pt {
    ($parse_fn:ident, $pt_fn:ident, $($input:expr => $output:expr),+$(,)?) => {
#[test]
fn $pt_fn() {
    $(
        let parsed = match $crate::result::IntoResult::into_result(super::$parse_fn($input)) {
            Ok(x) => x,
            Err(e) => panic!("parsing failed: {e}\ninput: {:?}", String::from_utf8_lossy($input).into_owned()),
        };
        let result = match $crate::result::IntoResult::into_result(super::$pt_fn(&parsed)) {
            Ok(x) => x,
            Err(e) => panic!("execution failed: {e}\ninput: {:?}", String::from_utf8_lossy($input).into_owned()),
        };
        let output = $output;
        if result != output {
            panic!("incorrect output, expected: {:?}, got: {:?}\ninput: {:?}", output, result, String::from_utf8_lossy($input).into_owned());
        }
    )+
}
    }
}
