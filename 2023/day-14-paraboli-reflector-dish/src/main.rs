use std::{
    collections::{BTreeSet, HashMap},
    fs,
};

use itertools::Itertools;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    println!("part 2 result: {:?}", part2(&read_input_file()));
}

fn read_input_file() -> String {
    fs::read_to_string("input.txt").expect("Something went wrong reading the file")
}

#[derive(Debug)]
struct Platform {
    width: usize,
    height: usize,
    rounded_rocks: BTreeSet<Coord>,
    stops: HashMap<Direction, Vec<Vec<usize>>>,
}

impl Platform {
    fn parse(input: &str) -> Self {
        let width = input.lines().next().unwrap().len();
        let height = input.lines().count();

        let mut rounded_rocks = BTreeSet::new();
        let mut cube_rocks = BTreeSet::new();

        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let pos = Coord::new(x, y);
                match c {
                    'O' => rounded_rocks.insert(pos),
                    '#' => cube_rocks.insert(pos),
                    '.' => false,
                    _ => panic!("Unexpected char: {:?}", c),
                };
            }
        }

        let stops = ALL_DIRECTIONS
            .iter()
            .map(|d| (*d, Self::calculate_stops(&width, &height, &cube_rocks, d)))
            .collect();

        Self {
            width,
            height,
            rounded_rocks,
            stops,
        }
    }

    fn calculate_stops(
        width: &usize,
        height: &usize,
        cube_rocks: &BTreeSet<Coord>,
        direction: &Direction,
    ) -> Vec<Vec<usize>> {
        let is_vertical = direction.is_vertical();
        let is_ascending = direction.is_ascending();

        // edge
        let edge: usize = match direction {
            Direction::North | Direction::West => 0,
            Direction::South => height - 1,
            Direction::East => width - 1,
        };
        let far_edge: usize = match direction {
            Direction::North => height - 1,
            Direction::West => width - 1,
            Direction::South | Direction::East => 0,
        };

        // for each column/row
        let range = if is_vertical { 0..*width } else { 0..*height };
        range
            .map(|p| {
                // find stops for cube rocks
                let mut blockages: Vec<usize> = cube_rocks
                    .iter()
                    .filter(|pos| {
                        if is_vertical {
                            pos.x == p && pos.y != far_edge
                        } else {
                            pos.y == p && pos.x != far_edge
                        }
                    })
                    .map(|pos| if is_vertical { pos.y } else { pos.x })
                    .map(|p| if is_ascending { p - 1 } else { p + 1 })
                    .collect();

                // get all the stops, and reverse to largest first if ascending direction
                let mut stops: Vec<usize> = vec![edge];
                stops.append(&mut blockages);
                stops.sort();
                if !is_ascending {
                    stops.reverse();
                }
                stops
            })
            .collect()
    }

    fn count_blocked_row_or_column(
        &self,
        rounded_rocks: &BTreeSet<Coord>,
        direction: &Direction,
        row_col: &usize,
    ) -> HashMap<&usize, usize> {
        let is_vertical = direction.is_vertical();
        let is_ascending = direction.is_ascending();

        // find blockages
        let stops_for_direction = self.stops.get(direction).unwrap();
        let stops = &stops_for_direction[*row_col];

        // get rock counts at each stop
        rounded_rocks
            .iter()
            .map(|rock| {
                if is_vertical {
                    (rock.x, rock.y)
                } else {
                    (rock.y, rock.x)
                }
            })
            .filter(|(r1, _r2)| r1 == row_col)
            .counts_by(|(_r1, r2)| {
                stops
                    .iter()
                    .find(|p| if is_ascending { **p >= r2 } else { **p <= r2 })
                    .unwrap()
            })
    }

    fn count_blocked(
        &self,
        rounded_rocks: &BTreeSet<Coord>,
        direction: &Direction,
    ) -> Vec<HashMap<&usize, usize>> {
        let range = if direction.is_vertical() {
            0..self.width
        } else {
            0..self.height
        };
        range
            .map(|p| self.count_blocked_row_or_column(rounded_rocks, direction, &p))
            .collect()
    }

    fn north_load_score_after_single_tilt_north(&self) -> usize {
        let tilted = self.tilt_rocks_in_direction(&self.rounded_rocks, &Direction::North);
        self.north_load_score(&tilted)
    }

    fn north_load_score_after_cycles(&self, cycles: usize) -> usize {
        let mut seen: HashMap<BTreeSet<Coord>, usize> = HashMap::new();
        seen.insert(self.rounded_rocks.clone(), 0);

        let mut last = self.rounded_rocks.clone();

        for i in 1..=cycles {
            let result = self.run_cycle(&last);
            if seen.contains_key(&result) {
                let repeated_index = seen.get(&result).unwrap();

                // calculate how far we can skip
                let repeat_width = i - repeated_index;
                let remaining = cycles - i;
                let remaining_skipping_repeated_sections = remaining % repeat_width;

                // we've already seen the winning set, just rewind a bit
                let result_index = repeated_index + remaining_skipping_repeated_sections;
                let winner = seen.iter().find(|(_k, v)| **v == result_index).unwrap().0;
                return self.north_load_score(winner);
            } else {
                seen.insert(result.clone(), i);
                last = result;
            }
        }

        panic!("No repeat found");
    }

    fn run_cycle(&self, rounded_rocks: &BTreeSet<Coord>) -> BTreeSet<Coord> {
        ALL_DIRECTIONS
            .iter()
            .fold(rounded_rocks.clone(), |last, dir| {
                let foo = self.tilt_rocks_in_direction(&last, dir);
                assert_eq!(last.len(), foo.len());
                foo
            })
    }

    fn tilt_rocks_in_direction(
        &self,
        rounded_rocks: &BTreeSet<Coord>,
        direction: &Direction,
    ) -> BTreeSet<Coord> {
        let counts = self.count_blocked(rounded_rocks, direction);
        let is_vertical = direction.is_vertical();
        let is_ascending = direction.is_ascending();

        counts
            .into_iter()
            .enumerate()
            .flat_map(|(r1, h)| {
                h.into_iter().flat_map(move |(p, num)| {
                    (0..num)
                        .map(|offset| {
                            let r2 = if is_ascending { p - offset } else { p + offset };
                            if is_vertical {
                                Coord::new(r1, r2)
                            } else {
                                Coord::new(r2, r1)
                            }
                        })
                        .collect::<BTreeSet<Coord>>()
                })
            })
            .collect()
    }

    fn north_load_score(&self, rounded_rocks: &BTreeSet<Coord>) -> usize {
        rounded_rocks.iter().map(|pos| self.height - pos.y).sum()
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
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Direction {
    North,
    West,
    South,
    East,
}

const ALL_DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::West,
    Direction::South,
    Direction::East,
];

impl Direction {
    fn is_vertical(&self) -> bool {
        *self == Direction::North || *self == Direction::South
    }

    fn is_ascending(&self) -> bool {
        *self == Direction::East || *self == Direction::South
    }
}

fn part1(input: &str) -> usize {
    let platform = Platform::parse(input);
    platform.north_load_score_after_single_tilt_north()
}

fn part2(input: &str) -> usize {
    let platform = Platform::parse(input);
    platform.north_load_score_after_cycles(1000000000)
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    static EXAMPLE: &str = indoc! {"
        O....#....
        O.OO#....#
        .....##...
        OO.#O....O
        .O.....O#.
        O.#..O.#.#
        ..O..#O..O
        .......O..
        #....###..
        #OO..#....
    "};

    static PART2_EXAMPLE_AFTER_1_CYCLE: &str = indoc! {"
        .....#....
        ....#...O#
        ...OO##...
        .OO#......
        .....OOO#.
        .O#...O#.#
        ....O#....
        ......OOOO
        #...O###..
        #..OO#....
    "};

    static PART2_EXAMPLE_AFTER_2_CYCLES: &str = indoc! {"
        .....#....
        ....#...O#
        .....##...
        ..O#......
        .....OOO#.
        .O#...O#.#
        ....O#...O
        .......OOO
        #..OO###..
        #.OOO#...O
    "};

    static PART2_EXAMPLE_AFTER_3_CYCLES: &str = indoc! {"
        .....#....
        ....#...O#
        .....##...
        ..O#......
        .....OOO#.
        .O#...O#.#
        ....O#...O
        .......OOO
        #...O###.O
        #.OOO#...O
    "};

    #[test]
    fn test_part1_example() {
        let result = part1(EXAMPLE);
        assert_eq!(result, 136);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file());
        assert_eq!(result, 105623);
    }

    #[test]
    fn test_part2_example() {
        let result = part2(EXAMPLE);
        assert_eq!(result, 64);
    }

    #[test]
    fn test_part2_example_after_limited_cycles() {
        let platform = Platform::parse(EXAMPLE);
        let expect_after1 = Platform::parse(PART2_EXAMPLE_AFTER_1_CYCLE);
        let expect_after2 = Platform::parse(PART2_EXAMPLE_AFTER_2_CYCLES);
        let expect_after3 = Platform::parse(PART2_EXAMPLE_AFTER_3_CYCLES);

        let after1 = platform.run_cycle(&platform.rounded_rocks.clone());
        assert_eq!(after1, expect_after1.rounded_rocks);

        let after2 = platform.run_cycle(&after1);
        assert_eq!(after2, expect_after2.rounded_rocks);

        let after3 = platform.run_cycle(&after2);
        assert_eq!(after3, expect_after3.rounded_rocks);
    }

    #[test]
    fn test_part2_solution() {
        let result = part2(&read_input_file());
        assert_eq!(result, 98029);
    }
}
