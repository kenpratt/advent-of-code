pub mod astar;

use astar::AStarInterface;

use std::collections::BTreeMap;
use std::fs;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    println!("part 2 result: {:?}", part2(&read_input_file()));
}

fn read_input_file() -> String {
    fs::read_to_string("input.txt").expect("Something went wrong reading the file")
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Coordinate {
    x: i16,
    y: i16,
}

impl Coordinate {
    fn neighbour(&self, direction: &Direction) -> Self {
        use Direction::*;
        match direction {
            Up => Self {
                x: self.x,
                y: self.y - 1,
            },
            Right => Coordinate {
                x: self.x + 1,
                y: self.y,
            },
            Down => Coordinate {
                x: self.x,
                y: self.y + 1,
            },
            Left => Coordinate {
                x: self.x - 1,
                y: self.y,
            },
        }
    }

    fn wrapping_neighbour(&self, direction: &Direction, bounds: &Bounds) -> Self {
        use Direction::*;
        match direction {
            Up => Self {
                x: self.x,
                y: if self.y == bounds.min_y {
                    bounds.max_y
                } else {
                    self.y - 1
                },
            },
            Right => Coordinate {
                x: if self.x == bounds.max_x {
                    bounds.min_x
                } else {
                    self.x + 1
                },
                y: self.y,
            },
            Down => Coordinate {
                x: self.x,
                y: if self.y == bounds.max_y {
                    bounds.min_y
                } else {
                    self.y + 1
                },
            },
            Left => Coordinate {
                x: if self.x == bounds.min_x {
                    bounds.max_x
                } else {
                    self.x - 1
                },
                y: self.y,
            },
        }
    }

    fn manhattan_distance(&self, other: &Coordinate) -> u16 {
        ((self.x - other.x).abs() + (self.y - other.y).abs())
            .try_into()
            .unwrap()
    }
}

#[derive(Debug)]
struct Bounds {
    min_x: i16,
    max_x: i16,
    min_y: i16,
    max_y: i16,
}

impl Bounds {
    fn calculate(positions: &[Coordinate], pad: i16) -> Self {
        let min_x = positions.iter().map(|p| p.x).min().unwrap();
        let max_x = positions.iter().map(|p| p.x).max().unwrap();
        let min_y = positions.iter().map(|p| p.y).min().unwrap();
        let max_y = positions.iter().map(|p| p.y).max().unwrap();
        Self {
            min_x: min_x + pad,
            max_x: max_x - pad,
            min_y: min_y + pad,
            max_y: max_y - pad,
        }
    }

    fn contains(&self, pos: &Coordinate) -> bool {
        pos.x >= self.min_x && pos.x <= self.max_x && pos.y >= self.min_y && pos.y <= self.max_y
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Cell {
    Ground,
    Wall,
    Blizzard(Direction),
}

impl Cell {
    fn parse(c: &char) -> Self {
        use Cell::*;
        use Direction::*;
        match c {
            '.' => Ground,
            '#' => Wall,
            '^' => Blizzard(Up),
            'v' => Blizzard(Down),
            '<' => Blizzard(Left),
            '>' => Blizzard(Right),
            _ => panic!("Unexpected input: {}", c),
        }
    }

    fn is_blizzard(&self) -> bool {
        match self {
            Cell::Blizzard(_) => true,
            _ => false,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

const DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
];

#[derive(Debug)]
struct Map {
    entrance: Coordinate,
    exit: Coordinate,
    blizzard_bounds: Bounds,
    blizzards: Vec<BTreeMap<Coordinate, Vec<Direction>>>,
}

impl Map {
    fn parse(input: &str) -> Self {
        let cells: Vec<Vec<(Coordinate, Cell)>> = input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| {
                        (
                            Coordinate {
                                x: x.try_into().unwrap(),
                                y: y.try_into().unwrap(),
                            },
                            Cell::parse(&c),
                        )
                    })
                    .collect::<Vec<(Coordinate, Cell)>>()
            })
            .collect();

        let walls = cells
            .iter()
            .flat_map(|row| {
                row.iter()
                    .filter(|(_p, c)| *c == Cell::Wall)
                    .map(|(p, _c)| *p)
                    .collect::<Vec<Coordinate>>()
            })
            .collect::<Vec<Coordinate>>();
        let blizzard_bounds = Bounds::calculate(&walls, 1);

        let initial_blizzards = cells
            .iter()
            .flat_map(|row| {
                row.iter()
                    .filter(|(_p, c)| c.is_blizzard())
                    .map(|(p, c)| match c {
                        Cell::Blizzard(d) => (*p, vec![*d]),
                        _ => panic!("Unreachable"),
                    })
                    .collect::<Vec<(Coordinate, Vec<Direction>)>>()
            })
            .collect();

        let blizzards = vec![initial_blizzards];

        let entrance = cells
            .first()
            .unwrap()
            .iter()
            .filter(|(_p, c)| *c == Cell::Ground)
            .map(|(p, _c)| *p)
            .next()
            .unwrap();

        let exit = cells
            .last()
            .unwrap()
            .iter()
            .filter(|(_p, c)| *c == Cell::Ground)
            .map(|(p, _c)| *p)
            .next()
            .unwrap();

        Self {
            entrance,
            exit,
            blizzard_bounds,
            blizzards,
        }
    }

    fn blizzards_at_minute(&mut self, minute: usize) -> &BTreeMap<Coordinate, Vec<Direction>> {
        while self.blizzards.len() <= minute {
            let last_blizzards = self.blizzards.last().unwrap();
            let next_blizzards = Self::blizzard_iter(last_blizzards, &self.blizzard_bounds);
            self.blizzards.push(next_blizzards);
        }
        &self.blizzards[minute]
    }

    fn blizzard_iter(
        curr_blizzards: &BTreeMap<Coordinate, Vec<Direction>>,
        bounds: &Bounds,
    ) -> BTreeMap<Coordinate, Vec<Direction>> {
        let mut next_blizzards = BTreeMap::new();
        for (curr_pos, directions) in curr_blizzards {
            for direction in directions {
                let next_pos = curr_pos.wrapping_neighbour(direction, bounds);
                next_blizzards
                    .entry(next_pos)
                    .or_insert(vec![])
                    .push(*direction);
            }
        }
        next_blizzards
    }

    fn possible_moves(&self, pos: &Coordinate) -> Vec<Coordinate> {
        let mut res: Vec<Coordinate> = DIRECTIONS
            .iter()
            .map(|d| pos.neighbour(d))
            .filter(|p| self.blizzard_bounds.contains(p) || p == &self.entrance || p == &self.exit)
            .collect();
        res.push(*pos); // stay still
        res
    }
}

#[derive(Debug)]
struct Solver {
    map: Map,
    goals: Vec<Coordinate>,
    future_goal_heuristic: u16,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct SolutionState {
    minute: usize,
    position: Coordinate,
    goal_index: usize,
}

impl Solver {
    fn run(map: Map, back_and_forth: bool) -> u16 {
        let start = SolutionState {
            minute: 0,
            position: map.entrance.clone(),
            goal_index: 0,
        };
        let goals = if back_and_forth {
            vec![map.exit.clone(), map.entrance.clone(), map.exit.clone()]
        } else {
            vec![map.exit.clone()]
        };
        let future_goal_heuristic = map.entrance.manhattan_distance(&map.exit);
        let mut solver = Solver {
            map,
            goals,
            future_goal_heuristic,
        };
        match solver.shortest_path(start, true) {
            Some((_path, length)) => length,
            None => panic!("No solution found"),
        }
    }
}

impl AStarInterface<SolutionState> for Solver {
    fn at_goal(&self, node: &SolutionState) -> bool {
        node.goal_index == self.goals.len()
    }

    fn heuristic(&self, from: &SolutionState) -> u16 {
        if self.at_goal(from) {
            0
        } else {
            let next_goal = &self.goals[from.goal_index];
            let heuristic_distance = from.position.manhattan_distance(next_goal);

            let remaining_goals = (self.goals.len() - from.goal_index - 1) as u16;
            heuristic_distance + (remaining_goals * self.future_goal_heuristic)
        }
    }

    fn neighbours(&mut self, from: &SolutionState) -> Vec<(SolutionState, u16)> {
        let next_minute = from.minute + 1;
        let possible_moves = self.map.possible_moves(&from.position);
        let blizzards = self.map.blizzards_at_minute(next_minute);
        possible_moves
            .into_iter()
            .filter(|p| !blizzards.contains_key(p))
            .map(|p| SolutionState {
                position: p,
                minute: next_minute,
                goal_index: if &p == &self.goals[from.goal_index] {
                    from.goal_index + 1
                } else {
                    from.goal_index
                },
            })
            .map(|s| (s, 1))
            .collect()
    }
}

fn part1(input: &str) -> u16 {
    let map = Map::parse(input);
    Solver::run(map, false)
}

fn part2(input: &str) -> u16 {
    let map = Map::parse(input);
    Solver::run(map, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    static EXAMPLE1: &str = indoc! {"
        #.#####
        #.....#
        #>....#
        #.....#
        #...v.#
        #.....#
        #####.#
    "};

    static EXAMPLE2: &str = indoc! {"
        #.######
        #>>.<^<#
        #.<..<<#
        #>v.><>#
        #<^v^^>#
        ######.#
    "};

    #[test]
    fn test_part1_example1() {
        let result = part1(EXAMPLE1);
        assert_eq!(result, 10);
    }

    #[test]
    fn test_part1_example2() {
        let result = part1(EXAMPLE2);
        assert_eq!(result, 18);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file());
        assert_eq!(result, 271);
    }

    #[test]
    fn test_part2_example1() {
        let result = part2(EXAMPLE1);
        assert_eq!(result, 30);
    }

    #[test]
    fn test_part2_example2() {
        let result = part2(EXAMPLE2);
        assert_eq!(result, 54);
    }

    #[test]
    fn test_part2_solution() {
        let result = part2(&read_input_file());
        assert_eq!(result, 813);
    }
}
