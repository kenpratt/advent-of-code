use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    // println!("part 2 result: {:?}", part2(&read_input_file()));
}

fn read_input_file() -> String {
    fs::read_to_string("input.txt").expect("Something went wrong reading the file")
}

#[derive(Debug)]
struct Simulation<'a> {
    grid: &'a Grid,
    beams: Vec<Beam>,
    visited: HashSet<(Coord, Direction)>,
}

impl<'a> Simulation<'a> {
    fn count_energized(grid: &'a Grid) -> usize {
        let mut s = Self::new(grid);
        s.run();

        // return number of unique positions visited by beams
        s.visited
            .iter()
            .map(|(pos, _dir)| pos)
            .collect::<HashSet<&Coord>>()
            .len()
    }

    fn new(grid: &'a Grid) -> Simulation<'a> {
        let initial = (Coord::new(0, 0), Direction::East);
        let beams = vec![Beam::new(&initial)];
        let visited = HashSet::from([initial]);

        Self {
            grid,
            beams,
            visited,
        }
    }

    fn run(&mut self) {
        while self.beams.iter().any(|b| b.active) {
            self.tick();
        }
    }

    fn tick(&mut self) {
        let mut new_beams = vec![];

        for beam in &mut self.beams {
            if beam.active {
                match Self::evaluate(&self.grid, &beam.curr) {
                    (None, None) => {
                        // beam is finished
                        beam.active = false;
                    }
                    (Some(new_state), None) | (None, Some(new_state)) => {
                        // beam moves onwards
                        beam.move_to(new_state.clone(), self.visited.contains(&new_state));
                        self.visited.insert(new_state);
                    }
                    (Some(new_state1), Some(new_state2)) => {
                        // beam split!
                        let mut new_beam = beam.clone();
                        new_beam.move_to(new_state1.clone(), self.visited.contains(&new_state1));
                        self.visited.insert(new_state1);
                        new_beams.push(new_beam);

                        beam.move_to(new_state2.clone(), self.visited.contains(&new_state2));
                        self.visited.insert(new_state2);
                    }
                }
            }
        }

        self.beams.append(&mut new_beams);
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

#[derive(Clone, Debug)]
struct Beam {
    active: bool,
    curr: (Coord, Direction),
    path: Vec<(Coord, Direction)>,
}

impl Beam {
    fn new(initial: &(Coord, Direction)) -> Self {
        Self {
            active: true,
            curr: *initial,
            path: vec![*initial],
        }
    }

    fn move_to(&mut self, new_state: (Coord, Direction), someone_visited_already: bool) {
        self.curr = new_state.clone();
        self.path.push(new_state);

        if someone_visited_already {
            // we looped back on a previous position & direction
            self.active = false;
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
    Simulation::count_energized(&grid)
}

// fn part2(input: &str) -> usize {
//     let grids = Data::parse(input);
//     0
// }

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

    // #[test]
    // fn test_part2_example() {
    //     let result = part2(EXAMPLE);
    //     assert_eq!(result, 0);
    // }

    // #[test]
    // fn test_part2_solution() {
    //     let result = part2(&read_input_file());
    //     assert_eq!(result, 0);
    // }
}
