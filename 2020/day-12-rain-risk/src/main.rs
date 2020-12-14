use std::fs;

// use lazy_static::lazy_static;
// use regex::Regex;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    // println!("part 2 result: {:?}", part2(&read_input_file()));
}

fn read_input_file() -> String {
    return fs::read_to_string("input.txt").expect("Something went wrong reading the file");
}

#[derive(Debug)]
struct Data {
    parts: Vec<Part>,
}

impl Data {
    fn parse(input: &str) -> Data {
        let parts = input.lines().map(|line| Part::parse(line)).collect();
        return Data {
            parts: parts,
        }
    }

    fn execute(&self) -> usize {
        return 0;
    }
}

#[derive(Debug)]
struct Part {
    foo: usize,
}

impl Part {
        fn parse(input: &str) -> Part {
        return Part {
            foo: input.len(),
        }
    }
}

fn part1(input: &str) -> usize {
    let data = Data::parse(input);
    return data.execute();
}

// fn part2(input: &str) -> usize {
//     let data = Data::parse(input);
//     return data.execute();
// }

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    static EXAMPLE1: &str = indoc! {"
        foo
    "};    

    #[test]
    fn test_part1_example1() {
        let result = part1(EXAMPLE1);
        assert_eq!(result, 0);
    }

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