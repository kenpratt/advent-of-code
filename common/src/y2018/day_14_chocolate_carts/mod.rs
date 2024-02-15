use crate::interface::AoC;

pub struct Day;
impl AoC<usize, String, usize> for Day {
    const FILE: &'static str = file!();

    fn parse(input: String) -> usize {
        input.parse::<usize>().unwrap()
    }

    fn part1(target: &usize) -> String {
        // run until size >= target + 10
        let scores = run_recipes(target + 10, &[3, 7]);

        // return string of the 10 digits starting with target index
        let r = *target as usize;
        scores[r..(r + 10)].iter().map(|d| d.to_string()).collect()
    }

    fn part2(_rounds: &usize) -> usize {
        0
    }
}

fn run_recipes(target_len: usize, initial: &[u8; 2]) -> Vec<u8> {
    let mut scores = initial.to_vec();

    let mut i = 0;
    let mut j = 1;
    while scores.len() < target_len {
        let si = scores[i];
        let sj = scores[j];
        let s = si + sj;

        if s >= 10 {
            scores.push(s / 10);
            scores.push(s % 10);
        } else {
            scores.push(s);
        }

        i = (i + si as usize + 1) % scores.len();
        j = (j + sj as usize + 1) % scores.len();
    }

    scores
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_examples() {
        assert_eq!(Day::part1(&9), "5158916779");
        assert_eq!(Day::part1(&5), "0124515891");
        assert_eq!(Day::part1(&18), "9251071085");
        assert_eq!(Day::part1(&2018), "5941429882");
    }

    #[test]
    fn test_part1_solution() {
        let result = Day::part1(&Day::parse_input_file());
        assert_eq!(result, "4910101614");
    }

    #[test]
    fn test_part2_example() {
        let result = Day::part2(&Day::parse_example_file());
        assert_eq!(result, 0);
    }

    #[test]
    fn test_part2_solution() {
        let result = Day::part2(&Day::parse_input_file());
        assert_eq!(result, 0);
    }
}
