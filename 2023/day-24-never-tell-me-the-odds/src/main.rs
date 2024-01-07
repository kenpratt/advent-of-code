use std::fs;

// use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
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

fn part1(input: &str) -> usize {
    let hailstones = Hailstone::parse_list(input);
    dbg!(&hailstones);
    0
}

// fn part2(input: &str) -> usize {
//     let hailstones = Data::parse(input);
//     dbg!(&hailstones);
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
        let result = part1(EXAMPLE);
        assert_eq!(result, 0);
    }

    // #[test]
    // fn test_part1_solution() {
    //     let result = part1(&read_input_file());
    //     assert_eq!(result, 0);
    // }

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
