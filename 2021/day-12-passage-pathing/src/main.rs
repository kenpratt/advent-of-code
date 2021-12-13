use std::collections::HashMap;
use std::fs;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    println!("part 2 result: {:?}", part2(&read_input_file()));
}

fn read_input_file() -> String {
    fs::read_to_string("input.txt").expect("Something went wrong reading the file")
}

const START: &str = "start";
const END: &str = "end";

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Cave {
    Start,
    End,
    Big(u8),
    Small(u8),
}

fn is_small_cave(name: &str) -> bool {
    name.chars().all(|c| c.is_lowercase())
}

#[derive(Debug)]
struct CaveMap {
    aliases: HashMap<String, u8>,
    connections: HashMap<Cave, Vec<Cave>>,
}

impl CaveMap {
    fn parse(input: &str) -> CaveMap {
        let mut map = CaveMap {
            aliases: HashMap::new(),
            connections: HashMap::new(),
        };
        for line in input.lines() {
            let (from, to) = map.parse_line(line);
            map.add_connection(from, to);
            map.add_connection(to, from);
        }
        map
    }

    fn parse_line(&mut self, line: &str) -> (Cave, Cave) {
        let parts: Vec<Cave> = line.split("-").map(|s| self.parse_cave(s)).collect();
        assert_eq!(parts.len(), 2);
        (parts[0], parts[1])
    }

    fn parse_cave(&mut self, str: &str) -> Cave {
        match str {
            START => Cave::Start,
            END => Cave::End,
            _ if is_small_cave(str) => Cave::Small(self.alias(str.to_string())),
            _ => Cave::Big(self.alias(str.to_string())),
        }
    }

    fn alias(&mut self, str: String) -> u8 {
        let maybe_value = self.aliases.get(&str);
        match maybe_value {
            Some(value) => *value,
            None => {
                let value = self.aliases.len() as u8;
                self.aliases.insert(str, value);
                value
            }
        }
    }

    fn add_connection(&mut self, from: Cave, to: Cave) {
        let entry = self.connections.entry(from).or_insert(vec![]);
        entry.push(to);
    }

    fn caves_leading_from(&self, from: &Cave) -> &Vec<Cave> {
        self.connections.get(from).unwrap()
    }
}

#[derive(Clone, Debug)]
struct Route {
    caves: Vec<Cave>,
    used_double_visit: bool,
}

impl Route {
    fn starting_route() -> Route {
        Route {
            caves: vec![Cave::Start],
            used_double_visit: false,
        }
    }
}

fn calculate_paths(map: &CaveMap, allow_double_visit: bool) -> Vec<Route> {
    let starting_route = Route::starting_route();
    let mut open_routes = vec![starting_route];
    let mut finished_routes = vec![];

    while open_routes.len() > 0 {
        let (next_open_routes, mut newly_finished_routes) =
            follow_next_paths(&open_routes, &map, allow_double_visit);
        open_routes = next_open_routes;
        finished_routes.append(&mut newly_finished_routes);
    }

    finished_routes
}

fn follow_next_paths(
    starting_routes: &[Route],
    map: &CaveMap,
    allow_double_visit: bool,
) -> (Vec<Route>, Vec<Route>) {
    let mut open_routes: Vec<Route> = vec![];
    let mut finished_routes: Vec<Route> = vec![];
    for route in starting_routes {
        let possible_caves = map.caves_leading_from(route.caves.last().unwrap());
        let caves_to_visit = possible_caves
            .iter()
            .filter_map(|cave| should_visit(cave, route, allow_double_visit));
        for (cave, counts_as_double_visit) in caves_to_visit {
            let is_end = cave == Cave::End;
            let mut new_route: Route = route.clone();
            new_route.caves.push(cave);
            if counts_as_double_visit && !new_route.used_double_visit {
                new_route.used_double_visit = true;
            }
            if is_end {
                finished_routes.push(new_route);
            } else {
                open_routes.push(new_route);
            }
        }
    }
    (open_routes, finished_routes)
}

// output:
// - yes: Some(cave, counts_as_double_visit)
// - no: None
fn should_visit(cave: &Cave, route: &Route, allow_double_visit: bool) -> Option<(Cave, bool)> {
    match cave {
        Cave::Start => None,
        Cave::Small(_) => {
            if route.caves.contains(cave) {
                if allow_double_visit && !route.used_double_visit {
                    // small cave but first double visit
                    Some((*cave, true))
                } else {
                    // small cave we don't want to revisit
                    None
                }
            } else {
                // small cave we haven't visited yet
                Some((*cave, false))
            }
        }
        // - big cave, visit as many times as we want
        // - end, will only be visited once
        Cave::Big(_) | Cave::End => Some((*cave, false)),
    }
}

fn part1(input: &str) -> usize {
    let map = CaveMap::parse(input);
    let paths = calculate_paths(&map, false);
    paths.len()
}

fn part2(input: &str) -> usize {
    let map = CaveMap::parse(input);
    let paths = calculate_paths(&map, true);
    paths.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    static EXAMPLE1: &str = indoc! {"
        start-A
        start-b
        A-c
        A-b
        b-d
        A-end
        b-end
    "};

    static EXAMPLE2: &str = indoc! {"
        dc-end
        HN-start
        start-kj
        dc-start
        dc-HN
        LN-dc
        HN-end
        kj-sa
        kj-HN
        kj-dc
    "};

    static EXAMPLE3: &str = indoc! {"
        fs-end
        he-DX
        fs-he
        start-DX
        pj-DX
        end-zg
        zg-sl
        zg-pj
        pj-he
        RW-he
        fs-DX
        pj-RW
        zg-RW
        start-pj
        he-WI
        zg-he
        pj-fs
        start-RW
    "};

    #[test]
    fn test_part1_example1() {
        let result = part1(EXAMPLE1);
        assert_eq!(result, 10);
    }

    #[test]
    fn test_part1_example2() {
        let result = part1(EXAMPLE2);
        assert_eq!(result, 19);
    }

    #[test]
    fn test_part1_example3() {
        let result = part1(EXAMPLE3);
        assert_eq!(result, 226);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file());
        assert_eq!(result, 3679);
    }

    #[test]
    fn test_part2_example1() {
        let result = part2(EXAMPLE1);
        assert_eq!(result, 36);
    }

    #[test]
    fn test_part2_example2() {
        let result = part2(EXAMPLE2);
        assert_eq!(result, 103);
    }

    #[test]
    fn test_part2_example3() {
        let result = part2(EXAMPLE3);
        assert_eq!(result, 3509);
    }

    #[test]
    fn test_part2_solution() {
        let result = part2(&read_input_file());
        assert_eq!(result, 107395);
    }
}
