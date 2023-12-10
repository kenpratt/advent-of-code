use std::{collections::HashMap, fs};

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    // println!("part 2 result: {:?}", part2(&read_input_file()));
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
                let pos = Location(x as isize, y as isize);

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
}

#[derive(Debug)]
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
}

#[derive(Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct Location(isize, isize);

impl Location {
    fn neighbours(&self, directions: &[Direction]) -> Vec<Self> {
        directions.iter().map(|d| self.neighbour(d)).collect()
    }

    fn neighbour(&self, direction: &Direction) -> Self {
        use Direction::*;

        let Location(x, y) = self;

        match direction {
            North => Location(*x, y - 1),
            East => Location(x + 1, *y),
            South => Location(*x, y + 1),
            West => Location(x - 1, *y),
        }
    }
}

fn part1(input: &str) -> usize {
    let maze = Maze::parse(input);
    let main_loop = maze.find_main_loop().unwrap();
    main_loop.len() / 2
}

// fn part2(input: &str) -> usize {
//     let mazes = Data::parse(input);
//     dbg!(&mazes);
//     0
// }

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
