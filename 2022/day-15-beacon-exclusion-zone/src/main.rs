pub mod geometry;
use geometry::*;

use std::cmp;
use std::collections::HashSet;
use std::fs;
use std::ops::RangeInclusive;

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file(), 2000000));
    println!("part 2 result: {:?}", part2(&read_input_file(), 0, 4000000));
}

fn read_input_file() -> String {
    fs::read_to_string("input.txt").expect("Something went wrong reading the file")
}

#[derive(Debug)]
struct Reading {
    sensor: Point,
    beacon: Point,
    distance: i32,
}

impl Reading {
    fn parse_list(input: &str) -> Vec<Self> {
        input
            .lines()
            .map(|line| Self::parse(line))
            .sorted_by_cached_key(|r| r.sensor.x)
            .collect()
    }

    fn parse(input: &str) -> Self {
        lazy_static! {
            static ref READING_RE: Regex = Regex::new(r"\ASensor at x=([\-\d]+), y=([\-\d]+): closest beacon is at x=([\-\d]+), y=([\-\d]+)\z").unwrap();
        }

        let caps = READING_RE.captures(input).unwrap();
        let nums: Vec<i32> = caps
            .iter()
            .skip(1)
            .map(|s| s.unwrap().as_str().parse::<i32>().unwrap())
            .collect();
        assert_eq!(nums.len(), 4);

        let sensor = point(nums[0], nums[1]);
        let beacon = point(nums[2], nums[3]);
        let distance = sensor.manhattan_distance(&beacon);

        Self {
            sensor,
            beacon,
            distance,
        }
    }

    fn x_range_at_row(&self, y: i32) -> Option<RangeInclusive<i32>> {
        let dy = (self.sensor.y - y).abs();
        if dy > self.distance {
            None
        } else {
            let dx = self.distance - dy;
            Some((self.sensor.x - dx)..=(self.sensor.x + dx))
        }
    }

    fn beacon_at_row(&self, y: i32) -> Option<Point> {
        if self.beacon.y == y {
            Some(self.beacon)
        } else {
            None
        }
    }

    fn edges(&self) -> Vec<Edge> {
        let top = point(self.sensor.x, self.sensor.y + self.distance);
        let bottom = point(self.sensor.x, self.sensor.y - self.distance);
        let left = point(self.sensor.x - self.distance, self.sensor.y);
        let right = point(self.sensor.x + self.distance, self.sensor.y);
        vec![
            segment(left, top).edge(Facing::Right),
            segment(top, right).edge(Facing::Left),
            segment(right, bottom).edge(Facing::Left),
            segment(bottom, left).edge(Facing::Right),
        ]
    }
}

fn ranges_for_row(readings: &[Reading], row: i32) -> Vec<RangeInclusive<i32>> {
    let mut iter = readings
        .iter()
        .flat_map(|r| r.x_range_at_row(row))
        .sorted_by_cached_key(|r| *r.start());

    let mut output = vec![];
    let mut curr = iter.next().unwrap();

    while let Some(next) = iter.next() {
        if (next.start() - curr.end()) <= 1 {
            // overlap detected, combine
            curr = *curr.start()..=*cmp::max(curr.end(), next.end());
        } else {
            // no overlap
            output.push(curr);
            curr = next;
        }
    }

    output.push(curr);
    output
}

fn num_positions_cannot_contain_beacon(readings: &[Reading], row: i32) -> usize {
    let combined_ranges = ranges_for_row(readings, row);
    let beacons: HashSet<Point> = readings.iter().flat_map(|r| r.beacon_at_row(row)).collect();

    combined_ranges
        .iter()
        .map(|r| (r.end() - r.start() + 1) as usize)
        .sum::<usize>()
        - beacons.len()
}

fn part1(input: &str, row: i32) -> usize {
    let readings = Reading::parse_list(input);
    num_positions_cannot_contain_beacon(&readings, row)
}

fn part2(input: &str, min: i32, max: i32) -> u64 {
    let readings = Reading::parse_list(input);

    let starting_shape = polygon(vec![
        point(min, min),
        point(min, max),
        point(max, max),
        point(max, min),
    ]);

    let mut polygons = vec![starting_shape];

    for reading in &readings {
        let edges = reading.edges();
        let mut new_polygons = vec![];

        for polygon in polygons {
            let mut remaining = polygon;
            for edge in &edges {
                let (outside, inside) = remaining.bisect(edge);

                if outside.is_some() {
                    new_polygons.push(outside.unwrap());
                }

                if inside.is_some() {
                    remaining = inside.unwrap();
                } else {
                    break; // nothing remaining in this polygon
                }
            }
        }

        polygons = new_polygons;
    }

    assert_eq!(polygons.len(), 1);
    let polygon = &polygons[0];
    assert_eq!(polygon.points.len(), 1);
    let point = &polygon.points[0];

    (point.x as u64) * 4000000 + (point.y as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    static EXAMPLE1: &str = indoc! {"
        Sensor at x=2, y=18: closest beacon is at x=-2, y=15
        Sensor at x=9, y=16: closest beacon is at x=10, y=16
        Sensor at x=13, y=2: closest beacon is at x=15, y=3
        Sensor at x=12, y=14: closest beacon is at x=10, y=16
        Sensor at x=10, y=20: closest beacon is at x=10, y=16
        Sensor at x=14, y=17: closest beacon is at x=10, y=16
        Sensor at x=8, y=7: closest beacon is at x=2, y=10
        Sensor at x=2, y=0: closest beacon is at x=2, y=10
        Sensor at x=0, y=11: closest beacon is at x=2, y=10
        Sensor at x=20, y=14: closest beacon is at x=25, y=17
        Sensor at x=17, y=20: closest beacon is at x=21, y=22
        Sensor at x=16, y=7: closest beacon is at x=15, y=3
        Sensor at x=14, y=3: closest beacon is at x=15, y=3
        Sensor at x=20, y=1: closest beacon is at x=15, y=3
    "};

    #[test]
    fn test_part1_example1() {
        let result = part1(EXAMPLE1, 10);
        assert_eq!(result, 26);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file(), 2000000);
        assert_eq!(result, 5564017);
    }

    #[test]
    fn test_part2_example1() {
        let result = part2(EXAMPLE1, 0, 20);
        assert_eq!(result, 56000011);
    }

    #[test]
    fn test_part2_solution() {
        let result = part2(&read_input_file(), 0, 4000000);
        assert_eq!(result, 11558423398893);
    }
}
