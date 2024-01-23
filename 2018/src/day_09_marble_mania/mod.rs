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
    num_players: usize,
    last_marble: usize,
}

impl Parameters {
    fn parse(input: &str) -> Self {
        lazy_static! {
            static ref ITEM_RE: Regex =
                Regex::new(r"\A(\d+) players; last marble is worth (\d+) points\z").unwrap();
        }

        let caps = ITEM_RE.captures(input).unwrap();
        let num_players = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let last_marble = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
        Self {
            num_players,
            last_marble,
        }
    }
}

#[derive(Debug)]
struct Game {
    neighbours: Vec<(usize, usize)>,
    current_marble: usize,
    scores: Vec<usize>,
    existing: [usize; 23], // used as temp storage for marble insertion calculation
}

impl Game {
    fn new(parameters: &Parameters) -> Self {
        Self {
            current_marble: 0,
            neighbours: vec![(0, 0); parameters.last_marble + 1],
            scores: vec![0; parameters.num_players],
            existing: [0; 23],
        }
    }

    fn play(parameters: &Parameters) -> usize {
        let mut game = Self::new(parameters);
        let last_scoring_round = parameters.last_marble / 23;

        // place first round
        game.place_first_round();

        // now batch process each remaining round
        for round in 2..=last_scoring_round {
            let marble = round * 23;
            let player_id = (marble - 1) % parameters.num_players;
            game.place_subsequent_round(round, marble, player_id);
        }

        game.high_score()
    }

    fn high_score(&self) -> usize {
        *self.scores.iter().max().unwrap()
    }

    fn place_first_round(&mut self) {
        // for the first round, we can't use the logic where we pre-calculate the
        // existing marbles, because there aren't enough marbles yet - placing the
        // new marbles will be neighbours to each other

        // place 22 marbles
        for i in 0..22 {
            let marble_id = i + 1;

            // find the marbles to insert between
            let left = self.neighbours[self.current_marble].1;
            let right = self.neighbours[left].1;

            // insert marble between left and right
            self.neighbours[left].1 = marble_id;
            self.neighbours[right].0 = marble_id;
            self.neighbours[marble_id] = (left, right);

            self.current_marble = marble_id;
        }

        // now delete one

        // find the target, the 7th marble counter-clockwise, and the marbles to the left/right of it
        let right = (0..6).fold(self.current_marble, |last, _| self.neighbours[last].0);
        let target = self.neighbours[right].0;
        let left = self.neighbours[target].0;

        // remove the target
        self.neighbours[left].1 = right;
        self.neighbours[right].0 = left;

        self.current_marble = right;
        self.scores[0] += 23 + target;
    }

    fn place_subsequent_round(&mut self, round: usize, marble_id: usize, player_id: usize) {
        self.update_existing();

        // place the 22 new marbles, and remove existing[18]
        let first_marble = (round - 1) * 23 + 1;
        for i in 0..22 {
            // special case around 17-18:
            // existing[18] is being removed, so the new 17 & 18 will point at each other
            let prev = if i == 18 {
                first_marble + 17
            } else {
                self.existing[i]
            };
            let next = if i == 17 {
                first_marble + 18
            } else {
                self.existing[i + 1]
            };

            let marble = first_marble + i;

            // insert the marble between prev & next
            self.neighbours[next].0 = marble;
            self.neighbours[prev].1 = marble;
            self.neighbours[marble] = (prev, next);
        }

        self.current_marble = first_marble + 18;
        self.scores[player_id] += marble_id + self.existing[18];
    }

    fn update_existing(&mut self) {
        // we're going to need the next 23 marbles
        let mut last = self.current_marble;
        for i in 0..23 {
            let next = self.neighbours[last].1;
            self.existing[i] = next;
            last = next;
        }
    }
}

fn parse(input: &str) -> Parameters {
    Parameters::parse(input)
}

fn part1(parameters: &Parameters) -> usize {
    Game::play(parameters)
}

fn part2(parameters: &Parameters) -> usize {
    let modified_parameters = Parameters {
        num_players: parameters.num_players,
        last_marble: parameters.last_marble * 100,
    };
    Game::play(&modified_parameters)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_examples(examples: &str) -> Vec<(Parameters, usize)> {
        examples.lines().map(|line| parse_example(line)).collect()
    }

    fn parse_example(example: &str) -> (Parameters, usize) {
        lazy_static! {
            static ref EXAMPLE_RE: Regex = Regex::new(r"\A(.*): high score is (\d+)\z").unwrap();
        }

        let caps = EXAMPLE_RE.captures(example).unwrap();
        let input = parse(caps.get(1).unwrap().as_str());
        let result = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
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
