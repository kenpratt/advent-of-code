pub mod grid;

use std::{
    collections::{BTreeSet, HashMap},
    fs,
    ops::RangeInclusive,
};

use grid::*;

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
struct Instruction {
    direction: Direction,
    distance: i32,
}

impl Instruction {
    fn parse_list(input: &str, flipped: bool) -> Vec<Self> {
        input
            .lines()
            .map(|line| Self::parse(line, flipped))
            .collect()
    }

    fn parse(input: &str, flipped: bool) -> Self {
        lazy_static! {
            static ref INSTRUCTION_RE: Regex =
                Regex::new(r"\A([UDLR]) (\d+) \(#([0-9a-f]{5})([0-3])\)\z").unwrap();
        }

        let caps = INSTRUCTION_RE.captures(input).unwrap();

        if !flipped {
            let direction = Self::parse_direction(caps.get(1).unwrap().as_str());
            let distance = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();

            Self {
                direction,
                distance,
            }
        } else {
            let distance = i32::from_str_radix(caps.get(3).unwrap().as_str(), 16).unwrap();
            let direction = Self::parse_direction(caps.get(4).unwrap().as_str());

            Self {
                direction,
                distance,
            }
        }
    }

    fn parse_direction(input: &str) -> Direction {
        match input {
            "U" | "3" => Direction::North,
            "D" | "1" => Direction::South,
            "L" | "2" => Direction::West,
            "R" | "0" => Direction::East,
            _ => panic!("Unreachable"),
        }
    }
}

#[derive(Debug)]
struct Edge {
    direction: Direction,
    points: (Coord, Coord),
}

impl Edge {
    fn build(instructions: &[Instruction], start: Coord) -> Vec<Self> {
        let mut last = start;
        let res: Vec<Self> = instructions
            .iter()
            .map(|inst| {
                let from = last;
                let to = from.shift(&inst.direction, &inst.distance);
                last = to.clone();
                Self {
                    direction: inst.direction,
                    points: (from, to),
                }
            })
            .collect();

        // ensure we finish at the origin
        assert_eq!(res[0].points.0, res[res.len() - 1].points.1);

        res
    }

    fn determine_facing(edges: &[Edge]) -> Vec<Direction> {
        let min_y = edges.iter().map(|e| *e.y_range().start()).min().unwrap();

        let starting_index = edges
            .iter()
            .position(|edge| edge.horizontal() && edge.points.0.y == min_y)
            .unwrap();

        let mut tmp_facing: Vec<Option<Direction>> = vec![None; edges.len()];
        tmp_facing[starting_index] = Some(Direction::South);
        let mut last_facing = Direction::South;
        let mut last_direction = edges[starting_index].direction;

        for i in ((starting_index + 1)..edges.len()).chain(0..starting_index) {
            let edge = &edges[i];
            let curr_facing = match (last_direction, edge.direction) {
                (Direction::North, Direction::East)
                | (Direction::East, Direction::South)
                | (Direction::South, Direction::West)
                | (Direction::West, Direction::North) => last_facing.clockwise(),
                (Direction::North, Direction::West)
                | (Direction::West, Direction::South)
                | (Direction::South, Direction::East)
                | (Direction::East, Direction::North) => last_facing.counterclockwise(),
                _ => panic!("Unreachable"),
            };

            tmp_facing[i] = Some(curr_facing);
            last_facing = curr_facing;
            last_direction = edge.direction;
        }

        tmp_facing.into_iter().flat_map(|f| f).collect()
    }

    fn x_range(&self) -> RangeInclusive<i32> {
        if self.points.0.x <= self.points.1.x {
            self.points.0.x..=self.points.1.x
        } else {
            self.points.1.x..=self.points.0.x
        }
    }

    fn y_range(&self) -> RangeInclusive<i32> {
        if self.points.0.y <= self.points.1.y {
            self.points.0.y..=self.points.1.y
        } else {
            self.points.1.y..=self.points.0.y
        }
    }

    fn horizontal(&self) -> bool {
        self.points.0.y == self.points.1.y
    }

    fn vertical(&self) -> bool {
        self.points.0.x == self.points.1.x
    }
}

#[derive(Debug)]
struct Plan {
    instructions: Vec<Instruction>,
}

impl Plan {
    fn new(instructions: Vec<Instruction>) -> Self {
        Self { instructions }
    }

    fn area(&self) -> usize {
        let edges = Edge::build(&self.instructions, Coord::new(0, 0));
        let facing = Edge::determine_facing(&edges);

        // organize horizontal lines by y
        let mut horizontal_segments_map: HashMap<i32, BTreeSet<(i32, i32)>> = HashMap::new();
        for edge in edges.iter().filter(|edge| edge.horizontal()) {
            let y = edge.points.0.y;
            let xr = edge.x_range();
            horizontal_segments_map
                .entry(y)
                .or_insert(BTreeSet::new())
                .insert((*xr.start(), *xr.end()));
        }

        // calculate target y's
        // we only need to consider rows with horizontal segments,
        // and the rows directly below those. the line afterward will be the same
        // as the row below.
        let target_ys: BTreeSet<i32> = horizontal_segments_map
            .keys()
            .flat_map(|y| [*y, y + 1])
            .collect();

        // compute vertical intersections at target y's
        let mut vertical_intersections_map: HashMap<i32, BTreeSet<(i32, Direction)>> =
            HashMap::new();
        for (i, edge) in edges
            .iter()
            .enumerate()
            .filter(|(_i, edge)| edge.vertical())
        {
            let x = edge.points.0.x;
            let facing = facing[i];
            let y_range = edge.y_range();
            for y in target_ys.iter().filter(|y| y_range.contains(y)) {
                vertical_intersections_map
                    .entry(*y)
                    .or_insert(BTreeSet::new())
                    .insert((x, facing));
            }
        }

        // go through the target y's, calculating area for that line
        // and fill in the gaps between target y's
        let mut total_area: usize = 0;
        let mut last_y = *target_ys.first().unwrap();
        let mut last_area: usize = 0;
        let empty_intersections = BTreeSet::new();
        let empty_segments = BTreeSet::new();

        for y in &target_ys {
            let diff = y - last_y;
            if diff > 1 {
                // need to fill in a gap
                total_area += last_area * (diff - 1) as usize;
            }

            // calculate area at this y
            let vertical_intersections = vertical_intersections_map
                .get(&y)
                .unwrap_or(&empty_intersections);
            let horizontal_segments = horizontal_segments_map.get(&y).unwrap_or(&empty_segments);
            let area = Self::line_area(vertical_intersections, horizontal_segments);
            total_area += area;

            last_y = *y;
            last_area = area;
        }

        total_area
    }

    fn line_area(
        vertical_intersections: &BTreeSet<(i32, Direction)>,
        horizontal_segments: &BTreeSet<(i32, i32)>,
    ) -> usize {
        let mut segments = horizontal_segments.clone();

        // find gaps between vertical lines
        for w in vertical_intersections
            .iter()
            .collect::<Vec<&(i32, Direction)>>()
            .windows(2)
        {
            match (w[0], w[1]) {
                ((l, Direction::East), (r, Direction::West)) => {
                    segments.insert((*l, *r));
                }
                _ => (),
            }
        }

        // sum up!
        let mut area = 0;
        let mut segments_iter = segments.iter().peekable();
        while let Some((l, r)) = segments_iter.next() {
            assert!(r > l);
            match segments_iter.peek() {
                Some((ol, _or)) if ol <= r => {
                    // overlapping ranges, use (ol - 1) instead of r
                    assert!(ol > l);
                    area += (ol - l) as usize;
                }
                _ => {
                    // normal
                    area += (r - l + 1) as usize;
                }
            }
        }
        area
    }
}

fn part1(input: &str) -> usize {
    let instructions = Instruction::parse_list(input, false);

    let plan = Plan::new(instructions);
    plan.area()
}

fn part2(input: &str) -> usize {
    let instructions = Instruction::parse_list(input, true);

    let plan = Plan::new(instructions);
    plan.area()
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    static EXAMPLE: &str = indoc! {"
        R 6 (#70c710)
        D 5 (#0dc571)
        L 2 (#5713f0)
        D 2 (#d2c081)
        R 2 (#59c680)
        D 2 (#411b91)
        L 5 (#8ceee2)
        U 2 (#caa173)
        L 1 (#1b58a2)
        U 2 (#caa171)
        R 2 (#7807d2)
        U 3 (#a77fa3)
        L 2 (#015232)
        U 2 (#7a21e3)
    "};

    #[test]
    fn test_part1_example() {
        let result = part1(EXAMPLE);
        assert_eq!(result, 62);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file());
        assert_eq!(result, 49578);
    }

    #[test]
    fn test_part2_example() {
        let result = part2(EXAMPLE);
        assert_eq!(result, 952408144115);
    }

    #[test]
    fn test_part2_solution() {
        let result = part2(&read_input_file());
        assert_eq!(result, 52885384955882);
    }
}
