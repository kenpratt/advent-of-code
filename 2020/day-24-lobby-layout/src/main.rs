use std::collections::HashSet;
use std::fs;

use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    // println!("part 2 result: {:?}", part2(&read_input_file()));
}

fn read_input_file() -> String {
    fs::read_to_string("input.txt").expect("Something went wrong reading the file")
}

#[derive(Debug)]
struct Data {
    paths: Vec<Path>,
}

impl Data {
    fn parse(input: &str) -> Data {
        let paths = input.lines().map(|line| Path::parse(line)).collect();
        Data {
            paths: paths,
        }
    }

    fn count_flipped_tiles(&self) -> usize {
        let start = Coordinate::start();
        let mut flipped = HashSet::new();

        for path in &self.paths {
            let coord = path.follow(&start);
            println!("{:?} => {:?}", path, coord);
            if flipped.contains(&coord) {
                flipped.remove(&coord);
                println!("removed {:?}", coord);
            } else {
                flipped.insert(coord);
                println!("added {:?}", coord);
            }
        }

        flipped.len()
    }
}

#[derive(Debug)]
struct Path {
    directions: Vec<Direction>,
}

impl Path {
    fn parse(input: &str) -> Path {
        Path {
            directions: Direction::parse_list(input),
        }
    }

    fn follow(&self, start: &Coordinate) -> Coordinate {
        self.directions.iter().fold(*start, |coord, direction| coord.in_direction(direction))
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct Coordinate {
    r: isize,
    g: isize,
    b: isize,
}

impl Coordinate {
    fn new(r: isize, g: isize, b: isize) -> Coordinate {
        Coordinate {
            r: r,
            g: g,
            b: b,
        }
    }

    fn start() -> Coordinate {
        Coordinate::new(0, 0, 0)
    }

    fn in_direction(&self, direction: &Direction) -> Coordinate {
        let (dr, dg, db) = direction.offsets();
        Coordinate {
            r: self.r + dr,
            g: self.g + dg,
            b: self.b + db,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Direction {
    East,
    Southeast,
    Southwest,
    West,
    Northwest,
    Northeast,
}

impl Direction {
    fn parse_list(input: &str) -> Vec<Direction> {
        lazy_static! {
            static ref DIRECTION_RE: Regex = Regex::new(r"(e|se|sw|w|nw|ne)").unwrap();
        }  
        DIRECTION_RE.captures_iter(input).map(|c| Direction::parse(&c[0])).collect()
    }

    fn parse(input: &str) -> Direction {
        match input {
            "e" => Direction::East,
            "se" => Direction::Southeast,
            "sw" => Direction::Southwest,
            "w" => Direction::West,
            "nw" => Direction::Northwest,
            "ne" => Direction::Northeast,
            _ => panic!("Unknown direction: {}", input),
        }
    }

    fn offsets(&self) -> (isize, isize, isize) {
        match self {
            Direction::East => (1, -1, 0),
            Direction::Southeast => (1, 0, -1),
            Direction::Southwest => (0, 1, -1),
            Direction::West => (-1, 1, 0),
            Direction::Northwest => (-1, 0, 1),
            Direction::Northeast => (0, -1, 1),            
        }
    }
}

fn part1(input: &str) -> usize {
    let data = Data::parse(input);
    data.count_flipped_tiles()
}

// fn part2(input: &str) -> usize {
//     let data = Data::parse(input);
//     data.execute()
// }

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    static EXAMPLE1: &str = indoc! {"
        sesenwnenenewseeswwswswwnenewsewsw
        neeenesenwnwwswnenewnwwsewnenwseswesw
        seswneswswsenwwnwse
        nwnwneseeswswnenewneswwnewseswneseene
        swweswneswnenwsewnwneneseenw
        eesenwseswswnenwswnwnwsewwnwsene
        sewnenenenesenwsewnenwwwse
        wenwwweseeeweswwwnwwe
        wsweesenenewnwwnwsenewsenwwsesesenwne
        neeswseenwwswnwswswnw
        nenwswwsewswnenenewsenwsenwnesesenew
        enewnwewneswsewnwswenweswnenwsenwsw
        sweneswneswneneenwnewenewwneswswnese
        swwesenesewenwneswnwwneseswwne
        enesenwswwswneneswsenwnewswseenwsese
        wnwnesenesenenwwnenwsewesewsesesew
        nenewswnwewswnenesenwnesewesw
        eneswnwswnwsenenwnwnwwseeswneewsenese
        neswnwewnwnwseenwseesewsenwsweewe
        wseweeenwnesenwwwswnew
    "};

    #[test]
    fn test_direction_to_coord1() {
        let path = Path::parse("esenee");
        assert_eq!(path.directions, vec![Direction::East, Direction::Southeast, Direction::Northeast, Direction::East]);
        assert_eq!(path.follow(&Coordinate::start()), Coordinate::new(3, -3, 0));
    }

    #[test]
    fn test_direction_to_coord2() {
        let path = Path::parse("esew");
        assert_eq!(path.directions, vec![Direction::East, Direction::Southeast, Direction::West]);
        assert_eq!(path.follow(&Coordinate::start()), Coordinate::new(1, 0, -1));
    }

    #[test]
    fn test_direction_to_coord3() {
        let path = Path::parse("nwwswee");
        assert_eq!(path.directions, vec![Direction::Northwest, Direction::West, Direction::Southwest, Direction::East, Direction::East]);
        assert_eq!(path.follow(&Coordinate::start()), Coordinate::new(0, 0, 0));
    }        

    #[test]
    fn test_part1_example1() {
        let result = part1(EXAMPLE1);
        assert_eq!(result, 10);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file());
        assert_eq!(result, 332);
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