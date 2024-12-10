const MAX_R_CUBES: u8 = 12;
const MAX_G_CUBES: u8 = 13;
const MAX_B_CUBES: u8 = 14;

pub fn part_1(input: &[u8]) -> i64 {
    let input = unsafe { std::str::from_utf8_unchecked(input) };
    let mut sum = 0;

    'game: for (game_id, line) in input.lines().enumerate() {
        let Some((_, line)) = line.split_once(": ") else {
            break;
        };

        for subset in line.split("; ") {
            for cubes in subset.split(", ") {
                let Some((num, color)) = cubes.split_once(" ") else {
                    break;
                };

                let num: u8 = num.parse().unwrap();

                match color {
                    "green" if num > MAX_G_CUBES => continue 'game,
                    "blue" if num > MAX_B_CUBES => continue 'game,
                    "red" if num > MAX_R_CUBES => continue 'game,

                    _ => {}
                }
            }
        }

        sum += game_id as i64 + 1;
    }

    sum
}

pub fn part_2(input: &[u8]) -> i64 {
    let input = unsafe { std::str::from_utf8_unchecked(input) };
    let mut sum = 0;

    for line in input.lines() {
        let Some((_, line)) = line.split_once(": ") else {
            break;
        };

        let mut max_r = 0;
        let mut max_g = 0;
        let mut max_b = 0;

        for subset in line.split("; ") {
            for cubes in subset.split(", ") {
                let Some((num, color)) = cubes.split_once(" ") else {
                    break;
                };

                let num: u8 = num.parse().unwrap();

                match color {
                    "green" if num > max_g => max_g = num,
                    "blue" if num > max_b => max_b = num,
                    "red" if num > max_r => max_r = num,

                    _ => {}
                }
            }
        }

        sum += max_r as i64 * max_g as i64 * max_b as i64;
    }

    sum
}
