use std::fs;
use std::ops::RangeInclusive;

use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    println!("part 2 result: {:?}", part2(&read_input_file()));
}

fn read_input_file() -> String {
    fs::read_to_string("input.txt").expect("Something went wrong reading the file")
}

type Pair = (RangeInclusive<usize>, RangeInclusive<usize>);

fn parse_pairs(input: &str) -> Vec<Pair> {
    input.lines().map(|line| parse_pair(line)).collect()
}

fn parse_pair(input: &str) -> Pair {
    lazy_static! {
        static ref PAIR_RE: Regex = Regex::new(r"\A(\d+)\-(\d+),(\d+)\-(\d+)\z").unwrap();
    }
    let caps = PAIR_RE.captures(input).unwrap();
    let nums: Vec<usize> = caps
        .iter()
        .skip(1)
        .map(|s| s.unwrap().as_str().parse::<usize>().unwrap())
        .collect();
    let pair = (nums[0]..=nums[1], nums[2]..=nums[3]);
    println!("{:?} -> {:?} -> {:?}", input, nums, pair);
    pair
}

fn fully_contains_range<U>(range: &RangeInclusive<U>, other: &RangeInclusive<U>) -> bool
where
    U: PartialOrd,
{
    range.contains(other.start()) && range.contains(other.end())
}

fn partially_contains_range<U>(range: &RangeInclusive<U>, other: &RangeInclusive<U>) -> bool
where
    U: PartialOrd,
{
    range.contains(other.start()) || range.contains(other.end())
}

fn part1(input: &str) -> usize {
    let pairs = parse_pairs(input);
    println!("{:?}", pairs);
    pairs
        .into_iter()
        .filter(|(a, b)| fully_contains_range(a, b) || fully_contains_range(b, a))
        .count()
}

fn part2(input: &str) -> usize {
    let pairs = parse_pairs(input);
    println!("{:?}", pairs);
    pairs
        .into_iter()
        .filter(|(a, b)| partially_contains_range(a, b) || partially_contains_range(b, a))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    static EXAMPLE1: &str = indoc! {"
        2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8
    "};

    #[test]
    fn test_part1_example1() {
        let result = part1(EXAMPLE1);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file());
        assert_eq!(result, 599);
    }

    #[test]
    fn test_part2_example1() {
        let result = part2(EXAMPLE1);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_part2_solution() {
        let result = part2(&read_input_file());
        assert_eq!(result, 928);
    }
}
