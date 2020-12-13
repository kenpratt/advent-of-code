use std::fs;
use regex::Regex;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    println!("part 2 result: {:?}", part2(&read_input_file()));
}

fn read_input_file() -> String {
    return fs::read_to_string("input.txt").expect("Something went wrong reading the file");
}

fn parse_input(input: &str) -> Vec<PasswordWithPolicy> {
    return input.lines().map(|line| PasswordWithPolicy::parse(line)).collect();
}

#[derive(Debug)]
struct PasswordWithPolicy {
    target_character: char,
    minimum: usize,
    maximum: usize,
    password: String,
}

impl PasswordWithPolicy { 
    fn parse(line: &str) -> PasswordWithPolicy {
        let re = Regex::new(r"^(\d+)\-(\d+) ([a-z]): ([a-z]+)$").unwrap();
        let captures = re.captures(line).unwrap();
        let minimum = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let maximum = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();
        let target_character = captures.get(3).unwrap().as_str().parse::<char>().unwrap();
        let password = captures.get(4).unwrap().as_str().into();
        return PasswordWithPolicy {
            target_character: target_character,
            minimum: minimum,
            maximum: maximum,
            password: password
        };        
    }

    fn valid_using_count_policy(&self) -> bool {
        let num_matches = self.password.chars().filter(|&c| c == self.target_character).count();
        return num_matches >= self.minimum && num_matches <= self.maximum;
    }

    fn valid_using_index_policy(&self) -> bool {
        let pass1 = self.password.chars().nth(self.minimum - 1) == Some(self.target_character);
        let pass2 = self.password.chars().nth(self.maximum - 1) == Some(self.target_character);
        return (pass1 || pass2) && !(pass1 && pass2);
    }
}

fn part1(input: &str) -> usize {
    let entries = parse_input(input);
    return entries.iter().filter(|&e| e.valid_using_count_policy()).count();
}

fn part2(input: &str) -> usize {
    let entries = parse_input(input);
    return entries.iter().filter(|&e| e.valid_using_index_policy()).count();
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    static EXAMPLE1: &str = indoc! {"
        1-3 a: abcde
        1-3 b: cdefg
        2-9 c: ccccccccc
    "};

    #[test]
    fn test_part1_example1() {
        let result = part1(EXAMPLE1);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file());
        assert_eq!(result, 636);
    }

    #[test]
    fn test_part2_example1() {
        let result = part2(EXAMPLE1);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_part2_solution() {
        let result = part2(&read_input_file());
        assert_eq!(result, 588);
    }
}