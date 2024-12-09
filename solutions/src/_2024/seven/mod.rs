use memchr::memchr;

static EX_INPUT: &[u8] = b"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";
static INPUT: &[u8] = include_bytes!("./input.txt");

pub fn run() -> super::Runner {
    assert_eq!(INPUT[INPUT.len() - 1], b'\n');

    (Some(part_1), None)
}

macro_rules! read_number {
    ($ty: ty, $input: expr, $idx: expr, $end: pat) => {{
        let mut number: $ty = 0;

        loop {
            match *$input.add($idx) {
                end @ $end => {
                    $idx += 1; // Skip whitespace.

                    break (number, end);
                }

                digit => {
                    number = number * 10 + (digit - b'0') as $ty;
                    $idx += 1;
                }
            }
        }
    }};
}

macro_rules! try_end_it {
    ($input_lt: lifetime, $test_value: expr, $x1: expr, $x2: expr, $idx: ident, $numbers: ident, $sum: expr, $on_end: expr) => {
        if $test_value == $x1 || $test_value == $x2 {
            $sum += $test_value;

            match ($on_end)() {
                Some(next_idx) if next_idx >= INPUT.len() => return $sum,
                Some(next_idx) => {
                    $idx = next_idx;
                    $numbers.clear();

                    continue $input_lt;
                }

                None => return $sum,
            }
        }

        $numbers.push($x1);
        $numbers.push($x2);
    };
}

// Answer: 7885693428401
fn part_1() -> u64 {
    let input = INPUT.as_ptr();
    let input_len = INPUT.len();
    let mut idx = 0;
    let mut sum = 0_u64;

    // Even index: addition
    // Odd index: multiplication
    let mut numbers = Vec::with_capacity(1 << (6 - 1));

    unsafe {
        'input: loop {
            let (test_value, _) = read_number!(u64, input, idx, b':');
            idx += 1; // Skip whitespace.

            // Lines have at least two numbers.
            let (a, _) = read_number!(u64, input, idx, b' ');
            let (b, end) = read_number!(u64, input, idx, b' ' | b'\n');
            let x1 = a + b;
            let x2 = a * b;

            try_end_it!('input, test_value, x1, x2, idx, numbers, sum, || (end == b'\n').then_some(idx));

            let mut height = 1_usize;

            if end != b'\n' {
                'line: loop {
                    let (c, end) = read_number!(u64, input, idx, b' ' | b'\n');

                    numbers.reserve(2 << (height - 1));

                    for _ in 0..2 << (height - 1) {
                        let parent = *numbers.as_ptr().add((numbers.len() - 1) / 2);
                        let x1 = parent + c;
                        let x2 = parent * c;

                        try_end_it!('input, test_value, x1, x2, idx, numbers, sum, || match end == b'\n' {
                            false => memchr(b'\n', &INPUT[idx..]).map(|off| idx + off + 1),
                            true => Some(idx),
                        });
                    }

                    if end == b'\n' {
                        break 'line;
                    } else {
                        height += 1;
                    }
                }
            }

            if idx >= input_len {
                break sum;
            } else {
                numbers.clear();
            }
        }
    }
}

// Answer: ???
fn part_2() -> u64 {
    0
}
