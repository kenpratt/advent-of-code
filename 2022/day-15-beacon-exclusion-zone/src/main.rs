pub mod coordinate;

use coordinate::*;

use std::cmp;
use std::collections::HashSet;
use std::fs;
use std::ops::RangeInclusive;

use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file(), 2000000));
    // println!("part 2 result: {:?}", part2(&read_input_file()));
}

fn read_input_file() -> String {
    fs::read_to_string("input.txt").expect("Something went wrong reading the file")
}

#[derive(Debug)]
struct Reading {
    sensor: Coordinate,
    beacon: Coordinate,
}

impl Reading {
    fn parse_list(input: &str) -> Vec<Self> {
        input.lines().map(|line| Self::parse(line)).collect()
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

        let sensor = Coordinate::new(nums[0], nums[1]);
        let beacon = Coordinate::new(nums[2], nums[3]);

        Self { sensor, beacon }
    }

    fn distance(&self) -> i32 {
        self.sensor.manhattan_distance(&self.beacon)
    }

    fn x_range_at_row(&self, y: i32) -> Option<RangeInclusive<i32>> {
        let beacon_distance = self.distance();
        let dy = abs_diff(self.sensor.y, y);
        if dy > beacon_distance {
            None
        } else {
            let dx = beacon_distance - dy;
            Some((self.sensor.x - dx)..=(self.sensor.x + dx))
        }
    }

    fn beacon_at_row(&self, y: i32) -> Option<Coordinate> {
        if self.beacon.y == y {
            Some(self.beacon)
        } else {
            None
        }
    }
}

fn num_positions_cannot_contain_beacon(readings: &[Reading], row: i32) -> usize {
    let ranges: Vec<RangeInclusive<i32>> = readings
        .iter()
        .flat_map(|r| r.x_range_at_row(row))
        .collect();

    let combined_ranges = combine_ranges(ranges);
    let beacons: HashSet<Coordinate> = readings.iter().flat_map(|r| r.beacon_at_row(row)).collect();

    combined_ranges
        .iter()
        .map(|r| (r.end() - r.start() + 1) as usize)
        .sum::<usize>()
        - beacons.len()
}

fn combine_ranges(mut ranges: Vec<RangeInclusive<i32>>) -> Vec<RangeInclusive<i32>> {
    ranges.sort_by(|a, b| {
        if a.start() == b.start() {
            a.end().cmp(b.end())
        } else {
            a.start().cmp(b.start())
        }
    });

    let mut output = vec![];

    let mut iter = ranges.into_iter();
    let mut curr = iter.next().unwrap();

    while let Some(next) = iter.next() {
        if next.start() <= curr.end() {
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

fn part1(input: &str, row: i32) -> usize {
    let readings = Reading::parse_list(input);
    num_positions_cannot_contain_beacon(&readings, row)
}

// fn part2(input: &str) -> usize {
//     let data = Data::parse(input);
//     dbg!(&data);
//     data.execute()
// }

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
