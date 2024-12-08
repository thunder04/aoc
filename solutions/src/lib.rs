#![feature(iter_map_windows, portable_simd, core_intrinsics, slice_as_chunks)]
#![allow(internal_features, dead_code)]

#[macro_use]
extern crate tracing;

pub mod _2023;
pub mod _2024;
pub mod utils;

pub type Runner = (Option<Solution>, Option<Solution>);
pub type Solution = fn() -> u64;

#[macro_export]
macro_rules! export_days {
    ($($title: literal: $day_mod: ident),*) => {
        pub use $crate::{Runner, Solution};
        use color_eyre::Section;

        $(pub mod $day_mod;)*

        pub fn run(day: u8) -> eyre::Result<()> {
            const AVAILABLE_DAYS: &[&'static str] = &[$($title),*];

            match day {
                $(_ if day == const { parse_day($title) } => return _run(day, $day_mod::run()),)*

                _ => {
                    return Err(
                        eyre::Report::msg("this day hasn't been implemented").with_suggestion(|| {
                            format!("the available days are: {}", AVAILABLE_DAYS.join(", "))
                        }),
                    )
                }
            }

            pub fn _run(id: u8, (part_1, part_2): Runner) -> eyre::Result<()> {
                if let Some(part_1) = part_1 {
                    info!("Day {id}, part 1's answer is \"{}\".", (part_1)());
                }

                if let Some(part_2) = part_2 {
                    info!("Day {id}, part 2's answer is \"{}\".", (part_2)());
                }

                Ok(())
            }
        }

        const fn parse_day(title: &'static str) -> u8 {
            let title = title.as_bytes();

            if title[0] != b'D' || title[1] != b'a' || title[2] != b'y' || title[3] != b' ' {
                panic!("A day's title must start with \"Day \", followed by its number.");
            }

            if title[title.len() - 1].is_ascii_whitespace() {
                panic!("Trailing whitespaces aren't allowed fam. Why? Because I can, duh!");
            }

            $crate::read_number_lazily!(title, 4, b':', {
                panic!("A day's name must start with \"Day ##: \"");
            }).0 as u8
        }
    };
}

#[macro_export]
macro_rules! bench_days {
    ($($year: ident: [ $($day: ident),* $(,)? ] ),* $(,)?) => {
        fn criterion_benchmark(c: &mut criterion::Criterion) {
            $($({
                let (part_1, part_2) = aoc_solutions::$year::$day::run();

                if let Some(part_1) = part_1 {
                    c.bench_function(&format!("{1}::{0}::P1", stringify!($day), &stringify!($year)[1..]), |b| b.iter(part_1));
                }

                if let Some(part_2) = part_2 {
                    c.bench_function(&format!("{1}::{0}::P2", stringify!($day), &stringify!($year)[1..]), |b| b.iter(part_2));
                }
            })*)*
        }
    };
}
