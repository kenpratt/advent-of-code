use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

const NEIGHBOUR_OFFSETS: [Coordinate; 26] = [
    (-1, -1, -1),
    (-1, -1, 0),
    (-1, -1, 1),
    (-1, 0, -1),
    (-1, 0, 0),
    (-1, 0, 1),
    (-1, 1, -1),
    (-1, 1, 0),
    (-1, 1, 1),
    (0, -1, -1),
    (0, -1, 0),
    (0, -1, 1),
    (0, 0, -1),
    (0, 0, 1),
    (0, 1, -1),
    (0, 1, 0),
    (0, 1, 1),
    (1, -1, -1),
    (1, -1, 0),
    (1, -1, 1),
    (1, 0, -1),
    (1, 0, 0),
    (1, 0, 1),
    (1, 1, -1),
    (1, 1, 0),
    (1, 1, 1),    
];

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    // println!("part 2 result: {:?}", part2(&read_input_file()));
}

fn read_input_file() -> String {
    return fs::read_to_string("input.txt").expect("Something went wrong reading the file");
}

type Coordinate = (isize, isize, isize);

#[derive(Debug)]
struct Grid {
    neighbour_map: HashMap<Coordinate, HashSet<Coordinate>>,
    active_map: HashMap<Coordinate, bool>,
}

impl Grid {
    fn parse(input: &str) -> Grid {
        let lines: Vec<Vec<bool>> = input.lines().map(|line| Grid::parse_line(line)).collect();

        let mut neighbour_map = HashMap::new();
        let mut active_map = HashMap::new();

        for (y, line) in lines.iter().enumerate() {
            for (x, active) in line.iter().enumerate() {
                let coord = (x as isize, y as isize, 0 as isize);
                let neighbours = Grid::neighbours_of_position(&coord);
                neighbour_map.insert(coord, neighbours);
                active_map.insert(coord, *active);
            }
        }

        return Grid {
            neighbour_map: neighbour_map,
            active_map: active_map,
        }
    }

    fn parse_line(input: &str) -> Vec<bool> {
        return input.chars().map(|c| Grid::parse_state(&c)).collect();
    }

    fn parse_state(input: &char) -> bool {
        return match input {
            '.' => false,
            '#' => true,
            _ => panic!("Unexpected state char: {}", input),
        };
    }

    fn neighbours_of_position(position: &Coordinate) -> HashSet<Coordinate> {
        return NEIGHBOUR_OFFSETS.iter().map(|offset| Grid::apply_position_offset(position, offset)).collect();
    }

    fn apply_position_offset(position: &Coordinate, offset: &Coordinate) -> Coordinate {
        return (position.0 + offset.0, position.1 + offset.1, position.2 + offset.2);
    }

    fn execute(&self) -> usize {
        return 0;
    }
}

fn part1(input: &str) -> usize {
    let grid = Grid::parse(input);
    return grid.execute();
}

// fn part2(input: &str) -> usize {
//     let data = Grid::parse(input);
//     return data.execute();
// }

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    static EXAMPLE1: &str = indoc! {"
        .#.
        ..#
        ###
    "};    

    #[test]
    fn test_part1_example1() {
        let result = part1(EXAMPLE1);
        assert_eq!(result, 0);
    }

    // #[test]
    // fn test_part1_solution() {
    //     let result = part1(&read_input_file());
    //     assert_eq!(result, 0);
    // }

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