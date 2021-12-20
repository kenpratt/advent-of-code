use crate::grid::Coordinate;
use crate::grid::Grid;

use std::collections::HashMap;

pub fn solve(
    grid: &Grid<usize>,
    start: &Coordinate,
    goal: &Coordinate,
) -> (Vec<Coordinate>, usize) {
    let mut open_set: HashMap<Coordinate, usize> = HashMap::new();
    let mut came_from: HashMap<Coordinate, Coordinate> = HashMap::new();
    let mut g_score: HashMap<Coordinate, usize> = HashMap::new();

    // add start to open set
    open_set.insert(*start, h(start, goal));
    g_score.insert(*start, 0);

    while !open_set.is_empty() {
        // select lowest score in open_set
        let current = *open_set
            .iter()
            .min_by_key(|(_pos, f_score)| *f_score)
            .unwrap()
            .0;
        println!("current: {:?}", current);

        // finished?
        if current == *goal {
            let path = reconstruct_path(goal, &came_from);
            let cost = *g_score.get(goal).unwrap();
            return (path, cost);
        }

        // remove from open_set
        open_set.remove(&current);

        let current_g_score = *g_score.get(&current).unwrap();
        println!("current_g_score: {:?}", current_g_score);

        for neighbour in grid.neighbours(&current) {
            println!("neighbour: {:?}", neighbour);

            let tentative_g_score = current_g_score + grid.value(&neighbour);
            println!("tentative_g_score: {:?}", tentative_g_score);

            let neighbour_g_score = g_score.get(&neighbour);
            println!("neighbour_g_score: {:?}", neighbour_g_score);

            if neighbour_g_score.is_none() || tentative_g_score < *neighbour_g_score.unwrap() {
                // new best path to neighbour!
                came_from.insert(neighbour, current);
                g_score.insert(neighbour, tentative_g_score);
                open_set.insert(neighbour, tentative_g_score + h(&neighbour, goal));
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
