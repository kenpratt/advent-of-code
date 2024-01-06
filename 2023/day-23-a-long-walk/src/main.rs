pub mod grid;

use std::fs;

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

#[derive(Debug)]
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
            '>' => Slope(West),
            'v' => Slope(South),
            '<' => Slope(East),
            _ => panic!("Unexpected terrain: {:?}", input),
        }
    }
}

fn part1(input: &str) -> usize {
    let map = Map::parse(input);
    dbg!(&map);
    0
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
        assert_eq!(result, 0);
    }

    // #[test]
    // fn test_part1_solution() {
    //     let result = part1(&read_input_file());
    //     assert_eq!(result, 0);
    // }

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
