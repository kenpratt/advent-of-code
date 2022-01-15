use std::fs;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref INSTRUCTION_RE: Regex =
        Regex::new(r"\A([a-z]+) ([a-z])( ([a-z]|\-?[0-9]+))?\z").unwrap();
    static ref VALUE_RE: Regex = Regex::new(r"\A\-?[0-9]+\z").unwrap();
}

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    // println!("part 2 result: {:?}", part2(&read_input_file()));
}

fn read_input_file() -> String {
    fs::read_to_string("input.txt").expect("Something went wrong reading the file")
}

fn parse(input: &str) -> Vec<Instruction> {
    input.lines().map(|line| Instruction::parse(line)).collect()
}

#[derive(Debug)]
struct Instruction {
    operation: Operation,
    first_arg: Register,
    second_arg: Option<RegisterOrValue>,
}

impl Instruction {
    fn parse(input: &str) -> Self {
        let captures = INSTRUCTION_RE.captures(input).unwrap();
        let operation = Operation::parse(captures.get(1).unwrap().as_str());
        let first_arg = Register::parse(captures.get(2).unwrap().as_str());
        let second_arg = captures.get(4).map(|m| RegisterOrValue::parse(m.as_str()));
        Self {
            operation,
            first_arg,
            second_arg,
        }
    }
}

#[derive(Debug)]
enum Operation {
    Read,
    Add,
    Multiply,
    Divide,
    Modulo,
    Equal,
}

impl Operation {
    fn parse(input: &str) -> Self {
        match input {
            "inp" => Self::Read,
            "add" => Self::Add,
            "mul" => Self::Multiply,
            "div" => Self::Divide,
            "mod" => Self::Modulo,
            "eql" => Self::Equal,
            _ => panic!("Unrecognized operation: {}", input),
        }
    }
}

#[derive(Debug)]
enum Register {
    W,
    X,
    Y,
    Z,
}

impl Register {
    fn parse(input: &str) -> Self {
        match input {
            "w" => Self::W,
            "x" => Self::X,
            "y" => Self::Y,
            "z" => Self::Z,
            _ => panic!("Unrecognized register: {}", input),
        }
    }
}

#[derive(Debug)]
enum RegisterOrValue {
    Register(Register),
    Value(isize),
}

impl RegisterOrValue {
    fn parse(input: &str) -> Self {
        if VALUE_RE.is_match(input) {
            let v: isize = input.parse().unwrap();
            Self::Value(v)
        } else {
            let r = Register::parse(input);
            Self::Register(r)
        }
    }
}

fn part1(input: &str) -> usize {
    let instructions = parse(input);
    println!("{:?}", instructions);
    0
}

// fn part2(input: &str) -> usize {
//     let data = Data::parse(input);
//     println!("{:?}", data);
//     data.execute()
// }

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    static EXAMPLE1: &str = indoc! {"
        inp x
        mul x -1
    "};

    static EXAMPLE2: &str = indoc! {"
        inp z
        inp x
        mul z 3
        eql z x
    "};

    static EXAMPLE3: &str = indoc! {"
        inp w
        add z w
        mod z 2
        div w 2
        add y w
        mod y 2
        div w 2
        add x w
        mod x 2
        div w 2
        mod w 2
    "};

    #[test]
    fn test_part1_example1() {
        let result = part1(EXAMPLE1);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_part1_example2() {
        let result = part1(EXAMPLE2);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_part1_example3() {
        let result = part1(EXAMPLE3);
        assert_eq!(result, 0);
    }

    // #[test]
    // fn test_part1_solution() {
    //     let result = part1(&read_input_file());
    //     assert_eq!(result, 0);
    // }

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
