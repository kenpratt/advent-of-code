use crate::file::*;
use crate::spatial::Coord;

use lazy_static::lazy_static;
use regex::Regex;

pub fn run() {
    let input = parse(&read_input_file!());
    println!("part 1 result: {:?}", part1(&input));
    println!("part 2 result: {:?}", part2(&input));
}

type NumT = i16;
type CoordT = Coord<NumT>;

#[derive(Debug)]
struct Point {
    position: CoordT,
    velocity: CoordT,
}

impl Point {
    fn parse_list(input: &str) -> Vec<Self> {
        input.lines().map(|line| Self::parse(line)).collect()
    }

    fn parse(input: &str) -> Self {
        lazy_static! {
            static ref POINT_RE: Regex =
                Regex::new(r"\Aposition=<(.+),(.+)> velocity=<(.+),(.+)>\z").unwrap();
        }

        let caps = POINT_RE.captures(input).unwrap();
        let mut nums = caps
            .iter()
            .skip(1)
            .flat_map(|c| c.unwrap().as_str().trim().parse::<NumT>());

        let position = Coord::new(nums.next().unwrap(), nums.next().unwrap());
        let velocity = Coord::new(nums.next().unwrap(), nums.next().unwrap());
        assert_eq!(nums.next(), None);

        Self { position, velocity }
    }
}

fn parse(input: &str) -> Vec<Point> {
    Point::parse_list(input)
}

fn part1(points: &[Point]) -> usize {
    dbg!(&points);
    0
}

fn part2(_points: &[Point]) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        let result = part1(&parse(&read_example_file!()));
        assert_eq!(result, 0);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&parse(&read_input_file!()));
        assert_eq!(result, 0);
    }

    #[test]
    fn test_part2_example() {
        let result = part2(&parse(&read_example_file!()));
        assert_eq!(result, 0);
    }

    #[test]
    fn test_part2_solution() {
        let result = part2(&parse(&read_input_file!()));
        assert_eq!(result, 0);
    }
}
