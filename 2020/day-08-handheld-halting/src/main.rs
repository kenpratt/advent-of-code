use std::collections::HashSet;
use std::fs;

use indoc::indoc;
// use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    println!("part 1 result: {:?}", part1(read_input_file()));
    //println!("part 2 result: {:?}", part2(read_input_file()));
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
        return Program {
            instructions: instructions,
            accumulator: 0,
            instruction_pointer: 0,
        }
    }

    fn execute(&mut self) -> isize {
        let mut seen_instructions = HashSet::new();
        let mut done = false;

        while !done {
            //println!("ip={:?}, acc={:?}, done={:?}, seen={:?}", self.instruction_pointer, self.accumulator, done, seen_instructions);
            if seen_instructions.contains(&self.instruction_pointer) {
                done = true;
            } else {
                seen_instructions.insert(self.instruction_pointer);
                self.execute_current_instruction();
            }
        }

        return self.accumulator;
    }

    fn execute_current_instruction(&mut self) {
        match self.instructions[self.instruction_pointer] {
            Instruction::Nop => {
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
}

#[derive(Debug)]
enum Instruction {
    Nop,
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
            "nop" => Instruction::Nop,
            "acc" => Instruction::Acc(argument),
            "jmp" => Instruction::Jmp(argument),
            _ => panic!("Unknown operation"),
        }
    }
}

fn part1(input: String) -> isize {
    let mut program = Program::parse(&input);
    return program.execute();
}

// fn part2(input: String) -> usize {
//     let data = Data::parse(input);
//     return data.execute();
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example1() {
        let input = indoc! {"
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
        let result = part1(input.to_string());
        assert_eq!(result, 5);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(
            read_input_file()
        );
        assert_eq!(result, 1614);
    }

    // #[test]
    // fn test_part2_example1() {
    //     let result = part2(
    //         "".to_string()
    //     );
    //     assert_eq!(result, 0);
    // }

    // #[test]
    // fn test_part2_solution() {
    //     let result = part2(
    //         read_input_file()
    //     );
    //     assert_eq!(result, 0);
    // }
}