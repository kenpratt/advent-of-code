use std::collections::VecDeque;
use std::fs;

// use lazy_static::lazy_static;
// use regex::Regex;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    // println!("part 2 result: {:?}", part2(&read_input_file()));
}

fn read_input_file() -> String {
    return fs::read_to_string("input.txt").expect("Something went wrong reading the file");
}

#[derive(Debug)]
struct Combat {
    decks: Vec<Deck>,
}

impl Combat {
    fn parse(input: &str) -> Combat {
        let decks = input.split("\n\n").map(|chunk| Deck::parse(chunk)).collect();
        return Combat {
            decks: decks,
        }
    }

    fn run(&mut self) {
        while !self.is_over() {
            self.tick()
        }
    }

    fn tick(&mut self) {
        let cards: Vec<usize> = self.decks.iter_mut().map(|deck| deck.draw().unwrap()).collect();
        let (winning_card_index, winning_card) = cards.iter().enumerate().max_by_key(|(_, card)| *card).unwrap();

        // append winning card
        self.decks[winning_card_index].append(*winning_card);

        // append losing card
        for losing_card in cards.iter().filter(|c| *c != winning_card) {
            self.decks[winning_card_index].append(*losing_card);
        }
    }

    fn is_over(&self) -> bool {
        self.decks.iter().any(|deck| deck.is_empty())
    }

    fn score(&self) -> usize {
        self.decks.iter().fold(0, |acc, deck| acc + deck.score())
    }
}

#[derive(Debug)]
struct Deck {
    player: String,
    cards: VecDeque<usize>,
}

impl Deck {
    fn parse(input: &str) -> Deck {
        let mut lines = input.lines();
        let player: String = lines.next().unwrap().to_string();
        let cards: VecDeque<usize> = lines.map(|s| s.parse::<usize>().unwrap()).collect();

        return Deck {
            player: player,
            cards: cards,
        }
    }

    fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    fn draw(&mut self) -> Option<usize> {
        self.cards.pop_front()
    }

    fn append(&mut self, card: usize) {
        self.cards.push_back(card)
    }

    fn score(&self) -> usize {
        let deck_size = self.cards.len();
        self.cards.iter().enumerate().fold(0, |acc, (index, card)| {
            let slot = deck_size - index;
            let score = slot * card;
            acc + score
        })
    }
}

fn part1(input: &str) -> usize {
    let mut game = Combat::parse(input);
    game.run();
    game.score()
}

// fn part2(input: &str) -> usize {
//     let game = Combat::parse(input);
//     return game.execute();
// }

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    static EXAMPLE1: &str = indoc! {"
        Player 1:
        9
        2
        6
        3
        1
        
        Player 2:
        5
        8
        4
        7
        10
    "};

    #[test]
    fn test_part1_example1() {
        let result = part1(EXAMPLE1);
        assert_eq!(result, 306);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file());
        assert_eq!(result, 31957);
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