#[macro_use] extern crate lazy_static;
extern crate regex;

use std::convert::TryInto;
use std::fs;
use mod_exp::mod_exp;
use regex::Regex;

fn main() {
    part1();
    part2();
}

fn part1() {
    let input_str = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let deck = shuffle(10007, &input_str);
    assert_eq!(deck.cards.iter().position(|&c| c == 2019), Some(2558));
}

fn part2() {
    let input_str = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let instructions = ShuffleInstructions::parse(119315717514047, &input_str);

    let collapsed_instructions = instructions.collapse();
    println!("collapsed: {:?}", collapsed_instructions);

    let repeated_instructions = collapsed_instructions.multiply(101741582076661);
    println!("repeated_instructions: {:?}", repeated_instructions);

    let from_index = repeated_instructions.calculate_from_index(2020);
    assert_eq!(from_index, 1);
}

fn shuffle(deck_size: usize, input_str: &str) -> Deck {
    let instructions = ShuffleInstructions::parse(deck_size, input_str);
    let mut deck = Deck::new(deck_size);
    deck.shuffle(&instructions);

    let collapsed_instructions = instructions.collapse();
    println!("collapsed: {:?}", collapsed_instructions);
    let mut deck2 = Deck::new(deck_size);
    deck2.shuffle(&collapsed_instructions.materialize());
    assert_eq!(deck.cards, deck2.cards);

    deck
}

#[derive(Clone, Copy, Debug)]
enum ShuffleInstruction {
    Cut(i128),
    DealWithIncrement(i128),
    DealNewStack,
}

impl ShuffleInstruction { 
    fn parse(line: &str) -> ShuffleInstruction {
        lazy_static! {
            static ref RE_CUT: Regex = Regex::new(r"^cut (\-?\d+)$").unwrap();
            static ref RE_DEAL_WITH_INCREMENT: Regex = Regex::new(r"^deal with increment (\d+)$").unwrap();
            static ref RE_DEAL_NEW_STACK: Regex = Regex::new(r"^deal into new stack$").unwrap();
        }

        if RE_CUT.is_match(line) {
            let captures = RE_CUT.captures(line).unwrap();
            let n = captures.get(1).unwrap().as_str().parse::<i128>().unwrap();
            return ShuffleInstruction::Cut(n);
        } else if RE_DEAL_WITH_INCREMENT.is_match(line) {
            let captures = RE_DEAL_WITH_INCREMENT.captures(line).unwrap();
            let increment = captures.get(1).unwrap().as_str().parse::<i128>().unwrap();
            return ShuffleInstruction::DealWithIncrement(increment);
        } else if RE_DEAL_NEW_STACK.is_match(line) {
            return ShuffleInstruction::DealNewStack;
        } else {
            panic!("cannot parse line: {}", line);
        }
    }
}

#[derive(Debug)]
struct ShuffleInstructions {
    deck_size: i128,
    instructions: Vec<ShuffleInstruction>,
}

impl ShuffleInstructions {
    fn parse(deck_size: usize, input: &str) -> ShuffleInstructions {
        let instructions = input.lines().map(|line| ShuffleInstruction::parse(line)).collect();
        ShuffleInstructions::new(deck_size as i128, instructions)
    }
    
    fn new(deck_size: i128, instructions: Vec<ShuffleInstruction>) -> ShuffleInstructions {
        return ShuffleInstructions {
            deck_size: deck_size,
            instructions: instructions,
        }
    }

    fn collapse(&self) -> CollapsedShuffleInstructions {
        self.normalize().fully_collapse()
    }

    fn normalize(&self) -> ShuffleInstructions {
        let mut new_instructions = vec![];
        for instruction in &self.instructions {
            match instruction {
                ShuffleInstruction::Cut(n) => {
                    if *n < 0 {
                        // negative cuts can be converted to positive cut
                        // eg for deck size 10, cut -3 == cut 7
                        new_instructions.push(ShuffleInstruction::Cut(self.deck_size + *n));
                    } else {
                        new_instructions.push(*instruction);
                    }
                },
                ShuffleInstruction::DealWithIncrement(_) => {
                    new_instructions.push(*instruction);
                },
                ShuffleInstruction::DealNewStack => {
                    // can be substituted with cut -1, deal with increment {deck size - 1}
                    new_instructions.push(ShuffleInstruction::Cut(self.deck_size - 1));
                    new_instructions.push(ShuffleInstruction::DealWithIncrement(self.deck_size - 1));
                },
            }
        }
        ShuffleInstructions::new(self.deck_size, new_instructions)
    }

    fn fully_collapse(&self) -> CollapsedShuffleInstructions {
        let mut acc_increment = 1;
        let mut acc_cut = 0;

        // iterate in reverse since cuts are multiplied by increments *after* them,
        // and since increments are multiplied by each other, order doesn't matter
        for instruction in self.instructions.iter().rev() {
            match instruction {
                ShuffleInstruction::Cut(n) => {
                    // cut is scaled by current increment and then added to current cut value
                    let cut_value = *n * acc_increment;
                    acc_cut = (acc_cut + cut_value) % self.deck_size;
                },
                ShuffleInstruction::DealWithIncrement(n) => {
                    // incrument is scaled
                    let increment_value = *n;
                    acc_increment = (acc_increment * increment_value) % self.deck_size;
                },
                ShuffleInstruction::DealNewStack => {
                    panic!("Unreachable");
                },
            }
        }

        CollapsedShuffleInstructions::new(self.deck_size, acc_increment, acc_cut)
    }
}

#[derive(Debug)]
struct CollapsedShuffleInstructions {
    deck_size: i128,
    increment_value: i128,
    cut_value: i128,
}

impl CollapsedShuffleInstructions {
    fn new(deck_size: i128, increment_value: i128, cut_value: i128) -> CollapsedShuffleInstructions {
        CollapsedShuffleInstructions {
            deck_size: deck_size,
            increment_value: increment_value,
            cut_value: cut_value,
        }
    }

    fn multiply(&self, n: i128) -> CollapsedShuffleInstructions {
        let new_increment_value = mod_exp(self.increment_value, n, self.deck_size);
        let new_cut_value = (self.cut_value * (new_increment_value - 1)) % self.deck_size;
        CollapsedShuffleInstructions::new(self.deck_size, new_increment_value, new_cut_value)
    }

    fn materialize(&self) -> ShuffleInstructions {
        let instructions = vec![
            ShuffleInstruction::DealWithIncrement(self.increment_value),
            ShuffleInstruction::Cut(self.cut_value),
        ];
        ShuffleInstructions::new(self.deck_size, instructions)
    }

    fn calculate_from_index(&self, to_index: usize) -> usize {
        let mut from_index = to_index as i128;

        // apply cut (positive since going in reverse)
        from_index = (from_index + self.cut_value) % self.deck_size;

        // apply increment
        let offset = from_index % self.increment_value;
        println!("want to reverse increment... from_index {}, increment value {}, offset {}, deck_size {}", from_index, self.increment_value, offset, self.deck_size);

        // TODO need to do inverse of:
        // let to_index = (from_index * self.increment_value) % self.deck_size;

        // let multiple = (1..self.increment_value).find(|m| offset == (self.increment_value - ((self.deck_size * m) % self.increment_value))).unwrap();
        // let base = self.deck_size * multiple;
        // println!("reverse increment... from_index {}, increment value {}, offset {}, multiple {}, deck_size {}, base {}", from_index, self.increment_value, offset, multiple, self.deck_size, base);
        // from_index = (from_index + base) / self.increment_value;

        from_index as usize
    }
}

#[derive(Debug)]
pub struct Deck {
    size: usize,
    cards: Vec<usize>,
    table: Vec<usize>,
}

impl Deck {
    pub fn new(size: usize) -> Deck {
        let cards = (0..size).collect();
        let table = vec![0; size as usize];
        return Deck {
            size: size,
            cards: cards,
            table: table,
        }
    }

    fn shuffle(&mut self, instructions: &ShuffleInstructions) {
        for instruction in &instructions.instructions {
            //println!("shuffle step: {:?}", instruction);
            match instruction {
                ShuffleInstruction::Cut(n) => self.cut(*n),
                ShuffleInstruction::DealWithIncrement(n) => self.deal_with_increment(*n),
                ShuffleInstruction::DealNewStack => self.deal_new_stack(),
            }
            //println!("after step: {:?}", self.cards);
        }
    }

    fn cut(&mut self, n: i128) {
        //println!("cut: {:?}", n);
        if n > 0 {
            self.cards.rotate_left(n.try_into().unwrap());
        } else {
            self.cards.rotate_right((-n).try_into().unwrap());
        }
    }

    fn deal_with_increment(&mut self, n: i128) {
        //println!("deal_with_increment: {:?}", n);
        for from_index in 0..self.size {
            let to_index = ((from_index as usize) * (n as usize)) % (self.size as usize);
            self.table[to_index] = self.cards[(from_index as usize)];
        }
        self.cards.swap_with_slice(&mut self.table);
    }

    fn deal_new_stack(&mut self) {
        //println!("deal_new_stack");
        self.cards.reverse();
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_deal_into_new_stack() {
        let deck = shuffle(10, "deal into new stack");
        assert_eq!(deck.cards, vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);

        let deck2 = shuffle(10, "deal into new stack\ndeal into new stack");
        assert_eq!(deck2.cards, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_cut() {
        let deck = shuffle(10, "cut 3");
        assert_eq!(deck.cards, vec![3, 4, 5, 6, 7, 8, 9, 0, 1, 2]);

        let deck2 = shuffle(10, "cut -4");
        assert_eq!(deck2.cards, vec![6, 7, 8, 9, 0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_deal_with_increment() {
        let deck = shuffle(10, "deal with increment 3");
        assert_eq!(deck.cards, vec![0, 7, 4, 1, 8, 5, 2, 9, 6, 3]);

        let deck2 = shuffle(10, "deal with increment 7");
        assert_eq!(deck2.cards, vec![0, 3, 6, 9, 2, 5, 8, 1, 4, 7]);
    }

    #[test]
    fn test_cut_reverse_cut() {
        let deck = shuffle(10, "cut 3\ndeal into new stack\ncut 2");
        assert_eq!(deck.cards, vec![0, 9, 8, 7, 6, 5, 4, 3, 2, 1]);
    }

    #[test]
    fn test_cut_increment_cut() {
        let deck = shuffle(10, "cut 3\ndeal with increment 3\ncut 2");
        assert_eq!(deck.cards, vec![7, 4, 1, 8, 5, 2, 9, 6, 3, 0]);
        //assert_eq!(deck.cards, vec![, , , , , , , , , ]);
    }

    #[test]
    fn test_reverse_increment() {
        let deck = shuffle(10, "deal into new stack\ndeal with increment 3");
        // 0 => 9 (-1) * 3 = -3 + 10 = 7
        // 1 => 8 (-2) * 3 = -6 + 10 = 4
        // 2 => 7 (-3) * 3 = -9 + 10 = 1
        // 3 => 6 (-4) * 3 = -12 + 20 = 8
        // 4 => 5 (-5) * 3 = -15 + 20 = 5
        // 5 => 4 (-6) * 3 = -18 + 20 = 2
        // 6 => 3 (-7) * 3 = -21 + 30 = 9
        // 7 => 2 (-8) * 3 = -24 + 30 = 6
        // 8 => 1 (-9) * 3 = -27 + 30 = 3
        // 9 => 0 (-10) * 3 = -30 + 30 = 0
        assert_eq!(deck.cards, vec![9, 2, 5, 8, 1, 4, 7, 0, 3, 6]);

        let mut deck2 = shuffle(10, "cut -1\ndeal with increment 9\ndeal with increment 3");
        assert_eq!(deck2.cards, deck.cards);

        deck2 = shuffle(10, "cut -1\ndeal with increment 27");
        assert_eq!(deck2.cards, deck.cards);

        deck2 = shuffle(10, "cut -1\ndeal with increment 7");
        assert_eq!(deck2.cards, deck.cards);

        deck2 = shuffle(10, "cut 9\ndeal with increment 7");
        assert_eq!(deck2.cards, deck.cards);

        deck2 = shuffle(10, "deal with increment 7\ncut -7");
        assert_eq!(deck2.cards, deck.cards);

        deck2 = shuffle(10, "deal with increment 7\ncut 3");
        assert_eq!(deck2.cards, deck.cards);

        deck2 = shuffle(10, "deal with increment 3\ndeal into new stack\ncut 2");
        assert_eq!(deck2.cards, deck.cards);

        deck2 = shuffle(10, "deal with increment 3\ncut -2\ndeal into new stack");
        assert_eq!(deck2.cards, deck.cards);

        //TODO can't ignore reversals when dealing with increments...
        //maybe can get rid of reversals with a transform though
    }

    #[test]
    fn test_increment_reverse() {
        let deck = shuffle(10, "deal with increment 3\ndeal into new stack");
        assert_eq!(deck.cards, vec![3, 6, 9, 2, 5, 8, 1, 4, 7, 0]);

        let mut deck2 = shuffle(10, "deal with increment 7\ncut 1");
        assert_eq!(deck2.cards, deck.cards);
    }

    #[test]
    fn test_reverse_increment_reverse() {
        let deck = shuffle(10, "deal into new stack\ndeal with increment 3\ndeal into new stack");
        assert_eq!(deck.cards, vec![6, 3, 0, 7, 4, 1, 8, 5, 2, 9]);

        let mut deck2 = shuffle(10, "cut -1\ndeal with increment 7\ndeal into new stack");
        assert_eq!(deck2.cards, deck.cards);

        deck2 = shuffle(10, "cut -1\ndeal with increment 3\ncut 1");
        assert_eq!(deck2.cards, deck.cards);

        deck2 = shuffle(10, "deal with increment 3\ncut -2");
        assert_eq!(deck2.cards, deck.cards);
    }

    #[test]
    fn test_foo() {
        let deck = shuffle(10, "deal into new stack");
        assert_eq!(deck.cards, vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);
        
        let mut deck2 = shuffle(10, "cut -1\ndeal with increment 9");
        assert_eq!(deck2.cards, deck.cards);
        
        deck2 = shuffle(10, "deal with increment 9\ncut -9");
        assert_eq!(deck2.cards, deck.cards);
        
        deck2 = shuffle(10, "deal with increment 9\ncut 1");
        assert_eq!(deck2.cards, deck.cards);
    }

    #[test]
    fn test_increment_reverse_increment() {
        let deck = shuffle(10, "deal with increment 3\ndeal into new stack\ndeal with increment 3");
        assert_eq!(deck.cards, vec![3, 4, 5, 6, 7, 8, 9, 0, 1, 2]);
        
        let mut deck2 = shuffle(10, "cut 3");
        assert_eq!(deck2.cards, deck.cards);
        
        deck2 = shuffle(10, "cut 3");
        assert_eq!(deck2.cards, deck.cards);
    }

    #[test]
    fn test_reverse_cut_increment() {
        let deck = shuffle(10, "deal into new stack\ncut 2\ndeal with increment 3");
        assert_eq!(deck.cards, vec![7, 0, 3, 6, 9, 2, 5, 8, 1, 4]);
    }

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
    fn test_part1_example2b() {
        let deck = shuffle(
            10,
            "cut 6\ndeal with increment 7",
        );
        assert_eq!(deck.cards, vec![6, 9, 2, 5, 8, 1, 4, 7, 0, 3]);
        //assert_eq!(deck.cards, vec![9, 2, 5, 8, 1, 4, 7, 0, 3, 6]);
    }    

    #[test]
    fn test_part1_example2c() {
        let deck = shuffle(
            10,
            "cut 6\ndeal into new stack",
        );
        assert_eq!(deck.cards, vec![5, 4, 3, 2, 1, 0, 9, 8, 7, 6]);
    }

    #[test]
    fn test_part1_example2d() {
        let deck = shuffle(
            10,
            "deal with increment 7\ndeal into new stack",
        );
        assert_eq!(deck.cards, vec![7, 4, 1, 8, 5, 2, 9, 6, 3, 0]);
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
