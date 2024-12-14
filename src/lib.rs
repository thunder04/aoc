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
    ($($day: ident $(: P1 == $p1_exp: literal)? $(, P2 == $p2_exp: literal )? $([ bench = $bench: literal ])?)*) => {
        $(pub mod $day;)*

        use std::{slice::from_raw_parts, str::from_utf8_unchecked, fmt::Display};
        use color_eyre::owo_colors::OwoColorize as _;
        use eyre::WrapErr as _;

        pub const ALL_DAYS: &[&str] = &[$(const {
            unsafe {
                let d = stringify!($day).as_bytes();

                if d[1] == b'0' {
                    from_utf8_unchecked(from_raw_parts(d.as_ptr().add(2), d.len() - 2))
                } else {
                    from_utf8_unchecked(from_raw_parts(d.as_ptr().add(1), d.len() - 1))
                }
            }
        }, )*];

        pub fn run(days: Vec<impl Into<String>>) -> eyre::Result<()> {
            for day in days {
                let day = day.into();

                match day {
                    $(_ if day == stringify!($day).trim_start_matches(|x| x == '_' || x == '0') => {
                        let year = module_path!().split("::").nth(1).unwrap().trim_start_matches('_');
                        let day = stringify!($day).trim_start_matches(|x| x == '_');
                        let prefix = eye_candy_prefix(year, day);

                        #[allow(unused_mut)]
                        let mut bench = true; $(if !$bench { bench = false; })?
                        let Some(input) = &read_input(year, day, &prefix)? else {
                            continue;
                        };

                        $(
                            let result = $day::part_1(input);

                            if result == $p1_exp {
                                info!(target: "aoc", "{prefix} Part 1: {} == {}", result.bold(), $p1_exp.dimmed());
                            } else {
                                warn!(target: "aoc", "{prefix} Part 1: {} != {}", result.bold(), $p1_exp.dimmed());
                            }

                            if bench {
                                info!(target: "aoc", "{}", easybench::bench(|| $day::part_1(input)));
                            }
                        )?

                        $(
                            let result = $day::part_2(input);

                            if result == $p2_exp {
                                info!(target: "aoc", "{prefix} Part 2: {} == {}", result.bold(), $p2_exp.dimmed());
                            } else {
                                warn!(target: "aoc", "{prefix} Part 2: {} != {}", result.bold(), $p2_exp.dimmed());
                            }

                            if bench {
                                info!(target: "aoc", "{}", easybench::bench(|| $day::part_2(input)));
                            }
                        )?
                    })*

                    _ => return Err(eyre::Report::msg("day hasn't been implemented")),
                }

                eprintln!();
            }

            Ok(())
        }

        fn read_input(year: &str, day: &str, prefix: &impl Display) -> eyre::Result<Option<Vec<u8>>> {
            let path = format!("inputs/{year}/{day}.txt");
            let input = std::fs::read(&path).wrap_err_with(|| format!("Failed to read input file \"{path}\""))?;

            if input.is_empty() {
                error!(target: "aoc", "{prefix} Input file is empty. Skipping!");
            } else if std::str::from_utf8(&input).is_err() {
                error!(target: "aoc", "{prefix} Input file must be valid UTF-8. Skipping!");
            } else if input[input.len() - 1] != b'\n' {
                error!(target: "aoc", "{prefix} Input file must end with a trailing newline. Skipping!");
            } else if &input[input.len() - 2..input.len() - 1] == b"\r\n" {
                error!(target: "aoc", "{prefix} Input file must have LF end-of-line, not CRLF. Skipping!");
            } else {
                return Ok(Some(input));
            }

            Ok(None)
        }

        fn eye_candy_prefix(year: impl Display, day: impl Display) -> impl Display {
            use color_eyre::owo_colors::{OwoColorize as _, Rgb};

            const BG: Rgb = Rgb(16, 16, 35);

            let day = format!("[Day {day}]");
            let year = format!("[Year {year}]");

            let day = day.color(Rgb(255, 255, 100)).on_color(BG);
            let year = year.color(Rgb(2, 183, 5)).on_color(BG);

            format!("{year} {day}").bold().to_string()
        }
    };
}
