pub mod coordinate;

use coordinate::*;

use std::collections::HashMap;
use std::fs;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    println!("part 2 result: {:?}", part2(&read_input_file()));
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

static SAND_STARTING_POINT: Coordinate = Coordinate { x: 500, y: 0 };

static SHIFT_SAND_OFFSETS: &'static [Coordinate] = &[
    Coordinate { x: 0, y: 1 },  // one step down
    Coordinate { x: -1, y: 1 }, // one step down and to the left
    Coordinate { x: 1, y: 1 },  // one step down and to the right
];

#[derive(Debug)]
struct Simulation {
    occupied: HashMap<Coordinate, Material>,
    max_y: isize,
    has_floor: bool,
}

impl Simulation {
    fn new(has_floor: bool) -> Self {
        Self {
            occupied: HashMap::new(),
            max_y: 0,
            has_floor: has_floor,
        }
    }

    fn run(paths: &[Path], has_floor: bool) -> Self {
        let mut sim = Self::new(has_floor);

        // add rock paths
        for path in paths {
            sim.add_rock_path(path);
        }

        // adjust max_y to floor, if applicable
        if has_floor {
            sim.max_y += 2;
        }

        // continue adding sands grains until an overflow occurs,
        // or the starting point is blocked
        let mut halt = false;
        while !halt {
            halt = sim.add_sand_grain();
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

    // result = overflowed | starting point blocked
    fn add_sand_grain(&mut self) -> bool {
        use ShiftSandResult::*;

        if self.is_blocked(&SAND_STARTING_POINT) {
            return true;
        }

        self.occupied.insert(SAND_STARTING_POINT, Material::Sand);

        let mut c = SAND_STARTING_POINT;
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
            if !self.is_blocked(&new_loc) {
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

    fn is_blocked(&self, location: &Coordinate) -> bool {
        if self.has_floor && location.y == self.max_y {
            true
        } else {
            self.occupied.contains_key(location)
        }
    }

    fn count_sand(&self) -> usize {
        self.occupied
            .values()
            .filter(|m| m == &&Material::Sand)
            .count()
    }
}

fn part1(input: &str) -> usize {
    let paths = Path::parse_paths(input);
    dbg!(&paths);
    let sim = Simulation::run(&paths, false);
    dbg!(&sim);
    sim.count_sand()
}

fn part2(input: &str) -> usize {
    let paths = Path::parse_paths(input);
    dbg!(&paths);
    let sim = Simulation::run(&paths, true);
    dbg!(&sim);
    sim.count_sand()
}

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

    #[test]
    fn test_part2_example1() {
        let result = part2(EXAMPLE1);
        assert_eq!(result, 93);
    }

    #[test]
    fn test_part2_solution() {
        let result = part2(&read_input_file());
        assert_eq!(result, 27976);
    }
}
