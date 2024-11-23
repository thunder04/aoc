use memchr::memmem::{Finder, FinderRev};

static INPUT: &[u8] = include_bytes!("./input.txt");

pub fn run() -> super::Runner {
    (Some(part_1), Some(part_2))
}

// Answer: 54304
fn part_1() -> u32 {
    let mut input = INPUT;
    let mut sum = 0_u32;

    'outer: loop {
        let first_digit;
        let mut second_digit;

        match input.iter().enumerate().find(|(_, b)| b.is_ascii_digit()) {
            Some((idx, digit)) => {
                first_digit = digit - b'0';
                second_digit = first_digit;
                input = &input[idx + 1..];
            }

            None => break sum,
        }

        loop {
            match input
                .iter()
                .enumerate()
                .find(|(_, b)| b.is_ascii_digit() || **b == b'\n')
            {
                Some((idx, b'\n')) => {
                    sum += (first_digit * 10 + second_digit) as u32;
                    input = &input[idx + 1..];

                    break;
                }

                Some((idx, digit)) => {
                    input = &input[idx + 1..];
                    second_digit = digit - b'0';
                }

                None => {
                    sum += (first_digit * 10 + second_digit) as u32;

                    break 'outer sum;
                }
            }
        }
    }
}

// I find the part 2's code a tiny bit inefficient, I should probably optimize it when I'm bored.

// Answer: 54418
fn part_2() -> u32 {
    let mut input = INPUT;
    let mut sum = 0;

    // The index corresponds to the digit found.
    let finders: [&[Finder]; 10] = [
        &[],
        &[Finder::new(b"1"), Finder::new(b"one")],
        &[Finder::new(b"2"), Finder::new(b"two")],
        &[Finder::new(b"3"), Finder::new(b"three")],
        &[Finder::new(b"4"), Finder::new(b"four")],
        &[Finder::new(b"5"), Finder::new(b"five")],
        &[Finder::new(b"6"), Finder::new(b"six")],
        &[Finder::new(b"7"), Finder::new(b"seven")],
        &[Finder::new(b"8"), Finder::new(b"eight")],
        &[Finder::new(b"9"), Finder::new(b"nine")],
    ];
    // The index corresponds to the digit found.
    let finders_rev: [&[FinderRev]; 10] = [
        &[],
        &[FinderRev::new(b"1"), FinderRev::new(b"one")],
        &[FinderRev::new(b"2"), FinderRev::new(b"two")],
        &[FinderRev::new(b"3"), FinderRev::new(b"three")],
        &[FinderRev::new(b"4"), FinderRev::new(b"four")],
        &[FinderRev::new(b"5"), FinderRev::new(b"five")],
        &[FinderRev::new(b"6"), FinderRev::new(b"six")],
        &[FinderRev::new(b"7"), FinderRev::new(b"seven")],
        &[FinderRev::new(b"8"), FinderRev::new(b"eight")],
        &[FinderRev::new(b"9"), FinderRev::new(b"nine")],
    ];

    while let Some(nl_idx) = memchr::memchr(b'\n', input) {
        let Some((mut line, new_input)) = input.split_at_checked(nl_idx + 1) else {
            break;
        };

        // Select the smallest index of which a digit matches.
        let Some((first_digit, digit_idx)) = finders
            .iter()
            .enumerate() // Get the matched digit as well.
            .skip(1) // Skip the empty array.
            .filter_map(|(digit, finders)| {
                // Test each finder for each digit.
                let digit_idx = finders
                    .iter()
                    .filter_map(|finder| finder.find(line))
                    .min()?;

                Some((digit, digit_idx))
            })
            .min_by(|(_, a_digit_idx), (_, b_digit_idx)| a_digit_idx.cmp(b_digit_idx))
        else {
            break;
        };

        line = line.split_at(digit_idx).1;

        // Select the smallest index of which a digit matches.
        let Some((second_digit, _)) = finders_rev
            .iter()
            .enumerate() // Get the matched digit as well.
            .skip(1) // Skip the empty array.
            .filter_map(|(digit, finders)| {
                // Test each finder for each digit.
                let digit_idx = finders
                    .iter()
                    .filter_map(|finder| finder.rfind(line))
                    .max()?;

                Some((digit, digit_idx))
            })
            .max_by(|(_, a_digit_idx), (_, b_digit_idx)| a_digit_idx.cmp(b_digit_idx))
        else {
            break;
        };

        sum += (first_digit * 10 + second_digit) as u32;
        input = new_input;
    }

    sum
}
