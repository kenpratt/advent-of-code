use std::fs;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    println!("part 2 result: {:?}", part2(&read_input_file()));
}

fn read_input_file() -> String {
    fs::read_to_string("input.txt").expect("Something went wrong reading the file")
}

#[derive(Debug)]
enum ParsingMode {
    Response,
    Outcome,
}

#[derive(Debug)]
struct Strategy {
    steps: Vec<StrategyStep>,
}

impl Strategy {
    fn parse(input: &str, parsing_mode: &ParsingMode) -> Strategy {
        let steps = input
            .lines()
            .map(|line| StrategyStep::parse(line, parsing_mode))
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
    outcome: Outcome,
}

impl StrategyStep {
    fn parse(input: &str, parsing_mode: &ParsingMode) -> StrategyStep {
        let pieces: Vec<&str> = input.split(" ").collect();
        assert_eq!(pieces.len(), 2);

        let opponent = Shape::parse_opponent_move(pieces[0]);

        let (response, outcome) = match parsing_mode {
            // part 1
            &ParsingMode::Response => {
                let response = Shape::parse_response_move(pieces[1]);
                let outcome = response.play(&opponent);
                (response, outcome)
            }

            // part 2
            &ParsingMode::Outcome => {
                let outcome = Outcome::parse(pieces[1]);
                let response = outcome.infer_response(&opponent);
                (response, outcome)
            }
        };

        StrategyStep {
            opponent: opponent,
            response: response,
            outcome: outcome,
        }
    }

    fn score(&self) -> usize {
        println!(
            "score: {:?} => {:?} + {:?}",
            self,
            self.outcome.score(),
            self.response.score()
        );
        self.outcome.score() + self.response.score()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn parse_opponent_move(input: &str) -> Shape {
        use Shape::*;

        match input {
            "A" => Rock,
            "B" => Paper,
            "C" => Scissors,
            _ => panic!("Bad input for opponent move: {}", input),
        }
    }

    fn parse_response_move(input: &str) -> Shape {
        use Shape::*;

        match input {
            "X" => Rock,
            "Y" => Paper,
            "Z" => Scissors,
            _ => panic!("Bad input for response move: {}", input),
        }
    }

    fn wins_against(&self) -> Shape {
        use Shape::*;

        match self {
            Rock => Scissors,
            Paper => Rock,
            Scissors => Paper,
        }
    }

    fn loses_to(&self) -> Shape {
        use Shape::*;

        match self {
            Rock => Paper,
            Paper => Scissors,
            Scissors => Rock,
        }
    }

    fn play(&self, other: &Shape) -> Outcome {
        use Outcome::*;

        if self == other {
            Draw
        } else if &self.wins_against() == other {
            Win
        } else {
            Loss
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
    fn parse(input: &str) -> Outcome {
        use Outcome::*;

        match input {
            "X" => Loss,
            "Y" => Draw,
            "Z" => Win,
            _ => panic!("Bad input for outcome: {}", input),
        }
    }

    fn infer_response(&self, opponent: &Shape) -> Shape {
        use Outcome::*;

        match self {
            Loss => opponent.wins_against(),
            Draw => *opponent,
            Win => opponent.loses_to(),
        }
    }

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
    let strategy = Strategy::parse(input, &ParsingMode::Response);
    println!("{:?}", strategy);
    strategy.total_score()
}

fn part2(input: &str) -> usize {
    let strategy = Strategy::parse(input, &ParsingMode::Outcome);
    println!("{:?}", strategy);
    strategy.total_score()
}

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

    #[test]
    fn test_part2_example1() {
        let result = part2(EXAMPLE1);
        assert_eq!(result, 12);
    }

    #[test]
    fn test_part2_solution() {
        let result = part2(&read_input_file());
        assert_eq!(result, 8295);
    }
}
