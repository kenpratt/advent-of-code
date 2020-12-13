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
    adapters: Vec<usize>,
}

impl Data {
    fn parse(input: &str) -> Data {
        let mut adapters: Vec<usize> = input.lines().map(|line| line.parse::<usize>().unwrap()).collect();

        adapters.sort();
        adapters.push(adapters.last().unwrap() + 3);
        println!("{:?}", adapters);

        return Data {
            adapters: adapters,
        }
    }

    fn differences(&self) -> Vec<usize> {
        let mut differences: Vec<usize> = vec![0, 0, 0, 0];

        for i in 0..self.adapters.len() {
            let left = if i == 0 {0} else {self.adapters[i - 1]};
            let right = self.adapters[i];
            let diff = right - left;
            differences[diff] += 1;
        }

        println!("{:?}", differences);
        return differences;
    }
}

fn part1(input: String) -> usize {
    let data = Data::parse(&input);
    let differences = data.differences();
    return differences[1] * differences[3];
}

// fn part2(input: String) -> usize {
//     let data = Data::parse(&input);
//     return data.execute();
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example1() {
        let input = indoc! {"
            16
            10
            15
            5
            1
            11
            7
            19
            6
            12
            4
        "};
        let result = part1(input.to_string());
        assert_eq!(result, 35);
    }

    #[test]
    fn test_part1_example2() {
        let input = indoc! {"
            28
            33
            18
            42
            31
            14
            46
            20
            48
            47
            24
            23
            49
            45
            19
            38
            39
            11
            1
            32
            25
            35
            8
            17
            7
            9
            4
            2
            34
            10
            3
        "};
        let result = part1(input.to_string());
        assert_eq!(result, 220);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(
            read_input_file()
        );
        assert_eq!(result, 1917);
    }

    // #[test]
    // fn test_part2_example1() {
    //     let input = indoc! {"
    //         input
    //     "};
    //     let result = part1(input.to_string());
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