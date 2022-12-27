use std::fs;

// use itertools::Itertools;
// use lazy_static::lazy_static;
// use regex::Regex;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    // println!("part 2 result: {:?}", part2(&read_input_file()));
}

fn read_input_file() -> String {
    fs::read_to_string("input.txt").expect("Something went wrong reading the file")
}

#[derive(Debug)]
struct Data {
    parts: Vec<Part>,
}

impl Data {
    fn parse(input: &str) -> Self {
        let parts = input.lines().map(|line| Part::parse(line)).collect();
        Self { parts: parts }
    }

    fn execute(&self) -> usize {
        0
    }
}

#[derive(Debug)]
struct Part {
    foo: usize,
}

impl Part {
    fn parse(input: &str) -> Self {
        Self { foo: input.len() }
    }
}

fn part1(input: &str) -> usize {
    let data = Data::parse(input);
    dbg!(&data);
    data.execute()
}

// fn part2(input: &str) -> usize {
//     let data = Data::parse(input);
//     dbg!(&data);
//     data.execute()
// }

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    static EXAMPLE1: &str = indoc! {"
        Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
        Valve BB has flow rate=13; tunnels lead to valves CC, AA
        Valve CC has flow rate=2; tunnels lead to valves DD, BB
        Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
        Valve EE has flow rate=3; tunnels lead to valves FF, DD
        Valve FF has flow rate=0; tunnels lead to valves EE, GG
        Valve GG has flow rate=0; tunnels lead to valves FF, HH
        Valve HH has flow rate=22; tunnel leads to valve GG
        Valve II has flow rate=0; tunnels lead to valves AA, JJ
        Valve JJ has flow rate=21; tunnel leads to valve II
    "};

    #[test]
    fn test_part1_example1() {
        let result = part1(EXAMPLE1);
        assert_eq!(result, 0);
    }

    // #[test]
    // fn test_part1_solution() {
    //     let result = part1(&read_input_file());
    //     assert_eq!(result, 0);
    // }

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
