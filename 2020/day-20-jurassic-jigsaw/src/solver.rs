use std::collections::HashMap;
use std::fmt;

use crate::tile::*;

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

pub fn solve(tiles: &Vec<Tile>, array_width: usize) -> Vec<Vec<(usize, Direction, usize)>> {
    Solver::new(tiles).solve(array_width)
}

#[derive(Debug)]
struct Solver {
    tiles: HashMap<TileRef, Edges>,
    edges_with_value: HashMap<usize, Edges>,
}

impl Solver {
    fn new(tile_list: &Vec<Tile>) -> Solver {
        let tile_edges_map = Solver::build_tile_edges_map(tile_list);
        let edges_with_value = Solver::build_edges_with_value_map(&tile_edges_map);
        Solver {
            tiles: tile_edges_map,
            edges_with_value: edges_with_value,
        }
    }

    fn build_tile_edges_map(tiles: &Vec<Tile>) -> HashMap<TileRef, Edges> {
        let mut combined = HashMap::new();
        for tile in tiles {
            combined.extend(Solver::calculate_edges_for_tile(tile));
        }
        combined
    }
    
    fn calculate_edges_for_tile(tile: &Tile) -> HashMap<TileRef, Edges> {
        let top_pixels = tile.top();
        let right_pixels = &tile.right();
        let bottom_pixels = tile.bottom();
        let left_pixels = &tile.left();

        let top = Tile::line_to_int(top_pixels.iter());
        let top_rev = Tile::line_to_int(top_pixels.iter().rev());
        let right = Tile::line_to_int(right_pixels.iter());
        let right_rev = Tile::line_to_int(right_pixels.iter().rev());
        let bottom = Tile::line_to_int(bottom_pixels.iter());
        let bottom_rev = Tile::line_to_int(bottom_pixels.iter().rev());
        let left = Tile::line_to_int(left_pixels.iter());
        let left_rev = Tile::line_to_int(left_pixels.iter().rev());

        let clockwise_tile = TileRef::new(tile.id, Direction::Clockwise);
        let clockwise_edges = vec![
            Edge::new(clockwise_tile, Side::Top, top, top_rev),
            Edge::new(clockwise_tile, Side::Right, right, right_rev),
            Edge::new(clockwise_tile, Side::Bottom, bottom_rev, bottom),
            Edge::new(clockwise_tile, Side::Left, left_rev, left),
        ];

        let counterclockwise_tile = TileRef::new(tile.id, Direction::Counterclockwise);
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

    fn solve(&self, array_width: usize) -> Vec<Vec<(usize, Direction, usize)>> {
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

        result.iter().map(|row| {
            row.iter().map(|tile| {
                let t = tile.unwrap();
                (t.0.id, t.0.direction, t.1)
            }).collect()
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