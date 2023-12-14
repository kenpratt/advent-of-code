use std::fs::{self};

use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    println!("part 2 result: {:?}", part2(&read_input_file()));
}

fn read_input_file() -> String {
    fs::read_to_string("input.txt").expect("Something went wrong reading the file")
}

fn parse(input: &str) -> Vec<&str> {
    input.trim().split(',').collect()
}

fn hash(input: &str) -> usize {
    input.chars().fold(0, |acc, c| hash_next_char(acc, &c))
}

fn hash_next_char(acc: usize, c: &char) -> usize {
    ((acc + *c as usize) * 17) % 256
}

#[derive(Debug)]
struct Step {
    label: String,
    operation: Operation,
    box_number: usize,
}

impl Step {
    fn parse(input: &str) -> Self {
        lazy_static! {
            static ref STEP_RE: Regex = Regex::new(r"\A([a-z]+)([\-\=])(\d*)\z").unwrap();
        }

        let caps = STEP_RE.captures(input).unwrap();
        let label = caps.get(1).unwrap().as_str().to_string();
        let operation_str = caps.get(2).unwrap().as_str();
        let operation_val_str = caps.get(3).unwrap().as_str();
        let operation = Operation::parse(operation_str, operation_val_str);

        let box_number = hash(&label);

        Self {
            label,
            operation,
            box_number,
        }
    }
}

#[derive(Debug)]
enum Operation {
    Remove,
    Add(usize),
}

impl Operation {
    fn parse(command: &str, value: &str) -> Self {
        match command {
            "-" => {
                assert_eq!(value, "");
                Operation::Remove
            }
            "=" => {
                let val = value.parse::<usize>().unwrap();
                Operation::Add(val)
            }
            _ => panic!("Unknown operation: {}", command),
        }
    }
}

fn calculate_focusing_power(steps: &[Step]) -> usize {
    let mut boxes: [Vec<(String, usize)>; 256] = vec![Vec::new(); 256].try_into().unwrap();

    for step in steps {
        let lenses = &mut boxes[step.box_number];
        match step.operation {
            Operation::Add(new_focal_length) => {
                // replace or append
                match lenses
                    .iter_mut()
                    .find(|(label, _focal_length)| label == &step.label)
                {
                    Some((_label, focal_length)) => *focal_length = new_focal_length,
                    None => lenses.push((step.label.clone(), new_focal_length)),
                }
            }
            Operation::Remove => {
                // remove this label
                lenses.retain(|(label, _focal_length)| label != &step.label)
            }
        }
    }

    boxes
        .iter()
        .enumerate()
        .map(|(box_i, lenses)| {
            lenses
                .iter()
                .enumerate()
                .map(|(lense_i, (_label, focal_length))| (box_i + 1) * (lense_i + 1) * focal_length)
                .sum::<usize>()
        })
        .sum()
}

fn part1(input: &str) -> usize {
    parse(input).into_iter().map(|s| hash(s)).sum()
}

fn part2(input: &str) -> usize {
    let steps: Vec<Step> = parse(input).into_iter().map(|s| Step::parse(s)).collect();
    calculate_focusing_power(&steps)
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    static EXAMPLE1: &str = indoc! {"
        HASH
    "};

    static EXAMPLE2: &str = indoc! {"
        rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
    "};

    #[test]
    fn test_part1_example1() {
        let result = part1(EXAMPLE1);
        assert_eq!(result, 52);
    }

    #[test]
    fn test_part1_example2() {
        let result = part1(EXAMPLE2);
        assert_eq!(result, 1320);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file());
        assert_eq!(result, 512283);
    }

    #[test]
    fn test_part2_example2() {
        let result = part2(EXAMPLE2);
        assert_eq!(result, 145);
    }

    #[test]
    fn test_part2_solution() {
        let result = part2(&read_input_file());
        assert_eq!(result, 215827);
    }
}
