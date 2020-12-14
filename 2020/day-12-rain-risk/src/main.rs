use std::fs;

use regex::Regex;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    // println!("part 2 result: {:?}", part2(&read_input_file()));
}

fn read_input_file() -> String {
    return fs::read_to_string("input.txt").expect("Something went wrong reading the file");
}

#[derive(Debug)]
struct Ship {
    instructions: Vec<Instruction>,
    state: SimpleNavigation,
}

impl Ship {
    fn parse(input: &str) -> Ship {
        let instructions = input.lines().map(|line| Instruction::parse(line)).collect();
        return Ship {
            instructions: instructions,
            state: SimpleNavigation {
                x: 0,
                y: 0,
                angle: 0,
            },
        }
    }

    fn execute(&mut self) {
        for instruction in &self.instructions {
            self.state.apply_instruction(&instruction);
            println!("i: {:?}, after: {:?}", instruction, self.state);
        }
    }

    fn manhattan_distance(&self) -> usize {
        return self.state.manhattan_distance();
    }
}

#[derive(Debug)]
struct SimpleNavigation {
    x: isize,
    y: isize,
    angle: isize,
}

impl SimpleNavigation {
    fn apply_instruction(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::North(value) => {
                self.y += *value as isize;
            },
            Instruction::South(value) => {
                self.y -= *value as isize;
            },
            Instruction::East(value) => {
                self.x += *value as isize;
            },
            Instruction::West(value) => {
                self.x -= *value as isize;
            },
            Instruction::Left(value) => {
                self.angle += *value as isize;
            },
            Instruction::Right(value) => {
                self.angle -= *value as isize;
            },
            Instruction::Forward(value) => {
                let radians = (self.angle as f64).to_radians();
                let x = radians.cos() * (*value as f64);
                let y = radians.sin() * (*value as f64);
                self.x += x as isize;
                self.y += y as isize;
            },
        }
    }

    fn manhattan_distance(&self) -> usize {
        let dx = self.x.abs();
        let dy = self.y.abs();
        println!("x: {}, y: {}, d: {}", self.x, self.y, dx + dy);
        return (dx + dy) as usize;
    }    
}

#[derive(Debug)]
enum Instruction {
    North(usize),
    South(usize),
    East(usize),
    West(usize),
    Left(usize),
    Right(usize),
    Forward(usize),
}

impl Instruction {
    fn parse(input: &str) -> Instruction {
        let re = Regex::new(r"^([A-Z])(\d+)$").unwrap();
        let captures = re.captures(input).unwrap();
        let action = captures.get(1).unwrap().as_str();
        let value = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();

        return match action {
            "N" => Instruction::North(value),
            "S" => Instruction::South(value),
            "E" => Instruction::East(value),
            "W" => Instruction::West(value),
            "L" => Instruction::Left(value),
            "R" => Instruction::Right(value),
            "F" => Instruction::Forward(value),
            _ => panic!("Unknown action: {}", action),
        };
    }
}

fn part1(input: &str) -> usize {
    let mut program = Ship::parse(input);
    program.execute();
    return program.manhattan_distance();
}

// fn part2(input: &str) -> usize {
//     let data = Ship::parse(input);
//     return data.execute();
// }

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    static EXAMPLE1: &str = indoc! {"
        F10
        N3
        F7
        R90
        F11
    "};    

    static EXAMPLE1_MOD1: &str = indoc! {"
        F10
        N3
        F7
        L90
        F11
    "};  

    static EXAMPLE1_MOD2: &str = indoc! {"
        F10
        N3
        F7
        L180
        F11
    "};

    #[test]
    fn test_part1_example1() {
        let result = part1(EXAMPLE1);
        assert_eq!(result, 25);

        assert_eq!(part1(EXAMPLE1_MOD1), 31);
        assert_eq!(part1(EXAMPLE1_MOD2), 9);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file());
        assert_eq!(result, 1710);
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