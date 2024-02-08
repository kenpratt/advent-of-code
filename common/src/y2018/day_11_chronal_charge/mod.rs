use std::ops::RangeInclusive;

use crate::interface::AoC;

pub struct Day;
impl AoC<usize, (i32, (usize, usize, usize)), (i32, (usize, usize, usize))> for Day {
    const FILE: &'static str = file!();

    fn parse(input: String) -> usize {
        input.parse::<usize>().unwrap()
    }

    fn part1(serial_number: &usize) -> (i32, (usize, usize, usize)) {
        let powers = calculate_powers(*serial_number);
        best_square_of_size(&powers, 3)
    }

    fn part2(serial_number: &usize) -> (i32, (usize, usize, usize)) {
        let powers = calculate_powers(*serial_number);
        best_square_dynamic_size(&powers, 1..=300)
    }
}

fn power_level(x: usize, y: usize, serial_number: usize) -> i32 {
    let rack_id = x + 10;
    let tmp = ((rack_id * y) + serial_number) * rack_id / 100 % 10;
    tmp as i32 - 5
}

const WIDTH: usize = 300;

const STARTING_SIZE: usize = 14; // based on examples
const HALT_THRESHOLD: i32 = 20;

fn calculate_powers(serial_number: usize) -> [[i32; WIDTH]; WIDTH] {
    let mut powers = [[0; WIDTH]; WIDTH];
    for x in 0..WIDTH {
        for y in 0..WIDTH {
            let power = power_level(x + 1, y + 1, serial_number);
            powers[x][y] = power;
        }
    }
    powers
}

fn best_square_of_size(
    powers: &[[i32; WIDTH]; WIDTH],
    size: usize,
) -> (i32, (usize, usize, usize)) {
    let mut rolling_col = [0; WIDTH];

    let mut best_score = i32::MIN;
    let mut best_coord = (0, 0, 0);

    for x in 0..WIDTH {
        let add_col = &powers[x];
        let sub_col = if x >= size {
            Some(&powers[x - size])
        } else {
            None
        };
        let check_best = x >= (size - 1);

        for y in 0..WIDTH {
            rolling_col[y] += add_col[y];

            if let Some(col) = sub_col {
                rolling_col[y] -= col[y];
            }
        }

        if check_best {
            let mut rolling_val = 0;
            for y in 0..WIDTH {
                rolling_val += rolling_col[y];

                if y >= size {
                    rolling_val -= rolling_col[y - size];
                }

                if y >= (size - 1) && rolling_val > best_score {
                    best_score = rolling_val;
                    best_coord = (x + 2 - size, y + 2 - size, size);
                }
            }
        }
    }

    (best_score, best_coord)
}

fn best_square_dynamic_size(
    powers: &[[i32; WIDTH]; WIDTH],
    sizes: RangeInclusive<usize>,
) -> (i32, (usize, usize, usize)) {
    let mut best = (i32::MIN, (0, 0, 0));

    // first, try increasing size
    for size in STARTING_SIZE..=*sizes.end() {
        let round_best = best_square_of_size(powers, size);
        if round_best.0 > best.0 {
            best = round_best;
        } else if (best.0 - round_best.0) > HALT_THRESHOLD {
            break;
        }
    }

    // then, try decreasing size
    for size in (*sizes.start()..STARTING_SIZE).rev() {
        let round_best = best_square_of_size(powers, size);
        if round_best.0 > best.0 {
            best = round_best;
        } else if (best.0 - round_best.0) > HALT_THRESHOLD {
            break;
        }
    }

    best
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_power_levels() {
        assert_eq!(power_level(3, 5, 8), 4);
        assert_eq!(power_level(122, 79, 57), -5);
        assert_eq!(power_level(217, 196, 39), 0);
        assert_eq!(power_level(101, 153, 71), 4);
    }

    #[test]
    fn test_part1_examples() {
        assert_eq!(Day::part1(&18), (29, (33, 45, 3)));
        assert_eq!(Day::part1(&42), (30, (21, 61, 3)));
    }

    #[test]
    fn test_part1_solution() {
        let result = Day::part1(&Day::parse_input_file());
        assert_eq!(result, (30, (20, 41, 3)));
    }

    #[test]
    fn test_part2_examples() {
        assert_eq!(Day::part2(&18), (113, (90, 269, 16)));
        assert_eq!(Day::part2(&42), (119, (232, 251, 12)));
    }

    #[test]
    fn test_part2_solution() {
        let result = Day::part2(&Day::parse_input_file());
        assert_eq!(result, (76, (236, 270, 11)));
    }
}
