use std::collections::BTreeSet;
use std::ops::Range;

use crate::interface::AoC;
use crate::spatial::*;

use lazy_static::lazy_static;
use regex::Regex;

pub struct Day;
impl AoC<(Vec<Instruction>, Windows), usize, usize> for Day {
    const FILE: &'static str = file!();

    fn parse(input: String) -> (Vec<Instruction>, Windows) {
        let instructions = Instruction::parse_list(&input);
        let windows = Windows::build(&instructions);
        (instructions, windows)
    }

    fn part1((instructions, windows): &(Vec<Instruction>, Windows)) -> usize {
        let mut lights: LightArray<bool> = LightArray::new(false, windows);
        lights.run(instructions, |action, on| action.apply_lit(on));
        lights.sum_by(|b| if *b { 1 } else { 0 })
    }

    fn part2((instructions, windows): &(Vec<Instruction>, Windows)) -> usize {
        let mut lights: LightArray<u8> = LightArray::new(0, windows);
        lights.run(instructions, |action, brightness| {
            action.apply_brightness(brightness)
        });
        lights.sum_by(|b| *b as usize)
    }
}

#[derive(Debug)]
pub struct Instruction {
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
pub struct Windows {
    x_splits: Vec<u32>,
    y_splits: Vec<u32>,
    width: usize,
    height: usize,
    sizes: Vec<u32>,
}

impl Windows {
    fn build(instructions: &[Instruction]) -> Self {
        let x_splits: Vec<u32> = instructions
            .iter()
            .fold(BTreeSet::new(), |mut set, inst| {
                set.insert(inst.start.x);
                set.insert(inst.end.x + 1);
                set
            })
            .into_iter()
            .collect();

        let y_splits: Vec<u32> = instructions
            .iter()
            .fold(BTreeSet::new(), |mut set, inst| {
                set.insert(inst.start.y);
                set.insert(inst.end.y + 1);
                set
            })
            .into_iter()
            .collect();

        let width = x_splits.len() - 1;
        let height = y_splits.len() - 1;

        let x_sizes: Vec<u32> = x_splits.windows(2).map(|w| w[1] - w[0]).collect();
        let y_sizes: Vec<u32> = y_splits.windows(2).map(|w| w[1] - w[0]).collect();

        let sizes = y_sizes
            .iter()
            .flat_map(|y| x_sizes.iter().map(|x| *x * *y))
            .collect();

        Self {
            x_splits,
            y_splits,
            width,
            height,
            sizes,
        }
    }

    fn range(splits: &[u32], start: u32, end: u32) -> Range<usize> {
        let from = splits.binary_search_by(|val| val.cmp(&start)).unwrap();
        let to = match splits[from..].binary_search_by(|val| val.cmp(&(end + 1))) {
            Ok(index) => from + index,
            Err(index) => from + index,
        };
        from..to
    }

    fn size(&self) -> usize {
        self.width * self.height
    }

    fn size_of(&self, index: usize) -> usize {
        self.sizes[index] as usize
    }

    fn apply_each_index<F>(&self, instruction: &Instruction, mut f: F)
    where
        F: FnMut(usize),
    {
        let x_range = Self::range(&self.x_splits, instruction.start.x, instruction.end.x);
        let y_range = Self::range(&self.y_splits, instruction.start.y, instruction.end.y);

        for yi in y_range {
            for xi in x_range.clone() {
                let i = self.index(xi, yi);
                f(i);
            }
        }
    }

    fn index(&self, xi: usize, yi: usize) -> usize {
        yi * self.width + xi
    }
}

#[derive(Debug)]
struct LightArray<'a, V> {
    windows: &'a Windows,
    values: Vec<V>,
}

impl<'a, V> LightArray<'a, V> {
    fn new(initial: V, windows: &'a Windows) -> Self
    where
        V: Copy,
    {
        let values = vec![initial; windows.size()];
        Self { windows, values }
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
        self.windows.apply_each_index(instruction, |i| {
            let new_val = apply_action(&instruction.action, &self.values[i]);
            self.values[i] = new_val;
        });
    }

    fn sum_by<F>(&self, f: F) -> usize
    where
        F: Fn(&V) -> usize,
    {
        self.values
            .iter()
            .enumerate()
            .map(|(i, v)| self.windows.size_of(i) * f(v))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        let result = Day::part1(&Day::parse_example_file());
        assert_eq!(result, 1000000 - 1000 - 4);
    }

    #[test]
    fn test_part1_solution() {
        let result = Day::part1(&Day::parse_input_file());
        assert_eq!(result, 569999);
    }

    #[test]
    fn test_part2_example() {
        let result = Day::part2(&Day::parse_example_file());
        assert_eq!(result, 1001996);
    }

    #[test]
    fn test_part2_solution() {
        let result = Day::part2(&Day::parse_input_file());
        assert_eq!(result, 17836115);
    }
}
