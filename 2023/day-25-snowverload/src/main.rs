use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    // println!("part 2 result: {:?}", part2(&read_input_file()));
}

fn read_input_file() -> String {
    fs::read_to_string("input.txt").expect("Something went wrong reading the file")
}

#[derive(Debug)]
struct ConnectionMap<'a> {
    conns: HashMap<&'a str, HashSet<&'a str>>,
}

impl<'a> ConnectionMap<'a> {
    fn parse(input: &'a str) -> Self {
        let lines = input.lines().map(|line| Self::parse_line(line));

        let mut conns: HashMap<&str, HashSet<&str>> = HashMap::new();
        for (from, tos) in lines {
            for to in tos {
                conns.entry(from).or_default().insert(to);
                conns.entry(to).or_default().insert(from);
            }
        }

        Self { conns }
    }

    fn parse_line(input: &str) -> (&str, Vec<&str>) {
        // jqt: rhn xhk nvd
        let mut parts = input.split(": ");
        let left = parts.next().unwrap();
        let right = parts.next().unwrap().split(" ").collect();
        assert_eq!(parts.next(), None);
        (left, right)
    }

    fn disconnect_groups(&self) -> (usize, usize) {
        // TODO
        (self.conns.len(), 0)
    }
}

fn part1(input: &str) -> usize {
    let map = ConnectionMap::parse(input);
    let (a, b) = map.disconnect_groups();
    a * b
}

// fn part2(input: &str) -> usize {
//     let items = Data::parse(input);
//     dbg!(&items);
//     0
// }

#[cfg(test)]
mod tests {
    use super::*;

    fn read_example_file() -> String {
        fs::read_to_string("example.txt").expect("Something went wrong reading the file")
    }

    #[test]
    fn test_part1_example() {
        let result = part1(&read_example_file());
        assert_eq!(result, 54);
    }

    // #[test]
    // fn test_part1_solution() {
    //     let result = part1(&read_input_file());
    //     assert_eq!(result, 0);
    // }

    // #[test]
    // fn test_part2_example() {
    //     let result = part2(&read_example_file());
    //     assert_eq!(result, 0);
    // }

    // #[test]
    // fn test_part2_solution() {
    //     let result = part2(&read_input_file());
    //     assert_eq!(result, 0);
    // }
}
