use std::fs;
use std::iter::zip;

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
struct Race {
    duration: usize,
    record: usize,
}

impl Race {
    fn parse_list(input: &str) -> Vec<Self> {
        lazy_static! {
            static ref INPUT_RE: Regex =
                Regex::new(r"\ATime:\s+([\s\d]+)\nDistance:\s+([\s\d]+)\n?\z").unwrap();
        }

        let caps = INPUT_RE.captures(input).unwrap();
        let durations = parse_number_list(caps.get(1).unwrap().as_str());
        let records = parse_number_list(caps.get(2).unwrap().as_str());

        zip(durations, records)
            .map(|(duration, record)| Race { duration, record })
            .collect()
    }

    fn strategies(&self) -> Vec<(usize, usize)> {
        (0..=self.duration)
            .map(|hold| {
                let travel = self.duration - hold;
                let distance = hold * travel;
                (hold, distance)
            })
            .collect()
    }

    fn num_strategies_beating_record(&self) -> usize {
        self.strategies()
            .iter()
            .filter(|(_hold, distance)| distance > &self.record)
            .count()
    }
}

fn parse_number_list(input: &str) -> Vec<usize> {
    input
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect()
}

fn part1(input: &str) -> usize {
    let races = Race::parse_list(input);
    races
        .iter()
        .map(|race| race.num_strategies_beating_record())
        .product()
}

// fn part2(input: &str) -> usize {
//     let races = Data::parse(input);
//     dbg!(&races);
//     0
// }

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    static EXAMPLE: &str = indoc! {"
        Time:      7  15   30
        Distance:  9  40  200
    "};

    #[test]
    fn test_part1_example() {
        let result = part1(EXAMPLE);
        assert_eq!(result, 288);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file());
        assert_eq!(result, 3316275);
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
