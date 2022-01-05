use std::collections::HashSet;
use std::fmt;
use std::fs;
use std::ops::RangeInclusive;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    println!("part 2 result: {:?}", part2(&read_input_file()));
}

fn read_input_file() -> String {
    fs::read_to_string("input.txt").expect("Something went wrong reading the file")
}

fn parse_line(line: &str) -> Vec<bool> {
    line.chars().map(|c| parse_char(&c)).collect()
}

fn parse_char(c: &char) -> bool {
    match c {
        '#' => true,
        '.' => false,
        _ => panic!("Unexpected char: {}", c),
    }
}

fn val_to_char(v: &bool) -> char {
    match v {
        true => '#',
        false => '.',
    }
}

#[derive(Debug)]
struct Map {
    algorithm: Algorithm,
    image: Image,
}

impl Map {
    fn parse(input: &str) -> Map {
        let parts: Vec<&str> = input.split("\n\n").collect();
        assert_eq!(parts.len(), 2);
        let algorithm = Algorithm::parse(parts[0]);
        let value_outside_bounds = false; // starts dark
        let image = Image::parse(parts[1], value_outside_bounds);
        Map { algorithm, image }
    }

    fn enhance(&mut self) {
        self.image = self.image.enhance(&self.algorithm);
    }

    fn enhance_times(&mut self, times: usize, print_each_iteration: bool) {
        println!("before:\n{}", self);
        for n in 1..=times {
            self.enhance();
            if print_each_iteration {
                println!("enhance {}:\n{}", n, self);
            }
        }
        println!("after:\n{}", self);
    }

    fn count_active(&self) -> usize {
        self.image.active.len()
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.image)
    }
}

#[derive(Debug)]
struct Algorithm {
    spec: Vec<bool>,
}

impl Algorithm {
    fn parse(input: &str) -> Algorithm {
        let spec = parse_line(input);
        assert_eq!(spec.len(), 512);
        Algorithm { spec }
    }

    fn enhance(&self, val: usize) -> bool {
        self.spec[val]
    }

    fn value_outside_bounds(&self, current: bool) -> bool {
        match current {
            false => self.spec[0],  // 000000000
            true => self.spec[511], // 111111111
        }
    }
}

#[derive(Debug)]
struct Image {
    active: HashSet<Coord>,
    bounds: Bounds,
    value_outside_bounds: bool,
}

type Coord = (isize, isize);

fn tile(coord: &Coord) -> Vec<Coord> {
    vec![
        (coord.0 - 1, coord.1 - 1),
        (coord.0, coord.1 - 1),
        (coord.0 + 1, coord.1 - 1),
        (coord.0 - 1, coord.1),
        (coord.0, coord.1),
        (coord.0 + 1, coord.1),
        (coord.0 - 1, coord.1 + 1),
        (coord.0, coord.1 + 1),
        (coord.0 + 1, coord.1 + 1),
    ]
}

impl Image {
    fn parse(input: &str, value_outside_bounds: bool) -> Image {
        let mut active = HashSet::new();
        for (y, line) in input.lines().enumerate() {
            for (x, val) in parse_line(line).iter().enumerate() {
                if *val {
                    active.insert((x as isize, y as isize));
                }
            }
        }
        let bounds = Bounds::calculate(&active);
        Image {
            active,
            bounds,
            value_outside_bounds,
        }
    }

    fn value(&self, coord: &Coord) -> bool {
        if self.bounds.inside(coord) {
            self.active.contains(coord)
        } else {
            self.value_outside_bounds
        }
    }

    fn enhance(&self, algorithm: &Algorithm) -> Image {
        let (x_range, y_range) = self.bounds.expanded().ranges();

        let mut new_active = HashSet::new();
        for y in y_range {
            for x in x_range.clone() {
                let coord = (x, y);
                let input = self.enhance_value_for_coord(&coord);
                let val = algorithm.enhance(input);
                if val {
                    new_active.insert(coord);
                }
            }
        }

        let new_bounds = Bounds::calculate(&new_active);
        let new_value_outside_bounds = algorithm.value_outside_bounds(self.value_outside_bounds);
        Image {
            active: new_active,
            bounds: new_bounds,
            value_outside_bounds: new_value_outside_bounds,
        }
    }

    fn enhance_value_for_coord(&self, coord: &Coord) -> usize {
        tile(coord)
            .iter()
            .map(|c| self.value(c))
            .fold(0, |acc, v| (acc << 1) | v as usize)
    }
}

impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (x_range, y_range) = self.bounds.expanded().ranges();
        for y in y_range {
            for x in x_range.clone() {
                let v = self.value(&(x, y));
                write!(f, "{}", val_to_char(&v))?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[derive(Debug)]
struct Bounds {
    min_x: isize,
    max_x: isize,
    min_y: isize,
    max_y: isize,
}

impl Bounds {
    fn calculate(coords: &HashSet<Coord>) -> Bounds {
        let min_x = coords.iter().map(|c| c.0).min().unwrap();
        let max_x = coords.iter().map(|c| c.0).max().unwrap();
        let min_y = coords.iter().map(|c| c.1).min().unwrap();
        let max_y = coords.iter().map(|c| c.1).max().unwrap();
        Bounds {
            min_x,
            max_x,
            min_y,
            max_y,
        }
    }

    fn expanded(&self) -> Bounds {
        // pad bounds by 2, since the empty space around the image up to 2
        // cells away from an active cell could be lit (3x3 tile).
        let min_x = self.min_x - 2;
        let max_x = self.max_x + 2;
        let min_y = self.min_y - 2;
        let max_y = self.max_y + 2;
        Bounds {
            min_x,
            max_x,
            min_y,
            max_y,
        }
    }

    fn ranges(self) -> (RangeInclusive<isize>, RangeInclusive<isize>) {
        let x_range = (self.min_x)..=(self.max_x);
        let y_range = (self.min_y)..=(self.max_y);
        (x_range, y_range)
    }

    fn inside(&self, coord: &Coord) -> bool {
        coord.0 >= self.min_x
            && coord.0 <= self.max_x
            && coord.1 >= self.min_y
            && coord.1 <= self.max_y
    }
}

fn part1(input: &str) -> usize {
    let mut map = Map::parse(input);
    map.enhance_times(2, true);
    map.count_active()
}

fn part2(input: &str) -> usize {
    let mut map = Map::parse(input);
    map.enhance_times(50, false);
    map.count_active()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read_example_file() -> String {
        fs::read_to_string("example.txt").expect("Something went wrong reading the file")
    }

    #[test]
    fn test_part1_example1() {
        let result = part1(&read_example_file());
        assert_eq!(result, 35);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file());
        assert_eq!(result, 5218);
    }

    #[test]
    fn test_part2_example1() {
        let result = part2(&read_example_file());
        assert_eq!(result, 3351);
    }

    #[test]
    fn test_part2_solution() {
        let result = part2(&read_input_file());
        assert_eq!(result, 15527);
    }
}
