use std::{
    cmp::Ordering,
    collections::{BTreeSet, HashMap},
    fs,
    ops::RangeInclusive,
};

use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    println!("part 2 result: {:?}", part2(&read_input_file()));
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

    fn falling_brick_counts(&self) -> HashMap<usize, usize> {
        // build supporting metadata
        let mut supporting: HashMap<usize, BTreeSet<usize>> = HashMap::new();
        let mut supported_by: HashMap<usize, BTreeSet<usize>> = HashMap::new();
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

        let mut result = HashMap::new();
        for brick in &self.bricks {
            // get count of removing just this one brick
            let val = Self::calculate_falling_brick_count(
                &BTreeSet::from([brick.id]),
                &BTreeSet::new(),
                &supporting,
                &supported_by,
            );
            result.insert(brick.id, val);
        }
        result
    }

    fn calculate_falling_brick_count(
        to_remove: &BTreeSet<usize>,
        already_removed: &BTreeSet<usize>,
        supporting: &HashMap<usize, BTreeSet<usize>>,
        supported_by: &HashMap<usize, BTreeSet<usize>>,
    ) -> usize {
        // what bricks are supported by the ones we're removing?
        let mut might_fall: BTreeSet<usize> = BTreeSet::new();
        for id in to_remove {
            match supporting.get(id) {
                Some(ids) => might_fall.extend(ids),
                None => (),
            }
        }

        // does each at-risk brick have more than just these bricks supporting them?
        // or are these the only bricks supporting them?
        let will_fall: BTreeSet<usize> = might_fall
            .iter()
            .filter(|id| {
                supported_by
                    .get(id)
                    .unwrap()
                    .iter()
                    .filter(|x| !already_removed.contains(x) && !to_remove.contains(x))
                    .count()
                    == 0
            })
            .cloned()
            .collect();

        // at least this many will fall
        let mut val = will_fall.len();

        // plus recur
        if !will_fall.is_empty() {
            let mut recur_removed = already_removed.clone();
            recur_removed.append(&mut to_remove.clone());

            let recur_val = Self::calculate_falling_brick_count(
                &will_fall,
                &recur_removed,
                supporting,
                supported_by,
            );
            val += recur_val;
        }

        val
    }

    fn num_safe_to_disintegrate(&self) -> usize {
        let counts = self.falling_brick_counts();
        counts.values().filter(|v| **v == 0).count()
    }

    fn sum_of_falling_brick_counts(&self) -> usize {
        let counts = self.falling_brick_counts();
        counts.values().sum()
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

fn part2(input: &str) -> usize {
    let mut simulation = Simulation::parse(input);
    simulation.settle();
    simulation.sum_of_falling_brick_counts()
}

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

    #[test]
    fn test_part2_example() {
        let result = part2(EXAMPLE);
        assert_eq!(result, 7);
    }

    #[test]
    fn test_part2_solution() {
        let result = part2(&read_input_file());
        assert_eq!(result, 84021);
    }
}
