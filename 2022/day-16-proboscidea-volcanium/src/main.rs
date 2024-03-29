pub mod astar;

use astar::AStarInterface;

use std::cmp;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::fmt;
use std::fs;

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    println!("part 2 result: {:?}", part2(&read_input_file()));
}

fn read_input_file() -> String {
    fs::read_to_string("input.txt").expect("Something went wrong reading the file")
}

#[derive(Debug)]
struct Reading {
    valve: String,
    flow_rate: u8,
    connections: Vec<String>,
}

impl Reading {
    fn parse_list(input: &str) -> Vec<Self> {
        input.lines().map(|line| Self::parse(line)).collect()
    }

    fn parse(input: &str) -> Self {
        lazy_static! {
            static ref READING_RE: Regex = Regex::new(
                r"\AValve ([A-Z]{2}) has flow rate=(\d+); tunnels? leads? to valves? ([A-Z, ]+)\z"
            )
            .unwrap();
        }

        let caps = READING_RE.captures(input).unwrap();
        let valve = caps.get(1).unwrap().as_str().to_string();
        let flow_rate = caps.get(2).unwrap().as_str().parse::<u8>().unwrap();
        let connections = caps
            .get(3)
            .unwrap()
            .as_str()
            .split(", ")
            .map(|s| s.to_string())
            .collect();

        Self {
            valve,
            flow_rate,
            connections,
        }
    }
}

#[derive(Debug)]
struct ScanMetadata {
    valve_numbers: HashMap<String, u8>,
    flow_rates: HashMap<u8, u8>,
    connections: HashMap<u8, Vec<u8>>,
    non_zero_valves: BTreeSet<u8>,
    shortest_paths: HashMap<u8, HashMap<u8, u8>>,
}

impl ScanMetadata {
    fn new(readings: &[Reading]) -> Self {
        let valve_numbers: HashMap<String, u8> = readings
            .iter()
            .map(|r| r.valve.clone())
            .sorted()
            .enumerate()
            .map(|(i, s)| (s, i as u8))
            .collect();

        let flow_rates: HashMap<u8, u8> = readings
            .iter()
            .map(|r| (*valve_numbers.get(&r.valve).unwrap(), r.flow_rate))
            .collect();
        let connections = readings
            .iter()
            .map(|r| {
                (
                    *valve_numbers.get(&r.valve).unwrap(),
                    r.connections
                        .iter()
                        .map(|c| *valve_numbers.get(c).unwrap())
                        .collect(),
                )
            })
            .collect::<HashMap<u8, Vec<u8>>>();
        let non_zero_valves: BTreeSet<u8> = flow_rates
            .iter()
            .filter(|(_k, v)| **v > 0)
            .map(|(k, _v)| k.clone())
            .collect();

        let mut shortest_paths: HashMap<u8, HashMap<u8, u8>> = HashMap::new();
        for from in flow_rates.keys() {
            for to in non_zero_valves.iter() {
                let distance = shortest_path(from, to, &connections);
                shortest_paths
                    .entry(from.clone())
                    .or_default()
                    .insert(to.clone(), distance);
            }
        }

        Self {
            valve_numbers,
            flow_rates,
            connections,
            non_zero_valves,
            shortest_paths,
        }
    }

    fn get_flow_rate(&self, valve: &u8) -> &u8 {
        self.flow_rates.get(valve).unwrap()
    }

    fn get_connections(&self, valve: &u8) -> &Vec<u8> {
        self.connections.get(valve).unwrap()
    }

    fn get_shortest_path(&self, from: &u8, to: &u8) -> &u8 {
        self.shortest_paths.get(from).unwrap().get(to).unwrap()
    }
}

struct Pathfinder<'a> {
    to: &'a u8,
    connections: &'a HashMap<u8, Vec<u8>>,
}

fn shortest_path(from: &u8, to: &u8, connections: &HashMap<u8, Vec<u8>>) -> u8 {
    let state = Pathfinder { to, connections };
    let (_path, length) = state.shortest_path(from, false).unwrap();
    length as u8
}

impl AStarInterface<u8> for Pathfinder<'_> {
    fn at_goal(&self, node: &u8) -> bool {
        node == self.to
    }

    fn heuristic(&self, _from: &u8) -> isize {
        1
    }

    fn neighbours(&self, from: &u8) -> Vec<(u8, isize)> {
        self.connections
            .get(from)
            .unwrap()
            .iter()
            .map(|n| (n.clone(), 1))
            .collect()
    }
}

struct Solver {
    scan_metadata: ScanMetadata,
    open_set: Vec<SolutionState>,
    best_states: BestSolutionStates,
    best_result: Option<SolutionState>,
}

impl Solver {
    fn new(
        readings: &[Reading],
        starting_location_name: String,
        mins: u8,
        elephant_enabled: bool,
    ) -> Self {
        let scan_metadata = ScanMetadata::new(readings);
        let starting_location = *scan_metadata
            .valve_numbers
            .get(&starting_location_name)
            .unwrap();
        let initial = SolutionState::new(starting_location, mins, elephant_enabled);

        let mut best_states = BestSolutionStates::new();
        best_states.record(&initial);

        let open_set = vec![initial];
        let best_result = None;

        Self {
            scan_metadata,
            open_set,
            best_states,
            best_result,
        }
    }

    fn run(
        readings: &[Reading],
        starting_location_name: String,
        mins: u8,
        elephant_enabled: bool,
    ) -> u32 {
        let mut solver = Self::new(readings, starting_location_name, mins, elephant_enabled);

        loop {
            let res = solver.tick();
            if !res {
                break;
            }
        }

        solver.best_result.unwrap().final_pressure_released
    }

    fn tick(&mut self) -> bool {
        let from_state = match self.open_set.pop() {
            Some(s) => s,
            None => return false,
        };

        if !self.best_states.best_states_either(&from_state) {
            println!(
                "  skipping {}, it's no longer the best option for the location & opened valves",
                &from_state
            );
            return true;
        } else if !self.possibly_beats_best_result(&from_state) {
            println!(
                "  skipping {}, it can't beat the current best result",
                &from_state
            );
            return true;
        }

        println!("  expanding {}", &from_state);

        let next_states = from_state.next_states(&self.scan_metadata);
        for next_state in next_states {
            if next_state.is_complete(&self.scan_metadata) {
                println!("    completed {}", &next_state);
                self.handle_complete(next_state);
            } else {
                self.add_state_to_open_set_if_wanted(next_state);
            }
        }

        true
    }

    fn handle_complete(&mut self, state: SolutionState) {
        if self.best_result.is_none()
            || self.best_result.as_ref().unwrap().final_pressure_released
                < state.final_pressure_released
        {
            self.best_result = Some(state);
        }
    }

    fn add_state_to_open_set_if_wanted(&mut self, next_state: SolutionState) {
        use RecordScoreResult::*;
        let best = self.best_states.record(&next_state);
        println!("    best: {:?}", best);
        match best {
            (Best, _) | (_, Best) | (Missing, Missing) => {
                if self.possibly_beats_best_result(&next_state) {
                    println!("    adding {} to open set", &next_state);
                    self.open_set.push(next_state);
                } else {
                    println!(
                        "    ignoring {} as it doesn't beat the current best result: {}",
                        &next_state,
                        self.best_result.as_ref().unwrap()
                    );
                }
            }
            (NotBest, _) | (_, NotBest) => {
                println!("    ignoring {} as it's not the best", &next_state);
            }
        };
    }

    fn possibly_beats_best_result(&self, state: &SolutionState) -> bool {
        match &self.best_result {
            Some(curr_best) => {
                let pressure_heuristic = state.final_pressure_heuristic(&self.scan_metadata);
                pressure_heuristic > curr_best.final_pressure_released
            }
            None => true,
        }
    }
}

struct BestSolutionStates {
    map: HashMap<(u8, u8, BTreeSet<u8>), (HashMap<u8, u32>, HashMap<u32, u8>)>,
}

impl BestSolutionStates {
    fn new() -> Self {
        let map = HashMap::new();
        Self { map }
    }

    fn key(state: &SolutionState) -> (u8, u8, BTreeSet<u8>) {
        // treat self/elephant as interchangeable for determining the best solution
        if &state.my_location <= &state.elephant_location {
            (
                state.my_location.clone(),
                state.elephant_location.clone(),
                state.open_valves.clone(),
            )
        } else {
            (
                state.elephant_location.clone(),
                state.my_location.clone(),
                state.open_valves.clone(),
            )
        }
    }

    fn best_states_either(&self, state: &SolutionState) -> bool {
        let key = Self::key(state);
        let (pressure_scores, time_scores) = self.map.get(&key).unwrap();
        let best_pressure =
            state.final_pressure_released >= *pressure_scores.get(&state.mins_remaining).unwrap();
        let best_time =
            state.mins_remaining >= *time_scores.get(&state.final_pressure_released).unwrap();
        best_pressure || best_time
    }

    fn record(&mut self, state: &SolutionState) -> (RecordScoreResult, RecordScoreResult) {
        let key = Self::key(state);
        let (pressure_scores, time_scores) = self
            .map
            .entry(key)
            .or_insert_with(|| (HashMap::new(), HashMap::new()));
        let best_time = Self::record_score(
            time_scores,
            state.final_pressure_released,
            state.mins_remaining,
        );
        let best_pressure = Self::record_score(
            pressure_scores,
            state.mins_remaining,
            state.final_pressure_released,
        );
        (best_time, best_pressure)
    }

    fn record_score<S: std::hash::Hash + Eq, T: std::cmp::PartialOrd>(
        scores: &mut HashMap<S, T>,
        key: S,
        new_score: T,
    ) -> RecordScoreResult {
        use RecordScoreResult::*;
        match scores.get_mut(&key) {
            Some(curr_score) => {
                if new_score > *curr_score {
                    *curr_score = new_score;
                    Best
                } else {
                    NotBest
                }
            }
            None => {
                scores.insert(key, new_score);
                Missing
            }
        }
    }
}

#[derive(Debug)]
enum RecordScoreResult {
    Best,
    NotBest,
    Missing,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct SolutionState {
    elephant_enabled: bool,
    my_location: u8,
    elephant_location: u8,
    mins_remaining: u8,
    open_valves: BTreeSet<u8>,
    final_pressure_released: u32,
}

impl SolutionState {
    fn new(starting_location: u8, mins_remaining: u8, elephant_enabled: bool) -> Self {
        let my_location = starting_location.clone();
        let elephant_location = starting_location.clone();
        let open_valves = BTreeSet::new();
        let final_pressure_released = 0;
        Self {
            elephant_enabled,
            my_location,
            elephant_location,
            mins_remaining,
            open_valves,
            final_pressure_released,
        }
    }

    fn is_complete(&self, scan_metadata: &ScanMetadata) -> bool {
        self.mins_remaining == 0 || self.open_valves == scan_metadata.non_zero_valves
    }

    fn next_states(&self, scan_metadata: &ScanMetadata) -> Vec<Self> {
        if self.elephant_enabled {
            self.calculate_next_states(false, scan_metadata)
                .into_iter()
                .flat_map(|my_next_state| my_next_state.calculate_next_states(true, scan_metadata))
                .collect()
        } else {
            self.calculate_next_states(false, scan_metadata)
        }
    }

    fn calculate_next_states(&self, for_elephant: bool, scan_metadata: &ScanMetadata) -> Vec<Self> {
        let current_location = if for_elephant {
            &self.elephant_location
        } else {
            &self.my_location
        };
        let flow_rate = scan_metadata.get_flow_rate(current_location);
        let destinations = scan_metadata.get_connections(current_location);

        let mut states: Vec<Self> = destinations
            .iter()
            .map(|d| self.move_to(d, for_elephant))
            .collect();

        // add an option to open the valve in the current room, if >0 and not open
        if flow_rate > &0 && !self.is_valve_open(current_location) {
            states.push(self.open_valve(current_location, flow_rate, for_elephant));
        }

        states
    }

    fn is_valve_open(&self, valve: &u8) -> bool {
        self.open_valves.contains(valve)
    }

    fn move_to(&self, valve: &u8, for_elephant: bool) -> Self {
        let mut new_state = self.clone();

        if !for_elephant {
            new_state.mins_remaining -= 1;
        }

        if for_elephant {
            new_state.elephant_location = valve.clone();
        } else {
            new_state.my_location = valve.clone();
        }

        new_state
    }

    fn open_valve(&self, valve: &u8, flow_rate: &u8, for_elephant: bool) -> Self {
        let mut new_state = self.clone();

        if !for_elephant {
            new_state.mins_remaining -= 1;
        }

        new_state.open_valves.insert(valve.clone());

        let pressure_increase = (*flow_rate as u32) * (new_state.mins_remaining as u32);
        new_state.final_pressure_released += pressure_increase;

        new_state
    }

    fn shortest_distance_to(&self, valve: &u8, scan_metadata: &ScanMetadata) -> u8 {
        let my_distance = scan_metadata.get_shortest_path(&self.my_location, valve);
        let el_distance = scan_metadata.get_shortest_path(&self.elephant_location, valve);
        cmp::min(*my_distance, *el_distance)
    }

    fn final_pressure_heuristic(&self, scan_metadata: &ScanMetadata) -> u32 {
        // guess the maximum final pressure
        // ensure we over-estimate
        let remaining_valves = scan_metadata.non_zero_valves.difference(&self.open_valves);

        // just add up the heuristic to each valve
        // assumes they are all in a straight line and doesn't account for time spent at each
        // but otherwise it's too hard to figure it out?
        let remaining_heuristic: u32 = remaining_valves
            .map(|valve| {
                let rate = scan_metadata.get_flow_rate(valve);
                let distance = self.shortest_distance_to(valve, scan_metadata);

                // it will take at least this many seconds to travel to the valve and then open it
                let time_to_open = distance + 1;
                if self.mins_remaining > time_to_open {
                    let max_time_at_valve = self.mins_remaining - time_to_open;
                    (max_time_at_valve as u32) * (*rate as u32)
                } else {
                    0
                }
            })
            .sum();

        self.final_pressure_released + remaining_heuristic
    }
}

impl fmt::Display for SolutionState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[@{}/{} #{} ~{} ({:?})]",
            self.my_location,
            self.elephant_location,
            self.mins_remaining,
            self.final_pressure_released,
            self.open_valves,
        )
    }
}

fn part1(input: &str) -> u32 {
    let readings = Reading::parse_list(input);
    Solver::run(&readings, "AA".to_string(), 30, false)
}

fn part2(input: &str) -> u32 {
    let readings = Reading::parse_list(input);
    Solver::run(&readings, "AA".to_string(), 26, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    static EXAMPLE1: &str = indoc! {"
        Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
        Valve BB has flow rate=13; tunnels lead to valves CC, AA
        Valve CC has flow rate=2; tunnels lead to valves DD, BB
        Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
        Valve EE has flow rate=3; tunnels lead to valves FF, DD
        Valve FF has flow rate=0; tunnels lead to valves EE, GG
        Valve GG has flow rate=0; tunnels lead to valves FF, HH
        Valve HH has flow rate=22; tunnel leads to valve GG
        Valve II has flow rate=0; tunnels lead to valves AA, JJ
        Valve JJ has flow rate=21; tunnel leads to valve II
    "};

    #[test]
    fn test_part1_example1() {
        let result = part1(EXAMPLE1);
        assert_eq!(result, 1651);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file());
        assert_eq!(result, 1862);
    }

    #[test]
    fn test_part2_example1() {
        let result = part2(EXAMPLE1);
        assert_eq!(result, 1707);
    }

    #[test]
    fn test_part2_solution() {
        let result = part2(&read_input_file());
        assert_eq!(result, 2422);
    }
}
