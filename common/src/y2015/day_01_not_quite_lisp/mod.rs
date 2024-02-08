use crate::interface::AoC;

pub struct Day;
impl AoC<Vec<Direction>, i16, Option<usize>> for Day {
    const FILE: &'static str = file!();

    fn parse(input: String) -> Vec<Direction> {
        Direction::parse_list(&input)
    }

    fn part1(directions: &Vec<Direction>) -> i16 {
        directions.iter().map(|d| d.value()).sum()
    }

    fn part2(directions: &Vec<Direction>) -> Option<usize> {
        directions
            .iter()
            .scan(0, |acc, d| {
                *acc += d.value();
                Some(*acc)
            })
            .position(|v| v < 0)
            .map(|i| i + 1)
    }
}

#[derive(Debug)]
pub enum Direction {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_examples() {
        assert_eq!(Day::part1(&Day::parse_str("(())")), 0);
        assert_eq!(Day::part1(&Day::parse_str("()()")), 0);
        assert_eq!(Day::part1(&Day::parse_str("(((")), 3);
        assert_eq!(Day::part1(&Day::parse_str("(()(()(")), 3);
        assert_eq!(Day::part1(&Day::parse_str("))(((((")), 3);
        assert_eq!(Day::part1(&Day::parse_str("())")), -1);
        assert_eq!(Day::part1(&Day::parse_str("))(")), -1);
        assert_eq!(Day::part1(&Day::parse_str(")))")), -3);
        assert_eq!(Day::part1(&Day::parse_str(")())())")), -3);
    }

    #[test]
    fn test_part1_solution() {
        let result = Day::part1(&Day::parse_input_file());
        assert_eq!(result, 232);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(Day::part2(&Day::parse_str(")")), Some(1));
        assert_eq!(Day::part2(&Day::parse_str("()())")), Some(5));
        assert_eq!(Day::part2(&Day::parse_str("()()")), None);
    }

    #[test]
    fn test_part2_solution() {
        let result = Day::part2(&Day::parse_input_file());
        assert_eq!(result, Some(1783));
    }
}
