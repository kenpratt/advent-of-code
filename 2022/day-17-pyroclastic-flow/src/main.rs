use std::fs;
use std::iter::Cycle;
use std::ops::Range;

use lazy_static::lazy_static;

const CHAMBER_WIDTH: usize = 7;
const SHAPE_HEIGHT: usize = 4;

const SHAPE_DEFINITIONS: [&str; 5] = [
    "####",
    ".#.\n###\n.#.",
    "..#\n..#\n###",
    "#\n#\n#\n#",
    "##\n##",
];

lazy_static! {
    static ref SHAPES: [Shape; 5] = SHAPE_DEFINITIONS
        .iter()
        .map(|s| Shape::parse(s))
        .collect::<Vec<Shape>>()
        .try_into()
        .unwrap();
    static ref SHAPE_FOR_X_OFFSET: [[Option<Shape>; CHAMBER_WIDTH]; 5] = SHAPES
        .iter()
        .map(|shape| shape.calculate_offset_variations())
        .collect::<Vec<[Option<Shape>; CHAMBER_WIDTH]>>()
        .try_into()
        .unwrap();
}

fn main() {
    println!(
        "part 1 result: {:?}",
        part1(&read_input_file(), 2022).height
    );
    println!(
        "part 2 result: {:?}",
        part2(&read_input_file(), 1000000000000)
    );
}

fn read_input_file() -> String {
    fs::read_to_string("input.txt").expect("Something went wrong reading the file")
}

// flip order, as input has index 0 at top, but in struct indexes 0 is bottom
fn parse_lines(input: &str) -> Vec<u8> {
    input.lines().map(|l| parse_line(l)).rev().collect()
}

// convert to 7 bits (in a u8), left-most/highest bit is at x=0
fn parse_line(line: &str) -> u8 {
    let mut chars = line.chars();
    let mut val: u8 = 0;
    for _ in 0..CHAMBER_WIDTH {
        val <<= 1;
        let c = chars.next();
        match c {
            Some('#') => val |= 1,
            Some('.') | None => {} // skip if None or '.'
            _ => panic!("Unexpected input: {:?}", c),
        };
    }
    val
}

#[derive(Clone, Debug)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    fn parse_list(input: &str) -> Vec<Self> {
        input.chars().map(|c| Self::parse(&c)).collect()
    }

    fn parse(c: &char) -> Self {
        use Direction::*;
        match c {
            '<' => Left,
            '>' => Right,
            _ => panic!("Unexpected direction: {}", c),
        }
    }
}

#[derive(Clone, Debug)]
struct Shape {
    // index 0 is the bottom
    lines: [u8; SHAPE_HEIGHT],
    offset: usize,
    width: usize,
    height: usize,
}

impl Shape {
    fn parse(input: &str) -> Self {
        let offset = 0;
        let width = input.lines().next().unwrap().len();

        // flip order, as input has index 0 at top, but struct index 0 is bottom
        let mut lines_vec = parse_lines(input);
        let height = lines_vec.len();

        // fill remainder with zeros so we can use constant size arrays
        lines_vec.resize(SHAPE_HEIGHT, 0);

        let lines = lines_vec.try_into().unwrap();

        Self {
            lines,
            offset,
            width,
            height,
        }
    }

    fn calculate_offset_variations(&self) -> [Option<Shape>; CHAMBER_WIDTH] {
        (0..CHAMBER_WIDTH)
            .map(|offset| self.offset_by(offset))
            .collect::<Vec<Option<Shape>>>()
            .try_into()
            .unwrap()
    }

    fn offset_by(&self, offset: usize) -> Option<Shape> {
        if (offset + self.width) > CHAMBER_WIDTH {
            return None;
        }

        let mut new_shape = self.clone();
        new_shape.offset = offset;
        for line in &mut new_shape.lines {
            *line >>= offset;
        }
        Some(new_shape)
    }
}

#[derive(Debug)]
struct FallingRock {
    shape_id: usize,
    width: usize,
    x: usize,
    y: usize,
}

impl FallingRock {
    fn new() -> Self {
        // temp values, will be overwritten on first run
        Self {
            shape_id: 0,
            width: 0,
            x: 0,
            y: 0,
        }
    }

    fn reset(&mut self, shape_id: usize, x: usize, y: usize) {
        self.shape_id = shape_id;
        self.width = SHAPES[shape_id].width;
        self.x = x;
        self.y = y;
    }
}

#[derive(Debug)]
struct Floor {
    layers: Vec<u8>,
    height: usize,
}

impl Floor {
    fn new() -> Self {
        let layers = vec![];
        let height = 0;
        Self { layers, height }
    }

    fn check_collision(&self, shape_id: usize, check_x: usize, check_y: usize) -> bool {
        if check_y < self.height {
            let offsets = &SHAPE_FOR_X_OFFSET[shape_id];
            let shape = offsets[check_x].as_ref().unwrap();

            for i in 0..shape.height {
                let y = check_y + i;
                if y < self.height {
                    let shape_row = &shape.lines[i];
                    let floor_row = &self.layers[y];
                    let res = shape_row & floor_row;
                    if res > 0 {
                        // any position collides
                        return true;
                    }
                }
            }

            // no collision, no lines below floor collided
            false
        } else {
            // can't be a collision, it's above the floor
            false
        }
    }

    fn meld(&mut self, rock: &FallingRock) {
        let offsets = &SHAPE_FOR_X_OFFSET[rock.shape_id];
        let shape = offsets[rock.x].as_ref().unwrap();

        let rock_height = rock.y + shape.height;
        if rock_height > self.height {
            self.layers.resize(rock_height, 0);
            self.height = self.layers.len();
        }

        for i in 0..shape.height {
            let y = rock.y + i;
            let shape_row = &shape.lines[i];
            self.layers[y] |= shape_row;
        }
    }
}

#[derive(Debug)]
struct Simulation {
    jet_iter: Cycle<std::vec::IntoIter<Direction>>,
    shape_iter: Cycle<Range<usize>>,
    falling_rock: FallingRock,
    floor: Floor,
}

impl Simulation {
    fn new(jets: Vec<Direction>) -> Self {
        let jet_iter = jets.into_iter().cycle();
        let shape_iter = (0..SHAPES.len()).cycle();
        let falling_rock = FallingRock::new();
        let floor = Floor::new();
        Self {
            jet_iter,
            shape_iter,
            falling_rock,
            floor,
        }
    }

    fn run(&mut self, num_rocks: usize) {
        for _ in 1..=num_rocks {
            self.drop_rock();
        }
    }

    fn drop_rock(&mut self) {
        // initialize the falling rock
        let shape_index = self.shape_iter.next().unwrap();
        let x = 2;
        let y = self.floor.height + 3;
        self.falling_rock.reset(shape_index, x, y);

        // fall & push until stopped during a fall
        loop {
            self.push();

            if self.fall() {
                break; // true = rock stopped
            }
        }
    }

    fn fall(&mut self) -> bool {
        if self.falling_rock.y == 0 {
            // special case, reaches floor with no collision
            self.floor.meld(&self.falling_rock);
            return true;
        }

        // check if there is a collision at y - 1.
        // if so, meld into floor at current y position and return true.
        // if not, increment y and return false.
        let y = self.falling_rock.y - 1;
        let collision =
            self.floor
                .check_collision(self.falling_rock.shape_id, self.falling_rock.x, y);
        if collision {
            self.floor.meld(&self.falling_rock);
        } else {
            self.falling_rock.y = y;
        }
        collision
    }

    fn push(&mut self) {
        use Direction::*;

        let direction = self.jet_iter.next().unwrap();
        let maybe_x = match direction {
            Left => {
                if self.falling_rock.x > 0 {
                    Some(self.falling_rock.x - 1)
                } else {
                    None
                }
            }
            Right => {
                let x = self.falling_rock.x + 1;
                if (x + self.falling_rock.width) <= CHAMBER_WIDTH {
                    Some(x)
                } else {
                    None
                }
            }
        };

        // check if this push cause a collision on the sides with other rocks
        match maybe_x {
            Some(x) => {
                let collision =
                    self.floor
                        .check_collision(self.falling_rock.shape_id, x, self.falling_rock.y);
                if !collision {
                    self.falling_rock.x = x;
                }
            }
            None => {}
        };
    }
}

fn part1(input: &str, num_rocks: usize) -> Floor {
    let jets = Direction::parse_list(input);

    let mut simulation = Simulation::new(jets);
    simulation.run(num_rocks);
    simulation.floor
}

fn part2(input: &str, total_rocks: usize) -> usize {
    let jets = Direction::parse_list(input);
    let jet_pattern_len = jets.len();
    let num_shapes = SHAPES.len();

    let mut simulation = Simulation::new(jets);

    // should be enough warmup to find a pattern
    let warmup_rocks = num_shapes * jet_pattern_len;
    simulation.run(warmup_rocks);
    let warmup_height = simulation.floor.height;

    // now start looking for a pattern
    let mut pattern_rocks = 0;
    let pattern_height = loop {
        pattern_rocks += num_shapes;
        simulation.run(num_shapes);

        // check for pattern
        let pattern_height = simulation.floor.height - warmup_height;
        let curr = &simulation.floor.layers[warmup_height..(warmup_height + pattern_height)];
        let last = &simulation.floor.layers[(warmup_height - pattern_height)..warmup_height];
        if curr == last {
            break pattern_height;
        }
    };

    // calculate how much we can skip due to repeating patterns
    let remaining_rocks = total_rocks - warmup_rocks - pattern_rocks;
    let skip_patterns = remaining_rocks / pattern_rocks;
    let skip_rocks = skip_patterns * pattern_rocks;
    let skip_height = skip_patterns * pattern_height;

    // continue dropping the leftover number of rocks
    let leftover_rocks = remaining_rocks - skip_rocks;
    simulation.run(leftover_rocks);

    simulation.floor.height + skip_height
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    static EXAMPLE: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    static EXAMPLE_AFTER1: &str = indoc! {"
        ..####.
    "};

    static EXAMPLE_AFTER2: &str = indoc! {"
        ...#...
        ..###..
        ...#...
        ..####.
    "};

    static EXAMPLE_AFTER3: &str = indoc! {"
        ..#....
        ..#....
        ####...
        ..###..
        ...#...
        ..####.
    "};

    static EXAMPLE_AFTER4: &str = indoc! {"
        ....#..
        ..#.#..
        ..#.#..
        #####..
        ..###..
        ...#...
        ..####.
    "};

    static EXAMPLE_AFTER5: &str = indoc! {"
        ....##.
        ....##.
        ....#..
        ..#.#..
        ..#.#..
        #####..
        ..###..
        ...#...
        ..####.
    "};

    static EXAMPLE_AFTER6: &str = indoc! {"
        .####..
        ....##.
        ....##.
        ....#..
        ..#.#..
        ..#.#..
        #####..
        ..###..
        ...#...
        ..####.
    "};

    static EXAMPLE_AFTER7: &str = indoc! {"
        ..#....
        .###...
        ..#....
        .####..
        ....##.
        ....##.
        ....#..
        ..#.#..
        ..#.#..
        #####..
        ..###..
        ...#...
        ..####.
    "};

    static EXAMPLE_AFTER8: &str = indoc! {"
        .....#.
        .....#.
        ..####.
        .###...
        ..#....
        .####..
        ....##.
        ....##.
        ....#..
        ..#.#..
        ..#.#..
        #####..
        ..###..
        ...#...
        ..####.
    "};

    static EXAMPLE_AFTER9: &str = indoc! {"
        ....#..
        ....#..
        ....##.
        ....##.
        ..####.
        .###...
        ..#....
        .####..
        ....##.
        ....##.
        ....#..
        ..#.#..
        ..#.#..
        #####..
        ..###..
        ...#...
        ..####.
    "};

    static EXAMPLE_AFTER10: &str = indoc! {"
        ....#..
        ....#..
        ....##.
        ##..##.
        ######.
        .###...
        ..#....
        .####..
        ....##.
        ....##.
        ....#..
        ..#.#..
        ..#.#..
        #####..
        ..###..
        ...#...
        ..####.
    "};

    #[test]
    fn test_part1_example_after1() {
        let floor = part1(EXAMPLE, 1);
        let expected = parse_lines(EXAMPLE_AFTER1);
        assert_eq!(floor.layers, expected);
        assert_eq!(floor.height, expected.len());
    }

    #[test]
    fn test_part1_example_after2() {
        let floor = part1(EXAMPLE, 2);
        let expected = parse_lines(EXAMPLE_AFTER2);
        assert_eq!(floor.layers, expected);
        assert_eq!(floor.height, expected.len());
    }

    #[test]
    fn test_part1_example_after3() {
        let floor = part1(EXAMPLE, 3);
        let expected = parse_lines(EXAMPLE_AFTER3);
        assert_eq!(floor.layers, expected);
        assert_eq!(floor.height, expected.len());
    }

    #[test]
    fn test_part1_example_after4() {
        let floor = part1(EXAMPLE, 4);
        let expected = parse_lines(EXAMPLE_AFTER4);
        assert_eq!(floor.layers, expected);
        assert_eq!(floor.height, expected.len());
    }

    #[test]
    fn test_part1_example_after5() {
        let floor = part1(EXAMPLE, 5);
        let expected = parse_lines(EXAMPLE_AFTER5);
        assert_eq!(floor.layers, expected);
        assert_eq!(floor.height, expected.len());
    }

    #[test]
    fn test_part1_example_after6() {
        let floor = part1(EXAMPLE, 6);
        let expected = parse_lines(EXAMPLE_AFTER6);
        assert_eq!(floor.layers, expected);
        assert_eq!(floor.height, expected.len());
    }

    #[test]
    fn test_part1_example_after7() {
        let floor = part1(EXAMPLE, 7);
        let expected = parse_lines(EXAMPLE_AFTER7);
        assert_eq!(floor.layers, expected);
        assert_eq!(floor.height, expected.len());
    }

    #[test]
    fn test_part1_example_after8() {
        let floor = part1(EXAMPLE, 8);
        let expected = parse_lines(EXAMPLE_AFTER8);
        assert_eq!(floor.layers, expected);
        assert_eq!(floor.height, expected.len());
    }

    #[test]
    fn test_part1_example_after9() {
        let floor = part1(EXAMPLE, 9);
        let expected = parse_lines(EXAMPLE_AFTER9);
        assert_eq!(floor.layers, expected);
        assert_eq!(floor.height, expected.len());
    }

    #[test]
    fn test_part1_example_after10() {
        let floor = part1(EXAMPLE, 10);
        let expected = parse_lines(EXAMPLE_AFTER10);
        assert_eq!(floor.layers, expected);
        assert_eq!(floor.height, expected.len());
    }

    #[test]
    fn test_part1_example_full() {
        let result = part1(EXAMPLE, 2022);
        assert_eq!(result.height, 3068);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file(), 2022);
        assert_eq!(result.height, 3173);
    }

    #[test]
    fn test_part2_example() {
        let result = part2(EXAMPLE, 1000000000000);
        assert_eq!(result, 1514285714288);
    }

    #[test]
    fn test_part2_solution() {
        let result = part2(&read_input_file(), 1000000000000);
        assert_eq!(result, 1570930232582);
    }
}
