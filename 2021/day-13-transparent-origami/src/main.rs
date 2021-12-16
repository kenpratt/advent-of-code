use std::collections::HashSet;
use std::collections::VecDeque;
use std::fmt;
use std::fs;

use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    println!("part 1 result: {}", part1(&read_input_file()));
    println!("part 2 result:\n{}", part2(&read_input_file()));
}

fn read_input_file() -> String {
    fs::read_to_string("input.txt").expect("Something went wrong reading the file")
}

lazy_static! {
    static ref INSTRUCTION_RE: Regex = Regex::new(r"\Afold along ([xy])=(\d+)\z").unwrap();
}

#[derive(Debug)]
struct Paper {
    dots: HashSet<Dot>,
    instructions: VecDeque<Instruction>,
}

impl Paper {
    fn parse(input: &str) -> Paper {
        let parts: Vec<&str> = input.split("\n\n").collect();
        assert_eq!(parts.len(), 2);

        let dots = parts[0].lines().map(|line| Dot::parse(line)).collect();
        let instructions = parts[1]
            .lines()
            .map(|line| Instruction::parse(line))
            .collect();

        Paper {
            dots: dots,
            instructions: instructions,
        }
    }

    fn fold(&mut self) {
        let instruction = self.instructions.pop_front().unwrap();
        self.dots = instruction.apply(&self.dots);
    }

    fn apply_all_folds(&mut self) {
        while !self.instructions.is_empty() {
            self.fold();
        }
    }

    fn count_visible_dots(&self) -> usize {
        self.dots.len()
    }
}

impl fmt::Display for Paper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let max_x = self.dots.iter().map(|dot| dot.x).max().unwrap();
        let max_y = self.dots.iter().map(|dot| dot.y).max().unwrap();
        for y in 0..=max_y {
            for x in 0..=max_x {
                let dot = Dot { x: x, y: y };
                if self.dots.contains(&dot) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
struct Dot {
    x: u16,
    y: u16,
}

impl Dot {
    fn parse(input: &str) -> Dot {
        let parts: Vec<&str> = input.split(",").collect();
        assert_eq!(parts.len(), 2);
        let x = parts[0].parse::<u16>().unwrap();
        let y = parts[1].parse::<u16>().unwrap();
        Dot { x: x, y: y }
    }
}

#[derive(Debug)]
enum Axis {
    Horizontal,
    Vertical,
}

impl Axis {
    fn parse(input: &str) -> Axis {
        match input {
            "x" => Axis::Horizontal,
            "y" => Axis::Vertical,
            _ => panic!("Invalid axis: {:?}", input),
        }
    }
}

#[derive(Debug)]
struct Instruction {
    axis: Axis,
    point: u16,
}

impl Instruction {
    fn parse(input: &str) -> Instruction {
        let captures = INSTRUCTION_RE.captures(input).unwrap();
        let axis = Axis::parse(captures.get(1).unwrap().as_str());
        let point = captures.get(2).unwrap().as_str().parse::<u16>().unwrap();
        Instruction {
            axis: axis,
            point: point,
        }
    }

    fn apply(&self, dots: &HashSet<Dot>) -> HashSet<Dot> {
        dots.iter().map(|dot| self.apply_to_dot(dot)).collect()
    }

    fn apply_to_dot(&self, dot: &Dot) -> Dot {
        match self.axis {
            Axis::Horizontal => {
                if dot.x < self.point {
                    *dot
                } else if dot.x > self.point {
                    let distance_from_fold = dot.x - self.point;
                    let new_x = self.point - distance_from_fold;
                    Dot { x: new_x, y: dot.y }
                } else {
                    panic!("Dot is on fold line: {:?}, {:?}", self, dot)
                }
            }
            Axis::Vertical => {
                if dot.y < self.point {
                    *dot
                } else if dot.y > self.point {
                    let distance_from_fold = dot.y - self.point;
                    let new_y = self.point - distance_from_fold;
                    Dot { x: dot.x, y: new_y }
                } else {
                    panic!("Dot is on fold line: {:?}, {:?}", self, dot)
                }
            }
        }
    }
}

fn part1(input: &str) -> usize {
    let mut paper = Paper::parse(input);
    paper.fold();
    paper.count_visible_dots()
}

fn part2(input: &str) -> String {
    let mut paper = Paper::parse(input);
    paper.apply_all_folds();
    format!("{}", paper)
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    static EXAMPLE1: &str = indoc! {"
        6,10
        0,14
        9,10
        0,3
        10,4
        4,11
        6,0
        6,12
        4,1
        0,13
        10,12
        3,4
        3,0
        8,4
        1,10
        2,14
        8,10
        9,0

        fold along y=7
        fold along x=5
    "};

    #[test]
    fn test_part1_example1() {
        let result = part1(EXAMPLE1);
        assert_eq!(result, 17);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file());
        assert_eq!(result, 695);
    }

    static PART2_EXAMPLE1_OUTPUT: &str = indoc! {"
        #####
        #...#
        #...#
        #...#
        #####
    "};

    #[test]
    fn test_part2_example1() {
        let result = part2(EXAMPLE1);
        assert_eq!(result, PART2_EXAMPLE1_OUTPUT);
    }

    static PART2_SOLUTION_OUTPUT: &str = indoc! {"
        .##....##.####..##..#....#..#.###....##
        #..#....#....#.#..#.#....#..#.#..#....#
        #.......#...#..#....#....#..#.#..#....#
        #.##....#..#...#.##.#....#..#.###.....#
        #..#.#..#.#....#..#.#....#..#.#....#..#
        .###..##..####..###.####..##..#.....##.
    "};

    #[test]
    fn test_part2_solution() {
        let result = part2(&read_input_file());
        assert_eq!(result, PART2_SOLUTION_OUTPUT);
    }
}
