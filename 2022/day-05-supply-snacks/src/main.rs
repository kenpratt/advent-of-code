use std::collections::HashMap;
use std::fs;

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
struct Ship {
    crates: Crates,
    instructions: Vec<Instruction>,
}

impl Ship {
    fn parse(input: &str) -> Self {
        let parts: Vec<&str> = input.split("\n\n").collect();
        assert_eq!(parts.len(), 2);
        let crates = Crates::parse(parts[0]);
        let instructions = parts[1]
            .lines()
            .map(|line| Instruction::parse(line))
            .collect();
        Self {
            crates,
            instructions,
        }
    }

    fn execute_instructions(&mut self) {
        for inst in self.instructions.iter() {
            println!("applying {:?}", inst);
            for _ in 0..inst.quantity {
                self.crates.move_crate(&inst.from, &inst.to);
            }
            println!("{:?}", self.crates);
        }
    }

    fn top_of_each_stack(&self) -> String {
        self.crates.top_of_each_stack()
    }
}

#[derive(Debug)]
struct Crates {
    ids: Vec<char>,
    stacks: HashMap<char, Vec<char>>,
}

impl Crates {
    fn parse(input: &str) -> Self {
        let mut lines = input.lines().rev();
        let ids = Self::parse_input_row(lines.next().unwrap());
        println!("ids: {:?}", ids);

        let crate_rows: Vec<Vec<char>> = lines
            .into_iter()
            .map(|line| Self::parse_input_row(line))
            .collect();
        println!("crate_rows: {:?}", crate_rows);

        let stacks = ids
            .iter()
            .enumerate()
            .map(|(index, id)| {
                (
                    *id,
                    crate_rows
                        .iter()
                        .map(|v| v[index])
                        .filter(|c| *c != ' ')
                        .collect(),
                )
            })
            .collect();
        println!("stacks: {:?}", stacks);

        Self { ids, stacks }
    }

    // we only care about the chars at index 1, 5, 9, etc
    fn parse_input_row(input: &str) -> Vec<char> {
        let chars: Vec<char> = input.chars().collect();
        (1..chars.len()).step_by(4).map(|i| chars[i]).collect()
    }

    fn move_crate(&mut self, from_stack_id: &char, to_stack_id: &char) {
        let value = self.pop(from_stack_id);
        self.push(to_stack_id, value);
    }

    fn push(&mut self, stack_id: &char, value: char) {
        self.stacks.get_mut(stack_id).unwrap().push(value);
    }

    fn pop(&mut self, stack_id: &char) -> char {
        self.stacks.get_mut(stack_id).unwrap().pop().unwrap()
    }

    fn top_of_each_stack(&self) -> String {
        self.ids
            .iter()
            .map(|id| self.stacks.get(id).unwrap().last().unwrap())
            .collect()
    }
}

#[derive(Debug)]
struct Instruction {
    quantity: usize,
    from: char,
    to: char,
}

impl Instruction {
    fn parse(input: &str) -> Self {
        lazy_static! {
            static ref INSTRUCTION_RE: Regex =
                Regex::new(r"\Amove (\d+) from (\d+) to (\d+)\z").unwrap();
        }
        let caps = INSTRUCTION_RE.captures(input).unwrap();
        let quantity = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let from = caps.get(2).unwrap().as_str().chars().next().unwrap();
        let to = caps.get(3).unwrap().as_str().chars().next().unwrap();
        let inst = Self { quantity, from, to };
        println!("{:?} -> {:?}", input, inst);
        inst
    }
}

fn part1(input: &str) -> String {
    let mut ship = Ship::parse(input);
    println!("{:?}", ship);
    ship.execute_instructions();
    ship.top_of_each_stack()
}

// fn part2(input: &str) -> usize {
//     let crates = Crates::parse(input);
//     println!("{:?}", crates);
//     crates.execute()
// }

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    static EXAMPLE1: &str = indoc! {"
            [D]    
        [N] [C]    
        [Z] [M] [P]
         1   2   3 
        
        move 1 from 2 to 1
        move 3 from 1 to 3
        move 2 from 2 to 1
        move 1 from 1 to 2
    "};

    #[test]
    fn test_part1_example1() {
        let result = part1(EXAMPLE1);
        assert_eq!(result, "CMZ");
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file());
        assert_eq!(result, "WSFTMRHPP");
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
