pub fn part_1(input: &[u8]) -> i64 {
    let mut sum = 0;

    read_input(input, |machine| {
        let (a, b) = _2x2_solver(machine);

        if a.fract() == 0.0 && b.fract() == 0.0 && a.trunc() <= 100.0 && b.trunc() <= 100.0 {
            sum += a.trunc() as i64 * 3 + b.trunc() as i64;
        }
    });

    sum
}

pub fn part_2(input: &[u8]) -> i64 {
    let mut sum = 0;

    read_input(input, |mut machine| {
        machine.prize.0 += 10000000000000;
        machine.prize.1 += 10000000000000;

        let (a, b) = _2x2_solver(machine);

        if a.fract() == 0.0 && b.fract() == 0.0 {
            sum += a.trunc() as i64 * 3 + b.trunc() as i64;
        }
    });

    sum
}

#[inline]
fn _2x2_solver(
    Machine {
        btn_a: (x_a, y_a),
        btn_b: (x_b, y_b),
        prize: (p_x, p_y),
    }: Machine,
) -> (f64, f64) {
    // Cramer's rule on a 2x2 determinant.
    let d = ((x_a * y_b) - (x_b * y_a)) as f64;
    let d_a = ((p_x * y_b) - (p_y * x_b)) as f64;
    let d_b = ((x_a * p_y) - (y_a * p_x)) as f64;

    (d_a / d, d_b / d)
}

#[derive(Debug)]
struct Machine {
    pub btn_a: (i64, i64),
    pub btn_b: (i64, i64),
    pub prize: (i64, i64),
}

#[inline]
fn read_input(mut input: &[u8], mut on_machine: impl FnMut(Machine)) {
    use atoi_simd::parse_any_pos as atoi;

    loop {
        input = unsafe { input.get_unchecked("Button A: X+".len()..) };
        let (x_a, off) = unsafe { atoi(input).unwrap_unchecked() };

        input = unsafe { input.get_unchecked(off + ", Y+".len()..) };
        let (y_a, off) = unsafe { atoi(input).unwrap_unchecked() };

        input = unsafe { input.get_unchecked(off + "\nButton B: X+".len()..) };
        let (x_b, off) = unsafe { atoi(input).unwrap_unchecked() };

        input = unsafe { input.get_unchecked(off + ", Y+".len()..) };
        let (y_b, off) = unsafe { atoi(input).unwrap_unchecked() };

        input = unsafe { input.get_unchecked(off + "\nPrize: X=".len()..) };
        let (p_x, off) = unsafe { atoi(input).unwrap_unchecked() };

        input = unsafe { input.get_unchecked(off + ", Y=".len()..) };
        let (p_y, off) = unsafe { atoi(input).unwrap_unchecked() };

        on_machine(Machine {
            btn_a: (x_a, y_a),
            btn_b: (x_b, y_b),
            prize: (p_x, p_y),
        });

        if off + "\n\n".len() < input.len() {
            input = unsafe { input.get_unchecked(off + "\n\n".len()..) };
        } else {
            break;
        }
    }
}
