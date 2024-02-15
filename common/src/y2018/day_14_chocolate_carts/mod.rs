use crate::interface::AoC;

pub struct Day;
impl AoC<String, String, usize> for Day {
    const FILE: &'static str = file!();

    fn parse(input: String) -> String {
        input
    }

    fn part1(input: &String) -> String {
        let target = input.parse::<usize>().unwrap();

        // run until size >= target + 10
        let until_size = target + 10;
        let scores = run_recipes(&[3, 7], |s| s.len() >= until_size);

        // return string of the 10 digits starting with target index
        let r = target as usize;
        scores[r..(r + 10)].iter().map(|d| d.to_string()).collect()
    }

    fn part2(input: &String) -> usize {
        let target: Vec<u8> = input
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();
        let target_len = target.len();

        // run until last chunk matches
        let last_chunk_matches =
            |s: &Vec<u8>| s.len() >= target_len && &s[(s.len() - target_len)..] == &target;
        let second_last_chunk_matches = |s: &Vec<u8>| {
            s.len() > target_len && &s[(s.len() - target_len - 1)..(s.len() - 1)] == &target
        };

        let scores = run_recipes(&[3, 7], |s| {
            last_chunk_matches(s) || second_last_chunk_matches(s)
        });

        // return length
        if last_chunk_matches(&scores) {
            scores.len() - target_len
        } else if second_last_chunk_matches(&scores) {
            scores.len() - target_len - 1
        } else {
            panic!("Unreachable");
        }
    }
}

fn run_recipes<F>(initial: &[u8; 2], stop: F) -> Vec<u8>
where
    F: Fn(&Vec<u8>) -> bool,
{
    let mut scores = initial.to_vec();

    let mut i = 0;
    let mut j = 1;
    while !stop(&scores) {
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
        assert_eq!(Day::part1(&"9".to_owned()), "5158916779");
        assert_eq!(Day::part1(&"5".to_owned()), "0124515891");
        assert_eq!(Day::part1(&"18".to_owned()), "9251071085");
        assert_eq!(Day::part1(&"2018".to_owned()), "5941429882");
    }

    #[test]
    fn test_part1_solution() {
        let result = Day::part1(&Day::parse_input_file());
        assert_eq!(result, "4910101614");
    }

    #[test]
    fn test_part2_examples() {
        assert_eq!(Day::part2(&"51589".to_owned()), 9);
        assert_eq!(Day::part2(&"01245".to_owned()), 5);
        assert_eq!(Day::part2(&"92510".to_owned()), 18);
        assert_eq!(Day::part2(&"59414".to_owned()), 2018);
    }

    #[test]
    fn test_part2_solution() {
        let result = Day::part2(&Day::parse_input_file());
        assert_eq!(result, 20253137);
    }
}
