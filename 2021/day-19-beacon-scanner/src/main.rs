use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::fs;

use itertools::Itertools;
use lazy_static::lazy_static;
use prehash::{DefaultPrehasher, PrehashedSet, Prehasher};
use regex::Regex;

lazy_static! {
    static ref SCANNER_NUMBER_RE: Regex = Regex::new(r"\A\-\-\- scanner (\d+) \-\-\-\z").unwrap();
    static ref PREHASHER: DefaultPrehasher = DefaultPrehasher::new();
}

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    println!("part 2 result: {:?}", part2(&read_input_file()));
}

fn read_input_file() -> String {
    fs::read_to_string("input.txt").expect("Something went wrong reading the file")
}

fn parse(input: &str) -> HashMap<usize, Scanner> {
    input
        .split("\n\n")
        .map(|s| Scanner::parse(s))
        .map(|s| (s.number, s))
        .collect()
}

#[derive(Clone, Debug)]
struct Scanner {
    number: usize,
    beacons: HashSet<Beacon>,
    orientations: HashMap<&'static Orientation, ScannerOrientation>,
}

impl Scanner {
    fn parse(input: &str) -> Scanner {
        let mut lines = input.lines();
        let description = lines.next().unwrap();
        let number = SCANNER_NUMBER_RE
            .captures(description)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap();
        let beacons = lines.map(|l| Beacon::parse(l)).collect();
        Self::build(number, beacons)
    }

    fn build(number: usize, beacons: HashSet<Beacon>) -> Scanner {
        let orientations = ScannerOrientation::build_orientations(&beacons)
            .into_iter()
            .map(|o| (o.orientation, o))
            .collect();
        Scanner {
            number,
            beacons,
            orientations,
        }
    }

    fn orientation(&self, orientation: &Orientation) -> &ScannerOrientation {
        self.orientations.get(orientation).unwrap()
    }

    fn overlaps(&self, base: &ScannerOrientation) -> Option<(&'static Orientation, Beacon)> {
        for (_, orientation) in &self.orientations {
            match base.overlaps(orientation) {
                Some(offset) => {
                    return Some((orientation.orientation, offset));
                }
                None => {}
            }
        }
        None
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Beacon {
    x: isize,
    y: isize,
    z: isize,
}

impl Beacon {
    fn parse(input: &str) -> Beacon {
        let parts: Vec<isize> = input
            .split(",")
            .map(|s| s.parse::<isize>().unwrap())
            .collect();
        assert_eq!(parts.len(), 3);
        Beacon {
            x: parts[0],
            y: parts[1],
            z: parts[2],
        }
    }

    fn add(&self, other: &Beacon) -> Beacon {
        let x = self.x + other.x;
        let y = self.y + other.y;
        let z = self.z + other.z;
        Beacon { x, y, z }
    }

    fn subtract(&self, other: &Beacon) -> Beacon {
        let x = self.x - other.x;
        let y = self.y - other.y;
        let z = self.z - other.z;
        Beacon { x, y, z }
    }

    fn distance(&self, other: &Beacon) -> usize {
        ((self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()) as usize
    }
}

impl fmt::Display for Beacon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{},{}", self.x, self.y, self.z)
    }
}

#[derive(Debug, Hash, Eq, PartialEq)]
enum Axis {
    X,
    Y,
    Z,
}

#[derive(Debug, Hash, Eq, PartialEq)]
enum Sign {
    Positive,
    Negative,
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct Mapping(Axis, Sign);

impl Mapping {
    fn apply(&self, beacon: &Beacon) -> isize {
        let value = match self.0 {
            Axis::X => beacon.x,
            Axis::Y => beacon.y,
            Axis::Z => beacon.z,
        };
        match self.1 {
            Sign::Positive => value,
            Sign::Negative => -value,
        }
    }
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct Orientation(&'static Mapping, &'static Mapping, &'static Mapping);

impl Orientation {
    fn apply(&self, beacon: &Beacon) -> Beacon {
        let x = self.0.apply(beacon);
        let y = self.1.apply(beacon);
        let z = self.2.apply(beacon);
        Beacon { x, y, z }
    }
}

static X_POS: &'static Mapping = &Mapping(Axis::X, Sign::Positive);
static X_NEG: &'static Mapping = &Mapping(Axis::X, Sign::Negative);
static Y_POS: &'static Mapping = &Mapping(Axis::Y, Sign::Positive);
static Y_NEG: &'static Mapping = &Mapping(Axis::Y, Sign::Negative);
static Z_POS: &'static Mapping = &Mapping(Axis::Z, Sign::Positive);
static Z_NEG: &'static Mapping = &Mapping(Axis::Z, Sign::Negative);

static ORIENTATIONS: &'static [Orientation] = &[
    Orientation(X_POS, Y_POS, Z_POS),
    Orientation(Y_NEG, X_POS, Z_POS),
    Orientation(X_NEG, Y_NEG, Z_POS),
    Orientation(Y_POS, X_NEG, Z_POS),
    Orientation(Z_POS, Y_POS, X_NEG),
    Orientation(Y_NEG, Z_POS, X_NEG),
    Orientation(Z_NEG, Y_NEG, X_NEG),
    Orientation(Y_POS, Z_NEG, X_NEG),
    Orientation(X_NEG, Y_POS, Z_NEG),
    Orientation(Y_NEG, X_NEG, Z_NEG),
    Orientation(X_POS, Y_NEG, Z_NEG),
    Orientation(Y_POS, X_POS, Z_NEG),
    Orientation(Z_NEG, Y_POS, X_POS),
    Orientation(Y_NEG, Z_NEG, X_POS),
    Orientation(Z_POS, Y_NEG, X_POS),
    Orientation(Y_POS, Z_POS, X_POS),
    Orientation(X_POS, Z_NEG, Y_POS),
    Orientation(Z_POS, X_POS, Y_POS),
    Orientation(X_NEG, Z_POS, Y_POS),
    Orientation(Z_NEG, X_NEG, Y_POS),
    Orientation(X_NEG, Z_NEG, Y_NEG),
    Orientation(Z_POS, X_NEG, Y_NEG),
    Orientation(X_POS, Z_POS, Y_NEG),
    Orientation(Z_NEG, X_POS, Y_NEG),
];
static BASE_ORIENTATION: &'static Orientation = &ORIENTATIONS[0];

#[derive(Clone, Debug)]
struct ScannerOrientation {
    orientation: &'static Orientation,
    beacons: HashSet<Beacon>,
    beacon_offsets: HashMap<Beacon, PrehashedSet<Beacon>>,
}

impl ScannerOrientation {
    fn build_orientations(beacons: &HashSet<Beacon>) -> Vec<ScannerOrientation> {
        ORIENTATIONS.iter().map(|o| Self::new(o, beacons)).collect()
    }

    fn new(
        orientation: &'static Orientation,
        input_beacons: &HashSet<Beacon>,
    ) -> ScannerOrientation {
        let beacons: HashSet<Beacon> = input_beacons.iter().map(|b| orientation.apply(b)).collect();
        Self::build(orientation, beacons)
    }

    fn build(orientation: &'static Orientation, beacons: HashSet<Beacon>) -> ScannerOrientation {
        let beacon_offsets = Self::build_beacon_offsets(&beacons);
        ScannerOrientation {
            orientation,
            beacons,
            beacon_offsets,
        }
    }

    fn build_beacon_offsets(beacons: &HashSet<Beacon>) -> HashMap<Beacon, PrehashedSet<Beacon>> {
        beacons
            .iter()
            .map(|b| (*b, Self::build_beacon_offset(b, beacons)))
            .collect()
    }

    fn build_beacon_offset(base: &Beacon, beacons: &HashSet<Beacon>) -> PrehashedSet<Beacon> {
        beacons
            .iter()
            .map(|b| b.subtract(base))
            .map(|b| PREHASHER.prehash(b))
            .collect()
    }

    fn overlaps(&self, other: &ScannerOrientation) -> Option<Beacon> {
        for (my_base_beacon, my_beacon_set) in &self.beacon_offsets {
            for (other_base_beacon, other_beacon_set) in &other.beacon_offsets {
                let num_in_common = my_beacon_set.intersection(other_beacon_set).count();
                if num_in_common >= 12 {
                    let offset = my_base_beacon.subtract(other_base_beacon);
                    return Some(offset);
                }
            }
        }
        None
    }

    fn combine(&self, other: &ScannerOrientation, offset: &Beacon) -> ScannerOrientation {
        let offset_beacons: HashSet<Beacon> = other.beacons.iter().map(|b| b.add(offset)).collect();
        let combined_beacons: HashSet<Beacon> =
            self.beacons.union(&offset_beacons).cloned().collect();
        Self::build(self.orientation, combined_beacons)
    }
}

impl fmt::Display for ScannerOrientation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "orientation: {:?}\n", self.orientation).unwrap();
        write!(f, "beacons:\n").unwrap();
        for b in &self.beacons {
            write!(f, "  {}\n", b).unwrap();
        }
        write!(f, "offsets:\n").unwrap();
        for (base, offsets) in &self.beacon_offsets {
            write!(f, "  base: {}\n", base).unwrap();
            for b in offsets {
                write!(f, "    {}\n", b).unwrap();
            }
        }
        Ok(())
    }
}

struct Solver<'a> {
    scanners: &'a HashMap<usize, Scanner>,
    base: ScannerOrientation,
    remaining: HashSet<usize>,
    locations: HashMap<usize, Beacon>,
}

impl Solver<'_> {
    fn solve(scanners: &HashMap<usize, Scanner>) -> (HashSet<Beacon>, HashMap<usize, Beacon>) {
        let base: ScannerOrientation = scanners
            .get(&0)
            .unwrap()
            .orientation(BASE_ORIENTATION)
            .clone();
        let remaining: HashSet<usize> = scanners.keys().map(|k| *k).filter(|k| k != &0).collect();
        let locations = HashMap::new();
        let mut solver = Solver {
            scanners,
            base,
            remaining,
            locations,
        };
        solver.run();
        (solver.base.beacons, solver.locations)
    }

    fn run(&mut self) {
        while !self.remaining.is_empty() {
            self.tick();
        }
    }

    fn tick(&mut self) {
        let (num, orientation, offset) = self
            .remaining
            .iter()
            .find_map(|num| self.overlaps(num))
            .unwrap();
        self.remaining.remove(&num);

        let target_scanner = self.scanner(&num);
        let target_orientation = target_scanner.orientation(orientation);
        self.base = self.base.combine(target_orientation, &offset);
        self.locations.insert(num, offset);
    }

    fn scanner(&self, num: &usize) -> &Scanner {
        self.scanners.get(&num).unwrap()
    }

    fn overlaps(&self, num: &usize) -> Option<(usize, &'static Orientation, Beacon)> {
        let target = self.scanner(num);
        target.overlaps(&self.base).map(|(a, b)| (*num, a, b))
    }
}

fn part1(input: &str) -> usize {
    let scanners = parse(input);
    let (beacons, _) = Solver::solve(&scanners);
    beacons.len()
}

fn part2(input: &str) -> usize {
    let scanners = parse(input);
    let (_, locations) = Solver::solve(&scanners);
    locations
        .values()
        .tuple_combinations()
        .map(|(a, b)| a.distance(b))
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read_example_file() -> String {
        fs::read_to_string("example.txt").expect("Something went wrong reading the file")
    }

    #[test]
    fn test_part1_example1() {
        let result = part1(&read_example_file());
        assert_eq!(result, 79);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file());
        assert_eq!(result, 405);
    }

    #[test]
    fn test_part2_example1() {
        let result = part2(&read_example_file());
        assert_eq!(result, 3621);
    }

    #[test]
    fn test_part2_solution() {
        let result = part2(&read_input_file());
        assert_eq!(result, 12306);
    }
}
