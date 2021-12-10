pub mod grid;

use grid::Coordinate;
use grid::Grid;

use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

use itertools::Itertools;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    println!("part 2 result: {:?}", part2(&read_input_file()));
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

    fn basins(&self) -> HashMap<Coordinate, usize> {
        let low_points: HashSet<Coordinate> =
            self.low_points().iter().map(|(pos, _val)| *pos).collect();
        let mut basin_sizes: HashMap<Coordinate, usize> = HashMap::new();
        for (pos, val) in self.grid.iter() {
            let basin = self.basin_for_point(&pos, val, &low_points);
            if basin.is_some() {
                let low_point = basin.unwrap();
                let counter = basin_sizes.entry(low_point).or_insert(0);
                *counter += 1;
            }
        }
        basin_sizes
    }

    fn basin_for_point(
        &self,
        pos: &Coordinate,
        val: &u8,
        low_points: &HashSet<Coordinate>,
    ) -> Option<Coordinate> {
        if val == &9 {
            // high point, ignore
            None
        } else if low_points.contains(&pos) {
            Some(*pos)
        } else {
            let neighbours = self.grid.neighbours(&pos);
            let lowest_neighbour = neighbours
                .iter()
                .min_by_key(|npos| self.grid.value(npos))
                .unwrap();
            self.basin_for_point(
                lowest_neighbour,
                self.grid.value(lowest_neighbour),
                low_points,
            )
        }
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

fn part2(input: &str) -> usize {
    let map = HeightMap::parse(input);
    println!("map: {:?}", map);
    let basins = map.basins();
    println!("basins: {:?}", basins);
    basins
        .values()
        .sorted()
        .rev()
        .take(3)
        .fold(1, |acc, val| acc * val)
}

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

    #[test]
    fn test_part2_example1() {
        let result = part2(EXAMPLE1);
        assert_eq!(result, 1134);
    }

    #[test]
    fn test_part2_solution() {
        let result = part2(&read_input_file());
        assert_eq!(result, 1038240);
    }
}
