mod grid;
mod part1;
mod part2;

use std::fs;
use crate::grid::*;

fn main() {
    // read input & split into lines
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let lines = contents.lines();

    let locations: Vec<Point> = lines.map(|line| parse_line(line)).collect();
    println!("locations: {:?}", locations);

    let part1 = part1::run(&locations);
    println!("part 1: {:?}", part1);

    let part2 = part2::run(&locations);
    println!("part 2: {:?}", part2);
}

fn parse_line(line: &str) -> Point {
    let parts: Vec<&str> = line.split(", ").collect();
    assert_eq!(parts.len(), 2);

    let x = parts[0].parse::<u16>().unwrap();
    let y = parts[1].parse::<u16>().unwrap();

    return Point::new(x, y);
}
