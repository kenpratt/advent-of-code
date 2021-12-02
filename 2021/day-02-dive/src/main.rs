use std::fs;

use regex::Regex;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    // println!("part 2 result: {:?}", part2(&read_input_file()));
}

fn read_input_file() -> String {
    fs::read_to_string("input.txt").expect("Something went wrong reading the file")
}

fn parse(input: &str) -> Vec<Command> {
    input.lines().map(|line| Command::parse(line)).collect()
}

#[derive(Debug)]
enum Command {
    Forward(usize),
    Down(usize),
    Up(usize),
}

impl Command {
    fn parse(input: &str) -> Command {
        let re = Regex::new(r"^(\w+) (\d+)$").unwrap();
        let captures = re.captures(input).unwrap();
        let direction = captures.get(1).unwrap().as_str();
        let amount = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();
        match direction {
            "forward" => Command::Forward(amount),
            "down" => Command::Down(amount),
            "up" => Command::Up(amount),
            _ => panic!("Bad input: {}", input)
        }
    }
}

#[derive(Debug)]
struct Submarine {
    position: usize,
    depth: usize,
}

impl Submarine {
    fn new() -> Submarine {
        Submarine {
            position: 0,
            depth: 0,
        }
    }

    fn navigate(&mut self, commands: &[Command]) {
        for command in commands {
            self.step(command);
        }
    }

    fn step(&mut self, command: &Command) {
        match command {
            Command::Forward(amount) => self.position += amount,
            Command::Down(amount) => self.depth += amount,
            Command::Up(amount) => self.depth -= amount,
        }
    }
}

fn part1(input: &str) -> usize {
    let commands = parse(input);
    let mut sub = Submarine::new();
    sub.navigate(&commands);
    sub.position * sub.depth
}

// fn part2(input: &str) -> usize {
//     let data = Data::parse(input);
//     data.execute()
// }

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    static EXAMPLE1: &str = indoc! {"
        forward 5
        down 5
        forward 8
        up 3
        down 8
        forward 2
    "};    

    #[test]
    fn test_part1_example1() {
        let result = part1(EXAMPLE1);
        assert_eq!(result, 150);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file());
        assert_eq!(result, 1990000);
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