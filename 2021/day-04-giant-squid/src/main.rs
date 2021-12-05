use std::fs;

use std::collections::HashMap;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    println!("part 2 result: {:?}", part2(&read_input_file()));
}

fn read_input_file() -> String {
    fs::read_to_string("input.txt").expect("Something went wrong reading the file")
}

#[derive(Debug)]
struct Game {
    numbers: Vec<usize>,
    boards: Vec<Board>,
}

impl Game {
    fn parse(input: &str) -> Game {
        let mut parts = input.split("\n\n");
        let numbers = parts
            .next()
            .unwrap()
            .split(',')
            .map(|s| s.parse::<usize>().unwrap())
            .collect();
        let boards = parts.map(|p| Board::parse(p)).collect();
        Game {
            numbers: numbers,
            boards: boards,
        }
    }

    fn execute(&mut self) -> Vec<usize> {
        self.draw_numbers();
        let mut winners: Vec<Bingo> = self.boards.iter().filter_map(|b| b.bingo).collect();
        winners.sort_by_cached_key(|(round, _, _)| *round);
        winners.into_iter().map(|(_, _, score)| score).collect()
    }

    fn draw_numbers(&mut self) {
        let numbers = self.numbers.clone();
        for (round, number) in numbers.iter().enumerate() {
            self.mark_boards(&round, number);
        }
    }

    fn mark_boards(&mut self, round: &usize, number: &usize) {
        for board in &mut self.boards {
            board.mark(round, number);
        }
    }
}

type Position = (usize, usize);
type Bingo = (usize, usize, usize);

#[derive(Debug)]
struct Board {
    unmarked: HashMap<usize, Position>,
    marked_in_row: [u8; 5],
    marked_in_column: [u8; 5],
    bingo: Option<Bingo>,
}

impl Board {
    fn parse(input: &str) -> Board {
        let rows: Vec<Vec<usize>> = input
            .trim()
            .split("\n")
            .map(|row| {
                row.split_whitespace()
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect()
            })
            .collect();

        let mut unmarked = HashMap::new();
        for (y, row) in rows.iter().enumerate() {
            for (x, number) in row.iter().enumerate() {
                unmarked.insert(*number, (x, y));
            }
        }

        Board {
            unmarked: unmarked,
            marked_in_row: [0; 5],
            marked_in_column: [0; 5],
            bingo: None,
        }
    }

    fn mark(&mut self, round: &usize, number: &usize) {
        if self.bingo.is_none() {
            match self.unmarked.remove(number) {
                Some(position) => {
                    let (x, y) = position;
                    self.marked_in_row[y] += 1;
                    self.marked_in_column[x] += 1;
                    if self.check_for_bingo() {
                        self.bingo = Some((*round, *number, self.score(number)));
                    }
                }
                None => {}
            }
        }
    }

    fn check_for_bingo(&self) -> bool {
        self.marked_in_row.iter().any(|&n| n == 5) || self.marked_in_column.iter().any(|&n| n == 5)
    }

    fn score(&self, number: &usize) -> usize {
        self.unmarked.keys().sum::<usize>() * number
    }
}

fn part1(input: &str) -> usize {
    let mut game = Game::parse(input);
    let winning_scores = game.execute();
    *winning_scores.first().unwrap()
}

fn part2(input: &str) -> usize {
    let mut game = Game::parse(input);
    let winning_scores = game.execute();
    *winning_scores.last().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    static EXAMPLE1: &str = indoc! {"
        7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

        22 13 17 11  0
        8  2 23  4 24
        21  9 14 16  7
        6 10  3 18  5
        1 12 20 15 19
        
        3 15  0  2 22
        9 18 13 17  5
        19  8  7 25 23
        20 11 10 24  4
        14 21 16 12  6
        
        14 21 17 24  4
        10 16 15  9 19
        18  8 23 26 20
        22 11 13  6  5
        2  0 12  3  7
    "};

    #[test]
    fn test_part1_example1() {
        let result = part1(EXAMPLE1);
        assert_eq!(result, 4512);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file());
        assert_eq!(result, 72770);
    }

    #[test]
    fn test_part2_example1() {
        let result = part2(EXAMPLE1);
        assert_eq!(result, 1924);
    }

    #[test]
    fn test_part2_solution() {
        let result = part2(&read_input_file());
        assert_eq!(result, 13912);
    }
}
