pub mod astar;
pub mod grid;

use astar::AStarInterface;
use grid::Cell;
use grid::Coordinate;
use grid::Grid;

use std::collections::hash_map::DefaultHasher;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::fmt;
use std::hash::{Hash, Hasher};

use indoc::indoc;
use itertools::Itertools;

static INPUT1: &str = indoc! {"
    #############
    #...........#
    ###B#B#D#A###
      #C#A#D#C#  
      #########  
"};

static INPUT2: &str = indoc! {"
    #############
    #...........#
    ###B#B#D#A###
      #D#C#B#A#  
      #D#B#A#C#  
      #C#A#D#C#  
      #########  
"};

fn main() {
    // println!("part 1 result: {:?}", lowest_cost_path(INPUT1));
    println!("part 2 result: {:?}", lowest_cost_path(INPUT2));
}

#[derive(Debug)]
struct Map {
    grid: Grid<Tile>,
    hallways: Vec<Coordinate>,
    rooms: BTreeMap<Amphipod, BTreeSet<Coordinate>>,
    initial_positions: BTreeMap<Coordinate, Amphipod>,
    paths: HashMap<(Coordinate, Coordinate), (Vec<Coordinate>, usize)>,
}

impl Map {
    fn new(input: &str) -> Map {
        let raw_input = Self::parse(input);

        // build grid
        let tiles: Vec<Vec<Tile>> = raw_input
            .iter()
            .map(|row| row.iter().map(|(tile, _amphipod)| *tile).collect())
            .collect();
        let mut grid = Grid::new(tiles);
        let rooms = Self::find_rooms(&grid);
        Self::fix_entrances(&mut grid, &rooms);

        let hallways = grid
            .iter()
            .filter(|c| c.value.is_hallway())
            .map(|c| c.position)
            .collect();

        // build initial positions map
        let initial_positions: BTreeMap<Coordinate, Amphipod> = raw_input
            .into_iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.into_iter()
                    .enumerate()
                    .map(|(x, (_tile, amphipod))| amphipod.map(|v| (Coordinate::new(x, y), v)))
                    .collect::<Vec<Option<(Coordinate, Amphipod)>>>()
            })
            .filter_map(|e| e)
            .collect();

        let visitable_locations: Vec<Coordinate> = grid
            .iter()
            .filter(|c| c.value.is_hallway() || c.value.is_room())
            .map(|c| c.position)
            .collect();
        let paths = Self::precompute_paths(&visitable_locations, &grid);

        Map {
            grid,
            hallways,
            rooms,
            initial_positions,
            paths,
        }
    }

    fn parse(input: &str) -> Vec<Vec<(Tile, Option<Amphipod>)>> {
        input
            .lines()
            .map(|line| line.chars().map(|c| Tile::parse(&c)).collect())
            .collect()
    }

    fn find_rooms(grid: &Grid<Tile>) -> BTreeMap<Amphipod, BTreeSet<Coordinate>> {
        let rooms: Vec<Coordinate> = grid
            .iter()
            .filter(|c| c.value.is_room())
            .map(|c| c.position)
            .collect();

        let x_coords: Vec<usize> = rooms.iter().map(|c| c.x).sorted().unique().collect();
        let amphipod_mappings: BTreeMap<usize, Amphipod> = x_coords
            .into_iter()
            .enumerate()
            .map(|(i, x)| (x, Amphipod::nth(i)))
            .collect();

        let mut output = BTreeMap::new();
        for room in rooms {
            let amphipod = *amphipod_mappings.get(&room.x).unwrap();
            let v = output.entry(amphipod).or_insert(BTreeSet::new());
            v.insert(room);
        }
        output
    }

    fn fix_entrances(
        grid: &mut Grid<Tile>,
        amphipod_rooms: &BTreeMap<Amphipod, BTreeSet<Coordinate>>,
    ) {
        for (_kind, rooms) in amphipod_rooms.iter() {
            let entrance = Self::find_entrance(grid, rooms);
            grid.cell_mut(&entrance).value = Tile::Entrance;
        }
    }

    fn find_entrance(grid: &Grid<Tile>, rooms: &BTreeSet<Coordinate>) -> Coordinate {
        let entrances: Vec<Coordinate> = rooms
            .iter()
            .flat_map(|c| {
                grid.neighbours(c)
                    .iter()
                    .filter(|n| grid.value(n).is_hallway())
                    .cloned()
                    .collect::<Vec<Coordinate>>()
            })
            .collect();
        assert_eq!(1, entrances.len());
        entrances[0]
    }

    fn rooms_for(&self, kind: &Amphipod) -> &BTreeSet<Coordinate> {
        self.rooms.get(kind).unwrap()
    }

    fn is_room(&self, location: &Coordinate) -> bool {
        self.grid.value(location).is_room()
    }

    fn is_room_of_kind(&self, location: &Coordinate, kind: &Amphipod) -> bool {
        self.rooms_for(kind).contains(location)
    }

    fn precompute_paths(
        locations: &[Coordinate],
        grid: &Grid<Tile>,
    ) -> HashMap<(Coordinate, Coordinate), (Vec<Coordinate>, usize)> {
        let mut map = HashMap::new();
        for from in locations {
            for to in locations {
                map.insert((*from, *to), Self::calculate_path_between(from, to, grid));
            }
        }
        map
    }

    fn calculate_path_between(
        from: &Coordinate,
        to: &Coordinate,
        grid: &Grid<Tile>,
    ) -> (Vec<Coordinate>, usize) {
        let mut pathfinding = Pathfinding {
            goal: to,
            grid: grid,
        };
        let (path_with_costs, total_cost) = pathfinding.shortest_path(from, false);
        let path = path_with_costs.into_iter().map(|(l, _c)| l).collect();
        (path, total_cost)
    }

    fn path_between(&self, from: &Coordinate, to: &Coordinate) -> &(Vec<Coordinate>, usize) {
        self.paths.get(&(*from, *to)).unwrap()
    }
}

#[derive(Debug)]
struct Pathfinding<'a> {
    goal: &'a Coordinate,
    grid: &'a Grid<Tile>,
}

impl AStarInterface<Coordinate> for Pathfinding<'_> {
    fn at_goal(&self, node: &Coordinate) -> bool {
        node == self.goal
    }

    fn heuristic(&self, from: &Coordinate) -> usize {
        from.manhattan_distance(self.goal)
    }

    fn neighbours(&mut self, from: &Coordinate) -> Vec<(Coordinate, usize)> {
        let mut possible_moves = vec![];
        for neighbour_location in self.grid.neighbours(from) {
            let neighbour_tile = self.grid.value(&neighbour_location);
            match neighbour_tile {
                Tile::Hallway | Tile::Room => {
                    // straightforward, move to location at cost of 1 unit
                    possible_moves.push((neighbour_location, 1));
                }
                Tile::Entrance => {
                    // can't stop at an entrance, need to keep moving!
                    let second_neighbours = self.neighbours(&neighbour_location);
                    for (n, cost) in second_neighbours {
                        // can't end where we started
                        if n != *from {
                            // add 1 to cost
                            possible_moves.push((n, cost + 1));
                        }
                    }
                }
                Tile::Wall | Tile::Empty => {
                    // do nothing, impassable
                }
            }
        }
        possible_moves
    }
}

#[derive(Clone, Copy, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
enum Amphipod {
    Amber,
    Bronze,
    Copper,
    Desert,
}

static AMPHIPODS: [Amphipod; 4] = [
    Amphipod::Amber,
    Amphipod::Bronze,
    Amphipod::Copper,
    Amphipod::Desert,
];

impl Amphipod {
    fn parse(c: &char) -> Self {
        match c {
            'A' => Self::Amber,
            'B' => Self::Bronze,
            'C' => Self::Copper,
            'D' => Self::Desert,
            _ => panic!("Unrecognized char: {}", c),
        }
    }

    fn to_char(&self) -> char {
        match self {
            Self::Amber => 'A',
            Self::Bronze => 'B',
            Self::Copper => 'C',
            Self::Desert => 'D',
        }
    }

    fn nth(i: usize) -> Self {
        match i {
            0 => Self::Amber,
            1 => Self::Bronze,
            2 => Self::Copper,
            3 => Self::Desert,
            _ => panic!("Unrecognized index: {}", i),
        }
    }

    fn cost(&self) -> usize {
        match self {
            Self::Amber => 1,
            Self::Bronze => 10,
            Self::Copper => 100,
            Self::Desert => 1000,
        }
    }
}

impl fmt::Display for Amphipod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

#[derive(Clone, Copy, Debug)]
enum Tile {
    Hallway,
    Entrance,
    Room,
    Wall,
    Empty,
}

impl Tile {
    fn parse(c: &char) -> (Self, Option<Amphipod>) {
        match c {
            '.' => (Self::Hallway, None),
            '#' => (Self::Wall, None),
            ' ' => (Self::Empty, None),
            'A' | 'B' | 'C' | 'D' => (Self::Room, Some(Amphipod::parse(c))),
            _ => panic!("Unrecognized char: {}", c),
        }
    }

    fn to_char(&self) -> char {
        match self {
            Self::Hallway => '.',
            Self::Entrance => 'e',
            Self::Room => 'r',
            Self::Wall => '#',
            Self::Empty => ' ',
        }
    }

    fn is_room(&self) -> bool {
        match self {
            Self::Room => true,
            _ => false,
        }
    }

    fn is_hallway(&self) -> bool {
        match self {
            Self::Hallway => true,
            _ => false,
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct GameState {
    positions: BTreeMap<Coordinate, Amphipod>,
}

impl GameState {
    fn initial(map: &Map) -> GameState {
        let positions = map.initial_positions.clone();
        GameState { positions }
    }

    fn heuristic(&self, map: &Map) -> usize {
        // TODO consider the case where a Amphipod is at home but blocking something else?
        // it's probably slowing down astar a bunch to consider those already at home
        self.positions
            .iter()
            .filter(|(c, a)| !map.is_room_of_kind(c, a)) // already in home room
            .map(|(c, a)| self.cost_to_closest_unoccupied_room(c, a, map))
            .sum()
    }

    fn cost_to_closest_unoccupied_room(
        &self,
        from: &Coordinate,
        kind: &Amphipod,
        map: &Map,
    ) -> usize {
        self.unoccupied_rooms_for(kind, map)
            .iter()
            .map(|to| {
                let (_path, distance) = map.path_between(from, to);
                distance * kind.cost()
            })
            .min()
            .unwrap_or(0)
    }

    fn is_complete(&self, map: &Map) -> bool {
        self.positions
            .iter()
            .all(|(c, a)| map.is_room_of_kind(c, a))
    }

    fn available_moves(&self, map: &Map) -> Vec<(GameState, usize)> {
        let unoccupied_rooms = self.unoccupied_rooms(map);
        let unoccupied_hallways = self.unoccupied_hallways(map);

        let mut moves = vec![];

        // TODO filter out amphipods that never need to move again (eg home + not blocking anything/empty spaces)
        for (position, kind) in &self.positions {
            // can always move to a room
            let unoccupied_rooms_for_kind = unoccupied_rooms.get(kind).unwrap();
            self.add_unobstructed_moves(
                position,
                kind,
                map,
                &unoccupied_rooms_for_kind,
                &mut moves,
            );

            if map.is_room(position) {
                // currently in the wrong room, could move to an open hallway spot as well as a room
                self.add_unobstructed_moves(position, kind, map, &unoccupied_hallways, &mut moves);
            }
        }

        moves
    }

    fn occupied(&self, location: &Coordinate) -> bool {
        self.positions.contains_key(location)
    }

    fn unoccupied_rooms_for<'a>(&self, kind: &Amphipod, map: &'a Map) -> Vec<&'a Coordinate> {
        map.rooms_for(kind)
            .iter()
            .filter(|c| !self.occupied(c))
            .collect()
    }

    fn unoccupied_rooms<'a>(
        &self,
        map: &'a Map,
    ) -> HashMap<&'static Amphipod, Vec<&'a Coordinate>> {
        AMPHIPODS
            .iter()
            .map(|kind| (kind, self.unoccupied_rooms_for(kind, map)))
            .collect()
    }

    fn unoccupied_hallways<'a>(&self, map: &'a Map) -> Vec<&'a Coordinate> {
        map.hallways.iter().filter(|c| !self.occupied(c)).collect()
    }

    fn add_unobstructed_moves(
        &self,
        position: &Coordinate,
        kind: &Amphipod,
        map: &Map,
        destinations: &[&Coordinate],
        moves: &mut Vec<(GameState, usize)>,
    ) {
        for destination in destinations {
            let (path, distance) = map.path_between(position, destination);
            let is_obstructed = path.iter().any(|l| self.occupied(l));
            if !is_obstructed {
                let cost = distance * kind.cost();
                let m = (self.state_after_move(position, destination, kind), cost);
                moves.push(m);
            }
        }
    }

    fn state_after_move(&self, from: &Coordinate, to: &Coordinate, kind: &Amphipod) -> GameState {
        let mut cloned = self.clone();
        cloned.apply_move(from, to, kind);
        cloned
    }

    fn apply_move(&mut self, from: &Coordinate, to: &Coordinate, kind: &Amphipod) {
        self.positions.remove(from);
        self.positions.insert(*to, *kind);
    }

    fn render(&self, map: &Map) -> String {
        (0..(map.grid.bounds.height))
            .map(|y| {
                (0..(map.grid.bounds.width))
                    .map(|x| self.render_cell(map.grid.cell(&Coordinate::new(x, y))))
                    .collect()
            })
            .collect::<Vec<String>>()
            .join("\n")
    }

    fn render_cell(&self, cell: &Cell<Tile>) -> char {
        if self.positions.contains_key(&cell.position) {
            self.positions.get(&cell.position).unwrap().to_char()
        } else {
            cell.value.to_char()
        }
    }
}

struct Solver<'a> {
    map: &'a Map,
    game_states: HashMap<u64, GameState>,
}

impl Solver<'_> {
    fn solve(map: &Map) -> usize {
        let initial_state = GameState::initial(map);
        let mut solver = Solver {
            map: map,
            game_states: HashMap::new(),
        };
        let initial_state = solver.find_or_insert_game_state(initial_state);
        let (path, cost) = solver.shortest_path(&initial_state, true);
        println!("solution (total cost: {})", cost);
        for (key, cost_for_step) in &path {
            let state = solver.game_state(key);
            println!("{}\ncost: {}\n", state.render(map), cost_for_step);
        }
        println!(
            "calculated total cost: {}",
            path.iter().map(|(_, c)| c).sum::<usize>()
        );
        cost
    }

    fn find_or_insert_game_state(&mut self, game_state: GameState) -> u64 {
        let key = calculate_hash(&game_state);
        if !self.game_states.contains_key(&key) {
            self.game_states.insert(key, game_state);
        }
        key
    }

    fn game_state(&self, hash: &u64) -> &GameState {
        &self.game_states[hash]
    }
}

impl AStarInterface<u64> for Solver<'_> {
    fn at_goal(&self, node: &u64) -> bool {
        self.game_state(node).is_complete(self.map)
    }

    fn heuristic(&self, from: &u64) -> usize {
        self.game_state(from).heuristic(self.map)
    }

    fn neighbours(&mut self, from: &u64) -> Vec<(u64, usize)> {
        self.game_state(from)
            .available_moves(self.map)
            .into_iter()
            .map(|(state, cost)| (self.find_or_insert_game_state(state), cost))
            .collect()
    }
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

fn lowest_cost_path(input: &str) -> usize {
    let map = Map::new(input);
    println!("{}", map.grid);
    Solver::solve(&map)
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE1: &str = indoc! {"
        #############
        #...........#
        ###B#C#B#D###
          #A#D#C#A#  
          #########  
    "};

    static EXAMPLE2: &str = indoc! {"
        #############
        #...........#
        ###B#C#B#D###
          #D#C#B#A#  
          #D#B#A#C#  
          #A#D#C#A#  
          #########  
    "};

    #[test]
    fn test_part1_example() {
        let result = lowest_cost_path(EXAMPLE1);
        assert_eq!(result, 12521);
    }

    #[test]
    fn test_part1_solution() {
        let result = lowest_cost_path(INPUT1);
        assert_eq!(result, 11608);
    }

    #[test]
    fn test_part2_example() {
        let result = lowest_cost_path(EXAMPLE2);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_part2_solution() {
        let result = lowest_cost_path(INPUT2);
        assert_eq!(result, 0);
    }
}
