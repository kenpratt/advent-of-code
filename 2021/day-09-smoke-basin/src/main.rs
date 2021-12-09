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
struct HeightMap {
    grid: Grid<u8>,
}

impl HeightMap {
    fn parse(input: &str) -> HeightMap {
        let heights: Vec<Vec<u8>> = input
            .lines()
            .map(|line| HeightMap::parse_line(line))
            .collect();
        let grid = Grid::new(heights);
        HeightMap { grid: grid }
    }

    fn parse_line(line: &str) -> Vec<u8> {
        line.chars()
            .flat_map(|c| c.to_digit(10))
            .map(|v| v as u8)
            .collect()
    }

    fn low_points(&self) -> Vec<(Coordinate, &u8)> {
        self.grid
            .iter()
            .filter(|(pos, val)| {
                self.grid
                    .neighbours(pos)
                    .iter()
                    .all(|npos| val < &self.grid.value(npos))
            })
            .collect()
    }
}

fn part1(input: &str) -> usize {
    let map = HeightMap::parse(input);
    println!("map: {:?}", map);
    let low_points = map.low_points();
    println!("low_points: {:?}", low_points);
    low_points
        .into_iter()
        .map(|(_pos, value)| *value as usize + 1)
        .sum()
}

// fn part2(input: &str) -> usize {
//     let map = HeightMap::parse(input);
//     println!("{:?}", map);
//     map.execute()
// }

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    static EXAMPLE1: &str = indoc! {"
        2199943210
        3987894921
        9856789892
        8767896789
        9899965678
    "};

    #[test]
    fn test_part1_example1() {
        let result = part1(EXAMPLE1);
        assert_eq!(result, 15);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file());
        assert_eq!(result, 564);
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
