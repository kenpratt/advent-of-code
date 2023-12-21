use std::collections::HashMap;

#[derive(Debug)]
pub struct Grid<T> {
    pub cells: HashMap<Coord, T>,
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

    pub fn neighbours(&self, pos: &Coord) -> Vec<Coord> {
        ALL_DIRECTIONS
            .iter()
            .flat_map(|d| self.shift(pos, d))
            .collect()
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
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Direction {
    North,
    West,
    South,
    East,
}

pub const ALL_DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::West,
    Direction::South,
    Direction::East,
];
