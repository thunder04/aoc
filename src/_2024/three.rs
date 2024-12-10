use core::panic;
use std::ptr::slice_from_raw_parts;

use atoi_simd::parse_any_pos as atoi;
use memchr::memmem::Finder;

const DONT_I: &[u8] = b"don't()";
const MUL_I: &[u8] = b"mul(";
const DO_I: &[u8] = b"do()";

pub fn part_1(input: &[u8]) -> i64 {
    let mut sum = 0;
    let mut input_len = input.len();
    let mut input = input.as_ptr();
    let mul_finder = Finder::new(MUL_I);

    eval_mul(&mul_finder, &mut input, &mut input_len, &mut sum);

    sum as i64
}

pub fn part_2(input: &[u8]) -> i64 {
    let dont_finder = Finder::new(DONT_I);
    let mul_finder = Finder::new(MUL_I);
    let do_finder = Finder::new(DO_I);

    let mut sum = 0;
    let mut input_len = input.len();
    let mut input = input.as_ptr();

    loop {
        let dont_instr = dont_finder
            .find(unsafe { &*slice_from_raw_parts(input, input_len) })
            .unwrap_or(input_len);
        let mut input_w_instr_len = dont_instr;

        eval_mul(&mul_finder, &mut input, &mut input_w_instr_len, &mut sum);

        input = unsafe { input.add(input_w_instr_len) };
        input_len -= input_w_instr_len;

        if let Some(do_instr) = do_finder.find(unsafe { &*slice_from_raw_parts(input, input_len) })
        {
            input = unsafe { input.add(do_instr + DO_I.len()) };
            input_len -= do_instr + DO_I.len();
        } else {
            break;
        }
    }

    sum as i64
}

#[inline(always)]
fn eval_mul(mul_finder: &Finder<'_>, input: &mut *const u8, input_len: &mut usize, sum: &mut u64) {
    while let Some(idx) = mul_finder.find(unsafe { &*slice_from_raw_parts(*input, *input_len) }) {
        *input = unsafe { input.add(idx + MUL_I.len()) };
        *input_len -= idx + MUL_I.len();

        let Ok((a, off)) = atoi::<u64>(unsafe { &*slice_from_raw_parts(*input, *input_len) })
        else {
            continue;
        };

        if unsafe { *input.add(off) } != b',' {
            continue;
        }

        *input = unsafe { input.add(off + 1) };
        *input_len -= off + 1;

        let (b, off) = atoi::<u64>(unsafe { &*slice_from_raw_parts(*input, *input_len) }).unwrap();

        if unsafe { *input.add(off) } != b')' {
            continue;
        }

        *input = unsafe { input.add(off + 1) };
        *input_len -= off + 1;
        *sum += a * b;
    }
}
