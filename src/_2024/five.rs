use std::cmp::Ordering;

use atoi_simd::parse_any_pos as atoi;

type Rules = [u128; BIGGEST_NODE];

const BIGGEST_NODE: usize = 99;

pub fn part_1(input: &[u8]) -> i64 {
    let mut sum = 0;

    parse_lines(input, |rules, buf, mut expected_rules: u128| {
        if buf.iter().all(|page| {
            // Remove the current page from the rules.
            // A rule like "X | X" doesn't make sense.
            expected_rules &= !(1 << page);

            contains_all_rules(rules, *page, expected_rules)
        }) {
            sum += buf[buf.len() / 2] as i64;
        }
    });

    sum
}

// Answer: 6336
pub fn part_2(input: &[u8]) -> i64 {
    let mut sum = 0;

    parse_lines(input, |rules, buf, mut expected_rules: u128| {
        for page in &*buf {
            expected_rules &= !(1 << page);

            if !contains_all_rules(rules, *page, expected_rules) {
                buf.sort_by(|a, b| {
                    if contains_all_rules(rules, *a, 1 << b) {
                        Ordering::Less
                    } else if contains_all_rules(rules, *b, 1 << a) {
                        Ordering::Greater
                    } else {
                        Ordering::Equal
                    }
                });

                sum += buf[buf.len() / 2] as i64;

                break;
            }
        }
    });

    sum
}

fn parse_lines(mut input: &[u8], mut cb: impl FnMut(&Rules, &mut [u8], u128)) {
    let mut rules: Rules = [0_u128; BIGGEST_NODE];
    let mut buf = Vec::with_capacity(32);

    // Read all rules from input.
    loop {
        // let node =
        //     unsafe { input.get_unchecked(0) & 0xf } * 10 + unsafe { input.get_unchecked(1) & 0xf
        // };

        // Why do I even check this? A pipe should be next.
        // if unsafe { *input.get_unchecked(2) } == b'\n' {
        //     input = unsafe { input.get_unchecked(1..) };
        //     break;
        // }

        let Ok((node, _)) = atoi::<u8>(input) else {
            input = unsafe { input.get_unchecked(1..) };

            break;
        };

        input = unsafe { input.get_unchecked(3..) };

        let (rule, _) = atoi::<u8>(input).unwrap();
        // let rule =
        // unsafe { input.get_unchecked(0) & 0xf } * 10 + unsafe { input.get_unchecked(1) & 0xf };

        debug_assert!(rule < BIGGEST_NODE as u8);
        debug_assert_ne!(node, 0);
        debug_assert_ne!(rule, 0);

        input = unsafe { input.get_unchecked(3..) };
        rules[node as usize] |= 1 << rule;
    }

    loop {
        let mut expected_rules = 0_u128;

        // Store comma separated numbers to `buf`.
        loop {
            let (rule, off) = atoi::<u8>(input).unwrap();
            let matched = input[off];

            expected_rules |= 1 << rule;
            input = &input[off + 1..];
            buf.push(rule);

            if matched == b'\n' {
                break;
            }
        }

        cb(&rules, &mut buf, expected_rules);

        if input.is_empty() {
            break;
        } else {
            buf.clear();
        }
    }
}

#[inline(always)]
const fn contains_all_rules(rules: &Rules, node: u8, expected: u128) -> bool {
    (rules[node as usize] & expected) == expected
}
