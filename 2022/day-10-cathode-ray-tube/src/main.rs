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
enum Instruction {
    AddX(isize),
    Noop,
}

impl Instruction {
    fn parse_instructions(input: &str) -> Vec<Self> {
        input.lines().map(|line| Self::parse(line)).collect()
    }

    fn parse(input: &str) -> Self {
        use Instruction::*;

        let mut parts = input.split(' ');
        let inst = match parts.next().unwrap() {
            "addx" => AddX(parts.next().unwrap().parse::<isize>().unwrap()),
            "noop" => Noop,
            _ => panic!("Unknown instruction: {}", input),
        };
        assert_eq!(parts.next(), None);
        inst
    }
}

#[derive(Debug)]
struct CPU {
    x: isize,
    cycle_count: usize,
    x_history: Vec<isize>,
}

impl CPU {
    fn new() -> Self {
        Self {
            x: 1,
            cycle_count: 0,
            x_history: vec![],
        }
    }

    fn execute(&mut self, instructions: &[Instruction]) {
        for inst in instructions {
            self.execute_instruction(inst);
        }
    }

    fn execute_instruction(&mut self, instruction: &Instruction) {
        use Instruction::*;

        match instruction {
            Noop => {
                self.cycle_count += 1;
                self.x_history.push(self.x);
            }
            AddX(value) => {
                self.cycle_count += 1;
                self.x_history.push(self.x);

                self.x += value;
                self.cycle_count += 1;
                self.x_history.push(self.x);
            }
        }
    }
}

fn part1(input: &str) -> isize {
    let instructions = Instruction::parse_instructions(input);
    println!("{:?}", instructions);

    let mut cpu = CPU::new();
    println!("{:?}", cpu);

    cpu.execute(&instructions);

    // offset -1 for 0-based indexing, and -1 because we want the value during
    // the operation, and the register history is afterward
    let offset = 2;

    cpu.x_history
        .iter()
        .enumerate()
        .skip((20 - offset) as usize)
        .step_by(40)
        .map(|(cycle, x)| (cycle as isize + offset) * *x)
        .sum()
}

// fn part2(input: &str) -> usize {
//     let data = Instruction::parse(input);
//     println!("{:?}", data);
//     data.execute()
// }

#[cfg(test)]
mod tests {
    use super::*;

    fn read_example_file() -> String {
        fs::read_to_string("example.txt").expect("Something went wrong reading the example file")
    }

    #[test]
    fn test_part1_example1() {
        let result = part1(&read_example_file());
        assert_eq!(result, 13140);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file());
        assert_eq!(result, 17180);
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
