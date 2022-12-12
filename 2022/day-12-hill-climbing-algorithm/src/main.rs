pub mod astar;
pub mod grid;

use astar::AStarInterface;
use grid::*;

use std::fs;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    println!("part 2 result: {:?}", part2(&read_input_file()));
}

fn read_input_file() -> String {
    fs::read_to_string("input.txt").expect("Something went wrong reading the file")
}

#[derive(Debug)]
struct Solver {
    map: Grid<char>,
    start: Coordinate,
    end: Coordinate,
}

impl Solver {
    fn parse(input: &str) -> Self {
        let mut map = Grid::new(input.lines().map(|line| line.chars().collect()).collect());

        let start = Self::find_and_replace(&mut map, 'S', 'a');
        let end = Self::find_and_replace(&mut map, 'E', 'z');

        Self { map, start, end }
    }

    fn find_and_replace(map: &mut Grid<char>, find: char, replace: char) -> Coordinate {
        let cell = map.iter_mut().find(|cell| cell.value == find).unwrap();
        cell.value = replace;
        cell.position
    }

    fn run(&self) -> usize {
        let (_path, length) = self.shortest_path(&self.start, true).unwrap();
        length
    }

    fn run_with_multiple_start_positions(&self) -> usize {
        let start_positions = self
            .map
            .iter()
            .filter(|c| c.value == 'a')
            .map(|c| c.position);
        start_positions
            .map(|pos| self.shortest_path(&pos, true))
            .filter_map(|s| s) // filter out bad solutions
            .map(|(_path, length)| length)
            .min()
            .unwrap()
    }
}

impl AStarInterface<Coordinate> for Solver {
    fn at_goal(&self, node: &Coordinate) -> bool {
        node == &self.end
    }

    fn heuristic(&self, from: &Coordinate) -> usize {
        from.manhattan_distance(&self.end)
    }

    fn neighbours(&self, from: &Coordinate) -> Vec<(Coordinate, usize)> {
        let h = *self.map.value(from) as isize;
        self.map
            .neighbours(from)
            .iter()
            .filter(|n| (*self.map.value(n) as isize - h) <= 1)
            .map(|n| (*n, 1))
            .collect()
    }
}

fn part1(input: &str) -> usize {
    let solver = Solver::parse(input);
    solver.run()
}

fn part2(input: &str) -> usize {
    let solver = Solver::parse(input);
    solver.run_with_multiple_start_positions()
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    static EXAMPLE1: &str = indoc! {"
        Sabqponm
        abcryxxl
        accszExk
        acctuvwj
        abdefghi
    "};

    #[test]
    fn test_part1_example1() {
        let result = part1(EXAMPLE1);
        assert_eq!(result, 31);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file());
        assert_eq!(result, 472);
    }

    #[test]
    fn test_part2_example1() {
        let result = part2(EXAMPLE1);
        assert_eq!(result, 29);
    }

    #[test]
    fn test_part2_solution() {
        let result = part2(&read_input_file());
        assert_eq!(result, 465);
    }
}
