use crate::file::*;
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

pub fn run() {
    let input = parse(&read_input_file!());
    println!("part 1 result: {:?}", part1(&input));
    println!("part 2 result: {:?}", part2(&input, 10000));
}

struct Map<'a, T> {
    bounds: &'a Bounds,
    width: usize,
    height: usize,
    cells: Vec<Option<T>>,
}

impl<'a, N> Map<'a, N> {
    fn new(bounds: &'a Bounds) -> Self {
        let width = bounds.width() as usize;
        let height = bounds.height() as usize;

        let mut cells = vec![];
        cells.resize_with(width * height, Default::default);

        Self {
            bounds,
            width,
            height,
            cells,
        }
    }

    fn coord_to_index(&self, coord: &Coord) -> usize {
        let x = (coord.x - self.bounds.left) as usize;
        let y = (coord.y - self.bounds.top) as usize;
        y * self.width + x
    }

    fn index_to_coord(&self, index: usize) -> Coord {
        let x = index % self.width + self.bounds.left;
        let y = index / self.width + self.bounds.top;
        Coord::new(x, y)
    }

    fn neighbours(&self, index: usize) -> [Option<usize>; 4] {
        let x = index % self.width;
        let y = index / self.width;

        let mut out = [None; 4];

        if x > 0 {
            out[0] = Some(index - 1);
        }
        if x < self.width - 1 {
            out[1] = Some(index + 1);
        }
        if y > 0 {
            out[2] = Some(index - self.width);
        }
        if y < self.height - 1 {
            out[3] = Some(index + self.width);
        }

        out
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn parse_list(input: &str) -> Vec<Self> {
        input.lines().map(|line| Self::parse(line)).collect()
    }

    fn parse(input: &str) -> Self {
        let mut parts = input.split(", ").map(|s| s.parse::<usize>().unwrap());
        let x = parts.next().unwrap();
        let y = parts.next().unwrap();
        assert_eq!(parts.next(), None);
        Self::new(x, y)
    }

    fn manhattan_distance(&self, other: &Coord) -> usize {
        abs_diff(self.x, other.x) as usize + abs_diff(self.y, other.y) as usize
    }
}

struct Bounds {
    left: usize,
    right: usize,
    top: usize,
    bottom: usize,
}

impl Bounds {
    fn calculate(coords: &[Coord]) -> Self {
        let left = coords.iter().map(|c| c.x).min().unwrap();
        let right = coords.iter().map(|c| c.x).max().unwrap();
        let top = coords.iter().map(|c| c.y).min().unwrap();
        let bottom = coords.iter().map(|c| c.y).max().unwrap();
        Self {
            left,
            right,
            top,
            bottom,
        }
    }

    fn width(&self) -> usize {
        self.right - self.left + 1
    }

    fn height(&self) -> usize {
        self.bottom - self.top + 1
    }
}

fn abs_diff(a: usize, b: usize) -> usize {
    if a <= b {
        b - a
    } else {
        a - b
    }
}

enum Territory {
    Node(usize),
    Conflict,
}

fn parse(input: &str) -> (Vec<Coord>, Bounds) {
    let nodes = Coord::parse_list(input);
    let bounds = Bounds::calculate(&nodes);
    (nodes, bounds)
}

fn part1((nodes, bounds): &(Vec<Coord>, Bounds)) -> usize {
    let mut infinite: HashSet<usize> = HashSet::new();
    let mut map: Map<Territory> = Map::new(&bounds);
    let node_indices: Vec<usize> = nodes.iter().map(|c| map.coord_to_index(c)).collect();

    for i in &node_indices {
        map.cells[*i] = Some(Territory::Node(*i));
    }

    // explore from the nodes
    let mut frontier: Vec<(usize, usize)> = node_indices.iter().map(|i| (*i, *i)).collect();
    while !frontier.is_empty() {
        // for each round of exploration, expand to neighbours, tracking
        // which nodes are exploring to the new cell this round, so we can
        // detect collisions
        let mut tmp_map: Map<Territory> = Map::new(&bounds);
        for (to_expand, node) in frontier {
            for maybe_neighbour in map.neighbours(to_expand) {
                match maybe_neighbour {
                    Some(neighbour) => {
                        if map.cells[neighbour].is_none() {
                            match &tmp_map.cells[neighbour] {
                                Some(Territory::Node(n)) if node == *n => (),
                                Some(Territory::Node(_)) => {
                                    // conflict
                                    tmp_map.cells[neighbour] = Some(Territory::Conflict);
                                }
                                Some(Territory::Conflict) => (),
                                None => {
                                    // new territory
                                    tmp_map.cells[neighbour] = Some(Territory::Node(node));
                                }
                            }
                        }
                    }
                    None => {
                        // no neighbour in this direction, must have hit edge => infinite area
                        infinite.insert(node);
                    }
                }
            }
        }

        let mut new_frontier = vec![];
        for (i, cell) in tmp_map.cells.into_iter().enumerate() {
            match cell {
                Some(val) => {
                    if let Territory::Node(node) = &val {
                        new_frontier.push((i, *node));
                    }
                    map.cells[i] = Some(val);
                }
                None => (),
            }
        }
        frontier = new_frontier;
    }

    // now tally the results
    let mut bounded: HashMap<usize, usize> = HashMap::new();
    for owner in &map.cells {
        match owner {
            Some(Territory::Node(id)) => {
                if !infinite.contains(id) {
                    *bounded.entry(*id).or_default() += 1;
                }
            }
            _ => (),
        }
    }

    // and return the highest
    *bounded.values().max().unwrap()
}

fn part2((nodes, bounds): &(Vec<Coord>, Bounds), target_distance: usize) -> usize {
    let mut map: Map<()> = Map::new(&bounds);

    let middle = Coord::new(
        bounds.left + (bounds.right - bounds.left) / 2,
        bounds.top + (bounds.bottom - bounds.top) / 2,
    );
    let middle_index = map.coord_to_index(&middle);

    // start exploring in the middle, and explore the area that fits the constraints
    let mut winners = 0;
    let mut to_explore: Vec<usize> = vec![middle_index];

    while let Some(curr) = to_explore.pop() {
        if map.cells[curr].is_none() {
            map.cells[curr] = Some(());

            let coord = map.index_to_coord(curr);

            let score: usize = nodes.iter().map(|c| coord.manhattan_distance(c)).sum();
            if score < target_distance {
                winners += 1;

                for maybe_neighbour in map.neighbours(curr) {
                    match maybe_neighbour {
                        Some(neighbour) => to_explore.push(neighbour),
                        None => (),
                    }
                }
            }
        }
    }

    winners
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_FILE: &'static str = "example.txt";

    #[test]
    fn test_part1_example() {
        let result = part1(&parse(&read_example_file!()));
        assert_eq!(result, 17);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&parse(&read_input_file!()));
        assert_eq!(result, 6047);
    }

    #[test]
    fn test_part2_example() {
        let result = part2(&parse(&read_example_file!()), 32);
        assert_eq!(result, 16);
    }

    #[test]
    fn test_part2_solution() {
        let result = part2(&parse(&read_input_file!()), 10000);
        assert_eq!(result, 46320);
    }
}
