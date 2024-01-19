use crate::file::*;
use std::collections::HashMap;

pub fn run() {
    let input = parse(&read_input_file!());
    println!("part 1 result: {:?}", part1(&input));
    println!("part 2 result: {:?}", part2(&input));
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

fn parse(input: &str) -> Vec<Box> {
    Box::parse_list(input)
}

fn part1(boxes: &[Box]) -> usize {
    let freqs: Vec<HashMap<&char, usize>> = boxes.iter().map(|b| b.char_frequencies()).collect();
    let twos = freqs.iter().filter(|f| f.values().any(|n| *n == 2)).count();
    let threes = freqs.iter().filter(|f| f.values().any(|n| *n == 3)).count();
    twos * threes
}

fn part2(boxes: &[Box]) -> String {
    // find two boxes that only differ by one character
    // in order to do so more efficiently than O(N*N),
    // let's build a map of variants of each box ID, by
    // removing one char at a time.
    let mut variants: HashMap<(&[char], &[char]), usize> = HashMap::new();
    for b in boxes {
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
        let result = part1(&parse(&read_file!(EXAMPLE_FILE_1)));
        assert_eq!(result, 12);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&parse(&read_input_file!()));
        assert_eq!(result, 4920);
    }

    #[test]
    fn test_part2_example() {
        let result = part2(&parse(&read_file!(EXAMPLE_FILE_2)));
        assert_eq!(result, "fgij");
    }

    #[test]
    fn test_part2_solution() {
        let result = part2(&parse(&read_input_file!()));
        assert_eq!(result, "fonbwmjquwtapeyzikghtvdxl");
    }
}
