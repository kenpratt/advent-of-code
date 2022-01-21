use std::fmt;
use std::fs;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref INSTRUCTION_RE: Regex =
        Regex::new(r"\A([a-z]+) ([a-z])( ([a-z]|\-?[0-9]+))?\z").unwrap();
    static ref VALUE_RE: Regex = Regex::new(r"\A\-?[0-9]+\z").unwrap();
}

fn read_input_file() -> String {
    fs::read_to_string("input.txt").expect("Something went wrong reading the file")
}

pub fn parse(input: &str) -> Vec<Instruction> {
    input.lines().map(|line| Instruction::parse(line)).collect()
}

pub fn alu_instructions() -> Vec<Instruction> {
    parse(&read_input_file())
}

pub fn limit_clauses(instructions: &[Instruction], n: usize) -> &[Instruction] {
    let mut input_indices = instructions
        .iter()
        .enumerate()
        .filter(|(_idx, inst)| inst.operator == Operator::Input)
        .map(|(idx, _inst)| idx);
    let split_at = input_indices.nth(n).unwrap();
    &instructions[0..split_at]
}

#[derive(Debug)]
pub struct Instruction {
    pub operator: Operator,
    pub first_arg: Register,
    pub second_arg: Option<RegisterOrValue>,
}

impl Instruction {
    fn parse(input: &str) -> Self {
        let captures = INSTRUCTION_RE.captures(input).unwrap();
        let operator = Operator::parse(captures.get(1).unwrap().as_str());
        let first_arg = Register::parse(captures.get(2).unwrap().as_str());
        let second_arg = captures.get(4).map(|m| RegisterOrValue::parse(m.as_str()));
        Self {
            operator,
            first_arg,
            second_arg,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Operator {
    Input,
    Add,
    Multiply,
    Divide,
    Modulo,
    Equal,
}

impl Operator {
    fn parse(input: &str) -> Self {
        match input {
            "inp" => Self::Input,
            "add" => Self::Add,
            "mul" => Self::Multiply,
            "div" => Self::Divide,
            "mod" => Self::Modulo,
            "eql" => Self::Equal,
            _ => panic!("Unrecognized operator: {}", input),
        }
    }

    pub fn apply(&self, v1: Value, v2: Value) -> Value {
        match self {
            Self::Input => panic!("Can't apply an input operator"),
            Self::Add => v1 + v2,
            Self::Multiply => v1 * v2,
            Self::Divide => v1 / v2,
            Self::Modulo => v1 % v2,
            Self::Equal => (v1 == v2) as isize,
        }
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Input => write!(f, "INPUT"),
            Self::Add => write!(f, "+"),
            Self::Multiply => write!(f, "*"),
            Self::Divide => write!(f, "/"),
            Self::Modulo => write!(f, "%"),
            Self::Equal => write!(f, "="),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialOrd, PartialEq)]
pub enum Register {
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

pub static REGISTERS: [Register; 4] = [Register::W, Register::X, Register::Y, Register::Z];

pub type Value = isize;

#[derive(Debug)]
pub enum RegisterOrValue {
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
pub enum Goal {
    Minimum,
    Maximum,
}
