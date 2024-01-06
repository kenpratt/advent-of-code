pub mod grid;

use std::{collections::BTreeSet, fs};

use grid::*;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
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
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Terrain {
    Path,
    Forest,
    Slope(Direction),
}

impl Terrain {
    fn parse(input: &char) -> Self {
        use Direction::*;
        use Terrain::*;

        match input {
            '.' => Path,
            '#' => Forest,
            '^' => Slope(North),
            '>' => Slope(East),
            'v' => Slope(South),
            '<' => Slope(West),
            _ => panic!("Unexpected terrain: {:?}", input),
        }
    }

    fn next_directions(&self) -> Vec<Direction> {
        use Terrain::*;

        match self {
            Path => ALL_DIRECTIONS.to_vec(),
            Forest => panic!("Shouldn't be on a forest tile"),
            Slope(d) => vec![*d],
        }
    }
}

struct Solver<'a> {
    grid: &'a Grid<Terrain>,
    end: Coord,
}

impl<'a> Solver<'a> {
    fn run(grid: &'a Grid<Terrain>) -> usize {
        // start on path in top row
        let start = (0..grid.width)
            .map(|x| Coord::new(x, 0))
            .find(|c| grid.value(c) == &Terrain::Path)
            .unwrap();

        // end on path in bottom row
        let end = (0..grid.width)
            .map(|x| Coord::new(x, grid.height - 1))
            .find(|c| grid.value(c) == &Terrain::Path)
            .unwrap();

        let mut solver = Solver { grid, end };
        let initial = SolutionState::new(start);

        solver.solve(initial)
    }

    fn solve(&mut self, initial: SolutionState) -> usize {
        let mut open_set = vec![initial];
        let mut at_goal = vec![];

        while let Some(curr) = open_set.pop() {
            if curr.pos == self.end {
                at_goal.push(curr);
            } else {
                let mut next = curr.next(self.grid);
                open_set.append(&mut next);
            }
        }

        let tiles = at_goal.into_iter().map(|s| s.visited.len()).max().unwrap();
        tiles - 1 // count steps
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct SolutionState {
    pos: Coord,
    visited: BTreeSet<Coord>,
}

impl SolutionState {
    fn new(start: Coord) -> Self {
        let visited = BTreeSet::from([start.clone()]);
        let pos = start;
        Self { pos, visited }
    }

    fn next(&self, grid: &Grid<Terrain>) -> Vec<SolutionState> {
        let curr_terrain = grid.value(&self.pos);
        curr_terrain
            .next_directions()
            .into_iter()
            .filter_map(|dir| grid.neighbour(&self.pos, &dir))
            .filter(|dest| !self.visited.contains(dest))
            .filter(|dest| grid.value(dest) != &Terrain::Forest)
            .map(|dest| self.move_to(dest))
            .collect()
    }

    fn move_to(&self, dest: Coord) -> Self {
        let mut visited = self.visited.clone();
        visited.insert(dest.clone());

        let pos = dest;
        Self { pos, visited }
    }
}

fn part1(input: &str) -> usize {
    let map = Map::parse(input);
    Solver::run(&map.grid)
}

// fn part2(input: &str) -> usize {
//     let items = Data::parse(input);
//     0
// }

#[cfg(test)]
mod tests {
    use super::*;

    fn read_example_file() -> String {
        fs::read_to_string("example.txt").expect("Something went wrong reading the file")
    }

    #[test]
    fn test_part1_example() {
        let result = part1(&read_example_file());
        assert_eq!(result, 94);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file());
        assert_eq!(result, 1966);
    }

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
