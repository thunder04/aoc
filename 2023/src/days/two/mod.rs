static INPUT: &str = include_str!("./input.txt");

const MAX_R_CUBES: u8 = 12;
const MAX_G_CUBES: u8 = 13;
const MAX_B_CUBES: u8 = 14;

pub fn run() -> color_eyre::Result<()> {
    info!(
        "[Part 1] The sum of the IDs of those games is: {}",
        part_1(INPUT)
    );

    info!(
        "[Part 2] The sum of the power of these sets is: {}",
        part_2(INPUT)
    );

    Ok(())
}

// Answer: 2545
fn part_1(input: &str) -> u32 {
    let mut sum = 0_u32;

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

        sum += game_id as u32 + 1;
    }

    sum
}

// Answer: 78111
fn part_2(input: &str) -> u32 {
    let mut sum = 0_u32;

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

        sum += max_r as u32 * max_g as u32 * max_b as u32;
    }

    sum
}
