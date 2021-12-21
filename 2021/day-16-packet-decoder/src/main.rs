pub mod bitstream;

use std::fs;

// use itertools::Itertools;
// use lazy_static::lazy_static;
// use regex::Regex;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    // println!("part 2 result: {:?}", part2(&read_input_file()));
}

fn read_input_file() -> String {
    fs::read_to_string("input.txt").expect("Something went wrong reading the file")
}

// fn decode(input: &str) -> Packet {
//     let mut stream = BitStream::new(input.chars());
//     Packet::decode(stream)
// }

#[derive(Debug)]
struct Data {
    parts: Vec<Part>,
}

impl Data {
    fn parse(input: &str) -> Data {
        let parts = input.lines().map(|line| Part::parse(line)).collect();
        Data { parts: parts }
    }

    fn execute(&self) -> usize {
        0
    }
}

#[derive(Debug)]
struct Part {
    foo: usize,
}

impl Part {
    fn parse(input: &str) -> Part {
        Part { foo: input.len() }
    }
}

fn part1(input: &str) -> usize {
    let data = Data::parse(input);
    println!("{:?}", data);
    data.execute()
}

// fn part2(input: &str) -> usize {
//     let data = Data::parse(input);
//     println!("{:?}", data);
//     data.execute()
// }

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE1: &str = "D2FE28";
    static EXAMPLE2: &str = "38006F45291200";
    static EXAMPLE3: &str = "EE00D40C823060";

    // #[test]
    // fn test_decoder() {
    //     let packet = Packet::decode("D2FE28");
    //     assert_eq!(0, 0);
    // }

    // static EXAMPLE1: &str = indoc! {"
    //     foo
    // "};

    // #[test]
    // fn test_part1_example1() {
    //     let result = part1(EXAMPLE1);
    //     assert_eq!(result, 0);
    // }

    // #[test]
    // fn test_part1_solution() {
    //     let result = part1(&read_input_file());
    //     assert_eq!(result, 0);
    // }

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
