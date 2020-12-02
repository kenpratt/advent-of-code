extern crate num_integer;

use std::collections::HashMap;
use std::fs;
use regex::Regex;
use num_integer::lcm;

fn main() {
    part1();
    part2();
}

fn part1() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let simulation = run_simulation(contents, 1000);
    println!("part 1 total energy: {:?}", simulation.total_energy());
}

fn part2() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let steps_until_repeat = run_simulation_until_repeat(contents);
    println!("part 2 steps: {:?}", steps_until_repeat);
}

fn run_simulation(contents: String, iterations: usize) -> Simulation {
    let mut simulation = construct_simulation(contents);
    for _ in 0..iterations {
        simulation.step();
        if simulation.steps % 1000000 == 0 {
            println!("{}", simulation.steps);
        }
    }
    return simulation;
}

fn run_simulation_until_repeat(contents: String) -> u64 {
    let mut simulation = construct_simulation(contents);
    let mut history = History::new();
    history.add(&simulation);
    loop {
        simulation.step();
        if simulation.steps % 1000000 == 0 {
            println!("{}", simulation.steps);
        }
        history.add(&simulation);
        if history.found_loop() {
            println!("found loop!");
            break;
        }
    }
    return history.steps_until_repeat();
}

fn construct_simulation(contents: String) -> Simulation {
    let moons: Vec<Moon> = contents.lines().map(|line| parse_line(line)).collect();
    return Simulation::new(moons);
}

fn parse_line(line: &str) -> Moon {
    let re = Regex::new(r"^<x=(\-?\d+), y=(\-?\d+), z=(\-?\d+)>$").unwrap();
    let captures = re.captures(line).unwrap();
    let x = captures.get(1).unwrap().as_str().parse::<i16>().unwrap();
    let y = captures.get(2).unwrap().as_str().parse::<i16>().unwrap();
    let z = captures.get(3).unwrap().as_str().parse::<i16>().unwrap();
    let moon = Moon::new(x, y, z);
    println!("{:?}", moon);
    return moon;
}

#[derive(Debug)]
pub struct Simulation {
    moons: Vec<Moon>,
    num_moons: usize,
    steps: u32,
}

impl Simulation {
    pub fn new(moons: Vec<Moon>) -> Simulation {
        let num_moons = moons.len();
        return Simulation {
            moons: moons,
            num_moons: num_moons,
            steps: 0,
        }
    }

    pub fn step(&mut self) {
        for i1 in 0..(self.num_moons - 1) {
            for i2 in (i1+1)..self.num_moons {
                self.gravitate(i1, i2);
                self.gravitate(i2, i1);
            }
        }

        for moon in &mut self.moons {
            moon.apply_velocity();
        }

        self.steps += 1;
        //println!("{:?}", self);
    }

    fn gravitate(&mut self, i1: usize, i2: usize) {
        let other_px = self.moons[i2].x.p;
        let other_py = self.moons[i2].y.p;
        let other_pz = self.moons[i2].z.p;
        let moon = &mut self.moons[i1];
        moon.gravitate(other_px, other_py, other_pz);
    }

    pub fn total_energy(&self) -> i16 {
        return self.moons.iter().map(|moon| moon.total_energy()).sum();
    }

    pub fn x_coords(&self) -> Vec<Coordinate> {
        return self.moons.iter().map(|moon| moon.x).collect();
    }

    pub fn y_coords(&self) -> Vec<Coordinate> {
        return self.moons.iter().map(|moon| moon.y).collect();
    }

    pub fn z_coords(&self) -> Vec<Coordinate> {
        return self.moons.iter().map(|moon| moon.z).collect();
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Coordinate {
    p: i16,
    v: i16,
}

impl Coordinate {
    pub fn new(p: i16) -> Coordinate {
        return Coordinate {
            p: p,
            v: 0,
        };
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Moon {
    x: Coordinate,
    y: Coordinate,
    z: Coordinate,
}

impl Moon {
    pub fn new(px: i16, py: i16, pz: i16) -> Moon {
        return Moon {
            x: Coordinate::new(px),
            y: Coordinate::new(py),
            z: Coordinate::new(pz),
        };
    }

    pub fn gravitate(&mut self, other_px: i16, other_py: i16, other_pz: i16) {
        if self.x.p < other_px {
            self.x.v += 1;
        } else if self.x.p > other_px {
            self.x.v -= 1;
        }

        if self.y.p < other_py {
            self.y.v += 1;
        } else if self.y.p > other_py {
            self.y.v -= 1;
        }

        if self.z.p < other_pz {
            self.z.v += 1;
        } else if self.z.p > other_pz {
            self.z.v -= 1;
        }
    }

    pub fn apply_velocity(&mut self) {
        self.x.p += self.x.v;
        self.y.p += self.y.v;
        self.z.p += self.z.v;
    }

    pub fn potential_energy(&self) -> i16 {
        return self.x.p.abs() + self.y.p.abs() + self.z.p.abs();
    }

    pub fn kinetic_energy(&self) -> i16 {
        return self.x.v.abs() + self.y.v.abs() + self.z.v.abs();
    }

    pub fn total_energy(&self) -> i16 {
        return self.potential_energy() * self.kinetic_energy();
    }

    pub fn values(&self) -> Vec<i16> {
        return vec![self.x.p, self.y.p, self.z.p, self.x.v, self.y.v, self.z.v];
    }
}

#[derive(Debug)]
pub struct History {
    x: AxisHistory,
    y: AxisHistory,
    z: AxisHistory,
}

impl History {
    pub fn new() -> History {
        return History {
            x: AxisHistory::new(),
            y: AxisHistory::new(),
            z: AxisHistory::new(),
        }
    }

    pub fn add(&mut self, simulation: &Simulation) {
        self.x.add(simulation.x_coords(), simulation.steps);
        self.y.add(simulation.y_coords(), simulation.steps);
        self.z.add(simulation.z_coords(), simulation.steps);
    }

    pub fn found_loop(&self) -> bool {
        return self.x.found_loop() && self.y.found_loop() && self.z.found_loop();
    }

    pub fn steps_until_repeat(&self) -> u64 {
        let x_loop = self.x.loop_info.unwrap();
        let y_loop = self.y.loop_info.unwrap();
        let z_loop = self.z.loop_info.unwrap();

        println!("{:?}", x_loop);
        println!("{:?}", y_loop);
        println!("{:?}", z_loop);

        if x_loop.offset != 0 || y_loop.offset != 0 || z_loop.offset != 0 {
            panic!("Don't know how to calculate repeat at non zero offset yet");
        }

        let lcm_xy = lcm(u64::from(x_loop.size), u64::from(y_loop.size));
        let lcm_xyz = lcm(lcm_xy, u64::from(z_loop.size));

        println!("{:?}", lcm_xyz);
        return lcm_xyz;
    }
}

#[derive(Debug)]
pub struct AxisHistory {
    data: HashMap<Vec<Coordinate>, u32>,
    loop_info: Option<LoopInfo>,
}

impl AxisHistory {
    pub fn new() -> AxisHistory {
        return AxisHistory {
            data: HashMap::new(),
            loop_info: None,
        }
    }

    pub fn found_loop(&self) -> bool {
        return self.loop_info.is_some();
    }

    pub fn add(&mut self, coords: Vec<Coordinate>, steps: u32) {
        if self.found_loop() {
            return;
        }

        match self.data.get(&coords) {
            Some(prior_steps) => {
                let size = steps - prior_steps;
                self.loop_info = Some(LoopInfo::new(prior_steps.clone(), size));
            },
            None => {
                self.data.insert(coords, steps);
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct LoopInfo {
    offset: u32,
    size: u32,
}

impl LoopInfo {
    pub fn new(offset: u32, size: u32) -> LoopInfo {
        return LoopInfo {
            offset: offset,
            size: size,
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_part1_example1() {
        let simulation = run_simulation(
            "<x=-1, y=0, z=2>\n<x=2, y=-10, z=-7>\n<x=4, y=-8, z=8>\n<x=3, y=5, z=-1>".to_string(),
            10,
        );
        assert_eq!(simulation.steps, 10);

        // pos=<x=  2, y=  1, z= -3>, vel=<x= -3, y= -2, z=  1>
        // pos=<x=  1, y= -8, z=  0>, vel=<x= -1, y=  1, z=  3>
        // pos=<x=  3, y= -6, z=  1>, vel=<x=  3, y=  2, z= -3>
        // pos=<x=  2, y=  0, z=  4>, vel=<x=  1, y= -1, z= -1>
        assert_eq!(simulation.moons[0].values(), vec![2, 1, -3, -3, -2, 1]);
        assert_eq!(simulation.moons[1].values(), vec![1, -8, 0, -1, 1, 3]);
        assert_eq!(simulation.moons[2].values(), vec![3, -6, 1, 3, 2, -3]);
        assert_eq!(simulation.moons[3].values(), vec![2, 0, 4, 1, -1, -1]);

        assert_eq!(simulation.total_energy(), 179);
    }

    #[test]
    fn test_part1_example2() {
        let simulation = run_simulation(
            "<x=-8, y=-10, z=0>\n<x=5, y=5, z=10>\n<x=2, y=-7, z=3>\n<x=9, y=-8, z=-3>".to_string(),
            100,
        );
        assert_eq!(simulation.steps, 100);

        // pos=<x=  8, y=-12, z= -9>, vel=<x= -7, y=  3, z=  0>
        // pos=<x= 13, y= 16, z= -3>, vel=<x=  3, y=-11, z= -5>
        // pos=<x=-29, y=-11, z= -1>, vel=<x= -3, y=  7, z=  4>
        // pos=<x= 16, y=-13, z= 23>, vel=<x=  7, y=  1, z=  1>
        assert_eq!(simulation.moons[0].values(), vec![  8, -12, -9, -7,   3,  0]);
        assert_eq!(simulation.moons[1].values(), vec![ 13,  16, -3,  3, -11, -5]);
        assert_eq!(simulation.moons[2].values(), vec![-29, -11, -1, -3,   7,  4]);
        assert_eq!(simulation.moons[3].values(), vec![ 16, -13, 23,  7,   1,  1]);

        assert_eq!(simulation.total_energy(), 1940);
    }

    #[test]
    fn test_part2_example1() {
        let steps_until_repeat = run_simulation_until_repeat(
            "<x=-1, y=0, z=2>\n<x=2, y=-10, z=-7>\n<x=4, y=-8, z=8>\n<x=3, y=5, z=-1>".to_string(),
        );
        assert_eq!(steps_until_repeat, 2772);
    }

    #[test]
    fn test_part2_example2() {
        let steps_until_repeat = run_simulation_until_repeat(
            "<x=-8, y=-10, z=0>\n<x=5, y=5, z=10>\n<x=2, y=-7, z=3>\n<x=9, y=-8, z=-3>".to_string(),
        );
        assert_eq!(steps_until_repeat, 4686774924);
    }
}