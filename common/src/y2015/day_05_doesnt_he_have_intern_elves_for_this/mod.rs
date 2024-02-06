use crate::file::*;

use std::collections::{HashMap, HashSet};

use lazy_static::lazy_static;

lazy_static! {
    static ref NAUGHTY: HashSet<(char, char)> =
        HashSet::from([('a', 'b'), ('c', 'd'), ('p', 'q'), ('x', 'y')]);
    static ref VOWELS: HashSet<char> = HashSet::from(['a', 'e', 'i', 'o', 'u']);
}

pub fn run() {
    let input = read_input_file!();
    println!("part 1 result: {:?}", part1(&input));
    println!("part 2 result: {:?}", part2(&input));
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

fn part1(input: &str) -> usize {
    input.lines().filter(|line| is_nice1(line)).count()
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

fn part2(input: &str) -> usize {
    input.lines().filter(|line| is_nice2(line)).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_examples() {
        assert_eq!(part1("ugknbfddgicrmopn"), 1);
        assert_eq!(part1("aaa"), 1);
        assert_eq!(part1("jchzalrnumimnmhp"), 0);
        assert_eq!(part1("haegwjzuvuyypxyu"), 0);
        assert_eq!(part1("dvszwmarrgswjxmb"), 0);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file!());
        assert_eq!(result, 236);
    }

    #[test]
    fn test_part2_examples() {
        assert_eq!(part2("qjhvhtzxzqqjkmpb"), 1);
        assert_eq!(part2("xxyxx"), 1);
        assert_eq!(part2("uurcxstgmygtbstg"), 0);
        assert_eq!(part2("ieodomkazucvgmuy"), 0);
    }

    #[test]
    fn test_part2_solution() {
        let result = part2(&read_input_file!());
        assert_eq!(result, 51);
    }
}
