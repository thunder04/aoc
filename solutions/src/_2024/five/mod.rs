use specialized_ll::*;

use crate::read_number_lazily;

// static INPUT: &[u8] = b"\
// 47|53
// 97|13
// 97|61
// 97|47
// 75|29
// 61|13
// 75|53
// 29|13
// 97|29
// 53|29
// 61|53
// 97|53
// 61|29
// 47|13
// 75|47
// 97|75
// 47|61
// 75|61
// 47|29
// 75|13
// 53|13
//
// 75,47,61,53,29
// 97,61,53,29,13
// 75,29,13
// 75,97,47,61,53
// 61,13,29
// 97,13,75,29,47\n";
static INPUT: &[u8] = include_bytes!("./input.txt");

const MAX_RULES_PER_NODE: usize = 32;
const BIGGEST_NODE: usize = 99;

pub fn run() -> super::Runner {
    (Some(part_1), None)
}

// Answer: 4689
fn part_1() -> u32 {
    let mut ll = SpecializedLinkedList::<BIGGEST_NODE>::new();
    let mut input = INPUT;
    let mut sum: u32 = 0;

    // Read all rules from input.
    #[allow(clippy::while_let_loop)]
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

    let mut buf = Vec::with_capacity(32);

    loop {
        let mut expected_rules = 0_u128;

        // Store comma separated numbers to `buf`.
        #[allow(clippy::while_let_loop)]
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

        if buf.iter().all(|page| {
            // Remove the current page from the rules. A rule like "X | X" doesn't make sense.
            expected_rules &= !(1 << page);

            ll.contains_all_rules(*page, expected_rules)
        }) {
            sum += buf[buf.len() / 2] as u32;
        }

        if input.is_empty() {
            break;
        } else {
            buf.clear();
        }
    }

    sum
}

// Answer: ???
fn part_2() -> u32 {
    0
}

mod specialized_ll {
    // TODO: After you are done with the day. Store [u8; MAX_RULES_PER_NODE] as a u128 bitmask,
    // since max node number ID can be 100.
    pub struct SpecializedLinkedList<const SIZE: usize>([u128; SIZE]);

    impl<const SIZE: usize> SpecializedLinkedList<SIZE> {
        #[inline(always)]
        pub const fn new() -> Self {
            Self([0; SIZE])
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
            let mut ll = SpecializedLinkedList::<100>::new();

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
