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

        ll.push_to(node as u8, rule as u8);
        input = &input[off_b + 1..];
    }

    let mut buf = Vec::with_capacity(32);
    let mut sum: u32 = 0;

    loop {
        // Store comma separated numbers to `buf`.
        #[allow(clippy::while_let_loop)]
        loop {
            let (a, off_a) = read_number_lazily!(input, 0, b',' | b'\n', {
                unreachable!("Invalid input");
            });
            let matched_char = input[off_a];

            input = &input[off_a + 1..];
            buf.push(a as u8);

            if matched_char == b'\n' {
                break;
            }
        }

        let mut is_valid = true;

        // debug!("LINE: {buf:?}");

        // FIXME: Suboptimal O(n^3) loop. Express buf and rules as bitmasks and perform some bit
        // magic for O(1). Will require u128s. Time was ~62μs on Default perf. profile.
        'outer: for (idx, page) in buf.iter().enumerate() {
            let rules = ll.rules_for(*page);

            // debug!(
            //     "RULES for {page}: {:?}. Will search: {:?}",
            //     &(&*rules)[0..rules.len()],
            //     &buf[idx + 1..]
            // );

            for page in &buf[idx + 1..] {
                if !rules.contains(page) {
                    is_valid = false;

                    break 'outer;
                }
            }
        }

        if is_valid {
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
    use std::ops::{Deref, Index};

    use super::MAX_RULES_PER_NODE as RULES_SIZE;

    // TODO: After you are done with the day. Store [u8; MAX_RULES_PER_NODE] as a u128 bitmask,
    // since max node number ID can be 100.
    pub struct SpecializedLinkedList<const SIZE: usize>([(usize, [u8; RULES_SIZE]); SIZE]);

    impl<const SIZE: usize> SpecializedLinkedList<SIZE> {
        pub const fn new() -> Self {
            Self([(0, [0; RULES_SIZE]); SIZE])
        }

        /// Push a new rule for node.
        pub const fn push_to(&mut self, node: u8, rule: u8) {
            let (len, buf) = &mut self.0[node as usize];

            buf[*len] = rule;
            *len += 1;
        }

        /// Get all rules for a node.
        pub const fn rules_for(&self, node: u8) -> RuleView {
            let (len, buf) = &self.0[node as usize];

            RuleView { buf, len: *len }
        }
    }

    #[derive(Debug)]
    pub struct RuleView<'b> {
        buf: &'b [u8; RULES_SIZE],
        len: usize,
    }

    impl RuleView<'_> {
        pub const fn len(&self) -> usize {
            self.len
        }
    }

    impl Index<usize> for RuleView<'_> {
        type Output = u8;

        #[inline(always)]
        fn index(&self, index: usize) -> &Self::Output {
            assert!(index < self.len);

            // The elements are stored backwards.
            // SAFETY: `index` will always be <= `self.1 - 1`.
            unsafe { self.buf.get_unchecked(self.len - 1 - index) }
        }
    }

    impl Deref for RuleView<'_> {
        type Target = [u8];

        fn deref(&self) -> &Self::Target {
            self.buf
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn basic_usage() {
            let mut ll = SpecializedLinkedList::<100>::new();

            ll.push_to(75, 29);
            ll.push_to(75, 53);
            ll.push_to(75, 47);
            ll.push_to(75, 61);
            ll.push_to(75, 13);

            let rules = ll.rules_for(75);
            assert_eq!(rules.len(), 5);

            assert_eq!(rules[0], 13, "rules[0] == 13");
            assert_eq!(rules[1], 61, "rules[1] == 61");
            assert_eq!(rules[2], 47, "rules[2] == 47");
            assert_eq!(rules[3], 53, "rules[3] == 53");
            assert_eq!(rules[4], 29, "rules[4] == 29");

            assert_eq!(ll.rules_for(0).len(), 0);
        }

        #[test]
        #[should_panic]
        fn improper_usage() {
            let ll = SpecializedLinkedList::<100>::new();
            let rules = ll.rules_for(75);

            std::hint::black_box(rules[0]);
        }
    }
}
