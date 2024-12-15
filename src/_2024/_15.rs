use std::{
    arch::x86_64::{__m512i, _mm512_cmpeq_epu8_mask},
    num::NonZeroU8,
    simd::u8x64,
};

const MAP_HEIGHT: usize = 8;
// const MAP_HEIGHT: usize = 50;
const MAP_WIDTH: usize = MAP_HEIGHT;

pub fn part_1(mut input: &[u8]) -> i64 {
    let Map {
        walls,
        mut boxes,
        robot: Point {
            x: ref mut rx,
            y: ref mut ry,
        },
    } = read_map(&mut input);

    print_map(&Map {
        robot: Point { x: *rx, y: *ry },
        walls,
        boxes,
    });

    read_moves(input, |Move { direction, times }| {
        // Loop for simplicity. Try to replace it, if you can figure out something better.
        for _ in 0..times.get() {
            match direction {
                b'>' => {}

                b'<' => {
                    let bry = &mut boxes[*ry as usize]; // ry's boxes.
                    let wy = walls[*ry as usize]; // ry's walls.

                    debug!(target: "boxes", ?ry, ?rx, "{bry:0>MAP_WIDTH$b}");
                    debug!(target: "walls", ?ry, ?rx, "{wy:0>MAP_WIDTH$b}");

                    let after_robot = *rx + 1;
                    let bf = *bry; // >> after_robot; // Boxes in Front of the robot.
                    let wf = wy >> after_robot; // Walls in Front of the robot.

                    debug!(target: "   bf", ?ry, ?rx, "{bf:0>MAP_WIDTH$b}");
                    debug!(target: "   wf", ?ry, ?rx, "{wf:0>MAP_WIDTH$b}");

                    debug!(?after_robot);

                    if wf == 0 {
                        // Bot is at the edge.
                        return;
                    } else if bf & 0b1 != 0 {
                        // There's a box in front of the robot. Figure out if you can move forwards.
                        let distance_to_1st_wall = wf.trailing_zeros();
                        let free_space = (1 << distance_to_1st_wall) - 1;
                        let try_to_move = (bf << 1) & free_space;

                        debug!(target: "   fr", ?ry, ?rx, "{free_space:0>MAP_WIDTH$b}");
                        debug!(target: "  tom", ?ry, ?rx, "{try_to_move:0>MAP_WIDTH$b}");

                        if try_to_move >> 1 == bf & free_space {
                            // No boxes would hit the next wall if we were to move them.
                            *bry |= try_to_move;
                            *bry &= !(1 << (*rx - 1));
                        } else {
                            print_map(&Map {
                                robot: Point { x: *rx, y: *ry },
                                boxes,
                                walls,
                            });

                            // If it can't move the boxes, it also can't move itself.
                            return;
                        }
                    } else {
                        // Free space, move forwards.
                    }

                    *rx -= 1;

                    print_map(&Map {
                        robot: Point { x: *rx, y: *ry },
                        boxes,
                        walls,
                    });
                }

                b'^' => {}

                b'v' => {}

                _ => unsafe { std::hint::unreachable_unchecked() },
            }
        }
    });

    sum_gps(boxes)
}

pub fn part_2(input: &[u8]) -> i64 {
    0
}

fn print_map(
    Map {
        walls,
        boxes,
        robot,
    }: &Map,
) {
    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            print!(
                "{}",
                if walls[y] & (1 << x) == 1 << x {
                    '#'
                } else if boxes[y] & (1 << x) == 1 << x {
                    'O'
                } else if robot.x == x as u16 && robot.y == y as u16 {
                    '@'
                } else {
                    '.'
                }
            );
        }

        println!();
    }

    println!();
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: u16,
    y: u16,
}

struct Move {
    direction: u8,
    /// The amount of times the robot will perform this move.
    /// Will be at least 1.
    times: NonZeroU8,
}

struct Map {
    // We know the input's map width is less than 64.
    walls: [u64; MAP_HEIGHT],
    // Store the positions in a bitfield for fast shifting.
    boxes: [u64; MAP_HEIGHT],
    robot: Point,
}

fn read_map(input: &mut &[u8]) -> Map {
    let mut map_y = 0;
    let mut map = Map {
        robot: Point { x: 0, y: 0 },
        walls: [0; MAP_HEIGHT],
        boxes: [0; MAP_HEIGHT],
    };

    loop {
        let chunk: __m512i = u8x64::load_or_default(input).into();
        // Find all newlines of this chunk.
        let mut nl_pos = unsafe { _mm512_cmpeq_epu8_mask(chunk, u8x64::splat(b'\n').into()) };

        // Used to isolate the current row.
        let zeros = nl_pos.trailing_zeros();
        nl_pos >>= zeros;
        let ones = nl_pos.trailing_ones();

        // Zero out elements that don't belong to this row.
        let mask = (1 << (zeros + ones)) - 1;
        let robot_mask = unsafe { _mm512_cmpeq_epu8_mask(chunk, u8x64::splat(b'@').into()) } & mask;

        map.walls[map_y] =
            unsafe { _mm512_cmpeq_epu8_mask(chunk, u8x64::splat(b'#').into()) } & mask;
        map.boxes[map_y] =
            unsafe { _mm512_cmpeq_epu8_mask(chunk, u8x64::splat(b'O').into()) } & mask;

        if robot_mask != 0 {
            map.robot.x = robot_mask.trailing_zeros() as u16;
            map.robot.y = map_y as u16;
        }

        if input.len() < (zeros + ones) as usize {
            break;
        } else {
            // Process the input row-by-row to simplify the code. The perf. regression becomes
            // less noticable as bigger maps are used.
            *input = unsafe { input.get_unchecked((zeros + ones) as usize..) };
            map_y += 1;

            // Double newline.
            if ones == 2 {
                break;
            }
        }
    }

    map
}

fn read_moves(input: &[u8], mut on_move: impl FnMut(Move)) {
    let mut off = 0;

    loop {
        let mut times = unsafe { NonZeroU8::new_unchecked(1) };
        let ch = input[off];
        let mut c;

        loop {
            off += 1;
            c = input.get(off);

            match c {
                Some(b'\n') => {}
                Some(&c) if ch == c => times = unsafe { NonZeroU8::new_unchecked(times.get() + 1) },
                _ => break,
            }
        }

        on_move(Move {
            direction: ch,
            times,
        });

        if c.is_none() {
            break;
        }
    }
}

fn sum_gps(boxes: [u64; MAP_HEIGHT]) -> i64 {
    let mut sum = 0;

    (0..MAP_HEIGHT).for_each(|y| {
        let mut off = MAP_WIDTH as u32 - 1;
        let mut row: u64 = boxes[y];

        row <<= u64::BITS - MAP_WIDTH as u32;

        // TODO: Try to multiply *once* 100 by bitmask.counting_ones(), for each y
        // After you finish the solution.

        while row != 0 {
            let zeros = row.leading_zeros();
            row <<= zeros;
            let ones = row.leading_ones();
            row <<= ones;

            off -= zeros + ones;
            sum += 100 * y as u32 * ones + solve_series(ones, off);
        }
    });

    sum as i64
}

/// Solves the series `Σ_{i=1}^{n} (c + i)`.
#[inline(always)]
const fn solve_series(n: u32, c: u32) -> u32 {
    (c * n) + ((n * (n + 1)) / 2)
}
