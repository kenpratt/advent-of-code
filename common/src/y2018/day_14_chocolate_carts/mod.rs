use std::collections::BTreeMap;

use crate::interface::AoC;

const INITIAL_SCORES: [u8; 2] = [3, 7];

pub struct Day;
impl AoC<String, String, usize> for Day {
    const FILE: &'static str = file!();

    fn parse(input: String) -> String {
        input
    }

    fn part1(input: &String) -> String {
        let target = input.parse::<usize>().unwrap();

        // run until size >= target + 10
        let want_size = target + 10;
        let (_, _, scores) = run_n_recipes(want_size);

        // return string of the 10 digits starting with target index
        let r = target as usize;
        scores[r..(r + 10)].iter().map(|d| d.to_string()).collect()
    }

    fn part2(input: &String) -> usize {
        run();
        panic!("fin");
    }
}

fn run_n_recipes(n: usize) -> (usize, usize, Vec<u8>) {
    let mut scores = INITIAL_SCORES.to_vec();

    let mut i = 0;
    let mut j = 1;
    while scores.len() < n {
        let si = scores[i];
        let sj = scores[j];
        let s = si + sj;

        let a = s / 10;
        if a > 0 {
            scores.push(a);
        }

        let b = s % 10;
        scores.push(b);

        i = (i + si as usize + 1) % scores.len();
        j = (j + sj as usize + 1) % scores.len();
    }

    (i, j, scores)
}

#[derive(Debug)]
enum SmartIndex {
    Prelude(usize, usize),
    Common(usize),
}

impl SmartIndex {
    fn score(&self, preludes: &Vec<Vec<u8>>, common: &Vec<u8>) -> u8 {
        use SmartIndex::*;

        match self {
            Prelude(p, i) => preludes[*p][*i],
            Common(i) => common[*i],
        }
    }

    fn advance(&self, preludes: &Vec<Vec<u8>>, common: &mut Vec<u8>) -> Self {
        use SmartIndex::*;

        match self {
            Prelude(p, i) => {
                if *i < preludes[*p].len() {
                    Prelude(*p, i + 1)
                } else {
                    Common(0)
                }
            }
            Common(i) => {
                if *i < common.len() {
                    Common(i + 1)
                } else {
                    panic!("Try to grab elements from pending scores")
                }
            }
        }
    }
}

#[derive(Debug)]
struct PendingScores {
    index: usize,
    scores: Vec<u8>,
}

impl PendingScores {}

fn run() {
    let (i, j, scores) = run_n_recipes(100);

    // TODO - I got stuck trying to figure out how to initialize the  indices and pending scores
    // it's a bit chicken & egg. this SmartIndex stuff isn't really set up to handle the first ~20 elements while it gets past the initial stuff
    // but on the other hand, so it'd be easier to initialize at iteration 100, but I'm not sure how to figure out where to put the smart indices??
    // and should pending scores be like, the remainder of scores starting at a max smart index *index value*, not score?

    let (preludes, mut common) = find_pattern(&scores);

    let mut i = SmartIndex::Prelude(0, 0);
    let mut j = SmartIndex::Prelude(1, 0);

    loop {
        let score = i.score(&preludes, &common) + j.score(&preludes, &common);

        // pending_scores.add(score)

        i = i.advance(&preludes, &mut common);
        j = j.advance(&preludes, &mut common);
    }
}

fn find_pattern(scores: &[u8]) -> (Vec<Vec<u8>>, Vec<u8>) {
    // an index will advance by the current score at that index, plus 1
    // hence, the most an index can advance, is if it's at the last element,
    // the score is 9:
    // - with i = N - 1
    // - ((N - 1) + 9 + 1) % N
    // - (N + 9) % N
    // - 9
    // so start index needs to be between 0-9, inclusive
    let start_indices = 0..=9;

    // find the first index they have in common
    let common_index = first_index_in_common(start_indices.clone().collect(), scores);

    // build a list of the preludes for each index
    let preludes: Vec<Vec<u8>> = start_indices
        .map(|start_index| {
            let mut prelude = vec![];
            let mut i = start_index;
            while i < common_index {
                let score = scores[i];
                prelude.push(score);
                i += score as usize + 1;
            }
            prelude
        })
        .collect();
    dbg!(&preludes);

    // grab the common values, too
    let mut common: Vec<u8> = vec![];
    let mut last_i = 0;
    let mut i = common_index;
    while i < scores.len() {
        let score = scores[i];
        common.push(score);
        last_i = i;
        i += score as usize + 1;
    }
    dbg!(&common);

    dbg!(&last_i);
    dbg!(&i);
    dbg!(&scores.len());

    (preludes, common)
}

fn first_index_in_common(mut indices: Vec<usize>, scores: &[u8]) -> usize {
    let mut counts: BTreeMap<usize, u8> = indices.iter().map(|i| (*i, 1)).collect();
    let n = indices.len() as u8;
    loop {
        // tick them all forward
        for i in &mut indices {
            // advance index
            let score = scores[*i] as usize;
            *i += score + 1;

            // increase count
            let e = counts.entry(*i).or_default();
            *e += 1;
            if *e == n {
                return *i;
            }
        }
    }
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
