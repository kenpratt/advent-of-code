use std::collections::HashMap;
use std::fs;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref STARTING_POSITION_RE: Regex =
        Regex::new(r"\APlayer (\d+) starting position: (\d+)\z").unwrap();
}

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    println!("part 2 result: {:?}", part2(&read_input_file()));
}

fn read_input_file() -> String {
    fs::read_to_string("input.txt").expect("Something went wrong reading the file")
}

fn parse(input: &str) -> Vec<(u8, u8)> {
    input.lines().map(|l| parse_line(l)).collect()
}

fn parse_line(line: &str) -> (u8, u8) {
    let captures = STARTING_POSITION_RE.captures(line).unwrap();
    let player_number = captures.get(1).unwrap().as_str().parse::<u8>().unwrap();
    let position = captures.get(2).unwrap().as_str().parse::<u8>().unwrap();
    (player_number, position)
}

#[derive(Debug)]
struct DeterministicDie {
    curr: u8,
}

impl DeterministicDie {
    fn new() -> DeterministicDie {
        DeterministicDie { curr: 1 }
    }
}

impl Iterator for DeterministicDie {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let res = self.curr;
        if self.curr == 100 {
            self.curr = 1;
        } else {
            self.curr += 1;
        }
        Some(res)
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Player {
    number: u8,
    position: u8,
    score: usize,
}

impl Player {
    fn from_starting_positions(starting_positions: &[(u8, u8)]) -> Vec<Player> {
        starting_positions
            .iter()
            .map(|(number, pos)| Player::new(*number, *pos))
            .collect()
    }

    fn new(number: u8, starting_position: u8) -> Player {
        let score = 0;

        // for simplicity, store position in 0-9 instead of 1-10
        let position = starting_position - 1;

        Player {
            number,
            position,
            score,
        }
    }

    fn advance(&mut self, spaces: u8) -> usize {
        self.position = (self.position + spaces) % 10;
        self.score += (self.position + 1) as usize;
        self.score
    }
}

#[derive(Debug)]
struct DeterministicGame {
    die: DeterministicDie,
    rolls: usize,
    players: Vec<Player>,
    over: bool,
}

impl DeterministicGame {
    fn new(starting_positions: &[(u8, u8)]) -> DeterministicGame {
        let die = DeterministicDie::new();
        let players = Player::from_starting_positions(starting_positions);
        let rolls = 0;
        let over = false;
        DeterministicGame {
            die,
            players,
            rolls,
            over,
        }
    }

    fn play(&mut self) {
        while !self.over {
            self.tick();
        }
    }

    fn tick(&mut self) {
        for idx in 0..self.players.len() {
            let to_advance = self.roll_three();
            let score = self.players[idx].advance(to_advance);
            if score >= 1000 {
                self.over = true;
                break;
            }
        }
    }

    fn roll(&mut self) -> u8 {
        self.rolls += 1;
        self.die.next().unwrap() % 10 // keep value smaller so we don't overflow u8
    }

    fn roll_three(&mut self) -> u8 {
        self.roll() + self.roll() + self.roll()
    }
}

fn part1(input: &str) -> usize {
    let starting_positions = parse(input);
    println!("{:?}", starting_positions);
    let mut game = DeterministicGame::new(&starting_positions);
    game.play();
    println!("final game state: {:?}", game);
    let losing_score = game.players.iter().map(|p| p.score).min().unwrap();
    let rolls = game.rolls;
    losing_score * rolls
}

// (sum-of-3-rolls, frequency)
static ROLL_OUTCOMES: [(u8, u8); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

#[derive(Debug, Hash, Eq, PartialEq)]
struct QuantumGameState {
    players: Vec<Player>,
    winner: Option<u8>,
}

impl QuantumGameState {
    fn new(starting_positions: &[(u8, u8)]) -> QuantumGameState {
        let players = Player::from_starting_positions(starting_positions);
        let winner = None;
        QuantumGameState { players, winner }
    }

    fn tick_player(&self, player_index: usize) -> Vec<(QuantumGameState, u8)> {
        ROLL_OUTCOMES
            .iter()
            .cloned()
            .map(|(roll, count)| (self.apply_roll(roll, player_index), count))
            .collect()
    }

    fn apply_roll(&self, roll: u8, player_index: usize) -> QuantumGameState {
        let mut new_players = self.players.clone();
        let score = new_players[player_index].advance(roll);
        let winner = if score >= 21 {
            Some(new_players[player_index].number)
        } else {
            None
        };
        QuantumGameState {
            players: new_players,
            winner: winner,
        }
    }
}

#[derive(Debug)]
struct QuantumGame {
    num_players: usize,
    games_in_progress: Option<HashMap<QuantumGameState, usize>>,
    winner_tally: HashMap<u8, usize>,
}

impl QuantumGame {
    fn new(starting_positions: &[(u8, u8)]) -> QuantumGame {
        let num_players = starting_positions.len();
        let initial_state = QuantumGameState::new(starting_positions);
        let mut games_in_progress = HashMap::new();
        games_in_progress.insert(initial_state, 1);
        let winner_tally = HashMap::new();
        QuantumGame {
            num_players,
            games_in_progress: Some(games_in_progress),
            winner_tally,
        }
    }

    fn play(&mut self) {
        while self.games_in_progress.is_some() {
            self.tick();
        }
    }

    fn tick(&mut self) {
        for i in 0..self.num_players {
            self.tick_player(i);
        }
    }

    fn tick_player(&mut self, player_index: usize) {
        if self.games_in_progress.is_none() {
            return;
        }

        let mut new_games_in_progress = HashMap::new();
        for (last_game, last_count) in self.games_in_progress.take().unwrap() {
            let resulting_games = last_game.tick_player(player_index);
            for (resulting_game, count) in resulting_games {
                let new_count = last_count * count as usize;
                match resulting_game.winner {
                    Some(winner) => {
                        let counter = self.winner_tally.entry(winner).or_insert(0);
                        *counter += new_count;
                    }
                    None => {
                        // no winner yet, add this game state to next games in progress state
                        let counter = new_games_in_progress.entry(resulting_game).or_insert(0);
                        *counter += new_count;
                    }
                };
            }
        }
        self.games_in_progress = if new_games_in_progress.is_empty() {
            None
        } else {
            Some(new_games_in_progress)
        }
    }
}

fn part2(input: &str) -> usize {
    let starting_positions = parse(input);
    println!("{:?}", starting_positions);
    let mut game = QuantumGame::new(&starting_positions);
    game.play();
    println!("final game state: {:?}", game);
    *game.winner_tally.values().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    static EXAMPLE1: &str = indoc! {"
        Player 1 starting position: 4
        Player 2 starting position: 8
    "};

    #[test]
    fn test_part1_example1() {
        let result = part1(EXAMPLE1);
        assert_eq!(result, 739785);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file());
        assert_eq!(result, 571032);
    }

    #[test]
    fn test_part2_example1() {
        let result = part2(EXAMPLE1);
        assert_eq!(result, 444356092776315);
    }

    #[test]
    fn test_part2_solution() {
        let result = part2(&read_input_file());
        assert_eq!(result, 49975322685009);
    }
}
