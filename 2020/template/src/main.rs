use std::fs;

fn main() {
    println!("part 1 result: {:?}", part1(read_input_file()));
    //println!("part 2 result: {:?}", part2(read_input_file()));
}

fn read_input_file() -> String {
    return fs::read_to_string("input.txt").expect("Something went wrong reading the file");
}

fn parse_input(input: String) -> Vec<u64> {
    return input.lines().map(|line| parse_line(line)).collect();
}

fn parse_line(line: &str) -> u64 {
    return line.parse::<u64>().unwrap();
}

fn part1(input: String) -> u64 {
    let entries = parse_input(input);
    panic!("Error parsing input");
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
            "".to_string()
        );
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