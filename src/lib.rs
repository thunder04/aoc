#![allow(internal_features)]
#![feature(
    iter_map_windows,
    portable_simd,
    slice_as_chunks,
    stdarch_x86_avx512,
    core_intrinsics
)]

#[macro_use]
extern crate tracing;

pub mod _2023;
pub mod _2024;
pub mod utils;

#[macro_export]
macro_rules! export_days {
    ($($day: ident $(: P1 == $p1_exp: expr)? $(, P2 == $p2_exp: expr )?)*) => {
        $(pub mod $day;)*

        // Remove the leading underscore in a cool way.
        pub const ALL_DAYS: &[&str] = &[$(const {
            unsafe {
                let d = stringify!($day);
                let d = core::slice::from_raw_parts(d.as_ptr().add(1), d.len() - 1);

                std::str::from_utf8_unchecked(d)
            }
        }, )*];

        pub fn run(days: Vec<impl Into<String>>) -> eyre::Result<()> {
            use eyre::WrapErr as _;

            for day in days {
                let day = day.into();

                match day {
                    $(_ if day == stringify!($day).trim_start_matches('_') => {
                        let year = module_path!().split("::").nth(1).unwrap().trim_start_matches('_');
                        let path = format!("inputs/{year}/{day}.txt");
                        let input = &std::fs::read(&path).wrap_err_with(|| format!("Failed to read file \"{path}\""))?;

                        if input.is_empty() {
                            error!(target: "aoc", "[Year {year}] [Day {day}] Input file is empty. Skipping!");
                            continue;
                        } else if std::str::from_utf8(input).is_err() {
                            error!(target: "aoc", "[Year {year}] [Day {day}] Input file must be valid UTF-8. Skipping!");
                            continue;
                        } else if input[input.len() - 1] != b'\n' {
                            error!(target: "aoc", "[Year {year}] [Day {day}] Input file must end with a trailing newline. Skipping!");
                            continue;
                        }

                        $(
                            let result = $day::part_1(input);

                            if result == $p1_exp {
                                info!(target: "aoc", "[Year {year}] [Day {day}] Part 1: {result} == {}", $p1_exp);
                            } else {
                                warn!(target: "aoc", "[Year {year}] [Day {day}] Part 1: {result} != {}", $p1_exp);
                            }

                            debug!(target: "aoc", "{}", easybench::bench(|| $day::part_1(input)));
                        )?

                        $(
                            let result = $day::part_2(input);

                            if result == $p2_exp {
                                info!(target: "aoc", "[Year {year}] [Day {day}] Part 2: {result} == {}", $p2_exp);
                            } else {
                                warn!(target: "aoc", "[Year {year}] [Day {day}] Part 2: {result} != {}", $p2_exp);
                            }

                            debug!(target: "aoc", "{}", easybench::bench(|| $day::part_2(input)));
                        )?
                    })*

                    _ => return Err(eyre::Report::msg("day hasn't been implemented")),
                }

                eprintln!();
            }

            Ok(())
        }
    };
}
