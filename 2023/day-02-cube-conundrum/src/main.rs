use std::cmp;
use std::collections::HashMap;
use std::fs;

use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    println!("part 2 result: {:?}", part2(&read_input_file()));
}

fn read_input_file() -> String {
    fs::read_to_string("input.txt").expect("Something went wrong reading the file")
}

#[derive(Debug)]
struct Game {
    id: usize,
    pulls: Vec<Pull>,
}

impl Game {
    fn parse_list(input: &str) -> Vec<Self> {
        input.lines().map(|line| Self::parse(line)).collect()
    }

    fn parse(input: &str) -> Self {
        lazy_static! {
            static ref GAME_RE: Regex = Regex::new(r"\AGame (\d+): (.+)\z").unwrap();
        }

        let caps = GAME_RE.captures(input).unwrap();
        let id = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let pulls = Pull::parse_list(caps.get(2).unwrap().as_str());
        Self { id, pulls }
    }

    fn is_possible(&self, requirements: &[(Colour, usize)]) -> bool {
        let max = self.max_found();
        requirements.iter().all(|(r_colour, r_count)| {
            let count = max.get(r_colour).unwrap();
            count <= r_count
        })
    }

    fn max_found(&self) -> HashMap<Colour, usize> {
        self.pulls
            .iter()
            .map(|p| p.max_found())
            .reduce(|l, r| {
                COLOURS
                    .iter()
                    .map(|c| (*c, cmp::max(*l.get(c).unwrap(), *r.get(c).unwrap())))
                    .collect()
            })
            .unwrap()
    }

    fn power(&self) -> usize {
        let max = self.max_found();
        max.iter()
            .map(|(_c, n)| *n)
            .reduce(|acc, n| acc * n)
            .unwrap()
    }
}

#[derive(Debug)]
struct Pull(Vec<(Colour, usize)>);

impl Pull {
    fn parse_list(input: &str) -> Vec<Self> {
        input.split("; ").map(|s| Self::parse(s)).collect()
    }

    fn parse(input: &str) -> Self {
        Self(input.split(", ").map(|s| Self::parse_item(s)).collect())
    }

    fn parse_item(input: &str) -> (Colour, usize) {
        lazy_static! {
            static ref ITEM_RE: Regex = Regex::new(r"\A(\d+) (\w+)\z").unwrap();
        }

        let caps = ITEM_RE.captures(input).unwrap();
        let count = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let colour = Colour::parse(caps.get(2).unwrap().as_str());
        (colour, count)
    }

    fn max_found(&self) -> HashMap<Colour, usize> {
        let mut out: HashMap<Colour, usize> = COLOURS.iter().map(|c| (*c, 0)).collect();
        for (c, n) in &self.0 {
            let curr = out.get(c).unwrap();
            out.insert(*c, cmp::max(*curr, *n));
        }
        out
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Colour {
    Blue,
    Green,
    Red,
}

const COLOURS: [Colour; 3] = [Colour::Blue, Colour::Green, Colour::Red];

impl Colour {
    fn parse(input: &str) -> Self {
        use Colour::*;

        match input {
            "blue" => Blue,
            "green" => Green,
            "red" => Red,
            _ => panic!("Unknown colour: {}", input),
        }
    }
}

fn part1(input: &str) -> usize {
    let games = Game::parse_list(input);
    let requirements = vec![(Colour::Blue, 14), (Colour::Green, 13), (Colour::Red, 12)];
    games
        .iter()
        .filter(|game| game.is_possible(&requirements))
        .map(|game| game.id)
        .sum()
}

fn part2(input: &str) -> usize {
    let games = Game::parse_list(input);
    games.iter().map(|game| game.power()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    static EXAMPLE: &str = indoc! {"
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    "};

    #[test]
    fn test_part1_example() {
        let result = part1(EXAMPLE);
        assert_eq!(result, 8);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file());
        assert_eq!(result, 2776);
    }

    #[test]
    fn test_part2_example() {
        let result = part2(EXAMPLE);
        assert_eq!(result, 2286);
    }

    #[test]
    fn test_part2_solution() {
        let result = part2(&read_input_file());
        assert_eq!(result, 68638);
    }
}
