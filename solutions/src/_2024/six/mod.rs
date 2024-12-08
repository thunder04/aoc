use memchr::{memchr, memchr_iter};

static INPUT: &[u8] = include_bytes!("./input.txt");
const COL_LENGTH: usize = 131;
const ROW_LENGTH: usize = COL_LENGTH - 1;

const DIRECTIONS: [fn(usize) -> Option<usize>; 4] = [
    // North
    |idx| idx.checked_sub(COL_LENGTH),
    // East
    |idx| idx.checked_add(1),
    // South
    |idx| idx.checked_add(COL_LENGTH),
    // West
    |idx| idx.checked_sub(1),
];

pub fn run() -> super::Runner {
    (Some(part_1), None)
}

// Answer: 4982
fn part_1() -> u64 {
    let mut idx = memchr(b'^', INPUT).expect("Invalid input");
    let mut distinct_positions = [0_u8; COL_LENGTH * ROW_LENGTH];
    let mut direction = 0_usize;

    distinct_positions[idx] = 1;

    loop {
        let Some(next_idx) = DIRECTIONS[direction](idx) else {
            break;
        };

        match INPUT[next_idx] {
            b'#' => direction = (direction + 1) % DIRECTIONS.len(),

            _ => {
                idx = next_idx;
                distinct_positions[idx] = 1;
            }
        }
        
    
    }

    memchr_iter(1, &distinct_positions).count() as u64
}

// Answer: ???
fn part_2() -> u64 {
    0
}
