use std::{
    collections::{HashMap, HashSet},
    fs,
    ops::Sub,
};

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    println!("part 2 result: {:?}", part2(&read_input_file()));
}

fn read_input_file() -> String {
    fs::read_to_string("input.txt").expect("Something went wrong reading the file")
}

#[derive(Debug)]
struct Maze {
    cells: HashMap<Location, Tile>,
    starting_position: Location,
}

impl Maze {
    fn parse(input: &str) -> Self {
        let mut cells = HashMap::new();
        let mut starting_position = None;

        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let pos = Location::new(x as isize, y as isize);

                match Tile::parse(&c) {
                    Some(Tile::Starting) => {
                        starting_position = Some(pos.clone());
                        cells.insert(pos, Tile::Starting)
                    }
                    Some(t) => cells.insert(pos, t),
                    None => None, // ground
                };
            }
        }

        Self {
            cells,
            starting_position: starting_position.unwrap(),
        }
    }

    fn neighbours(&self, pos: &Location, exclude: Option<&Location>) -> Vec<Location> {
        let tile = self.cells.get(pos).unwrap();
        pos.neighbours(&tile.adjacent())
            .into_iter()
            .filter(|pos| Some(pos) != exclude)
            .filter(|pos| self.cells.contains_key(pos))
            .collect()
    }

    fn find_main_loop(&self) -> Option<Vec<Location>> {
        let neighbours = self.neighbours(&self.starting_position, None);

        // exclude neighbours that don't connect back to the starting position
        let initial_positions: Vec<Location> = neighbours
            .into_iter()
            .filter(|neighbour| {
                self.neighbours(neighbour, None)
                    .contains(&self.starting_position)
            })
            .collect();

        for initial_position in initial_positions {
            match self.trace(&initial_position) {
                Some(path) => return Some(path),
                None => (), // continue searching
            }
        }

        None
    }

    fn trace(&self, initial_position: &Location) -> Option<Vec<Location>> {
        let mut last_pos = self.starting_position;
        let mut curr_pos = *initial_position;
        let mut path = vec![*initial_position];

        loop {
            let mut next_positions = self.neighbours(&curr_pos, Some(&last_pos));
            match next_positions.len() {
                0 => {
                    // nowhere left to go - must be a dead end
                    return None;
                }
                1 => {
                    let next_pos = next_positions.pop().unwrap();
                    path.push(next_pos.clone());

                    if next_pos == self.starting_position {
                        // found a way back to the start!
                        return Some(path);
                    } else {
                        // keep on navigating
                        last_pos = curr_pos;
                        curr_pos = next_pos;
                    }
                }
                _ => panic!(
                    "There should not be more than 1 possible next position: {:?}",
                    next_positions
                ),
            }
        }
    }

    fn infer_starting_tile_type(&self, main_loop: &Vec<Location>) -> Tile {
        use Direction::*;

        assert_eq!(main_loop[main_loop.len() - 1], self.starting_position);

        let a_location = &main_loop[0];
        let b_location = &main_loop[main_loop.len() - 2];

        let a_direction = Direction::from_diff(&(*a_location - self.starting_position));
        let b_direction = Direction::from_diff(&(*b_location - self.starting_position));

        match (&a_direction, &b_direction) {
            (North, South) | (South, North) => Tile::Vertical,
            (West, East) | (East, West) => Tile::Horizontal,
            (East, North) | (North, East) => Tile::BendL,
            (West, North) | (North, West) => Tile::BendJ,
            (East, South) | (South, East) => Tile::BendF,
            (West, South) | (South, West) => Tile::Bend7,
            _ => panic!(
                "Impossible starting location connections: {:?}, {:?}",
                a_direction, b_direction
            ),
        }
    }

    fn count_enclosed_tiles(&self, main_loop: &Vec<Location>) -> usize {
        let starting_tile_type = self.infer_starting_tile_type(main_loop);

        let mut tile_types = self.cells.clone();
        tile_types.insert(self.starting_position, starting_tile_type);

        let main_loop_set: HashSet<Location> = main_loop.iter().cloned().collect();

        // find leftmost, topmost tile
        let min_x = main_loop_set.iter().map(|l| l.x).min().unwrap();
        let initial_location = main_loop_set
            .iter()
            .filter(|l| l.x == min_x)
            .min_by_key(|l| l.y)
            .unwrap();

        // must be bendF because it's the leftmost, topmost tile
        let initial_tile_type = tile_types.get(&initial_location).unwrap();
        assert_eq!(*initial_tile_type, Tile::BendF);

        let initial_facing = Facing::SouthEast;

        // go clockwise through main loop
        let initial_index = main_loop
            .iter()
            .position(|l| l == initial_location)
            .unwrap();
        let mut loop_order: Vec<usize> = (initial_index + 1..main_loop.len()).collect();
        loop_order.extend(0..initial_index);

        // build the loop with metadata about tile facing
        let mut loop_with_facing: Vec<(Location, Tile, Facing)> =
            vec![(*initial_location, *initial_tile_type, initial_facing)];
        for loop_index in &loop_order {
            let (last_location, last_tile_type, last_facing) = loop_with_facing.last().unwrap();

            let curr_location = &main_loop[*loop_index];
            let curr_tile_type = tile_types.get(curr_location).unwrap();

            let departing_direction = Direction::from_diff(&(*curr_location - *last_location));
            let departing_facing =
                last_tile_type.departing_facing(&last_facing, &departing_direction);

            let curr_facing =
                curr_tile_type.incoming_facing(&departing_direction, &departing_facing);

            loop_with_facing.push((*curr_location, *curr_tile_type, curr_facing));
        }

        // find locations adjacent to the main loop to visit
        let mut to_visit: HashSet<Location> = HashSet::new();
        for (location, tile_type, facing) in &loop_with_facing {
            for l in tile_type.inside(facing, location) {
                if !main_loop_set.contains(&l) {
                    to_visit.insert(l);
                }
            }
        }

        // search for more stuff inside the shape
        let mut inside_shape: HashSet<Location> = to_visit.clone();
        while !to_visit.is_empty() {
            let curr_location = *to_visit.iter().next().unwrap();
            to_visit.remove(&curr_location);

            let neighbours = curr_location.neighbours(&ALL_DIRECTIONS);
            for neighbour in &neighbours {
                // add any new locations to the search
                if !main_loop_set.contains(neighbour) && !inside_shape.contains(neighbour) {
                    inside_shape.insert(*neighbour);
                    to_visit.insert(*neighbour);
                }
            }
        }
        inside_shape.len()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Tile {
    Vertical,
    Horizontal,
    BendL,
    BendJ,
    Bend7,
    BendF,
    Starting,
}

impl Tile {
    fn parse(c: &char) -> Option<Tile> {
        use Tile::*;

        match c {
            '|' => Some(Vertical),
            '-' => Some(Horizontal),
            'L' => Some(BendL),
            'J' => Some(BendJ),
            '7' => Some(Bend7),
            'F' => Some(BendF),
            'S' => Some(Starting),
            '.' => None,
            _ => panic!("Unknown tile: {}", c),
        }
    }

    fn adjacent(&self) -> Vec<Direction> {
        use Direction::*;
        use Tile::*;

        match self {
            Vertical => vec![North, South],
            Horizontal => vec![East, West],
            BendL => vec![North, East],
            BendJ => vec![North, West],
            Bend7 => vec![South, West],
            BendF => vec![South, East],
            Starting => vec![North, East, South, West],
        }
    }

    fn departing_facing(&self, facing: &Facing, departing_direction: &Direction) -> Facing {
        use Tile::*;

        match (self, facing, departing_direction) {
            (Vertical | Horizontal, f, _) => *f,

            (BendL, Facing::NorthEast, Direction::North) => Facing::East,
            (BendL, Facing::NorthEast, Direction::East) => Facing::North,
            (BendL, Facing::SouthWest, Direction::North) => Facing::West,
            (BendL, Facing::SouthWest, Direction::East) => Facing::South,

            (BendJ, Facing::NorthWest, Direction::North) => Facing::West,
            (BendJ, Facing::NorthWest, Direction::West) => Facing::North,
            (BendJ, Facing::SouthEast, Direction::North) => Facing::East,
            (BendJ, Facing::SouthEast, Direction::West) => Facing::South,

            (Bend7, Facing::NorthEast, Direction::South) => Facing::East,
            (Bend7, Facing::NorthEast, Direction::West) => Facing::North,
            (Bend7, Facing::SouthWest, Direction::South) => Facing::West,
            (Bend7, Facing::SouthWest, Direction::West) => Facing::South,

            (BendF, Facing::NorthWest, Direction::South) => Facing::West,
            (BendF, Facing::NorthWest, Direction::East) => Facing::North,
            (BendF, Facing::SouthEast, Direction::South) => Facing::East,
            (BendF, Facing::SouthEast, Direction::East) => Facing::South,

            (Starting, _, _) => panic!("Facing not implemented for Starting tile"),

            _ => panic!(
                "Impossible tile/facing/departing direction combination: {:?}, {:?}, {:?}",
                self, facing, departing_direction
            ),
        }
    }

    fn incoming_facing(&self, departing_direcion: &Direction, departing_facing: &Facing) -> Facing {
        use Tile::*;

        match (self, departing_direcion, departing_facing) {
            (Vertical | Horizontal, _, f) => *f,

            (BendL, Direction::South, Facing::West) => Facing::SouthWest,
            (BendL, Direction::South, Facing::East) => Facing::NorthEast,
            (BendL, Direction::West, Facing::North) => Facing::NorthEast,
            (BendL, Direction::West, Facing::South) => Facing::SouthWest,

            (BendJ, Direction::South, Facing::West) => Facing::NorthWest,
            (BendJ, Direction::South, Facing::East) => Facing::SouthEast,
            (BendJ, Direction::East, Facing::North) => Facing::NorthWest,
            (BendJ, Direction::East, Facing::South) => Facing::SouthEast,

            (Bend7, Direction::North, Facing::West) => Facing::SouthWest,
            (Bend7, Direction::North, Facing::East) => Facing::NorthEast,
            (Bend7, Direction::East, Facing::North) => Facing::NorthEast,
            (Bend7, Direction::East, Facing::South) => Facing::SouthWest,

            (BendF, Direction::North, Facing::West) => Facing::NorthWest,
            (BendF, Direction::North, Facing::East) => Facing::SouthEast,
            (BendF, Direction::West, Facing::North) => Facing::NorthWest,
            (BendF, Direction::West, Facing::South) => Facing::SouthEast,

            (Starting, _, _) => panic!("Facing not implemented for Starting tile"),

            _ => panic!(
                "Impossible tile/departing direction/departing facing combination: {:?}, {:?}, {:?}",
                self, departing_direcion, departing_facing
            ),
        }
    }

    fn inside(&self, facing: &Facing, location: &Location) -> Vec<Location> {
        use Tile::*;

        match (self, facing) {
            (Vertical, Facing::West) => location.at_facings(&[Facing::West]),
            (Vertical, Facing::East) => location.at_facings(&[Facing::East]),

            (Horizontal, Facing::North) => location.at_facings(&[Facing::North]),
            (Horizontal, Facing::South) => location.at_facings(&[Facing::South]),

            (BendL, Facing::NorthEast) => location.at_facings(&[Facing::NorthEast]),
            (BendL, Facing::SouthWest) => {
                location.at_facings(&[Facing::West, Facing::SouthWest, Facing::South])
            }

            (BendJ, Facing::NorthWest) => location.at_facings(&[Facing::NorthWest]),
            (BendJ, Facing::SouthEast) => {
                location.at_facings(&[Facing::East, Facing::SouthEast, Facing::South])
            }

            (Bend7, Facing::NorthEast) => {
                location.at_facings(&[Facing::North, Facing::NorthEast, Facing::East])
            }
            (Bend7, Facing::SouthWest) => location.at_facings(&[Facing::SouthWest]),

            (BendF, Facing::NorthWest) => {
                location.at_facings(&[Facing::North, Facing::NorthWest, Facing::West])
            }
            (BendF, Facing::SouthEast) => location.at_facings(&[Facing::SouthEast]),

            (Starting, _) => panic!("Facing not implemented for Starting tile"),

            _ => panic!(
                "Impossible tile/facing combination: {:?}, {:?}",
                self, facing
            ),
        }
    }
}

#[derive(Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

const ALL_DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::East,
    Direction::South,
    Direction::West,
];

impl Direction {
    fn from_diff(diff: &Location) -> Self {
        use Direction::*;

        match (diff.x, diff.y) {
            (0, -1) => North,
            (0, 1) => South,
            (-1, 0) => West,
            (1, 0) => East,
            _ => panic!("Bad direction diff: {:?}", diff),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Facing {
    North,
    East,
    South,
    West,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct Location {
    x: isize,
    y: isize,
}

impl Location {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn neighbours(&self, directions: &[Direction]) -> Vec<Self> {
        directions.iter().map(|d| self.neighbour(d)).collect()
    }

    fn neighbour(&self, direction: &Direction) -> Self {
        use Direction::*;

        let Location { x, y } = self;

        match direction {
            North => Location::new(*x, y - 1),
            East => Location::new(x + 1, *y),
            South => Location::new(*x, y + 1),
            West => Location::new(x - 1, *y),
        }
    }

    fn at_facings(&self, facings: &[Facing]) -> Vec<Self> {
        facings.iter().map(|f| self.at_facing(f)).collect()
    }

    fn at_facing(&self, facing: &Facing) -> Self {
        use Facing::*;

        let Location { x, y } = self;

        match facing {
            North => Location::new(*x, y - 1),
            East => Location::new(x + 1, *y),
            South => Location::new(*x, y + 1),
            West => Location::new(x - 1, *y),
            NorthWest => Location::new(x - 1, y - 1),
            NorthEast => Location::new(x + 1, y - 1),
            SouthWest => Location::new(x - 1, y + 1),
            SouthEast => Location::new(x + 1, y + 1),
        }
    }
}

impl Sub for Location {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

fn part1(input: &str) -> usize {
    let maze = Maze::parse(input);
    let main_loop = maze.find_main_loop().unwrap();
    main_loop.len() / 2
}

fn part2(input: &str) -> usize {
    let maze = Maze::parse(input);
    let main_loop = maze.find_main_loop().unwrap();
    maze.count_enclosed_tiles(&main_loop)
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    static EXAMPLE1: &str = indoc! {"
        .....
        .S-7.
        .|.|.
        .L-J.
        .....
    "};

    static EXAMPLE2: &str = indoc! {"
        -L|F7
        7S-7|
        L|7||
        -L-J|
        L|-JF
    "};

    static EXAMPLE3: &str = indoc! {"
        ..F7.
        .FJ|.
        SJ.L7
        |F--J
        LJ...
    "};

    static EXAMPLE4: &str = indoc! {"
        7-F7-
        .FJ|7
        SJLL7
        |F--J
        LJ.LJ
    "};

    static EXAMPLE5: &str = indoc! {"
        ...........
        .S-------7.
        .|F-----7|.
        .||.....||.
        .||.....||.
        .|L-7.F-J|.
        .|..|.|..|.
        .L--J.L--J.
        ...........
    "};

    static EXAMPLE6: &str = indoc! {"
        ..........
        .S------7.
        .|F----7|.
        .||....||.
        .||....||.
        .|L-7F-J|.
        .|..||..|.
        .L--JL--J.
        ..........
    "};

    static EXAMPLE7: &str = indoc! {"
        .F----7F7F7F7F-7....
        .|F--7||||||||FJ....
        .||.FJ||||||||L7....
        FJL7L7LJLJ||LJ.L-7..
        L--J.L7...LJS7F-7L7.
        ....F-J..F7FJ|L7L7L7
        ....L7.F7||L7|.L7L7|
        .....|FJLJ|FJ|F7|.LJ
        ....FJL-7.||.||||...
        ....L---J.LJ.LJLJ...
    "};

    static EXAMPLE8: &str = indoc! {"
        FF7FSF7F7F7F7F7F---7
        L|LJ||||||||||||F--J
        FL-7LJLJ||||||LJL-77
        F--JF--7||LJLJ7F7FJ-
        L---JF-JLJ.||-FJLJJ7
        |F|F-JF---7F7-L7L|7|
        |FFJF7L7F-JF7|JL---7
        7-L-JL7||F7|L7F-7F7|
        L.L7LFJ|||||FJL7||LJ
        L7JLJL-JLJLJL--JLJ.L
    "};

    #[test]
    fn test_part1_example1() {
        let result = part1(EXAMPLE1);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_part1_example2() {
        let result = part1(EXAMPLE2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_part1_example3() {
        let result = part1(EXAMPLE3);
        assert_eq!(result, 8);
    }

    #[test]
    fn test_part1_example4() {
        let result = part1(EXAMPLE4);
        assert_eq!(result, 8);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file());
        assert_eq!(result, 6942);
    }

    #[test]
    fn test_part2_example5() {
        let result = part2(EXAMPLE5);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_part2_example6() {
        let result = part2(EXAMPLE6);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_part2_example7() {
        let result = part2(EXAMPLE7);
        assert_eq!(result, 8);
    }

    #[test]
    fn test_part2_example8() {
        let result = part2(EXAMPLE8);
        assert_eq!(result, 10);
    }

    #[test]
    fn test_part2_solution() {
        let result = part2(&read_input_file());
        assert_eq!(result, 297);
    }
}
