use memchr::{memchr, memchr_iter};

static EX_INPUT: &[u8] =
b"....#.....\n.........#\n..........\n..#.......\n.......#..\n..........\n.#..^.....\n........#.\n#.........\n......#...\n";
const EX_LINE_LENGTH: usize = "....#.....\n".len();

static INPUT: &[u8] = include_bytes!("./input.txt");
const LINE_LENGTH: usize = 131;
const ROW_LENGTH: usize = LINE_LENGTH - 1;

const DIRECTIONS: [fn(usize) -> Option<usize>; 4] = [
    // North
    |idx| idx.checked_sub(LINE_LENGTH),
    // East
    |idx| idx.checked_add(1),
    // South
    |idx| idx.checked_add(LINE_LENGTH),
    // West
    |idx| idx.checked_sub(1),
];

pub fn run() -> super::Runner {
    (Some(part_1), None)
}

// Answer: 4982
fn part_1() -> u32 {
    let mut idx = memchr(b'^', INPUT).expect("Invalid input");
    let mut distinct_positions = [0_u8; LINE_LENGTH * ROW_LENGTH];
    let mut direction = 0_usize;

    distinct_positions[idx] = 1;

    loop {
        let Some(next_idx) = DIRECTIONS[direction](idx) else {
            break memchr_iter(1, &distinct_positions).count() as u32;
        };

        match INPUT[next_idx] {
            b'#' => {
                direction = (direction + 1) % DIRECTIONS.len();
            }

            b'\n' => {
                break memchr_iter(1, &distinct_positions).count() as u32;
            }

            _ => {
                idx = next_idx;
                distinct_positions[idx] = 1;
            }
        }
    }
}

// Answer: ???
fn part_2() -> u32 {
    0
}
