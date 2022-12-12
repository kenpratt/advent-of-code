use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

pub trait AStarInterface<N: Copy + Debug + Hash + Ord> {
    fn at_goal(&self, node: &N) -> bool;
    fn heuristic(&self, from: &N) -> usize;
    fn neighbours(&self, from: &N) -> Vec<(N, usize)>;

    fn shortest_path(&self, start: &N, print_stats: bool) -> Option<(Vec<(N, usize)>, usize)> {
        let mut open_set: OpenSet<N> = OpenSet::new();
        let mut came_from: HashMap<N, (N, usize)> = HashMap::new();
        let mut g_score: HashMap<N, usize> = HashMap::new();
        let mut iterations = 0;

        // add start to open set
        open_set.add(*start, self.heuristic(start));
        g_score.insert(*start, 0);

        while !open_set.is_empty() {
            iterations += 1;

            // select lowest score in open_set
            let (current, _score) = open_set.pop().unwrap();

            // finished?
            if self.at_goal(&current) {
                let path = Self::reconstruct_path(&current, &came_from);
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

        None
    }

    fn reconstruct_path(to: &N, came_from: &HashMap<N, (N, usize)>) -> Vec<(N, usize)> {
        let mut path: Vec<(N, usize)> = vec![];
        let mut last = to;
        loop {
            match came_from.get(&last) {
                Some((previous, cost)) => {
                    path.push((*last, *cost));
                    last = previous;
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

impl<N: Ord> OpenSet<N> {
    fn new() -> OpenSet<N> {
        OpenSet(BinaryHeap::new())
    }

    fn add(&mut self, node: N, cost: usize) {
        self.0.push(OpenSetEntry { node, cost });
    }

    fn pop(&mut self) -> Option<(N, usize)> {
        match self.0.pop() {
            Some(entry) => Some((entry.node, entry.cost)),
            None => None,
        }
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct OpenSetEntry<N> {
    node: N,
    cost: usize,
}

impl<N: Ord> Ord for OpenSetEntry<N> {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.node.cmp(&other.node))
    }
}

impl<N: Ord> PartialOrd for OpenSetEntry<N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
