use std::collections::HashSet;
use std::fs;
use std::ops::Add;
use std::ops::RangeInclusive;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    println!("part 2 result: {:?}", part2(&read_input_file()));
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

struct Bounds {
    x: RangeInclusive<i8>,
    y: RangeInclusive<i8>,
    z: RangeInclusive<i8>,
}

impl Bounds {
    fn calculate(coords: &HashSet<Coord>) -> Self {
        let min_x = coords.iter().map(|c| c.x).min().unwrap() - 1;
        let max_x = coords.iter().map(|c| c.x).max().unwrap() + 1;
        let min_y = coords.iter().map(|c| c.y).min().unwrap() - 1;
        let max_y = coords.iter().map(|c| c.y).max().unwrap() + 1;
        let min_z = coords.iter().map(|c| c.z).min().unwrap() - 1;
        let max_z = coords.iter().map(|c| c.z).max().unwrap() + 1;
        Self {
            x: min_x..=max_x,
            y: min_y..=max_y,
            z: min_z..=max_z,
        }
    }

    fn contains(&self, c: &Coord) -> bool {
        self.x.contains(&c.x) && self.y.contains(&c.y) && self.z.contains(&c.z)
    }

    fn bounding_box(&self) -> HashSet<Coord> {
        let mut coords = HashSet::new();

        for x in self.x.clone() {
            for y in self.y.clone() {
                coords.insert(Coord {
                    x: x,
                    y: y,
                    z: *self.z.start(),
                });
                coords.insert(Coord {
                    x: x,
                    y: y,
                    z: *self.z.end(),
                });
            }
        }

        for x in self.x.clone() {
            for z in self.z.clone() {
                coords.insert(Coord {
                    x: x,
                    y: *self.y.start(),
                    z: z,
                });
                coords.insert(Coord {
                    x: x,
                    y: *self.y.end(),
                    z: z,
                });
            }
        }

        for y in self.y.clone() {
            for z in self.z.clone() {
                coords.insert(Coord {
                    x: *self.x.start(),
                    y: y,
                    z: z,
                });
                coords.insert(Coord {
                    x: *self.x.end(),
                    y: y,
                    z: z,
                });
            }
        }

        coords
    }
}

fn part1(input: &str) -> usize {
    let coords = Coord::parse_list(input);
    coords
        .iter()
        .map(|c| c.neighbours().difference(&coords).count())
        .sum::<usize>()
}

fn expand_steam(cubes: &HashSet<Coord>, bounds: &Bounds) -> HashSet<Coord> {
    let mut area = bounds.bounding_box();
    let mut last_frontier = area.clone();

    while !last_frontier.is_empty() {
        let frontier: HashSet<Coord> = last_frontier
            .iter()
            .flat_map(|c| c.neighbours())
            .filter(|c| bounds.contains(c) && !cubes.contains(c) && !area.contains(&c))
            .collect();

        // add to area
        area = area.union(&frontier).cloned().collect();
        last_frontier = frontier;
    }

    area
}

fn part2(input: &str) -> usize {
    let coords = Coord::parse_list(input);
    let bounds = Bounds::calculate(&coords);

    let steam = expand_steam(&coords, &bounds);

    let neighbours: HashSet<Coord> = coords.iter().flat_map(|c| c.neighbours()).collect();
    let air: HashSet<Coord> = neighbours.difference(&coords).cloned().collect();
    let trapped_air: HashSet<Coord> = air.difference(&steam).cloned().collect();

    let coords_and_air_pockets: HashSet<Coord> = coords.union(&trapped_air).cloned().collect();
    coords
        .iter()
        .map(|c| c.neighbours().difference(&coords_and_air_pockets).count())
        .sum::<usize>()
}

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

    #[test]
    fn test_part2_example() {
        let result = part2(EXAMPLE);
        assert_eq!(result, 58);
    }

    #[test]
    fn test_part2_solution() {
        let result = part2(&read_input_file());
        assert_eq!(result, 2498);
    }
}
