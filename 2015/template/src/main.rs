use std::fs;

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

fn part1(input: &str) -> usize {
    let items = Item::parse_list(input);
    dbg!(&items);
    0
}

// fn part2(input: &str) -> usize {
// }

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_FILE: &'static str = "example.txt";

    #[test]
    fn test_part1_example() {
        let result = part1(&read_file(EXAMPLE_FILE));
        assert_eq!(result, 0);
    }

    // #[test]
    // fn test_part1_solution() {
    //     let result = part1(&read_file(INPUT_FILE));
    //     assert_eq!(result, 0);
    // }

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