use memchr::memmem::Finder;

static INPUT: &[u8] = include_bytes!("./input.txt");

pub fn run() -> super::Runner {
    (Some(part_1), Some(part_2))
}

macro_rules! lazy_read_number {
    ($input: expr, $idx: expr, $off: expr, $last_arm: expr) => {
        match $input[$idx + $off] {
            d1 @ b'0'..=b'9' => match $input[$idx + $off + 1] {
                d2 @ b'0'..=b'9' => match $input[$idx + $off + 2] {
                    d3 @ b'0'..=b'9' => match $input[$idx + $off + 3] {
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
        let (a, next_offset) = lazy_read_number!(INPUT, candidate, 4, b',');
        let (b, _) = lazy_read_number!(INPUT, candidate, next_offset, b')');

        sum += a * b;
    }

    sum
}

// Answer: 74838033
fn part_2() -> u32 {
    const DONT_NEEDLE: &[u8] = b"don't()";
    const MUL_NEEDLE: &[u8] = b"mul(";
    const DO_NEEDLE: &[u8] = b"do()";

    let dont_finder = Finder::new(DONT_NEEDLE);
    let mul_finder = Finder::new(MUL_NEEDLE);
    let do_finder = Finder::new(DO_NEEDLE);

    let mut input = INPUT;
    let mut sum = 0;

    loop {
        let limit_idx = dont_finder.find(input).unwrap_or(input.len());
        let (input_w_instructions, rest_input) = input.split_at(limit_idx);

        for candidate in mul_finder.find_iter(input_w_instructions) {
            let (a, next_offset) = lazy_read_number!(input_w_instructions, candidate, 4, b',');
            let (b, _) = lazy_read_number!(input_w_instructions, candidate, next_offset, b')');

            sum += a * b;
        }

        if let Some(continue_idx) = do_finder.find(rest_input) {
            input = &rest_input[continue_idx + DO_NEEDLE.len()..];
        } else {
            break;
        }
    }

    sum
}
