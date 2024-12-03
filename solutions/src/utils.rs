use std::fmt::Display;

/// Print an entire array. Creates a new line at every `split_at` elements.
pub fn debug_array<T: Display>(slice: &[T], title: &str, split_at: usize) {
    println!("===== {title} =====\n");

    for (idx, num) in slice.iter().enumerate() {
        let line = idx / (split_at + 1);

        print!("a[{idx:0>2}]={num: <7} ");

        if idx == (split_at * (line + 1) + line) {
            println!();
        }
    }

    println!("\n\n");
}
