use crate::file::*;

use lazy_static::lazy_static;
use regex::Regex;

pub fn run() {
    let input = parse(&read_input_file!());
    println!("part 1 result: {:?}", part1(&input));
    println!("part 2 result: {:?}", part2(&input));
}

#[allow(dead_code)]
#[derive(Debug)]
struct Item {
    foo: String,
    bar: usize,
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
        let foo = caps.get(1).unwrap().as_str().to_string();
        let bar = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
        Self { foo, bar }
    }
}

fn parse(input: &str) -> Vec<Item> {
    Item::parse_list(input)
}

fn part1(items: &[Item]) -> usize {
    dbg!(&items);
    0
}

fn part2(_items: &[Item]) -> usize {
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
