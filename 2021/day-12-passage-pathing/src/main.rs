use std::collections::HashMap;
use std::fs;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    println!("part 2 result: {:?}", part2(&read_input_file()));
}

fn read_input_file() -> String {
    fs::read_to_string("input.txt").expect("Something went wrong reading the file")
}

fn parse(input: &str) -> Vec<Connection> {
    input.lines().map(|line| Connection::parse(line)).collect()
}

static START: &str = "start";
static END: &str = "end";

#[derive(Debug)]
struct Connection {
    from: String,
    to: String,
}

impl Connection {
    fn parse(input: &str) -> Connection {
        let parts: Vec<&str> = input.split("-").collect();
        assert_eq!(parts.len(), 2);
        Connection {
            from: parts[0].to_string(),
            to: parts[1].to_string(),
        }
    }
}

type CaveMap = HashMap<String, Vec<String>>;

fn build_route_map(connections: &[Connection]) -> CaveMap {
    let mut map = HashMap::new();

    // forwards
    for connection in connections {
        let entry = map.entry(connection.from.clone()).or_insert(vec![]);
        entry.push(connection.to.clone());
    }

    // backwards
    for connection in connections {
        let entry = map.entry(connection.to.clone()).or_insert(vec![]);
        entry.push(connection.from.clone());
    }

    map
}

#[derive(Clone, Debug)]
struct Route {
    caves: Vec<String>,
    used_double_visit: bool,
}

impl Route {
    fn starting_route() -> Route {
        Route {
            caves: vec![START.to_string()],
            used_double_visit: false,
        }
    }
}

fn calculate_paths(connections: &[Connection], allow_double_visit: bool) -> Vec<Route> {
    let route_map = build_route_map(connections);

    let starting_route = Route::starting_route();
    let mut open_routes = vec![starting_route];
    let mut finished_routes = vec![];

    while open_routes.len() > 0 {
        let (next_open_routes, mut newly_finished_routes) =
            follow_next_paths(&open_routes, &route_map, allow_double_visit);
        open_routes = next_open_routes;
        finished_routes.append(&mut newly_finished_routes);
    }

    finished_routes
}

fn is_small_cave(name: &str) -> bool {
    name.chars().all(|c| c.is_lowercase())
}

fn follow_next_paths(
    starting_routes: &[Route],
    route_map: &CaveMap,
    allow_double_visit: bool,
) -> (Vec<Route>, Vec<Route>) {
    let mut open_routes: Vec<Route> = vec![];
    let mut finished_routes: Vec<Route> = vec![];
    for route in starting_routes {
        let possible_paths = route_map.get(route.caves.last().unwrap()).unwrap();
        let paths_to_follow = possible_paths
            .iter()
            .filter_map(|p| should_follow(p, route, allow_double_visit));
        for (path, counts_as_double_visit) in paths_to_follow {
            let is_end = path == END;

            let mut new_route: Route = route.clone();
            new_route.caves.push(path);
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
// - yes: Some(path, counts_as_double_visit)
// - no: None
fn should_follow(path: &String, route: &Route, allow_double_visit: bool) -> Option<(String, bool)> {
    if path == START {
        // don't go back to start
        None
    } else if is_small_cave(path) {
        if route.caves.contains(path) {
            if allow_double_visit && !route.used_double_visit {
                // small cave but first double visit
                Some((path.to_string(), true))
            } else {
                // small cave we don't want to revisit
                None
            }
        } else {
            // small cave we haven't visited yet
            Some((path.to_string(), false))
        }
    } else {
        // either:
        // - big cave, visit as many times as we want
        // - end, will only be visited once
        Some((path.to_string(), false))
    }
}

fn part1(input: &str) -> usize {
    let connections = parse(input);
    let paths = calculate_paths(&connections, false);
    for p in &paths {
        println!("{:?}", p);
    }
    paths.len()
}

fn part2(input: &str) -> usize {
    let connections = parse(input);
    let paths = calculate_paths(&connections, true);
    for p in &paths {
        println!("{:?}", p);
    }
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
