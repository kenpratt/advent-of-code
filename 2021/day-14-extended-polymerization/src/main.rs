use std::collections::HashMap;
use std::fs;

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

lazy_static! {
    static ref RULE_RE: Regex = Regex::new(r"\A([A-Z])([A-Z]) \-> ([A-Z])\z").unwrap();
}

#[derive(Debug)]
struct Manual {
    template: Vec<char>,
    rules: HashMap<(char, char), char>,
}

impl Manual {
    fn parse(input: &str) -> Manual {
        let parts: Vec<&str> = input.split("\n\n").collect();
        assert_eq!(parts.len(), 2);

        let template = parts[0].chars().collect();
        let rules = parts[1]
            .lines()
            .map(|line| Manual::parse_rule(line))
            .collect();
        Manual { template, rules }
    }

    fn parse_rule(input: &str) -> ((char, char), char) {
        let captures = RULE_RE.captures(input).unwrap();
        let input1 = captures.get(1).unwrap().as_str().chars().next().unwrap();
        let input2 = captures.get(2).unwrap().as_str().chars().next().unwrap();
        let output = captures.get(3).unwrap().as_str().chars().next().unwrap();
        ((input1, input2), output)
    }

    fn run_n_iterations(&self, n: usize) -> Vec<char> {
        let mut value = self.template.clone();
        for _ in 1..=n {
            value = self.iterate(&value);
        }
        value
    }

    fn iterate(&self, value: &[char]) -> Vec<char> {
        let mut result: Vec<char> = vec![];
        for pair in value.iter().cloned().tuple_windows::<(_, _)>() {
            // push first char
            result.push(pair.0);

            // push new middle char, if applicable
            match self.rules.get(&pair) {
                Some(foo) => result.push(*foo),
                None => {}
            }
        }

        // don't forget the last char
        result.push(*value.last().unwrap());

        result
    }
}

fn frequency_per_char(input: &[char]) -> HashMap<char, usize> {
    let mut result = HashMap::new();
    for char in input.iter() {
        let counter = result.entry(*char).or_insert(0);
        *counter += 1;
    }
    result
}

fn part1(input: &str) -> usize {
    let manual = Manual::parse(input);
    println!("{:?}", manual);
    let result = manual.run_n_iterations(10);
    let frequencies = frequency_per_char(&result);
    println!("{:?}", frequencies);
    let most_frequent = frequencies.values().max().unwrap();
    let least_frequent = frequencies.values().min().unwrap();
    most_frequent - least_frequent
}

// fn part2(input: &str) -> usize {
//     let data = Manual::parse(input);
//     println!("{:?}", data);
//     data.execute()
// }

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    static EXAMPLE1: &str = indoc! {"
        NNCB

        CH -> B
        HH -> N
        CB -> H
        NH -> C
        HB -> C
        HC -> B
        HN -> C
        NN -> C
        BH -> H
        NC -> B
        NB -> B
        BN -> B
        BB -> N
        BC -> B
        CC -> N
        CN -> C
    "};

    #[test]
    fn test_part1_example1() {
        let result = part1(EXAMPLE1);
        assert_eq!(result, 1588);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file());
        assert_eq!(result, 3587);
    }

    // #[test]
    // fn test_part2_example1() {
    //     let result = part2(EXAMPLE1);
    //     assert_eq!(result, 0);
    // }

    // #[test]
    // fn test_part2_solution() {
    //     let result = part2(&read_input_file());
    //     assert_eq!(result, 0);
    // }
}
