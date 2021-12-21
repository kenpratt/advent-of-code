use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::hash::Hash;

pub trait AStarInterface<N: Copy + Hash + Ord> {
    fn start(&self) -> &N;
    fn goal(&self) -> &N;
    fn heuristic(&self, from: &N, to: &N) -> usize;
    fn neighbours(&self, from: &N) -> Vec<(N, usize)>;

    fn shortest_path(&self) -> (Vec<N>, usize) {
        let start = self.start();
        let goal = self.goal();

        let mut open_set: OpenSet<N> = OpenSet::new();
        let mut came_from: HashMap<N, N> = HashMap::new();
        let mut g_score: HashMap<N, usize> = HashMap::new();

        // add start to open set
        open_set.add(*start, self.heuristic(start, goal));
        g_score.insert(*start, 0);

        while !open_set.is_empty() {
            // select lowest score in open_set
            let (current, _score) = open_set.pop().unwrap();

            // finished?
            if current == *goal {
                let path = Self::reconstruct_path(goal, &came_from);
                let cost = *g_score.get(goal).unwrap();
                return (path, cost);
            }

            let current_g_score = *g_score.get(&current).unwrap();

            for (neighbour, cost_to_neighbour) in self.neighbours(&current) {
                let tentative_g_score = current_g_score + cost_to_neighbour;
                let neighbour_g_score = g_score.get(&neighbour);

                if neighbour_g_score.is_none() || tentative_g_score < *neighbour_g_score.unwrap() {
                    // new best path to neighbour!
                    came_from.insert(neighbour, current);
                    g_score.insert(neighbour, tentative_g_score);
                    open_set.add(
                        neighbour,
                        tentative_g_score + self.heuristic(&neighbour, goal),
                    );
                }
            }
        }

        panic!("No path to goal");
    }

    fn reconstruct_path(to: &N, came_from: &HashMap<N, N>) -> Vec<N> {
        let mut path: Vec<N> = vec![*to];
        let mut last = to;
        loop {
            match came_from.get(&last) {
                Some(previous) => {
                    path.push(*previous);
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
