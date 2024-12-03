use memchr::memmem::Finder;

static INPUT: &[u8] = include_bytes!("./input.txt");
const DONT_I: &[u8] = b"don't()";
const MUL_I: &[u8] = b"mul(";
const DO_I: &[u8] = b"do()";

pub fn run() -> super::Runner {
    (Some(part_1), Some(part_2))
}

macro_rules! read_number_lazily {
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
    let mul_finder = Finder::new(MUL_I);
    let mut sum = 0;

    for candidate in mul_finder.find_iter(INPUT) {
        let (a, next_offset) = read_number_lazily!(INPUT, candidate, MUL_I.len(), b',');
        let (b, _) = read_number_lazily!(INPUT, candidate, next_offset, b')');

        sum += a * b;
    }

    sum
}

// Answer: 74838033
fn part_2() -> u32 {
    let dont_finder = Finder::new(DONT_I);
    let mul_finder = Finder::new(MUL_I);
    let do_finder = Finder::new(DO_I);

    let mut input = INPUT;
    let mut sum = 0;

    loop {
        let limit_idx = dont_finder.find(input).unwrap_or(input.len());
        let (input_w_instr, rest_input) = input.split_at(limit_idx);

        for candidate in mul_finder.find_iter(input_w_instr) {
            let (a, next_offset) = read_number_lazily!(input_w_instr, candidate, MUL_I.len(), b',');
            let (b, _) = read_number_lazily!(input_w_instr, candidate, next_offset, b')');

            sum += a * b;
        }

        if let Some(continue_idx) = do_finder.find(rest_input) {
            input = &rest_input[continue_idx + DO_I.len()..];
        } else {
            break;
        }
    }

    sum
}
