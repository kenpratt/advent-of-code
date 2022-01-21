use crate::common::*;

use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::fmt;

use itertools::Itertools;
use lazy_static::lazy_static;

lazy_static! {
    static ref MODEL_NUMBER_VARIABLE_INPUTS: Vec<InputSpec> =
        ('a'..='n').map(|c| InputSpec::new(c, 1, 9)).collect();
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct InputSpec {
    name: char,
    min: isize,
    max: isize,
}

impl InputSpec {
    fn new(name: char, min: isize, max: isize) -> InputSpec {
        InputSpec { name, min, max }
    }
}

impl fmt::Display for InputSpec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Debug)]
struct Range {
    min: isize,
    max: isize,
}

impl Range {
    fn new(min: isize, max: isize) -> Self {
        Self { min, max }
    }

    fn apply(&self, operator: &Operator, other: &Self) -> Self {
        match operator {
            Operator::Input => panic!("Can't apply an input operator"),
            Operator::Add => self.add(other),
            Operator::Multiply => self.multiply(other),
            Operator::Divide => self.divide(other),
            Operator::Modulo => self.modulo(other),
            Operator::Equal => self.equal(other),
        }
    }

    fn add(&self, other: &Self) -> Self {
        let min = self.min + other.min;
        let max = self.max + other.max;
        Self::new(min, max)
    }

    fn multiply(&self, other: &Self) -> Self {
        // TODO negative numbers might mess this up?
        assert!(self.min >= 0 && self.max >= 0 && other.max >= 0 && other.max >= 0);
        let min = self.min * other.min;
        let max = self.max * other.max;
        Self::new(min, max)
    }

    fn divide(&self, other: &Self) -> Self {
        // TODO again, negative numbers?
        assert!(self.min >= 0 && self.max >= 0 && other.max >= 0 && other.max >= 0);
        let min = self.min / other.max;
        let max = self.max / other.min;
        Self::new(min, max)
    }

    fn modulo(&self, other: &Self) -> Self {
        // no matter what the first arg is, the range will be 0..other.max - 1
        Self::new(0, other.max - 1)
    }

    fn equal(&self, _other: &Self) -> Self {
        // always 0 or 1
        Self::new(0, 1)
    }

    fn contains(&self, value: isize) -> bool {
        value >= self.min && value <= self.max
    }

    fn non_overlapping(&self, other: &Self) -> bool {
        // self 0..5, other 7..10
        // self 7..10, other 0..5
        self.max < other.min || other.max < self.min
    }
}

impl fmt::Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}..{}", self.min, self.max)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Expression {
    Value(isize),
    Input(InputSpec),
    Operation(Operator, Vec<Expression>),
}

impl Expression {
    fn is_value(&self) -> bool {
        match self {
            Self::Value(_) => true,
            _ => false,
        }
    }

    fn value(&self) -> Option<isize> {
        match self {
            Self::Value(v) => Some(*v),
            _ => None,
        }
    }

    fn cleanly_divisible_by(&self, x: &isize) -> bool {
        let res = Self::apply_operation(&Operator::Modulo, self, &Self::Value(*x));
        res == Self::Value(0)
    }

    fn is_operation(&self) -> bool {
        match self {
            Self::Operation(_, _) => true,
            _ => false,
        }
    }

    fn is_equal_operation(&self) -> bool {
        match self {
            Self::Operation(Operator::Equal, _) => true,
            _ => false,
        }
    }

    fn range(&self) -> Range {
        match self {
            Self::Value(v) => Range::new(*v, *v),
            Self::Input(i) => Range::new(i.min, i.max),
            Self::Operation(operator, operands) => operands
                .iter()
                .map(|o| o.range())
                .reduce(|acc, r| acc.apply(operator, &r))
                .unwrap(),
        }
    }

    fn apply_operation(operator: &Operator, operand1: &Self, operand2: &Self) -> Self {
        if *operator == Operator::Input {
            panic!("Can't apply an input operator");
        }
        let applied = Self::make_operation(*operator, vec![operand1.clone(), operand2.clone()]);
        applied.simplify()
    }

    fn make_operation(operator: Operator, mut operands: Vec<Self>) -> Self {
        match operator {
            Operator::Input => panic!("TODO?"),
            Operator::Add | Operator::Multiply => {
                assert!(operands.len() >= 2);
                operands.sort();
                Self::Operation(operator, operands)
            }
            _ => {
                assert_eq!(operands.len(), 2);
                Self::Operation(operator, operands)
            }
        }
    }

    fn simplify(self) -> Self {
        match self {
            Self::Operation(Operator::Input, _) => self,
            Self::Operation(Operator::Add, operands) => Self::simplify_add(operands),
            Self::Operation(Operator::Multiply, operands) => Self::simplify_multiply(operands),
            Self::Operation(Operator::Divide, operands) => Self::simplify_divide(operands),
            Self::Operation(Operator::Modulo, operands) => Self::simplify_modulo(operands, true),
            Self::Operation(Operator::Equal, operands) => Self::simplify_equal(operands),
            _ => self,
        }
    }

    fn simplify_add(operands: Vec<Expression>) -> Self {
        // first, try to combine nested additions
        let mut new_operands = Self::flatten(operands, &Operator::Add);

        // drop zeroes, if present
        new_operands = new_operands
            .into_iter()
            .filter(|o| *o != Expression::Value(0))
            .collect();

        match new_operands.len() {
            // nothing left after dropping zeroes, return 0
            0 => Self::Value(0),

            // only 1 expression left, return it alone
            1 => new_operands.pop().unwrap(),

            _ => Self::Operation(Operator::Add, new_operands),
        }
    }

    fn simplify_multiply(operands: Vec<Expression>) -> Self {
        // first, try to combine nested multiplications
        let mut new_operands = Self::flatten(operands, &Operator::Multiply);

        // now, drop trailing one, if present
        if new_operands.last() == Some(&Expression::Value(1)) {
            new_operands.pop();
        }

        // if the trailing value is a zero, the whole expression is zero!
        if new_operands.last() == Some(&Expression::Value(0)) {
            return Self::Value(0);
        }

        // special case: if the last operand is a value and the one to the left of it is an
        // expression, expand it into the expression.
        if new_operands.len() >= 2
            && new_operands.last().unwrap().is_value()
            && new_operands[new_operands.len() - 2].is_operation()
        {
            let value = new_operands.pop().unwrap();
            match new_operands.pop().unwrap() {
                Self::Operation(Operator::Add, add_operands) => {
                    // to expand into add expression, multiply all the terms
                    let new_add_operands = add_operands
                        .iter()
                        .map(|o| Self::apply_operation(&Operator::Multiply, o, &value))
                        .collect();
                    return Self::Operation(Operator::Add, new_add_operands);
                }
                Self::Operation(Operator::Divide, divide_operands) => {
                    // to expand into divide expression, multiply by the dividend
                    let dividend = &divide_operands[0];
                    let divisor = &divide_operands[1];
                    let new_dividend = Self::apply_operation(&Operator::Multiply, dividend, &value);
                    return Self::apply_operation(&Operator::Multiply, &new_dividend, divisor);
                }
                other => {
                    // TODO figure out if we can expand into this operation type?
                    // put the values back...
                    new_operands.push(other);
                    new_operands.push(value);
                }
            };
        }

        match new_operands.len() {
            0 => panic!("Unreachable"),

            // only 1 expression left, return it alone
            1 => new_operands.pop().unwrap(),

            _ => Self::Operation(Operator::Multiply, new_operands),
        }
    }

    fn simplify_divide(operands: Vec<Expression>) -> Self {
        assert_eq!(operands.len(), 2);
        let dividend = &operands[0];
        let divisor = &operands[1];

        match (dividend, divisor) {
            // 0 / y = 0
            (Self::Value(0), _) => Self::Value(0),

            // x / 0 = panic
            (_, Self::Value(0)) => panic!("Divide by 0"),

            // x / 1 = x
            (_, Self::Value(1)) => dividend.clone(),

            // normal simple values case
            (Self::Value(x), Self::Value(y)) => Self::Value(x / y),

            // special case: if applying modulo to an expression that cannot have a value
            // greater than the divisor, the result will be 0
            (_, Self::Value(m)) if dividend.range().min >= 0 && dividend.range().max < *m => {
                Self::Value(0)
            }

            // special case, dividend is also a division, can multiply the divisors
            (Self::Operation(Operator::Divide, inner_operands), _) => {
                let inner_dividend = &inner_operands[0];
                let inner_divisor = &inner_operands[1];
                let new_divisor =
                    Self::apply_operation(&Operator::Multiply, inner_divisor, divisor);
                Self::apply_operation(&Operator::Divide, inner_dividend, &new_divisor)
            }

            // special case of multiply where the last element is cleanly divisible - can just divide the last element
            // TODO: technically could probably divide _any_ element that is cleanly divisible?
            (Self::Operation(Operator::Multiply, inner_operands), Self::Value(y))
                if inner_operands.last().unwrap().cleanly_divisible_by(y) =>
            {
                // can just divide the last thing by the divisor
                let mut new_inner_operands = inner_operands.clone();
                let last_elem = new_inner_operands.pop().unwrap();
                let new_last_elem = Self::apply_operation(&Operator::Divide, &last_elem, divisor);
                new_inner_operands.push(new_last_elem);
                Self::Operation(Operator::Multiply, new_inner_operands).simplify()
            }

            // special case of add where all operands are cleanly divisible by the divisor
            // (or all but one are cleanly divisible - still works since the dropped remainder will
            // be the same)
            // can divide everything inside.
            (Self::Operation(Operator::Add, inner_operands), Self::Value(y))
                if inner_operands
                    .iter()
                    .map(|o| o.cleanly_divisible_by(y))
                    .count()
                    >= inner_operands.len() - 1 =>
            {
                let new_inner_operands = inner_operands
                    .iter()
                    .map(|o| Self::apply_operation(&Operator::Divide, o, divisor))
                    .collect();
                Self::Operation(Operator::Add, new_inner_operands)
            }

            // no reduction
            _ => Self::Operation(Operator::Divide, operands),
        }
    }

    fn simplify_modulo(operands: Vec<Expression>, expand: bool) -> Self {
        assert_eq!(operands.len(), 2);
        let dividend = &operands[0];
        let divisor = &operands[1];

        match (dividend, divisor) {
            // 0 % y = 0
            (Self::Value(0), _) => Self::Value(0),

            // x % 0 = panic
            (_, Self::Value(0)) => panic!("Modulo by 0"),

            // x % 1 = 0
            (_, Self::Value(1)) => Self::Value(0),

            // normal simple values case
            (Self::Value(x), Self::Value(y)) => Self::Value(x % y),

            // special case: if applying modulo to an expression that cannot have a value
            // greater than the modulo, we can ignore the modulo operation.
            (_, Self::Value(m)) if dividend.range().min >= 0 && dividend.range().max < *m => {
                dividend.clone()
            }

            // special case: for add and multiply, can modulo everything inside as well
            (Self::Operation(inner_operator, inner_operands), _)
                if expand
                    && (*inner_operator == Operator::Add
                        || *inner_operator == Operator::Multiply) =>
            {
                let new_inner_operands = inner_operands
                    .iter()
                    .map(|o| Self::apply_operation(&Operator::Modulo, o, divisor))
                    .collect();
                let new_inner_operation = Self::Operation(*inner_operator, new_inner_operands);
                println!("MOD NEST BEFORE {}", new_inner_operation);
                let new_inner_operation2 = new_inner_operation.simplify();
                println!("MOD NEST AFTER  {}", new_inner_operation2);

                // but still need to modulo the result as well!
                Self::simplify_modulo(
                    vec![new_inner_operation2, divisor.clone()],
                    false, // don't expand again or we could cause an infinite expansion loop
                )
            }

            // no reduction
            _ => Self::Operation(Operator::Modulo, operands),
        }
    }

    fn simplify_equal(operands: Vec<Expression>) -> Self {
        assert_eq!(operands.len(), 2);
        let left = &operands[0];
        let right = &operands[1];

        if *left == *right {
            return Expression::Value(1); // true
        }

        let left_range = left.range();
        let right_range = right.range();
        if left_range.non_overlapping(&right_range) {
            // cannot be equal, as there is no overlap between the possible ranges
            Expression::Value(0) // false
        } else {
            // no reduction
            Self::Operation(Operator::Equal, operands)
        }
    }

    fn flatten(operands: Vec<Expression>, operator: &Operator) -> Vec<Expression> {
        let mut new_operands = vec![];
        for expr in operands {
            match expr {
                Self::Operation(expr_operator, mut expr_operands) if expr_operator == *operator => {
                    // same operator, flatten!
                    new_operands.append(&mut expr_operands);
                }
                _ => new_operands.push(expr),
            };
        }
        new_operands.sort();

        // if the result has more than one plain value, combine them
        if new_operands.iter().filter(|o| o.is_value()).count() > 1 {
            let values = new_operands
                .iter()
                .filter(|o| o.is_value())
                .map(|o| o.value().unwrap());
            let combined_value = match operator {
                Operator::Add => values.sum(),
                Operator::Multiply => values.fold(1, |acc, v| acc * v), // product
                _ => panic!("Unreachable"),
            };
            let mut without_values: Vec<Expression> =
                new_operands.into_iter().filter(|o| !o.is_value()).collect();
            without_values.push(Expression::Value(combined_value));
            new_operands = without_values;
        }

        new_operands
    }

    fn variable_names(&self) -> Vec<&char> {
        let mut names = vec![];
        match self {
            Self::Value(_) => {}
            Self::Input(i) => names.push(&i.name),
            Self::Operation(_, operands) => {
                for operand in operands {
                    let mut operand_variable_names = operand.variable_names();
                    names.append(&mut operand_variable_names);
                }
            }
        };
        names
    }

    fn substitite_variable_value(&self, variable_name: &char, value: &isize) -> Expression {
        match self {
            Self::Value(_) => self.clone(),
            Self::Input(i) => {
                if i.name == *variable_name {
                    Self::Value(*value)
                } else {
                    self.clone()
                }
            }
            Self::Operation(operator, operands) => {
                let new_operands = operands
                    .iter()
                    .map(|o| o.substitite_variable_value(variable_name, value))
                    .collect();
                Self::Operation(*operator, new_operands).simplify()
            }
        }
    }

    fn valid_for_variable_value(
        &self,
        variable_name: &char,
        value: &isize,
        outcome: &bool,
    ) -> bool {
        let substituted = self.substitite_variable_value(variable_name, value);
        match substituted {
            Self::Value(0) => !*outcome, // false
            _ => *outcome,               // truthy?
        }
    }
}

impl Ord for Expression {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_var_names = self.variable_names();
        let other_var_names = other.variable_names();
        match (self_var_names.len(), other_var_names.len()) {
            (0, 0) => Ordering::Equal,
            (_, 0) => Ordering::Less,
            (0, _) => Ordering::Greater,
            _ => self_var_names.cmp(&other_var_names),
        }
    }
}

impl PartialOrd for Expression {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Value(v) => write!(f, "{}", v),
            Self::Input(i) => write!(f, "{}", i),
            Self::Operation(operator, operands) => {
                if operands.len() < 2 {
                    panic!(
                        "Error in display code - found operation with too few operands: {:?}, {:?}",
                        operator, operands
                    );
                }
                write!(f, "({}", operands[0])?;
                for operand in &operands[1..] {
                    write!(f, " {} {}", operator, operand)?;
                }
                write!(f, ")")
            }
        }
    }
}

#[derive(Clone, Debug)]
struct Branch {
    inputs: VecDeque<InputSpec>,
    constraints: Constraints,
    w: Expression,
    x: Expression,
    y: Expression,
    z: Expression,
}

impl Branch {
    fn new(input_specs: &[InputSpec]) -> Self {
        let inputs = input_specs.iter().cloned().collect();
        let constraints = Constraints::new();
        let w = Expression::Value(0);
        let x = Expression::Value(0);
        let y = Expression::Value(0);
        let z = Expression::Value(0);
        Self {
            inputs,
            constraints,
            w,
            x,
            y,
            z,
        }
    }

    fn apply(&mut self, instruction: &Instruction) -> Option<Branch> {
        match instruction.operator {
            Operator::Input => {
                let register = instruction.first_arg;
                assert!(instruction.second_arg.is_none());
                let input = self.inputs.pop_front().unwrap();
                self.write(&register, Expression::Input(input));
            }
            _ => {
                // arithmetic operator
                let register = instruction.first_arg;

                let first_operand = self.read(&register);

                let tmp: Expression;
                let second_operand = match instruction.second_arg.as_ref().unwrap() {
                    RegisterOrValue::Register(r) => self.read(r),
                    RegisterOrValue::Value(v) => {
                        tmp = Expression::Value(*v);
                        &tmp
                    }
                };

                let result = Expression::apply_operation(
                    &instruction.operator,
                    first_operand,
                    second_operand,
                );

                // found an unresolved equals after reduction
                if result.is_equal_operation() {
                    let offshoot_branch = self.bifrucate(&register, &result);
                    return Some(offshoot_branch);
                } else {
                    self.write(&register, result);
                }
            }
        }

        None
    }

    fn read(&self, register: &Register) -> &Expression {
        match *register {
            Register::W => &self.w,
            Register::X => &self.x,
            Register::Y => &self.y,
            Register::Z => &self.z,
        }
    }

    fn write(&mut self, register: &Register, expr: Expression) {
        match *register {
            Register::W => self.w = expr,
            Register::X => self.x = expr,
            Register::Y => self.y = expr,
            Register::Z => self.z = expr,
        }
    }

    fn bifrucate(&mut self, register: &Register, constraint: &Expression) -> Branch {
        println!("bifrucate! {:?}, {}", register, constraint);

        let mut other_branch = self.clone();

        // this branch will continue with true value
        self.constraints.add(constraint, &true);
        self.write(&register, Expression::Value(1));

        // other branch will continue with false value
        other_branch.constraints.add(constraint, &false);
        other_branch.write(&register, Expression::Value(0));

        other_branch
    }

    fn solve_inputs_for_z_of_0(
        &self,
        input_specs: &[InputSpec],
        goal: &Goal,
    ) -> Option<Vec<isize>> {
        if !self.z.range().contains(0) {
            // cannot contain 0, don't need to solve it
            return None;
        }

        Some(self.constraints.solve_inputs(input_specs, goal))
    }
}

#[derive(Clone, Debug)]
struct Constraints {
    list: Vec<(Expression, bool)>,
}

impl Constraints {
    fn new() -> Self {
        Self { list: vec![] }
    }

    fn add(&mut self, constraint: &Expression, outcome: &bool) {
        self.list.push((constraint.clone(), *outcome));
    }

    fn solve_inputs(&self, input_specs: &[InputSpec], goal: &Goal) -> Vec<isize> {
        let mut values: HashMap<char, isize> = HashMap::new();
        let mut remaining_constraints = self.list.clone();

        for input in input_specs {
            let mut value_range: Box<dyn Iterator<Item = isize>> = match goal {
                Goal::Minimum => Box::new(input.min..=input.max),
                Goal::Maximum => Box::new((input.min..=input.max).rev()),
            };

            if !values.contains_key(&input.name) {
                // find expressions with this input
                let constraints_with_input: Vec<&mut (Expression, bool)> = remaining_constraints
                    .iter_mut()
                    .filter(|(e, _)| e.variable_names().into_iter().contains(&input.name))
                    .collect();

                // find the first input value that works for all the constraints
                let target_value = value_range.find(|val| {
                    constraints_with_input
                        .iter()
                        .all(|(c, o)| c.valid_for_variable_value(&input.name, val, o))
                });

                match target_value {
                    Some(val) => {
                        // store the result, and adjust the constraints for this variable value
                        values.insert(input.name, val);
                        for (c, _) in constraints_with_input {
                            *c = c.substitite_variable_value(&input.name, &val);
                        }
                    }
                    None => panic!("Could not find a target value for {}", input.name),
                };
            }
        }

        if input_specs.iter().any(|i| !values.contains_key(&i.name)) {
            panic!(
                "Missing solutions for some variables: {:?} {:?}",
                input_specs, values
            );
        }

        input_specs
            .iter()
            .map(|i| *values.get(&i.name).unwrap())
            .collect()
    }
}

impl fmt::Display for Constraints {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, (constraint, outcome)) in self.list.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{} is {}", constraint, outcome)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct Reducer<'a> {
    input_specs: &'a [InputSpec],
    branches: Vec<Branch>,
}

impl<'a> Reducer<'a> {
    fn new(input_specs: &'a [InputSpec]) -> Self {
        let base_branch = Branch::new(input_specs);
        let branches = vec![base_branch];
        Self {
            input_specs,
            branches,
        }
    }

    fn reduce(instructions: &[Instruction], input_specs: &'a [InputSpec]) -> Self {
        let mut reducer = Self::new(input_specs);
        for instruction in instructions {
            reducer.apply(instruction);
        }
        reducer
    }

    fn apply(&mut self, instruction: &Instruction) {
        println!("\napplying: {:?}", instruction);
        let mut new_branches = vec![];
        for branch in &mut self.branches {
            match branch.apply(instruction) {
                Some(new_branch) => new_branches.push(new_branch),
                None => {}
            };
        }
        self.branches.append(&mut new_branches);
        self.debug();
    }

    fn debug(&self) {
        for (i, b) in self.branches.iter().enumerate() {
            println!("branch {}:", i);
            println!("  constraints: {}", b.constraints);
            println!("  w: {}", b.w);
            println!("  x: {}", b.x);
            println!("  y: {}", b.y);
            println!("  z: {}", b.z);
        }
    }

    pub fn solve_inputs_for_z_of_0(&self, goal: &Goal) -> Vec<isize> {
        self.branches
            .iter()
            .filter_map(|b| b.solve_inputs_for_z_of_0(self.input_specs, goal))
            .max()
            .unwrap()
    }
}

pub fn reduce_model_number(instructions: &[Instruction]) -> Reducer {
    Reducer::reduce(&instructions, &MODEL_NUMBER_VARIABLE_INPUTS)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clause1_reduce() {
        let instructions = alu_instructions();
        let instructions_slice = limit_clauses(&instructions, 1);
        let reducer = reduce_model_number(instructions_slice);
        assert_eq!(reducer.branches.len(), 1);
        let b = &reducer.branches[0];
        assert_eq!(format!("{}", b.w), "a");
        assert_eq!(format!("{}", b.x), "1");
        assert_eq!(format!("{}", b.y), "(a + 12)");
        assert_eq!(format!("{}", b.z), "(a + 12)");
        assert_eq!(format!("{}", b.constraints), "");
    }

    #[test]
    fn test_clause2_reduce() {
        let instructions = alu_instructions();
        let instructions_slice = limit_clauses(&instructions, 2);
        let reducer = reduce_model_number(instructions_slice);
        assert_eq!(reducer.branches.len(), 1);
        let b = &reducer.branches[0];
        assert_eq!(format!("{}", b.w), "b");
        assert_eq!(format!("{}", b.x), "1");
        assert_eq!(format!("{}", b.y), "(b + 6)");
        assert_eq!(format!("{}", b.z), "((a * 26) + b + 318)");
        assert_eq!(format!("{}", b.constraints), "");
    }

    #[test]
    fn test_clause3_reduce() {
        let instructions = alu_instructions();
        let instructions_slice = limit_clauses(&instructions, 3);
        let reducer = reduce_model_number(instructions_slice);
        assert_eq!(reducer.branches.len(), 1);
        let b = &reducer.branches[0];
        assert_eq!(format!("{}", b.w), "c");
        assert_eq!(format!("{}", b.x), "1");
        assert_eq!(format!("{}", b.y), "(c + 4)");
        assert_eq!(format!("{}", b.z), "((a * 676) + (b * 26) + c + 8272)");
        assert_eq!(format!("{}", b.constraints), "");
    }

    #[test]
    fn test_clause4_reduce() {
        let instructions = alu_instructions();
        let instructions_slice = limit_clauses(&instructions, 4);
        let reducer = reduce_model_number(instructions_slice);
        assert_eq!(reducer.branches.len(), 1);
        let b = &reducer.branches[0];
        assert_eq!(format!("{}", b.w), "d");
        assert_eq!(format!("{}", b.x), "1");
        assert_eq!(format!("{}", b.y), "(d + 5)");
        assert_eq!(
            format!("{}", b.z),
            "((a * 17576) + (b * 676) + (c * 26) + d + 215077)"
        );
        assert_eq!(format!("{}", b.constraints), "");
    }

    #[test]
    fn test_clause5_reduce() {
        let instructions = alu_instructions();
        let instructions_slice = limit_clauses(&instructions, 5);
        let reducer = reduce_model_number(instructions_slice);
        assert_eq!(reducer.branches.len(), 1);
        let b = &reducer.branches[0];
        assert_eq!(format!("{}", b.w), "e");
        assert_eq!(format!("{}", b.x), "1");
        assert_eq!(format!("{}", b.y), "e");
        assert_eq!(
            format!("{}", b.z),
            "((a * 456976) + (b * 17576) + (c * 676) + (d * 26) + e + 5592002)"
        );
        assert_eq!(format!("{}", b.constraints), "");
    }

    #[test]
    fn test_clause6_reduce() {
        let instructions = alu_instructions();
        let instructions_slice = limit_clauses(&instructions, 6);
        let reducer = reduce_model_number(instructions_slice);
        assert_eq!(reducer.branches.len(), 2);
        let b1 = &reducer.branches[0];
        assert_eq!(format!("{}", b1.w), "f");
        assert_eq!(format!("{}", b1.x), "0");
        assert_eq!(format!("{}", b1.y), "0");
        assert_eq!(
            format!("{}", b1.z),
            "((a * 17576) + (b * 676) + (c * 26) + d + 215077)"
        );
        assert_eq!(format!("{}", b1.constraints), "((e + -7) = f) is true");

        let b2 = &reducer.branches[1];
        assert_eq!(format!("{}", b2.w), "f");
        assert_eq!(format!("{}", b2.x), "1");
        assert_eq!(format!("{}", b2.y), "(f + 4)");
        assert_eq!(
            format!("{}", b2.z),
            "((a * 456976) + (b * 17576) + (c * 676) + (d * 26) + f + 5592006)"
        );
        assert_eq!(format!("{}", b2.constraints), "((e + -7) = f) is false");
    }

    #[test]
    fn test_part1_reduce() {
        let instructions = alu_instructions();
        let reducer = reduce_model_number(&instructions);
        assert_eq!(reducer.branches.len(), 79);
        let b1 = &reducer.branches[0];
        assert_eq!(format!("{}", b1.w), "n");
        assert_eq!(format!("{}", b1.x), "0");
        assert_eq!(format!("{}", b1.y), "0");
        assert_eq!(format!("{}", b1.z), "0");
        assert_eq!(format!("{}", b1.constraints), "((e + -7) = f) is true, ((d + -8) = g) is true, ((h + 7) = i) is true, ((j + 5) = k) is true, ((c + 2) = l) is true, ((b + -3) = m) is true, ((a + -2) = n) is true");
    }
}
