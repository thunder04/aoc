use memchr::memmem::Finder;

static INPUT: &[u8] = include_bytes!("./input.txt");

pub fn run() -> super::Runner {
    (Some(part_1), None)
}

macro_rules! lazy_read_number {
    ($idx: expr, $off: expr, $last_arm: expr) => {
        match INPUT[$idx + $off] {
            d1 @ b'0'..=b'9' => match INPUT[$idx + $off + 1] {
                d2 @ b'0'..=b'9' => match INPUT[$idx + $off + 2] {
                    d3 @ b'0'..=b'9' => match INPUT[$idx + $off + 3] {
                        $last_arm => (
                            ((d1 - b'0') as u32 * 100)
                                + ((d2 - b'0') as u32 * 10)
                                + ((d3 - b'0') as u32),
                            $off + 4,
                        ),
                        _ => continue,
                    },

                    $last_arm => (((d1 - b'0') as u32 * 10) + ((d2 - b'0') as u32), $off + 3),
                    _ => continue,
                },

                $last_arm => ((d1 - b'0') as u32, $off + 2),
                _ => continue,
            },

            _ => continue,
        }
    };
}

// Answer: 170807108
fn part_1() -> u32 {
    let finder = Finder::new(b"mul(");
    let mut sum = 0;

    for candidate in finder.find_iter(INPUT) {
        let (a, next_offset) = lazy_read_number!(candidate, 4, b',');
        let (b, _) = lazy_read_number!(candidate, next_offset, b')');

        sum += a * b;
    }

    sum
}

// Answer: ???
fn part_2() -> u32 {
    let mut sum = 0;

    sum
}
