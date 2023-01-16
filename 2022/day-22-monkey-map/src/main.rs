pub mod grid;

use grid::*;

use std::fs;

use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    // println!("part 2 result: {:?}", part2(&read_input_file()));
}

fn read_file(filename: &str) -> String {
    fs::read_to_string(filename).expect("Something went wrong reading the file")
}

fn read_input_file() -> String {
    read_file("input.txt")
}

fn parse(input: &str) -> (Map, Vec<Instruction>) {
    let mut iter = input.split("\n\n");
    let map = Map::parse(iter.next().unwrap());
    let instructions = Instruction::parse_list(iter.next().unwrap());
    assert_eq!(None, iter.next());
    (map, instructions)
}

#[derive(Debug)]
enum Instruction {
    Move(u8),
    Turn(Rotation),
}

impl Instruction {
    fn parse_list(input: &str) -> Vec<Self> {
        use Instruction::*;

        lazy_static! {
            static ref INSTRUCTION_RE: Regex = Regex::new(r"((\d+)|([A-Z]))").unwrap();
        }

        INSTRUCTION_RE
            .captures_iter(input)
            .map(|caps| match (caps.get(2), caps.get(3)) {
                (Some(s), None) => Move(s.as_str().parse::<u8>().unwrap()),
                (None, Some(s)) => Turn(Rotation::parse(s.as_str())),
                _ => panic!("Unreachable"),
            })
            .collect()
    }
}

#[derive(Debug)]
enum Rotation {
    Clockwise,
    Counterclockwise,
}

impl Rotation {
    fn parse(input: &str) -> Self {
        use Rotation::*;
        match input {
            "L" => Counterclockwise,
            "R" => Clockwise,
            _ => panic!("Bad direction: {}", input),
        }
    }
}

#[derive(Debug)]
struct Map(Grid<Tile>);

impl Map {
    fn parse(input: &str) -> Self {
        let mut rows: Vec<Vec<Tile>> = input
            .lines()
            .map(|line| line.chars().map(|c| Tile::parse(&c)).collect())
            .collect();

        // make the grid rectangular, filling with more empty space on the
        // right side
        let width = rows.iter().map(|r| r.len()).max().unwrap();
        for row in &mut rows {
            row.resize(width, Tile::Empty);
        }

        let grid = Grid::new(rows);
        Self(grid)
    }
}

#[derive(Copy, Clone, Debug)]
enum Tile {
    Empty,
    Open,
    Wall,
}

impl Tile {
    fn parse(input: &char) -> Self {
        use Tile::*;
        match input {
            ' ' => Empty,
            '.' => Open,
            '#' => Wall,
            _ => panic!("Bad tile: {}", input),
        }
    }
}

fn part1(input: &str) -> usize {
    let (map, instructions) = parse(input);
    dbg!(&map);
    dbg!(&instructions);
    0
}

// fn part2(input: &str) -> usize {
//     let items = Data::parse(input);
//     dbg!(&items);
//     0
// }

#[cfg(test)]
mod tests {
    use super::*;

    fn read_example_file() -> String {
        read_file("example.txt")
    }

    #[test]
    fn test_part1_example() {
        let result = part1(&read_example_file());
        assert_eq!(result, 0);
    }

    // #[test]
    // fn test_part1_solution() {
    //     let result = part1(&read_input_file());
    //     assert_eq!(result, 0);
    // }

    // #[test]
    // fn test_part2_example() {
    //     let result = part2(&read_example_file());
    //     assert_eq!(result, 0);
    // }

    // #[test]
    // fn test_part2_solution() {
    //     let result = part2(&read_input_file());
    //     assert_eq!(result, 0);
    // }
}
