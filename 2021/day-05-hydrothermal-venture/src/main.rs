use std::collections::HashMap;
use std::fs;

use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    println!("part 2 result: {:?}", part2(&read_input_file()));
}

fn read_input_file() -> String {
    fs::read_to_string("input.txt").expect("Something went wrong reading the file")
}

lazy_static! {
    static ref LINE_RE: Regex = Regex::new(r"\A(\d+),(\d+) -> (\d+),(\d+)\z").unwrap();
}

fn parse(input: &str) -> Vec<Line> {
    input.lines().map(|line| Line::parse(line)).collect()
}

fn abs_diff(a: usize, b: usize) -> usize {
    if a >= b {
        a - b
    } else {
        b - a
    }
}

fn inclusive_range(v1: usize, v2: usize) -> Box<dyn Iterator<Item = usize>> {
    if v1 <= v2 {
        Box::new(v1..=v2)
    } else {
        Box::new((v2..=v1).rev())
    }
}

type Point = (usize, usize);

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn parse(input: &str) -> Line {
        let captures = LINE_RE.captures(input).unwrap();
        let x1 = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let y1 = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();
        let x2 = captures.get(3).unwrap().as_str().parse::<usize>().unwrap();
        let y2 = captures.get(4).unwrap().as_str().parse::<usize>().unwrap();
        Line {
            start: (x1, y1),
            end: (x2, y2),
        }
    }

    fn is_vertical(&self) -> bool {
        self.start.0 == self.end.0
    }

    fn is_horizontal(&self) -> bool {
        self.start.1 == self.end.1
    }

    fn is_perfect_diagonal(&self) -> bool {
        abs_diff(self.start.0, self.end.0) == abs_diff(self.start.1, self.end.1)
    }

    fn points(&self) -> Vec<Point> {
        if self.is_vertical() {
            let x = self.start.0;
            inclusive_range(self.start.1, self.end.1)
                .map(|y| (x, y))
                .collect()
        } else if self.is_horizontal() {
            let y = self.start.1;
            inclusive_range(self.start.0, self.end.0)
                .map(|x| (x, y))
                .collect()
        } else if self.is_perfect_diagonal() {
            let xs = inclusive_range(self.start.0, self.end.0);
            let ys = inclusive_range(self.start.1, self.end.1);
            xs.zip(ys).collect()
        } else {
            panic!("invalid line geometry {:?}", self)
        }
    }
}

fn num_intersections(lines: &[Line]) -> usize {
    let mut cells = HashMap::new();
    for points in lines.iter().map(|l| l.points()) {
        for point in points {
            let entry = cells.entry(point).or_insert(0);
            *entry += 1;
        }
    }

    // intersections are where at least two lines passed through
    cells.values().filter(|&v| v >= &2).count()
}

fn part1(input: &str) -> usize {
    let lines = parse(input);

    let straight_lines: Vec<Line> = lines
        .into_iter()
        .filter(|l| l.is_horizontal() || l.is_vertical())
        .collect();

    num_intersections(&straight_lines)
}

fn part2(input: &str) -> usize {
    let lines = parse(input);
    num_intersections(&lines)
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    static EXAMPLE1: &str = indoc! {"
        0,9 -> 5,9
        8,0 -> 0,8
        9,4 -> 3,4
        2,2 -> 2,1
        7,0 -> 7,4
        6,4 -> 2,0
        0,9 -> 2,9
        3,4 -> 1,4
        0,0 -> 8,8
        5,5 -> 8,2
    "};

    #[test]
    fn test_part1_example1() {
        let result = part1(EXAMPLE1);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file());
        assert_eq!(result, 5092);
    }

    #[test]
    fn test_part2_example1() {
        let result = part2(EXAMPLE1);
        assert_eq!(result, 12);
    }

    #[test]
    fn test_part2_solution() {
        let result = part2(&read_input_file());
        assert_eq!(result, 20484);
    }
}
