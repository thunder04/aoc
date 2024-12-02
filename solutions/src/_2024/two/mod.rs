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

// TODO: Reimplement this with a manual approach. Skipping a level should be easier there

// Answer: ???
fn part_2() -> u32 {
    b"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"
        .split(|&x| x == b'\n')
        .filter(|line| {
            if !line.is_empty() {
                let bad_levels = line
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
                    .filter(|valid| !valid)
                    .count();

                bad_levels == 0 || bad_levels == 1
            } else {
                false
            }
        })
        .count() as u32
}
