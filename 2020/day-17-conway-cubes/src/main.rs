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
    println!("part 1 result: {:?}", part1(&read_input_file(), 6));
    // println!("part 2 result: {:?}", part2(&read_input_file()));
}

fn read_input_file() -> String {
    return fs::read_to_string("input.txt").expect("Something went wrong reading the file");
}

type Coordinate = (isize, isize, isize);

#[derive(Debug)]
struct Grid {
    neighbour_map: NeighbourMap,
    active_set: HashSet<Coordinate>,
}

impl Grid {
    fn parse(input: &str) -> Grid {
        let lines: Vec<Vec<bool>> = input.lines().map(|line| Grid::parse_line(line)).collect();

        let mut active_set = HashSet::new();
        for (y, line) in lines.iter().enumerate() {
            for (x, active) in line.iter().enumerate() {
                if *active {
                    let coord = (x as isize, y as isize, 0 as isize);
                    active_set.insert(coord);
                }
            }
        }

        return Grid {
            neighbour_map: NeighbourMap::new(),
            active_set: active_set,
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

    fn iterate(&mut self) {
        let to_visit = self.positions_to_iterate();

        let mut new_active_set = HashSet::new();
        for p in to_visit {
            let currently_active = self.active_set.contains(&p);
            let num_active_neighbours = self.count_active_neighbours(&p);
            if num_active_neighbours == 3 || (num_active_neighbours == 2 && currently_active) {
                new_active_set.insert(p);
            }
        }
        self.active_set = new_active_set;
    }

    fn positions_to_iterate(&mut self) -> HashSet<Coordinate> {
        let mut to_visit = HashSet::new();
        for position in &self.active_set {
            to_visit.insert(*position);

            let neighbours = self.neighbour_map.get(position);    
            for n in neighbours {
                to_visit.insert(*n);
            }
        }
        return to_visit;
    }

    fn count_active_neighbours(&mut self, position: &Coordinate) -> usize {
        let neighbours = self.neighbour_map.get(position);    
        return self.active_set.intersection(neighbours).count();
    }

    fn print(&self) {
        let min_x = self.active_set.iter().map(|s| s.0).min().unwrap();
        let max_x = self.active_set.iter().map(|s| s.0).max().unwrap();
        let min_y = self.active_set.iter().map(|s| s.1).min().unwrap();
        let max_y = self.active_set.iter().map(|s| s.1).max().unwrap();
        let min_z = self.active_set.iter().map(|s| s.2).min().unwrap();
        let max_z = self.active_set.iter().map(|s| s.2).max().unwrap();

        for z in min_z..=max_z {
            println!("\nz={}", z);
            for y in min_y..=max_y {
                let line: String = (min_x..=max_x).map(|x| if self.active_set.contains(&(x, y, z)) {'#'} else {'.'}).collect();
                println!("{}", line);
            }
        }
    }

    fn execute(&mut self, num_cycles: usize) -> usize {
        println!("start:");
        self.print();

        for i in 0..num_cycles {
            self.iterate();
            println!("\niter {}:", i);
            self.print();
        }

        return self.active_set.len();
    }
}


#[derive(Debug)]
struct NeighbourMap {
    map: HashMap<Coordinate, HashSet<Coordinate>>,
}

impl NeighbourMap {
    fn new() -> NeighbourMap {
        return NeighbourMap {
            map: HashMap::new(),
        };
    }

    fn get(&mut self, position: &Coordinate) -> &HashSet<Coordinate> {
        if !self.map.contains_key(position) {
            let neighbours = NeighbourMap::calculate_neighbours(position);
            self.map.insert(*position, neighbours);
        }

        return self.map.get(position).unwrap();
    }

    fn calculate_neighbours(position: &Coordinate) -> HashSet<Coordinate> {
        return NEIGHBOUR_OFFSETS.iter().map(|offset| NeighbourMap::apply_offset(position, offset)).collect();
    }

    fn apply_offset(position: &Coordinate, offset: &Coordinate) -> Coordinate {
        return (position.0 + offset.0, position.1 + offset.1, position.2 + offset.2);
    }
}

fn part1(input: &str, num_cycles: usize) -> usize {
    let mut grid = Grid::parse(input);
    return grid.execute(num_cycles);
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
        let result = part1(EXAMPLE1, 6);
        assert_eq!(result, 112);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file(), 6);
        assert_eq!(result, 322);
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