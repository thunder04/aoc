use std::cmp::Ordering;

const X: u32 = 101;
const Y: u32 = 103;

type Point = (u32, u32);
type Velocity = (i32, i32);

pub fn part_1(input: &[u8]) -> i64 {
    let mut quadrants = [0_i64; 4];

    read_input(input, |p0, v| {
        let (x, y) = simulate(p0, v, 100);

        match x.cmp(&(X / 2)) {
            Ordering::Greater => match y.cmp(&(Y / 2)) {
                Ordering::Greater => quadrants[0] += 1,
                Ordering::Less => quadrants[3] += 1,
                Ordering::Equal => {}
            },

            Ordering::Less => match y.cmp(&(Y / 2)) {
                Ordering::Greater => quadrants[1] += 1,
                Ordering::Less => quadrants[2] += 1,
                Ordering::Equal => {}
            },

            Ordering::Equal => {}
        }
    });

    quadrants.into_iter().product()
}

pub fn part_2(input: &[u8]) -> i64 {
    0
}

#[inline]
fn simulate(p0: Point, v @ (v_x, v_y): Velocity, t: u32) -> Point {
    let (p0_x, p0_y) = (p0.0 as i32, p0.1 as i32);
    let t = t as i32;

    debug_assert_ne!(v, (0, 0), "No velocity");

    let x = p0_x + (v_x * t);
    let x = match v_x.signum() {
        1 => x % X as i32,
        -1 => match x % X as i32 {
            0 => 0,
            rem => X as i32 + rem,
        },

        _ => unsafe { std::hint::unreachable_unchecked() },
    };

    let y = p0_y + (v_y * t);
    let y = match v_y.signum() {
        1 => y % Y as i32,
        -1 => match y % Y as i32 {
            0 => 0,
            rem => Y as i32 + rem,
        },

        _ => unsafe { std::hint::unreachable_unchecked() },
    };

    (x as u32, y as u32)
}

fn read_input(mut input: &[u8], mut on_robot: impl FnMut(Point, Velocity)) {
    use atoi_simd::parse_any as atoi;

    loop {
        input = unsafe { input.get_unchecked("p=".len()..) };
        let (p0_x, off) = unsafe { atoi(input).unwrap_unchecked() };

        input = unsafe { input.get_unchecked(off + ",".len()..) };
        let (p0_y, off) = unsafe { atoi(input).unwrap_unchecked() };

        input = unsafe { input.get_unchecked(off + " v=".len()..) };
        let (v_x, off) = unsafe { atoi(input).unwrap_unchecked() };

        input = unsafe { input.get_unchecked(off + ",".len()..) };
        let (v_y, off) = unsafe { atoi(input).unwrap_unchecked() };

        on_robot((p0_x, p0_y), (v_x, v_y));

        if off + "\n".len() + 1 < input.len() {
            input = unsafe { input.get_unchecked(off + "\n".len()..) };
        } else {
            break;
        }
    }
}
