use std::collections::HashMap;

use std::fmt;
use std::fs;

use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    // println!("part 2 result: {:?}", part2(&read_input_file()));
}

fn read_input_file() -> String {
    return fs::read_to_string("input.txt").expect("Something went wrong reading the file");
}

#[derive(Debug)]
struct Tile {
    id: usize,
    pixels: Vec<Vec<bool>>,
    width: usize,
    height: usize,
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
enum Side {
    Top,
    Right,
    Bottom,
    Left,
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
enum Direction {
    Clockwise,
    Counterclockwise,
}

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
struct TileRef {
    id: usize,
    direction: Direction,
}

impl TileRef {
    fn new(id: usize, direction: Direction) -> TileRef {
        TileRef {
            id: id,
            direction: direction,
        }
    }
}

impl fmt::Debug for TileRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}-{:?}", self.id, self.direction)
    }
}

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
struct Edge {
    tile: TileRef,
    side: Side,
    value: usize,
}

type Edges = Vec<Edge>;

impl Edge {
    fn new(tile: TileRef, side: Side, value: usize) -> Edge {
        Edge {
            tile: tile,
            side: side,
            value: value,
        }
    }
}

impl fmt::Debug for Edge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}-{:?}[{:?}]", self.tile, self.side, self.value)
    }
}

impl Tile {
    fn parse(input: &str) -> Tile {
        let mut lines = input.lines();
        let id = Tile::parse_id(lines.next().unwrap());
        let pixels = Tile::parse_pixels(lines);
        let width = pixels[0].len();
        let height = pixels.len();
        return Tile {
            id: id,
            pixels: pixels,
            width: width,
            height: height,
        }
    }

    fn parse_id(input: &str) -> usize {
        lazy_static! {
            static ref ID_RE: Regex = Regex::new(r"\ATile (\d+):\z").unwrap();
        }
        let captures = ID_RE.captures(input).unwrap();
        captures.get(1).unwrap().as_str().parse::<usize>().unwrap()
    }

    fn parse_pixels(lines: std::str::Lines) -> Vec<Vec<bool>> {
        lines.map(|line| line.chars().map(|c| Tile::parse_pixel(&c)).collect()).collect()
    }

    fn parse_pixel(c: &char) -> bool {
        match c {
            '#' => true,
            '.' => false,
            _ => panic!("Unsupported pixel char: {}", c),
        }
    }

    fn top(&self) -> &Vec<bool> {
        &self.pixels[0]
    }

    fn bottom(&self) -> &Vec<bool> {
        &self.pixels[self.height - 1]
    }
    
    fn left(&self) -> Vec<bool> {
        (0..self.height).map(|y| self.pixels[y][0]).collect()
    }
    
    fn right(&self) -> Vec<bool> {
        (0..self.height).map(|y| self.pixels[y][self.width - 1]).collect()
    }

    fn calculate_edges(&self) -> HashMap<TileRef, Edges> {
        let top = self.top();
        let bottom = self.bottom();
        let left = &self.left();
        let right = &self.right();

        let clockwise_tile = TileRef::new(self.id, Direction::Clockwise);
        let clockwise_edges = vec![
            Edge::new(clockwise_tile, Side::Top, Tile::line_to_int(top.iter())),
            Edge::new(clockwise_tile, Side::Right, Tile::line_to_int(right.iter())),
            Edge::new(clockwise_tile, Side::Bottom, Tile::line_to_int(bottom.iter().rev())),
            Edge::new(clockwise_tile, Side::Left, Tile::line_to_int(left.iter().rev())),
        ];

        let counterclockwise_tile = TileRef::new(self.id, Direction::Counterclockwise);
        let counterclockwise_edges = vec![
            Edge::new(counterclockwise_tile, Side::Top, Tile::line_to_int(top.iter().rev())),
            Edge::new(counterclockwise_tile, Side::Right, Tile::line_to_int(left.iter())),
            Edge::new(counterclockwise_tile, Side::Bottom, Tile::line_to_int(bottom.iter())),
            Edge::new(counterclockwise_tile, Side::Left, Tile::line_to_int(right.iter().rev())),
        ];

        vec![
            (clockwise_tile, clockwise_edges),
            (counterclockwise_tile, counterclockwise_edges),
        ].into_iter().collect()
    }

    fn line_to_int<'a>(line: impl Iterator<Item=&'a bool>) -> usize {
        line.map(|b| if *b { 1 } else { 0 }).fold(0, |acc, bit| (acc << 1) ^ bit)
    }
}

#[derive(Debug)]
struct CameraArray {
    tiles: Vec<Tile>,
}

impl CameraArray {
    fn parse(input: &str) -> CameraArray {
        let tiles: Vec<Tile> = input.split("\n\n").map(|chunk| Tile::parse(chunk)).collect();

        // assert all tiles are the same size
        assert!(tiles[1..].iter().all(|t| t.width == tiles[0].width));
        assert!(tiles[1..].iter().all(|t| t.height == tiles[0].height));

        return CameraArray {
            tiles: tiles,
        }
    }

    fn solve_for_corner_ids(&self) -> Vec<usize> {
        let tile_edges = self.calculate_tile_edges();
        let solver = Solver::new(&tile_edges);
        solver.solve_for_corner_ids()
    }

    fn calculate_tile_edges(&self) -> HashMap<TileRef, Edges> {
        let mut combined = HashMap::new();
        for tile in &self.tiles {
            combined.extend(tile.calculate_edges());
        }
        combined
    }
}

#[derive(Debug)]
struct Solver<'a> {
    tiles: &'a HashMap<TileRef, Edges>,
    edges_with_value: HashMap<usize, Edges>,
}

impl Solver<'_> {
    fn new<'a>(tiles: &'a HashMap<TileRef, Edges>) -> Solver<'a> {
        let edges_with_value = Solver::build_edges_with_value_map(tiles);
        Solver {
            tiles: tiles,
            edges_with_value: edges_with_value,
        }
    }

    fn build_edges_with_value_map<'a>(tiles: &'a HashMap<TileRef, Edges>) -> HashMap<usize, Edges> {
        let mut result = HashMap::new();
        for (_, edges) in tiles {
            for edge in edges {
                let list = result.entry(edge.value).or_insert(vec![]);
                list.push(*edge);
            }
        }
        result
    }

    fn solve_for_corner_ids(&self) -> Vec<usize> {
        let mut ids: Vec<usize> = self.corner_tiles().iter().map(|tile| tile.id).collect();
        ids.sort();
        ids.dedup();
        assert_eq!(ids.len(), 4);
        ids
    }

    fn corner_tiles(&self) -> Vec<TileRef> {
        self.tiles.iter().filter(|(_, edges)| {
            self.num_unreachable_edges(edges) == 2
        }).map(|(t, _)| *t).collect()
    }

    fn num_unreachable_edges(&self, edges: &Edges) -> usize {
        edges.iter().filter(|edge| {
            self.num_valid_connections_for_edge(edge) == 0
        }).count()
    }

    fn num_valid_connections_for_edge(&self, edge: &Edge) -> usize {
        let options = self.edges_with_value.get(&edge.value).unwrap();
        // only count connections to a different tile
        options.iter().filter(|other_edge| other_edge.tile.id != edge.tile.id).count()
    }
}

fn part1(input: &str) -> usize {
    let array = CameraArray::parse(input);
    array.solve_for_corner_ids().iter().fold(1, |acc, id| acc * id)
}

// fn part2(input: &str) -> usize {
//     let data = CameraArray::parse(input);
//     return data.execute();
// }

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    static EXAMPLE1: &str = indoc! {"
        Tile 2311:
        ..##.#..#.
        ##..#.....
        #...##..#.
        ####.#...#
        ##.##.###.
        ##...#.###
        .#.#.#..##
        ..#....#..
        ###...#.#.
        ..###..###
    
        Tile 1951:
        #.##...##.
        #.####...#
        .....#..##
        #...######
        .##.#....#
        .###.#####
        ###.##.##.
        .###....#.
        ..#.#..#.#
        #...##.#..
    
        Tile 1171:
        ####...##.
        #..##.#..#
        ##.#..#.#.
        .###.####.
        ..###.####
        .##....##.
        .#...####.
        #.##.####.
        ####..#...
        .....##...
    
        Tile 1427:
        ###.##.#..
        .#..#.##..
        .#.##.#..#
        #.#.#.##.#
        ....#...##
        ...##..##.
        ...#.#####
        .#.####.#.
        ..#..###.#
        ..##.#..#.
    
        Tile 1489:
        ##.#.#....
        ..##...#..
        .##..##...
        ..#...#...
        #####...#.
        #..#.#.#.#
        ...#.#.#..
        ##.#...##.
        ..##.##.##
        ###.##.#..
        
        Tile 2473:
        #....####.
        #..#.##...
        #.##..#...
        ######.#.#
        .#...#.#.#
        .#########
        .###.#..#.
        ########.#
        ##...##.#.
        ..###.#.#.
    
        Tile 2971:
        ..#.#....#
        #...###...
        #.#.###...
        ##.##..#..
        .#####..##
        .#..####.#
        #..#.#..#.
        ..####.###
        ..#.#.###.
        ...#.#.#.#
    
        Tile 2729:
        ...#.#.#.#
        ####.#....
        ..#.#.....
        ....#..#.#
        .##..##.#.
        .#.####...
        ####.#.#..
        ##.####...
        ##..#.##..
        #.##...##.
    
        Tile 3079:
        #.#.#####.
        .#..######
        ..#.......
        ######....
        ####.#..#.
        .#...#.##.
        #.#####.##
        ..#.###...
        ..#.......
        ..#.###...
    "};    

    #[test]
    fn test_part1_example1() {
        let result = part1(EXAMPLE1);
        assert_eq!(result, 20899048083289);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file());
        assert_eq!(result, 111936085519519);
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