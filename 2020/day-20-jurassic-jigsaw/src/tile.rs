use lazy_static::lazy_static;
use regex::Regex;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum Side {
    Top,
    Right,
    Bottom,
    Left,
}

impl Side {
    pub fn calculate_rotation(&self, other_side: &Side) -> usize {
        (4 + other_side.value() - self.value()) % 4
    }

    fn value(&self) -> usize {
        match *self {
            Side::Top => 0,
            Side::Left => 1, // 1 rotation clockwise to make left=top
            Side::Bottom => 2,
            Side::Right => 3, // 3 rotations clockwise to make right=top
        }
    }

    pub fn rotate(&self, amount: usize) -> Side {
        if amount > 0 {
            let side_val = (self.value() + amount) % 4;
            match side_val {
                0 => Side::Top,
                1 => Side::Left,
                2 => Side::Bottom,
                3 => Side::Right,
                _ => panic!("Unreachable"),
            }
        } else {
            *self
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum Direction {
    Clockwise,
    Counterclockwise,
}

type Pixels = Vec<Vec<bool>>;

#[derive(Debug)]
pub struct Tile {
    pub id: usize,
    pixels: Pixels,
    pub width: usize,
    pub height: usize,
}

impl Tile {
    pub fn parse(input: &str) -> Tile {
        let mut lines = input.lines();
        let id = Tile::parse_id(lines.next().unwrap());
        let pixels = Tile::parse_pixels(lines);
        Tile::from_pixels(id, pixels)
    }

    fn from_pixels(id: usize, pixels: Pixels) -> Tile {
        let width = pixels[0].len();
        let height = pixels.len();
        Tile {
            id: id,
            pixels: pixels,
            width: width,
            height: height,
        }
    }

    fn parse_id(input: &str) -> usize {
        lazy_static! {
            static ref ID_RE: Regex = Regex::new(r"\ATile (\d+):\z").unwrap();
        }
        let captures = ID_RE.captures(input).unwrap();
        captures.get(1).unwrap().as_str().parse::<usize>().unwrap()
    }

    fn parse_pixels(lines: std::str::Lines) -> Pixels {
        lines.map(|line| line.chars().map(|c| Tile::parse_pixel(&c)).collect()).collect()
    }

    fn parse_pixel(c: &char) -> bool {
        match c {
            '#' => true,
            '.' => false,
            _ => panic!("Unsupported pixel char: {}", c),
        }
    }

    fn pixel_to_char(b: &bool) -> char {
        match b {
            true => '#',
            false => '.',
        }
    }

    pub fn to_string(&self) -> String {
        Tile::pixels_to_string(&self.pixels)
    }

    fn pixels_to_string(pixels: &Pixels) -> String {
        let row_strings: Vec<String> = pixels.iter().map(|row| {
            row.iter().map(|p| Tile::pixel_to_char(p)).collect()
        }).collect();
        row_strings.join("\n")
    }

    pub fn top(&self) -> &Vec<bool> {
        &self.pixels[0]
    }

    pub fn bottom(&self) -> &Vec<bool> {
        &self.pixels[self.height - 1]
    }
    
    pub fn left(&self) -> Vec<bool> {
        (0..self.height).map(|y| self.pixels[y][0]).collect()
    }
    
    pub fn right(&self) -> Vec<bool> {
        (0..self.height).map(|y| self.pixels[y][self.width - 1]).collect()
    }

    pub fn line_to_int<'a>(line: impl Iterator<Item=&'a bool>) -> usize {
        line.map(|b| if *b { 1 } else { 0 }).fold(0, |acc, bit| (acc << 1) ^ bit)
    }

    pub fn merge(tiles: &Vec<Vec<(&Tile, Direction, usize)>>) -> Tile {
        let pixels: Pixels = tiles.iter().map(|row| {
            Tile::merge_pixels_in_row(row)
        }).flatten().collect();

        println!("merged pixels: {:?}", pixels);
        Tile::from_pixels(0, pixels)
    }

    fn merge_pixels_in_row(tiles: &Vec<(&Tile, Direction, usize)>) -> Pixels {
        let pixels_per_tile: Vec<Pixels> = tiles.iter().map(|(tile, direction, rotation)| {
            tile.calculate_pixels(direction, rotation)
        }).collect();

        let mut combined = vec![vec![]; pixels_per_tile[0].len()];
        for pixels in pixels_per_tile {
            for (i, row) in pixels.iter().enumerate() {
                combined[i].extend(row);
            }
        }
        combined
    }

    fn calculate_pixels(&self, direction: &Direction, rotation: &usize) -> Pixels {
        let coords = Tile::calculate_coords_for_direction_and_rotation(direction, rotation, self.width);
        println!("calculate_pixels {:?} {:?} -> {:?}", direction, rotation, coords);

        // trim borders
        coords[1..(self.width-1)].iter().map(|row| {
            row[1..(self.width-1)].iter().map(|(x, y)| {
                self.pixels[*y][*x]
            }).collect()
        }).collect()
    }

    fn calculate_coords_for_direction_and_rotation(direction: &Direction, rotation: &usize, width: usize) -> Vec<Vec<(usize, usize)>> {
        let (swap_x_and_y, reverse_x, reverse_y) = match (direction, rotation) {
            (&Direction::Clockwise, &0) => (false, false, false),
            (&Direction::Clockwise, &1) => (true, false, true),
            (&Direction::Clockwise, &2) => (false, true, true),
            (&Direction::Clockwise, &3) => (true, true, false),

            (&Direction::Counterclockwise, &0) => (false, true, false),
            (&Direction::Counterclockwise, &1) => (true, true, true),
            (&Direction::Counterclockwise, &2) => (false, false, true),
            (&Direction::Counterclockwise, &3) => (true, false, false),

            _ => panic!("Unreachable"),
        };

        let x_vals: Vec<usize> = if reverse_x { (0..width).rev().collect() } else { (0..width).collect() };
        let y_vals: Vec<usize> = if reverse_y { (0..width).rev().collect() } else { (0..width).collect() };

        if swap_x_and_y {
            x_vals.iter().map(|x| {
                y_vals.iter().map(|y| {
                    (*x, *y)
                }).collect()
            }).collect()
        } else {
            y_vals.iter().map(|y| {
                x_vals.iter().map(|x| {
                    (*x, *y)
                }).collect()
            }).collect()
        }
    }
}