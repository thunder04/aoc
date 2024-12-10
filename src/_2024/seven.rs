use memchr::memchr;

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
    ($input_lt: lifetime, $input_len: expr, $test_value: expr, $x1: expr, $x2: expr $(; $x3: expr)?, $idx: ident, $numbers: ident, $sum: expr, $on_end: expr) => {
        if $test_value == $x1 || $test_value == $x2 $(|| $test_value == $x3)? {
            $sum += $test_value;

            match ($on_end)() {
                Some(next_idx) if next_idx >= $input_len => return $sum,
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
pub fn part_1(input: &[u8]) -> i64 {
    let input_len = input.len();
    let inp = input.as_ptr();
    let mut idx = 0;
    let mut sum = 0;

    let mut numbers = Vec::with_capacity(1 << (6 - 1));

    unsafe {
        'input: loop {
            let (test_value, _) = read_number!(i64, inp, idx, b':');
            idx += 1; // Skip whitespace.

            // Lines have at least two numbers.
            let (a, _) = read_number!(i64, inp, idx, b' ');
            let (b, end) = read_number!(i64, inp, idx, b' ' | b'\n');
            let x1 = a + b;
            let x2 = a * b;

            try_end_it!('input, input_len, test_value, x1, x2, idx, numbers, sum, || (end == b'\n').then_some(idx));

            let mut height = 1_usize;

            if end != b'\n' {
                'line: loop {
                    let (c, end) = read_number!(i64, inp, idx, b' ' | b'\n');

                    numbers.reserve(2 << (height - 1));

                    for _ in 0..2 << (height - 1) {
                        let parent = *numbers.as_ptr().add((numbers.len() - 1) / 2);
                        let x1 = parent + c;
                        let x2 = parent * c;

                        try_end_it!('input, input_len, test_value, x1, x2, idx, numbers, sum, || match end == b'\n' {
                            false => memchr(b'\n', &input[idx..]).map(|off| idx + off + 1),
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

// pub fn part_2(input: &[u8]) -> i64 {
//     let input = INPUT.as_ptr();
//     let input_len = INPUT.len();
//     let mut idx = 0;
//     let mut sum = 0_i64;

//     let mut numbers = Vec::with_capacity(1 << (6 - 1));
//     let mut strs = Vec::with_capacity(1 << (6 - 1));

//     unsafe {
//         'input: loop {
//             let (test_value, _) = read_number!(i64, input, idx, b':');
//             idx += 1; // Skip whitespace.

//             // Lines have at least two numbers.
//             let (a, _) = read_number!(i64, input, idx, b' ');
//             let (b, end) = read_number!(i64, input, idx, b' ' | b'\n');
//             let x1 = a + b;
//             let x2 = a * b;
//             let x3 = concat(a, b);

//             strs.push(format!("{a} + {b}"));
//             strs.push(format!("{a} * {b}"));
//             strs.push(format!("{a} || {b}"));

//             debug!("{strs:#?}");

//             try_end_it!('input, test_value, x1, x2; x3, idx, numbers, sum, || (end ==
// b'\n').then_some(idx));

//             let mut height = 1_usize;

//             if end != b'\n' {
//                 'line: loop {
//                     let (c, end) = read_number!(i64, input, idx, b' ' | b'\n');

//                     height += 1;
//                     // numbers.reserve(2 << (height - 1));

//                     for _ in 0..3_u32.pow(height as u32) {
//                         let parent = *numbers.as_ptr().add((numbers.len() - 3) / 3);
//                         let s_parent = &*strs.as_ptr().add((numbers.len() - 3) / 3);
//                         let x1 = parent + c;
//                         let x2 = parent * c;
//                         let x3 = concat(parent, c);

//                         let x = [
//                             format!("{s_parent} + {c}"),
//                             format!("{s_parent} * {c}"),
//                             format!("{s_parent} || {c}"),
//                         ];

//                         strs.extend(x);
//                         debug!("{strs:#?}");

//                         try_end_it!('input, test_value, x1, x2; x3, idx, numbers, sum, || match
// end == b'\n' {                             false => memchr(b'\n', &INPUT[idx..]).map(|off| idx +
// off + 1),                             true => Some(idx),
//                         });
//                     }

//                     if end == b'\n' {
//                         break 'line;
//                     }
//                 }
//             }

//             if idx >= input_len {
//                 break sum;
//             } else {
//                 strs.clear();
//                 numbers.clear();
//             }
//         }
//     }
// }

// #[inline]
// fn concat(a: i64, b: i64) -> i64 {
//     a * 10_i64.pow((b as f64).log10().floor() as u32) + b
// }
