use crate::common::*;

use std::collections::BTreeMap;
use std::collections::VecDeque;

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
        match instruction.operator {
            Operator::Input => {
                let register = instruction.first_arg;
                assert!(instruction.second_arg.is_none());
                let value = self.inputs.pop_front().unwrap();
                // println!("input: store {:?} in {:?}", value, register);
                self.write(&register, value);
            }
            _ => {
                // arithmetic operator
                let register = instruction.first_arg;
                let first_value = self.read(&register);
                let second_value = self.value_or_read(instruction.second_arg.as_ref().unwrap());
                let result_value = instruction.operator.apply(first_value, second_value);
                // println!(
                //     "arithmetic: {:?}({:?}, {:?}) = {:?}, store in {:?}",
                //     instruction.operator, first_value, second_value, result_value, register
                // );
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

type ModelNumber = Vec<u8>; // vec of 1-9's

pub fn validate_model_number(number: &ModelNumber, program: &[Instruction]) -> bool {
    // ALU input must be a 14-digit number with no zeroes
    assert_eq!(number.len(), 14);

    // for simplicity, allow zeros but don't run the program
    if number.iter().any(|i| *i == 0) {
        return false;
    }

    let alu_inputs: Vec<isize> = number.iter().map(|d| *d as isize).collect();
    let alu = ALU::run(program, &alu_inputs);
    let z_value = alu.read_z();
    match z_value {
        0 => true,
        _ => false,
    }
}

fn base9_usize_to_model_number(input: &usize) -> ModelNumber {
    input
        .to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap() as u8) // already 1-9
        .collect()
}

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
    fn test_run_part1_example1() {
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
    fn test_run_part1_example2() {
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
    fn test_run_part1_example3() {
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

    #[test]
    fn test_part1_try_invalid_model_number() {
        let instructions = alu_instructions();
        let model_number = base9_usize_to_model_number(&13579246899999);
        let valid = validate_model_number(&model_number, &instructions);
        assert_eq!(valid, false);
    }

    #[test]
    fn test_part1_try_valid_model_number() {
        let instructions = alu_instructions();
        let model_number = base9_usize_to_model_number(&99799212949967);
        let valid = validate_model_number(&model_number, &instructions);
        assert_eq!(valid, true);
    }

    #[test]
    fn test_run_clause1() {
        let instructions = alu_instructions();
        let instructions_slice = limit_clauses(&instructions, 1);

        for a in 1..=9 {
            let alu = ALU::run(instructions_slice, &vec![a]);
            assert_eq!(alu.read_w(), a);
            assert_eq!(alu.read_x(), 1);
            assert_eq!(alu.read_y(), a + 12);
            assert_eq!(alu.read_z(), a + 12);
        }
    }

    #[test]
    fn test_run_clause2() {
        let instructions = alu_instructions();
        let instructions_slice = limit_clauses(&instructions, 2);

        for a in 1..=9 {
            for b in 1..=9 {
                let alu = ALU::run(instructions_slice, &vec![a, b]);
                assert_eq!(alu.read_w(), b);
                assert_eq!(alu.read_x(), 1);
                assert_eq!(alu.read_y(), b + 6);
                assert_eq!(alu.read_z(), a * 26 + b + 318);
            }
        }
    }

    #[test]
    fn test_run_clause3() {
        let instructions = alu_instructions();
        let instructions_slice = limit_clauses(&instructions, 3);

        for a in 1..=9 {
            for b in 1..=9 {
                for c in 1..=9 {
                    let alu = ALU::run(instructions_slice, &vec![a, b, c]);
                    assert_eq!(alu.read_w(), c);
                    assert_eq!(alu.read_x(), 1);
                    assert_eq!(alu.read_y(), c + 4);
                    assert_eq!(alu.read_z(), a * 676 + b * 26 + c + 8272);
                }
            }
        }
    }
}
