use std::fs;

use indoc::indoc;
// use lazy_static::lazy_static;
// use regex::Regex;

fn main() {
    println!("part 1 result: {:?}", part1(read_input_file()));
    //println!("part 2 result: {:?}", part2(read_input_file()));
}

fn read_input_file() -> String {
    return fs::read_to_string("input.txt").expect("Something went wrong reading the file");
}

#[derive(Debug)]
struct Data {
    lines: Vec<usize>,
}

impl Data {
    fn parse(input: String) -> Data {
        let lines = input.lines().map(|line| Data::parse_line(line)).collect();
        return Data {
            lines: lines,
        }
    }
    
    fn parse_line(line: &str) -> usize {
        return line.parse::<usize>().unwrap();
    }

    fn execute(&self) -> usize {
        return 0;
    }
}

fn part1(input: String) -> usize {
    let data = Data::parse(input);
    return data.execute();
}

// fn part2(input: String) -> usize {
//     let data = Data::parse(input);
//     return data.execute();
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example1() {
        let input = indoc! {"
            input
        "};
        let result = part1(input.to_string());
        assert_eq!(result, 0);
    }

    // #[test]
    // fn test_part1_solution() {
    //     let result = part1(
    //         read_input_file()
    //     );
    //     assert_eq!(result, 0);
    // }

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