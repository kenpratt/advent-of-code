use std::{collections::HashSet, fs};

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
struct Card {
    id: usize,
    winning: HashSet<usize>,
    have: HashSet<usize>,
}

impl Card {
    fn parse_list(input: &str) -> Vec<Self> {
        input.lines().map(|line| Self::parse(line)).collect()
    }

    fn parse(input: &str) -> Self {
        lazy_static! {
            static ref CARD_RE: Regex =
                Regex::new(r"\ACard\s+(\d+): ([\d\s]+) \| ([\d\s]+)\z").unwrap();
        }

        let caps = CARD_RE.captures(input).unwrap();
        let id = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let winning = Self::parse_cards(caps.get(2).unwrap().as_str());
        let have: HashSet<usize> = Self::parse_cards(caps.get(3).unwrap().as_str());
        Self { id, winning, have }
    }

    fn parse_cards(input: &str) -> HashSet<usize> {
        input
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect()
    }

    fn score(&self) -> usize {
        let nums = self.winning_numbers();
        if nums.len() > 0 {
            let base: usize = 2;
            base.pow(nums.len() as u32 - 1)
        } else {
            0
        }
    }

    fn winning_numbers(&self) -> HashSet<usize> {
        self.winning.intersection(&self.have).cloned().collect()
    }
}

fn part1(input: &str) -> usize {
    let cards = Card::parse_list(input);
    cards.iter().map(|card| card.score()).sum()
}

// fn part2(input: &str) -> usize {
//     let cards = Data::parse(input);
//     dbg!(&cards);
//     0
// }

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    static EXAMPLE: &str = indoc! {"
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
    "};

    #[test]
    fn test_part1_example() {
        let result = part1(EXAMPLE);
        assert_eq!(result, 13);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file());
        assert_eq!(result, 26346);
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
