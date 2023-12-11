use std::{collections::HashSet, fs};

use itertools::Itertools;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    println!("part 2 result: {:?}", part2(&read_input_file(), 1000000));
}

fn read_input_file() -> String {
    fs::read_to_string("input.txt").expect("Something went wrong reading the file")
}

#[derive(Debug)]
struct Universe {
    galaxies: HashSet<Coord>,
    empty_cols: HashSet<usize>,
    empty_rows: HashSet<usize>,
}

impl Universe {
    fn parse(input: &str) -> Self {
        let mut galaxies: HashSet<Coord> = HashSet::new();

        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    galaxies.insert(Coord::new(x, y));
                }
            }
        }

        let width = input.lines().next().unwrap().len();
        let height = input.lines().count();

        let empty_cols: HashSet<usize> = (0..width)
            .filter(|x| !galaxies.iter().any(|g| g.x == *x))
            .collect();
        let empty_rows: HashSet<usize> = (0..height)
            .filter(|y| !galaxies.iter().any(|g| g.y == *y))
            .collect();

        Self {
            galaxies,
            empty_cols,
            empty_rows,
        }
    }

    fn distance(&self, from: &Coord, to: &Coord, expansion_factor: &usize) -> usize {
        let dx = if from.x <= to.x {
            Self::linear_distance(from.x, to.x, &self.empty_cols, expansion_factor)
        } else {
            Self::linear_distance(to.x, from.x, &self.empty_cols, expansion_factor)
        };

        let dy = if from.y <= to.y {
            Self::linear_distance(from.y, to.y, &self.empty_rows, expansion_factor)
        } else {
            Self::linear_distance(to.y, from.y, &self.empty_rows, expansion_factor)
        };

        dx + dy
    }

    fn linear_distance(
        from: usize,
        to: usize,
        empty: &HashSet<usize>,
        expansion_factor: &usize,
    ) -> usize {
        assert!(from <= to);
        let base = to - from;
        let extra = ((from + 1)..=to).filter(|x| empty.contains(x)).count();
        base + extra * (expansion_factor - 1)
    }

    fn sum_distance_between_pairs(&self, expansion_factor: usize) -> usize {
        self.galaxies
            .iter()
            .combinations(2)
            .map(|v| self.distance(v[0], v[1], &expansion_factor))
            .sum()
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
    let universe = Universe::parse(input);
    universe.sum_distance_between_pairs(2)
}

fn part2(input: &str, expansion_factor: usize) -> usize {
    let universe = Universe::parse(input);
    universe.sum_distance_between_pairs(expansion_factor)
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    static EXAMPLE: &str = indoc! {"
        ...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#.....
    "};

    #[test]
    fn test_part1_example() {
        let result = part1(EXAMPLE);
        assert_eq!(result, 374);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file());
        assert_eq!(result, 9799681);
    }

    #[test]
    fn test_part2_example_with_10() {
        let result = part2(EXAMPLE, 10);
        assert_eq!(result, 1030);
    }

    #[test]
    fn test_part2_example_with_100() {
        let result = part2(EXAMPLE, 100);
        assert_eq!(result, 8410);
    }

    #[test]
    fn test_part2_solution() {
        let result = part2(&read_input_file(), 1000000);
        assert_eq!(result, 513171773355);
    }
}
