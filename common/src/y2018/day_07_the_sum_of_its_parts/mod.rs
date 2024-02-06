use crate::file::*;
use std::{
    cmp::{self, Reverse},
    collections::{BinaryHeap, HashMap, HashSet},
};

use lazy_static::lazy_static;
use regex::Regex;

pub fn run() {
    let input = parse(&read_input_file!());
    println!("part 1 result: {:?}", part1(&input));
    println!("part 2 result: {:?}", part2(&input, 5, 60));
}

#[derive(Debug)]
struct Requirement {
    prerequisite: char,
    step: char,
}

impl Requirement {
    fn parse_list(input: &str) -> Vec<Self> {
        input.lines().map(|line| Self::parse(line)).collect()
    }

    fn parse(input: &str) -> Self {
        lazy_static! {
            static ref ITEM_RE: Regex =
                Regex::new(r"\AStep ([A-Z]) must be finished before step ([A-Z]) can begin.\z")
                    .unwrap();
        }

        let caps = ITEM_RE.captures(input).unwrap();
        let prerequisite = caps.get(1).unwrap().as_str().chars().next().unwrap();
        let step = caps.get(2).unwrap().as_str().chars().next().unwrap();
        Self { prerequisite, step }
    }
}

struct Dependencies {
    metadata: HashMap<char, (HashSet<char>, HashSet<char>)>,
    starting_nodes: Vec<char>,
}

impl Dependencies {
    fn parse(input: &str) -> Self {
        let requirements = Requirement::parse_list(input);

        // build list of all steps
        let steps = {
            let mut tmp: HashSet<char> = requirements.iter().map(|r| r.step).collect();
            tmp.extend(requirements.iter().map(|r| r.prerequisite));
            tmp
        };

        // build map of prerequisites & expansions for each step
        let mut metadata: HashMap<char, (HashSet<char>, HashSet<char>)> = steps
            .iter()
            .map(|step| (*step, (HashSet::new(), HashSet::new())))
            .collect();
        for requirement in &requirements {
            // insert prereq
            metadata
                .get_mut(&requirement.step)
                .unwrap()
                .0
                .insert(requirement.prerequisite);

            // insert expansion
            metadata
                .get_mut(&requirement.prerequisite)
                .unwrap()
                .1
                .insert(requirement.step);
        }

        // find starting nodes (those with no prerequisites)
        let starting_nodes = metadata
            .iter()
            .filter(|(_step, (prereqs, _expansions))| prereqs.is_empty())
            .map(|(step, _)| *step)
            .collect();

        Self {
            metadata,
            starting_nodes,
        }
    }

    fn prerequisites(&self, node: &char) -> &HashSet<char> {
        &self.metadata.get(node).unwrap().0
    }

    fn expansions(&self, node: &char) -> &HashSet<char> {
        &self.metadata.get(node).unwrap().1
    }

    fn prerequisites_met(&self, node: &char, visited: &HashSet<char>) -> bool {
        let prereqs = self.prerequisites(node);
        prereqs.is_subset(visited)
    }
}

fn parse(input: &str) -> Dependencies {
    Dependencies::parse(input)
}

fn part1(dependencies: &Dependencies) -> String {
    // reversed binary heap will pop the lowest char first
    let mut frontier: BinaryHeap<Reverse<char>> = dependencies
        .starting_nodes
        .iter()
        .map(|c| Reverse(*c))
        .collect();

    let mut visited = HashSet::new();
    let mut visited_order = vec![];

    // visit first node in alphabetic order
    while let Some(Reverse(curr)) = frontier.pop() {
        visited.insert(curr);
        visited_order.push(curr);

        // expand all nodes who now have their dependencies met
        for to_expand in dependencies.expansions(&curr) {
            if dependencies.prerequisites_met(to_expand, &visited) {
                frontier.push(Reverse(*to_expand));
            }
        }
    }

    visited_order.into_iter().collect()
}

fn part2(dependencies: &Dependencies, num_workers: usize, base_duration: usize) -> usize {
    // reversed binary heap will pop the lowest item first
    let mut frontier: BinaryHeap<Reverse<(usize, char)>> = dependencies
        .starting_nodes
        .iter()
        .map(|c| Reverse((0, *c)))
        .collect();

    // also track worker available times in a binary heap
    let mut workers: BinaryHeap<Reverse<usize>> = BinaryHeap::new();
    for _ in 0..num_workers {
        workers.push(Reverse(0));
    }

    let mut visited = HashSet::new();

    // visit first available node (using alphabitec order as tie-break)
    while let Some(Reverse((curr_available_at, curr_id))) = frontier.pop() {
        visited.insert(curr_id);

        let Reverse(worker_available_at) = workers.pop().unwrap();

        let start_at = cmp::max(worker_available_at, curr_available_at);
        let end_at = start_at + base_duration + (curr_id as usize - 64);

        // schedule the worker for post completion
        workers.push(Reverse(end_at));

        // expand all nodes who now have their dependencies met
        for to_expand in dependencies.expansions(&curr_id) {
            if dependencies.prerequisites_met(to_expand, &visited) {
                frontier.push(Reverse((end_at, *to_expand)));
            }
        }
    }

    workers.into_iter().map(|Reverse(v)| v).max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        let result = part1(&parse(&read_example_file!()));
        assert_eq!(result, "CABDFE");
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&parse(&read_input_file!()));
        assert_eq!(result, "EPWCFXKISTZVJHDGNABLQYMORU");
    }

    #[test]
    fn test_part2_example() {
        let result = part2(&parse(&read_example_file!()), 2, 0);
        assert_eq!(result, 15);
    }

    #[test]
    fn test_part2_solution() {
        let result = part2(&parse(&read_input_file!()), 5, 60);
        assert_eq!(result, 952);
    }
}
