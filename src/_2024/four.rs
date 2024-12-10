#![allow(clippy::identity_op)]

use memchr::memchr_iter;

const COL_LEN: usize = 140; // Unlike other days, this IS the amount of lines in the input.
const ROW_LEN: usize = 140 + 1 /* Account for the trailing newline */;

pub fn part_1(input: &[u8]) -> i64 {
    let mut sum = 0;

    for idx in memchr_iter(b'X', input) {
        let col = idx % ROW_LEN;
        let row = idx / ROW_LEN;

        // e (East) — w (West) — s (South) — n (North)
        let can_go_e = col < ROW_LEN - 3;
        let can_go_w = col > 2;

        let can_go_s = row < COL_LEN - 3;
        let can_go_n = row > 2;

        sum += (can_go_e
            && input[idx + 1] == b'M'
            && input[idx + 2] == b'A'
            && input[idx + 3] == b'S') as i64;

        sum += (can_go_w
            && input[idx - 1] == b'M'
            && input[idx - 2] == b'A'
            && input[idx - 3] == b'S') as i64;

        sum += (can_go_s
            && input[idx + (ROW_LEN * 1)] == b'M'
            && input[idx + (ROW_LEN * 2)] == b'A'
            && input[idx + (ROW_LEN * 3)] == b'S') as i64;

        sum += (can_go_n
            && input[idx - (ROW_LEN * 1)] == b'M'
            && input[idx - (ROW_LEN * 2)] == b'A'
            && input[idx - (ROW_LEN * 3)] == b'S') as i64;

        sum += ((can_go_e && can_go_s)
            && input[idx + (1) + (ROW_LEN * 1)] == b'M'
            && input[idx + (2) + (ROW_LEN * 2)] == b'A'
            && input[idx + (3) + (ROW_LEN * 3)] == b'S') as i64;

        sum += ((can_go_e && can_go_n)
            && input[idx + (1) - (ROW_LEN * 1)] == b'M'
            && input[idx + (2) - (ROW_LEN * 2)] == b'A'
            && input[idx + (3) - (ROW_LEN * 3)] == b'S') as i64;

        sum += ((can_go_w && can_go_s)
            && input[idx - (1) + (ROW_LEN * 1)] == b'M'
            && input[idx - (2) + (ROW_LEN * 2)] == b'A'
            && input[idx - (3) + (ROW_LEN * 3)] == b'S') as i64;

        sum += ((can_go_w && can_go_n)
            && input[idx - (1) - (ROW_LEN * 1)] == b'M'
            && input[idx - (2) - (ROW_LEN * 2)] == b'A'
            && input[idx - (3) - (ROW_LEN * 3)] == b'S') as i64;
    }

    sum
}

pub fn part_2(input: &[u8]) -> i64 {
    let mut sum = 0;

    for idx in memchr_iter(b'A', input) {
        let col = idx % ROW_LEN;
        let row = idx / ROW_LEN;

        // e (East) — w (West) — s (South) — n (North)
        let can_go_e = col < ROW_LEN - 1;
        let can_go_w = col > 0;

        let can_go_s = row < COL_LEN - 1;
        let can_go_n = row > 0;

        // NW  N  NE
        //  W  +  E
        // SW  S  SE
        macro_rules! get {
            [SE] => { input[idx + 1 + ROW_LEN] };
            [NE] => { input[idx + 1 - ROW_LEN] };
            [SW] => { input[idx - 1 + ROW_LEN] };
            [NW] => { input[idx - 1 - ROW_LEN] };
        }

        // The code below is a branch-less version of this pseudocode:
        //
        // if (NW == M && SE == S || NW == S && SE == M)
        //     if (NE == M && SW == S) || (NE == S && SW == M)
        //          sum += 1

        sum += ((can_go_w && can_go_n && can_go_e && can_go_s)
            && ((get![NW] == b'M' && get![SE] == b'S') || (get![NW] == b'S' && get![SE] == b'M'))
            && ((get![NE] == b'M' && get![SW] == b'S') || (get![NE] == b'S' && get![SW] == b'M')))
            as i64;
    }

    sum
}
