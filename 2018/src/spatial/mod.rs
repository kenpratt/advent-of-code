use std::{
    fmt::Debug,
    ops::{Add, Sub},
    str::FromStr,
};

use num::{one, PrimInt};

macro_rules! cast {
    ($val:expr) => {
        num::cast($val).unwrap()
    };
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
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
        let mut parts = input.split(separator).map(|s| s.parse::<T>().unwrap());
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

pub struct Bounds<T> {
    pub left: T,
    pub right: T,
    pub top: T,
    pub bottom: T,
}

impl<T> Bounds<T>
where
    T: Copy + PrimInt,
{
    pub fn calculate(coords: &[Coord<T>]) -> Self {
        let left = coords.iter().map(|c| c.x).min().unwrap();
        let right = coords.iter().map(|c| c.x).max().unwrap();
        let top = coords.iter().map(|c| c.y).min().unwrap();
        let bottom = coords.iter().map(|c| c.y).max().unwrap();
        Self {
            left,
            right,
            top,
            bottom,
        }
    }

    pub fn width(&self) -> T {
        self.right - self.left + one()
    }

    pub fn height(&self) -> T {
        self.bottom - self.top + one()
    }
}

pub struct Grid<'a, T, V> {
    bounds: &'a Bounds<T>,
    width: usize,
    height: usize,
    cells: Vec<Option<V>>,
}

impl<'a, T, V> Grid<'a, T, V>
where
    T: FromStr + PrimInt,
    <T as FromStr>::Err: Debug,
{
    pub fn new(bounds: &'a Bounds<T>) -> Self {
        let width = cast!(bounds.width());
        let height = cast!(bounds.height());

        let mut cells = vec![];
        cells.resize_with(width * height, Default::default);

        Self {
            bounds,
            width,
            height,
            cells,
        }
    }

    pub fn coord_to_index(&self, coord: &Coord<T>) -> usize {
        let x: usize = cast!(coord.x - self.bounds.left);
        let y: usize = cast!(coord.y - self.bounds.top);
        y * self.width + x
    }

    pub fn index_to_coord(&self, index: usize) -> Coord<T> {
        let dx = index % self.width;
        let dy = index / self.width;
        let x = self.bounds.left + cast!(dx);
        let y = self.bounds.top + cast!(dy);
        Coord::new(x, y)
    }

    pub fn get(&self, index: usize) -> &Option<V> {
        &self.cells[index]
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
