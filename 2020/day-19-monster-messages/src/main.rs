use std::collections::HashMap;

use std::fs;

use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    // println!("part 2 result: {:?}", part2(&read_input_file()));
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

        let rules = parts[0].lines().map(|line| Rule::parse(line)).collect();
        let messages = parts[1].lines().map(|line| line.to_string()).collect();
        
        MessageValidator {
            rules: rules,
            messages: messages,
        }
    }

    fn num_valid_messages(&self) -> usize {
        let re = self.compile_rules(&0);
        self.messages.iter().filter(|s| re.is_match(s)).count()
    }

    fn compile_rules(&self, starting_id: &usize) -> Regex {
        let mut cache = HashMap::new();
        let compiled: String = self.get_compiled_rule(starting_id, &mut cache);
        let re_str = format!("^{}$", compiled);
        Regex::new(&re_str).unwrap()
    }

    fn get_compiled_rule(&self, id: &usize, cache: &mut HashMap<usize, String>) -> String {
        if !cache.contains_key(id) {
            let value = self.compile_rule(id, cache);
            cache.insert(*id, value);
        }
        cache.get(id).unwrap().clone()
    }

    fn compile_rule(&self, id: &usize, cache: &mut HashMap<usize, String>) -> String {
        let rule = self.rules.get(id).unwrap();
        match rule {
            Rule::SingleCharacter(char) => char.to_string(),
            Rule::Delegate(ids) => ids.iter().map(|i| self.get_compiled_rule(i, cache)).collect(),
            Rule::DelegateOr(ids1, ids2) => {
                let left: String = ids1.iter().map(|i| self.get_compiled_rule(i, cache)).collect();
                let right: String = ids2.iter().map(|i| self.get_compiled_rule(i, cache)).collect();
                format!("({}|{})", left, right)
            },
        }
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

fn part1(input: &str) -> usize {
    let validator = MessageValidator::parse(input);
    println!("{:?}", validator);
    return validator.num_valid_messages();
}

// fn part2(input: &str) -> usize {
//     let data = MessageValidator::parse(input);
//     return data.execute();
// }

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

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

    #[test]
    fn test_part1_example1() {
        let result = part1(EXAMPLE1);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file());
        assert_eq!(result, 165);
    }

    // #[test]
    // fn test_part2_example1() {
    //     let result = part2(EXAMPLE1);
    //     assert_eq!(result, 0);
    // }

    // #[test]
    // fn test_part2_solution() {
    //     let result = part2(&read_input_file());
    //     assert_eq!(result, 0);
    // }
}