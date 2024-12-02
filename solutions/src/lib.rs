#![feature(iter_map_windows)]
#![feature(portable_simd)]

#[macro_use]
extern crate tracing;

pub mod _2023;
pub mod _2024;

pub type Runner = (Option<Solution>, Option<Solution>);
pub type Solution = fn() -> u32;

#[macro_export]
macro_rules! export_days {
    ($($id: literal: $name: literal, $day_mod: ident)*) => {
        pub use $crate::{Runner, Solution};
        use color_eyre::Section;

        $(pub mod $day_mod;)*

        pub fn run(day: u8) -> eyre::Result<()> {
            const AVAILABLE_DAYS: &[&'static str] = &[$($name),*];

            match day {
                $($id => return _run($id, $day_mod::run()),)*

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
