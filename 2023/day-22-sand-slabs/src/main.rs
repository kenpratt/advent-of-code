use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    fs,
    ops::RangeInclusive,
};

use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    // println!("part 2 result: {:?}", part2(&read_input_file()));
}

fn read_input_file() -> String {
    fs::read_to_string("input.txt").expect("Something went wrong reading the file")
}

#[derive(Debug)]
struct Simulation {
    bricks: Vec<Brick>,
    settled: HashMap<Coord2D, Vec<(RangeInclusive<u16>, usize)>>,
}

impl Simulation {
    fn parse(input: &str) -> Self {
        let bricks = Brick::parse_list(input);
        let settled = HashMap::new();
        Self { bricks, settled }
    }

    fn settle(&mut self) {
        // shift each one down, one at a time
        for brick in &self.bricks {
            let columns = brick.columns();

            // find the z that this brick will settle at
            let settle_z = columns
                .iter()
                .map(|column| {
                    match self.settled.get(column) {
                        Some(settled_in_column) => {
                            // settle one higher than the current occupant
                            settled_in_column.last().unwrap().0.end() + 1
                        }
                        None => {
                            // nothing settled yet, we can drop to z=0
                            0
                        }
                    }
                })
                .max()
                .unwrap();

            // shouldn't settle above where it is now
            assert!(settle_z <= brick.left.z);

            // calculate z range it'll settle at
            let drop_z = brick.left.z - settle_z;
            let settle_z_range = (brick.left.z - drop_z)..=(brick.right.z - drop_z);

            // settle the brick
            for column in columns {
                self.settled
                    .entry(column)
                    .or_default()
                    .push((settle_z_range.clone(), brick.id));
            }
        }
    }

    fn num_safe_to_disintegrate(&self) -> usize {
        let mut supporting: HashMap<usize, HashSet<usize>> = HashMap::new();
        let mut supported_by: HashMap<usize, HashSet<usize>> = HashMap::new();

        for column in self.settled.values() {
            for w in column.windows(2) {
                let (z_under, id_under) = &w[0];
                let (z_over, id_over) = &w[1];
                if *z_over.start() == z_under.end() + 1 {
                    // under is directly supporting over
                    supporting.entry(*id_under).or_default().insert(*id_over);
                    supported_by.entry(*id_over).or_default().insert(*id_under);
                }
            }
        }

        self.bricks
            .iter()
            .filter(|brick| {
                // can we disintegrate this brick?
                match supporting.get(&brick.id) {
                    Some(supp) => {
                        // do all the supported bricks have more than just this brick supporting them?
                        // if so, safe to dissolve this one
                        supp.iter().all(|s| supported_by.get(s).unwrap().len() > 1)
                    }
                    None => {
                        // this brick is not supporting anything, safe to dissolve
                        true
                    }
                }
            })
            .count()
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Brick {
    id: usize,
    left: Coord3D,
    right: Coord3D,
}

impl Brick {
    fn new(id: usize, left: Coord3D, right: Coord3D) -> Self {
        // ensure left and right are in ascending order on all axes
        assert!(right.x >= left.x);
        assert!(right.y >= left.y);
        assert!(right.z >= left.z);
        Self { id, left, right }
    }

    fn parse_list(input: &str) -> Vec<Self> {
        let mut bricks: Vec<Self> = input
            .lines()
            .enumerate()
            .map(|(i, line)| Self::parse(i, line))
            .collect();
        bricks.sort();
        bricks
    }

    fn parse(id: usize, input: &str) -> Self {
        lazy_static! {
            static ref ITEM_RE: Regex =
                Regex::new(r"\A(\d+),(\d+),(\d+)~(\d+),(\d+),(\d+)\z").unwrap();
        }

        let caps = ITEM_RE.captures(input).unwrap();
        let nums: Vec<u16> = caps
            .iter()
            .skip(1)
            .map(|c| c.unwrap().as_str().parse::<u16>().unwrap())
            .collect();
        assert_eq!(nums.len(), 6);

        let left = Coord3D::from_slice(&nums[0..3]);
        let right = Coord3D::from_slice(&nums[3..6]);
        Self::new(id, left, right)
    }

    // TODO see if I can make this an iter instead of building an array?
    // or maybe just make a cached fn with (x1, x2, y1, y2)?
    // or don't bother, if it's fast enough
    fn columns(&self) -> Vec<Coord2D> {
        let mut out = vec![];

        for x in self.left.x..=self.right.x {
            for y in self.left.y..=self.right.y {
                out.push(Coord2D::new(x, y));
            }
        }

        out
    }
}

impl Ord for Brick {
    fn cmp(&self, other: &Self) -> Ordering {
        self.left
            .cmp(&other.left)
            .then(self.right.cmp(&other.right))
            .then(self.id.cmp(&other.id))
    }
}

impl PartialOrd for Brick {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Coord3D {
    x: u16,
    y: u16,
    z: u16,
}

impl Coord3D {
    fn new(x: u16, y: u16, z: u16) -> Self {
        Self { x, y, z }
    }

    fn from_slice(nums: &[u16]) -> Self {
        assert_eq!(nums.len(), 3);
        Self::new(nums[0], nums[1], nums[2])
    }
}

// sort so z=0 is first
impl Ord for Coord3D {
    fn cmp(&self, other: &Self) -> Ordering {
        self.z
            .cmp(&other.z)
            .then(self.x.cmp(&other.x))
            .then(self.y.cmp(&other.y))
    }
}

impl PartialOrd for Coord3D {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Coord2D {
    x: u16,
    y: u16,
}

impl Coord2D {
    fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
}

fn part1(input: &str) -> usize {
    let mut simulation = Simulation::parse(input);
    simulation.settle();
    simulation.num_safe_to_disintegrate()
}

// fn part2(input: &str) -> usize {
//     let items = Data::parse(input);
//     0
// }

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    static EXAMPLE: &str = indoc! {"
        1,0,1~1,2,1
        0,0,2~2,0,2
        0,2,3~2,2,3
        0,0,4~0,2,4
        2,0,5~2,2,5
        0,1,6~2,1,6
        1,1,8~1,1,9
    "};

    #[test]
    fn test_part1_example() {
        let result = part1(EXAMPLE);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file());
        assert_eq!(result, 480);
    }

    // #[test]
    // fn test_part2_example() {
    //     let result = part2(EXAMPLE);
    //     assert_eq!(result, 0);
    // }

    // #[test]
    // fn test_part2_solution() {
    //     let result = part2(&read_input_file());
    //     assert_eq!(result, 0);
    // }
}
