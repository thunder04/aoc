use std::fmt::Debug;

/// Reads up to three digits from `$input` withn `$off` offset.
///
/// ## Returns
/// A tuple containing the parsed number and the index after the last digit.
#[macro_export]
macro_rules! read_number_lazily {
    ($input: expr, $off: expr, $last_pat: pat) => {
        read_number_lazily!($input, $off, $last_pat, {
            continue;
        })
    };

    ($input: expr, $off: expr, $last_pat: pat, $invalid_arm: block) => {
        match $input[$off] {
            d1 @ b'0'..=b'9' => match $input[$off + 1] {
                d2 @ b'0'..=b'9' => match $input[$off + 2] {
                    d3 @ b'0'..=b'9' => match $input[$off + 3] {
                        $last_pat => (
                            ((d1 - b'0') as u64 * 100)
                                + ((d2 - b'0') as u64 * 10)
                                + ((d3 - b'0') as u64),
                            $off + 3,
                        ),
                        _ => $invalid_arm,
                    },

                    $last_pat => (((d1 - b'0') as u64 * 10) + ((d2 - b'0') as u64), $off + 2),
                    _ => $invalid_arm,
                },

                $last_pat => ((d1 - b'0') as u64, $off + 1),
                _ => $invalid_arm,
            },

            _ => $invalid_arm,
        }
    };
}

/// Print an entire array. Creates a new line at every `split_at` elements.
pub fn debug_array<I>(slice: I, title: &str, split_at: usize)
where
    I: IntoIterator,
    I::Item: Debug,
{
    println!("===== {title} =====\n");

    for (idx, num) in slice.into_iter().enumerate() {
        let line = idx / (split_at + 1);

        print!("a[{idx: >5}]={: <2} ", format!("{num:?}"));

        if idx == (split_at * (line + 1) + line) {
            println!();
        }
    }

    println!("\n\n");
}
