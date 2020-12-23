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

impl Side {
    fn calculate_rotation(&self, other_side: &Side) -> usize {
        (4 + other_side.value() - self.value()) % 4
    }

    fn value(&self) -> usize {
        match *self {
            Side::Top => 0,
            Side::Left => 1, // 1 rotation clockwise to make left=top
            Side::Bottom => 2,
            Side::Right => 3, // 3 rotations clockwise to make right=top
        }
    }

    fn rotate(&self, amount: usize) -> Side {
        if amount > 0 {
            let side_val = (self.value() + amount) % 4;
            match side_val {
                0 => Side::Top,
                1 => Side::Left,
                2 => Side::Bottom,
                3 => Side::Right,
                _ => panic!("Unreachable"),
            }
        } else {
            *self
        }
    }
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

type TileRefWithRotation = (TileRef, usize);

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
struct Edge {
    tile: TileRef,
    side: Side,
    value: usize,
    value_rev: usize,
}

type Edges = Vec<Edge>;

impl Edge {
    fn new(tile: TileRef, side: Side, value: usize, value_rev: usize) -> Edge {
        Edge {
            tile: tile,
            side: side,
            value: value,
            value_rev: value_rev,
        }
    }
}

impl fmt::Debug for Edge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}-{:?}[{}/{}]", self.tile, self.side, self.value, self.value_rev)
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

    fn pixel_to_char(b: &bool) -> char {
        match b {
            true => '#',
            false => '.',
        }
    }

    fn to_string(&self) -> String {
        let row_strings: Vec<String> = self.pixels.iter().map(|row| {
            row.iter().map(|p| Tile::pixel_to_char(p)).collect()
        }).collect();
        row_strings.join("\n")
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
        let top_pixels = self.top();
        let right_pixels = &self.right();
        let bottom_pixels = self.bottom();
        let left_pixels = &self.left();

        let top = Tile::line_to_int(top_pixels.iter());
        let top_rev = Tile::line_to_int(top_pixels.iter().rev());
        let right = Tile::line_to_int(right_pixels.iter());
        let right_rev = Tile::line_to_int(right_pixels.iter().rev());
        let bottom = Tile::line_to_int(bottom_pixels.iter());
        let bottom_rev = Tile::line_to_int(bottom_pixels.iter().rev());
        let left = Tile::line_to_int(left_pixels.iter());
        let left_rev = Tile::line_to_int(left_pixels.iter().rev());

        let clockwise_tile = TileRef::new(self.id, Direction::Clockwise);
        let clockwise_edges = vec![
            Edge::new(clockwise_tile, Side::Top, top, top_rev),
            Edge::new(clockwise_tile, Side::Right, right, right_rev),
            Edge::new(clockwise_tile, Side::Bottom, bottom_rev, bottom),
            Edge::new(clockwise_tile, Side::Left, left_rev, left),
        ];

        let counterclockwise_tile = TileRef::new(self.id, Direction::Counterclockwise);
        let counterclockwise_edges = vec![
            Edge::new(counterclockwise_tile, Side::Top, top_rev, top),
            Edge::new(counterclockwise_tile, Side::Right, left, left_rev),
            Edge::new(counterclockwise_tile, Side::Bottom, bottom, bottom_rev),
            Edge::new(counterclockwise_tile, Side::Left, right_rev, right),
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
    array_width: usize,
}

impl CameraArray {
    fn parse(input: &str) -> CameraArray {
        let tiles: Vec<Tile> = input.split("\n\n").map(|chunk| Tile::parse(chunk)).collect();

        // assert all tiles are the same size, and square
        assert!(tiles[1..].iter().all(|t| t.width == tiles[0].width));
        assert!(tiles[1..].iter().all(|t| t.height == tiles[0].height));
        assert_eq!(tiles[0].width, tiles[0].height);

        // figure out array width
        let array_width = (tiles.len() as f64).sqrt().round() as usize;
        assert_eq!(array_width * array_width, tiles.len());

        return CameraArray {
            tiles: tiles,
            array_width: array_width,
        }
    }

    fn solve_tile_layout(&self) -> Vec<Vec<TileRefWithRotation>> {
        let tile_edges = self.calculate_tile_edges();
        let solver = Solver::new(&tile_edges);
        solver.solve(self.array_width)
    }

    fn solve_for_corner_ids(&self) -> Vec<usize> {
        let layout = self.solve_tile_layout();
        let max = self.array_width - 1;
        vec![
            layout[0][0].0.id,
            layout[0][max].0.id,
            layout[max][0].0.id,
            layout[max][max].0.id,
        ]
    }

    fn solve_for_combined_image(&self) -> Tile {
        let tile_layout = self.solve_tile_layout();

        // TODO write code to create a new tile out of all these tiles
        panic!("ahhhh");

        // combined_tile.to_string()
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
                // use value_rev as when matching eg the right edge of tile A
                // with the left edge of tile B, and it's a correct match,
                // A's value will be read clockwise, and B will be read
                // counterclockwise.
                let list = result.entry(edge.value_rev).or_insert(vec![]);
                list.push(*edge);
            }
        }
        result
    }

    fn solve(&self, array_width: usize) -> Vec<Vec<TileRefWithRotation>> {
        let mut result: Vec<Vec<Option<TileRefWithRotation>>> = vec![vec![None; array_width]; array_width];

        println!("tiles:");
        let mut refs: Vec<&TileRef> = self.tiles.keys().collect();
        refs.sort_by_key(|r| r.id);
        for r in refs {
            println!("{:?}: {:?}", r, self.tiles.get(r).unwrap());
        }
        println!();

        println!("value map:");
        let mut vals: Vec<&usize> = self.edges_with_value.keys().collect();
        vals.sort();
        for v in vals {
            println!("{}: {}", v, self.edges_with_value.get(v).unwrap().len());
        }
        println!();

        for y in 0..array_width {
            for x in 0..array_width {
                let tile = if x == 0 && y == 0 {
                    // special case, first cell
                    self.choose_starting_tile()
                } else {
                    let (previous_tile, side_to_match) = if x == 0 {
                        // prev=tile above this one
                        (result[y-1][x].unwrap(), Side::Bottom)
                    } else {
                        // prev=tile to the left of this one
                        (result[y][x-1].unwrap(), Side::Right)
                    };                  
                    self.choose_tile_with_edge(&previous_tile, side_to_match)
                };
                println!("solved {},{}: {:?} {:?}", x, y, tile, self.tiles.get(&tile.0).unwrap());
                result[y][x] = Some(tile);
            }
        }

        println!("solved! {:?}", result);

        // same grid but unpack the Option types
        result.iter().map(|row| {
            row.iter().map(|tile| tile.unwrap()).collect()
        }).collect()
    }

    fn choose_starting_tile(&self) -> TileRefWithRotation {
        // find the top left corner tile (top and left edges unreachable)
        let tile = *self.tiles.iter().find(|(tile, edges)| {
            let top = edges.iter().find(|edge| edge.side == Side::Top).unwrap();
            let left = edges.iter().find(|edge| edge.side == Side::Left).unwrap();
            tile.direction == Direction::Clockwise &&
                self.connection_for_edge(top) == None &&
                self.connection_for_edge(left) == None
        }).unwrap().0;
        (tile, 0)
    }

    fn choose_tile_with_edge(&self, previous_tile: &TileRefWithRotation, side_to_match: Side) -> TileRefWithRotation {
        let previous_edge = self.get_edge(previous_tile, side_to_match);
        println!("choose_tile_with_edge {:?} {:?} {:?}", previous_tile, side_to_match, previous_edge);
        let new_edge = self.connection_for_edge(previous_edge).unwrap();
        println!("new_edge: {:?}", new_edge);

        let intended_side = match side_to_match {
            Side::Bottom => Side::Top,
            Side::Right => Side::Left,
            _ => panic!("solver currently only solves bottom and right sides of tiles"),
        };

        let rotation = intended_side.calculate_rotation(&new_edge.side);
        (new_edge.tile, rotation)
    }

    fn get_edge(&self, tile: &TileRefWithRotation, side_before_rotation: Side) -> &Edge {
        let side = side_before_rotation.rotate(tile.1);
        let edges = self.tiles.get(&tile.0).unwrap();
        edges.iter().find(|edge| edge.side == side).unwrap()
    }

    fn connection_for_edge(&self, edge: &Edge) -> Option<&Edge> {
        let options = self.edges_with_value.get(&edge.value).unwrap();
        // only count connections to a different tile
        options.iter().find(|other_edge| other_edge.tile.id != edge.tile.id)
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

    static EXAMPLE1_COMBINED_IMAGE: &str = indoc! {"
        .#.#..#.##...#.##..#####
        ###....#.#....#..#......
        ##.##.###.#.#..######...
        ###.#####...#.#####.#..#
        ##.#....#.##.####...#.##
        ...########.#....#####.#
        ....#..#...##..#.#.###..
        .####...#..#.....#......
        #..#.##..#..###.#.##....
        #.####..#.####.#.#.###..
        ###.#.#...#.######.#..##
        #.####....##..########.#
        ##..##.#...#...#.#.#.#..
        ...#..#..#.#.##..###.###
        .#.#....#.##.#...###.##.
        ###.#...#..#.##.######..
        .#.#.###.##.##.#..#.##..
        .####.###.#...###.#..#.#
        ..#.#..#..#.#.#.####.###
        #..####...#.#.#.###.###.
        #####..#####...###....##
        #.##..#..#...#..####...#
        .#.###..##..##..####.##.
        ...###...##...#...#..###
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

    #[test]
    fn test_example1_combined_image() {
        let array = CameraArray::parse(EXAMPLE1);
        let image = array.solve_for_combined_image();
        assert_eq!(image.to_string(), EXAMPLE1_COMBINED_IMAGE);
    }

    // #[test]
    // fn test_part2_solution() {
    //     let result = part2(&read_input_file());
    //     assert_eq!(result, 0);
    // }
}