pub mod coordinate;

use coordinate::*;

use std::collections::HashMap;
use std::fs;

// use itertools::Itertools;
// use lazy_static::lazy_static;
// use regex::Regex;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    // println!("part 2 result: {:?}", part2(&read_input_file()));
}

fn read_input_file() -> String {
    fs::read_to_string("input.txt").expect("Something went wrong reading the file")
}

#[derive(Debug)]
struct Path {
    coords: Vec<Coordinate>,
}

impl Path {
    fn parse_paths(input: &str) -> Vec<Self> {
        input.lines().map(|line| Self::parse(line)).collect()
    }

    fn parse(input: &str) -> Self {
        let coords = input.split(" -> ").map(|s| Coordinate::parse(s)).collect();
        Self { coords }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Material {
    Rock,
    Sand,
}

#[derive(Debug)]
enum ShiftSandResult {
    Moved(Coordinate),
    Blocked,
    Overflowed,
}

static SHIFT_SAND_OFFSETS: &'static [Coordinate] = &[
    Coordinate { x: 0, y: 1 },  // one step down
    Coordinate { x: -1, y: 1 }, // one step down and to the left
    Coordinate { x: 1, y: 1 },  // one step down and to the right
];

#[derive(Debug)]
struct Simulation {
    occupied: HashMap<Coordinate, Material>,
    max_y: isize,
}

impl Simulation {
    fn new() -> Self {
        Self {
            occupied: HashMap::new(),
            max_y: 0,
        }
    }

    fn run(paths: &[Path]) -> Self {
        let mut sim = Self::new();

        // add rock paths
        for path in paths {
            sim.add_rock_path(path);
        }

        // continue adding sands grains until an overflow occurs
        let mut overflowed = false;
        while !overflowed {
            overflowed = sim.add_sand_grain();
        }

        sim
    }

    fn add_rock_path(&mut self, path: &Path) {
        for coords in path.coords.windows(2) {
            for coord in coords[0].line_to(&coords[1]) {
                self.occupied.insert(coord, Material::Rock);
                if coord.y > self.max_y {
                    self.max_y = coord.y;
                }
            }
        }
    }

    // result = overflowed?
    fn add_sand_grain(&mut self) -> bool {
        use ShiftSandResult::*;

        let mut c = Coordinate::new(500, 0);
        loop {
            match self.shift_sand_grain(&c) {
                Moved(new_c) => c = new_c, // keep flowing sand
                Blocked => return false,
                Overflowed => return true,
            }
        }
    }

    fn shift_sand_grain(&mut self, location: &Coordinate) -> ShiftSandResult {
        for offset in SHIFT_SAND_OFFSETS.iter() {
            let new_loc = *location + *offset;
            if !self.occupied.contains_key(&new_loc) {
                // we can move here
                self.occupied.remove(location);

                if new_loc.y > self.max_y {
                    // overflow!
                    return ShiftSandResult::Overflowed;
                } else {
                    // normal sand move
                    self.occupied.insert(new_loc, Material::Sand);
                    return ShiftSandResult::Moved(new_loc);
                }
            }
        }

        // all locations are blocked
        ShiftSandResult::Blocked
    }
}

fn part1(input: &str) -> usize {
    let paths = Path::parse_paths(input);
    dbg!(&paths);
    let sim = Simulation::run(&paths);
    dbg!(&sim);
    sim.occupied
        .values()
        .filter(|m| m == &&Material::Sand)
        .count()
}

// fn part2(input: &str) -> usize {
//     let data = Data::parse(input);
//     dbg!(&data);
//     data.execute()
// }

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    static EXAMPLE1: &str = indoc! {"
        498,4 -> 498,6 -> 496,6
        503,4 -> 502,4 -> 502,9 -> 494,9
    "};

    #[test]
    fn test_part1_example1() {
        let result = part1(EXAMPLE1);
        assert_eq!(result, 24);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file());
        assert_eq!(result, 1001);
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
