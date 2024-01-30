use std::cmp;

use crate::file::*;
use crate::spatial::{Bounds, Coord};

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
                Regex::new(r"\Aposition=<(.+)> velocity=<(.+)>\z").unwrap();
        }

        let caps = POINT_RE.captures(input).unwrap();
        let position = Coord::parse(caps.get(1).unwrap().as_str(), ",");
        let velocity = Coord::parse(caps.get(2).unwrap().as_str(), ",");

        Self { position, velocity }
    }

    fn fast_forward(&self, n: NumT) -> Self {
        Self {
            position: self.position + self.velocity * n,
            velocity: self.velocity,
        }
    }

    fn find_word(initial_points: &[Self]) -> String {
        // the arrangement at convergence point should be the one that spells the word!
        let (convergence, _) = Self::find_convergence_arrangement(initial_points);
        let coords: Vec<CoordT> = convergence.into_iter().map(|p| p.position).collect();

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

    fn find_convergence_arrangement(initial_points: &[Self]) -> (Vec<Self>, NumT) {
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

        // for a start time, take the min of those two, and subtract BUF to ensure we don't miss the word
        let time = cmp::min(converge_x, converge_y);

        // advance the simulation to the start time
        let points = Self::fast_forward_arrangement(initial_points, time);

        (points, time)
    }

    fn fast_forward_arrangement(points: &[Self], num: NumT) -> Vec<Self> {
        points.iter().map(|point| point.fast_forward(num)).collect()
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
    Point::find_word(points)
}

fn part2(points: &[Point]) -> NumT {
    let (_, took) = Point::find_convergence_arrangement(points);
    took
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
        assert_eq!(result, 3);
    }

    #[test]
    fn test_part2_solution() {
        let result = part2(&parse(&read_input_file!()));
        assert_eq!(result, 10619);
    }
}
