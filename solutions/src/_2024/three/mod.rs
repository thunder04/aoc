use memchr::memmem::Finder;

use crate::read_number_lazily;

static INPUT: &[u8] = include_bytes!("./input.txt");
const DONT_I: &[u8] = b"don't()";
const MUL_I: &[u8] = b"mul(";
const DO_I: &[u8] = b"do()";

pub fn run() -> super::Runner {
    (Some(part_1), Some(part_2))
}

// Answer: 170807108
fn part_1() -> u32 {
    let mul_finder = Finder::new(MUL_I);
    let mut sum = 0;

    for idx in mul_finder.find_iter(INPUT) {
        let (a, next_offset) = read_number_lazily!(&INPUT[idx..], MUL_I.len(), b',');
        let (b, _) = read_number_lazily!(&INPUT[idx..], next_offset + 1, b')');

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

        for idx in mul_finder.find_iter(input_w_instr) {
            let (a, next_offset) = read_number_lazily!(&input_w_instr[idx..], MUL_I.len(), b',');
            let (b, _) = read_number_lazily!(&input_w_instr[idx..], next_offset + 1, b')');

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
