pub mod astar;
pub mod grid;

use grid::Coordinate;
use grid::Grid;

use std::fs;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    // println!("part 2 result: {:?}", part2(&read_input_file()));
}

fn read_input_file() -> String {
    fs::read_to_string("input.txt").expect("Something went wrong reading the file")
}

#[derive(Debug)]
struct Map {
    grid: Grid<usize>,
    start: Coordinate,
    goal: Coordinate,
}

impl Map {
    fn parse(input: &str) -> Map {
        let risks: Vec<Vec<usize>> = input
            .lines()
            .map(|line| {
                line.chars()
                    .flat_map(|c| c.to_digit(10))
                    .map(|x| x as usize)
                    .collect()
            })
            .collect();
        let grid = Grid::new(risks);
        let start = grid.top_left();
        let goal = grid.bottom_right();

        Map {
            grid: grid,
            start: start,
            goal: goal,
        }
    }

    fn lowest_risk_path(&self) -> usize {
        let (_path, cost) = astar::solve(&self.grid, &self.start, &self.goal);
        cost
    }
}

fn part1(input: &str) -> usize {
    let map = Map::parse(input);
    println!("{:?}", map);
    map.lowest_risk_path()
}

// fn part2(input: &str) -> usize {
//     let map = Map::parse(input);
//     println!("{:?}", map);
//     map.execute()
// }

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    static EXAMPLE1: &str = indoc! {"
        1163751742
        1381373672
        2136511328
        3694931569
        7463417111
        1319128137
        1359912421
        3125421639
        1293138521
        2311944581
    "};

    #[test]
    fn test_part1_example1() {
        let result = part1(EXAMPLE1);
        assert_eq!(result, 40);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file());
        assert_eq!(result, 527);
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
