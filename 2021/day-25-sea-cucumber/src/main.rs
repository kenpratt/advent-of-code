use std::fmt;
use std::fs;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
}

fn read_input_file() -> String {
    fs::read_to_string("input.txt").expect("Something went wrong reading the file")
}

#[derive(Debug)]
struct Map {
    cucumbers: Vec<Option<Facing>>,
    width: usize,
    height: usize,
}

impl Map {
    fn parse(input: &str) -> Map {
        let width = input.lines().next().unwrap().chars().count();
        let height = input.lines().count();
        let mut cucumbers = vec![];
        for line in input.lines() {
            for c in line.chars() {
                cucumbers.push(Self::parse_char(&c));
            }
        }
        Map {
            cucumbers,
            width,
            height,
        }
    }

    fn parse_char(c: &char) -> Option<Facing> {
        match c {
            '.' => None,
            '>' => Some(Facing::East),
            'v' => Some(Facing::South),
            _ => panic!("Unknown char: {:?}", c),
        }
    }

    fn run(&mut self) -> usize {
        let mut ticks = 0;
        loop {
            ticks += 1;
            let num_moves = self.tick();
            // println!("map after tick {}:\n{}", ticks, self);

            if num_moves == 0 {
                break;
            }
        }
        ticks
    }

    fn tick(&mut self) -> usize {
        self.move_cucumbers(&Facing::East) + self.move_cucumbers(&Facing::South)
    }

    fn move_cucumbers(&mut self, facing_to_move: &Facing) -> usize {
        let mut changes = vec![];
        for (current_position, value) in self.cucumbers.iter().enumerate() {
            match value {
                Some(facing) if facing == facing_to_move => {
                    // try_to_move
                    let new_position = self.try_to_move(current_position, facing);
                    if new_position != current_position {
                        changes.push((current_position, new_position));
                    }
                }
                _ => {}
            }
        }

        let num_changes = changes.len();
        for (old_position, new_position) in changes {
            self.cucumbers.swap(old_position, new_position);
        }
        num_changes
    }

    fn try_to_move(&self, position: usize, facing: &Facing) -> usize {
        let neighbour_pos = self.neighbour_position(position, facing);
        if self.cucumbers[neighbour_pos].is_some() {
            // occupied, stay in current position
            position
        } else {
            // empty, move to new position
            neighbour_pos
        }
    }

    fn neighbour_position(&self, coord: usize, facing: &Facing) -> usize {
        match facing {
            Facing::East => {
                let x = coord % self.width;
                let new_x = (x + 1) % self.width;
                coord + new_x - x
            }
            Facing::South => {
                let x = coord % self.width;
                let y = coord / self.width;
                let new_y = (y + 1) % self.height;
                new_y * self.width + x
            }
        }
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let coord = self.width * y + x;
                match self.cucumbers[coord] {
                    Some(Facing::East) => write!(f, ">")?,
                    Some(Facing::South) => write!(f, "v")?,
                    None => write!(f, ".")?,
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Facing {
    East,
    South,
}

fn part1(input: &str) -> usize {
    let mut map = Map::parse(input);
    println!("initial:\n{}", map);
    map.run()
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    static EXAMPLE1: &str = indoc! {"
        v...>>.vv>
        .vv>>.vv..
        >>.>v>...v
        >>v>>.>.v.
        v>v.vv.v..
        >.>>..v...
        .vv..>.>v.
        v.v..>>v.v
        ....v..v.>
    "};

    #[test]
    fn test_part1_example1() {
        let result = part1(EXAMPLE1);
        assert_eq!(result, 58);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file());
        assert_eq!(result, 308);
    }
}
