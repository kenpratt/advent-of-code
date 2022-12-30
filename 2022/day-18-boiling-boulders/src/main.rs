use std::collections::HashSet;
use std::fs;
use std::ops::Add;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    // println!("part 2 result: {:?}", part2(&read_input_file()));
}

fn read_input_file() -> String {
    fs::read_to_string("input.txt").expect("Something went wrong reading the file")
}

const OFFSETS: [Coord; 6] = [
    Coord { x: -1, y: 0, z: 0 },
    Coord { x: 1, y: 0, z: 0 },
    Coord { x: 0, y: -1, z: 0 },
    Coord { x: 0, y: 1, z: 0 },
    Coord { x: 0, y: 0, z: -1 },
    Coord { x: 0, y: 0, z: 1 },
];

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
struct Coord {
    x: i8,
    y: i8,
    z: i8,
}

impl Coord {
    fn parse_list(input: &str) -> HashSet<Self> {
        input.lines().map(|l| Self::parse(l)).collect()
    }

    fn parse(input: &str) -> Self {
        let vals: Vec<i8> = input.split(",").map(|s| s.parse::<i8>().unwrap()).collect();
        assert_eq!(vals.len(), 3);
        Self {
            x: vals[0],
            y: vals[1],
            z: vals[2],
        }
    }

    fn neighbours(&self) -> HashSet<Coord> {
        OFFSETS.iter().map(|o| *o + *self).collect()
    }
}

impl Add for Coord {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

fn part1(input: &str) -> usize {
    let coords = Coord::parse_list(input);
    coords
        .iter()
        .map(|c| c.neighbours().difference(&coords).count())
        .sum::<usize>()
}

// fn part2(input: &str) -> usize {
//     let data = Data::parse(input);
//     dbg!(&data);
//     data.execute()
// }

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    static EXAMPLE: &str = indoc! {"
        2,2,2
        1,2,2
        3,2,2
        2,1,2
        2,3,2
        2,2,1
        2,2,3
        2,2,4
        2,2,6
        1,2,5
        3,2,5
        2,1,5
        2,3,5
    "};

    #[test]
    fn test_part1_example() {
        let result = part1(EXAMPLE);
        assert_eq!(result, 64);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file());
        assert_eq!(result, 4460);
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
