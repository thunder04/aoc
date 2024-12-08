use std::cmp::Ordering;

use specialized_ll::*;

use crate::read_number_lazily;

// static INPUT: &[u8] = b"\
// 47|53\n97|13\n97|61\n97|47\n75|29\n61|13\n75|53
// 29|13\n97|29\n53|29\n61|53\n97|53\n61|29\n47|13
// 75|47\n97|75\n47|61\n75|61\n47|29\n75|13\n53|13

// 75,47,61,53,29\n97,61,53,29,13\n75,29,13
// 75,97,47,61,53\n61,13,29\n97,13,75,29,47\n";
static INPUT: &[u8] = include_bytes!("./input.txt");

const MAX_RULES_PER_NODE: usize = 32;
const BIGGEST_NODE: usize = 99;

pub fn run() -> super::Runner {
    (Some(part_1), Some(part_2))
}

// Answer: 4689
fn part_1() -> u64 {
    let mut sum: u64 = 0;

    parse_lines(|ll, buf, mut expected_rules: u128| {
        if buf.iter().all(|page| {
            // Remove the current page from the rules. A rule like "X | X" doesn't make sense.
            expected_rules &= !(1 << page);

            ll.contains_all_rules(*page, expected_rules)
        }) {
            sum += buf[buf.len() / 2] as u64;
        }
    });

    sum
}

// Answer: 6336
fn part_2() -> u64 {
    let mut sum: u64 = 0;

    parse_lines(|ll, buf, mut expected_rules: u128| {
        for page in &*buf {
            expected_rules &= !(1 << page);

            if !ll.contains_all_rules(*page, expected_rules) {
                buf.sort_by(|a, b| {
                    if ll.contains_all_rules(*a, 1 << b) {
                        Ordering::Less
                    } else if ll.contains_all_rules(*b, 1 << a) {
                        Ordering::Greater
                    } else {
                        Ordering::Equal
                    }
                });

                sum += buf[buf.len() / 2] as u64;

                break;
            }
        }
    });

    sum
}

fn parse_lines(mut cb: impl FnMut(&SpecializedLinkedList, &mut [u8], u128)) {
    let mut ll = SpecializedLinkedList::new();
    let mut buf = Vec::with_capacity(32);
    let mut input = INPUT;

    // Read all rules from input.
    loop {
        let (node, off_a) = read_number_lazily!(input, 0, b'|', {
            input = &input[1..]; // Matching should've failed at the empty newline. Go to the next line.
            break;
        });

        let (rule, off_b) = read_number_lazily!(input, off_a + 1, b'\n', {
            unreachable!("Invalid input");
        });

        debug_assert_ne!(node, 0); // The code doesn't expect 0 integers.
        debug_assert_ne!(rule, 0); // The code doesn't expect 0 integers.

        ll.add_rule(node as u8, rule as u8);
        input = &input[off_b + 1..];
    }

    loop {
        let mut expected_rules = 0_u128;

        // Store comma separated numbers to `buf`.
        loop {
            let (a, off_a) = read_number_lazily!(input, 0, b',' | b'\n', {
                unreachable!("Invalid input");
            });
            let matched_char = input[off_a];

            input = &input[off_a + 1..];
            expected_rules |= 1 << a;
            buf.push(a as u8);

            if matched_char == b'\n' {
                break;
            }
        }

        cb(&ll, &mut buf, expected_rules);

        if input.is_empty() {
            break;
        } else {
            buf.clear();
        }
    }
}

mod specialized_ll {
    use super::BIGGEST_NODE;

    // TODO: After you are done with the day. Store [u8; MAX_RULES_PER_NODE] as a u128 bitmask,
    // since max node number ID can be 100.
    pub struct SpecializedLinkedList([u128; BIGGEST_NODE]);

    impl SpecializedLinkedList {
        #[inline(always)]
        pub const fn new() -> Self {
            Self([0; BIGGEST_NODE])
        }

        /// Add a new rule for node.
        #[inline(always)]
        pub const fn add_rule(&mut self, node: u8, rule: u8) {
            assert!(rule < u128::BITS as u8);

            self.0[node as usize] |= 1 << rule;
        }

        /// Obtain all rules of a node.
        #[inline(always)]
        pub const fn rules_of(&self, node: u8) -> u128 {
            self.0[node as usize]
        }

        /// Ensures a node contains all rules specified by the `rules` bitmask.
        #[inline(always)]
        pub const fn contains_all_rules(&self, node: u8, rules: u128) -> bool {
            (self.rules_of(node) & rules) == rules
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn basic_usage() {
            let mut ll = SpecializedLinkedList::new();

            ll.add_rule(75, 29);
            ll.add_rule(75, 13);
            ll.add_rule(75, 1); // Extra rule.

            ll.add_rule(10, 29);
            ll.add_rule(10, 13);

            assert!(ll.contains_all_rules(75, ll.rules_of(10)));
            assert!(!ll.contains_all_rules(10, ll.rules_of(75)));
        }
    }
}
