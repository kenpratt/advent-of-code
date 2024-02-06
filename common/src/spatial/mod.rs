use std::{
    cmp::Ordering,
    fmt::Debug,
    ops::{Add, Div, Mul, Sub},
    str::FromStr,
};

use num::{one, PrimInt};

macro_rules! cast {
    ($val:expr) => {
        num::cast($val).unwrap()
    };
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Coord<T> {
    pub x: T,
    pub y: T,
}

impl<T> Coord<T>
where
    T: PrimInt + FromStr,
    <T as FromStr>::Err: Debug,
{
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn parse(input: &str, separator: &str) -> Self {
        let mut parts = input
            .split(separator)
            .map(|s| s.trim().parse::<T>().unwrap());
        let x = parts.next().unwrap();
        let y = parts.next().unwrap();
        assert!(parts.next().is_none());
        Self::new(x, y)
    }

    pub fn manhattan_distance(&self, other: &Coord<T>) -> T {
        Self::abs_diff(self.x, other.x) + Self::abs_diff(self.y, other.y)
    }

    fn abs_diff(a: T, b: T) -> T {
        if a <= b {
            b - a
        } else {
            a - b
        }
    }

    pub fn shift(&self, direction: &Direction) -> Self {
        match direction {
            Direction::North => Coord::new(self.x, self.y - T::one()),
            Direction::South => Coord::new(self.x, self.y + T::one()),
            Direction::West => Coord::new(self.x - T::one(), self.y),
            Direction::East => Coord::new(self.x + T::one(), self.y),
        }
    }
}

impl<T> Ord for Coord<T>
where
    T: PrimInt,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.y.cmp(&other.y).then(self.x.cmp(&other.x))
    }
}

impl<T> PartialOrd for Coord<T>
where
    T: PrimInt,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Add for Coord<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T> Sub for Coord<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T, N> Mul<N> for Coord<T>
where
    T: Mul<N, Output = T>,
    N: Copy,
{
    type Output = Coord<T>;
    fn mul(self, rhs: N) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T, N> Div<N> for Coord<T>
where
    T: Div<N, Output = T>,
    N: Copy,
{
    type Output = Coord<T>;
    fn div(self, rhs: N) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    pub fn clockwise(&self) -> Self {
        use Direction::*;

        match self {
            North => East,
            South => West,
            West => North,
            East => South,
        }
    }

    pub fn counterclockwise(&self) -> Self {
        use Direction::*;

        match self {
            North => West,
            South => East,
            West => South,
            East => North,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Bounds<T> {
    pub left: T,
    pub right: T,
    pub top: T,
    pub bottom: T,
    pub width: T,
    pub height: T,
}

impl<T> Bounds<T>
where
    T: Copy + FromStr + PrimInt,
    <T as FromStr>::Err: Debug,
{
    pub fn new(left: T, right: T, top: T, bottom: T) -> Self {
        let width = right - left + one();
        let height = bottom - top + one();
        Self {
            left,
            right,
            top,
            bottom,
            width,
            height,
        }
    }

    pub fn calculate(coords: &[Coord<T>]) -> Self {
        let left = coords.iter().map(|c| c.x).min().unwrap();
        let right = coords.iter().map(|c| c.x).max().unwrap();
        let top = coords.iter().map(|c| c.y).min().unwrap();
        let bottom = coords.iter().map(|c| c.y).max().unwrap();
        Self::new(left, right, top, bottom)
    }

    pub fn coord_to_index<I: PrimInt>(&self, coord: &Coord<T>) -> I {
        let x: I = cast!(coord.x - self.left);
        let y: I = cast!(coord.y - self.top);
        y * cast!(self.width) + x
    }

    pub fn index_to_coord<I: PrimInt>(&self, index: I) -> Coord<T> {
        let dx = index % cast!(self.width);
        let dy = index / cast!(self.width);
        let x = self.left + cast!(dx);
        let y = self.top + cast!(dy);
        Coord::new(x, y)
    }

    pub fn size(&self) -> T {
        self.width * self.height
    }
}

#[derive(Clone, Debug)]
pub struct Grid<T, V> {
    bounds: Bounds<T>,
    width: usize,
    height: usize,
    cells: Vec<Option<V>>,
}

impl<T, V> Grid<T, V>
where
    T: FromStr + PrimInt,
    <T as FromStr>::Err: Debug,
{
    pub fn parse<F>(input: &str, mut parse_val: F) -> Self
    where
        F: FnMut(&char, &Coord<T>) -> Option<V>,
    {
        let mut values: Vec<(Coord<T>, V)> = vec![];

        let left: T = T::zero();
        let mut right = T::zero();
        let top = T::zero();
        let mut bottom = T::zero();

        for (yu, line) in input.lines().enumerate() {
            let y = T::from(yu).unwrap();
            bottom = y;

            for (xu, c) in line.chars().enumerate() {
                let x = T::from(xu).unwrap();
                right = x;

                let coord = Coord::new(x, y);
                match parse_val(&c, &coord) {
                    Some(val) => {
                        values.push((coord, val));
                    }
                    None => (),
                }
            }
        }

        let bounds: Bounds<T> = Bounds::new(left, right, top, bottom);

        let mut grid = Self::new(bounds);

        for (coord, val) in values {
            let i = grid.coord_to_index(&coord);
            grid.set(i, val);
        }

        grid
    }

    pub fn new(bounds: Bounds<T>) -> Self {
        Self::new_with_intial(bounds, Default::default)
    }

    pub fn new_with_intial<F>(bounds: Bounds<T>, initial: F) -> Self
    where
        F: FnMut() -> Option<V>,
    {
        let width = cast!(bounds.width);
        let height = cast!(bounds.height);
        let len = width * height;

        let mut cells = vec![];
        cells.resize_with(len, initial);

        Self {
            bounds,
            width,
            height,
            cells,
        }
    }

    pub fn coord_to_index(&self, coord: &Coord<T>) -> usize {
        self.bounds.coord_to_index(coord)
    }

    pub fn index_to_coord(&self, index: usize) -> Coord<T> {
        self.bounds.index_to_coord(index)
    }

    pub fn get(&self, index: usize) -> &Option<V> {
        &self.cells[index]
    }

    pub fn get_mut(&mut self, index: usize) -> &mut Option<V> {
        &mut self.cells[index]
    }

    pub fn set(&mut self, index: usize, val: V) {
        self.cells[index] = Some(val);
    }

    pub fn iter(&self) -> impl Iterator<Item = (usize, &V)> {
        self.cells
            .iter()
            .enumerate()
            .filter_map(|(i, maybe_val)| maybe_val.as_ref().map(|val| (i, val)))
    }

    pub fn into_iter(self) -> impl Iterator<Item = (usize, V)> {
        self.cells
            .into_iter()
            .enumerate()
            .filter_map(|(i, maybe_val)| maybe_val.map(|val| (i, val)))
    }

    pub fn print<F>(&self, print_val: F)
    where
        F: Fn(&Option<V>),
        std::ops::RangeInclusive<T>: Iterator<Item = T>,
    {
        for y in self.bounds.top..=self.bounds.bottom {
            for x in self.bounds.left..=self.bounds.right {
                let val = self.get(self.coord_to_index(&Coord::new(x, y)));
                print_val(val);
            }
            println!();
        }
    }

    pub fn neighbours(&self, index: usize) -> [Option<usize>; 4] {
        let x = index % self.width;
        let y = index / self.width;

        let mut out = [None; 4];

        if x > 0 {
            out[0] = Some(index - 1);
        }
        if x < self.width - 1 {
            out[1] = Some(index + 1);
        }
        if y > 0 {
            out[2] = Some(index - self.width);
        }
        if y < self.height - 1 {
            out[3] = Some(index + self.width);
        }

        out
    }
}
