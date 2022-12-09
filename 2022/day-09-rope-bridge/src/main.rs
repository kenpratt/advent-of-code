use std::collections::HashSet;
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
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn parse(input: &str) -> Self {
        use Direction::*;

        match input {
            "L" => Left,
            "R" => Right,
            "U" => Up,
            "D" => Down,
            _ => panic!("Bad direction input: {}", input),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Coord {
    x: isize,
    y: isize,
}

impl Coord {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn shift_in_direction(&self, direction: &Direction, distance: isize) -> Self {
        use Direction::*;

        match direction {
            Left => Coord::new(self.x - distance, self.y),
            Right => Coord::new(self.x + distance, self.y),
            Up => Coord::new(self.x, self.y + distance),
            Down => Coord::new(self.x, self.y - distance),
        }
    }
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    distance: usize,
}

impl Instruction {
    fn parse_instructions(input: &str) -> Vec<Self> {
        input.lines().map(|line| Self::parse(line)).collect()
    }

    fn parse(input: &str) -> Self {
        let parts: Vec<&str> = input.split(' ').collect();
        assert_eq!(parts.len(), 2);

        let direction = Direction::parse(parts[0]);
        let distance = parts[1].parse::<usize>().unwrap();

        Self {
            direction,
            distance,
        }
    }
}

#[derive(Debug)]
struct Rope {
    head: Coord,
    tail: Coord,
    tail_visited: HashSet<Coord>,
}

impl Rope {
    fn new() -> Self {
        Self {
            head: Coord::new(0, 0),
            tail: Coord::new(0, 0),
            tail_visited: HashSet::new(),
        }
    }

    fn execute_instructions(&mut self, instructions: &[Instruction]) {
        for instruction in instructions {
            self.execute_instruction(instruction);
        }
    }

    fn execute_instruction(&mut self, instruction: &Instruction) {
        println!("instruction: {:?}", instruction);
        for _ in 0..instruction.distance {
            self.move_in_direction(&instruction.direction);
        }
    }

    fn move_in_direction(&mut self, direction: &Direction) {
        println!("  direction: {:?}", direction);
        self.head = self.head.shift_in_direction(direction, 1);
        self.tail = Self::follow_head(&self.head, &self.tail);
        self.tail_visited.insert(self.tail);
        println!("    head: {:?}", self.head);
        println!("    tail: {:?}", self.tail);
    }

    fn follow_head(head: &Coord, tail: &Coord) -> Coord {
        let dx = head.x - tail.x;
        let dy = head.y - tail.y;

        if dx.abs() <= 1 && dy.abs() <= 1 {
            // head & tail are touching, don't need to move
            *tail
        } else if dx == 0 {
            // same column but different row, shift up/down
            Coord::new(tail.x, tail.y + dy.signum())
        } else if dy == 0 {
            // same row but different column, shift left/right
            Coord::new(tail.x + dx.signum(), tail.y)
        } else {
            // not touching & different row and column, do a diagonal catch-up move
            Coord::new(tail.x + dx.signum(), tail.y + dy.signum())
        }
    }
}

fn part1(input: &str) -> usize {
    let instructions = Instruction::parse_instructions(input);
    println!("{:?}", instructions);

    let mut rope = Rope::new();
    rope.execute_instructions(&instructions);

    rope.tail_visited.len()
}

// fn part2(input: &str) -> usize {
//     let data = Instruction::parse(input);
//     println!("{:?}", data);
//     data.execute()
// }

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    static EXAMPLE1: &str = indoc! {"
        R 4
        U 4
        L 3
        D 1
        R 4
        D 1
        L 5
        R 2
    "};

    #[test]
    fn test_part1_example1() {
        let result = part1(EXAMPLE1);
        assert_eq!(result, 13);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file());
        assert_eq!(result, 6406);
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
