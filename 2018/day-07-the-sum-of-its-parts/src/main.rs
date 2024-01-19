use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    fs,
};

use lazy_static::lazy_static;
use regex::Regex;

const INPUT_FILE: &'static str = "input.txt";

fn main() {
    println!("part 1 result: {:?}", part1(&read_file(INPUT_FILE)));
    // println!("part 2 result: {:?}", part2(&read_file(INPUT_FILE)));
}

fn read_file(filename: &str) -> String {
    fs::read_to_string(filename).expect("Something went wrong reading the file")
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

fn part1(input: &str) -> String {
    let dependencies = Dependencies::parse(input);

    // reversed binary heap will pop the lowest char first
    let mut frontier: BinaryHeap<Reverse<char>> = dependencies
        .starting_nodes
        .iter()
        .map(|c| Reverse(*c))
        .collect();
    let mut visited = HashSet::new();
    let mut visited_order = vec![];

    while let Some(Reverse(curr)) = frontier.pop() {
        visited.insert(curr);
        visited_order.push(curr);

        for to_expand in dependencies.expansions(&curr) {
            if dependencies.prerequisites_met(to_expand, &visited) {
                frontier.push(Reverse(*to_expand))
            }
        }
    }

    visited_order.into_iter().collect()
}

// fn part2(input: &str) -> usize {
// }

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_FILE: &'static str = "example.txt";

    #[test]
    fn test_part1_example() {
        let result = part1(&read_file(EXAMPLE_FILE));
        assert_eq!(result, "CABDFE");
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_file(INPUT_FILE));
        assert_eq!(result, "EPWCFXKISTZVJHDGNABLQYMORU");
    }

    // #[test]
    // fn test_part2_example() {
    //     let result = part2(&read_file(EXAMPLE_FILE));
    //     assert_eq!(result, 0);
    // }

    // #[test]
    // fn test_part2_solution() {
    //     let result = part2(&read_file(INPUT_FILE));
    //     assert_eq!(result, 0);
    // }
}
