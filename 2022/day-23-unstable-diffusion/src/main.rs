use std::fmt;
use std::fs;
use std::ops::RangeInclusive;

use cached::proc_macro::cached;
use lazy_static::lazy_static;
use rustc_hash::FxHashSet;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    println!("part 2 result: {:?}", part2(&read_input_file()));
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

#[cached]
fn neighbours(pos: Coordinate) -> [(Direction, Coordinate); 8] {
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

fn direction_set(directions: &[Direction]) -> FxHashSet<Direction> {
    let mut set = FxHashSet::default();
    for d in directions {
        set.insert(*d);
    }
    set
}

lazy_static! {
    static ref POSSIBLE_MOVES: [(Direction, FxHashSet<Direction>); 4] = [
        (
            Direction::North,
            direction_set(&[Direction::NorthWest, Direction::North, Direction::NorthEast])
        ),
        (
            Direction::South,
            direction_set(&[Direction::SouthEast, Direction::South, Direction::SouthWest])
        ),
        (
            Direction::West,
            direction_set(&[Direction::SouthWest, Direction::West, Direction::NorthWest])
        ),
        (
            Direction::East,
            direction_set(&[Direction::NorthEast, Direction::East, Direction::SouthEast])
        ),
    ];
}

const POSSIBLE_MOVE_INDICES: [[usize; 4]; 4] =
    [[0, 1, 2, 3], [1, 2, 3, 0], [2, 3, 0, 1], [3, 0, 1, 2]];

#[derive(Clone, Debug, Eq, PartialEq)]
struct ElfPositions(FxHashSet<Coordinate>);

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

    fn simulate_n_rounds(&self, rounds: usize) -> Self {
        let mut curr = self.clone();
        for round in 0..rounds {
            curr = curr.tick(round);
        }
        curr
    }

    fn simulate_until_done(&self) -> usize {
        let mut curr = self.clone();
        let mut round = 0;
        loop {
            let next = curr.tick(round);
            if curr == next {
                break;
            } else {
                curr = next;
                round += 1;
            }
        }
        round + 1 // return number of rounds elapsed
    }

    fn tick(&self, round: usize) -> Self {
        let possible_move_indices = &POSSIBLE_MOVE_INDICES[round % 4];

        let proposed_moves: Vec<(Coordinate, Option<Coordinate>)> = self
            .0
            .iter()
            .map(|p| (*p, self.propose_move(p, possible_move_indices)))
            .collect();

        // detect conflicting moves
        let mut seen: FxHashSet<Option<Coordinate>> = FxHashSet::default();
        let mut duplicates: FxHashSet<Option<Coordinate>> = FxHashSet::default();
        for (_curr_pos, proposed) in proposed_moves.iter() {
            if seen.contains(proposed) {
                duplicates.insert(*proposed);
            } else {
                seen.insert(*proposed);
            }
        }

        let new_positions: FxHashSet<Coordinate> = proposed_moves
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
        let neighbours = neighbours(*curr_pos);
        let occupied = neighbours
            .iter()
            .filter(|(_d, c)| self.0.contains(c))
            .map(|(d, _c)| *d)
            .collect::<FxHashSet<Direction>>();

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
    let result = positions.simulate_n_rounds(10);
    result.num_empty()
}

fn part2(input: &str) -> usize {
    let positions = ElfPositions::parse(input);
    positions.simulate_until_done()
}

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

    #[test]
    fn test_part2_example() {
        let result = part2(EXAMPLE);
        assert_eq!(result, 20);
    }

    #[test]
    fn test_part2_solution() {
        let result = part2(&read_input_file());
        assert_eq!(result, 893);
    }
}
