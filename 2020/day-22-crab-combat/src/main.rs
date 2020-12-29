use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs;

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
    previously_seen_decks: HashSet<Vec<Deck>>,
    game: usize,
    round: usize,
}

impl Combat {
    fn parse(input: &str) -> Combat {
        let decks = input.split("\n\n").map(|chunk| Deck::parse(chunk)).collect();
        Combat {
            decks: decks,
            previously_seen_decks: HashSet::new(),
            game: 1,
            round: 0,
        }
    }

    fn play(&mut self, recursive: bool) -> usize {
        println!("\nGame {}:\n", self.game);

        while self.winner(recursive).is_none() {
            self.tick(recursive)
        }

        let winner = self.winner(recursive).unwrap();
        println!("\nPlayer {} wins game {}\n", winner + 1, self.game);
        winner
    }

    fn tick(&mut self, recursive: bool) {
        self.round += 1;
        println!("Round {} (Game {}):", self.round, self.game);
        for deck in &self.decks {
            println!("  {} {:?}", deck.player, deck.cards);
        }
        
        // add deck states to list for recursive victory
        self.previously_seen_decks.insert(self.decks.clone());

        let cards: Vec<usize> = self.decks.iter_mut().map(|deck| deck.draw().unwrap()).collect();
        println!("cards: {:?}", cards);

        // recursive game initiation
        let winner_index: usize = if recursive && cards.iter().enumerate().all(|(index, card)| self.decks[index].len() >= *card) {
            // recursive game
            let decks_for_sub_game: Vec<Deck> = cards.iter().enumerate().map(|(index, card)| {
                self.decks[index].sub_deck(*card)
            }).collect();
            let mut sub_game = self.sub_game(decks_for_sub_game);
            sub_game.play(recursive)
        } else {
            // normal rules, highest card wins
            let (winning_card_index, _) = cards.iter().enumerate().max_by_key(|(_, card)| *card).unwrap();
            winning_card_index
        };

        println!("  Player {} wins Round {} of Game {}", winner_index+1, self.round, self.game);

        // append winning card
        let winning_card = cards[winner_index];
        self.decks[winner_index].append(winning_card);

        // append losing card
        for losing_card in cards.iter().filter(|c| **c != winning_card) {
            self.decks[winner_index].append(*losing_card);
        }        
    }

    fn sub_game(&self, sub_game_decks: Vec<Deck>) -> Combat {
        Combat {
            decks: sub_game_decks,
            previously_seen_decks: HashSet::new(),
            game: self.game + 1,
            round: 0,
        }
    }

    fn winner(&self, recursive: bool) -> Option<usize> {
        // if there is exactly one non-empty deck left, that's the winner
        let non_empty_deck_indices: Vec<usize> = self.decks.iter().enumerate().filter(|(_, deck)| !deck.is_empty()).map(|(index, _)| index).collect();
        if non_empty_deck_indices.len() == 1 {
            return Some(non_empty_deck_indices[0]);
        }

        // recursive victory condition
        if recursive && self.previously_seen_decks.contains(&self.decks) {
            // player 1 victory
            return Some(0);
        }

        None
    }

    fn score(&self) -> usize {
        self.decks.iter().fold(0, |acc, deck| acc + deck.score())
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Deck {
    player: String,
    cards: VecDeque<usize>,
}

impl Deck {
    fn parse(input: &str) -> Deck {
        let mut lines = input.lines();
        let player: String = lines.next().unwrap().to_string();
        let cards: VecDeque<usize> = lines.map(|s| s.parse::<usize>().unwrap()).collect();

        Deck {
            player: player,
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
            player: self.player.clone(),
            cards: self.cards.iter().take(num).cloned().collect(),
        }
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