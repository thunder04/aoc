use core::{ops::RangeInclusive, str};

static INPUT: &[u8] = include_bytes!("./input.txt");
const ACCEPTABLE_LEVEL_DIFF: RangeInclusive<i8> = 1..=3;

pub fn run() -> super::Runner {
    (Some(part_1), None)
}

// Answer: 213
fn part_1() -> u32 {
    INPUT
        .split(|&x| x == b'\n')
        .filter(|line| {
            !line.is_empty()
                && !line
                    .split(|&x| x == b' ')
                    .filter_map(|num| str::from_utf8(num).unwrap().parse::<u8>().ok())
                    .map_windows(|[a, b]| {
                        let a = *a as i8;
                        let b = *b as i8;

                        b - a
                    })
                    .map_windows(|[diff_a, diff_b]| {
                        ACCEPTABLE_LEVEL_DIFF.contains(&diff_a.abs())
                            && ACCEPTABLE_LEVEL_DIFF.contains(&diff_b.abs())
                            && diff_a.signum() == diff_b.signum()
                    })
                    .any(|valid| !valid)
        })
        .count() as u32
}

// Answer: ???
fn part_2() -> u32 {
    let mut sum = 0;

    sum
}
