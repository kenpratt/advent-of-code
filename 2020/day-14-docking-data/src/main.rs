use std::collections::HashMap;
use std::fs;

use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    println!("part 2 result: {:?}", part2(&read_input_file()));
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
    
    fn execute(&mut self, v2: bool) {
        for instruction in &self.instructions {
            self.state.execute_instruction(instruction, v2);
        }
    }
    
    fn sum_of_memory(&self) -> u64 {
        return self.state.sum_of_memory();
    }
}

#[derive(Debug)]
enum Instruction {
    Mask(String),
    Write(u64, u64),
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
            return Instruction::Mask(mask_str.to_string());
        } else if WRITE_RE.is_match(input) {
            let captures = WRITE_RE.captures(input).unwrap();
            let address = captures.get(1).unwrap().as_str().parse::<u64>().unwrap();
            let value = captures.get(2).unwrap().as_str().parse::<u64>().unwrap();
            return Instruction::Write(address, value);
        } else {
            panic!("Invalid input line: {}", input)
        }
    }
}

#[derive(Debug)]
struct ProgramState {
    mask: Mask,
    memory: HashMap<u64, u64>,
}

impl ProgramState {
    fn new() -> ProgramState {
        return ProgramState {
            mask: Mask::Simple((0, u64::MAX)),
            memory: HashMap::new(),
        }
    }

    fn execute_instruction(&mut self, instruction: &Instruction, v2: bool) {
        match instruction {
            Instruction::Mask(mask_str) => {
                self.mask = Mask::parse(mask_str, v2);
                println!("exec mask {:?}", self.mask);
            },
            Instruction::Write(address, value) => {
                if v2 {
                    self.v2_write(address, value);
                } else {
                    self.v1_write(address, value);
                }
            },
        }
    }

    fn v1_write(&mut self, address: &u64, value: &u64) {
        let (and_mask, or_mask) = match self.mask {
            Mask::Simple(m) => m,
            Mask::Floating(_) => panic!("Unreachable"),
        };

        let masked_value = (value & and_mask) | or_mask;

        println!("exec v1 write {} {} -> {}", address, value, masked_value);
        self.memory.insert(*address, masked_value);
    }

    fn v2_write(&mut self, address: &u64, value: &u64) {
        let floating_masks = match &self.mask {
            Mask::Simple(_) => panic!("Unreachable"),
            Mask::Floating(v) => v,
        };        

        let addresses: Vec<u64> = floating_masks.iter().map(|(and_mask, or_mask)| (address & and_mask) | or_mask).collect();

        println!("exec v2 write {} {} {:?}, {:?}", address, value, floating_masks, addresses);
        for address in addresses {
            self.memory.insert(address, *value);
        }
    }

    fn sum_of_memory(&self) -> u64 {
        return self.memory.values().fold(0, |acc, x| acc + x);
    }
}

#[derive(Debug)]
enum Mask {
    Simple((u64, u64)),
    Floating(Vec<(u64, u64)>),
}

impl Mask {
    fn parse(mask_str: &str, v2: bool) -> Mask {
        return if v2 {
            Mask::Floating(Mask::parse_floating_masks(mask_str))
        } else {
            Mask::Simple(Mask::parse_mask(mask_str))
        };
    }

    fn parse_mask(mask_str: &str) -> (u64, u64) {
        let and_mask_str: String = mask_str.chars().map(|c| if c == '0' {'0'} else {'1'}).collect();
        let and_mask = u64::from_str_radix(&and_mask_str, 2).unwrap();

        let or_mask_str: String = mask_str.chars().map(|c| if c == '1' {'1'} else {'0'}).collect();
        let or_mask = u64::from_str_radix(&or_mask_str, 2).unwrap();
        
        println!("mask: {} -> {}={}, {}={}", mask_str, and_mask_str, and_mask, or_mask_str, or_mask);
        return (and_mask, or_mask);
    }

    fn parse_floating_masks(mask_str: &str) -> Vec<(u64, u64)> {
        let mut permutations: Vec<Vec<char>> = vec![vec![]];

        for c in mask_str.chars() {
            match c {
                'X' => {
                    // Add both 0 and 1 overwrite permutations.
                    let mut new_permutations: Vec<Vec<char>> = vec![];
                    for p in &permutations {
                        let mut p0 = p.clone();
                        p0.push('0');
                        new_permutations.push(p0);

                        let mut p1 = p.clone();
                        p1.push('1');
                        new_permutations.push(p1);
                    }
                    permutations = new_permutations;
                },
                '0' => {
                    for p in &mut permutations {
                        // In v1, X format means ignore this char which is what
                        // we want for 0 in floating masks.
                        p.push('X');
                    }                    
                },
                '1' => {
                    for p in &mut permutations {
                        // 1 stays the same (overwrite with 1).
                        p.push('1');
                    }                    
                },
                _ => panic!("Unreachable"),
            }
        }

        let permutations_strs: Vec<String> = permutations.iter().map(|v| v.into_iter().collect()).collect();
        println!("permutations: {:?}", permutations_strs);

        return permutations_strs.iter().map(|s| Mask::parse_mask(s)).collect();
    }    
}

fn part1(input: &str) -> u64 {
    let mut program = Program::parse(input);
    program.execute(false);
    return program.sum_of_memory();
}

fn part2(input: &str) -> u64 {
    let mut program = Program::parse(input);
    program.execute(true);
    return program.sum_of_memory();
}

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

    static EXAMPLE2: &str = indoc! {"
        mask = 000000000000000000000000000000X1001X
        mem[42] = 100
        mask = 00000000000000000000000000000000X0XX
        mem[26] = 1
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

    #[test]
    fn test_part2_example2() {
        let result = part2(EXAMPLE2);
        assert_eq!(result, 208);
    }

    #[test]
    fn test_part2_solution() {
        let result = part2(&read_input_file());
        assert_eq!(result, 4330547254348);
    }
}