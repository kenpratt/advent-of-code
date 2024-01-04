use std::fs;

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
struct Brick {
    left: Coord,
    right: Coord,
}

impl Brick {
    fn parse_list(input: &str) -> Vec<Self> {
        input.lines().map(|line| Self::parse(line)).collect()
    }

    fn parse(input: &str) -> Self {
        lazy_static! {
            static ref ITEM_RE: Regex =
                Regex::new(r"\A(\d+),(\d+),(\d+)~(\d+),(\d+),(\d+)\z").unwrap();
        }

        let caps = ITEM_RE.captures(input).unwrap();
        let nums: Vec<u16> = caps
            .iter()
            .skip(1)
            .map(|c| c.unwrap().as_str().parse::<u16>().unwrap())
            .collect();
        assert_eq!(nums.len(), 6);

        let left = Coord::from_slice(&nums[0..3]);
        let right = Coord::from_slice(&nums[3..6]);

        Self { left, right }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Coord {
    x: u16,
    y: u16,
    z: u16,
}

impl Coord {
    fn new(x: u16, y: u16, z: u16) -> Self {
        Self { x, y, z }
    }

    fn from_slice(nums: &[u16]) -> Self {
        assert_eq!(nums.len(), 3);
        Self::new(nums[0], nums[1], nums[2])
    }
}

fn part1(input: &str) -> usize {
    let bricks = Brick::parse_list(input);
    dbg!(&bricks);
    0
}

// fn part2(input: &str) -> usize {
//     let items = Data::parse(input);
//     dbg!(&items);
//     0
// }

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    static EXAMPLE: &str = indoc! {"
        1,0,1~1,2,1
        0,0,2~2,0,2
        0,2,3~2,2,3
        0,0,4~0,2,4
        2,0,5~2,2,5
        0,1,6~2,1,6
        1,1,8~1,1,9
    "};

    #[test]
    fn test_part1_example() {
        let result = part1(EXAMPLE);
        assert_eq!(result, 5);
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
