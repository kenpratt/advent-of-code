use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Direction {
    Up,
    Left,
    Right,
    Down,
}

static DIRECTIONS: &'static [Direction] = &[
    Direction::Up,
    Direction::Left,
    Direction::Right,
    Direction::Down,
];

#[derive(Clone, Copy, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub struct Coordinate {
    pub x: usize,
    pub y: usize,
}

impl Coordinate {
    pub fn new(x: usize, y: usize) -> Coordinate {
        Coordinate { x: x, y: y }
    }

    pub fn manhattan_distance(&self, other: &Coordinate) -> usize {
        abs_diff(self.x, other.x) + abs_diff(self.y, other.y)
    }
}

#[derive(Debug, PartialEq)]
pub struct Bounds {
    pub width: usize,
    pub height: usize,
}

fn coordinate_to_index(pos: &Coordinate, bounds: &Bounds) -> usize {
    (pos.y * bounds.width) + pos.x
}

fn index_to_coordinate(index: &usize, bounds: &Bounds) -> Coordinate {
    Coordinate::new(index % bounds.width, index / bounds.width)
}

fn neighbours<'a>(
    pos: &'a Coordinate,
    bounds: &'a Bounds,
) -> impl Iterator<Item = Coordinate> + 'a {
    DIRECTIONS.iter().filter_map(|d| neighbour(pos, d, bounds))
}

fn neighbour(pos: &Coordinate, direction: &Direction, bounds: &Bounds) -> Option<Coordinate> {
    match direction {
        Direction::Up => {
            if pos.y > 0 {
                Some(Coordinate::new(pos.x, pos.y - 1))
            } else {
                None
            }
        }
        Direction::Down => {
            if pos.y < (bounds.height - 1) {
                Some(Coordinate::new(pos.x, pos.y + 1))
            } else {
                None
            }
        }
        Direction::Left => {
            if pos.x > 0 {
                Some(Coordinate::new(pos.x - 1, pos.y))
            } else {
                None
            }
        }
        Direction::Right => {
            if pos.x < (bounds.width - 1) {
                Some(Coordinate::new(pos.x + 1, pos.y))
            } else {
                None
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Grid<T> {
    cells: Vec<Cell<T>>,
    pub bounds: Bounds,
}

impl<T> Grid<T> {
    pub fn new(values: Vec<Vec<T>>) -> Grid<T> {
        let height = values.len();
        let width = values[0].len();
        if values.iter().any(|row| row.len() != width) {
            panic!("Grid data is not rectangular");
        }
        let bounds = Bounds {
            height: height,
            width: width,
        };
        let cells = values
            .into_iter()
            .flatten()
            .enumerate()
            .map(|(index, value)| Cell {
                position: index_to_coordinate(&index, &bounds),
                value: value,
            })
            .collect();
        Grid { cells, bounds }
    }

    pub fn len(&self) -> usize {
        self.cells.len()
    }

    pub fn cell(&self, pos: &Coordinate) -> &Cell<T> {
        let index = coordinate_to_index(pos, &self.bounds);
        &self.cells[index]
    }

    pub fn cell_mut(&mut self, pos: &Coordinate) -> &mut Cell<T> {
        let index = coordinate_to_index(pos, &self.bounds);
        &mut self.cells[index]
    }

    pub fn value(&self, pos: &Coordinate) -> &T {
        &self.cell(pos).value
    }

    pub fn neighbours<'a>(&'a self, pos: &'a Coordinate) -> impl Iterator<Item = Coordinate> + 'a {
        neighbours(pos, &self.bounds)
    }

    pub fn iter(&self) -> std::slice::Iter<Cell<T>> {
        self.cells.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<Cell<T>> {
        self.cells.iter_mut()
    }
}

impl<T: fmt::Display> fmt::Display for Grid<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.bounds.height {
            for x in 0..self.bounds.width {
                write!(f, "{}", self.value(&Coordinate::new(x, y))).unwrap();
            }
            write!(f, "\n").unwrap();
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub struct Cell<T> {
    pub position: Coordinate,
    pub value: T,
}

fn abs_diff(a: usize, b: usize) -> usize {
    if a >= b {
        a - b
    } else {
        b - a
    }
}
