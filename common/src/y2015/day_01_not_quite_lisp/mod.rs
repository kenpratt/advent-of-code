use crate::file::*;

pub fn run() {
    let input = parse(&read_input_file!());
    println!("part 1 result: {:?}", part1(&input));
    println!("part 2 result: {:?}", part2(&input));
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

fn parse(input: &str) -> Vec<Direction> {
    Direction::parse_list(input)
}

fn part1(directions: &[Direction]) -> i16 {
    directions.iter().map(|d| d.value()).sum()
}

fn part2(directions: &[Direction]) -> Option<usize> {
    directions
        .iter()
        .scan(0, |acc, d| {
            *acc += d.value();
            Some(*acc)
        })
        .position(|v| v < 0)
        .map(|i| i + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_examples() {
        assert_eq!(part1(&parse(&"(())")), 0);
        assert_eq!(part1(&parse(&"()()")), 0);
        assert_eq!(part1(&parse(&"(((")), 3);
        assert_eq!(part1(&parse(&"(()(()(")), 3);
        assert_eq!(part1(&parse(&"))(((((")), 3);
        assert_eq!(part1(&parse(&"())")), -1);
        assert_eq!(part1(&parse(&"))(")), -1);
        assert_eq!(part1(&parse(&")))")), -3);
        assert_eq!(part1(&parse(&")())())")), -3);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&parse(&read_input_file!()));
        assert_eq!(result, 232);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(&parse(&")")), Some(1));
        assert_eq!(part2(&parse(&"()())")), Some(5));
        assert_eq!(part2(&parse(&"()()")), None);
    }

    #[test]
    fn test_part2_solution() {
        let result = part2(&parse(&read_input_file!()));
        assert_eq!(result, Some(1783));
    }
}
