use std::collections::BTreeMap;

use crate::{interface::AoC, spatial::*};

pub struct Day;
impl AoC<State, Coord<usize>, Coord<usize>> for Day {
    const FILE: &'static str = file!();

    fn parse(input: String) -> State {
        State::parse(&input)
    }

    fn part1(state: &State) -> Coord<usize> {
        state.clone().first_collision()
    }

    fn part2(state: &State) -> Coord<usize> {
        state.clone().run_removing_collisions()
    }
}

#[derive(Clone, Debug)]
pub struct State {
    grid: Grid<usize, Track>,
    carts: BTreeMap<usize, Cart>,
}

impl State {
    fn parse(input: &str) -> Self {
        let mut carts_tmp = vec![];

        let grid = Grid::parse(input, |c, pos| {
            let (maybe_track, maybe_cart) = Self::parse_char(c);
            match maybe_cart {
                Some(cart) => carts_tmp.push((*pos, cart)),
                None => (),
            };
            maybe_track
        });

        let carts: BTreeMap<usize, Cart> = carts_tmp
            .into_iter()
            .map(|(pos, cart)| (grid.coord_to_index(&pos), cart))
            .collect();

        Self { grid, carts }
    }

    fn parse_char(c: &char) -> (Option<Track>, Option<Cart>) {
        match c {
            '|' => (Some(Track::Vertical), None),
            '-' => (Some(Track::Horizontal), None),
            '/' => (Some(Track::CurveRight), None),
            '\\' => (Some(Track::CurveLeft), None),
            '+' => (Some(Track::Intersection), None),
            '^' => (Some(Track::Vertical), Some(Cart::new(Direction::North))),
            'v' => (Some(Track::Vertical), Some(Cart::new(Direction::South))),
            '<' => (Some(Track::Horizontal), Some(Cart::new(Direction::West))),
            '>' => (Some(Track::Horizontal), Some(Cart::new(Direction::East))),
            ' ' => (None, None),
            _ => panic!("Unexpected input char: {:?}", c),
        }
    }

    fn first_collision(&mut self) -> Coord<usize> {
        loop {
            let collisions = self.tick();
            if !collisions.is_empty() {
                return collisions[0];
            }
        }
    }

    fn run_removing_collisions(&mut self) -> Coord<usize> {
        while self.carts.len() > 1 {
            self.tick();
        }

        let (index, _cart) = self.carts.pop_first().unwrap();
        self.grid.index_to_coord(index)
    }

    fn tick(&mut self) -> Vec<Coord<usize>> {
        let cart_indices: Vec<usize> = self.carts.keys().cloned().collect();

        let mut collisions: Vec<Coord<usize>> = vec![];

        for index in cart_indices {
            let cart = match self.carts.get(&index) {
                Some(c) => c,
                None => {
                    // must have been deleted in a collision
                    continue;
                }
            };

            let track = self.grid.get(index).unwrap();
            let pos = self.grid.index_to_coord(index);

            let (next_cart, next_pos) = cart.tick(&pos, &track);
            let next_index = self.grid.coord_to_index(&next_pos);

            // remove the cart from the old location
            self.carts.remove(&index);

            if self.carts.remove(&next_index).is_some() {
                // collision! delete the cart in the colliding location,
                // and delete the current cart by neglecting to add it back
                collisions.push(next_pos);
            } else {
                // no collision, move the cart to the new location
                self.carts.insert(next_index, next_cart);
            }
        }

        collisions
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Track {
    Vertical,
    Horizontal,
    CurveRight,
    CurveLeft,
    Intersection,
}

#[derive(Clone, Copy, Debug)]
struct Cart {
    facing: Direction,
    intersections: usize,
}

impl Cart {
    fn new(facing: Direction) -> Self {
        Self {
            facing,
            intersections: 0,
        }
    }

    fn tick(&self, pos: &Coord<usize>, track: &Track) -> (Self, Coord<usize>) {
        let move_direction = match track {
            Track::Vertical | Track::Horizontal => {
                // continue in same direction
                self.facing
            }
            Track::CurveLeft => match self.facing {
                Direction::North => Direction::West,
                Direction::South => Direction::East,
                Direction::West => Direction::North,
                Direction::East => Direction::South,
            },
            Track::CurveRight => match self.facing {
                Direction::North => Direction::East,
                Direction::South => Direction::West,
                Direction::West => Direction::South,
                Direction::East => Direction::North,
            },
            Track::Intersection => {
                // depends on number of intersections we've seen
                match self.intersections % 3 {
                    0 => self.facing.counterclockwise(), // turn left
                    1 => self.facing,                    // go straight
                    2 => self.facing.clockwise(),        // turn right
                    _ => panic!("Unreachable"),
                }
            }
        };

        let next_pos = pos.shift(&move_direction);
        let next_cart = Cart {
            facing: move_direction,
            intersections: self.intersections + if *track == Track::Intersection { 1 } else { 0 },
        };
        (next_cart, next_pos)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_FILE_2: &'static str = "example2.txt";

    #[test]
    fn test_part1_example() {
        let result = Day::part1(&Day::parse_example_file());
        assert_eq!(result, Coord::new(7, 3));
    }

    #[test]
    fn test_part1_solution() {
        let result = Day::part1(&Day::parse_input_file());
        assert_eq!(result, Coord::new(123, 18));
    }

    #[test]
    fn test_part2_example() {
        let result = Day::part2(&Day::parse_file(EXAMPLE_FILE_2));
        assert_eq!(result, Coord::new(6, 4));
    }

    #[test]
    fn test_part2_solution() {
        let result = Day::part2(&Day::parse_input_file());
        assert_eq!(result, Coord::new(71, 123));
    }
}
