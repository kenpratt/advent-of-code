use std::fs;

use regex::Regex;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    println!("part 2 result: {:?}", part2(&read_input_file()));
}

fn read_input_file() -> String {
    return fs::read_to_string("input.txt").expect("Something went wrong reading the file");
}

#[derive(Debug)]
struct Ship {
    instructions: Vec<Instruction>,
}

impl Ship {
    fn parse(input: &str) -> Ship {
        let instructions = input.lines().map(|line| Instruction::parse(line)).collect();
        return Ship {
            instructions: instructions,
        }
    }

    fn execute(&self, use_waypoint: bool) -> usize {
        let mut nav = Ship::construct_navigation(use_waypoint);
        for instruction in &self.instructions {
            nav.apply_instruction(&instruction);
            println!("instruction: {:?}, after: {:?}", instruction, nav);
        }
        return nav.manhattan_distance();
    }

    fn construct_navigation(use_waypoint: bool) -> Box<dyn Navigation> {
        return if use_waypoint {
            Box::new(WaypointNavigation::new(10, 1))
        } else {
            Box::new(SimpleNavigation::new())
        };
    }    
}

trait Navigation: std::fmt::Debug {
    fn apply_instruction(&mut self, instruction: &Instruction);
    fn manhattan_distance(&self) -> usize;
}

#[derive(Debug)]
struct SimpleNavigation {
    position: Point,
    angle: isize,
}

impl SimpleNavigation {
    fn new() -> SimpleNavigation {
        return SimpleNavigation {
            position: Point {
                x: 0,
                y: 0,
            },
            angle: 0,
        };
    }
}

impl Navigation for SimpleNavigation {
    fn apply_instruction(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::North(value) => {
                self.position.y += *value as isize;
            },
            Instruction::South(value) => {
                self.position.y -= *value as isize;
            },
            Instruction::East(value) => {
                self.position.x += *value as isize;
            },
            Instruction::West(value) => {
                self.position.x -= *value as isize;
            },
            Instruction::Left(value) => {
                self.angle += *value as isize;
            },
            Instruction::Right(value) => {
                self.angle -= *value as isize;
            },
            Instruction::Forward(value) => {
                let radians = (self.angle as f64).to_radians();
                let x = radians.cos() * (*value as f64);
                let y = radians.sin() * (*value as f64);
                self.position.x += x.round() as isize;
                self.position.y += y.round() as isize;
            },
        }
    }

    fn manhattan_distance(&self) -> usize {
        return self.position.manhattan_distance();
    }
}

#[derive(Debug)]
struct WaypointNavigation {
    position: Point,
    waypoint: Point,
}

impl WaypointNavigation {
    fn new(waypoint_x: isize, waypoint_y: isize) -> WaypointNavigation {
        return WaypointNavigation {
            position: Point {
                x: 0,
                y: 0,
            },
            waypoint: Point {
                x: waypoint_x,
                y: waypoint_y,
            },
        };
    }
}

impl Navigation for WaypointNavigation {
    fn apply_instruction(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::North(value) => {
                self.waypoint.y += *value as isize;
            },
            Instruction::South(value) => {
                self.waypoint.y -= *value as isize;
            },
            Instruction::East(value) => {
                self.waypoint.x += *value as isize;
            },
            Instruction::West(value) => {
                self.waypoint.x -= *value as isize;
            },
            Instruction::Left(value) => {
                self.waypoint.rotate(*value as isize);
            },
            Instruction::Right(value) => {
                self.waypoint.rotate(-(*value as isize));
            },
            Instruction::Forward(value) => {
                self.position.x += self.waypoint.x * (*value as isize);
                self.position.y += self.waypoint.y * (*value as isize);
            },
        }
    }

    fn manhattan_distance(&self) -> usize {
        return self.position.manhattan_distance();
    }
}

#[derive(Debug)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn rotate(&mut self, degrees: isize) {
        let x = self.x as f64;
        let y = self.y as f64;
        let radians = (degrees as f64).to_radians();
        let cos = radians.cos();
        let sin = radians.sin();

        self.x = (x * cos - y * sin).round() as isize;
        self.y = (x * sin + y * cos).round() as isize;
    }

    fn manhattan_distance(&self) -> usize {
        let dx = self.x.abs();
        let dy = self.y.abs();
        println!("x: {}, y: {}, d: {}", self.x, self.y, dx + dy);
        return (dx + dy) as usize;
    }    
}

#[derive(Debug)]
enum Instruction {
    North(usize),
    South(usize),
    East(usize),
    West(usize),
    Left(usize),
    Right(usize),
    Forward(usize),
}

impl Instruction {
    fn parse(input: &str) -> Instruction {
        let re = Regex::new(r"^([A-Z])(\d+)$").unwrap();
        let captures = re.captures(input).unwrap();
        let action = captures.get(1).unwrap().as_str();
        let value = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();

        return match action {
            "N" => Instruction::North(value),
            "S" => Instruction::South(value),
            "E" => Instruction::East(value),
            "W" => Instruction::West(value),
            "L" => Instruction::Left(value),
            "R" => Instruction::Right(value),
            "F" => Instruction::Forward(value),
            _ => panic!("Unknown action: {}", action),
        };
    }
}

fn part1(input: &str) -> usize {
    let ship = Ship::parse(input);
    return ship.execute(false);
}

fn part2(input: &str) -> usize {
    let ship = Ship::parse(input);
    return ship.execute(true);
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    static EXAMPLE1: &str = indoc! {"
        F10
        N3
        F7
        R90
        F11
    "};    

    static EXAMPLE1_MOD1: &str = indoc! {"
        F10
        N3
        F7
        L90
        F11
    "};  

    static EXAMPLE1_MOD2: &str = indoc! {"
        F10
        N3
        F7
        L180
        F11
    "};

    #[test]
    fn test_part1_example1() {
        let result = part1(EXAMPLE1);
        assert_eq!(result, 25);

        assert_eq!(part1(EXAMPLE1_MOD1), 31);
        assert_eq!(part1(EXAMPLE1_MOD2), 9);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file());
        assert_eq!(result, 1710);
    }

    #[test]
    fn test_part2_example1() {
        let result = part2(EXAMPLE1);
        assert_eq!(result, 286);

        // p = 170,38
        // w = -4,10
        // F11: p + 11*w
        // 170,38 + 11*(-4, 10)
        // (170-44),(38+110)
        // 126,148 => 274
        assert_eq!(part2(EXAMPLE1_MOD1), 274);

        // p = 170,38
        // w = -10,-4
        // F11: p + 11*w
        // 170,38 + 11*(-110, -44)
        // (170-110),(38-44)
        // 60,-6 => 66
        assert_eq!(part2(EXAMPLE1_MOD2), 66);
    }

    #[test]
    fn test_part2_solution() {
        let result = part2(&read_input_file());
        assert_eq!(result, 62045);
    }
}