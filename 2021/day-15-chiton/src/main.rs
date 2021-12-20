pub mod astar;
pub mod grid;

use grid::Coordinate;
use grid::Grid;

use std::fs;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    println!("part 2 result: {:?}", part2(&read_input_file()));
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
    fn parse(input: &str) -> Vec<Vec<usize>> {
        input
            .lines()
            .map(|line| {
                line.chars()
                    .flat_map(|c| c.to_digit(10))
                    .map(|x| x as usize)
                    .collect()
            })
            .collect()
    }

    fn new(risks: Vec<Vec<usize>>) -> Map {
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
    let map = Map::new(Map::parse(input));
    map.lowest_risk_path()
}

fn part2(input: &str) -> usize {
    let risks = Map::parse(input);
    let expanded_risks = expand_risks(&risks, 5);
    let map = Map::new(expanded_risks);
    map.lowest_risk_path()
}

fn expand_risks(input: &Vec<Vec<usize>>, n: usize) -> Vec<Vec<usize>> {
    let input_height = input.len();
    let input_width = input[0].len();
    let output_height = input_height * n;
    let output_width = input_width * n;
    (0..output_height)
        .map(|y| {
            (0..output_width)
                .map(|x| {
                    let input_x = x % input_width;
                    let input_y = y % input_height;
                    let window_x = x / input_width;
                    let window_y = y / input_height;
                    let input_value = input[input_y][input_x];
                    let offset = window_x + window_y;
                    let output_value = input_value + offset;
                    clamp_between_1_and_9(output_value)
                })
                .collect()
        })
        .collect()
}

fn clamp_between_1_and_9(value: usize) -> usize {
    let mut result = value;
    while result > 9 {
        result -= 9;
    }
    result
}

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

    #[test]
    fn test_part2_example1() {
        let result = part2(EXAMPLE1);
        assert_eq!(result, 315);
    }

    #[test]
    fn test_part2_solution() {
        let result = part2(&read_input_file());
        assert_eq!(result, 2887);
    }
}
