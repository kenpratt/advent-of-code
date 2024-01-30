use std::cmp;

use crate::file::*;
use crate::spatial::{Bounds, Coord};

use itertools::*;
use lazy_static::lazy_static;
use regex::Regex;

pub fn run() {
    let input = parse(&read_input_file!());
    println!("part 1 result: {:?}", part1(&input));
    println!("part 2 result: {:?}", part2(&input));
}

type NumT = i32;
type CoordT = Coord<NumT>;

#[derive(Clone, Copy, Debug)]
struct Point {
    position: CoordT,
    velocity: CoordT,
}

impl Point {
    fn parse_list(input: &str) -> Vec<Self> {
        input.lines().map(|line| Self::parse(line)).collect()
    }

    fn parse(input: &str) -> Self {
        lazy_static! {
            static ref POINT_RE: Regex =
                Regex::new(r"\Aposition=<(.+),(.+)> velocity=<(.+),(.+)>\z").unwrap();
        }

        let caps = POINT_RE.captures(input).unwrap();
        let mut nums = caps
            .iter()
            .skip(1)
            .map(|c| c.unwrap().as_str().trim().parse::<NumT>().unwrap());

        let position = Coord::new(nums.next().unwrap(), nums.next().unwrap());
        let velocity = Coord::new(nums.next().unwrap(), nums.next().unwrap());
        assert_eq!(nums.next(), None);

        Self { position, velocity }
    }

    fn next(&self) -> Self {
        Self {
            position: self.position + self.velocity,
            velocity: self.velocity,
        }
    }

    fn fast_forward(&self, n: NumT) -> Self {
        // TODO try to implement mul for Coord
        Self {
            position: Coord {
                x: self.position.x + self.velocity.x * n,
                y: self.position.y + self.velocity.y * n,
            },
            velocity: self.velocity,
        }
    }

    fn find_word(initial_points: &[Self]) -> String {
        // the arrangement with the least entropy should be the one that spells the word!
        let least_entropy = Self::find_least_entropy_arrangement(initial_points);
        let coords: Vec<CoordT> = least_entropy.into_iter().map(|p| p.position).collect();

        // build an output string
        let bounds = Bounds::calculate(&coords);
        let width = bounds.width() + 1;
        let height = bounds.height();

        let mut chars: Vec<char> = vec!['.'; (width * height) as usize];
        for coord in &coords {
            let i = (coord.y - bounds.top) * width + (coord.x - bounds.left);
            chars[i as usize] = '#';
        }

        // add newlines
        for y in 0..height {
            let i = y * width + width - 1;
            chars[i as usize] = '\n';
        }

        chars.into_iter().collect()
    }

    fn find_least_entropy_arrangement(initial_points: &[Self]) -> Vec<Self> {
        // find the time when the furthest x points will intersect
        let min_x = initial_points.iter().min_by_key(|p| p.position.x).unwrap();
        let max_x = initial_points.iter().max_by_key(|p| p.position.x).unwrap();
        let converge_x = Self::converge_at(
            min_x.position.x,
            min_x.velocity.x,
            max_x.position.x,
            max_x.velocity.x,
        );

        // find the time when the furthest y points will intersect
        let min_y = initial_points.iter().min_by_key(|p| p.position.y).unwrap();
        let max_y = initial_points.iter().max_by_key(|p| p.position.y).unwrap();
        let converge_y = Self::converge_at(
            min_y.position.y,
            min_y.velocity.y,
            max_y.position.y,
            max_y.velocity.y,
        );

        // for a start time, take the min of those two, and subtract 100 to ensure we don't miss the word
        let fast_forward = cmp::min(converge_x, converge_y) - 100;

        // advance the simulation to the start time
        let mut prev_points = if fast_forward > 0 {
            Self::fast_forward_arrangement(initial_points, fast_forward)
        } else {
            initial_points.to_vec()
        };

        // track the lowest entropy
        let mut least_entropy = Self::calculate_entropy(&prev_points);
        let mut least_entropy_points = prev_points.clone();
        let mut time_since_least_entropy = 0;

        // TODO mutate the points instead of making new ones each time?

        // run until we haven't beaten our best in 50 ticks
        while time_since_least_entropy < 50 {
            let points = Self::next_arrangement(&prev_points);
            let entropy = Self::calculate_entropy(&points);

            if entropy < least_entropy {
                least_entropy = entropy;
                least_entropy_points = points.clone();
                time_since_least_entropy = 0;
            } else {
                time_since_least_entropy += 1;
            }

            prev_points = points;
        }

        least_entropy_points
    }

    fn next_arrangement(points: &[Self]) -> Vec<Self> {
        points.iter().map(|point| point.next()).collect()
    }

    fn fast_forward_arrangement(points: &[Self], num: NumT) -> Vec<Self> {
        points.iter().map(|point| point.fast_forward(num)).collect()
    }

    fn calculate_entropy(points: &[Self]) -> usize {
        let unique_xs = points.iter().map(|p| p.position.x).unique().count();
        let unique_ys = points.iter().map(|p| p.position.y).unique().count();
        unique_xs + unique_ys
    }

    fn converge_at(p1: NumT, v1: NumT, p2: NumT, v2: NumT) -> NumT {
        let pd = p2 - p1;
        let vd = v2 - v1;
        (pd / vd).abs()
    }
}

fn parse(input: &str) -> Vec<Point> {
    Point::parse_list(input)
}

fn part1(points: &[Point]) -> String {
    let result = Point::find_word(points);
    println!("{}", result);
    result
}

fn part2(_points: &[Point]) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    const EXAMPLE_SOLUTION: &'static str = indoc! {"
        #...#..###
        #...#...#.
        #...#...#.
        #####...#.
        #...#...#.
        #...#...#.
        #...#...#.
        #...#..###
    "};

    const REAL_SOLUTION: &'static str = indoc! {"
        ..##....#####....####...#....#.....###..#####...#....#..######
        .#..#...#....#..#....#..#....#......#...#....#..#....#..#.....
        #....#..#....#..#........#..#.......#...#....#...#..#...#.....
        #....#..#....#..#........#..#.......#...#....#...#..#...#.....
        #....#..#####...#.........##........#...#####.....##....#####.
        ######..#....#..#..###....##........#...#....#....##....#.....
        #....#..#....#..#....#...#..#.......#...#....#...#..#...#.....
        #....#..#....#..#....#...#..#...#...#...#....#...#..#...#.....
        #....#..#....#..#...##..#....#..#...#...#....#..#....#..#.....
        #....#..#####....###.#..#....#...###....#####...#....#..#.....
    "};

    #[test]
    fn test_part1_example() {
        let result = part1(&parse(&read_example_file!()));
        assert_eq!(result, EXAMPLE_SOLUTION);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&parse(&read_input_file!()));
        assert_eq!(result, REAL_SOLUTION);
    }

    #[test]
    fn test_part2_example() {
        let result = part2(&parse(&read_example_file!()));
        assert_eq!(result, 0);
    }

    #[test]
    fn test_part2_solution() {
        let result = part2(&parse(&read_input_file!()));
        assert_eq!(result, 0);
    }
}
