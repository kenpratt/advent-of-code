use std::collections::HashMap;
use std::fs;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    // println!("part 2 result: {:?}", part2(&read_input_file()));
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

type Route = Vec<String>;
type RouteMap = HashMap<String, Route>;

fn build_route_map(connections: &[Connection]) -> RouteMap {
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

fn calculate_paths(connections: &[Connection]) -> Vec<Route> {
    let route_map = build_route_map(connections);

    let starting_route = vec![START.to_string()];
    let mut open_routes = vec![starting_route];
    let mut finished_routes = vec![];

    while open_routes.len() > 0 {
        let (next_open_routes, mut newly_finished_routes) =
            follow_next_paths(&open_routes, &route_map);
        open_routes = next_open_routes;
        finished_routes.append(&mut newly_finished_routes);
    }

    finished_routes
}

fn is_small_cave(name: &str) -> bool {
    name.chars().all(|c| c.is_lowercase())
}

fn follow_next_paths(starting_routes: &[Route], route_map: &RouteMap) -> (Vec<Route>, Vec<Route>) {
    let mut open_routes = vec![];
    let mut finished_routes = vec![];
    for route in starting_routes {
        let possible_paths = route_map.get(route.last().unwrap()).unwrap();
        let paths_to_follow = possible_paths
            .iter()
            .filter(|p| !(is_small_cave(p) && route.contains(p)));
        for path in paths_to_follow {
            let mut new_route = route.clone();
            new_route.push(path.to_string());
            if path == END {
                finished_routes.push(new_route);
            } else {
                open_routes.push(new_route);
            }
        }
    }
    (open_routes, finished_routes)
}

fn part1(input: &str) -> usize {
    let connections = parse(input);
    println!("connections: {:?}", connections);
    let paths: Vec<Vec<String>> = calculate_paths(&connections);
    println!("paths: {:?}", paths);
    paths.len()
}

// fn part2(input: &str) -> usize {
//     let data = Data::parse(input);
//     println!("{:?}", data);
//     data.execute()
// }

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
