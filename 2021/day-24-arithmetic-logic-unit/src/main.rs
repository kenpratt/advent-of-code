use std::collections::BTreeMap;
use std::collections::VecDeque;
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
    Input,
    Add,
    Multiply,
    Divide,
    Modulo,
    Equal,
}

impl Operation {
    fn parse(input: &str) -> Self {
        match input {
            "inp" => Self::Input,
            "add" => Self::Add,
            "mul" => Self::Multiply,
            "div" => Self::Divide,
            "mod" => Self::Modulo,
            "eql" => Self::Equal,
            _ => panic!("Unrecognized operation: {}", input),
        }
    }

    fn apply(&self, v1: Value, v2: Value) -> Value {
        match self {
            Self::Input => panic!("Can't apply an input operation"),
            Self::Add => v1 + v2,
            Self::Multiply => v1 * v2,
            Self::Divide => v1 / v2,
            Self::Modulo => v1 % v2,
            Self::Equal => (v1 == v2) as isize,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialOrd, PartialEq)]
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

static REGISTERS: [Register; 4] = [Register::W, Register::X, Register::Y, Register::Z];

type Value = isize;

#[derive(Debug)]
enum RegisterOrValue {
    Register(Register),
    Value(Value),
}

impl RegisterOrValue {
    fn parse(input: &str) -> Self {
        if VALUE_RE.is_match(input) {
            let v = input.parse().unwrap();
            Self::Value(v)
        } else {
            let r = Register::parse(input);
            Self::Register(r)
        }
    }
}

#[derive(Debug)]
struct ALU {
    inputs: VecDeque<Value>,
    registers: BTreeMap<Register, Value>,
}

impl ALU {
    fn new(input_values: &[Value]) -> Self {
        let inputs = input_values.iter().cloned().collect();
        let registers = REGISTERS.iter().map(|r| (*r, 0)).collect();
        ALU { inputs, registers }
    }

    fn run(instructions: &[Instruction], inputs: &[Value]) -> Self {
        let mut alu = Self::new(inputs);
        for instruction in instructions {
            alu.execute(instruction);
        }
        alu
    }

    fn execute(&mut self, instruction: &Instruction) {
        match instruction.operation {
            Operation::Input => {
                let register = instruction.first_arg;
                assert!(instruction.second_arg.is_none());
                let value = self.inputs.pop_front().unwrap();
                println!("input: store {:?} in {:?}", value, register);
                self.write(&register, value);
            }
            _ => {
                // arithmetic operation
                let register = instruction.first_arg;
                let first_value = self.read(&register);
                let second_value = self.value_or_read(instruction.second_arg.as_ref().unwrap());
                let result_value = instruction.operation.apply(first_value, second_value);
                println!(
                    "arithmetic: {:?}({:?}, {:?}) = {:?}, store in {:?}",
                    instruction.operation, first_value, second_value, result_value, register
                );
                self.write(&register, result_value);
            }
        }
    }

    fn read(&self, register: &Register) -> Value {
        *self.registers.get(register).unwrap()
    }

    fn write(&mut self, register: &Register, value: Value) {
        let v = self.registers.get_mut(register).unwrap();
        *v = value;
    }

    fn value_or_read(&self, input: &RegisterOrValue) -> Value {
        match input {
            RegisterOrValue::Register(r) => self.read(r),
            RegisterOrValue::Value(v) => *v,
        }
    }

    fn read_w(&self) -> Value {
        self.read(&Register::W)
    }

    fn read_x(&self) -> Value {
        self.read(&Register::X)
    }

    fn read_y(&self) -> Value {
        self.read(&Register::Y)
    }

    fn read_z(&self) -> Value {
        self.read(&Register::Z)
    }
}

fn part1(input: &str) -> usize {
    let instructions = parse(input);
    println!("{:?}", instructions);
    // let alu = ALU::run(&instructions);
    // println!("{:?}", alu);
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
        let instructions = parse(EXAMPLE1);

        let mut alu = ALU::run(&instructions, &vec![5]);
        assert_eq!(alu.read_w(), 0);
        assert_eq!(alu.read_x(), -5);
        assert_eq!(alu.read_y(), 0);
        assert_eq!(alu.read_z(), 0);

        alu = ALU::run(&instructions, &vec![-12]);
        assert_eq!(alu.read_w(), 0);
        assert_eq!(alu.read_x(), 12);
        assert_eq!(alu.read_y(), 0);
        assert_eq!(alu.read_z(), 0);
    }

    #[test]
    fn test_part1_example2() {
        let instructions = parse(EXAMPLE2);

        let mut alu = ALU::run(&instructions, &vec![3, 9]);
        assert_eq!(alu.read_w(), 0);
        assert_eq!(alu.read_x(), 9);
        assert_eq!(alu.read_y(), 0);
        assert_eq!(alu.read_z(), 1);

        alu = ALU::run(&instructions, &vec![3, 5]);
        assert_eq!(alu.read_w(), 0);
        assert_eq!(alu.read_x(), 5);
        assert_eq!(alu.read_y(), 0);
        assert_eq!(alu.read_z(), 0);
    }

    #[test]
    fn test_part1_example3() {
        let instructions = parse(EXAMPLE3);

        let mut alu = ALU::run(&instructions, &vec![5]);
        assert_eq!(alu.read_w(), 0);
        assert_eq!(alu.read_x(), 1);
        assert_eq!(alu.read_y(), 0);
        assert_eq!(alu.read_z(), 1);

        alu = ALU::run(&instructions, &vec![15]);
        assert_eq!(alu.read_w(), 1);
        assert_eq!(alu.read_x(), 1);
        assert_eq!(alu.read_y(), 1);
        assert_eq!(alu.read_z(), 1);
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
