use std::fs;

use std::collections::HashMap;

// use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    println!("part 2 result: {:?}", part2(&read_input_file()));
}

fn read_input_file() -> String {
    fs::read_to_string("input.txt").expect("Something went wrong reading the file")
}

#[derive(Debug)]
struct Monkeys {
    jobs: HashMap<String, Job>,
}

impl Monkeys {
    fn parse(input: &str) -> Self {
        let jobs = input.lines().map(|line| Self::parse_line(line)).collect();
        Self { jobs }
    }

    fn parse_line(input: &str) -> (String, Job) {
        let mut iter = input.split(": ");
        let name = iter.next().unwrap().to_string();
        let job = Job::parse(iter.next().unwrap());
        assert_eq!(None, iter.next());
        (name, job)
    }

    fn run(&self, name: &String) -> isize {
        let mut stack = Stack::new();
        stack.run(name, &self.jobs)
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Job {
    Value(isize),
    Operation(Operator, String, String),
    Variable,
}

impl Job {
    fn parse(input: &str) -> Self {
        use Job::*;

        lazy_static! {
            static ref VALUE_RE: Regex = Regex::new(r"\A(\d+)\z").unwrap();
            static ref OPERATION_RE: Regex =
                Regex::new(r"\A([a-z]+) ([\+\-\*/]) ([a-z]+)\z").unwrap();
        }

        match VALUE_RE.captures(input) {
            Some(caps) => {
                let value = caps.get(1).unwrap().as_str().parse::<isize>().unwrap();
                return Value(value);
            }
            None => {}
        }

        match OPERATION_RE.captures(input) {
            Some(caps) => {
                let left = caps.get(1).unwrap().as_str().to_string();
                let operator = Operator::parse(caps.get(2).unwrap().as_str());
                let right = caps.get(3).unwrap().as_str().to_string();
                return Operation(operator, left, right);
            }
            None => {}
        }

        panic!("Unexpected job input: {}", input);
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Equals,
}

impl Operator {
    fn parse(input: &str) -> Self {
        use Operator::*;
        match input {
            "+" => Plus,
            "-" => Minus,
            "*" => Multiply,
            "/" => Divide,
            _ => panic!("Unexpected operator: {}", input),
        }
    }

    fn apply(&self, left: isize, right: isize) -> isize {
        use Operator::*;
        match self {
            Plus => left + right,
            Minus => left - right,
            Multiply => left * right,
            Divide => left / right,
            Equals => panic!("Should not directly apply equals"),
        }
    }
}

#[derive(Debug)]
struct Stack {
    main: Vec<MainStackEntry>,
    values: Vec<ValueStackEntry>,
}

impl Stack {
    fn new() -> Self {
        Self {
            main: vec![],
            values: vec![],
        }
    }

    fn run(&mut self, start_name: &String, jobs: &HashMap<String, Job>) -> isize {
        use MainStackEntry::*;

        // start with just one name entry on the stack, to expand
        self.main.push(Name(start_name.clone()));

        while !self.main.is_empty() {
            let entry = self.main.pop().unwrap();
            match entry {
                Name(name) => {
                    // expansion, add job to stack
                    let job = jobs.get(&name).unwrap();
                    self.push_job(job)
                }
                Operator(op) => {
                    // apply operation, getting operands from values stack, and
                    // push result onto pending values stack
                    let left = self.values.pop().unwrap();
                    let right = self.values.pop().unwrap();
                    let result = ValueStackEntry::apply_operator(op, left, right);
                    self.values.push(result);
                }
                Value(v) => {
                    // move this to the pending values stack
                    // for the operator to apply
                    self.values.push(ValueStackEntry::Value(v));
                }
                Variable => {
                    // move this to the pending values stack
                    // for the operator to apply
                    self.values.push(ValueStackEntry::Variable);
                }
            }
        }

        // return last value on the stack
        assert_eq!(1, self.values.len());
        match self.values.pop().unwrap() {
            ValueStackEntry::Value(v) => v,
            _ => panic!("Expected last item on the values stack to be a value"),
        }
    }

    fn push_job(&mut self, job: &Job) {
        use MainStackEntry::*;

        match job {
            Job::Operation(op, l, r) => {
                self.main.push(Operator(*op));
                self.main.push(Name(l.clone()));
                self.main.push(Name(r.clone()));
            }
            Job::Value(v) => {
                self.main.push(Value(*v));
            }
            Job::Variable => {
                self.main.push(Variable);
            }
        }
    }
}

#[derive(Debug)]
enum MainStackEntry {
    Name(String),
    Operator(Operator),
    Value(isize),
    Variable,
}

#[derive(Debug)]
enum ValueStackEntry {
    Value(isize),
    Variable,
    VariableOperation(Operator, Box<ValueStackEntry>, Box<ValueStackEntry>),
}

impl ValueStackEntry {
    fn apply_operator(operator: Operator, left: Self, right: Self) -> Self {
        use ValueStackEntry::*;

        match (operator, left, right) {
            // if both sides are literal values, reduce now
            (op, Value(l), Value(r)) => Value(op.apply(l, r)),

            // found an equals with a variable
            (Operator::Equals, l, Value(r)) => Self::solve_for_variable(l, r),
            (Operator::Equals, Value(l), r) => Self::solve_for_variable(r, l),

            // otherwise, it must be a non-equals operation containing a
            // variable -- leave it for later
            (op, l, r) => VariableOperation(op, Box::new(l), Box::new(r)),
        }
    }

    fn solve_for_variable(initial_expression: ValueStackEntry, initial_rhs: isize) -> Self {
        use Operator::*;
        use ValueStackEntry::*;

        let mut expression = initial_expression;
        let mut rhs = initial_rhs;
        loop {
            match expression {
                VariableOperation(operation, left, right) => {
                    match (operation, *left, *right) {
                        (Plus, expr, Value(val)) | (Plus, Value(val), expr) => {
                            // order doesn't matter, either way subtract the value from rhs
                            expression = expr;
                            rhs = Minus.apply(rhs, val);
                        }
                        (Multiply, expr, Value(val)) | (Multiply, Value(val), expr) => {
                            // order doesn't matter, either way divide rhs by the value
                            expression = expr;
                            rhs = Divide.apply(rhs, val);
                        }
                        (Minus, expr, Value(val)) => {
                            expression = expr;
                            rhs = Plus.apply(rhs, val);
                        }
                        (Minus, Value(val), expr) => {
                            expression = expr;
                            rhs = -Minus.apply(rhs, val);
                        }
                        (Divide, expr, Value(val)) => {
                            expression = expr;
                            rhs = Multiply.apply(rhs, val);
                        }
                        (Divide, Value(_), _) => {
                            panic!("Division with value on left is unsupported");
                        }
                        (Equals, _, _) => {
                            panic!("Equals nested inside an expression is unsupported");
                        } 
                        _ => panic!("Unexpected operation format"),
                    }
                }
                Variable => return Value(rhs), // done!
                Value(_) => panic!("Unexpected value in expression argument: {:?}", expression),
            }
        }
    }
}

lazy_static! {
    static ref ROOT: String = "root".to_string();
    static ref HUMN: String = "humn".to_string();
}

fn part1(input: &str) -> isize {
    let monkeys = Monkeys::parse(input);
    monkeys.run(&ROOT)
}

fn part2(input: &str) -> isize {
    let mut monkeys = Monkeys::parse(input);

    // change operator of root to =
    let root = monkeys.jobs.get_mut(&*ROOT).unwrap();
    if let Job::Operation(_, left, right) = root {
        *root = Job::Operation(Operator::Equals, left.clone(), right.clone());
    } else {
        panic!("root must be an operation");
    }

    // change the value of humn to a variable
    let humn = monkeys.jobs.get_mut(&*HUMN).unwrap();
    *humn = Job::Variable;

    monkeys.run(&*ROOT)
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    static EXAMPLE: &str = indoc! {"
        root: pppw + sjmn
        dbpl: 5
        cczh: sllz + lgvd
        zczc: 2
        ptdq: humn - dvpt
        dvpt: 3
        lfqf: 4
        humn: 5
        ljgn: 2
        sjmn: drzm * dbpl
        sllz: 4
        pppw: cczh / lfqf
        lgvd: ljgn * ptdq
        drzm: hmdt - zczc
        hmdt: 32
    "};

    #[test]
    fn test_part1_example() {
        let result = part1(EXAMPLE);
        assert_eq!(result, 152);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file());
        assert_eq!(result, 194501589693264);
    }

    #[test]
    fn test_part2_example() {
        let result = part2(EXAMPLE);
        assert_eq!(result, 301);
    }

    #[test]
    fn test_part2_solution() {
        let result = part2(&read_input_file());
        assert_eq!(result, 3887609741189);
    }
}
