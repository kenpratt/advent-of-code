use std::fs;
use std::iter::zip;

use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    println!("part 2 result: {:?}", part2(&read_input_file()));
}

fn read_input_file() -> String {
    fs::read_to_string("input.txt").expect("Something went wrong reading the file")
}

#[derive(Debug)]
struct Race {
    duration: usize,
    record: usize,
}

impl Race {
    fn parse_list(input: &str, collapse_numbers: bool) -> Vec<Self> {
        lazy_static! {
            static ref INPUT_RE: Regex =
                Regex::new(r"\ATime:\s+([\s\d]+)\nDistance:\s+([\s\d]+)\n?\z").unwrap();
        }

        let caps = INPUT_RE.captures(input).unwrap();
        let durations = parse_number_list(caps.get(1).unwrap().as_str(), collapse_numbers);
        let records = parse_number_list(caps.get(2).unwrap().as_str(), collapse_numbers);

        zip(durations, records)
            .map(|(duration, record)| Race { duration, record })
            .collect()
    }

    fn num_strategies_beating_record(&self) -> usize {
        let hold_times: Vec<usize> = (0..=self.duration).collect();

        let discard = hold_times.partition_point(|hold| self.distance(hold) <= self.record);
        let winning =
            hold_times[discard..].partition_point(|hold| self.distance(hold) > self.record);
        winning
    }

    fn distance(&self, hold: &usize) -> usize {
        hold * (self.duration - hold)
    }
}

fn parse_number_list(input: &str, collapse_numbers: bool) -> Vec<usize> {
    if collapse_numbers {
        let num = input
            .split_whitespace()
            .collect::<String>()
            .parse::<usize>()
            .unwrap();
        vec![num]
    } else {
        input
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect()
    }
}

fn part1(input: &str) -> usize {
    let races = Race::parse_list(input, false);
    races
        .iter()
        .map(|race| race.num_strategies_beating_record())
        .product()
}

fn part2(input: &str) -> usize {
    let races = Race::parse_list(input, true);
    races
        .iter()
        .map(|race| race.num_strategies_beating_record())
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    static EXAMPLE: &str = indoc! {"
        Time:      7  15   30
        Distance:  9  40  200
    "};

    #[test]
    fn test_part1_example() {
        let result = part1(EXAMPLE);
        assert_eq!(result, 288);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file());
        assert_eq!(result, 3316275);
    }

    #[test]
    fn test_part2_example() {
        let result = part2(EXAMPLE);
        assert_eq!(result, 71503);
    }

    #[test]
    fn test_part2_solution() {
        let result = part2(&read_input_file());
        assert_eq!(result, 27102791);
    }
}
