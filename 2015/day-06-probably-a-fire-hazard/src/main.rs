mod spatial;

use spatial::*;

use std::fs;

use lazy_static::lazy_static;
use regex::Regex;

const INPUT_FILE: &'static str = "input.txt";

fn main() {
    println!("part 1 result: {:?}", part1(&read_file(INPUT_FILE)));
    println!("part 2 result: {:?}", part2(&read_file(INPUT_FILE)));
}

fn read_file(filename: &str) -> String {
    fs::read_to_string(filename).expect("Something went wrong reading the file")
}

#[derive(Debug)]
struct Instruction {
    action: Action,
    start: Coord<u32>,
    end: Coord<u32>,
}

impl Instruction {
    fn parse_list(input: &str) -> Vec<Self> {
        input.lines().map(|line| Self::parse(line)).collect()
    }

    fn parse(input: &str) -> Self {
        lazy_static! {
            static ref INSTRUCTION_RE: Regex =
                Regex::new(r"\A(.+) (\d+,\d+) through (\d+,\d+)\z").unwrap();
        }

        let caps = INSTRUCTION_RE.captures(input).unwrap();
        let action = Action::parse(caps.get(1).unwrap().as_str());
        let start = Coord::parse(caps.get(2).unwrap().as_str(), ",");
        let end = Coord::parse(caps.get(3).unwrap().as_str(), ",");
        Self { action, start, end }
    }
}

#[derive(Debug)]
enum Action {
    TurnOn,
    TurnOff,
    Toggle,
}

impl Action {
    fn parse(input: &str) -> Self {
        match input {
            "turn on" => Action::TurnOn,
            "turn off" => Action::TurnOff,
            "toggle" => Action::Toggle,
            _ => panic!("Unexpected action: {:?}", input),
        }
    }

    fn apply(&self, on: bool) -> bool {
        match self {
            Action::TurnOn => true,
            Action::TurnOff => false,
            Action::Toggle => !on,
        }
    }
}

#[derive(Debug)]
struct LightArray {
    grid: Grid<u32, bool>,
}

impl LightArray {
    fn new() -> Self {
        let bounds = Bounds::new(0, 999, 0, 999);
        let grid = Grid::new(bounds, || Some(false));
        Self { grid }
    }

    fn run(&mut self, instructions: &[Instruction]) -> usize {
        for instruction in instructions {
            self.apply_instruction(instruction);
        }
        self.count_on()
    }

    fn apply_instruction(&mut self, instruction: &Instruction) {
        for y in instruction.start.y..=instruction.end.y {
            for x in instruction.start.x..=instruction.end.x {
                let pos = Coord::new(x, y);
                let val = self.grid.get_mut(self.grid.coord_to_index(&pos));
                let new_val = instruction.action.apply(val.unwrap());
                *val = Some(new_val);
            }
        }
    }

    fn count_on(&self) -> usize {
        self.grid.iter().filter(|(_index, v)| **v).count()
    }
}

fn part1(input: &str) -> usize {
    let instructions = Instruction::parse_list(input);
    let mut lights = LightArray::new();
    lights.run(&instructions)
}

fn part2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_FILE: &'static str = "example.txt";

    #[test]
    fn test_part1_example() {
        let result = part1(&read_file(EXAMPLE_FILE));
        assert_eq!(result, 1000000 - 1000 - 4);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_file(INPUT_FILE));
        assert_eq!(result, 569999);
    }

    #[test]
    fn test_part2_example() {
        let result = part2(&read_file(EXAMPLE_FILE));
        assert_eq!(result, 0);
    }

    #[test]
    fn test_part2_solution() {
        let result = part2(&read_file(INPUT_FILE));
        assert_eq!(result, 0);
    }
}
