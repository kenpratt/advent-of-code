use std::collections::HashMap;
use std::fs;

use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    // println!("part 2 result: {:?}", part2(&read_input_file()));
}

fn read_input_file() -> String {
    return fs::read_to_string("input.txt").expect("Something went wrong reading the file");
}

#[derive(Debug)]
struct Program {
    instructions: Vec<Instruction>,
    state: ProgramState,
}

impl Program {
    fn parse(input: &str) -> Program {
        let instructions = input.lines().map(|line| Instruction::parse(line)).collect();
        return Program {
            instructions: instructions,
            state: ProgramState::new(),
        }
    }
    
    fn execute(&mut self) {
        for instruction in &self.instructions {
            self.state.execute_instruction(instruction);
        }
    }
    
    fn sum_of_memory(&self) -> u64 {
        return self.state.sum_of_memory();
    }
}

#[derive(Debug)]
struct ProgramState {
    mask: (u64, u64),
    memory: HashMap<u64, u64>,
}

impl ProgramState {
    fn new() -> ProgramState {
        return ProgramState {
            mask: (0, u64::MAX),
            memory: HashMap::new(),
        }
    }

    fn execute_instruction(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Mask(and_mask, or_mask) => {
                self.mask = (*and_mask, *or_mask);
                println!("exec mask {} {}", and_mask, or_mask);
            },
            Instruction::Write(address, value) => {
                let masked_value = self.apply_mask(value);
                println!("exec write {} {} -> {}", address, value, masked_value);
                self.memory.insert(*address, masked_value);
            },
        }
    }

    fn apply_mask(&self, value: &u64) -> u64 {
        let (and_mask, or_mask) = self.mask;
        return (value & and_mask) | or_mask;
    }

    fn sum_of_memory(&self) -> u64 {
        return self.memory.values().fold(0, |acc, x| acc + x);
    }
}

#[derive(Debug)]
enum Instruction {
    Mask(u64, u64),
    Write(u64, u64)
}

impl Instruction {
    fn parse(input: &str) -> Instruction {
        lazy_static! {
            static ref MASK_RE: Regex = Regex::new(r"^mask = ([X01]+)$").unwrap();
        }

        lazy_static! {
            static ref WRITE_RE: Regex = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();
        }
                
        if MASK_RE.is_match(input) {
            let captures = MASK_RE.captures(input).unwrap();
            let mask_str = captures.get(1).unwrap().as_str();

            let and_mask_str: String = mask_str.chars().map(|c| if c == '0' {'0'} else {'1'}).collect();
            let and_mask = u64::from_str_radix(&and_mask_str, 2).unwrap();

            let or_mask_str: String = mask_str.chars().map(|c| if c == '1' {'1'} else {'0'}).collect();
            let or_mask = u64::from_str_radix(&or_mask_str, 2).unwrap();

            println!("mask {}, {}, {}, {}, {}", mask_str, and_mask_str, and_mask, or_mask_str, or_mask);
            return Instruction::Mask(and_mask, or_mask);
        } else if WRITE_RE.is_match(input) {
            let captures = WRITE_RE.captures(input).unwrap();
            let address = captures.get(1).unwrap().as_str().parse::<u64>().unwrap();
            let value = captures.get(2).unwrap().as_str().parse::<u64>().unwrap();
            println!("write {}, {}", address, value);
            return Instruction::Write(address, value);
        } else {
            panic!("Invalid input line: {}", input)
        }
    }
}

fn part1(input: &str) -> u64 {
    let mut program = Program::parse(input);
    program.execute();
    return program.sum_of_memory();
}

// fn part2(input: &str) -> u64 {
//     let program = Program::parse(input);
//     return program.execute();
// }

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    static EXAMPLE1: &str = indoc! {"
        mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
        mem[8] = 11
        mem[7] = 101
        mem[8] = 0
    "};    

    #[test]
    fn test_part1_example1() {
        let result = part1(EXAMPLE1);
        assert_eq!(result, 165);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file());
        assert_eq!(result, 11926135976176);
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