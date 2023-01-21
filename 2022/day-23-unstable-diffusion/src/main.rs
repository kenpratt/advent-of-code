use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::fs;
use std::ops::RangeInclusive;

use lazy_static::lazy_static;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    // println!("part 2 result: {:?}", part2(&read_input_file()));
}

fn read_input_file() -> String {
    fs::read_to_string("input.txt").expect("Something went wrong reading the file")
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Coordinate {
    x: isize,
    y: isize,
}

impl Coordinate {
    fn neighbour(&self, direction: &Direction) -> Self {
        use Direction::*;
        match direction {
            North => Self {
                x: self.x,
                y: self.y - 1,
            },
            NorthEast => Coordinate {
                x: self.x + 1,
                y: self.y - 1,
            },
            East => Coordinate {
                x: self.x + 1,
                y: self.y,
            },
            SouthEast => Coordinate {
                x: self.x + 1,
                y: self.y + 1,
            },
            South => Coordinate {
                x: self.x,
                y: self.y + 1,
            },
            SouthWest => Coordinate {
                x: self.x - 1,
                y: self.y + 1,
            },
            West => Coordinate {
                x: self.x - 1,
                y: self.y,
            },
            NorthWest => Coordinate {
                x: self.x - 1,
                y: self.y - 1,
            },
        }
    }
}

fn neighbours(pos: &Coordinate) -> [(Direction, Coordinate); 8] {
    ALL_DIRECTIONS
        .iter()
        .map(|d| (*d, pos.neighbour(d)))
        .collect::<Vec<(Direction, Coordinate)>>()
        .try_into()
        .unwrap()
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

const ALL_DIRECTIONS: [Direction; 8] = [
    Direction::North,
    Direction::NorthEast,
    Direction::East,
    Direction::SouthEast,
    Direction::South,
    Direction::SouthWest,
    Direction::West,
    Direction::NorthWest,
];

lazy_static! {
    static ref POSSIBLE_MOVES: [(Direction, HashSet<Direction>); 4] = [
        (
            Direction::North,
            HashSet::from([Direction::NorthWest, Direction::North, Direction::NorthEast])
        ),
        (
            Direction::South,
            HashSet::from([Direction::SouthEast, Direction::South, Direction::SouthWest])
        ),
        (
            Direction::West,
            HashSet::from([Direction::SouthWest, Direction::West, Direction::NorthWest])
        ),
        (
            Direction::East,
            HashSet::from([Direction::NorthEast, Direction::East, Direction::SouthEast])
        ),
    ];
}

const POSSIBLE_MOVE_INDICES: [[usize; 4]; 4] =
    [[0, 1, 2, 3], [1, 2, 3, 0], [2, 3, 0, 1], [3, 0, 1, 2]];

#[derive(Clone, Debug)]
struct ElfPositions(HashSet<Coordinate>);

impl ElfPositions {
    fn parse(input: &str) -> Self {
        let positions = input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter(|&(_x, c)| c == '#')
                    .map(|(x, _)| Coordinate {
                        x: x.try_into().unwrap(),
                        y: y.try_into().unwrap(),
                    })
                    .collect::<Vec<Coordinate>>()
            })
            .collect();
        Self(positions)
    }

    fn simulate(&self, rounds: usize) -> Self {
        let mut curr = self.clone();
        for round in 0..rounds {
            curr = curr.tick(round);
            // println!("after round {}:\n{}", round, curr);
        }
        curr
    }

    fn tick(&self, round: usize) -> Self {
        let possible_move_indices = &POSSIBLE_MOVE_INDICES[round % 4];

        let proposed_moves: HashMap<Coordinate, Option<Coordinate>> = self
            .0
            .iter()
            .map(|p| (*p, self.propose_move(p, possible_move_indices)))
            .collect();

        // detect conflicting moves
        let mut seen: HashSet<Option<Coordinate>> = HashSet::new();
        let mut duplicates: HashSet<Option<Coordinate>> = HashSet::new();
        for proposed in proposed_moves.values() {
            if seen.contains(proposed) {
                duplicates.insert(*proposed);
            } else {
                seen.insert(*proposed);
            }
        }

        let new_positions: HashSet<Coordinate> = proposed_moves
            .into_iter()
            .map(|(curr_pos, proposed)| {
                if proposed.is_none() || duplicates.contains(&proposed) {
                    curr_pos
                } else {
                    proposed.unwrap()
                }
            })
            .collect();
        Self(new_positions)
    }

    fn propose_move(
        &self,
        curr_pos: &Coordinate,
        possible_move_indices: &[usize],
    ) -> Option<Coordinate> {
        let neighbours = neighbours(curr_pos);
        let occupied = neighbours
            .iter()
            .filter(|(_d, c)| self.0.contains(c))
            .map(|(d, _c)| *d)
            .collect::<HashSet<Direction>>();

        // if there are no occupied neighbours, do nothing
        if occupied.is_empty() {
            return None;
        }

        for possible_move_index in possible_move_indices {
            let (move_direction, neighbour_directions) = &POSSIBLE_MOVES[*possible_move_index];
            if occupied.is_disjoint(neighbour_directions) {
                // there are no elves in the 3 relevant positions, propose this move
                return Some(curr_pos.neighbour(move_direction));
            }
        }

        // no valid moves
        None
    }

    fn bounds(&self) -> (RangeInclusive<isize>, RangeInclusive<isize>) {
        let min_x = self.0.iter().map(|c| c.x).min().unwrap();
        let max_x = self.0.iter().map(|c| c.x).max().unwrap();
        let min_y = self.0.iter().map(|c| c.y).min().unwrap();
        let max_y = self.0.iter().map(|c| c.y).max().unwrap();
        ((min_x..=max_x), (min_y..=max_y))
    }

    fn num_empty(&self) -> usize {
        let (x_range, y_range) = self.bounds();
        let width = x_range.end() + 1 - x_range.start();
        let height = y_range.end() + 1 - y_range.start();
        let size = width * height;
        size as usize - self.0.len()
    }
}

impl fmt::Display for ElfPositions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (x_range, y_range) = self.bounds();
        for y in y_range {
            for x in x_range.clone() {
                write!(
                    f,
                    "{}",
                    if self.0.contains(&Coordinate { x, y }) {
                        '#'
                    } else {
                        '.'
                    }
                )?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn part1(input: &str) -> usize {
    let positions = ElfPositions::parse(input);
    let result = positions.simulate(10);
    result.num_empty()
}

// fn part2(input: &str) -> usize {
//     let items = Data::parse(input);
//     dbg!(&items);
//     0
// }

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    static EXAMPLE: &str = indoc! {"
        ....#..
        ..###.#
        #...#.#
        .#...##
        #.###..
        ##.#.##
        .#..#..
    "};

    #[test]
    fn test_part1_example() {
        let result = part1(EXAMPLE);
        assert_eq!(result, 110);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file());
        assert_eq!(result, 3815);
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
