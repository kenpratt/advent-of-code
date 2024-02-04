use crate::file::*;
use crate::spatial::*;

pub fn run() {
    let input = parse(&read_input_file!());
    println!("part 1 result: {:?}", part1(input.clone()));
    println!("part 2 result: {:?}", part2(input));
}

#[derive(Clone, Debug)]
struct State {
    grid: Grid<usize, (Track, Option<Cart>)>,
}

impl State {
    fn parse(input: &str) -> Self {
        let grid = Grid::parse(input, |c| match c {
            '|' => Some((Track::Vertical, None)),
            '-' => Some((Track::Horizontal, None)),
            '/' => Some((Track::CurveRight, None)),
            '\\' => Some((Track::CurveLeft, None)),
            '+' => Some((Track::Intersection, None)),
            '^' => Some((Track::Vertical, Some(Cart::new(Direction::North)))),
            'v' => Some((Track::Vertical, Some(Cart::new(Direction::South)))),
            '<' => Some((Track::Horizontal, Some(Cart::new(Direction::West)))),
            '>' => Some((Track::Horizontal, Some(Cart::new(Direction::East)))),
            ' ' => None,
            _ => panic!("Unexpected input char: {:?}", c),
        });
        Self { grid }
    }

    fn first_collision(&mut self) -> Coord<usize> {
        loop {
            match self.tick() {
                Some(coord) => return coord,
                None => (),
            }
        }
    }

    fn tick(&mut self) -> Option<Coord<usize>> {
        let carts: Vec<(usize, Track, Cart)> = self
            .grid
            .iter()
            .filter(|(_index, (_track, maybe_cart))| maybe_cart.is_some())
            .map(|(index, (track, cart))| (index, *track, cart.unwrap()))
            .collect();

        for (index, track, cart) in carts {
            let pos = self.grid.index_to_coord(index);
            let (next_cart, next_pos) = cart.tick(&pos, &track);
            let next_index = self.grid.coord_to_index(&next_pos);

            if self.grid.get(next_index).unwrap().1.is_some() {
                // collision!
                return Some(next_pos);
            } else {
                // no collision, move the cart to the new location
                self.set_cart(index, None);
                self.set_cart(next_index, Some(next_cart));
            }
        }

        // no collision
        None
    }

    fn set_cart(&mut self, index: usize, cart: Option<Cart>) {
        match self.grid.get_mut(index) {
            Some((_track, maybe_cart)) => *maybe_cart = cart,
            None => panic!("expected a track at index {}", index),
        }
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

fn parse(input: &str) -> State {
    State::parse(input)
}

fn part1(mut state: State) -> Coord<usize> {
    state.first_collision()
}

fn part2(_state: State) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        let result = part1(parse(&read_example_file!()));
        assert_eq!(result, Coord::new(7, 3));
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(parse(&read_input_file!()));
        assert_eq!(result, Coord::new(123, 18));
    }

    #[test]
    fn test_part2_example() {
        let result = part2(parse(&read_example_file!()));
        assert_eq!(result, 0);
    }

    #[test]
    fn test_part2_solution() {
        let result = part2(parse(&read_input_file!()));
        assert_eq!(result, 0);
    }
}
