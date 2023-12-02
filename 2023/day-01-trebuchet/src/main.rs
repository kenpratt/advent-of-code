use std::fs;

use regex::Regex;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    println!("part 2 result: {:?}", part2(&read_input_file()));
}

fn read_input_file() -> String {
    fs::read_to_string("input.txt").expect("Something went wrong reading the file")
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| calibration_value(line, false))
        .sum()
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| calibration_value(line, true))
        .sum()
}

fn calibration_value(input: &str, allow_words: bool) -> u32 {
    let first = first_digit(input, allow_words);
    let last = last_digit(input, allow_words);
    first * 10 + last
}

fn first_digit(input: &str, allow_words: bool) -> u32 {
    let re = if allow_words {
        Regex::new(r"(one|two|three|four|five|six|seven|eight|nine|\d)").unwrap()
    } else {
        Regex::new(r"\d").unwrap()
    };
    let m = re.find(input).unwrap().as_str();
    parse_digit(m)
}

fn last_digit(input: &str, allow_words: bool) -> u32 {
    let re = if allow_words {
        Regex::new(r"(eno|owt|eerht|ruof|evif|xis|neves|thgie|enin|\d)").unwrap()
    } else {
        Regex::new(r"\d").unwrap()
    };
    let input_rev = reverse_str(input);
    let m = re.find(&input_rev).unwrap().as_str();
    parse_digit(&reverse_str(m))
}

fn parse_digit(s: &str) -> u32 {
    match s {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => s.parse::<u32>().unwrap(),
    }
}

fn reverse_str(input: &str) -> String {
    input.chars().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    static EXAMPLE1: &str = indoc! {"
        1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet
    "};

    static EXAMPLE2: &str = indoc! {"
        two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen
    "};

    #[test]
    fn test_part1_example() {
        let result = part1(EXAMPLE1);
        assert_eq!(result, 142);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file());
        assert_eq!(result, 54388);
    }

    #[test]
    fn test_part2_example() {
        let result = part2(EXAMPLE2);
        assert_eq!(result, 281);
    }

    #[test]
    fn test_calibration_value() {
        assert_eq!(calibration_value("twofiveight", true), 28);
    }

    #[test]
    fn test_part2_solution() {
        let result = part2(&read_input_file());
        assert_eq!(result, 53515);
    }
}
