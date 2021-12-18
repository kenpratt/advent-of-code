use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::hash::Hash;

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

lazy_static! {
    static ref RULE_RE: Regex = Regex::new(r"\A([A-Z])([A-Z]) \-> ([A-Z])\z").unwrap();
}

type Pair = (char, char);

#[derive(Debug)]
struct Counter<T>(HashMap<T, usize>);

impl<T: Copy + Hash + Eq> Counter<T> {
    fn new() -> Counter<T> {
        Counter(HashMap::new())
    }

    fn add(&mut self, key: &T, increment: usize) {
        let counter = self.0.entry(*key).or_insert(0);
        *counter += increment;
    }

    fn min(&self) -> usize {
        *self.0.values().min().unwrap()
    }

    fn max(&self) -> usize {
        *self.0.values().max().unwrap()
    }
}

#[derive(Debug)]
struct Manual {
    template: Vec<char>,
    rules: HashMap<Pair, char>,
    pair_output_map: HashMap<Pair, (Pair, Pair)>,
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
        let pair_output_map = Manual::build_pair_output_map(&rules);
        Manual {
            template,
            rules,
            pair_output_map,
        }
    }

    fn parse_rule(input: &str) -> (Pair, char) {
        let captures = RULE_RE.captures(input).unwrap();
        let input1 = captures.get(1).unwrap().as_str().chars().next().unwrap();
        let input2 = captures.get(2).unwrap().as_str().chars().next().unwrap();
        let output = captures.get(3).unwrap().as_str().chars().next().unwrap();
        ((input1, input2), output)
    }

    fn frequencies_after_n_iterations(&self, n: usize) -> Counter<char> {
        let mut counts = self.starting_pair_counts();
        for _ in 1..=n {
            counts = self.iterate(&counts);
        }

        let mut element_frequencies = Counter::new();
        for (pair, final_count) in &counts.0 {
            element_frequencies.add(&pair.0, *final_count);
        }

        // need to add the last char in
        element_frequencies.add(self.template.last().unwrap(), 1);

        element_frequencies
    }

    fn iterate(&self, last_counts: &Counter<Pair>) -> Counter<Pair> {
        let mut counts = Counter::new();
        for (last_pair, last_count) in &last_counts.0 {
            let (next_pair1, next_pair2) = self.pair_output_map.get(last_pair).unwrap();
            counts.add(next_pair1, *last_count);
            counts.add(next_pair2, *last_count);
        }
        counts
    }

    fn starting_pair_counts(&self) -> Counter<Pair> {
        let mut counts = Counter::new();
        for pair in self.template.iter().cloned().tuple_windows() {
            counts.add(&pair, 1);
        }
        counts
    }

    fn possible_elements(rules: &HashMap<Pair, char>) -> HashSet<char> {
        let mut out: HashSet<char> = HashSet::new();
        for ((i1, i2), o) in rules {
            out.insert(*i1);
            out.insert(*i2);
            out.insert(*o);
        }
        out
    }

    fn possible_element_pairs(rules: &HashMap<Pair, char>) -> Vec<Pair> {
        let mut pairs: Vec<Pair> = vec![];
        let elements = Manual::possible_elements(rules);
        for c1 in &elements {
            for c2 in &elements {
                pairs.push((*c1, *c2));
            }
        }
        pairs
    }

    fn build_pair_output_map(rules: &HashMap<Pair, char>) -> HashMap<Pair, (Pair, Pair)> {
        Manual::possible_element_pairs(rules)
            .iter()
            .map(|pair| (*pair, Manual::output_for_pair(pair, rules)))
            .collect()
    }

    fn output_for_pair(pair: &Pair, rules: &HashMap<Pair, char>) -> (Pair, Pair) {
        match rules.get(pair) {
            Some(new_char) => ((pair.0, *new_char), (*new_char, pair.1)),
            None => panic!("Not expected"),
        }
    }
}

fn run_n_iterations_and_return_frequency_difference(input: &str, n: usize) -> usize {
    let manual = Manual::parse(input);
    let frequencies = manual.frequencies_after_n_iterations(n);
    frequencies.max() - frequencies.min()
}

fn part1(input: &str) -> usize {
    run_n_iterations_and_return_frequency_difference(input, 10)
}

fn part2(input: &str) -> usize {
    run_n_iterations_and_return_frequency_difference(input, 40)
}

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

    #[test]
    fn test_part2_example1() {
        let result = part2(EXAMPLE1);
        assert_eq!(result, 2188189693529);
    }

    #[test]
    fn test_part2_solution() {
        let result = part2(&read_input_file());
        assert_eq!(result, 3906445077999);
    }
}
