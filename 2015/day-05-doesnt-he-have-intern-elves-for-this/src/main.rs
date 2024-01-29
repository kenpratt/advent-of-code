use std::{collections::HashSet, fs};

use lazy_static::lazy_static;

const INPUT_FILE: &'static str = "input.txt";

lazy_static! {
    static ref NAUGHTY: HashSet<(char, char)> =
        HashSet::from([('a', 'b'), ('c', 'd'), ('p', 'q'), ('x', 'y')]);
    static ref VOWELS: HashSet<char> = HashSet::from(['a', 'e', 'i', 'o', 'u']);
}

fn main() {
    println!("part 1 result: {:?}", part1(&read_file(INPUT_FILE)));
    // println!("part 2 result: {:?}", part2(&read_file(INPUT_FILE)));
}

fn read_file(filename: &str) -> String {
    fs::read_to_string(filename).expect("Something went wrong reading the file")
}

fn is_nice(input: &str) -> bool {
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
    input.lines().filter(|line| is_nice(line)).count()
}

// fn part2(input: &str) -> usize {
// }

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
        let result = part1(&read_file(INPUT_FILE));
        assert_eq!(result, 236);
    }

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
