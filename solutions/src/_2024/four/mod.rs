use memchr::memchr_iter;

static INPUT: &[u8] = include_bytes!("./input.txt");
const INPUT_LINES: usize = 140; // Unlike other days, this IS the amount of lines in the input.
const ROW_LEN: usize = 140 + 1 /* Account for the LF newline */;

pub fn run() -> super::Runner {
    (Some(part_1), Some(part_2))
}

// Answer: 2562
#[allow(clippy::identity_op)]
fn part_1() -> u32 {
    let mut sum = 0;

    for idx in memchr_iter(b'X', INPUT) {
        let col = idx % ROW_LEN;
        let row = idx / ROW_LEN;

        // e (East) — w (West) — s (South) — n (North)
        let can_go_e = col < ROW_LEN - 3;
        let can_go_w = col > 2;

        let can_go_s = row < INPUT_LINES - 3;
        let can_go_n = row > 2;

        sum += (can_go_e
            && INPUT[idx + 1] == b'M'
            && INPUT[idx + 2] == b'A'
            && INPUT[idx + 3] == b'S') as u32;

        sum += (can_go_w
            && INPUT[idx - 1] == b'M'
            && INPUT[idx - 2] == b'A'
            && INPUT[idx - 3] == b'S') as u32;

        sum += (can_go_s
            && INPUT[idx + (ROW_LEN * 1)] == b'M'
            && INPUT[idx + (ROW_LEN * 2)] == b'A'
            && INPUT[idx + (ROW_LEN * 3)] == b'S') as u32;

        sum += (can_go_n
            && INPUT[idx - (ROW_LEN * 1)] == b'M'
            && INPUT[idx - (ROW_LEN * 2)] == b'A'
            && INPUT[idx - (ROW_LEN * 3)] == b'S') as u32;

        sum += ((can_go_e && can_go_s)
            && INPUT[idx + (1) + (ROW_LEN * 1)] == b'M'
            && INPUT[idx + (2) + (ROW_LEN * 2)] == b'A'
            && INPUT[idx + (3) + (ROW_LEN * 3)] == b'S') as u32;

        sum += ((can_go_e && can_go_n)
            && INPUT[idx + (1) - (ROW_LEN * 1)] == b'M'
            && INPUT[idx + (2) - (ROW_LEN * 2)] == b'A'
            && INPUT[idx + (3) - (ROW_LEN * 3)] == b'S') as u32;

        sum += ((can_go_w && can_go_s)
            && INPUT[idx - (1) + (ROW_LEN * 1)] == b'M'
            && INPUT[idx - (2) + (ROW_LEN * 2)] == b'A'
            && INPUT[idx - (3) + (ROW_LEN * 3)] == b'S') as u32;

        sum += ((can_go_w && can_go_n)
            && INPUT[idx - (1) - (ROW_LEN * 1)] == b'M'
            && INPUT[idx - (2) - (ROW_LEN * 2)] == b'A'
            && INPUT[idx - (3) - (ROW_LEN * 3)] == b'S') as u32;
    }

    sum
}

// Answer: 1902
#[allow(clippy::identity_op)]
fn part_2() -> u32 {
    let mut sum = 0;

    for idx in memchr_iter(b'A', INPUT) {
        let col = idx % ROW_LEN;
        let row = idx / ROW_LEN;

        // e (East) — w (West) — s (South) — n (North)
        let can_go_e = col < ROW_LEN - 1;
        let can_go_w = col > 0;

        let can_go_s = row < INPUT_LINES - 1;
        let can_go_n = row > 0;

        // NW  N  NE
        //  W  +  E
        // SW  S  SE
        macro_rules! get {
            [SE] => { INPUT[idx + 1 + ROW_LEN] };
            [NE] => { INPUT[idx + 1 - ROW_LEN] };
            [SW] => { INPUT[idx - 1 + ROW_LEN] };
            [NW] => { INPUT[idx - 1 - ROW_LEN] };
        }

        // The code below is a branch-less version of this pseudocode:
        //
        // if (NW == M && SE == S || NW == S && SE == M)
        //     if (NE == M && SW == S) || (NE == S && SW == M)
        //          sum += 1

        sum += ((can_go_w && can_go_n && can_go_e && can_go_s)
            && ((get![NW] == b'M' && get![SE] == b'S') || (get![NW] == b'S' && get![SE] == b'M'))
            && ((get![NE] == b'M' && get![SW] == b'S') || (get![NE] == b'S' && get![SW] == b'M')))
            as u32;
    }

    sum
}
