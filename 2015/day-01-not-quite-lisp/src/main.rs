use std::fs;

const INPUT_FILE: &'static str = "input.txt";

fn main() {
    println!("part 1 result: {:?}", part1(&read_file(INPUT_FILE)));
    // println!("part 2 result: {:?}", part2(&read_file(INPUT_FILE)));
}

fn read_file(filename: &str) -> String {
    fs::read_to_string(filename).expect("Something went wrong reading the file")
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
}

impl Direction {
    fn parse_list(input: &str) -> Vec<Self> {
        input.chars().map(|c| Self::from_char(&c)).collect()
    }

    fn from_char(c: &char) -> Self {
        use Direction::*;

        match c {
            '(' => Up,
            ')' => Down,
            _ => panic!("Unrecognized input char: {:?}", c),
        }
    }

    fn value(&self) -> i16 {
        use Direction::*;

        match self {
            Up => 1,
            Down => -1,
        }
    }
}

fn part1(input: &str) -> i16 {
    let directions = Direction::parse_list(input);
    directions.iter().map(|d| d.value()).sum()
}

// fn part2(input: &str) -> usize {
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_examples() {
        assert_eq!(part1(&"(())"), 0);
        assert_eq!(part1(&"()()"), 0);
        assert_eq!(part1(&"((("), 3);
        assert_eq!(part1(&"(()(()("), 3);
        assert_eq!(part1(&"))((((("), 3);
        assert_eq!(part1(&"())"), -1);
        assert_eq!(part1(&"))("), -1);
        assert_eq!(part1(&")))"), -3);
        assert_eq!(part1(&")())())"), -3);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_file(INPUT_FILE));
        assert_eq!(result, 232);
    }

    // #[test]
    // fn test_part2_example() {
    //     let result = part2(&read_file(EXAMPLE_FILE));
    //     assert_eq!(result, 0);
    // }

    // #[test]
    // fn test_part2_solution() {
    //     let result = part2(&read_file(INPUT_FILE));
    //     assert_eq!(result, 0);
    // }
}
