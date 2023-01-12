use std::fs;

use std::collections::HashMap;

// use itertools::Itertools;
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
        }
    }
}

#[derive(Debug)]
struct Stack {
    main: Vec<StackEntry>,
    values: Vec<isize>,
}

impl Stack {
    fn new() -> Self {
        Self {
            main: vec![],
            values: vec![],
        }
    }

    fn run(&mut self, start_name: &String, jobs: &HashMap<String, Job>) -> isize {
        use StackEntry::*;

        // start with just one name entry on the stack, to expand
        self.main.push(Name(start_name.clone()));

        while !self.main.is_empty() {
            // println!("\nbefore: {:?}", self);
            let entry = self.main.pop().unwrap();
            // println!("applying {:?}", entry);
            match entry {
                Name(name) => {
                    // expansion, add job to stack
                    let job = jobs.get(&name).unwrap();
                    self.push_job(job)
                }
                Operator(op) => {
                    // apply operation an push result onto stack
                    let left = self.values.pop().unwrap();
                    let right = self.values.pop().unwrap();
                    let result = op.apply(left, right);
                    self.main.push(Value(result));
                }
                Value(v) => {
                    // move this to the pending values stack
                    // for the operator to apply
                    self.values.push(v);
                }
            }
            // println!("after: {:?}", self);
        }

        // return last value on the stack
        assert_eq!(1, self.values.len());
        self.values.pop().unwrap()
    }

    fn push_job(&mut self, job: &Job) {
        use StackEntry::*;

        match job {
            Job::Operation(op, l, r) => {
                self.main.push(Operator(*op));
                self.main.push(Name(l.clone()));
                self.main.push(Name(r.clone()));
            }
            Job::Value(v) => {
                self.main.push(Value(*v));
            }
        }
    }
}

#[derive(Debug)]
enum StackEntry {
    Name(String),
    Operator(Operator),
    Value(isize),
}

fn part1(input: &str) -> isize {
    let monkeys = Monkeys::parse(input);
    monkeys.run(&"root".to_string())
}

// fn part2(input: &str) -> usize {
//     let items = Data::parse(input);
//     dbg!(&items);
//     0
// }

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

    // #[test]
    // fn test_part2_example() {
    //     let result = part2(EXAMPLE);
    //     assert_eq!(result, 0);
    // }

    // #[test]
    // fn test_part2_solution() {
    //     let result = part2(&read_input_file());
    //     assert_eq!(result, 0);
    // }
}
