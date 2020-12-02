use std::fs;
use regex::Regex;

fn main() {
    println!("part 1 result: {:?}", part1(read_input_file()));
    //println!("part 2 result: {:?}", part2(read_input_file()));
}

fn read_input_file() -> String {
    return fs::read_to_string("input.txt").expect("Something went wrong reading the file");
}

fn parse_input(input: String) -> Vec<PasswordWithPolicy> {
    return input.lines().map(|line| PasswordWithPolicy::parse(line)).collect();
}

#[derive(Debug)]
struct PasswordWithPolicy {
    target_character: char,
    minimum: u8,
    maximum: u8,
    password: String,
}

impl PasswordWithPolicy { 
    fn parse(line: &str) -> PasswordWithPolicy {
        let re = Regex::new(r"^(\d+)\-(\d+) ([a-z]): ([a-z]+)$").unwrap();
        let captures = re.captures(line).unwrap();
        let minimum = captures.get(1).unwrap().as_str().parse::<u8>().unwrap();
        let maximum = captures.get(2).unwrap().as_str().parse::<u8>().unwrap();
        let target_character = captures.get(3).unwrap().as_str().parse::<char>().unwrap();
        let password = captures.get(4).unwrap().as_str().into();
        return PasswordWithPolicy {
            target_character: target_character,
            minimum: minimum,
            maximum: maximum,
            password: password
        };        
    }

    fn valid(&self) -> bool {
        let num_matches = self.password.chars().filter(|&c| c == self.target_character).count();
        return num_matches >= self.minimum.into() && num_matches <= self.maximum.into();
    }
}

fn part1(input: String) -> usize {
    let entries = parse_input(input);
    return entries.iter().filter(|&e| e.valid()).count();
}

fn part2(input: String) -> u64 {
    let entries = parse_input(input);
    panic!("Error parsing input");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example1() {
        let result = part1(
            "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc".to_string()
        );
        assert_eq!(result, 2);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(
            read_input_file()
        );
        assert_eq!(result, 636);
    }

    // #[test]
    // fn test_part2_example1() {
    //     let result = part2(
    //         "".to_string()
    //     );
    //     assert_eq!(result, 0);
    // }

    // #[test]
    // fn test_part2_solution() {
    //     let result = part2(
    //         read_input_file()
    //     );
    //     assert_eq!(result, 0);
    // }
}