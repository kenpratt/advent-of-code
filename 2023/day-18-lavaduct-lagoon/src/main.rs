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
    // println!("part 2 result: {:?}", part2(&read_input_file()));
}

fn read_input_file() -> String {
    fs::read_to_string("input.txt").expect("Something went wrong reading the file")
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    distance: i16,
}

impl Instruction {
    fn parse_list(input: &str) -> Vec<Self> {
        input.lines().map(|line| Self::parse(line)).collect()
    }

    fn parse(input: &str) -> Self {
        lazy_static! {
            static ref INSTRUCTION_RE: Regex =
                Regex::new(r"\A([UDLR]) (\d+) \(#([0-9a-f]{6})\)\z").unwrap();
        }

        let caps = INSTRUCTION_RE.captures(input).unwrap();
        let direction = Self::parse_direction(caps.get(1).unwrap().as_str());
        let distance = caps.get(2).unwrap().as_str().parse::<i16>().unwrap();
        let _colour = caps.get(3).unwrap().as_str().to_string();

        Self {
            direction,
            distance,
        }
    }

    fn parse_direction(input: &str) -> Direction {
        match input {
            "U" => Direction::North,
            "D" => Direction::South,
            "L" => Direction::West,
            "R" => Direction::East,
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

    fn determine_facing(edges: &[Edge], min_y: i16) -> Vec<Direction> {
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

    fn x_range(&self) -> RangeInclusive<i16> {
        if self.points.0.x <= self.points.1.x {
            self.points.0.x..=self.points.1.x
        } else {
            self.points.1.x..=self.points.0.x
        }
    }

    fn y_range(&self) -> RangeInclusive<i16> {
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

        let min_y = edges.iter().map(|e| *e.y_range().start()).min().unwrap();
        let max_y = edges.iter().map(|e| *e.y_range().end()).max().unwrap();

        let facing = Edge::determine_facing(&edges, min_y);

        // compute metadata for rasterization
        let mut vertical_intersections_map: HashMap<i16, BTreeSet<(i16, Direction)>> =
            HashMap::new();
        let mut horizontal_segments_map: HashMap<i16, BTreeSet<(i16, i16)>> = HashMap::new();
        for y in min_y..=max_y {
            vertical_intersections_map.insert(y, BTreeSet::new());
            horizontal_segments_map.insert(y, BTreeSet::new());
        }
        for (i, edge) in edges.iter().enumerate() {
            if edge.vertical() {
                let x = edge.points.0.x;
                let facing = facing[i];
                for y in edge.y_range() {
                    vertical_intersections_map
                        .get_mut(&y)
                        .unwrap()
                        .insert((x, facing));
                }
            } else {
                let y = edge.points.0.y;
                let xr = edge.x_range();
                horizontal_segments_map
                    .get_mut(&y)
                    .unwrap()
                    .insert((*xr.start(), *xr.end()));
            }
        }

        // rasterize
        (min_y..=max_y)
            .map(|y| {
                let vertical_intersections = vertical_intersections_map.get(&y).unwrap();
                let horizontal_segments = horizontal_segments_map.get(&y).unwrap();
                Self::line_area(vertical_intersections, horizontal_segments)
            })
            .sum()
    }

    fn line_area(
        vertical_intersections: &BTreeSet<(i16, Direction)>,
        horizontal_segments: &BTreeSet<(i16, i16)>,
    ) -> usize {
        let mut segments = horizontal_segments.clone();

        // find gaps between vertical lines
        for w in vertical_intersections
            .iter()
            .collect::<Vec<&(i16, Direction)>>()
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
    let instructions = Instruction::parse_list(input);

    let plan = Plan::new(instructions);
    plan.area()
}

// fn part2(input: &str) -> usize {
//     let instructions = Data::parse(input);
//     0
// }

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
