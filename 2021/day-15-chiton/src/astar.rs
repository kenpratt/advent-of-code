use crate::grid::Coordinate;
use crate::grid::Grid;

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;

pub fn solve(
    grid: &Grid<usize>,
    start: &Coordinate,
    goal: &Coordinate,
) -> (Vec<Coordinate>, usize) {
    let mut open_set: OpenSet<Coordinate> = OpenSet::new();
    let mut came_from: HashMap<Coordinate, Coordinate> = HashMap::new();
    let mut g_score: HashMap<Coordinate, usize> = HashMap::new();

    // add start to open set
    open_set.add(*start, h(start, goal));
    g_score.insert(*start, 0);

    while !open_set.is_empty() {
        // select lowest score in open_set
        let (current, _score) = open_set.pop().unwrap();

        // finished?
        if current == *goal {
            let path = reconstruct_path(goal, &came_from);
            let cost = *g_score.get(goal).unwrap();
            return (path, cost);
        }

        let current_g_score = *g_score.get(&current).unwrap();

        for neighbour in grid.neighbours(&current) {
            let tentative_g_score = current_g_score + grid.value(&neighbour);
            let neighbour_g_score = g_score.get(&neighbour);

            if neighbour_g_score.is_none() || tentative_g_score < *neighbour_g_score.unwrap() {
                // new best path to neighbour!
                came_from.insert(neighbour, current);
                g_score.insert(neighbour, tentative_g_score);
                open_set.add(neighbour, tentative_g_score + h(&neighbour, goal));
            }
        }
    }

    panic!("No path to goal");
}

fn h(from: &Coordinate, to: &Coordinate) -> usize {
    from.manhattan_distance(to)
}

fn reconstruct_path(
    to: &Coordinate,
    came_from: &HashMap<Coordinate, Coordinate>,
) -> Vec<Coordinate> {
    let mut path: Vec<Coordinate> = vec![*to];
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
