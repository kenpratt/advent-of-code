use std::collections::HashMap;
use std::fmt;
use std::fs;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    // println!("part 2 result: {:?}", part2(&read_input_file()));
}

fn read_input_file() -> String {
    fs::read_to_string("input.txt").expect("Something went wrong reading the file")
}

#[derive(Debug)]
struct Map {
    cucumbers: HashMap<Coordinate, Facing>,
    width: usize,
    height: usize,
}

impl Map {
    fn parse(input: &str) -> Map {
        let width = input.lines().next().unwrap().chars().count();
        let height = input.lines().count();
        let mut cucumbers = HashMap::new();
        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                match Self::parse_char(&c) {
                    Some(facing) => {
                        cucumbers.insert(Coordinate::new(x, y), facing);
                    }
                    None => {}
                };
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
        let mut new_cucumbers = HashMap::new();
        let mut num_moves = 0;

        for (coord, facing) in &self.cucumbers {
            if facing == facing_to_move {
                // try_to_move
                let new_position = self.try_to_move(coord, facing);
                if new_position != *coord {
                    num_moves += 1;
                }
                new_cucumbers.insert(new_position, *facing);
            } else {
                // not your turn
                new_cucumbers.insert(*coord, *facing);
            }
        }

        self.cucumbers = new_cucumbers;
        num_moves
    }

    fn try_to_move(&self, position: &Coordinate, facing: &Facing) -> Coordinate {
        let neighbour_pos = self.neighbour_position(position, facing);
        if self.cucumbers.contains_key(&neighbour_pos) {
            // occupied, stay in current position
            *position
        } else {
            // empty, move to new position
            neighbour_pos
        }
    }

    fn neighbour_position(&self, coord: &Coordinate, facing: &Facing) -> Coordinate {
        match facing {
            Facing::East => Coordinate::new((coord.x + 1) % self.width, coord.y),
            Facing::South => Coordinate::new(coord.x, (coord.y + 1) % self.height),
        }
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let coord = Coordinate::new(x, y);
                match self.cucumbers.get(&coord) {
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

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    fn new(x: usize, y: usize) -> Coordinate {
        Coordinate { x, y }
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

// fn part2(input: &str) -> usize {
//     let data = Data::parse(input);
//     println!("{:?}", data);
//     data.execute()
// }

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

    // #[test]
    // fn test_part2_example1() {
    //     let result = part2(EXAMPLE1);
    //     assert_eq!(result, 0);
    // }

    // #[test]
    // fn test_part2_solution() {
    //     let result = part2(&read_input_file());
    //     assert_eq!(result, 0);
    // }
}
