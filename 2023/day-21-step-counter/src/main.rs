pub mod grid;

use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
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
    coords_cache: CoordsCache,
    memory: TileMemory,
    next_state_calculator: NextStateCalculator,
}

impl<'a> Solver<'a> {
    fn solve(grid: &'a Grid<Terrain>, steps: usize, exploration_distance: usize) -> usize {
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
            coords_cache: CoordsCache::new(),
            memory: TileMemory::new(),
            next_state_calculator: NextStateCalculator::new(),
        };

        solver.run(steps, exploration_distance)
    }

    fn run(&mut self, steps: usize, exploration_distance: usize) -> usize {
        // set up initial state
        let initial_state = TileState::from_coords(
            BTreeSet::from([self.starting_location]),
            &mut self.coords_cache,
            &self.grid,
        );
        self.memory
            .active_tiles
            .insert(Coord::new(0, 0), initial_state);

        // run simulation
        let mut step = 0;
        while step < steps
            && !self
                .memory
                .captured_enough(&self.starting_location, exploration_distance)
        {
            step = self.step(step);
        }

        self.memory
            .final_plot_count(step, steps, &self.coords_cache)
    }

    fn step(&mut self, curr_step: usize) -> usize {
        let next_step = curr_step + 1;

        // get set of tiles to calculate next states for, and build inputs for them
        let to_explore = self.memory.to_explore(curr_step);
        let input_states: Vec<(Coord, &TileState, TileInputState)> = to_explore
            .into_iter()
            .map(|(pos, state)| {
                (
                    pos,
                    state,
                    TileInputState::build(&pos, curr_step, &self.memory),
                )
            })
            .collect();

        // calculate the next states
        let next_state_results = self.next_state_calculator.next_states(
            input_states,
            &mut self.coords_cache,
            &self.grid,
            &self.rocks,
        );

        // update memory
        self.memory
            .process_next_states(next_step, next_state_results);

        next_step
    }
}

struct TileMemory {
    active_tiles: HashMap<Coord, TileState>,
    looping_tiles: HashSet<Coord>,
    histories: HashMap<Coord, (usize, Vec<(TileInputState, TileState)>)>,
}

impl TileMemory {
    fn new() -> Self {
        Self {
            active_tiles: HashMap::new(),
            looping_tiles: HashSet::new(),
            histories: HashMap::new(),
        }
    }

    fn get(&self, tile_pos: &Coord, target_step: usize) -> &TileState {
        if self.active_tiles.contains_key(tile_pos) {
            self.active_tiles.get(tile_pos).unwrap()
        } else if self.looping_tiles.contains(tile_pos) {
            let (started_step, history) = self.histories.get(tile_pos).unwrap();

            let hstep = target_step - started_step; // steps into history
            Self::looped_state_at_step(history, hstep)
        } else {
            &EMPTY_TILE_STATE
        }
    }

    fn to_explore(&self, step: usize) -> HashMap<Coord, &TileState> {
        // start with current active set
        let mut res: HashMap<Coord, &TileState> = self
            .active_tiles
            .iter()
            .map(|(pos, state)| (*pos, state))
            .collect();

        // add neighbours
        for (pos, state) in self.active_tiles.iter() {
            for direction in ALL_DIRECTIONS {
                let neighbour = pos.shift(&direction);

                // ignore tiles that are already looping
                // also ignore tiles where our edge facing them is empty
                if !res.contains_key(&neighbour)
                    && !self.looping_tiles.contains(&neighbour)
                    && state.edges[direction.index()] != *EMPTY_STATE
                {
                    res.insert(neighbour, self.get(&neighbour, step));
                }
            }
        }

        res
    }

    fn process_next_states(
        &mut self,
        step: usize,
        next_states: Vec<(Coord, TileInputState, TileState)>,
    ) {
        let mut next_active_tiles: HashMap<Coord, TileState> = HashMap::new();

        for (pos, input, next_state) in next_states {
            if next_state != *EMPTY_TILE_STATE {
                self.histories
                    .entry(pos)
                    .or_insert((step, vec![]))
                    .1
                    .push((input, next_state));

                if self.tile_is_looping(&pos) {
                    self.looping_tiles.insert(pos);
                } else {
                    next_active_tiles.insert(pos, next_state);
                }
            }
        }

        self.active_tiles = next_active_tiles;
    }

    fn tile_is_looping(&self, tile_pos: &Coord) -> bool {
        let (_, history) = self.histories.get(tile_pos).unwrap();
        let l = history.len();

        // last 4 entries are a repeated pair
        l >= 20 && &history[l - 3] == &history[l - 1] && &history[l - 4] == &history[l - 2]
    }

    fn captured_enough(&self, origin: &Coord, exploration_distance: usize) -> bool {
        // have we found enough looping tiles to infer remaining expansion?
        let have = self
            .looping_tiles
            .iter()
            .filter(|pos| origin.manhattan_distance(pos) <= exploration_distance)
            .count();

        // want all tiles within manhattan distance
        let diameter = exploration_distance * 2 + 1;
        let area = ((diameter * diameter) as f64 / 2.0).ceil() as usize;

        have >= area
    }

    fn final_plot_count(&self, step: usize, steps: usize, coords_cache: &CoordsCache) -> usize {
        if step == steps {
            self.simple_final_plot_count(steps, coords_cache)
        } else {
            self.infer_final_plot_count(steps, coords_cache)
        }
    }

    fn simple_final_plot_count(&self, steps: usize, coords_cache: &CoordsCache) -> usize {
        // start with active tiles
        let mut tiles: Vec<(&Coord, &TileState)> = self.active_tiles.iter().collect();

        // add looping tiles
        for pos in &self.looping_tiles {
            let state = self.get(pos, steps);
            tiles.push((pos, state));
        }

        // sum counts
        tiles
            .into_iter()
            .map(|(_pos, state)| coords_cache.for_state(state).len())
            .sum()
    }

    fn looped_state_at_step(history: &Vec<(TileInputState, TileState)>, step: usize) -> &TileState {
        // let's say history is len 3:
        // step 0 => 0
        // step 1 => 1
        // step 2 => 2
        // step 3 => 1 [1 past end]
        // step 4 => 2 [2 past end]
        let hlen = history.len();
        let last_index = hlen - 1;

        let index = if step <= last_index {
            // special case, we are still in the history range
            step
        } else {
            // if overshoot is even, final elem == last elem, second-last elem for odd
            let overshoot = step - last_index;
            let index_from_end = if overshoot % 2 == 0 { 1 } else { 2 };
            hlen - index_from_end
        };

        let (_input, state) = &history[index];
        state
    }

    fn infer_final_plot_count(&self, steps: usize, coords_cache: &CoordsCache) -> usize {
        // group looping histories by common history
        let mut groups: HashMap<&Vec<(TileInputState, TileState)>, Vec<(&Coord, &usize)>> =
            HashMap::new();
        for pos in &self.looping_tiles {
            let (started, hist) = self.histories.get(pos).unwrap();
            groups.entry(hist).or_default().push((pos, started));
        }

        // should have exactly 8 groups with more than 1 element: N/S/E/W lines, NW/NE/SW/SE quadrants
        let num_larger_than_1 = groups
            .iter()
            .filter(|(_hist, tiles)| tiles.len() > 1)
            .count();
        assert_eq!(num_larger_than_1, 8);

        let mut frequency: HashMap<TileState, usize> = HashMap::new();
        for (hist, tiles) in groups {
            self.add_history_frequency_for_group(hist, tiles.clone(), steps, &mut frequency);
        }

        frequency
            .into_iter()
            .map(|(state, num)| {
                let len = coords_cache.for_state(&state).len();
                len * num
            })
            .sum()
    }

    fn add_history_frequency_for_group(
        &self,
        history: &Vec<(TileInputState, TileState)>,
        tiles: Vec<(&Coord, &usize)>,
        steps: usize,
        frequency: &mut HashMap<TileState, usize>,
    ) {
        if tiles.len() == 1 {
            // special case - not a growing pattern, just need the simple count
            let (_pos, started_step) = tiles[0];
            let hstep = steps - started_step;
            let state = Self::looped_state_at_step(history, hstep);
            *frequency.entry(*state).or_default() += 1;
            return;
        }

        // ensure all tiles in this group are in the same quadrant/line
        let signs: HashSet<(i16, i16)> = tiles
            .iter()
            .map(|(pos, _)| (pos.x.signum(), pos.y.signum()))
            .collect();
        assert_eq!(signs.len(), 1);

        // group tiles by starting step
        let mut by_step: BTreeMap<&usize, Vec<&Coord>> = BTreeMap::new();
        for (pos, start) in &tiles {
            by_step.entry(start).or_default().push(pos);
        }

        // record sequence start
        let pattern_start = **by_step.first_key_value().unwrap().0;

        // calculate diff of starting step and tile count between each pair of starting step groups
        let uniq_diffs: HashSet<(usize, usize)> = by_step
            .into_iter()
            .collect::<Vec<(&usize, Vec<&Coord>)>>()
            .windows(2)
            .map(|w| {
                let (start0, tiles0) = &w[0];
                let (start1, tiles1) = &w[1];
                (*start1 - *start0, tiles1.len() - tiles0.len())
            })
            .collect();

        // ensure there's a single, consistent pattern in the history sequence
        assert_eq!(uniq_diffs.len(), 1);
        let (pattern_interval, pattern_growth) = uniq_diffs.into_iter().next().unwrap();

        // this is how much time we have to fill in total
        let pattern_steps = steps - pattern_start;

        // we can safely fill this much with looping tiles, to give enough time to settle
        let looping_steps = pattern_steps - history.len();
        let loop_count = looping_steps / pattern_interval + 1;

        // this part is what's left after the looping tiles
        let remaining_steps = pattern_steps - loop_count * pattern_interval;

        // handle main part, the looping tiles
        // alternating pattern of two groups, since pattern_interval can be odd and the looping
        // tiles have two states.
        let second_group_loop_count = loop_count / 2;
        let first_group_loop_count = loop_count - second_group_loop_count;

        let (first_group_tile_count, second_group_tile_count) = match pattern_growth {
            0 => {
                // linear pattern, N/E/S/W
                // same count as number of loops
                (first_group_loop_count, second_group_loop_count)
            }
            1 => {
                // quadrant pattern, NE/SE/SW/NW
                // grows by size 1 at each iteration

                // determine sum of increasing sequences
                let first = first_group_loop_count * first_group_loop_count;
                let second = second_group_loop_count * (second_group_loop_count + 1);

                // double check the math
                let looping_tile_count = loop_count * (loop_count + 1) / 2;
                assert_eq!(first + second, looping_tile_count);

                (first, second)
            }
            _ => panic!("Unexpected pattern growth: {}", pattern_growth),
        };

        // first group starts at local [0] / global [pattern_start]
        // second group starts at local [interval] / global [pattern_start + interval]
        let first_group_steps = pattern_steps;
        let second_group_steps = pattern_steps - pattern_interval;

        // get the final state for each group
        let first_group_state = Self::looped_state_at_step(history, first_group_steps);
        let second_group_state = Self::looped_state_at_step(history, second_group_steps);

        // add the looping tile groups to the frequency map
        *frequency.entry(*first_group_state).or_default() += first_group_tile_count;
        *frequency.entry(*second_group_state).or_default() += second_group_tile_count;

        // handle remainder
        for i in 0..=(remaining_steps / pattern_interval) {
            let rem_steps = remaining_steps - (i * pattern_interval);

            let tile_count = match pattern_growth {
                0 => 1,
                1 => loop_count + 1 + i,
                _ => panic!("Unexpected pattern growth: {}", pattern_growth),
            };

            let state = Self::looped_state_at_step(history, rem_steps);
            *frequency.entry(*state).or_default() += tile_count;
        }
    }
}

struct CoordsCache {
    cache: HashMap<u64, BTreeSet<Coord>>,
}

impl CoordsCache {
    fn new() -> Self {
        let mut cache = Self {
            cache: HashMap::new(),
        };

        // initialize with empty set
        cache.insert(BTreeSet::new());

        cache
    }

    fn key(coords: &BTreeSet<Coord>) -> u64 {
        calculate_hash(coords)
    }

    fn insert(&mut self, coords: BTreeSet<Coord>) -> u64 {
        let key = Self::key(&coords);
        self.cache.insert(key, coords);
        key
    }

    fn get(&self, key: &u64) -> &BTreeSet<Coord> {
        self.cache.get(key).unwrap()
    }

    fn for_state(&self, state: &TileState) -> &BTreeSet<Coord> {
        self.get(&state.whole)
    }
}

struct NextStateCalculator {
    next_state_cache: HashMap<TileInputState, TileState>,
    outputs_cache: TileOutputsCache,
}

impl NextStateCalculator {
    fn new() -> Self {
        Self {
            next_state_cache: HashMap::new(),
            outputs_cache: TileOutputsCache::new(),
        }
    }

    fn next_states(
        &mut self,
        to_explore: Vec<(Coord, &TileState, TileInputState)>,
        coords_cache: &mut CoordsCache,
        grid: &Grid<Terrain>,
        rocks: &HashSet<Coord>,
    ) -> Vec<(Coord, TileInputState, TileState)> {
        // first, build the outputs for each tile, as we'll need those in place to calculate the next states
        for (_pos, curr_state, input) in &to_explore {
            self.outputs_cache
                .calculate(curr_state, input, coords_cache, grid, rocks);
        }

        // then, calculate the next states
        to_explore
            .into_iter()
            .map(|(pos, _, input)| (pos, input, self.next_state(&input, coords_cache, grid)))
            .collect()
    }

    fn next_state(
        &mut self,
        input: &TileInputState,
        coords_cache: &mut CoordsCache,
        grid: &Grid<Terrain>,
    ) -> TileState {
        match self.next_state_cache.get(input) {
            Some(val) => *val,
            None => {
                let val = self._calculate_next_state(input, coords_cache, grid);
                self.next_state_cache.insert(*input, val);
                val
            }
        }
    }

    fn _calculate_next_state(
        &self,
        input: &TileInputState,
        coords_cache: &mut CoordsCache,
        grid: &Grid<Terrain>,
    ) -> TileState {
        // get coords for main tile
        let own_output = self.outputs_cache.get_own_output(input);
        let own_coords = coords_cache.get(own_output);

        // build set of next state coords
        let mut next_coords = own_coords.clone();

        // get coords for neighbouring edges
        for direction in ALL_DIRECTIONS {
            let edge_output = self
                .outputs_cache
                .get_output_for_neighbouring_edge(input, &direction);
            let mut edge_coords = coords_cache.get(edge_output).clone();
            next_coords.append(&mut edge_coords);
        }

        // translate into a tile state (will cache in coords_cache)
        TileState::from_coords(next_coords, coords_cache, grid)
    }
}

struct TileOutputsCache {
    own_cache: HashMap<u64, u64>,
    edge_cache: HashMap<(u64, Direction), u64>,
}

impl TileOutputsCache {
    fn new() -> Self {
        Self {
            own_cache: HashMap::new(),
            edge_cache: HashMap::new(),
        }
    }

    fn get_own_output(&self, input: &TileInputState) -> &u64 {
        self.own_cache.get(&input.own).unwrap_or(&EMPTY_STATE)
    }

    fn get_output_for_neighbouring_edge(
        &self,
        input: &TileInputState,
        direction: &Direction,
    ) -> &u64 {
        // for eg our neighbour to the North, we want their South facing edge
        let edge = input.neighbouring_edges[direction.index()];
        let key = (edge, direction.rev());
        self.edge_cache.get(&key).unwrap_or(&EMPTY_STATE)
    }

    fn calculate(
        &mut self,
        state: &TileState,
        input: &TileInputState,
        coords_cache: &mut CoordsCache,
        grid: &Grid<Terrain>,
        rocks: &HashSet<Coord>,
    ) {
        let key = &input.own;
        if !self.own_cache.contains_key(key) {
            self._calculate(state, input, coords_cache, grid, rocks);
        }
    }

    fn _calculate(
        &mut self,
        state: &TileState,
        input: &TileInputState,
        coords_cache: &mut CoordsCache,
        grid: &Grid<Terrain>,
        rocks: &HashSet<Coord>,
    ) {
        // build outputs, both on our own tile, and bleeding into adjacent tiles
        let coords = coords_cache.get(&input.own);
        let mut own_outputs = BTreeSet::new();
        let mut adjacent_outputs: [BTreeSet<Coord>; 4] = Default::default();
        for pos in coords {
            for (neighbour_pos, neighbour_tile_direction) in grid.neighbours(pos) {
                if !rocks.contains(&neighbour_pos) {
                    match neighbour_tile_direction {
                        Some(d) => adjacent_outputs[d.index()].insert(neighbour_pos),
                        None => own_outputs.insert(neighbour_pos),
                    };
                }
            }
        }

        // store own tile outputs in cache
        let own = coords_cache.insert(own_outputs);
        self.own_cache.insert(input.own, own);

        // store adjacent outputs in cache
        let mut adjacent: [u64; 4] = [0; 4];
        for (i, outputs) in adjacent_outputs.into_iter().enumerate() {
            let direction = Direction::from_index(i);

            // add the new adjacent outputs to the coords cache
            let hash = coords_cache.insert(outputs);
            adjacent[i] = hash;

            // add the adjacent outputs hash value to the edge cache
            self.edge_cache.insert((state.edges[i], direction), hash);
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct TileState {
    whole: u64,
    edges: [u64; 4],
}

lazy_static! {
    static ref EMPTY_TILE_STATE: TileState = {
        TileState {
            whole: *EMPTY_STATE,
            edges: [*EMPTY_STATE; 4],
        }
    };
}

impl TileState {
    fn from_coords(
        coords: BTreeSet<Coord>,
        coords_cache: &mut CoordsCache,
        grid: &Grid<Terrain>,
    ) -> Self {
        let mut edges: [u64; 4] = [0; 4];
        for direction in ALL_DIRECTIONS {
            let edge_coords: BTreeSet<Coord> = coords
                .iter()
                .filter(|c| match direction {
                    Direction::North => c.y == 0,
                    Direction::West => c.x == 0,
                    Direction::South => c.y == grid.height - 1,
                    Direction::East => c.x == grid.width - 1,
                })
                .cloned()
                .collect();

            let edge = coords_cache.insert(edge_coords);
            edges[direction.index()] = edge;
        }

        let whole = coords_cache.insert(coords);

        Self { whole, edges }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct TileInputState {
    own: u64,
    neighbouring_edges: [u64; 4],
}

impl TileInputState {
    fn build(tile_pos: &Coord, step: usize, memory: &TileMemory) -> Self {
        let own = memory.get(tile_pos, step).whole;

        let mut neighbouring_edges: [u64; 4] = [0; 4];
        for direction in ALL_DIRECTIONS {
            let neighbour_tile_pos = tile_pos.shift(&direction);
            let i = direction.index();

            // get state at prev step (matters for looping tile calculation)
            neighbouring_edges[i] =
                memory.get(&neighbour_tile_pos, step).edges[direction.rev().index()];
        }

        TileInputState {
            own,
            neighbouring_edges,
        }
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
        Solver::solve(&self.grid, steps, 4)
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
    part1(input, steps)
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
        assert_eq!(part2(EXAMPLE, 6), 16);
        assert_eq!(part2(EXAMPLE, 10), 50);
        assert_eq!(part2(EXAMPLE, 50), 1594);
        assert_eq!(part2(EXAMPLE, 100), 6536);
        assert_eq!(part2(EXAMPLE, 500), 167004);
        assert_eq!(part2(EXAMPLE, 1000), 668697);
        assert_eq!(part2(EXAMPLE, 5000), 16733044);
    }

    #[test]
    fn test_part2_solution() {
        let result = part2(&read_input_file(), 26501365);
        assert_eq!(result, 627960775905777);
    }
}
