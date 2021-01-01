use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    println!("part 2 result: {:?}", part2(&read_input_file(), 100));
}

fn read_input_file() -> String {
    fs::read_to_string("input.txt").expect("Something went wrong reading the file")
}

#[derive(Debug)]
struct Floor {
    flipped_tiles: HashSet<Coordinate>,
    neighbour_map: NeighbourMap,
}

impl Floor {
    fn parse(input: &str) -> Floor {
        let paths = input.lines().map(|line| Path::parse(line)).collect();
        Floor {
            flipped_tiles: Floor::initial_flipped_tiles(&paths),
            neighbour_map: NeighbourMap::new(),
        }
    }

    fn initial_flipped_tiles(paths: &Vec<Path>) -> HashSet<Coordinate> {
        let start = Coordinate::start();
        let mut flipped = HashSet::new();

        for path in paths {
            let coord = path.follow(&start);
            if flipped.contains(&coord) {
                flipped.remove(&coord);
            } else {
                flipped.insert(coord);
            }
        }

        flipped
    }

    fn count_flipped_tiles(&self) -> usize {
        self.flipped_tiles.len()
    }

    fn run(&mut self, days: usize) {
        for _ in 0..days {
            self.tick();
        }
    }

    fn tick(&mut self) {
        let mut new_flipped = HashSet::new();
        let to_visit = self.positions_to_check();
        for p in to_visit {
            let is_currently_flipped = self.flipped_tiles.contains(&p);
            let num_flipped_neighbours = self.count_flipped_neighbours(&p);
            if (is_currently_flipped && (num_flipped_neighbours == 1 || num_flipped_neighbours == 2))
                || (!is_currently_flipped && num_flipped_neighbours == 2) {
                new_flipped.insert(p);
            }
        }
        self.flipped_tiles = new_flipped;
    }

    fn positions_to_check(&mut self) -> HashSet<Coordinate> {
        let mut to_visit = HashSet::new();
        for position in &self.flipped_tiles {
            to_visit.insert(*position);

            let neighbours = self.neighbour_map.get(position);    
            for n in neighbours {
                to_visit.insert(*n);
            }
        }
        to_visit
    }

    fn count_flipped_neighbours(&mut self, position: &Coordinate) -> usize {
        let neighbours = self.neighbour_map.get(position);    
        self.flipped_tiles.intersection(neighbours).count()
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

static DIRECTIONS: &[Direction; 6] = &[
    Direction::East,
    Direction::Southeast,
    Direction::Southwest,
    Direction::West,
    Direction::Northwest,
    Direction::Northeast,
];

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

#[derive(Debug)]
struct NeighbourMap {
    map: HashMap<Coordinate, HashSet<Coordinate>>,
}

impl NeighbourMap {
    fn new() -> NeighbourMap {
        NeighbourMap {
            map: HashMap::new(),
        }
    }

    fn get(&mut self, position: &Coordinate) -> &HashSet<Coordinate> {
        if !self.map.contains_key(position) {
            let neighbours = self.calculate_neighbours(position);
            self.map.insert(*position, neighbours);
        }
        self.map.get(position).unwrap()
    }

    fn calculate_neighbours(&self, position: &Coordinate) -> HashSet<Coordinate> {
        DIRECTIONS.iter().map(|d| position.in_direction(d)).collect()
    }
}

fn part1(input: &str) -> usize {
    let floor = Floor::parse(input);
    floor.count_flipped_tiles()
}

fn part2(input: &str, days: usize) -> usize {
    let mut floor = Floor::parse(input);
    floor.run(days);
    floor.count_flipped_tiles()
}

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

    #[test]
    fn test_part2_example1() {
        assert_eq!(part2(EXAMPLE1, 0), 10);

        assert_eq!(part2(EXAMPLE1, 1), 15);
        assert_eq!(part2(EXAMPLE1, 2), 12);
        assert_eq!(part2(EXAMPLE1, 3), 25);
        assert_eq!(part2(EXAMPLE1, 4), 14);
        assert_eq!(part2(EXAMPLE1, 5), 23);
        assert_eq!(part2(EXAMPLE1, 6), 28);
        assert_eq!(part2(EXAMPLE1, 7), 41);
        assert_eq!(part2(EXAMPLE1, 8), 37);
        assert_eq!(part2(EXAMPLE1, 9), 49);
        assert_eq!(part2(EXAMPLE1, 10), 37);

        assert_eq!(part2(EXAMPLE1, 20), 132);
        assert_eq!(part2(EXAMPLE1, 30), 259);
        assert_eq!(part2(EXAMPLE1, 40), 406);
        assert_eq!(part2(EXAMPLE1, 50), 566);
        assert_eq!(part2(EXAMPLE1, 60), 788);
        assert_eq!(part2(EXAMPLE1, 70), 1106);
        assert_eq!(part2(EXAMPLE1, 80), 1373);
        assert_eq!(part2(EXAMPLE1, 90), 1844);
        assert_eq!(part2(EXAMPLE1, 100), 2208);
    }

    #[test]
    fn test_part2_solution() {
        let result = part2(&read_input_file(), 100);
        assert_eq!(result, 3900);
    }
}