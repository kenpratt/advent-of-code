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

    fn apply_lit(&self, on: &bool) -> bool {
        match self {
            Action::TurnOn => true,
            Action::TurnOff => false,
            Action::Toggle => !on,
        }
    }

    fn apply_brightness(&self, brightness: &u8) -> u8 {
        match self {
            Action::TurnOn => brightness + 1,
            Action::TurnOff => {
                if *brightness > 1 {
                    brightness - 1
                } else {
                    0
                }
            }
            Action::Toggle => brightness + 2,
        }
    }
}

#[derive(Debug)]
struct LightArray<V> {
    grid: Grid<u32, V>,
}

impl<V> LightArray<V> {
    fn new(initial: V) -> Self
    where
        V: Copy,
    {
        let bounds = Bounds::new(0, 999, 0, 999);
        let grid = Grid::new(bounds, || Some(initial));
        Self { grid }
    }

    fn run<F>(&mut self, instructions: &[Instruction], apply_action: F)
    where
        F: Fn(&Action, &V) -> V,
    {
        for instruction in instructions {
            self.apply_instruction(instruction, &apply_action);
        }
    }

    fn apply_instruction<F>(&mut self, instruction: &Instruction, apply_action: F)
    where
        F: Fn(&Action, &V) -> V,
    {
        for y in instruction.start.y..=instruction.end.y {
            for x in instruction.start.x..=instruction.end.x {
                let pos = Coord::new(x, y);
                let val = self.grid.get_mut(self.grid.coord_to_index(&pos));
                let new_val = apply_action(&instruction.action, val.as_ref().unwrap());
                *val = Some(new_val);
            }
        }
    }

    fn sum_by<F>(&self, f: F) -> usize
    where
        F: Fn(&V) -> usize,
    {
        self.grid.iter().map(|(_index, v)| f(v)).sum()
    }
}

fn part1(input: &str) -> usize {
    let instructions = Instruction::parse_list(input);
    let mut lights: LightArray<bool> = LightArray::new(false);
    lights.run(&instructions, |action, on| action.apply_lit(on));
    lights.sum_by(|b| if *b { 1 } else { 0 })
}

fn part2(input: &str) -> usize {
    let instructions = Instruction::parse_list(input);
    let mut lights: LightArray<u8> = LightArray::new(0);
    lights.run(&instructions, |action, brightness| {
        action.apply_brightness(brightness)
    });
    lights.sum_by(|b| *b as usize)
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
        assert_eq!(result, 1001996);
    }

    #[test]
    fn test_part2_solution() {
        let result = part2(&read_file(INPUT_FILE));
        assert_eq!(result, 17836115);
    }
}
