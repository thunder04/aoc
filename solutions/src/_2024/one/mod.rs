use std::{cmp::Reverse, collections::BinaryHeap, simd::num::SimdUint};

static INPUT: &[u8] = include_bytes!("./input.txt");

const DIGITS: usize = 5;

pub fn run() -> super::Runner {
    (Some(part_1), None)
}

// Answer: 2970687
fn part_1() -> u32 {
    let mut sum = 0;
    let (mut heap_a, mut heap_b): (BinaryHeap<_>, BinaryHeap<_>) = INPUT
        .split(|&x| x == b'\n')
        .filter_map(|line| {
            Some((
                Reverse(parse_digits(line.first_chunk::<DIGITS>()?)),
                Reverse(parse_digits(line.last_chunk::<DIGITS>()?)),
            ))
        })
        .unzip();

    while let (Some(Reverse(a)), Some(Reverse(b))) = (heap_a.pop(), heap_b.pop()) {
        sum += a.abs_diff(b);
    }

    sum
}

// Answer: 54418
fn part_2() -> u32 {
    let input = INPUT;

    0
}

// fn parse_digits<const N: usize>(buf: &[u8; N]) -> u32 {
//     let mut num = 0;

//     for exp in 0..N {
//         // SAFETY: exp is smaller than N, buf's length.
//         num +=
//             unsafe { (*buf.get_unchecked(exp) - b'0') as u32 } * 10_u32.pow((N - 1 - exp) as
// u32);     }

//     num
// }

fn parse_digits(buf: &[u8; DIGITS]) -> u32 {
    use std::simd::u32x8;

    const OFFSETS: u32x8 = u32x8::from_array([0x30, 0x30, 0x30, 0x30, 0x30, 0, 0, 0]); // 0x30 is the character "0" in ASCII
    const POWERS: u32x8 = u32x8::from_array([10_000, 1_000, 100, 10, 1, 0, 0, 0]);

    #[allow(
        clippy::assertions_on_constants,
        reason = "I might forget to check this function"
    )]
    {
        assert!(DIGITS >= 5);
    }

    let digits = u32x8::from_array([
        unsafe { *buf.get_unchecked(0) } as u32,
        unsafe { *buf.get_unchecked(1) } as u32,
        unsafe { *buf.get_unchecked(2) } as u32,
        unsafe { *buf.get_unchecked(3) } as u32,
        unsafe { *buf.get_unchecked(4) } as u32,
        0,
        0,
        0,
    ]);

    ((digits - OFFSETS) * POWERS).reduce_sum()
}
