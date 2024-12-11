use std::{
    arch::x86_64::{__m512i, _mm512_and_epi32, _mm512_cmpeq_epu8_mask, _mm512_set1_epi8},
    collections::VecDeque,
    simd::u8x64,
};

const ROW_LEN: usize = 53 + 1;
const COL_LEN: usize = 53;

const LEFT: u8 = 1 << 0;
const RIGHT: u8 = 1 << 1;
const TOP: u8 = 1 << 2;
const BOTTOM: u8 = 1 << 3;

pub fn part_1(mut input: &[u8]) -> i64 {
    const ROW_LEN: usize = 8 + 1;
    const COL_LEN: usize = 8;
    let mut input: &[u8] = std::hint::black_box(
        b"\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
",
    );

    let digit_mask: __m512i = u8x64::splat(0xF).into();
    let newline = u8x64::splat(0x0A);

    // Store up to 64 heights¹ per row. We know the lowest four bits correspond to the digits
    // themselves. We know `\n`'s is 0b1010 (0xA, 10); We use that for "out of bounds" checking.
    //
    // The elements have a logical order, that is:
    // - map[0][0..4] is the first element
    // - map[0][4..8] is the second element
    // - ...and so forth.
    //
    // ¹ 128b / 4b = 32 elements * 2 = 64 elements.
    // A line full of newlines will simplify the checks for going up (+ 1).
    // Another line at the end will simplify the checks for going down (+ 2).
    let mut map = [u8x64::splat(0x0A); COL_LEN + 2];
    let mut map_len = 1;
    let mut sum = 0;

    loop {
        let line: __m512i = u8x64::load_or(input, newline).into();
        let line: __m512i = unsafe { _mm512_and_epi32(line, digit_mask) };

        map[map_len] = u8x64::from(line);
        map_len += 1;

        if input.len() < 64 {
            break;
        } else {
            input = &input[64..];
        }
    }

    let mut checked_directions = [[0_u8; 64]; COL_LEN + 2];
    let mut queue = VecDeque::with_capacity(128);
    let mut map_idx = 1;

    while map_idx < map_len {
        let mut zero_idxs =
            unsafe { _mm512_cmpeq_epu8_mask(map[map_idx].into(), _mm512_set1_epi8(0)) };

        // let map = unsafe { transmute::<[u8x64; COL_LEN + 2], [u8; 64 * (COL_LEN + 2)]>(map) };

        while zero_idxs != 0 {
            let digit_idx = zero_idxs.trailing_zeros() as usize;

            queue.push_back((map_idx, digit_idx));
            zero_idxs &= !(1 << digit_idx);

            while let Some((map_idx, digit_idx)) = queue.pop_front() {
                // TODO: You should probably rethink this approach.

                let chck_dirs = checked_directions[map_idx][digit_idx];

                // debug!(target: "new", "digit={}, checked={chck_dirs:0>4b}",
                // map[map_idx][digit_idx],);

                if chck_dirs & LEFT == 0 {
                    let mut next_digit_idx = digit_idx.saturating_sub(1);
                    let mut prev_digit = map[map_idx][digit_idx];

                    checked_directions[map_idx][digit_idx] |= LEFT;

                    loop {
                        let digit = map[map_idx][next_digit_idx];

                        // trace!(target: "left", ?prev_digit, ?digit);

                        if digit == prev_digit + 1 {
                            if digit == 9 {
                                sum += 1;
                                break;
                            }

                            // Skip right because we come from left.
                            checked_directions[map_idx][next_digit_idx] |= RIGHT;
                            queue.push_back((map_idx, next_digit_idx));

                            if next_digit_idx == 0 {
                                break;
                            } else {
                                next_digit_idx -= 1;
                                prev_digit = digit;
                            }
                        } else {
                            break;
                        }
                    }
                }

                if chck_dirs & RIGHT == 0 && digit_idx != 63 {
                    let mut prev_digit = map[map_idx][digit_idx];
                    let mut next_digit_idx = digit_idx + 1;

                    checked_directions[map_idx][digit_idx] |= RIGHT;

                    loop {
                        let digit = map[map_idx][next_digit_idx];

                        // trace!(target: "right", ?prev_digit, ?digit);

                        if digit == prev_digit + 1 {
                            if digit == 9 {
                                sum += 1;
                                break;
                            }

                            // Skip left because we come from right.
                            checked_directions[map_idx][next_digit_idx] |= LEFT;
                            queue.push_back((map_idx, next_digit_idx));

                            // -1 to skip the newline.
                            if next_digit_idx == 63 {
                                break;
                            } else {
                                next_digit_idx += 1;
                                prev_digit = digit;
                            }
                        } else {
                            break;
                        }
                    }
                }

                if chck_dirs & TOP == 0 {
                    let mut prev_digit = map[map_idx][digit_idx];
                    let (mut next_digit_idx, mut next_map_idx) = if digit_idx < ROW_LEN - 1 {
                        (digit_idx, map_idx - 1)
                    } else {
                        (digit_idx - (ROW_LEN - 1), map_idx)
                    };

                    checked_directions[map_idx][digit_idx] |= TOP;

                    loop {
                        let digit = map[next_map_idx][next_digit_idx];

                        // trace!(target: "top", ?prev_digit, ?digit);

                        if digit != 10 && digit == prev_digit + 1 {
                            if digit == 9 {
                                sum += 1;
                                break;
                            }

                            // Skip bottom because we come from top.
                            checked_directions[next_map_idx][next_digit_idx] |= BOTTOM;
                            queue.push_back((next_map_idx, next_digit_idx));

                            if next_digit_idx < ROW_LEN - 1 {
                                next_map_idx -= 1;
                            } else {
                                next_digit_idx -= ROW_LEN - 1;
                            };

                            prev_digit = digit;
                        } else {
                            break;
                        }
                    }
                }

                if chck_dirs & BOTTOM == 0 {
                    let mut prev_digit = map[map_idx][digit_idx];
                    let (mut next_digit_idx, mut next_map_idx) = if digit_idx > ROW_LEN - 1 {
                        (digit_idx, map_idx + 1)
                    } else {
                        (digit_idx + ROW_LEN - 1, map_idx)
                    };

                    checked_directions[map_idx][digit_idx] |= BOTTOM;

                    loop {
                        let digit = map[next_map_idx][next_digit_idx];

                        // trace!(target: "bottom", ?prev_digit, ?digit);

                        if digit != 10 && digit == prev_digit + 1 {
                            if digit == 9 {
                                sum += 1;
                                break;
                            }

                            // Skip top because we come from bottom.
                            checked_directions[next_map_idx][next_digit_idx] |= TOP;
                            queue.push_back((next_map_idx, next_digit_idx));

                            if next_digit_idx > ROW_LEN - 1 {
                                next_map_idx += 1;
                            } else {
                                next_digit_idx += ROW_LEN - 1;
                            };

                            prev_digit = digit;
                        } else {
                            break;
                        }
                    }
                }
            }
        }

        // std::process::abort();

        map_idx += 1;
    }

    sum
}

// const fn element

fn debug_numbers(arr: &[u8]) {
    for digit in arr {
        print!("{} ", digit);
    }

    println!();
}
