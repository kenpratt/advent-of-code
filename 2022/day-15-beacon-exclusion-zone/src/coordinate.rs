use std::ops::Add;

#[derive(Clone, Copy, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

impl Coordinate {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn parse(s: &str) -> Self {
        let parts: Vec<&str> = s.split(",").map(|c| c.trim()).collect();
        assert_eq!(parts.len(), 2);
        let x = parts[0].parse::<i32>().unwrap();
        let y = parts[1].parse::<i32>().unwrap();
        Self { x, y }
    }

    pub fn line_to(&self, other: &Self) -> Vec<Self> {
        let step = Self {
            x: (other.x - self.x).signum(),
            y: (other.y - self.y).signum(),
        };
        let mut res = vec![self.clone()];
        let mut curr = self.clone();

        while &curr != other {
            curr = curr + step;
            res.push(curr);
        }

        res
    }

    pub fn manhattan_distance(&self, other: &Coordinate) -> i32 {
        abs_diff(self.x, other.x) + abs_diff(self.y, other.y)
    }
}

impl Add for Coordinate {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

pub fn abs_diff(a: i32, b: i32) -> i32 {
    if a >= b {
        a - b
    } else {
        b - a
    }
}
