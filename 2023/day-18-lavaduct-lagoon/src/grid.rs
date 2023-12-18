#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

impl Coord {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn shift(&self, direction: &Direction, distance: &i32) -> Self {
        use Direction::*;
        match direction {
            North => Self::new(self.x, self.y - distance),
            West => Self::new(self.x - distance, self.y),
            South => Self::new(self.x, self.y + distance),
            East => Self::new(self.x + distance, self.y),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
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
