use std::{collections::HashSet, fs};

use itertools::Itertools;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    // println!("part 2 result: {:?}", part2(&read_input_file()));
}

fn read_input_file() -> String {
    fs::read_to_string("input.txt").expect("Something went wrong reading the file")
}

#[derive(Debug)]
struct Platform {
    width: usize,
    height: usize,
    rounded_rocks: HashSet<Coord>,
    cube_rocks: HashSet<Coord>,
}

impl Platform {
    fn parse(input: &str) -> Self {
        let width = input.lines().next().unwrap().len();
        let height = input.lines().count();

        let mut rounded_rocks = HashSet::new();
        let mut cube_rocks = HashSet::new();

        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let pos = Coord::new(x, y);
                match c {
                    'O' => rounded_rocks.insert(pos),
                    '#' => cube_rocks.insert(pos),
                    '.' => false,
                    _ => panic!("Unexpected char: {:?}", c),
                };
            }
        }

        Self {
            width,
            height,
            rounded_rocks,
            cube_rocks,
        }
    }

    fn load_score_tilted_north(&self) -> usize {
        (0..self.width).map(|x| self.load_score_for_column(x)).sum()
    }

    fn load_score_for_column(&self, x: usize) -> usize {
        // find blockages
        let mut blockages_y: Vec<usize> = self
            .cube_rocks
            .iter()
            .filter(|pos| pos.x == x)
            .map(|pos| pos.y + 1)
            .collect();

        // get all the stops, and reverse to largest first
        let mut stops_y: Vec<usize> = vec![0];
        stops_y.append(&mut blockages_y);
        stops_y.sort();
        stops_y.reverse();

        // get rock counts at each stop
        let counts = self
            .rounded_rocks
            .iter()
            .filter(|pos| pos.x == x)
            .counts_by(|pos| stops_y.iter().find(|y| **y <= pos.y).unwrap());

        let score = counts
            .into_iter()
            .map(|(y, num)| self.load_score_for_blockage(*y, num))
            .sum();
        score
    }

    fn load_score_for_blockage(&self, y: usize, num: usize) -> usize {
        (0..num).map(|offset| self.height - (y + offset)).sum()
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

fn part1(input: &str) -> usize {
    let platform = Platform::parse(input);
    platform.load_score_tilted_north()
}

// fn part2(input: &str) -> usize {
//     let platforms = Data::parse(input);
//     dbg!(&platforms);
//     0
// }

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    static EXAMPLE: &str = indoc! {"
        O....#....
        O.OO#....#
        .....##...
        OO.#O....O
        .O.....O#.
        O.#..O.#.#
        ..O..#O..O
        .......O..
        #....###..
        #OO..#....
    "};

    #[test]
    fn test_part1_example() {
        let result = part1(EXAMPLE);
        assert_eq!(result, 136);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file());
        assert_eq!(result, 105623);
    }

    // #[test]
    // fn test_part2_example() {
    //     let result = part2(EXAMPLE);
    //     assert_eq!(result, 0);
    // }

    // #[test]
    // fn test_part2_solution() {
    //     let result = part2(&read_input_file());
    //     assert_eq!(result, 0);
    // }
}
