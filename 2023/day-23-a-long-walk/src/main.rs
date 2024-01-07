pub mod grid;

use std::{
    collections::{HashMap, HashSet},
    fs,
};

use grid::*;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    println!("part 2 result: {:?}", part2(&read_input_file()));
}

fn read_input_file() -> String {
    fs::read_to_string("input.txt").expect("Something went wrong reading the file")
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

    fn to_graph(&self, use_slopes: bool) -> Graph {
        // start on path in top row
        let start = (0..self.grid.width)
            .map(|x| Coord::new(x, 0))
            .find(|c| self.grid.value(c) == &Terrain::Path)
            .unwrap();

        // end on path in bottom row
        let end = (0..self.grid.width)
            .map(|x| Coord::new(x, self.grid.height - 1))
            .find(|c| self.grid.value(c) == &Terrain::Path)
            .unwrap();

        // find the neighbours of each tile
        // and factor in whether to treat slopes as slippery (part 1 vs 2)
        let neighbours_map: HashMap<Coord, Vec<Coord>> = self
            .grid
            .cells
            .iter()
            .filter(|(_pos, t)| t != &&Terrain::Forest)
            .map(|(pos, t)| (*pos, self.neighbours(pos, t, use_slopes)))
            .collect();

        // find all the junctions where you can make a choice of where to go
        let mut junctions: HashSet<Coord> = neighbours_map
            .iter()
            .filter(|(_, neighbours)| neighbours.len() > 2)
            .map(|(pos, _)| *pos)
            .collect();

        // add start & end as synthetic junctions
        junctions.insert(start.clone());
        junctions.insert(end.clone());

        // explore where junctions/start/end connect to each other (these will be the nodes)
        let mut connections: HashMap<Coord, Vec<(Coord, usize)>> = HashMap::new();
        for pos in &junctions {
            let conn = Self::find_connections(pos, &neighbours_map, &junctions);
            connections.insert(*pos, conn);
        }

        // okay, now we can discard all the coords and convert to simple IDs, and return a graph
        let id_map: HashMap<&Coord, u8> = junctions
            .iter()
            .enumerate()
            .map(|(id, pos)| (pos, id.try_into().unwrap()))
            .collect();

        // convert connections hash to a vec of ids
        let mut graph_connections_tmp: Vec<Option<Vec<(u8, usize)>>> = vec![None; id_map.len()];
        for (pos, id) in &id_map {
            let conns = connections.get(pos).unwrap();
            let id_conns: Vec<(u8, usize)> = conns
                .iter()
                .map(|(c, d)| (*id_map.get(c).unwrap(), *d))
                .collect();
            graph_connections_tmp[*id as usize] = Some(id_conns);
        }
        let graph_connections = graph_connections_tmp.into_iter().flatten().collect();

        // return a graph structure
        Graph {
            start: *id_map.get(&start).unwrap(),
            end: *id_map.get(&end).unwrap(),
            connections: graph_connections,
        }
    }

    fn neighbours(&self, pos: &Coord, terrain: &Terrain, use_slopes: bool) -> Vec<Coord> {
        if terrain == &Terrain::Forest {
            panic!("Shouldn't be a forest");
        }

        if use_slopes {
            let directions = terrain.next_directions();
            self.neighbours_in_directions(pos, &directions)
        } else {
            self.neighbours_in_directions(pos, &ALL_DIRECTIONS)
        }
    }

    fn neighbours_in_directions(&self, pos: &Coord, directions: &[Direction]) -> Vec<Coord> {
        directions
            .iter()
            .filter_map(|dir| self.grid.neighbour(pos, dir))
            .filter(|dest| self.grid.value(dest) != &Terrain::Forest)
            .collect()
    }

    fn find_connections(
        origin: &Coord,
        neighbours_map: &HashMap<Coord, Vec<Coord>>,
        junctions: &HashSet<Coord>,
    ) -> Vec<(Coord, usize)> {
        let mut to_explore: Vec<(&Coord, &Coord, usize)> = neighbours_map
            .get(origin)
            .unwrap()
            .iter()
            .map(|n| (origin, n, 1))
            .collect();

        let mut results = vec![];

        while let Some((from, to, dist)) = to_explore.pop() {
            if junctions.contains(to) {
                results.push((*to, dist));
            } else {
                for next in neighbours_map.get(to).unwrap() {
                    if next != from {
                        to_explore.push((to, next, dist + 1));
                    }
                }
            }
        }

        if results.iter().any(|(c, _)| c == origin) {
            panic!("Connection to origin coord");
        }

        results
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Terrain {
    Path,
    Forest,
    Slope(Direction),
}

impl Terrain {
    fn parse(input: &char) -> Self {
        use Direction::*;
        use Terrain::*;

        match input {
            '.' => Path,
            '#' => Forest,
            '^' => Slope(North),
            '>' => Slope(East),
            'v' => Slope(South),
            '<' => Slope(West),
            _ => panic!("Unexpected terrain: {:?}", input),
        }
    }

    fn next_directions(&self) -> Vec<Direction> {
        use Terrain::*;

        match self {
            Path => ALL_DIRECTIONS.to_vec(),
            Forest => panic!("Shouldn't be on a forest tile"),
            Slope(d) => vec![*d],
        }
    }
}

#[derive(Debug)]
struct Graph {
    start: u8,
    end: u8,
    connections: Vec<Vec<(u8, usize)>>,
}

impl Graph {
    fn solve(&self) -> usize {
        let mut high_score = 0;
        let mut open_set = vec![SolutionState::new(self.start, self.connections.len())];

        while let Some(curr) = open_set.pop() {
            if curr.pos == self.end {
                // we're at the goal!
                if curr.distance > high_score {
                    high_score = curr.distance;
                }
            } else {
                let conns = &self.connections[curr.pos as usize];
                let mut next = curr.next(conns);
                open_set.append(&mut next);
            }
        }

        high_score
    }
}

#[derive(Debug)]
struct SolutionState {
    pos: u8,
    visited: Vec<bool>,
    distance: usize,
}

impl SolutionState {
    fn new(start: u8, num_ids: usize) -> Self {
        let mut visited = vec![false; num_ids];
        visited[start as usize] = true;

        Self {
            pos: start,
            visited: visited,
            distance: 0,
        }
    }

    fn next(&self, connections: &[(u8, usize)]) -> Vec<SolutionState> {
        connections
            .iter()
            .filter(|(dest, _)| !self.visited[*dest as usize])
            .map(|(dest, dist)| self.move_to(dest, dist))
            .collect()
    }

    fn move_to(&self, destination: &u8, distance: &usize) -> Self {
        let mut next_visited = self.visited.clone();
        next_visited[*destination as usize] = true;

        Self {
            pos: *destination,
            visited: next_visited,
            distance: self.distance + distance,
        }
    }
}

fn part1(input: &str) -> usize {
    let map = Map::parse(input);
    let graph = map.to_graph(true);
    graph.solve()
}

fn part2(input: &str) -> usize {
    let map = Map::parse(input);
    let graph = map.to_graph(false);
    graph.solve()
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
        assert_eq!(result, 94);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file());
        assert_eq!(result, 1966);
    }

    #[test]
    fn test_part2_example() {
        let result = part2(&read_example_file());
        assert_eq!(result, 154);
    }

    #[test]
    fn test_part2_solution() {
        let result = part2(&read_input_file());
        assert_eq!(result, 6286);
    }
}
