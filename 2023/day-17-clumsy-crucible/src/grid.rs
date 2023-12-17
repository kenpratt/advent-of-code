use std::collections::HashMap;

#[derive(Debug)]
pub struct Grid<T> {
    cells: HashMap<Coord, T>,
    pub width: usize,
    pub height: usize,
}

impl<T> Grid<T> {
    pub fn parse(input: &str, parse_value: fn(&char) -> T) -> Self {
        let width = input.lines().next().unwrap().len();
        let height = input.lines().count();

        let mut cells = HashMap::new();

        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let pos = Coord::new(x, y);
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

    pub fn shift(&self, pos: &Coord, dir: &Direction) -> Option<Coord> {
        if (dir == &Direction::West && pos.x == 0)
            || (dir == &Direction::North && pos.y == 0)
            || (dir == &Direction::East && pos.x == (self.width - 1))
            || (dir == &Direction::South && pos.y == (self.height - 1))
        {
            None
        } else {
            Some(pos.shift(dir))
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn shift(&self, direction: &Direction) -> Self {
        use Direction::*;
        match direction {
            North => Self::new(self.x, self.y - 1),
            West => Self::new(self.x - 1, self.y),
            South => Self::new(self.x, self.y + 1),
            East => Self::new(self.x + 1, self.y),
        }
    }

    pub fn manhattan_distance(&self, other: &Coord) -> usize {
        abs_diff(self.x, other.x) + abs_diff(self.y, other.y)
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
    pub fn clockwise(&self) -> Self {
        use Direction::*;

        match self {
            North => East,
            West => North,
            South => West,
            East => South,
        }
    }

    pub fn counterclockwise(&self) -> Self {
        use Direction::*;

        match self {
            North => West,
            West => South,
            South => East,
            East => North,
        }
    }
}

fn abs_diff(x: usize, y: usize) -> usize {
    if x <= y {
        y - x
    } else {
        x - y
    }
}
