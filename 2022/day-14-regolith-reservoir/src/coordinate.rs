use std::ops::Add;

#[derive(Clone, Copy, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub struct Coordinate {
    pub x: isize,
    pub y: isize,
}

impl Coordinate {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn parse(s: &str) -> Self {
        let parts: Vec<&str> = s.split(",").map(|c| c.trim()).collect();
        assert_eq!(parts.len(), 2);
        let x = parts[0].parse::<isize>().unwrap();
        let y = parts[1].parse::<isize>().unwrap();
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
