use memchr::memchr_iter;

static INPUT: &[u8] = include_bytes!("./input.txt");
const INPUT_LINES: usize = 140; // Unlike other days, this IS the amount of lines in the input.
const ROW_LEN: usize = 140 + 1;

// static INPUT: &[u8] =  b"MMMSXXMASM\nMSAMXMSMSA\nAMXSXMAAMM\nMSAMASMSMX\n\
// XMASAMXAMM\nXXAMMXXAMA\nSMSMSASXSS\nSAXAMASAAA\nMAMMMXMMMM\nMXMXAXMASX\n",
// const INPUT_LINES: usize = 10; // Unlike other days, this IS the amount of lines in the input.
// const ROW_LEN: usize = 10 + 1;

pub fn run() -> super::Runner {
    (Some(part_1), None)
}

// Answer: 2562
#[allow(clippy::identity_op)]
fn part_1() -> u32 {
    let mut sum = 0;

    for idx in memchr_iter(b'X', INPUT) {
        let col = idx % ROW_LEN;
        let row = idx / ROW_LEN;

        // e (East) — w (West) — s (South) — n (North)
        let can_search_e = col < ROW_LEN - 3;
        let can_search_w = col > 2;

        let can_search_s = row < INPUT_LINES - 3;
        let can_search_n = row > 2;

        sum += (can_search_e
            && INPUT[idx + 1] == b'M'
            && INPUT[idx + 2] == b'A'
            && INPUT[idx + 3] == b'S') as u32;

        sum += (can_search_w
            && INPUT[idx - 1] == b'M'
            && INPUT[idx - 2] == b'A'
            && INPUT[idx - 3] == b'S') as u32;

        sum += (can_search_s
            && INPUT[idx + (ROW_LEN * 1)] == b'M'
            && INPUT[idx + (ROW_LEN * 2)] == b'A'
            && INPUT[idx + (ROW_LEN * 3)] == b'S') as u32;

        sum += (can_search_n
            && INPUT[idx - (ROW_LEN * 1)] == b'M'
            && INPUT[idx - (ROW_LEN * 2)] == b'A'
            && INPUT[idx - (ROW_LEN * 3)] == b'S') as u32;

        sum += ((can_search_e && can_search_s)
            && INPUT[idx + (1) + (ROW_LEN * 1)] == b'M'
            && INPUT[idx + (2) + (ROW_LEN * 2)] == b'A'
            && INPUT[idx + (3) + (ROW_LEN * 3)] == b'S') as u32;

        sum += ((can_search_e && can_search_n)
            && INPUT[idx + (1) - (ROW_LEN * 1)] == b'M'
            && INPUT[idx + (2) - (ROW_LEN * 2)] == b'A'
            && INPUT[idx + (3) - (ROW_LEN * 3)] == b'S') as u32;

        sum += ((can_search_w && can_search_s)
            && INPUT[idx - (1) + (ROW_LEN * 1)] == b'M'
            && INPUT[idx - (2) + (ROW_LEN * 2)] == b'A'
            && INPUT[idx - (3) + (ROW_LEN * 3)] == b'S') as u32;

        sum += ((can_search_w && can_search_n)
            && INPUT[idx - (1) - (ROW_LEN * 1)] == b'M'
            && INPUT[idx - (2) - (ROW_LEN * 2)] == b'A'
            && INPUT[idx - (3) - (ROW_LEN * 3)] == b'S') as u32;
    }

    sum
}

// Answer: ???
fn part_2() -> u32 {
    0
}
