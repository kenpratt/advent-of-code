use std::{cmp::Ordering, fs};

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    // println!("part 2 result: {:?}", part2(&read_input_file()));
}

fn read_input_file() -> String {
    fs::read_to_string("input.txt").expect("Something went wrong reading the file")
}

#[derive(Debug, Eq, PartialEq)]
struct Round {
    hand: Hand,
    bid: usize,
}

impl Round {
    fn parse_list(input: &str) -> Vec<Self> {
        input.lines().map(|line| Self::parse(line)).collect()
    }

    fn parse(input: &str) -> Self {
        lazy_static! {
            static ref ROUND_RE: Regex = Regex::new(r"\A([1-9AKQJT]+) (\d+)\z").unwrap();
        }

        let caps = ROUND_RE.captures(input).unwrap();
        let hand = Hand::parse(caps.get(1).unwrap().as_str());
        let bid = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();

        Self { hand, bid }
    }
}

impl Ord for Round {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hand.cmp(&other.hand)
    }
}

impl PartialOrd for Round {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Hand {
    cards: [u8; 5],
    strength: u8,
}

impl Hand {
    fn parse(input: &str) -> Self {
        let cards = Self::parse_cards(input);
        let strength = Self::calculate_strength(&cards);

        Self { cards, strength }
    }

    fn parse_cards(input: &str) -> [u8; 5] {
        assert_eq!(input.len(), 5);
        input
            .chars()
            .map(|c| match c {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => 11,
                'T' => 10,
                '9' => 9,
                '8' => 8,
                '7' => 7,
                '6' => 6,
                '5' => 5,
                '4' => 4,
                '3' => 3,
                '2' => 2,
                '1' => 1,
                _ => panic!("Unknown card: {}", c),
            })
            .collect::<Vec<u8>>()
            .try_into()
            .unwrap()
    }

    fn calculate_strength(cards: &[u8; 5]) -> u8 {
        let counts = cards.iter().counts();
        let num_duplicates: Vec<&usize> = counts.values().sorted().rev().collect();

        match num_duplicates.as_slice() {
            [5] => 6,             // five of a kind
            [4, 1] => 5,          // four of a kind
            [3, 2] => 4,          // full house
            [3, 1, 1] => 3,       // three of a kind
            [2, 2, 1] => 2,       // two pair
            [2, 1, 1, 1] => 1,    // one pair
            [1, 1, 1, 1, 1] => 0, // high card
            _ => panic!("Unknown hand type: {:?}", num_duplicates),
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.strength
            .cmp(&other.strength)
            .then(self.cards.cmp(&other.cards))
            .reverse()
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn part1(input: &str) -> usize {
    let mut rounds = Round::parse_list(input);
    rounds.sort();

    let num_rounds = rounds.len();
    let winnings = rounds.iter().enumerate().map(|(index, round)| {
        let rank = num_rounds - index;
        rank * round.bid
    });
    winnings.sum()
}

// fn part2(input: &str) -> usize {
//     let hands = Data::parse(input);
//     dbg!(&hands);
//     0
// }

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    static EXAMPLE: &str = indoc! {"
        32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483
    "};

    #[test]
    fn test_part1_example() {
        let result = part1(EXAMPLE);
        assert_eq!(result, 6440);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file());
        assert_eq!(result, 250120186);
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
