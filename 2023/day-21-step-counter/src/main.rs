pub mod grid;

use std::{collections::HashSet, fs};

use grid::*;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file(), 64));
    // println!("part 2 result: {:?}", part2(&read_input_file()));
}

fn read_input_file() -> String {
    fs::read_to_string("input.txt").expect("Something went wrong reading the file")
}

#[derive(Debug)]
struct Map {
    grid: Grid<Terrain>,
}

impl Map {
    fn parse(input: &str) -> Self {
        let grid = Grid::parse(input, |c| Terrain::parse(c));
        Self { grid }
    }

    fn starting_position(&self) -> &Coord {
        self.grid
            .cells
            .iter()
            .find(|(_pos, terrain)| **terrain == Terrain::Start)
            .map(|(pos, _terrain)| pos)
            .unwrap()
    }

    fn count_plots_within_reach(&self, steps: usize) -> usize {
        let start = self.starting_position();

        // locations to expand during next step
        let mut frontier: HashSet<Coord> = HashSet::new();
        frontier.insert(*start);

        // rock locations
        let rocks: HashSet<&Coord> = self
            .grid
            .cells
            .iter()
            .filter(|(_pos, terrain)| **terrain == Terrain::Rock)
            .map(|(pos, _terrain)| pos)
            .collect();

        // take the steps!
        for _step in 1..=steps {
            // explore the whole frontier, and build a new one
            let mut new_frontier = HashSet::new();

            for to_expand in frontier {
                for neighbour in self.grid.neighbours(&to_expand) {
                    if !rocks.contains(&neighbour) {
                        new_frontier.insert(neighbour);
                    }
                }
            }

            frontier = new_frontier;
        }

        frontier.len()
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Terrain {
    Garden,
    Rock,
    Start,
}

impl Terrain {
    fn parse(c: &char) -> Self {
        match c {
            '.' => Terrain::Garden,
            '#' => Terrain::Rock,
            'S' => Terrain::Start,
            _ => panic!("Unexpected terrain: {:?}", c),
        }
    }
}

fn part1(input: &str, steps: usize) -> usize {
    let map = Map::parse(input);
    map.count_plots_within_reach(steps)
}

// fn part2(input: &str) -> usize {
//     let items = Data::parse(input);
//     dbg!(&items);
//     0
// }

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    static EXAMPLE: &str = indoc! {"
        ...........
        .....###.#.
        .###.##..#.
        ..#.#...#..
        ....#.#....
        .##..S####.
        .##..#...#.
        .......##..
        .##.#.####.
        .##..##.##.
        ...........
    "};

    #[test]
    fn test_part1_example() {
        let result = part1(EXAMPLE, 6);
        assert_eq!(result, 16);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file(), 64);
        assert_eq!(result, 3768);
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
