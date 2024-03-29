use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    println!("part 2 result: {:?}", part2(&read_input_file()));
}

fn read_input_file() -> String {
    fs::read_to_string("input.txt").expect("Something went wrong reading the file")
}

#[derive(Debug)]
struct Simulation<'a> {
    grid: &'a Grid,
    pending_beams: Vec<(Coord, Direction)>,
    visited: HashSet<(Coord, Direction)>,
}

impl<'a> Simulation<'a> {
    fn count_energized(initial: &(Coord, Direction), grid: &'a Grid) -> usize {
        let mut s = Self::new(initial, grid);
        s.run();

        // return number of unique positions visited by beams
        s.visited
            .iter()
            .map(|(pos, _dir)| pos)
            .collect::<HashSet<&Coord>>()
            .len()
    }

    fn new(initial: &(Coord, Direction), grid: &'a Grid) -> Simulation<'a> {
        let pending_beams = vec![*initial];
        let visited = HashSet::from([*initial]);

        Self {
            grid,
            pending_beams,
            visited,
        }
    }

    fn run(&mut self) {
        while !self.pending_beams.is_empty() {
            let beam = self.pending_beams.pop().unwrap();
            self.run_beam(beam);
        }
    }

    fn run_beam(&mut self, mut beam: (Coord, Direction)) {
        let mut active = true;
        while active {
            match Self::evaluate(&self.grid, &beam) {
                (None, None) => {
                    // beam is finished
                    active = false;
                }
                (Some(new_beam), None) | (None, Some(new_beam)) => {
                    // beam moves onwards
                    if !self.visited.contains(&new_beam) {
                        self.visited.insert(new_beam.clone());
                        beam = new_beam;
                    } else {
                        active = false;
                    }
                }
                (Some(new_beam1), Some(new_beam2)) => {
                    match (
                        self.visited.contains(&new_beam1),
                        self.visited.contains(&new_beam2),
                    ) {
                        (true, true) => active = false,
                        (true, false) => {
                            // not a real split
                            self.visited.insert(new_beam2.clone());
                            beam = new_beam2;
                        }
                        (false, true) => {
                            // not a real split
                            self.visited.insert(new_beam1.clone());
                            beam = new_beam1;
                        }
                        (false, false) => {
                            // real split!
                            self.visited.insert(new_beam1.clone());
                            beam = new_beam1;

                            self.visited.insert(new_beam2.clone());
                            self.pending_beams.push(new_beam2);
                        }
                    }
                }
            }
        }
    }

    fn evaluate(
        grid: &Grid,
        curr: &(Coord, Direction),
    ) -> (Option<(Coord, Direction)>, Option<(Coord, Direction)>) {
        let (curr_pos, curr_direction) = curr;
        let curr_cell = grid.value(curr_pos);
        let (new_dir1, maybe_new_dir2) = curr_cell.next_directions(curr_direction);

        (
            Self::advance(grid, curr, new_dir1),
            maybe_new_dir2.map_or(None, |new_dir2| Self::advance(grid, curr, new_dir2)),
        )
    }

    fn advance(
        grid: &Grid,
        curr: &(Coord, Direction),
        new_direction: Direction,
    ) -> Option<(Coord, Direction)> {
        let (curr_pos, _curr_direction) = curr;
        match grid.shift(curr_pos, &new_direction) {
            Some(new_pos) => Some((new_pos, new_direction)),
            None => None,
        }
    }
}

#[derive(Debug)]
struct Grid {
    cells: HashMap<Coord, Cell>,
    width: usize,
    height: usize,
}

impl Grid {
    fn parse(input: &str) -> Self {
        let width = input.lines().next().unwrap().len();
        let height = input.lines().count();

        let mut cells = HashMap::new();

        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let pos = Coord::new(x, y);
                let cell = Cell::parse(&c);
                cells.insert(pos, cell);
            }
        }

        Self {
            cells,
            width,
            height,
        }
    }

    fn value(&self, pos: &Coord) -> &Cell {
        self.cells.get(pos).unwrap()
    }

    fn shift(&self, pos: &Coord, dir: &Direction) -> Option<Coord> {
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
enum Cell {
    Empty,
    MirrorNorthWest,
    MirrorNorthEast,
    SplitterVertical,
    SplitterHorizontal,
}

impl Cell {
    fn parse(c: &char) -> Self {
        use Cell::*;

        match c {
            '.' => Empty,
            '/' => MirrorNorthWest,
            '\\' => MirrorNorthEast,
            '|' => SplitterVertical,
            '-' => SplitterHorizontal,
            _ => panic!("Unknown cell value: {:?}", c),
        }
    }

    fn next_directions(&self, curr_direction: &Direction) -> (Direction, Option<Direction>) {
        use Cell::*;
        use Direction::*;

        match (self, curr_direction) {
            (Empty, _) | (SplitterVertical, North | South) | (SplitterHorizontal, West | East) => {
                (*curr_direction, None)
            }
            (SplitterVertical, West | East) => (North, Some(South)),
            (SplitterHorizontal, North | South) => (West, Some(East)),
            (MirrorNorthWest, North) => (East, None),
            (MirrorNorthWest, West) => (South, None),
            (MirrorNorthWest, South) => (West, None),
            (MirrorNorthWest, East) => (North, None),
            (MirrorNorthEast, North) => (West, None),
            (MirrorNorthEast, West) => (North, None),
            (MirrorNorthEast, South) => (East, None),
            (MirrorNorthEast, East) => (South, None),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn new(x: usize, y: usize) -> Self {
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
enum Direction {
    North,
    West,
    South,
    East,
}

fn part1(input: &str) -> usize {
    let grid = Grid::parse(input);
    Simulation::count_energized(&(Coord::new(0, 0), Direction::East), &grid)
}

fn part2(input: &str) -> usize {
    let grid = Grid::parse(input);

    let mut to_try = vec![];
    for x in 0..grid.width {
        to_try.push((Coord::new(x, 0), Direction::South));
        to_try.push((Coord::new(x, grid.height - 1), Direction::North));
    }
    for y in 0..grid.height {
        to_try.push((Coord::new(0, y), Direction::East));
        to_try.push((Coord::new(grid.width - 1, y), Direction::West));
    }

    to_try
        .iter()
        .map(|initial| Simulation::count_energized(initial, &grid))
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read_example_file() -> String {
        fs::read_to_string("example.txt").expect("Something went wrong reading the file")
    }

    #[test]
    fn test_part1_example() {
        let result = part1(&read_example_file());
        assert_eq!(result, 46);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file());
        assert_eq!(result, 6795);
    }

    #[test]
    fn test_part2_example() {
        let result = part2(&read_example_file());
        assert_eq!(result, 51);
    }

    #[test]
    fn test_part2_solution() {
        let result = part2(&read_input_file());
        assert_eq!(result, 7154);
    }
}
