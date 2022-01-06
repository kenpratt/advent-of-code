use std::fs;

// use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref STARTING_POSITION_RE: Regex =
        Regex::new(r"\APlayer (\d+) starting position: (\d+)\z").unwrap();
}

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    // println!("part 2 result: {:?}", part2(&read_input_file()));
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

#[derive(Debug)]
struct Player {
    number: u8,
    position: u8,
    score: usize,
}

impl Player {
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

    fn advance(&mut self, spaces: u8) -> bool {
        self.position = (self.position + spaces) % 10;
        self.score += (self.position + 1) as usize;
        let won = self.score >= 1000;
        won
    }
}

#[derive(Debug)]
struct Game {
    die: DeterministicDie,
    rolls: usize,
    players: Vec<Player>,
    over: bool,
}

impl Game {
    fn new(starting_positions: &[(u8, u8)]) -> Game {
        let die = DeterministicDie::new();
        let players = starting_positions
            .iter()
            .map(|(number, pos)| Player::new(*number, *pos))
            .collect();
        let rolls = 0;
        let over = false;
        Game {
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
            let won = self.players[idx].advance(to_advance);
            if won {
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
    let mut game = Game::new(&starting_positions);
    game.play();
    println!("final game state: {:?}", game);
    let losing_score = game.players.iter().map(|p| p.score).min().unwrap();
    let rolls = game.rolls;
    losing_score * rolls
}

// fn part2(input: &str) -> usize {
//     let data = Data::parse(input);
//     println!("{:?}", data);
//     data.execute()
// }

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

    // #[test]
    // fn test_part2_example1() {
    //     let result = part2(EXAMPLE1);
    //     assert_eq!(result, 0);
    // }

    // #[test]
    // fn test_part2_solution() {
    //     let result = part2(&read_input_file());
    //     assert_eq!(result, 0);
    // }
}
