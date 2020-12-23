use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
pub struct Tile {
    pub id: usize,
    pixels: Vec<Vec<bool>>,
    pub width: usize,
    pub height: usize,
}

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

impl Tile {
    pub fn parse(input: &str) -> Tile {
        let mut lines = input.lines();
        let id = Tile::parse_id(lines.next().unwrap());
        let pixels = Tile::parse_pixels(lines);
        let width = pixels[0].len();
        let height = pixels.len();
        return Tile {
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

    fn parse_pixels(lines: std::str::Lines) -> Vec<Vec<bool>> {
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
        let row_strings: Vec<String> = self.pixels.iter().map(|row| {
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
}