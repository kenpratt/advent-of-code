use std::fs;

// use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    // println!("part 2 result: {:?}", part2(&read_input_file()));
}

fn read_input_file() -> String {
    fs::read_to_string("input.txt").expect("Something went wrong reading the file")
}

lazy_static! {
    static ref INPUT_RE: Regex =
        Regex::new(r"\Atarget area: x=([\-\d]+)\.\.([\-\d]+), y=([\-\d]+)\.\.([\-\d]+)\z").unwrap();
}

#[derive(Debug)]
struct TargetArea {
    x_min: isize,
    x_max: isize,
    y_min: isize,
    y_max: isize,
}

impl TargetArea {
    fn parse(input: &str) -> TargetArea {
        let captures = INPUT_RE.captures(input).unwrap();
        let x_min = captures.get(1).unwrap().as_str().parse::<isize>().unwrap();
        let x_max = captures.get(2).unwrap().as_str().parse::<isize>().unwrap();
        let y_min = captures.get(3).unwrap().as_str().parse::<isize>().unwrap();
        let y_max = captures.get(4).unwrap().as_str().parse::<isize>().unwrap();
        TargetArea {
            x_min,
            x_max,
            y_min,
            y_max,
        }
    }

    fn in_range(&self, projectile: &Projectile) -> RangePosition {
        let x = RangePosition::for_x(&projectile.x, &self.x_min, &self.x_max);
        let y = RangePosition::for_y(&projectile.y, &self.y_min, &self.y_max);
        match (x, y) {
            // hit
            (RangePosition::Inside, RangePosition::Inside) => RangePosition::Inside,

            // if either axis is after, we've overshot
            (RangePosition::After, _) => RangePosition::After,
            (_, RangePosition::After) => RangePosition::After,

            // otherwise, must be before!
            _ => RangePosition::Before,
        }
    }
}

#[derive(Debug)]
enum RangePosition {
    Before,
    Inside,
    After,
}

impl RangePosition {
    fn for_x(x: &isize, x_min: &isize, x_max: &isize) -> Self {
        if x < x_min {
            Self::Before
        } else if x > x_max {
            Self::After
        } else {
            Self::Inside
        }
    }

    fn for_y(y: &isize, y_min: &isize, y_max: &isize) -> Self {
        if y < y_min {
            Self::After
        } else if y > y_max {
            Self::Before
        } else {
            Self::Inside
        }
    }
}

#[derive(Debug)]
struct Projectile {
    vx: isize,
    vy: isize,
    x: isize,
    y: isize,
    max_y: isize,
    steps: usize,
}

impl Projectile {
    fn new(vx: isize, vy: isize) -> Projectile {
        Projectile {
            vx,
            vy,
            x: 0,
            y: 0,
            max_y: 0,
            steps: 0,
        }
    }

    fn fire(&mut self, target: &TargetArea) -> (bool, isize, usize) {
        loop {
            // adjust positions
            self.x += self.vx;
            self.y += self.vy;
            if self.y > self.max_y {
                self.max_y = self.y;
            }

            // apply drag
            if self.vx > 0 {
                self.vx -= 1;
            } else if self.vx < 0 {
                self.vx += 1;
            }

            // apply gravity
            self.vy -= 1;

            self.steps += 1;

            // check for result
            match target.in_range(self) {
                RangePosition::Before => {} // continue
                RangePosition::Inside => {
                    // hit the target
                    return (true, self.max_y, self.steps);
                }
                RangePosition::After => {
                    // missed the target
                    return (false, self.max_y, self.steps);
                }
            }
        }
    }
}

fn part1(input: &str) -> isize {
    let target = TargetArea::parse(input);

    let mut hits = vec![];
    for vx in 0..target.x_max {
        for vy in target.y_min..200 {
            let mut projectile = Projectile::new(vx, vy);
            let (hit, max_y, steps) = projectile.fire(&target);
            if hit {
                println!("hit: {},{} / {} / {}", vx, vy, max_y, steps);
                hits.push((vx, vy, max_y, steps));
            }
        }
    }

    *hits
        .iter()
        .map(|(_vx, _vy, max_y, _steps)| max_y)
        .max()
        .unwrap()
}

// fn part2(input: &str) -> usize {
//     let data = TargetArea::parse(input);
//     println!("{:?}", data);
//     data.execute()
// }

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE1: &str = "target area: x=20..30, y=-10..-5";

    #[test]
    fn test_part1_example1() {
        let result = part1(EXAMPLE1);
        assert_eq!(result, 45);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file());
        assert_eq!(result, 3160);
    }

    // #[test]
    // fn test_part2_example1() {
    //     let result = part2(EXAMPLE1);
    //     assert_eq!(result, 0);
    // }

    // #[test]
    // fn test_part2_solution() {
    //     let result = part2(&read_input_file());
    //     assert_eq!(result, 0);
    // }
}
