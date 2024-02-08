use crate::interface::AoC;

use std::collections::{HashMap, HashSet};

use lazy_static::lazy_static;

pub struct Day;
impl AoC<String, usize, usize> for Day {
    const FILE: &'static str = file!();

    fn parse(input: String) -> String {
        input
    }

    fn part1(input: &String) -> usize {
        input.lines().filter(|line| is_nice1(line)).count()
    }

    fn part2(input: &String) -> usize {
        input.lines().filter(|line| is_nice2(line)).count()
    }
}

lazy_static! {
    static ref NAUGHTY: HashSet<(char, char)> =
        HashSet::from([('a', 'b'), ('c', 'd'), ('p', 'q'), ('x', 'y')]);
    static ref VOWELS: HashSet<char> = HashSet::from(['a', 'e', 'i', 'o', 'u']);
}

fn is_nice1(input: &str) -> bool {
    let mut has_double_letter = false;
    let mut vowel_count = 0;

    let mut chars = input.chars();

    // special case for first char to simplify letter pair logic
    let mut last_char = chars.next().unwrap();
    if VOWELS.contains(&last_char) {
        vowel_count += 1;
    }

    for curr_char in chars {
        if NAUGHTY.contains(&(last_char, curr_char)) {
            return false;
        }

        if VOWELS.contains(&curr_char) {
            vowel_count += 1;
        }

        if curr_char == last_char {
            has_double_letter = true;
        }

        last_char = curr_char;
    }

    has_double_letter && vowel_count >= 3
}

fn is_nice2(input: &str) -> bool {
    let chars: Vec<char> = input.chars().collect();

    let has_sandwich = chars.windows(3).any(|w| w[0] == w[2]);
    if !has_sandwich {
        return false;
    }

    let mut seen_pairs: HashMap<&[char], usize> = HashMap::new();
    for (curr_index, pair) in chars.windows(2).enumerate() {
        match seen_pairs.get(&pair) {
            Some(existing_index) => {
                if curr_index - existing_index > 1 {
                    return true;
                }
            }
            None => {
                seen_pairs.insert(pair, curr_index);
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_examples() {
        assert_eq!(Day::part1(&Day::parse_str("ugknbfddgicrmopn")), 1);
        assert_eq!(Day::part1(&Day::parse_str("aaa")), 1);
        assert_eq!(Day::part1(&Day::parse_str("jchzalrnumimnmhp")), 0);
        assert_eq!(Day::part1(&Day::parse_str("haegwjzuvuyypxyu")), 0);
        assert_eq!(Day::part1(&Day::parse_str("dvszwmarrgswjxmb")), 0);
    }

    #[test]
    fn test_part1_solution() {
        let result = Day::part1(&Day::parse_input_file());
        assert_eq!(result, 236);
    }

    #[test]
    fn test_part2_examples() {
        assert_eq!(Day::part2(&Day::parse_str("qjhvhtzxzqqjkmpb")), 1);
        assert_eq!(Day::part2(&Day::parse_str("xxyxx")), 1);
        assert_eq!(Day::part2(&Day::parse_str("uurcxstgmygtbstg")), 0);
        assert_eq!(Day::part2(&Day::parse_str("ieodomkazucvgmuy")), 0);
    }

    #[test]
    fn test_part2_solution() {
        let result = Day::part2(&Day::parse_input_file());
        assert_eq!(result, 51);
    }
}
