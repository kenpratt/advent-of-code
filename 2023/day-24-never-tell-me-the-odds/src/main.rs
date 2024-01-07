use std::{fs, ops::RangeInclusive};

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

const PART1_RANGE: RangeInclusive<i64> = 200000000000000..=400000000000000;

fn main() {
    println!(
        "part 1 result: {:?}",
        part1(&read_input_file(), PART1_RANGE)
    );
    // println!("part 2 result: {:?}", part2(&read_input_file()));
}

fn read_input_file() -> String {
    fs::read_to_string("input.txt").expect("Something went wrong reading the file")
}

#[derive(Debug)]
struct Hailstone {
    position: Coord,
    velocity: Coord,
}

impl Hailstone {
    fn parse_list(input: &str) -> Vec<Self> {
        input.lines().map(|line| Self::parse(line)).collect()
    }

    fn parse(input: &str) -> Self {
        lazy_static! {
            static ref HAILSTONE_RE: Regex = Regex::new(r"\A(.+) @ (.+)\z").unwrap();
        }

        let caps = HAILSTONE_RE.captures(input).unwrap();
        let position = Coord::parse(caps.get(1).unwrap().as_str());
        let velocity = Coord::parse(caps.get(2).unwrap().as_str());
        Self { position, velocity }
    }

    fn count_intersections_2d(stones: &[Hailstone], range: &RangeInclusive<i64>) -> usize {
        let range_f64 = (*range.start() as f64)..=(*range.end() as f64);
        stones
            .iter()
            .combinations(2)
            .filter(|v| v[0].intersects_2d(&v[1], &range_f64))
            .count()
    }

    fn slope_and_y_intercept_2d(&self) -> (f64, f64) {
        let slope = (self.velocity.y as f64) / (self.velocity.x as f64);
        let intercept = (self.position.y as f64) - slope * (self.position.x as f64);
        (slope, intercept)
    }

    fn intersects_2d(&self, other: &Hailstone, range: &RangeInclusive<f64>) -> bool {
        let (s1, i1) = self.slope_and_y_intercept_2d();
        let (s2, i2) = other.slope_and_y_intercept_2d();

        let x = (i2 - i1) / (s1 - s2);
        let y = s1 * x + i1;

        let f1 = (y - (self.position.y as f64)).signum() as i64 == self.velocity.y.signum();
        let f2 = (y - (other.position.y as f64)).signum() as i64 == other.velocity.y.signum();

        f1 && f2 && range.contains(&x) && range.contains(&y)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Coord {
    x: i64,
    y: i64,
    z: i64,
}

impl Coord {
    fn parse(input: &str) -> Self {
        let nums: Vec<i64> = input
            .split(",")
            .map(|s| s.trim().parse::<i64>().unwrap())
            .collect();
        Self::from_slice(&nums)
    }

    fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }

    fn from_slice(nums: &[i64]) -> Self {
        assert_eq!(nums.len(), 3);
        Self::new(nums[0], nums[1], nums[2])
    }
}

fn part1(input: &str, val_range: RangeInclusive<i64>) -> usize {
    let hailstones = Hailstone::parse_list(input);
    Hailstone::count_intersections_2d(&hailstones, &val_range)
}

// fn part2(input: &str) -> usize {
//     let hailstones = Data::parse(input);
//     0
// }

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    static EXAMPLE: &str = indoc! {"
        19, 13, 30 @ -2,  1, -2
        18, 19, 22 @ -1, -1, -2
        20, 25, 34 @ -2, -2, -4
        12, 31, 28 @ -1, -2, -1
        20, 19, 15 @  1, -5, -3
    "};

    #[test]
    fn test_part1_example() {
        let result = part1(EXAMPLE, 7..=27);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file(), PART1_RANGE);
        assert_eq!(result, 16589);
    }

    // #[test]
    // fn test_part2_example() {
    //     let result = part2(EXAMPLE);
    //     assert_eq!(result, 0);
    // }

    // #[test]
    // fn test_part2_solution() {
    //     let result = part2(&read_input_file());
    //     assert_eq!(result, 0);
    // }
}
