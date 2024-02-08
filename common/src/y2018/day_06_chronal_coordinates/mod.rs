use crate::{interface::AoCWithParams, spatial::*};
use std::collections::{HashMap, HashSet};

type NumT = u16;
type BoundsT = Bounds<NumT>;
type CoordT = Coord<NumT>;

pub struct Day;
impl AoCWithParams<(Vec<CoordT>, BoundsT), (), usize, usize, usize> for Day {
    const FILE: &'static str = file!();

    const PARAMS_PART1: () = ();
    const PARAMS_PART2: usize = 10000;

    fn parse(input: String) -> (Vec<CoordT>, BoundsT) {
        let nodes: Vec<CoordT> = input.lines().map(|line| Coord::parse(line, ", ")).collect();
        let bounds = Bounds::calculate(&nodes);
        (nodes, bounds)
    }

    fn part1((nodes, bounds): &(Vec<CoordT>, BoundsT), _: ()) -> usize {
        let mut infinite: HashSet<usize> = HashSet::new();
        let mut map: Grid<NumT, Territory> = Grid::new(*bounds);
        let node_indices: Vec<usize> = nodes.iter().map(|c| map.coord_to_index(c)).collect();

        for i in &node_indices {
            map.set(*i, Territory::Node(*i));
        }

        // explore from the nodes
        let mut frontier: Vec<(usize, usize)> = node_indices.iter().map(|i| (*i, *i)).collect();
        while !frontier.is_empty() {
            // for each round of exploration, expand to neighbours, tracking
            // which nodes are exploring to the new cell this round, so we can
            // detect collisions
            let mut tmp_map: Grid<NumT, Territory> = Grid::new(*bounds);
            for (to_expand, node) in frontier {
                for maybe_neighbour in map.neighbours(to_expand) {
                    match maybe_neighbour {
                        Some(neighbour) => {
                            if map.get(neighbour).is_none() {
                                match &tmp_map.get(neighbour) {
                                    Some(Territory::Node(n)) if node == *n => (),
                                    Some(Territory::Node(_)) => {
                                        // conflict
                                        tmp_map.set(neighbour, Territory::Conflict);
                                    }
                                    Some(Territory::Conflict) => (),
                                    None => {
                                        // new territory
                                        tmp_map.set(neighbour, Territory::Node(node));
                                    }
                                }
                            }
                        }
                        None => {
                            // no neighbour in this direction, must have hit edge => infinite area
                            infinite.insert(node);
                        }
                    }
                }
            }

            let mut new_frontier = vec![];
            for (i, val) in tmp_map.into_iter() {
                if let Territory::Node(node) = val {
                    new_frontier.push((i, node));
                }
                map.set(i, val);
            }
            frontier = new_frontier;
        }

        // now tally the results
        let mut bounded: HashMap<usize, usize> = HashMap::new();
        for (_i, owner) in map.iter() {
            match owner {
                Territory::Node(id) => {
                    if !infinite.contains(id) {
                        *bounded.entry(*id).or_default() += 1;
                    }
                }
                Territory::Conflict => (),
            }
        }

        // and return the highest
        *bounded.values().max().unwrap()
    }

    fn part2((nodes, bounds): &(Vec<CoordT>, BoundsT), target_distance: usize) -> usize {
        let mut map: Grid<NumT, ()> = Grid::new(*bounds);

        let middle = Coord::new(
            bounds.left + (bounds.right - bounds.left) / 2,
            bounds.top + (bounds.bottom - bounds.top) / 2,
        );
        let middle_index = map.coord_to_index(&middle);

        // start exploring in the middle, and explore the area that fits the constraints
        let mut winners = 0;
        let mut to_explore: Vec<usize> = vec![middle_index];

        while let Some(curr) = to_explore.pop() {
            if map.get(curr).is_none() {
                map.set(curr, ());

                let coord = map.index_to_coord(curr);

                let score: usize = nodes
                    .iter()
                    .map(|c| coord.manhattan_distance(c) as usize)
                    .sum();
                if score < target_distance {
                    winners += 1;

                    for maybe_neighbour in map.neighbours(curr) {
                        match maybe_neighbour {
                            Some(neighbour) => to_explore.push(neighbour),
                            None => (),
                        }
                    }
                }
            }
        }

        winners
    }
}

enum Territory {
    Node(usize),
    Conflict,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        let result = Day::part1(&Day::parse_example_file(), ());
        assert_eq!(result, 17);
    }

    #[test]
    fn test_part1_solution() {
        let result = Day::part1(&Day::parse_input_file(), ());
        assert_eq!(result, 6047);
    }

    #[test]
    fn test_part2_example() {
        let result = Day::part2(&Day::parse_example_file(), 32);
        assert_eq!(result, 16);
    }

    #[test]
    fn test_part2_solution() {
        let result = Day::part2(&Day::parse_input_file(), Day::PARAMS_PART2);
        assert_eq!(result, 46320);
    }
}
