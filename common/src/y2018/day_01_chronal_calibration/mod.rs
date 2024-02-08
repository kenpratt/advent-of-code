use std::collections::HashSet;

use crate::interface::AoC;

pub struct Day;
impl AoC<Vec<i32>, i32, i32> for Day {
    const FILE: &'static str = file!();

    fn parse(input: String) -> Vec<i32> {
        input
            .lines()
            .map(|line| line.parse::<i32>().unwrap())
            .collect()
    }

    fn part1(changes: &Vec<i32>) -> i32 {
        changes.into_iter().sum()
    }

    fn part2(changes: &Vec<i32>) -> i32 {
        let mut curr = 0;
        let mut seen = HashSet::from([curr]);
        for c in changes.iter().cycle() {
            let next = curr + c;
            if seen.contains(&next) {
                return next;
            } else {
                seen.insert(next);
                curr = next;
            }
        }
        panic!("Unreachable");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_examples() {
        assert_eq!(Day::part1(&Day::parse_str("+1\n-2\n+3\n+1")), 3);
        assert_eq!(Day::part1(&Day::parse_str("+1\n+1\n+1")), 3);
        assert_eq!(Day::part1(&Day::parse_str("+1\n+1\n-2")), 0);
        assert_eq!(Day::part1(&Day::parse_str("-1\n-2\n-3")), -6);
    }

    #[test]
    fn test_part1_solution() {
        let result = Day::part1(&Day::parse_input_file());
        assert_eq!(result, 472);
    }

    #[test]
    fn test_part2_examples() {
        assert_eq!(Day::part2(&Day::parse_str("+1\n-2\n+3\n+1")), 2);
        assert_eq!(Day::part2(&Day::parse_str("+1\n-1")), 0);
        assert_eq!(Day::part2(&Day::parse_str("+3\n+3\n+4\n-2\n-4")), 10);
        assert_eq!(Day::part2(&Day::parse_str("-6\n+3\n+8\n+5\n-6")), 5);
        assert_eq!(Day::part2(&Day::parse_str("+7\n+7\n-2\n-7\n-4")), 14);
    }

    #[test]
    fn test_part2_solution() {
        let result = Day::part2(&Day::parse_input_file());
        assert_eq!(result, 66932);
    }
}
