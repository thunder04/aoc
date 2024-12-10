use memchr::memmem::Finder;

use crate::read_number_lazily;

const DONT_I: &[u8] = b"don't()";
const MUL_I: &[u8] = b"mul(";
const DO_I: &[u8] = b"do()";

pub fn part_1(input: &[u8]) -> i64 {
    let mul_finder = Finder::new(MUL_I);
    let mut sum = 0;

    for idx in mul_finder.find_iter(input) {
        let (a, next_offset) = read_number_lazily!(&input[idx..], MUL_I.len(), b',');
        let (b, _) = read_number_lazily!(&input[idx..], next_offset + 1, b')');

        sum += a * b;
    }

    sum as i64
}

pub fn part_2(input: &[u8]) -> i64 {
    let dont_finder = Finder::new(DONT_I);
    let mul_finder = Finder::new(MUL_I);
    let do_finder = Finder::new(DO_I);

    let mut input = input;
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

    sum as i64
}
