use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs;
use std::hash::{Hash, Hasher};

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    println!("part 2 result: {:?}", part2(&read_input_file()));
}

fn read_input_file() -> String {
    return fs::read_to_string("input.txt").expect("Something went wrong reading the file");
}

#[derive(Debug)]
struct Combat {
    decks: Vec<Deck>,
    decks_hash: u64,
    previously_seen_decks: HashSet<u64>,
    game: usize,
    round: usize,
}

impl Combat {
    fn parse(input: &str) -> Combat {
        let decks = input.split("\n\n").map(|chunk| Deck::parse(chunk)).collect();
        let hash = Combat::calculate_decks_hash(&decks);
        Combat {
            decks: decks,
            decks_hash: hash,
            previously_seen_decks: HashSet::new(),
            game: 1,
            round: 0,
        }
    }

    fn play(&mut self, recursive: bool) -> usize {
        while self.winner(recursive).is_none() {
            self.tick(recursive)
        }

        self.winner(recursive).unwrap()
    }

    fn tick(&mut self, recursive: bool) {
        self.round += 1;

        // add deck states to list for recursive victory
        self.previously_seen_decks.insert(self.decks_hash);

        let cards: Vec<u8> = self.decks.iter_mut().map(|deck| deck.draw().unwrap()).collect();

        // recursive game initiation
        let winner_index: usize = if recursive && cards.iter().enumerate().all(|(index, card)| self.decks[index].len() >= (*card as usize)) {
            // recursive game
            let decks_for_sub_game: Vec<Deck> = cards.iter().enumerate().map(|(index, card)| {
                self.decks[index].sub_deck(*card as usize)
            }).collect();
            let mut sub_game = self.sub_game(decks_for_sub_game);
            sub_game.play(recursive)
        } else {
            // normal rules, highest card wins
            let (winning_card_index, _) = cards.iter().enumerate().max_by_key(|(_, card)| *card).unwrap();
            winning_card_index
        };

        // append winning card
        let winning_card = cards[winner_index];
        self.decks[winner_index].append(winning_card);

        // append losing card
        for losing_card in cards.iter().filter(|c| **c != winning_card) {
            self.decks[winner_index].append(*losing_card);
        }

        // recalculate decks hash
        self.decks_hash = Combat::calculate_decks_hash(&self.decks);
    }

    fn sub_game(&self, sub_game_decks: Vec<Deck>) -> Combat {
        let hash = Combat::calculate_decks_hash(&sub_game_decks);
        Combat {
            decks: sub_game_decks,
            decks_hash: hash,
            previously_seen_decks: HashSet::new(),
            game: self.game + 1,
            round: 0,
        }
    }

    fn winner(&self, recursive: bool) -> Option<usize> {
        // if there's an empty deck, the game is over
        if self.decks.iter().any(|deck| deck.is_empty()) {
            // non-empty deck is the winner
            let non_empty_deck_indices: Vec<usize> = self.decks.iter().enumerate().filter(|(_, deck)| !deck.is_empty()).map(|(index, _)| index).collect();
            assert_eq!(non_empty_deck_indices.len(), 1);
            return Some(non_empty_deck_indices[0]);
        }

        // recursive victory condition
        if recursive && self.previously_seen_decks.contains(&self.decks_hash) {
            // player 1 victory
            return Some(0);
        }

        None
    }

    fn score(&self) -> usize {
        self.decks.iter().fold(0, |acc, deck| acc + deck.score())
    }

    fn calculate_decks_hash(decks: &Vec<Deck>) -> u64 {
        let mut hasher = DefaultHasher::new();
        for deck in decks {
            deck.hash(&mut hasher);
        }
        hasher.finish()
    }
}

#[derive(Debug, Hash, PartialEq)]
struct Deck {
    cards: VecDeque<u8>,
}

impl Deck {
    fn parse(input: &str) -> Deck {
        let mut lines = input.lines();
        lines.next(); // Player X
        let cards: VecDeque<u8> = lines.map(|s| s.parse::<u8>().unwrap()).collect();

        Deck {
            cards: cards,
        }
    }

    fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    fn len(&self) -> usize {
        self.cards.len()
    }

    fn sub_deck(&self, num: usize) -> Deck {
        Deck {
            cards: self.cards.iter().take(num).cloned().collect(),
        }
    }

    fn draw(&mut self) -> Option<u8> {
        self.cards.pop_front()
    }

    fn append(&mut self, card: u8) {
        self.cards.push_back(card)
    }

    fn score(&self) -> usize {
        let deck_size = self.cards.len();
        self.cards.iter().enumerate().fold(0, |acc, (index, card)| {
            let slot = deck_size - index;
            let score = slot * (*card as usize);
            acc + score
        })
    }
}

fn part1(input: &str) -> usize {
    let mut game = Combat::parse(input);
    game.play(false);
    game.score()
}

fn part2(input: &str) -> usize {
    let mut game = Combat::parse(input);
    game.play(true);
    game.score()
}

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

    #[test]
    fn test_part2_example1() {
        let result = part2(EXAMPLE1);
        assert_eq!(result, 291);
    }

    #[test]
    fn test_part2_solution() {
        let result = part2(&read_input_file());
        assert_eq!(result, 33212);
    }
}