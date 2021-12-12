#[derive(Debug, PartialEq)]
pub enum Direction {
    UpLeft,
    Up,
    UpRight,
    Left,
    Right,
    DownLeft,
    Down,
    DownRight,
}

static DIRECTIONS: &'static [Direction] = &[
    Direction::UpLeft,
    Direction::Up,
    Direction::UpRight,
    Direction::Left,
    Direction::Right,
    Direction::DownLeft,
    Direction::Down,
    Direction::DownRight,
];

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    fn new(x: usize, y: usize) -> Coordinate {
        Coordinate { x: x, y: y }
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

fn decrease(v: usize, limit: usize) -> Option<usize> {
    if v > limit {
        Some(v - 1)
    } else {
        None
    }
}

fn increase(v: usize, limit: usize) -> Option<usize> {
    if v < (limit - 1) {
        Some(v + 1)
    } else {
        None
    }
}

fn maybe_coord(x: Option<usize>, y: Option<usize>) -> Option<Coordinate> {
    if x.is_some() && y.is_some() {
        Some(Coordinate::new(x.unwrap(), y.unwrap()))
    } else {
        None
    }
}

fn neighbour(pos: &Coordinate, direction: &Direction, bounds: &Bounds) -> Option<Coordinate> {
    match direction {
        Direction::UpLeft => maybe_coord(decrease(pos.x, 0), decrease(pos.y, 0)),
        Direction::Up => maybe_coord(Some(pos.x), decrease(pos.y, 0)),
        Direction::UpRight => maybe_coord(increase(pos.x, bounds.width), decrease(pos.y, 0)),
        Direction::Left => maybe_coord(decrease(pos.x, 0), Some(pos.y)),
        Direction::Right => maybe_coord(increase(pos.x, bounds.width), Some(pos.y)),
        Direction::DownLeft => maybe_coord(decrease(pos.x, 0), increase(pos.y, bounds.height)),
        Direction::Down => maybe_coord(Some(pos.x), increase(pos.y, bounds.height)),
        Direction::DownRight => maybe_coord(
            increase(pos.x, bounds.width),
            increase(pos.y, bounds.height),
        ),
    }
}

#[derive(Debug, PartialEq)]
pub struct Grid<T> {
    cells: Vec<Cell<T>>,
    bounds: Bounds,
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
        Grid {
            cells: values
                .into_iter()
                .flatten()
                .enumerate()
                .map(|(index, value)| Cell {
                    position: index_to_coordinate(&index, &bounds),
                    value: value,
                })
                .collect(),
            bounds: bounds,
        }
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

    pub fn neighbours(&self, pos: &Coordinate) -> Vec<Coordinate> {
        neighbours(pos, &self.bounds)
    }

    pub fn iter(&self) -> std::slice::Iter<Cell<T>> {
        self.cells.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<Cell<T>> {
        self.cells.iter_mut()
    }

    pub fn render(&self, value_to_str: fn(&T) -> String) -> String {
        (0..(self.bounds.height))
            .map(|y| {
                (0..(self.bounds.width))
                    .map(|x| value_to_str(self.value(&Coordinate::new(x, y))))
                    .collect::<Vec<String>>()
                    .join("")
            })
            .collect::<Vec<String>>()
            .join("\n")
    }
}

#[derive(Debug, PartialEq)]
pub struct Cell<T> {
    pub position: Coordinate,
    pub value: T,
}
