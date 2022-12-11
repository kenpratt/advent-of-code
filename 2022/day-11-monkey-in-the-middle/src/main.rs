use std::collections::HashMap;
use std::fs;

use itertools::Itertools;
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
    monkeys: HashMap<usize, Monkey>,
    monkey_ids: Vec<usize>,
}

impl Monkeys {
    fn parse(input: &str) -> Self {
        let monkeys: HashMap<usize, Monkey> = input
            .split("\n\n")
            .map(|line| Monkey::parse(line))
            .map(|m| (m.id, m))
            .collect();
        let mut monkey_ids: Vec<usize> = monkeys.keys().cloned().collect();
        monkey_ids.sort();
        Self {
            monkeys,
            monkey_ids,
        }
    }

    fn run(&mut self, num_rounds: usize, reduce_fn: impl Fn(usize) -> usize) {
        for r in 0..num_rounds {
            println!("round {}:", r + 1);
            self.run_round(&reduce_fn);

            for id in &self.monkey_ids {
                println!(
                    "  {}: {:?} ({})",
                    id, self.monkeys[id].items, self.monkeys[id].num_inspected
                );
            }
        }
    }

    fn run_round(&mut self, reduce_fn: &dyn Fn(usize) -> usize) {
        for id in &self.monkey_ids {
            let thrown = self.monkeys.get_mut(id).unwrap().throw_items(reduce_fn);
            for (thrown_id, thrown_item) in thrown {
                self.monkeys.get_mut(&thrown_id).unwrap().catch(thrown_item);
            }
        }
    }
}

#[derive(Debug)]
struct Monkey {
    id: usize,
    items: Vec<usize>,
    num_inspected: usize,
    operation: Operation,
    test: Test,
}

impl Monkey {
    fn parse(input: &str) -> Self {
        let mut lines = input.lines();
        let id = Self::parse_id(lines.next().unwrap());
        let items = Self::parse_items(lines.next().unwrap());
        let num_inspected = 0;
        let operation = Operation::parse(lines.next().unwrap());
        let test = Test::parse(&mut lines);
        assert_eq!(lines.next(), None);
        Self {
            id,
            items,
            num_inspected,
            operation,
            test,
        }
    }

    fn parse_id(line: &str) -> usize {
        lazy_static! {
            static ref MONKEY_ID_RE: Regex = Regex::new(r"\AMonkey (\d+):\z").unwrap();
        }
        let caps = MONKEY_ID_RE.captures(line).unwrap();
        caps.get(1).unwrap().as_str().parse::<usize>().unwrap()
    }

    fn parse_items(line: &str) -> Vec<usize> {
        lazy_static! {
            static ref ITEMS_RE: Regex = Regex::new(r"\A  Starting items: ([\d, ]+)\z").unwrap();
        }
        let caps = ITEMS_RE.captures(line).unwrap();
        caps.get(1)
            .unwrap()
            .as_str()
            .split(", ")
            .map(|s| s.parse::<usize>().unwrap())
            .collect()
    }

    // calculate where to throw items to, and new priorities
    fn throw_items(&mut self, reduce_fn: &dyn Fn(usize) -> usize) -> Vec<(usize, usize)> {
        self.num_inspected += self.items.len();
        self.items
            .drain(..)
            .map(|item| {
                let base_new_priority = self.operation.apply(item);
                let new_priority = reduce_fn(base_new_priority);
                let new_id = self.test.apply(new_priority);
                (new_id, new_priority)
            })
            .collect()
    }

    fn catch(&mut self, item: usize) {
        self.items.push(item);
    }
}

#[derive(Debug)]
struct Operation {
    left_operand: Operand,
    operator: Operator,
    right_operand: Operand,
}

impl Operation {
    fn parse(line: &str) -> Self {
        lazy_static! {
            static ref OPERATION_RE: Regex =
                Regex::new(r"\A  Operation: new = (old|\d+) ([\+\*]) (old|\d+)\z").unwrap();
        }
        let caps = OPERATION_RE.captures(line).unwrap();
        let left_operand = Operand::parse(caps.get(1).unwrap().as_str());
        let operator = Operator::parse(caps.get(2).unwrap().as_str());
        let right_operand = Operand::parse(caps.get(3).unwrap().as_str());
        Self {
            left_operand,
            operator,
            right_operand,
        }
    }

    fn apply(&self, old_value: usize) -> usize {
        let left = self.left_operand.apply(old_value);
        let right = self.right_operand.apply(old_value);
        self.operator.apply(left, right)
    }
}

#[derive(Debug)]
enum Operator {
    Plus,
    Multiply,
}

impl Operator {
    fn parse(input: &str) -> Self {
        use Operator::*;

        match input {
            "+" => Plus,
            "*" => Multiply,
            _ => panic!("Unknown operator: {}", input),
        }
    }

    fn apply(&self, left: usize, right: usize) -> usize {
        use Operator::*;
        match self {
            Plus => left + right,
            Multiply => left * right,
        }
    }
}

#[derive(Debug)]
enum Operand {
    Variable,
    Fixed(usize),
}

impl Operand {
    fn parse(input: &str) -> Self {
        use Operand::*;

        match input {
            "old" => Variable,
            _ => Fixed(input.parse::<usize>().unwrap()),
        }
    }

    fn apply(&self, value: usize) -> usize {
        use Operand::*;

        match self {
            Variable => value,
            Fixed(x) => *x,
        }
    }
}

#[derive(Debug)]
struct Test {
    divisor: usize,
    true_id: usize,
    false_id: usize,
}

impl Test {
    fn parse(lines: &mut std::str::Lines) -> Self {
        lazy_static! {
            static ref TEST_RE_1: Regex = Regex::new(r"\A  Test: divisible by (\d+)\z").unwrap();
            static ref TEST_RE_2: Regex =
                Regex::new(r"\A    If true: throw to monkey (\d+)\z").unwrap();
            static ref TEST_RE_3: Regex =
                Regex::new(r"\A    If false: throw to monkey (\d+)\z").unwrap();
        }
        let line1 = lines.next().unwrap();
        let line2 = lines.next().unwrap();
        let line3 = lines.next().unwrap();

        let caps1 = TEST_RE_1.captures(line1).unwrap();
        let caps2 = TEST_RE_2.captures(line2).unwrap();
        let caps3 = TEST_RE_3.captures(line3).unwrap();

        let divisor = caps1.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let true_id = caps2.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let false_id = caps3.get(1).unwrap().as_str().parse::<usize>().unwrap();

        Self {
            divisor,
            true_id,
            false_id,
        }
    }

    fn apply(&self, value: usize) -> usize {
        if value % self.divisor == 0 {
            self.true_id
        } else {
            self.false_id
        }
    }
}

fn least_common_multiple(values: &[usize]) -> usize {
    values
        .iter()
        .cloned()
        .reduce(|acc, val| num::integer::lcm(acc, val))
        .unwrap()
}

fn part1(input: &str) -> usize {
    let mut monkeys = Monkeys::parse(input);
    dbg!(&monkeys);
    monkeys.run(20, |n| n / 3);
    monkeys
        .monkeys
        .iter()
        .map(|(_, m)| m.num_inspected)
        .sorted()
        .rev()
        .take(2)
        .reduce(|acc, v| acc * v)
        .unwrap()
}

fn part2(input: &str) -> usize {
    let mut monkeys = Monkeys::parse(input);
    dbg!(&monkeys);

    let divisors: Vec<usize> = monkeys
        .monkeys
        .iter()
        .map(|(_, m)| m.test.divisor)
        .collect();
    let lcm = least_common_multiple(&divisors);

    monkeys.run(10000, |n| n % lcm);

    monkeys
        .monkeys
        .iter()
        .map(|(_, m)| m.num_inspected)
        .sorted()
        .rev()
        .take(2)
        .reduce(|acc, v| acc * v)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    static EXAMPLE1: &str = indoc! {"
        Monkey 0:
          Starting items: 79, 98
          Operation: new = old * 19
          Test: divisible by 23
            If true: throw to monkey 2
            If false: throw to monkey 3
        
        Monkey 1:
          Starting items: 54, 65, 75, 74
          Operation: new = old + 6
          Test: divisible by 19
            If true: throw to monkey 2
            If false: throw to monkey 0
        
        Monkey 2:
          Starting items: 79, 60, 97
          Operation: new = old * old
          Test: divisible by 13
            If true: throw to monkey 1
            If false: throw to monkey 3
        
        Monkey 3:
          Starting items: 74
          Operation: new = old + 3
          Test: divisible by 17
            If true: throw to monkey 0
            If false: throw to monkey 1
    "};

    #[test]
    fn test_part1_example1() {
        let result = part1(EXAMPLE1);
        assert_eq!(result, 10605);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file());
        assert_eq!(result, 182293);
    }

    #[test]
    fn test_part2_example1() {
        let result = part2(EXAMPLE1);
        assert_eq!(result, 2713310158);
    }

    #[test]
    fn test_part2_solution() {
        let result = part2(&read_input_file());
        assert_eq!(result, 54832778815);
    }
}
