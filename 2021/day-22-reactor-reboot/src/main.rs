use std::cmp;
use std::fmt;
use std::fs;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref INSTRUCTION_RE: Regex = Regex::new(
        r"\A(on|off) x=(\-?\d+)\.\.(\-?\d+),y=(\-?\d+)\.\.(\-?\d+),z=(\-?\d+)\.\.(\-?\d+)\z"
    )
    .unwrap();
}

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    // println!("part 2 result: {:?}", part2(&read_input_file()));
}

fn read_input_file() -> String {
    fs::read_to_string("input.txt").expect("Something went wrong reading the file")
}

#[derive(Debug)]
struct RebootProcedure {
    instructions: Vec<Instruction>,
    state: ReactorState,
}

impl RebootProcedure {
    fn parse(input: &str) -> RebootProcedure {
        let instructions = input.lines().map(|line| Instruction::parse(line)).collect();
        let state = ReactorState::new();
        RebootProcedure {
            instructions,
            state,
        }
    }

    fn execute(&mut self) {
        for instruction in &self.instructions {
            println!("applying {}", instruction,);
            self.state = self.state.apply(instruction);
        }
    }
}

#[derive(Debug)]
struct Instruction {
    on: bool,
    area: Cuboid,
}

impl Instruction {
    fn parse(input: &str) -> Instruction {
        let captures = INSTRUCTION_RE.captures(input).unwrap();
        let on_off = captures.get(1).unwrap().as_str();
        let x_min = captures.get(2).unwrap().as_str().parse::<isize>().unwrap();
        let x_max = captures.get(3).unwrap().as_str().parse::<isize>().unwrap();
        let y_min = captures.get(4).unwrap().as_str().parse::<isize>().unwrap();
        let y_max = captures.get(5).unwrap().as_str().parse::<isize>().unwrap();
        let z_min = captures.get(6).unwrap().as_str().parse::<isize>().unwrap();
        let z_max = captures.get(7).unwrap().as_str().parse::<isize>().unwrap();
        let x = InclusiveRange::new(x_min, x_max);
        let y = InclusiveRange::new(y_min, y_max);
        let z = InclusiveRange::new(z_min, z_max);
        let area = Cuboid::new(x, y, z);
        let on = match on_off {
            "on" => true,
            "off" => false,
            _ => panic!("Unreachable"),
        };
        Instruction { on, area }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let on_off = if self.on { "on" } else { "off" };
        write!(f, "{} {}", on_off, self.area)
    }
}

#[derive(Clone, Copy, Debug)]
enum Axis {
    X,
    Y,
    Z,
}

#[derive(Clone, Copy, Debug)]
struct InclusiveRange {
    min: isize,
    max: isize,
}

impl InclusiveRange {
    fn new(min: isize, max: isize) -> InclusiveRange {
        assert!(min <= max);
        InclusiveRange { min, max }
    }

    fn overlaps(&self, other: &InclusiveRange) -> bool {
        self.contains(other.min)
            || self.contains(other.max)
            || other.contains(self.min)
            || other.contains(self.max)
    }

    fn contains(&self, coord: isize) -> bool {
        coord >= self.min && coord <= self.max
    }

    fn subset_of(&self, other: &InclusiveRange) -> bool {
        self.min >= other.min && self.max <= other.max
    }

    fn maybe_bisect(&self, coord: isize) -> Option<(InclusiveRange, InclusiveRange)> {
        if coord > self.min && coord <= self.max {
            let left = InclusiveRange::new(self.min, coord - 1);
            let right = InclusiveRange::new(coord, self.max);
            Some((left, right))
        } else {
            None
        }
    }

    fn len(&self) -> usize {
        (self.max - self.min + 1) as usize
    }

    fn count_within(&self, target: &InclusiveRange) -> usize {
        let min = cmp::max(self.min, target.min);
        let max = cmp::min(self.max, target.max);
        if max >= min {
            (max - min + 1) as usize
        } else {
            0
        }
    }
}

impl fmt::Display for InclusiveRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}..{}", self.min, self.max)
    }
}

#[derive(Clone, Debug)]
struct Cuboid {
    x: InclusiveRange,
    y: InclusiveRange,
    z: InclusiveRange,
}

impl Cuboid {
    fn new(x: InclusiveRange, y: InclusiveRange, z: InclusiveRange) -> Cuboid {
        Cuboid { x, y, z }
    }

    fn len(&self) -> usize {
        self.x.len() * self.y.len() * self.z.len()
    }

    fn count_within(&self, area: &Cuboid) -> usize {
        self.x.count_within(&area.x) * self.y.count_within(&area.y) * self.z.count_within(&area.z)
    }

    fn overlaps(&self, area: &Cuboid) -> bool {
        self.x.overlaps(&area.x) && self.y.overlaps(&area.y) && self.z.overlaps(&area.z)
    }

    fn subset_of(&self, area: &Cuboid) -> bool {
        self.x.subset_of(&area.x) && self.y.subset_of(&area.y) && self.z.subset_of(&area.z)
    }

    fn bisect_all(cuboids: Vec<Cuboid>, axis: Axis, coord: isize) -> Vec<Cuboid> {
        let mut output = vec![];
        for cuboid in cuboids {
            let (first, maybe_second) = cuboid.bisect(axis, coord);
            output.push(first);
            if maybe_second.is_some() {
                output.push(maybe_second.unwrap());
            }
        }
        output
    }

    fn bisect(self, axis: Axis, coord: isize) -> (Cuboid, Option<Cuboid>) {
        match axis {
            Axis::X => match self.x.maybe_bisect(coord) {
                Some((first_x, second_x)) => {
                    let first = Cuboid::new(first_x, self.y, self.z);
                    let second = Cuboid::new(second_x, self.y, self.z);
                    (first, Some(second))
                }
                None => (self, None),
            },
            Axis::Y => match self.y.maybe_bisect(coord) {
                Some((first_y, second_y)) => {
                    let first = Cuboid::new(self.x, first_y, self.z);
                    let second = Cuboid::new(self.x, second_y, self.z);
                    (first, Some(second))
                }
                None => (self, None),
            },
            Axis::Z => match self.z.maybe_bisect(coord) {
                Some((first_z, second_z)) => {
                    let first = Cuboid::new(self.x, self.y, first_z);
                    let second = Cuboid::new(self.x, self.y, second_z);
                    (first, Some(second))
                }
                None => (self, None),
            },
        }
    }
}

impl fmt::Display for Cuboid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x={},y={},z={} ({})", self.x, self.y, self.z, self.len())
    }
}

#[derive(Debug)]
struct ReactorState {
    on_regions: Vec<Cuboid>,
}

impl ReactorState {
    fn new() -> ReactorState {
        ReactorState { on_regions: vec![] }
    }

    fn apply(&self, instruction: &Instruction) -> ReactorState {
        if instruction.on {
            self.apply_on(&instruction.area)
        } else {
            self.apply_off(&instruction.area)
        }
    }

    fn apply_on(&self, area: &Cuboid) -> ReactorState {
        // first, remove any overlaps, just as if this was an off instruction
        let mut new_on_regions = self.remove_region(area);

        // now, add the new area to the set
        new_on_regions.push(area.clone());

        ReactorState {
            on_regions: new_on_regions,
        }
    }

    fn apply_off(&self, area: &Cuboid) -> ReactorState {
        ReactorState {
            on_regions: self.remove_region(area),
        }
    }

    fn remove_region(&self, area: &Cuboid) -> Vec<Cuboid> {
        // bisect along all planes of the area, to avoid any messy overlap
        let mut non_overlapping = vec![];
        let mut working_set =
            Self::filter_overlapping(self.on_regions.clone(), &mut non_overlapping, &area);

        working_set = Cuboid::bisect_all(working_set, Axis::X, area.x.min);
        working_set = Self::filter_overlapping(working_set, &mut non_overlapping, &area);

        working_set = Cuboid::bisect_all(working_set, Axis::X, area.x.max + 1);
        working_set = Self::filter_overlapping(working_set, &mut non_overlapping, &area);

        working_set = Cuboid::bisect_all(working_set, Axis::Y, area.y.min);
        working_set = Self::filter_overlapping(working_set, &mut non_overlapping, &area);

        working_set = Cuboid::bisect_all(working_set, Axis::Y, area.y.max + 1);
        working_set = Self::filter_overlapping(working_set, &mut non_overlapping, &area);

        working_set = Cuboid::bisect_all(working_set, Axis::Z, area.z.min);
        working_set = Self::filter_overlapping(working_set, &mut non_overlapping, &area);

        working_set = Cuboid::bisect_all(working_set, Axis::Z, area.z.max + 1);
        working_set = Self::filter_overlapping(working_set, &mut non_overlapping, &area);

        // ensure all remaining cuboids in working set should be removed
        assert!(working_set.into_iter().all(|c| c.subset_of(area)));

        non_overlapping
    }

    fn filter_overlapping(
        working_set: Vec<Cuboid>,
        non_overlapping: &mut Vec<Cuboid>,
        area: &Cuboid,
    ) -> Vec<Cuboid> {
        let mut result = vec![];
        for c in working_set {
            if c.overlaps(area) {
                result.push(c);
            } else {
                non_overlapping.push(c);
            }
        }
        result
    }

    fn count_within(&self, area: &Cuboid) -> usize {
        self.on_regions.iter().map(|c| c.count_within(area)).sum()
    }
}

impl fmt::Display for ReactorState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for region in &self.on_regions {
            write!(f, "\n  {}", region)?;
        }
        Ok(())
    }
}

fn part1(input: &str) -> usize {
    let mut reboot = RebootProcedure::parse(input);
    reboot.execute();
    let range = InclusiveRange::new(-50, 50);
    let count_area = Cuboid::new(range, range, range);
    reboot.state.count_within(&count_area)
}

// fn part2(input: &str) -> usize {
//     let data = Data::parse(input);
//     println!("{:?}", data);
//     data.execute()
// }

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    static EXAMPLE1: &str = indoc! {"
        on x=10..12,y=10..12,z=10..12
        on x=11..13,y=11..13,z=11..13
        off x=9..11,y=9..11,z=9..11
        on x=10..10,y=10..10,z=10..10
    "};

    static EXAMPLE2: &str = indoc! {"
        on x=-20..26,y=-36..17,z=-47..7
        on x=-20..33,y=-21..23,z=-26..28
        on x=-22..28,y=-29..23,z=-38..16
        on x=-46..7,y=-6..46,z=-50..-1
        on x=-49..1,y=-3..46,z=-24..28
        on x=2..47,y=-22..22,z=-23..27
        on x=-27..23,y=-28..26,z=-21..29
        on x=-39..5,y=-6..47,z=-3..44
        on x=-30..21,y=-8..43,z=-13..34
        on x=-22..26,y=-27..20,z=-29..19
        off x=-48..-32,y=26..41,z=-47..-37
        on x=-12..35,y=6..50,z=-50..-2
        off x=-48..-32,y=-32..-16,z=-15..-5
        on x=-18..26,y=-33..15,z=-7..46
        off x=-40..-22,y=-38..-28,z=23..41
        on x=-16..35,y=-41..10,z=-47..6
        off x=-32..-23,y=11..30,z=-14..3
        on x=-49..-5,y=-3..45,z=-29..18
        off x=18..30,y=-20..-8,z=-3..13
        on x=-41..9,y=-7..43,z=-33..15
        on x=-54112..-39298,y=-85059..-49293,z=-27449..7877
        on x=967..23432,y=45373..81175,z=27513..53682
    "};

    #[test]
    fn test_part1_example1() {
        let result = part1(EXAMPLE1);
        assert_eq!(result, 39);
    }

    #[test]
    fn test_part1_example2() {
        let result = part1(EXAMPLE2);
        assert_eq!(result, 590784);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file());
        assert_eq!(result, 612714);
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
