use std::fs;

use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    // println!("part 2 result: {:?}", part2(&read_input_file()));
}

fn read_file(filename: &str) -> String {
    fs::read_to_string(filename).expect("Something went wrong reading the file")
}

fn read_input_file() -> String {
    read_file("input.txt")
}

fn parse(input: &str) -> (Map, Vec<Instruction>) {
    let mut iter = input.split("\n\n");
    let map = Map::parse(iter.next().unwrap());
    let instructions = Instruction::parse_list(iter.next().unwrap());
    assert_eq!(None, iter.next());
    (map, instructions)
}

#[derive(Debug)]
enum Instruction {
    Advance(u8),
    Turn(Rotation),
}

impl Instruction {
    fn parse_list(input: &str) -> Vec<Self> {
        use Instruction::*;

        lazy_static! {
            static ref INSTRUCTION_RE: Regex = Regex::new(r"((\d+)|([A-Z]))").unwrap();
        }

        INSTRUCTION_RE
            .captures_iter(input)
            .map(|caps| match (caps.get(2), caps.get(3)) {
                (Some(s), None) => Advance(s.as_str().parse::<u8>().unwrap()),
                (None, Some(s)) => Turn(Rotation::parse(s.as_str())),
                _ => panic!("Unreachable"),
            })
            .collect()
    }
}

#[derive(Clone, Copy, Debug)]
enum Rotation {
    Clockwise,
    Counterclockwise,
}

impl Rotation {
    fn parse(input: &str) -> Self {
        use Rotation::*;
        match input {
            "L" => Counterclockwise,
            "R" => Clockwise,
            _ => panic!("Bad direction: {}", input),
        }
    }

    fn apply(&self, facing: &Facing) -> Facing {
        use Facing::*;
        use Rotation::*;
        match self {
            Clockwise => match facing {
                Up => Right,
                Left => Up,
                Right => Down,
                Down => Left,
            },
            Counterclockwise => match facing {
                Up => Left,
                Left => Down,
                Right => Up,
                Down => Right,
            },
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Coordinate {
    x: usize,
    y: usize,
}

#[derive(Debug)]
enum Facing {
    Right,
    Left,
    Down,
    Up,
}

impl Facing {
    fn value(&self) -> usize {
        use Facing::*;
        match self {
            Right => 0,
            Left => 2,
            Down => 1,
            Up => 3,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Tile {
    Open,
    Wall,
}

impl Tile {
    fn parse(input: &char) -> Option<Self> {
        use Tile::*;
        match input {
            ' ' => None,
            '.' => Some(Open),
            '#' => Some(Wall),
            _ => panic!("Bad tile: {}", input),
        }
    }
}

#[derive(Debug)]
struct Map {
    rows: Vec<Line>,
    cols: Vec<Line>,
}

impl Map {
    fn parse(input: &str) -> Self {
        let mut rows_raw: Vec<Vec<Option<Tile>>> = input
            .lines()
            .map(|line| line.chars().map(|c| Tile::parse(&c)).collect())
            .collect();

        // make the grid rectangular, filling with more empty space on the
        // right side, or else transpose will break
        let width = rows_raw.iter().map(|r| r.len()).max().unwrap();
        for row in &mut rows_raw {
            row.resize(width, None);
        }

        let rows = rows_raw.iter().map(|row| Line::new(row)).collect();

        let cols_raw = transpose(rows_raw);
        let cols = cols_raw.iter().map(|col| Line::new(col)).collect();

        Self { rows, cols }
    }

    fn first_tile_in_row(&self, y: usize) -> (Coordinate, Tile) {
        let row = &self.rows[y];
        let x = row.from;
        let tile = row.tiles[0];
        (Coordinate { x, y }, tile)
    }

    fn advance(&self, position: &Coordinate, facing: &Facing, distance: u8) -> Coordinate {
        use Facing::*;
        match facing {
            Right => {
                let x = self.rows[position.y].advance(position.x, distance, false);
                Coordinate {
                    x: x,
                    y: position.y,
                }
            }
            Left => {
                let x = self.rows[position.y].advance(position.x, distance, true);
                Coordinate {
                    x: x,
                    y: position.y,
                }
            }
            Down => {
                let y = self.cols[position.x].advance(position.y, distance, false);
                Coordinate {
                    x: position.x,
                    y: y,
                }
            }
            Up => {
                let y = self.cols[position.x].advance(position.y, distance, true);
                Coordinate {
                    x: position.x,
                    y: y,
                }
            }
        }
    }
}

#[derive(Debug)]
struct Line {
    from: usize,
    to: usize,
    tiles: Vec<Tile>,
}

impl Line {
    fn new(input: &[Option<Tile>]) -> Self {
        let non_empty: Vec<(usize, Tile)> = input
            .iter()
            .enumerate()
            .filter(|(_i, t)| t.is_some())
            .map(|(i, t)| (i, t.unwrap()))
            .collect();
        let (from, _) = *non_empty.first().unwrap();
        let (to, _) = *non_empty.last().unwrap();
        assert_eq!(to + 1 - from, non_empty.len());
        let tiles = non_empty.into_iter().map(|(_i, t)| t).collect();
        Self { from, to, tiles }
    }

    fn len(&self) -> usize {
        self.to + 1 - self.from
    }

    fn advance(&self, pos: usize, distance: u8, reverse: bool) -> usize {
        let num_to_skip = if reverse {
            self.to + 1 - pos
        } else {
            pos + 1 - self.from
        };

        let mut iter = self.indices(reverse, num_to_skip);

        let mut last_index = pos - self.from;
        for _ in 0..distance {
            let index = iter.next().unwrap();
            let tile = self.tiles[index];
            match tile {
                Tile::Open => last_index = index,
                Tile::Wall => break,
            }
        }
        last_index + self.from
    }

    fn indices(&self, reverse: bool, num_to_skip: usize) -> Box<dyn Iterator<Item = usize>> {
        let indices = 0..self.len();
        if reverse {
            Box::new(indices.rev().cycle().skip(num_to_skip))
        } else {
            Box::new(indices.cycle().skip(num_to_skip))
        }
    }
}

#[derive(Debug)]
struct Player {
    position: Coordinate,
    facing: Facing,
}

impl Player {
    fn initial(map: &Map) -> Self {
        // start in top left, facing right
        let (position, tile) = map.first_tile_in_row(0);
        assert_eq!(Tile::Open, tile);
        Self {
            position: position,
            facing: Facing::Right,
        }
    }

    fn navigate(map: &Map, instructions: &[Instruction]) -> Self {
        let mut player = Self::initial(map);
        for instruction in instructions {
            match instruction {
                Instruction::Advance(n) => player.advance(*n, map),
                Instruction::Turn(r) => player.turn(*r),
            };
        }
        player
    }

    fn advance(&mut self, distance: u8, map: &Map) {
        self.position = map.advance(&self.position, &self.facing, distance);
    }

    fn turn(&mut self, rotation: Rotation) {
        self.facing = rotation.apply(&self.facing);
    }
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn part1(input: &str) -> usize {
    let (map, instructions) = parse(input);
    let player = Player::navigate(&map, &instructions);
    1000 * (player.position.y + 1) + 4 * (player.position.x + 1) + player.facing.value()
}

// fn part2(input: &str) -> usize {
//     let items = Data::parse(input);
//     dbg!(&items);
//     0
// }

#[cfg(test)]
mod tests {
    use super::*;

    fn read_example_file() -> String {
        read_file("example.txt")
    }

    #[test]
    fn test_part1_example() {
        let result = part1(&read_example_file());
        assert_eq!(result, 6032);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file());
        assert_eq!(result, 56372);
    }

    // #[test]
    // fn test_part2_example() {
    //     let result = part2(&read_example_file());
    //     assert_eq!(result, 0);
    // }

    // #[test]
    // fn test_part2_solution() {
    //     let result = part2(&read_input_file());
    //     assert_eq!(result, 0);
    // }
}
