use std::fs;

fn main() {
    println!("part 1 result: {:?}", part1(read_input_file()));
    println!("part 2 result: {:?}", part2(read_input_file()));
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
    for x in &entries {
        for y in &entries {
            if x + y == 2020 {
                return x * y;
            }
        }
    }
    panic!("Nothing added up to 2020");
}

fn part2(input: String) -> u64 {
    let entries = parse_input(input);
    for x in &entries {
        for y in &entries {
            for z in &entries {
                if x + y + z == 2020 {
                    return x * y * z;
                }
            }
        }
    }
    panic!("Nothing added up to 2020");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example1() {
        let result = part1(
            "1721\n979\n366\n299\n675\n1456".to_string()
        );
        assert_eq!(result, 514579);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(
            read_input_file()
        );
        assert_eq!(result, 751776);
    }

    #[test]
    fn test_part2_example1() {
        let result = part2(
            "1721\n979\n366\n299\n675\n1456".to_string()
        );
        assert_eq!(result, 241861950);
    }

    #[test]
    fn test_part2_solution() {
        let result = part2(
            read_input_file()
        );
        assert_eq!(result, 42275090);
    }    
}