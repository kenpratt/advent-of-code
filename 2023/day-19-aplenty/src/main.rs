use std::{collections::HashMap, fs};

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
struct System<'a> {
    workflows: HashMap<&'a str, Workflow<'a>>,
    parts: Vec<Part>,
}

impl<'a> System<'a> {
    fn parse(input: &'a str) -> Self {
        let mut iter = input.split("\n\n");
        let workflows = Workflow::parse_list(iter.next().unwrap())
            .into_iter()
            .map(|w| (w.name, w))
            .collect();
        let parts = Part::parse_list(iter.next().unwrap());
        Self { workflows, parts }
    }

    fn run_part(&self, part: &Part) -> WorkflowResult {
        let mut to_run = INITIAL_WORKFLOW;

        loop {
            let workflow = self.workflows.get(to_run).unwrap();
            let result = workflow.run(part);
            match result {
                WorkflowResult::Accepted | WorkflowResult::Rejected => {
                    return result;
                }
                WorkflowResult::Redirected(name) => {
                    // redirect to another workflow
                    to_run = name;
                }
            }
        }
    }

    fn accepted_parts(&self) -> Vec<&Part> {
        self.parts
            .iter()
            .filter(|part| self.run_part(part) == WorkflowResult::Accepted)
            .collect()
    }
}

const INITIAL_WORKFLOW: &'static str = "in";

#[derive(Debug)]
struct Workflow<'a> {
    name: &'a str,
    rules: Vec<Rule<'a>>,
}

impl<'a> Workflow<'a> {
    fn parse_list(input: &'a str) -> Vec<Self> {
        input.lines().map(|line| Self::parse(line)).collect()
    }

    fn parse(input: &'a str) -> Self {
        lazy_static! {
            static ref WORKFLOW_RE: Regex = Regex::new(r"\A([a-z]+)\{(.+)\}\z").unwrap();
        }

        let caps = WORKFLOW_RE.captures(input).unwrap();
        let name = caps.get(1).unwrap().as_str();
        let rules = Rule::parse_list(caps.get(2).unwrap().as_str());
        Self { name, rules }
    }

    fn run(&self, part: &Part) -> WorkflowResult {
        let result = self.rules.iter().find_map(|rule| rule.run(part));
        // the final rule should always return a result, if none of the other ones do
        result.unwrap()
    }
}

#[derive(Debug)]
struct Rule<'a> {
    condition: Option<Condition>,
    result: WorkflowResult<'a>,
}

impl<'a> Rule<'a> {
    fn parse_list(input: &'a str) -> Vec<Self> {
        input.split(',').map(|chunk| Self::parse(chunk)).collect()
    }

    fn parse(input: &'a str) -> Self {
        let chunks: Vec<&str> = input.split(':').collect();
        match chunks.len() {
            1 => {
                let condition = None;
                let result = WorkflowResult::parse(chunks[0]);
                Self { condition, result }
            }
            2 => {
                let condition = Some(Condition::parse(chunks[0]));
                let result = WorkflowResult::parse(chunks[1]);
                Self { condition, result }
            }
            _ => panic!("Unexpected rule input: {:?}", input),
        }
    }

    fn run(&self, part: &Part) -> Option<WorkflowResult> {
        match &self.condition {
            Some(cond) => match cond.run(part) {
                true => Some(self.result),
                false => None,
            },
            None => Some(self.result),
        }
    }
}

#[derive(Debug)]
struct Condition {
    operation: Operation,
    argument: PartProperty,
    value: u16,
}

impl Condition {
    fn parse(input: &str) -> Self {
        lazy_static! {
            static ref CONDITION_RE: Regex = Regex::new(r"\A([xmas])([<>])(\d+)\z").unwrap();
        }

        let caps = CONDITION_RE.captures(input).unwrap();
        let argument = PartProperty::parse(caps.get(1).unwrap().as_str());
        let operation = Operation::parse(caps.get(2).unwrap().as_str());
        let value = caps.get(3).unwrap().as_str().parse::<u16>().unwrap();
        Self {
            operation,
            argument,
            value,
        }
    }

    fn run(&self, part: &Part) -> bool {
        let var = part.property(&self.argument);
        self.operation.run(var, self.value)
    }
}

#[derive(Debug)]
enum Operation {
    LessThan,
    GreaterThan,
}

impl Operation {
    fn parse(input: &str) -> Self {
        use Operation::*;

        match input {
            "<" => LessThan,
            ">" => GreaterThan,
            _ => panic!("Unexpected operation: {:?}", input),
        }
    }

    fn run(&self, x: u16, y: u16) -> bool {
        use Operation::*;

        match self {
            LessThan => x < y,
            GreaterThan => x > y,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum WorkflowResult<'a> {
    Accepted,
    Rejected,
    Redirected(&'a str),
}

impl<'a> WorkflowResult<'a> {
    fn parse(input: &'a str) -> Self {
        use WorkflowResult::*;

        match input {
            "A" => Accepted,
            "R" => Rejected,
            _ => Redirected(input),
        }
    }
}

#[derive(Debug)]
struct Part {
    x: u16,
    m: u16,
    a: u16,
    s: u16,
}

impl Part {
    fn parse_list(input: &str) -> Vec<Self> {
        input.lines().map(|line| Self::parse(line)).collect()
    }

    fn parse(input: &str) -> Self {
        lazy_static! {
            static ref PART_RE: Regex =
                Regex::new(r"\A\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}\z").unwrap();
        }

        let caps = PART_RE.captures(input).unwrap();
        let x = caps.get(1).unwrap().as_str().parse::<u16>().unwrap();
        let m = caps.get(2).unwrap().as_str().parse::<u16>().unwrap();
        let a = caps.get(3).unwrap().as_str().parse::<u16>().unwrap();
        let s = caps.get(4).unwrap().as_str().parse::<u16>().unwrap();
        Self { x, m, a, s }
    }

    fn property(&self, prop: &PartProperty) -> u16 {
        use PartProperty::*;

        match prop {
            X => self.x,
            M => self.m,
            A => self.a,
            S => self.s,
        }
    }

    fn rating(&self) -> u16 {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Debug)]
enum PartProperty {
    X,
    M,
    A,
    S,
}

impl PartProperty {
    fn parse(input: &str) -> Self {
        use PartProperty::*;

        match input {
            "x" => X,
            "m" => M,
            "a" => A,
            "s" => S,
            _ => panic!("Unexpected property: {:?}", input),
        }
    }
}

fn part1(input: &str) -> usize {
    let system = System::parse(input);
    system
        .accepted_parts()
        .into_iter()
        .map(|part| part.rating() as usize)
        .sum()
}

// fn part2(input: &str) -> usize {
//     0
// }

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    static EXAMPLE: &str = indoc! {"
        px{a<2006:qkq,m>2090:A,rfg}
        pv{a>1716:R,A}
        lnx{m>1548:A,A}
        rfg{s<537:gd,x>2440:R,A}
        qs{s>3448:A,lnx}
        qkq{x<1416:A,crn}
        crn{x>2662:A,R}
        in{s<1351:px,qqz}
        qqz{s>2770:qs,m<1801:hdj,R}
        gd{a>3333:R,R}
        hdj{m>838:A,pv}
        
        {x=787,m=2655,a=1222,s=2876}
        {x=1679,m=44,a=2067,s=496}
        {x=2036,m=264,a=79,s=2244}
        {x=2461,m=1339,a=466,s=291}
        {x=2127,m=1623,a=2188,s=1013}
    "};

    #[test]
    fn test_part1_example() {
        let result = part1(EXAMPLE);
        assert_eq!(result, 19114);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file());
        assert_eq!(result, 420739);
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
