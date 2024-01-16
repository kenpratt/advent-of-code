use std::{cmp, fs};

use lazy_static::lazy_static;
use regex::Regex;

const INPUT_FILE: &'static str = "input.txt";

fn main() {
    println!("part 1 result: {:?}", part1(&read_file(INPUT_FILE)));
    // println!("part 2 result: {:?}", part2(&read_file(INPUT_FILE)));
}

fn read_file(filename: &str) -> String {
    fs::read_to_string(filename).expect("Something went wrong reading the file")
}

#[derive(Debug)]
struct Present {
    length: usize,
    width: usize,
    height: usize,
}

impl Present {
    fn parse_list(input: &str) -> Vec<Self> {
        input.lines().map(|line| Self::parse(line)).collect()
    }

    fn parse(input: &str) -> Self {
        lazy_static! {
            static ref PRESENT_RE: Regex = Regex::new(r"\A(\d+)x(\d+)x(\d+)\z").unwrap();
        }

        let caps = PRESENT_RE.captures(input).unwrap();
        let length = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let width = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
        let height = caps.get(3).unwrap().as_str().parse::<usize>().unwrap();

        Self {
            length,
            width,
            height,
        }
    }

    fn paper_needed(&self) -> usize {
        let top = self.length * self.width;
        let front = self.width * self.height;
        let side = self.height * self.length;
        let smallest = cmp::min(top, cmp::min(front, side));
        top * 2 + front * 2 + side * 2 + smallest
    }
}

fn part1(input: &str) -> usize {
    let presents = Present::parse_list(input);
    presents.iter().map(|p| p.paper_needed()).sum()
}

// fn part2(input: &str) -> usize {
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_examples() {
        assert_eq!(part1(&"2x3x4"), 58);
        assert_eq!(part1(&"1x1x10"), 43);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_file(INPUT_FILE));
        assert_eq!(result, 1588178);
    }

    // #[test]
    // fn test_part2_example() {
    //     let result = part2(&read_file(EXAMPLE_FILE));
    //     assert_eq!(result, 0);
    // }

    // #[test]
    // fn test_part2_solution() {
    //     let result = part2(&read_file(INPUT_FILE));
    //     assert_eq!(result, 0);
    // }
}
