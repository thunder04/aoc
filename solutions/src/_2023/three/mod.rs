use std::collections::HashSet;

static INPUT: &str = include_str!("./input.txt");

pub fn run() -> super::Runner {
    (Some(part_1), Some(part_2))
}

// Answer: 526404
fn part_1() -> u64 {
    let mut sum = 0_u64;

    let mut lines = INPUT.lines().enumerate().peekable();
    // I assume there are no symbols in the first line.
    let mut prev_line = lines.next().expect("expected at least two lines");
    let mut added_numbers = HashSet::with_capacity(1 << 12);

    while let Some(line) = lines.next() {
        let mut offset = 0;

        #[cfg(debug_assertions)]
        debug!("current line: {:?}", line);

        while let Some(idx) = line.1[offset..].find(is_symbol) {
            let idx = offset + idx;

            #[cfg(debug_assertions)]
            debug!("{x: >4}found symbol={:?}", &line.1[idx..=idx], x = "");

            let positions = [
                (Some(prev_line), idx - 1),       // Top left
                (Some(prev_line), idx),           // Top center
                (Some(prev_line), idx + 1),       // Top right
                (Some(line), idx - 1),            // Center left
                (Some(line), idx + 1),            // Center right
                (lines.peek().copied(), idx - 1), // Bottom left
                (lines.peek().copied(), idx),     // Bottom center
                (lines.peek().copied(), idx + 1), // Bottom right
            ];

            for (line, idx) in positions {
                if let Some((line_id, line)) = line
                    .map(|(line_id, line)| (line_id, line.as_bytes()))
                    .filter(|(_, line)| line[idx].is_ascii_digit())
                {
                    let idxs = find_digit_indexes(line, idx);

                    if added_numbers.insert((line_id, idxs)) {
                        #[cfg(debug_assertions)]
                        debug!(
                            "{x: >8}line=\"{}\", digit={}",
                            std::str::from_utf8(line).unwrap(),
                            std::str::from_utf8(&line[idxs.0..=idxs.1]).unwrap(),
                            x = ""
                        );

                        sum += simple_parse_digit(line, idxs) as u64;
                    }
                }
            }

            // I'm not sure what happens if there's a symbol at the end of a line.
            offset = idx + 1;
        }

        prev_line = line;
    }

    sum
}

// Answer: ???
fn part_2() -> u64 {
    let mut sum = 0_u64;

    let mut lines = INPUT.lines().enumerate().peekable();
    // I assume there are no symbols in the first line.
    let mut prev_line = lines.next().expect("expected at least two lines");
    let mut added_numbers = HashSet::with_capacity(1 << 12);

    while let Some(line) = lines.next() {
        let mut offset = 0;

        #[cfg(debug_assertions)]
        debug!("current line: {:?}", line);

        while let Some(idx) = line.1[offset..].find('*') {
            let idx = offset + idx;
            let mut first = 0;
            let mut second = 0;

            #[cfg(debug_assertions)]
            debug!("{x: >4}found symbol={:?}", &line.1[idx..=idx], x = "");

            let positions = [
                (Some(prev_line), idx - 1),       // Top left
                (Some(prev_line), idx),           // Top center
                (Some(prev_line), idx + 1),       // Top right
                (Some(line), idx - 1),            // Center left
                (Some(line), idx + 1),            // Center right
                (lines.peek().copied(), idx - 1), // Bottom left
                (lines.peek().copied(), idx),     // Bottom center
                (lines.peek().copied(), idx + 1), // Bottom right
            ];

            for (line, idx) in positions {
                if let Some((line_id, line)) = line
                    .map(|(line_id, line)| (line_id, line.as_bytes()))
                    .filter(|(_, line)| line[idx].is_ascii_digit())
                {
                    let idxs = find_digit_indexes(line, idx);

                    if added_numbers.insert((line_id, idxs)) {
                        #[cfg(debug_assertions)]
                        debug!(
                            "{x: >8}line=\"{}\", digit={}",
                            std::str::from_utf8(line).unwrap(),
                            std::str::from_utf8(&line[idxs.0..=idxs.1]).unwrap(),
                            x = ""
                        );

                        // This assumes the numbers in the input are never zero.
                        if first == 0 {
                            first = simple_parse_digit(line, idxs) as u64;
                        } else if second == 0 {
                            second = simple_parse_digit(line, idxs) as u64;
                        } else {
                            break;
                        }
                    }
                }
            }

            if first > 0 && second > 0 {
                sum += first * second;
            }

            // I'm not sure what happens if there's a symbol at the end of a line.
            offset = idx + 1;
        }

        prev_line = line;
    }

    sum
}

#[inline]
const fn is_symbol(ch: char) -> bool {
    ch != '.' && ch.is_ascii_punctuation()
}

#[inline]
fn find_digit_indexes(line: &[u8], idx: usize) -> (usize, usize) {
    let mut start = idx;
    let mut end = idx;

    // Go left until you encounter a dot, or reach the first element.
    for i in (0..idx).rev() {
        if line[i].is_ascii_digit() {
            start = i;
        } else {
            break;
        }
    }

    // Go right until you encounter a dot, or reach the last element.
    for (i, char) in line.iter().enumerate().skip(idx + 1) {
        if char.is_ascii_digit() {
            end = i;
        } else {
            break;
        }
    }

    (start, end)
}

#[inline]
const fn simple_parse_digit(line: &[u8], (start, end): (usize, usize)) -> u16 {
    match end - start {
        2 => {
            ((line[end - 2] - b'0') as u16 * 100)
                + ((line[end - 1] - b'0') as u16 * 10)
                + (line[end] - b'0') as u16
        }

        1 => (((line[end - 1] - b'0') * 10) + (line[end] - b'0')) as u16,
        0 => (line[end] - b'0') as u16,

        #[allow(unconditional_panic, clippy::out_of_bounds_indexing)]
        _ => [][0], // The number matched was in range [1000, +inf)
    }
}
