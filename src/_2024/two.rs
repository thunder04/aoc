use core::ops::RangeInclusive;

use memchr::memchr;

const ACCEPTABLE_LEVEL_DIFF: RangeInclusive<i16> = -3..=3;

pub fn part_1(input: &[u8]) -> i64 {
    answer_generator_v10000::<false>(input)
}

/// The function implements the algorithm that solves the day's problems.
///
/// ## START
/// Read a character.
///     - It is a number. Store the digit to `s.lvl`. Advance input by one. Go to [START].
///     - It is a whitespace or newline.
///           - a) Do you have a `s.prev_lvl`?
///                - Yes. Calculate `s.lvl - s.prev_lvl`.
///                    - a) Is the result 0?
///                        - Yes. Go to [JUDGEMENT TIME].
///                        - No. Continue below.
///                    - b) Is the result within [-3, 3]?
///                        - Yes. Continue below.
///                        - No. Go to [JUDGEMENT TIME].
///                    - c) Do you have a `s.prev_signum`?
///                        - No. Store `sig(s.lvl)` to `s.prev_signum`. Go to [ALL GOOD].
///                        - Yes. Are `sig(s.lvl)` and `s.prev_signum` the same?
///                            - Yes. Go to [ALL GOOD].
///                            - No. Go to [JUDGEMENT TIME].
///                - No. Continue below.
///           - b) Was it a newline?
///                 - Yes. Increase `safe_reports` by one. Reset `s`. Advance input by one. Go to
///                   [START].
///                 - No. Go to [ALL GOOD].
///     - Is it something else? Stop. It is invalid input.
///
/// ## JUDGEMENT TIME
/// Which part are you solving?
///   - Part 1. Skip the entire line. Reset `s`. Go to [START].
///   - Part 2. Is `s.saw_unsafe_lvl` true?
///       - Yes. Skip the entire line. Reset `s`. Go to [START].
///       - No. Set `s.saw_unsafe_lvl` to true. Test the following scenario: Let prev =
///         `s.prev_prev_lvl` (it must exist at this stage), curr = `s.lvl`. Is it valid?
///         - Yes. The previous level is invalid. Set `s.prev_lvl` to `s.prev_prev_lvl` and `s.lvl`
///           to 0. Advance input by one. Go to [START].
///         - No. The current number is invalid. Set `s.lvl` to 0. Advance input by one. Go to
///           [START].
///
/// ## ALL GOOD
/// Set `s.prev_prev_lvl` to `s.prev_lvl`. Set `s.prev_lvl` to `s.lvl`. Set `s.lvl` to `0`.
/// Advance input by one. Go to [1].
fn answer_generator_v10000<const IS_PART_2: bool>(mut input: &[u8]) -> i64 {
    #[derive(Default)]
    struct State {
        saw_unsafe_lvl: bool,
        signum: Option<i16>,
        prev_prev_lvl: Option<u16>,
        prev_lvl: Option<u16>,
        lvl: u16,
    }

    impl State {
        #[inline]
        fn signum_if_valid(&self, prev: u16, curr: u16) -> Option<i16> {
            let diff = curr as i16 - prev as i16;
            let diff_signum = diff.signum();

            if diff == 0
                || !ACCEPTABLE_LEVEL_DIFF.contains(&diff)
                || self.signum.is_some_and(|signum| signum != diff_signum)
            {
                None
            } else {
                Some(diff_signum)
            }
        }
    }

    let mut s = State::default();
    let mut safe_reports = 0_i64;

    loop {
        match input.first().copied() {
            Some(ch @ b'0'..=b'9') => {
                s.lvl = (s.lvl * 10) + (ch - b'0') as u16;
                input = &input[1..];
                continue;
            }

            // FIXME: Currently part 2 is broken. I cba to fix it.
            Some(ch @ (b' ' | b'\n')) => {
                if let Some(prev_lvl) = s.prev_lvl {
                    if let Some(diff_signum) = s.signum_if_valid(prev_lvl, s.lvl) {
                        s.signum = Some(diff_signum);
                    } else if !IS_PART_2 || s.saw_unsafe_lvl {
                        if let Some(next_off) = memchr(b'\n', input) {
                            input = &input[next_off + 1..];
                            s = State::default();
                            continue;
                        } else {
                            break;
                        }
                    } else {
                        // pp, p, c, | n
                        //  4, 5, 3, | 7 (current is invalid)
                        //  7, 4, 8, | … (previous is invalid)

                        // if let Some(prev_lvl) = s.prev_prev_lvl {
                        //     if s.signum_if_valid(prev_lvl, s.lvl).is_some() {
                        s.prev_lvl = s.prev_prev_lvl;
                        //     }
                        // }

                        s.saw_unsafe_lvl = true;
                        input = &input[1..];
                        s.lvl = 0;

                        continue;
                    }
                }

                if ch == b'\n' {
                    s = State::default();
                    safe_reports += 1;
                } else {
                    s.prev_prev_lvl = s.prev_lvl;
                    s.prev_lvl = Some(s.lvl);
                    s.lvl = 0;
                }

                input = &input[1..];
            }

            Some(_) => unreachable!("Invalid input"),
            None => break,
        }
    }

    safe_reports
}
