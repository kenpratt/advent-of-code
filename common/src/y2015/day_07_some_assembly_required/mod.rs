use crate::interface::AoC;

use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;

pub struct Day;
impl AoC<Circuit, u16, u16> for Day {
    const FILE: &'static str = file!();

    fn parse(input: String) -> Circuit {
        Circuit::parse(&input)
    }

    fn part1(circuit: &Circuit) -> u16 {
        circuit.value("a")
    }

    fn part2(_instructions: &Circuit) -> u16 {
        0
    }
}

#[derive(Debug)]
struct Instruction {
    operation: Operation,
    output: String,
}

impl Instruction {
    fn parse_list(input: &str) -> Vec<Self> {
        input.lines().map(|line| Self::parse(line)).collect()
    }

    fn parse(input: &str) -> Self {
        lazy_static! {
            static ref INSTRUCTION_RE: Regex = Regex::new(r"\A(.+) \-> ([a-z]+)\z").unwrap();
        }

        let caps = INSTRUCTION_RE.captures(input).unwrap();
        let operation = Operation::parse(caps.get(1).unwrap().as_str());
        let output = caps.get(2).unwrap().as_str().to_string();
        Self { operation, output }
    }
}

#[derive(Debug)]
enum Operation {
    Value(u16),
    Passthrough(String),
    And(String, String),
    AndValue(String, u16),
    Or(String, String),
    ShiftLeft(String, u8),
    ShiftRight(String, u8),
    Not(String),
}

impl Operation {
    fn parse(input: &str) -> Self {
        lazy_static! {
            static ref VALUE_RE: Regex = Regex::new(r"\A(\d+)\z").unwrap();
            static ref PASSTHROUGH_RE: Regex = Regex::new(r"\A([a-z]+)\z").unwrap();
            static ref AND_RE: Regex = Regex::new(r"\A([a-z]+) AND ([a-z]+)\z").unwrap();
            static ref AND_VAL_RE: Regex = Regex::new(r"\A([\d]+) AND ([a-z]+)\z").unwrap();
            static ref OR_RE: Regex = Regex::new(r"\A([a-z]+) OR ([a-z]+)\z").unwrap();
            static ref LSHIFT_RE: Regex = Regex::new(r"\A([a-z]+) LSHIFT (\d+)\z").unwrap();
            static ref RSHIFT_RE: Regex = Regex::new(r"\A([a-z]+) RSHIFT (\d+)\z").unwrap();
            static ref NOT_RE: Regex = Regex::new(r"\ANOT ([a-z]+)\z").unwrap();
        }

        if let Some(caps) = VALUE_RE.captures(input) {
            let val = caps.get(1).unwrap().as_str().parse::<u16>().unwrap();
            return Operation::Value(val);
        }

        if let Some(caps) = PASSTHROUGH_RE.captures(input) {
            let label = caps.get(1).unwrap().as_str().to_string();
            return Operation::Passthrough(label);
        }

        if let Some(caps) = AND_RE.captures(input) {
            let l = caps.get(1).unwrap().as_str().to_string();
            let r = caps.get(2).unwrap().as_str().to_string();
            return Operation::And(l, r);
        }

        if let Some(caps) = AND_VAL_RE.captures(input) {
            let val = caps.get(1).unwrap().as_str().parse::<u16>().unwrap();
            let label = caps.get(2).unwrap().as_str().to_string();
            return Operation::AndValue(label, val);
        }

        if let Some(caps) = OR_RE.captures(input) {
            let l = caps.get(1).unwrap().as_str().to_string();
            let r = caps.get(2).unwrap().as_str().to_string();
            return Operation::Or(l, r);
        }

        if let Some(caps) = LSHIFT_RE.captures(input) {
            let label = caps.get(1).unwrap().as_str().to_string();
            let amount = caps.get(2).unwrap().as_str().parse::<u8>().unwrap();
            return Operation::ShiftLeft(label, amount);
        }

        if let Some(caps) = RSHIFT_RE.captures(input) {
            let label = caps.get(1).unwrap().as_str().to_string();
            let amount = caps.get(2).unwrap().as_str().parse::<u8>().unwrap();
            return Operation::ShiftRight(label, amount);
        }

        if let Some(caps) = NOT_RE.captures(input) {
            let label = caps.get(1).unwrap().as_str().to_string();
            return Operation::Not(label);
        }

        panic!("No operation regex matched input: {:?}", input);
    }

    fn inputs(&self) -> Vec<&str> {
        use Operation::*;

        match self {
            Value(_) => vec![],
            Passthrough(l) => vec![l],
            And(l, r) => vec![l, r],
            AndValue(l, _) => vec![l],
            Or(l, r) => vec![l, r],
            ShiftLeft(l, _) => vec![l],
            ShiftRight(l, _) => vec![l],
            Not(l) => vec![l],
        }
    }

    fn apply(&self, values: &HashMap<&str, u16>) -> u16 {
        use Operation::*;

        match self {
            Value(v) => *v,
            Passthrough(l) => *values.get(l.as_str()).unwrap(),
            And(l, r) => {
                let vl = *values.get(l.as_str()).unwrap();
                let vr = *values.get(r.as_str()).unwrap();
                vl & vr
            }
            AndValue(l, v) => {
                let vl = *values.get(l.as_str()).unwrap();
                vl & v
            }
            Or(l, r) => {
                let vl = *values.get(l.as_str()).unwrap();
                let vr = *values.get(r.as_str()).unwrap();
                vl | vr
            }
            ShiftLeft(l, n) => values.get(l.as_str()).unwrap() << n,
            ShiftRight(l, n) => values.get(l.as_str()).unwrap() >> n,
            Not(l) => !values.get(l.as_str()).unwrap(),
        }
    }
}

#[derive(Debug)]
pub struct Circuit {
    instructions: HashMap<String, Operation>, // output: operation
}

impl Circuit {
    fn parse(input: &str) -> Self {
        let instructions = Instruction::parse_list(input);
        Self {
            instructions: instructions
                .into_iter()
                .map(|inst| (inst.output, inst.operation))
                .collect(),
        }
    }

    fn value(&self, wire: &str) -> u16 {
        let mut to_resolve: Vec<&str> = vec![wire]; // TODO regular Vec, and change from front to back?
        let mut values: HashMap<&str, u16> = HashMap::new();

        while let Some(resolve) = to_resolve.pop() {
            let operation = self.instructions.get(resolve).unwrap();
            let inputs = operation.inputs();

            if inputs.iter().all(|input| values.contains_key(input)) {
                // all inputs have values; ready to resolve
                if !values.contains_key(resolve) {
                    let value = operation.apply(&values);
                    values.insert(resolve, value);
                }
            } else {
                // not ready yet
                to_resolve.push(resolve);
                for input in inputs.iter().filter(|input| !values.contains_key(*input)) {
                    to_resolve.push(input);
                }
            }
        }

        *values.get(wire).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PART1_EXAMPLE_VALUES: [(&str, u16); 8] = [
        ("d", 72),
        ("e", 507),
        ("f", 492),
        ("g", 114),
        ("h", 65412),
        ("i", 65079),
        ("x", 123),
        ("y", 456),
    ];

    #[test]
    fn test_part1_example() {
        let circuit = Day::parse_example_file();
        for (wire, value) in &PART1_EXAMPLE_VALUES {
            assert_eq!(circuit.value(wire), *value);
        }
    }

    #[test]
    fn test_part1_solution() {
        let result = Day::part1(&Day::parse_input_file());
        assert_eq!(result, 46065);
    }

    #[test]
    fn test_part2_example() {
        let result = Day::part2(&Day::parse_example_file());
        assert_eq!(result, 0);
    }

    #[test]
    fn test_part2_solution() {
        let result = Day::part2(&Day::parse_input_file());
        assert_eq!(result, 0);
    }
}
