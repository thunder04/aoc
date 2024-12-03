use std::simd::{num::SimdUint, u32x64, u8x64};

static INPUT: &[u8] = include_bytes!("./input.txt");
const INPUT_LINES: usize = 10_000; // Optimistic amount of lines in the file.

pub fn run() -> super::Runner {
    (Some(part_1), Some(part_2))
}

// Answer: 2970687
fn part_1() -> u32 {
    let mut list_a = Vec::<u32>::with_capacity(INPUT_LINES);
    let mut list_b = Vec::<u32>::with_capacity(INPUT_LINES);
    let mut sum = 0;

    #[rustfmt::skip]
    read_lines(INPUT, |[(a1, b1), (a2, b2), (a3, b3), (a4, b4)]| {
        list_a.push(a1); list_b.push(b1);
        list_a.push(a2); list_b.push(b2);
        list_a.push(a3); list_b.push(b3);
        list_a.push(a4); list_b.push(b4);
    });

    list_a.sort_unstable();
    list_b.sort_unstable();

    while let (Some(a), Some(b)) = (list_a.pop(), list_b.pop()) {
        sum += a.abs_diff(b);
    }

    sum
}

// Answer: 23963899
fn part_2() -> u32 {
    // Try to use small numbers (e.g. u16) so it can fit in the L{1/2/3} cache.
    let mut counter = vec![0_u16; 99999 /* Biggest 5-digit number. */];
    let mut nums = Vec::<u32>::with_capacity(INPUT_LINES);

    #[rustfmt::skip]
    read_lines(INPUT, |[(a1, b1), (a2, b2), (a3, b3), (a4, b4)]| {
        nums.push(a1); counter[b1 as usize] += 1;
        nums.push(a2); counter[b2 as usize] += 1;
        nums.push(a3); counter[b3 as usize] += 1;
        nums.push(a4); counter[b4 as usize] += 1;
    });

    nums.into_iter()
        .map(|x| x * counter[x as usize] as u32)
        .sum()
}

fn read_lines(mut input: &[u8], mut cb: impl FnMut([(u32, u32); 4])) {
    macro_rules! num_mask {
        (offset: $off: expr) => {
            const {
                let mut array = [0; 64];
                let mut idx = 0;

                while idx < 5 {
                    array[$off + idx] = u32::MAX;
                    idx += 1;
                }

                u32x64::from_array(array)
            }
        };
    }

    const CHUNK_SIZE: usize = 55;
    const NL: u32 = b'\n' as _;
    const Z: u32 = b'0' as _;
    const S: u32 = b' ' as _;

    const E5: u32 = 10_000;
    const E4: u32 = 1_000;
    const E3: u32 = 100;
    const E2: u32 = 10;
    const E1: u32 = 1;

    #[rustfmt::skip]
    const POWERS: u32x64 = u32x64::from_array([
        E5, E4, E3, E2, E1 /* N1 */, 0, 0, 0 /* whitespaces */, E5, E4, E3, E2, E1 /* N2 */, 0 /* newline */,
        E5, E4, E3, E2, E1 /* N3 */, 0, 0, 0 /* whitespaces */, E5, E4, E3, E2, E1 /* N4 */, 0 /* newline */,
        E5, E4, E3, E2, E1 /* N5 */, 0, 0, 0 /* whitespaces */, E5, E4, E3, E2, E1 /* N6 */, 0 /* newline */,
        E5, E4, E3, E2, E1 /* N7 */, 0, 0, 0 /* whitespaces */, E5, E4, E3, E2, E1 /* N8 */, 0 /* newline */,
        0, 0, 0, 0, 0, 0, 0, 0,
    ]);

    #[rustfmt::skip]
    const OFFSETS: u32x64 = u32x64::from_array([
        Z, Z, Z, Z, Z /* N1 */, S, S, S /* whitespaces */, Z, Z, Z, Z, Z /* N2 */, NL /* newline */,
        Z, Z, Z, Z, Z /* N3 */, S, S, S /* whitespaces */, Z, Z, Z, Z, Z /* N4 */, NL /* newline */,
        Z, Z, Z, Z, Z /* N5 */, S, S, S /* whitespaces */, Z, Z, Z, Z, Z /* N6 */, NL /* newline */,
        Z, Z, Z, Z, Z /* N7 */, S, S, S /* whitespaces */, Z, Z, Z, Z, Z /* N8 */, NL /* newline */,
        0, 0, 0, 0, 0, 0, 0, 0,
    ]);

    const A1: u32x64 = num_mask!(offset: 0);
    const B1: u32x64 = num_mask!(offset: 8);
    const A2: u32x64 = num_mask!(offset: 14);
    const B2: u32x64 = num_mask!(offset: 22);
    const A3: u32x64 = num_mask!(offset: 28);
    const B3: u32x64 = num_mask!(offset: 36);
    const A4: u32x64 = num_mask!(offset: 42);
    const B4: u32x64 = num_mask!(offset: 50);

    loop {
        let (curr_chunk, next_chunk) = input.split_at(input.len().min(CHUNK_SIZE));
        let lines: u32x64 = u8x64::load_or_default(curr_chunk).cast();
        let lines = (lines - OFFSETS) * POWERS;

        cb([
            ((lines & A1).reduce_sum(), (lines & B1).reduce_sum()),
            ((lines & A2).reduce_sum(), (lines & B2).reduce_sum()),
            ((lines & A3).reduce_sum(), (lines & B3).reduce_sum()),
            ((lines & A4).reduce_sum(), (lines & B4).reduce_sum()),
        ]);

        match (next_chunk.first(), next_chunk.get(1)) {
            (Some(b'\n'), Some(_)) => input = &next_chunk[1..],
            (Some(b'\n'), None) => break,
            (None, _) => break,
            _ => unreachable!("Invalid input"),
        }
    }
}
