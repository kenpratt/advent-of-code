use std::collections::HashMap;
use std::fs;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    println!("part 2 result: {:?}", part2(&read_input_file()));
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
struct TreeMetadata {
    sides: HashMap<Side, (bool, usize)>,
}

impl TreeMetadata {
    fn new() -> Self {
        Self {
            sides: HashMap::new(),
        }
    }

    fn set(&mut self, s: &Side, metadata: (bool, usize)) {
        self.sides.insert(*s, metadata);
    }

    fn unobstructed_view(&self) -> bool {
        // unobstructed from at least one edge
        self.sides.iter().any(|(_, (blocked, _))| !blocked)
    }

    fn scenic_score(&self) -> usize {
        self.sides
            .iter()
            .map(|(_, (_, d))| *d)
            .reduce(|acc, m| acc * m)
            .unwrap()
    }
}

#[derive(Debug)]
struct TreeMetadataMap {
    trees: HashMap<Coord, TreeMetadata>,
}

impl TreeMetadataMap {
    fn calculate(heights: &TreeHeights) -> Self {
        let mut map = Self::new();

        let width = heights.width;
        let height = heights.height;

        for side in SIDES {
            println!("\n{:?}", side);
            for coords in side.lanes(width, height) {
                let mut coords_with_heights = coords.map(|c| {
                    let h = heights.get(&c);
                    (c, h)
                });

                let (first_coord, first_height) = coords_with_heights.next().unwrap();
                map.set(&first_coord, &side, (false, 0));

                let mut distance_to_edge = 0;
                let mut distance_to_tree_height: HashMap<u32, usize> = HashMap::new();
                distance_to_tree_height.insert(first_height, 0);

                for (curr_coord, curr_height) in coords_with_heights {
                    // increase distances to edge & other trees
                    distance_to_edge += 1;
                    for (_h, d) in distance_to_tree_height.iter_mut() {
                        *d += 1;
                    }

                    // find distance to a tree blocking the view (or edge)
                    let result = match distance_to_tree_height
                        .iter()
                        .filter(|(h, _d)| **h >= curr_height)
                        .min_by_key(|(_h, d)| *d)
                    {
                        Some((_h, d)) => (true, *d),
                        None => (false, distance_to_edge),
                    };
                    map.set(&curr_coord, &side, result);

                    // will overwrite an existing tree of the same height
                    distance_to_tree_height.insert(curr_height, 0);
                }
            }
        }

        map
    }

    fn new() -> Self {
        Self {
            trees: HashMap::new(),
        }
    }

    fn set(&mut self, c: &Coord, s: &Side, metadata: (bool, usize)) {
        self.trees
            .entry(*c)
            .or_insert(TreeMetadata::new())
            .set(s, metadata);
    }

    fn num_visible_from_edge(&self) -> usize {
        self.trees
            .iter()
            .filter(|(_, metadata)| metadata.unobstructed_view())
            .count()
    }

    fn highest_scenic_score(&self) -> usize {
        self.trees
            .iter()
            .map(|(_, metadata)| metadata.scenic_score())
            .max()
            .unwrap()
    }
}

fn part1(input: &str) -> usize {
    let heights = TreeHeights::parse(input);
    println!("heights: {:?}", heights);

    let visible = TreeMetadataMap::calculate(&heights);
    println!("visible: {:?}", visible);

    visible.num_visible_from_edge()
}

fn part2(input: &str) -> usize {
    let heights = TreeHeights::parse(input);
    println!("heights: {:?}", heights);

    let visible = TreeMetadataMap::calculate(&heights);
    println!("visible: {:?}", visible);

    visible.highest_scenic_score()
}

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

    #[test]
    fn test_part2_example1() {
        let result = part2(EXAMPLE1);
        assert_eq!(result, 8);
    }

    #[test]
    fn test_part2_solution() {
        let result = part2(&read_input_file());
        assert_eq!(result, 157320);
    }
}
