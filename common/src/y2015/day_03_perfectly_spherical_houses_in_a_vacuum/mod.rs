use std::collections::HashSet;

use crate::interface::AoC;

pub struct Day;
impl AoC<Vec<Direction>, usize, usize> for Day {
    const FILE: &'static str = file!();

    fn parse(input: String) -> Vec<Direction> {
        Direction::parse_list(&input)
    }

    fn part1(directions: &Vec<Direction>) -> usize {
        let mut pos = Coord::new(0, 0);
        let mut visited = HashSet::from([pos]);

        for direction in directions {
            let next = pos.shift(direction);
            visited.insert(next);
            pos = next;
        }

        visited.len()
    }

    fn part2(directions: &Vec<Direction>) -> usize {
        let mut santa = Coord::new(0, 0);
        let mut robot = santa.clone();
        let mut use_santa = true;

        let mut visited = HashSet::from([santa]);

        for direction in directions {
            let pos = if use_santa { &mut santa } else { &mut robot };
            let next = pos.shift(direction);
            visited.insert(next.clone());
            *pos = next;
            use_santa = !use_santa;
        }

        visited.len()
    }
}

#[derive(Debug)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn parse_list(input: &str) -> Vec<Self> {
        input.chars().map(|c| Self::parse(&c)).collect()
    }

    fn parse(input: &char) -> Self {
        use Direction::*;

        match input {
            '^' => North,
            'v' => South,
            '>' => East,
            '<' => West,
            _ => panic!("Unexpected char: {:?}", input),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Coord {
    x: i16,
    y: i16,
}

impl Coord {
    fn new(x: i16, y: i16) -> Self {
        Self { x, y }
    }

    fn shift(&self, direction: &Direction) -> Self {
        use Direction::*;

        match direction {
            North => Self::new(self.x, self.y - 1),
            South => Self::new(self.x, self.y + 1),
            East => Self::new(self.x + 1, self.y),
            West => Self::new(self.x - 1, self.y),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_examples() {
        assert_eq!(Day::part1(&Day::parse_str(">")), 2);
        assert_eq!(Day::part1(&Day::parse_str("^>v<")), 4);
        assert_eq!(Day::part1(&Day::parse_str("^v^v^v^v^v")), 2);
    }

    #[test]
    fn test_part1_solution() {
        let result = Day::part1(&Day::parse_input_file());
        assert_eq!(result, 2572);
    }

    #[test]
    fn test_part2_examples() {
        assert_eq!(Day::part2(&Day::parse_str("^v")), 3);
        assert_eq!(Day::part2(&Day::parse_str("^>v<")), 3);
        assert_eq!(Day::part2(&Day::parse_str("^v^v^v^v^v")), 11);
    }

    #[test]
    fn test_part2_solution() {
        let result = Day::part2(&Day::parse_input_file());
        assert_eq!(result, 2631);
    }
}
