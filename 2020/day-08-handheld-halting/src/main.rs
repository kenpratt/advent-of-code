use std::collections::HashSet;
use std::fs;

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
    accumulator: isize,
    instruction_pointer: usize,
}

impl Program {
    fn parse(input: &str) -> Program {
        let instructions = input.lines().map(|line| Instruction::parse(line)).collect();
        return Program::new(instructions);
    }

    fn new(instructions: Vec<Instruction>) -> Program {
        return Program {
            instructions: instructions,
            accumulator: 0,
            instruction_pointer: 0,
        };
    }

    fn execute(&mut self) -> (isize, Termination) {
        let mut seen_instructions = HashSet::new();

        // println!("\n");
        // for (pos, i) in self.instructions.iter().enumerate() {
        //   println!("{}: {:?}", pos, i);
        // }
        // println!("");

        loop {
            //println!("ip={:?}, acc={:?}, seen={:?}", self.instruction_pointer, self.accumulator, seen_instructions);
            if self.instruction_pointer >= self.instructions.len() {
                //println!("EOF");
                return (self.accumulator, Termination::EndOfProgram);
            } else if seen_instructions.contains(&self.instruction_pointer) {
                //println!("Loop {}", self.instruction_pointer);
                return (self.accumulator, Termination::InfiniteLoop);
            } else {
                seen_instructions.insert(self.instruction_pointer);
                self.execute_current_instruction();
            }
        }
    }

    fn execute_current_instruction(&mut self) {
        match self.instructions[self.instruction_pointer] {
            Instruction::Nop(_) => {
                self.instruction_pointer += 1;
            },
            Instruction::Acc(argument) => {
                self.accumulator += argument;
                self.instruction_pointer += 1;
            },
            Instruction::Jmp(argument) => {
                let foo = self.instruction_pointer as isize;
                self.instruction_pointer = (foo + argument) as usize;
            },                        
        }
    }

    fn repair_and_execute(&self) -> isize {
        for i in 0..self.instructions.len() {
            // swap jmp & nop
            let replacement = match self.instructions[i] {
                Instruction::Nop(argument) => Some(Instruction::Jmp(argument)),
                Instruction::Acc(_) => None,
                Instruction::Jmp(argument) => Some(Instruction::Nop(argument)),
            };

            if replacement.is_some() {
                // run modified program and terminate if it exits normally
                let mut modified_instructions = self.instructions.clone();
                modified_instructions[i] = replacement.unwrap();
                let mut modified_program = Program::new(modified_instructions);
                let (acc, result) = modified_program.execute();
                if result == Termination::EndOfProgram {
                    return acc;
                }
            }           
        }

        panic!("Unable to repair program");
    }
}

#[derive(Debug, PartialEq)]
enum Termination {
    InfiniteLoop,
    EndOfProgram,
}

#[derive(Debug, Clone)]
enum Instruction {
    Nop(isize),
    Acc(isize),
    Jmp(isize),
}

impl Instruction {
    fn parse(input: &str) -> Instruction {
        let re = Regex::new(r"^([a-z]+) ([\+\-]\d+)$").unwrap();
        let captures = re.captures(input).unwrap();
        let operation = captures.get(1).unwrap().as_str();
        let argument = captures.get(2).unwrap().as_str().parse::<isize>().unwrap();

        return match operation {
            "nop" => Instruction::Nop(argument),
            "acc" => Instruction::Acc(argument),
            "jmp" => Instruction::Jmp(argument),
            _ => panic!("Unknown operation"),
        }
    }
}

fn part1(input: &str) -> isize {
    let mut program = Program::parse(input);
    let (accumulator, _) = program.execute();
    return accumulator;
}

fn part2(input: &str) -> isize {
    let program = Program::parse(input);
    let accumulator = program.repair_and_execute();
    return accumulator;
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    static EXAMPLE1: &str = indoc! {"
        nop +0
        acc +1
        jmp +4
        acc +3
        jmp -3
        acc -99
        acc +1
        jmp -4
        acc +6
    "};

    #[test]
    fn test_part1_example1() {
        let result = part1(EXAMPLE1);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file());
        assert_eq!(result, 1614);
    }

    #[test]
    fn test_part2_example1() {
        let result = part2(EXAMPLE1);
        assert_eq!(result, 8);
    }

    #[test]
    fn test_part2_solution() {
        let result = part2(&read_input_file());
        assert_eq!(result, 1260);
    }
}