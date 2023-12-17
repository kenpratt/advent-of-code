pub mod astar;
pub mod grid;

use std::{cmp, fmt, fs};

use astar::AStarInterface;
use grid::*;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    println!("part 2 result: {:?}", part2(&read_input_file()));
}

fn read_input_file() -> String {
    fs::read_to_string("input.txt").expect("Something went wrong reading the file")
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Cursor {
    pos: Coord,
    direction: Direction,
    moved_straight: u16,
}

impl Cursor {
    fn new(pos: Coord, direction: Direction) -> Self {
        Self {
            pos,
            direction,
            moved_straight: 0,
        }
    }

    fn next(&self, grid: &Grid<u16>, turning_restrictions: &(u16, u16)) -> Vec<(Self, u16)> {
        let can_turn = self.moved_straight >= turning_restrictions.0;
        let can_go_straight = self.moved_straight < turning_restrictions.1;

        vec![
            if can_turn { self.go_left(grid) } else { None },
            if can_turn { self.go_right(grid) } else { None },
            if can_go_straight {
                self.go_straight(grid)
            } else {
                None
            },
        ]
        .into_iter()
        .filter_map(|r| r)
        .collect()
    }

    fn go_left(&self, grid: &Grid<u16>) -> Option<(Self, u16)> {
        self.go_in_direction(self.direction.counterclockwise(), 0, grid)
    }

    fn go_right(&self, grid: &Grid<u16>) -> Option<(Self, u16)> {
        self.go_in_direction(self.direction.clockwise(), 0, grid)
    }

    fn go_straight(&self, grid: &Grid<u16>) -> Option<(Self, u16)> {
        self.go_in_direction(self.direction, self.moved_straight, grid)
    }

    fn go_in_direction(
        &self,
        move_in_direction: Direction,
        moved_straight: u16,
        grid: &Grid<u16>,
    ) -> Option<(Self, u16)> {
        grid.shift(&self.pos, &move_in_direction).map(|new_pos| {
            (
                Self {
                    pos: new_pos,
                    direction: move_in_direction,
                    moved_straight: moved_straight + 1,
                },
                *grid.value(&new_pos),
            )
        })
    }
}

impl fmt::Debug for Cursor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Cursor")
            .field("pos", &self.pos)
            .field("direction", &self.direction)
            .field("moved_straight", &self.moved_straight)
            .finish()
    }
}

struct Solver<'a> {
    grid: &'a Grid<u16>,
    turning_restrictions: (u16, u16),
    end_pos: Coord,
}

impl<'a> Solver<'a> {
    fn run(
        grid: &'a Grid<u16>,
        turning_restrictions: (u16, u16),
        initial_direction: Direction,
    ) -> u16 {
        let initial = Cursor::new(Coord::new(0, 0), initial_direction);
        let end_pos = Coord::new(grid.width - 1, grid.height - 1);

        let mut solver = Solver {
            grid,
            turning_restrictions,
            end_pos,
        };
        match solver.shortest_path(initial, true) {
            Some((_path, cost)) => cost,
            None => panic!("No solution found"),
        }
    }
}

impl AStarInterface<Cursor> for Solver<'_> {
    fn at_goal(&self, node: &Cursor) -> bool {
        node.pos == self.end_pos && node.moved_straight >= self.turning_restrictions.0
    }

    fn heuristic(&self, from: &Cursor) -> u16 {
        from.pos.manhattan_distance(&self.end_pos) as u16
    }

    fn neighbours(&mut self, from: &Cursor) -> Vec<(Cursor, u16)> {
        from.next(self.grid, &self.turning_restrictions)
            .into_iter()
            .map(|to| to)
            .collect()
    }
}

fn part1(input: &str) -> u16 {
    let grid = Grid::parse(input, |c| c.to_digit(10).unwrap() as u16);
    Solver::run(&grid, (0, 3), Direction::East)
}

fn part2(input: &str) -> u16 {
    let grid = Grid::parse(input, |c| c.to_digit(10).unwrap() as u16);
    let s1 = Solver::run(&grid, (4, 10), Direction::East);
    let s2 = Solver::run(&grid, (4, 10), Direction::South);
    cmp::min(s1, s2)
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    static EXAMPLE: &str = indoc! {"
        2413432311323
        3215453535623
        3255245654254
        3446585845452
        4546657867536
        1438598798454
        4457876987766
        3637877979653
        4654967986887
        4564679986453
        1224686865563
        2546548887735
        4322674655533
    "};

    static EXAMPLE2: &str = indoc! {"
        111111111111
        999999999991
        999999999991
        999999999991
        999999999991
    "};

    #[test]
    fn test_part1_example() {
        let result = part1(EXAMPLE);
        assert_eq!(result, 102);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file());
        assert_eq!(result, 1155);
    }

    #[test]
    fn test_part2_example() {
        let result = part2(EXAMPLE);
        assert_eq!(result, 94);
    }

    #[test]
    fn test_part2_example2() {
        let result = part2(EXAMPLE2);
        assert_eq!(result, 71);
    }

    #[test]
    fn test_part2_solution() {
        let result = part2(&read_input_file());
        assert_eq!(result, 1283);
    }
}
