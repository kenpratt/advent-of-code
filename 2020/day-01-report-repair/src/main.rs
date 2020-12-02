use std::fs;

fn main() {
    part1();
    part2();
}

fn part1() {
    let input = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let result = run_part1(input);
    println!("part 1 result: {:?}", result);
}

fn parse_input(input: String) -> Vec<u64> {
    return input.lines().map(|line| parse_line(line)).collect();
}

fn parse_line(line: &str) -> u64 {
    return line.parse::<u64>().unwrap();
}

fn run_part1(input: String) -> u64 {
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

fn part2() {
    let input = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let result = run_part2(input);
    println!("part 2 result: {:?}", result);    
}

fn run_part2(input: String) -> u64 {
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
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_part1_example1() {
        let result = run_part1(
            "1721\n979\n366\n299\n675\n1456".to_string()
        );
        assert_eq!(result, 514579);
    }

    #[test]
    fn test_part2_example1() {
        let result = run_part2(
            "1721\n979\n366\n299\n675\n1456".to_string()
        );
        assert_eq!(result, 241861950);
    }    
}