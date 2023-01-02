use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

pub trait AStarInterface<N: Copy + Debug + Eq + Hash> {
    fn at_goal(&self, node: &N) -> bool;
    fn heuristic(&self, from: &N) -> u16;
    fn neighbours(&mut self, from: &N) -> Vec<(N, u16)>;

    fn shortest_path(&mut self, start: N, print_stats: bool) -> Option<(Vec<(N, u16)>, u16)> {
        let mut open_set: OpenSet<N> = OpenSet::new();
        let mut came_from: HashMap<N, (N, u16)> = HashMap::new();
        let mut g_score: HashMap<N, u16> = HashMap::new();
        let mut iterations = 0;

        // add start to open set
        let start_heuristic = self.heuristic(&start);
        open_set.add(start, start_heuristic);
        g_score.insert(start, 0);

        while !open_set.is_empty() {
            iterations += 1;

            // select lowest score in open_set
            let (current, _score) = open_set.pop().unwrap();

            // finished?
            if self.at_goal(&current) {
                let path = Self::reconstruct_path(current, &came_from);
                let cost = *g_score.get(&current).unwrap();
                if print_stats {
                    println!(
                        "A* stats: {} iterations, {} states tracked",
                        iterations,
                        g_score.len()
                    );
                }
                return Some((path, cost));
            }

            let current_g_score = *g_score.get(&current).unwrap();

            for (neighbour, cost_to_neighbour) in self.neighbours(&current) {
                let tentative_g_score = current_g_score + cost_to_neighbour;
                let neighbour_g_score = g_score.get(&neighbour);

                if neighbour_g_score.is_none() || tentative_g_score < *neighbour_g_score.unwrap() {
                    // new best path to neighbour!
                    came_from.insert(neighbour, (current, cost_to_neighbour));
                    g_score.insert(neighbour, tentative_g_score);
                    open_set.add(neighbour, tentative_g_score + self.heuristic(&neighbour));
                }
            }
        }

        if print_stats {
            println!(
                "A* stats: {} iterations, {} states tracked",
                iterations,
                g_score.len()
            );
        }
        None
    }

    fn reconstruct_path(to: N, came_from: &HashMap<N, (N, u16)>) -> Vec<(N, u16)> {
        let mut path: Vec<(N, u16)> = vec![];
        let mut last = to;
        loop {
            match came_from.get(&last) {
                Some((previous, cost)) => {
                    path.push((last, *cost));
                    last = *previous;
                }
                None => {
                    path.reverse();
                    return path;
                }
            }
        }
    }
}

#[derive(Debug)]
struct OpenSet<N>(BinaryHeap<OpenSetEntry<N>>);

impl<N: Eq> OpenSet<N> {
    fn new() -> OpenSet<N> {
        OpenSet(BinaryHeap::new())
    }

    fn add(&mut self, node: N, cost: u16) {
        self.0.push(OpenSetEntry { node, cost });
    }

    fn pop(&mut self) -> Option<(N, u16)> {
        match self.0.pop() {
            Some(entry) => Some((entry.node, entry.cost)),
            None => None,
        }
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[derive(Debug, Eq, PartialEq)]
struct OpenSetEntry<N> {
    node: N,
    cost: u16,
}

impl<N: Eq> Ord for OpenSetEntry<N> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl<N: Eq> PartialOrd for OpenSetEntry<N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
