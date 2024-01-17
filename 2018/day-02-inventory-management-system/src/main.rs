use std::{collections::HashMap, fs};

const INPUT_FILE: &'static str = "input.txt";

fn main() {
    println!("part 1 result: {:?}", part1(&read_file(INPUT_FILE)));
    println!("part 2 result: {:?}", part2(&read_file(INPUT_FILE)));
}

fn read_file(filename: &str) -> String {
    fs::read_to_string(filename).expect("Something went wrong reading the file")
}

#[derive(Debug)]
struct Box {
    id: Vec<char>,
}

impl Box {
    fn parse_list(input: &str) -> Vec<Self> {
        input.lines().map(|line| Self::parse(line)).collect()
    }

    fn parse(input: &str) -> Self {
        let id = input.chars().collect();
        Self { id }
    }

    fn char_frequencies(&self) -> HashMap<&char, usize> {
        let mut freq = HashMap::new();
        for c in &self.id {
            *freq.entry(c).or_default() += 1;
        }
        freq
    }
}

fn part1(input: &str) -> usize {
    let boxes = Box::parse_list(input);
    let freqs: Vec<HashMap<&char, usize>> = boxes.iter().map(|b| b.char_frequencies()).collect();
    let twos = freqs.iter().filter(|f| f.values().any(|n| *n == 2)).count();
    let threes = freqs.iter().filter(|f| f.values().any(|n| *n == 3)).count();
    twos * threes
}

fn part2(input: &str) -> String {
    let boxes = Box::parse_list(input);

    // find two boxes that only differ by one character
    // in order to do so more efficiently than O(N*N),
    // let's build a map of variants of each box ID, by
    // removing one char at a time.
    let mut variants: HashMap<(&[char], &[char]), usize> = HashMap::new();
    for b in &boxes {
        for i in 0..b.id.len() {
            let l = &b.id[0..i];
            let r = &b.id[i + 1..b.id.len()];
            *variants.entry((l, r)).or_default() += 1;
        }
    }

    // find the collision in the map -- there should only be one!
    let collisions: Vec<((&[char], &[char]), usize)> =
        variants.into_iter().filter(|(_k, v)| *v > 1).collect();
    assert_eq!(collisions.len(), 1);

    let (key, count) = collisions[0];
    assert_eq!(count, 2);

    // return chars in common
    let (left, right) = key;
    let mut left_str: String = left.iter().collect();
    let right_str: String = right.iter().collect();
    left_str.push_str(&right_str);
    left_str
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_FILE_1: &'static str = "example1.txt";
    const EXAMPLE_FILE_2: &'static str = "example2.txt";

    #[test]
    fn test_part1_example() {
        let result = part1(&read_file(EXAMPLE_FILE_1));
        assert_eq!(result, 12);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_file(INPUT_FILE));
        assert_eq!(result, 4920);
    }

    #[test]
    fn test_part2_example() {
        let result = part2(&read_file(EXAMPLE_FILE_2));
        assert_eq!(result, "fgij");
    }

    #[test]
    fn test_part2_solution() {
        let result = part2(&read_file(INPUT_FILE));
        assert_eq!(result, "fonbwmjquwtapeyzikghtvdxl");
    }
}
