use std::{
    collections::{BTreeSet, HashMap},
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
struct Platform {
    height: usize,
    fixed: BTreeSet<Coord>,
    marbles: BTreeSet<Coord>,
}

impl Platform {
    fn parse(input: &str) -> Self {
        let width = input.lines().next().unwrap().len();
        let height = input.lines().count();

        let mut fixed = BTreeSet::new();
        let mut marbles = BTreeSet::new();

        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let pos = Coord::new(x + 1, y + 1); // index rocks at 1,1 to leave room for edge
                match c {
                    'O' => marbles.insert(pos),
                    '#' => fixed.insert(pos),
                    '.' => false,
                    _ => panic!("Unexpected char: {:?}", c),
                };
            }
        }

        // add edges
        for x in 1..=width {
            fixed.insert(Coord::new(x, 0));
            fixed.insert(Coord::new(x, height + 1));
        }
        for y in 1..=height {
            fixed.insert(Coord::new(0, y));
            fixed.insert(Coord::new(width + 1, y));
        }

        Self {
            height,
            fixed,
            marbles,
        }
    }

    fn north_load_score_after_single_tilt_north(&self) -> usize {
        let tilted = self.tilt_in_direction(&self.marbles, &Direction::North);
        self.north_load_score(&tilted)
    }

    fn north_load_score_after_cycles(&self, cycles: usize) -> usize {
        let mut seen: HashMap<BTreeSet<Coord>, usize> = HashMap::new();
        seen.insert(self.marbles.clone(), 0);

        let mut last = self.marbles.clone();

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

    fn run_cycle(&self, marbles: &BTreeSet<Coord>) -> BTreeSet<Coord> {
        ALL_DIRECTIONS.iter().fold(marbles.clone(), |last, dir| {
            self.tilt_in_direction(&last, dir)
        })
    }

    fn tilt_in_direction(
        &self,
        marbles: &BTreeSet<Coord>,
        direction: &Direction,
    ) -> BTreeSet<Coord> {
        let mut new_positions = BTreeSet::new();

        if direction.is_ascending() {
            for pos in marbles.iter().rev() {
                new_positions.insert(self.find_new_marble_position(pos, &new_positions, direction));
            }
        } else {
            for pos in marbles.iter() {
                new_positions.insert(self.find_new_marble_position(pos, &new_positions, direction));
            }
        }

        new_positions
    }

    fn find_new_marble_position(
        &self,
        initial: &Coord,
        other_marbles: &BTreeSet<Coord>,
        direction: &Direction,
    ) -> Coord {
        let mut new_pos = *initial;
        loop {
            let try_pos = new_pos.shift(direction);
            if self.fixed.contains(&try_pos) || other_marbles.contains(&try_pos) {
                // we are blocked by a wall/cube rock or another marble
                break;
            } else {
                new_pos = try_pos;
            }
        }
        new_pos
    }

    fn north_load_score(&self, marbles: &BTreeSet<Coord>) -> usize {
        marbles.iter().map(|pos| self.height - (pos.y - 1)).sum()
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

const ALL_DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::West,
    Direction::South,
    Direction::East,
];

impl Direction {
    fn is_ascending(&self) -> bool {
        *self == Direction::South || *self == Direction::East
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

        let after1 = platform.run_cycle(&platform.marbles.clone());
        assert_eq!(after1, expect_after1.marbles);

        let after2 = platform.run_cycle(&after1);
        assert_eq!(after2, expect_after2.marbles);

        let after3 = platform.run_cycle(&after2);
        assert_eq!(after3, expect_after3.marbles);
    }

    #[test]
    fn test_part2_solution() {
        let result = part2(&read_input_file());
        assert_eq!(result, 98029);
    }
}
