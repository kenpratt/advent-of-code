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

    fn distance(&self, from: &Coord, to: &Coord) -> usize {
        let dx = if from.x <= to.x {
            self.x_distance(from.x, to.x)
        } else {
            self.x_distance(to.x, from.x)
        };

        let dy = if from.y <= to.y {
            self.y_distance(from.y, to.y)
        } else {
            self.y_distance(to.y, from.y)
        };

        dx + dy
    }

    fn x_distance(&self, from_x: usize, to_x: usize) -> usize {
        let base = to_x - from_x;
        let extra = (from_x..=to_x)
            .filter(|x| self.empty_cols.contains(x))
            .count();
        base + extra
    }

    fn y_distance(&self, from_y: usize, to_y: usize) -> usize {
        let base = to_y - from_y;
        let extra = (from_y..=to_y)
            .filter(|y| self.empty_rows.contains(y))
            .count();
        base + extra
    }

    fn sum_distance_between_pairs(&self) -> usize {
        self.galaxies
            .iter()
            .combinations(2)
            .map(|v| self.distance(v[0], v[1]))
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
    universe.sum_distance_between_pairs()
}

// fn part2(input: &str) -> usize {
//     let Universes = Data::parse(input);
//     dbg!(&Universes);
//     0
// }

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
