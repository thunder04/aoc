use std::{
    intrinsics::simd::simd_shuffle,
    simd::{num::SimdUint, u32x64, u8x64, usizex64},
};

static INPUT: &[u8] = include_bytes!("./input.txt");
const AAA_NODE_ID: usize = 4276545;
const ZZZ_NODE_ID: usize = 5921370;
const RIGHT: u8 = b'R';
const LEFT: u8 = b'L';

pub fn run() -> super::Runner {
    (Some(part_1), None)
}

// Answer: 23147
fn part_1() -> u32 {
    // TODO Ideas: I don't really like a ~900μs execution time, how about:
    //  - Maintain a queue of "traverse operations" while you fill `nodes`?
    //  - SIMD accelerated line reading?
    //    - Remove invalid lines (last potentially empty line and first two) before looping

    let mut lines = INPUT.splitn(3, |&x| x == b'\n');
    let mut nodes = vec![(0_usize, 0_usize); ZZZ_NODE_ID + 1];
    let directions = lines.next().expect("No directions");

    // Safety: You either live or you don't.
    unsafe {
        read_lines(
            lines.nth(1).expect("Where's the third line?"),
            |[(n1_id, n1), (n2_id, n2), (n3_id, n3)]| {
                nodes[n1_id] = n1;
                nodes[n2_id] = n2;
                nodes[n3_id] = n3;
            },
        );
    }

    let mut curr = nodes[AAA_NODE_ID];
    let mut count = 1;

    'outer: loop {
        for &direction in directions {
            let next = match direction {
                RIGHT => curr.1,
                LEFT => curr.0,
                _ => unreachable!("Invalid direction \"{direction}\""),
            };

            if next == ZZZ_NODE_ID {
                break 'outer count;
            } else {
                curr = nodes[next];
                count += 1;
            }
        }
    }
}

// Answer: ???
pub fn part_2() -> u32 {
    0
}

/// Reads three lines at once from input.
/// # Safety
/// - Input must be perfect. Each line must be of this form: `??? = (???, ???)\n`, where `?` is an
///   ASCII character. A trailing newline is accepted.
/// - Platform must support SIMD instructions. IDK which exactly, but it should, yep ¯\\_(ツ)\_/¯
unsafe fn read_lines(mut input: &[u8], mut cb: impl FnMut([(usize, (usize, usize)); 3])) {
    const CHUNK_SIZE: usize = 50;
    const M: u32 = u32::MAX;

    #[rustfmt::skip]
    const SHL: u32x64 = u32x64::from_array([
        16, 8, 0 /* node */,  0, 0, 0, 0 /* " = (" */, 16, 8, 0 /* L node */, 0, 0 /* ", " */, 16, 8, 0 /* R node */, 0, 0 /* ")\n" */,
        16, 8, 0 /* node */,  0, 0, 0, 0 /* " = (" */, 16, 8, 0 /* L node */, 0, 0 /* ", " */, 16, 8, 0 /* R node */, 0, 0 /* ")\n" */,
        16, 8, 0 /* node */,  0, 0, 0, 0 /* " = (" */, 16, 8, 0 /* L node */, 0, 0 /* ", " */, 16, 8, 0 /* R node */, 0, 0 /* ")\n" */,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 /* filling bytes */
    ]);

    #[rustfmt::skip]
    const MASK: u32x64 = u32x64::from_array([
        M, M, M /* node */,  0, 0, 0, 0 /* " = (" */, M, M, M /* L node */, 0, 0 /* ", " */, M, M, M /* R node */, 0, 0 /* ")\n" */,
        M, M, M /* node */,  0, 0, 0, 0 /* " = (" */, M, M, M /* L node */, 0, 0 /* ", " */, M, M, M /* R node */, 0, 0 /* ")\n" */,
        M, M, M /* node */,  0, 0, 0, 0 /* " = (" */, M, M, M /* L node */, 0, 0 /* ", " */, M, M, M /* R node */, 0, 0 /* ")\n" */,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 /* filling bytes */
    ]);

    #[rustfmt::skip]
    #[allow(clippy::zero_prefixed_literal)]
    const PERMUTE_VECTOR_1: u32x64 = u32x64::from_array([
        01, 02, 03, 04, 05, 06, 07, 08, 09, 10, 11, 12, 13, 14, 15, 16, 00,
        18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 17,
        35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 34,

        51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63,
    ]);

    #[rustfmt::skip]
    #[allow(clippy::zero_prefixed_literal)]
    const PERMUTE_VECTOR_2: u32x64 = u32x64::from_array([
        02, 03, 04, 05, 06, 07, 08, 09, 10, 11, 12, 13, 14, 15, 16, 00, 01,
        19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 17, 18,
        36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 34, 35,

        51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63,
    ]);

    loop {
        let (curr_chunk, next_chunk) = input.split_at(input.len().min(CHUNK_SIZE));

        let lines: u32x64 = u8x64::load_or_default(curr_chunk).cast();
        let lines = (lines & MASK) << SHL;

        // SAFETY:
        //   - "x", "y" and "idx" are vectors according to the module's documentation (entities with
        //     #[repr(simd)]).
        //   - PERMUTE_VECTOR_{1, 2} is a **const** vector of u32's with the same length as "x".
        //   - The return type is the same as "x".
        let rotl_by_1: u32x64 = unsafe { simd_shuffle(lines, u32x64::splat(0), PERMUTE_VECTOR_1) };
        let rotl_by_2: u32x64 = unsafe { simd_shuffle(lines, u32x64::splat(0), PERMUTE_VECTOR_2) };
        let numbers: usizex64 = (lines | rotl_by_1 | rotl_by_2).cast();

        cb([
            (numbers[0], (numbers[7], numbers[12])),
            (numbers[17], (numbers[24], numbers[29])),
            (numbers[34], (numbers[41], numbers[46])),
        ]);

        match (next_chunk.first(), next_chunk.get(1)) {
            (Some(b'\n'), Some(_)) => input = &next_chunk[1..],
            (Some(b'\n'), None) => break,
            (None, _) => break,
            _ => unreachable!("Invalid input"),
        }
    }
}
