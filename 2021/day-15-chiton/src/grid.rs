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
    x: usize,
    y: usize,
}

impl Coordinate {
    fn new(x: usize, y: usize) -> Coordinate {
        Coordinate { x: x, y: y }
    }

    pub fn manhattan_distance(&self, other: &Coordinate) -> usize {
        abs_diff(self.x, other.x) + abs_diff(self.y, other.y)
    }
}

#[derive(Debug, PartialEq)]
pub struct Bounds {
    width: usize,
    height: usize,
}

fn coordinate_to_index(pos: &Coordinate, bounds: &Bounds) -> usize {
    (pos.y * bounds.width) + pos.x
}

fn index_to_coordinate(index: &usize, bounds: &Bounds) -> Coordinate {
    Coordinate::new(index % bounds.width, index / bounds.width)
}

fn neighbours(pos: &Coordinate, bounds: &Bounds) -> Vec<Coordinate> {
    DIRECTIONS
        .iter()
        .filter_map(|d| neighbour(pos, d, bounds))
        .collect()
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
    cells: Vec<T>,
    bounds: Bounds,
}

impl<T> Grid<T> {
    pub fn new(values: Vec<Vec<T>>) -> Grid<T> {
        let height = values.len();
        let width = values[0].len();
        if values.iter().any(|row| row.len() != width) {
            panic!("Grid data is not rectangular");
        }
        Grid {
            cells: values.into_iter().flatten().collect(),
            bounds: Bounds {
                height: height,
                width: width,
            },
        }
    }

    fn len(&self) -> usize {
        self.cells.len()
    }

    pub fn value(&self, pos: &Coordinate) -> &T {
        let index = coordinate_to_index(pos, &self.bounds);
        &self.cells[index]
    }

    pub fn neighbours(&self, pos: &Coordinate) -> Vec<Coordinate> {
        neighbours(pos, &self.bounds)
    }

    pub fn top_left(&self) -> Coordinate {
        Coordinate::new(0, 0)
    }

    pub fn bottom_right(&self) -> Coordinate {
        Coordinate::new(self.bounds.width - 1, self.bounds.height - 1)
    }

    pub fn iter(&self) -> GridIterator<'_, T> {
        GridIterator {
            grid: self,
            index: 0,
        }
    }
}

pub struct GridIterator<'a, T> {
    grid: &'a Grid<T>,
    index: usize,
}

impl<'a, T> Iterator for GridIterator<'a, T> {
    type Item = (Coordinate, &'a T);
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.grid.len() {
            let pos = index_to_coordinate(&self.index, &self.grid.bounds);
            let value = &self.grid.cells[self.index];
            let result = (pos, value);
            self.index += 1;
            Some(result)
        } else {
            None
        }
    }
}

fn abs_diff(a: usize, b: usize) -> usize {
    if a >= b {
        a - b
    } else {
        b - a
    }
}
