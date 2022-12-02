use std::fs;

// use itertools::Itertools;
// use lazy_static::lazy_static;
// use regex::Regex;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    // println!("part 2 result: {:?}", part2(&read_input_file()));
}

fn read_input_file() -> String {
    fs::read_to_string("input.txt").expect("Something went wrong reading the file")
}

#[derive(Debug)]
struct Strategy {
    steps: Vec<StrategyStep>,
}

impl Strategy {
    fn parse(input: &str) -> Strategy {
        let steps = input
            .lines()
            .map(|line| StrategyStep::parse(line))
            .collect();
        Strategy { steps: steps }
    }

    fn total_score(&self) -> usize {
        self.steps.iter().map(|step| step.score()).sum()
    }
}

#[derive(Debug)]
struct StrategyStep {
    opponent: Shape,
    response: Shape,
}

impl StrategyStep {
    fn parse(input: &str) -> StrategyStep {
        let pieces: Vec<&str> = input.split(" ").collect();
        assert_eq!(pieces.len(), 2);
        let opponent = Self::parse_opponent(pieces[0]);
        let response = Self::parse_response(pieces[1]);
        StrategyStep {
            opponent: opponent,
            response: response,
        }
    }

    fn parse_opponent(input: &str) -> Shape {
        match input {
            "A" => Shape::Rock,
            "B" => Shape::Paper,
            "C" => Shape::Scissors,
            _ => panic!("Bad input for opponent move: {}", input),
        }
    }

    fn parse_response(input: &str) -> Shape {
        match input {
            "X" => Shape::Rock,
            "Y" => Shape::Paper,
            "Z" => Shape::Scissors,
            _ => panic!("Bad input for response move: {}", input),
        }
    }

    fn play(&self) -> Outcome {
        self.response.play(&self.opponent)
    }

    fn score(&self) -> usize {
        let outcome = self.play();
        println!(
            "score: {:?} => {:?}, {:?} + {:?}",
            self,
            outcome,
            outcome.score(),
            self.response.score()
        );
        outcome.score() + self.response.score()
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn play(&self, other: &Shape) -> Outcome {
        use Outcome::*;
        use Shape::*;

        match (self, other) {
            (x, y) if x == y => Draw,
            (Paper, Rock) => Win,
            (Scissors, Paper) => Win,
            (Rock, Scissors) => Win,
            _ => Loss,
        }
    }

    fn score(&self) -> usize {
        use Shape::*;

        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }
}

#[derive(Debug)]
enum Outcome {
    Loss,
    Draw,
    Win,
}

impl Outcome {
    fn score(&self) -> usize {
        use Outcome::*;

        match self {
            Loss => 0,
            Draw => 3,
            Win => 6,
        }
    }
}

fn part1(input: &str) -> usize {
    let strategy = Strategy::parse(input);
    println!("{:?}", strategy);
    strategy.total_score()
}

// fn part2(input: &str) -> usize {
//     let strategy = Strategy::parse(input);
//     println!("{:?}", strategy);
//     strategy.execute()
// }

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    static EXAMPLE1: &str = indoc! {"
        A Y
        B X
        C Z
    "};

    #[test]
    fn test_part1_example1() {
        let result = part1(EXAMPLE1);
        assert_eq!(result, 15);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file());
        assert_eq!(result, 11150);
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
