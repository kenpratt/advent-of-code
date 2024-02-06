use crate::file::*;

use lazy_static::lazy_static;
use regex::Regex;

pub fn run() {
    let input = parse(&read_input_file!());
    println!("part 1 result: {:?}", part1(&input));
    println!("part 2 result: {:?}", part2(&input));
}

#[derive(Debug)]
struct Parameters {
    num_players: u32,
    last_marble: u32,
}

impl Parameters {
    fn parse(input: &str) -> Self {
        lazy_static! {
            static ref ITEM_RE: Regex =
                Regex::new(r"\A(\d+) players; last marble is worth (\d+) points\z").unwrap();
        }

        let caps = ITEM_RE.captures(input).unwrap();
        let num_players = caps.get(1).unwrap().as_str().parse::<u32>().unwrap();
        let last_marble = caps.get(2).unwrap().as_str().parse::<u32>().unwrap();
        Self {
            num_players,
            last_marble,
        }
    }
}

// my Game implementation uses a fixed vec for the marbles with the full final tally of marbles
// with pointers to the currently used window within the vec, that shifts to the right and grows
// over time
#[derive(Debug)]
struct Game {
    num_rounds: u32,
    num_marbles: usize,
    marbles: Vec<u32>,
    start_index: usize,
    end_index: usize,
    scores: Vec<u32>,
}

impl Game {
    // game state after round 1, with cursor at index 0
    const AFTER_ONE_ROUND: [u32; 22] = [
        19, 2, 20, 10, 21, 5, 22, 11, 1, 12, 6, 13, 3, 14, 7, 15, 0, 16, 8, 17, 4, 18,
    ];

    fn play(parameters: &Parameters) -> u32 {
        let mut game = Self::after_one_round(parameters);

        // now process each remaining round
        for round in 2..=game.num_rounds {
            let player_id = ((round - 1) % parameters.num_players) as usize;
            game.place_subsequent_round(round, player_id);
        }

        game.high_score()
    }

    // hard-code the first round, to simplify things
    fn after_one_round(parameters: &Parameters) -> Self {
        let num_rounds = parameters.last_marble / 23;

        // we start with one marble, add 21 per round
        let num_marbles = (num_rounds * 21 + 1) as usize;

        let mut marbles = vec![0; num_marbles];
        for (i, marble) in Self::AFTER_ONE_ROUND.iter().enumerate() {
            marbles[i] = *marble;
        }
        let start_index = 0;
        let end_index = Self::AFTER_ONE_ROUND.len();

        let mut scores = vec![0; parameters.num_players as usize];
        scores[0] = 32; // 23 plus first round removal, 9

        Self {
            num_rounds,
            num_marbles,
            marbles,
            start_index,
            end_index,
            scores,
        }
    }

    fn high_score(&self) -> u32 {
        *self.scores.iter().max().unwrap()
    }

    #[inline]
    fn from_start(&self, offset: usize) -> usize {
        self.from(self.start_index, offset)
    }

    #[inline]
    fn from_end(&self, offset: usize) -> usize {
        self.from(self.end_index, offset)
    }

    #[inline]
    fn before_start(&self, offset: usize) -> usize {
        self.before(self.start_index, offset)
    }

    #[inline]
    fn from(&self, index: usize, offset: usize) -> usize {
        let res = index + offset;
        if res >= self.num_marbles {
            res - self.num_marbles
        } else {
            res
        }
    }

    #[inline]
    fn before(&self, index: usize, offset: usize) -> usize {
        if index >= offset {
            index - offset
        } else {
            index + self.num_marbles - offset
        }
    }

    fn place_subsequent_round(&mut self, round: u32, player_id: usize) {
        // this round will look like this (with o = existing element, n = new element added this round)
        //
        //  o0, o1, n0, o2, n1, o3, ... o17, n16, o18, n17 | n18, o20, n19, o21, n20, o22, n21, o23, o24, ...
        //   0,  1,  2,  3,  4,  5, ...  33,  34,  35,  36 |  37,  38,  39,  40,  41,  42,  43,  44,  45, ...
        //
        // the dividing line is where o19 is removed, and where the new cursor (beginning of the window) will be.
        // everything after the line should be prepended to the queue, and before the line appended

        let first_new_marble = (round - 1) * 23 + 1;

        // initial marble memory for round two:
        //   19, 2, 20, 10, 21, 5, 22, 11, 1, 12, 6, 13, 3, 14, 7, 15, 0, 16, 8, 17, 4, 18, -, -, -, -, -, -, -, -, -, -, -, -, -, -, -, -, -, -, -, -, -, -, -, -, -, -, -, -, -, -, -, -, -, -, -, -, -,

        // swap first marble to the back:
        //   -, 2, 20, 10, 21, 5, 22, 11, 1, 12, 6, 13, 3, 14, 7, 15, 0, 16, 8, 17, 4, 18, 19, -, -, -, -, -, -, -, -, -, -, -, -, -, -, -, -, -, -, -, -, -, -, -, -, -, -, -, -, -, -, -, -, -, -, -, -,
        self.marbles[self.end_index] = self.marbles[self.start_index];
        self.start_index = self.from_start(1);
        self.end_index = self.from_end(1);

        // place 18 marbles (24-41), interspersing 18 old ones (swapping them in)
        //   -, -, -, -, -, -, -, -, -, -, -, -, -, -, -, -, -, -, -, 17, 4, 18, 19, 2, 24, 20, 25, 10, 26, 21, 27, 5, 28, 22, 29, 11, 30, 1, 31, 12, 32, 6, 33, 13, 34, 3, 35, 14, 36, 7, 37, 15, 38, 0, 39, 16, 40, 8, 41, -, -, -,
        for n in 0..18 {
            let new_marble = first_new_marble + n;
            let i = n as usize;

            let existing_marble_prev_index = self.from_start(i);
            let existing_marble_new_index = self.from_end(i * 2);
            self.marbles[existing_marble_new_index] = self.marbles[existing_marble_prev_index];

            let new_marble_index = self.from_end(i * 2 + 1);
            self.marbles[new_marble_index] = new_marble;
        }
        self.start_index = self.from_start(18);
        self.end_index = self.from_end(36);

        // remove first marble
        //      -, -, -, -, -, -, -, -, -, -, -, -, -, -, -, -, -, -, -, -, 4, 18, 19, 2, 24, 20, 25, 10, 26, 21, 27, 5, 28, 22, 29, 11, 30, 1, 31, 12, 32, 6, 33, 13, 34, 3, 35, 14, 36, 7, 37, 15, 38, 0, 39, 16, 40, 8, 41, -, -, -,
        let removed = self.marbles[self.start_index];
        self.start_index = self.from_start(1);

        // place remaining 4 marbles (42-45), interspersing 3 old ones, at the front!
        // first need to make room by pulling the next three backwards
        //      -, -, -, -, -, -, -, -, -, -, -, -, -, -, -, -, 42, 4, 43, 18, 44, 19, 45, 2, 24, 20, 25, 10, 26, 21, 27, 5, 28, 22, 29, 11, 30, 1, 31, 12, 32, 6, 33, 13, 34, 3, 35, 14, 36, 7, 37, 15, 38, 0, 39, 16, 40, 8, 41, -, -, -,
        let new_start_index = self.before_start(4);
        for n in 0..4 {
            let new_marble = first_new_marble + n + 18;
            let i = n as usize;

            if i < 4 {
                let existing_marble_prev_index = self.from_start(i);
                let existing_marble_new_index = self.from(new_start_index, i * 2 + 1);
                self.marbles[existing_marble_new_index] = self.marbles[existing_marble_prev_index];
            }

            let new_marble_index = self.from(new_start_index, i * 2);
            self.marbles[new_marble_index] = new_marble;
        }
        self.start_index = new_start_index;

        self.scores[player_id] += round * 23 + removed;
    }
}

fn parse(input: &str) -> Parameters {
    Parameters::parse(input)
}

fn part1(parameters: &Parameters) -> u32 {
    Game::play(parameters)
}

fn part2(parameters: &Parameters) -> u32 {
    let modified_parameters = Parameters {
        num_players: parameters.num_players,
        last_marble: parameters.last_marble * 100,
    };
    Game::play(&modified_parameters)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_examples(examples: &str) -> Vec<(Parameters, u32)> {
        examples.lines().map(|line| parse_example(line)).collect()
    }

    fn parse_example(example: &str) -> (Parameters, u32) {
        lazy_static! {
            static ref EXAMPLE_RE: Regex = Regex::new(r"\A(.*): high score is (\d+)\z").unwrap();
        }

        let caps = EXAMPLE_RE.captures(example).unwrap();
        let input = parse(caps.get(1).unwrap().as_str());
        let result = caps.get(2).unwrap().as_str().parse::<u32>().unwrap();
        (input, result)
    }

    #[test]
    fn test_part1_examples() {
        let examples = parse_examples(&read_example_file!());
        for (input, result) in examples {
            assert_eq!(part1(&input), result);
        }
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&parse(&read_input_file!()));
        assert_eq!(result, 434674);
    }

    #[test]
    fn test_part2_solution() {
        let result = part2(&parse(&read_input_file!()));
        assert_eq!(result, 3653994575);
    }
}
