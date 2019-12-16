use std::collections::HashMap;
use std::fs;
use regex::Regex;

fn main() {
    //part1();
    //part2();
    run_simulation_until_repeat(
        "<x=-8, y=-10, z=0>\n<x=5, y=5, z=10>\n<x=2, y=-7, z=3>\n<x=9, y=-8, z=-3>".to_string(),
        10000000, // 10M
    );
}

fn part1() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let simulation = run_simulation(contents, 1000);
    println!("part 1 total energy: {:?}", simulation.total_energy());
}

fn part2() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let steps_until_repeat = run_simulation_until_repeat(contents, 1000);
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

fn run_simulation_until_repeat(contents: String, max_iterations: usize) -> usize {
    let mut simulation = construct_simulation(contents);
    let mut history = History::new(simulation.moons.len());
    history.add(&simulation);
    for _ in 0..max_iterations {
        simulation.step();
        if simulation.steps % 1000 == 0 {
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
    steps: usize,
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
    data: Vec<CoordinateHistory>,
}

impl History {
    pub fn new(num_moons: usize) -> History {
        let mut data = vec![];
        for _ in 0..(num_moons * 3) {
            data.push(CoordinateHistory::new());
        }
        return History {
            data: data,
        }
    }

    pub fn add(&mut self, simulation: &Simulation) {
        for (moon_index, moon) in simulation.moons.iter().enumerate() {
            let i = moon_index * 3;
            self.data[i].add(moon.x, simulation.steps);
            self.data[i+1].add(moon.y, simulation.steps);
            self.data[i+2].add(moon.z, simulation.steps);
        }
    }

    pub fn found_loop(&self) -> bool {
        return self.data.iter().all(|h| h.found_loop());
    }

    pub fn steps_until_repeat(&self) -> usize {
        for c in self.data.iter() {
            println!("{:?}", c.loop_info);
        }
        return 0;
    }
}

#[derive(Debug)]
pub struct CoordinateHistory {
    data: HashMap<Coordinate, usize>,
    loop_info: Option<LoopInfo>,
}

impl CoordinateHistory {
    pub fn new() -> CoordinateHistory {
        return CoordinateHistory {
            data: HashMap::new(),
            loop_info: None,
        }
    }

    pub fn found_loop(&self) -> bool {
        return self.loop_info.is_some();
    }

    pub fn add(&mut self, coord: Coordinate, steps: usize) {
        if self.found_loop() {
            return;
        }

        match self.data.get(&coord) {
            Some(prior_steps) => {
                let size = steps - prior_steps;
                self.loop_info = Some(LoopInfo::new(prior_steps.clone(), size));
            },
            None => {
                self.data.insert(coord, steps);
            }
        }
    }
}

#[derive(Debug)]
pub struct LoopInfo {
    offset: usize,
    size: usize,
}

impl LoopInfo {
    pub fn new(offset: usize, size: usize) -> LoopInfo {
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
            3000,
        );
        assert_eq!(steps_until_repeat, 2772);
    }

    // #[test]
    // fn test_part2_example2() {
    //     let simulation = run_simulation_until_repeat(
    //     //let steps_until_repeat = run_simulation(
    //         "<x=-8, y=-10, z=0>\n<x=5, y=5, z=10>\n<x=2, y=-7, z=3>\n<x=9, y=-8, z=-3>".to_string(),
    //         10000000, // 10M
    //         //100000000, // 100M
    //         // 4,686,774,924
    //     );
    //     //assert_eq!(steps_until_repeat, 4686774924);
    // }    
}