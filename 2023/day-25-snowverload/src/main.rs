use std::{
    collections::{HashMap, HashSet},
    fs,
    ops::{Add, RangeInclusive, Sub},
};

use rand::Rng;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
}

fn read_input_file() -> String {
    fs::read_to_string("input.txt").expect("Something went wrong reading the file")
}

#[derive(Debug)]
struct ConnectionMap<'a> {
    ids: HashSet<&'a str>,
    connections: Vec<(&'a str, &'a str)>,
    connected_to: HashMap<&'a str, HashSet<&'a str>>,
}

impl<'a> ConnectionMap<'a> {
    fn parse(input: &'a str) -> Self {
        let lines = input.lines().map(|line| Self::parse_line(line));

        let mut ids = HashSet::new();
        let mut connections = vec![];
        let mut connected_to: HashMap<&str, HashSet<&str>> = HashMap::new();

        for (from, tos) in lines {
            ids.insert(from);

            for to in tos {
                ids.insert(to);

                connections.push((from, to));

                connected_to.entry(from).or_default().insert(to);
                connected_to.entry(to).or_default().insert(from);
            }
        }

        Self {
            ids,
            connections,
            connected_to,
        }
    }

    fn parse_line(input: &str) -> (&str, Vec<&str>) {
        // jqt: rhn xhk nvd
        let mut parts = input.split(": ");
        let left = parts.next().unwrap();
        let right = parts.next().unwrap().split(" ").collect();
        assert_eq!(parts.next(), None);
        (left, right)
    }

    fn neighbours(&self, id: &str) -> &HashSet<&str> {
        self.connected_to.get(id).unwrap()
    }

    fn disconnect_groups(&self) -> (usize, usize) {
        let sim = Simulation::new(self);
        Simulation::run(sim)
    }
}

#[derive(Clone, Copy, Debug)]
struct Coord {
    x: f64,
    y: f64,
    z: f64,
}

impl Coord {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    fn random(bounds: &RangeInclusive<f64>) -> Self {
        let x = rand_in_range(bounds);
        let y = rand_in_range(bounds);
        let z = rand_in_range(bounds);
        Self::new(x, y, z)
    }

    fn distance(&self, other: &Coord) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx.powi(2) + dy.powi(2) + dz.powi(2)).sqrt()
    }

    fn average(coords: &[&Coord]) -> Coord {
        let n = coords.len() as f64;
        let x = coords.iter().map(|c| c.x).sum::<f64>() / n;
        let y = coords.iter().map(|c| c.y).sum::<f64>() / n;
        let z = coords.iter().map(|c| c.z).sum::<f64>() / n;
        Self::new(x, y, z)
    }
}

impl Add for Coord {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Coord {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl std::ops::Mul<Coord> for f64 {
    type Output = Coord;

    fn mul(self, coord: Coord) -> Coord {
        Coord {
            x: coord.x * self,
            y: coord.y * self,
            z: coord.z * self,
        }
    }
}

const INITIAL_BOUNDS: RangeInclusive<f64> = -1000.0..=1000.0;
const SETTLE_THRESHOLD: f64 = 1.0;
const NUM_DISCONNECTS: usize = 3;

#[derive(Debug)]
struct Simulation<'a> {
    map: &'a ConnectionMap<'a>,
    positions: HashMap<&'a str, Coord>,
    remaining_disconnects: usize,
    disconnected: HashSet<(&'a str, &'a str)>,
}

impl<'a> Simulation<'a> {
    fn new(map: &'a ConnectionMap<'a>) -> Self {
        let positions: HashMap<&str, Coord> = map
            .ids
            .iter()
            .map(|k| (*k, Coord::random(&INITIAL_BOUNDS)))
            .collect();
        let remaining_disconnects = NUM_DISCONNECTS;
        let disconnected = HashSet::new();

        Self {
            map,
            positions,
            remaining_disconnects,
            disconnected,
        }
    }

    fn run(mut sim: Simulation) -> (usize, usize) {
        sim.settle();

        let mut curr = sim;
        while curr.remaining_disconnects > 0 {
            curr = curr.disconnect();
            curr.settle();
        }

        let group1 = curr.reachable(curr.map.ids.iter().next().unwrap());
        let group2: HashSet<&str> = curr
            .map
            .ids
            .iter()
            .filter(|id| !group1.contains(*id))
            .cloned()
            .collect();

        (group1.len(), group2.len())
    }

    fn settle(&mut self) {
        loop {
            let settled = self.tick(SETTLE_THRESHOLD);
            if settled {
                return;
            }
        }
    }

    fn tick(&mut self, threshold: f64) -> bool {
        let mut new_positions: HashMap<&str, Coord> = HashMap::new();
        let mut over_threshold = false;

        for (node, position) in &self.positions {
            let neighbours = self.map.neighbours(node);
            let neighbour_positions: Vec<&Coord> = neighbours
                .iter()
                .filter(|n| !self.is_disconnected(*node, *n))
                .map(|n| self.positions.get(n).unwrap())
                .collect();
            let new_position = Self::calculate_new_position(&neighbour_positions);

            let moved = position.distance(&new_position);
            if moved > threshold {
                over_threshold = true;
            }

            new_positions.insert(*node, new_position);
        }

        self.positions = new_positions;
        !over_threshold
    }

    fn calculate_new_position(neighbours: &[&Coord]) -> Coord {
        Coord::average(neighbours)
    }

    fn disconnect(&self) -> Self {
        let (conn, _dist) = self
            .map
            .connections
            .iter()
            .filter(|c| !self.disconnected.contains(c))
            .map(|c| {
                let p1 = self.positions.get(c.0).unwrap();
                let p2 = self.positions.get(c.1).unwrap();
                let d = p1.distance(p2);
                (c, d)
            })
            .max_by(|(_, d1), (_, d2)| d1.partial_cmp(d2).unwrap())
            .unwrap();

        let mut new_disconnected = self.disconnected.clone();
        new_disconnected.insert(*conn);

        Self {
            map: self.map,
            positions: self.positions.clone(),
            remaining_disconnects: self.remaining_disconnects - 1,
            disconnected: new_disconnected,
        }
    }

    fn is_disconnected(&self, a: &str, b: &str) -> bool {
        self.disconnected.contains(&(a, b)) || self.disconnected.contains(&(b, a))
    }

    // crawl graph to see what's reachable from a given starting node
    fn reachable(&self, id: &'a str) -> HashSet<&'a str> {
        let mut found = HashSet::from([id]);

        let mut to_explore = vec![id];

        while let Some(curr) = to_explore.pop() {
            for neighbour in self.map.neighbours(curr) {
                if !found.contains(neighbour) && !self.is_disconnected(curr, &neighbour) {
                    found.insert(neighbour);
                    to_explore.push(neighbour);
                }
            }
        }

        found
    }
}

fn rand_in_range(range: &RangeInclusive<f64>) -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(range.clone())
}

fn part1(input: &str) -> usize {
    let map = ConnectionMap::parse(input);
    let (a, b) = map.disconnect_groups();
    a * b
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read_example_file() -> String {
        fs::read_to_string("example.txt").expect("Something went wrong reading the file")
    }

    #[test]
    fn test_part1_example() {
        let result = part1(&read_example_file());
        assert_eq!(result, 54);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file());
        assert_eq!(result, 582692);
    }
}
