use std::{collections::HashMap, fs};

use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    // println!("part 2 result: {:?}", part2(&read_input_file()));
}

fn read_input_file() -> String {
    fs::read_to_string("input.txt").expect("Something went wrong reading the file")
}

#[derive(Debug)]
struct Configuration<'a> {
    modules: HashMap<&'a str, Module<'a>>,
}

impl<'a> Configuration<'a> {
    fn parse(input: &'a str) -> Self {
        let modules = Module::parse_list(input)
            .into_iter()
            .map(|m| (m.name, m))
            .collect();
        Self { modules }
    }
}

#[derive(Debug)]
struct Module<'a> {
    name: &'a str,
    kind: ModuleType,
    destinations: Vec<&'a str>,
}

impl<'a> Module<'a> {
    fn parse_list(input: &'a str) -> Vec<Self> {
        input.lines().map(|line| Self::parse(line)).collect()
    }

    fn parse(input: &'a str) -> Self {
        lazy_static! {
            static ref MODULE_RE: Regex =
                Regex::new(r"\A([%&]?)([a-z]+) \-> ([a-z ,]+)\z").unwrap();
        }

        let caps = MODULE_RE.captures(input).unwrap();
        let kind = ModuleType::parse(caps.get(1).unwrap().as_str());
        let name = caps.get(2).unwrap().as_str();
        let destinations = caps.get(3).unwrap().as_str().split(", ").collect();

        if kind == ModuleType::Broadcast {
            assert_eq!(name, "broadcaster");
        }

        Self {
            name,
            kind,
            destinations,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum ModuleType {
    FlipFlop,
    Conjunction,
    Broadcast,
}

impl ModuleType {
    fn parse(input: &str) -> Self {
        use ModuleType::*;

        match input {
            "%" => FlipFlop,
            "&" => Conjunction,
            "" => Broadcast,
            _ => panic!("Unexpected module type: {:?}", input),
        }
    }
}

fn part1(input: &str) -> usize {
    let config = Configuration::parse(input);
    dbg!(&config);
    0
}

// fn part2(input: &str) -> usize {
//     let items = Data::parse(input);
//     dbg!(&items);
//     0
// }

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    static EXAMPLE1: &str = indoc! {"
        broadcaster -> a, b, c
        %a -> b
        %b -> c
        %c -> inv
        &inv -> a
    "};

    static EXAMPLE2: &str = indoc! {"
        broadcaster -> a
        %a -> inv, con
        &inv -> b
        %b -> con
        &con -> output
    "};

    #[test]
    fn test_part1_example1() {
        let result = part1(EXAMPLE1);
        assert_eq!(result, 32000000);
    }

    #[test]
    fn test_part1_example2() {
        let result = part1(EXAMPLE2);
        assert_eq!(result, 11687500);
    }

    // #[test]
    // fn test_part1_solution() {
    //     let result = part1(&read_input_file());
    //     assert_eq!(result, 0);
    // }

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
