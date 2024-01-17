use std::{collections::HashSet, fs};

const INPUT_FILE: &'static str = "input.txt";

fn main() {
    println!("part 1 result: {:?}", part1(&read_file(INPUT_FILE)));
    println!("part 2 result: {:?}", part2(&read_file(INPUT_FILE)));
}

fn read_file(filename: &str) -> String {
    fs::read_to_string(filename).expect("Something went wrong reading the file")
}

#[derive(Debug)]
struct Change(i32);

fn parse_list(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect()
}

fn part1(input: &str) -> i32 {
    let changes = parse_list(input);
    changes.into_iter().sum()
}

fn part2(input: &str) -> i32 {
    let changes = parse_list(input);

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_examples() {
        assert_eq!(part1(&"+1\n-2\n+3\n+1"), 3);
        assert_eq!(part1(&"+1\n+1\n+1"), 3);
        assert_eq!(part1(&"+1\n+1\n-2"), 0);
        assert_eq!(part1(&"-1\n-2\n-3"), -6);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_file(INPUT_FILE));
        assert_eq!(result, 472);
    }

    #[test]
    fn test_part2_examples() {
        assert_eq!(part2(&"+1\n-2\n+3\n+1"), 2);
        assert_eq!(part2(&"+1\n-1"), 0);
        assert_eq!(part2(&"+3\n+3\n+4\n-2\n-4"), 10);
        assert_eq!(part2(&"-6\n+3\n+8\n+5\n-6"), 5);
        assert_eq!(part2(&"+7\n+7\n-2\n-7\n-4"), 14);
    }

    #[test]
    fn test_part2_solution() {
        let result = part2(&read_file(INPUT_FILE));
        assert_eq!(result, 66932);
    }
}
