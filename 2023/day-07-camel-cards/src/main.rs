use std::{cmp::Ordering, collections::HashMap, fs};

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    println!("part 2 result: {:?}", part2(&read_input_file()));
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
    fn parse_list(input: &str, use_jokers: bool) -> Vec<Self> {
        input
            .lines()
            .map(|line| Self::parse(line, use_jokers))
            .collect()
    }

    fn parse(input: &str, use_jokers: bool) -> Self {
        lazy_static! {
            static ref ROUND_RE: Regex = Regex::new(r"\A([2-9AKQJT]+) (\d+)\z").unwrap();
        }

        let caps = ROUND_RE.captures(input).unwrap();
        let hand = Hand::parse(caps.get(1).unwrap().as_str(), use_jokers);
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

const JOKER: u8 = 1;

impl Hand {
    fn parse(input: &str, use_jokers: bool) -> Self {
        let cards = Self::parse_cards(input, use_jokers);
        let strength = Self::calculate_strength(&cards);

        Self { cards, strength }
    }

    fn parse_cards(input: &str, use_jokers: bool) -> [u8; 5] {
        assert_eq!(input.len(), 5);
        input
            .chars()
            .map(|c| match c {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => {
                    if use_jokers {
                        JOKER
                    } else {
                        11
                    }
                }
                'T' => 10,
                '9' => 9,
                '8' => 8,
                '7' => 7,
                '6' => 6,
                '5' => 5,
                '4' => 4,
                '3' => 3,
                '2' => 2,
                _ => panic!("Unknown card: {}", c),
            })
            .collect::<Vec<u8>>()
            .try_into()
            .unwrap()
    }

    fn calculate_strength(cards: &[u8; 5]) -> u8 {
        let mut card_counts = cards.iter().cloned().counts();
        let num_jokers = *card_counts.get(&JOKER).unwrap_or(&0);

        match num_jokers {
            // special case, five of a kind of jokers is just a five of a kind hand
            0 | 5 => Self::calculate_strength_once(&card_counts),

            // for 1-4 jokers, need to run joker substitution logic
            1..=4 => {
                // remove jokers
                card_counts.remove(&JOKER);

                // add jokers back as any card value
                (2..=14)
                    .into_iter()
                    .map(|card| {
                        let mut modified_card_counts = card_counts.clone();

                        let entry = modified_card_counts.entry(card as u8);
                        *entry.or_default() += num_jokers;

                        Self::calculate_strength_once(&modified_card_counts)
                    })
                    .max()
                    .unwrap()
            }

            _ => panic!("Unexpected joker count: {}", num_jokers),
        }
    }

    fn calculate_strength_once(card_counts: &HashMap<u8, usize>) -> u8 {
        let counts: Vec<&usize> = card_counts.values().sorted().rev().collect();
        match counts.as_slice() {
            [5] => 6,             // five of a kind
            [4, 1] => 5,          // four of a kind
            [3, 2] => 4,          // full house
            [3, 1, 1] => 3,       // three of a kind
            [2, 2, 1] => 2,       // two pair
            [2, 1, 1, 1] => 1,    // one pair
            [1, 1, 1, 1, 1] => 0, // high card
            _ => panic!("Unknown hand type: {:?}", counts),
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

fn calculate_total_winnings(input: &str, use_jokers: bool) -> usize {
    let mut rounds = Round::parse_list(input, use_jokers);
    rounds.sort();

    let num_rounds = rounds.len();
    let winnings = rounds.iter().enumerate().map(|(index, round)| {
        let rank = num_rounds - index;
        rank * round.bid
    });
    winnings.sum()
}

fn part1(input: &str) -> usize {
    calculate_total_winnings(input, false)
}

fn part2(input: &str) -> usize {
    calculate_total_winnings(input, true)
}

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

    #[test]
    fn test_part2_example() {
        let result = part2(EXAMPLE);
        assert_eq!(result, 5905);
    }

    #[test]
    fn test_part2_solution() {
        let result = part2(&read_input_file());
        assert_eq!(result, 250665248);
    }
}
