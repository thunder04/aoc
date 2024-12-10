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

#[rustfmt::skip]
macro_rules! day_to_number {
    (one) => { 1 }; (two) => { 2 }; (three) => { 3 }; (four) => { 4 }; (five) => { 5 }; (six) => { 6 };
    (seven) => { 7 }; (eight) => { 8 }; (nine) => { 9 }; (ten) => { 10 }; (eleven) => { 11 }; (twelve) => { 12 };
    (thirteen) => { 13 }; (fourteen) => { 14 }; (fifteen) => { 15 }; (sixteen) => { 16 }; (seventeen) => { 17 }; (eighteen) => { 18 };
    (nineteen) => { 19 }; (twenty) => { 20 }; (twenty_one) => { 21 }; (twenty_two) => { 22 }; (twenty_three) => { 23 }; (twenty_four) => { 24 };
    (twenty_five) => { 25 }; (twenty_six) => { 26 }; (twenty_seven) => { 27 }; (twenty_eight) => { 28 }; (twenty_nine) => { 29 }; (thirty) => { 30 };
    (thirty_one) => { 31 };
}

pub(crate) use day_to_number;

#[macro_export]
macro_rules! export_days {
    ($($day: ident $(: P1 == $p1_exp: expr)? $(, P2 == $p2_exp: expr )?)*) => {
        $(pub mod $day;)*

        pub const ALL_DAYS: &[u8] = &[$($crate::day_to_number!($day),)*];

        pub fn run(days: Vec<u8>) -> eyre::Result<()> {
            use eyre::WrapErr as _;

            for day in days {
                match day {
                    $(_ if day == $crate::day_to_number!($day) => {
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
