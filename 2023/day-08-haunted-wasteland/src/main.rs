use std::{collections::HashMap, fs};

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
struct Navigation {
    instructions: Vec<Direction>,
    nodes: HashMap<String, (String, String)>,
}

impl Navigation {
    fn parse(input: &str) -> Self {
        let parts: Vec<&str> = input.split("\n\n").collect();
        assert_eq!(parts.len(), 2);

        let instructions = Direction::parse_list(&parts[0]);
        let nodes = parts[1]
            .lines()
            .map(|line| Self::parse_node(line))
            .collect();

        Self {
            instructions,
            nodes,
        }
    }

    fn parse_node(input: &str) -> (String, (String, String)) {
        lazy_static! {
            static ref NODE_RE: Regex = Regex::new(r"\A(\w+) = \((\w+), (\w+)\)\z").unwrap();
        }

        let caps = NODE_RE.captures(input).unwrap();
        let node = caps.get(1).unwrap().as_str().to_string();
        let left = caps.get(2).unwrap().as_str().to_string();
        let right = caps.get(3).unwrap().as_str().to_string();
        (node, (left, right))
    }

    fn navigate(&self, node: &str, instruction: &Direction) -> &str {
        let pair = self.nodes.get(node).unwrap();
        match instruction {
            Direction::Left => &pair.0,
            Direction::Right => &pair.1,
        }
    }

    fn run(&self, start_suffix: &str, end_suffix: &str) -> usize {
        let starting = self.starting_nodes(start_suffix);

        let steps: Vec<usize> = starting
            .into_iter()
            .map(|node| self.run_single(node, end_suffix))
            .collect();

        lcm(&steps)
    }

    fn starting_nodes(&self, suffix: &str) -> Vec<&str> {
        self.nodes
            .keys()
            .filter(|s| s.ends_with(suffix))
            .map(|s| s.as_str())
            .collect()
    }

    fn run_single(&self, start: &str, end_suffix: &str) -> usize {
        let mut instructions_iter = self.instructions.iter().cycle();

        let mut curr = start;
        let mut steps = 0;

        while !curr.ends_with(end_suffix) {
            let instruction = instructions_iter.next().unwrap();
            curr = self.navigate(curr, instruction);
            steps += 1;
        }

        steps
    }
}

fn lcm(values: &[usize]) -> usize {
    values
        .iter()
        .cloned()
        .reduce(|acc, x| num::integer::lcm(acc, x))
        .unwrap()
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    fn parse_list(input: &str) -> Vec<Self> {
        input.chars().map(|c| Self::parse(&c)).collect()
    }

    fn parse(input: &char) -> Self {
        use Direction::*;
        match input {
            'L' => Left,
            'R' => Right,
            _ => panic!("Unknown direction: {}", input),
        }
    }
}

fn part1(input: &str) -> usize {
    let navigation = Navigation::parse(input);
    navigation.run(&"AAA", &"ZZZ")
}

fn part2(input: &str) -> usize {
    let navigation = Navigation::parse(input);
    navigation.run(&"A", &"Z")
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    static EXAMPLE1: &str = indoc! {"
        RL

        AAA = (BBB, CCC)
        BBB = (DDD, EEE)
        CCC = (ZZZ, GGG)
        DDD = (DDD, DDD)
        EEE = (EEE, EEE)
        GGG = (GGG, GGG)
        ZZZ = (ZZZ, ZZZ)
    "};

    static EXAMPLE2: &str = indoc! {"
        LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)
    "};

    static EXAMPLE3: &str = indoc! {"
        LR

        11A = (11B, XXX)
        11B = (XXX, 11Z)
        11Z = (11B, XXX)
        22A = (22B, XXX)
        22B = (22C, 22C)
        22C = (22Z, 22Z)
        22Z = (22B, 22B)
        XXX = (XXX, XXX)
    "};

    #[test]
    fn test_part1_example1() {
        let result = part1(EXAMPLE1);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_part1_example2() {
        let result = part1(EXAMPLE2);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file());
        assert_eq!(result, 21389);
    }

    #[test]
    fn test_part2_example3() {
        let result = part2(EXAMPLE3);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_part2_solution() {
        let result = part2(&read_input_file());
        assert_eq!(result, 21083806112641);
    }
}
