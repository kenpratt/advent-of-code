use crate::interface::AoC;

use lazy_static::lazy_static;
use regex::Regex;

pub struct Day;
impl AoC<Vec<Item>, usize, usize> for Day {
    const FILE: &'static str = file!();

    fn parse(input: String) -> Vec<Item> {
        Item::parse_list(&input)
    }

    fn part1(items: &Vec<Item>) -> usize {
        dbg!(&items);
        0
    }

    fn part2(_items: &Vec<Item>) -> usize {
        0
    }
}

#[derive(Debug)]
pub struct Item {
    _foo: String,
    _bar: usize,
}

impl Item {
    fn parse_list(input: &str) -> Vec<Self> {
        input.lines().map(|line| Self::parse(line)).collect()
    }

    fn parse(input: &str) -> Self {
        lazy_static! {
            static ref ITEM_RE: Regex = Regex::new(r"\A(.+)=(\d+)\z").unwrap();
        }

        let caps = ITEM_RE.captures(input).unwrap();
        let _foo = caps.get(1).unwrap().as_str().to_string();
        let _bar = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
        Self { _foo, _bar }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        let result = Day::part1(&Day::parse_example_file());
        assert_eq!(result, 0);
    }

    #[test]
    fn test_part1_solution() {
        let result = Day::part1(&Day::parse_input_file());
        assert_eq!(result, 0);
    }

    #[test]
    fn test_part2_example() {
        let result = Day::part2(&Day::parse_example_file());
        assert_eq!(result, 0);
    }

    #[test]
    fn test_part2_solution() {
        let result = Day::part2(&Day::parse_input_file());
        assert_eq!(result, 0);
    }
}
