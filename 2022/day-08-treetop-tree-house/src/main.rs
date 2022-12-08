use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    // println!("part 2 result: {:?}", part2(&read_input_file()));
}

fn read_input_file() -> String {
    fs::read_to_string("input.txt").expect("Something went wrong reading the file")
}

type UsizeIter = Box<dyn Iterator<Item = usize>>;
type CoordIter = Box<dyn Iterator<Item = Coord>>;
type LanesIter = Box<dyn Iterator<Item = CoordIter>>;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn row(x_range: UsizeIter, y: usize) -> CoordIter {
        Box::new(x_range.map(move |x| Self::new(x, y)))
    }

    fn col(x: usize, y_range: UsizeIter) -> CoordIter {
        Box::new(y_range.map(move |y| Self::new(x, y)))
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Side {
    Top,
    Bottom,
    Left,
    Right,
}

static SIDES: [Side; 4] = [Side::Top, Side::Bottom, Side::Left, Side::Right];

impl Side {
    fn lanes(&self, width: usize, height: usize) -> LanesIter {
        use Side::*;

        match self {
            Top => Box::new((0..width).map(move |x| Coord::col(x, Box::new(0..height)))),
            Bottom => Box::new((0..width).map(move |x| Coord::col(x, Box::new((0..height).rev())))),
            Left => Box::new((0..height).map(move |y| Coord::row(Box::new(0..width), y))),
            Right => Box::new((0..height).map(move |y| Coord::row(Box::new((0..width).rev()), y))),
        }
    }
}

#[derive(Debug)]
struct TreeHeights {
    map: Vec<Vec<u32>>,
    width: usize,
    height: usize,
}

impl TreeHeights {
    fn parse(input: &str) -> Self {
        let map: Vec<Vec<u32>> = input.lines().map(|line| Self::parse_row(line)).collect();
        let width = map[0].len();
        let height = map.len();
        Self { map, width, height }
    }

    fn parse_row(input: &str) -> Vec<u32> {
        input.chars().map(|c| c.to_digit(10).unwrap()).collect()
    }

    fn get(&self, c: &Coord) -> u32 {
        self.map[c.y][c.x]
    }
}

#[derive(Debug)]
struct VisibleTrees {
    trees: HashMap<Coord, HashSet<Side>>,
}

impl VisibleTrees {
    fn calculate(heights: &TreeHeights) -> Self {
        let mut visible = Self::new();

        let width = heights.width;
        let height = heights.height;

        for side in SIDES {
            for coords in side.lanes(width, height) {
                let mut coords_heights = coords.map(|c| {
                    let h = heights.get(&c);
                    (c, h)
                });

                let (first_coord, first_height) = coords_heights.next().unwrap();
                visible.add(&first_coord, &side);

                let mut max_height = first_height;
                for (curr_coord, curr_height) in coords_heights {
                    if curr_height > max_height {
                        visible.add(&curr_coord, &side);
                        max_height = curr_height;
                    }
                }
            }
        }

        visible
    }

    fn new() -> Self {
        Self {
            trees: HashMap::new(),
        }
    }

    fn add(&mut self, c: &Coord, s: &Side) {
        self.trees.entry(*c).or_insert(HashSet::new()).insert(*s);
    }
}

fn part1(input: &str) -> usize {
    let heights = TreeHeights::parse(input);
    println!("heights: {:?}", heights);

    let visible = VisibleTrees::calculate(&heights);
    println!("visible: {:?}", visible);

    visible.trees.len()
}

// fn part2(input: &str) -> usize {
//     let data = TreeHeights::parse(input);
//     println!("{:?}", data);
//     data.execute()
// }

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    static EXAMPLE1: &str = indoc! {"
        30373
        25512
        65332
        33549
        35390
    "};

    #[test]
    fn test_part1_example1() {
        let result = part1(EXAMPLE1);
        assert_eq!(result, 21);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file());
        assert_eq!(result, 1832);
    }

    // #[test]
    // fn test_part2_example1() {
    //     let result = part2(EXAMPLE1);
    //     assert_eq!(result, 0);
    // }

    // #[test]
    // fn test_part2_solution() {
    //     let result = part2(&read_input_file());
    //     assert_eq!(result, 0);
    // }
}
