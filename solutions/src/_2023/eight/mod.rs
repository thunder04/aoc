static INPUT: &[u8] = include_bytes!("./input.txt");
const HIGHEST_NODE_ID: usize = hash_node_id(b"ZZZ");
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

    let mut lines = INPUT.split(|&x| x == b'\n');
    let mut nodes = vec![(0_usize, 0_usize); HIGHEST_NODE_ID + 1];

    let directions = lines.next().expect("No directions");

    for line in lines {
        if !line.is_empty() {
            nodes[hash_node_id(&line[0..3])] =
                (hash_node_id(&line[7..10]), hash_node_id(&line[12..15]));
        }
    }

    let mut curr = nodes[hash_node_id(b"AAA")];
    let mut count = 1;

    'outer: loop {
        for &direction in directions {
            let next = match direction {
                RIGHT => curr.1,
                LEFT => curr.0,
                _ => unreachable!("Invalid direction \"{direction}\""),
            };

            if next == HIGHEST_NODE_ID {
                break 'outer count;
            } else {
                curr = nodes[next];
                count += 1;
            }
        }
    }
}

// Answer: ???
fn part_2() -> u32 {
    let mut sum = 0;

    sum
}

const fn hash_node_id(id: &[u8]) -> usize {
    id[2] as usize | ((id[1] as usize) << 8) | ((id[0] as usize) << 16)
}
