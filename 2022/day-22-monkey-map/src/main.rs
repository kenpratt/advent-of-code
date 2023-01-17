use std::cmp;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::ops::{Add, Sub};

use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    println!("part 2 result: {:?}", part2(&read_input_file()));
}

fn read_file(filename: &str) -> String {
    fs::read_to_string(filename).expect("Something went wrong reading the file")
}

fn read_input_file() -> String {
    read_file("input.txt")
}

fn parse(input: &str, mode: WrapMode) -> (Map, Vec<Instruction>) {
    let mut iter = input.split("\n\n");
    let map = Map::parse(iter.next().unwrap(), mode);
    let instructions = Instruction::parse_list(iter.next().unwrap());
    assert_eq!(None, iter.next());
    (map, instructions)
}

#[derive(Debug)]
enum Instruction {
    Advance(u8),
    Turn(Rotation),
}

impl Instruction {
    fn parse_list(input: &str) -> Vec<Self> {
        use Instruction::*;

        lazy_static! {
            static ref INSTRUCTION_RE: Regex = Regex::new(r"((\d+)|([A-Z]))").unwrap();
        }

        INSTRUCTION_RE
            .captures_iter(input)
            .map(|caps| match (caps.get(2), caps.get(3)) {
                (Some(s), None) => Advance(s.as_str().parse::<u8>().unwrap()),
                (None, Some(s)) => Turn(Rotation::parse(s.as_str())),
                _ => panic!("Unreachable"),
            })
            .collect()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Rotation {
    Clockwise,
    Counterclockwise,
}

impl Rotation {
    fn parse(input: &str) -> Self {
        use Rotation::*;
        match input {
            "L" => Counterclockwise,
            "R" => Clockwise,
            _ => panic!("Bad direction: {}", input),
        }
    }

    fn apply(&self, facing: &Facing) -> Facing {
        use Facing::*;
        use Rotation::*;
        match self {
            Clockwise => match facing {
                Up => Right,
                Left => Up,
                Right => Down,
                Down => Left,
            },
            Counterclockwise => match facing {
                Up => Left,
                Left => Down,
                Right => Up,
                Down => Right,
            },
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn advance(
        &self,
        facing: &Facing,
        distance: usize,
        width: usize,
        height: usize,
    ) -> Option<Self> {
        use Facing::*;
        match facing {
            Right => {
                (self.x + distance < width).then(|| Coordinate::new(self.x + distance, self.y))
            }
            Left => (self.x >= distance).then(|| Coordinate::new(self.x - distance, self.y)),
            Down => {
                (self.y + distance < height).then(|| Coordinate::new(self.x, self.y + distance))
            }
            Up => (self.y >= distance).then(|| Coordinate::new(self.x, self.y - distance)),
        }
    }
}

impl<'a, 'b> Add<&'b Coordinate> for &'a Coordinate {
    type Output = Coordinate;

    fn add(self, other: &'b Coordinate) -> Coordinate {
        Coordinate {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<'a, 'b> Sub<&'b Coordinate> for &'a Coordinate {
    type Output = Coordinate;

    fn sub(self, other: &'b Coordinate) -> Coordinate {
        Coordinate {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Facing {
    Right,
    Left,
    Down,
    Up,
}

impl Facing {
    fn opposite(&self) -> Self {
        use Facing::*;
        match self {
            Right => Left,
            Left => Right,
            Down => Up,
            Up => Down,
        }
    }

    fn rotate(&self, rotation: &Facing) -> Self {
        use Facing::*;
        match rotation {
            Right => Rotation::Clockwise.apply(self),
            Left => Rotation::Counterclockwise.apply(self),
            Up => *self,
            Down => Rotation::Clockwise.apply(&Rotation::Clockwise.apply(self)),
        }
    }

    fn unrotate(&self, rotation: &Facing) -> Self {
        use Facing::*;
        match rotation {
            Right => Rotation::Counterclockwise.apply(self),
            Left => Rotation::Clockwise.apply(self),
            Up => *self,
            Down => Rotation::Clockwise.apply(&Rotation::Clockwise.apply(self)),
        }
    }

    fn value(&self) -> usize {
        use Facing::*;
        match self {
            Right => 0,
            Left => 2,
            Down => 1,
            Up => 3,
        }
    }
}

static FACINGS: &'static [Facing] = &[Facing::Right, Facing::Left, Facing::Down, Facing::Up];

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Side {
    Top,
    Bottom,
    Left,
    Right,
    Front,
    Back,
}

impl Side {
    fn fold(&self, facing: &Facing) -> (Self, Facing) {
        match self {
            Side::Top => match facing {
                Facing::Right => (Side::Right, Facing::Right),
                Facing::Left => (Side::Left, Facing::Left),
                Facing::Down => (Side::Front, Facing::Up),
                Facing::Up => (Side::Back, Facing::Down),
            },
            Side::Bottom => match facing {
                Facing::Right => (Side::Right, Facing::Left),
                Facing::Left => (Side::Left, Facing::Right),
                Facing::Down => (Side::Back, Facing::Down),
                Facing::Up => (Side::Front, Facing::Up),
            },
            Side::Left => match facing {
                Facing::Right => (Side::Front, Facing::Up),
                Facing::Left => (Side::Back, Facing::Up),
                Facing::Down => (Side::Bottom, Facing::Left),
                Facing::Up => (Side::Top, Facing::Right),
            },
            Side::Right => match facing {
                Facing::Right => (Side::Back, Facing::Up),
                Facing::Left => (Side::Front, Facing::Up),
                Facing::Down => (Side::Bottom, Facing::Right),
                Facing::Up => (Side::Top, Facing::Left),
            },
            Side::Front => match facing {
                Facing::Right => (Side::Right, Facing::Up),
                Facing::Left => (Side::Left, Facing::Up),
                Facing::Down => (Side::Bottom, Facing::Up),
                Facing::Up => (Side::Top, Facing::Up),
            },
            Side::Back => match facing {
                Facing::Right => (Side::Left, Facing::Up),
                Facing::Left => (Side::Right, Facing::Up),
                Facing::Down => (Side::Bottom, Facing::Down),
                Facing::Up => (Side::Top, Facing::Down),
            },
        }
    }

    fn value(&self) -> usize {
        use Side::*;
        match self {
            Top => 0,
            Bottom => 1,
            Left => 2,
            Right => 3,
            Front => 4,
            Back => 5,
        }
    }

    fn from_value(value: usize) -> Self {
        use Side::*;
        match value {
            0 => Top,
            1 => Bottom,
            2 => Left,
            3 => Right,
            4 => Front,
            5 => Back,
            _ => panic!("Bad value: {}", value),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Tile {
    Open,
    Wall,
}

impl Tile {
    fn parse(input: &char) -> Option<Self> {
        use Tile::*;
        match input {
            ' ' => None,
            '.' => Some(Open),
            '#' => Some(Wall),
            _ => panic!("Bad tile: {}", input),
        }
    }
}

#[derive(Debug)]
struct Map {
    tiles: HashMap<Coordinate, Option<Tile>>,
    panels: Vec<Panel>,
}

impl Map {
    fn parse(input: &str, mode: WrapMode) -> Self {
        let mut rows: Vec<Vec<Option<Tile>>> = input
            .lines()
            .map(|line| line.chars().map(|c| Tile::parse(&c)).collect())
            .collect();

        let map_height = rows.len();
        let map_width = rows.iter().map(|r| r.len()).max().unwrap();
        let panel_width = cmp::max(map_height, map_width) / 4;

        // make the grid rectangular, filling with more empty space on the
        // right side, or else transpose will break
        for row in &mut rows {
            row.resize(map_width, None);
        }

        let tiles: HashMap<Coordinate, Option<Tile>> = rows
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .map(move |(x, tile)| (Coordinate::new(x, y), *tile))
            })
            .collect();

        let panel_offsets: Vec<Coordinate> = (0..map_height)
            .step_by(panel_width)
            .flat_map(|y| {
                (0..map_width)
                    .step_by(panel_width)
                    .map(move |x| Coordinate::new(x, y))
            })
            .filter(|c| tiles.get(&c).unwrap().is_some())
            .collect();

        assert_eq!(6, panel_offsets.len());

        let mut panel_direct_neighbours: [[Option<(usize, Facing)>; 4]; 6] = [[None; 4]; 6];

        let panel_offset_to_id: HashMap<&Coordinate, usize> = panel_offsets
            .iter()
            .enumerate()
            .map(|(id, offset)| (offset, id))
            .collect();

        // fill in direct neighbours
        for (panel_id, panel_offset) in panel_offsets.iter().enumerate() {
            for facing in FACINGS {
                match panel_offset.advance(facing, panel_width, map_width, map_height) {
                    Some(neighbour_offset) => {
                        match panel_offset_to_id.get(&neighbour_offset) {
                            Some(neighbour_id) => {
                                panel_direct_neighbours[panel_id][facing.value()] =
                                    Some((*neighbour_id, *facing));
                            }
                            None => {} // ignore
                        }
                    }
                    None => {} // ignore
                }
            }
        }

        let cube_sides = if mode == WrapMode::Cube {
            Some(Self::solve_cube(&panel_direct_neighbours))
        } else {
            None
        };

        let mut panel_wraps: [[Option<(usize, Facing, bool)>; 4]; 6] = [[None; 4]; 6];
        for (panel_id, _panel_offset) in panel_offsets.iter().enumerate() {
            for facing in FACINGS {
                let wrap = match mode {
                    WrapMode::Flat => Self::calculate_panel_wrap_for_flat_mode(
                        panel_id,
                        facing,
                        &panel_direct_neighbours,
                    ),
                    WrapMode::Cube => Self::calculate_panel_wrap_for_cube_mode(
                        panel_id,
                        facing,
                        &cube_sides.unwrap(),
                    ),
                };
                panel_wraps[panel_id][facing.value()] = Some(wrap);
            }
        }

        let panels = panel_offsets
            .iter()
            .enumerate()
            .map(|(panel_id, panel_offset)| Panel {
                id: panel_id,
                width: panel_width,
                offset: *panel_offset,
                wrap: panel_wraps[panel_id]
                    .into_iter()
                    .flat_map(|o| o)
                    .collect::<Vec<(usize, Facing, bool)>>()
                    .try_into()
                    .unwrap(),
            })
            .collect();

        Self { tiles, panels }
    }

    fn calculate_panel_wrap_for_flat_mode(
        panel_id: usize,
        facing: &Facing,
        panel_direct_neighbours: &[[Option<(usize, Facing)>; 4]; 6],
    ) -> (usize, Facing, bool) {
        match &panel_direct_neighbours[panel_id][facing.value()] {
            Some((wrap_id, wrap_facing)) => (*wrap_id, *wrap_facing, false),
            None => {
                let opposite_facing = facing.opposite();
                let mut last_panel_id = panel_id;
                loop {
                    match panel_direct_neighbours[last_panel_id][opposite_facing.value()] {
                        Some(wrap) => last_panel_id = wrap.0,
                        None => break,
                    }
                }
                (last_panel_id, *facing, false)
            }
        }
    }

    fn solve_cube(
        panel_direct_neighbours: &[[Option<(usize, Facing)>; 4]; 6],
    ) -> [(usize, Facing); 6] {
        let mut sides: [Option<(usize, Facing)>; 6] = [None; 6];
        let mut expanded = HashSet::new();

        sides[Side::Top.value()] = Some((0, Facing::Up));

        let mut to_expand = vec![Side::Top];
        while !to_expand.is_empty() {
            let expand_side = to_expand.pop().unwrap();
            expanded.insert(expand_side);

            let (expand_id, expand_rotation) = &sides[expand_side.value()].unwrap();

            let direct_neigbours = &panel_direct_neighbours[*expand_id];
            for (neighbour_id, neighbour_facing) in
                direct_neigbours.iter().filter_map(|n| n.as_ref())
            {
                let (fold_side, fold_rotation) =
                    expand_side.fold(&neighbour_facing.rotate(expand_rotation));
                let final_rotation = fold_rotation.rotate(expand_rotation);
                if !expanded.contains(&fold_side) {
                    sides[fold_side.value()] = Some((*neighbour_id, final_rotation));
                    to_expand.push(fold_side);
                } else {
                    assert_eq!(
                        sides[fold_side.value()],
                        Some((*neighbour_id, final_rotation))
                    );
                }
            }
        }

        sides
            .into_iter()
            .flat_map(|o| o)
            .collect::<Vec<(usize, Facing)>>()
            .try_into()
            .unwrap()
    }

    fn calculate_panel_wrap_for_cube_mode(
        panel_id: usize,
        facing: &Facing,
        cube_sides: &[(usize, Facing); 6],
    ) -> (usize, Facing, bool) {
        let (from_side, from_rotation) = cube_sides
            .iter()
            .enumerate()
            .find(|(_, (id, _))| *id == panel_id)
            .map(|(side_val, (_, rotation))| (Side::from_value(side_val), rotation))
            .unwrap();

        // for the side we want, apply rotation of the current panel
        let rotated_facing = &facing.rotate(from_rotation);

        // edge fold, and find the target panel
        let (to_side, to_rotation) = from_side.fold(rotated_facing);
        let (to_panel_id, to_panel_rotation) = cube_sides[to_side.value()];

        // apply the rotation from the edge fold
        let folded_facing = &rotated_facing.rotate(&to_rotation);

        // apply the rotation of the new panel (in reverse)
        let final_facing = &folded_facing.unrotate(&to_panel_rotation);

        // do the coordinate directions differ along this fold?
        let edge_direction_reversed = match (facing, final_facing) {
            (Facing::Right, Facing::Left)
            | (Facing::Right, Facing::Down)
            | (Facing::Left, Facing::Right)
            | (Facing::Left, Facing::Up)
            | (Facing::Down, Facing::Right)
            | (Facing::Down, Facing::Up)
            | (Facing::Up, Facing::Left)
            | (Facing::Up, Facing::Down) => true,
            _ => false,
        };

        (to_panel_id, *final_facing, edge_direction_reversed)
    }

    fn tile(&self, pos: &Coordinate) -> &Option<Tile> {
        self.tiles.get(pos).unwrap()
    }

    fn initial_cursor(&self) -> Cursor {
        // start in top left, facing right
        let panel = &self.panels[0];
        let position = &panel.offset;
        assert_eq!(&Some(Tile::Open), self.tile(position));
        Cursor::new(panel.id, *position, Facing::Right)
    }

    fn advance(&self, cursor: &Cursor, distance: u8) -> Cursor {
        let mut curr = *cursor;
        for _ in 0..distance {
            let next = self.next(&curr);
            match self.tile(&next.position) {
                Some(Tile::Open) => curr = next, // continue moving
                Some(Tile::Wall) => break,       // halt
                None => panic!("Bad navigation, should not navigate to empty space"),
            };
        }
        curr
    }

    fn next(&self, cursor: &Cursor) -> Cursor {
        let panel = &self.panels[cursor.panel_id];
        match panel.next(&cursor.position, &cursor.facing) {
            Some(next_position) => {
                // move ahead on same panel, same facing
                Cursor::new(cursor.panel_id, next_position, cursor.facing)
            }
            None => {
                // wrap!
                let (wrap_panel_id, wrap_facing, wrap_offset, edge_direction_reversed) =
                    panel.wrap(&cursor.position, &cursor.facing);

                let wrap_panel = &self.panels[wrap_panel_id];
                let wrap_position = wrap_panel.enter(&wrap_facing, wrap_offset, edge_direction_reversed);
                Cursor::new(wrap_panel_id, wrap_position, wrap_facing)
            }
        }
    }

    fn navigate(&self, instructions: &[Instruction]) -> Cursor {
        let mut cursor = self.initial_cursor();
        for instruction in instructions {
            cursor = match instruction {
                Instruction::Advance(distance) => self.advance(&cursor, *distance),
                Instruction::Turn(r) => cursor.rotate(r),
            };
        }
        cursor
    }
}

#[derive(Debug)]
struct Panel {
    id: usize,
    offset: Coordinate,
    width: usize,
    wrap: [(usize, Facing, bool); 4],
}

impl Panel {
    fn next(&self, pos: &Coordinate, facing: &Facing) -> Option<Coordinate> {
        assert_eq!(true, self.contains(pos));

        let rel = pos - &self.offset; // relative to panel
        rel.advance(facing, 1, self.width, self.width)
            .map(|r| &self.offset + &r)
    }

    fn wrap(&self, pos: &Coordinate, facing: &Facing) -> (usize, Facing, usize, bool) {
        use Facing::*;

        assert_eq!(None, self.next(pos, facing));
        let (wrap_id, wrap_facing, edge_direction_reversed) = &self.wrap[facing.value()];

        let rel = pos - &self.offset; // relative to panel
        let wrap_offset = match facing {
            Right | Left => rel.y,
            Down | Up => rel.x,
        };

        (*wrap_id, *wrap_facing, wrap_offset, *edge_direction_reversed)
    }

    fn enter(&self, facing: &Facing, base_offset: usize, edge_direction_reversed: bool) -> Coordinate {
        use Facing::*;

        let offset = if edge_direction_reversed {
            self.width - base_offset - 1
        } else {
            base_offset
        };

        let rel = match facing {
            Right => Coordinate::new(0, offset),
            Left => Coordinate::new(self.width - 1, offset),
            Down => Coordinate::new(offset, 0),
            Up => Coordinate::new(offset, self.width - 1),
        };
        &rel + &self.offset
    }

    fn contains(&self, pos: &Coordinate) -> bool {
        pos.x >= self.offset.x
            && pos.x < (self.offset.x + self.width)
            && pos.y >= self.offset.y
            && pos.y < (self.offset.y + self.width)
    }
}

#[derive(Clone, Copy, Debug)]
struct Cursor {
    panel_id: usize,
    position: Coordinate,
    facing: Facing,
}

impl Cursor {
    fn new(panel_id: usize, position: Coordinate, facing: Facing) -> Self {
        Self {
            panel_id,
            position,
            facing,
        }
    }

    fn rotate(&self, rotation: &Rotation) -> Cursor {
        let new_facing = rotation.apply(&self.facing);
        Self::new(self.panel_id, self.position, new_facing)
    }

    fn score(&self) -> usize {
        1000 * (self.position.y + 1) + 4 * (self.position.x + 1) + self.facing.value()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum WrapMode {
    Flat,
    Cube,
}

fn part1(input: &str) -> usize {
    let (map, instructions) = parse(input, WrapMode::Flat);
    let cursor = map.navigate(&instructions);
    cursor.score()
}

fn part2(input: &str) -> usize {
    let (map, instructions) = parse(input, WrapMode::Cube);
    let cursor = map.navigate(&instructions);
    cursor.score()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read_example_file() -> String {
        read_file("example.txt")
    }

    #[test]
    fn test_part1_example() {
        let result = part1(&read_example_file());
        assert_eq!(result, 6032);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file());
        assert_eq!(result, 56372);
    }

    #[test]
    fn test_part2_example() {
        let result = part2(&read_example_file());
        assert_eq!(result, 5031);
    }

    #[test]
    fn test_part2_solution() {
        let result = part2(&read_input_file());
        assert_eq!(result, 197047);
    }
}
