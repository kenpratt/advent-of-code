use std::collections::HashMap;

use std::fs;

use indoc::indoc;
use lazy_static::lazy_static;
use regex::Regex;

static PART_2_RULE_OVERRIDES: &str = indoc! {"
    8: 42 | 42 8
    11: 42 31 | 42 11 31
"};

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    println!("part 2 result: {:?}", part2(&read_input_file()));
}

fn read_input_file() -> String {
    return fs::read_to_string("input.txt").expect("Something went wrong reading the file");
}

#[derive(Debug)]
struct MessageValidator {
    rules: HashMap<usize, Rule>,
    messages: Vec<String>,
}

impl MessageValidator {
    fn parse(input: &str) -> MessageValidator {
        let parts: Vec<&str> = input.split("\n\n").collect();
        assert_eq!(parts.len(), 2);

        let rules = MessageValidator::parse_rules(parts[0]);
        let messages = parts[1].lines().map(|line| line.to_string()).collect();
        
        MessageValidator {
            rules: rules,
            messages: messages,
        }
    }

    fn apply_rule_overrides(&mut self, input: &str) {
        let overrides = MessageValidator::parse_rules(input);
        self.rules.extend(overrides);
    }

    fn parse_rules(input: &str) -> HashMap<usize, Rule> {
        input.lines().map(|line| Rule::parse(line)).collect()
    }

    fn num_valid_messages(&self) -> usize {
        self.messages.iter().filter(|m| self.is_message_valid(m)).count()
    }

    fn is_message_valid(&self, message: &str) -> bool {
        Solver::is_valid(message, &self.rules, 0)
    }
}

#[derive(Debug)]
enum Rule {
    SingleCharacter(char),
    Delegate(Vec<usize>),
    DelegateOr(Vec<usize>, Vec<usize>),
}

impl Rule {
    fn parse(input: &str) -> (usize, Rule) {
        lazy_static! {
            static ref SINGLE_CHAR_RE: Regex = Regex::new(r#"\A(\d+): "([a-z])"\z"#).unwrap();
            static ref DELEGATION_RE: Regex = Regex::new(r#"\A(\d+): ([\d\s]+)\z"#).unwrap();
            static ref MULTI_DELEGATION_RE: Regex = Regex::new(r#"\A(\d+): ([\d\s]+) \| ([\d\s]+)\z"#).unwrap();
        }

        if SINGLE_CHAR_RE.is_match(input) {
            let captures = SINGLE_CHAR_RE.captures(input).unwrap();
            let id = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let target_char = captures.get(2).unwrap().as_str().parse::<char>().unwrap();
            (id, Rule::SingleCharacter(target_char))
        } else if DELEGATION_RE.is_match(input) {
            let captures = DELEGATION_RE.captures(input).unwrap();
            let id = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let delegate_ids = Rule::parse_ids(captures.get(2).unwrap().as_str());
            (id, Rule::Delegate(delegate_ids))
        } else if MULTI_DELEGATION_RE.is_match(input) {
            let captures = MULTI_DELEGATION_RE.captures(input).unwrap();
            let id = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let delegate_ids1 = Rule::parse_ids(captures.get(2).unwrap().as_str());
            let delegate_ids2 = Rule::parse_ids(captures.get(3).unwrap().as_str());
            (id, Rule::DelegateOr(delegate_ids1, delegate_ids2))
        } else {
            panic!("Can't find rule pattern matching input: {}", input)
        }
    }

    fn parse_ids(input: &str) -> Vec<usize> {
        input.split(' ').map(|s| s.parse::<usize>().unwrap()).collect()
    }
}

#[derive(Debug)]
struct Solver<'a> {
    input: &'a Vec<char>,
    last_input_index: usize,
    rules: &'a HashMap<usize, Rule>,
    branches: Vec<Branch>,
}

#[derive(Debug)]
struct Branch {
    input_index: usize,
    current_rule: usize,
    leftover_rules: Vec<usize>,
}

#[derive(Debug)]
enum StepResult {
    Valid,
    Invalid,
    Continue(Vec<Branch>),
}

impl Solver<'_> {
    fn is_valid(input: &str, rules: &HashMap<usize, Rule>, starting_id: usize) -> bool {
        let starting_branch = Branch {
            input_index: 0,
            current_rule: starting_id,
            leftover_rules: vec![],
        };

        let chars = input.chars().collect();
        let mut state = Solver {
            input: &chars,
            last_input_index: chars.len() - 1,
            rules: rules,
            branches: vec![starting_branch],
        };

        // run until a valid branch is found, or we run out of branches to try
        let mut is_valid = false;
        while !is_valid && !state.branches.is_empty() {
            is_valid = Solver::step(&mut state);
        }
        is_valid
    }

    fn step(state: &mut Solver) -> bool {
        let branch = state.branches.pop().unwrap();
        match Solver::step_branch(&branch, state) {
            StepResult::Valid => true,
            StepResult::Invalid => false,
            StepResult::Continue(branches) => {
                state.branches.extend(branches);
                false
            }
        }
    }

    fn step_branch(branch: &Branch, state: &Solver) -> StepResult {
        let rule = state.rules.get(&branch.current_rule).unwrap();
        match rule {
            Rule::SingleCharacter(char) => {
                if state.input[branch.input_index] == *char {
                    // we have a char match!
                    let end_of_input = branch.input_index == state.last_input_index;
                    let out_of_rules = branch.leftover_rules.is_empty();

                    match (end_of_input, out_of_rules) {
                        // done this rule tree, and done input string, yay!
                        (true, true) => StepResult::Valid,

                        // no more input but we have leftovers :(
                        (true, false) => StepResult::Invalid,

                        // still have input, but out of rules :(
                        (false, true) => StepResult::Invalid,

                        // not done yet!
                        (false, false) => StepResult::Continue(vec![branch.advance()]),
                    }
                } else {
                    // char mismatch
                    StepResult::Invalid
                }
            },
            Rule::Delegate(ids) => {
                StepResult::Continue(vec![branch.delegate(ids)])
            }
            Rule::DelegateOr(ids1, ids2) => {
                StepResult::Continue(vec![branch.delegate(ids1), branch.delegate(ids2)])
            },
        }
    }
}

impl Branch {
    fn advance(&self) -> Branch {
        Branch {
            input_index: self.input_index + 1,
            current_rule: self.leftover_rules[0],
            leftover_rules: self.leftover_rules[1..].to_vec(),
        }
    }

    fn delegate(&self, ids: &Vec<usize>) -> Branch {
        let mut leftover_rules = ids[1..].to_vec();
        leftover_rules.extend(&self.leftover_rules);
    
        Branch {
            input_index: self.input_index,
            current_rule: ids[0],
            leftover_rules: leftover_rules,
        }
    }
}

fn part1(input: &str) -> usize {
    let validator = MessageValidator::parse(input);
    return validator.num_valid_messages();
}

fn part2(input: &str) -> usize {
    let mut validator = MessageValidator::parse(input);
    validator.apply_rule_overrides(PART_2_RULE_OVERRIDES);
    return validator.num_valid_messages();
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE1: &str = indoc! {"
        0: 4 1 5
        1: 2 3 | 3 2
        2: 4 4 | 5 5
        3: 4 5 | 5 4
        4: \"a\"
        5: \"b\"
    
        ababbb
        bababa
        abbbab
        aaabbb
        aaaabbb
    "};

    static EXAMPLE2: &str = indoc! {"
        42: 9 14 | 10 1
        9: 14 27 | 1 26
        10: 23 14 | 28 1
        1: \"a\"
        11: 42 31
        5: 1 14 | 15 1
        19: 14 1 | 14 14
        12: 24 14 | 19 1
        16: 15 1 | 14 14
        31: 14 17 | 1 13
        6: 14 14 | 1 14
        2: 1 24 | 14 4
        0: 8 11
        13: 14 3 | 1 12
        15: 1 | 14
        17: 14 2 | 1 7
        23: 25 1 | 22 14
        28: 16 1
        4: 1 1
        20: 14 14 | 1 15
        3: 5 14 | 16 1
        27: 1 6 | 14 18
        14: \"b\"
        21: 14 1 | 1 14
        25: 1 1 | 1 14
        22: 14 14
        8: 42
        26: 14 22 | 1 20
        18: 15 15
        7: 14 5 | 1 21
        24: 14 1
    
        abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
        bbabbbbaabaabba
        babbbbaabbbbbabbbbbbaabaaabaaa
        aaabbbbbbaaaabaababaabababbabaaabbababababaaa
        bbbbbbbaaaabbbbaaabbabaaa
        bbbababbbbaaaaaaaabbababaaababaabab
        ababaaaaaabaaab
        ababaaaaabbbaba
        baabbaaaabbaaaababbaababb
        abbbbabbbbaaaababbbbbbaaaababb
        aaaaabbaabaaaaababaa
        aaaabbaaaabbaaa
        aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
        babaaabbbaaabaababbaabababaaab
        aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba
    "};

    #[test]
    fn test_part1_example1() {
        let result = part1(EXAMPLE1);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_part1_example2() {
        let result = part1(EXAMPLE2);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file());
        assert_eq!(result, 165);
    }

    #[test]
    fn test_part2_example2() {
        let result = part2(EXAMPLE2);
        assert_eq!(result, 12);
    }

    #[test]
    fn test_part2_solution() {
        let result = part2(&read_input_file());
        assert_eq!(result, 274);
    }
}