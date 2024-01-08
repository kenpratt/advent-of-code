use std::{
    fs,
    ops::{Add, RangeInclusive, Sub},
};

use itertools::Itertools;
use lazy_static::lazy_static;
use nalgebra::{matrix, vector};
use regex::Regex;

const PART1_RANGE: RangeInclusive<i64> = 200000000000000..=400000000000000;

fn main() {
    println!(
        "part 1 result: {:?}",
        part1(&read_input_file(), PART1_RANGE)
    );
    println!("part 2 result: {:?}", part2(&read_input_file()));
}

fn read_input_file() -> String {
    fs::read_to_string("input.txt").expect("Something went wrong reading the file")
}

#[derive(Debug)]
struct Hailstone {
    position: Coord,
    velocity: Coord,
}

impl Hailstone {
    fn parse_list(input: &str) -> Vec<Self> {
        input.lines().map(|line| Self::parse(line)).collect()
    }

    fn parse(input: &str) -> Self {
        lazy_static! {
            static ref HAILSTONE_RE: Regex = Regex::new(r"\A(.+) @ (.+)\z").unwrap();
        }

        let caps = HAILSTONE_RE.captures(input).unwrap();
        let position = Coord::parse(caps.get(1).unwrap().as_str());
        let velocity = Coord::parse(caps.get(2).unwrap().as_str());
        Self { position, velocity }
    }

    fn count_intersections_2d(stones: &[Hailstone], range: &RangeInclusive<i64>) -> usize {
        let range_f64 = (*range.start() as f64)..=(*range.end() as f64);
        stones
            .iter()
            .combinations(2)
            .filter(|v| v[0].intersects_2d(&v[1], &range_f64))
            .count()
    }

    fn slope_and_y_intercept_2d(&self) -> (f64, f64) {
        let slope = (self.velocity.y as f64) / (self.velocity.x as f64);
        let intercept = (self.position.y as f64) - slope * (self.position.x as f64);
        (slope, intercept)
    }

    fn intersects_2d(&self, other: &Hailstone, range: &RangeInclusive<f64>) -> bool {
        let (s1, i1) = self.slope_and_y_intercept_2d();
        let (s2, i2) = other.slope_and_y_intercept_2d();

        let x = (i2 - i1) / (s1 - s2);
        let y = s1 * x + i1;

        let f1 = (y - (self.position.y as f64)).signum() as i64 == self.velocity.y.signum();
        let f2 = (y - (other.position.y as f64)).signum() as i64 == other.velocity.y.signum();

        f1 && f2 && range.contains(&x) && range.contains(&y)
    }

    fn solve_perfect_throw(stones: &[Hailstone]) -> (Coord, Coord) {
        /*
        start with the throw, and 3 hailstones:
        x, y, z @ u, v, w
        a1, b1, c1 @ d1, e1, f1
        a2, b2, c2 @ d2, e2, f2
        a3, b3, c3 @ d3, e3, f3

        turn that into a system of 9 equations, 3 for each hailstone:
        x + u*t = a + d*t
        y + v*t = b + e*t
        z + w*t = c + f*t

        and refactor the equations for time:

        x equation:
        x + u*t = a + d*t
        u*t - d*t = a - x
        t*(u - d) = a - x
        t = (a - x)/(u - d)

        the same 3 equations for each hailstone, but with time isolated:
        t = (a - x)/(u - d)
        t = (b - y)/(v - e)
        t = (c - z)/(w - f)

        now get rid of the time by combining each pair of equations x=y, x=z, y=z.
        (a - x)/(u - d) = (b - y)/(v - e)
        (a - x)/(u - d) = (c - z)/(w - f)
        (b - y)/(v - e) = (c - z)/(w - f)

        now simplify, and isolate the terms that use hailstone terms to the right side

        simplify x=y:
        (a - x)/(u - d) = (b - y)/(v - e)
        (a - x)*(v - e) = (b - y)*(u - d)
        a*v - a*e - x*v + e*x = b*u - b*d - y*u + d*y
        y*u - x*v = b*u - b*d + d*y - a*v + a*e - e*x

        simplify x=z:
        (a - x)/(u - d) = (c - z)/(w - f)
        (a - x)*(w - f) = (c - z)*(u - d)
        a*w - a*f - x*w + f*x = c*u - c*d - z*u + d*z
        z*u - x*w = c*u - c*d + d*z - a*w + a*f - f*x

        simplify y=z:
        (b - y)/(v - e) = (c - z)/(w - f)
        (b - y)*(w - f) = (c - z)*(v - e)
        b*w - b*f - y*w + f*y = c*v - c*e - z*v + e*z
        z*v - y*w = c*v - c*e + e*z - b*w + b*f - f*y

        so now we have 3 equations for each hailstone, for xy, xz, yz:
        y*u - x*v = b*u - b*d + d*y - a*v + a*e - e*x
        z*u - x*w = c*u - c*d + d*z - a*w + a*f - f*x
        z*v - y*w = c*v - c*e + e*z - b*w + b*f - f*y

        now turn it into 9 equations, plugging in the hailstone numbers:
        y*u - x*v = b1*u - b1*d1 + d1*y - a1*v + a1*e1 - e1*x
        z*u - x*w = c1*u - c1*d1 + d1*z - a1*w + a1*f1 - f1*x
        z*v - y*w = c1*v - c1*e1 + e1*z - b1*w + b1*f1 - f1*y
        y*u - x*v = b2*u - b2*d2 + d2*y - a2*v + a2*e2 - e2*x
        z*u - x*w = c2*u - c2*d2 + d2*z - a2*w + a2*f2 - f2*x
        z*v - y*w = c2*v - c2*e2 + e2*z - b2*w + b2*f2 - f2*y
        y*u - x*v = b3*u - b3*d3 + d3*y - a3*v + a3*e3 - e3*x
        z*u - x*w = c3*u - c3*d3 + d3*z - a3*w + a3*f3 - f3*x
        z*v - y*w = c3*v - c3*e3 + e3*z - b3*w + b3*f3 - f3*y

        now, since third equation has the left hand side in common, we can substitute in pairs, and rewrite to isolate the x/y/z/u/v/w vars on the left:

        first, do the xy equations:

        #1 = #4:
        b1*u - b1*d1 + d1*y - a1*v + a1*e1 - e1*x = b2*u - b2*d2 + d2*y - a2*v + a2*e2 - e2*x
        b1*u + d1*y - a1*v - e1*x - b2*u - d2*y + a2*v + e2*x = a2*e2 + b1*d1 - a1*e1 - b2*d2
        e2*x - e1*x + d1*y - d2*y + b1*u - b2*u + a2*v - a1*v = b1*d1 - b2*d2 + a2*e2 - a1*e1
        x*(e2 - e1) + y*(d1 - d2) + u*(b1 - b2) + v*(a2 - a1) = a2*e2 - a1*e1 + b1*d1 - b2*d2

        #1 = #7:
        b1*u - b1*d1 + d1*y - a1*v + a1*e1 - e1*x = b3*u - b3*d3 + d3*y - a3*v + a3*e3 - e3*x
        ...
        x*(e3 - e1) + y*(d1 - d3) + u*(b1 - b3) + v*(a3 - a1) = a3*e3 - a1*e1 + b1*d1 - b3*d3

        #4 = #7:
        b2*u - b2*d2 + d2*y - a2*v + a2*e2 - e2*x = b3*u - b3*d3 + d3*y - a3*v + a3*e3 - e3*x
        x*(e3 - e2) + y*(d2 - d3) + u*(b2 - b3) + v*(a3 - a2) = a3*e3 - a2*e2 + b2*d2 - b3*d3

        then the xz equations:

        #2 = #5:
        c1*u - c1*d1 + d1*z - a1*w + a1*f1 - f1*x = c2*u - c2*d2 + d2*z - a2*w + a2*f2 - f2*x
        ...
        x*(f2 - f1) + z*(d1 - d2) + u*(c1 - c2) + w*(a2 - a1) = a2*f2 - a1*f1 + c1*d1 - c2*d2

        #2 = #8:
        x*(f3 - f1) + z*(d1 - d3) + u*(c1 - c3) + w*(a3 - a1) = a3*f3 - a1*f1 + c1*d1 - c3*d3

        #5 = #8:
        x*(f3 - f2) + z*(d2 - d3) + u*(c2 - c3) + w*(a3 - a2) = a3*f3 - a2*f2 + c2*d2 - c3*d3

        then the yz equations:

        #3 = #6:
        c1*v - c1*e1 + e1*z - b1*w + b1*f1 - f1*y = c2*v - c2*e2 + e2*z - b2*w + b2*f2 - f2*y
        ...
        y*(f2 - f1) + z*(e1 - e2) + v*(c1 - c2) + w*(b2 - b1) = b2*f2 - b1*f1 + c1*e1 - c2*e2

        #3 = #9:
        y*(f3 - f1) + z*(e1 - e3) + v*(c1 - c3) + w*(b3 - b1) = b3*f3 - b1*f1 + c1*e1 - c3*e3

        #6 = #9
        y*(f3 - f2) + z*(e2 - e3) + v*(c2 - c3) + w*(b3 - b2) = b3*f3 - b2*f2 + c2*e2 - c3*e3

        now we have 9 equations:
        x*(e2 - e1) + y*(d1 - d2)               + u*(b1 - b2) + v*(a2 - a1)               = a2*e2 - a1*e1 + b1*d1 - b2*d2
        x*(e3 - e1) + y*(d1 - d3)               + u*(b1 - b3) + v*(a3 - a1)               = a3*e3 - a1*e1 + b1*d1 - b3*d3
        x*(e3 - e2) + y*(d2 - d3)               + u*(b2 - b3) + v*(a3 - a2)               = a3*e3 - a2*e2 + b2*d2 - b3*d3
        x*(f2 - f1)               + z*(d1 - d2) + u*(c1 - c2)               + w*(a2 - a1) = a2*f2 - a1*f1 + c1*d1 - c2*d2
        x*(f3 - f1)               + z*(d1 - d3) + u*(c1 - c3)               + w*(a3 - a1) = a3*f3 - a1*f1 + c1*d1 - c3*d3
        x*(f3 - f2)               + z*(d2 - d3) + u*(c2 - c3)               + w*(a3 - a2) = a3*f3 - a2*f2 + c2*d2 - c3*d3
                      y*(f2 - f1) + z*(e1 - e2)               + v*(c1 - c2) + w*(b2 - b1) = b2*f2 - b1*f1 + c1*e1 - c2*e2
                      y*(f3 - f1) + z*(e1 - e3)               + v*(c1 - c3) + w*(b3 - b1) = b3*f3 - b1*f1 + c1*e1 - c3*e3
                      y*(f3 - f2) + z*(e2 - e3)               + v*(c2 - c3) + w*(b3 - b2) = b3*f3 - b2*f2 + c2*e2 - c3*e3

        this can be put in a matrix to solve. however, we only need 6 of these equations, since there are only 6 variables we're solving for (x/y/z/u/v/w).
        so take the first two xy, first & third xz, and second and third yz, to get the best coverage in terms of x/y/z and 1/2/3.

        x*(e2 - e1) + y*(d1 - d2)               + u*(b1 - b2) + v*(a2 - a1)               = a2*e2 - a1*e1 + b1*d1 - b2*d2
        x*(e3 - e1) + y*(d1 - d3)               + u*(b1 - b3) + v*(a3 - a1)               = a3*e3 - a1*e1 + b1*d1 - b3*d3
        x*(f2 - f1)               + z*(d1 - d2) + u*(c1 - c2)               + w*(a2 - a1) = a2*f2 - a1*f1 + c1*d1 - c2*d2
        x*(f3 - f2)               + z*(d2 - d3) + u*(c2 - c3)               + w*(a3 - a2) = a3*f3 - a2*f2 + c2*d2 - c3*d3
                      y*(f3 - f1) + z*(e1 - e3)               + v*(c1 - c3) + w*(b3 - b1) = b3*f3 - b1*f1 + c1*e1 - c3*e3
                      y*(f3 - f2) + z*(e2 - e3)               + v*(c2 - c3) + w*(b3 - b2) = b3*f3 - b2*f2 + c2*e2 - c3*e3

        now we can put the left side in a matrix, invert it, and multiply by a vector of the right side, which will give the solution.

        */

        // we only need to use the first three hailstones
        let s1 = &stones[0];
        let s2 = &stones[1];
        let s3 = &stones[2];

        // unpack all the variables to make things cleaner
        let a1 = s1.position.x as f64;
        let b1 = s1.position.y as f64;
        let c1 = s1.position.z as f64;
        let d1 = s1.velocity.x as f64;
        let e1 = s1.velocity.y as f64;
        let f1 = s1.velocity.z as f64;

        let a2 = s2.position.x as f64;
        let b2 = s2.position.y as f64;
        let c2 = s2.position.z as f64;
        let d2 = s2.velocity.x as f64;
        let e2 = s2.velocity.y as f64;
        let f2 = s2.velocity.z as f64;

        let a3 = s3.position.x as f64;
        let b3 = s3.position.y as f64;
        let c3 = s3.position.z as f64;
        let d3 = s3.velocity.x as f64;
        let e3 = s3.velocity.y as f64;
        let f3 = s3.velocity.z as f64;

        // we can put the left side into a matrix
        // x, y, z, u, v, w;
        let m = matrix![
            e2 - e1, d1 - d2, 0.0, b1 - b2, a2 - a1, 0.0;
            e3 - e1, d1 - d3, 0.0, b1 - b3, a3 - a1, 0.0;
            f2 - f1, 0.0, d1 - d2, c1 - c2, 0.0, a2 - a1;
            f3 - f2, 0.0, d2 - d3, c2 - c3, 0.0, a3 - a2;
            0.0, f3 - f1, e1 - e3, 0.0, c1 - c3, b3 - b1;
            0.0, f3 - f2, e2 - e3, 0.0, c2 - c3, b3 - b2;
        ];

        // and the right side into a vector
        let v = vector![
            a2 * e2 - a1 * e1 + b1 * d1 - b2 * d2,
            a3 * e3 - a1 * e1 + b1 * d1 - b3 * d3,
            a2 * f2 - a1 * f1 + c1 * d1 - c2 * d2,
            a3 * f3 - a2 * f2 + c2 * d2 - c3 * d3,
            b3 * f3 - b1 * f1 + c1 * e1 - c3 * e3,
            b3 * f3 - b2 * f2 + c2 * e2 - c3 * e3,
        ];

        // invert!
        let inv = m.try_inverse().unwrap();

        // and multiply by the vector to get the result
        let res = inv * v;

        let x = res[0].round() as i64;
        let y = res[1].round() as i64;
        let z = res[2].round() as i64;
        let u = res[3].round() as i64;
        let v = res[4].round() as i64;
        let w = res[5].round() as i64;
        (Coord::new(x, y, z), Coord::new(u, v, w))
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Coord {
    x: i64,
    y: i64,
    z: i64,
}

impl Coord {
    fn parse(input: &str) -> Self {
        let nums: Vec<i64> = input
            .split(",")
            .map(|s| s.trim().parse::<i64>().unwrap())
            .collect();
        Self::from_slice(&nums)
    }

    fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }

    fn from_slice(nums: &[i64]) -> Self {
        assert_eq!(nums.len(), 3);
        Self::new(nums[0], nums[1], nums[2])
    }
}

impl Add for Coord {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Coord {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl std::ops::Mul<Coord> for i64 {
    type Output = Coord;

    fn mul(self, coord: Coord) -> Coord {
        Coord {
            x: coord.x * self,
            y: coord.y * self,
            z: coord.z * self,
        }
    }
}

fn part1(input: &str, val_range: RangeInclusive<i64>) -> usize {
    let hailstones = Hailstone::parse_list(input);
    Hailstone::count_intersections_2d(&hailstones, &val_range)
}

fn part2(input: &str) -> usize {
    let hailstones = Hailstone::parse_list(input);
    let (position, _velocity) = Hailstone::solve_perfect_throw(&hailstones);
    (position.x + position.y + position.z) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    static EXAMPLE: &str = indoc! {"
        19, 13, 30 @ -2,  1, -2
        18, 19, 22 @ -1, -1, -2
        20, 25, 34 @ -2, -2, -4
        12, 31, 28 @ -1, -2, -1
        20, 19, 15 @  1, -5, -3
    "};

    #[test]
    fn test_part1_example() {
        let result = part1(EXAMPLE, 7..=27);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file(), PART1_RANGE);
        assert_eq!(result, 16589);
    }

    #[test]
    fn test_part2_example() {
        let result = part2(EXAMPLE);
        assert_eq!(result, 47);
    }

    #[test]
    fn test_part2_solution() {
        let result = part2(&read_input_file());
        assert_eq!(result, 781390555762385);
    }
}
