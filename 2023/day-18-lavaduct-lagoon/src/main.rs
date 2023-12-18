pub mod grid;

use std::fs;

use grid::*;

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
struct Instruction {
    direction: Direction,
    distance: usize,
    colour: String,
}

impl Instruction {
    fn parse_list(input: &str) -> Vec<Self> {
        input.lines().map(|line| Self::parse(line)).collect()
    }

    fn parse(input: &str) -> Self {
        lazy_static! {
            static ref INSTRUCTION_RE: Regex =
                Regex::new(r"\A([UDLR]) (\d+) \(#([0-9a-f]{6})\)\z").unwrap();
        }

        let caps = INSTRUCTION_RE.captures(input).unwrap();
        let direction = Self::parse_direction(caps.get(1).unwrap().as_str());
        let distance = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
        let colour = caps.get(3).unwrap().as_str().to_string();

        Self {
            direction,
            distance,
            colour,
        }
    }

    fn parse_direction(input: &str) -> Direction {
        match input {
            "U" => Direction::North,
            "D" => Direction::South,
            "L" => Direction::West,
            "R" => Direction::East,
            _ => panic!("Unreachable"),
        }
    }
}

fn part1(input: &str) -> usize {
    let instructions = Instruction::parse_list(input);
    dbg!(&instructions);
    0
}

// fn part2(input: &str) -> usize {
//     let instructions = Data::parse(input);
//     dbg!(&instructions);
//     0
// }

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    static EXAMPLE: &str = indoc! {"
        R 6 (#70c710)
        D 5 (#0dc571)
        L 2 (#5713f0)
        D 2 (#d2c081)
        R 2 (#59c680)
        D 2 (#411b91)
        L 5 (#8ceee2)
        U 2 (#caa173)
        L 1 (#1b58a2)
        U 2 (#caa171)
        R 2 (#7807d2)
        U 3 (#a77fa3)
        L 2 (#015232)
        U 2 (#7a21e3)
    "};

    #[test]
    fn test_part1_example() {
        let result = part1(EXAMPLE);
        assert_eq!(result, 0);
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
