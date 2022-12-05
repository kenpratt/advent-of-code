use std::collections::HashSet;
use std::fs;

use itertools::Itertools;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    println!("part 2 result: {:?}", part2(&read_input_file()));
}

fn read_input_file() -> String {
    fs::read_to_string("input.txt").expect("Something went wrong reading the file")
}

fn common_char(strings: &[&str]) -> char {
    let in_common = strings
        .iter()
        .map(|s| s.chars().collect::<HashSet<char>>())
        .reduce(|acc, set| acc.intersection(&set).cloned().collect())
        .unwrap();
    assert_eq!(in_common.len(), 1);
    in_common.into_iter().next().unwrap()
}

fn item_priority(item: &char) -> usize {
    let n = *item as usize;
    match n {
        // a..z
        97..=122 => (n - 97) + 1, // a=1
        // A..Z
        65..=90 => (n - 65) + 27, // A=27
        _ => panic!("Unexpected char: {}", item),
    }
}

fn part1(input: &str) -> usize {
    let in_common: Vec<char> = input
        .lines()
        .map(|line| common_char_in_rucksack_compartments(line))
        .collect();
    println!("in_common: {:?}", in_common);

    let priorities: Vec<usize> = in_common.iter().map(|item| item_priority(item)).collect();
    println!("priorities: {:?}", priorities);

    let sum = priorities.iter().sum();
    println!("sum: {:?}", sum);

    sum
}

fn common_char_in_rucksack_compartments(input: &str) -> char {
    assert!(input.len() % 2 == 0);
    let midpoint = input.len() / 2;
    let first_compartment = &input[0..midpoint];
    let second_compartment = &input[midpoint..];
    common_char(&[first_compartment, second_compartment])
}

fn part2(input: &str) -> usize {
    let in_common: Vec<char> = input
        .lines()
        .chunks(3)
        .into_iter()
        .map(|chunk| chunk.collect::<Vec<&str>>())
        .map(|chunk| common_char(&chunk))
        .collect();
    println!("in_common: {:?}", in_common);

    let priorities: Vec<usize> = in_common.iter().map(|item| item_priority(item)).collect();
    println!("priorities: {:?}", priorities);

    let sum = priorities.iter().sum();
    println!("sum: {:?}", sum);

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    static EXAMPLE1: &str = indoc! {"
        vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw
    "};

    #[test]
    fn test_part1_example1() {
        let result = part1(EXAMPLE1);
        assert_eq!(result, 157);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file());
        assert_eq!(result, 7597);
    }

    #[test]
    fn test_part2_example1() {
        let result = part2(EXAMPLE1);
        assert_eq!(result, 70);
    }

    #[test]
    fn test_part2_solution() {
        let result = part2(&read_input_file());
        assert_eq!(result, 2607);
    }
}
