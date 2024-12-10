use std::{ptr::slice_from_raw_parts, simd::prelude::*};

use atoi_simd::parse_any_pos as atoi;

const INPUT_LINES: usize = 1_000;

pub fn part_1(input: &[u8]) -> i64 {
    let mut list_a = Vec::<u32>::with_capacity(INPUT_LINES);
    let mut list_b = Vec::<u32>::with_capacity(INPUT_LINES);
    let mut sum = 0_i64;

    let mut input_len = input.len();
    let mut input = input.as_ptr();

    loop {
        let Ok((a, _)) = atoi::<u32>(unsafe { &*slice_from_raw_parts(input, input_len) }) else {
            break;
        };

        input = unsafe { input.add(5 + 3) };
        input_len -= 5 + 3;

        let (b, _) = atoi::<u32>(unsafe { &*slice_from_raw_parts(input, input_len) }).unwrap();

        input = unsafe { input.add(5 + 1) };
        input_len -= 5 + 1;

        list_a.push(a);
        list_b.push(b);
    }

    list_a.sort_unstable();
    list_b.sort_unstable();

    let mut idx = 0;

    while idx + 64 < list_a.len() {
        let list_a: i32x64 =
            u32x64::from_slice(unsafe { list_a.get_unchecked(idx..idx + 64) }).cast();
        let list_b: i32x64 =
            u32x64::from_slice(unsafe { list_b.get_unchecked(idx..idx + 64) }).cast();

        sum += (list_a - list_b).abs().reduce_sum() as i64;
        idx += 64;
    }

    while idx < list_a.len() {
        sum += unsafe {
            list_a
                .get_unchecked(idx)
                .abs_diff(*list_b.get_unchecked(idx))
        } as i64;

        idx += 1;
    }

    sum
}

pub fn part_2(input: &[u8]) -> i64 {
    let mut counter = vec![0_u8; 99999 /* Biggest 5-digit number. */];
    let mut nums = Vec::<u32>::with_capacity(INPUT_LINES);

    let mut input_len = input.len();
    let mut input = input.as_ptr();

    loop {
        let Ok((a, _)) = atoi::<u32>(unsafe { &*slice_from_raw_parts(input, input_len) }) else {
            break;
        };

        input = unsafe { input.add(5 + 3) };
        input_len -= 5 + 3;

        let (b, _) = atoi::<u32>(unsafe { &*slice_from_raw_parts(input, input_len) }).unwrap();

        input = unsafe { input.add(5 + 1) };
        input_len -= 5 + 1;

        counter[b as usize] += 1;
        nums.push(a);
    }

    nums.into_iter()
        .map(|x| x * unsafe { *counter.get_unchecked(x as usize) as u32 })
        .sum::<u32>() as i64
}
