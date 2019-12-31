#[macro_use] extern crate lazy_static;
extern crate regex;

use std::convert::TryInto;
use std::fs;
use regex::Regex;

fn main() {
    part1();
}

fn part1() {
    let input_str = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let deck = shuffle(10007, &input_str);
    assert_eq!(deck.cards.iter().position(|&c| c == 2019), Some(2558));
}

fn shuffle(deck_size: u16, shuffle_instructions: &str) -> Deck {
    let instructions: Vec<ShuffleInstruction> = shuffle_instructions.lines().map(|line| parse_shuffle_instruction(line)).collect();

    let mut deck = Deck::new(deck_size);
    deck.shuffle(&instructions);
    deck
}

#[derive(Debug)]
enum ShuffleInstruction {
    Cut(i16),
    DealWithIncrement(u8),
    DealNewStack,
}

fn parse_shuffle_instruction(line: &str) -> ShuffleInstruction {
    lazy_static! {
        static ref RE_CUT: Regex = Regex::new(r"^cut (\-?\d+)$").unwrap();
        static ref RE_DEAL_WITH_INCREMENT: Regex = Regex::new(r"^deal with increment (\d+)$").unwrap();
        static ref RE_DEAL_NEW_STACK: Regex = Regex::new(r"^deal into new stack$").unwrap();
    }

    if RE_CUT.is_match(line) {
        let captures = RE_CUT.captures(line).unwrap();
        let n = captures.get(1).unwrap().as_str().parse::<i16>().unwrap();
        return ShuffleInstruction::Cut(n);
    } else if RE_DEAL_WITH_INCREMENT.is_match(line) {
        let captures = RE_DEAL_WITH_INCREMENT.captures(line).unwrap();
        let n = captures.get(1).unwrap().as_str().parse::<u8>().unwrap();
        return ShuffleInstruction::DealWithIncrement(n);
    } else if RE_DEAL_NEW_STACK.is_match(line) {
        return ShuffleInstruction::DealNewStack;
    } else {
        panic!("cannot parse line: {}", line);
    }
}

#[derive(Debug)]
pub struct Deck {
    size: u16,
    cards: Vec<u16>,
    table: Vec<u16>,
}

impl Deck {
    pub fn new(size: u16) -> Deck {
        let cards = (0..size).collect();
        let table = vec![0; size as usize];
        return Deck {
            size: size,
            cards: cards,
            table: table,
        }
    }

    fn shuffle(&mut self, instructions: &[ShuffleInstruction]) {
        for instruction in instructions {
            println!("shuffle step: {:?}", instruction);
            match instruction {
                ShuffleInstruction::Cut(n) => self.cut(*n),
                ShuffleInstruction::DealWithIncrement(n) => self.deal_with_increment(*n),
                ShuffleInstruction::DealNewStack => self.deal_new_stack(),
            }
            println!("after step: {:?}", self.cards);
        }
    }

    fn cut(&mut self, n: i16) {
        println!("cut: {:?}", n);
        if n > 0 {
            self.cards.rotate_left(n.try_into().unwrap());
        } else {
            self.cards.rotate_right((-n).try_into().unwrap());
        }
    }

    fn deal_with_increment(&mut self, n: u8) {
        println!("deal_with_increment: {:?}", n);
        for from_index in 0..self.size {
            let to_index = ((from_index as usize) * (n as usize)) % (self.size as usize);
            self.table[to_index] = self.cards[(from_index as usize)];
        }
        self.cards.swap_with_slice(&mut self.table);
    }

    fn deal_new_stack(&mut self) {
        println!("deal_new_stack");
        self.cards.reverse();
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_part1_example1() {
        let deck = shuffle(
            10,
            "deal with increment 7\ndeal into new stack\ndeal into new stack",
        );
        assert_eq!(deck.cards, vec![0, 3, 6, 9, 2, 5, 8, 1, 4, 7]);
    }

    #[test]
    fn test_part1_example2() {
        let deck = shuffle(
            10,
            "cut 6\ndeal with increment 7\ndeal into new stack",
        );
        assert_eq!(deck.cards, vec![3, 0, 7, 4, 1, 8, 5, 2, 9, 6]);
    }

    #[test]
    fn test_part1_example3() {
        let deck = shuffle(
            10,
            "deal with increment 7\ndeal with increment 9\ncut -2",
        );
        assert_eq!(deck.cards, vec![6, 3, 0, 7, 4, 1, 8, 5, 2, 9]);
    }

    #[test]
    fn test_part1_example4() {
        let deck = shuffle(
            10,
            "deal into new stack\ncut -2\ndeal with increment 7\ncut 8\ncut -4\ndeal with increment 7\ncut 3\ndeal with increment 9\ndeal with increment 3\ncut -1",
        );
        assert_eq!(deck.cards, vec![9, 2, 5, 8, 1, 4, 7, 0, 3, 6]);
    }
}
