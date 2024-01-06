use std::collections::HashMap;

use cached::proc_macro::cached;

#[derive(Debug)]
pub struct Grid<T> {
    pub cells: HashMap<Coord, T>,
    pub width: i16,
    pub height: i16,
}

impl<T> Grid<T> {
    pub fn parse(input: &str, parse_value: fn(&char) -> T) -> Self {
        let width = input.lines().next().unwrap().len() as i16;
        let height = input.lines().count() as i16;

        let mut cells = HashMap::new();

        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let pos = Coord::new(x as i16, y as i16);
                let cell = parse_value(&c);
                cells.insert(pos, cell);
            }
        }

        Self {
            cells,
            width,
            height,
        }
    }

    pub fn value(&self, pos: &Coord) -> &T {
        self.cells.get(pos).unwrap()
    }

    pub fn neighbours(&self, pos: &Coord) -> Vec<(Coord, Option<Direction>)> {
        neighbours(*pos, self.width, self.height)
    }
}

#[cached]
fn neighbours(pos: Coord, width: i16, height: i16) -> Vec<(Coord, Option<Direction>)> {
    ALL_DIRECTIONS
        .iter()
        .map(|d: &Direction| neighbour(pos, *d, width, height))
        .collect()
}

#[cached]
fn neighbour(
    pos: Coord,
    direction: Direction,
    width: i16,
    height: i16,
) -> (Coord, Option<Direction>) {
    let r = width - 1;
    let b = height - 1;
    match (direction, pos.x, pos.y) {
        (Direction::North, x, 0) => (Coord::new(x, b), Some(Direction::North)),
        (Direction::West, 0, y) => (Coord::new(r, y), Some(Direction::West)),
        (Direction::South, x, y) if y == b => (Coord::new(x, 0), Some(Direction::South)),
        (Direction::East, x, y) if x == r => (Coord::new(0, y), Some(Direction::East)),
        _ => (pos.shift(&direction), None),
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Coord {
    pub x: i16,
    pub y: i16,
}

impl Coord {
    pub fn new(x: i16, y: i16) -> Self {
        Self { x, y }
    }

    pub fn shift(&self, direction: &Direction) -> Self {
        use Direction::*;
        match direction {
            North => Self::new(self.x, self.y - 1),
            West => Self::new(self.x - 1, self.y),
            South => Self::new(self.x, self.y + 1),
            East => Self::new(self.x + 1, self.y),
        }
    }

    pub fn neighbours(&self) -> Vec<Coord> {
        ALL_DIRECTIONS.iter().map(|d| self.shift(d)).collect()
    }

    pub fn manhattan_distance(&self, other: &Coord) -> usize {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as usize
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Direction {
    North,
    West,
    South,
    East,
}

impl Direction {
    pub fn index(&self) -> usize {
        match self {
            Direction::North => 0,
            Direction::West => 1,
            Direction::South => 2,
            Direction::East => 3,
        }
    }

    pub fn from_index(i: usize) -> Self {
        match i {
            0 => Direction::North,
            1 => Direction::West,
            2 => Direction::South,
            3 => Direction::East,
            _ => panic!("Unexpected direction index: {}", i),
        }
    }

    pub fn rev(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::West => Direction::East,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
        }
    }
}

pub const ALL_DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::West,
    Direction::South,
    Direction::East,
];
