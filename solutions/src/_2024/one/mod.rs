use std::simd::{num::SimdUint, u32x16, u8x16};

static INPUT: &[u8] = include_bytes!("./input.txt");

pub fn run() -> super::Runner {
    (Some(part_1), Some(part_2))
}

// Answer: 2970687
fn part_1() -> u32 {
    let mut list_a = Vec::with_capacity(10000 /* Optimistic amount of lines in the file. */);
    let mut list_b = Vec::with_capacity(10000 /* Optimistic amount of lines in the file. */);
    let mut sum = 0;

    read_lines(|(a1,), (b1,)| {
        list_a.push(a1);
        list_b.push(b1);
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
    let mut nums =
        Vec::<u32>::with_capacity(10_000 /* Optimistic amount of lines in the file. */);

    read_lines(|(a1,), (b1,)| {
        counter[b1 as usize] += 1;
        nums.push(a1);
    });

    nums.into_iter()
        .map(|x| x * counter[x as usize] as u32)
        .sum()
}

fn read_lines(mut cb: impl FnMut((u32,), (u32,))) {
    // NOTE: Technically I could read 4 lines at once with SIMD, but I CBA to deal with line endings
    // on Windows.

    /// The length of a single line.
    const LINE_LEN: usize = 5 + 3 + 5;

    #[rustfmt::skip]
    const NUMBER_A1: u32x16 = u32x16::from_array([
        u32::MAX, u32::MAX, u32::MAX, u32::MAX, u32::MAX,
        0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0,
    ]);

    #[rustfmt::skip]
    const NUMBER_B1: u32x16 = u32x16::from_array([
        0, 0, 0, 0, 0,
        0, 0, 0,
        u32::MAX, u32::MAX, u32::MAX, u32::MAX, u32::MAX,
        0, 0, 0,
    ]);

    // 0x30: Zero ASCII character.
    const OFFSETS: u32x16 = u32x16::from_array([
        0x30, 0x30, 0x30, 0x30, 0x30, // number a
        0, 0, 0, // whitespaces
        0x30, 0x30, 0x30, 0x30, 0x30, //number b
        0, 0, 0,
    ]);

    const POWERS: u32x16 = u32x16::from_array([
        10_000, 1_000, 100, 10, 1, // number a
        0, 0, 0, // whitespaces
        10_000, 1_000, 100, 10, 1, // number b
        0, 0, 0,
    ]);

    let mut input = INPUT;

    loop {
        let chunk: u32x16 = u8x16::load_or_default(&input[0..LINE_LEN]).cast();
        let numbers = (chunk - OFFSETS) * POWERS;

        cb(
            ((numbers & NUMBER_A1).reduce_sum(),),
            ((numbers & NUMBER_B1).reduce_sum(),),
        );

        input = &input[LINE_LEN..];
        match (input.first(), input.get(1), input.get(2)) {
            (Some(b'\r'), Some(b'\n'), Some(_)) => input = &input[2..],
            (Some(b'\r'), Some(b'\n'), None) => break,

            (Some(b'\n'), Some(_), _) => input = &input[1..],
            (Some(b'\n'), None, _) => break,

            (None, _, _) => break,
            _ => {}
        }
    }
}
