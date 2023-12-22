pub mod grid;

use std::{
    collections::{BTreeSet, HashMap, HashSet},
    fs,
};

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use grid::*;

use lazy_static::lazy_static;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file(), 64));
    println!("part 2 result: {:?}", part2(&read_input_file(), 26501365));
}

fn read_input_file() -> String {
    fs::read_to_string("input.txt").expect("Something went wrong reading the file")
}

const EMPTY_COORDS: BTreeSet<Coord> = BTreeSet::new();
lazy_static! {
    static ref EMPTY_STATE: u64 = calculate_hash(&EMPTY_COORDS);
}

struct Solver<'a> {
    grid: &'a Grid<Terrain>,
    rocks: HashSet<Coord>,
    starting_location: Coord,
    next_state_cache: HashMap<(u64, [u64; 4]), u64>,
    coords_cache: HashMap<u64, BTreeSet<Coord>>,
    outputs_cache: HashMap<u64, (BTreeSet<Coord>, [BTreeSet<Coord>; 4])>,
    active_tiles: HashMap<Coord, u64>,
    looping_tiles: HashSet<Coord>,
    tile_histories: HashMap<Coord, (usize, Vec<((u64, [u64; 4]), u64)>)>,
}

impl<'a> Solver<'a> {
    fn solve(grid: &'a Grid<Terrain>, steps: usize) -> usize {
        let rocks: HashSet<Coord> = grid
            .cells
            .iter()
            .filter(|(_pos, terrain)| **terrain == Terrain::Rock)
            .map(|(pos, _terrain)| *pos)
            .collect();

        let starting_location = *grid
            .cells
            .iter()
            .find(|(_pos, terrain)| **terrain == Terrain::Start)
            .map(|(pos, _terrain)| pos)
            .unwrap();

        let mut solver = Solver {
            grid,
            rocks,
            starting_location,
            next_state_cache: HashMap::new(),
            coords_cache: HashMap::new(),
            outputs_cache: HashMap::new(),
            active_tiles: HashMap::new(),
            looping_tiles: HashSet::new(),
            tile_histories: HashMap::new(),
        };

        solver.run(steps)
    }

    fn run(&mut self, steps: usize) -> usize {
        // insert empty coords
        self.coords_cache.insert(*EMPTY_STATE, EMPTY_COORDS);

        // insert initial state
        let mut initial_coords: BTreeSet<Coord> = BTreeSet::new();
        initial_coords.insert(self.starting_location);
        let initial_state = calculate_hash(&initial_coords);
        self.coords_cache.insert(initial_state, initial_coords);
        self.active_tiles.insert(Coord::new(0, 0), initial_state);

        let mut step = 0;
        while step < steps {
            step += 1;
            self.step(step);
        }

        let active_sum: usize = self
            .active_tiles
            .values()
            .map(|state| self.coords_cache.get(state).unwrap().len())
            .sum();

        let looping_sum: usize = self
            .looping_tiles
            .iter()
            .map(|tile_pos| {
                let (started_step, history) = self.tile_histories.get(tile_pos).unwrap();
                let missing = step - (started_step + history.len() - 1);

                // if missing history is even, final elem == last elem, second-last elem for odd
                let index_from_end = if missing % 2 == 0 { 1 } else { 2 };

                // coords len for state
                let (_input_states, state) = history[history.len() - index_from_end];
                self.coords_cache.get(&state).unwrap().len()
            })
            .sum();

        active_sum + looping_sum
    }

    fn step(&mut self, step: usize) {
        // explore active tiles, plus direct neigbours
        let mut to_explore: HashSet<Coord> = HashSet::new();
        for tile in self.active_tiles.keys() {
            to_explore.insert(*tile);

            for neighbour in tile.neighbours() {
                // ignore tiles that are already looping
                if !self.looping_tiles.contains(&neighbour) {
                    to_explore.insert(neighbour);
                }
            }
        }

        let mut next_active_tiles: HashMap<Coord, u64> = HashMap::new();

        for tile_pos in to_explore {
            let tile_state = *self.active_tiles.get(&tile_pos).unwrap_or(&EMPTY_STATE);

            let mut neighbour_tile_states: [u64; 4] = [0; 4];
            for direction in ALL_DIRECTIONS {
                let neighbour_tile_pos = tile_pos.shift(&direction);
                neighbour_tile_states[direction.index()] = *self
                    .active_tiles
                    .get(&neighbour_tile_pos)
                    .unwrap_or(&EMPTY_STATE);
            }

            let next_tile_state = self.next_state(tile_state, neighbour_tile_states);
            if next_tile_state != *EMPTY_STATE {
                self.tile_histories
                    .entry(tile_pos)
                    .or_insert((step, vec![]))
                    .1
                    .push(((tile_state, neighbour_tile_states), next_tile_state));

                if self.tile_is_looping(&tile_pos) {
                    self.looping_tiles.insert(tile_pos);
                } else {
                    next_active_tiles.insert(tile_pos, next_tile_state);
                }
            }
        }

        self.active_tiles = next_active_tiles;
    }

    fn next_state(&mut self, curr_state: u64, neighbour_states: [u64; 4]) -> u64 {
        let key = (curr_state, neighbour_states);
        match self.next_state_cache.get(&key) {
            Some(val) => *val,
            None => {
                let val = self.calculate_next_state(curr_state, neighbour_states);
                self.next_state_cache.insert(key, val);
                val
            }
        }
    }

    fn calculate_next_state(&mut self, curr_state: u64, neighbour_states: [u64; 4]) -> u64 {
        let (own_outputs, _) = self.outputs(curr_state);
        let mut coords = own_outputs.clone();

        for direction in ALL_DIRECTIONS {
            let outputs = self.neighbour_outputs(neighbour_states, direction);
            coords.append(&mut outputs.clone());
        }

        // cache result in coords
        let state = calculate_hash(&coords);
        self.coords_cache.insert(state, coords);
        state
    }

    fn outputs(&mut self, curr_state: u64) -> &(BTreeSet<Coord>, [BTreeSet<Coord>; 4]) {
        if !self.outputs_cache.contains_key(&curr_state) {
            let val = self.calculate_outputs(curr_state);
            self.outputs_cache.insert(curr_state, val);
        }
        self.outputs_cache.get(&curr_state).unwrap()
    }

    fn neighbour_outputs(
        &mut self,
        neighbour_states: [u64; 4],
        direction: Direction,
    ) -> &BTreeSet<Coord> {
        let state = neighbour_states[direction.index()];
        let (_, in_directions) = self.outputs(state);
        &in_directions[direction.rev().index()]
    }

    fn calculate_outputs(&mut self, curr_state: u64) -> (BTreeSet<Coord>, [BTreeSet<Coord>; 4]) {
        let expand_coords = self.coords_cache.get(&curr_state).unwrap();

        let mut own_outputs = BTreeSet::new();
        let mut neighbour_outputs: [BTreeSet<Coord>; 4] = Default::default();

        for expand_pos in expand_coords {
            for (neighbour_pos, neighbour_tile_direction) in self.grid.neighbours(expand_pos) {
                if !self.rocks.contains(&neighbour_pos) {
                    match neighbour_tile_direction {
                        Some(d) => neighbour_outputs[d.index()].insert(neighbour_pos),
                        None => own_outputs.insert(neighbour_pos),
                    };
                }
            }
        }

        (own_outputs, neighbour_outputs)
    }

    fn tile_is_looping(&self, tile_pos: &Coord) -> bool {
        let (_, history) = self.tile_histories.get(tile_pos).unwrap();
        let l = history.len();

        // last 4 entries are a repeated pair
        l >= 20 && &history[l - 3] == &history[l - 1] && &history[l - 4] == &history[l - 2]
    }
}

#[derive(Debug)]
struct Map {
    grid: Grid<Terrain>,
}

impl Map {
    fn parse(input: &str) -> Self {
        let grid = Grid::parse(input, |c| Terrain::parse(c));
        Self { grid }
    }

    fn count_plots_within_reach(&self, steps: usize) -> usize {
        Solver::solve(&self.grid, steps)
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Terrain {
    Garden,
    Rock,
    Start,
}

impl Terrain {
    fn parse(c: &char) -> Self {
        match c {
            '.' => Terrain::Garden,
            '#' => Terrain::Rock,
            'S' => Terrain::Start,
            _ => panic!("Unexpected terrain: {:?}", c),
        }
    }
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

fn part1(input: &str, steps: usize) -> usize {
    let map = Map::parse(input);
    map.count_plots_within_reach(steps)
}

fn part2(input: &str, steps: usize) -> usize {
    let map = Map::parse(input);
    map.count_plots_within_reach(steps)
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    static EXAMPLE: &str = indoc! {"
        ...........
        .....###.#.
        .###.##..#.
        ..#.#...#..
        ....#.#....
        .##..S####.
        .##..#...#.
        .......##..
        .##.#.####.
        .##..##.##.
        ...........
    "};

    #[test]
    fn test_part1_example() {
        let result = part1(EXAMPLE, 6);
        assert_eq!(result, 16);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file(), 64);
        assert_eq!(result, 3768);
    }

    #[test]
    fn test_part2_example() {
        // assert_eq!(part2(EXAMPLE, 6), 16);
        // assert_eq!(part2(EXAMPLE, 10), 50);
        // assert_eq!(part2(EXAMPLE, 50), 1594);
        // assert_eq!(part2(EXAMPLE, 100), 6536);
        // assert_eq!(part2(EXAMPLE, 500), 167004);
        assert_eq!(part2(EXAMPLE, 1000), 668697);
        // assert_eq!(part2(EXAMPLE, 5000), 16733044);
    }

    // #[test]
    // fn test_part2_solution() {
    //     let result = part2(&read_input_file(), 300);
    //     assert_eq!(result, 0);
    // }
}
