use crate::interface::AoC;

use std::cmp;

use lazy_static::lazy_static;
use regex::Regex;

pub struct Day;
impl AoC<Vec<Present>, usize, usize> for Day {
    const FILE: &'static str = file!();

    fn parse(input: String) -> Vec<Present> {
        Present::parse_list(&input)
    }

    fn part1(presents: &Vec<Present>) -> usize {
        presents.iter().map(|p| p.paper_needed()).sum()
    }

    fn part2(presents: &Vec<Present>) -> usize {
        presents.iter().map(|p| p.ribbon_needed()).sum()
    }
}

#[derive(Debug)]
pub struct Present {
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
        let mut nums_iter = caps
            .iter()
            .skip(1)
            .flat_map(|c| c.unwrap().as_str().parse::<usize>());

        let length = nums_iter.next().unwrap();
        let width = nums_iter.next().unwrap();
        let height = nums_iter.next().unwrap();
        assert_eq!(None, nums_iter.next());

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
        let smallest = min(top, front, side);
        top * 2 + front * 2 + side * 2 + smallest
    }

    fn ribbon_needed(&self) -> usize {
        let top = 2 * (self.length + self.width);
        let front = 2 * (self.width + self.height);
        let side = 2 * (self.height + self.length);
        let smallest = min(top, front, side);
        smallest + self.volume()
    }

    fn volume(&self) -> usize {
        self.length * self.width * self.height
    }
}

fn min(a: usize, b: usize, c: usize) -> usize {
    cmp::min(a, cmp::min(b, c))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_examples() {
        assert_eq!(Day::part1(&Day::parse_str("2x3x4")), 58);
        assert_eq!(Day::part1(&Day::parse_str("1x1x10")), 43);
    }

    #[test]
    fn test_part1_solution() {
        let result = Day::part1(&Day::parse_input_file());
        assert_eq!(result, 1588178);
    }

    #[test]
    fn test_part2_examples() {
        assert_eq!(Day::part2(&Day::parse_str("2x3x4")), 34);
        assert_eq!(Day::part2(&Day::parse_str("1x1x10")), 14);
    }

    #[test]
    fn test_part2_solution() {
        let result = Day::part2(&Day::parse_input_file());
        assert_eq!(result, 3783758);
    }
}
