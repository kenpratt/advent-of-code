#[macro_use] extern crate lazy_static;
extern crate regex;

use std::collections::HashMap;
use std::convert::TryInto;
use std::fs;
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

    // let mut foo = collapsed_instructions;
    // for i in (0..(101741582076661 as i128)) {
    //     if i % 1000000 == 0 {
    //         println!("i: {}", i);
    //     }
    //     let bar = foo.double().collapse_rest();
    //     //println!("adding instructions: {}, {:?}", i, bar);
    //     foo = bar;
    // }
    // println!("foo: {:?}", foo);

    let foo = collapsed_instructions.multiply(3);
    println!("foo: {:?}", foo);

    // let composite_instruction = CompositeShuffleInstruction::from_instructions(&instructions, 119315717514047);
    // println!("composite: {:?}", composite_instruction);
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

    // let composite_instruction = CompositeShuffleInstruction::from_instructions(&instructions, deck_size);
    // println!("composite: {:?}", composite_instruction);

    // let mut composite_cards = vec![0; deck_size];
    // for i in 0..deck_size {
    //     let to_index = composite_instruction.run(i);
    //     composite_cards[to_index] = i;
    // }
    //println!("composite res: {:?}", composite_cards);
    //assert_eq!(deck.cards, composite_cards);

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

fn pow_with_modulus(base: i128, exponent: i128, modulus: i128) -> i128 {
    if (base < 1) || (exponent < 0) || (modulus < 1) {
        panic!("invalid");
    }

    let mut result = 1;
    let mut curr_base = base;
    let mut curr_exponent = exponent;

    while curr_exponent > 0 {
        if (curr_exponent % 2) == 1 {
            result = (result * curr_base) % modulus;
        }
        curr_base = (curr_base * curr_base) % modulus;
        curr_exponent = curr_exponent / 2;
    }

    result
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
        let new_increment_value = pow_with_modulus(self.increment_value, n, self.deck_size);
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
}

#[derive(Debug)]
struct CompositeShuffleInstruction {
    deck_size: i128,
    multiplier: i128,
    offset: i128,
    reversed: bool,
}

impl CompositeShuffleInstruction {
    pub fn new(deck_size: i128) -> CompositeShuffleInstruction {
        return CompositeShuffleInstruction {
            deck_size: deck_size,
            multiplier: 1,
            offset: 0,
            reversed: false,
        }
    }

    fn from_instructions(instructions: &ShuffleInstructions, deck_size: usize) -> CompositeShuffleInstruction {
        let mut res = CompositeShuffleInstruction::new(deck_size as i128);
        res.apply_all(instructions);
        res
    }

    fn apply_all(&mut self, instructions: &ShuffleInstructions) {
        for instruction in &instructions.instructions {
            self.apply(instruction);
        }
    }

    fn apply(&mut self, instruction: &ShuffleInstruction) {
        match instruction {
            ShuffleInstruction::Cut(n) => {
                let x = *n as i128;
                if !self.reversed {
                    self.offset = self.normalize(self.offset - x);
                } else {
                    self.offset = self.normalize(self.offset + x);
                }
            },
            ShuffleInstruction::DealWithIncrement(n) => {
                let x = *n as i128;
                self.multiplier = self.normalize(self.multiplier * x);
                self.offset = self.normalize(self.offset * x); // offset is scaled
            },
            ShuffleInstruction::DealNewStack => {
                self.reversed = !self.reversed;
                //self.offset *= -1; // offset is relative to reversal
            },
        }
        println!("{:?}", self);
    }

    fn normalize(&self, x: i128) -> i128 {
        if x < 0 {
            //-(-x % self.deck_size)
            println!("normalize neg {} {}", x, -(-x % self.deck_size) + self.deck_size - 1);
            -(-x % self.deck_size) + self.deck_size
             //TODO + self.deck_size - 1 (?)
        } else {
            x % self.deck_size
        }
    }

    fn run(&self, from_index: usize) -> usize {
        // if self.reversed {
        //     panic!("Don't know how to run a reversed composite shuffle");
        // }

        let mut to_index = from_index as i128;

        println!("run from_index = {}, X to_index = {}", from_index, to_index);

        // if still reversed, index into right side
        // if self.reversed {
        //     to_index = -(to_index + 1);
        // }

        println!("run from_index = {}, A to_index = {}", from_index, to_index);

        // scale up index by current multiplier
        to_index *= self.multiplier;

        println!("run from_index = {}, B to_index = {}", from_index, to_index);

        // add offset
        to_index += self.offset;
        // if !self.reversed {
        //     to_index += self.offset;
        // } else {
        //     to_index -= self.offset;
        // }

        println!("run from_index = {}, C to_index = {}", from_index, to_index);

        // if negative (due to offsets), convert to a positive index
        // by adding enough multiples of deck size
        if to_index < 0 {
            to_index += ((-to_index / self.deck_size) + 1) * self.deck_size;
        }

        println!("run from_index = {}, D to_index = {}", from_index, to_index);

        // normalize by deck size
        to_index = to_index % self.deck_size;

        println!("run from_index = {}, E to_index = {}", from_index, to_index);

        if self.reversed {
           to_index = self.deck_size - to_index - 1;
        }

        // safety checks to ensure in bounds
        if to_index < 0 {
            panic!("index below 0. from_index: {}, to_index: {}", from_index, to_index);
        } else if to_index >= self.deck_size {
            panic!("index above deck size. from_index: {}, to_index: {}", from_index, to_index);
        } 

        // if reversed, index into right side of array
        // if self.reversed {
        //     to_index = self.deck_size - to_index - 1;
        // }

        println!("run from_index = {}, final to_index = {}", from_index, to_index);

        to_index as usize
    }

    // fn run_reverse(&self, to_index: usize) -> usize {
    //     0
    // }
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

#[derive(Debug)]
pub struct Solver {
    deck_size: usize,
    instructions: Vec<ShuffleInstruction>,
    increment_offsets: HashMap<usize, Vec<usize>>,
}

impl Solver {
    fn run(deck_size: usize, runs: usize, to_index: usize, instructions: Vec<ShuffleInstruction>) -> usize {
        let mut solver = Solver {
            deck_size: deck_size,
            instructions: instructions,
            increment_offsets: HashMap::new(),
        };
        solver.build_increment_offset_cache();
        println!("Running solver: deck_size = {}, runs = {}, to_index = {}", deck_size, runs, to_index);
        let mut result_index = to_index;
        let mut seen: HashMap<usize, usize> = HashMap::new();
        seen.insert(result_index, 0);
        for i in 1..(runs+1) {
            if i % 100000 == 0 {
              println!("{}", i);
            }
            let foo = solver.run_instructions(result_index);
            if seen.contains_key(&foo) {
                println!("found duplicate! {}, {}", seen.get(&foo).unwrap(), i)
            } else {
                seen.insert(foo, i);
            }
            //println!("{}, {}, {}", result_index, foo, (result_index as i128) - (foo as i128));
            result_index = foo;
        }
        result_index
    }

    fn build_increment_offset_cache(&mut self) {
        for instruction in &mut self.instructions {
            match instruction {
                ShuffleInstruction::DealWithIncrement(raw_increment) => {
                    let increment = *raw_increment as usize;
                    if !self.increment_offsets.contains_key(&increment) {
                        let offsets = Solver::build_offsets_for_increment(increment, self.deck_size);
                        self.increment_offsets.insert(increment, offsets);
                    }
                },
                _ => {},
            }
        }
        //println!("offset cache: {:?}", self.increment_offsets);
    }

    fn build_offsets_for_increment(increment: usize, deck_size: usize) -> Vec<usize> {
        let mut offsets = vec![0; increment];
        for x in 1..increment {
            // deck size 10, increment 3, x = 1
            // x = 1: base = 10, offset = (3 - (10 % 3)) = (3 - 1) = 2
            // x = 2: base = 20, offset = (3 - (20 % 3)) = (3 - 2) = 1
            //
            // deck size 10, increment 7, x = 1
            // x = 1: base = 10, offset = (7 - (10 % 7)) = (7 - 3) = 4
            // x = 2: base = 20, offset = (7 - (20 % 7)) = (7 - 6) = 1
            // x = 3: base = 30, offset = (7 - (30 % 7)) = (7 - 2) = 5
            // x = 4: base = 40, offset = (7 - (40 % 7)) = (7 - 5) = 2
            // x = 5: base = 50, offset = (7 - (50 % 7)) = (7 - 1) = 6
            // x = 6: base = 60, offset = (7 - (60 % 7)) = (7 - 4) = 3
            let base = deck_size * x;
            let offset = increment - (base % increment);
            offsets[offset] = base;
        }
        offsets
    }

    fn get_increment_offset(&self, increment: usize, index: usize) -> usize {
        let offset = index % increment;
        self.increment_offsets[&increment][offset]
    }

    fn run_instructions(&self, to_index: usize) -> usize {
        let mut result = to_index;
        for instruction in self.instructions.iter().rev() {
            result = self.run_instruction(result, instruction);
        }
        result
    }

    fn run_instruction(&self, to_index: usize, instruction: &ShuffleInstruction) -> usize {
        match instruction {
            ShuffleInstruction::Cut(n) => {
                let from_index = ((to_index as i128) + (*n as i128)) as usize;
                //println!("Cut: to_index = {}, n = {}, from_index = {}", to_index, n, from_index);
                from_index
            },
            ShuffleInstruction::DealWithIncrement(raw_increment) => {
                let increment = *raw_increment as usize;
                let base = self.get_increment_offset(increment, to_index);
                let from_index = (to_index + base) / increment;
                //println!("DealWithIncrement: to_index = {}, increment = {}, base = {}, from_index = {}", to_index, increment, base, from_index);
                from_index
            },
            ShuffleInstruction::DealNewStack => {
                let from_index = self.deck_size - to_index - 1;
                //println!("DealNewStack: to_index = {}, from_index = {}", to_index, from_index);
                from_index
            },
        }
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
